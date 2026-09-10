//! Membrane integration for `DendriticSuperNeuron`: per-tick leak, dendritic coupling, the
//! threshold, the refractory window, BAC plateaus and threshold adaptation, all in saturating
//! Q16.16 (whitepaper §5.2.1, §6.1 step 5, §8.8; ADR-0018; brief 011).
//!
//! Potentials are relative to rest, so rest is zero and an image at rest (§8.7) is at rest.
//! Every time constant is a right shift of the fine tick (10 µs), and every decay takes at
//! least one LSB so that a potential reaches rest instead of stalling above it. The worker that
//! holds the turn (axiom A3) calls [`DendriticSuperNeuron::integrate`] once per tick with the
//! inputs that arrived; the method touches the plain fields only.

use super::neuron::DendriticSuperNeuron;

/// 1.0 in Q16.16.
pub const Q16_ONE: i32 = 0x0001_0000;

/// Somatic leak: $\tau_m = 2^{11}$ ticks ≈ 20.5 ms.
pub const SOMA_LEAK_SHIFT: u32 = 11;
/// Basal leak: $2^9$ ticks ≈ 5.1 ms.
pub const BASAL_LEAK_SHIFT: u32 = 9;
/// Apical leak: $2^{10}$ ticks ≈ 10.2 ms.
pub const APICAL_LEAK_SHIFT: u32 = 10;
/// Dendrite-to-soma coupling: a $2^{-4}$ fraction of the potential difference per tick.
pub const COUPLING_SHIFT: u32 = 4;
/// Apical coupling during a BAC plateau: a $2^{-2}$ fraction, four times the resting coupling.
pub const PLATEAU_COUPLING_SHIFT: u32 = 2;

/// The somatic potential after a spike: −0.25.
pub const V_RESET: i32 = -0x4000;
/// Refractory window after a spike: 200 ticks = 2 ms.
pub const REFRACTORY_TICKS: u16 = 200;
/// Refractory window during a plateau: 50 ticks = 0.5 ms, the burst's inter-spike interval.
pub const BURST_REFRACTORY_TICKS: u16 = 50;
/// Apical depolarisation at or above which a somatic spike starts a plateau: 0.5.
pub const BAC_APICAL_THRESHOLD: i32 = 0x8000;
/// Plateau length: 200 ticks = 2 ms.
pub const BAC_PLATEAU_TICKS: u16 = 200;

/// The threshold every unit adapts toward: 1.0.
pub const THRESHOLD_BASE: i32 = Q16_ONE;
/// Threshold increment per spike: 0.02.
pub const THRESHOLD_STEP: i32 = 0x051E;
/// Threshold adaptation decays toward the base with $\tau = 2^{12}$ ticks ≈ 41 ms.
pub const THRESHOLD_DECAY_SHIFT: u32 = 12;

/// `flags` bit: a BAC plateau is in progress and the unit is bursting.
pub const FLAG_BURST_MODE: u8 = 0x01;
/// `flags` bit: the unit is inhibitory (its fan-out weights are negative); read by the
/// runtime, not by this method.
pub const FLAG_INHIBITORY: u8 = 0x02;

/// Moves `v` toward zero by a `2^-shift` fraction of itself, and by at least one LSB, so that
/// a decay reaches rest instead of stalling at `2^shift - 1` (ADR-0016's lesson).
#[inline(always)]
fn leak(v: i32, shift: u32) -> i32 {
    if v > 0 {
        v - (v >> shift).max(1).min(v)
    } else if v < 0 {
        v + ((-(v as i64)) >> shift).max(1).min(-(v as i64)) as i32
    } else {
        0
    }
}

/// `v + delta`, widened and clamped to the `i32` range.
#[inline(always)]
fn add(v: i32, delta: i64) -> i32 {
    let sum = v as i64 + delta;
    if sum > i32::MAX as i64 {
        i32::MAX
    } else if sum < i32::MIN as i64 {
        i32::MIN
    } else {
        sum as i32
    }
}

