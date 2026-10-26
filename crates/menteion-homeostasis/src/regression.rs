//! The estimator's rule over a caller's series (whitepaper §5.2.16, §8.8; ADR-0047): the
//! population's spikes binned from a spike train at any width, and the lag-$k$ least-squares
//! slope of a series, of which the record's `estimate_branching_ratio` is the lag-one case
//! over a window's bins. A caller that holds a train can read the slope at a bin near the
//! generation time and at several lags, which the record cannot hold; whether it should is
//! the measurement ADR-0047 records.

use crate::{ACTIVITY_COUNT_MAX, SIGMA_MAX_Q16};

/// Spikes of `train` per bin: `out[i]` counts the spikes whose tick is in
/// `[from + i × width, from + (i + 1) × width)`, saturating at [`ACTIVITY_COUNT_MAX`]; a
/// spike before `from` or past the last bin is not counted; a width of zero counts nothing.
/// The train need not be sorted.
pub fn count_bins(train: &[(u32, u32)], from: u32, width: u32, out: &mut [u32]) {
    for bin in out.iter_mut() {
        *bin = 0;
    }
    for &(tick, _) in train {
        // Before `from`, or a width of zero: not counted.
        let Some(offset) = tick.checked_sub(from) else {
            continue;
        };
        let Some(bin) = offset.checked_div(width) else {
            continue;
        };
        if let Some(count) = out.get_mut(bin as usize) {
            *count = count.saturating_add(1).min(ACTIVITY_COUNT_MAX);
        }
    }
}

