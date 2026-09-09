//! Connectome Priors and Laminar Microcolumn Specifications

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

const _: () = {
    assert!(core::mem::size_of::<CortexFileHeader>() == 64);
    assert!(core::mem::align_of::<CortexFileHeader>() == 64);
};
