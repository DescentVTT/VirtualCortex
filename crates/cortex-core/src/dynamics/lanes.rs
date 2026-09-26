//! The membrane's rule in lanes (ADR-0103, ADR-0104): [`DendriticSuperNeuron::integrate`] for
//! [`LANES`] units at once, each unit's integrated fields in one lane of a fixed-size array, so
//! that the compiler's vectorizer can run a step on several units in one instruction.
//!
//! `integrate` is the specification and does not change. The lane form computes, for every
//! unit and every input, exactly what `integrate` computes, and a property test over the
//! lattice of `testkit/prop.rs` holds every lane to it. It is written for the build's baseline,
//! four `i32` lanes of SSE2 on x86-64 and of NEON on AArch64, with no intrinsic, no `unsafe`
//! and no allocation, as the hypervector body's rules are (ADR-0039): every step is an `i32`
//! operation in every lane, a branch is a select, and the rule's two `i64` steps are rewritten
//! in `i32` with the same result:
//!
//! - `leak` moves by the magnitude's fraction, at least one LSB and never past zero; the lane
//!   form takes the magnitude as `u32`, so that `i32::MIN` has one, and the sign as a mask;
//! - `add` widens an `i32` input to `i64` and clamps, which for two `i32` values is the
//!   saturating `i32` sum;
//! - the coupling shifts a difference taken in `i64`: `(x − y) >> k` is
//!   `(x >> k) − (y >> k)` plus the floor of the low bits' difference over `2^k`, which is
//!   `−1` or `0`, and every term fits in `i32`.
//!
//! A lane array is scratch within one turn phase (ADR-0001): the executor loads a chunk of
//! its scheduled units, integrates the lanes and stores them back before the chunk's spikes
//! are recorded, so the 64-byte record stays the only home of a unit's state (quality goal 2).

use super::membrane::{
    APICAL_LEAK_SHIFT, BAC_APICAL_THRESHOLD, BAC_PLATEAU_TICKS, BASAL_LEAK_SHIFT,
    BURST_REFRACTORY_TICKS, COUPLING_SHIFT, FLAG_BURST_MODE, PLATEAU_COUPLING_SHIFT,
    REFRACTORY_TICKS, SOMA_LEAK_SHIFT, THRESHOLD_BASE, THRESHOLD_DECAY_SHIFT, THRESHOLD_STEP,
    V_RESET,
};
use super::neuron::DendriticSuperNeuron;

/// The units integrated together: two vectors of four `i32` lanes at the baseline.
pub const LANES: usize = 8;

/// The integrated fields of [`LANES`] units and their inputs, one unit to a lane: the four
/// potentials, the two countdowns and the burst flag, each widened to an `i32` lane so that
/// every step runs at one width. Scratch: it holds a unit's state only between
/// [`load`](Self::load) and [`store`](Self::store).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MembraneLanes {
    pub v_soma: [i32; LANES],
    pub v_basal: [i32; LANES],
    pub v_apical: [i32; LANES],
    pub v_thresh: [i32; LANES],
    /// `bac_plateau_ticks`, `0..=u16::MAX`.
    pub plateau: [i32; LANES],
    /// `refractory_ticks`, `0..=u16::MAX`.
    pub refractory: [i32; LANES],
    /// The `FLAG_BURST_MODE` bit of `flags`, 0 or 1.
    pub burst: [i32; LANES],
    /// The tick's basal input, as `integrate` takes it.
    pub basal_in: [i32; LANES],
    /// The tick's apical input.
    pub apical_in: [i32; LANES],
    /// 1 in a lane whose unit fired in the last [`integrate`](Self::integrate), else 0.
    pub spiked: [i32; LANES],
}

impl Default for MembraneLanes {
    fn default() -> Self {
        Self::new()
    }
}

