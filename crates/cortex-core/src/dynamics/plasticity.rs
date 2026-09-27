//! Short-term plasticity (Tsodyks–Markram) on the two Q0.8 fields of `DendriticSuperNeuron`,
//! event-driven: one call per presynaptic spike with the ticks elapsed since the previous one
//! (whitepaper §5.2.1, §8.1, §8.8; ADR-0019; brief 010).
//!
//! Between spikes the release fraction $u$ relaxes toward $U$ with $\tau_f$ and the resource $R$
//! recovers toward 1 with $\tau_d$; at a spike $u$ facilitates by $U(1-u)$, the release is
//! $u R$ of the resource, and $R$ is depleted by it. The exponential factors are
//! $(1 - 2^{-k})^{\Delta t}$ in Q16.16 by binary exponentiation, so an interval of any length
//! costs at most 32 multiplications and no table. The fields stay Q0.8; the update is computed
//! in Q16.16 and rounded back, and every relaxation toward a target takes at least one LSB, so
//! the state reaches its rest exactly instead of stalling near it.
//!
//! One set of constants is ADR-0019's and every unit steps under it by default
//! ([`DendriticSuperNeuron::step_stp`]). Since ADR-0114 the image may carry a class of its own
//! ([`StpClass`], ADR-0113), and a unit marked `FLAG_FACILITATING` steps under that class
//! ([`DendriticSuperNeuron::step_stp_class`]): the same rule at other constants, and
//! `step_stp` itself at ADR-0019's.

use super::neuron::DendriticSuperNeuron;

/// 1.0 in Q16.16.
const Q16_ONE: i64 = 0x0001_0000;
/// 1.0 in Q0.8 is not representable; 255 is the resting resource and the ceiling of both fields.
pub const STP_MAX: u8 = 255;
/// Baseline release fraction $U$: 51/256 ≈ 0.2 in Q0.8.
pub const STP_U: u8 = 51;
/// Facilitation time constant $\tau_f = 2^{14}$ ticks ≈ 164 ms.
pub const STP_TAU_F_SHIFT: u32 = 14;
/// Depression recovery time constant $\tau_d = 2^{15}$ ticks ≈ 328 ms.
pub const STP_TAU_D_SHIFT: u32 = 15;

/// A class of short-term plasticity (ADR-0113, ADR-0114): the baseline release fraction $U$ in
/// Q0.8 and the two time constants as shifts, $\tau_f = 2^{\text{tau\_f\_shift}}$ and
/// $\tau_d = 2^{\text{tau\_d\_shift}}$ ticks. The image carries one; a unit marked
/// `FLAG_FACILITATING` steps under it
/// ([`step_stp_class`](DendriticSuperNeuron::step_stp_class)) and every other unit under
/// ADR-0019's constants ([`step_stp`](DendriticSuperNeuron::step_stp)), which are
/// [`StpClass::REFERENCE`]. Not a record: its three bytes sit in the image's modulator section.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StpClass {
    /// $U$ in Q0.8, from 1 to 255.
    pub u: u8,
    /// $\log_2 \tau_f$, from 1 to 16.
    pub tau_f_shift: u8,
    /// $\log_2 \tau_d$, from 1 to 16.
    pub tau_d_shift: u8,
}

impl StpClass {
    /// ADR-0019's constants: $U = 51/256$, $\tau_f = 2^{14}$, $\tau_d = 2^{15}$ ticks.
    pub const REFERENCE: Self = Self {
        u: STP_U,
        tau_f_shift: STP_TAU_F_SHIFT as u8,
        tau_d_shift: STP_TAU_D_SHIFT as u8,
    };

    /// True for a class the rule resolves: $U$ from 1 to 255, and each shift from 1 to 16. A
    /// $U$ of zero never facilitates and never releases; a shift of zero is no time constant;
    /// and a shift above 16 is one [`stp_decay_factor_q16`] reads as 16 (ADR-0028), so a
    /// class that named it would name a time constant the rule does not run.
    pub const fn is_valid(self) -> bool {
        self.u != 0 && matches!(self.tau_f_shift, 1..=16) && matches!(self.tau_d_shift, 1..=16)
    }
}

