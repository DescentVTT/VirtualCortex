//! Connectome: the `.cortex` image header, the section directory record and the CRC-64 that
//! seals them (whitepaper §5.2.2, §8.7; ADR-0007, ADR-0024). The writer and the loader are the
//! runtime's (`runtime/cortex-runtime`); the laminar priors are Specified.

#![no_std]

/// The reflected form of the ECMA-182 polynomial `0x42F0E1EBA9EA3693`: the CRC-64/XZ
/// parameters (reflected input and output, initial and final value all ones).
pub const CRC64_POLY_REFLECTED: u64 = 0xC96C_5795_D787_0F42;

/// A CRC-64/XZ in progress: `update` over any number of chunks, then `finish`. Bitwise, so it
/// needs no table (ADR-0024); one shift-and-conditional-xor per bit.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Crc64 {
    state: u64,
}

impl Crc64 {
    /// The initial state.
    pub const fn new() -> Self {
        Self { state: u64::MAX }
    }

    /// Feeds `bytes`.
    pub fn update(&mut self, bytes: &[u8]) {
        let mut crc = self.state;
        for &byte in bytes {
            crc ^= byte as u64;
            let mut bit = 0;
            while bit < 8 {
                crc = if crc & 1 == 1 {
                    (crc >> 1) ^ CRC64_POLY_REFLECTED
                } else {
                    crc >> 1
                };
                bit += 1;
            }
        }
        self.state = crc;
    }

    /// The checksum of everything fed so far.
    pub const fn finish(self) -> u64 {
        self.state ^ u64::MAX
    }
}

impl Default for Crc64 {
    fn default() -> Self {
        Self::new()
    }
}

/// The CRC-64/XZ of `bytes` in one call. Check value: `crc64(b"123456789")` is
/// `0x995D_C9BB_DF19_39FA`.
pub fn crc64(bytes: &[u8]) -> u64 {
    let mut crc = Crc64::new();
    crc.update(bytes);
    crc.finish()
}

/// Section kinds are the row numbers of the whitepaper's Appendix A capacity table, so that a
/// section names the arena it holds without a second numbering.
pub const SECTION_MACRO_COLUMN: u32 = 1;
/// `DendriticSuperNeuron` arena.
pub const SECTION_NEURON: u32 = 2;
/// `SynapseBlock` arena.
pub const SECTION_SYNAPSE: u32 = 3;
/// `PlasticDelta` arena (Tier 2).
pub const SECTION_PLASTIC_DELTA: u32 = 37;
/// The laminar priors (Specified).
pub const SECTION_LAMINAR: u32 = 38;
/// The routing table (Specified).
pub const SECTION_ROUTING: u32 = 39;
/// `TermNode` arena of `cortex-reasoning` (ADR-0025; the loader's support is Specified).
pub const SECTION_TERM: u32 = 40;

/// Why a header is refused.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HeaderError {
    /// The first eight bytes are not `VCORTEX1`.
    BadMagic,
    /// A version other than [`CortexFileHeader::FORMAT_VERSION`]; the image MUST fail closed.
    ForeignVersion(u32),
    /// The stored checksum does not match the header's bytes.
    BadCrc,
    /// The reserved bytes are not zero.
    BadPadding,
}

/// The first 64 bytes of every `.cortex` image (whitepaper §5.2.2).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(C, align(64))]
pub struct CortexFileHeader {
    pub magic: [u8; 8],      // [0..8] "VCORTEX1"
    pub version: u32,        // [8..12] FORMAT_VERSION
    pub reserved_flags: u32, // [12..16] Feature flags; MUST be zero
    pub num_columns: u64,    // [16..24] Cortical hyper-column count
    pub num_neurons: u64,    // [24..32] DendriticSuperNeuron record count
    pub num_synapses: u64,   // [32..40] SynapseBlock record count (ADR-0024: blocks, not slots)
    pub layers_offset: u64,  // [40..48] Byte offset of the laminar section; 0 for none
    pub crc64: u64,          // [48..56] CRC-64/XZ of the 64 bytes with this field zero
    pub section_count: u32,  // [56..60] Directory entries that follow the header (ADR-0024)
    pub _padding: u32,       // [60..64] Reserved; MUST be zero
}

impl CortexFileHeader {
    /// ASCII `VCORTEX1`, the first eight bytes of every `.cortex` image.
    pub const MAGIC: [u8; 8] = *b"VCORTEX1";

