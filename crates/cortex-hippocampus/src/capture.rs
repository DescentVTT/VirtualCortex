//! Tagging from a spike train (whitepaper §5.2.15, §6.6; ADR-0048): the two rules that turn
//! what a network's own activity shows into the pattern of an [`Episode`](crate::Episode).
//! A train is a caller's slice of `(tick, unit)` sorted by tick, as the runtime's forks
//! report it. [`burst`] finds the densest span of a ripple's length; [`capture`] ranks the
//! units that fired within a span and takes the first [`PATTERN_MAX`]. Neither allocates,
//! and every loop is a walk over the slice.

use crate::PATTERN_MAX;

/// The densest span of a train: where it starts and how many spikes it holds.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Burst {
    /// The tick the span starts at: a spike's tick.
    pub from: u32,
    /// The spikes in `[from, from + window)`.
    pub spikes: u32,
}

/// The densest span of `window` ticks over `train`, a `(tick, unit)` slice sorted by tick:
/// the span holding the most spikes, the earliest at equal counts, every span starting at a
/// spike's tick (a span that starts between spikes holds no more than the one starting at
/// the next spike). `None` for an empty train or a window of zero.
pub fn burst(train: &[(u32, u32)], window: u32) -> Option<Burst> {
    if window == 0 {
        return None;
    }
    let mut best: Option<Burst> = None;
    for (i, &(from, _)) in train.iter().enumerate() {
        // Widened: `from + window` may pass the clock's width.
        let limit = (from as u64).saturating_add(window as u64);
        let spikes = train
            .get(i..)
            .map_or(0, |rest| {
                rest.iter()
                    .take_while(|&&(t, _)| (t as u64) < limit)
                    .count()
            })
            .min(u32::MAX as usize) as u32;
        let denser = match best {
            Some(b) => spikes > b.spikes,
            None => true,
        };
        if denser {
            best = Some(Burst { from, spikes });
        }
    }
    best
}

/// The rank of a unit in a span: its spikes, its first spike's tick, its index.
type Key = (u32, u32, u32);

/// True when `a` ranks before `b`: more spikes; at equal spikes an earlier first spike; at
/// equal first spikes a lower index.
const fn before(a: Key, b: Key) -> bool {
    a.0 > b.0 || (a.0 == b.0 && (a.1 < b.1 || (a.1 == b.1 && a.2 < b.2)))
}

/// The last slot of a pattern.
const LAST: usize = PATTERN_MAX - 1;

