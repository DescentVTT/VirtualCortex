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

/// $(1 - 2^{-\text{tau\_shift}})^{\text{elapsed}}$ in Q16.16, by binary exponentiation: the
/// fraction of a deviation that survives `elapsed_ticks` of relaxation with time constant
/// $2^{\text{tau\_shift}}$ ticks. 1.0 for zero elapsed; 0 once the deviation has vanished. A
/// time constant above $2^{16}$ ticks is read as $2^{16}$, the longest a Q16.16 base resolves
/// (ADR-0028).
pub fn stp_decay_factor_q16(elapsed_ticks: u32, tau_shift: u32) -> u32 {
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
