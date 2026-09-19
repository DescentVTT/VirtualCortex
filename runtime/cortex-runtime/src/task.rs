//! A task, a readout and a reward (ADR-0059): the composition that closes the reward path of
//! three-factor plasticity (ADR-0032) on a behaviour the engine reads back from its own spike
//! train (ADR-0050). Nothing here is a rule of its own: the stimulus is an input drawn as
//! [`Drive`] draws, a function of the trial's index and a seed, so the whole run is an input
//! trace (§8.3); the readout counts a trial's spikes per set from the train and hands the
//! counts to `cortex-basal-ganglia`'s `compute_gating`, the crate that owns action selection
//! (ADR-0016), which decides; the reward is [`Executor::reward`], an input between ticks, its
//! sign the outcome's and its magnitude the caller's constant. The harness's state (the
//! trial's cursor, the counts, the channels) is the caller's, as ADR-0043's affect state was
//! until ADR-0052 moved it into the image; a learning run is therefore not resumable from an
//! image, which ADR-0059 states as accepted debt.
//!
//! A trial is `ticks` fine ticks: the stimulus set's units each receive the stimulus's
//! messages before the first tick (integrated on the tick after the one that drains the
//! injector ring, so the set fires together about twelve ticks on, as a replay does,
//! ADR-0038), the background drive runs every tick, the train is read once at the end, the
//! two channels select, and the reward is delivered before the next trial's first tick, inside
//! the eligibility trace's window ([`cortex_core::ELIGIBILITY_TAU_SHIFT`]) whatever the
//! trial's length below it. A run's `(stimulus, selection, correct)` sequence is bit-identical
//! on every worker count, since the train is (ADR-0023, ADR-0050).

use crate::executor::{Executor, Inject, InjectError};
use crate::synthesis::{Drive, mix64};
use cortex_basal_ganglia::BasalGangliaChannelState;
use cortex_core::{BURST_REFRACTORY_TICKS, MODULATION_ONE_Q16, REFRACTORY_TICKS, spike_message};

/// The shortest interval between two spikes of one unit, in ticks: a spike opens a refractory
/// window of `REFRACTORY_TICKS` (`BURST_REFRACTORY_TICKS` after a plateau, the shorter) during
/// which the unit integrates nothing and cannot fire, so a unit fires at most once per this
/// many ticks and the most spikes a trial can hold is a bound the caller's train is checked
/// against.
pub const MIN_INTERVAL_TICKS: u32 = if BURST_REFRACTORY_TICKS < REFRACTORY_TICKS {
    BURST_REFRACTORY_TICKS as u32
} else {
    REFRACTORY_TICKS as u32
};
const _: () = assert!(MIN_INTERVAL_TICKS >= 1);

/// The most spikes one unit can hold in `ticks` ticks: one per [`MIN_INTERVAL_TICKS`], the
/// last interval counted whole.
pub const fn spikes_per_unit(ticks: u32) -> u32 {
    ticks.div_ceil(MIN_INTERVAL_TICKS)
}

/// A contiguous set of units, `first..first + len`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Set {
    pub first: u32,
    pub len: u32,
}

impl Set {
    /// One past the last unit, widened so that a set at the top of the index space has an end.
    pub const fn end(&self) -> u64 {
        (self.first as u64).wrapping_add(self.len as u64)
    }

    /// True for a unit of the set.
    pub const fn contains(&self, unit: u32) -> bool {
        unit >= self.first && (unit as u64) < self.end()
    }

    /// True when the two sets share a unit; an empty set shares none.
    pub const fn overlaps(&self, other: &Set) -> bool {
        self.len != 0
            && other.len != 0
            && (self.first as u64) < other.end()
            && (other.first as u64) < self.end()
    }
}

/// A stimulus: `messages` basal messages of `efficacy_q16` into every unit of `set`, injected
/// between ticks before a trial's first tick. Two messages of 1.25 (the replay drive's,
/// ADR-0038) fire a unit at its base threshold exactly once.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Stimulus {
    pub set: Set,
    pub messages: u32,
    pub efficacy_q16: i32,
}

impl Stimulus {
    /// Injects the stimulus; returns the messages injected. An injector that refuses one stops
    /// the stimulus there, as [`Drive::step`] stops.
    pub fn inject(&self, inject: &Inject) -> Result<u32, InjectError> {
        let message = spike_message(self.efficacy_q16, false);
        let mut sent = 0u32;
        for k in 0..self.set.len {
            // Below the arena, which `Task::check` bounded below `u32::MAX`.
            let unit = self.set.first.wrapping_add(k);
            for _ in 0..self.messages {
                inject.inject(unit, message)?;
                sent = sent.saturating_add(1);
            }
        }
        Ok(sent)
    }
}

/// The readout: two disjoint sets of units and the two action channels their spike counts
/// drive. A trial's spikes are counted per set from the train; each channel's own count is
/// its direct-pathway drive and the other's its indirect-pathway drive, the hyperdirect drive
/// zero, and `compute_gating` selects: a channel whose count exceeds the other's, and neither
/// at a tie (a net output of exactly zero is not a selection, the crate's own rule).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Readout {
    sets: [Set; 2],
    channels: [BasalGangliaChannelState; 2],
}