/// The pattern of the span `[from, from + window)` of `train`: the distinct units that fired
/// in it, ranked by their spikes in the span (most first), then by their first spike in it
/// (earliest first), then by index (lowest first); the first [`PATTERN_MAX`] of them written
/// to `out` in that order, the slots beyond them zero. Returns how many were written: 0 for a
/// span with no spike. Every unit written fired in the span and none is written twice, so
/// `Episode::tag` accepts the result whenever it is not empty.
pub fn capture(train: &[(u32, u32)], from: u32, window: u32, out: &mut [u32; PATTERN_MAX]) -> u8 {
    let limit = (from as u64).saturating_add(window as u64);
    let span = train
        .iter()
        .filter(|&&(t, _)| (t as u64) >= from as u64 && (t as u64) < limit);
    let mut keys = [(0u32, 0u32, 0u32); PATTERN_MAX];
    let mut len = 0usize;
    for &(tick, unit) in span.clone() {
        if keys[..len].iter().any(|k| k.2 == unit) {
            continue;
        }
        let spikes = span
            .clone()
            .filter(|&&(_, u)| u == unit)
            .count()
            .min(u32::MAX as usize) as u32;
        let first = span
            .clone()
            .find(|&&(_, u)| u == unit)
            .map_or(tick, |&(t, _)| t);
        let key = (spikes, first, unit);
        let at = keys[..len]
            .iter()
            .position(|&k| before(key, k))
            .unwrap_or(len);
        if at > LAST {
            continue;
        }
        // The slots from `at` to the last kept move one down; the last drops when the
        // pattern is full. Both indices stay within the array: `at <= LAST`.
        let kept = len.min(LAST);
        for j in (at..kept).rev() {
            keys[j.wrapping_add(1)] = keys[j];
        }
        keys[at] = key;
        len = len.wrapping_add(1).min(PATTERN_MAX);
    }
    for (slot, key) in out.iter_mut().zip(keys.iter()) {
        *slot = key.2;
    }
    for slot in out.iter_mut().skip(len) {
        *slot = 0;
    }
    len as u8
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Episode;

    /// Unit 5 fires three times, units 1 and 7 twice (1 first), unit 9 once; a spike before
    /// the span and one at its end are outside it.
    const TRAIN: [(u32, u32); 10] = [
        (5, 9),
        (10, 1),
        (11, 5),
        (12, 7),
        (13, 5),
        (14, 1),
        (15, 7),
        (16, 5),
        (17, 9),
        (30, 2),
    ];

    #[test]
    fn a_span_ranks_its_units_by_spikes_then_first_spike_then_index() {
        let mut out = [7u32; PATTERN_MAX];
        assert_eq!(capture(&TRAIN, 10, 20, &mut out), 4);
        assert_eq!(&out[..4], &[5, 1, 7, 9]);
        assert_eq!(&out[4..], &[0; 8], "the slots beyond are zero");
        assert!(Episode::tag(10, &out[..4], 1).is_some());
        // The span's bounds: `from` inclusive, `from + window` exclusive.
        assert_eq!(
            capture(&TRAIN, 5, 5, &mut out),
            1,
            "the spike at 5, not the one at 10"
        );
        assert_eq!(out[0], 9);
        assert_eq!(capture(&TRAIN, 5, 6, &mut out), 2);
        assert_eq!(&out[..2], &[9, 1], "one spike each: the earlier first");
        // Equal spikes and equal first ticks: the lower index.
        let tie = [(3, 8), (3, 2), (4, 8), (4, 2)];
        assert_eq!(capture(&tie, 0, 10, &mut out), 2);
        assert_eq!(&out[..2], &[2, 8]);
        // A later spike of a ranked unit is not ranked again.
        let again = [(1, 4), (2, 6), (3, 4)];
        assert_eq!(capture(&again, 0, 10, &mut out), 2);
        assert_eq!(&out[..2], &[4, 6]);
        // No spike in the span, an empty train, a window of zero.
        assert_eq!(capture(&TRAIN, 18, 12, &mut out), 0);
        assert_eq!(out, [0; PATTERN_MAX]);
        assert_eq!(capture(&[], 0, u32::MAX, &mut out), 0);
        assert_eq!(capture(&TRAIN, 10, 0, &mut out), 0);
        // The span's end is widened: a window past the clock's width reaches every spike.
        assert_eq!(capture(&TRAIN, 6, u32::MAX, &mut out), 5);
    }

    #[test]
    fn a_full_pattern_keeps_the_twelve_best_and_drops_the_worst_for_a_better_one() {
        // Fourteen units, one spike each in index order; then unit 20 fires twice, late.
        let mut train = [(0u32, 0u32); 16];
        for (i, spike) in train.iter_mut().enumerate().take(14) {
            *spike = (i as u32, i as u32);
        }
        train[14] = (50, 20);
        train[15] = (51, 20);
        let mut out = [0u32; PATTERN_MAX];
        assert_eq!(capture(&train[..14], 0, 100, &mut out), 12);
        let first_twelve: [u32; 12] = core::array::from_fn(|i| i as u32);
        assert_eq!(
            out, first_twelve,
            "the thirteenth and fourteenth, later, drop"
        );
        assert_eq!(capture(&train, 0, 100, &mut out), 12);
        assert_eq!(out[0], 20, "two spikes rank first");
        assert_eq!(&out[1..], &first_twelve[..11], "and the twelfth drops");
        // A thirteenth unit no better than the last is not written.
        let mut worse = train;
        worse[14] = (60, 20);
        worse[15] = (61, 21);
        assert_eq!(capture(&worse, 0, 100, &mut out), 12);
        assert_eq!(out, first_twelve);
    }

    #[test]
    fn the_burst_is_the_densest_span_starting_at_a_spike_the_earliest_at_equal_counts() {
        assert_eq!(
            burst(&TRAIN, 8),
            Some(Burst {
                from: 10,
                spikes: 8
            })
        );
        assert_eq!(
            burst(&TRAIN, 3),
            Some(Burst {
                from: 10,
                spikes: 3
            }),
            "10, 11, 12; the span at 11 also holds three and is later"
        );
        assert_eq!(
            burst(&TRAIN, 1),
            Some(Burst { from: 5, spikes: 1 }),
            "every span holds one: the first"
        );
        assert_eq!(
            burst(&TRAIN, u32::MAX),
            Some(Burst {
                from: 5,
                spikes: 10
            }),
            "a window past the width holds the whole train"
        );
        assert_eq!(burst(&TRAIN, 0), None);
        assert_eq!(burst(&[], 8), None);
        let late = [(u32::MAX, 1), (u32::MAX, 2)];
        assert_eq!(
            burst(&late, 5),
            Some(Burst {
                from: u32::MAX,
                spikes: 2
            }),
            "at the clock's width"
        );
    }
}

