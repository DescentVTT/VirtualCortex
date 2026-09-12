//! Homeostasis: metabolic drive pools, the circadian sleep gate and criticality control
//! (whitepaper §5.2.16; ADR-0020, ADR-0036). Interoception is `cortex-affect`'s and hardware
//! vitals are `cortex-autonomic`'s (ADR-0016).
//!
//! Criticality control (ADR-0036) is a closed loop on the branching ratio $\sigma$, the number
//! of spikes each spike causes: the executor counts the population's spikes into bins of
//! $2^{12}$ ticks, a window of $2^5$ bins gives the lag-one least-squares slope of bin against
//! previous bin, which is $\sigma$ for a fully observed branching process (Wilting and
//! Priesemann 2018), and once per window a global synaptic gain moves by
//! $g \leftarrow g\,(1 - \kappa\,\operatorname{clamp}(\hat\sigma - 1, -1, 1))$, bounded, which
//! the turn holder applies to every unit's input sums: the whitepaper's
//! $W_{ij} \leftarrow W_{ij}[1 - \kappa(\sigma - 1)]$ as one factor per unit instead of a sweep
//! over every weight. At $\kappa = 0$ the gain is 1.0 and nothing moves.

#![no_std]
// §8.1: an operation on a state field saturates or wraps by name; plain arithmetic is refused
// here (ADR-0029; migrated under brief 016 on 2026-09-10).
#![deny(clippy::arithmetic_side_effects)]

/// 1.0 in Q16.16.
const Q16_ONE: i64 = 0x0001_0000;

/// A bin of population activity is $2^{12}$ ticks (40.96 ms at 10 µs): above the wheel's
/// horizon (2 560 ticks), so that every direct descendant of a bin's spikes lands in that bin
/// or the next and the lag-one slope sees it. A bin below the horizon would read a delayed
/// network as sub-critical, and the controller would raise the gain without bound.
pub const ACTIVITY_BIN_SHIFT: u32 = 12;
/// A window is $2^5$ bins (1.31 s); the gain moves once per window.
pub const ACTIVITY_WINDOW_SHIFT: u32 = 5;
/// The bins of a window: 32.
pub const ACTIVITY_WINDOW_BINS: u8 = 1 << ACTIVITY_WINDOW_SHIFT;
/// The most spikes one bin counts, $2^{24} - 1$: with it, a window's three sums fit `u64` and
/// the estimator's products fit `i64`.
pub const ACTIVITY_COUNT_MAX: u32 = 0x00FF_FFFF;
/// A gain of 1.0 in Q16.16: the reference dynamics.
pub const GAIN_ONE_Q16: u32 = 0x0001_0000;
/// The lowest gain the controller reaches, 0.25.
pub const GAIN_MIN_Q16: u32 = 0x4000;
/// The highest gain the controller reaches, 4.0.
pub const GAIN_MAX_Q16: u32 = 0x0004_0000;
/// The largest control step $\kappa$, 0.5 in Q0.16, so that one window moves the gain by half
/// at most and the factor $1 - \kappa e$ stays positive.
pub const CONTROL_STEP_MAX_Q0_16: u16 = 0x8000;
/// The largest branching ratio the estimator reports, 16.0.
pub const SIGMA_MAX_Q16: u32 = 0x0010_0000;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(C, align(64))]
pub struct HomeostaticDrivePool {
    pub energy_level: u32,        // [0..4] Energy reserve (Q16.16)
    pub sensory_fatigue: u32,     // [4..8] Accumulated synaptic load (Q16.16)
    pub curiosity_drive: u32, // [8..12] Intrinsic novelty seeking drive (Q16.16; the organism's one scalar, ADR-0016)
    pub bin_activity: u32, // [12..16] Spikes counted in the open bin, at most ACTIVITY_COUNT_MAX (ADR-0036)
    pub circadian_phase: u16, // [16..18] Internal phase angle, wraps at 16 bits
    pub sleep_mode_active: u8, // [18] 1 in SWR memory consolidation sleep, 0 awake
    pub window_bins: u8,   // [19] Bins closed in the open window, below ACTIVITY_WINDOW_BINS
    pub control_step_q0_16: u16, // [20..22] The control step kappa (Q0.16); 0 leaves the gain
    pub _reserved: u16,    // [22..24] Reserved; MUST be zero
    pub branching_ratio_q16: u32, // [24..28] Self-Organized Criticality sigma (~1.0), the last estimate
    pub synaptic_gain_q16: u32, // [28..32] The global synaptic gain (Q16.16), in [GAIN_MIN_Q16, GAIN_MAX_Q16]
    pub sum_prev: u64,          // [32..40] Sum of the previous bin over the window's pairs
    pub sum_prev_sq: u64,       // [40..48] Sum of its square
    pub sum_pair: u64,          // [48..56] Sum of previous bin times bin
    pub last_activity: u32,     // [56..60] The last closed bin's count
    pub first_activity: u32,    // [60..64] The window's first bin's count
}

impl HomeostaticDrivePool {
    /// The record at rest: every drive and count zero, the gain 1.0, no control step. Equal to
    /// `Default`. An all-zero record is not this one: its gain is 0, which the loader refuses.
    pub const fn new() -> Self {
        Self {
            energy_level: 0,
            sensory_fatigue: 0,
            curiosity_drive: 0,
            bin_activity: 0,
            circadian_phase: 0,
            sleep_mode_active: 0,
            window_bins: 0,
            control_step_q0_16: 0,
            _reserved: 0,
            branching_ratio_q16: 0,
            synaptic_gain_q16: GAIN_ONE_Q16,
            sum_prev: 0,
            sum_prev_sq: 0,
            sum_pair: 0,
            last_activity: 0,
            first_activity: 0,
        }
    }

