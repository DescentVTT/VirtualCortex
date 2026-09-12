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
//!
//! Sleep (ADR-0037) is a state machine stepped once per window on the same cadence: a sleep
//! pressure $S$ that rises while awake and falls while asleep (process S of Borbély's
//! two-process model), a circadian phase advanced by one per window so that its sixteen bits
//! are a day, onset and wake thresholds lower in the night quarter (process C), and three
//! stages, awake, slow-wave and REM, the two sleep stages alternating under a budget of
//! windows. The executor replays the episodic ledger during slow-wave sleep and lowers an
//! episode's tag during REM (`cortex-hippocampus`, ADR-0038). A wake is an input between
//! ticks. With the shift at 0 the pressure and the stage stay where the image put them.

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

/// The sleep stage awake (ADR-0037): the pressure rises.
pub const STAGE_AWAKE: u8 = 0;
/// Slow-wave sleep: the pressure falls and the executor replays the ledger on the ripple
/// cadence (ADR-0038).
pub const STAGE_SWS: u8 = 1;
/// REM sleep: the pressure falls and a ripple lowers an episode's tag; nothing is replayed.
pub const STAGE_REM: u8 = 2;
/// The largest sleep shift, 15: the pressure's rise has a time constant of $2^{15}$ windows
/// (11.9 h at the fine tick) and its fall of $2^{13}$ (3.0 h), near the human values.
pub const SLEEP_SHIFT_MAX: u8 = 15;
/// The sleep pressure's ceiling, 1.0 in Q16.16.
pub const PRESSURE_MAX_Q16: u32 = 0x0001_0000;
/// The circadian phase at which the night quarter begins: three quarters of the cycle.
pub const NIGHT_PHASE: u16 = 0xC000;
/// Sleep begins by day when the pressure reaches 0.875.
pub const SLEEP_ONSET_DAY_Q16: u32 = 0xE000;
/// Sleep begins in the night quarter when the pressure reaches 0.5.
pub const SLEEP_ONSET_NIGHT_Q16: u32 = 0x8000;
/// Sleep ends by day when the pressure falls to 0.375.
pub const WAKE_DAY_Q16: u32 = 0x6000;
/// Sleep ends in the night quarter when the pressure falls to 0.125.
pub const WAKE_NIGHT_Q16: u32 = 0x2000;
/// Windows of slow-wave sleep before REM begins (5.2 s at the fine tick; a Target to tune).
pub const SWS_WINDOWS: u8 = 4;
/// Windows of REM before slow-wave sleep resumes (2.6 s; a Target to tune).
pub const REM_WINDOWS: u8 = 2;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(C, align(64))]
pub struct HomeostaticDrivePool {
    pub energy_level: u32,        // [0..4] Energy reserve (Q16.16)
    pub sleep_pressure_q16: u32,  // [4..8] Sleep pressure S in [0, 1.0] (Q16.16; ADR-0037)
    pub curiosity_drive: u32, // [8..12] Intrinsic novelty seeking drive (Q16.16; the organism's one scalar, ADR-0016)
    pub bin_activity: u32, // [12..16] Spikes counted in the open bin, at most ACTIVITY_COUNT_MAX (ADR-0036)
    pub circadian_phase: u16, // [16..18] Internal phase angle, wraps at 16 bits
    pub sleep_stage: u8,   // [18] STAGE_AWAKE, STAGE_SWS or STAGE_REM (ADR-0037)
    pub window_bins: u8,   // [19] Bins closed in the open window, below ACTIVITY_WINDOW_BINS
    pub control_step_q0_16: u16, // [20..22] The control step kappa (Q0.16); 0 leaves the gain
    pub sleep_shift: u8, // [22] The pressure's time constant, 2^shift windows; 0 leaves the stage (ADR-0037)
    pub stage_windows: u8, // [23] Windows in the current stage, saturating (ADR-0037)
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
            sleep_pressure_q16: 0,
            curiosity_drive: 0,
            bin_activity: 0,
            circadian_phase: 0,
            sleep_stage: STAGE_AWAKE,
            window_bins: 0,
            control_step_q0_16: 0,
            sleep_shift: 0,
            stage_windows: 0,
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