/// The lag-`lag` least-squares slope of `series`, Q16.16 in $[0, 16]$: over the $n$ pairs
/// $(a_t, a_{t+k})$, $\hat\sigma_k = (n \sum a_t a_{t+k} - \sum a_t \sum a_{t+k}) / (n \sum
/// a_t^2 - (\sum a_t)^2)$, rounded to nearest, with an intercept for the external drive. A
/// series without a spike is 0 (no descendants); an empty series, a lag of zero, fewer than
/// two pairs, or first members that never varied is no estimate (`None`); a negative slope
/// is 0; the slope saturates at [`SIGMA_MAX_Q16`]. Each count is read at most
/// [`ACTIVITY_COUNT_MAX`], so the sums of a series of up to $2^{32}$ bins fit `i128`. At lag
/// one over a window's bins this is exactly the record's `estimate_branching_ratio`, its
/// cases in its order.
pub fn slope_at_lag(series: &[u32], lag: usize) -> Option<u32> {
    if lag == 0 || series.is_empty() {
        return None;
    }
    if series.iter().all(|&count| count == 0) {
        return Some(0);
    }
    let pairs = series.len().checked_sub(lag)?;
    if pairs < 2 {
        return None;
    }
    let (mut sa, mut sb, mut saa, mut sab) = (0i128, 0i128, 0i128, 0i128);
    for (i, &first) in series.iter().enumerate().take(pairs) {
        // `i + lag` is below the length: `pairs` is the length less the lag.
        let &second = series.get(i.wrapping_add(lag))?;
        let a = first.min(ACTIVITY_COUNT_MAX) as i128;
        let b = second.min(ACTIVITY_COUNT_MAX) as i128;
        sa = sa.saturating_add(a);
        sb = sb.saturating_add(b);
        saa = saa.saturating_add(a.saturating_mul(a));
        sab = sab.saturating_add(a.saturating_mul(b));
    }
    if sa == 0 && sb == 0 {
        // The pairs hold no spike (a lag past half the length leaves bins between them).
        return Some(0);
    }
    let n = pairs as i128;
    let numerator = n.saturating_mul(sab).saturating_sub(sa.saturating_mul(sb));
    let denominator = n.saturating_mul(saa).saturating_sub(sa.saturating_mul(sa));
    if denominator <= 0 {
        return None;
    }
    if numerator <= 0 {
        return Some(0);
    }
    // The numerator is below 2^112 for the counts' cap and any series the width holds: the
    // shift cannot reach the width.
    let sigma = (numerator << 16)
        .saturating_add(denominator >> 1)
        .checked_div(denominator)
        .unwrap_or(0);
    Some(sigma.min(SIGMA_MAX_Q16 as i128) as u32)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ACTIVITY_WINDOW_BINS, HomeostaticDrivePool};

    #[test]
    fn a_geometric_series_has_the_slope_m_to_the_k_at_lag_k() {
        let halving = [1024u32, 512, 256, 128, 64, 32, 16, 8, 4, 2, 1];
        assert_eq!(slope_at_lag(&halving, 1), Some(0x8000), "0.5");
        assert_eq!(slope_at_lag(&halving, 2), Some(0x4000), "0.25");
        assert_eq!(slope_at_lag(&halving, 4), Some(0x1000), "0.0625");
        assert_eq!(
            slope_at_lag(&halving, 8),
            Some(0x0100),
            "three pairs: 1/256"
        );
        assert_eq!(slope_at_lag(&halving, 9), Some(0x0080), "two pairs: 1/512");
        assert_eq!(slope_at_lag(&halving, 10), None, "one pair");
        assert_eq!(slope_at_lag(&halving, 11), None, "no pair");
        assert_eq!(slope_at_lag(&halving, 0), None, "no lag");
        let doubling = [1u32, 2, 4, 8, 16];
        assert_eq!(slope_at_lag(&doubling, 1), Some(0x0002_0000), "2.0");
        assert_eq!(slope_at_lag(&doubling, 2), Some(0x0004_0000), "4.0");
        // With an intercept: a constant drive of 100 added to the halving series leaves the
        // slope at 0.5 exactly at lag one.
        let driven: [u32; 11] = core::array::from_fn(|i| halving[i].wrapping_add(100));
        assert_eq!(slope_at_lag(&driven, 1), Some(0x8000));
    }

    #[test]
    fn the_edges_no_spike_no_variance_a_negative_slope_and_the_ceiling() {
        assert_eq!(slope_at_lag(&[0, 0, 0, 0], 1), Some(0), "no spike");
        assert_eq!(
            slope_at_lag(&[0], 1),
            Some(0),
            "one silent bin, as the record reads it"
        );
        assert_eq!(slope_at_lag(&[0, 0], 3), Some(0), "silence at any lag");
        assert_eq!(slope_at_lag(&[], 1), None, "no bin");
        assert_eq!(slope_at_lag(&[4], 1), None, "one bin, no pair");
        assert_eq!(slope_at_lag(&[4, 6], 1), None, "one pair");
        assert_eq!(
            slope_at_lag(&[0, 0, 9, 0, 0], 3),
            Some(0),
            "the pairs hold no spike"
        );
        assert_eq!(
            slope_at_lag(&[0, 0, 0, 5], 1),
            None,
            "the first members never varied"
        );
        assert_eq!(slope_at_lag(&[5, 5, 5, 5], 1), None, "never varied");
        assert_eq!(
            slope_at_lag(&[3, 3, 3, 9], 1),
            None,
            "the first members never varied"
        );
        assert_eq!(
            slope_at_lag(&[9, 3, 3, 3], 1),
            Some(0),
            "the first members varied"
        );
        assert_eq!(
            slope_at_lag(&[1, 9, 2, 8, 3, 7], 1),
            Some(0),
            "a negative slope is 0"
        );
        assert_eq!(
            slope_at_lag(&[1, 100, 10_000], 1),
            Some(SIGMA_MAX_Q16),
            "100.0 reads as the ceiling"
        );
        let capped = [ACTIVITY_COUNT_MAX, u32::MAX, ACTIVITY_COUNT_MAX, u32::MAX];
        assert_eq!(
            slope_at_lag(&capped, 1),
            None,
            "read at the cap: constant first members"
        );
        assert_eq!(slope_at_lag(&[u32::MAX, 0, u32::MAX, 0], 1), Some(0));
        // Rounded to nearest: 2/3 rounds up to 0xAAAB, 9/11 down to 0xD174 (an oracle outside
        // the tree computed both); a slope of exactly zero is 0, not an absent estimate.
        assert_eq!(slope_at_lag(&[0, 1, 1, 1, 3], 1), Some(0xAAAB));
        assert_eq!(slope_at_lag(&[0, 0, 1, 2, 2], 1), Some(0xD174));
        assert_eq!(
            slope_at_lag(&[0, 1, 2, 1], 1),
            Some(0),
            "the pairs (0, 1), (1, 2), (2, 1)"
        );
    }

    #[test]
    fn at_lag_one_over_a_window_the_slope_is_the_record_s_estimate() {
        // A rise to a peak at bin 16 and a fall, with a period-four ripple: the slope at lag
        // one is 0.961 and at lag two 0.884 (an oracle outside the tree).
        let series: [u32; 32] = core::array::from_fn(|i| {
            let i = i as u32;
            40u32
                .saturating_sub(i.abs_diff(16).wrapping_mul(3))
                .wrapping_add(i.wrapping_rem(4))
        });
        assert_eq!(&series[..6], &[0, 1, 2, 4, 4, 8]);
        let mut pool = HomeostaticDrivePool::new();
        for &count in &series {
            pool.count_activity(count);
            assert!(pool.close_bin().is_some());
        }
        assert_eq!(pool.window_bins, ACTIVITY_WINDOW_BINS);
        assert_eq!(pool.estimate_branching_ratio(), slope_at_lag(&series, 1));
        assert_eq!(slope_at_lag(&series, 1), Some(62_974));
        assert_eq!(slope_at_lag(&series, 2), Some(57_924));
        // A silent window and a constant one agree too.
        let mut silent = HomeostaticDrivePool::new();
        let mut constant = HomeostaticDrivePool::new();
        for _ in 0..4 {
            silent.count_activity(0);
            silent.close_bin();
            constant.count_activity(7);
            constant.close_bin();
        }
        assert_eq!(silent.estimate_branching_ratio(), slope_at_lag(&[0; 4], 1));
        assert_eq!(
            constant.estimate_branching_ratio(),
            slope_at_lag(&[7; 4], 1)
        );
    }

    #[test]
    fn spikes_are_counted_into_bins_from_a_start_at_a_width_saturating() {
        let train = [
            (0u32, 1u32),
            (3, 2),
            (4, 3),
            (7, 4),
            (8, 5),
            (12, 6),
            (2, 7),
        ];
        let mut out = [9u32; 3];
        count_bins(&train, 0, 4, &mut out);
        assert_eq!(
            out,
            [3, 2, 1],
            "[0, 4), [4, 8), [8, 12); 12 is past the last bin"
        );
        count_bins(&train, 3, 4, &mut out);
        assert_eq!(out, [2, 2, 1], "from 3: [3, 7), [7, 11), [11, 15)");
        count_bins(&train, 0, 0, &mut out);
        assert_eq!(out, [0; 3], "a width of zero counts nothing");
        count_bins(&train, 100, 4, &mut out);
        assert_eq!(out, [0; 3], "every spike before the start");
        let mut none: [u32; 0] = [];
        count_bins(&train, 0, 4, &mut none);
        let mut one = [0u32; 1];
        count_bins(&[(u32::MAX, 0); 3], u32::MAX, 1, &mut one);
        assert_eq!(one, [3], "at the clock's width");
        let many: [(u32, u32); 40] = [(1, 0); 40];
        count_bins(&many, 0, 2, &mut one);
        assert_eq!(one, [40]);
        let mut cap = [ACTIVITY_COUNT_MAX.wrapping_sub(1); 1];
        count_bins(&[(0, 0), (0, 0), (0, 0)], 0, 1, &mut cap);
        assert_eq!(cap, [3], "the bins start at zero");
    }
}