    /// The branching ratio of self-organised criticality (ADR-0020, whitepaper §8.12) in its
    /// causal form: `sigma = descendants / ancestors` in Q16.16, the spikes a window's spikes
    /// caused divided by the spikes that caused them; 1.0 is criticality. Widened, saturating;
    /// a window with no ancestors leaves `sigma` unchanged, since nothing was measured. Returns
    /// `sigma`. The executor's estimator is [`regulate`](Self::regulate), which needs no
    /// attribution of a spike to its cause.
    pub fn update_branching_ratio(&mut self, descendants: u32, ancestors: u32) -> u32 {
        // A window with no ancestors measured nothing: the division is refused and `sigma`
        // stays as it was.
        let Some(sigma) = ((descendants as u64) << 16).checked_div(ancestors as u64) else {
            return self.branching_ratio_q16;
        };
        self.branching_ratio_q16 = sigma.min(u32::MAX as u64) as u32;
        self.branching_ratio_q16
    }

    /// Advances the 16-bit circadian phase and sets the sleep gate. The phase is a counter
    /// whose wrap is the intended semantics (whitepaper §8.1), so the addition is explicitly
    /// wrapping: the low sixteen bits of the delta are what a sixteen-bit phase can take.
    #[inline(always)]
    pub fn update_circadian_tick(&mut self, dt_ticks: u32) {
        self.circadian_phase = self.circadian_phase.wrapping_add(dt_ticks as u16);
        // Sleep phase active when fatigue exceeds threshold or in nocturnal phase
        if self.sensory_fatigue > 0x8000_0000 || self.circadian_phase > 0xC000 {
            self.sleep_mode_active = 1;
        } else {
            self.sleep_mode_active = 0;
        }
    }

    /// Counts `spikes` into the open bin, saturating at [`ACTIVITY_COUNT_MAX`]. Returns the
    /// bin's count.
    pub fn count_activity(&mut self, spikes: u32) -> u32 {
        self.bin_activity = self
            .bin_activity
            .saturating_add(spikes)
            .min(ACTIVITY_COUNT_MAX);
        self.bin_activity
    }

    /// Closes the open bin into the window: the first bin of a window is remembered, every
    /// later one pairs with the bin before it into the three sums; the open bin starts again
    /// at zero. Returns the bins of the window, or `None`, with nothing changed, when the
    /// window is full ([`ACTIVITY_WINDOW_BINS`]): a full window is regulated before another
    /// bin closes.
    pub fn close_bin(&mut self) -> Option<u8> {
        if self.window_bins >= ACTIVITY_WINDOW_BINS {
            return None;
        }
        let bin = self.bin_activity.min(ACTIVITY_COUNT_MAX);
        self.bin_activity = 0;
        if self.window_bins == 0 {
            self.first_activity = bin;
        } else {
            let a = self.last_activity as u64;
            let b = bin as u64;
            // Each count is at most 2^24 - 1, each product below 2^48, and a window holds at
            // most 31 pairs: the sums stay far below the width, and saturate by name.
            self.sum_prev = self.sum_prev.saturating_add(a);
            self.sum_prev_sq = self.sum_prev_sq.saturating_add(a.saturating_mul(a));
            self.sum_pair = self.sum_pair.saturating_add(a.saturating_mul(b));
        }
        self.last_activity = bin;
        // Below the window's bins, checked above: the increment cannot wrap.
        self.window_bins = self.window_bins.wrapping_add(1);
        Some(self.window_bins)
    }

    /// The branching ratio of the open window, Q16.16 in $[0, 16]$: the lag-one least-squares
    /// slope of a bin against the bin before it, with an intercept for the external drive,
    /// $\hat\sigma = (n \sum ab - \sum a \sum b) / (n \sum a^2 - (\sum a)^2)$ over the $n$ pairs,
    /// rounded to nearest, which is $\sigma$ for a fully observed branching process (Wilting and
    /// Priesemann 2018). A window without a spike is $\hat\sigma = 0$ (no descendants); a
    /// window whose activity never varied, or with fewer than two pairs, is no estimate
    /// (`None`); a negative slope is 0.
    pub fn estimate_branching_ratio(&self) -> Option<u32> {
        if self.window_bins == 0 {
            return None;
        }
        if self.sum_prev == 0 && self.last_activity == 0 {
            return Some(0);
        }
        let n = self.window_bins.wrapping_sub(1) as i64;
        let sa = self.sum_prev as i64;
        let saa = self.sum_prev_sq as i64;
        let sab = self.sum_pair as i64;
        // The bins after the first, which the pairs' second members are: the first bin out,
        // the last bin in.
        let sb = sa
            .saturating_sub(self.first_activity as i64)
            .saturating_add(self.last_activity as i64);
        let numerator = n.saturating_mul(sab).saturating_sub(sa.saturating_mul(sb));
        let denominator = n.saturating_mul(saa).saturating_sub(sa.saturating_mul(sa));
        if denominator <= 0 {
            return None;
        }
        if numerator <= 0 {
            return Some(0);
        }
        // Both below 2^61 for a well-formed record; the shift needs the width of i128.
        let scaled = (numerator as i128) << 16;
        let sigma = scaled
            .saturating_add((denominator as i128) >> 1)
            .checked_div(denominator as i128)
            .unwrap_or(0);
        Some(sigma.min(SIGMA_MAX_Q16 as i128) as u32)
    }