    /// Current image format version. Bumped on any change to any record in the workspace,
    /// including reserved bytes and field semantics (whitepaper rule L-6, ADR-0007).
    ///
    /// - 1: the layout documented by whitepaper 3.0.0 (synaptic weights labelled Q16.16 in a
    ///   16-bit field; `intention_vector_ptr`).
    /// - 2: synaptic weights are Q1.15 (ADR-0012); `intention_vector_idx`. Byte layout
    ///   unchanged; the meaning of the weight bytes changed.
    /// - 3: `CerebellarMicrozone`'s reserved bytes became `pred_ring` and `delay_ctl`
    ///   (brief 004); a version-2 image has them zero, which reads as an empty delay line.
    /// - 4: `DendriticSuperNeuron::mailbox_tag` became `mailbox_reserved` and the mailbox head
    ///   encodes `node index + 1`, zero when empty (ADR-0017). A version-3 image at rest has
    ///   both zero, which reads correctly; the meaning of the head changed, hence the bump.
    /// - 5: reserved bytes became fields in six records (ADR-0020, ADR-0021): `InteroceptiveState`
    ///   (free energy, valence, existential stake), `GlobalWorkspaceSlot` (attention schema,
    ///   criticality distance), `MentalCanvasFrame` (reflection, self-model, wandering),
    ///   `SymbolicHypervectorHeader` (blend source, domain mask, depth), `LinguisticFrameSlot`
    ///   (parent, child, metaphor), `SocialPerspectiveNode` (repairs, turn state, common
    ///   ground). A version-4 image has them zero, which every rule reads as "not yet".
    /// - 6: `SynapseBlock` indices are `index + 1` (zero: an empty slot, the end of a chain, a
    ///   unit without fan-out) and its reserved bytes became `last_release_q16` and
    ///   `apical_mask` (ADR-0022); `DendriticSuperNeuron::synapse_slab_idx` is `index + 1` too.
    ///   A version-5 arena's zero indices now read as empty rather than as block 0, and every
    ///   non-zero index moved by one, so a version-5 image MUST NOT be read as version 6.
    /// - 7: the header gained `section_count` and its CRC covers all 64 bytes;
    ///   `DendriticSuperNeuron::plastic_delta_head` is 32 bits at `[60..64)` as `index + 1`
    ///   (`[52..54)` reserved), resolving finding F-20; `PlasticDelta` is a new 16-byte record
    ///   and `num_synapses` counts blocks (ADR-0024).
    /// - 8: reserved bytes became fields in five records (ADR-0026, ADR-0027):
    ///   `SocialPerspectiveNode` (the agent's expectation of the self, insincerity),
    ///   `SemanticOntologyNode` (anomaly, paradigm epoch, representation flags, count),
    ///   `SymbolicHypervectorHeader` (the padding byte became the rebase count),
    ///   `InteroceptiveState` (benign incongruity, mirth), `LinguisticFrameSlot` (the intended
    ///   speech act); `VocalFrame` is a new embodiment frame outside the image. A version-7
    ///   image has them zero, which every rule reads as "not yet".
    /// - 9: `EthicalEvaluationGate::veto_decision_flag` is `DECISION_*` (0 not yet evaluated,
    ///   1 vetoed, 2 permitted; ADR-0028). A version-8 image's zero reads as not yet evaluated,
    ///   which fails closed; nothing else moved.
    pub const FORMAT_VERSION: u32 = 9;

    /// A header for an image of these counts, sealed.
    pub fn new(num_columns: u64, num_neurons: u64, num_synapses: u64, section_count: u32) -> Self {
        let mut header = Self {
            magic: Self::MAGIC,
            version: Self::FORMAT_VERSION,
            reserved_flags: 0,
            num_columns,
            num_neurons,
            num_synapses,
            layers_offset: 0,
            crc64: 0,
            section_count,
            _padding: 0,
        };
        header.crc64 = header.checksum();
        header
    }

    /// The header's bytes, little-endian.
    pub fn encode(&self) -> [u8; 64] {
        let mut out = [0u8; 64];
        out[0..8].copy_from_slice(&self.magic);
        out[8..12].copy_from_slice(&self.version.to_le_bytes());
        out[12..16].copy_from_slice(&self.reserved_flags.to_le_bytes());
        out[16..24].copy_from_slice(&self.num_columns.to_le_bytes());
        out[24..32].copy_from_slice(&self.num_neurons.to_le_bytes());
        out[32..40].copy_from_slice(&self.num_synapses.to_le_bytes());
        out[40..48].copy_from_slice(&self.layers_offset.to_le_bytes());
        out[48..56].copy_from_slice(&self.crc64.to_le_bytes());
        out[56..60].copy_from_slice(&self.section_count.to_le_bytes());
        out[60..64].copy_from_slice(&self._padding.to_le_bytes());
        out
    }

