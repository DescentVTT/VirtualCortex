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
//! modulation state (ADR-0032: the modulator record and the baseline; since ADR-0086 the
//! inhibitory baseline beside them, set or unset; since ADR-0094 the signed gate, set or
//! unset; since ADR-0114 the class of short-term plasticity, set or unset), its homeostasis
//! state (ADR-0036, ADR-0037) and its hippocampal state (ADR-0038) are sections of their own,
//! always written, and the episodic ledger a section written when it is not empty, so that the
//! image defines the run (§8.3); since ADR-0052 so are the engine's affect state and induction
//! record, always, and its term arena and clause store when they hold anything. A unit's mark
//! for the class (ADR-0114) is a bit of its record's `flags`; since ADR-0123 the slow current's
//! constants sit in the modulation state beside the class, set or unset, and a unit's mark for
//! it is another bit of `flags`, its slow potential a field of its record; since ADR-0131 the
//! critic's constants sit there too, set or unset, and a unit's weight onto it is a field of its
//! record, zero in every unit while the critic is unset; since ADR-0134 the critic's window sits
//! beside them, zero while unset and, when set, the shortest delay of any synapse the image
//! carries.

use crate::executor::{AmendError, Config, ConfigError, Executor, InjectError};
use cortex_affect::InteroceptiveState;
use cortex_connectome::{
    CortexFileHeader, Crc64, HeaderError, SECTION_AFFECT, SECTION_AMENDMENT, SECTION_CLAUSE,
    SECTION_EPISODE, SECTION_HIPPOCAMPUS, SECTION_HOMEOSTASIS, SECTION_INDUCTION,
    SECTION_MODULATOR, SECTION_NEURON, SECTION_PLASTIC_DELTA, SECTION_SYNAPSE, SECTION_TERM,
    SectionEntry, crc64,
};
use cortex_core::{
    DendriticSuperNeuron, FLAG_FACILITATING, FLAG_SLOW, MAX_TOKEN_BLOCK, PlasticDelta,
    SYNAPSES_PER_BLOCK, SlowCurrent, StpClass, SynapseBlock, TICK_NS, WorkerWheel,
};
use cortex_executive::PolicyAmendment;
use cortex_hippocampus::{Episode, HippocampalAttractorState};
use cortex_homeostasis::{CONTROL_STEP_MAX_Q0_16, HomeostaticDrivePool, SLEEP_SHIFT_MAX};
use cortex_neuromod::{NeuromodulatorState, ValueCritic};
use cortex_reasoning::{INVENTED_BASE, INVENTED_LIMIT, InductionState, TermNode};
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
    /// The homeostasis record (section kind 43, ADR-0036, ADR-0037) is outside the bounds or
    /// the consistency the rules keep: a gain outside its bounds, a full window, a count above
    /// the cap, a sum beyond what the pairs allow or inconsistent with the others, a stage the
    /// constants do not name, a pressure above 1.0, a sleep stage at its budget, or a window
    /// whose bin count is not the one the clock at the write implies.
    MalformedHomeostasis,
    /// The hippocampal record (section kind 44, ADR-0038) is not well formed (its hand at or
    /// beyond its length, a reserved byte) or its length is not the episode section's count.
    MalformedHippocampus,
    /// An episode (section kind 45, ADR-0038) is one `Episode::tag` could not have produced
    /// (no unit, too many, a unit twice, a pad or reserved byte), names a unit outside the
    /// arena, or is bound to a symbol outside the invented band (ADR-0052).
    MalformedEpisode(u32),
    /// A term node (section kind 40, ADR-0052) is not well formed, is empty, names a child
    /// at or beyond its own index, or is a variable outside the binding table.
    MalformedTerm(u32),
    /// A clause index (section kind 49, ADR-0052) is at or beyond the arena's cursor, names
    /// a node that is not a clause, or appears twice.
    MalformedClause(u32),
    /// The induction record (section kind 48, ADR-0052) is not well formed, or its cursor,
    /// its length or its next variable do not describe the loaded arena and store.
    MalformedInduction,
    /// The affect record (section kind 47, ADR-0052) is not well formed or is not primed to
    /// the loaded store's description length.
    MalformedAffect,
    /// A unit is marked `FLAG_FACILITATING` while the image carries no class of short-term
    /// plasticity for it to step under (ADR-0114).
    MarkWithoutClass(u32),
    /// A unit is marked `FLAG_SLOW` while the image carries no slow current for it to integrate
    /// under (ADR-0123).
    MarkWithoutSlowCurrent(u32),
    /// A unit carries a value weight while the image carries no critic to read it (ADR-0131):
    /// nothing but the critic writes one.
    ValueWithoutCritic(u32),
    /// The critic's window (ADR-0134) is set to a length that is not the shortest delay of any
    /// synapse the image carries (`shortest`, none when it carries no synapse): the rule that
    /// reads the window from the image's anatomy does not resolve it.
    WindowNotShortestDelay { window: u16, shortest: Option<u16> },
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