/// Property tests (ADR-0030): the capture against a brute-force ranking, the burst against
/// every start tick.
#[cfg(test)]
mod prop {
    use super::*;
    use crate::Episode;
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../testkit/prop.rs"
    ));

    /// A sorted train of up to 64 spikes over 24 units within 200 ticks.
    fn train(rng: &mut Lcg, out: &mut [(u32, u32); 64]) -> usize {
        let len = rng.below(65) as usize;
        for spike in out.iter_mut().take(len) {
            *spike = (rng.below(200), rng.below(24));
        }
        out[..len].sort_unstable_by_key(|&(t, _)| t);
        len
    }

    #[test]
    fn capture_equals_a_brute_force_ranking_and_burst_a_scan_of_every_start() {
        let mut rng = Lcg::new(53);
        let mut buffer = [(0u32, 0u32); 64];
        for round in 0..3_000u32 {
            let len = train(&mut rng, &mut buffer);
            let train = &buffer[..len];
            let from = rng.below(220);
            let window = rng.below(60);
            let limit = (from as u64).saturating_add(window as u64);
            // Every unit's key in the span, by brute force.
            let mut keys = [(0u32, 0u32, 0u32); 24];
            let mut distinct = 0usize;
            for unit in 0..24u32 {
                let mut spikes = 0u32;
                let mut first = u32::MAX;
                for &(t, u) in train {
                    if u == unit && (t as u64) >= from as u64 && (t as u64) < limit {
                        spikes = spikes.wrapping_add(1);
                        first = first.min(t);
                    }
                }
                if spikes > 0 {
                    keys[distinct] = (spikes, first, unit);
                    distinct = distinct.wrapping_add(1);
                }
            }
            keys[..distinct].sort_unstable_by(|&a, &b| {
                if before(a, b) {
                    core::cmp::Ordering::Less
                } else {
                    core::cmp::Ordering::Greater
                }
            });
            let mut out = [u32::MAX; PATTERN_MAX];
            let written = capture(train, from, window, &mut out) as usize;
            assert_eq!(written, distinct.min(PATTERN_MAX), "round {round}");
            for (slot, key) in out.iter().zip(keys.iter()).take(written) {
                assert_eq!(*slot, key.2, "round {round}: {out:?} against {keys:?}");
            }
            assert!(out[written..].iter().all(|&u| u == 0));
            if written > 0 {
                assert!(Episode::tag(from, &out[..written], 1).is_some());
            }
            // The burst against every start tick in the train's range.
            let found = burst(train, window);
            let mut best = 0usize;
            for start in 0..201u32 {
                let end = (start as u64).saturating_add(window as u64);
                let count = train
                    .iter()
                    .filter(|&&(t, _)| (t as u64) >= start as u64 && (t as u64) < end)
                    .count();
                best = best.max(count);
            }
            match found {
                Some(b) => {
                    assert_eq!(b.spikes as usize, best, "round {round}");
                    assert!(train.iter().any(|&(t, _)| t == b.from), "starts at a spike");
                    let end = (b.from as u64).saturating_add(window as u64);
                    let at_from = train
                        .iter()
                        .filter(|&&(t, _)| (t as u64) >= b.from as u64 && (t as u64) < end)
                        .count();
                    assert_eq!(at_from, best);
                    for &(t, _) in train.iter().take_while(|&&(t, _)| t < b.from) {
                        let end = (t as u64).saturating_add(window as u64);
                        let earlier = train
                            .iter()
                            .filter(|&&(s, _)| (s as u64) >= t as u64 && (s as u64) < end)
                            .count();
                        assert!(earlier < best, "round {round}: an earlier span as dense");
                    }
                }
                None => assert!(len == 0 || window == 0, "round {round}"),
            }
        }
    }
}
