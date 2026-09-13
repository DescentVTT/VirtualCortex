//! The anatomical prior of a synthesized network (ADR-0044; whitepaper §5.2.2, §8.7): a
//! seeded, integer-only rule that says which unit is inhibitory and, for every unit, where its
//! synapses go, with what weight, what delay and into which compartment. The prior is a rule,
//! not a record: it yields synapses to a caller that writes them into arenas (the runtime's
//! `synthesize`), and it depends on nothing (TC-2), so it names no record of another crate.
//!
//! The wiring is a ring lattice with a local window and a rewired fraction (Watts and
//! Strogatz 1998): unit $i$ sends each synapse to a unit within $\pm W$ of it on the ring,
//! with a delay from the local band, or, for a fraction $p$ of synapses, to a unit drawn
//! uniformly from the whole ring, with a delay from the far band (a local axon is short, a
//! long-range one long). Every $k$-th unit is inhibitory (Brunel 2000's sparse network has a
//! fifth), and an inhibitory unit's synapses carry a negative weight, since the delivery
//! encoding carries the sign and the membrane reads it; the unit's own flag is the reader's
//! annotation, not a rule's input. The same seed gives the same network on every target
//! (§8.3: seeded structural growth).

/// Knuth's MMIX linear congruential generator (Knuth 1997, vol. 2, §3.3.4), the property
/// kit's: 64-bit state, the high 32 bits as output. Full period for every seed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Lcg(u64);

impl Lcg {
    /// A generator at `seed`.
    pub const fn new(seed: u64) -> Self {
        Self(seed)
    }

    /// The next 64-bit state.
    pub fn next_u64(&mut self) -> u64 {
        self.0 = self
            .0
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        self.0
    }

    /// The next draw: the high 32 bits of the state.
    pub fn next_u32(&mut self) -> u32 {
        (self.next_u64() >> 32) as u32
    }

    /// A draw in `0..n`, by the product's high word; 0 for `n == 0`.
    pub fn below(&mut self, n: u32) -> u32 {
        ((self.next_u32() as u64).wrapping_mul(n as u64) >> 32) as u32
    }

    /// A draw in `lo..=hi` for `lo <= hi`; `lo` otherwise.
    pub fn between(&mut self, lo: u32, hi: u32) -> u32 {
        if hi <= lo {
            return lo;
        }
        // `hi - lo + 1` is at most `u32::MAX` when `lo` is at least 1; at `lo == 0` and
        // `hi == u32::MAX` the width wraps to 0 and `below` yields 0: a full-width draw is
        // `next_u32`, which no caller here needs.
        lo.wrapping_add(self.below(hi.wrapping_sub(lo).wrapping_add(1)))
    }
}

/// One synapse the prior yields: the unit it leaves, the unit it reaches, its weight (Q1.15,
/// negative from an inhibitory unit), its delay in fine ticks and its compartment.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Synapse {
    pub source: u32,
    pub target: u32,
    pub weight_q1_15: i16,
    pub delay_ticks: u16,
    pub apical: bool,
}

/// The parameters of a synthesized network. `is_well_formed` says which combinations the
/// rule accepts; a caller that refuses none of them writes a network the rule did not
/// define.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Prior {
    /// Units on the ring, at least 2.
    pub units: u32,
    /// Every this-many-th unit is inhibitory: unit $i$ when $(i + 1) \bmod k = 0$; 0 for
    /// none.
    pub inhibitory_every: u32,
    /// Synapses each unit sends, at least 1.
    pub synapses_per_unit: u32,
    /// The local window: a local synapse reaches a unit within this many places on the ring,
    /// on either side; at least 1 and below `units`.
    pub window: u32,
    /// The fraction of synapses, in 256ths, whose target is drawn from the whole ring.
    pub rewire_q0_8: u8,
    /// The local delay band, inclusive, in fine ticks, for a synapse within the window; at
    /// least 1.
    pub delay_min: u16,
    pub delay_max: u16,
    /// The far delay band, inclusive, for a rewired synapse; at least 1.
    pub far_delay_min: u16,
    pub far_delay_max: u16,
    /// The excitatory weight band, inclusive, Q1.15; at least 1.
    pub weight_min: i16,
    pub weight_max: i16,
    /// An inhibitory synapse's magnitude is the drawn excitatory weight times this, in
    /// sixteenths, saturating at the width.
    pub inhibitory_gain_q4_4: u8,
    /// The fraction of synapses, in 256ths, that land in the apical compartment.
    pub apical_q0_8: u8,
    /// The seed.
    pub seed: u64,
}