    /// True when the window's activity is at or above `saturation_per_bin` in every bin on
    /// average: a saturated network's bins no longer form a branching process (every unit
    /// fires as often as its refractory window lets it), so the slope is not $\sigma$ there;
    /// the caller names the ceiling (the executor's is a quarter of the refractory-limited
    /// maximum), and a window at or above it is regulated as supercritical.
    pub fn is_saturated(&self, saturation_per_bin: u32) -> bool {
        if self.window_bins == 0 {
            return false;
        }
        let total = self.sum_prev.saturating_add(self.last_activity as u64);
        total >= (saturation_per_bin as u64).saturating_mul(self.window_bins as u64)
    }

    /// Once per window: with an estimate (the window's slope, or [`SIGMA_MAX_Q16`] for a window
    /// at or above `saturation_per_bin`), stores it as `branching_ratio_q16` and moves the gain
    /// by $g \leftarrow g\,(1 - \kappa\,\operatorname{clamp}(\hat\sigma - 1, -1, 1))$, the
    /// product rounded to nearest and clamped to $[$[`GAIN_MIN_Q16`]$, $[`GAIN_MAX_Q16`]$]$,
    /// with $\kappa$ the record's `control_step_q0_16`; without one, leaves the gain. Either way
    /// the window's pairs are cleared for the next window. Returns the gain when an estimate
    /// moved it (or left it, at $\kappa = 0$), `None` when there was none.
    pub fn regulate(&mut self, saturation_per_bin: u32) -> Option<u32> {
        let estimate = if self.is_saturated(saturation_per_bin) {
            Some(SIGMA_MAX_Q16)
        } else {
            self.estimate_branching_ratio()
        };
        if let Some(sigma) = estimate {
            self.branching_ratio_q16 = sigma;
            let error = (sigma as i64)
                .saturating_sub(Q16_ONE)
                .clamp(-Q16_ONE, Q16_ONE);
            // kappa (Q0.16) times the error (Q16.16), rounded to nearest, is Q16.16.
            let step = (self.control_step_q0_16 as i64)
                .saturating_mul(error)
                .saturating_add(0x8000)
                >> 16;
            let factor = Q16_ONE.saturating_sub(step);
            let gain = (self.synaptic_gain_q16 as i64)
                .saturating_mul(factor)
                .saturating_add(0x8000)
                >> 16;
            self.synaptic_gain_q16 = gain.clamp(GAIN_MIN_Q16 as i64, GAIN_MAX_Q16 as i64) as u32;
        }
        self.sum_prev = 0;
        self.sum_prev_sq = 0;
        self.sum_pair = 0;
        self.first_activity = 0;
        self.last_activity = 0;
        self.window_bins = 0;
        estimate.map(|_| self.synaptic_gain_q16)
    }

    /// True for a record the rules can produce between windows: the gain within its bounds, the
    /// control step within its, fewer bins than a window holds (a full window is regulated in
    /// the step that fills it), every count at or below the cap, every sum at or below what
    /// the pairs and the cap allow, no first or last bin without a bin, the sleep flag 0 or 1,
    /// the reserved bytes zero. The loader refuses a record that is not (ADR-0028).
    pub fn is_well_formed(&self) -> bool {
        let max = ACTIVITY_COUNT_MAX as u64;
        let pairs = (self.window_bins as u64).saturating_sub(1);
        let square_bound = pairs.saturating_mul(max.saturating_mul(max));
        (GAIN_MIN_Q16..=GAIN_MAX_Q16).contains(&self.synaptic_gain_q16)
            && self.control_step_q0_16 <= CONTROL_STEP_MAX_Q0_16
            && self.window_bins < ACTIVITY_WINDOW_BINS
            && self.bin_activity <= ACTIVITY_COUNT_MAX
            && self.last_activity <= ACTIVITY_COUNT_MAX
            && self.first_activity <= ACTIVITY_COUNT_MAX
            && self.sum_prev <= pairs.saturating_mul(max)
            && self.sum_prev_sq <= square_bound
            && self.sum_pair <= square_bound
            && (self.window_bins > 0 || (self.first_activity == 0 && self.last_activity == 0))
            && self.sleep_mode_active <= 1
            && self._reserved == 0
    }

    /// The record's 64 bytes, little-endian, field by field (§8.7).
    pub fn encode(&self) -> [u8; 64] {
        let mut out = [0u8; 64];
        out[0..4].copy_from_slice(&self.energy_level.to_le_bytes());
        out[4..8].copy_from_slice(&self.sensory_fatigue.to_le_bytes());
        out[8..12].copy_from_slice(&self.curiosity_drive.to_le_bytes());
        out[12..16].copy_from_slice(&self.bin_activity.to_le_bytes());
        out[16..18].copy_from_slice(&self.circadian_phase.to_le_bytes());
        out[18] = self.sleep_mode_active;
        out[19] = self.window_bins;
        out[20..22].copy_from_slice(&self.control_step_q0_16.to_le_bytes());
        out[22..24].copy_from_slice(&self._reserved.to_le_bytes());
        out[24..28].copy_from_slice(&self.branching_ratio_q16.to_le_bytes());
        out[28..32].copy_from_slice(&self.synaptic_gain_q16.to_le_bytes());
        out[32..40].copy_from_slice(&self.sum_prev.to_le_bytes());
        out[40..48].copy_from_slice(&self.sum_prev_sq.to_le_bytes());
        out[48..56].copy_from_slice(&self.sum_pair.to_le_bytes());
        out[56..60].copy_from_slice(&self.last_activity.to_le_bytes());
        out[60..64].copy_from_slice(&self.first_activity.to_le_bytes());
        out
    }