/// The modulator section's inhibitory baseline (ADR-0086; format 15): a flag byte at
/// `[24]`, `INHIBITORY_BASELINE_SET` while the baseline is set and zero while it is unset,
/// and the baseline at `[28..32)`, an `i32` in Q16.16 that is zero while unset. The bytes
/// between and after are reserved and must be zero. A record a format-14 writer left zero
/// there reads as unset, which is the rule before ADR-0086 bit for bit.
const INHIBITORY_FLAG: usize = 24;
const INHIBITORY_VALUE: core::ops::Range<usize> = 28..32;
const INHIBITORY_BASELINE_SET: u8 = 1;

/// The modulator section's signed gate (ADR-0094; format 16): a flag byte at `[25]`,
/// `SIGNED_GATE_SET` while the gate is set and zero while it is unset. A record a format-15
/// writer left zero there reads as unset, which is the rule before ADR-0094 bit for bit.
const SIGNED_GATE_FLAG: usize = 25;
const SIGNED_GATE_SET: u8 = 1;

/// The modulator section's class of short-term plasticity (ADR-0114; format 17): a flag byte at
/// `[32]`, `STP_CLASS_SET` while a class is set and zero while none is, and the class's $U$,
/// $\tau_f$ shift and $\tau_d$ shift at `[33]`, `[34]` and `[35]`, zero while unset.
/// `[26..28)` stay reserved and must be zero. A record a format-16 writer left zero there reads
/// as unset, which is the rule before ADR-0114 bit for bit.
const STP_CLASS_FLAG: usize = 32;
const STP_CLASS_U: usize = 33;
const STP_CLASS_TAU_F: usize = 34;
const STP_CLASS_TAU_D: usize = 35;
const STP_CLASS_SET: u8 = 1;

/// The modulator section's slow current (ADR-0123; format 18): a flag byte at `[36]`,
/// `SLOW_CURRENT_SET` while the constants are set and zero while they are not; the leak shift at
/// `[37]` and the input shift at `[38]`; `[39]` reserved; $V_{lo}$ at `[40..44)` and
/// $V_{hi}$ at `[44..48)`, `i32` in Q16.16; every byte zero while unset. `[48..64)` stay
/// reserved and must be zero. A record a format-17 writer left zero there reads as unset, which
/// is the rule before ADR-0123 bit for bit.
const SLOW_CURRENT_FLAG: usize = 36;
const SLOW_CURRENT_LEAK: usize = 37;
const SLOW_CURRENT_INPUT: usize = 38;
const SLOW_CURRENT_V_LO: core::ops::Range<usize> = 40..44;
const SLOW_CURRENT_V_HI: core::ops::Range<usize> = 44..48;
const SLOW_CURRENT_SET: u8 = 1;
/// The slow current's bytes after its flag, `[37..48)`, the reserved `[39]` among them.
const SLOW_CURRENT_BYTES: core::ops::Range<usize> = 37..48;

/// The modulator section's critic (ADR-0131; format 19): a flag byte at `[48]`, `CRITIC_SET`
/// while the constants are set and zero while they are not; the step's shift at `[49]` and the
/// weight's scale at `[50]`; every byte zero while unset. `[51..64)` stay reserved and must be
/// zero. A record a format-18 writer left zero there reads as unset, which is the rule before
/// ADR-0131 bit for bit.
const CRITIC_FLAG: usize = 48;
const CRITIC_SHIFT: usize = 49;
const CRITIC_SCALE: usize = 50;
const CRITIC_SET: u8 = 1;

/// The modulator section's critic's window (ADR-0134; format 20): its length in ticks at
/// `[52..54)`, a `u16`, zero while unset. `[51]` and `[54..64)` stay reserved and must be zero. A
/// record a format-19 writer left zero there reads as unset, which is the critic of ADR-0131 bit
/// for bit.
const CRITIC_WINDOW: core::ops::Range<usize> = 52..54;