const _: () = assert!(StpClass::REFERENCE.is_valid());
const _: () = assert!(STP_TAU_F_SHIFT <= 16 && STP_TAU_D_SHIFT <= 16);

/// $(1 - 2^{-\text{tau\_shift}})^{\text{elapsed}}$ in Q16.16, by binary exponentiation: the
/// fraction of a deviation that survives `elapsed_ticks` of relaxation with time constant
/// $2^{\text{tau\_shift}}$ ticks. 1.0 for zero elapsed; 0 once the deviation has vanished. A
/// time constant above $2^{16}$ ticks is read as $2^{16}$, the longest a Q16.16 base resolves
/// (ADR-0028).
pub fn stp_decay_factor_q16(elapsed_ticks: u32, tau_shift: u32) -> u32 {
    let mut base = Q16_ONE.saturating_sub(Q16_ONE >> tau_shift.min(16));
    let mut result = Q16_ONE;
    // Binary exponentiation over the interval's significant bits, a range: the loop ends by
    // construction, not by `exp > 0 && result > 0` (ADR-0062). A result that has reached zero
    // stays zero through the bits that remain, so the early exit it had is not a result.
    let bits = u32::BITS.saturating_sub(elapsed_ticks.leading_zeros());
    for bit in 0..bits {
        if (elapsed_ticks >> bit) & 1 == 1 {
            result = result.saturating_mul(base) >> 16;
        }
        base = base.saturating_mul(base) >> 16;
    }
    result as u32
}

/// Moves `value` (Q0.8) toward `target` by the fraction of the gap that `decay_q16` says has
/// vanished, rounded to nearest, and by at least one LSB when the gap is not zero and some
/// time has elapsed (a factor of 1.0 means no time, and nothing moves).
#[inline]
fn relax_q0_8(value: u8, target: u8, decay_q16: u32) -> u8 {
    if value == target || decay_q16 >= Q16_ONE as u32 {
        return value;
    }
    let gap = (target as i64).saturating_sub(value as i64);
    let remaining = gap
        .abs()
        .saturating_mul(decay_q16 as i64)
        .saturating_add(Q16_ONE >> 1)
        >> 16;
    let moved = gap.abs().saturating_sub(remaining).max(1);
    // The gap is not zero here (`value == target` returned above), so its sign is its direction.
    if gap.is_positive() {
        (value as i64).saturating_add(moved).min(target as i64) as u8
    } else {
        (value as i64).saturating_sub(moved).max(target as i64) as u8
    }
}

impl DendriticSuperNeuron {
    /// One presynaptic spike of this unit, `elapsed_ticks` after the previous one (the caller's
    /// `ticks_since_spike` before the new stamp; `u32::MAX` for a first spike after a long rest).
    /// Relaxes `stp_u_rel` toward `STP_U` and `stp_r_ves` toward `STP_MAX` for the interval,
    /// facilitates `u` by $U(1-u)$, and returns the pair `(u, R)` the release uses, which is the
    /// input of [`synaptic_efficacy_q16`](super::synaptic_efficacy_q16); then depletes `R` by the
    /// release $uR$. Every intermediate is Q16.16 in `i64`; the fields never leave `[0, 255]`.
    pub fn step_stp(&mut self, elapsed_ticks: u32) -> (u8, u8) {
        let f_f = stp_decay_factor_q16(elapsed_ticks, STP_TAU_F_SHIFT);
        let f_d = stp_decay_factor_q16(elapsed_ticks, STP_TAU_D_SHIFT);
        let u = relax_q0_8(self.stp_u_rel, STP_U, f_f);
        let r = relax_q0_8(self.stp_r_ves, STP_MAX, f_d);

        let facilitation = (STP_U as i64)
            .saturating_mul(256_i64.saturating_sub(u as i64))
            .saturating_add(128)
            >> 8;
        let u = (u as i64).saturating_add(facilitation).min(STP_MAX as i64) as u8;
        let released = (u as i64).saturating_mul(r as i64).saturating_add(128) >> 8;
        self.stp_u_rel = u;
        self.stp_r_ves = (r as i64).saturating_sub(released).max(0) as u8;
        (u, r)
    }