    /// A record from its 64 bytes; not validated (`is_well_formed` is the check).
    pub fn decode(bytes: &[u8; 64]) -> Self {
        let u32_at = |at: usize| {
            u32::from_le_bytes(bytes[at..at.wrapping_add(4)].try_into().unwrap_or([0; 4]))
        };
        let u64_at = |at: usize| {
            u64::from_le_bytes(bytes[at..at.wrapping_add(8)].try_into().unwrap_or([0; 8]))
        };
        let u16_at = |at: usize| {
            u16::from_le_bytes(bytes[at..at.wrapping_add(2)].try_into().unwrap_or([0; 2]))
        };
        Self {
            energy_level: u32_at(0),
            sensory_fatigue: u32_at(4),
            curiosity_drive: u32_at(8),
            bin_activity: u32_at(12),
            circadian_phase: u16_at(16),
            sleep_mode_active: bytes[18],
            window_bins: bytes[19],
            control_step_q0_16: u16_at(20),
            _reserved: u16_at(22),
            branching_ratio_q16: u32_at(24),
            synaptic_gain_q16: u32_at(28),
            sum_prev: u64_at(32),
            sum_prev_sq: u64_at(40),
            sum_pair: u64_at(48),
            last_activity: u32_at(56),
            first_activity: u32_at(60),
        }
    }
}

impl Default for HomeostaticDrivePool {
    fn default() -> Self {
        Self::new()
    }
}

const _: () = {
    assert!(core::mem::size_of::<HomeostaticDrivePool>() == 64);
    assert!(core::mem::align_of::<HomeostaticDrivePool>() == 64);
    assert!(ACTIVITY_WINDOW_BINS == 32);
    assert!(GAIN_MIN_Q16 < GAIN_ONE_Q16 && GAIN_ONE_Q16 < GAIN_MAX_Q16);
    // The cap keeps a window's sums within u64 and the estimator's products within i64:
    // 31 pairs of squares below 2^48, then a product with n or with another sum below 2^61.
    assert!((ACTIVITY_COUNT_MAX as u64) < (1 << 24));
    assert!(CONTROL_STEP_MAX_Q0_16 <= 0x8000);
};

#[cfg(test)]
mod tests {
    use super::*;

    const ONE: u32 = GAIN_ONE_Q16;

    #[test]
    fn the_branching_ratio_is_descendants_over_ancestors_and_an_empty_window_measures_nothing() {
        let mut p = pool(0, 0);
        p.branching_ratio_q16 = 0x1234;
        assert_eq!(p.update_branching_ratio(100, 100), 0x0001_0000, "critical");
        assert_eq!(
            p.update_branching_ratio(50, 100),
            0x0000_8000,
            "subcritical"
        );
        assert_eq!(
            p.update_branching_ratio(300, 100),
            0x0003_0000,
            "supercritical"
        );
        assert_eq!(
            p.update_branching_ratio(7, 0),
            0x0003_0000,
            "no ancestors: unchanged"
        );
        assert_eq!(p.update_branching_ratio(u32::MAX, 1), u32::MAX, "saturates");
        assert_eq!(p.update_branching_ratio(0, 5), 0, "an avalanche that died");
    }

    fn pool(circadian_phase: u16, sensory_fatigue: u32) -> HomeostaticDrivePool {
        HomeostaticDrivePool {
            sensory_fatigue,
            circadian_phase,
            ..HomeostaticDrivePool::new()
        }
    }

    #[test]
    fn the_record_is_one_line_and_rest_has_a_gain_of_one() {
        assert_eq!(core::mem::size_of::<HomeostaticDrivePool>(), 64);
        assert_eq!(core::mem::align_of::<HomeostaticDrivePool>(), 64);
        let p = HomeostaticDrivePool::new();
        assert_eq!(p, HomeostaticDrivePool::default());
        assert_eq!(p.synaptic_gain_q16, ONE);
        assert_eq!(p.control_step_q0_16, 0);
        assert!(p.is_well_formed());
        assert!(
            !HomeostaticDrivePool::decode(&[0; 64]).is_well_formed(),
            "an all-zero record has a gain of zero: not the record at rest"
        );
        assert_eq!(
            (
                ACTIVITY_BIN_SHIFT,
                ACTIVITY_WINDOW_SHIFT,
                ACTIVITY_WINDOW_BINS,
                ACTIVITY_COUNT_MAX
            ),
            (12, 5, 32, 0x00FF_FFFF)
        );
        assert_eq!(
            (
                GAIN_MIN_Q16,
                GAIN_MAX_Q16,
                CONTROL_STEP_MAX_Q0_16,
                SIGMA_MAX_Q16
            ),
            (0x4000, 0x0004_0000, 0x8000, 0x0010_0000)
        );
    }

    #[test]
    fn phase_wraps_at_sixteen_bits() {
        let mut p = pool(0xFFFF, 0);
        p.update_circadian_tick(1);
        assert_eq!(p.circadian_phase, 0);
        assert_eq!(p.sleep_mode_active, 0);
    }

    #[test]
    fn phase_wraps_for_the_largest_tick_delta() {
        let mut p = pool(0xFFFF, 0);
        p.update_circadian_tick(u32::MAX);
        assert_eq!(p.circadian_phase, 0xFFFE);
        assert_eq!(p.sleep_mode_active, 1);
        let mut q = pool(5, 0);
        q.update_circadian_tick(0x0001_0002);
        assert_eq!(q.circadian_phase, 7, "the low sixteen bits of the delta");
    }

    #[test]
    fn sleep_gate_opens_one_tick_past_three_quarters() {
        let mut p = pool(0xBFFF, 0);
        p.update_circadian_tick(1);
        assert_eq!(p.circadian_phase, 0xC000);
        assert_eq!(p.sleep_mode_active, 0);
        p.update_circadian_tick(1);
        assert_eq!(p.circadian_phase, 0xC001);
        assert_eq!(p.sleep_mode_active, 1);
    }