/// The modulator record's reserved bytes: between the signed gate and the target period, the
/// slow current's one, the one between the critic and its window, and the tail after the window.
const MODULATOR_RESERVED: [core::ops::Range<usize>; 4] = [26..28, 39..40, 51..52, 54..64];

/// The critic's window a modulator record carries (ADR-0134), in ticks: zero while unset.
fn critic_window_of(record: &[u8]) -> u16 {
    u16::from_le_bytes(record[CRITIC_WINDOW].try_into().unwrap_or([0; 2]))
}

/// The shortest delay of any synapse `blocks` carry (ADR-0133, ADR-0134): the least
/// `delays_ticks` over every slot that holds a synapse, whichever unit's chain names its block;
/// none when no slot does. The rule the critic's window is read by: within that many ticks after
/// a reward no spike can have caused another through a synapse, since a delay $d$ scheduled at
/// tick $t$ is integrated at $t + d$ and a delay of zero at $t + 1$. The loader refuses a window
/// that is not this length, so a zero here, a synapse through the mailbox, resolves none.
pub fn shortest_delay(blocks: &[SynapseBlock]) -> Option<u16> {
    blocks
        .iter()
        .flat_map(|block| {
            (0..SYNAPSES_PER_BLOCK)
                .filter(move |&slot| block.target(slot).is_some())
                .map(move |slot| block.delays_ticks[slot])
        })
        .min()
}

/// The critic a modulator record carries (ADR-0131): none while its flag and its two bytes are
/// zero; the constants while its flag is `CRITIC_SET`, which `Executor::new` refuses as it
/// refuses the configuration's when the rule does not resolve them; any other flag, or a byte
/// beside a zero flag, is one the writer never produces.
fn critic_of(record: &[u8]) -> Result<Option<ValueCritic>, ImageError> {
    let critic = ValueCritic {
        shift: record[CRITIC_SHIFT],
        scale: record[CRITIC_SCALE],
    };
    match record[CRITIC_FLAG] {
        0 if record[CRITIC_SHIFT] == 0 && record[CRITIC_SCALE] == 0 => Ok(None),
        CRITIC_SET => Ok(Some(critic)),
        _ => Err(ImageError::ReservedNotZero {
            section: SECTION_MODULATOR,
            index: 0,
        }),
    }
}

/// The slow current a modulator record carries (ADR-0123): none while its flag and its bytes
/// are zero; the constants while its flag is `SLOW_CURRENT_SET` and its reserved byte zero,
/// which `Executor::new` refuses as it refuses the configuration's when the rule does not
/// resolve them; any other flag, or a byte beside a zero flag, is one the writer never produces.
fn slow_current_of(record: &[u8]) -> Result<Option<SlowCurrent>, ImageError> {
    let word =
        |at: core::ops::Range<usize>| i32::from_le_bytes(record[at].try_into().unwrap_or([0; 4]));
    let current = SlowCurrent {
        leak_shift: record[SLOW_CURRENT_LEAK],
        input_shift: record[SLOW_CURRENT_INPUT],
        v_lo_q16: word(SLOW_CURRENT_V_LO),
        v_hi_q16: word(SLOW_CURRENT_V_HI),
    };
    match record[SLOW_CURRENT_FLAG] {
        0 if record[SLOW_CURRENT_BYTES].iter().all(|&b| b == 0) => Ok(None),
        SLOW_CURRENT_SET => Ok(Some(current)),
        _ => Err(ImageError::ReservedNotZero {
            section: SECTION_MODULATOR,
            index: 0,
        }),
    }
}