    /// [`step_stp`](Self::step_stp) under `class` (ADR-0113, ADR-0114): the same rule with the
    /// class's $U$ and time constants in place of ADR-0019's — `stp_u_rel` relaxes toward the
    /// class's $U$ with its $\tau_f$ and `stp_r_ves` toward `STP_MAX` with its $\tau_d$, $u$
    /// facilitates by $U(1-u)$, the pair `(u, R)` the release uses is returned and `R` is
    /// depleted by $uR$. `step_stp` is not touched, and at [`StpClass::REFERENCE`] this is it
    /// bit for bit. The executor calls it for a unit marked `FLAG_FACILITATING` while the
    /// image carries a class; a class the rule does not resolve (`StpClass::is_valid`) is
    /// refused before it reaches here, and stepping under one changes no field's range.
    pub fn step_stp_class(&mut self, elapsed_ticks: u32, class: StpClass) -> (u8, u8) {
        let f_f = stp_decay_factor_q16(elapsed_ticks, u32::from(class.tau_f_shift));
        let f_d = stp_decay_factor_q16(elapsed_ticks, u32::from(class.tau_d_shift));
        let u = relax_q0_8(self.stp_u_rel, class.u, f_f);
        let r = relax_q0_8(self.stp_r_ves, STP_MAX, f_d);

        let facilitation = (class.u as i64)
            .saturating_mul(256_i64.saturating_sub(u as i64))
            .saturating_add(128)
            >> 8;
        let u = (u as i64).saturating_add(facilitation).min(STP_MAX as i64) as u8;
        let released = (u as i64).saturating_mul(r as i64).saturating_add(128) >> 8;
        self.stp_u_rel = u;
        self.stp_r_ves = (r as i64).saturating_sub(released).max(0) as u8;
        (u, r)
    }
}

#[cfg(test)]
mod tests {
    use super::super::synaptic_efficacy_q16;
    use super::*;

    #[test]
    fn relaxation_rounds_to_nearest_moves_at_least_one_lsb_and_stops_at_the_target() {
        assert_eq!(
            relax_q0_8(0, 255, Q16_ONE as u32 / 2),
            127,
            "half the gap survives, rounded"
        );
        assert_eq!(relax_q0_8(255, 0, Q16_ONE as u32 / 2), 128);
        assert_eq!(
            relax_q0_8(0, 255, Q16_ONE as u32 / 4),
            191,
            "a quarter survives"
        );
        assert_eq!(
            relax_q0_8(0, 255, Q16_ONE as u32 - 1),
            1,
            "almost nothing vanishes: one LSB"
        );
        assert_eq!(
            relax_q0_8(254, 255, 1),
            255,
            "a small gap reaches the target exactly"
        );
        assert_eq!(relax_q0_8(0, 255, 0), 255, "everything vanished");
        assert_eq!(
            relax_q0_8(0, 255, Q16_ONE as u32),
            0,
            "no time, nothing moves"
        );
        assert_eq!(relax_q0_8(51, 51, 0), 51);
    }

    const TICKS_20_MS: u32 = 2_000;

    fn rested() -> DendriticSuperNeuron {
        let mut u = DendriticSuperNeuron::new(1);
        u.stp_u_rel = STP_U;
        u.stp_r_ves = STP_MAX;
        u
    }

    #[test]
    fn the_decay_factor_is_one_at_zero_elapsed_halves_near_the_time_constant_and_reaches_zero() {
        assert_eq!(stp_decay_factor_q16(0, STP_TAU_D_SHIFT), Q16_ONE as u32);
        let one_tau = stp_decay_factor_q16(1 << STP_TAU_D_SHIFT, STP_TAU_D_SHIFT);
        // (1 - 2^-15)^(2^15) ≈ e^-1 = 0.3679; the truncating products bias it slightly low.
        assert!((0x5A00..0x5E30).contains(&one_tau), "{one_tau:#x}");
        assert_eq!(stp_decay_factor_q16(u32::MAX, STP_TAU_F_SHIFT), 0);
        let a = stp_decay_factor_q16(1000, STP_TAU_F_SHIFT);
        let b = stp_decay_factor_q16(2000, STP_TAU_F_SHIFT);
        assert!(a > b && b > 0, "monotone in the interval");
    }