    #[test]
    fn fatigue_at_exactly_half_does_not_force_sleep() {
        let mut p = pool(0, 0x8000_0000);
        p.update_circadian_tick(0);
        assert_eq!(p.sleep_mode_active, 0);
        let mut q = pool(0, 0x8000_0001);
        q.update_circadian_tick(0);
        assert_eq!(q.sleep_mode_active, 1);
    }

    #[test]
    fn a_bin_counts_spikes_up_to_the_cap() {
        let mut p = HomeostaticDrivePool::new();
        assert_eq!(p.count_activity(0), 0);
        assert_eq!(p.count_activity(5), 5);
        assert_eq!(p.count_activity(7), 12);
        assert_eq!(
            p.count_activity(ACTIVITY_COUNT_MAX),
            ACTIVITY_COUNT_MAX,
            "capped"
        );
        assert_eq!(p.count_activity(u32::MAX), ACTIVITY_COUNT_MAX);
        let mut q = HomeostaticDrivePool::new();
        assert_eq!(
            q.count_activity(ACTIVITY_COUNT_MAX),
            ACTIVITY_COUNT_MAX,
            "exactly the cap"
        );
        assert_eq!(
            q.count_activity(1),
            ACTIVITY_COUNT_MAX,
            "one more is the cap"
        );
        assert!(q.is_well_formed());
    }

    /// Closes bins of these counts into a fresh record's window.
    fn window(counts: &[u32]) -> HomeostaticDrivePool {
        let mut p = HomeostaticDrivePool::new();
        for &c in counts {
            p.count_activity(c);
            assert!(p.close_bin().is_some());
        }
        p
    }

    #[test]
    fn a_closed_bin_pairs_with_the_bin_before_it_and_a_full_window_takes_no_more() {
        let mut p = HomeostaticDrivePool::new();
        p.count_activity(10);
        assert_eq!(p.close_bin(), Some(1));
        assert_eq!(
            (
                p.first_activity,
                p.last_activity,
                p.bin_activity,
                p.window_bins
            ),
            (10, 10, 0, 1),
            "the first bin is remembered and pairs with nothing"
        );
        assert_eq!((p.sum_prev, p.sum_prev_sq, p.sum_pair), (0, 0, 0));
        p.count_activity(3);
        assert_eq!(p.close_bin(), Some(2));
        assert_eq!((p.first_activity, p.last_activity), (10, 3));
        assert_eq!((p.sum_prev, p.sum_prev_sq, p.sum_pair), (10, 100, 30));
        p.count_activity(4);
        assert_eq!(p.close_bin(), Some(3));
        assert_eq!((p.sum_prev, p.sum_prev_sq, p.sum_pair), (13, 109, 42));
        assert!(p.is_well_formed());
        // Up to the window's bins, then refused with nothing changed.
        for k in 4..=ACTIVITY_WINDOW_BINS {
            p.count_activity(1);
            assert_eq!(p.close_bin(), Some(k));
        }
        let full = p;
        p.count_activity(9);
        assert_eq!(p.close_bin(), None, "the window is full");
        assert_eq!(p.bin_activity, 9, "the open bin keeps counting");
        assert_eq!(
            HomeostaticDrivePool {
                bin_activity: 0,
                ..p
            },
            full
        );
        assert!(
            !p.is_well_formed(),
            "a full window is regulated before it is written"
        );
        // The cap holds at the close, however the open bin got there.
        let mut q = HomeostaticDrivePool::new();
        q.bin_activity = u32::MAX;
        assert_eq!(q.close_bin(), Some(1));
        assert_eq!(q.last_activity, ACTIVITY_COUNT_MAX);
        q.bin_activity = u32::MAX;
        assert_eq!(q.close_bin(), Some(2));
        assert_eq!(
            q.sum_prev_sq,
            (ACTIVITY_COUNT_MAX as u64) * (ACTIVITY_COUNT_MAX as u64)
        );
        assert!(q.is_well_formed());
    }

    #[test]
    fn the_estimate_is_the_lag_one_slope_with_an_intercept() {
        assert_eq!(
            HomeostaticDrivePool::new().estimate_branching_ratio(),
            None,
            "no bin, no estimate"
        );
        assert_eq!(
            window(&[0]).estimate_branching_ratio(),
            Some(0),
            "silent: no descendants"
        );
        assert_eq!(window(&[0, 0, 0, 0]).estimate_branching_ratio(), Some(0));
        assert_eq!(
            window(&[7]).estimate_branching_ratio(),
            None,
            "one bin: no pair"
        );
        assert_eq!(
            window(&[7, 7]).estimate_branching_ratio(),
            None,
            "one pair: no variance"
        );
        assert_eq!(
            window(&[5, 5, 5, 5]).estimate_branching_ratio(),
            None,
            "constant activity: no estimate"
        );
        // Halving: the slope is exactly one half.
        assert_eq!(
            window(&[1024, 512, 256, 128, 64]).estimate_branching_ratio(),
            Some(0x8000)
        );
        // Doubling: exactly two.
        assert_eq!(
            window(&[1, 2, 4, 8, 16, 32]).estimate_branching_ratio(),
            Some(0x0002_0000)
        );
        // Doubling over a drive of three: the intercept takes the drive, the slope stays two;
        // the slope through the origin would not.
        assert_eq!(
            window(&[1, 5, 13, 29, 61, 125]).estimate_branching_ratio(),
            Some(0x0002_0000)
        );
        // Critical: each bin the one before plus a drive.
        assert_eq!(
            window(&[10, 14, 18, 22, 26]).estimate_branching_ratio(),
            Some(ONE)
        );
        // A negative slope is no branching.
        assert_eq!(
            window(&[10, 0, 10, 0, 10]).estimate_branching_ratio(),
            Some(0),
            "alternation"
        );
        // A slope beyond the cap.
        assert_eq!(
            window(&[1, 100, 10_000]).estimate_branching_ratio(),
            Some(SIGMA_MAX_Q16)
        );
        // Rounded to nearest: x_{t+1} = x_t / 3 exactly, 0.3333.. is 21 845.33 in Q16.16.
        assert_eq!(
            window(&[729, 243, 81, 27, 9]).estimate_branching_ratio(),
            Some(21_845)
        );
        // x_{t+1} = 2 x_t / 3: 0.6666.. is 43 690.67, rounded up.
        assert_eq!(
            window(&[729, 486, 324, 216, 144]).estimate_branching_ratio(),
            Some(43_691)
        );
        // Silence with a single spike in the last bin: no pair varies, no estimate.
        assert_eq!(window(&[0, 0, 1]).estimate_branching_ratio(), None);
        // A spike in the first bin and silence after: the first pair's variance and a slope
        // of zero.
        assert_eq!(window(&[1, 0, 0]).estimate_branching_ratio(), Some(0));
        // The cap's bins: sums at their bounds still estimate.
        let cap = ACTIVITY_COUNT_MAX;
        assert_eq!(
            window(&[cap, cap, cap, 0]).estimate_branching_ratio(),
            None,
            "two equal pairs and one drop: variance from one pair, but the slope is the ratio of that drop, which the regression reads as no branching: n * sab - sa * sb is zero"
        );
    }