impl Prior {
    /// The bounds the rule needs: two units or more, a window of at least one that leaves a
    /// unit off the ring's far side, ordered non-empty bands, at least one synapse per unit.
    pub const fn is_well_formed(&self) -> bool {
        self.units >= 2
            && self.synapses_per_unit >= 1
            && self.window >= 1
            && self.window < self.units
            && self.delay_min >= 1
            && self.delay_min <= self.delay_max
            && self.far_delay_min >= 1
            && self.far_delay_min <= self.far_delay_max
            && self.weight_min >= 1
            && self.weight_min <= self.weight_max
    }

    /// True for a unit the rule makes inhibitory.
    pub const fn is_inhibitory(&self, unit: u32) -> bool {
        // `checked_rem` is `None` at a divisor of zero: no unit is inhibitory then.
        matches!(
            unit.wrapping_add(1).checked_rem(self.inhibitory_every),
            Some(0)
        )
    }

    /// The synapses of the whole network, unit by unit in index order, each unit's in
    /// slot order: `units × synapses_per_unit` of them. The generator is seeded once, so a
    /// synapse's draw depends on every synapse before it: the network is one walk.
    pub fn synapses(&self) -> Synapses {
        Synapses {
            prior: *self,
            lcg: Lcg::new(self.seed),
            unit: 0,
            slot: 0,
        }
    }

    /// The synapses of the whole network, counted by kind.
    pub fn census(&self) -> Census {
        let mut census = Census::default();
        for synapse in self.synapses() {
            census.count(self, &synapse);
        }
        census.inhibitory_units = (0..self.units)
            .filter(|&u| self.is_inhibitory(u))
            .count()
            .min(u32::MAX as usize) as u32;
        census
    }

    /// Where one synapse of `source` goes: a target within the window, or anywhere for a
    /// rewired one; never `source` itself (a self-synapse is moved one place along).
    fn target(&self, lcg: &mut Lcg, source: u32) -> (u32, bool) {
        let rewired = lcg.below(256) < self.rewire_q0_8 as u32;
        let target = if rewired {
            let t = lcg.below(self.units);
            if t == source {
                t.wrapping_add(1).checked_rem(self.units).unwrap_or(0)
            } else {
                t
            }
        } else {
            // A place in `1..=2W`: the first `W` to the right, the rest to the left.
            let place = lcg.between(1, self.window.saturating_mul(2));
            let offset = if place <= self.window {
                place
            } else {
                self.units.wrapping_sub(place.wrapping_sub(self.window))
            };
            source
                .wrapping_add(offset)
                .checked_rem(self.units)
                .unwrap_or(0)
        };
        (target, rewired)
    }
}

/// The walk of [`Prior::synapses`].
#[derive(Clone, Debug)]
pub struct Synapses {
    prior: Prior,
    lcg: Lcg,
    unit: u32,
    slot: u32,
}

impl Synapses {
    /// The walk's position: the `(unit, slot)` of the synapse it yields next.
    pub const fn position(&self) -> (u32, u32) {
        (self.unit, self.slot)
    }
}

impl Iterator for Synapses {
    type Item = Synapse;

    fn next(&mut self) -> Option<Synapse> {
        if !self.prior.is_well_formed() || self.unit >= self.prior.units {
            return None;
        }
        let source = self.unit;
        let (target, rewired) = self.prior.target(&mut self.lcg, source);
        let (lo, hi) = if rewired {
            (self.prior.far_delay_min, self.prior.far_delay_max)
        } else {
            (self.prior.delay_min, self.prior.delay_max)
        };
        let delay_ticks = self.lcg.between(lo as u32, hi as u32) as u16;
        let excitatory = self
            .lcg
            .between(self.prior.weight_min as u32, self.prior.weight_max as u32)
            as i32;
        let weight_q1_15 = if self.prior.is_inhibitory(source) {
            // `excitatory × gain / 16`, at most `i16::MAX` in magnitude, negated: the
            // product fits `i32` (`i16::MAX × 255`).
            let magnitude = (excitatory.saturating_mul(self.prior.inhibitory_gain_q4_4 as i32)
                >> 4)
                .min(i16::MAX as i32);
            magnitude.saturating_neg() as i16
        } else {
            excitatory as i16
        };
        let apical = self.lcg.below(256) < self.prior.apical_q0_8 as u32;
        self.slot = self.slot.wrapping_add(1);
        if self.slot >= self.prior.synapses_per_unit {
            self.slot = 0;
            self.unit = self.unit.wrapping_add(1);
        }
        Some(Synapse {
            source,
            target,
            weight_q1_15,
            delay_ticks,
            apical,
        })
    }
}