impl DendriticSuperNeuron {
    /// One fine tick. The compartments leak and take their inputs; the soma leaks and receives a
    /// fraction of its difference to each compartment (the apical fraction is larger during a
    /// plateau); the threshold decays toward its base. In the refractory window inputs are
    /// dropped and nothing can fire. A somatic potential at or above a positive threshold fires:
    /// the spike tick is stamped, the soma resets, the refractory window starts, the threshold
    /// steps up, and if the apical compartment is at or above `BAC_APICAL_THRESHOLD` a plateau
    /// begins (or is refreshed) with the burst's shorter refractory window. A threshold at or
    /// below zero is an unconfigured unit, which never fires. Returns `true` on the tick the unit
    /// fires.
    pub fn integrate(&mut self, basal_q16: i32, apical_q16: i32, now_tick: u32) -> bool {
        if self.bac_plateau_ticks > 0 {
            self.bac_plateau_ticks -= 1;
            if self.bac_plateau_ticks == 0 {
                self.flags &= !FLAG_BURST_MODE;
            }
        }
        if self.v_thresh > THRESHOLD_BASE {
            let excess = self.v_thresh - THRESHOLD_BASE;
            self.v_thresh -= (excess >> THRESHOLD_DECAY_SHIFT).max(1);
        }

        let in_refractory = self.refractory_ticks > 0;
        if in_refractory {
            self.refractory_ticks -= 1;
        }
        let basal_in = if in_refractory { 0 } else { basal_q16 as i64 };
        let apical_in = if in_refractory { 0 } else { apical_q16 as i64 };
        self.v_basal = add(leak(self.v_basal, BASAL_LEAK_SHIFT), basal_in);
        self.v_apical = add(leak(self.v_apical, APICAL_LEAK_SHIFT), apical_in);

        let apical_shift = if self.flags & FLAG_BURST_MODE != 0 {
            PLATEAU_COUPLING_SHIFT
        } else {
            COUPLING_SHIFT
        };
        let from_basal = (self.v_basal as i64 - self.v_soma as i64) >> COUPLING_SHIFT;
        let from_apical = (self.v_apical as i64 - self.v_soma as i64) >> apical_shift;
        self.v_soma = add(leak(self.v_soma, SOMA_LEAK_SHIFT), from_basal + from_apical);

        if in_refractory || self.v_thresh <= 0 || self.v_soma < self.v_thresh {
            return false;
        }
        self.last_soma_spike_tick = now_tick;
        self.v_soma = V_RESET;
        self.v_thresh = self.v_thresh.saturating_add(THRESHOLD_STEP);
        if self.v_apical >= BAC_APICAL_THRESHOLD {
            self.bac_plateau_ticks = BAC_PLATEAU_TICKS;
            self.flags |= FLAG_BURST_MODE;
            self.refractory_ticks = BURST_REFRACTORY_TICKS;
        } else {
            self.refractory_ticks = REFRACTORY_TICKS;
        }
        true
    }

