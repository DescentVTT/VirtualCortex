//! Neuromodulation: the 16-byte modulator vector, one per macro-column (whitepaper §5.2.14),
//! the reward that moves its dopamine (ADR-0027), and the modulation of three-factor
//! plasticity that the dopamine signal sets (ADR-0032): the fraction of a synapse's
//! eligibility trace that `cortex-core` consolidates into the weight at a presynaptic spike.
//! The record's bytes for the image and the per-tick decay the executor applies are here too,
//! and the rule of the critic that forms the prediction error the dopamine signal receives
//! (ADR-0130, ADR-0131): the value, the error and the step over the units' value weights and
//! their spikes since the previous reward, which the executor composes.

#![no_std]
// §8.1: an operation on a state field saturates or wraps by name; plain arithmetic is refused
// here (ADR-0029; migrated under brief 016 on 2026-09-10).
#![deny(clippy::arithmetic_side_effects)]

/// 1.0 in Q16.16: the modulation above which nothing more of a trace can be consolidated.
const Q16_ONE: i32 = 0x0001_0000;
/// −1.0 in Q16.16: the signed modulation below which nothing more of a trace can be moved
/// against its sign (ADR-0094). A literal; the assertion ties it to 1.0.
const Q16_MINUS_ONE: i32 = -0x0001_0000;
const _: () = assert!(Q16_MINUS_ONE == -Q16_ONE);

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

    /// The signed modulation of the signed gate (ADR-0093, ADR-0094): `baseline + dopamine`,
    /// saturating, clamped to $[-1, 1]$ in Q16.16 rather than to $[0, 1]$. At or above zero it
    /// is [`modulation`](Self::modulation); below zero it is the fraction of an addressed
    /// excitatory synapse's trace that `SynapseBlock::consolidate_signed` moves into the
    /// weight **against** the trace's sign — the anti-Hebbian reversal `modulation` clamps
    /// away, as in the signed reward-modulated rule of Florian 2007 and Frémaux, Sprekeler and
    /// Gerstner 2010. The executor reads it only for an addressed excitatory synapse while the
    /// signed gate is set.
    pub fn signed_modulation(&self, baseline_q16: i32) -> i32 {
        baseline_q16
            .saturating_add(self.dopamine_rpe)
            .clamp(Q16_MINUS_ONE, Q16_ONE)
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

/// The longest step shift the critic's rule resolves (ADR-0131): at 31 or more one spike under
/// any error of the `i32` lattice moves a weight by nothing or by the floor's one LSB down, the
/// error's sign and not a step.
pub const CRITIC_SHIFT_MAX: u8 = 30;
/// The widest scale the critic's rule resolves (ADR-0131): at 15 or more one spike at the
/// weight's positive rail, `i16::MAX`, predicts less than one LSB of reward.
pub const CRITIC_SCALE_MAX: u8 = 14;

/// The critic of the engine's own (ADR-0130, ADR-0131): a value weight on every unit, the
/// value of a reward the weights times each unit's spikes since the previous reward, the
/// prediction error the reward less the value, and each weight moved by the error times its
/// unit's spikes — the delta rule of a linear critic (Sutton and Barto 2018). The rule is pure:
/// the weights live in the units' records and the counts in the executor, which composes it.
///
/// The widths, written first: a weight is an `i16` whose one LSB predicts $2^{-\text{scale}}$
/// of a Q16.16 LSB of reward per spike; a count is a `u32`, saturating; the value sums
/// $w_i c_i$ in `i64` — each product within $2^{15} \cdot 2^{32} = 2^{47}$ — saturating, takes
/// the floor over $2^{\text{scale}}$ and is clamped to the `i32` of a Q16.16 reward; the step
/// is $\lfloor \delta c_i / 2^{\text{shift}} \rfloor$, the product within
/// $2^{31} \cdot 2^{32} = 2^{63}$ in `i64`, and the weight after it is clamped to the `i16`.
/// Not a record: the constants are a parameter of the image.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ValueCritic {
    /// The step's shift $k$: a weight moves by the floor of $\delta c / 2^k$.
    pub shift: u8,
    /// The weight's scale $s$: the value is the floor of $\sum_i w_i c_i / 2^s$, Q16.16.
    pub scale: u8,
}

