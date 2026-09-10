//! Neuromodulation: the 16-byte modulator vector, one per macro-column (whitepaper §5.2.14),
//! the reward that moves its dopamine (ADR-0027), and the modulation of three-factor
//! plasticity that the dopamine signal sets (ADR-0032): the fraction of a synapse's
//! eligibility trace that `cortex-core` consolidates into the weight at a presynaptic spike.
//! The record's bytes for the image and the per-tick decay the executor applies are here too.

#![no_std]
// §8.1: an operation on a state field saturates or wraps by name; plain arithmetic is refused
// here (ADR-0029; migrated under brief 016 on 2026-09-10).
#![deny(clippy::arithmetic_side_effects)]

/// 1.0 in Q16.16: the modulation above which nothing more of a trace can be consolidated.
const Q16_ONE: i32 = 0x0001_0000;

/// The dopamine signal's time constant for the executor's per-tick decay, $2^{14}$ ticks
/// (164 ms at 10 µs; ADR-0032). A reward that arrives after a pairing consolidates the
/// pairing's eligibility for about that long.
pub const DOPAMINE_TAU_SHIFT: u32 = 14;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(C, align(16))]
pub struct NeuromodulatorState {
    pub dopamine_rpe: i32,   // [0..4] DA (TD-RPE, Q16.16)
    pub norepinephrine: u32, // [4..8] NE (Arousal/Surprise, Q16.16)
    pub serotonin: u32,      // [8..12] 5-HT (Discount factor, Q16.16)
    pub acetylcholine: u32,  // [12..16] ACh (Sensory precision, Q16.16)
}

impl NeuromodulatorState {
    /// The record at rest: every signal zero. Equal to `Default`.
    pub const fn new() -> Self {
        Self {
            dopamine_rpe: 0,
            norepinephrine: 0,
            serotonin: 0,
            acetylcholine: 0,
        }
    }

    /// True when every signal is zero: the state an image at rest holds, and the one the
    /// writer leaves out of the image.
    pub const fn is_at_rest(&self) -> bool {
        self.dopamine_rpe == 0
            && self.norepinephrine == 0
            && self.serotonin == 0
            && self.acetylcholine == 0
    }

    /// The modulation of three-factor plasticity (ADR-0032): `baseline + dopamine`, saturating,
    /// clamped to $[0, 1]$ in Q16.16. It is the fraction of each synapse's eligibility trace
    /// that `SynapseBlock::consolidate` moves into the weight at the presynaptic spike: 1.0
    /// consolidates everything at once (the rule of ADR-0022), 0 consolidates nothing and lets
    /// the trace decay, a value between them consolidates that fraction. The baseline is the
    /// caller's argument (the executor's configuration); a positive reward-prediction error
    /// raises the modulation toward 1.0, a negative one lowers it toward 0. There is no
    /// anti-Hebbian reversal: a dip below zero is clamped, as in Izhikevich 2007.
    pub fn modulation(&self, baseline_q16: i32) -> i32 {
        baseline_q16
            .saturating_add(self.dopamine_rpe)
            .clamp(0, Q16_ONE)
    }

    /// The record's 16 bytes, little-endian, field by field (§8.7).
    pub fn encode(&self) -> [u8; 16] {
        let mut out = [0u8; 16];
        out[0..4].copy_from_slice(&self.dopamine_rpe.to_le_bytes());
        out[4..8].copy_from_slice(&self.norepinephrine.to_le_bytes());
        out[8..12].copy_from_slice(&self.serotonin.to_le_bytes());
        out[12..16].copy_from_slice(&self.acetylcholine.to_le_bytes());
        out
    }

    /// A record from its 16 bytes.
    pub fn decode(bytes: &[u8; 16]) -> Self {
        Self {
            dopamine_rpe: i32::from_le_bytes(bytes[0..4].try_into().unwrap_or([0; 4])),
            norepinephrine: u32::from_le_bytes(bytes[4..8].try_into().unwrap_or([0; 4])),
            serotonin: u32::from_le_bytes(bytes[8..12].try_into().unwrap_or([0; 4])),
            acetylcholine: u32::from_le_bytes(bytes[12..16].try_into().unwrap_or([0; 4])),
        }
    }

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

    #[test]
    fn the_modulation_is_the_baseline_plus_the_dopamine_signal_clamped_to_the_unit_interval() {
        let mut m = NeuromodulatorState::new();
        assert!(m.is_at_rest());
        assert_eq!(m, NeuromodulatorState::default());
        assert_eq!(m.modulation(Q16_ONE), Q16_ONE, "at rest, the baseline");
        assert_eq!(m.modulation(0), 0);
        assert_eq!(m.modulation(0x8000), 0x8000);
        m.dopamine_rpe = 0x4000;
        assert!(!m.is_at_rest());
        assert_eq!(m.modulation(0x8000), 0xC000, "a reward raises it");
        assert_eq!(m.modulation(Q16_ONE), Q16_ONE, "never above 1.0");
        assert_eq!(m.modulation(0xC000), Q16_ONE, "exactly 1.0 at the bound");
        assert_eq!(m.modulation(0xC001), Q16_ONE);
        m.dopamine_rpe = -0x4000;
        assert_eq!(m.modulation(0x8000), 0x4000, "a dip lowers it");
        assert_eq!(m.modulation(0x4000), 0, "exactly 0 at the bound");
        assert_eq!(
            m.modulation(0),
            0,
            "never below 0: no anti-Hebbian reversal"
        );
        assert_eq!(m.modulation(0x3FFF), 0);
        m.dopamine_rpe = i32::MAX;
        assert_eq!(
            m.modulation(i32::MAX),
            Q16_ONE,
            "the sum saturates before the clamp"
        );
        m.dopamine_rpe = i32::MIN;
        assert_eq!(m.modulation(i32::MIN), 0);
        assert_eq!(DOPAMINE_TAU_SHIFT, 14, "164 ms at 10 µs");
    }

    #[test]
    fn the_record_round_trips_through_its_sixteen_bytes() {
        let m = NeuromodulatorState {
            dopamine_rpe: -0x0001_2345,
            norepinephrine: 0x8000_0001,
            serotonin: 7,
            acetylcholine: u32::MAX,
        };
        let bytes = m.encode();
        assert_eq!(&bytes[0..4], &(-0x0001_2345i32).to_le_bytes());
        assert_eq!(&bytes[12..16], &[0xFF; 4]);
        assert_eq!(NeuromodulatorState::decode(&bytes), m);
        assert_eq!(
            NeuromodulatorState::decode(&[0; 16]),
            NeuromodulatorState::new(),
            "sixteen zero bytes are the record at rest"
        );
        for field in 0..4 {
            let mut one = NeuromodulatorState::new();
            match field {
                0 => one.dopamine_rpe = 1,
                1 => one.norepinephrine = 1,
                2 => one.serotonin = 1,
                _ => one.acetylcholine = 1,
            }
            assert!(!one.is_at_rest(), "any signal takes the record off rest");
        }
    }
}
