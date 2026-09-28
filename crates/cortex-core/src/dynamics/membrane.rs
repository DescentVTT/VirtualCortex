//! Membrane integration for `DendriticSuperNeuron`: per-tick leak, dendritic coupling, the
//! threshold, the refractory window, BAC plateaus and threshold adaptation, all in saturating
//! Q16.16 (whitepaper §5.2.1, §6.1 step 5, §8.8; ADR-0018; brief 011).
//!
//! Potentials are relative to rest, so rest is zero and an image at rest (§8.7) is at rest.
//! Every time constant is a right shift of the fine tick (10 µs), and every decay takes at
//! least one LSB so that a potential reaches rest instead of stalling above it. The worker that
//! holds the turn (axiom A3) calls [`DendriticSuperNeuron::integrate`] once per tick with the
//! inputs that arrived; the method touches the plain fields only.
//!
//! Since ADR-0123 the image may carry the constants of a slow current ([`SlowCurrent`],
//! ADR-0122), and a unit marked [`FLAG_SLOW`] integrates under
//! [`DendriticSuperNeuron::integrate_slow`] instead: `integrate`'s rule with a slow potential
//! that integrates the unit's excitatory synaptic input and reaches the soma through a gate on
//! the soma's own voltage. `integrate` itself is untouched.

use super::neuron::DendriticSuperNeuron;
use super::synapse::NO_SPIKE_ON_RECORD;

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
/// `flags` bit: the unit is facilitating (ADR-0113, ADR-0114): its synapses release under the
/// image's class of short-term plasticity (`DendriticSuperNeuron::step_stp_class`) rather than
/// ADR-0019's constants; read by the runtime, not by this method, which leaves it as it finds
/// it.
pub const FLAG_FACILITATING: u8 = 0x04;
/// `flags` bit: the unit carries the slow current (ADR-0122, ADR-0123): it integrates under
/// [`DendriticSuperNeuron::integrate_slow`] with the image's constants rather than under
/// [`DendriticSuperNeuron::integrate`]; read by the runtime, not by either method, which leave
/// it as they find it.
pub const FLAG_SLOW: u8 = 0x08;

/// The slow current's constants (ADR-0122, ADR-0123): the slow potential's leak and input
/// shifts, and the two voltages of its gate. The image carries one; a unit marked [`FLAG_SLOW`]
/// integrates under it ([`integrate_slow`](DendriticSuperNeuron::integrate_slow)) and every
/// other unit under [`integrate`](DendriticSuperNeuron::integrate). Not a record: its bytes sit
/// in the image's modulator section.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SlowCurrent {
    /// $\log_2 \tau_s$: the slow potential leaks by $2^{-k_s}$ of itself a tick, from 1 to 16.
    pub leak_shift: u8,
    /// $g$: the slow potential takes $2^{-g}$ of its input, from 0 to 16.
    pub input_shift: u8,
    /// $V_{lo}$, Q16.16: at or below it the gate is shut.
    pub v_lo_q16: i32,
    /// $V_{hi}$, Q16.16: at or above it the gate is open.
    pub v_hi_q16: i32,
}

/// The slow current reaches the soma by the coupling's own fraction, $2^{-4}$, of the gate's
/// opening in Q16.16 times the slow potential: one shift of both.
const SLOW_CURRENT_SHIFT: u32 = 20;

const _: () = assert!(SLOW_CURRENT_SHIFT == 16 + COUPLING_SHIFT);

impl SlowCurrent {
    /// True for constants the rule resolves:
    /// - a leak shift from 1 to 16: zero is no time constant, and above 16 a slow potential
    ///   below 1.0 leaks by one LSB a tick whatever its size, a line rather than a time
    ///   constant;
    /// - an input shift from 0 to 16: above it a message of at most 1.0 gives the slow
    ///   potential less than one LSB;
    /// - $0 < V_{lo} < V_{hi} \le$ `THRESHOLD_BASE`, each within the threshold's base.
    pub const fn is_valid(self) -> bool {
        matches!(self.leak_shift, 1..=16)
            && self.input_shift <= 16
            && self.v_lo_q16 > 0
            && self.v_lo_q16 < self.v_hi_q16
            && self.v_hi_q16 <= THRESHOLD_BASE
    }

    /// The gate's opening at the somatic potential `v_soma`, Q16.16 in $[0, 1]$:
    /// $\operatorname{clamp}\bigl((v - V_{lo}) / (V_{hi} - V_{lo}), 0, 1\bigr)$, floored. The
    /// distance above $V_{lo}$ is clamped to the gate's span before the division, so the edges
    /// need no comparison of their own.
    pub fn gate_q16(self, v_soma: i32) -> i32 {
        let span = i64::from(self.v_hi_q16).saturating_sub(i64::from(self.v_lo_q16));
        let above = i64::from(v_soma)
            .saturating_sub(i64::from(self.v_lo_q16))
            .max(0)
            .min(span);
        // `span` is above zero for constants the rule resolves (`is_valid`).
        (above << 16).checked_div(span).unwrap_or(0) as i32
    }

