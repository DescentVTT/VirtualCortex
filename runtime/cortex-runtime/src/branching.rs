//! The causal branching ratio by perturbation (ADR-0044; whitepaper §8.8, §8.12). The
//! whitepaper defines $\sigma$ causally, the spikes a window's spikes caused over the spikes
//! that caused them (`HomeostaticDrivePool::update_branching_ratio`), and nothing in the tree
//! attributed a spike to its cause; the executor's estimator (ADR-0036) reads the slope of
//! population activity instead. A deterministic engine can attribute exactly: two forks of one
//! image under one drive differ only by the one spike a kick adds to one of them, so every
//! spike in the perturbed fork and not in the baseline is that spike's descendant, and every
//! spike in the baseline and not in the perturbed fork is one it moved or removed. The
//! kicked unit's synapses are known, so a first-generation descendant is an extra spike of a
//! target within a latency after one of those synapses' delays from an ancestor spike,
//! whatever the delay bands; the rest of the difference is the later generations and the
//! two forks drifting apart. The forks run outside the tick loop, where the writer runs
//! (TC-5), and allocate their traces.

use crate::executor::{Config, Executor, InjectError};
use crate::image::{Image, ImageError};
use crate::synthesis::Drive;
use cortex_core::spike_message;
use cortex_homeostasis::HomeostaticDrivePool;

/// One kick: `messages` basal messages of `efficacy_q16` each into `unit` before `tick` runs.
/// Three strong messages (the executor's tests' kick) fire a unit at rest and leave enough in
/// the dendrite to fire it again after its refractory window; a unit a drive holds near its
/// threshold fires once on one message of about 1.5, and not at all when the drive left it
/// low, which the cascade reports as no ancestor.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Perturbation {
    pub unit: u32,
    pub tick: u64,
    pub messages: u32,
    pub efficacy_q16: i32,
}

impl Perturbation {
    /// Injects the kick, between ticks.
    pub fn inject(&self, inject: &crate::executor::Inject) -> Result<(), InjectError> {
        let message = spike_message(self.efficacy_q16, false);
        for _ in 0..self.messages {
            inject.inject(self.unit, message)?;
        }
        Ok(())
    }
}

/// Why a fork did not run to a trace.
#[derive(Debug)]
pub enum ForkError {
    /// The image did not decode.
    Image(ImageError),
    /// The drive or the kick was refused.
    Inject(InjectError),
    /// A worker dropped a spike from its trace: the configuration's `trace_capacity` is too
    /// small for the run, and a partial trace attributes nothing.
    Dropped(u64),
    /// A kick at a tick the fork had passed: the kicks are given in tick order, at or after
    /// the image's clock.
    Passed(u64),
    /// A kick at a tick beyond the run's end, which the fork would never reach.
    Unreached(u64),
}

impl From<ImageError> for ForkError {
    fn from(e: ImageError) -> Self {
        Self::Image(e)
    }
}

impl From<InjectError> for ForkError {
    fn from(e: InjectError) -> Self {
        Self::Inject(e)
    }
}

/// Runs `exec` from its clock to `until` under `drive`, the drive's messages injected before
/// each tick they are due.
pub fn run_driven<const CAP: usize>(
    exec: &mut Executor<CAP>,
    drive: &Drive,
    until: u64,
) -> Result<(), InjectError> {
    let inject = exec.injector();
    while exec.ticks() < until {
        drive.step(&inject, exec.ticks())?;
        exec.tick();
    }
    Ok(())
}

/// Every spike of `exec`, as `(tick, unit)` sorted, from the workers' traces; refused when a
/// trace dropped one. Consumes the executor.
pub fn trace<const CAP: usize>(exec: Executor<CAP>) -> Result<Vec<(u32, u32)>, ForkError> {
    let reports = exec.shutdown();
    let dropped: u64 = reports.iter().map(|r| r.dropped).sum();
    if dropped != 0 {
        return Err(ForkError::Dropped(dropped));
    }
    let mut spikes: Vec<(u32, u32)> = reports
        .iter()
        .flat_map(|r| r.spikes.iter().map(|&(u, t)| (t, u)))
        .collect();
    spikes.sort_unstable();
    Ok(spikes)
}