/// A count as a Q16.16 drive: `count × 1.0`, saturating at the width. [`Task::check`] refuses
/// a set whose count could reach the saturation, so that two counts never tie there.
fn count_q16(count: u32) -> i32 {
    // Below $2^{48}$ in `i64`; the minimum keeps it within `i32`.
    (i64::from(count) << 16).min(i64::from(i32::MAX)) as i32
}

impl Readout {
    /// A readout over `sets`, the channels at rest with ids 0 and 1.
    pub fn new(sets: [Set; 2]) -> Self {
        let channel = |id: u32| BasalGangliaChannelState {
            channel_id: id,
            striatal_d1_drive: 0,
            striatal_d2_drive: 0,
            stn_hyperdirect_drive: 0,
            gpi_snr_inhibition: 0,
            dopamine_modulation: 0,
            habit_strength: 0,
            selected_flag: 0,
            _reserved: [0; 32],
        };
        Self {
            sets,
            channels: [channel(0), channel(1)],
        }
    }

    /// The two sets.
    pub const fn sets(&self) -> &[Set; 2] {
        &self.sets
    }

    /// The two channels as the last selection left them.
    pub const fn channels(&self) -> &[BasalGangliaChannelState; 2] {
        &self.channels
    }

    /// The spikes of each set among the entries of `train` whose tick is within `ticks` of
    /// `start` (a wrapping difference, §8.4). `train` is in tick order and holds nothing after
    /// the trial, as the executor's train does when it is read at the trial's end, so the scan
    /// runs from the newest entry back to the first one before the trial and stops.
    pub fn count(&self, train: &[(u32, u32)], start: u32, ticks: u32) -> [u32; 2] {
        let mut counts = [0u32; 2];
        for &(tick, unit) in train.iter().rev() {
            if tick.wrapping_sub(start) >= ticks {
                break;
            }
            for (count, set) in counts.iter_mut().zip(self.sets.iter()) {
                if set.contains(unit) {
                    *count = count.saturating_add(1);
                }
            }
        }
        counts
    }

    /// The selection from two counts: each channel's own count into its direct drive, the
    /// other's into its indirect drive, and `compute_gating` on both; the channel selected, or
    /// none when neither is or both are.
    pub fn select(&mut self, counts: [u32; 2]) -> Option<u8> {
        let drives = [count_q16(counts[0]), count_q16(counts[1])];
        let mut selected = [false; 2];
        for (k, channel) in self.channels.iter_mut().enumerate() {
            let other = if k == 0 { 1 } else { 0 };
            channel.striatal_d1_drive = drives[k];
            channel.striatal_d2_drive = drives[other];
            channel.stn_hyperdirect_drive = 0;
            selected[k] = channel.compute_gating();
        }
        match selected {
            [true, false] => Some(0),
            [false, true] => Some(1),
            _ => None,
        }
    }
}

/// Where a trial's reward takes its sign from.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Feedback {
    /// The answer: the reward for a selection equal to the stimulus's rewarded readout, the
    /// punishment otherwise, a tie included.
    Answer,
    /// A coin drawn from the trial's index by [`mix64`], the stimuli unchanged: the same total
    /// dopamine with none of the information (the shuffled-reward control of ADR-0060).
    Shuffled,
    /// No reward call at all: the pair rule under the baseline alone (the fixed-modulation
    /// control).
    Withheld,
}

/// Why a task is refused, or a trial not run.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TaskError {
    /// A stimulus with no message would present nothing.
    NoStimulus,
    /// A stimulus or readout set of no units.
    EmptySet,
    /// A set that reaches past the unit arena.
    SetOutsideArena,
    /// Two of the four sets share a unit.
    SetsOverlap,
    /// A trial of no ticks.
    NoTicks,
    /// The executor's train holds fewer spikes than a trial can produce
    /// (`units × spikes_per_unit(ticks)`), so a trial could be mis-read rather than read.
    TrainTooSmall,
    /// A readout set whose count could reach the drive's width (`i16::MAX` spikes), where two
    /// counts would tie at the saturation.
    CountBeyondWidth,
    /// A reward magnitude below zero: the sign is the outcome's, never the constant's.
    NegativeReward,
    /// A reward magnitude of zero with feedback that would deliver it.
    NoReward,
    /// The modulation baseline at 1.0 with a reward that would be delivered: a positive reward
    /// adds nothing at the ceiling (the clamp is there already), the configuration that
    /// silently does nothing.
    RewardAtCeiling,
    /// The injector refused a message of the stimulus or of the drive; the trial stopped there.
    Inject(InjectError),
}

impl From<InjectError> for TaskError {
    fn from(e: InjectError) -> Self {
        TaskError::Inject(e)
    }
}

/// What one trial did.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Outcome {
    /// The trial's index.
    pub trial: u64,
    /// The stimulus presented, 0 or 1.
    pub stimulus: u8,
    /// The trial's spikes in each readout set.
    pub counts: [u32; 2],
    /// The readout selected, or none at a tie.
    pub selection: Option<u8>,
    /// Whether the selection was the stimulus's rewarded readout.
    pub correct: bool,
    /// The reward delivered, signed; zero when withheld.
    pub reward_q16: i32,
    /// The modulator's dopamine signal after the reward.
    pub signal_q16: i32,
}