    /// True in the night quarter of the circadian cycle: the phase at or above
    /// [`NIGHT_PHASE`].
    #[inline]
    pub const fn is_night(&self) -> bool {
        self.circadian_phase >= NIGHT_PHASE
    }

    /// True in either sleep stage, or in a stage the constants do not name.
    #[inline]
    pub const fn is_asleep(&self) -> bool {
        self.sleep_stage != STAGE_AWAKE
    }

    /// The pressure at which sleep begins: [`SLEEP_ONSET_NIGHT_Q16`] in the night quarter,
    /// [`SLEEP_ONSET_DAY_Q16`] otherwise (process C of the two-process model).
    pub const fn sleep_onset_q16(&self) -> u32 {
        if self.is_night() {
            SLEEP_ONSET_NIGHT_Q16
        } else {
            SLEEP_ONSET_DAY_Q16
        }
    }

    /// The pressure at which sleep ends: [`WAKE_NIGHT_Q16`] in the night quarter,
    /// [`WAKE_DAY_Q16`] otherwise.
    pub const fn wake_threshold_q16(&self) -> u32 {
        if self.is_night() {
            WAKE_NIGHT_Q16
        } else {
            WAKE_DAY_Q16
        }
    }

    /// Once per window (ADR-0037): the circadian phase advances by one, wrapping (sixteen
    /// bits of windows of $2^{17}$ ticks are $2^{33}$ ticks, 23.86 h at the fine tick); then,
    /// with the shift $k > 0$, the sleep pressure $S$ (process S of Borbély's two-process
    /// model) rises while awake by $(1 - S) \gg k$ and falls while asleep by
    /// $S \gg \max(k - 2, 0)$, four times as fast, each by at least one LSB so that 1.0 and 0
    /// are reached exactly; then the stage moves: awake to slow-wave sleep when $S$ reaches
    /// the onset threshold, either sleep stage to awake when $S$ falls to the wake threshold,
    /// slow-wave to REM after [`SWS_WINDOWS`] windows in the stage and REM back to slow-wave
    /// after [`REM_WINDOWS`], the wake test first. A stage the constants do not name wakes.
    /// At $k = 0$ the pressure and the stage stay where the image put them; a shift above
    /// [`SLEEP_SHIFT_MAX`] is read as the bound. Returns the stage.
    pub fn step_sleep(&mut self) -> u8 {
        self.circadian_phase = self.circadian_phase.wrapping_add(1);
        // A shift above the bound (reachable through the public field; the loader refuses it)
        // is read as the bound, so no shift reaches the width (ADR-0028).
        let k = (self.sleep_shift as u32).min(SLEEP_SHIFT_MAX as u32);
        if k == 0 {
            return self.sleep_stage;
        }
        // This window counts toward the stage, saturating: only the two budgets read it.
        self.stage_windows = self.stage_windows.saturating_add(1);
        match self.sleep_stage {
            STAGE_AWAKE => {
                let gap = PRESSURE_MAX_Q16.saturating_sub(self.sleep_pressure_q16);
                self.sleep_pressure_q16 = self
                    .sleep_pressure_q16
                    .saturating_add((gap >> k).max(1))
                    .min(PRESSURE_MAX_Q16);
                if self.sleep_pressure_q16 >= self.sleep_onset_q16() {
                    self.enter(STAGE_SWS);
                }
            }
            STAGE_SWS | STAGE_REM => {
                // Two shifts fewer than the rise (Borbély's ratio is about 4.5), floored at a
                // whole step.
                let fall = k.saturating_sub(2);
                self.sleep_pressure_q16 = self
                    .sleep_pressure_q16
                    .saturating_sub((self.sleep_pressure_q16 >> fall).max(1));
                if self.sleep_pressure_q16 <= self.wake_threshold_q16() {
                    self.enter(STAGE_AWAKE);
                } else if self.sleep_stage == STAGE_SWS && self.stage_windows >= SWS_WINDOWS {
                    self.enter(STAGE_REM);
                } else if self.sleep_stage == STAGE_REM && self.stage_windows >= REM_WINDOWS {
                    self.enter(STAGE_SWS);
                }
            }
            _ => self.enter(STAGE_AWAKE),
        }
        self.sleep_stage
    }