impl ValueCritic {
    /// Constants the rule resolves: a shift of at most `CRITIC_SHIFT_MAX` and a scale of at
    /// most `CRITIC_SCALE_MAX`.
    pub const fn is_valid(&self) -> bool {
        self.shift <= CRITIC_SHIFT_MAX && self.scale <= CRITIC_SCALE_MAX
    }

    /// The value, Q16.16: the floor of $\sum_i w_i c_i / 2^{\text{scale}}$ over the
    /// `(weight, count)` pairs, the sum in `i64` saturating, clamped to the `i32`. A scale above
    /// `CRITIC_SCALE_MAX` is taken as it, as the configuration and the loader refuse it.
    pub fn value_q16<I: IntoIterator<Item = (i16, u32)>>(&self, features: I) -> i32 {
        let sum = features.into_iter().fold(0i64, |sum, (weight, count)| {
            sum.saturating_add(i64::from(weight).saturating_mul(i64::from(count)))
        });
        (sum >> self.scale.min(CRITIC_SCALE_MAX)).clamp(i64::from(i32::MIN), i64::from(i32::MAX))
            as i32
    }

    /// The prediction error of a reward against the value: the reward less the value,
    /// saturating. It is what the modulator receives in the reward's place.
    pub const fn error_q16(reward_q16: i32, value_q16: i32) -> i32 {
        reward_q16.saturating_sub(value_q16)
    }

    /// A weight after the step of `error_q16` on a unit that fired `count` times since the
    /// previous reward: the weight plus the floor of $\delta c / 2^{\text{shift}}$, clamped to
    /// the `i16`. A count of zero moves nothing; a positive error moves a weight up by at least
    /// nothing, a negative one down by at least one LSB. A shift above `CRITIC_SHIFT_MAX` is
    /// taken as it.
    pub fn step(&self, weight: i16, error_q16: i32, count: u32) -> i16 {
        let step = i64::from(error_q16).saturating_mul(i64::from(count))
            >> self.shift.min(CRITIC_SHIFT_MAX);
        i64::from(weight)
            .saturating_add(step)
            .clamp(i64::from(i16::MIN), i64::from(i16::MAX)) as i16
    }
}

