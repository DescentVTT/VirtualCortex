//! The `.cortex` image on disk: the writer, the loader and the write-ahead log the clock sweep
//! evicts into (whitepaper §5.2.2, §6.7, §8.6, §8.7; ADR-0024).
//!
//! Layout: the 64-byte header, `section_count` directory entries of 64 bytes, then the sections,
//! each starting on a 64-byte boundary and sealed by a CRC-64/XZ in its entry. The loader reads
//! the whole file into memory and decodes every record into the arenas (a copy; `mmap` is
//! Specified); it fails closed on a foreign version, a bad checksum, a truncated file, a
//! malformed directory, a record that is not at rest, a dangling index or a delay the wheel
//! cannot hold. An image is written at a quiescent point: every mailbox empty, no token in
//! flight; a scheduled unit with an empty mailbox is written idle and woken again on load.

use crate::executor::{Config, ConfigError, Executor};
use cortex_connectome::{
    CortexFileHeader, Crc64, HeaderError, SECTION_NEURON, SECTION_PLASTIC_DELTA, SECTION_SYNAPSE,
    SectionEntry, crc64,
};
use cortex_core::{DendriticSuperNeuron, MAX_TOKEN_BLOCK, PlasticDelta, SynapseBlock, WorkerWheel};
use std::fs::{self, File, OpenOptions};
use std::io;
use std::path::Path;

/// Why an image could not be written or read.
#[derive(Debug)]
pub enum ImageError {
    /// The file system said no.
    Io(io::Error),
    /// The header failed validation (magic, version, checksum, padding).
    Header(HeaderError),
    /// The file ends before a section or the directory does.
    Truncated,
    /// A directory entry is malformed: unaligned, a partial record, or an unknown record size
    /// for its kind.
    Directory(u32),
    /// A required section (neurons, synapses) is missing.
    MissingSection(u32),
    /// A section's bytes do not match its checksum.
    SectionCrc(u32),
    /// A unit is not at rest: its gate is not idle or its mailbox is not empty.
    NotAtRest(u32),
    /// A synapse targets a unit outside the arena, or a chain or delta list names an index
    /// outside its arena.
    DanglingIndex { block: u32, slot: u8 },
    /// A synapse's delay is at or beyond the wheel's horizon (§6.2).
    DelayBeyondHorizon { block: u32, slot: u8 },
    /// More blocks than a synapse token can name (finding F-23).
    TooManyBlocks(u64),
    /// The executor's configuration was refused.
    Config(ConfigError),
    /// The executor is not quiescent: a mailbox holds a message or a token is in flight.
    NotQuiescent,
    /// No write-ahead log is attached, so nothing can be evicted or re-hydrated.
    NoLog,
    /// The log entry for a unit is missing or short.
    LogCorrupt(u32),
}

