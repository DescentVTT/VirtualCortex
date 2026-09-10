//! Neuromodulation: the 16-byte modulator vector, one per macro-column (whitepaper §5.2.14),
//! and the reward that moves its dopamine (ADR-0027). The three-factor rule is Specified
//! (§8.8).

#![no_std]
// §8.1: an operation on a state field saturates or wraps by name; plain arithmetic is refused
// here (ADR-0029; migrated under brief 016 on 2026-09-10).
#![deny(clippy::arithmetic_side_effects)]

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(C, align(16))]
pub struct NeuromodulatorState {
    pub dopamine_rpe: i32,   // DA (TD-RPE, Q16.16)
    pub norepinephrine: u32, // NE (Arousal/Surprise, Q16.16)
    pub serotonin: u32,      // 5-HT (Discount factor, Q16.16)
    pub acetylcholine: u32,  // ACh (Sensory precision, Q16.16)
}

impl NeuromodulatorState {
    /// Adds a reward-prediction error to the dopamine signal, saturating (ADR-0027): the
    /// mirth of `cortex-affect` arrives here as a quarter of itself, a confirmed prediction as
    /// its own value, a disappointment as a negative one. Returns the dopamine signal.
    pub fn reward(&mut self, reward_prediction_error_q16: i32) -> i32 {
        self.dopamine_rpe = self
            .dopamine_rpe
            .saturating_add(reward_prediction_error_q16);
        self.dopamine_rpe
    }

    /// Moves the dopamine signal toward zero by $2^{-\text{shift}}$ of itself and at least one
    /// LSB, so a signal decays to rest exactly (a shift past the width is one LSB per call).
    /// Returns it.
    pub fn decay_dopamine(&mut self, shift: u32) -> i32 {
        let d = self.dopamine_rpe as i64;
        let step = (d.abs() >> shift.min(62)).max(1).min(d.abs());
        // In `i64` with `step <= |d|` neither operation can reach the width; the signal is a
        // Q16.16 state field, so both saturate by name (§8.1).
        self.dopamine_rpe = d.saturating_sub(d.signum().saturating_mul(step)) as i32;
        self.dopamine_rpe
    }
}

const _: () = {
    assert!(core::mem::size_of::<NeuromodulatorState>() == 16);
    assert!(core::mem::align_of::<NeuromodulatorState>() == 16);
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn record_is_sixteen_bytes_sixteen_aligned() {
        assert_eq!(core::mem::size_of::<NeuromodulatorState>(), 16);
        assert_eq!(core::mem::align_of::<NeuromodulatorState>(), 16);
    }

    #[test]
    fn dopamine_is_the_only_signed_field() {
        let m = NeuromodulatorState {
            dopamine_rpe: -0x0001_0000,
            norepinephrine: 0,
            serotonin: 0,
            acetylcholine: 0,
        };
        assert!(
            m.dopamine_rpe < 0,
            "a negative reward-prediction error is representable"
        );
        let copy = m;
        assert_eq!(copy, m, "the record is Copy and Eq");
    }

    #[test]
    fn reward_adds_saturating_and_decay_reaches_rest_exactly_from_both_sides() {
        let mut m = NeuromodulatorState {
            dopamine_rpe: 0,
            norepinephrine: 0,
            serotonin: 0,
            acetylcholine: 0,
        };
        assert_eq!(m.reward(0x4000), 0x4000);
        assert_eq!(m.reward(-0x8000), -0x4000);
        m.dopamine_rpe = i32::MAX - 1;
        assert_eq!(m.reward(10), i32::MAX, "saturates");
        m.dopamine_rpe = i32::MIN + 1;
        assert_eq!(m.reward(-10), i32::MIN);
        m.dopamine_rpe = 1000;
        for _ in 0..100 {
            m.decay_dopamine(3);
        }
        assert_eq!(m.dopamine_rpe, 0);
        m.dopamine_rpe = -1000;
        for _ in 0..100 {
            m.decay_dopamine(3);
        }
        assert_eq!(m.dopamine_rpe, 0, "from below too");
        assert_eq!(m.decay_dopamine(3), 0);
        m.dopamine_rpe = 1000;
        assert_eq!(
            m.decay_dopamine(32),
            999,
            "a shift past the width is one LSB"
        );
        assert_eq!(m.decay_dopamine(u32::MAX), 998);
        m.dopamine_rpe = i32::MIN;
        assert_eq!(
            m.decay_dopamine(0),
            0,
            "the most negative signal decays to rest in one step"
        );
        m.dopamine_rpe = i32::MAX;
        assert_eq!(m.decay_dopamine(0), 0);
    }
}