    /// A header from its bytes; not validated.
    pub fn decode(bytes: &[u8; 64]) -> Self {
        Self {
            magic: bytes[0..8].try_into().unwrap_or([0; 8]),
            version: u32::from_le_bytes(bytes[8..12].try_into().unwrap_or([0; 4])),
            reserved_flags: u32::from_le_bytes(bytes[12..16].try_into().unwrap_or([0; 4])),
            num_columns: u64::from_le_bytes(bytes[16..24].try_into().unwrap_or([0; 8])),
            num_neurons: u64::from_le_bytes(bytes[24..32].try_into().unwrap_or([0; 8])),
            num_synapses: u64::from_le_bytes(bytes[32..40].try_into().unwrap_or([0; 8])),
            layers_offset: u64::from_le_bytes(bytes[40..48].try_into().unwrap_or([0; 8])),
            crc64: u64::from_le_bytes(bytes[48..56].try_into().unwrap_or([0; 8])),
            section_count: u32::from_le_bytes(bytes[56..60].try_into().unwrap_or([0; 4])),
            _padding: u32::from_le_bytes(bytes[60..64].try_into().unwrap_or([0; 4])),
        }
    }

    /// The CRC-64/XZ of the header's 64 bytes with the `crc64` field read as zero.
    pub fn checksum(&self) -> u64 {
        let mut bytes = self.encode();
        bytes[48..56].copy_from_slice(&[0; 8]);
        crc64(&bytes)
    }

    /// Magic, version, checksum and padding, in that order; the first failure is reported. A
    /// header that fails MUST NOT be read further (§8.7).
    pub fn validate(&self) -> Result<(), HeaderError> {
        if self.magic != Self::MAGIC {
            return Err(HeaderError::BadMagic);
        }
        if self.version != Self::FORMAT_VERSION {
            return Err(HeaderError::ForeignVersion(self.version));
        }
        if self.crc64 != self.checksum() {
            return Err(HeaderError::BadCrc);
        }
        if self.reserved_flags != 0 || self._padding != 0 {
            return Err(HeaderError::BadPadding);
        }
        Ok(())
    }
}

/// One entry of the section directory that follows the header: where an arena's bytes are
/// and what seals them (whitepaper §8.7; ADR-0024). 64 bytes.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(C, align(64))]
pub struct SectionEntry {
    pub kind: u32,           // [0..4] SECTION_* (an Appendix A row number)
    pub record_size: u32,    // [4..8] Bytes per record: 64 for the arenas, 16 for deltas
    pub offset: u64,         // [8..16] Byte offset of the section in the image; a multiple of 64
    pub length: u64,         // [16..24] Bytes of records; the section is padded to 64 after it
    pub crc64: u64,          // [24..32] CRC-64/XZ of the `length` bytes at `offset`
    pub _reserved: [u8; 32], // [32..64] Reserved; MUST be zero
}

impl SectionEntry {
    /// An entry.
    pub const fn new(kind: u32, record_size: u32, offset: u64, length: u64, crc64: u64) -> Self {
        Self {
            kind,
            record_size,
            offset,
            length,
            crc64,
            _reserved: [0; 32],
        }
    }

    /// Records in the section; zero when `record_size` is zero or does not divide `length`.
    pub const fn record_count(&self) -> u64 {
        if self.record_size == 0 || self.length % self.record_size as u64 != 0 {
            0
        } else {
            self.length / self.record_size as u64
        }
    }

    /// True when the offset is 64-byte aligned, the record size divides the length and the
    /// reserved bytes are zero.
    pub fn is_well_formed(&self) -> bool {
        self.offset % 64 == 0
            && self.record_size != 0
            && self.length % self.record_size as u64 == 0
            && self._reserved == [0; 32]
    }

    /// The entry's bytes, little-endian.
    pub fn encode(&self) -> [u8; 64] {
        let mut out = [0u8; 64];
        out[0..4].copy_from_slice(&self.kind.to_le_bytes());
        out[4..8].copy_from_slice(&self.record_size.to_le_bytes());
        out[8..16].copy_from_slice(&self.offset.to_le_bytes());
        out[16..24].copy_from_slice(&self.length.to_le_bytes());
        out[24..32].copy_from_slice(&self.crc64.to_le_bytes());
        out[32..64].copy_from_slice(&self._reserved);
        out
    }

    /// An entry from its bytes; not validated.
    pub fn decode(bytes: &[u8; 64]) -> Self {
        Self {
            kind: u32::from_le_bytes(bytes[0..4].try_into().unwrap_or([0; 4])),
            record_size: u32::from_le_bytes(bytes[4..8].try_into().unwrap_or([0; 4])),
            offset: u64::from_le_bytes(bytes[8..16].try_into().unwrap_or([0; 8])),
            length: u64::from_le_bytes(bytes[16..24].try_into().unwrap_or([0; 8])),
            crc64: u64::from_le_bytes(bytes[24..32].try_into().unwrap_or([0; 8])),
            _reserved: bytes[32..64].try_into().unwrap_or([0; 32]),
        }
    }
}