impl From<io::Error> for ImageError {
    fn from(e: io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<HeaderError> for ImageError {
    fn from(e: HeaderError) -> Self {
        Self::Header(e)
    }
}

impl From<ConfigError> for ImageError {
    fn from(e: ConfigError) -> Self {
        Self::Config(e)
    }
}

/// Rounds up to the next multiple of 64.
fn align64(n: u64) -> u64 {
    n.div_ceil(64) * 64
}

/// The bytes of one entry of the write-ahead log: the unit index, then the record.
const LOG_ENTRY: usize = 8 + 64;

/// The write-ahead log the clock sweep evicts unit records into (§8.6): append-only, one entry
/// per eviction, the newest entry of a unit the one re-hydration reads. Positional reads and
/// writes, so no seek state is shared between workers; only the coordinator touches it.
pub struct WriteAheadLog {
    file: File,
    len: u64,
    /// The offset of each unit's newest entry, or `u64::MAX` for none.
    offsets: Vec<u64>,
}

impl WriteAheadLog {
    /// Creates (or truncates) the log at `path` for `units` units.
    pub fn create(path: &Path, units: usize) -> io::Result<Self> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(true)
            .open(path)?;
        Ok(Self {
            file,
            len: 0,
            offsets: vec![u64::MAX; units],
        })
    }

    /// Appends `unit`'s record and remembers where it is.
    pub fn append(&mut self, unit: u32, record: &[u8; 64]) -> io::Result<()> {
        let mut entry = [0u8; LOG_ENTRY];
        entry[0..8].copy_from_slice(&(unit as u64).to_le_bytes());
        entry[8..].copy_from_slice(record);
        write_at(&self.file, self.len, &entry)?;
        self.offsets[unit as usize] = self.len;
        self.len += LOG_ENTRY as u64;
        Ok(())
    }

    /// True when the log holds a record for `unit`.
    pub fn holds(&self, unit: u32) -> bool {
        self.offsets
            .get(unit as usize)
            .is_some_and(|&o| o != u64::MAX)
    }

    /// The newest record of `unit`.
    pub fn read(&self, unit: u32) -> Result<[u8; 64], ImageError> {
        let offset = self
            .offsets
            .get(unit as usize)
            .copied()
            .filter(|&o| o != u64::MAX)
            .ok_or(ImageError::LogCorrupt(unit))?;
        let mut entry = [0u8; LOG_ENTRY];
        read_at(&self.file, offset, &mut entry)?;
        if u64::from_le_bytes(entry[0..8].try_into().unwrap_or([0; 8])) != unit as u64 {
            return Err(ImageError::LogCorrupt(unit));
        }
        let mut record = [0u8; 64];
        record.copy_from_slice(&entry[8..]);
        Ok(record)
    }

    /// Entries appended so far.
    pub fn entries(&self) -> u64 {
        self.len / LOG_ENTRY as u64
    }
}

#[cfg(unix)]
fn read_at(file: &File, offset: u64, buf: &mut [u8]) -> io::Result<()> {
    use std::os::unix::fs::FileExt;
    file.read_exact_at(buf, offset)
}

#[cfg(unix)]
fn write_at(file: &File, offset: u64, buf: &[u8]) -> io::Result<()> {
    use std::os::unix::fs::FileExt;
    file.write_all_at(buf, offset)
}

#[cfg(windows)]
fn read_at(file: &File, mut offset: u64, mut buf: &mut [u8]) -> io::Result<()> {
    use std::os::windows::fs::FileExt;
    while !buf.is_empty() {
        let n = file.seek_read(buf, offset)?;
        if n == 0 {
            return Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "the log ends early",
            ));
        }
        offset += n as u64;
        buf = &mut buf[n..];
    }
    Ok(())
}

#[cfg(windows)]
fn write_at(file: &File, mut offset: u64, mut buf: &[u8]) -> io::Result<()> {
    use std::os::windows::fs::FileExt;
    while !buf.is_empty() {
        let n = file.seek_write(buf, offset)?;
        if n == 0 {
            return Err(io::Error::new(io::ErrorKind::WriteZero, "nothing written"));
        }
        offset += n as u64;
        buf = &buf[n..];
    }
    Ok(())
}

/// The `.cortex` writer and loader.
pub struct Image;