/// `leak` in a lane: `v` moved toward zero by `|v| >> shift`, by at least one LSB, never past
/// zero. The step is at most `|v|`, and below $2^{31}$ for a shift of one or more, so the move
/// is exact in `i32`; the sign mask negates the step for a negative `v`.
#[inline(always)]
fn leak(v: i32, shift: u32) -> i32 {
    let magnitude = v.unsigned_abs();
    let step = ((magnitude >> shift) as i32).max((magnitude != 0) as i32);
    let negative = v >> 31;
    v.wrapping_sub((step ^ negative).wrapping_sub(negative))
}

/// `(x − y) >> shift` as `integrate` takes it in `i64`, in `i32`: the shifted operands'
/// difference plus the floor of the low bits' difference, `−1` or `0`. For a shift of two or
/// more the result is within `i32`, and so is every term.
#[inline(always)]
fn shifted_difference(x: i32, y: i32, shift: u32) -> i32 {
    let low = (1i32 << shift).wrapping_sub(1);
    let borrow = (x & low).wrapping_sub(y & low) >> shift;
    (x >> shift).wrapping_sub(y >> shift).wrapping_add(borrow)
}

impl MembraneLanes {
    /// Every lane zero.
    pub const fn new() -> Self {
        Self {
            v_soma: [0; LANES],
            v_basal: [0; LANES],
            v_apical: [0; LANES],
            v_thresh: [0; LANES],
            plateau: [0; LANES],
            refractory: [0; LANES],
            burst: [0; LANES],
            basal_in: [0; LANES],
            apical_in: [0; LANES],
            spiked: [0; LANES],
        }
    }

    /// Takes `unit`'s integrated fields and the tick's inputs into `lane`. Panics if `lane` is
    /// not below [`LANES`].
    #[inline]
    pub fn load(
        &mut self,
        lane: usize,
        unit: &DendriticSuperNeuron,
        basal_q16: i32,
        apical_q16: i32,
    ) {
        self.v_soma[lane] = unit.v_soma;
        self.v_basal[lane] = unit.v_basal;
        self.v_apical[lane] = unit.v_apical;
        self.v_thresh[lane] = unit.v_thresh;
        self.plateau[lane] = i32::from(unit.bac_plateau_ticks);
        self.refractory[lane] = i32::from(unit.refractory_ticks);
        self.burst[lane] = i32::from(unit.flags & FLAG_BURST_MODE);
        self.basal_in[lane] = basal_q16;
        self.apical_in[lane] = apical_q16;
    }

    /// Writes `lane` back into `unit`, stamping `now_tick` if it fired, and returns whether it
    /// fired: what `integrate` would have left in `unit` and returned. Panics if `lane` is not
    /// below [`LANES`].
    #[inline]
    pub fn store(&self, lane: usize, unit: &mut DendriticSuperNeuron, now_tick: u32) -> bool {
        unit.v_soma = self.v_soma[lane];
        unit.v_basal = self.v_basal[lane];
        unit.v_apical = self.v_apical[lane];
        unit.v_thresh = self.v_thresh[lane];
        // Both countdowns are within `u16` in every lane `load` filled: a countdown only falls
        // or is set to a window's length.
        unit.bac_plateau_ticks = self.plateau[lane] as u16;
        unit.refractory_ticks = self.refractory[lane] as u16;
        unit.flags = (unit.flags & !FLAG_BURST_MODE).wrapping_add(self.burst[lane] as u8);
        let fired = self.spiked[lane] != 0;
        if fired {
            unit.last_soma_spike_tick = now_tick;
        }
        fired
    }

