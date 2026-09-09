//! Global Neuronal Workspace (GNWT), Non-Linear Ignition & Metacognition

#[repr(C, align(64))]
pub struct GlobalWorkspaceSlot {
    pub slot_id: u32,                // [0..4] Active workspace slot index (0..7)
    pub binding_hash: u32,           // [4..8] Hash of bound cortical concept
    pub ignition_potential: i32,     // [8..12] Non-linear threshold accumulator (Q16.16)
    pub persistence_ticks: u32,      // [12..16] Working memory reverberation counter
    pub confidence_q16: u32,         // [16..20] Metacognitive certainty metric (0..1.0 in Q16.16)
    pub broadcast_channel_mask: u32, // [20..24] Target cortical recipient mask
    pub p300_wave_phase: u32,        // [24..28] Ignition oscillation phase angle
    pub is_ignited: u32,             // [28..32] 1 if consciously ignited, 0 if subliminal
    pub _reserved: [u8; 32],         // [32..64] Strict 64-byte cache-line alignment padding
}

impl GlobalWorkspaceSlot {
    pub const IGNITION_THRESHOLD: i32 = 0x0001_8000; // 1.5 in Q16.16

    #[inline(always)]
    pub fn step_ignition(&mut self, bottom_up_evidence: i32) -> bool {
        self.ignition_potential += bottom_up_evidence;
        if self.ignition_potential >= Self::IGNITION_THRESHOLD {
            self.is_ignited = 1;
            self.persistence_ticks = 300; // 300ms conscious persistence window
            true
        } else {
            self.is_ignited = 0;
            false
        }
    }
}

const _: () = {
    assert!(core::mem::size_of::<GlobalWorkspaceSlot>() == 64);
    assert!(core::mem::align_of::<GlobalWorkspaceSlot>() == 64);
};