/// The class a modulator record carries (ADR-0114): none while its flag and its three bytes
/// are zero; the class while its flag is `STP_CLASS_SET`, which `Executor::new` refuses as it
/// refuses the configuration's when the rule does not resolve it, the one place the class's
/// range is held; any other flag, or a byte beside a zero flag, is one the writer never
/// produces.
fn stp_class_of(record: &[u8]) -> Result<Option<StpClass>, ImageError> {
    let class = StpClass {
        u: record[STP_CLASS_U],
        tau_f_shift: record[STP_CLASS_TAU_F],
        tau_d_shift: record[STP_CLASS_TAU_D],
    };
    match record[STP_CLASS_FLAG] {
        0 if record[STP_CLASS_U..=STP_CLASS_TAU_D] == [0; 3] => Ok(None),
        STP_CLASS_SET => Ok(Some(class)),
        _ => Err(ImageError::ReservedNotZero {
            section: SECTION_MODULATOR,
            index: 0,
        }),
    }
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

// The positioned read and write are the platform's one call each, and nothing else is
// gated: the loops around them, `read_at` and `write_at`, are one form on every platform, so
// their tests run on every platform (ADR-0062). The Unix forms are `pread` and `pwrite`, the
// Windows forms `seek_read` and `seek_write`, under distinct names so that an exclusion in
// `.cargo/mutants.toml` can name the forms no CI runner compiles and nothing else.

/// One positioned read: `Ok(n)` for the bytes moved, fewer than asked when the platform
/// stops short, zero at the end of the file.
#[cfg(unix)]
fn pread(file: &File, offset: u64, buf: &mut [u8]) -> io::Result<usize> {
    use std::os::unix::fs::FileExt;
    file.read_at(buf, offset)
}

/// One positioned write: `Ok(n)` for the bytes moved, fewer than asked when the platform
/// stops short.
#[cfg(unix)]
fn pwrite(file: &File, offset: u64, buf: &[u8]) -> io::Result<usize> {
    use std::os::unix::fs::FileExt;
    file.write_at(buf, offset)
}

#[cfg(windows)]
fn seek_read(file: &File, offset: u64, buf: &mut [u8]) -> io::Result<usize> {
    use std::os::windows::fs::FileExt;
    file.seek_read(buf, offset)
}

#[cfg(windows)]
fn seek_write(file: &File, offset: u64, buf: &[u8]) -> io::Result<usize> {
    use std::os::windows::fs::FileExt;
    file.seek_write(buf, offset)
}

#[cfg(unix)]
use self::{pread as read_some_at, pwrite as write_some_at};
#[cfg(windows)]
use self::{seek_read as read_some_at, seek_write as write_some_at};

/// Fills `buf` from `offset`: the platform's read repeated, each call taking what the last
/// one moved off the buffer, which is the countdown the loop ends by; a read of nothing
/// before the buffer is full is the file ending early (`UnexpectedEof`). An interrupted call
/// is returned like any other error, not retried: the one branch no test can reach is not
/// written.
fn read_at(file: &File, mut offset: u64, mut buf: &mut [u8]) -> io::Result<()> {
    while !buf.is_empty() {
        let n = read_some_at(file, offset, buf)?;
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

/// Writes all of `buf` at `offset`, the same way: a write of nothing is `WriteZero`.
fn write_at(file: &File, mut offset: u64, mut buf: &[u8]) -> io::Result<()> {
    while !buf.is_empty() {
        let n = write_some_at(file, offset, buf)?;
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
        // bytes, the baseline at `[16..20)`, the inhibitory rule's target period at `[20..24)`
        // (ADR-0053), the inhibitory baseline's flag at `[24]` and its value at `[28..32)`
        // (ADR-0086), the signed gate's flag at `[25]` (ADR-0094), the class of short-term
        // plasticity's flag at `[32]` and its three bytes at `[33..36)` (ADR-0114; format 17),
        // the slow current's flag at `[36]`, its shifts at `[37]` and `[38]` and its voltages
        // at `[40..48)` (ADR-0123; format 18), the critic's flag at `[48]` and its shift and
        // scale at `[49]` and `[50]` (ADR-0131; format 19), the critic's window at `[52..54)`
        // (ADR-0134; format 20) and 14 reserved bytes. The baselines, the period, the gate, the
        // class, the slow current, the critic and its window change what a run does, so they are
        // in the image, not in a configuration (§8.3).
        let mut modulator_bytes = vec![0u8; 64];
        modulator_bytes[0..16].copy_from_slice(&exec.modulator().encode());
        modulator_bytes[16..20].copy_from_slice(&exec.modulation_baseline_q16().to_le_bytes());
        modulator_bytes[20..24].copy_from_slice(&exec.istdp_target_period_ticks().to_le_bytes());
        if let Some(inhibitory) = exec.inhibitory_baseline_q16() {
            modulator_bytes[INHIBITORY_FLAG] = INHIBITORY_BASELINE_SET;
            modulator_bytes[INHIBITORY_VALUE].copy_from_slice(&inhibitory.to_le_bytes());
        }
        if exec.signed_gate() {
            modulator_bytes[SIGNED_GATE_FLAG] = SIGNED_GATE_SET;
        }
        if let Some(class) = exec.stp_class() {
            modulator_bytes[STP_CLASS_FLAG] = STP_CLASS_SET;
            modulator_bytes[STP_CLASS_U] = class.u;
            modulator_bytes[STP_CLASS_TAU_F] = class.tau_f_shift;
            modulator_bytes[STP_CLASS_TAU_D] = class.tau_d_shift;
        }
        if let Some(current) = exec.slow_current() {
            modulator_bytes[SLOW_CURRENT_FLAG] = SLOW_CURRENT_SET;
            modulator_bytes[SLOW_CURRENT_LEAK] = current.leak_shift;
            modulator_bytes[SLOW_CURRENT_INPUT] = current.input_shift;
            modulator_bytes[SLOW_CURRENT_V_LO].copy_from_slice(&current.v_lo_q16.to_le_bytes());
            modulator_bytes[SLOW_CURRENT_V_HI].copy_from_slice(&current.v_hi_q16.to_le_bytes());
        }
        if let Some(critic) = exec.critic() {
            modulator_bytes[CRITIC_FLAG] = CRITIC_SET;
            modulator_bytes[CRITIC_SHIFT] = critic.shift;
            modulator_bytes[CRITIC_SCALE] = critic.scale;
        }
        modulator_bytes[CRITIC_WINDOW].copy_from_slice(&exec.critic_window_ticks().to_le_bytes());
        sections.push((SECTION_MODULATOR, 64, modulator_bytes));
        // The engine's homeostasis state, always: the gain and the estimator's window change
        // what a run does, so they are in the image (ADR-0036).
        sections.push((
            SECTION_HOMEOSTASIS,
            64,
            exec.homeostasis().encode().to_vec(),
        ));
        // The engine's hippocampal state, always, and the ledger when it is not empty: what
        // the ripples replay changes what a run does (ADR-0038).
        sections.push((
            SECTION_HIPPOCAMPUS,
            64,
            exec.hippocampus().encode().to_vec(),
        ));
        let episodes = exec.episodes();
        if !episodes.is_empty() {
            let mut episode_bytes = Vec::with_capacity(episodes.len().saturating_mul(64));
            for e in episodes {
                episode_bytes.extend_from_slice(&e.encode());
            }
            sections.push((SECTION_EPISODE, 64, episode_bytes));
        }
        // The term arena and the clause store when they hold anything, the affect state and
        // the induction record always (ADR-0052): what the discovery loop reads and writes
        // changes what a run does, so all of it is in the image.
        let terms = exec.terms();
        if !terms.is_empty() {
            let mut term_bytes = Vec::with_capacity(terms.len().saturating_mul(64));
            for t in terms {
                term_bytes.extend_from_slice(&t.encode());
            }
            sections.push((SECTION_TERM, 64, term_bytes));
        }
        let clauses = exec.clauses();
        if !clauses.is_empty() {
            let mut clause_bytes = Vec::with_capacity(clauses.len().saturating_mul(4));
            for c in clauses {
                clause_bytes.extend_from_slice(&c.to_le_bytes());
            }
            sections.push((SECTION_CLAUSE, 4, clause_bytes));
        }
        sections.push((SECTION_AFFECT, 64, exec.affect().encode().to_vec()));
        sections.push((SECTION_INDUCTION, 64, exec.induction().encode().to_vec()));
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
                | SECTION_HOMEOSTASIS | SECTION_HIPPOCAMPUS | SECTION_EPISODE | SECTION_TERM
                | SECTION_AFFECT | SECTION_INDUCTION => 64,
                SECTION_PLASTIC_DELTA => 16,
                SECTION_CLAUSE => 4,
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
        let hippocampus =
            find(SECTION_HIPPOCAMPUS).ok_or(ImageError::MissingSection(SECTION_HIPPOCAMPUS))?;
        let episode = find(SECTION_EPISODE);
        let term = find(SECTION_TERM);
        let clause = find(SECTION_CLAUSE);
        let affect = find(SECTION_AFFECT).ok_or(ImageError::MissingSection(SECTION_AFFECT))?;
        let induction =
            find(SECTION_INDUCTION).ok_or(ImageError::MissingSection(SECTION_INDUCTION))?;
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
        let episodes = episode.map_or(0, |e| e.record_count() as usize);
        let terms = term.map_or(0, |t| t.record_count() as usize);
        let clauses = clause.map_or(0, |c| c.record_count() as usize);
        // The modulator's one record (ADR-0032), read here for the class of short-term
        // plasticity (ADR-0114), the slow current (ADR-0123), the critic (ADR-0131) and its
        // window (ADR-0134): each worker holds the first two from `Executor::new`, which sizes
        // the critic's counts and refuses a window without a critic, and a unit marked for
        // either of the first two, or carrying a weight for the third, is refused below while
        // there is none. The image's, set or unset, outrank the configuration's (§8.3).
        if modulator.record_count() != 1 {
            return Err(ImageError::Directory(SECTION_MODULATOR));
        }
        let stp_class = stp_class_of(section_of(bytes, &modulator)?)?;
        let slow_current = slow_current_of(section_of(bytes, &modulator)?)?;
        let critic = critic_of(section_of(bytes, &modulator)?)?;
        let critic_window_ticks = critic_window_of(section_of(bytes, &modulator)?);
        let mut exec = Executor::<CAP>::new(Config {
            units,
            blocks,
            deltas,
            amendments: amendments.saturating_add(config.amendments),
            episodes: episodes.saturating_add(config.episodes),
            terms: terms.saturating_add(config.terms),
            clauses: clauses.saturating_add(config.clauses),
            stp_class,
            slow_current,
            critic,
            critic_window_ticks,
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
        // The critic's window (ADR-0134), when set, is the shortest delay of any synapse the
        // image carries, read from the arena just loaded: a window of another length, or one on
        // an image with no synapse, is refused.
        if critic_window_ticks != 0 {
            let shortest = shortest_delay(exec.blocks());
            if shortest != Some(critic_window_ticks) {
                return Err(ImageError::WindowNotShortestDelay {
                    window: critic_window_ticks,
                    shortest,
                });
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
                if unit.flags & FLAG_FACILITATING != 0 && stp_class.is_none() {
                    return Err(ImageError::MarkWithoutClass(i as u32));
                }
                if unit.flags & FLAG_SLOW != 0 && slow_current.is_none() {
                    return Err(ImageError::MarkWithoutSlowCurrent(i as u32));
                }
                if unit.value_weight != 0 && critic.is_none() {
                    return Err(ImageError::ValueWithoutCritic(i as u32));
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
            // One record, the engine's: the modulator's 16 bytes, the baseline at `[16..20)`,
            // the inhibitory rule's target period at `[20..24)` (each within its bounds, as
            // `Executor::new` would have demanded), the inhibitory baseline's flag at `[24]`
            // and its value at `[28..32)` (ADR-0086), the signed gate's flag at `[25]`
            // (ADR-0094), the class of short-term plasticity at `[32..36)`, the slow current at
            // `[36..48)`, the critic at `[48..51)` and its window at `[52..54)`, read above
            // (ADR-0114, ADR-0123, ADR-0131, ADR-0134), 14 reserved bytes; its count was held to
            // one above.
            let record = section_of(bytes, &modulator)?;
            let reserved_not_zero = || ImageError::ReservedNotZero {
                section: SECTION_MODULATOR,
                index: 0,
            };
            if MODULATOR_RESERVED
                .iter()
                .any(|r| record[r.clone()].iter().any(|&b| b != 0))
            {
                return Err(reserved_not_zero());
            }
            let baseline = i32::from_le_bytes(record[16..20].try_into().unwrap_or([0; 4]));
            if !exec.set_modulation_baseline(baseline) {
                return Err(ImageError::Config(ConfigError::ModulationOutOfRange));
            }
            let period = u32::from_le_bytes(record[20..24].try_into().unwrap_or([0; 4]));
            if !exec.set_istdp_target_period(period) {
                return Err(ImageError::Config(ConfigError::IstdpPeriodOutOfRange));
            }
            // The inhibitory baseline (ADR-0086): unset while its flag is zero, where a value
            // that is not zero is a byte the writer never produces; set while its flag is
            // `INHIBITORY_BASELINE_SET`, where the value is refused outside [0, 1] as the
            // configuration's is; any other flag is a byte the writer never produces. The
            // image's, set or unset, outranks the configuration's (§8.3).
            let value = i32::from_le_bytes(record[INHIBITORY_VALUE].try_into().unwrap_or([0; 4]));
            let inhibitory = match record[INHIBITORY_FLAG] {
                0 if value == 0 => None,
                INHIBITORY_BASELINE_SET => Some(value),
                _ => return Err(reserved_not_zero()),
            };
            if !exec.set_inhibitory_baseline(inhibitory) {
                return Err(ImageError::Config(
                    ConfigError::InhibitoryBaselineOutOfRange,
                ));
            }
            // The signed gate (ADR-0094): unset while its flag is zero, set while it is
            // `SIGNED_GATE_SET`; any other flag is a byte the writer never produces. The
            // image's, set or unset, outranks the configuration's (§8.3).
            let signed_gate = match record[SIGNED_GATE_FLAG] {
                0 => false,
                SIGNED_GATE_SET => true,
                _ => return Err(reserved_not_zero()),
            };
            exec.set_signed_gate(signed_gate);
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
            if pool.sleep_shift > SLEEP_SHIFT_MAX {
                return Err(ImageError::Config(ConfigError::SleepShiftOutOfRange));
            }
            if pool.window_bins != Executor::<CAP>::window_bins_at(header.written_tick)
                || !exec.set_homeostasis(pool)
            {
                return Err(ImageError::MalformedHomeostasis);
            }
        }
        {
            // The ledger (ADR-0038): every episode well formed and naming units of the arena,
            // appended in the image's order; then the hippocampal record, one, well formed,
            // its length the section's count (or zero without a section: an image whose
            // record says the ledger is not empty needs the section).
            if hippocampus.record_count() != 1 {
                return Err(ImageError::Directory(SECTION_HIPPOCAMPUS));
            }
            if let Some(section) = episode {
                for (i, record) in section_of(bytes, &section)?.chunks_exact(64).enumerate() {
                    let e = Episode::decode(record.try_into().unwrap_or(&[0; 64]));
                    // A bound episode names an invented predicate (ADR-0052): the band is
                    // the runtime's to know, not the ledger's.
                    let symbol_in_band =
                        e.symbol == 0 || (INVENTED_BASE..INVENTED_LIMIT).contains(&e.symbol);
                    if !e.is_well_formed() || !symbol_in_band || !exec.load_episode(e) {
                        return Err(ImageError::MalformedEpisode(i as u32));
                    }
                }
            }
            let record = section_of(bytes, &hippocampus)?;
            let state = HippocampalAttractorState::decode(record.try_into().unwrap_or(&[0; 64]));
            if state.episodes > 0 && episode.is_none() {
                return Err(ImageError::MissingSection(SECTION_EPISODE));
            }
            if !exec.set_hippocampus(state) {
                return Err(ImageError::MalformedHippocampus);
            }
        }
        {
            // The term arena, the clause store, the induction record and the affect state
            // (ADR-0052): every node well formed and bottom-up (so no loaded arena is
            // cyclic), every store index a clause below the cursor and named once, the
            // record describing what was loaded, the affect state primed to the store's
            // length; the two records one each, required.
            if let Some(section) = term {
                for (i, record) in section_of(bytes, &section)?.chunks_exact(64).enumerate() {
                    let node = TermNode::decode(record.try_into().unwrap_or(&[0; 64]));
                    if !exec.load_term(node) {
                        return Err(ImageError::MalformedTerm(i as u32));
                    }
                }
            }
            if let Some(section) = clause {
                for (i, record) in section_of(bytes, &section)?.chunks_exact(4).enumerate() {
                    let index = u32::from_le_bytes(record.try_into().unwrap_or([0; 4]));
                    if !exec.load_clause(index) {
                        return Err(ImageError::MalformedClause(i as u32));
                    }
                }
            }
            if induction.record_count() != 1 {
                return Err(ImageError::Directory(SECTION_INDUCTION));
            }
            let record = section_of(bytes, &induction)?;
            let state = InductionState::decode(record.try_into().unwrap_or(&[0; 64]));
            if !exec.set_induction(state) {
                return Err(ImageError::MalformedInduction);
            }
            if affect.record_count() != 1 {
                return Err(ImageError::Directory(SECTION_AFFECT));
            }
            let record = section_of(bytes, &affect)?;
            let state = InteroceptiveState::decode(record.try_into().unwrap_or(&[0; 64]));
            if !exec.set_affect(state) {
                return Err(ImageError::MalformedAffect);
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
