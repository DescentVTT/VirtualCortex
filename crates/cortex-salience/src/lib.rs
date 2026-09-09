//! Amygdalar Threat Valuation, 12ms Subcortical Low-Road & Emotional Tagging

#[repr(C, align(64))]
pub struct SalienceNodeState {
    pub node_id: u32,                // [0..4] Salience node index
    pub threat_valence: i32,         // [4..8] Threat intensity (-1.0..+1.0 in Q16.16)
    pub low_road_ticks: u32,         // [8..12] Countdown for 12ms emergency reflex
    pub fear_conditioning_w: i32,    // [12..16] Conditioned stimulus weight (Q16.16)
    pub defense_mode_flags: u32,     // [16..20] 0: None, 1: Freeze, 2: Flight, 3: Fight
    pub emotional_tag_priority: u32, // [20..24] Priority boost for hippocampal SWR replay
    pub unconditioned_stimulus: i32, // [24..28] Pain / shock immediate input (Q16.16)
    pub override_active: u32,        // [28..32] 1 if motor override engaged, 0 otherwise
    pub _reserved: [u8; 32],         // [32..64] Strict 64-byte cache-line alignment padding
}

impl SalienceNodeState {
    #[inline(always)]
    pub fn evaluate_threat(&mut self, sensory_shock: i32) -> bool {
        self.unconditioned_stimulus = sensory_shock;
        if sensory_shock > 0x0002_0000 || self.fear_conditioning_w > 0x0001_0000 {
            self.threat_valence = 0x0001_0000;
            self.defense_mode_flags = 1; // Freeze reflex
            self.override_active = 1;
            self.emotional_tag_priority = 255;
            true
        } else {
            self.override_active = 0;
            false
        }
    }
}

const _: () = {
    assert!(core::mem::size_of::<SalienceNodeState>() == 64);
    assert!(core::mem::align_of::<SalienceNodeState>() == 64);
};