/// One fork of `image` under `config` and `drive`, run to `ticks`, with every kick of
/// `perturb` injected before its tick (the kicks in tick order; a kick before the fork's
/// clock is refused as a kick at a tick the fork has passed, one after `ticks` as a kick it
/// would never reach); its trace. No kick is the baseline.
pub fn fork<const CAP: usize>(
    image: &[u8],
    config: &Config,
    drive: &Drive,
    perturb: &[Perturbation],
    ticks: u64,
) -> Result<Vec<(u32, u32)>, ForkError> {
    let mut exec = Image::decode::<CAP>(image, config.clone())?;
    for p in perturb {
        if p.tick < exec.ticks() {
            return Err(ForkError::Passed(p.tick));
        }
        if p.tick > ticks {
            return Err(ForkError::Unreached(p.tick));
        }
        run_driven(&mut exec, drive, p.tick)?;
        p.inject(&exec.injector())?;
    }
    run_driven(&mut exec, drive, ticks)?;
    trace(exec)
}

/// What one kick caused, read from the two traces and the kicked unit's synapses.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Cascade {
    /// The tick the kicked unit fired at in the perturbed fork and not in the baseline: the
    /// ancestor. `None` when the kick added no spike of its own (the unit was refractory, or
    /// fired at that tick anyway), in which case nothing below is attributed.
    pub ancestor: Option<u32>,
    /// Ancestor spikes: the ancestor, and every later spike of the kicked unit in the
    /// perturbed fork and not in the baseline (the kick firing it again after its refractory
    /// window), each of which sends the unit's synapses.
    pub ancestors: u32,
    /// First-generation descendants: spikes in the perturbed fork and not in the baseline of
    /// a unit the kicked unit reaches, within `latency` ticks after one of its synapses'
    /// delays from one of the ancestor spikes.
    pub first: u32,
    /// First-generation descendants that the baseline holds as a spike of the same unit
    /// within `advance` ticks after the descendant and the perturbed fork lacks: spikes the
    /// kick moved earlier rather than added, so that `first - advanced` is what it added.
    pub advanced: u32,
    /// Spikes in the perturbed fork and not in the baseline after the ancestor that are not
    /// first-generation: the later generations, and the two forks drifting apart.
    pub extra: u32,
    /// Spikes in the baseline and not in the perturbed fork after the ancestor that no
    /// descendant accounts for: removed, or drift.
    pub missing: u32,
}

impl Cascade {
    /// `first - advanced`: the spikes the kick added to the first generation net of those it
    /// moved into it.
    pub const fn net_first(&self) -> i64 {
        (self.first as i64).wrapping_sub(self.advanced as i64)
    }
}

/// The cascade of a kick into `unit` at or after `at`, from the baseline's and the perturbed
/// fork's sorted traces and the kicked unit's synapses as `(target, delay)`. A spike of a
/// target in the perturbed fork and not in the baseline is a first-generation descendant
/// when it falls within `latency` ticks after an ancestor spike plus a synapse's delay to
/// that target (the message integrated, the unit fired); it is advanced rather than added
/// when the baseline has a spike of that unit within `advance` ticks after it that the
/// perturbed fork lacks. Spikes before the ancestor are the same in both forks and are not
/// counted.
pub fn cascade(
    baseline: &[(u32, u32)],
    perturbed: &[(u32, u32)],
    unit: u32,
    at: u32,
    synapses: &[(u32, u16)],
    latency: u32,
    advance: u32,
) -> Cascade {
    let extra = difference(perturbed, baseline);
    let mut missing = difference(baseline, perturbed);
    let Some(&(ancestor, _)) = extra.iter().find(|&&(tick, u)| u == unit && tick >= at) else {
        return Cascade::default();
    };
    let ancestors: Vec<u32> = extra
        .iter()
        .filter(|&&(tick, u)| u == unit && tick >= ancestor)
        .map(|&(tick, _)| tick)
        .collect();
    let mut cascade = Cascade {
        ancestor: Some(ancestor),
        ancestors: ancestors.len().min(u32::MAX as usize) as u32,
        ..Cascade::default()
    };
    // A missing spike paired with an advanced descendant is accounted for once.
    let mut paired = vec![false; missing.len()];
    for &(tick, u) in &extra {
        if tick < ancestor || u == unit {
            continue;
        }
        let first = ancestors.iter().any(|&a| {
            synapses.iter().any(|&(target, delay)| {
                target == u && {
                    let arrival = a.wrapping_add(delay as u32);
                    tick > arrival && tick.wrapping_sub(arrival) <= latency
                }
            })
        });
        if !first {
            cascade.extra = cascade.extra.saturating_add(1);
            continue;
        }
        cascade.first = cascade.first.saturating_add(1);
        if let Some(i) = missing.iter().enumerate().position(|(i, &(t, v))| {
            !paired[i] && v == u && t > tick && t.wrapping_sub(tick) <= advance
        }) {
            paired[i] = true;
            cascade.advanced = cascade.advanced.saturating_add(1);
        }
    }
    missing.retain(|&(tick, _)| tick >= ancestor);
    let paired_count = paired.iter().filter(|&&p| p).count();
    cascade.missing = missing
        .len()
        .saturating_sub(paired_count)
        .min(u32::MAX as usize) as u32;
    cascade
}