const _: () = {
    assert!(core::mem::size_of::<NeuromodulatorState>() == 16);
    assert!(core::mem::align_of::<NeuromodulatorState>() == 16);
    // One spike at the positive rail predicts one LSB at the widest scale and none beyond it,
    // and the largest positive error on one spike moves a weight by one LSB at the longest
    // shift and by none beyond it.
    assert!(i16::MAX as i64 >> CRITIC_SCALE_MAX == 1);
    assert!(i16::MAX as i64 >> (CRITIC_SCALE_MAX + 1) == 0);
    assert!(i32::MAX as i64 >> CRITIC_SHIFT_MAX == 1);
    assert!(i32::MAX as i64 >> (CRITIC_SHIFT_MAX + 1) == 0);
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

    /// The signed modulation (ADR-0094) at its edges: the modulation at and above zero, one
    /// LSB below zero one LSB below it, exactly −1.0 at the bound and never below it, the sum
    /// saturating before the clamp; and at the fixed points of a punishment after every trial
    /// (ADR-0093), −1.0 after the punishment and −0.712 at a trial's end.
    #[test]
    fn the_signed_modulation_is_the_modulation_above_zero_and_reaches_minus_one_below_it() {
        let mut m = NeuromodulatorState::new();
        assert_eq!(
            m.signed_modulation(Q16_ONE),
            Q16_ONE,
            "at rest, the baseline"
        );
        assert_eq!(m.signed_modulation(0), 0);
        m.dopamine_rpe = 0x4000;
        assert_eq!(m.signed_modulation(0x8000), 0xC000, "a reward raises it");
        assert_eq!(m.signed_modulation(0xC001), Q16_ONE, "never above 1.0");
        m.dopamine_rpe = -0x4000;
        assert_eq!(m.signed_modulation(0x8000), 0x4000, "a dip lowers it");
        assert_eq!(m.signed_modulation(0x4000), 0, "exactly 0 at zero");
        assert_eq!(m.signed_modulation(0x3FFF), -1, "one LSB below zero");
        assert_eq!(m.modulation(0x3FFF), 0, "where the modulation is clamped");
        assert_eq!(m.signed_modulation(0), -0x4000);
        m.dopamine_rpe = -Q16_ONE;
        assert_eq!(
            m.signed_modulation(0),
            Q16_MINUS_ONE,
            "exactly −1.0 at the bound"
        );
        assert_eq!(m.signed_modulation(1), Q16_MINUS_ONE + 1);
        m.dopamine_rpe = -Q16_ONE - 1;
        assert_eq!(m.signed_modulation(0), Q16_MINUS_ONE, "never below −1.0");
        m.dopamine_rpe = i32::MIN;
        assert_eq!(
            m.signed_modulation(i32::MIN),
            Q16_MINUS_ONE,
            "the sum saturates before the clamp"
        );
        m.dopamine_rpe = i32::MAX;
        assert_eq!(m.signed_modulation(i32::MAX), Q16_ONE);
        // The gate's baseline, zero, at the fixed points of a punishment after every trial.
        m.dopamine_rpe = -112_227;
        assert_eq!(
            m.signed_modulation(0),
            Q16_MINUS_ONE,
            "after the punishment"
        );
        m.dopamine_rpe = -46_691;
        assert_eq!(m.signed_modulation(0), -46_691, "at a trial's end");
        assert_eq!(m.modulation(0), 0, "where the gate's modulation is zero");
    }

    /// The critic's constants (ADR-0131) at their bounds on both sides.
    #[test]
    fn the_critic_s_constants_are_valid_within_what_the_rule_resolves() {
        let critic = |shift, scale| ValueCritic { shift, scale };
        assert_eq!((CRITIC_SHIFT_MAX, CRITIC_SCALE_MAX), (30, 14));
        for (shift, scale) in [(0, 0), (30, 0), (0, 14), (30, 14), (9, 2)] {
            assert!(critic(shift, scale).is_valid(), "{shift} {scale}");
        }
        for (shift, scale) in [(31, 0), (0, 15), (31, 15), (u8::MAX, 2), (9, u8::MAX)] {
            assert!(!critic(shift, scale).is_valid(), "{shift} {scale}");
        }
    }

    /// The value (ADR-0131), each number worked by hand: the weights times the counts, summed,
    /// over $2^{\text{scale}}$ and floored; a unit that did not fire adds nothing whatever its
    /// weight; the sum saturates in `i64` and the value at the `i32`; a scale past the bound is
    /// the bound.
    #[test]
    fn the_value_is_the_weights_times_the_counts_over_the_scale_floored_and_clamped() {
        let at = |scale| ValueCritic { shift: 9, scale };
        assert_eq!(at(0).value_q16([]), 0, "no unit, no value");
        let pairs = [(4, 3), (-2, 5)];
        assert_eq!(at(0).value_q16(pairs), 2, "12 less 10");
        assert_eq!(at(1).value_q16(pairs), 1);
        assert_eq!(at(2).value_q16(pairs), 0, "a half, floored");
        assert_eq!(
            at(1).value_q16([(-3, 1)]),
            -2,
            "minus one and a half, floored"
        );
        assert_eq!(
            at(0).value_q16([(i16::MAX, 0), (i16::MIN, 0)]),
            0,
            "no spike"
        );
        assert_eq!(at(2).value_q16([(1_285, 51)]), 16_383, "65 535 over four");
        assert_eq!(at(0).value_q16([(i16::MAX, u32::MAX)]), i32::MAX);
        assert_eq!(at(0).value_q16([(i16::MIN, u32::MAX)]), i32::MIN);
        // 2^16 products of 2^47 each reach the `i64` rail, which the sum holds.
        let many = core::iter::repeat_n((i16::MAX, u32::MAX), 1 << 17);
        assert_eq!(at(14).value_q16(many.clone()), i32::MAX);
        let back = many.chain(core::iter::once((i16::MIN, u32::MAX)));
        assert_eq!(at(14).value_q16(back), i32::MAX, "saturated, not wrapped");
        let wide = [(i16::MAX, 3), (-7, 11)];
        assert_eq!(at(14).value_q16(wide), 5, "98 224 over 16 384, floored");
        assert_eq!(at(u8::MAX).value_q16(wide), at(14).value_q16(wide));
        assert_eq!(at(15).value_q16(wide), at(14).value_q16(wide));
    }

    /// The error and the step (ADR-0131), each number worked by hand: the error saturates; a
    /// unit that did not fire keeps its weight under any error; the step is the floor of the
    /// error times the count over $2^{\text{shift}}$, so a positive error below one step moves
    /// nothing and a negative one moves one LSB down; the weight saturates at its rails; a shift
    /// past the bound is the bound.
    #[test]
    fn a_zero_feature_moves_nothing_and_the_step_has_the_error_s_sign_and_its_floor() {
        assert_eq!(ValueCritic::error_q16(0x1_0000, 0x4000), 0xC000);
        assert_eq!(ValueCritic::error_q16(-0x1_0000, 0x4000), -0x1_4000);
        assert_eq!(ValueCritic::error_q16(i32::MIN, 1), i32::MIN, "saturates");
        assert_eq!(ValueCritic::error_q16(i32::MAX, -1), i32::MAX);
        let at = |shift| ValueCritic { shift, scale: 2 };
        for error in [i32::MIN, -1, 0, 1, i32::MAX] {
            for w in [i16::MIN, -1, 0, 1, i16::MAX] {
                assert_eq!(at(0).step(w, error, 0), w, "{w} {error}: no spike");
                assert_eq!(at(30).step(w, error, 0), w);
            }
        }
        assert_eq!(at(9).step(0, 512, 1), 1);
        assert_eq!(at(9).step(0, 511, 1), 0, "below one step, nothing");
        assert_eq!(at(9).step(0, 0, 1), 0);
        assert_eq!(at(9).step(0, -1, 1), -1, "the floor, one LSB down");
        assert_eq!(at(9).step(0, -512, 1), -1);
        assert_eq!(at(9).step(0, -513, 1), -2);
        assert_eq!(at(9).step(0, 512, 3), 3, "the count multiplies");
        assert_eq!(at(9).step(0, 100, 6), 1, "600 over 512");
        assert_eq!(at(9).step(-100, 1_024, 2), -96);
        assert_eq!(at(0).step(i16::MAX, 1 << 20, 1), i16::MAX, "the rail");
        assert_eq!(at(0).step(i16::MIN, -(1 << 20), 1), i16::MIN);
        assert_eq!(at(1).step(i16::MAX - 1, 2, 1), i16::MAX);
        assert_eq!(at(1).step(i16::MAX - 1, 4, 1), i16::MAX);
        assert_eq!(at(1).step(i16::MIN + 1, -3, 1), i16::MIN);
        assert_eq!(at(30).step(0, i32::MAX, 1), 1);
        assert_eq!(at(30).step(0, i32::MIN, 1), -2);
        assert_eq!(at(30).step(0, i32::MIN, u32::MAX), i16::MIN, "in i64");
        assert_eq!(at(0).step(0, i32::MAX, u32::MAX), i16::MAX);
        assert_eq!(at(u8::MAX).step(0, i32::MAX, 1), 1, "the bound");
        assert_eq!(at(31).step(0, -1_000, 1), at(30).step(0, -1_000, 1));
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

/// The lattice property (ADR-0030) of the signed modulation (ADR-0094): over the lattice's
/// pairs and a seeded walk of baselines and signals, it is `baseline + dopamine` clamped to
/// $[-1, 1]$ by an oracle in `i64`, and the modulation is it at no less than zero — the two
/// rules one number at and above zero.
#[cfg(test)]
mod prop {
    use super::*;
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../testkit/prop.rs"
    ));

    #[test]
    fn the_signed_modulation_is_the_sum_clamped_to_plus_and_minus_one_and_the_modulation_its_floor_at_zero()
     {
        let check = |baseline: i32, dopamine: i32| {
            let m = NeuromodulatorState {
                dopamine_rpe: dopamine,
                ..NeuromodulatorState::new()
            };
            let signed = m.signed_modulation(baseline);
            let oracle = i64::from(baseline)
                .saturating_add(i64::from(dopamine))
                .clamp(i64::from(i32::MIN), i64::from(i32::MAX))
                .clamp(i64::from(Q16_MINUS_ONE), i64::from(Q16_ONE));
            assert_eq!(i64::from(signed), oracle, "{baseline} {dopamine}");
            assert_eq!(
                m.modulation(baseline),
                signed.max(0),
                "{baseline} {dopamine}: the modulation is the signed one's floor at zero"
            );
        };
        for &baseline in I32_LATTICE.iter() {
            for &dopamine in I32_LATTICE.iter() {
                check(baseline, dopamine);
            }
        }
        let mut rng = Lcg::new(0x94);
        for _ in 0..100_000 {
            check(rng.i32_edge_biased(), rng.i32_edge_biased());
        }
    }

    /// The critic's step over the lattice (ADR-0131): for every weight, error and count of the
    /// lattices and every shift the rule resolves, the step is an `i128` oracle's — the weight
    /// plus the error times the count, floored over $2^{\text{shift}}$ by `div_euclid`, clamped
    /// to the `i16` — written from the rule's text and not from `step`.
    #[test]
    fn the_critic_s_step_is_the_error_times_the_count_floored_over_the_shift_at_every_lattice_point()
     {
        for shift in 0..=CRITIC_SHIFT_MAX {
            let critic = ValueCritic { shift, scale: 0 };
            let divisor = 1i128 << shift;
            for &weight in I16_LATTICE.iter() {
                for &error in I32_LATTICE.iter() {
                    for &count in U32_LATTICE.iter() {
                        let oracle = (i128::from(weight)
                            + (i128::from(error) * i128::from(count)).div_euclid(divisor))
                        .clamp(i128::from(i16::MIN), i128::from(i16::MAX));
                        assert_eq!(
                            i128::from(critic.step(weight, error, count)),
                            oracle,
                            "{shift} {weight} {error} {count}"
                        );
                    }
                }
            }
        }
    }

    /// The critic's value and its step together over seeded draws (ADR-0131): the value is an
    /// `i128` oracle's — the weights times the counts summed exactly, floored over
    /// $2^{\text{scale}}$, clamped to the `i32` — and after every unit steps under the error of a
    /// reward against it, the value of the same features moves with the error's sign or not at
    /// all: the delta rule moves the value toward the reward.
    #[test]
    fn the_critic_s_value_is_the_exact_sum_floored_and_a_step_moves_it_with_the_error_s_sign() {
        let mut rng = Lcg::new(0x131);
        for draw in 0..20_000 {
            let critic = ValueCritic {
                shift: rng.below(u32::from(CRITIC_SHIFT_MAX) + 1) as u8,
                scale: rng.below(u32::from(CRITIC_SCALE_MAX) + 1) as u8,
            };
            let units = rng.below(9) as usize;
            let mut weights = [0i16; 8];
            let mut counts = [0u32; 8];
            for k in 0..units {
                weights[k] = if rng.below(2) == 0 {
                    rng.pick(&I16_LATTICE)
                } else {
                    rng.next_i16()
                };
                counts[k] = if rng.below(2) == 0 {
                    rng.below(4)
                } else {
                    rng.u32_edge_biased()
                };
            }
            let pairs = || {
                weights[..units]
                    .iter()
                    .copied()
                    .zip(counts[..units].iter().copied())
            };
            let exact: i128 = pairs().map(|(w, c)| i128::from(w) * i128::from(c)).sum();
            let oracle = exact
                .div_euclid(1i128 << critic.scale)
                .clamp(i128::from(i32::MIN), i128::from(i32::MAX));
            let value = critic.value_q16(pairs());
            assert_eq!(i128::from(value), oracle, "draw {draw}");
            let reward = rng.i32_edge_biased();
            let error = ValueCritic::error_q16(reward, value);
            let mut after = weights;
            for k in 0..units {
                after[k] = critic.step(weights[k], error, counts[k]);
            }
            let moved = critic.value_q16(
                after[..units]
                    .iter()
                    .copied()
                    .zip(counts[..units].iter().copied()),
            );
            assert!(
                (error >= 0 && moved >= value) || (error < 0 && moved <= value),
                "draw {draw}: {value} to {moved} under {error}"
            );
        }
    }
}