    /// One fine tick in every lane: `integrate`'s rule, step for step, with each branch a
    /// select, so that a lane holds after it exactly what `integrate` leaves in its unit, and
    /// `spiked` what it returns.
    pub fn integrate(&mut self) {
        for i in 0..LANES {
            // A plateau that is running counts down, and ends when it reaches zero.
            let plateau = self.plateau[i];
            let running = (plateau != 0) as i32;
            let burst = (self.burst[i] != 0) & (plateau != 1);
            let plateau = plateau.wrapping_sub(running);

            // A threshold above its base decays toward it, by at least one LSB.
            let thresh = self.v_thresh[i];
            let excess = thresh.wrapping_sub(THRESHOLD_BASE);
            let decay = if thresh > THRESHOLD_BASE {
                (excess >> THRESHOLD_DECAY_SHIFT).max(1)
            } else {
                0
            };
            let thresh = thresh.wrapping_sub(decay);

            // Inside the refractory window the window counts down and the inputs are dropped.
            // The inputs are read in every lane and masked: a read only where the window is
            // open is a conditional load, which the baseline has no vector form for.
            let refractory = self.refractory[i];
            let open = refractory == 0;
            let refractory = refractory.wrapping_sub((!open) as i32);
            let taken = (open as i32).wrapping_neg();
            let basal_in = self.basal_in[i] & taken;
            let apical_in = self.apical_in[i] & taken;
            let basal = leak(self.v_basal[i], BASAL_LEAK_SHIFT).saturating_add(basal_in);
            let apical = leak(self.v_apical[i], APICAL_LEAK_SHIFT).saturating_add(apical_in);

            // The soma leaks and takes a fraction of its difference to each compartment, the
            // apical fraction the plateau's while bursting.
            let soma = self.v_soma[i];
            let from_basal = shifted_difference(basal, soma, COUPLING_SHIFT);
            let from_apical = if burst {
                shifted_difference(apical, soma, PLATEAU_COUPLING_SHIFT)
            } else {
                shifted_difference(apical, soma, COUPLING_SHIFT)
            };
            let soma =
                leak(soma, SOMA_LEAK_SHIFT).saturating_add(from_basal.wrapping_add(from_apical));

            // A spike, outside the window, at or above a positive threshold; with the apical
            // compartment at or above its threshold, a plateau and the burst's window.
            let spike = open & (thresh > 0) & (soma >= thresh);
            let plateau_starts = spike & (apical >= BAC_APICAL_THRESHOLD);
            self.v_soma[i] = if spike { V_RESET } else { soma };
            self.v_basal[i] = basal;
            self.v_apical[i] = apical;
            self.v_thresh[i] = if spike {
                thresh.saturating_add(THRESHOLD_STEP)
            } else {
                thresh
            };
            self.plateau[i] = if plateau_starts {
                i32::from(BAC_PLATEAU_TICKS)
            } else {
                plateau
            };
            self.refractory[i] = if plateau_starts {
                i32::from(BURST_REFRACTORY_TICKS)
            } else if spike {
                i32::from(REFRACTORY_TICKS)
            } else {
                refractory
            };
            self.burst[i] = (burst | plateau_starts) as i32;
            self.spiked[i] = spike as i32;
        }
    }
}