/// A two-alternative task on an executor: two stimuli, two readouts, a background drive, a
/// trial's length, a seed, a reward magnitude, the assignment of stimuli to readouts and
/// where the reward's sign comes from. Every field is the caller's; `check` says what a run
/// needs of them and of the executor.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Task {
    pub stimuli: [Stimulus; 2],
    pub readout: Readout,
    /// The background drive, run on every tick of every trial; a drive whose `every` is zero
    /// is never due.
    pub drive: Drive,
    /// Ticks per trial, at least one.
    pub ticks: u32,
    /// The seed the trial's stimulus and the shuffled coin are drawn from.
    pub seed: u64,
    /// The reward's magnitude, Q16.16, at least zero; the sign is the outcome's.
    pub reward_q16: i32,
    /// Stimulus `s` is rewarded at readout `s` when false, at the other when true.
    pub mirrored: bool,
    pub feedback: Feedback,
}

impl Task {
    /// The readout stimulus `stimulus` is rewarded at.
    pub const fn answer(&self, stimulus: u8) -> u8 {
        if self.mirrored {
            stimulus ^ 1
        } else {
            stimulus
        }
    }

    /// The stimulus of trial `trial`: the low bit of [`mix64`] of the seed and the index.
    pub const fn stimulus_at(&self, trial: u64) -> u8 {
        (mix64(self.seed ^ trial) & 1) as u8
    }

    /// The shuffled reward's sign for trial `trial`: bit 32 of the same draw, a coin the
    /// stimulus's bit does not read.
    pub const fn coin_at(&self, trial: u64) -> bool {
        (mix64(self.seed ^ trial) >> 32) & 1 == 1
    }

    /// What a run needs: every set non-empty and inside `exec`'s arena, no two sharing a
    /// unit, a stimulus with a message, a trial with a tick, a train that holds the most
    /// spikes a trial can produce, a readout count that cannot reach the drive's width, and
    /// a reward that is delivered only where it can do something.
    pub fn check<const CAP: usize>(&self, exec: &Executor<CAP>) -> Result<(), TaskError> {
        let units = exec.units().len() as u64;
        let sets = [
            self.stimuli[0].set,
            self.stimuli[1].set,
            self.readout.sets[0],
            self.readout.sets[1],
        ];
        for set in &sets {
            if set.len == 0 {
                return Err(TaskError::EmptySet);
            }
            if set.end() > units {
                return Err(TaskError::SetOutsideArena);
            }
        }
        for (i, a) in sets.iter().enumerate() {
            for b in sets.iter().skip(i.saturating_add(1)) {
                if a.overlaps(b) {
                    return Err(TaskError::SetsOverlap);
                }
            }
        }
        if self.stimuli.iter().any(|s| s.messages == 0) {
            return Err(TaskError::NoStimulus);
        }
        if self.ticks == 0 {
            return Err(TaskError::NoTicks);
        }
        let per_unit = u64::from(spikes_per_unit(self.ticks));
        if (exec.train_capacity() as u64) < units.saturating_mul(per_unit) {
            return Err(TaskError::TrainTooSmall);
        }
        if self
            .readout
            .sets
            .iter()
            .any(|set| u64::from(set.len).saturating_mul(per_unit) > i16::MAX as u64)
        {
            return Err(TaskError::CountBeyondWidth);
        }
        if self.reward_q16 < 0 {
            return Err(TaskError::NegativeReward);
        }
        if self.feedback != Feedback::Withheld {
            if self.reward_q16 == 0 {
                return Err(TaskError::NoReward);
            }
            if exec.modulation_baseline_q16() == MODULATION_ONE_Q16 {
                return Err(TaskError::RewardAtCeiling);
            }
        }
        Ok(())
    }