const _: () = {
    assert!(core::mem::size_of::<CortexFileHeader>() == 64);
    assert!(core::mem::align_of::<CortexFileHeader>() == 64);
    assert!(core::mem::size_of::<SectionEntry>() == 64);
    assert!(core::mem::align_of::<SectionEntry>() == 64);
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn magic_and_version_are_the_documented_constants() {
        assert_eq!(&CortexFileHeader::MAGIC, b"VCORTEX1");
        assert_eq!(
            u64::from_be_bytes(CortexFileHeader::MAGIC),
            0x5643_4F52_5445_5831
        );
        assert_eq!(CortexFileHeader::FORMAT_VERSION, 8);
    }

    #[test]
    fn the_crc_matches_the_published_check_value_and_streams() {
        assert_eq!(crc64(b"123456789"), 0x995D_C9BB_DF19_39FA);
        assert_eq!(crc64(b""), 0);
        let mut streamed = Crc64::default();
        streamed.update(b"1234");
        streamed.update(b"");
        streamed.update(b"56789");
        assert_eq!(streamed.finish(), 0x995D_C9BB_DF19_39FA);
        assert_ne!(crc64(&[0; 64]), 0, "sixty-four zero bytes are not zero");
        assert_ne!(crc64(b"VCORTEX1"), crc64(b"VCORTEX2"));
    }

    #[test]
    fn a_header_round_trips_and_validates() {
        let h = CortexFileHeader::new(3, 1000, 4000, 3);
        assert_eq!(h.validate(), Ok(()));
        let bytes = h.encode();
        assert_eq!(&bytes[0..8], b"VCORTEX1");
        assert_eq!(CortexFileHeader::decode(&bytes), h);
        assert_eq!(CortexFileHeader::decode(&bytes).validate(), Ok(()));
        assert_eq!(
            (h.num_neurons, h.num_synapses, h.section_count),
            (1000, 4000, 3)
        );
    }

    #[test]
    fn a_foreign_version_a_bad_crc_a_bad_magic_and_padding_are_refused_in_that_order() {
        let good = CortexFileHeader::new(1, 1, 1, 0);
        let mut foreign = good;
        foreign.version = CortexFileHeader::FORMAT_VERSION + 1;
        foreign.crc64 = foreign.checksum();
        assert_eq!(
            foreign.validate(),
            Err(HeaderError::ForeignVersion(
                CortexFileHeader::FORMAT_VERSION + 1
            )),
            "a foreign version fails closed even with a valid checksum"
        );
        let mut older = good;
        older.version = 6;
        assert_eq!(older.validate(), Err(HeaderError::ForeignVersion(6)));
        let mut corrupt = good;
        corrupt.num_neurons += 1;
        assert_eq!(corrupt.validate(), Err(HeaderError::BadCrc));
        let mut bad_crc = good;
        bad_crc.crc64 ^= 1;
        assert_eq!(bad_crc.validate(), Err(HeaderError::BadCrc));
        let mut magic = good;
        magic.magic[7] = b'2';
        assert_eq!(magic.validate(), Err(HeaderError::BadMagic));
        let mut padded = good;
        padded._padding = 1;
        padded.crc64 = padded.checksum();
        assert_eq!(padded.validate(), Err(HeaderError::BadPadding));
        let mut flags = good;
        flags.reserved_flags = 1;
        flags.crc64 = flags.checksum();
        assert_eq!(flags.validate(), Err(HeaderError::BadPadding));
    }

    #[test]
    fn a_section_entry_round_trips_counts_its_records_and_knows_its_shape() {
        let e = SectionEntry::new(SECTION_NEURON, 64, 128, 64 * 10, 0xABCD);
        assert_eq!(SectionEntry::decode(&e.encode()), e);
        assert_eq!(e.record_count(), 10);
        assert!(e.is_well_formed());
        let deltas = SectionEntry::new(SECTION_PLASTIC_DELTA, 16, 64, 16 * 5, 1);
        assert_eq!(deltas.record_count(), 5);
        assert!(
            !SectionEntry::new(SECTION_SYNAPSE, 64, 100, 64, 0).is_well_formed(),
            "unaligned"
        );
        assert!(
            !SectionEntry::new(SECTION_SYNAPSE, 64, 64, 65, 0).is_well_formed(),
            "a partial record"
        );
        assert!(!SectionEntry::new(SECTION_SYNAPSE, 0, 64, 64, 0).is_well_formed());
        assert_eq!(
            SectionEntry::new(SECTION_SYNAPSE, 0, 64, 64, 0).record_count(),
            0
        );
        let mut dirty = e;
        dirty._reserved[5] = 1;
        assert!(!dirty.is_well_formed());
        assert_eq!(SectionEntry::default().record_count(), 0);
        assert_eq!(
            (SECTION_NEURON, SECTION_SYNAPSE, SECTION_PLASTIC_DELTA),
            (2, 3, 37),
            "Appendix A row numbers"
        );
    }
}