    #[test]
    fn regulation_moves_the_gain_by_the_bounded_step_and_clears_the_window() {
        // No estimate: the gain stays, the window clears, None.
        let mut p = window(&[7]);
        p.control_step_q0_16 = CONTROL_STEP_MAX_Q0_16;
        assert_eq!(p.regulate(u32::MAX), None);
        assert_eq!(p.synaptic_gain_q16, ONE);
        assert_eq!(p.branching_ratio_q16, 0, "no estimate was stored");
        assert_eq!(
            (
                p.window_bins,
                p.sum_prev,
                p.sum_prev_sq,
                p.sum_pair,
                p.first_activity,
                p.last_activity
            ),
            (0, 0, 0, 0, 0, 0)
        );
        // kappa 0: whatever the estimate, the gain is bit for bit the same.
        for counts in [&[1u32, 2, 4, 8][..], &[0, 0, 0][..], &[8, 4, 2, 1][..]] {
            let mut p = window(counts);
            p.synaptic_gain_q16 = 0x0001_2345;
            assert_eq!(p.regulate(u32::MAX), Some(0x0001_2345));
            assert_eq!(p.synaptic_gain_q16, 0x0001_2345);
            assert_eq!(p.window_bins, 0);
        }
        // Supercritical at the largest step: the error clamps to 1, the factor is 0.5.
        let mut p = window(&[1, 2, 4, 8, 16]);
        p.control_step_q0_16 = CONTROL_STEP_MAX_Q0_16;
        assert_eq!(p.regulate(u32::MAX), Some(0x8000));
        assert_eq!(p.branching_ratio_q16, 0x0002_0000, "the estimate is stored");
        // Silent at the largest step: the error is -1, the factor 1.5.
        let mut p = window(&[0, 0]);
        p.control_step_q0_16 = CONTROL_STEP_MAX_Q0_16;
        assert_eq!(p.regulate(u32::MAX), Some(0x0001_8000));
        assert_eq!(p.branching_ratio_q16, 0);
        // Critical: the factor is 1.
        let mut p = window(&[10, 14, 18, 22]);
        p.control_step_q0_16 = CONTROL_STEP_MAX_Q0_16;
        assert_eq!(p.regulate(u32::MAX), Some(ONE));
        // An estimate of 1.25 at kappa 0.25: a step of 1/16, the gain 0.9375.
        let mut p = window(&[64, 80, 100, 125]);
        assert_eq!(p.estimate_branching_ratio(), Some(0x0001_4000));
        p.control_step_q0_16 = 0x4000;
        assert_eq!(p.regulate(u32::MAX), Some(0xF000));
        // An estimate of 0.5 at kappa 0.25: a step of -1/8, the gain 1.125.
        let mut p = window(&[64, 32, 16, 8]);
        p.control_step_q0_16 = 0x4000;
        assert_eq!(p.regulate(u32::MAX), Some(0x0001_2000));
        // Rounded to nearest: a gain of 1 + 2^-16 halved is 0.5 + 2^-17, which rounds up.
        let mut p = window(&[1, 2, 4, 8]);
        p.control_step_q0_16 = CONTROL_STEP_MAX_Q0_16;
        p.synaptic_gain_q16 = 0x0001_0001;
        assert_eq!(p.regulate(u32::MAX), Some(0x8001));
        // The step rounds to nearest too: kappa 1/65536 times an error of 1.0 is one LSB.
        let mut p = window(&[1, 2, 4, 8]);
        p.control_step_q0_16 = 1;
        assert_eq!(p.regulate(u32::MAX), Some(ONE - 1));
        let mut p = window(&[1, 2, 4, 8]);
        p.control_step_q0_16 = 1;
        p.synaptic_gain_q16 = 0x8000;
        assert_eq!(
            p.regulate(u32::MAX),
            Some(0x8000),
            "half a gain times one LSB rounds away"
        );
        // The rails: at the ceiling a silent window leaves it there, at the floor a
        // supercritical one leaves it there; one step short of each moves onto the rail.
        let mut p = window(&[0, 0]);
        p.control_step_q0_16 = CONTROL_STEP_MAX_Q0_16;
        p.synaptic_gain_q16 = GAIN_MAX_Q16;
        assert_eq!(p.regulate(u32::MAX), Some(GAIN_MAX_Q16));
        let mut p = window(&[0, 0]);
        p.control_step_q0_16 = CONTROL_STEP_MAX_Q0_16;
        p.synaptic_gain_q16 = GAIN_MAX_Q16 - 1;
        assert_eq!(
            p.regulate(u32::MAX),
            Some(GAIN_MAX_Q16),
            "clamped, not wrapped"
        );
        let mut p = window(&[1, 2, 4, 8]);
        p.control_step_q0_16 = CONTROL_STEP_MAX_Q0_16;
        p.synaptic_gain_q16 = GAIN_MIN_Q16;
        assert_eq!(p.regulate(u32::MAX), Some(GAIN_MIN_Q16));
        let mut p = window(&[1, 2, 4, 8]);
        p.control_step_q0_16 = CONTROL_STEP_MAX_Q0_16;
        p.synaptic_gain_q16 = GAIN_MIN_Q16 + 1;
        assert_eq!(p.regulate(u32::MAX), Some(GAIN_MIN_Q16));
        // A step above its bound is clamped nowhere here: the loader and the executor refuse
        // it; the rule takes the field as it is, so a factor of 1 - 0.99998 is 2^-16.
        let mut p = window(&[1, 2, 4, 8]);
        p.control_step_q0_16 = u16::MAX;
        assert_eq!(
            p.regulate(u32::MAX),
            Some(GAIN_MIN_Q16),
            "clamped to the floor"
        );
        // Saturation: a window at or above the ceiling in every bin on average is regulated as
        // supercritical whatever its slope; one below it is regulated by its slope.
        let mut p = window(&[100, 100, 100]);
        assert!(p.is_saturated(100) && !p.is_saturated(101));
        assert!(
            !HomeostaticDrivePool::new().is_saturated(0),
            "no bin, no saturation"
        );
        p.control_step_q0_16 = CONTROL_STEP_MAX_Q0_16;
        assert_eq!(
            p.regulate(100),
            Some(0x8000),
            "read as supercritical: halved"
        );
        assert_eq!(p.branching_ratio_q16, SIGMA_MAX_Q16);
        let mut q = window(&[100, 100, 100]);
        q.control_step_q0_16 = CONTROL_STEP_MAX_Q0_16;
        assert_eq!(
            q.regulate(101),
            None,
            "below the ceiling: a constant window, no estimate"
        );
        assert_eq!(q.synaptic_gain_q16, ONE);
        // The mean is over every bin of the window, the first and the last included.
        let mut r = window(&[300, 0, 0]);
        assert!(r.is_saturated(100) && !r.is_saturated(101));
        assert_eq!(
            r.regulate(101),
            Some(ONE),
            "a slope of zero at kappa 0: the gain stays"
        );
        assert_eq!(r.branching_ratio_q16, 0);
        // The open bin survives a regulation: it belongs to the next window.
        let mut p = window(&[1, 2, 4, 8]);
        p.count_activity(6);
        p.regulate(u32::MAX);
        assert_eq!(p.bin_activity, 6);
        assert_eq!(p.close_bin(), Some(1));
        assert_eq!(p.first_activity, 6);
    }