    /// One trial: the stimulus injected, `ticks` ticks under the drive, the train read once,
    /// the selection, and the reward delivered between ticks, its sign by `feedback`. Refused
    /// as `check` refuses, and when the injector refuses a message.
    pub fn trial<const CAP: usize>(
        &mut self,
        exec: &mut Executor<CAP>,
        trial: u64,
    ) -> Result<Outcome, TaskError> {
        self.check(exec)?;
        let start = exec.ticks();
        let stimulus = self.stimulus_at(trial);
        let inject = exec.injector();
        self.stimuli[stimulus as usize].inject(&inject)?;
        for _ in 0..self.ticks {
            self.drive.step(&inject, exec.ticks())?;
            exec.tick();
        }
        // The train's stamp is the tick's low word (§8.4), as `start` is read here.
        let counts = self.readout.count(exec.train(), start as u32, self.ticks);
        let selection = self.readout.select(counts);
        let correct = selection == Some(self.answer(stimulus));
        let positive = match self.feedback {
            Feedback::Answer => correct,
            Feedback::Shuffled => self.coin_at(trial),
            Feedback::Withheld => {
                return Ok(Outcome {
                    trial,
                    stimulus,
                    counts,
                    selection,
                    correct,
                    reward_q16: 0,
                    signal_q16: exec.modulator().dopamine_rpe,
                });
            }
        };
        let reward_q16 = if positive {
            self.reward_q16
        } else {
            self.reward_q16.saturating_neg()
        };
        let signal_q16 = exec.reward(reward_q16);
        Ok(Outcome {
            trial,
            stimulus,
            counts,
            selection,
            correct,
            reward_q16,
            signal_q16,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::executor::Config;
    use cortex_core::{STP_MAX, STP_U, THRESHOLD_BASE};
    use cortex_neuromod::{DOPAMINE_TAU_SHIFT, NeuromodulatorState};

    const ONE: i32 = MODULATION_ONE_Q16;
    /// The replay drive's message (ADR-0038): two of 1.25 fire an armed unit once.
    const CUE_Q16: i32 = 0x0001_4000;
    const TICKS: u32 = 64;
    const REWARD: i32 = 0x4000;

    fn set(first: u32, len: u32) -> Set {
        Set { first, len }
    }

    fn stimulus(first: u32) -> Stimulus {
        Stimulus {
            set: set(first, 4),
            messages: 2,
            efficacy_q16: CUE_Q16,
        }
    }

    /// Sixteen armed units without synapses: stimuli at `[0, 4)` and `[8, 12)`, readouts at
    /// `[4, 8)` and `[12, 16)`; a train that holds a trial.
    fn network(workers: usize, baseline_q16: i32) -> Executor<8> {
        let mut exec = Executor::<8>::new(Config {
            workers,
            units: 16,
            injector_capacity: 64,
            train_capacity: spikes_per_unit(TICKS).saturating_mul(16) as usize,
            modulation_baseline_q16: baseline_q16,
            ..Config::default()
        })
        .unwrap();
        for unit in exec.units_mut() {
            unit.v_thresh = THRESHOLD_BASE;
            unit.stp_u_rel = STP_U;
            unit.stp_r_ves = STP_MAX;
        }
        exec
    }

    fn task(feedback: Feedback) -> Task {
        Task {
            stimuli: [stimulus(0), stimulus(8)],
            readout: Readout::new([set(4, 4), set(12, 4)]),
            drive: Drive {
                every: 0,
                messages: 0,
                efficacy_q16: 0,
                units: 16,
                seed: 0,
            },
            ticks: TICKS,
            seed: 0,
            reward_q16: REWARD,
            mirrored: false,
            feedback,
        }
    }

    /// Cues every unit of `set` through the injector, as a stimulus would.
    fn cue(exec: &Executor<8>, set: Set) {
        let inject = exec.injector();
        for k in 0..set.len {
            for _ in 0..2 {
                inject
                    .inject(set.first.wrapping_add(k), spike_message(CUE_Q16, false))
                    .unwrap();
            }
        }
    }

    #[test]
    fn the_spike_bound_is_one_per_refractory_window_the_last_counted_whole() {
        assert_eq!(MIN_INTERVAL_TICKS, 50, "the burst window, the shorter");
        assert_eq!(spikes_per_unit(0), 0);
        assert_eq!(spikes_per_unit(1), 1);
        assert_eq!(spikes_per_unit(50), 1);
        assert_eq!(spikes_per_unit(51), 2);
        assert_eq!(spikes_per_unit(4096), 82);
        assert_eq!(spikes_per_unit(u32::MAX), 85_899_346);
    }

    #[test]
    fn a_set_knows_its_end_its_units_and_its_overlaps() {
        let s = set(4, 4);
        assert_eq!(s.end(), 8);
        assert!(!s.contains(3) && s.contains(4) && s.contains(7) && !s.contains(8));
        assert!(s.overlaps(&set(7, 10)) && s.overlaps(&set(0, 5)) && s.overlaps(&set(5, 1)));
        assert!(!s.overlaps(&set(8, 4)) && !s.overlaps(&set(0, 4)));
        assert!(!s.overlaps(&set(5, 0)), "an empty set shares no unit");
        assert!(!set(5, 0).overlaps(&s));
        let top = set(u32::MAX - 1, 2);
        assert_eq!(
            top.end(),
            1u64 << 32,
            "the end past the index space is widened"
        );
        assert!(top.contains(u32::MAX) && !top.contains(u32::MAX - 2));
    }

    #[test]
    fn every_refusal_is_named() {
        let exec = network(1, ONE / 2);
        let ok = task(Feedback::Answer);
        assert_eq!(ok.check(&exec), Ok(()));
        let mut t = ok;
        t.stimuli[1].messages = 0;
        assert_eq!(t.check(&exec), Err(TaskError::NoStimulus));
        let mut t = ok;
        t.readout = Readout::new([set(4, 0), set(12, 4)]);
        assert_eq!(t.check(&exec), Err(TaskError::EmptySet));
        let mut t = ok;
        t.stimuli[0].set = set(0, 0);
        assert_eq!(t.check(&exec), Err(TaskError::EmptySet));
        let mut t = ok;
        t.readout = Readout::new([set(4, 4), set(13, 4)]);
        assert_eq!(t.check(&exec), Err(TaskError::SetOutsideArena));
        let mut t = ok;
        t.stimuli[1].set = set(u32::MAX - 1, 2);
        assert_eq!(t.check(&exec), Err(TaskError::SetOutsideArena));
        let mut t = ok;
        t.stimuli[1].set = set(7, 4);
        assert_eq!(
            t.check(&exec),
            Err(TaskError::SetsOverlap),
            "a stimulus on a readout"
        );
        let mut t = ok;
        t.readout = Readout::new([set(4, 4), set(6, 2)]);
        assert_eq!(
            t.check(&exec),
            Err(TaskError::SetsOverlap),
            "the two readouts"
        );
        let mut t = ok;
        t.stimuli = [stimulus(0), stimulus(3)];
        assert_eq!(
            t.check(&exec),
            Err(TaskError::SetsOverlap),
            "the two stimuli"
        );
        let mut t = ok;
        t.ticks = 0;
        assert_eq!(t.check(&exec), Err(TaskError::NoTicks));
        let mut t = ok;
        t.ticks = 2 * MIN_INTERVAL_TICKS;
        assert_eq!(
            t.check(&exec),
            Ok(()),
            "two windows: two spikes per unit, which the train holds"
        );
        t.ticks = 2 * MIN_INTERVAL_TICKS + 1;
        assert_eq!(
            t.check(&exec),
            Err(TaskError::TrainTooSmall),
            "a third refractory window is one more spike per unit than the train holds"
        );
        let wide = {
            let mut exec = Executor::<8>::new(Config {
                units: 1 << 16,
                injector_capacity: 64,
                train_capacity: 1 << 22,
                modulation_baseline_q16: ONE / 2,
                ..Config::default()
            })
            .unwrap();
            exec.units_mut()[0].v_thresh = THRESHOLD_BASE;
            exec
        };
        let mut t = ok;
        t.readout = Readout::new([set(4, 4), set(16, 32_768)]);
        t.ticks = 1;
        assert_eq!(
            t.check(&wide),
            Err(TaskError::CountBeyondWidth),
            "32 768 units at one spike each reach the width"
        );
        t.readout = Readout::new([set(4, 4), set(16, 32_767)]);
        assert_eq!(t.check(&wide), Ok(()), "one fewer fits");
        t.ticks = 51;
        assert_eq!(
            t.check(&wide),
            Err(TaskError::CountBeyondWidth),
            "two spikes per unit double the bound"
        );
        let mut t = ok;
        t.reward_q16 = -1;
        assert_eq!(t.check(&exec), Err(TaskError::NegativeReward));
        let mut t = ok;
        t.reward_q16 = 0;
        assert_eq!(t.check(&exec), Err(TaskError::NoReward));
        t.feedback = Feedback::Shuffled;
        assert_eq!(t.check(&exec), Err(TaskError::NoReward));
        t.feedback = Feedback::Withheld;
        assert_eq!(t.check(&exec), Ok(()), "withheld, the magnitude is unused");
        let ceiling = network(1, ONE);
        assert_eq!(ok.check(&ceiling), Err(TaskError::RewardAtCeiling));
        let mut t = ok;
        t.feedback = Feedback::Shuffled;
        assert_eq!(t.check(&ceiling), Err(TaskError::RewardAtCeiling));
        t.feedback = Feedback::Withheld;
        assert_eq!(
            t.check(&ceiling),
            Ok(()),
            "the fixed-modulation control: the baseline at 1.0 and no reward call"
        );
        let below = network(1, ONE - 1);
        assert_eq!(
            ok.check(&below),
            Ok(()),
            "one LSB below the ceiling has room"
        );
        // A trial refuses as `check` refuses, before it injects anything.
        let mut exec = network(1, ONE / 2);
        let mut t = ok;
        t.ticks = 0;
        assert_eq!(t.trial(&mut exec, 0), Err(TaskError::NoTicks));
        assert_eq!(exec.ticks(), 0);
        assert!(exec.is_quiescent(), "nothing injected");
        // The injector's refusal is the trial's: a ring of four holds two of the eight.
        let mut small = Executor::<8>::new(Config {
            units: 16,
            injector_capacity: 4,
            train_capacity: spikes_per_unit(TICKS).saturating_mul(16) as usize,
            modulation_baseline_q16: ONE / 2,
            ..Config::default()
        })
        .unwrap();
        let mut t = ok;
        assert_eq!(
            t.trial(&mut small, 0),
            Err(TaskError::Inject(InjectError::Full))
        );
        assert_eq!(
            TaskError::from(InjectError::NoSuchUnit),
            TaskError::Inject(InjectError::NoSuchUnit)
        );
    }

    #[test]
    fn a_trial_with_every_spike_in_one_set_selects_it_and_a_tie_selects_nothing() {
        let mut exec = network(2, ONE / 2);
        let mut t = task(Feedback::Answer);
        // Trial 0's stimulus, whichever it is, fires its own four units; the readouts hold no
        // spike, so the two counts tie and nothing is selected: an error, punished.
        let tie = t.trial(&mut exec, 0).unwrap();
        assert_eq!(tie.stimulus, t.stimulus_at(0));
        assert_eq!(tie.counts, [0, 0]);
        assert_eq!(tie.selection, None);
        assert!(!tie.correct);
        assert_eq!(tie.reward_q16, -REWARD);
        let channels = t.readout.channels();
        assert_eq!(
            (channels[0].gpi_snr_inhibition, channels[0].selected_flag),
            (0, 0)
        );
        assert_eq!(
            (channels[1].gpi_snr_inhibition, channels[1].selected_flag),
            (0, 0)
        );
        assert_eq!((channels[0].channel_id, channels[1].channel_id), (0, 1));
        // The stimulus fired its set: four spikes in the trial, none in a readout.
        let fired: Vec<u32> = exec.train().iter().map(|&(_, u)| u).collect();
        let s = t.stimuli[tie.stimulus as usize].set;
        assert_eq!(fired, (s.first..s.first + 4).collect::<Vec<u32>>());
        // Readout 0 cued before the trial fires within it: every readout spike in set 0.
        exec.run(400);
        cue(&exec, *t.readout.sets().first().unwrap());
        let one = t.trial(&mut exec, 1).unwrap();
        assert_eq!(one.counts, [4, 0]);
        assert_eq!(one.selection, Some(0));
        assert_eq!(one.correct, one.stimulus == 0);
        let channels = t.readout.channels();
        assert_eq!(
            (
                channels[0].striatal_d1_drive,
                channels[0].striatal_d2_drive,
                channels[0].stn_hyperdirect_drive,
                channels[0].gpi_snr_inhibition,
                channels[0].selected_flag
            ),
            (4 * ONE, 0, 0, -4 * ONE, 1)
        );
        assert_eq!(
            (
                channels[1].striatal_d1_drive,
                channels[1].striatal_d2_drive,
                channels[1].gpi_snr_inhibition,
                channels[1].selected_flag
            ),
            (0, 4 * ONE, 4 * ONE, 0)
        );
        // Readout 1 cued: the other channel.
        exec.run(400);
        cue(&exec, t.readout.sets()[1]);
        let two = t.trial(&mut exec, 2).unwrap();
        assert_eq!(two.counts, [0, 4]);
        assert_eq!(two.selection, Some(1));
        assert_eq!(two.correct, two.stimulus == 1);
        // Both cued: a tie of four and four, no selection.
        exec.run(400);
        cue(&exec, t.readout.sets()[0]);
        cue(&exec, t.readout.sets()[1]);
        let both = t.trial(&mut exec, 3).unwrap();
        assert_eq!(both.counts, [4, 4]);
        assert_eq!(both.selection, None);
        assert!(!both.correct);
        // Mirrored, the same counts answer the other stimulus.
        let mut mirrored = task(Feedback::Answer);
        mirrored.mirrored = true;
        assert_eq!((mirrored.answer(0), mirrored.answer(1)), (1, 0));
        assert_eq!((t.answer(0), t.answer(1)), (0, 1));
        exec.run(400);
        cue(&exec, mirrored.readout.sets()[0]);
        let m = mirrored.trial(&mut exec, 1).unwrap();
        assert_eq!(m.selection, Some(0));
        assert_eq!(m.correct, m.stimulus == 1);
        assert_eq!(
            m.stimulus, one.stimulus,
            "the same seed draws the same stimulus"
        );
    }

    #[test]
    fn the_counts_are_those_of_a_hand_built_train_across_the_wrap() {
        let readout = Readout::new([set(4, 4), set(12, 4)]);
        // A trial of 8 ticks from `u32::MAX - 3`: the stamps wrap through zero.
        let start = u32::MAX - 3;
        let train = [
            (start - 1, 4),  // before the trial: not counted
            (start - 1, 12), // before the trial
            (start, 4),      // the first tick: set 0
            (start, 5),      // set 0
            (start, 9),      // a stimulus unit: neither set
            (start + 3, 12), // after the wrap: set 1
            (0, 7),          // set 0
            (1, 15),         // set 1
            (3, 3),          // just below set 0
            (3, 8),          // just above set 0
            (3, 16),         // above set 1
            (3, 13),         // the last tick of the trial: set 1
        ];
        assert_eq!(readout.count(&train, start, 8), [3, 3]);
        assert_eq!(
            readout.count(&train[..8], start, 7),
            [3, 2],
            "a trial one tick shorter, read at its end: the last tick is out"
        );
        assert_eq!(
            readout.count(&train[..5], start, 1),
            [2, 0],
            "the first tick alone"
        );
        assert_eq!(
            readout.count(&train, start - 1, 9),
            [4, 4],
            "one tick earlier takes the two before"
        );
        assert_eq!(readout.count(&[], start, 8), [0, 0]);
        assert_eq!(
            readout.count(&train[..2], start, 8),
            [0, 0],
            "only older entries"
        );
        assert_eq!(
            readout.count(&train[2..], 0, 5),
            [1, 2],
            "a trial from zero reads the wrapped tail"
        );
        assert_eq!(
            readout.count(&train, start, 7),
            [0, 0],
            "an entry after the trial stops the scan: the contract is a train read at the trial's end"
        );
    }

    #[test]
    fn a_selection_is_the_sign_of_the_count_difference_and_saturates_at_the_width() {
        let mut r = Readout::new([set(4, 4), set(12, 4)]);
        assert_eq!(r.select([0, 0]), None);
        assert_eq!(r.select([1, 0]), Some(0));
        assert_eq!(r.select([0, 1]), Some(1));
        assert_eq!(r.select([32_767, 32_766]), Some(0));
        assert_eq!(r.select([32_766, 32_767]), Some(1));
        assert_eq!(r.channels()[1].striatal_d1_drive, 32_767 * ONE);
        assert_eq!(count_q16(32_767), 32_767 * ONE);
        assert_eq!(count_q16(32_768), i32::MAX, "the width, saturated");
        assert_eq!(count_q16(u32::MAX), i32::MAX);
        assert_eq!(
            r.select([32_769, 32_768]),
            None,
            "beyond the width two counts tie: the mis-read `check` refuses"
        );
        assert_eq!(
            r.select([32_768, 32_767]),
            Some(0),
            "one at the width still reads"
        );
    }

    #[test]
    fn the_reward_s_sign_and_magnitude_reach_the_modulator_against_an_oracle() {
        let mut exec = network(1, ONE / 2);
        let mut oracle = NeuromodulatorState::new();
        let mut t = task(Feedback::Answer);
        // Trial 0: readout 0 cued; correct when the stimulus is 0.
        cue(&exec, t.readout.sets()[0]);
        let first = t.trial(&mut exec, 0).unwrap();
        let expected = if first.correct { REWARD } else { -REWARD };
        assert_eq!(first.reward_q16, expected);
        assert_eq!(oracle.reward(expected), first.signal_q16);
        assert_eq!(exec.modulator().dopamine_rpe, first.signal_q16);
        assert_eq!(
            exec.modulator().modulation(ONE / 2),
            oracle.modulation(ONE / 2)
        );
        // Every tick until the next reward decays the signal once (the first trial's ticks
        // came before its reward); the second reward adds to what is left, the other way
        // round (readout 1 cued, so the outcome flips).
        exec.run(400 - TICKS as u64);
        for _ in 0..(400 - TICKS) {
            oracle.decay_dopamine(DOPAMINE_TAU_SHIFT);
        }
        cue(&exec, t.readout.sets()[1]);
        let second = t.trial(&mut exec, 0).unwrap();
        assert_eq!(second.stimulus, first.stimulus);
        assert_eq!(second.correct, !first.correct, "the other readout won");
        for _ in 0..TICKS {
            oracle.decay_dopamine(DOPAMINE_TAU_SHIFT);
        }
        assert_eq!(second.reward_q16, -expected);
        assert_eq!(oracle.reward(-expected), second.signal_q16);
        assert_eq!(exec.modulator().dopamine_rpe, second.signal_q16);
        assert_eq!(
            exec.modulator().modulation(ONE / 2),
            (ONE / 2).saturating_add(second.signal_q16).clamp(0, ONE)
        );
        // Shuffled: the sign is the coin's, whatever the outcome.
        let mut exec = network(1, ONE / 2);
        let mut s = task(Feedback::Shuffled);
        let coin = (0..8u64).find(|&k| s.coin_at(k) != s.coin_at(0)).unwrap();
        let heads = s.trial(&mut exec, 0).unwrap();
        assert_eq!(
            heads.reward_q16,
            if s.coin_at(0) { REWARD } else { -REWARD }
        );
        assert!(!heads.correct, "a tie, so the answer would have punished");
        let mut exec = network(1, ONE / 2);
        let tails = s.trial(&mut exec, coin).unwrap();
        assert_eq!(tails.reward_q16, -heads.reward_q16);
        assert_eq!(tails.signal_q16, tails.reward_q16);
        // Withheld: no reward call, the signal at rest whatever the outcome.
        let mut exec = network(1, ONE);
        let mut w = task(Feedback::Withheld);
        cue(&exec, w.readout.sets()[0]);
        let none = w.trial(&mut exec, 0).unwrap();
        assert_eq!(none.selection, Some(0));
        assert_eq!((none.reward_q16, none.signal_q16), (0, 0));
        assert!(exec.modulator().is_at_rest());
    }

    #[test]
    fn a_trial_reads_the_same_counts_on_one_and_four_workers() {
        let mut outcomes = Vec::new();
        for workers in [1, 4] {
            let mut exec = network(workers, ONE / 2);
            let mut t = task(Feedback::Answer);
            cue(&exec, t.readout.sets()[0]);
            let mut run = Vec::new();
            for trial in 0..4u64 {
                run.push(t.trial(&mut exec, trial).unwrap());
                exec.run(400);
                cue(&exec, t.readout.sets()[(trial % 2) as usize]);
            }
            outcomes.push(run);
        }
        assert_eq!(outcomes[0], outcomes[1]);
        assert_eq!(outcomes[0][0].counts, [4, 0]);
        assert_eq!(outcomes[0][1].counts, [4, 0]);
        assert_eq!(outcomes[0][2].counts, [0, 4]);
    }

    #[test]
    fn the_stimulus_and_the_coin_are_functions_of_the_seed_and_the_trial() {
        let t = task(Feedback::Answer);
        let mut ones = 0u32;
        let mut heads = 0u32;
        let mut agree = 0u32;
        for trial in 0..4096u64 {
            let s = t.stimulus_at(trial);
            assert!(s < 2);
            assert_eq!(s, t.stimulus_at(trial), "the same draw twice");
            ones = ones.saturating_add(u32::from(s));
            heads = heads.saturating_add(u32::from(t.coin_at(trial)));
            agree = agree.saturating_add(u32::from(t.coin_at(trial) == (s == 1)));
        }
        assert!((1900..2200).contains(&ones), "{ones}");
        assert!((1900..2200).contains(&heads), "{heads}");
        assert!(
            (1900..2200).contains(&agree),
            "the coin does not read the stimulus: {agree}"
        );
        let other = Task { seed: 1, ..t };
        let differ = (0..64u64)
            .filter(|&k| other.stimulus_at(k) != t.stimulus_at(k))
            .count();
        assert!(differ > 16, "{differ}");
        assert_eq!(t.stimulus_at(0), (mix64(0) & 1) as u8);
        assert_eq!(t.coin_at(0), (mix64(0) >> 32) & 1 == 1);
    }

    #[test]
    fn a_stimulus_injects_its_messages_and_stops_where_the_ring_refuses() {
        let exec = network(1, ONE / 2);
        let inject = exec.injector();
        assert_eq!(stimulus(0).inject(&inject), Ok(8));
        let big = Stimulus {
            set: set(0, 16),
            messages: 4,
            efficacy_q16: CUE_Q16,
        };
        assert_eq!(
            big.inject(&inject),
            Err(InjectError::Full),
            "a ring of 64 with 8 in it refuses the 57th"
        );
        let mut exec = exec;
        exec.run(2);
        assert_eq!(exec.delivered(), 64, "what fitted was delivered");
    }
}

/// The lattice property (ADR-0030): over seeded trains and seeded set pairs the selection is
/// the sign of the count difference, none exactly at equal counts.
#[cfg(test)]
mod prop {
    use super::*;
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../testkit/prop.rs"
    ));

    #[test]
    fn the_selection_is_the_sign_of_the_count_difference() {
        let mut lcg = Lcg::new(27);
        for _ in 0..2000 {
            // Two disjoint sets on a ring of 256, the second after a gap.
            let a_first = lcg.below(128);
            let a_len = lcg.below(32).saturating_add(1);
            let b_first = a_first.saturating_add(a_len).saturating_add(lcg.below(32));
            let b_len = lcg.below(32).saturating_add(1);
            let readout = Readout::new([
                Set {
                    first: a_first,
                    len: a_len,
                },
                Set {
                    first: b_first,
                    len: b_len,
                },
            ]);
            // A tick-ordered train: some entries before the trial, the rest inside it.
            let start = lcg.next_u32();
            let ticks = lcg.below(4096).saturating_add(1);
            let before = lcg.below(64);
            let inside = lcg.below(256);
            let mut train: Vec<(u32, u32)> = Vec::new();
            for _ in 0..before {
                train.push((
                    start.wrapping_sub(lcg.below(1000).saturating_add(1)),
                    lcg.below(256),
                ));
            }
            for _ in 0..inside {
                train.push((start.wrapping_add(lcg.below(ticks)), lcg.below(256)));
            }
            train.sort_by_key(|&(tick, unit)| (tick.wrapping_sub(start).wrapping_add(1000), unit));
            let counts = readout.count(&train, start, ticks);
            let expected = [
                train
                    .iter()
                    .filter(|&&(t, u)| {
                        t.wrapping_sub(start) < ticks && readout.sets()[0].contains(u)
                    })
                    .count() as u32,
                train
                    .iter()
                    .filter(|&&(t, u)| {
                        t.wrapping_sub(start) < ticks && readout.sets()[1].contains(u)
                    })
                    .count() as u32,
            ];
            assert_eq!(counts, expected);
            let mut readout = readout;
            let selection = readout.select(counts);
            let expected = match counts[0].cmp(&counts[1]) {
                core::cmp::Ordering::Greater => Some(0),
                core::cmp::Ordering::Less => Some(1),
                core::cmp::Ordering::Equal => None,
            };
            assert_eq!(selection, expected, "{counts:?}");
        }
        // The lattice of counts within the width.
        let mut readout = Readout::new([Set { first: 0, len: 1 }, Set { first: 1, len: 1 }]);
        for &a in &[0u32, 1, 2, 3, 32_766, 32_767] {
            for &b in &[0u32, 1, 2, 3, 32_766, 32_767] {
                let expected = match a.cmp(&b) {
                    core::cmp::Ordering::Greater => Some(0),
                    core::cmp::Ordering::Less => Some(1),
                    core::cmp::Ordering::Equal => None,
                };
                assert_eq!(readout.select([a, b]), expected, "{a} {b}");
            }
        }
    }
}