/// Property tests (ADR-0030, ADR-0104): every lane is `integrate`, over the lattice of
/// `testkit/prop.rs`, its seeded walk, and the two boundaries a walk reaches only by chance.
#[cfg(test)]
mod prop {
    use super::*;
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../testkit/prop.rs"
    ));

    /// Every field but the atomics, which neither form touches: what the rule integrates, and
    /// the rest of the record.
    type Plain = (
        (i32, i32, i32, i32, u16, u16, u32, u8),
        (u64, u32, u32, u32, u16, u16, u8, u8, u32),
    );

    fn plain(u: &DendriticSuperNeuron) -> Plain {
        (
            (
                u.v_soma,
                u.v_basal,
                u.v_apical,
                u.v_thresh,
                u.bac_plateau_ticks,
                u.refractory_ticks,
                u.last_soma_spike_tick,
                u.flags,
            ),
            (
                u.id,
                u.last_synaptic_tick,
                u._reserved_20,
                u.synapse_slab_idx,
                u._reserved,
                u.spatial_voxel_morton,
                u.stp_r_ves,
                u.stp_u_rel,
                u.plastic_delta_head,
            ),
        )
    }

    fn copy(u: &DendriticSuperNeuron) -> DendriticSuperNeuron {
        let mut c = DendriticSuperNeuron::new(u.id);
        c.last_synaptic_tick = u.last_synaptic_tick;
        c._reserved_20 = u._reserved_20;
        c.v_soma = u.v_soma;
        c.v_basal = u.v_basal;
        c.v_apical = u.v_apical;
        c.v_thresh = u.v_thresh;
        c.bac_plateau_ticks = u.bac_plateau_ticks;
        c.refractory_ticks = u.refractory_ticks;
        c.last_soma_spike_tick = u.last_soma_spike_tick;
        c.synapse_slab_idx = u.synapse_slab_idx;
        c._reserved = u._reserved;
        c.spatial_voxel_morton = u.spatial_voxel_morton;
        c.flags = u.flags;
        c.stp_r_ves = u.stp_r_ves;
        c.stp_u_rel = u.stp_u_rel;
        c.plastic_delta_head = u.plastic_delta_head;
        c
    }

    /// A countdown: none running half the time, so that the unit takes its inputs; otherwise
    /// the windows' lengths and their neighbours, the ends of `u16`, or any value.
    fn countdown(rng: &mut Lcg) -> u16 {
        const EDGES: [u16; 11] = [0, 1, 2, 49, 50, 51, 199, 200, 201, u16::MAX - 1, u16::MAX];
        match rng.below(4) {
            0 => rng.next_u16(),
            1 => rng.pick(&EDGES),
            _ => 0,
        }
    }

    /// A potential or an input in the working range, within two of rest either way.
    fn working(rng: &mut Lcg) -> i32 {
        ((rng.next_u32() >> 14) as i32).wrapping_sub(0x0002_0000)
    }

    /// A unit anywhere in the record's domain: every potential edge-biased, both countdowns,
    /// every bit of `flags`, so the burst flag with or without a plateau.
    fn arbitrary(rng: &mut Lcg) -> DendriticSuperNeuron {
        let mut u = DendriticSuperNeuron::new(rng.next_u64());
        u.last_synaptic_tick = rng.next_u32();
        u.v_soma = rng.i32_edge_biased();
        u.v_basal = rng.i32_edge_biased();
        u.v_apical = rng.i32_edge_biased();
        u.v_thresh = rng.i32_edge_biased();
        u.bac_plateau_ticks = countdown(rng);
        u.refractory_ticks = countdown(rng);
        u.last_soma_spike_tick = rng.next_u32();
        u.synapse_slab_idx = rng.next_u32();
        u.spatial_voxel_morton = rng.next_u16();
        u.flags = rng.next_u8();
        u.stp_r_ves = rng.next_u8();
        u.stp_u_rel = rng.next_u8();
        u.plastic_delta_head = rng.next_u32();
        u
    }

    /// What the chunks reached, counted from the units before and after `integrate`: each
    /// branch the rule has, so that a test holds its own coverage.
    #[derive(Default)]
    struct Seen {
        spikes: u32,
        plateaus: u32,
        refractory: u32,
        ended: u32,
        bursting: u32,
        above_base: u32,
        unconfigured: u32,
        high: u32,
        low: u32,
    }

    fn count(tally: &mut u32, reached: bool) {
        *tally = tally.saturating_add(u32::from(reached));
    }

    /// Integrates `units` with `inputs` at `now` both ways, holds every lane, and what `store`
    /// returns, to `integrate`, and leaves `units` as `integrate` left them.
    fn both(
        units: &mut [DendriticSuperNeuron; LANES],
        inputs: &[(i32, i32); LANES],
        now: u32,
        seen: &mut Seen,
    ) {
        let mut lanes = MembraneLanes::new();
        for (lane, (u, &(basal, apical))) in units.iter().zip(inputs).enumerate() {
            lanes.load(lane, u, basal, apical);
        }
        lanes.integrate();
        for (lane, (u, &(basal, apical))) in units.iter_mut().zip(inputs).enumerate() {
            count(
                &mut seen.refractory,
                u.refractory_ticks != 0 && (basal, apical) != (0, 0),
            );
            count(&mut seen.ended, u.bac_plateau_ticks == 1);
            count(
                &mut seen.bursting,
                u.flags & FLAG_BURST_MODE != 0 && u.bac_plateau_ticks != 1,
            );
            count(&mut seen.above_base, u.v_thresh > THRESHOLD_BASE);
            count(&mut seen.unconfigured, u.v_thresh <= 0);
            let mut stored = copy(u);
            let fired = lanes.store(lane, &mut stored, now);
            let expected = u.integrate(basal, apical, now);
            assert_eq!(
                (plain(&stored), fired),
                (plain(u), expected),
                "lane {lane} at tick {now} with inputs ({basal}, {apical})"
            );
            count(&mut seen.spikes, fired);
            count(
                &mut seen.plateaus,
                fired && u.bac_plateau_ticks == BAC_PLATEAU_TICKS,
            );
            count(
                &mut seen.high,
                [u.v_soma, u.v_basal, u.v_apical].contains(&i32::MAX),
            );
            count(
                &mut seen.low,
                [u.v_soma, u.v_basal, u.v_apical].contains(&i32::MIN),
            );
        }
    }

    fn reached_every_branch(seen: &Seen) {
        assert!(
            seen.spikes > 1_000 && seen.plateaus > 100,
            "spikes {} plateaus {}",
            seen.spikes,
            seen.plateaus
        );
        assert!(
            seen.refractory > 1_000 && seen.ended > 100 && seen.bursting > 1_000,
            "refractory {} ended {} bursting {}",
            seen.refractory,
            seen.ended,
            seen.bursting
        );
        assert!(
            seen.above_base > 1_000 && seen.unconfigured > 1_000,
            "above the base {} unconfigured {}",
            seen.above_base,
            seen.unconfigured
        );
        assert!(
            seen.high > 100 && seen.low > 100,
            "both saturating ends: {} {}",
            seen.high,
            seen.low
        );
    }

    #[test]
    fn every_lane_is_integrate_for_every_lattice_pair_of_inputs_on_units_anywhere_in_the_domain() {
        const PAIRS: usize = I32_LATTICE.len() * I32_LATTICE.len();
        let mut rng = Lcg::new(0x5EED_1A4E_0000_0001);
        let mut seen = Seen::default();
        let pairs: [(i32, i32); PAIRS] = core::array::from_fn(|k| {
            (
                I32_LATTICE[k / I32_LATTICE.len()],
                I32_LATTICE[k % I32_LATTICE.len()],
            )
        });
        // Every pair in every lane over the rounds: the pairs are dealt to the lanes from an
        // offset the round moves.
        for round in 0..64usize {
            for chunk in 0..PAIRS.div_ceil(LANES) {
                let inputs: [(i32, i32); LANES] = core::array::from_fn(|lane| {
                    pairs[chunk
                        .wrapping_mul(LANES)
                        .wrapping_add(lane)
                        .wrapping_add(round)
                        .wrapping_rem(PAIRS)]
                });
                let mut units: [DendriticSuperNeuron; LANES] =
                    core::array::from_fn(|_| arbitrary(&mut rng));
                both(&mut units, &inputs, rng.next_u32(), &mut seen);
            }
        }
        reached_every_branch(&seen);
    }

    #[test]
    fn a_seeded_walk_of_a_million_lane_ticks_is_integrate_tick_for_tick() {
        let mut rng = Lcg::new(0x9E37_79B9_7F4A_7C15);
        let mut seen = Seen::default();
        let mut units: [DendriticSuperNeuron; LANES] = core::array::from_fn(|lane| {
            let mut u = DendriticSuperNeuron::new(lane as u64);
            u.v_thresh = THRESHOLD_BASE;
            u
        });
        for t in 0..(1u32 << 17) {
            // Now and then a lane restarts anywhere in the domain, so that the walk reaches the
            // unconfigured threshold and the countdowns' edges as well as what a drive makes.
            if rng.below(64) == 0 {
                units[rng.below(LANES as u32) as usize] = arbitrary(&mut rng);
            }
            let inputs: [(i32, i32); LANES] = core::array::from_fn(|_| {
                if rng.below(2) == 0 {
                    (rng.i32_edge_biased(), rng.i32_edge_biased())
                } else {
                    // A drive in the working range, which fires and bursts.
                    (
                        (rng.next_u32() >> 20) as i32,
                        ((rng.next_u32() >> 20) as i32).wrapping_sub(0x400),
                    )
                }
            });
            both(&mut units, &inputs, t, &mut seen);
        }
        reached_every_branch(&seen);
    }

    /// The two comparisons a walk meets at equality only by chance: a soma that lands exactly
    /// on its threshold fires and one LSB short does not; an apical potential exactly at the
    /// plateau's threshold starts one on a spike and one LSB below does not. The soma a tick
    /// leaves does not depend on the threshold, so a threshold at or below its base is set to
    /// it; the apical potential is the last one's leak plus the input, so the input is set to
    /// reach it.
    #[test]
    fn a_soma_exactly_at_its_threshold_fires_and_an_apical_potential_exactly_at_the_plateau_threshold_starts_one()
     {
        let mut rng = Lcg::new(0x0B0A_4DA2_1E50_0001);
        let (mut landed, mut short, mut started, mut below) = (0u32, 0u32, 0u32, 0u32);
        let mut seen = Seen::default();
        for _ in 0..4_096u32 {
            let mut units: [DendriticSuperNeuron; LANES] = core::array::from_fn(|_| {
                let mut u = arbitrary(&mut rng);
                u.refractory_ticks = 0;
                u
            });
            let mut inputs: [(i32, i32); LANES] =
                core::array::from_fn(|_| (rng.i32_edge_biased(), rng.i32_edge_biased()));
            for (lane, (u, input)) in units.iter_mut().zip(inputs.iter_mut()).enumerate() {
                // Lanes 0 and 4 at equality, 2 and 6 one LSB off; 1, 3, 5, 7 the plateau's.
                let offset = i32::from(lane & 2 != 0);
                if lane & 1 == 0 {
                    // The soma's landing, from a copy that cannot fire, in the working range.
                    u.v_soma = working(&mut rng);
                    u.v_basal = working(&mut rng);
                    u.v_apical = working(&mut rng);
                    *input = (working(&mut rng), working(&mut rng));
                    let mut probe = copy(u);
                    probe.v_thresh = i32::MIN;
                    probe.integrate(input.0, input.1, 0);
                    if (1..=THRESHOLD_BASE).contains(&probe.v_soma) {
                        u.v_thresh = probe.v_soma.saturating_add(offset);
                        count(&mut landed, offset == 0);
                        count(&mut short, offset == 1);
                    }
                } else {
                    // The apical potential at the plateau's threshold, or one LSB below it, on
                    // a unit that fires.
                    let mut probe = copy(u);
                    probe.integrate(input.0, 0, 0);
                    if let Some(apical) = BAC_APICAL_THRESHOLD
                        .saturating_sub(offset)
                        .checked_sub(probe.v_apical)
                    {
                        input.1 = apical;
                        u.v_thresh = 1;
                        u.v_soma = i32::MAX;
                        count(&mut started, offset == 0);
                        count(&mut below, offset == 1);
                    }
                }
            }
            both(&mut units, &inputs, rng.next_u32(), &mut seen);
        }
        assert!(
            landed > 1_000 && short > 1_000 && started > 1_000 && below > 1_000,
            "{landed} {short} {started} {below}"
        );
        assert!(seen.spikes > 4_000 && seen.plateaus > 1_000);
    }
}