    /// The stage changes and its window count starts again.
    #[inline]
    fn enter(&mut self, stage: u8) {
        self.sleep_stage = stage;
        self.stage_windows = 0;
    }

    /// An input between ticks (ADR-0037): whatever the stage, the engine is awake, with its
    /// windows in the stage at zero and its pressure kept, as an alarm leaves a sleeper.
    /// Returns whether it was not awake.
    pub fn wake(&mut self) -> bool {
        if self.sleep_stage == STAGE_AWAKE {
            return false;
        }
        self.enter(STAGE_AWAKE);
        true
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
    /// the caller names the ceiling (the executor's is one spike per unit per bin), and a
    /// window at or above it is regulated as supercritical.
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

    /// True for a record within the bounds and the consistency the rules keep between
    /// windows: the gain within its bounds, the control step within its, fewer bins than a
    /// window holds (a full window is regulated in the step that fills it), every count at or
    /// below the cap, every sum at or below what the pairs and the cap allow, the sums
    /// consistent with each other (the square of the sum at most the pairs times the sum of
    /// squares, and the pair sum at most the cap times the sum), no first or last bin without
    /// a bin, a one-bin window's first and last the same bin, the stage one of three, the
    /// pressure at most 1.0, the shift at most [`SLEEP_SHIFT_MAX`], a sleep stage's windows
    /// below its budget. The loader refuses a record that is not (ADR-0028). Not every
    /// record that passes is one the rules produced (the sums are not the bins), and the
    /// estimator answers every record that passes without an estimate at worst.
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
            // Cauchy–Schwarz over the pairs' first members, and each pair's product at most the
            // cap times its first member; both hold for every series of counts.
            && self.sum_prev.saturating_mul(self.sum_prev) <= pairs.saturating_mul(self.sum_prev_sq)
            && self.sum_pair <= self.sum_prev.saturating_mul(max)
            && (self.window_bins > 0 || (self.first_activity == 0 && self.last_activity == 0))
            && (self.window_bins != 1 || self.first_activity == self.last_activity)
            && self.sleep_stage <= STAGE_REM
            && self.sleep_pressure_q16 <= PRESSURE_MAX_Q16
            && self.sleep_shift <= SLEEP_SHIFT_MAX
            && (self.sleep_stage != STAGE_SWS || self.stage_windows < SWS_WINDOWS)
            && (self.sleep_stage != STAGE_REM || self.stage_windows < REM_WINDOWS)
    }

    /// The record's 64 bytes, little-endian, field by field (§8.7).
    pub fn encode(&self) -> [u8; 64] {
        let mut out = [0u8; 64];
        out[0..4].copy_from_slice(&self.energy_level.to_le_bytes());
        out[4..8].copy_from_slice(&self.sleep_pressure_q16.to_le_bytes());
        out[8..12].copy_from_slice(&self.curiosity_drive.to_le_bytes());
        out[12..16].copy_from_slice(&self.bin_activity.to_le_bytes());
        out[16..18].copy_from_slice(&self.circadian_phase.to_le_bytes());
        out[18] = self.sleep_stage;
        out[19] = self.window_bins;
        out[20..22].copy_from_slice(&self.control_step_q0_16.to_le_bytes());
        out[22] = self.sleep_shift;
        out[23] = self.stage_windows;
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
            sleep_pressure_q16: u32_at(4),
            curiosity_drive: u32_at(8),
            bin_activity: u32_at(12),
            circadian_phase: u16_at(16),
            sleep_stage: bytes[18],
            window_bins: bytes[19],
            control_step_q0_16: u16_at(20),
            sleep_shift: bytes[22],
            stage_windows: bytes[23],
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
    // The thresholds are ordered so that the machine cannot sleep and wake in one step, and
    // the night's are below the day's; the shift stays below the width of the pressure.
    assert!(WAKE_DAY_Q16 < SLEEP_ONSET_DAY_Q16 && SLEEP_ONSET_DAY_Q16 <= PRESSURE_MAX_Q16);
    assert!(WAKE_NIGHT_Q16 < SLEEP_ONSET_NIGHT_Q16 && SLEEP_ONSET_NIGHT_Q16 < SLEEP_ONSET_DAY_Q16);
    assert!(WAKE_NIGHT_Q16 < WAKE_DAY_Q16);
    assert!(SWS_WINDOWS > 0 && REM_WINDOWS > 0);
    assert!((SLEEP_SHIFT_MAX as u32) < 32);
    assert!(STAGE_AWAKE < STAGE_SWS && STAGE_SWS < STAGE_REM);
};

#[cfg(test)]
mod tests {
    use super::*;

