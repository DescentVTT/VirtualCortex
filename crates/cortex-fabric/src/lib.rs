//! Distributed Scale-Out Cluster Mesh, Kernel-Bypass RDMA & CXL Fabric

#[repr(C, align(64))]
pub struct FabricPacketHeader {
    pub magic: [u8; 4],        // [0..4] "VCFB" (0x56434642)
    pub src_node_id: u16,      // [4..6] Source cluster node ID
    pub dst_node_id: u16,      // [6..8] Destination cluster node ID
    pub epoch_barrier_id: u64, // [8..16] Distributed causal epoch barrier counter
    pub sequence_number: u64,  // [16..24] Packet sequence for zero-drop RDMA
    pub payload_bytes: u32,    // [24..28] Event payload length
    pub packet_type: u16,      // [28..30] 0: SpikeBatch, 1: NeuromodBroadcast, 2: Sync
    pub checksum_crc16: u16,   // [30..32] Hardware packet validation CRC
    pub _reserved: [u8; 32],   // [32..64] Strict 64-byte cache-line alignment padding
}

impl FabricPacketHeader {
    pub const MAGIC: [u8; 4] = *b"VCFB";
}

const _: () = {
    assert!(core::mem::size_of::<FabricPacketHeader>() == 64);
    assert!(core::mem::align_of::<FabricPacketHeader>() == 64);
};