    #[test]
    fn well_formed_fails_on_each_clause_alone() {
        let ok = window(&[3, 4]);
        assert!(ok.is_well_formed());
        type Mutation = fn(&mut HomeostaticDrivePool);
        let cases: [(&str, Mutation); 14] = [
            ("gain below the floor", |p| {
                p.synaptic_gain_q16 = GAIN_MIN_Q16 - 1
            }),
            ("gain above the ceiling", |p| {
                p.synaptic_gain_q16 = GAIN_MAX_Q16 + 1
            }),
            ("step above its bound", |p| {
                p.control_step_q0_16 = CONTROL_STEP_MAX_Q0_16 + 1
            }),
            ("a full window", |p| p.window_bins = ACTIVITY_WINDOW_BINS),
            ("an open bin above the cap", |p| {
                p.bin_activity = ACTIVITY_COUNT_MAX + 1
            }),
            ("a last bin above the cap", |p| {
                p.last_activity = ACTIVITY_COUNT_MAX + 1
            }),
            ("a first bin above the cap", |p| {
                p.first_activity = ACTIVITY_COUNT_MAX + 1
            }),
            ("a sum above its bound", |p| {
                p.sum_prev = ACTIVITY_COUNT_MAX as u64 + 1
            }),
            ("a square sum above its bound", |p| {
                p.sum_prev_sq = (ACTIVITY_COUNT_MAX as u64) * (ACTIVITY_COUNT_MAX as u64) + 1
            }),
            ("a pair sum above its bound", |p| {
                p.sum_pair = (ACTIVITY_COUNT_MAX as u64) * (ACTIVITY_COUNT_MAX as u64) + 1
            }),
            ("a sleep flag that is neither", |p| p.sleep_mode_active = 2),
            ("reserved bytes", |p| p._reserved = 1),
            ("a sum without a pair", |p| {
                p.window_bins = 1;
                p.sum_prev = 1;
            }),
            ("a square sum without a pair", |p| {
                p.window_bins = 1;
                p.sum_pair = 1;
            }),
        ];
        for (what, mutate) in cases {
            let mut p = ok;
            mutate(&mut p);
            assert!(!p.is_well_formed(), "{what}");
        }
        // A first or a last bin without a bin, on a record with no pair, so that this clause
        // is the only one violated.
        let mut first = HomeostaticDrivePool::new();
        first.first_activity = 1;
        assert!(!first.is_well_formed(), "a first bin without a bin");
        let mut last = HomeostaticDrivePool::new();
        last.last_activity = 1;
        assert!(!last.is_well_formed(), "a last bin without a bin");
        let mut one_bin = HomeostaticDrivePool::new();
        one_bin.window_bins = 1;
        one_bin.first_activity = 1;
        one_bin.last_activity = 1;
        assert!(
            one_bin.is_well_formed(),
            "one bin, remembered as first and last"
        );
        // The neighbours on the right side of each bound are well formed.
        let mut p = ok;
        p.synaptic_gain_q16 = GAIN_MIN_Q16;
        assert!(p.is_well_formed());
        p.synaptic_gain_q16 = GAIN_MAX_Q16;
        assert!(p.is_well_formed());
        p.control_step_q0_16 = CONTROL_STEP_MAX_Q0_16;
        assert!(p.is_well_formed());
        p.window_bins = ACTIVITY_WINDOW_BINS - 1;
        p.sum_prev = 30 * ACTIVITY_COUNT_MAX as u64;
        p.sum_prev_sq = 30 * (ACTIVITY_COUNT_MAX as u64) * (ACTIVITY_COUNT_MAX as u64);
        p.sum_pair = p.sum_prev_sq;
        p.bin_activity = ACTIVITY_COUNT_MAX;
        p.last_activity = ACTIVITY_COUNT_MAX;
        p.first_activity = ACTIVITY_COUNT_MAX;
        p.sleep_mode_active = 1;
        assert!(p.is_well_formed(), "every bound at its edge");
        p.sum_prev += 1;
        assert!(!p.is_well_formed());
    }