impl Image {
    /// The image of `exec`'s arenas at a quiescent point, as bytes: header, directory, the
    /// neuron, synapse and (if the executor has one) delta sections. An evicted unit's record
    /// is read back from the log. A scheduled unit with an empty mailbox is written idle.
    pub fn encode<const CAP: usize>(exec: &Executor<CAP>) -> Result<Vec<u8>, ImageError> {
        if !exec.is_quiescent() {
            return Err(ImageError::NotQuiescent);
        }
        let units = exec.units();
        let blocks = exec.blocks();
        let deltas = exec.deltas();
        let mut neuron_bytes = Vec::with_capacity(units.len() * 64);
        for (i, unit) in units.iter().enumerate() {
            let record = if exec.is_evicted(i as u32) {
                exec.log().ok_or(ImageError::NoLog)?.read(i as u32)?
            } else {
                if !unit.mailbox_is_empty() {
                    return Err(ImageError::NotAtRest(i as u32));
                }
                let mut bytes = unit.encode();
                bytes[56] = 0; // a scheduled unit is written idle; the loader wakes it
                bytes
            };
            neuron_bytes.extend_from_slice(&record);
        }
        let mut synapse_bytes = Vec::with_capacity(blocks.len() * 64);
        for block in blocks {
            synapse_bytes.extend_from_slice(&block.encode());
        }
        let mut delta_bytes = Vec::with_capacity(deltas.len() * 16);
        for delta in deltas {
            delta_bytes.extend_from_slice(&delta.encode());
        }
        let mut sections: Vec<(u32, u32, Vec<u8>)> = vec![
            (SECTION_NEURON, 64, neuron_bytes),
            (SECTION_SYNAPSE, 64, synapse_bytes),
        ];
        if !deltas.is_empty() {
            sections.push((SECTION_PLASTIC_DELTA, 16, delta_bytes));
        }
        let directory_len = 64 * sections.len() as u64;
        let mut offset = 64 + directory_len;
        let mut entries = Vec::with_capacity(sections.len());
        for (kind, record_size, bytes) in &sections {
            entries.push(SectionEntry::new(
                *kind,
                *record_size,
                offset,
                bytes.len() as u64,
                crc64(bytes),
            ));
            offset += align64(bytes.len() as u64);
        }
        let header = CortexFileHeader::new(
            0,
            units.len() as u64,
            blocks.len() as u64,
            sections.len() as u32,
        );
        let mut out = Vec::with_capacity(offset as usize);
        out.extend_from_slice(&header.encode());
        for entry in &entries {
            out.extend_from_slice(&entry.encode());
        }
        for (entry, (_, _, bytes)) in entries.iter().zip(&sections) {
            debug_assert_eq!(out.len() as u64, entry.offset);
            out.extend_from_slice(bytes);
            out.resize(align64(out.len() as u64) as usize, 0);
        }
        Ok(out)
    }

    /// Writes the image of `exec` to `path`.
    pub fn write<const CAP: usize>(exec: &Executor<CAP>, path: &Path) -> Result<(), ImageError> {
        let bytes = Self::encode(exec)?;
        fs::write(path, bytes)?;
        Ok(())
    }

    /// Opens the image at `path` into a new executor: the arena sizes come from the image, the
    /// rest of the configuration from `config`. Fails closed on anything malformed.
    pub fn open<const CAP: usize>(
        path: &Path,
        config: Config,
    ) -> Result<Executor<CAP>, ImageError> {
        let bytes = fs::read(path)?;
        Self::decode(&bytes, config)
    }

