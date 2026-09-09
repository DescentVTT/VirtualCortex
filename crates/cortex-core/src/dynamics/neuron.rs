use core::sync::atomic::{AtomicU64, AtomicU8};

/// 64-Byte POD Cache-Line Aligned Matthew Larkum BAC Dendritic Super-Neuron
#[repr(C, align(64))]
pub struct DendriticSuperNeuron {
    pub id: u64,                        // [0..8] Global neuron ID
    pub mailbox_head_ptr: AtomicU64,    // [8..16] Lock-free MPSC mailbox pointer
    pub mailbox_tag: u64,               // [16..24] 64-bit ABA tag
    pub v_soma: i32,                    // [24..28] Soma potential (Q16.16)
    pub v_basal: i32,                   // [28..32] Basal feedforward potential (Q16.16)
    pub v_apical: i32,                  // [32..36] Apical contextual potential (Q16.16)
    pub v_thresh: i32,                  // [36..40] Dynamic adaptive threshold (Q16.16)
    pub bac_plateau_ticks: u16,         // [40..42] Larkum BAC calcium burst countdown
    pub refractory_ticks: u16,          // [42..44] Absolute refractory countdown
    pub last_soma_spike_tick: u32,      // [44..48] Somatic action potential timestamp
    pub synapse_slab_idx: u32,          // [48..52] Index into SynapseBlock arena
    pub plastic_delta_head: u16,        // [52..54] Index into CXL.mem delta table
    pub spatial_voxel_morton: u16,      // [54..56] 16-bit Morton spatial voxel code
    pub gate_state: AtomicU8,           // [56] Virtual actor state machine flag
    pub flags: u8,                      // [57] BURST_MODE / Inhibitory Flags
    pub stp_r_ves: u8,                  // [58] Tsodyks-Markram vesicle pool (STD)
    pub stp_u_rel: u8,                  // [59] Tsodyks-Markram release fraction (STF)
    pub _reserved: [u8; 4],             // [60..64] Hardware cache-line alignment padding
}

/// Strict 64-byte synaptic connection block
#[repr(C, align(64))]
pub struct SynapseBlock {
    pub target_neuron_ids: [u32; 4],    // [0..16] 4 target neuron indices
    pub weights_q16: [i16; 4],          // [16..24] 4 static weights (Q16.16)
    pub delays_ticks: [u16; 4],         // [24..32] Axonal transmission delays
    pub next_block_idx: u32,            // [32..36] Index to chained overflow block
    pub last_spike_tick: u32,           // [36..40] Synapse timestamp for STDP
    pub _reserved: [u8; 24],            // [40..64] Cache-line alignment padding
}

const _: () = {
    assert!(core::mem::size_of::<DendriticSuperNeuron>() == 64);
    assert!(core::mem::align_of::<DendriticSuperNeuron>() == 64);
    assert!(core::mem::size_of::<SynapseBlock>() == 64);
    assert!(core::mem::align_of::<SynapseBlock>() == 64);
};
