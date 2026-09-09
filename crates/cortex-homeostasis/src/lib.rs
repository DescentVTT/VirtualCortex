//! Autonomic Homeostatic Drive Pools & Circadian Sleep-Wake Engine

#[repr(C, align(64))]
pub struct HomeostaticDrivePool {
    pub energy_level: u32,          // [0..4] Glucose/Battery reserve (Q16.16)
    pub sensory_fatigue: u32,       // [4..8] Accumulated synaptic load (Q16.16)
    pub curiosity_drive: u32,       // [8..12] Intrinsic novelty seeking drive (Q16.16)
    pub thermal_stress: u32,        // [12..16] Processing temperature / strain (Q16.16)
    pub circadian_phase: u32,       // [16..20] 24h internal phase angle (0..65535)
    pub sleep_mode_active: u32,     // [20..24] 1 if in SWR memory consolidation sleep, 0 awake
    pub branching_ratio_q16: u32,   // [24..28] Self-Organized Criticality sigma (~1.0)
    pub target_threshold_bias: i32, // [28..32] Dynamic threshold correction (mV, Q16.16)
    pub _reserved: [u8; 32],        // [32..64] Strict 64-byte cache-line alignment padding
}

impl HomeostaticDrivePool {
    #[inline(always)]
    pub fn update_circadian_tick(&mut self, dt_ticks: u32) {
        self.circadian_phase = (self.circadian_phase + dt_ticks) & 0xFFFF;
        // Sleep phase active when fatigue exceeds threshold or in nocturnal phase
        if self.sensory_fatigue > 0x8000_0000 || self.circadian_phase > 0xC000 {
            self.sleep_mode_active = 1;
        } else {
            self.sleep_mode_active = 0;
        }
    }
}

const _: () = {
    assert!(core::mem::size_of::<HomeostaticDrivePool>() == 64);
    assert!(core::mem::align_of::<HomeostaticDrivePool>() == 64);
};