    #[test]
    fn a_time_constant_past_the_resolution_is_the_longest_one_and_never_a_panic() {
        let longest = stp_decay_factor_q16(1, 16);
        assert_eq!(longest, Q16_ONE as u32 - 1);
        assert_eq!(stp_decay_factor_q16(1, 17), longest);
        assert_eq!(
            stp_decay_factor_q16(1, 64),
            longest,
            "a shift past the width"
        );
        assert_eq!(stp_decay_factor_q16(1, u32::MAX), longest);
        assert_eq!(
            stp_decay_factor_q16(u32::MAX, u32::MAX),
            0,
            "and it still decays"
        );
    }

    #[test]
    fn the_first_spike_from_rest_facilitates_releases_the_full_resource_and_depletes_it() {
        let mut n = rested();
        let (u, r) = n.step_stp(u32::MAX);
        // u = U + U(1 - U): 51 + round(51 × 205 / 256) = 51 + 41 = 92; the release sees R = 255.
        assert_eq!((u, r), (92, 255));
        // R after: 255 - round(92 × 255 / 256) = 255 - 92 = 163.
        assert_eq!((n.stp_u_rel, n.stp_r_ves), (92, 163));
    }

    #[test]
    fn a_fifty_hertz_train_reaches_a_steady_state_where_depression_dominates() {
        let mut n = rested();
        let (u1, r1) = n.step_stp(u32::MAX);
        let mut last = (0, 0);
        let mut steady_at = None;
        for spike in 1..200u32 {
            let now = n.step_stp(TICKS_20_MS);
            if now == last {
                steady_at = Some(spike);
                break;
            }
            last = now;
        }
        let steady_at = steady_at.expect("reaches a fixed point");
        assert!(steady_at < 60, "within sixty spikes: {steady_at}");
        let (u_n, r_n) = last;
        assert!(u_n > u1, "facilitated: {u_n} > {u1}");
        assert!(r_n < r1, "depleted: {r_n} < {r1}");
        let w = 0x4000;
        assert!(
            synaptic_efficacy_q16(w, u_n, r_n) < synaptic_efficacy_q16(w, u1, r1),
            "the efficacy of the depleted synapse is below the rested one"
        );
    }

    #[test]
    fn silence_recovers_the_resource_fully_and_relaxes_facilitation_to_baseline_exactly() {
        let mut n = rested();
        for _ in 0..20 {
            n.step_stp(TICKS_20_MS);
        }
        assert!(n.stp_r_ves < STP_MAX && n.stp_u_rel > STP_U);
        let (u, r) = n.step_stp(1 << 20);
        assert_eq!(
            r, STP_MAX,
            "a second of silence restores the resource before the release"
        );
        assert_eq!(u, 92, "and facilitation restarts from the baseline");
        // A short rest still moves each field by at least one LSB toward its target.
        let mut near = rested();
        near.stp_r_ves = STP_MAX - 1;
        near.stp_u_rel = STP_U + 1;
        let (u, r) = near.step_stp(1);
        assert_eq!(r, STP_MAX, "one tick recovers the last LSB");
        assert_eq!(u, 92, "and u relaxed to the baseline before facilitating");
    }

    #[test]
    fn the_fields_never_leave_their_range() {
        let mut n = DendriticSuperNeuron::new(1);
        n.stp_u_rel = STP_MAX;
        n.stp_r_ves = STP_MAX;
        let (u, r) = n.step_stp(0);
        assert_eq!((u, r), (STP_MAX, STP_MAX));
        assert_eq!(
            n.stp_r_ves, 1,
            "a full release of a full resource leaves one LSB"
        );
        let mut empty = DendriticSuperNeuron::new(1);
        let (u, r) = empty.step_stp(0);
        assert_eq!(r, 0, "zero elapsed recovers nothing from an empty resource");
        assert_eq!(u, STP_U, "and u facilitates from zero to U");
        assert_eq!(empty.stp_r_ves, 0);
        for _ in 0..1000 {
            empty.step_stp(3);
            assert!(
                empty.stp_u_rel >= STP_U,
                "u never falls below the baseline once facilitated"
            );
        }
    }