    /// Ticks since the last somatic spike at `now_tick`, as a wrapping difference (whitepaper
    /// §8.4): correct across the `u32` wrap of the tick counter.
    #[inline]
    pub const fn ticks_since_spike(&self, now_tick: u32) -> u32 {
        now_tick.wrapping_sub(self.last_soma_spike_tick)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rest_is_a_fixed_point_and_a_spike_leaves_the_soma_below_rest() {
        let mut u = unit();
        for t in 0..1_000u32 {
            assert!(!u.integrate(0, 0, t));
            assert_eq!(
                (u.v_soma, u.v_basal, u.v_apical, u.v_thresh),
                (0, 0, 0, THRESHOLD_BASE),
                "nothing moves at rest"
            );
        }
        let mut v = unit();
        let mut spiked = false;
        for t in 0..2_000u32 {
            if v.integrate(2 * THRESHOLD_BASE, 0, t) {
                spiked = true;
                break;
            }
        }
        assert!(spiked, "a strong drive fires");
        assert!(v.v_soma < 0, "the reset is below rest: {}", v.v_soma);
        assert_eq!(v.v_soma, V_RESET);
    }

    fn unit() -> DendriticSuperNeuron {
        let mut u = DendriticSuperNeuron::new(1);
        u.v_thresh = THRESHOLD_BASE;
        u
    }

    /// Runs `ticks` ticks of constant input; returns the ticks at which the unit fired.
    fn drive(u: &mut DendriticSuperNeuron, basal: i32, apical: i32, ticks: u32) -> [u32; 8] {
        let mut fired = [u32::MAX; 8];
        let mut n = 0;
        for t in 0..ticks {
            if u.integrate(basal, apical, t) && n < 8 {
                fired[n] = t;
                n += 1;
            }
        }
        fired
    }

    fn plain_fields(u: &DendriticSuperNeuron) -> (i32, i32, i32, i32, u16, u16, u32, u8) {
        (
            u.v_soma,
            u.v_basal,
            u.v_apical,
            u.v_thresh,
            u.bac_plateau_ticks,
            u.refractory_ticks,
            u.last_soma_spike_tick,
            u.flags,
        )
    }

    #[test]
    fn a_displaced_potential_leaks_to_rest_exactly_in_every_compartment_and_both_signs() {
        for start in [0x1000, -0x1000, i32::MAX, i32::MIN + 1] {
            let mut u = unit();
            u.v_soma = start;
            u.v_basal = start;
            u.v_apical = start;
            let mut settled_at = None;
            for t in 0..200_000u32 {
                u.integrate(0, 0, t);
                if (u.v_soma, u.v_basal, u.v_apical) == (0, 0, 0) {
                    settled_at = Some(t);
                    break;
                }
            }
            assert!(settled_at.is_some(), "start {start:#x} never reached rest");
            assert_eq!((u.v_soma, u.v_basal, u.v_apical), (0, 0, 0));
        }
        assert_eq!(leak(1, SOMA_LEAK_SHIFT), 0, "the last LSB goes too");
        assert_eq!(leak(-1, SOMA_LEAK_SHIFT), 0);
    }

    #[test]
    fn constant_sub_threshold_input_settles_below_threshold_and_never_fires() {
        // v_basal* = 2^9 × 128 = 1.0; the soma settles near half of it with no apical drive.
        let mut u = unit();
        let fired = drive(&mut u, 128, 0, 20_000);
        assert_eq!(fired[0], u32::MAX, "no spike");
        assert!(
            u.v_basal > 0xF000 && u.v_basal <= Q16_ONE,
            "basal at its fixed point"
        );
        assert!(
            u.v_soma > 0x6000 && u.v_soma < 0xA000,
            "soma near half of it: {:#x}",
            u.v_soma
        );
        assert!(u.v_soma < u.v_thresh);
    }

    #[test]
    fn supra_threshold_input_fires_on_a_predictable_tick_then_periodically() {
        // v_basal* = 2^9 × 512 = 4.0; the soma reaches 1.0 when the basal compartment is near
        // 2.0, about 2^9 × ln 2 ≈ 355 ticks in, plus the coupling lag.
        let mut u = unit();
        let mut rising = true;
        let mut last = 0;
        let mut first = None;
        for t in 0..1000u32 {
            let before = u.v_soma;
            if u.integrate(512, 0, t) {
                first = Some(t);
                break;
            }
            rising &= u.v_soma >= before;
            last = u.v_soma;
        }
        let first = first.expect("fires within a thousand ticks");
        assert!((300..600).contains(&first), "first spike at {first}");
        assert!(rising, "the soma rose monotonically to threshold");
        assert!(
            last >= THRESHOLD_BASE - 0x2000,
            "and was near it: {last:#x}"
        );
        assert_eq!(u.v_soma, V_RESET);
        assert_eq!(u.last_soma_spike_tick, first);
        assert_eq!(u.refractory_ticks, REFRACTORY_TICKS);
        let later = drive(&mut u, 512, 0, 2_000);
        assert!(
            later[0] >= REFRACTORY_TICKS as u32,
            "no spike inside the refractory window"
        );
        assert!(
            later[1] > later[0] && later[2] > later[1],
            "periodic firing under constant drive"
        );
    }

    #[test]
    fn inputs_during_the_refractory_window_are_dropped_and_the_window_ends_on_time() {
        let mut u = unit();
        u.v_soma = 2 * THRESHOLD_BASE;
        assert!(u.integrate(0, 0, 0), "well above threshold fires");
        for t in 1..=REFRACTORY_TICKS as u32 {
            assert!(
                !u.integrate(i32::MAX, i32::MAX, t),
                "tick {t} is refractory"
            );
            assert_eq!(u.refractory_ticks, REFRACTORY_TICKS - t as u16);
        }
        assert_eq!(u.refractory_ticks, 0);
        assert_eq!(
            (u.v_basal, u.v_apical),
            (0, 0),
            "dropped inputs left no trace"
        );
        assert!(
            u.integrate(i32::MAX, 0, REFRACTORY_TICKS as u32 + 1),
            "the next tick can fire"
        );
    }

    #[test]
    fn every_potential_saturates_at_the_extremes() {
        let mut u = unit();
        u.v_basal = i32::MAX;
        u.v_apical = i32::MAX;
        assert!(
            u.integrate(i32::MAX, i32::MAX, 0),
            "a saturated drive fires"
        );
        assert_eq!((u.v_basal, u.v_apical), (i32::MAX, i32::MAX));
        assert_eq!(u.v_soma, V_RESET, "and the soma reset rather than wrapped");
        let mut d = unit();
        d.v_basal = i32::MIN;
        d.v_apical = i32::MIN;
        d.v_soma = i32::MIN;
        assert!(!d.integrate(i32::MIN, i32::MIN, 0));
        assert_eq!((d.v_basal, d.v_apical), (i32::MIN, i32::MIN));
        assert!(d.v_soma < 0);
    }

    #[test]
    fn a_zero_or_negative_threshold_is_an_unconfigured_unit_that_never_fires() {
        let mut u = DendriticSuperNeuron::new(1);
        assert_eq!(u.v_thresh, 0);
        assert_eq!(drive(&mut u, i32::MAX, i32::MAX, 5_000)[0], u32::MAX);
        u.v_thresh = -1;
        assert_eq!(drive(&mut u, i32::MAX, i32::MAX, 5_000)[0], u32::MAX);
    }

    #[test]
    fn the_spike_stamp_is_compared_across_the_tick_wrap() {
        let mut u = unit();
        u.v_soma = 2 * THRESHOLD_BASE;
        assert!(u.integrate(0, 0, u32::MAX - 5));
        assert_eq!(u.ticks_since_spike(u32::MAX - 5), 0);
        assert_eq!(u.ticks_since_spike(10), 16, "wraps rather than underflows");
    }

    #[test]
    fn two_units_given_the_same_trace_are_identical_after_a_hundred_thousand_ticks() {
        let mut a = unit();
        let mut b = unit();
        let mut x = 0x9E37_79B9u32;
        let mut spikes = 0;
        for t in 0..100_000u32 {
            x = x.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
            let basal = ((x >> 8) & 0x3FF) as i32;
            let apical = ((x >> 20) & 0x3FF) as i32 - 0x100;
            let fa = a.integrate(basal, apical, t);
            let fb = b.integrate(basal, apical, t);
            assert_eq!(fa, fb);
            spikes += fa as u32;
            assert_eq!(plain_fields(&a), plain_fields(&b));
        }
        assert!(spikes > 0, "the trace drives spikes");
    }

    #[test]
    fn a_spike_with_apical_depolarisation_starts_a_plateau_and_a_burst() {
        let mut u = unit();
        u.v_soma = 2 * THRESHOLD_BASE;
        u.v_apical = 3 * BAC_APICAL_THRESHOLD / 2;
        assert!(u.integrate(0, 0, 0));
        assert_eq!(u.bac_plateau_ticks, BAC_PLATEAU_TICKS);
        assert_ne!(u.flags & FLAG_BURST_MODE, 0);
        assert_eq!(
            u.refractory_ticks, BURST_REFRACTORY_TICKS,
            "bursts fire closer together"
        );
        let mut quiet = unit();
        quiet.v_soma = 2 * THRESHOLD_BASE;
        quiet.v_apical = BAC_APICAL_THRESHOLD / 2;
        assert!(quiet.integrate(0, 0, 0));
        assert_eq!(quiet.bac_plateau_ticks, 0);
        assert_eq!(quiet.flags & FLAG_BURST_MODE, 0);
        assert_eq!(quiet.refractory_ticks, REFRACTORY_TICKS);
        // The plateau ends on time and clears the flag.
        for t in 1..=BAC_PLATEAU_TICKS as u32 {
            u.integrate(0, 0, t);
        }
        assert_eq!(u.bac_plateau_ticks, 0);
        assert_eq!(u.flags & FLAG_BURST_MODE, 0);
    }

    #[test]
    fn the_threshold_steps_up_per_spike_and_decays_back_to_its_base() {
        let mut u = unit();
        u.v_soma = 2 * THRESHOLD_BASE;
        assert!(u.integrate(0, 0, 0));
        assert_eq!(u.v_thresh, THRESHOLD_BASE + THRESHOLD_STEP);
        let mut t = 1;
        while u.v_thresh > THRESHOLD_BASE && t < 100_000 {
            u.integrate(0, 0, t);
            t += 1;
        }
        assert_eq!(u.v_thresh, THRESHOLD_BASE, "reaches the base exactly");
        let mut raised = unit();
        raised.v_thresh = THRESHOLD_BASE + 0x2000;
        raised.integrate(0, 0, 0);
        assert_eq!(
            raised.v_thresh,
            THRESHOLD_BASE + 0x2000 - (0x2000 >> THRESHOLD_DECAY_SHIFT),
            "one tick takes 2^-12 of the excess"
        );
        let mut raised = unit();
        raised.v_thresh = THRESHOLD_BASE + 0x2000;
        raised.integrate(0, 0, 0);
        assert_eq!(
            raised.v_thresh,
            THRESHOLD_BASE + 0x2000 - (0x2000 >> THRESHOLD_DECAY_SHIFT),
            "one tick takes 2^-12 of the excess"
        );
        let mut low = unit();
        low.v_thresh = THRESHOLD_BASE / 2;
        low.integrate(0, 0, 0);
        assert_eq!(
            low.v_thresh,
            THRESHOLD_BASE / 2,
            "a threshold below the base is left alone"
        );
    }
}

/// Property tests (ADR-0030): the invariants of integration over the lattice and a seeded walk.
#[cfg(test)]
mod prop {
    use super::*;
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../testkit/prop.rs"
    ));

    fn check(u: &DendriticSuperNeuron) {
        assert!(
            u.v_thresh >= THRESHOLD_BASE,
            "the threshold never falls below its base"
        );
        assert!(u.refractory_ticks <= REFRACTORY_TICKS);
        assert!(u.bac_plateau_ticks <= BAC_PLATEAU_TICKS);
        assert_eq!(
            u.flags & FLAG_BURST_MODE != 0,
            u.bac_plateau_ticks > 0,
            "the burst flag is the plateau's"
        );
    }

    #[test]
    fn a_million_edge_biased_ticks_keep_every_invariant_and_never_panic() {
        let mut rng = Lcg::new(0x9E37_79B9_7F4A_7C15);
        let mut u = DendriticSuperNeuron::new(1);
        u.v_thresh = THRESHOLD_BASE;
        let mut fired = 0u32;
        for t in 0..1_000_000u32 {
            let (basal, apical) = (rng.i32_edge_biased(), rng.i32_edge_biased());
            let refractory_before = u.refractory_ticks;
            let (basal_before, apical_before) = (u.v_basal, u.v_apical);
            let spiked = u.integrate(basal, apical, t);
            check(&u);
            if spiked {
                assert_eq!(refractory_before, 0, "a spike only outside the window");
                assert_eq!(u.v_soma, V_RESET);
                assert_eq!(u.last_soma_spike_tick, t);
                assert!(u.v_thresh > THRESHOLD_BASE, "the threshold stepped up");
                if u.v_apical >= BAC_APICAL_THRESHOLD {
                    assert_eq!(u.bac_plateau_ticks, BAC_PLATEAU_TICKS, "a plateau begins");
                    assert_eq!(u.refractory_ticks, BURST_REFRACTORY_TICKS);
                } else {
                    assert_eq!(u.refractory_ticks, REFRACTORY_TICKS);
                }
                fired = fired.saturating_add(1);
            } else if refractory_before > 0 {
                assert_eq!(u.refractory_ticks, refractory_before.saturating_sub(1));
                assert!(
                    u.v_basal.unsigned_abs() <= basal_before.unsigned_abs()
                        && u.v_apical.unsigned_abs() <= apical_before.unsigned_abs(),
                    "inputs are dropped in the window: the compartments only leak"
                );
            }
        }
        assert!(fired > 1_000, "the walk fires: {fired}");
    }

    #[test]
    fn every_lattice_pair_drives_a_unit_for_a_thousand_ticks_without_a_panic() {
        for &basal in I32_LATTICE.iter() {
            for &apical in I32_LATTICE.iter() {
                let mut u = DendriticSuperNeuron::new(1);
                u.v_thresh = THRESHOLD_BASE;
                for t in 0..1_000u32 {
                    u.integrate(basal, apical, t);
                    check(&u);
                }
                // Every field at an extreme: the invariants hold and nothing panics, whether
                // or not the unit fires (a threshold at the top is reachable by inputs at the top).
                let mut extreme = DendriticSuperNeuron::new(2);
                extreme.v_thresh = i32::MAX;
                extreme.v_soma = i32::MAX;
                extreme.v_basal = i32::MIN;
                extreme.v_apical = i32::MIN;
                for t in 0..64u32 {
                    extreme.integrate(basal, apical, t);
                    check(&extreme);
                }
                let mut unconfigured = DendriticSuperNeuron::new(3);
                unconfigured.v_thresh = i32::MIN;
                for t in 0..64u32 {
                    assert!(
                        !unconfigured.integrate(basal, apical, t),
                        "a non-positive threshold never fires"
                    );
                }
            }
        }
    }

    #[test]
    fn ticks_since_spike_counts_forward_across_the_wrap_and_never_backward() {
        for (last, now, expected) in [
            (0u32, 0u32, 0u32),
            (5, 5, 0),
            (5, 6, 1),
            (u32::MAX, 0, 1),
            (u32::MAX - 1, 1, 3),
            (0, u32::MAX, u32::MAX),
            (1 << 31, 0, 1 << 31),
        ] {
            let mut u = DendriticSuperNeuron::new(1);
            u.last_soma_spike_tick = last;
            assert_eq!(u.ticks_since_spike(now), expected, "{last} -> {now}");
        }
        for &last in U32_LATTICE.iter() {
            for &now in U32_LATTICE.iter() {
                let mut u = DendriticSuperNeuron::new(1);
                u.last_soma_spike_tick = last;
                let since = u.ticks_since_spike(now);
                assert_eq!(
                    last.wrapping_add(since),
                    now,
                    "the stamp plus the count is now"
                );
            }
        }
    }
}
