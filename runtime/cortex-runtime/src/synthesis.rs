//! Synthesis of a network from a prior, and a stationary drive for it (ADR-0044). The prior of
//! `cortex_connectome::Prior` yields synapses; [`synthesize`] writes them into an executor's
//! arenas unit by unit (the threshold at its base, the short-term-plasticity factors at rest,
//! the inhibitory flag as the unit's annotation, the blocks chained in index order) and
//! returns the census. [`Drive`] is an external input that is a function of the tick and a
//! seed alone, so two forks of one image under one drive receive the same messages at the
//! same ticks whatever else they do: the input trace of §8.3 as a rule instead of a list.
//! Both run outside the tick loop, where the writer runs (TC-5).

use crate::executor::{Inject, InjectError};
use cortex_connectome::{Census, Prior};
use cortex_core::{
    DendriticSuperNeuron, FLAG_INHIBITORY, MAX_CHAIN_INDEX, STP_MAX, STP_U, SYNAPSES_PER_BLOCK,
    SynapseBlock, THRESHOLD_BASE, WorkerWheel, spike_message,
};

/// Why a prior was not written.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SynthesisError {
    /// `Prior::is_well_formed` is false.
    Prior,
    /// The unit arena is not the prior's size.
    Units,
    /// The block arena holds fewer blocks than [`blocks_for`] the prior, or more than a chain
    /// index can name (`MAX_CHAIN_INDEX + 1`), which `link` would refuse part-way.
    Blocks,
    /// The prior's longest delay, in either band, is at or beyond the wheel's horizon, which
    /// the loader would refuse and the fan-out would abort on.
    Delay,
}

/// The blocks a prior needs: each unit's synapses in blocks of `SYNAPSES_PER_BLOCK`, the last
/// one part-filled.
pub const fn blocks_per_unit(prior: &Prior) -> u32 {
    prior.synapses_per_unit.div_ceil(SYNAPSES_PER_BLOCK as u32)
}

/// The blocks a prior needs in all.
pub const fn blocks_for(prior: &Prior) -> u64 {
    (prior.units as u64).saturating_mul(blocks_per_unit(prior) as u64)
}

/// Writes `prior` into `units` and `blocks`: every unit armed at its base threshold with the
/// short-term-plasticity factors at rest and `FLAG_INHIBITORY` set on the units the prior
/// makes inhibitory, its fan-out at block `unit × blocks_per_unit`, its blocks chained in
/// order; every synapse into the slot the walk names. Refused, with nothing written, for a
/// prior that is not well formed, a unit arena of another size, too few blocks, or a delay at
/// the horizon. Returns the census of what was written.
pub fn synthesize(
    units: &mut [DendriticSuperNeuron],
    blocks: &mut [SynapseBlock],
    prior: &Prior,
) -> Result<Census, SynthesisError> {
    if !prior.is_well_formed() {
        return Err(SynthesisError::Prior);
    }
    if units.len() as u64 != prior.units as u64 {
        return Err(SynthesisError::Units);
    }
    let per_unit = blocks_per_unit(prior);
    if (blocks.len() as u64) < blocks_for(prior)
        || blocks.len() as u64 > (MAX_CHAIN_INDEX as u64).saturating_add(1)
    {
        return Err(SynthesisError::Blocks);
    }
    if prior.delay_max.max(prior.far_delay_max) as u64 >= WorkerWheel::horizon_ticks() {
        return Err(SynthesisError::Delay);
    }
    let mut census = Census::default();
    for (i, unit) in units.iter_mut().enumerate() {
        unit.v_thresh = THRESHOLD_BASE;
        unit.stp_u_rel = STP_U;
        unit.stp_r_ves = STP_MAX;
        // Below `prior.units`, which the walk bounds below `u32::MAX` by its width.
        let index = i as u32;
        if prior.is_inhibitory(index) {
            unit.flags |= FLAG_INHIBITORY;
            census.inhibitory_units = census.inhibitory_units.saturating_add(1);
        }
        // `units × per_unit` blocks exist, checked above: the products fit and the indices
        // are in the arena.
        let first = index.saturating_mul(per_unit);
        if !unit.set_first_block(first) {
            return Err(SynthesisError::Blocks);
        }
        for k in 1..per_unit {
            let block = first.saturating_add(k);
            if !blocks[block.saturating_sub(1) as usize].link(block) {
                return Err(SynthesisError::Blocks);
            }
        }
    }
    for synapse in prior.synapses() {
        let slot = census
            .synapses
            .checked_rem(prior.synapses_per_unit)
            .unwrap_or(0);
        // `SYNAPSES_PER_BLOCK` is not zero: neither division is `None`.
        let block = synapse
            .source
            .saturating_mul(per_unit)
            .saturating_add(slot.checked_div(SYNAPSES_PER_BLOCK as u32).unwrap_or(0));
        if !blocks[block as usize].set_synapse(
            slot.checked_rem(SYNAPSES_PER_BLOCK as u32).unwrap_or(0) as usize,
            synapse.target,
            synapse.weight_q1_15,
            synapse.delay_ticks,
            synapse.apical,
        ) {
            return Err(SynthesisError::Blocks);
        }
        census.count(prior, &synapse);
    }
    Ok(census)
}