/// Property tests (ADR-0030): the slope at lag one over a window's bins equals the record's
/// on random series, and the rule's bounds hold on the lattice.
#[cfg(test)]
mod prop {
    use super::*;
    use crate::{ACTIVITY_WINDOW_BINS, HomeostaticDrivePool};
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../testkit/prop.rs"
    ));

    #[test]
    fn the_record_and_the_slice_agree_on_every_window_and_every_slope_is_within_bounds() {
        let mut rng = Lcg::new(61);
        for round in 0..2_000u32 {
            let bins = rng.below(ACTIVITY_WINDOW_BINS as u32).wrapping_add(1) as usize;
            let mut series = [0u32; 32];
            let scale = [1u32, 16, 1 << 12, ACTIVITY_COUNT_MAX][rng.below(4) as usize];
            for count in series.iter_mut().take(bins) {
                *count = if rng.below(4) == 0 {
                    0
                } else {
                    rng.below(scale.saturating_add(1))
                };
            }
            let mut pool = HomeostaticDrivePool::new();
            for &count in &series[..bins] {
                pool.count_activity(count);
                assert!(pool.close_bin().is_some());
            }
            assert_eq!(
                pool.estimate_branching_ratio(),
                slope_at_lag(&series[..bins], 1),
                "round {round}: {:?}",
                &series[..bins]
            );
            for lag in 1..=4usize {
                if let Some(sigma) = slope_at_lag(&series[..bins], lag) {
                    assert!(sigma <= SIGMA_MAX_Q16);
                }
            }
        }
    }
}