    /// What the slow potential `v_slow` gives the soma at `v_soma` in one tick, Q16.16:
    /// $\gamma(v) \cdot s \cdot 2^{-4}$, floored, and nothing while $s \le 0$.
    pub fn current_q16(self, v_soma: i32, v_slow: i32) -> i64 {
        i64::from(self.gate_q16(v_soma)).saturating_mul(i64::from(v_slow).max(0))
            >> SLOW_CURRENT_SHIFT
    }
}

/// The ticks within which a spike is a descendant of the last synapse's message that reached
/// the unit (ADR-0054): 128 ticks, 1.28 ms, the latency the oracle of ADR-0044 attributes a
/// descendant within (a message's effect on the soma peaks within a few coupling steps of
/// the basal time constant's start). The in-loop count is the oracle's first-generation
/// rule without its counterfactual.
pub const CAUSAL_LATENCY_TICKS: u32 = 128;

/// Moves `v` toward zero by a `2^-shift` fraction of itself, and by at least one LSB, so that
/// a decay reaches rest instead of stalling at `2^shift - 1` (ADR-0016's lesson). The step is
/// clamped to `|v|`, so the move never crosses zero and the saturating form never saturates.
#[inline(always)]
fn leak(v: i32, shift: u32) -> i32 {
    if v > 0 {
        v.saturating_sub((v >> shift).max(1).min(v))
    } else if v < 0 {
        let magnitude = v.unsigned_abs() as i64;
        v.saturating_add((magnitude >> shift).max(1).min(magnitude) as i32)
    } else {
        0
    }
}