    /// [`open`](Self::open) from bytes already in memory.
    pub fn decode<const CAP: usize>(
        bytes: &[u8],
        config: Config,
    ) -> Result<Executor<CAP>, ImageError> {
        let header_bytes: &[u8; 64] = bytes
            .get(0..64)
            .and_then(|b| b.try_into().ok())
            .ok_or(ImageError::Truncated)?;
        let header = CortexFileHeader::decode(header_bytes);
        header.validate()?;
        let mut entries = Vec::with_capacity(header.section_count as usize);
        for i in 0..header.section_count as usize {
            let start = 64 + 64 * i;
            let entry_bytes: &[u8; 64] = bytes
                .get(start..start + 64)
                .and_then(|b| b.try_into().ok())
                .ok_or(ImageError::Truncated)?;
            let entry = SectionEntry::decode(entry_bytes);
            let expected_size = match entry.kind {
                SECTION_NEURON | SECTION_SYNAPSE => 64,
                SECTION_PLASTIC_DELTA => 16,
                other => return Err(ImageError::Directory(other)),
            };
            if !entry.is_well_formed() || entry.record_size != expected_size {
                return Err(ImageError::Directory(entry.kind));
            }
            let end = entry
                .offset
                .checked_add(entry.length)
                .ok_or(ImageError::Truncated)?;
            let section = bytes
                .get(entry.offset as usize..end as usize)
                .ok_or(ImageError::Truncated)?;
            let mut crc = Crc64::new();
            crc.update(section);
            if crc.finish() != entry.crc64 {
                return Err(ImageError::SectionCrc(entry.kind));
            }
            entries.push(entry);
        }
        let find = |kind: u32| entries.iter().find(|e| e.kind == kind).copied();
        let neuron = find(SECTION_NEURON).ok_or(ImageError::MissingSection(SECTION_NEURON))?;
        let synapse = find(SECTION_SYNAPSE).ok_or(ImageError::MissingSection(SECTION_SYNAPSE))?;
        let delta = find(SECTION_PLASTIC_DELTA);
        if neuron.record_count() != header.num_neurons
            || synapse.record_count() != header.num_synapses
        {
            return Err(ImageError::Directory(SECTION_NEURON));
        }
        if synapse.record_count() > MAX_TOKEN_BLOCK as u64 + 1 {
            return Err(ImageError::TooManyBlocks(synapse.record_count()));
        }
        let units = neuron.record_count() as usize;
        let blocks = synapse.record_count() as usize;
        let deltas = delta.map_or(0, |d| d.record_count() as usize);
        let mut exec = Executor::<CAP>::new(Config {
            units,
            blocks,
            deltas,
            ..config
        })?;
        let horizon = WorkerWheel::horizon_ticks();
        {
            let arena = exec.blocks_mut();
            for (i, record) in bytes
                [synapse.offset as usize..(synapse.offset + synapse.length) as usize]
                .chunks_exact(64)
                .enumerate()
            {
                let block = SynapseBlock::decode(record.try_into().unwrap_or(&[0; 64]));
                for slot in 0..4 {
                    if let Some(target) = block.target(slot) {
                        if target as usize >= units {
                            return Err(ImageError::DanglingIndex {
                                block: i as u32,
                                slot: slot as u8,
                            });
                        }
                        if block.delays_ticks[slot] as u64 >= horizon {
                            return Err(ImageError::DelayBeyondHorizon {
                                block: i as u32,
                                slot: slot as u8,
                            });
                        }
                    }
                }
                if block.next().is_some_and(|n| n as usize >= blocks) {
                    return Err(ImageError::DanglingIndex {
                        block: i as u32,
                        slot: 4,
                    });
                }
                arena[i] = block;
            }
        }
        if let Some(delta) = delta {
            let arena = exec.deltas_mut();
            for (i, record) in bytes[delta.offset as usize..(delta.offset + delta.length) as usize]
                .chunks_exact(16)
                .enumerate()
            {
                let d = PlasticDelta::decode(record.try_into().unwrap_or(&[0; 16]));
                if d.block().is_some_and(|b| b as usize >= blocks)
                    || d.next_delta().is_some_and(|n| n as usize >= deltas)
                {
                    return Err(ImageError::DanglingIndex {
                        block: i as u32,
                        slot: 5,
                    });
                }
                arena[i] = d;
            }
        }
        let mut wake = Vec::new();
        {
            let arena = exec.units_mut();
            for (i, record) in bytes
                [neuron.offset as usize..(neuron.offset + neuron.length) as usize]
                .chunks_exact(64)
                .enumerate()
            {
                let unit = DendriticSuperNeuron::decode(record.try_into().unwrap_or(&[0; 64]));
                if !unit.is_at_rest_image() {
                    return Err(ImageError::NotAtRest(i as u32));
                }
                if unit.first_block().is_some_and(|b| b as usize >= blocks)
                    || unit.delta_head().is_some_and(|d| d as usize >= deltas)
                {
                    return Err(ImageError::DanglingIndex {
                        block: i as u32,
                        slot: 6,
                    });
                }
                if !crate::executor::at_rest(&unit) {
                    wake.push(i as u32);
                }
                arena[i] = unit;
            }
        }
        exec.wake_now(&wake);
        Ok(exec)
    }
}