/// SplitMix64's finaliser (Steele, Lea and Flood 2014): a bijection of the 64-bit words
/// whose output for consecutive inputs is well mixed, so a schedule can draw from the tick.
pub const fn mix64(x: u64) -> u64 {
    let mut z = x.wrapping_add(0x9E37_79B9_7F4A_7C15);
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

/// A stationary external drive: on every tick that is a multiple of `every`, `messages`
/// basal messages of `efficacy_q16` each, into units drawn uniformly from `0..units` by
/// [`mix64`] of the seed, the tick and the message's index. A function of the tick alone,
/// so it needs no state and two forks under it are driven alike.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Drive {
    pub every: u32,
    pub messages: u32,
    pub efficacy_q16: i32,
    pub units: u32,
    pub seed: u64,
}

impl Drive {
    /// True on the ticks the drive delivers.
    pub const fn is_due(&self, tick: u64) -> bool {
        // `checked_rem` is `None` at a period of zero: a drive that is never due.
        matches!(tick.checked_rem(self.every as u64), Some(0))
    }

    /// The unit the `index`-th message of `tick` reaches.
    pub const fn unit_at(&self, tick: u64, index: u32) -> u32 {
        // The index rotated into the high half: a bijection of the index's 32 bits, where a
        // shift kept sixteen (the review of brief 022 found indices 65 536 apart colliding).
        let draw = mix64(
            self.seed ^ tick.wrapping_mul(0x0000_0001_0000_0001) ^ (index as u64).rotate_left(48),
        );
        ((draw >> 32).wrapping_mul(self.units as u64) >> 32) as u32
    }

