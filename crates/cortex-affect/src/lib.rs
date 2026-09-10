//! Interoceptive state: the integration of pain, strain and recovery into an allostatic load,
//! a comfort signal, a slow mood baseline, and, since ADR-0020, the affective valence as the
//! negative time derivative of free energy and the existential stake as the volatility of that
//! free energy (whitepaper §5.2.23, §8.8, §8.12; admitted by ADR-0016).
//!
//! `cortex-homeostasis` holds the metabolic drives and the circadian gate; this crate holds
//! what the body feels like. One record per interoceptive region. The integration and valence
//! rules and the metaphor source-domain map (ADR-0021) are Implemented; how the mood baseline
//! biases `cortex-neuromod` and how the stake preempts executive bandwidth are Specified.

#![no_std]

/// 1.0 in Q16.16.
pub const Q16_ONE: u32 = 0x0001_0000;
/// Mood follows comfort with a time constant of 2^MOOD_SHIFT steps.
pub const MOOD_SHIFT: u32 = 6;
/// The existential stake follows |dF/dt| with a time constant of 2^STAKE_SHIFT steps.
pub const STAKE_SHIFT: u32 = 4;

/// Metaphor source domains (ADR-0021): which bodily condition dominates, for `cortex-symbolic`
/// to blend into a target concept and `cortex-linguistic` to realise.
/// Thermal strain dominates: heat, fire, fever.
pub const DOMAIN_HEAT: u16 = 0x0001;
/// Allostatic load dominates: weight, burden, fatigue.
pub const DOMAIN_WEIGHT: u16 = 0x0002;
/// Energy resilience is low: dusk, ebb, dimming.
pub const DOMAIN_DUSK: u16 = 0x0004;
/// Comfort is high and nothing strains: calm, still water, warmth without heat.
pub const DOMAIN_CALM: u16 = 0x0008;

/// `mirth_q16` moves by $2^{-2}$ of its gap to the latest benign incongruity, at least one LSB.
pub const MIRTH_SHIFT: u32 = 2;
/// A threat above 0.25 is not benign: the incongruity it comes with is not funny (ADR-0027).
pub const BENIGN_THREAT_MAX_Q16: u32 = Q16_ONE / 4;
/// A mirth at or above 0.25 reads as amusement.
pub const MIRTH_THRESHOLD_Q16: u32 = Q16_ONE / 4;

/// 64-byte interoceptive state (whitepaper §5.2.23).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(C, align(64))]
pub struct InteroceptiveState {
    pub somatic_comfort_q16: i32, // [0..4] +1.0 at zero load, -1.0 at a load of 1.0 or more (Q16.16)
    pub allostatic_load_q16: u32, // [4..8] Accumulated strain not yet recovered (Q16.16)
    pub thermal_strain_q16: u32,  // [8..12] Last thermal strain input (Q16.16)
    pub energy_resilience_q16: u32, // [12..16] Reserve available to absorb load (Q16.16, Specified)
    pub mood_baseline_q16: i32,   // [16..20] Slow average of comfort (Q16.16)
    pub pain_signal_burst: u32,   // [20..24] Last pain input (Q16.16)
    pub free_energy_prev_q16: u32, // [24..28] Free energy at the previous valence update (Q16.16)
    pub valence_df_dt_q16: i32, // [28..32] Valence: minus the change of free energy per update (Q16.16)
    pub existential_stake_q16: u32, // [32..36] Slow average of |dF/dt|: how much the body's predictions fail (Q16.16)
    pub benign_incongruity_q16: u32, // [36..40] The last surprise that carried no threat (Q16.16, ADR-0027)
    pub mirth_q16: u32, // [40..44] Slow average of benign incongruity: the appraisal that reads as amusement (Q16.16, ADR-0027)
    pub _reserved: [u8; 20], // [44..64] Reserved; MUST be zero
}