/// The spikes of `a` that are not in `b`, both sorted; a spike twice in `a` and once in `b`
/// counts once (a unit fires at most once per tick, so neither holds a duplicate).
fn difference(a: &[(u32, u32)], b: &[(u32, u32)]) -> Vec<(u32, u32)> {
    let mut out = Vec::new();
    let mut j = 0;
    for &x in a {
        while j < b.len() && b[j] < x {
            j = j.saturating_add(1);
        }
        if j < b.len() && b[j] == x {
            j = j.saturating_add(1);
        } else {
            out.push(x);
        }
    }
    out
}

/// The cascades with an ancestor, summed: what the record's causal rule takes.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Attribution {
    /// Kicks that added a spike.
    pub kicks: u32,
    /// Ancestor spikes, in all.
    pub ancestors: u32,
    /// First-generation descendants, in all.
    pub descendants: u32,
    /// Descendants the kicks advanced rather than added, in all.
    pub advanced: u32,
    /// Extra spikes not attributed to the first generation, in all.
    pub extra: u32,
    /// Missing spikes no descendant accounts for, in all.
    pub missing: u32,
}

impl Attribution {
    /// The sums over the cascades that have an ancestor; every count saturating.
    pub fn of(cascades: &[Cascade]) -> Self {
        let mut a = Self::default();
        for c in cascades.iter().filter(|c| c.ancestor.is_some()) {
            a.kicks = a.kicks.saturating_add(1);
            a.ancestors = a.ancestors.saturating_add(c.ancestors);
            a.descendants = a.descendants.saturating_add(c.first);
            a.advanced = a.advanced.saturating_add(c.advanced);
            a.extra = a.extra.saturating_add(c.extra);
            a.missing = a.missing.saturating_add(c.missing);
        }
        a
    }

    /// The causal branching ratio as `HomeostaticDrivePool::update_branching_ratio` computes
    /// it from these counts, descendants over ancestors, Q16.16 truncated; `None` for no
    /// ancestor, where the rule measures nothing.
    pub fn branching_ratio_q16(&self) -> Option<u32> {
        if self.ancestors == 0 {
            return None;
        }
        let mut pool = HomeostaticDrivePool::new();
        Some(pool.update_branching_ratio(self.descendants, self.ancestors))
    }