    /// Injects the messages due at `tick`, between ticks, before the tick runs; they are
    /// integrated on the tick after the one that drains the ring. Returns how many were
    /// injected; an injector that refuses one stops the step there.
    pub fn step(&self, inject: &Inject, tick: u64) -> Result<u32, InjectError> {
        if !self.is_due(tick) || self.units == 0 {
            return Ok(0);
        }
        let message = spike_message(self.efficacy_q16, false);
        for index in 0..self.messages {
            inject.inject(self.unit_at(tick, index), message)?;
        }
        Ok(self.messages)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::executor::{Config, Executor};

    fn prior() -> Prior {
        Prior {
            units: 32,
            inhibitory_every: 4,
            synapses_per_unit: 6,
            window: 3,
            rewire_q0_8: 32,
            delay_min: 10,
            delay_max: 20,
            far_delay_min: 30,
            far_delay_max: 40,
            weight_min: 100,
            weight_max: 200,
            inhibitory_gain_q4_4: 32,
            apical_q0_8: 16,
            seed: 11,
        }
    }

    #[test]
    fn a_prior_is_written_unit_by_unit_with_the_blocks_chained_and_the_census_is_the_prior_s() {
        let p = prior();
        assert_eq!(blocks_per_unit(&p), 2);
        assert_eq!(blocks_for(&p), 64);
        let mut exec = Executor::<8>::new(Config {
            units: 32,
            blocks: 64,
            ..Config::default()
        })
        .unwrap();
        let (u, b) = exec.arenas_mut();
        let census = synthesize(u, b, &p).unwrap();
        assert_eq!(census, p.census());
        assert_eq!(census.inhibitory_units, 8);
        assert_eq!(census.synapses, 192);
        let mut seen = 0;
        for (i, unit) in exec.units().iter().enumerate() {
            assert_eq!(unit.v_thresh, THRESHOLD_BASE);
            assert_eq!((unit.stp_u_rel, unit.stp_r_ves), (STP_U, STP_MAX));
            assert_eq!(unit.flags & FLAG_INHIBITORY != 0, (i + 1) % 4 == 0);
            assert_eq!(unit.first_block(), Some(2 * i as u32));
            let chain: Vec<u32> = unit.chain(exec.blocks()).collect();
            assert_eq!(chain, vec![2 * i as u32, 2 * i as u32 + 1]);
            for s in unit.fan_out(exec.blocks()) {
                assert_eq!(
                    s.weight_q1_15 < 0,
                    (i + 1) % 4 == 0,
                    "an inhibitory unit's synapses are negative"
                );
                assert!((10..=20).contains(&s.delay_ticks) || (30..=40).contains(&s.delay_ticks));
                seen += 1;
            }
        }
        assert_eq!(
            seen, 192,
            "six synapses per unit: four in the first block, two in the second"
        );
        assert_eq!(
            exec.blocks()[1].target(2),
            None,
            "the last block's third slot is empty"
        );
        let mut walk = p.synapses();
        for unit in exec.units() {
            for s in unit.fan_out(exec.blocks()) {
                let w = walk.next().unwrap();
                assert_eq!(
                    (s.target, s.weight_q1_15, s.delay_ticks, s.apical),
                    (w.target, w.weight_q1_15, w.delay_ticks, w.apical)
                );
            }
        }
    }

    #[test]
    fn synthesis_is_refused_for_a_bad_prior_the_wrong_arenas_and_a_delay_at_the_horizon() {
        let p = prior();
        let make = |units, blocks| {
            Executor::<8>::new(Config {
                units,
                blocks,
                ..Config::default()
            })
            .unwrap()
        };
        let mut e = make(32, 64);
        let (u, b) = e.arenas_mut();
        assert_eq!(
            synthesize(u, b, &Prior { window: 0, ..p }),
            Err(SynthesisError::Prior)
        );
        let mut e = make(31, 64);
        let (u, b) = e.arenas_mut();
        assert_eq!(synthesize(u, b, &p), Err(SynthesisError::Units));
        let mut e = make(32, 63);
        let (u, b) = e.arenas_mut();
        assert_eq!(synthesize(u, b, &p), Err(SynthesisError::Blocks));
        let mut e = make(32, 64);
        let (u, b) = e.arenas_mut();
        assert_eq!(
            synthesize(
                u,
                b,
                &Prior {
                    delay_max: WorkerWheel::horizon_ticks() as u16,
                    ..p
                }
            ),
            Err(SynthesisError::Delay)
        );
        let (u, b) = e.arenas_mut();
        assert_eq!(
            synthesize(
                u,
                b,
                &Prior {
                    far_delay_max: WorkerWheel::horizon_ticks() as u16,
                    ..p
                }
            ),
            Err(SynthesisError::Delay),
            "either band"
        );
        assert!(e.units().iter().all(|u| u.v_thresh == 0), "nothing written");
        let (u, b) = e.arenas_mut();
        assert!(
            synthesize(
                u,
                b,
                &Prior {
                    delay_max: WorkerWheel::horizon_ticks() as u16 - 1,
                    ..p
                }
            )
            .is_ok()
        );
    }

    /// SplitMix64's outputs for a state of 0 (the published sequence: `e220a8397b1dcdaf`,
    /// `6e789e6aa1b965f4`), and two more computed by the reference algorithm outside the
    /// tree.
    #[test]
    fn mix64_is_splitmix64_s_finaliser() {
        assert_eq!(mix64(0), 0xE220_A839_7B1D_CDAF);
        assert_eq!(mix64(0x9E37_79B9_7F4A_7C15), 0x6E78_9E6A_A1B9_65F4);
        assert_eq!(mix64(1), 0x910A_2DEC_8902_5CC1);
        assert_eq!(mix64(u64::MAX), 0xE4D9_7177_1B65_2C20);
    }

    #[test]
    fn the_drive_is_a_function_of_the_tick_and_lands_in_range() {
        let d = Drive {
            every: 4,
            messages: 3,
            efficacy_q16: 0x1000,
            units: 10,
            seed: 5,
        };
        assert!(d.is_due(0) && d.is_due(8) && !d.is_due(6));
        assert!(!Drive { every: 0, ..d }.is_due(0));
        let mut counts = [0u32; 10];
        for tick in 0..4000u64 {
            for m in 0..3 {
                let u = d.unit_at(tick, m);
                assert!(u < 10);
                counts[u as usize] += 1;
                assert_eq!(u, d.unit_at(tick, m), "the same draw twice");
            }
        }
        assert!(counts.iter().all(|&c| c > 900 && c < 1500), "{counts:?}");
        assert_ne!(d.unit_at(1, 0), Drive { seed: 6, ..d }.unit_at(1, 0));
        let wide = Drive {
            units: 1 << 20,
            ..d
        };
        let collisions = (0..1000u64)
            .filter(|&t| wide.unit_at(t, 0) == wide.unit_at(t, 65_536))
            .count();
        assert!(
            collisions < 10,
            "indices 65 536 apart are distinct draws: {collisions}"
        );
        let mut exec = Executor::<8>::new(Config {
            units: 10,
            injector_capacity: 8,
            ..Config::default()
        })
        .unwrap();
        let inject = exec.injector();
        assert_eq!(d.step(&inject, 1), Ok(0));
        assert_eq!(d.step(&inject, 4), Ok(3));
        assert_eq!(Drive { units: 0, ..d }.step(&inject, 4), Ok(0));
        assert_eq!(
            Drive { messages: 9, ..d }.step(&inject, 8),
            Err(InjectError::Full),
            "a ring of eight with three in it refuses the sixth"
        );
        exec.run(2);
        assert_eq!(exec.delivered(), 8);
    }
}
