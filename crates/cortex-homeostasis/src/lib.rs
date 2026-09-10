//! Homeostasis: metabolic drive pools, the circadian sleep gate and criticality control
//! (whitepaper §5.2.16). Interoception is `cortex-affect`'s and hardware vitals are
//! `cortex-autonomic`'s (ADR-0016).

#![no_std]

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
    /// Advances the 16-bit circadian phase and sets the sleep gate. The phase is a counter
    /// whose wrap is the intended semantics (whitepaper §8.1), so the addition is explicitly
    /// wrapping before the mask; the mask alone would not prevent a debug-build overflow
    /// panic for a large `dt_ticks`.
    #[inline(always)]
    pub fn update_circadian_tick(&mut self, dt_ticks: u32) {
        self.circadian_phase = self.circadian_phase.wrapping_add(dt_ticks) & 0xFFFF;
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

#[cfg(test)]
mod tests {
    use super::*;

    fn pool(circadian_phase: u32, sensory_fatigue: u32) -> HomeostaticDrivePool {
        HomeostaticDrivePool {
            energy_level: 0,
            sensory_fatigue,
            curiosity_drive: 0,
            thermal_stress: 0,
            circadian_phase,
            sleep_mode_active: 0,
            branching_ratio_q16: 0,
            target_threshold_bias: 0,
            _reserved: [0; 32],
        }
    }

    #[test]
    fn phase_wraps_at_sixteen_bits() {
        let mut p = pool(0xFFFF, 0);
        p.update_circadian_tick(1);
        assert_eq!(p.circadian_phase, 0);
        assert_eq!(p.sleep_mode_active, 0);
    }

    #[test]
    fn phase_wraps_for_the_largest_tick_delta() {
        let mut p = pool(0xFFFF, 0);
        p.update_circadian_tick(u32::MAX);
        assert_eq!(p.circadian_phase, 0xFFFE);
        assert_eq!(p.sleep_mode_active, 1);
    }

    #[test]
    fn sleep_gate_opens_one_tick_past_three_quarters() {
        let mut p = pool(0xBFFF, 0);
        p.update_circadian_tick(1);
        assert_eq!(p.circadian_phase, 0xC000);
        assert_eq!(p.sleep_mode_active, 0);
        p.update_circadian_tick(1);
        assert_eq!(p.circadian_phase, 0xC001);
        assert_eq!(p.sleep_mode_active, 1);
    }

    #[test]
    fn fatigue_at_exactly_half_does_not_force_sleep() {
        let mut p = pool(0, 0x8000_0000);
        p.update_circadian_tick(0);
        assert_eq!(p.sleep_mode_active, 0);
        let mut q = pool(0, 0x8000_0001);
        q.update_circadian_tick(0);
        assert_eq!(q.sleep_mode_active, 1);
    }
}
