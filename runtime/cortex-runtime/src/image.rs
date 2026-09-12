//! The `.cortex` image on disk: the writer, the loader and the write-ahead log the clock sweep
//! evicts into (whitepaper §5.2.2, §6.7, §8.6, §8.7; ADR-0024).
//!
//! Layout: the 64-byte header, `section_count` directory entries of 64 bytes, then the sections,
//! each starting on a 64-byte boundary and sealed by a CRC-64/XZ in its entry. The loader reads
//! the whole file into memory and decodes every record into the arenas (a copy; `mmap` is
//! Specified); it fails closed on a foreign version, a tick duration that is not this build's
//! (ADR-0033), a bad checksum, a truncated file, a malformed directory, a record that is not
//! at rest, a dangling index or a delay the wheel cannot hold. An image is written at a
//! quiescent point: every mailbox empty, no token in flight; a scheduled unit with an empty
//! mailbox is written idle and woken again on load; the executor's clock is written with it
//! and resumed by the loader, so the stamps keep their meaning (ADR-0033). The engine's
//! modulation state (ADR-0032: the modulator record and the baseline) is a section of its
//! own, always written, so that the image defines the run (§8.3).

use crate::executor::{AmendError, Config, ConfigError, Executor, InjectError};
use cortex_connectome::{
    CortexFileHeader, Crc64, HeaderError, SECTION_AMENDMENT, SECTION_HOMEOSTASIS,
    SECTION_MODULATOR, SECTION_NEURON, SECTION_PLASTIC_DELTA, SECTION_SYNAPSE, SectionEntry, crc64,
};
use cortex_core::{
    DendriticSuperNeuron, MAX_TOKEN_BLOCK, PlasticDelta, SYNAPSES_PER_BLOCK, SynapseBlock, TICK_NS,
    WorkerWheel,
};
use cortex_executive::PolicyAmendment;
use cortex_homeostasis::{CONTROL_STEP_MAX_Q0_16, HomeostaticDrivePool};
use cortex_neuromod::NeuromodulatorState;
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
    /// A required section (neurons, synapses, the modulation state) is missing.
    MissingSection(u32),
    /// A section's bytes do not match its checksum.
    SectionCrc(u32),
    /// A unit is not at rest: its gate is not idle, its mailbox is not empty, or a reserved
    /// byte is not zero.
    NotAtRest(u32),
    /// A synapse targets a unit outside the arena, a chain or delta list names an index
    /// outside its arena, or a delta names a slot its block does not have (`slot` is 5).
    DanglingIndex { block: u32, slot: u8 },
    /// A reserved or padding byte of a synapse block or a delta is not zero (ADR-0028).
    ReservedNotZero { section: u32, index: u32 },
    /// A synapse's delay is at or beyond the wheel's horizon (§6.2).
    DelayBeyondHorizon { block: u32, slot: u8 },
    /// More blocks than a synapse token can name (finding F-23).
    TooManyBlocks(u64),
    /// The executor's configuration was refused.
    Config(ConfigError),
    /// The executor is not quiescent: a mailbox holds a message, a token is in flight, or the
    /// injector ring holds a pair not yet drained.
    NotQuiescent,
    /// No write-ahead log is attached, so nothing can be evicted or re-hydrated.
    NoLog,
    /// The log entry for a unit is missing or short.
    LogCorrupt(u32),
    /// An amendment record (ADR-0031) is one its state machine could not have produced, is
    /// out of order, or claims a commit that does not follow from the ones before it.
    MalformedAmendment(u32),
    /// A trial (ADR-0031) was asked for an amendment the arena does not hold or has not
    /// admitted.
    Amendment(AmendError),
    /// The image a trial forks does not carry the value the amendment started from: it was
    /// written before a later commit to that parameter, so its baseline is not the live one.
    StaleBaseline(u16),
    /// A fork's spike train is not wholly traced (`Config::trace_capacity` is zero, or a spike
    /// was dropped), so its behaviour hash would not cover it.
    NoTrace,
    /// An injection into a fork was refused.
    Injection(InjectError),
    /// The header's tick duration is not the one every `*_ticks` field of this build counts
    /// (`cortex_core::TICK_NS`; ADR-0033): the image's delays and stamps would mean other
    /// times.
    TickMismatch(u32),
    /// The homeostasis record (section kind 43, ADR-0036) is not one the rules produce: a gain
    /// outside its bounds, a full window, a count above the cap, a sum beyond what the pairs
    /// allow, a reserved byte, or a window whose bin count is not the one the clock at the
    /// write implies.
    MalformedHomeostasis,
}