    #[test]
    fn the_record_round_trips_through_its_bytes() {
        let p = HomeostaticDrivePool {
            energy_level: 1,
            sensory_fatigue: 2,
            curiosity_drive: 3,
            bin_activity: 4,
            circadian_phase: 0x1234,
            sleep_mode_active: 1,
            window_bins: 31,
            control_step_q0_16: 0x5678,
            _reserved: 0,
            branching_ratio_q16: 0x0001_0000,
            synaptic_gain_q16: 0x0002_0000,
            sum_prev: 0x0123_4567_89AB_CDEF,
            sum_prev_sq: u64::MAX,
            sum_pair: 7,
            last_activity: 0x00FF_FFFF,
            first_activity: 9,
        };
        let bytes = p.encode();
        assert_eq!(&bytes[16..18], &[0x34, 0x12]);
        assert_eq!(bytes[18], 1);
        assert_eq!(bytes[19], 31);
        assert_eq!(&bytes[28..32], &0x0002_0000u32.to_le_bytes());
        assert_eq!(&bytes[40..48], &[0xFF; 8]);
        assert_eq!(&bytes[60..64], &9u32.to_le_bytes());
        assert_eq!(HomeostaticDrivePool::decode(&bytes), p);
        assert_eq!(
            HomeostaticDrivePool::decode(&HomeostaticDrivePool::new().encode()),
            HomeostaticDrivePool::new()
        );
        let mut reserved = HomeostaticDrivePool::new();
        reserved._reserved = 0xBEEF;
        assert_eq!(&reserved.encode()[22..24], &[0xEF, 0xBE]);
        assert_eq!(
            HomeostaticDrivePool::decode(&reserved.encode())._reserved,
            0xBEEF
        );
    }
}

/// Property tests (ADR-0030): every rule keeps a well-formed record well formed, the estimate
/// stays within its range, the gain within its bounds, and at a control step of zero the gain
/// never moves.
#[cfg(test)]
mod prop {
    use super::*;
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../testkit/prop.rs"
    ));

    #[test]
    fn every_rule_keeps_the_record_well_formed_and_the_gain_within_its_bounds() {
        let mut rng = Lcg::new(29);
        for round in 0..400u32 {
            let mut p = HomeostaticDrivePool::new();
            p.control_step_q0_16 = if round % 4 == 0 {
                0
            } else {
                (rng.next_u32() as u16).min(CONTROL_STEP_MAX_Q0_16)
            };
            let frozen = p.control_step_q0_16 == 0;
            let mut windows = 0u32;
            for step in 0..2_000u32 {
                let count = match rng.below(6) {
                    0 => rng.pick(&U32_LATTICE),
                    1 => 0,
                    _ => rng.below(1 << 12),
                };
                p.count_activity(count);
                assert!(p.bin_activity <= ACTIVITY_COUNT_MAX);
                if step % 3 == 0 {
                    let bins = p.close_bin();
                    assert!(bins.is_some(), "the window is regulated when full");
                    if bins == Some(ACTIVITY_WINDOW_BINS) {
                        let before = p.synaptic_gain_q16;
                        if let Some(sigma) = p.estimate_branching_ratio() {
                            assert!(sigma <= SIGMA_MAX_Q16);
                        }
                        let after = p.regulate(if round % 2 == 0 { u32::MAX } else { 2_000 });
                        assert_eq!(p.window_bins, 0);
                        assert!((GAIN_MIN_Q16..=GAIN_MAX_Q16).contains(&p.synaptic_gain_q16));
                        if frozen {
                            assert_eq!(p.synaptic_gain_q16, before, "a step of zero moves nothing");
                        }
                        if let Some(gain) = after {
                            assert_eq!(gain, p.synaptic_gain_q16);
                        } else {
                            assert_eq!(p.synaptic_gain_q16, before, "no estimate moves nothing");
                        }
                        windows = windows.wrapping_add(1);
                    }
                }
                assert!(p.is_well_formed(), "round {round}, step {step}: {p:?}");
                assert_eq!(HomeostaticDrivePool::decode(&p.encode()), p);
            }
            assert!(windows > 0);
        }
    }
}
