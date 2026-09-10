//! Connectome: the `.cortex` image header (whitepaper §5.2.2). Sections, the CRC, the loader
//! and the laminar priors are Specified (§8.7).

#![no_std]

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(C, align(64))]
pub struct CortexFileHeader {
    pub magic: [u8; 8],      // "VCORTEX1" (0x56434F5254455831)
    pub version: u32,        // Format version
    pub reserved_flags: u32, // Feature flags
    pub num_columns: u64,    // Total cortical hyper-columns
    pub num_neurons: u64,    // Total neuron count
    pub num_synapses: u64,   // Total initial synapse count
    pub layers_offset: u64,  // Byte offset to laminar section
    pub crc64: u64,          // Header integrity checksum
    pub _padding: [u8; 8],   // Strict 64-byte alignment
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
    pub const FORMAT_VERSION: u32 = 3;
}

const _: () = {
    assert!(core::mem::size_of::<CortexFileHeader>() == 64);
    assert!(core::mem::align_of::<CortexFileHeader>() == 64);
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
        assert_eq!(CortexFileHeader::FORMAT_VERSION, 3);
    }
}