impl InteroceptiveState {
    /// One integration step. Load gains the pain and strain inputs and loses `recovery`, all
    /// saturating and never below zero; comfort is `1 - 2·min(load, 1)`; mood moves toward
    /// comfort by a 2^-MOOD_SHIFT fraction of the difference and by at least one LSB, so a
    /// held comfort is reached exactly. Returns the comfort.
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
        // At least one LSB toward comfort, never past it: an arithmetic shift floors only
        // negatives, so without the floor the mood would stall 2^MOOD_SHIFT - 1 below +1.0.
        let step = (gap.abs() >> MOOD_SHIFT).max(1).min(gap.abs());
        self.mood_baseline_q16 = (self.mood_baseline_q16 as i64 + gap.signum() * step) as i32;
        self.somatic_comfort_q16
    }

    /// Valence as the negative time derivative of variational free energy (ADR-0020, whitepaper
    /// §8.12): with `free_energy_q16` the current free energy, `valence = F_prev - F_now`,
    /// widened and clamped, so a collapse of uncertainty is positive and a failure to resolve
    /// prediction error is negative. The existential stake follows `|F_now - F_prev|` as a slow
    /// average that takes at least one LSB toward its target, so a quiet body's stake reaches
    /// zero. The first call after rest has no previous value and reads it as zero. Returns the
    /// valence.
    pub fn update_valence(&mut self, free_energy_q16: u32) -> i32 {
        let delta = self.free_energy_prev_q16 as i64 - free_energy_q16 as i64;
        self.valence_df_dt_q16 = delta.clamp(i32::MIN as i64, i32::MAX as i64) as i32;
        let magnitude = delta.unsigned_abs().min(u32::MAX as u64) as u32;
        let stake = self.existential_stake_q16;
        self.existential_stake_q16 = if magnitude > stake {
            stake.saturating_add(((magnitude - stake) >> STAKE_SHIFT).max(1))
        } else if magnitude < stake {
            stake - ((stake - magnitude) >> STAKE_SHIFT).max(1)
        } else {
            stake
        };
        self.free_energy_prev_q16 = free_energy_q16;
        self.valence_df_dt_q16
    }

    /// The benign-violation appraisal (ADR-0027, whitepaper §8.17): `surprise_q16` is the
    /// prediction error of the moment and `threat_q16` the salience threat that came with it.
    /// The incongruity is benign when the threat is at most 0.25, and is then the surprise;
    /// otherwise zero. Mirth moves toward it by $2^{-2}$ of the gap and at least one LSB, so
    /// it reaches zero exactly when nothing incongruous has happened for a while. Returns the
    /// benign incongruity.
    pub fn appraise_incongruity(&mut self, surprise_q16: u32, threat_q16: u32) -> u32 {
        let benign = if threat_q16 > BENIGN_THREAT_MAX_Q16 {
            0
        } else {
            surprise_q16
        };
        self.benign_incongruity_q16 = benign;
        let current = self.mirth_q16;
        self.mirth_q16 = if benign > current {
            current.saturating_add(((benign - current) >> MIRTH_SHIFT).max(1))
        } else if benign < current {
            current - ((current - benign) >> MIRTH_SHIFT).max(1)
        } else {
            current
        };
        benign
    }

    /// True while the mirth reads as amusement: `cortex-neuromod` takes a quarter of it as
    /// reward, `cortex-imagination` wanders at it, `cortex-linguistic` may mark play.
    #[inline]
    pub const fn is_amused(&self) -> bool {
        self.mirth_q16 >= MIRTH_THRESHOLD_Q16
    }

    /// The metaphor source domain the body currently offers (ADR-0021, whitepaper §8.13):
    /// `DOMAIN_CALM` when comfort is at least 0.5 and none of thermal strain, allostatic load
    /// and the energy deficit `1 - resilience` exceeds 0.25; otherwise the largest of the
    /// three, ties broken in that order. Deterministic; the lexicon turns the domain into words.
    pub const fn metaphor_source_domain(&self) -> u16 {
        let quarter = Q16_ONE / 4;
        let deficit = Q16_ONE.saturating_sub(self.energy_resilience_q16);
        if self.somatic_comfort_q16 >= (Q16_ONE / 2) as i32
            && self.thermal_strain_q16 <= quarter
            && self.allostatic_load_q16 <= quarter
            && deficit <= quarter
        {
            return DOMAIN_CALM;
        }
        if self.thermal_strain_q16 >= self.allostatic_load_q16 && self.thermal_strain_q16 >= deficit
        {
            DOMAIN_HEAT
        } else if self.allostatic_load_q16 >= deficit {
            DOMAIN_WEIGHT
        } else {
            DOMAIN_DUSK
        }
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
        let d = InteroceptiveState::default();
        assert_eq!(d.allostatic_load_q16, 0);
        assert_eq!((d.valence_df_dt_q16, d.existential_stake_q16), (0, 0));
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
        assert_eq!(
            s.mood_baseline_q16, ONE as i32,
            "a held comfort is reached exactly, not 63 LSB short"
        );
        for _ in 0..2000 {
            s.integrate(ONE, 0, 0);
        }
        assert_eq!(s.mood_baseline_q16, -(ONE as i32));
    }

    #[test]
    fn valence_is_minus_the_change_of_free_energy() {
        let mut s = InteroceptiveState::default();
        assert_eq!(
            s.update_valence(ONE),
            -(ONE as i32),
            "from rest, rising free energy is negative"
        );
        let mut t = InteroceptiveState {
            free_energy_prev_q16: ONE,
            ..Default::default()
        };
        assert_eq!(t.update_valence(ONE), 0, "unchanged free energy is neutral");
        assert_eq!(
            t.update_valence(ONE / 2),
            (ONE / 2) as i32,
            "uncertainty collapsing is positive"
        );
        assert_eq!(
            t.update_valence(2 * ONE),
            -((3 * ONE / 2) as i32),
            "prediction error growing is negative"
        );
        assert_eq!(t.free_energy_prev_q16, 2 * ONE);
        let mut x = InteroceptiveState {
            free_energy_prev_q16: u32::MAX,
            ..Default::default()
        };
        assert_eq!(x.update_valence(0), i32::MAX, "clamps rather than wraps");
        assert_eq!(x.update_valence(u32::MAX), i32::MIN);
    }

    #[test]
    fn the_existential_stake_rises_with_volatile_free_energy_and_falls_to_zero_when_steady() {
        let mut s = InteroceptiveState::default();
        for i in 0..200u32 {
            s.update_valence(if i % 2 == 0 { ONE } else { 0 });
        }
        assert!(
            s.existential_stake_q16 > ONE / 2,
            "swings of 1.0 drive the stake up: {:#x}",
            s.existential_stake_q16
        );
        for _ in 0..5000 {
            s.update_valence(ONE);
        }
        assert_eq!(
            s.existential_stake_q16, 0,
            "steady free energy lets the stake reach zero exactly"
        );
        let mut m = InteroceptiveState::default();
        m.update_valence(u32::MAX);
        assert_eq!(m.existential_stake_q16, (u32::MAX >> STAKE_SHIFT).max(1));
    }

    #[test]
    fn the_metaphor_domain_follows_the_dominant_condition() {
        let calm = InteroceptiveState {
            somatic_comfort_q16: ONE as i32,
            energy_resilience_q16: ONE,
            ..Default::default()
        };
        assert_eq!(calm.metaphor_source_domain(), DOMAIN_CALM);
        let starved = InteroceptiveState {
            somatic_comfort_q16: ONE as i32,
            energy_resilience_q16: 0,
            ..Default::default()
        };
        assert_eq!(
            starved.metaphor_source_domain(),
            DOMAIN_DUSK,
            "comfort with an empty reserve is not calm"
        );
        let hot = InteroceptiveState {
            thermal_strain_q16: ONE / 2,
            allostatic_load_q16: ONE / 4,
            energy_resilience_q16: ONE,
            ..Default::default()
        };
        assert_eq!(hot.metaphor_source_domain(), DOMAIN_HEAT);
        let heavy = InteroceptiveState {
            thermal_strain_q16: ONE / 8,
            allostatic_load_q16: ONE / 2,
            energy_resilience_q16: ONE,
            ..Default::default()
        };
        assert_eq!(heavy.metaphor_source_domain(), DOMAIN_WEIGHT);
        let dim = InteroceptiveState {
            energy_resilience_q16: ONE / 8,
            ..Default::default()
        };
        assert_eq!(
            dim.metaphor_source_domain(),
            DOMAIN_DUSK,
            "an unfed body at rest is dusk, not calm"
        );
        let tie = InteroceptiveState {
            thermal_strain_q16: ONE / 2,
            allostatic_load_q16: ONE / 2,
            energy_resilience_q16: ONE / 2,
            ..Default::default()
        };
        assert_eq!(
            tie.metaphor_source_domain(),
            DOMAIN_HEAT,
            "ties go to heat, then weight, then dusk"
        );
    }

    #[test]
    fn a_surprise_without_threat_is_benign_and_raises_mirth_and_a_threat_makes_it_nothing() {
        let mut s = InteroceptiveState::default();
        assert_eq!(
            s.appraise_incongruity(ONE / 2, 0),
            ONE / 2,
            "a half surprise, no threat"
        );
        assert_eq!(s.mirth_q16, ONE >> (MIRTH_SHIFT + 1));
        assert!(!s.is_amused(), "one half surprise is not yet amusement");
        assert_eq!(
            s.appraise_incongruity(ONE, ONE / 4),
            ONE,
            "a threat of exactly a quarter is still benign"
        );
        assert!(s.is_amused());
        assert_eq!(
            s.appraise_incongruity(ONE, ONE / 4 + 1),
            0,
            "just above it is not"
        );
        assert_eq!(s.benign_incongruity_q16, 0);
        assert!(s.mirth_q16 < ONE >> 1, "mirth falls toward zero");
        for _ in 0..100 {
            s.appraise_incongruity(0, 0);
        }
        assert_eq!(s.mirth_q16, 0, "exactly zero after a quiet while");
        assert!(!s.is_amused());
    }

    #[test]
    fn mirth_saturates_toward_the_surprise_and_two_bodies_appraise_alike() {
        let mut a = InteroceptiveState::default();
        let mut b = InteroceptiveState::default();
        for i in 0..64u32 {
            let surprise = (i * 1000) % ONE;
            let threat = if i % 5 == 0 { ONE } else { 0 };
            assert_eq!(
                a.appraise_incongruity(surprise, threat),
                b.appraise_incongruity(surprise, threat)
            );
            assert!(a.mirth_q16 <= ONE);
        }
        assert_eq!(a, b);
    }
}