    /// A class is valid with $U$ from 1 to 255 and each shift from 1 to 16, and at no edge
    /// beyond (ADR-0114); ADR-0019's constants are one.
    #[test]
    fn a_class_is_valid_from_one_to_255_and_its_shifts_from_one_to_sixteen() {
        let class = |u, tau_f_shift, tau_d_shift| StpClass {
            u,
            tau_f_shift,
            tau_d_shift,
        };
        assert_eq!(StpClass::REFERENCE, class(51, 14, 15));
        assert!(StpClass::REFERENCE.is_valid());
        for u in [1, 26, 51, 255] {
            for (f, d) in [(1, 1), (1, 16), (16, 1), (16, 16), (16, 13)] {
                assert!(class(u, f, d).is_valid(), "{u} {f} {d}");
            }
        }
        assert!(!class(0, 16, 13).is_valid(), "a U of zero");
        for bad in [0, 17, 255] {
            assert!(!class(51, bad, 13).is_valid(), "tau_f shift {bad}");
            assert!(!class(51, 16, bad).is_valid(), "tau_d shift {bad}");
        }
    }

    /// The class's step at its edges (ADR-0114), each pair and each state after it computed
    /// by hand from the rule: a first spike after any state is the class's rest, facilitated
    /// once, at $U$ of 1, 26, 51 and 255; an empty pool with no time elapsed stays empty and
    /// releases nothing; a shift of one halves a deviation a tick, and a shift of sixteen
    /// moves it by the one LSB the rule guarantees; and each time constant is its own field's.
    #[test]
    fn the_class_s_step_at_its_edges() {
        let class = |u, tau_f_shift, tau_d_shift| StpClass {
            u,
            tau_f_shift,
            tau_d_shift,
        };
        let step = |state: (u8, u8), elapsed: u32, c: StpClass| {
            let mut n = DendriticSuperNeuron::new(1);
            (n.stp_u_rel, n.stp_r_ves) = state;
            let pair = n.step_stp_class(elapsed, c);
            (pair, (n.stp_u_rel, n.stp_r_ves))
        };
        // A first spike: every state relaxes to the class's rest before it facilitates.
        assert_eq!(
            step((0, 0), u32::MAX, class(1, 16, 16)),
            ((2, 255), (2, 253))
        );
        assert_eq!(
            step((0, 0), u32::MAX, class(255, 16, 16)),
            ((255, 255), (255, 1)),
            "the ceiling"
        );
        assert_eq!(
            step((200, 3), u32::MAX, class(26, 16, 13)),
            ((49, 255), (49, 206)),
            "ADR-0113's set (ii)"
        );
        assert_eq!(
            step((7, 9), u32::MAX, class(51, 16, 13)),
            ((92, 255), (92, 163)),
            "set (i): ADR-0019's first release"
        );
        // An empty pool, no time: nothing recovers and nothing is released.
        assert_eq!(step((100, 0), 0, class(51, 16, 13)), ((131, 0), (131, 0)));
        assert_eq!(step((0, 0), 0, class(255, 1, 1)), ((255, 0), (255, 0)));
        // A shift of one: half of each deviation survives a tick, a quarter two.
        assert_eq!(step((255, 0), 1, class(51, 1, 1)), ((174, 127), (174, 41)));
        assert_eq!(step((255, 0), 2, class(51, 1, 1)), ((133, 191), (133, 92)));
        // A shift of sixteen: one tick moves each field by one LSB; 2^16 ticks by 1 - 1/e.
        assert_eq!(step((255, 0), 1, class(51, 16, 16)), ((254, 1), (254, 0)));
        assert_eq!(
            step((255, 0), 65_536, class(51, 16, 16)),
            ((152, 162), (152, 66))
        );
        // The facilitation's time constant is u's and the depression's is R's.
        assert_eq!(step((255, 0), 1, class(51, 1, 16)), ((174, 1), (174, 0)));
        assert_eq!(step((255, 0), 1, class(51, 16, 1)), ((254, 127), (254, 1)));
    }