    /// The net causal branching ratio: descendants less those the kicks advanced, over the
    /// ancestors, Q16.16 rounded to nearest (a tie toward positive infinity), clamped to the
    /// width; `None` for no ancestor. In a driven network a kick advances spikes the drive
    /// would have produced, which the gross count reads as descendants; the net count is
    /// what the kick added to the first generation.
    pub fn net_branching_ratio_q16(&self) -> Option<i32> {
        if self.ancestors == 0 {
            return None;
        }
        let net = (self.descendants as i64).wrapping_sub(self.advanced as i64);
        // Both counts are below 2^32, so the shift fits `i64`; the divisor is at least one.
        let scaled = (net << 16).saturating_add(self.ancestors as i64 >> 1);
        Some(
            scaled
                .div_euclid(self.ancestors as i64)
                .clamp(i32::MIN as i64, i32::MAX as i64) as i32,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_cascade_attributes_the_extra_spikes_through_the_synapses_and_pairs_the_advanced_ones() {
        // Unit 4 reaches 5 at delay 90 and 6 at delay 100; unit 7 is reached by nobody here.
        let synapses = [(5, 90), (6, 100)];
        let baseline = [(5, 1), (10, 2), (150, 6), (300, 7), (400, 9)];
        // The kick fires 4 at 12; 5 fires at 105 (arrival 102, within 20); 6 at 115 (arrival
        // 112, within 20) and the baseline's 6 at 150 is gone: advanced; 7 at 420 is nobody's.
        let perturbed = [
            (5, 1),
            (10, 2),
            (12, 4),
            (105, 5),
            (115, 6),
            (300, 7),
            (420, 8),
        ];
        let c = cascade(&baseline, &perturbed, 4, 11, &synapses, 20, 100);
        assert_eq!(
            c,
            Cascade {
                ancestor: Some(12),
                ancestors: 1,
                first: 2,
                advanced: 1,
                extra: 1,
                missing: 1
            }
        );
        assert_eq!(c.net_first(), 1);
        assert_eq!(
            cascade(&baseline, &perturbed, 4, 13, &synapses, 20, 100).ancestor,
            None,
            "the kicked unit's extra spike is before `at`"
        );
        assert_eq!(
            cascade(&baseline, &perturbed, 2, 0, &synapses, 20, 100),
            Cascade::default()
        );
        assert_eq!(
            cascade(&baseline, &baseline, 1, 0, &synapses, 20, 100),
            Cascade::default()
        );
        // Too late after the arrival is not first-generation; a second ancestor spike opens a
        // second window.
        let late = [(12, 4), (140, 5)];
        let c = cascade(&[], &late, 4, 0, &synapses, 20, 100);
        assert_eq!((c.first, c.extra), (0, 1));
        let twice = [(12, 4), (250, 4), (345, 5)];
        let c = cascade(&[], &twice, 4, 0, &synapses, 20, 100);
        assert_eq!(
            (c.ancestors, c.first, c.extra),
            (2, 1, 0),
            "arrival 340 from the second"
        );
        // At the arrival tick exactly is not after it.
        let exact = [(12, 4), (102, 5), (103, 6)];
        let c = cascade(&[], &exact, 4, 0, &synapses, 20, 100);
        assert_eq!((c.first, c.extra), (0, 2));
        // A missing spike is paired once: two advanced descendants of one unit and one
        // missing spike leave one advanced; with two missing spikes in range the second
        // descendant takes the second (the review of brief 022 found the first draft
        // stopping at the first candidate, paired or not).
        let b = [(200, 5)];
        let p = [(12, 4), (105, 5), (109, 5)];
        let c = cascade(&b, &p, 4, 0, &synapses, 20, 100);
        assert_eq!((c.first, c.advanced, c.missing), (2, 1, 0));
        let b2 = [(200, 5), (210, 5)];
        let c = cascade(&b2, &p, 4, 0, &synapses, 20, 200);
        assert_eq!((c.first, c.advanced, c.missing), (2, 2, 0));
    }

    #[test]
    fn the_difference_is_the_set_difference_of_sorted_traces() {
        assert_eq!(
            difference(&[(1, 1), (2, 2), (3, 3)], &[(2, 2)]),
            vec![(1, 1), (3, 3)]
        );
        assert_eq!(difference(&[], &[(2, 2)]), vec![]);
        assert_eq!(difference(&[(2, 2)], &[]), vec![(2, 2)]);
        assert_eq!(difference(&[(2, 2)], &[(1, 1), (3, 3)]), vec![(2, 2)]);
    }

    #[test]
    fn the_attribution_sums_the_cascades_with_an_ancestor_and_the_ratios_follow() {
        let c = |a: Option<u32>, ancestors, first, advanced| Cascade {
            ancestor: a,
            ancestors,
            first,
            advanced,
            extra: 1,
            missing: 2,
        };
        assert_eq!(Attribution::of(&[]), Attribution::default());
        assert_eq!(Attribution::of(&[c(None, 9, 9, 9)]), Attribution::default());
        assert_eq!(Attribution::default().branching_ratio_q16(), None);
        assert_eq!(Attribution::default().net_branching_ratio_q16(), None);
        let a = Attribution::of(&[c(Some(1), 1, 1, 0), c(Some(2), 2, 2, 1), c(None, 9, 9, 9)]);
        assert_eq!(
            a,
            Attribution {
                kicks: 2,
                ancestors: 3,
                descendants: 3,
                advanced: 1,
                extra: 2,
                missing: 4
            }
        );
        assert_eq!(
            a.branching_ratio_q16(),
            Some(0x0001_0000),
            "three over three"
        );
        assert_eq!(
            a.net_branching_ratio_q16(),
            Some(0xAAAB),
            "two over three, to nearest"
        );
        let negative = Attribution::of(&[c(Some(1), 1, 0, 2), c(Some(1), 1, 1, 0)]);
        assert_eq!(negative.branching_ratio_q16(), Some(0x8000));
        assert_eq!(
            negative.net_branching_ratio_q16(),
            Some(-0x8000),
            "minus one over two"
        );
        let truncated = Attribution::of(&[c(Some(1), 3, 2, 0)]);
        assert_eq!(
            truncated.branching_ratio_q16(),
            Some(0xAAAA),
            "two over three, truncated as the rule truncates"
        );
        assert_eq!(truncated.net_branching_ratio_q16(), Some(0xAAAB));
    }
}
