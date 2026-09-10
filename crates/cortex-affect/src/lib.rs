//! Interoceptive state: the integration of pain, strain and recovery into an allostatic load,
//! a comfort signal and a slow mood baseline (whitepaper §5.2.23, §8.8; admitted by
//! ADR-0016).
//!
//! `cortex-homeostasis` holds the metabolic drives and the circadian gate; this crate holds
//! what the body feels like. One record per interoceptive region. The integration rule is
//! Implemented; how the mood baseline biases `cortex-neuromod` is Specified.

#![no_std]

/// 1.0 in Q16.16.
pub const Q16_ONE: u32 = 0x0001_0000;
/// Mood follows comfort with a time constant of 2^MOOD_SHIFT steps.
pub const MOOD_SHIFT: u32 = 6;

/// 64-byte interoceptive state (whitepaper §5.2.23).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(C, align(64))]
pub struct InteroceptiveState {
    pub somatic_comfort_q16: i32, // [0..4] +1.0 at zero load, -1.0 at a load of 1.0 or more (Q16.16)
    pub allostatic_load_q16: u32, // [4..8] Accumulated strain not yet recovered (Q16.16)
    pub thermal_strain_q16: u32,  // [8..12] Last thermal strain input (Q16.16)
    pub energy_resilience_q16: u32, // [12..16] Reserve available to absorb load (Q16.16, Specified)
    pub mood_baseline_q16: i32,   // [16..20] Slow average of comfort (Q16.16)
    pub pain_signal_burst: u32,   // [20..24] Last pain input (Q16.16)
    pub _reserved: [u8; 40],      // [24..64] Reserved; MUST be zero
}

// Arrays longer than 32 elements do not implement Default, so the all-zero record is
// spelled out; every field of a default record is zero.
impl Default for InteroceptiveState {
    fn default() -> Self {
        Self {
            somatic_comfort_q16: 0,
            allostatic_load_q16: 0,
            thermal_strain_q16: 0,
            energy_resilience_q16: 0,
            mood_baseline_q16: 0,
            pain_signal_burst: 0,
            _reserved: [0; 40],
        }
    }
}

impl InteroceptiveState {
    /// One integration step. Load gains the pain and strain inputs and loses `recovery`, all
    /// saturating and never below zero; comfort is `1 - 2·min(load, 1)`; mood moves toward
    /// comfort by a 2^-MOOD_SHIFT fraction of the difference. Returns the comfort.
    pub fn integrate(
        &mut self,
        pain_burst_q16: u32,
        thermal_strain_q16: u32,
        recovery_q16: u32,
    ) -> i32 {
        self.pain_signal_burst = pain_burst_q16;
        self.thermal_strain_q16 = thermal_strain_q16;
        self.allostatic_load_q16 = self
            .allostatic_load_q16
            .saturating_add(pain_burst_q16)
            .saturating_add(thermal_strain_q16)
            .saturating_sub(recovery_q16);
        let bounded = self.allostatic_load_q16.min(Q16_ONE) as i32;
        self.somatic_comfort_q16 = Q16_ONE as i32 - 2 * bounded;
        let gap = self.somatic_comfort_q16 as i64 - self.mood_baseline_q16 as i64;
        self.mood_baseline_q16 = (self.mood_baseline_q16 as i64 + (gap >> MOOD_SHIFT)) as i32;
        self.somatic_comfort_q16
    }
}

const _: () = {
    assert!(core::mem::size_of::<InteroceptiveState>() == 64);
    assert!(core::mem::align_of::<InteroceptiveState>() == 64);
};

#[cfg(test)]
mod tests {
    use super::*;

    const ONE: u32 = Q16_ONE;

    #[test]
    fn record_is_one_cache_line_and_default_is_zero() {
        assert_eq!(core::mem::size_of::<InteroceptiveState>(), 64);
        assert_eq!(core::mem::align_of::<InteroceptiveState>(), 64);
        assert_eq!(InteroceptiveState::default().allostatic_load_q16, 0);
    }

    #[test]
    fn no_load_is_full_comfort_and_a_full_load_is_full_discomfort() {
        let mut s = InteroceptiveState::default();
        assert_eq!(s.integrate(0, 0, 0), ONE as i32);
        assert_eq!(s.integrate(ONE, 0, 0), -(ONE as i32));
        assert_eq!(
            s.integrate(ONE, ONE, 0),
            -(ONE as i32),
            "comfort is bounded below at -1"
        );
    }

    #[test]
    fn recovery_reduces_load_and_never_below_zero() {
        let mut s = InteroceptiveState::default();
        s.integrate(ONE / 2, 0, 0);
        assert_eq!(s.allostatic_load_q16, ONE / 2);
        assert_eq!(
            s.integrate(0, 0, ONE / 4),
            (ONE / 2) as i32,
            "a quarter load is half comfort"
        );
        assert_eq!(s.allostatic_load_q16, ONE / 4);
        s.integrate(0, 0, u32::MAX);
        assert_eq!(s.allostatic_load_q16, 0);
        assert_eq!(s.somatic_comfort_q16, ONE as i32);
    }

    #[test]
    fn load_saturates_instead_of_wrapping() {
        let mut s = InteroceptiveState::default();
        s.integrate(u32::MAX, u32::MAX, 0);
        assert_eq!(s.allostatic_load_q16, u32::MAX);
        assert_eq!(s.somatic_comfort_q16, -(ONE as i32));
    }

    #[test]
    fn mood_follows_comfort_slowly_and_converges() {
        let mut s = InteroceptiveState::default();
        s.integrate(0, 0, 0);
        assert_eq!(
            s.mood_baseline_q16,
            (ONE >> MOOD_SHIFT) as i32,
            "one step moves 1/64 of the gap"
        );
        for _ in 0..2000 {
            s.integrate(0, 0, 0);
        }
        assert!(
            (s.mood_baseline_q16 - ONE as i32).abs() <= (1 << MOOD_SHIFT),
            "within one step of comfort"
        );
        for _ in 0..2000 {
            s.integrate(ONE, 0, 0);
        }
        assert!((s.mood_baseline_q16 + ONE as i32).abs() <= (1 << MOOD_SHIFT));
    }
}