/// Blocks a synapse token can name: $2^{26}$ (finding F-23, ADR-0024). A literal, so that no
/// arithmetic in the loader's bound exists for a mutant to touch; the assertion ties it to the
/// token's constant.
const MAX_BLOCKS: u64 = 67_108_864;
const _: () = assert!(MAX_BLOCKS == MAX_TOKEN_BLOCK as u64 + 1);

/// True for more blocks than a token can name. Its own function, so that the mutants of the
/// comparison at a bound no image can reach are excluded by this name and nothing else.
pub(crate) fn too_many_blocks(count: u64) -> bool {
    count > MAX_BLOCKS
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

/// Rounds up to the next multiple of 64; saturates above `u64::MAX - 63`, which no length held
/// in memory reaches.
fn align64(n: u64) -> u64 {
    n.div_ceil(64).saturating_mul(64)
}

/// The bytes of one entry of the write-ahead log: the unit index, then the record.
const LOG_ENTRY: usize = 8 + 64;
/// The same as a file length: a constant divisor for [`WriteAheadLog::entries`].
const LOG_ENTRY_BYTES: u64 = LOG_ENTRY as u64;

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

    /// Appends `unit`'s record and remembers where it is. A unit outside the log is refused
    /// before anything is written (`InvalidInput`).
    pub fn append(&mut self, unit: u32, record: &[u8; 64]) -> io::Result<()> {
        if unit as usize >= self.offsets.len() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "unit outside the log",
            ));
        }
        let mut entry = [0u8; LOG_ENTRY];
        entry[0..8].copy_from_slice(&(unit as u64).to_le_bytes());
        entry[8..].copy_from_slice(record);
        write_at(&self.file, self.len, &entry)?;
        self.offsets[unit as usize] = self.len;
        self.len = self.len.saturating_add(LOG_ENTRY_BYTES);
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
        self.len / LOG_ENTRY_BYTES
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
        offset = offset.saturating_add(n as u64);
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
        offset = offset.saturating_add(n as u64);
        buf = &buf[n..];
    }
    Ok(())
}

/// A section's bytes by its directory entry: the end is a checked sum, so an entry forged to
/// wrap is refused as truncated, like one that runs past the file.
fn section_of<'a>(bytes: &'a [u8], entry: &SectionEntry) -> Result<&'a [u8], ImageError> {
    let end = entry
        .offset
        .checked_add(entry.length)
        .ok_or(ImageError::Truncated)?;
    bytes
        .get(entry.offset as usize..end as usize)
        .ok_or(ImageError::Truncated)
}

/// The `.cortex` writer and loader.
pub struct Image;