/// `v + delta`, widened and clamped to the `i32` range.
#[inline(always)]
fn add(v: i32, delta: i64) -> i32 {
    let sum = (v as i64).saturating_add(delta);
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
        // A plateau that is running counts down; `checked_sub` is `None` exactly at zero, so no
        // comparison exists for a mutant to move off the bound.
        if let Some(left) = self.bac_plateau_ticks.checked_sub(1) {
            self.bac_plateau_ticks = left;
            if left == 0 {
                self.flags &= !FLAG_BURST_MODE;
            }
        }
        if self.v_thresh > THRESHOLD_BASE {
            let excess = self.v_thresh.saturating_sub(THRESHOLD_BASE);
            self.v_thresh = self
                .v_thresh
                .saturating_sub((excess >> THRESHOLD_DECAY_SHIFT).max(1));
        }

        let in_refractory = self.refractory_ticks > 0;
        if in_refractory {
            self.refractory_ticks = self.refractory_ticks.saturating_sub(1);
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
        let from_basal = (self.v_basal as i64).saturating_sub(self.v_soma as i64) >> COUPLING_SHIFT;
        let from_apical = (self.v_apical as i64).saturating_sub(self.v_soma as i64) >> apical_shift;
        self.v_soma = add(
            leak(self.v_soma, SOMA_LEAK_SHIFT),
            from_basal.saturating_add(from_apical),
        );

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

    /// [`integrate`](Self::integrate) with the slow current (ADR-0122, ADR-0123), for a unit
    /// marked [`FLAG_SLOW`]; `integrate` is not touched. Before the soma's update the slow
    /// potential `v_slow` leaks by $2^{-k_s}$ of itself and takes $2^{-g}$ of `slow_q16`, the
    /// positive efficacies of the synaptic messages that landed in the basal compartment,
    /// saturating; it takes its input in the refractory window too, since the channel's opening
    /// is its input's and the soma's refractoriness is the soma's. The soma then receives, beside
    /// its coupling to the compartments, $\gamma(v) \cdot s \cdot 2^{-4}$, the gate read on the
    /// soma as the tick found it ([`SlowCurrent::current_q16`]). Everything else is
    /// `integrate`'s, line for line: with the slow potential at zero and no slow input this is
    /// `integrate` bit for bit, which a property test holds over the lattice. The executor calls
    /// it for a marked unit while the image carries the constants; constants the rule does not
    /// resolve (`SlowCurrent::is_valid`) are refused before they reach here.
    pub fn integrate_slow(
        &mut self,
        basal_q16: i32,
        apical_q16: i32,
        slow_q16: i32,
        now_tick: u32,
        current: SlowCurrent,
    ) -> bool {
        if let Some(left) = self.bac_plateau_ticks.checked_sub(1) {
            self.bac_plateau_ticks = left;
            if left == 0 {
                self.flags &= !FLAG_BURST_MODE;
            }
        }
        if self.v_thresh > THRESHOLD_BASE {
            let excess = self.v_thresh.saturating_sub(THRESHOLD_BASE);
            self.v_thresh = self
                .v_thresh
                .saturating_sub((excess >> THRESHOLD_DECAY_SHIFT).max(1));
        }

        let in_refractory = self.refractory_ticks > 0;
        if in_refractory {
            self.refractory_ticks = self.refractory_ticks.saturating_sub(1);
        }
        let basal_in = if in_refractory { 0 } else { basal_q16 as i64 };
        let apical_in = if in_refractory { 0 } else { apical_q16 as i64 };
        self.v_basal = add(leak(self.v_basal, BASAL_LEAK_SHIFT), basal_in);
        self.v_apical = add(leak(self.v_apical, APICAL_LEAK_SHIFT), apical_in);
        // The slow potential takes its input whether or not the soma is refractory.
        self.v_slow = add(
            leak(self.v_slow, u32::from(current.leak_shift)),
            i64::from(slow_q16 >> current.input_shift),
        );

        let apical_shift = if self.flags & FLAG_BURST_MODE != 0 {
            PLATEAU_COUPLING_SHIFT
        } else {
            COUPLING_SHIFT
        };
        let from_basal = (self.v_basal as i64).saturating_sub(self.v_soma as i64) >> COUPLING_SHIFT;
        let from_apical = (self.v_apical as i64).saturating_sub(self.v_soma as i64) >> apical_shift;
        // The gate reads the soma before this tick's update: nothing above has moved it.
        let from_slow = current.current_q16(self.v_soma, self.v_slow);
        self.v_soma = add(
            leak(self.v_soma, SOMA_LEAK_SHIFT),
            from_basal
                .saturating_add(from_apical)
                .saturating_add(from_slow),
        );

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

    /// A synapse's message reached the unit at `now_tick` (ADR-0054): the stamp the
    /// descendant rule reads, written by the turn holder from the batch it drained.
    #[inline]
    pub fn note_synaptic_input(&mut self, now_tick: u32) {
        self.last_synaptic_tick = now_tick;
    }

    /// True when a spike at `now_tick` is a descendant (ADR-0054): a synapse's message
    /// reached the unit within [`CAUSAL_LATENCY_TICKS`] before it, as a wrapping difference
    /// (§8.4). A stamp of zero is no message on record: a delivery is integrated the tick
    /// after it, so no message reaches a turn at tick 0.
    #[inline]
    pub const fn is_descendant(&self, now_tick: u32) -> bool {
        self.last_synaptic_tick != NO_SPIKE_ON_RECORD
            && now_tick.wrapping_sub(self.last_synaptic_tick) <= CAUSAL_LATENCY_TICKS
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_spike_within_the_latency_of_a_synapse_s_message_is_a_descendant_and_none_before_one() {
        let mut u = unit();
        assert!(!u.is_descendant(0), "no message on record");
        assert!(!u.is_descendant(CAUSAL_LATENCY_TICKS));
        u.note_synaptic_input(1000);
        assert_eq!(u.last_synaptic_tick, 1000);
        assert!(u.is_descendant(1000), "the same tick");
        assert!(
            u.is_descendant(1000 + CAUSAL_LATENCY_TICKS),
            "at the latency"
        );
        assert!(!u.is_descendant(1001 + CAUSAL_LATENCY_TICKS), "one past it");
        assert!(
            !u.is_descendant(999),
            "before the message: a wrapping difference reads as far"
        );
        u.note_synaptic_input(u32::MAX);
        assert!(u.is_descendant(CAUSAL_LATENCY_TICKS - 1), "across the wrap");
        assert!(!u.is_descendant(CAUSAL_LATENCY_TICKS));
        assert_eq!(CAUSAL_LATENCY_TICKS, 128, "the oracle's latency (ADR-0044)");
    }

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
        let mut free = fired.iter_mut();
        for t in 0..ticks {
            if u.integrate(basal, apical, t) {
                if let Some(slot) = free.next() {
                    *slot = t;
                }
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
    fn one_tick_of_leak_is_the_compartment_s_fraction_of_the_displacement_in_both_signs() {
        let mut up = unit();
        up.v_basal = 0x0001_0000;
        up.v_apical = -0x0001_0000;
        up.integrate(0, 0, 1);
        assert_eq!(
            up.v_basal,
            0x0001_0000 - 0x80,
            "a positive basal potential leaks by 2^-9 of itself"
        );
        assert_eq!(
            up.v_apical,
            -0x0001_0000 + 0x40,
            "a negative apical potential leaks by 2^-10 of its magnitude"
        );
        let mut small = unit();
        small.v_basal = -3;
        small.integrate(0, 0, 1);
        assert_eq!(small.v_basal, -2, "and by at least one LSB");
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

    /// The rule is "at or above": a soma that lands exactly on the threshold fires. From every
    /// compartment at 1.0 with no input, one tick leaves the soma at 1.0 less its own leak
    /// (2^-11) and the two couplings' share of the compartments' leaks (2^-13 and 2^-14),
    /// which is 0xFFD4, worked by hand from the rule before the number was pinned (ADR-0062).
    #[test]
    fn a_soma_that_lands_exactly_on_a_positive_threshold_fires_and_one_lsb_short_does_not() {
        const LANDING: i32 = 0xFFD4;
        for (thresh, fires) in [(LANDING, true), (0xFFD5, false)] {
            let mut u = unit();
            u.v_thresh = thresh;
            u.v_soma = Q16_ONE;
            u.v_basal = Q16_ONE;
            u.v_apical = Q16_ONE;
            assert_eq!(u.integrate(0, 0, 7), fires, "threshold {thresh:#x}");
            if fires {
                assert_eq!(u.v_soma, V_RESET);
                assert_eq!(u.last_soma_spike_tick, 7);
            } else {
                assert_eq!(u.v_soma, LANDING, "the soma landed one LSB short");
            }
        }
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

    /// The facilitating mark and the inhibitory flag are the runtime's (ADR-0114): a plateau
    /// that begins sets the burst bit beside them and one that ends clears it alone, so the two
    /// stand as they were through a burst and a quiet spike, set or clear.
    #[test]
    fn a_plateau_sets_and_clears_the_burst_bit_alone_and_leaves_the_other_flags() {
        for others in [
            0,
            FLAG_FACILITATING,
            FLAG_INHIBITORY,
            FLAG_FACILITATING | FLAG_INHIBITORY,
            !FLAG_BURST_MODE,
        ] {
            let mut u = unit();
            u.flags = others;
            u.v_soma = 2 * THRESHOLD_BASE;
            u.v_apical = 3 * BAC_APICAL_THRESHOLD / 2;
            assert!(u.integrate(0, 0, 0));
            assert_eq!(
                u.flags,
                others | FLAG_BURST_MODE,
                "{others:#04x}: a plateau begins"
            );
            for t in 1..=BAC_PLATEAU_TICKS as u32 {
                u.integrate(0, 0, t);
            }
            assert_eq!(u.flags, others, "{others:#04x}: and ends");
            let mut quiet = unit();
            quiet.flags = others;
            quiet.v_soma = 2 * THRESHOLD_BASE;
            assert!(quiet.integrate(0, 0, 0));
            assert_eq!(
                quiet.flags, others,
                "{others:#04x}: a spike without a plateau"
            );
        }
        assert_eq!(
            FLAG_FACILITATING & (FLAG_BURST_MODE | FLAG_INHIBITORY),
            0,
            "a bit of its own"
        );
    }

    /// Constants with a gate from 0.25 to 0.75, so that its middle is 0.5 and its span 0.5.
    const QUARTERS: SlowCurrent = SlowCurrent {
        leak_shift: 13,
        input_shift: 2,
        v_lo_q16: 0x4000,
        v_hi_q16: 0xC000,
    };

    #[test]
    fn a_slow_current_is_valid_within_its_bounds_and_its_bit_is_its_own() {
        let c = |leak_shift, input_shift, v_lo_q16, v_hi_q16| SlowCurrent {
            leak_shift,
            input_shift,
            v_lo_q16,
            v_hi_q16,
        };
        for good in [
            QUARTERS,
            c(1, 0, 1, 2),
            c(16, 16, 1, THRESHOLD_BASE),
            c(13, 1, 28_561, THRESHOLD_BASE),
            c(2, 15, THRESHOLD_BASE - 1, THRESHOLD_BASE),
        ] {
            assert!(good.is_valid(), "{good:?}");
        }
        for bad in [
            c(0, 2, 0x4000, 0xC000),
            c(17, 2, 0x4000, 0xC000),
            c(u8::MAX, 2, 0x4000, 0xC000),
            c(13, 17, 0x4000, 0xC000),
            c(13, u8::MAX, 0x4000, 0xC000),
            c(13, 2, 0, 0xC000),
            c(13, 2, -1, 0xC000),
            c(13, 2, 0x4000, 0x4000),
            c(13, 2, 0xC000, 0x4000),
            c(13, 2, 0x4000, THRESHOLD_BASE + 1),
            c(13, 2, i32::MIN, i32::MAX),
        ] {
            assert!(!bad.is_valid(), "{bad:?}");
        }
        assert_eq!(
            FLAG_SLOW & (FLAG_BURST_MODE | FLAG_INHIBITORY | FLAG_FACILITATING),
            0,
            "a bit of its own"
        );
    }

    #[test]
    fn the_gate_is_shut_at_and_below_its_low_voltage_open_at_and_above_its_high_and_linear_between()
    {
        let c = QUARTERS;
        for (v, gate) in [
            (i32::MIN, 0),
            (-0x4000, 0),
            (0, 0),
            (0x3FFF, 0),
            (0x4000, 0),
            (0x4001, 2),
            (0x6000, 0x4000),
            (0x8000, 0x8000),
            (0xBFFF, 0xFFFE),
            (0xC000, Q16_ONE),
            (0xC001, Q16_ONE),
            (i32::MAX, Q16_ONE),
        ] {
            assert_eq!(c.gate_q16(v), gate, "at {v:#x}");
        }
        // ADR-0122's voltages, the drive's mean standing and the threshold's base: shut at the
        // standing, 30 421 / 36 975 of the way at 0.9, floored.
        let adr = SlowCurrent {
            v_lo_q16: 28_561,
            v_hi_q16: THRESHOLD_BASE,
            ..QUARTERS
        };
        assert_eq!(adr.gate_q16(28_561), 0);
        assert_eq!(adr.gate_q16(58_982), 53_919);
        assert_eq!(adr.gate_q16(THRESHOLD_BASE), Q16_ONE);
    }

    #[test]
    fn the_current_is_the_gate_times_the_slow_potential_over_sixteen_and_nothing_below_zero() {
        let c = QUARTERS;
        // Half open, a slow potential of 2.0: 0.0625.
        assert_eq!(c.current_q16(0x8000, 0x2_0000), 0x1000);
        // Open, one LSB short of sixteen LSB: floored to nothing; sixteen give one.
        assert_eq!(c.current_q16(0xC000, 15), 0);
        assert_eq!(c.current_q16(0xC000, 16), 1);
        assert_eq!(c.current_q16(0xC000, 0x1_0000), 0x1000);
        // Shut, or no slow potential, or a negative one: nothing.
        assert_eq!(c.current_q16(0x4000, 0x7FFF_FFFF), 0);
        assert_eq!(c.current_q16(0x8000, 0), 0);
        assert_eq!(c.current_q16(0x8000, -0x2_0000), 0);
        assert_eq!(c.current_q16(0xC000, i32::MIN), 0);
        // The widest: open and the slow potential at the width.
        assert_eq!(c.current_q16(0xC000, i32::MAX), 134_217_727);
    }

    #[test]
    fn one_tick_of_the_slow_rule_is_worked_by_hand() {
        // A unit at rest with a slow potential of 1.0 and a slow input of 1/16 under a gate from
        // 0.25 to 0.75: the slow potential leaks 2^-13 of itself, eight LSB, and takes a quarter
        // of the input, 0x400; the soma at zero is below the gate, so it gets nothing.
        let mut u = unit();
        u.v_slow = 0x1_0000;
        assert!(!u.integrate_slow(0, 0, 0x1000, 3, QUARTERS));
        assert_eq!(u.v_slow, 0x1_03F8);
        assert_eq!((u.v_soma, u.v_basal, u.v_apical), (0, 0, 0));
        // The soma at 0.5, the gate half open: the soma leaks sixteen LSB and loses a sixteenth
        // of 0.5 to each empty compartment, 2 048 each, and gains 0x8000 · 0x103F8 >> 20 = 2 079.
        let mut u = unit();
        u.v_slow = 0x1_0000;
        u.v_soma = 0x8000;
        assert!(!u.integrate_slow(0, 0, 0x1000, 3, QUARTERS));
        assert_eq!(u.v_slow, 0x1_03F8);
        assert_eq!(u.v_soma, 0x8000 - 16 - 2 * 2_048 + 2_079);
        // `integrate` from the same state moves the soma by all of that but the current.
        let mut plain = unit();
        plain.v_soma = 0x8000;
        assert!(!plain.integrate(0, 0, 3));
        assert_eq!(plain.v_soma, 0x8000 - 16 - 2 * 2_048);
    }

    #[test]
    fn the_slow_potential_leaks_by_its_shift_and_takes_its_input_by_its_shift_at_their_bounds() {
        let step = |slow: i32, input: i32, leak_shift: u8, input_shift: u8| {
            let mut u = unit();
            u.v_slow = slow;
            u.integrate_slow(
                0,
                0,
                input,
                1,
                SlowCurrent {
                    leak_shift,
                    input_shift,
                    ..QUARTERS
                },
            );
            u.v_slow
        };
        assert_eq!(step(0x1_0000, 0, 1, 0), 0x8000, "half of itself at 2^-1");
        assert_eq!(step(0x1_0000, 0, 16, 0), 0xFFFF, "one LSB of 1.0 at 2^-16");
        assert_eq!(step(5, 0, 16, 0), 4, "and at least one LSB");
        assert_eq!(step(-0x1_0000, 0, 1, 0), -0x8000, "toward zero from below");
        assert_eq!(step(1, 0, 13, 0), 0, "the last LSB goes too");
        assert_eq!(step(0, 0x123, 13, 0), 0x123, "the whole input at g = 0");
        assert_eq!(step(0, 0x123, 13, 4), 0x12, "a sixteenth, floored");
        assert_eq!(step(0, 0xFFFF, 13, 16), 0, "below one LSB at g = 16");
        assert_eq!(step(0, 0x1_0000, 13, 16), 1);
        assert_eq!(
            step(i32::MAX, i32::MAX, 13, 0),
            i32::MAX,
            "saturating at the top"
        );
        assert_eq!(step(i32::MIN, 0, 16, 0), i32::MIN + 0x8000);
    }

    #[test]
    fn in_the_refractory_window_the_slow_potential_takes_its_input_and_the_compartments_drop_theirs()
     {
        let mut u = unit();
        u.v_soma = 2 * THRESHOLD_BASE;
        assert!(u.integrate_slow(0, 0, 0, 0, QUARTERS), "fires");
        assert_eq!(u.refractory_ticks, REFRACTORY_TICKS);
        let mut slow = 0i32;
        for t in 1..=u32::from(REFRACTORY_TICKS) {
            assert!(!u.integrate_slow(i32::MAX, i32::MAX, 0x400, t, QUARTERS));
            // leak(slow, 13) + 0x400 >> 2: 2^-13 of itself, at least one LSB, never past zero.
            let leaked = (slow >> 13).max(1).min(slow);
            slow = slow.saturating_sub(leaked).saturating_add(0x100);
            assert_eq!(u.v_slow, slow, "tick {t}");
        }
        assert_eq!(
            (u.v_basal, u.v_apical),
            (0, 0),
            "the compartments dropped theirs"
        );
        assert!(u.v_slow > 0x100 * 190, "the slow potential took its input");
        assert!(
            u.v_soma < 0x4000,
            "the gate shut below 0.25: nothing reached the soma"
        );
        assert_eq!(u.refractory_ticks, 0);
    }

    /// The slow rule fires "at or above", as `integrate` does. With no slow potential its soma lands
    /// where `integrate`'s does, 0xFFD4 from every compartment at 1.0. With a slow potential of 1.0
    /// under the gate open at 1.0, the potential leaks eight LSB to 0xFFF8 and the soma gains
    /// 0x10000 · 0xFFF8 >> 20 = 4 095 more, landing at 0x10FD3; a threshold above its base first
    /// decays by one LSB there, so the one it meets is the one set less one. Worked by hand from the
    /// rule.
    #[test]
    fn the_slow_rule_fires_where_the_soma_lands_on_the_threshold_and_not_one_lsb_short() {
        const LANDING: i32 = 0xFFD4;
        for (slow, landing, decay) in [(0, LANDING, 0), (0x1_0000, LANDING + 4_095, 1)] {
            for (thresh, fires) in [(landing + decay, true), (landing + decay + 1, false)] {
                let mut u = unit();
                u.v_thresh = thresh;
                u.v_soma = Q16_ONE;
                u.v_basal = Q16_ONE;
                u.v_apical = Q16_ONE;
                u.v_slow = slow;
                assert_eq!(
                    u.integrate_slow(0, 0, 0, 7, QUARTERS),
                    fires,
                    "slow {slow:#x} threshold {thresh:#x}"
                );
                if !fires {
                    assert_eq!(u.v_soma, landing, "the soma landed one LSB short");
                }
            }
        }
    }

    #[test]
    fn a_slow_potential_fires_a_unit_under_an_open_gate_that_the_same_inputs_do_not_fire() {
        let mut slow = unit();
        slow.v_soma = 0xB000;
        slow.v_basal = 0xB000;
        slow.v_slow = 0x8_0000;
        let mut plain = unit();
        plain.v_soma = 0xB000;
        plain.v_basal = 0xB000;
        let fired = (0..64u32)
            .find(|&t| slow.integrate_slow(0, 0, 0, t, QUARTERS))
            .expect("the slow current fires it");
        assert!(
            (0..64u32).all(|t| !plain.integrate(0, 0, t)),
            "the same inputs without it do not"
        );
        assert_eq!(slow.last_soma_spike_tick, fired);
        assert_eq!(slow.v_soma, V_RESET);
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
        // Every bit but the burst bit set: `integrate` leaves them as it finds them (ADR-0114).
        u.flags = FLAG_FACILITATING | FLAG_INHIBITORY | 0xF8;
        let mut fired = 0u32;
        for t in 0..1_000_000u32 {
            let (basal, apical) = (rng.i32_edge_biased(), rng.i32_edge_biased());
            let refractory_before = u.refractory_ticks;
            let (basal_before, apical_before) = (u.v_basal, u.v_apical);
            let spiked = u.integrate(basal, apical, t);
            check(&u);
            assert_eq!(
                u.flags & !FLAG_BURST_MODE,
                FLAG_FACILITATING | FLAG_INHIBITORY | 0xF8,
                "the other flags as they were"
            );
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

    /// Every field the two rules write: the potentials, the threshold, the two countdowns, the
    /// spike's stamp, the flags and the slow potential.
    type Fields = (i32, i32, i32, i32, u16, u16, u32, u8, i32);

    fn fields(u: &DendriticSuperNeuron) -> Fields {
        (
            u.v_soma,
            u.v_basal,
            u.v_apical,
            u.v_thresh,
            u.bac_plateau_ticks,
            u.refractory_ticks,
            u.last_soma_spike_tick,
            u.flags,
            u.v_slow,
        )
    }

    /// The constants the property tests run the slow rule under: each shift at both bounds, a
    /// gate one LSB wide and one the width of the threshold, and ADR-0122's.
    const CURRENTS: [SlowCurrent; 5] = [
        SlowCurrent {
            leak_shift: 1,
            input_shift: 0,
            v_lo_q16: 1,
            v_hi_q16: 2,
        },
        SlowCurrent {
            leak_shift: 16,
            input_shift: 16,
            v_lo_q16: 1,
            v_hi_q16: THRESHOLD_BASE,
        },
        SlowCurrent {
            leak_shift: 13,
            input_shift: 1,
            v_lo_q16: 28_561,
            v_hi_q16: THRESHOLD_BASE,
        },
        SlowCurrent {
            leak_shift: 2,
            input_shift: 4,
            v_lo_q16: 0x4000,
            v_hi_q16: 0xC000,
        },
        SlowCurrent {
            leak_shift: 7,
            input_shift: 9,
            v_lo_q16: THRESHOLD_BASE - 1,
            v_hi_q16: THRESHOLD_BASE,
        },
    ];

    #[test]
    fn with_no_slow_potential_and_no_slow_input_the_slow_rule_is_integrate_bit_for_bit() {
        assert!(CURRENTS.iter().all(|c| c.is_valid()));
        for current in CURRENTS {
            for &basal in I32_LATTICE.iter() {
                for &apical in I32_LATTICE.iter() {
                    for start in 0..3 {
                        let mut plain = DendriticSuperNeuron::new(1);
                        match start {
                            0 => plain.v_thresh = THRESHOLD_BASE,
                            1 => {
                                plain.v_thresh = i32::MAX;
                                plain.v_soma = i32::MAX;
                                plain.v_basal = i32::MIN;
                                plain.v_apical = i32::MIN;
                            }
                            _ => plain.v_thresh = i32::MIN,
                        }
                        plain.flags = FLAG_SLOW;
                        let mut slow = DendriticSuperNeuron::new(1);
                        slow.restore_plain_fields(&plain);
                        for t in 0..64u32 {
                            let a = plain.integrate(basal, apical, t);
                            let b = slow.integrate_slow(basal, apical, 0, t, current);
                            assert_eq!(a, b, "{current:?} {basal:#x} {apical:#x} tick {t}");
                            assert_eq!(fields(&plain), fields(&slow));
                        }
                    }
                }
            }
        }
        // A million edge-biased ticks, every flag but the burst bit set and left as found.
        let mut rng = Lcg::new(0x5EED_0123_0052_0000);
        let mut plain = DendriticSuperNeuron::new(1);
        plain.v_thresh = THRESHOLD_BASE;
        plain.flags = !FLAG_BURST_MODE;
        let mut slow = DendriticSuperNeuron::new(1);
        slow.restore_plain_fields(&plain);
        let mut fired = 0u32;
        for t in 0..1_000_000u32 {
            let current = rng.pick(&CURRENTS);
            let (basal, apical) = (rng.i32_edge_biased(), rng.i32_edge_biased());
            let a = plain.integrate(basal, apical, t);
            assert_eq!(a, slow.integrate_slow(basal, apical, 0, t, current));
            assert_eq!(fields(&plain), fields(&slow), "tick {t}");
            assert_eq!(slow.flags & !FLAG_BURST_MODE, !FLAG_BURST_MODE);
            fired = fired.saturating_add(u32::from(a));
        }
        assert!(fired > 1_000, "the walk fires: {fired}");
    }

    /// ADR-0122's rule, written from ADR-0018's text and ADR-0122's in `i64` rather than from
    /// `integrate_slow`: the fields one tick leaves and whether the unit fired.
    fn oracle(
        u: &DendriticSuperNeuron,
        basal: i32,
        apical: i32,
        slow: i32,
        now: u32,
        c: SlowCurrent,
    ) -> (Fields, bool) {
        // Toward zero by a 2^-shift fraction of the magnitude, at least one LSB, never past zero.
        let leak = |v: i64, shift: u32| {
            let m = v.abs();
            v.saturating_sub(v.signum().saturating_mul((m >> shift).max(1).min(m)))
        };
        let width = |v: i64| v.clamp(i64::from(i32::MIN), i64::from(i32::MAX));
        let (mut plateau, mut flags) = (u.bac_plateau_ticks, u.flags);
        if plateau > 0 {
            plateau = plateau.saturating_sub(1);
            if plateau == 0 {
                flags &= !FLAG_BURST_MODE;
            }
        }
        let base = i64::from(THRESHOLD_BASE);
        let mut thresh = i64::from(u.v_thresh);
        if thresh > base {
            thresh = thresh
                .saturating_sub((thresh.saturating_sub(base) >> THRESHOLD_DECAY_SHIFT).max(1));
        }
        let refractory = u.refractory_ticks > 0;
        let mut window = u.refractory_ticks.saturating_sub(1);
        let taken = |x: i32| if refractory { 0 } else { i64::from(x) };
        let b = width(leak(i64::from(u.v_basal), BASAL_LEAK_SHIFT).saturating_add(taken(basal)));
        let a = width(leak(i64::from(u.v_apical), APICAL_LEAK_SHIFT).saturating_add(taken(apical)));
        // The slow potential: its leak and 2^-g of its input, in the window too.
        let s = width(
            leak(i64::from(u.v_slow), u32::from(c.leak_shift))
                .saturating_add(i64::from(slow) >> c.input_shift),
        );
        // The gate on the soma as the tick found it, and the current, only while s > 0.
        let v = i64::from(u.v_soma);
        let (lo, hi) = (i64::from(c.v_lo_q16), i64::from(c.v_hi_q16));
        let gate = if v <= lo {
            0
        } else if v >= hi {
            1 << 16
        } else {
            (v.saturating_sub(lo) << 16)
                .checked_div(hi.saturating_sub(lo))
                .expect("a span above zero")
        };
        let current = if s > 0 {
            gate.saturating_mul(s) >> 20
        } else {
            0
        };
        let apical_shift = if flags & FLAG_BURST_MODE != 0 { 2 } else { 4 };
        let mut soma = width(
            leak(v, SOMA_LEAK_SHIFT)
                .saturating_add(b.saturating_sub(v) >> 4)
                .saturating_add(a.saturating_sub(v) >> apical_shift)
                .saturating_add(current),
        );
        let mut last = u.last_soma_spike_tick;
        let fired = !refractory && thresh > 0 && soma >= thresh;
        if fired {
            last = now;
            soma = i64::from(V_RESET);
            thresh = thresh
                .saturating_add(i64::from(THRESHOLD_STEP))
                .min(i64::from(i32::MAX));
            if a >= i64::from(BAC_APICAL_THRESHOLD) {
                plateau = BAC_PLATEAU_TICKS;
                flags |= FLAG_BURST_MODE;
                window = BURST_REFRACTORY_TICKS;
            } else {
                window = REFRACTORY_TICKS;
            }
        }
        (
            (
                soma as i32,
                b as i32,
                a as i32,
                thresh as i32,
                plateau,
                window,
                last,
                flags,
                s as i32,
            ),
            fired,
        )
    }

    fn checked(
        u: &DendriticSuperNeuron,
        basal: i32,
        apical: i32,
        slow: i32,
        now: u32,
        c: SlowCurrent,
    ) {
        let expected = oracle(u, basal, apical, slow, now, c);
        let mut v = DendriticSuperNeuron::new(u.id);
        v.restore_plain_fields(u);
        let fired = v.integrate_slow(basal, apical, slow, now, c);
        assert_eq!(
            (fields(&v), fired),
            expected,
            "{c:?} from {:?} with {basal:#x} {apical:#x} {slow:#x}",
            fields(u)
        );
    }

    #[test]
    fn the_slow_rule_is_adr_0122_s_by_an_i64_oracle_at_any_state() {
        // Over the lattice of the soma, the slow potential and the slow input, the soma also at
        // each gate's edges and their neighbours, under every constant.
        for c in CURRENTS {
            let edges = [
                c.v_lo_q16.saturating_sub(1),
                c.v_lo_q16,
                c.v_lo_q16.saturating_add(1),
                c.v_hi_q16.saturating_sub(1),
                c.v_hi_q16,
                c.v_hi_q16.saturating_add(1),
            ];
            for &soma in I32_LATTICE.iter().chain(edges.iter()) {
                for &v_slow in I32_LATTICE.iter() {
                    for &slow in I32_LATTICE.iter() {
                        let mut u = DendriticSuperNeuron::new(1);
                        u.v_thresh = THRESHOLD_BASE;
                        u.flags = FLAG_SLOW;
                        u.v_soma = soma;
                        u.v_basal = soma;
                        u.v_slow = v_slow;
                        checked(&u, 0, 0, slow, 9, c);
                        u.refractory_ticks = 1;
                        checked(&u, 0x100, -0x100, slow, 9, c);
                    }
                }
            }
        }
        // And 400 000 seeded draws of a whole state, its inputs and its constants.
        let mut rng = Lcg::new(0x0122_0052_5EED_0000);
        let thresholds = [
            i32::MIN,
            -1,
            0,
            1,
            THRESHOLD_BASE,
            THRESHOLD_BASE + 1,
            THRESHOLD_BASE + THRESHOLD_STEP,
            i32::MAX,
        ];
        let windows = [0u16, 1, 2, 50, 199, 200, u16::MAX];
        for _ in 0..400_000 {
            let c = rng.pick(&CURRENTS);
            let mut u = DendriticSuperNeuron::new(1);
            u.v_soma = rng.i32_edge_biased();
            u.v_basal = rng.i32_edge_biased();
            u.v_apical = rng.i32_edge_biased();
            u.v_slow = rng.i32_edge_biased();
            u.v_thresh = rng.pick(&thresholds);
            u.refractory_ticks = rng.pick(&windows);
            u.bac_plateau_ticks = rng.pick(&windows);
            u.flags = rng.next_u8();
            u.last_soma_spike_tick = rng.next_u32();
            checked(
                &u,
                rng.i32_edge_biased(),
                rng.i32_edge_biased(),
                rng.i32_edge_biased(),
                rng.next_u32(),
                c,
            );
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