    const ONE: u32 = GAIN_ONE_Q16;

    #[test]
    fn the_branching_ratio_is_descendants_over_ancestors_and_an_empty_window_measures_nothing() {
        let mut p = pool(0, 0, 0);
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

    fn pool(
        circadian_phase: u16,
        sleep_pressure_q16: u32,
        sleep_shift: u8,
    ) -> HomeostaticDrivePool {
        HomeostaticDrivePool {
            sleep_pressure_q16,
            circadian_phase,
            sleep_shift,
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
        assert_eq!(
            (
                p.sleep_stage,
                p.sleep_shift,
                p.sleep_pressure_q16,
                p.stage_windows
            ),
            (STAGE_AWAKE, 0, 0, 0)
        );
        assert!(!p.is_asleep() && !p.is_night());
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
        assert_eq!((STAGE_AWAKE, STAGE_SWS, STAGE_REM), (0, 1, 2));
        assert_eq!(
            (
                SLEEP_SHIFT_MAX,
                PRESSURE_MAX_Q16,
                NIGHT_PHASE,
                SWS_WINDOWS,
                REM_WINDOWS
            ),
            (15, 0x0001_0000, 0xC000, 4, 2)
        );
        assert_eq!(
            (
                SLEEP_ONSET_DAY_Q16,
                SLEEP_ONSET_NIGHT_Q16,
                WAKE_DAY_Q16,
                WAKE_NIGHT_Q16
            ),
            (0xE000, 0x8000, 0x6000, 0x2000)
        );
    }

    #[test]
    fn awake_the_pressure_rises_by_the_shift_and_sleep_begins_at_the_onset_threshold() {
        let mut p = pool(0, 0, 3);
        let expected = [
            0x2000u32, 0x3C00, 0x5480, 0x69F0, 0x7CB2, 0x8D1B, 0x9B77, 0xA808, 0xB307, 0xBCA6,
            0xC511, 0xCC6E, 0xD2E0, 0xD884, 0xDD73,
        ];
        for (i, &s) in expected.iter().enumerate() {
            assert_eq!(p.step_sleep(), STAGE_AWAKE, "window {i}");
            assert_eq!(p.sleep_pressure_q16, s, "window {i}: an eighth of the gap");
            assert_eq!(
                p.circadian_phase as usize,
                i + 1,
                "one phase step per window"
            );
            assert_eq!(p.stage_windows as usize, i + 1);
            assert!(!p.is_asleep());
        }
        assert_eq!(p.step_sleep(), STAGE_SWS, "0xE1C4 is past 0.875");
        assert_eq!(
            (p.sleep_pressure_q16, p.stage_windows, p.circadian_phase),
            (0xE1C4, 0, 16)
        );
        assert!(p.is_asleep());
        // Exactly at the threshold: one LSB short at the largest shift rises by the floor of
        // one LSB and lands on it; two short stays one below.
        let mut at = pool(0, SLEEP_ONSET_DAY_Q16 - 1, SLEEP_SHIFT_MAX);
        assert_eq!(at.step_sleep(), STAGE_SWS);
        assert_eq!(at.sleep_pressure_q16, SLEEP_ONSET_DAY_Q16);
        let mut below = pool(0, SLEEP_ONSET_DAY_Q16 - 2, SLEEP_SHIFT_MAX);
        assert_eq!(below.step_sleep(), STAGE_AWAKE);
        assert_eq!(below.sleep_pressure_q16, SLEEP_ONSET_DAY_Q16 - 1);
        assert_eq!(below.stage_windows, 1);
    }

    #[test]
    fn the_night_quarter_lowers_both_thresholds_and_begins_at_three_quarters_exactly() {
        assert!(!pool(0xBFFF, 0, 0).is_night());
        assert!(pool(NIGHT_PHASE, 0, 0).is_night());
        assert!(pool(0xFFFF, 0, 0).is_night());
        let day = pool(0, 0, 0);
        assert_eq!(
            (day.sleep_onset_q16(), day.wake_threshold_q16()),
            (SLEEP_ONSET_DAY_Q16, WAKE_DAY_Q16)
        );
        let night = pool(NIGHT_PHASE, 0, 0);
        assert_eq!(
            (night.sleep_onset_q16(), night.wake_threshold_q16()),
            (SLEEP_ONSET_NIGHT_Q16, WAKE_NIGHT_Q16)
        );
        // The step advances the phase before it reads the threshold: the window that enters
        // the night quarter sleeps at 0.5.
        let mut dusk = pool(0xBFFF, SLEEP_ONSET_NIGHT_Q16 - 1, SLEEP_SHIFT_MAX);
        assert_eq!(dusk.step_sleep(), STAGE_SWS);
        assert_eq!(dusk.circadian_phase, NIGHT_PHASE);
        let mut earlier = pool(0xBFFE, SLEEP_ONSET_NIGHT_Q16 - 1, SLEEP_SHIFT_MAX);
        assert_eq!(
            earlier.step_sleep(),
            STAGE_AWAKE,
            "one window earlier the onset is still 0.875"
        );
        // The phase wraps from 0xFFFF to 0: dawn, and the day's thresholds again.
        let mut dawn = pool(0xFFFF, WAKE_DAY_Q16, SLEEP_SHIFT_MAX);
        dawn.sleep_stage = STAGE_SWS;
        assert_eq!(
            dawn.step_sleep(),
            STAGE_AWAKE,
            "0x6000 falls to 0x5FFD, at or below the day's wake threshold"
        );
        assert_eq!((dawn.circadian_phase, dawn.sleep_pressure_q16), (0, 0x5FFD));
        let mut late = pool(0xFFFE, WAKE_DAY_Q16, SLEEP_SHIFT_MAX);
        late.sleep_stage = STAGE_SWS;
        assert_eq!(
            late.step_sleep(),
            STAGE_SWS,
            "still night: 0.125 is the wake threshold"
        );
    }

    #[test]
    fn asleep_the_pressure_falls_four_times_as_fast_and_the_stages_alternate_under_their_budgets() {
        let mut p = pool(NIGHT_PHASE, PRESSURE_MAX_Q16, 5);
        p.sleep_stage = STAGE_SWS;
        let expected: [(u32, u8, u8); 16] = [
            (57_344, STAGE_SWS, 1),
            (50_176, STAGE_SWS, 2),
            (43_904, STAGE_SWS, 3),
            (38_416, STAGE_REM, 0),
            (33_614, STAGE_REM, 1),
            (29_413, STAGE_SWS, 0),
            (25_737, STAGE_SWS, 1),
            (22_520, STAGE_SWS, 2),
            (19_705, STAGE_SWS, 3),
            (17_242, STAGE_REM, 0),
            (15_087, STAGE_REM, 1),
            (13_202, STAGE_SWS, 0),
            (11_552, STAGE_SWS, 1),
            (10_108, STAGE_SWS, 2),
            (8_845, STAGE_SWS, 3),
            (7_740, STAGE_AWAKE, 0),
        ];
        for (i, &(s, stage, windows)) in expected.iter().enumerate() {
            assert_eq!(p.step_sleep(), stage, "window {i}");
            assert_eq!(
                (p.sleep_pressure_q16, p.stage_windows),
                (s, windows),
                "window {i}: an eighth of the pressure off"
            );
            assert!(p.is_well_formed());
        }
        // Awake again, the pressure rises from where it was, by a thirty-second of the gap.
        assert_eq!(p.step_sleep(), STAGE_AWAKE);
        assert_eq!(p.sleep_pressure_q16, 7_740 + ((65_536 - 7_740) >> 5));
        // The wake test comes first: a slow-wave stage at its budget with the pressure landing
        // on the threshold wakes rather than dreams; one LSB above it, the budget moves the
        // stage.
        let mut tired = pool(0, 0x6003, SLEEP_SHIFT_MAX);
        tired.sleep_stage = STAGE_SWS;
        tired.stage_windows = SWS_WINDOWS - 1;
        assert_eq!(tired.step_sleep(), STAGE_AWAKE);
        assert_eq!(
            tired.sleep_pressure_q16, WAKE_DAY_Q16,
            "exactly the threshold"
        );
        let mut rested = pool(0, 0x6004, SLEEP_SHIFT_MAX);
        rested.sleep_stage = STAGE_SWS;
        rested.stage_windows = SWS_WINDOWS - 1;
        assert_eq!(rested.step_sleep(), STAGE_REM);
        assert_eq!(rested.sleep_pressure_q16, 0x6001);
        let mut dreaming = pool(0, 0x6004, SLEEP_SHIFT_MAX);
        dreaming.sleep_stage = STAGE_REM;
        dreaming.stage_windows = REM_WINDOWS - 1;
        assert_eq!(dreaming.step_sleep(), STAGE_SWS, "REM's budget spent");
        assert_eq!(dreaming.stage_windows, 0);
        let mut short = pool(0, 0x6004, SLEEP_SHIFT_MAX);
        short.sleep_stage = STAGE_REM;
        short.stage_windows = REM_WINDOWS - 2;
        assert_eq!(short.step_sleep(), STAGE_REM, "one window short of it");
        assert_eq!(short.stage_windows, REM_WINDOWS - 1);
        // At the two smallest shifts the fall is the whole pressure in one window; at three
        // it is a half.
        for k in [1u8, 2] {
            let mut fast = pool(NIGHT_PHASE, PRESSURE_MAX_Q16, k);
            fast.sleep_stage = STAGE_REM;
            assert_eq!(fast.step_sleep(), STAGE_AWAKE, "shift {k}");
            assert_eq!(fast.sleep_pressure_q16, 0);
        }
        let mut half = pool(NIGHT_PHASE, PRESSURE_MAX_Q16, 3);
        half.sleep_stage = STAGE_SWS;
        assert_eq!(half.step_sleep(), STAGE_SWS);
        assert_eq!(half.sleep_pressure_q16, 0x8000);
        // A shift above the bound, reachable through the public field, is read as the bound:
        // the same step as fifteen, and no shift reaches the width.
        let mut wide = pool(0, 0x1000, u8::MAX);
        let mut bound = pool(0, 0x1000, SLEEP_SHIFT_MAX);
        assert_eq!(wide.step_sleep(), bound.step_sleep());
        assert_eq!(wide.sleep_pressure_q16, bound.sleep_pressure_q16);
        assert_eq!(wide.sleep_pressure_q16, 0x1000 + ((0x10000 - 0x1000) >> 15));
        let mut wide_asleep = pool(NIGHT_PHASE, 0x9000, 16);
        wide_asleep.sleep_stage = STAGE_REM;
        assert_eq!(wide_asleep.step_sleep(), STAGE_REM);
        assert_eq!(wide_asleep.sleep_pressure_q16, 0x9000 - (0x9000 >> 13));
    }

    #[test]
    fn the_pressure_reaches_one_and_zero_exactly_by_the_one_lsb_floor() {
        // Kept awake past onset (an alarm each window), the pressure saturates at 1.0.
        let mut p = pool(0, PRESSURE_MAX_Q16 - 3, SLEEP_SHIFT_MAX);
        for expected in [
            PRESSURE_MAX_Q16 - 2,
            PRESSURE_MAX_Q16 - 1,
            PRESSURE_MAX_Q16,
            PRESSURE_MAX_Q16,
        ] {
            assert_eq!(p.step_sleep(), STAGE_SWS);
            assert_eq!(p.sleep_pressure_q16, expected);
            assert!(p.wake());
        }
        // Asleep with three LSB of pressure: one off per window, to zero, which is a fixed
        // point; each step wakes, since the threshold is far above, and is put back.
        let mut q = pool(0, 3, SLEEP_SHIFT_MAX);
        for (expected, stage) in [
            (2, STAGE_REM),
            (1, STAGE_SWS),
            (0, STAGE_REM),
            (0, STAGE_SWS),
        ] {
            q.sleep_stage = stage;
            assert_eq!(q.step_sleep(), STAGE_AWAKE);
            assert_eq!(q.sleep_pressure_q16, expected);
        }
    }

    #[test]
    fn a_shift_of_zero_moves_nothing_but_the_phase() {
        for stage in [STAGE_AWAKE, STAGE_SWS, STAGE_REM] {
            let mut p = pool(0xFFFF, 0x1234, 0);
            p.sleep_stage = stage;
            p.stage_windows = 1;
            assert_eq!(p.step_sleep(), stage);
            assert_eq!(
                (p.sleep_pressure_q16, p.stage_windows, p.circadian_phase),
                (0x1234, 1, 0),
                "the phase wrapped, nothing else moved"
            );
            assert!(p.is_well_formed());
        }
    }

    #[test]
    fn wake_returns_the_stage_to_awake_keeps_the_pressure_and_is_nothing_when_awake() {
        let mut p = pool(7, 0x9000, 4);
        assert!(!p.wake());
        assert_eq!(p, pool(7, 0x9000, 4));
        for stage in [STAGE_SWS, STAGE_REM] {
            let mut q = pool(7, 0x9000, 4);
            q.sleep_stage = stage;
            q.stage_windows = 1;
            assert!(q.wake());
            assert_eq!(
                (
                    q.sleep_stage,
                    q.stage_windows,
                    q.sleep_pressure_q16,
                    q.circadian_phase
                ),
                (STAGE_AWAKE, 0, 0x9000, 7)
            );
            assert!(!q.wake());
        }
        // A stage the constants do not name (the loader refuses it; the public field can hold
        // it) wakes on its next step, with the pressure neither risen nor fallen.
        let mut odd = pool(0, 0x9000, 4);
        odd.sleep_stage = 3;
        odd.stage_windows = 9;
        assert!(odd.is_asleep(), "not awake");
        assert!(!odd.is_well_formed());
        assert_eq!(odd.step_sleep(), STAGE_AWAKE);
        assert_eq!((odd.stage_windows, odd.sleep_pressure_q16), (0, 0x9000));
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
        let cases: [(&str, Mutation); 17] = [
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
            ("a stage the constants do not name", |p| {
                p.sleep_stage = STAGE_REM + 1
            }),
            ("a pressure above 1.0", |p| {
                p.sleep_pressure_q16 = PRESSURE_MAX_Q16 + 1
            }),
            ("a shift above its bound", |p| {
                p.sleep_shift = SLEEP_SHIFT_MAX + 1
            }),
            ("a slow-wave stage at its budget", |p| {
                p.sleep_stage = STAGE_SWS;
                p.stage_windows = SWS_WINDOWS;
            }),
            ("a REM stage at its budget", |p| {
                p.sleep_stage = STAGE_REM;
                p.stage_windows = REM_WINDOWS;
            }),
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
        one_bin.last_activity = 2;
        assert!(
            !one_bin.is_well_formed(),
            "a one-bin window whose first and last differ"
        );
        // The sums' consistency: the square of the sum at most the pairs times the sum of
        // squares (equal for a constant series), the pair sum at most the cap times the sum.
        let constant = window(&[5, 5, 5]);
        assert_eq!((constant.sum_prev, constant.sum_prev_sq), (10, 50));
        assert!(constant.is_well_formed(), "100 is exactly 2 times 50");
        let mut inconsistent = constant;
        inconsistent.sum_prev_sq = 49;
        assert!(!inconsistent.is_well_formed(), "100 exceeds 2 times 49");
        let mut impossible = window(&[3, 4]);
        impossible.sum_prev_sq = 0;
        assert!(
            !impossible.is_well_formed(),
            "a sum of 3 with no sum of squares"
        );
        let mut pair_too_large = window(&[1, 4]);
        pair_too_large.sum_pair = ACTIVITY_COUNT_MAX as u64 + 1;
        assert!(
            !pair_too_large.is_well_formed(),
            "a pair sum above the cap times the sum"
        );
        pair_too_large.sum_pair = ACTIVITY_COUNT_MAX as u64;
        assert!(
            pair_too_large.is_well_formed(),
            "exactly the cap times the sum"
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
        p.sleep_stage = STAGE_REM;
        p.stage_windows = REM_WINDOWS - 1;
        p.sleep_pressure_q16 = PRESSURE_MAX_Q16;
        p.sleep_shift = SLEEP_SHIFT_MAX;
        assert!(p.is_well_formed(), "every bound at its edge");
        p.sum_prev += 1;
        assert!(!p.is_well_formed());
        let mut sws = ok;
        sws.sleep_stage = STAGE_SWS;
        sws.stage_windows = SWS_WINDOWS - 1;
        assert!(sws.is_well_formed(), "one window short of the budget");
        let mut awake = ok;
        awake.stage_windows = u8::MAX;
        assert!(awake.is_well_formed(), "awake, any count of windows");
    }

    #[test]
    fn the_record_round_trips_through_its_bytes() {
        let p = HomeostaticDrivePool {
            energy_level: 1,
            sleep_pressure_q16: 2,
            curiosity_drive: 3,
            bin_activity: 4,
            circadian_phase: 0x1234,
            sleep_stage: 1,
            window_bins: 31,
            control_step_q0_16: 0x5678,
            sleep_shift: 15,
            stage_windows: 3,
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
        assert_eq!((bytes[22], bytes[23]), (15, 3));
        assert_eq!(&bytes[28..32], &0x0002_0000u32.to_le_bytes());
        assert_eq!(&bytes[40..48], &[0xFF; 8]);
        assert_eq!(&bytes[60..64], &9u32.to_le_bytes());
        assert_eq!(HomeostaticDrivePool::decode(&bytes), p);
        assert_eq!(
            HomeostaticDrivePool::decode(&HomeostaticDrivePool::new().encode()),
            HomeostaticDrivePool::new()
        );
        let mut odd = HomeostaticDrivePool::new().encode();
        odd[22] = 0xEF;
        odd[23] = 0xBE;
        let decoded = HomeostaticDrivePool::decode(&odd);
        assert_eq!((decoded.sleep_shift, decoded.stage_windows), (0xEF, 0xBE));
        assert!(!decoded.is_well_formed());
    }
}

/// Property tests (ADR-0030): every rule keeps a well-formed record well formed, the estimate
/// stays within its range, the gain within its bounds, and at a control step of zero the gain
/// never moves; every sleep step keeps the pressure within its range and the stage one of
/// three, and a shift above zero cycles.
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

    #[test]
    fn every_sleep_step_keeps_the_record_well_formed_and_the_stage_one_of_three() {
        let mut rng = Lcg::new(31);
        for round in 0..200u32 {
            let mut p = HomeostaticDrivePool::new();
            p.sleep_shift = rng.below(SLEEP_SHIFT_MAX as u32 + 1) as u8;
            p.circadian_phase = rng.next_u16();
            p.sleep_pressure_q16 = rng.below(PRESSURE_MAX_Q16 + 1);
            let mut transitions = 0u32;
            let mut previous = p.sleep_stage;
            for step in 0..4_000u32 {
                if rng.below(97) == 0 {
                    p.wake();
                }
                let stage = p.step_sleep();
                assert!(stage <= STAGE_REM);
                assert!(p.sleep_pressure_q16 <= PRESSURE_MAX_Q16);
                assert!(p.is_well_formed(), "round {round}, step {step}: {p:?}");
                assert_eq!(HomeostaticDrivePool::decode(&p.encode()), p);
                if stage != previous {
                    transitions = transitions.wrapping_add(1);
                    previous = stage;
                }
                if p.sleep_shift == 0 {
                    assert_eq!(stage, STAGE_AWAKE, "off: the stage stays where it was");
                }
            }
            if p.sleep_shift != 0 && p.sleep_shift <= 6 {
                assert!(
                    transitions > 2,
                    "round {round}: a shift of {} cycles within 4 000 windows",
                    p.sleep_shift
                );
            }
        }
    }
}