    #[test]
    fn two_units_given_the_same_train_stay_identical() {
        let mut a = rested();
        let mut b = rested();
        let mut x = 0x2545_F491u32;
        for _ in 0..10_000 {
            x = x.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
            let elapsed = x >> 18;
            assert_eq!(a.step_stp(elapsed), b.step_stp(elapsed));
            assert_eq!((a.stp_u_rel, a.stp_r_ves), (b.stp_u_rel, b.stp_r_ves));
        }
    }
}

/// Property tests (ADR-0030): the decay factor is monotone and bounded, a release after a long
/// rest is the same from any state, the pool only depletes at a spike, and the efficacy is
/// bounded by 1.0 over the whole lattice.
#[cfg(test)]
mod prop {
    use super::super::synaptic_efficacy_q16;
    use super::*;
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../testkit/prop.rs"
    ));

    #[test]
    fn the_decay_factor_is_bounded_and_monotone_in_the_interval_and_in_the_time_constant() {
        let mut rng = Lcg::new(7);
        for _ in 0..20_000 {
            let (a, b) = (rng.u32_edge_biased(), rng.u32_edge_biased());
            let (lo, hi) = if a <= b { (a, b) } else { (b, a) };
            let tau = rng.below(20);
            let (f_lo, f_hi) = (stp_decay_factor_q16(lo, tau), stp_decay_factor_q16(hi, tau));
            assert!(f_lo <= 0x0001_0000 && f_hi <= 0x0001_0000);
            assert!(f_lo >= f_hi, "more time, less survives: {lo} {hi} {tau}");
            let slower = stp_decay_factor_q16(lo, tau.saturating_add(1));
            assert!(slower >= f_lo, "a longer time constant keeps more");
        }
        for &tau in [0u32, 1, 11, 14, 15, 16, 31, 63, u32::MAX].iter() {
            assert_eq!(stp_decay_factor_q16(0, tau), 0x0001_0000);
            assert_eq!(stp_decay_factor_q16(u32::MAX, tau), 0);
        }
    }

    /// The decay factor before ADR-0062: the same exponentiation ended by a comparison alone,
    /// `exp > 0 && result > 0`, kept here as the oracle the range form is held to.
    fn decay_factor_before_the_range(elapsed_ticks: u32, tau_shift: u32) -> u32 {
        let mut base = Q16_ONE.saturating_sub(Q16_ONE >> tau_shift.min(16));
        let mut result = Q16_ONE;
        let mut exp = elapsed_ticks;
        while exp > 0 && result > 0 {
            if exp & 1 == 1 {
                result = result.saturating_mul(base) >> 16;
            }
            base = base.saturating_mul(base) >> 16;
            exp >>= 1;
        }
        result as u32
    }

    #[test]
    fn the_decay_factor_over_a_range_of_bits_is_the_previous_form_bit_for_bit() {
        let taus = [0u32, 1, 11, 14, 15, 16, 17, 31, 63, u32::MAX];
        for &elapsed in U32_LATTICE.iter() {
            for &tau in taus.iter() {
                assert_eq!(
                    stp_decay_factor_q16(elapsed, tau),
                    decay_factor_before_the_range(elapsed, tau),
                    "{elapsed} {tau}"
                );
            }
        }
        let mut rng = Lcg::new(0x0062);
        for _ in 0..200_000 {
            let elapsed = rng.u32_edge_biased();
            let tau = if rng.below(4) == 0 {
                rng.next_u32()
            } else {
                rng.below(20)
            };
            assert_eq!(
                stp_decay_factor_q16(elapsed, tau),
                decay_factor_before_the_range(elapsed, tau),
                "{elapsed} {tau}"
            );
        }
    }

    #[test]
    fn a_spike_after_a_long_rest_releases_the_same_from_any_state_and_the_pool_only_depletes() {
        let mut rng = Lcg::new(11);
        for _ in 0..50_000 {
            let mut u = DendriticSuperNeuron::new(1);
            u.stp_u_rel = rng.next_u8();
            u.stp_r_ves = rng.next_u8();
            let (uu, r) = u.step_stp(u32::MAX);
            assert_eq!((uu, r), (92, 255), "the rested release is one value");
            assert_eq!(u.stp_r_ves, 255 - 92);
            let elapsed = rng.u32_edge_biased();
            let r_before = u.stp_r_ves;
            let (u2, r2) = u.step_stp(elapsed);
            assert!(r2 >= r_before, "recovery before the release");
            let released = ((u2 as i64 * r2 as i64) + 128) >> 8;
            assert_eq!(
                u.stp_r_ves as i64,
                (r2 as i64 - released).max(0),
                "then depletion by exactly the release"
            );
        }
    }

    /// A unit with the factors `(u, r)`.
    fn with(u: u8, r: u8) -> DendriticSuperNeuron {
        let mut n = DendriticSuperNeuron::new(1);
        n.stp_u_rel = u;
        n.stp_r_ves = r;
        n
    }

    /// ADR-0114: the class's step at ADR-0019's constants is `step_stp` bit for bit — the pair
    /// it returns and the two fields it leaves — over the lattice of factors and intervals, and
    /// over a seeded walk of trains that carries each state into the next spike.
    #[test]
    fn the_class_s_step_at_adr_0019_s_constants_is_step_stp_bit_for_bit() {
        for &u in U8_LATTICE.iter() {
            for &r in U8_LATTICE.iter() {
                for &elapsed in U32_LATTICE.iter() {
                    let (mut a, mut b) = (with(u, r), with(u, r));
                    assert_eq!(
                        a.step_stp(elapsed),
                        b.step_stp_class(elapsed, StpClass::REFERENCE),
                        "{u} {r} {elapsed}"
                    );
                    assert_eq!((a.stp_u_rel, a.stp_r_ves), (b.stp_u_rel, b.stp_r_ves));
                }
            }
        }
        let mut rng = Lcg::new(0x0114);
        for _ in 0..2_000 {
            let (u, r) = (rng.next_u8(), rng.next_u8());
            let (mut a, mut b) = (with(u, r), with(u, r));
            for _ in 0..100 {
                let elapsed = if rng.below(8) == 0 {
                    rng.u32_edge_biased()
                } else {
                    rng.below(1 << 18)
                };
                assert_eq!(
                    a.step_stp(elapsed),
                    b.step_stp_class(elapsed, StpClass::REFERENCE)
                );
                assert_eq!((a.stp_u_rel, a.stp_r_ves), (b.stp_u_rel, b.stp_r_ves));
            }
        }
    }

    /// Tsodyks–Markram in `i64`, written from the rule rather than from the step: each factor's
    /// deviation from its rest — $u$ from $U$, $R$ from `STP_MAX` — survives the interval by the
    /// factor, rounded to nearest, and is at least one LSB smaller once time has passed, so a
    /// state reaches its rest; then $u \mathrel{+}= U(1 - u)$, rounded, at most `STP_MAX`; the
    /// release is $uR$, rounded, and $R$ loses it, at least zero. Returns the pair the release
    /// uses and the two fields after it.
    fn tsodyks_markram(u: u8, r: u8, elapsed: u32, c: StpClass) -> ((u8, u8), (u8, u8)) {
        let survives = |deviation: i64, shift: u8| -> i64 {
            let f = i64::from(decay_factor_before_the_range(elapsed, u32::from(shift)));
            if deviation == 0 || f == Q16_ONE {
                return deviation;
            }
            let kept = deviation
                .abs()
                .saturating_mul(f)
                .saturating_add(Q16_ONE / 2)
                .checked_div(Q16_ONE)
                .unwrap_or(0)
                .min(deviation.abs().saturating_sub(1));
            kept.saturating_mul(deviation.signum())
        };
        let big_u = i64::from(c.u);
        let u1 = big_u.saturating_add(survives(i64::from(u).saturating_sub(big_u), c.tau_f_shift));
        let r1 = 255_i64.saturating_add(survives(i64::from(r).saturating_sub(255), c.tau_d_shift));
        let facilitation = big_u
            .saturating_mul(256_i64.saturating_sub(u1))
            .saturating_add(128)
            .checked_div(256)
            .unwrap_or(0);
        let u2 = u1.saturating_add(facilitation).min(255);
        let released = u2
            .saturating_mul(r1)
            .saturating_add(128)
            .checked_div(256)
            .unwrap_or(0);
        let r2 = r1.saturating_sub(released).max(0);
        ((u2 as u8, r1 as u8), (u2 as u8, r2 as u8))
    }

    /// ADR-0114: at any constants the rule resolves, the class's step is the `i64` oracle —
    /// over the lattice of factors and intervals at the edges of each constant ($U$ of 1, 2,
    /// 26, 51, 254 and 255; shifts of 1, 2, 13, 15 and 16, in every pairing), and over 400 000
    /// seeded draws of a class, a state and an interval.
    #[test]
    fn the_class_s_step_is_tsodyks_markram_at_any_constants_by_an_i64_oracle() {
        let shifts = [1u8, 2, 13, 15, 16];
        for &big_u in [1u8, 2, 26, 51, 254, 255].iter() {
            for &f in shifts.iter() {
                for &d in shifts.iter() {
                    let c = StpClass {
                        u: big_u,
                        tau_f_shift: f,
                        tau_d_shift: d,
                    };
                    for &u in U8_LATTICE.iter() {
                        for &r in U8_LATTICE.iter() {
                            for &elapsed in U32_LATTICE.iter() {
                                let mut n = with(u, r);
                                let pair = n.step_stp_class(elapsed, c);
                                assert_eq!(
                                    (pair, (n.stp_u_rel, n.stp_r_ves)),
                                    tsodyks_markram(u, r, elapsed, c),
                                    "{c:?} {u} {r} {elapsed}"
                                );
                            }
                        }
                    }
                }
            }
        }
        let mut rng = Lcg::new(0x0113);
        for _ in 0..400_000 {
            let c = StpClass {
                u: rng.below(255).saturating_add(1) as u8,
                tau_f_shift: rng.below(16).saturating_add(1) as u8,
                tau_d_shift: rng.below(16).saturating_add(1) as u8,
            };
            assert!(c.is_valid());
            let (u, r) = (rng.next_u8(), rng.next_u8());
            let elapsed = if rng.below(4) == 0 {
                rng.u32_edge_biased()
            } else {
                rng.below(1 << 20)
            };
            let mut n = with(u, r);
            let pair = n.step_stp_class(elapsed, c);
            assert_eq!(
                (pair, (n.stp_u_rel, n.stp_r_ves)),
                tsodyks_markram(u, r, elapsed, c),
                "{c:?} {u} {r} {elapsed}"
            );
        }
    }

    #[test]
    fn the_efficacy_is_bounded_by_one_over_the_lattice_and_a_walk() {
        for &w in I16_LATTICE.iter() {
            for &uu in U8_LATTICE.iter() {
                for &r in U8_LATTICE.iter() {
                    let e = synaptic_efficacy_q16(w, uu, r);
                    assert!(
                        (-0x0001_0000..=0x0001_0000).contains(&e),
                        "{w} {uu} {r} -> {e}"
                    );
                }
            }
        }
        let mut rng = Lcg::new(13);
        for _ in 0..200_000 {
            let e = synaptic_efficacy_q16(rng.next_i16(), rng.next_u8(), rng.next_u8());
            assert!((-0x0001_0000..=0x0001_0000).contains(&e));
        }
    }

    /// The whole domain, $2^{32}$ inputs: run with `cargo test --release -p cortex-core --
    /// --ignored exhaustive` (a few seconds in release; ADR-0030).
    #[test]
    #[ignore]
    fn exhaustive_the_efficacy_is_bounded_by_one_for_every_input() {
        for w in i16::MIN..=i16::MAX {
            for uu in u8::MIN..=u8::MAX {
                for r in u8::MIN..=u8::MAX {
                    let e = synaptic_efficacy_q16(w, uu, r);
                    assert!(
                        (-0x0001_0000..=0x0001_0000).contains(&e),
                        "{w} {uu} {r} -> {e}"
                    );
                }
            }
        }
    }
}