impl Image {
    /// The image of `exec`'s arenas at a quiescent point, as bytes: header, directory, the
    /// neuron, synapse and (if the executor has any) delta and amendment sections. An evicted
    /// unit's record is read back from the log. A scheduled unit with an empty mailbox is
    /// written idle.
    pub fn encode<const CAP: usize>(exec: &Executor<CAP>) -> Result<Vec<u8>, ImageError> {
        if !exec.is_quiescent() {
            return Err(ImageError::NotQuiescent);
        }
        let units = exec.units();
        let blocks = exec.blocks();
        let deltas = exec.deltas();
        let mut neuron_bytes = Vec::with_capacity(units.len().saturating_mul(64));
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
        let mut synapse_bytes = Vec::with_capacity(blocks.len().saturating_mul(64));
        for block in blocks {
            synapse_bytes.extend_from_slice(&block.encode());
        }
        let mut delta_bytes = Vec::with_capacity(deltas.len().saturating_mul(16));
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
        let amendments = exec.amendments();
        if !amendments.is_empty() {
            let mut amendment_bytes = Vec::with_capacity(amendments.len().saturating_mul(64));
            for a in amendments {
                amendment_bytes.extend_from_slice(&a.encode());
            }
            sections.push((SECTION_AMENDMENT, 64, amendment_bytes));
        }
        // The engine's modulation state, always: one 64-byte record holding the modulator's 16
        // bytes, the baseline at `[16..20)` and 44 reserved bytes. The baseline changes what a
        // run does, so it is in the image, not in a configuration (§8.3).
        let mut modulator_bytes = vec![0u8; 64];
        modulator_bytes[0..16].copy_from_slice(&exec.modulator().encode());
        modulator_bytes[16..20].copy_from_slice(&exec.modulation_baseline_q16().to_le_bytes());
        sections.push((SECTION_MODULATOR, 64, modulator_bytes));
        // The engine's homeostasis state, always: the gain and the estimator's window change
        // what a run does, so they are in the image (ADR-0036).
        sections.push((
            SECTION_HOMEOSTASIS,
            64,
            exec.homeostasis().encode().to_vec(),
        ));
        // The offsets of an image held in memory: each fits, and saturating says so by name.
        let directory_len = (sections.len() as u64).saturating_mul(64);
        let mut offset = directory_len.saturating_add(64);
        let mut entries = Vec::with_capacity(sections.len());
        for (kind, record_size, bytes) in &sections {
            entries.push(SectionEntry::new(
                *kind,
                *record_size,
                offset,
                bytes.len() as u64,
                crc64(bytes),
            ));
            offset = offset.saturating_add(align64(bytes.len() as u64));
        }
        let header = CortexFileHeader::new(
            0,
            units.len() as u64,
            blocks.len() as u64,
            sections.len() as u32,
            TICK_NS,
            exec.ticks(),
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
        let (header_bytes, after_header) = bytes
            .split_first_chunk::<64>()
            .ok_or(ImageError::Truncated)?;
        let header = CortexFileHeader::decode(header_bytes);
        header.validate()?;
        if header.tick_ns != TICK_NS {
            return Err(ImageError::TickMismatch(header.tick_ns));
        }
        // The entries follow the header, 64 bytes each: the next chunk, not an offset. The
        // directory cannot hold more entries than the file has chunks after the header,
        // whatever a sealed header says; checked before the count sizes an allocation, and
        // counted by the iterator, so no division exists for a mutant to touch.
        let mut directory = after_header.chunks_exact(64);
        if header.section_count as usize > directory.len() {
            return Err(ImageError::Truncated);
        }
        let mut entries = Vec::with_capacity(header.section_count as usize);
        for _ in 0..header.section_count {
            let entry_bytes: &[u8; 64] = directory
                .next()
                .and_then(|b| b.try_into().ok())
                .ok_or(ImageError::Truncated)?;
            let entry = SectionEntry::decode(entry_bytes);
            let expected_size = match entry.kind {
                SECTION_NEURON | SECTION_SYNAPSE | SECTION_AMENDMENT | SECTION_MODULATOR
                | SECTION_HOMEOSTASIS => 64,
                SECTION_PLASTIC_DELTA => 16,
                other => return Err(ImageError::Directory(other)),
            };
            if !entry.is_well_formed() || entry.record_size != expected_size {
                return Err(ImageError::Directory(entry.kind));
            }
            let section = section_of(bytes, &entry)?;
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
        let amendment = find(SECTION_AMENDMENT);
        let modulator =
            find(SECTION_MODULATOR).ok_or(ImageError::MissingSection(SECTION_MODULATOR))?;
        let homeostasis =
            find(SECTION_HOMEOSTASIS).ok_or(ImageError::MissingSection(SECTION_HOMEOSTASIS))?;
        if neuron.record_count() != header.num_neurons
            || synapse.record_count() != header.num_synapses
        {
            return Err(ImageError::Directory(SECTION_NEURON));
        }
        if too_many_blocks(synapse.record_count()) {
            return Err(ImageError::TooManyBlocks(synapse.record_count()));
        }
        let units = neuron.record_count() as usize;
        let blocks = synapse.record_count() as usize;
        let deltas = delta.map_or(0, |d| d.record_count() as usize);
        let amendments = amendment.map_or(0, |a| a.record_count() as usize);
        let mut exec = Executor::<CAP>::new(Config {
            units,
            blocks,
            deltas,
            amendments: amendments.saturating_add(config.amendments),
            ..config
        })?;
        let horizon = WorkerWheel::horizon_ticks();
        {
            let arena = exec.blocks_mut();
            for (i, record) in section_of(bytes, &synapse)?.chunks_exact(64).enumerate() {
                let block = SynapseBlock::decode(record.try_into().unwrap_or(&[0; 64]));
                for slot in 0..4 {
                    // An empty slot carries nothing: a trace or a compartment on one is a
                    // record the writer never produces (ADR-0028's rule for reserved bytes).
                    if block.target(slot).is_none()
                        && (block.eligibility_q1_15[slot] != 0 || block.is_apical(slot))
                    {
                        return Err(ImageError::ReservedNotZero {
                            section: SECTION_SYNAPSE,
                            index: i as u32,
                        });
                    }
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
            for (i, record) in section_of(bytes, &delta)?.chunks_exact(16).enumerate() {
                let d = PlasticDelta::decode(record.try_into().unwrap_or(&[0; 16]));
                if d.block().is_some_and(|b| b as usize >= blocks)
                    || d.next_delta().is_some_and(|n| n as usize >= deltas)
                    || d.slot as usize >= SYNAPSES_PER_BLOCK
                {
                    return Err(ImageError::DanglingIndex {
                        block: i as u32,
                        slot: 5,
                    });
                }
                if d._pad != 0 {
                    return Err(ImageError::ReservedNotZero {
                        section: SECTION_PLASTIC_DELTA,
                        index: i as u32,
                    });
                }
                arena[i] = d;
            }
        }
        let mut wake = Vec::new();
        {
            let arena = exec.units_mut();
            for (i, record) in section_of(bytes, &neuron)?.chunks_exact(64).enumerate() {
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
        if let Some(section) = amendment {
            for (i, record) in section_of(bytes, &section)?.chunks_exact(64).enumerate() {
                let a = PolicyAmendment::decode(record.try_into().unwrap_or(&[0; 64]));
                // The id is the position plus one; a position the id width cannot name above
                // is a record the writer could not have written.
                let id = (i as u32).checked_add(1);
                if !a.is_well_formed() || Some(a.amendment_id) != id || !exec.load_amendment(a) {
                    return Err(ImageError::MalformedAmendment(i as u32));
                }
            }
        }
        {
            // One record, the engine's: the modulator's 16 bytes, the baseline at `[16..20)`
            // (within its bounds, as `Executor::new` would have demanded), 44 reserved bytes.
            if modulator.record_count() != 1 {
                return Err(ImageError::Directory(SECTION_MODULATOR));
            }
            let record = section_of(bytes, &modulator)?;
            if record[20..64].iter().any(|&b| b != 0) {
                return Err(ImageError::ReservedNotZero {
                    section: SECTION_MODULATOR,
                    index: 0,
                });
            }
            let baseline = i32::from_le_bytes(record[16..20].try_into().unwrap_or([0; 4]));
            if !exec.set_modulation_baseline(baseline) {
                return Err(ImageError::Config(ConfigError::ModulationOutOfRange));
            }
            exec.set_modulator(NeuromodulatorState::decode(
                record[0..16].try_into().unwrap_or(&[0; 16]),
            ));
        }
        {
            // One record, the engine's homeostasis state (ADR-0036): its step within the
            // configuration's bound, the record as the rules leave it, and its window at the
            // bin count the clock at the write implies.
            if homeostasis.record_count() != 1 {
                return Err(ImageError::Directory(SECTION_HOMEOSTASIS));
            }
            let record = section_of(bytes, &homeostasis)?;
            let pool = HomeostaticDrivePool::decode(record.try_into().unwrap_or(&[0; 64]));
            if pool.control_step_q0_16 > CONTROL_STEP_MAX_Q0_16 {
                return Err(ImageError::Config(ConfigError::ControlStepOutOfRange));
            }
            if pool.window_bins != Executor::<CAP>::window_bins_at(header.written_tick)
                || !exec.set_homeostasis(pool)
            {
                return Err(ImageError::MalformedHomeostasis);
            }
        }
        // The clock resumes where the image was written, so every stamp in it (a unit's last
        // spike, a block's, the intervals the plasticity rules and the sweep read from them)
        // keeps its meaning (ADR-0033).
        exec.resume_clock(header.written_tick);
        exec.wake_now(&wake);
        Ok(exec)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_block_bound_is_the_token_s_and_is_tested_at_its_edge() {
        assert_eq!(MAX_BLOCKS, MAX_TOKEN_BLOCK as u64 + 1);
        assert!(!too_many_blocks(0));
        assert!(
            !too_many_blocks(MAX_BLOCKS),
            "exactly as many as a token can name"
        );
        assert!(too_many_blocks(MAX_BLOCKS + 1), "one more is refused");
    }
}