/// What a prior's walk yielded, counted.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Census {
    pub inhibitory_units: u32,
    pub synapses: u32,
    pub inhibitory_synapses: u32,
    /// Synapses whose target is outside the source's window on the ring: rewired ones that
    /// landed far, since a rewired synapse may land inside the window too.
    pub long_range: u32,
    pub apical: u32,
}

impl Census {
    /// Counts `synapse` under `prior`.
    pub fn count(&mut self, prior: &Prior, synapse: &Synapse) {
        self.synapses = self.synapses.saturating_add(1);
        if synapse.weight_q1_15 < 0 {
            self.inhibitory_synapses = self.inhibitory_synapses.saturating_add(1);
        }
        if synapse.apical {
            self.apical = self.apical.saturating_add(1);
        }
        if ring_distance(prior.units, synapse.source, synapse.target) > prior.window {
            self.long_range = self.long_range.saturating_add(1);
        }
    }
}

/// The places between `a` and `b` on a ring of `units`, the shorter way round; a place at or
/// past `units` is read modulo the ring, and a ring of no units has no distance.
pub const fn ring_distance(units: u32, a: u32, b: u32) -> u32 {
    // A place at or past `units` is read modulo the ring; `checked_rem` is `None` only at
    // zero units, which has no distance.
    let (Some(a), Some(b)) = (a.checked_rem(units), b.checked_rem(units)) else {
        return 0;
    };
    let direct = if a <= b {
        b.wrapping_sub(a)
    } else {
        a.wrapping_sub(b)
    };
    let around = units.wrapping_sub(direct);
    if direct < around { direct } else { around }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn prior() -> Prior {
        Prior {
            units: 64,
            inhibitory_every: 5,
            synapses_per_unit: 8,
            window: 4,
            rewire_q0_8: 64,
            delay_min: 100,
            delay_max: 200,
            far_delay_min: 300,
            far_delay_max: 400,
            weight_min: 1_000,
            weight_max: 2_000,
            inhibitory_gain_q4_4: 64,
            apical_q0_8: 32,
            seed: 7,
        }
    }

    /// Every synapse of a 64-unit, 8-synapse prior, in the walk's order.
    fn all(p: &Prior) -> [Synapse; 512] {
        let mut out = [Synapse {
            source: 0,
            target: 0,
            weight_q1_15: 0,
            delay_ticks: 0,
            apical: false,
        }; 512];
        let mut n = 0;
        for s in p.synapses() {
            out[n] = s;
            n = n.saturating_add(1);
        }
        assert_eq!(n, 512, "the walk ends after every synapse");
        out
    }

    #[test]
    fn the_walk_yields_every_synapse_of_every_unit_in_order_and_the_same_twice() {
        let p = prior();
        let all = all(&p);
        for (i, s) in all.iter().enumerate() {
            assert_eq!(s.source as usize, i / 8, "unit by unit");
            assert_ne!(s.source, s.target, "no self-synapse");
            assert!(s.target < 64);
            assert!((100..=200).contains(&s.delay_ticks) || (300..=400).contains(&s.delay_ticks));
            if p.is_inhibitory(s.source) {
                assert!(
                    (-8_000..=-4_000).contains(&s.weight_q1_15),
                    "{}",
                    s.weight_q1_15
                );
            } else {
                assert!((1_000..=2_000).contains(&s.weight_q1_15));
            }
        }
        assert_eq!(all, super::tests::all(&p), "one seed, one network");
        assert_ne!(all, super::tests::all(&Prior { seed: 8, ..p }));
    }

    #[test]
    fn a_local_synapse_lands_within_the_window_and_a_rewired_one_may_not() {
        let local = Prior {
            rewire_q0_8: 0,
            ..prior()
        };
        for s in local.synapses() {
            assert!(ring_distance(64, s.source, s.target) <= 4, "{s:?}");
            assert!((100..=200).contains(&s.delay_ticks), "a local delay");
        }
        assert_eq!(local.census().long_range, 0);
        let far = Prior {
            rewire_q0_8: 255,
            ..prior()
        };
        assert!(far.census().long_range > 64 * 8 / 2);
        let far_delays = far
            .synapses()
            .filter(|s| (300..=400).contains(&s.delay_ticks))
            .count();
        assert!(
            far_delays > 500,
            "a rewired synapse takes the far band, near or not: {far_delays}"
        );
        let mixed = prior();
        for s in mixed.synapses() {
            if ring_distance(64, s.source, s.target) > 4 {
                assert!(
                    (300..=400).contains(&s.delay_ticks),
                    "a far target has a far delay"
                );
            }
        }
    }

    #[test]
    fn the_census_counts_the_inhibitory_units_and_synapses_and_the_apical_ones() {
        let c = prior().census();
        assert_eq!(c.inhibitory_units, 12, "every fifth of 64: 4, 9, ..., 59");
        assert_eq!(c.synapses, 512);
        assert_eq!(c.inhibitory_synapses, 12 * 8);
        assert!(
            c.apical > 32 && c.apical < 96,
            "an eighth of 512: {}",
            c.apical
        );
        let none = Prior {
            inhibitory_every: 0,
            apical_q0_8: 0,
            ..prior()
        }
        .census();
        assert_eq!(
            (none.inhibitory_units, none.inhibitory_synapses, none.apical),
            (0, 0, 0)
        );
    }

    #[test]
    fn the_bounds_are_the_rule_s_and_a_malformed_prior_yields_nothing() {
        assert!(prior().is_well_formed());
        let bad = [
            Prior {
                units: 1,
                ..prior()
            },
            Prior {
                synapses_per_unit: 0,
                ..prior()
            },
            Prior {
                window: 0,
                ..prior()
            },
            Prior {
                window: 64,
                ..prior()
            },
            Prior {
                delay_min: 0,
                ..prior()
            },
            Prior {
                delay_min: 201,
                ..prior()
            },
            Prior {
                far_delay_min: 0,
                ..prior()
            },
            Prior {
                far_delay_min: 401,
                ..prior()
            },
            Prior {
                weight_min: 0,
                ..prior()
            },
            Prior {
                weight_min: 2_001,
                ..prior()
            },
        ];
        for p in bad {
            assert!(!p.is_well_formed(), "{p:?}");
            assert_eq!(p.synapses().next(), None);
        }
        assert!(
            Prior {
                window: 63,
                ..prior()
            }
            .is_well_formed()
        );
        assert!(
            Prior {
                delay_min: 200,
                ..prior()
            }
            .is_well_formed()
        );
        assert!(
            Prior {
                far_delay_min: 400,
                ..prior()
            }
            .is_well_formed()
        );
        assert!(
            Prior {
                weight_min: 2_000,
                ..prior()
            }
            .is_well_formed()
        );
    }

    #[test]
    fn the_inhibitory_magnitude_saturates_at_the_width_and_zero_gain_is_silent() {
        let strong = Prior {
            weight_min: 30_000,
            weight_max: 32_767,
            inhibitory_gain_q4_4: 255,
            ..prior()
        };
        let i = strong
            .synapses()
            .find(|s| strong.is_inhibitory(s.source))
            .unwrap();
        assert_eq!(i.weight_q1_15, -32_767);
        let silent = Prior {
            inhibitory_gain_q4_4: 0,
            ..prior()
        };
        let i = silent
            .synapses()
            .find(|s| silent.is_inhibitory(s.source))
            .unwrap();
        assert_eq!(i.weight_q1_15, 0);
        assert_eq!(
            silent.census().inhibitory_synapses,
            0,
            "a zero weight is not negative"
        );
    }

    #[test]
    fn ring_distance_is_the_shorter_way_round() {
        assert_eq!(ring_distance(64, 0, 63), 1);
        assert_eq!(ring_distance(64, 63, 0), 1);
        assert_eq!(ring_distance(64, 0, 32), 32);
        assert_eq!(ring_distance(64, 5, 5), 0);
        assert_eq!(ring_distance(0, 5, 9), 0);
        assert_eq!(ring_distance(10, 2, 9), 3);
        assert_eq!(ring_distance(10, 9, 2), 3);
        assert_eq!(ring_distance(10, 12, 9), 3, "read modulo the ring");
        assert_eq!(ring_distance(7, 0, 4), 3);
    }

    #[test]
    fn the_generator_s_draws_are_in_range() {
        let mut lcg = Lcg::new(1);
        for _ in 0..10_000 {
            assert!(lcg.below(7) < 7);
            let v = lcg.between(10, 20);
            assert!((10..=20).contains(&v));
        }
        assert_eq!(lcg.below(0), 0);
        assert_eq!(lcg.between(9, 3), 9);
        assert_eq!(Lcg::new(3), Lcg::new(3));
    }
}
