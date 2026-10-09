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
//! ADR-0038), the background drive runs every tick, the train is read once at the end over
//! the task's [`Window`] of the trial (the whole trial, or the ticks in which a stimulus's
//! local synapses land; ADR-0065), the channels select, and the reward is delivered
//! before the next trial's first tick, inside the eligibility trace's window
//! ([`cortex_core::ELIGIBILITY_TAU_SHIFT`]) whatever the trial's length below it. A [`Set`]
//! is a periodic pattern of units since ADR-0065, so that a stimulus can be units spaced
//! beyond the prior's local window, each firing once. A run's `(stimulus, selection,
//! correct)` sequence is bit-identical on every worker count, since the train is (ADR-0023,
//! ADR-0050).
//!
//! A stimulus may carry a [`Cancel`] (ADR-0076): basal messages of negative efficacy into the
//! same units, injected between ticks inside the trial from an offset, for as many consecutive
//! ticks as the cancel says. The membrane rule drops an input that lands inside a unit's
//! refractory window (ADR-0018), so a cancel meant to stop a unit firing again at the window's
//! end lands on the first tick the unit integrates again, `REFRACTORY_TICKS + 1` after its
//! spike; a set whose volley spreads over several ticks is covered by as many ticks of the
//! cancel. A stimulus with no cancel injects what it injected before the cancel existed, and
//! every run pinned before ADR-0076 reruns unchanged.
//!
//! A task may carry a [`Critic`] (ADR-0106, ADR-0107): an expected reward for each stimulus,
//! from which the outcome's reward is taken before the modulator receives it, so that the
//! dopamine signal receives a prediction error and not the reward itself. The critic is the
//! task's state, as the seed and the trial's index are: no record holds it and no image carries
//! it. A task with no critic delivers the outcome's reward, and every run pinned before
//! ADR-0107 reruns unchanged.
//!
//! The engine may carry a critic of its own instead (ADR-0130, ADR-0131): a value weight on
//! every unit, read from the engine's own spikes since the previous reward. With it set the
//! task delivers the outcome's reward, the engine takes its value from it, and the trial
//! records the error the modulator received and the value; a task that carries a critic of its
//! own on such an engine is refused, since two critics would take the expectation twice.
//!
//! Where a trial's dopamine term reaches is the task's [`Delivery`] (ADR-0068): every synapse,
//! or the synapses from the stimulus it presented onto the readout the engine selected. Since
//! ADR-0139 the engine may draw the sources itself: the units its critic counted within its
//! window, onto the readout the engine selected, so that the task gives the channel the
//! engine's own selection chose and not the stimulus it drew. The two deliveries before it
//! write what they wrote, and every run pinned under them reruns unchanged.
//!
//! A task may carry a [`Hold`] (ADR-0143, ADR-0144): the gate's output delivered to the network.
//! With it the selection is made at the readout window's close, from the counts it reads at the
//! trial's end without it, and from there the channel the gate holds — the one whose net
//! output `compute_gating` left above zero, the channel not selected — receives basal messages
//! of negative efficacy into every unit of its set, on a cadence, until the hold's last tick;
//! nothing at a tie, where both outputs are zero. Beside it stands the released delivery
//! ([`Delivery::Released`]): the address's sources drawn by the engine as the drawn delivery
//! draws them and every unit a target, at a tie as at any trial. A task with no hold runs the
//! trial it ran, and the three deliveries before the released one write what they wrote.
//!
//! A task's reward may be right seven times in eight ([`Feedback::SevenInEight`], ADR-0147,
//! ADR-0148): its sign the outcome's unless the trial's coin is misleading, and then the
//! opposite, whatever the outcome was. The coin is three bits of the trial's own draw that
//! neither the stimulus nor the shuffled control reads ([`MISLEADING_BITS`]), so a run stays a
//! function of its seed. What a trial records as correct is the selection, whatever reward it
//! then received. Under the three feedbacks before it a trial is the trial it was.
//!
//! A readout holds as many channels as it has sets (ADR-0151, ADR-0152): two unless the type
//! names another number. Each channel's direct drive is its own count and its indirect drive
//! the largest of the other channels' counts, so a channel is selected when its count is above
//! every other's and none where the largest is shared. The mapping is an answer for each
//! stimulus, a readout's index, and a flip moves every answer to the next readout, the last to
//! the first. With two channels the largest of the others is the other and a flip trades the
//! two answers, so a task of two readouts runs the trial it ran.
//!
//! A task's selection may be drawn ([`Exploration::ValueGated`], ADR-0162, ADR-0163). At the
//! trial's end the task reads the engine's own value, the one the trial's reward is then taken
//! against, from what the executor exposes; with a probability equal to the part of that value
//! below zero over the reward's magnitude, the selection is one of the readout's channels, each
//! with the same chance, whatever the counts, and the gate's otherwise. The coin and the draw
//! among the channels are bits of the trial's own draw that no other draw reads
//! ([`EXPLORATION_COIN_BITS`], [`EXPLORATION_CHANNEL_BITS`]), so a run stays a function of its
//! seed. Everything after the selection is as it was: what is correct, what is addressed and
//! the reward's sign are the selection's. With the exploration unset a trial is the trial it
//! was, and with it set nothing changes where the value is at or above zero.

use crate::executor::{AddressError, Executor, Inject, InjectError};
use crate::synthesis::{Drive, mix64};
use cortex_basal_ganglia::BasalGangliaChannelState;
use cortex_core::{BURST_REFRACTORY_TICKS, MODULATION_ONE_Q16, REFRACTORY_TICKS, spike_message};

/// The shortest interval between two spikes of one unit, in ticks: a spike opens a refractory
/// window during which the unit integrates nothing and cannot fire, `REFRACTORY_TICKS` long,
/// or `BURST_REFRACTORY_TICKS` after a plateau, the shorter of the two; so a unit fires at most
/// once per this many ticks and the most spikes a trial can hold is a bound the caller's train
/// is checked against. That the burst window is not the longer is asserted at compile time,
/// so the constant needs no comparison.
pub const MIN_INTERVAL_TICKS: u32 = BURST_REFRACTORY_TICKS as u32;
const _: () = assert!(
    REFRACTORY_TICKS
        .checked_sub(BURST_REFRACTORY_TICKS)
        .is_some(),
    "the burst refractory window is the shorter"
);
const _: () = assert!(MIN_INTERVAL_TICKS >= 1);

/// The most spikes one unit can hold in `ticks` ticks: one per [`MIN_INTERVAL_TICKS`], the
/// last interval counted whole.
pub const fn spikes_per_unit(ticks: u32) -> u32 {
    ticks.div_ceil(MIN_INTERVAL_TICKS)
}

/// The longest period a set's pattern can have: the width of its mask.
pub const MAX_PERIOD: u32 = 32;

/// A set of units as a periodic pattern (ADR-0065): the units `first + k × period + b` for
/// every `k` below `count` and every offset `b` below `period` whose bit of `mask` is set, in
/// that order. A contiguous run is a period of one (`mask` 1, `count` its length), a lattice
/// of one unit every `stride` is a period of `stride` with `mask` 1, and several roles
/// interleaved in one period are one mask each over the same `first` and `period`; all are
/// one shape, so the readout and the refusals read one rule. The period is at most
/// [`MAX_PERIOD`]; a mask with a bit at or beyond the period would name a unit of the next
/// period twice, and [`Task::check`] refuses it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Set {
    pub first: u32,
    pub period: u32,
    pub mask: u32,
    pub count: u32,
}

impl Set {
    /// The contiguous run `first..first + len` (ADR-0059's shape).
    pub const fn contiguous(first: u32, len: u32) -> Self {
        Self {
            first,
            period: 1,
            mask: 1,
            count: len,
        }
    }

    /// A period of at least one and at most [`MAX_PERIOD`], and a non-empty mask with no bit
    /// at or beyond the period. `count` may be zero: an empty set, refused as one.
    pub const fn is_well_formed(&self) -> bool {
        self.period >= 1
            && self.period <= MAX_PERIOD
            && self.mask != 0
            // `checked_shr` is `None` at a shift of the whole width, where any mask fits.
            && matches!(self.mask.checked_shr(self.period), None | Some(0))
    }

    /// The units of one period: the set bits of the mask.
    pub const fn per_period(&self) -> u32 {
        self.mask.count_ones()
    }

    /// The units the set names, widened.
    pub const fn len(&self) -> u64 {
        (self.count as u64).saturating_mul(self.per_period() as u64)
    }

    /// True for a set that names no unit.
    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// One past the last unit, widened so that a set at the top of the index space has an
    /// end; `first` for an empty set.
    pub const fn end(&self) -> u64 {
        let Some(last_period) = self.count.checked_sub(1) else {
            return self.first as u64;
        };
        if self.mask == 0 {
            return self.first as u64;
        }
        let top = 31u32.wrapping_sub(self.mask.leading_zeros());
        (self.first as u64)
            .saturating_add((last_period as u64).saturating_mul(self.period as u64))
            .saturating_add(top as u64)
            .saturating_add(1)
    }

    /// True for a unit of the set: at or after `first`, in a period below `count`, at an
    /// offset the mask names. A set with no period names nothing.
    pub const fn contains(&self, unit: u32) -> bool {
        let Some(offset) = unit.checked_sub(self.first) else {
            return false;
        };
        // `checked_div` and `checked_rem` are `None` at a period of zero.
        let (Some(k), Some(b)) = (
            offset.checked_div(self.period),
            offset.checked_rem(self.period),
        ) else {
            return false;
        };
        k < self.count && matches!(self.mask.checked_shr(b), Some(bit) if bit & 1 == 1)
    }

    /// The set's units in order, period by period and offset by offset within a period;
    /// none for a set the rule refuses, so that no malformed period is walked. The iterator
    /// is `Clone`, so that an addressing can scan it before it writes (ADR-0068).
    pub fn units(&self) -> impl Iterator<Item = u32> + Clone + '_ {
        let count = if self.is_well_formed() { self.count } else { 0 };
        (0..count).flat_map(move |k| {
            let base = self.first.wrapping_add(k.wrapping_mul(self.period));
            (0..self.period)
                .filter(move |&b| matches!(self.mask.checked_shr(b), Some(bit) if bit & 1 == 1))
                .map(move |b| base.wrapping_add(b))
        })
    }

    /// True when the two sets share a unit; an empty set shares none. This set's units are
    /// tested against the other's membership, so the scan is bounded by this set's length,
    /// and a caller that knows the smaller set asks it (as [`Task::check`] does, the stimuli
    /// before the readouts).
    pub fn overlaps(&self, other: &Set) -> bool {
        self.units().any(|unit| other.contains(unit))
    }
}

/// A cancel (ADR-0076): `messages` basal messages of a negative `efficacy_q16` into every unit
/// of the stimulus's set, injected between ticks before each of `ticks` consecutive ticks of
/// the trial from `offset` (the trial's ticks counted from zero), so that the first lands on
/// the tick after `offset` and the last on the tick after `offset + ticks - 1`, as the first
/// injection, made before tick zero, lands on tick one. The membrane rule drops an input that
/// lands inside a unit's refractory window (ADR-0018), so a cancel meant for the end of the
/// window lands on the first tick the unit integrates again, `REFRACTORY_TICKS + 1` after its
/// spike, and a set whose spikes spread over several ticks is covered by as many ticks. The
/// executor scales the message by the tick's synaptic gain as it scales a synapse's (F-47), and
/// clamps one message's efficacy at −2.0 (`spike_message`), so a larger cancel is more
/// messages. [`Task::check`] refuses a cancel of no message or no tick, one at offset zero (it
/// would be the first injection), one whose last message would land after the trial's last
/// tick, and one whose efficacy is not negative (a second drive, not a cancel).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Cancel {
    pub offset: u32,
    pub ticks: u32,
    pub messages: u32,
    pub efficacy_q16: i32,
}

impl Cancel {
    /// True when a message is due before trial tick `k`: `offset <= k < offset + ticks`,
    /// widened.
    pub const fn is_due(&self, k: u32) -> bool {
        (k as u64) >= (self.offset as u64) && (k as u64) < self.end()
    }

    /// One past the last tick before which a message is due, widened so that a cancel at the
    /// top of the tick space has an end.
    pub const fn end(&self) -> u64 {
        (self.offset as u64).wrapping_add(self.ticks as u64)
    }
}

/// A stimulus: `messages` basal messages of `efficacy_q16` into every unit of `set`, injected
/// between ticks before a trial's first tick, and the [`Cancel`] it carries, if any, injected
/// inside the trial (ADR-0076). The executor scales an injected message by the tick's synaptic
/// gain as it scales a synapse's (F-47), so what two messages of 1.25 (the replay drive's,
/// ADR-0038) do depends on the gain: at a gain of 1.0 they fire a unit at its base threshold
/// exactly once, and at the 1.75 the reference network runs at they arrive as 4.375 and fire
/// a unit at rest twice, at 5 and 206 ticks, the second at the end of the refractory window
/// from what the basal compartment still holds (ADR-0074's probe).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Stimulus {
    pub set: Set,
    pub messages: u32,
    pub efficacy_q16: i32,
    /// The cancel, when the stimulus carries one; none injects nothing inside the trial.
    pub cancel: Option<Cancel>,
}

impl Stimulus {
    /// Injects the stimulus; returns the messages injected. An injector that refuses one stops
    /// the stimulus there, as [`Drive::step`] stops.
    pub fn inject(&self, inject: &Inject) -> Result<u32, InjectError> {
        self.send(
            inject,
            self.messages,
            spike_message(self.efficacy_q16, false),
        )
    }

    /// Injects the cancel's messages before trial tick `k` when the stimulus carries a cancel
    /// that is due there; returns the messages injected, none otherwise. An injector that
    /// refuses one stops the cancel there, as [`inject`](Self::inject) stops.
    pub fn cancel_at(&self, inject: &Inject, k: u32) -> Result<u32, InjectError> {
        match self.cancel {
            Some(cancel) if cancel.is_due(k) => self.send(
                inject,
                cancel.messages,
                spike_message(cancel.efficacy_q16, false),
            ),
            _ => Ok(0),
        }
    }

    /// `messages` of `message` into every unit of the set, in the set's order.
    fn send(&self, inject: &Inject, messages: u32, message: u32) -> Result<u32, InjectError> {
        send(&self.set, inject, messages, message)
    }
}

/// `messages` of `message` into every unit of `set`, in the set's order; returns the messages
/// injected. An injector that refuses one stops there.
fn send(set: &Set, inject: &Inject, messages: u32, message: u32) -> Result<u32, InjectError> {
    let mut sent = 0u32;
    // Every unit below the arena, which `Task::check` bounded.
    for unit in set.units() {
        for _ in 0..messages {
            inject.inject(unit, message)?;
            sent = sent.saturating_add(1);
        }
    }
    Ok(sent)
}

/// The gate's output delivered (ADR-0143, ADR-0144): `messages` basal messages of a negative
/// `efficacy_q16` into every unit of the channel the gate holds, injected between ticks before
/// every `every`-th tick of the trial from the readout window's close while the tick is below
/// `until` (the trial's ticks counted from zero), so that the first lands on the tick after
/// the close and the last on `until` at the latest, as a [`Cancel`]'s lands on the tick after
/// its offset. The channel held is the gate's own reading: the one whose net output
/// `compute_gating` left above zero when the selection was made at the close, the channel not
/// selected; at a tie both outputs are zero and nothing is delivered. The membrane rule drops
/// an input that lands inside a unit's refractory window (ADR-0018), so a unit that fired in
/// the window's last ticks takes its first message after its window ends. The executor scales
/// the message by the tick's synaptic gain and clamps one message's efficacy at −2.0 (F-47), so
/// a deeper hold is more messages or a shorter cadence. [`Task::check`] refuses a hold of no
/// message or no cadence, one whose efficacy is not negative, one that ends at or before the
/// window's close (no tick of it is due), and one whose last message would land after the
/// trial's last tick.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Hold {
    /// One past the last trial tick before which a message can be due.
    pub until: u32,
    /// The cadence: a message is due before the tick the window closes at and before every
    /// `every`-th tick after it.
    pub every: u32,
    /// The messages into every unit of the channel held, each time one is due.
    pub messages: u32,
    /// Each message's efficacy, Q16.16, below zero.
    pub efficacy_q16: i32,
}

impl Hold {
    /// True when the hold's messages are due before trial tick `k`, the readout window closing
    /// at `close`: `k` at or after the close, below `until`, and a whole number of cadences
    /// after the close. A cadence of zero is never due.
    pub const fn is_due(&self, close: u32, k: u32) -> bool {
        match k.checked_sub(close) {
            // `checked_rem` is `None` at a cadence of zero.
            Some(since) => k < self.until && matches!(since.checked_rem(self.every), Some(0)),
            None => false,
        }
    }
}

/// The sub-window of a trial the readout counts (ADR-0065): the ticks from `from` after the
/// trial's first tick, `ticks` long. [`Window::whole`] is the trial itself, ADR-0059's readout;
/// a window derived from the prior's delay bands reads the ticks in which a stimulus's local
/// synapses land and leaves the rest of the trial's background out of the count.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Window {
    pub from: u32,
    pub ticks: u32,
}

impl Window {
    /// The whole trial of `ticks` ticks.
    pub const fn whole(ticks: u32) -> Self {
        Self { from: 0, ticks }
    }

    /// One past the window's last tick, from the trial's first, widened.
    pub const fn end(&self) -> u64 {
        (self.from as u64).wrapping_add(self.ticks as u64)
    }
}

/// The most channels a readout can hold: an action channel's index is `0..=63`
/// (`BasalGangliaChannelState::channel_id`), so a selection fits the `u8` an outcome records it
/// in. [`Readout::new`] holds a readout to it where it is compiled.
pub const MAX_CHANNELS: usize = 64;

/// The readout: `N` disjoint sets of units and the `N` action channels their spike counts
/// drive, two unless the type names another number (ADR-0151, ADR-0152). A trial's spikes are
/// counted per set from the train; each channel's own count is its direct-pathway drive and
/// the largest of the other channels' counts its indirect-pathway drive, the hyperdirect drive
/// zero, and `compute_gating` selects: the channel whose count exceeds every other's, and none
/// where the largest is shared (a net output of exactly zero is not a selection, the crate's
/// own rule). With two channels the largest of the others is the other's.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Readout<const N: usize = 2> {
    sets: [Set; N],
    channels: [BasalGangliaChannelState; N],
}

/// A count as a Q16.16 drive: `count × 1.0`, saturating at the width. [`Task::check`] refuses
/// a set whose count could reach the saturation, so that two counts never tie there.
fn count_q16(count: u32) -> i32 {
    // Below $2^{48}$ in `i64`; the minimum keeps it within `i32`.
    (i64::from(count) << 16).min(i64::from(i32::MAX)) as i32
}

impl<const N: usize> Readout<N> {
    /// A readout over `sets`, the channels at rest with their indices as ids, 0 and 1 for two.
    /// A readout of more than [`MAX_CHANNELS`] channels does not compile.
    pub fn new(sets: [Set; N]) -> Self {
        const {
            assert!(
                N <= MAX_CHANNELS,
                "a readout holds at most MAX_CHANNELS channels"
            )
        };
        let channel = |id: usize| BasalGangliaChannelState {
            // Below `MAX_CHANNELS`, held above.
            channel_id: id as u32,
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
            channels: core::array::from_fn(channel),
        }
    }

    /// The sets, one a channel.
    pub const fn sets(&self) -> &[Set; N] {
        &self.sets
    }

    /// The channels as the last selection left them.
    pub const fn channels(&self) -> &[BasalGangliaChannelState; N] {
        &self.channels
    }

    /// The spikes of each set among the entries of `train` whose tick is within `ticks` of
    /// `start` (a wrapping difference, §8.4). `train` is in tick order and holds nothing after
    /// the trial, as the executor's train does when it is read at the trial's end, so the scan
    /// runs from the newest entry back to the first one before the trial and stops.
    pub fn count(&self, train: &[(u32, u32)], start: u32, ticks: u32) -> [u32; N] {
        let mut counts = [0u32; N];
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

    /// The spikes of each set among the entries of `train` whose tick is within `ticks` of
    /// `start`, where the train may hold entries after the window (ADR-0065): the scan runs
    /// from the newest entry back, passes over the entries after the window and stops at the
    /// first one before it. Which side an entry is on is the sign of its wrapping distance
    /// from `start` read as `i32`, so the train's span must be below $2^{31}$ ticks (§8.4).
    /// On a train read at the trial's end with the whole trial as the window, this is
    /// [`count`](Self::count).
    pub fn count_window(&self, train: &[(u32, u32)], start: u32, ticks: u32) -> [u32; N] {
        let mut counts = [0u32; N];
        for &(tick, unit) in train.iter().rev() {
            let distance = tick.wrapping_sub(start);
            if (distance as i32) < 0 {
                break;
            }
            if distance >= ticks {
                continue;
            }
            for (count, set) in counts.iter_mut().zip(self.sets.iter()) {
                if set.contains(unit) {
                    *count = count.saturating_add(1);
                }
            }
        }
        counts
    }

    /// The selection from the channels' counts (ADR-0151, ADR-0152): each channel's own count
    /// into its direct drive, the largest of the other channels' counts into its indirect
    /// drive — zero for a channel with no other — its hyperdirect drive zero, and
    /// `compute_gating` on each. The gate releases a channel whose count is above every
    /// other's, so it releases at most one: the channel selected, or none where the largest
    /// count is shared. With two channels the largest of the others is the other's, the
    /// selection before ADR-0152.
    pub fn select(&mut self, counts: [u32; N]) -> Option<u8> {
        let drives = counts.map(count_q16);
        let mut selected = None;
        for (k, channel) in self.channels.iter_mut().enumerate() {
            let others = drives
                .iter()
                .enumerate()
                .filter(|&(j, _)| j != k)
                .map(|(_, &drive)| drive)
                .max()
                .unwrap_or(0);
            channel.striatal_d1_drive = drives[k];
            channel.striatal_d2_drive = others;
            channel.stn_hyperdirect_drive = 0;
            if channel.compute_gating() {
                // Below `MAX_CHANNELS`, which `new` held.
                selected = Some(k as u8);
            }
        }
        selected
    }

    /// The gate's output delivered (ADR-0143): `hold`'s messages into every unit of each
    /// channel the gate holds, a channel whose net output the last selection left above zero —
    /// with two channels the one not selected; none where every channel shares the largest
    /// count, where every output is zero, and none before a selection, the channels at rest.
    /// Among more than two (ADR-0152) it is the gate's reading still: every channel below the
    /// largest count is held, so where two share the largest a third below them is held though
    /// nothing was selected. Returns the messages injected; an injector that refuses one stops
    /// there.
    pub fn hold(&self, inject: &Inject, hold: &Hold) -> Result<u32, InjectError> {
        let message = spike_message(hold.efficacy_q16, false);
        let mut sent = 0u32;
        for (set, channel) in self.sets.iter().zip(self.channels.iter()) {
            if channel.gpi_snr_inhibition > 0 {
                sent = sent.saturating_add(send(set, inject, hold.messages, message)?);
            }
        }
        Ok(sent)
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
    /// The answer in seven trials of eight and its opposite in one (ADR-0147, ADR-0148): the
    /// reward's sign is the outcome's unless the trial's coin is misleading
    /// ([`Task::misleading_at`]), and then the other, whatever the outcome was — a correct
    /// selection punished, a wrong one or a tie rewarded.
    SevenInEight,
}

/// The bits of a trial's draw the misleading coin reads (ADR-0148): 48, 49 and 50 of
/// [`mix64`] of the seed and the trial's index. The stimulus is bit 0 of that draw and the
/// shuffled control's coin bit 32; that the three are neither, and are three, is asserted at
/// compile time, so the coin's share is one in eight and it reads no bit another draw reads.
pub const MISLEADING_BITS: u64 = 0x0007_0000_0000_0000;
const _: () = assert!(
    MISLEADING_BITS.count_ones() == 3 && MISLEADING_BITS & 0x0000_0001_0000_0001 == 0,
    "three bits, neither the stimulus's nor the shuffled coin's"
);

/// The bits of a trial's draw the exploration's coin reads (ADR-0163): 16 to 31 of [`mix64`]
/// of the seed and the trial's index, the draw's second sixteen-bit word, which no other draw
/// reads. Sixteen bits, the width of a Q16.16 fraction: at a reward of 1.0 one coin of the
/// 65 536 is one LSB of the value.
pub const EXPLORATION_COIN_BITS: u64 = 0x0000_0000_FFFF_0000;

/// The bits of a trial's draw that name the channel of a drawn selection (ADR-0163): 33 to 47
/// of the same draw, the fifteen of its third word above the shuffled control's coin.
pub const EXPLORATION_CHANNEL_BITS: u64 = 0x0000_FFFE_0000_0000;

// The two fields are whole runs of bits at the shifts `Task::exploration_coin_at` and
// `Task::drawn_at` read them at, they share no bit, and neither holds the stimulus's bit, the
// shuffled control's coin or a bit of the misleading coin.
const _: () = assert!(
    EXPLORATION_COIN_BITS == 0xFFFF << 16
        && EXPLORATION_CHANNEL_BITS == 0x7FFF << 33
        && EXPLORATION_COIN_BITS & EXPLORATION_CHANNEL_BITS == 0
        && (EXPLORATION_COIN_BITS | EXPLORATION_CHANNEL_BITS)
            & (MISLEADING_BITS | 0x0000_0001_0000_0001)
            == 0,
    "sixteen bits and fifteen, apart, and none of the stimulus's, the shuffled coin's or the misleading coin's"
);

/// Whether a trial's selection may be drawn (ADR-0162, ADR-0163).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Exploration {
    /// The selection is the gate's at every trial: the trial before ADR-0163.
    Unset,
    /// The engine's value gates it (ADR-0162). At the trial's end the task reads the engine's
    /// value $V$ — its critic's value of each unit's weight and its count since the previous
    /// reward, the value the trial's reward is then taken against — and with probability
    /// $p = \min(\max(-V, 0), r) / r$, $r$ the reward's magnitude, the selection is one of the
    /// readout's channels, each with the same chance, whatever the counts; otherwise it is the
    /// gate's. Nothing changes where the value is at or above zero. [`Task::check`] refuses it
    /// on an engine without the critic, with a hold and with a reward's magnitude of zero.
    ValueGated,
}

impl Exploration {
    /// Whether a coin of sixteen bits draws at the value `value_q16` under a reward of
    /// magnitude `reward_q16` (ADR-0163): `coin × r < below × 2^16`, where `below` is the part
    /// of the value below zero, at most the magnitude. The product is in `i64`, each factor
    /// within $2^{31}$ and $2^{16}$. So the coins that draw are the first
    /// $\lceil \text{below} \cdot 2^{16} / r \rceil$ of the 65 536: none at a value at or above
    /// zero, at least one at any value below it, and every one at minus the magnitude and
    /// beyond. The probability is $p$ rounded up to the coin's width, and $p$ itself wherever
    /// the magnitude divides $\text{below} \cdot 2^{16}$, as 1.0 does at every value. A
    /// magnitude that is not above zero draws at no coin.
    pub fn draws(coin: u16, value_q16: i32, reward_q16: i32) -> bool {
        let below = value_q16.saturating_neg().max(0).min(reward_q16);
        i64::from(coin).saturating_mul(i64::from(reward_q16)) < i64::from(below) << 16
    }

    /// The channel a draw of fifteen bits names among `channels` (ADR-0163): the floor of
    /// `draw × channels / 2^15`, so that the draws are dealt in order into `channels` runs
    /// whose lengths differ by at most one — equal where `channels` is a power of two, and
    /// 10 923, 10 923 and 10 922 of the 32 768 among three. A draw is read by its low fifteen
    /// bits, so the channel is below `channels`; zero where there is no channel.
    pub fn channel(draw: u16, channels: usize) -> u8 {
        // Within $2^{15} \cdot 2^{6}$ for a readout's channels, and below `channels` after the
        // shift, which `MAX_CHANNELS` holds within the `u8`.
        (u64::from(draw & 0x7FFF).saturating_mul(channels as u64) >> 15) as u8
    }
}

/// Where a trial's dopamine term reaches (ADR-0068): which synapses consolidate the next
/// trial's traces under `clamp(baseline + dopamine, 0, 1)`, every other synapse consolidating
/// under the baseline alone. The addressed set is a function of the trial's outcome only.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Delivery {
    /// Every synapse alike: the executor's rule before the addressing, ADR-0066's global
    /// form, so that a run under it is the run before ADR-0068 bit for bit.
    Global,
    /// The synapses from the units of the stimulus presented onto the units of the readout
    /// the engine selected through `cortex-basal-ganglia`'s gate; none at a tie, where
    /// nothing was selected. The presynaptic side narrows the set because the reward is
    /// consolidated at the next presynaptic spike, which is the next presentation of a
    /// stimulus: without the narrowing the reward of one trial would reach the other
    /// stimulus's synapses at half the trials (ADR-0068).
    Addressed,
    /// The synapses from the units the engine's critic counted within its window since the
    /// previous reward onto the units of the readout the engine selected; none at a tie, as
    /// under the addressed delivery (ADR-0138, ADR-0139). The executor draws the sources from
    /// its own counts (`Executor::address_drawn`), not from the stimulus the task presented:
    /// the task gives only the channel the engine's own selection chose. [`Task::check`]
    /// refuses it on an engine without the critic or without its window.
    Drawn,
    /// The synapses from the units the engine's critic counted within its window since the
    /// previous reward onto every unit: the drawn delivery with its targets released
    /// (ADR-0143, ADR-0144). The selection enters the address nowhere, so it is written at a
    /// tie as at any trial, and a synapse is addressed exactly when its source was drawn.
    /// [`Task::check`] refuses it as it refuses the drawn delivery.
    Released,
}

/// The critic of the reward-prediction error (ADR-0106, ADR-0107): the expected reward of each
/// stimulus, Q16.16, and the shift its update takes. At a trial the outcome's reward `r` — the
/// task's magnitude signed as the feedback signs it — meets the presented stimulus's
/// expectation `V`: the prediction error `δ = r − V`, saturating, is what [`Executor::reward`]
/// receives and what the trial records as its reward, and `V` then moves by `δ ≫ shift`, an
/// arithmetic shift, the floor of `δ / 2^shift`, saturating. The step is never larger than the
/// error and has its sign, so from an expectation within `[−|r|, |r|]` the expectation moves
/// toward the reward and never past it and stays within the bound, which [`Task::check`] holds
/// it to before every trial. A shift of 31 or more is taken as 31, where the floor already is
/// what any wider shift gives: none for an error at or above zero, one LSB down below it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Critic {
    /// The expected reward of each stimulus, Q16.16; zero at a run's start.
    pub expected_q16: [i32; 2],
    /// The update's shift: the expectation moves by `2^-shift` of the error, rounded down.
    pub shift: u32,
}

impl Critic {
    /// A critic whose every expectation is zero, as a run starts, with the update's `shift`.
    pub const fn new(shift: u32) -> Self {
        Self {
            expected_q16: [0; 2],
            shift,
        }
    }

    /// The prediction error of `reward_q16` against the expectation of `stimulus` (0 or 1),
    /// and that expectation moved by the error shifted: returns the error, and the expectation
    /// before and after the move.
    pub fn predict(&mut self, stimulus: u8, reward_q16: i32) -> (i32, i32, i32) {
        let expected = &mut self.expected_q16[usize::from(stimulus)];
        let before = *expected;
        let error = reward_q16.saturating_sub(before);
        *expected = before.saturating_add(error >> self.shift.min(31));
        (error, before, *expected)
    }
}

/// Why a task is refused, or a trial not run.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TaskError {
    /// A stimulus with no message would present nothing.
    NoStimulus,
    /// A cancel of no message or no tick would cancel nothing (ADR-0076).
    EmptyCancel,
    /// A cancel at offset zero: it would land with the first injection and be part of it.
    CancelAtInjection,
    /// A cancel whose last message would land after the trial's last tick, where the trial is
    /// over and the next trial's train begins.
    CancelOutsideTrial,
    /// A cancel whose efficacy is not negative: a second drive, not a cancel.
    CancelNotNegative,
    /// A hold of no message or no cadence would deliver nothing (ADR-0144).
    EmptyHold,
    /// A hold whose efficacy is not negative: a drive into the channel, not a hold.
    HoldNotNegative,
    /// A hold that ends at or before the readout window's close, where its first message
    /// would be due: no tick of it is.
    HoldBeforeClose,
    /// A hold whose last message would land after the trial's last tick, where the trial is
    /// over and the next trial's train begins.
    HoldOutsideTrial,
    /// A stimulus or readout set of no units.
    EmptySet,
    /// A set whose pattern the rule refuses (ADR-0065): a period of zero or beyond the mask's
    /// width, a mask of no bit, or a mask with a bit at or beyond the period, which would name
    /// a unit of the next period twice.
    MalformedSet,
    /// A set that reaches past the unit arena.
    SetOutsideArena,
    /// Two of the sets, the stimuli's and the readout's, share a unit.
    SetsOverlap,
    /// A stimulus's answer names no readout among the task's (ADR-0152): an index at or beyond
    /// the number of channels.
    AnswerOutsideReadout,
    /// A trial of no ticks.
    NoTicks,
    /// A readout window of no ticks: every trial a tie.
    EmptyWindow,
    /// A readout window that ends after the trial, where the train holds nothing yet.
    WindowOutsideTrial,
    /// The executor's train holds fewer spikes than a trial can produce
    /// (`units × spikes_per_unit(ticks)`), so a trial could be mis-read rather than read.
    TrainTooSmall,
    /// A readout set whose count over the window could reach the drive's width (`i16::MAX`
    /// spikes), where two counts would tie at the saturation.
    CountBeyondWidth,
    /// A reward magnitude below zero: the sign is the outcome's, never the constant's.
    NegativeReward,
    /// A reward magnitude of zero with feedback that would deliver it.
    NoReward,
    /// A critic whose expectation of a stimulus lies beyond the reward's magnitude, where the
    /// rule, which moves an expectation toward the reward and never past it, could not have
    /// taken it from zero (ADR-0107).
    ExpectationBeyondReward,
    /// A task that carries a critic on an engine that carries its own (ADR-0131): the
    /// expectation would be taken twice.
    TwoCritics,
    /// The modulation baseline at 1.0 with a reward that would be delivered: a positive reward
    /// adds nothing at the ceiling (the clamp is there already), the configuration that
    /// silently does nothing.
    RewardAtCeiling,
    /// The injector refused a message of the stimulus or of the drive; the trial stopped there.
    Inject(InjectError),
    /// The executor refused the addressed set (ADR-0068): a unit outside the arena, or, under
    /// the drawn delivery or the released one, an engine without the critic or without its
    /// window (ADR-0139, ADR-0144). `check` holds every readout inside the arena and refuses
    /// either delivery on such an engine before any tick, so a trial's addressing is never
    /// refused; the refusal is the executor's, surfaced here as the injector's is.
    Address(AddressError),
    /// The exploration on an engine without the critic (ADR-0163): its probability is the
    /// engine's own value, and such an engine holds none.
    ExplorationWithoutCritic,
    /// The exploration with a hold (ADR-0163): the hold delivers the gate's output from the
    /// readout window's close, before the value the reward is taken against is read, and no
    /// rule says which channel it holds at a trial whose selection is then drawn.
    ExplorationWithHold,
    /// The exploration with a reward magnitude of zero (ADR-0163): its probability is the
    /// value's part below zero over the magnitude.
    ExplorationWithoutReward,
}

impl From<InjectError> for TaskError {
    fn from(e: InjectError) -> Self {
        TaskError::Inject(e)
    }
}

impl From<AddressError> for TaskError {
    fn from(e: AddressError) -> Self {
        TaskError::Address(e)
    }
}

/// What one trial did, of a task of `N` readouts, two unless the type names another number.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Outcome<const N: usize = 2> {
    /// The trial's index.
    pub trial: u64,
    /// The stimulus presented, 0 or 1.
    pub stimulus: u8,
    /// The spikes in each readout set within the task's window of the trial.
    pub counts: [u32; N],
    /// The readout selected: the gate's, or none where the largest count was shared; where the
    /// selection was drawn (ADR-0163), the trial's drawn channel whatever the counts.
    pub selection: Option<u8>,
    /// Whether the selection was the stimulus's rewarded readout, whatever reward the trial
    /// then received (ADR-0148).
    pub correct: bool,
    /// The reward delivered, signed; zero when withheld. Under a critic, the prediction error
    /// the modulator received (ADR-0107); under the engine's critic with the whole punishment
    /// set (ADR-0155), what the modulator received, which is the reward itself where it and
    /// the value were both below zero.
    pub reward_q16: i32,
    /// The modulator's dopamine signal after the reward.
    pub signal_q16: i32,
    /// Under a critic, the presented stimulus's expected reward before the trial and after it,
    /// `[before, after]`, the same when the reward is withheld; none without a critic.
    pub expected_q16: Option<[i32; 2]>,
    /// Under the engine's critic (ADR-0131), the engine's value of the trial's reward, before
    /// its weights moved; none without it, or when the reward is withheld.
    pub value_q16: Option<i32>,
    /// The messages the hold delivered in the trial (ADR-0144); zero without a hold, and at a
    /// tie.
    pub held: u32,
    /// Whether the selection was drawn (ADR-0163): the exploration's coin drew at the engine's
    /// value, so the selection is the trial's drawn channel and not the gate's reading, which
    /// the readout's channels still hold. False with the exploration unset.
    pub drawn: bool,
}

/// A task on an executor: two stimuli, a readout of `N` channels — two unless the type names
/// another number (ADR-0151, ADR-0152) — a background drive, a trial's length, a seed, a
/// reward magnitude, the readout each stimulus is rewarded at, where the reward's sign comes
/// from, where its dopamine term reaches, the critic, when it has one, the hold, when it has
/// one, and whether its selection may be drawn. Every field is the caller's; `check` says what
/// a run needs of them and of the executor.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Task<const N: usize = 2> {
    pub stimuli: [Stimulus; 2],
    pub readout: Readout<N>,
    /// The background drive, run on every tick of every trial; a drive whose `every` is zero
    /// is never due.
    pub drive: Drive,
    /// Ticks per trial, at least one.
    pub ticks: u32,
    /// The sub-window of the trial the readout counts, inside the trial and at least one tick
    /// long; [`Window::whole`] of `ticks` counts the whole trial.
    pub window: Window,
    /// The seed the trial's stimulus, the shuffled coin and the misleading coin are drawn from.
    pub seed: u64,
    /// The reward's magnitude, Q16.16, at least zero; the sign is the outcome's.
    pub reward_q16: i32,
    /// The mapping (ADR-0152): stimulus `s` is rewarded at readout `answers[s]`, an index
    /// below the number of channels. With two readouts `[0, 1]` is the assignment and `[1, 0]`
    /// the mirrored one, the two values of the flag this field replaced.
    pub answers: [u8; 2],
    pub feedback: Feedback,
    /// Where the dopamine term reaches (ADR-0068).
    pub delivery: Delivery,
    /// The critic (ADR-0107): the reward delivered is the outcome's less the presented
    /// stimulus's expected reward; none delivers the outcome's reward itself.
    pub critic: Option<Critic>,
    /// The gate's output delivered (ADR-0144): the selection made at the readout window's
    /// close and the channel not selected held from there; none selects at the trial's end
    /// and delivers nothing, the trial before ADR-0144.
    pub hold: Option<Hold>,
    /// Whether the selection may be drawn (ADR-0163): unset, it is the gate's at every trial,
    /// the trial before ADR-0163.
    pub exploration: Exploration,
}

impl<const N: usize> Task<N> {
    /// The readout stimulus `stimulus`, 0 or 1, is rewarded at.
    pub const fn answer(&self, stimulus: u8) -> u8 {
        self.answers[stimulus as usize]
    }

    /// The mapping moved on (ADR-0151, ADR-0152): every stimulus's answer to the next readout,
    /// the last to the first. With two readouts each answer moves to the other, the flag
    /// negated. An answer `check` would refuse moves to the first readout.
    pub fn flip(&mut self) {
        for answer in &mut self.answers {
            let next = answer.wrapping_add(1);
            *answer = if usize::from(next) < N { next } else { 0 };
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

    /// Whether the reward of trial `trial` is misleading under [`Feedback::SevenInEight`]
    /// (ADR-0148): the three [`MISLEADING_BITS`] of the same draw all zero, one trial in
    /// eight, a coin neither the stimulus's bit nor the shuffled control's reads.
    pub const fn misleading_at(&self, trial: u64) -> bool {
        mix64(self.seed ^ trial) & MISLEADING_BITS == 0
    }

    /// The exploration's coin for trial `trial` (ADR-0163): the sixteen
    /// [`EXPLORATION_COIN_BITS`] of the same draw as a number, which [`Exploration::draws`]
    /// reads against the engine's value.
    pub const fn exploration_coin_at(&self, trial: u64) -> u16 {
        ((mix64(self.seed ^ trial) & EXPLORATION_COIN_BITS) >> 16) as u16
    }

    /// The channel a drawn selection of trial `trial` is (ADR-0163): the fifteen
    /// [`EXPLORATION_CHANNEL_BITS`] of the same draw dealt among the task's channels by
    /// [`Exploration::channel`], each with the same chance to within one draw of the 32 768.
    pub fn drawn_at(&self, trial: u64) -> u8 {
        let draw = ((mix64(self.seed ^ trial) & EXPLORATION_CHANNEL_BITS) >> 33) as u16;
        Exploration::channel(draw, N)
    }

    /// What a run needs: every set well-formed, non-empty and inside `exec`'s arena, no two
    /// sharing a unit, a stimulus with a message, an answer for each stimulus that names a
    /// readout among the task's, a trial with a tick, a cancel (where a
    /// stimulus carries one) with a message and a tick, after the first injection, inside the
    /// trial and negative, a readout window with a tick and inside the trial, a train that
    /// holds the most spikes a trial can produce, a readout count that cannot reach the
    /// drive's width, a reward that is delivered only where it can do something, and a
    /// critic's expectations, where the task has one, within the reward's magnitude, and no
    /// critic of the task's on an engine that carries its own; a hold, where the task carries
    /// one, with a message and a cadence, negative, due at the window's close and landing
    /// inside the trial; under the drawn delivery or the released one an engine that draws;
    /// and with the exploration set an engine that carries the critic, no hold and a reward
    /// whose magnitude is above zero.
    pub fn check<const CAP: usize>(&self, exec: &Executor<CAP>) -> Result<(), TaskError> {
        let units = exec.units().len() as u64;
        // The stimuli's sets, then the readout's, one a channel.
        let sets = || {
            self.stimuli
                .iter()
                .map(|stimulus| &stimulus.set)
                .chain(self.readout.sets.iter())
        };
        for set in sets() {
            if !set.is_well_formed() {
                return Err(TaskError::MalformedSet);
            }
            if set.is_empty() {
                return Err(TaskError::EmptySet);
            }
            if set.end() > units {
                return Err(TaskError::SetOutsideArena);
            }
        }
        for (i, a) in sets().enumerate() {
            for b in sets().skip(i.saturating_add(1)) {
                if a.overlaps(b) {
                    return Err(TaskError::SetsOverlap);
                }
            }
        }
        if self.stimuli.iter().any(|s| s.messages == 0) {
            return Err(TaskError::NoStimulus);
        }
        // An answer names a readout among the task's (ADR-0152).
        if self.answers.iter().any(|&answer| usize::from(answer) >= N) {
            return Err(TaskError::AnswerOutsideReadout);
        }
        if self.ticks == 0 {
            return Err(TaskError::NoTicks);
        }
        for cancel in self.stimuli.iter().filter_map(|s| s.cancel) {
            if cancel.messages == 0 || cancel.ticks == 0 {
                return Err(TaskError::EmptyCancel);
            }
            if cancel.offset == 0 {
                return Err(TaskError::CancelAtInjection);
            }
            // The last message is injected before tick `end - 1` and lands on tick `end`,
            // which must be inside the trial.
            if cancel.end() >= u64::from(self.ticks) {
                return Err(TaskError::CancelOutsideTrial);
            }
            if cancel.efficacy_q16 >= 0 {
                return Err(TaskError::CancelNotNegative);
            }
        }
        if self.window.ticks == 0 {
            return Err(TaskError::EmptyWindow);
        }
        if self.window.end() > u64::from(self.ticks) {
            return Err(TaskError::WindowOutsideTrial);
        }
        if let Some(hold) = self.hold {
            if hold.messages == 0 || hold.every == 0 {
                return Err(TaskError::EmptyHold);
            }
            if hold.efficacy_q16 >= 0 {
                return Err(TaskError::HoldNotNegative);
            }
            // The first message is due before the tick the window closes at.
            if u64::from(hold.until) <= self.window.end() {
                return Err(TaskError::HoldBeforeClose);
            }
            // The last message is injected before tick `until - 1` at the latest and lands on
            // tick `until`, which must be inside the trial.
            if hold.until >= self.ticks {
                return Err(TaskError::HoldOutsideTrial);
            }
        }
        let per_unit = u64::from(spikes_per_unit(self.ticks));
        if (exec.train_capacity() as u64) < units.saturating_mul(per_unit) {
            return Err(TaskError::TrainTooSmall);
        }
        // The count is the window's, so its bound is one spike per refractory interval of
        // the window per unit; for the whole trial it is the trial's.
        let per_window = u64::from(spikes_per_unit(self.window.ticks));
        if self
            .readout
            .sets
            .iter()
            .any(|set| set.len().saturating_mul(per_window) > i16::MAX as u64)
        {
            return Err(TaskError::CountBeyondWidth);
        }
        if self.reward_q16 < 0 {
            return Err(TaskError::NegativeReward);
        }
        // The magnitude is at least zero here, so its bound is its own value.
        let bound = self.reward_q16.unsigned_abs();
        if self
            .critic
            .is_some_and(|c| c.expected_q16.iter().any(|v| v.unsigned_abs() > bound))
        {
            return Err(TaskError::ExpectationBeyondReward);
        }
        if self.critic.is_some() && exec.critic().is_some() {
            return Err(TaskError::TwoCritics);
        }
        // The drawn delivery and the released one need the counts they draw from (ADR-0139,
        // ADR-0144), refused before any tick.
        if matches!(self.delivery, Delivery::Drawn | Delivery::Released) {
            exec.draws()?;
        }
        // The exploration reads the engine's own value at the trial's end against the reward's
        // magnitude (ADR-0163), refused before any tick where either is missing or a hold
        // would act on the gate's reading before it.
        if self.exploration == Exploration::ValueGated {
            if exec.critic().is_none() {
                return Err(TaskError::ExplorationWithoutCritic);
            }
            if self.hold.is_some() {
                return Err(TaskError::ExplorationWithHold);
            }
            if self.reward_q16 == 0 {
                return Err(TaskError::ExplorationWithoutReward);
            }
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

    /// One trial: the stimulus injected, `ticks` ticks under the drive with the stimulus's
    /// cancel, if it carries one, injected before each tick it is due at, the train read once
    /// over the task's window of the trial, the selection, and the reward delivered between
    /// ticks, its sign by `feedback` — under a critic, the reward less the presented
    /// stimulus's expectation, which then moves (ADR-0107). Under a hold (ADR-0144) the train
    /// is read and the selection made before the tick the window closes at, the first the
    /// hold is due at, and the hold's messages go into the channel the gate holds before each
    /// tick it is due at; the window is whole by then, so the counts and the selection are
    /// the ones the trial's end would read. With the exploration set (ADR-0163) the engine's
    /// value is read at the trial's end, before the reward, and where the trial's coin draws
    /// at it the selection is the trial's drawn channel in the gate's place; what is correct,
    /// what is addressed and the reward's sign are then that selection's. Refused as `check`
    /// refuses, and when the injector refuses a message.
    pub fn trial<const CAP: usize>(
        &mut self,
        exec: &mut Executor<CAP>,
        trial: u64,
    ) -> Result<Outcome<N>, TaskError> {
        self.check(exec)?;
        let start = exec.ticks();
        let stimulus = self.stimulus_at(trial);
        let inject = exec.injector();
        let presented = self.stimuli[usize::from(stimulus)];
        presented.inject(&inject)?;
        // The train's stamp is the tick's low word (§8.4), as `start` is read here; the
        // window's first tick is inside the trial, which `check` held, and so is its close.
        let opens = (start as u32).wrapping_add(self.window.from);
        let close = self.window.from.wrapping_add(self.window.ticks);
        let mut decided: Option<([u32; N], Option<u8>)> = None;
        let mut held = 0u32;
        for k in 0..self.ticks {
            presented.cancel_at(&inject, k)?;
            if let Some(hold) = self.hold.filter(|hold| hold.is_due(close, k)) {
                // The selection, made once, before the first tick the hold is due at: the
                // tick the window closes at, every tick of the window run and in the train.
                decided.get_or_insert_with(|| {
                    let counts = self
                        .readout
                        .count_window(exec.train(), opens, self.window.ticks);
                    (counts, self.readout.select(counts))
                });
                held = held.saturating_add(self.readout.hold(&inject, &hold)?);
            }
            self.drive.step(&inject, exec.ticks())?;
            exec.tick();
        }
        // Without a hold the train is read and the selection made here, at the trial's end.
        let (counts, gated) = match decided {
            Some(read) => read,
            None => {
                let counts = self
                    .readout
                    .count_window(exec.train(), opens, self.window.ticks);
                (counts, self.readout.select(counts))
            }
        };
        // The exploration (ADR-0162, ADR-0163): the engine's value read here, between the
        // trial's last tick and its reward, is the one the reward is taken against; where the
        // trial's coin draws at it the selection is the trial's drawn channel, whatever the
        // gate read, and the gate's everywhere else.
        let drawn = match self.exploration {
            Exploration::Unset => false,
            Exploration::ValueGated => engine_value(exec).is_some_and(|value_q16| {
                Exploration::draws(self.exploration_coin_at(trial), value_q16, self.reward_q16)
            }),
        };
        let selection = if drawn {
            Some(self.drawn_at(trial))
        } else {
            gated
        };
        let correct = selection == Some(self.answer(stimulus));
        // Where the dopamine term reaches from the next tick (ADR-0068): under the addressed
        // delivery the synapses from the stimulus presented onto the readout the engine
        // selected, none at a tie; under the drawn one (ADR-0139) from the units the engine's
        // critic counted in its window onto the same readout, the counts this trial's since
        // the reward comes after; under the released one (ADR-0144) from those units onto
        // every unit, at a tie too; under the global one every synapse alike. Written
        // whatever the feedback, so that the set is the outcome's and not the reward's.
        match self.delivery {
            Delivery::Global => exec.address_all(),
            Delivery::Addressed => {
                let sources = self.stimuli[usize::from(stimulus)].set.units();
                match selection {
                    Some(readout) => {
                        exec.address(sources, self.readout.sets[usize::from(readout)].units())?
                    }
                    None => exec.address(sources, core::iter::empty())?,
                }
            }
            Delivery::Drawn => match selection {
                Some(readout) => {
                    exec.address_drawn(self.readout.sets[usize::from(readout)].units())?
                }
                None => exec.address_drawn(core::iter::empty())?,
            },
            Delivery::Released => {
                let units = exec.units().len() as u32;
                exec.address_drawn(0..units)?
            }
        }
        let positive = match self.feedback {
            Feedback::Answer => correct,
            Feedback::Shuffled => self.coin_at(trial),
            // The outcome's sign unless the coin is misleading, whatever the outcome
            // (ADR-0148); `correct` above stays the selection's.
            Feedback::SevenInEight => correct != self.misleading_at(trial),
            Feedback::Withheld => {
                return Ok(Outcome {
                    trial,
                    stimulus,
                    counts,
                    selection,
                    correct,
                    reward_q16: 0,
                    signal_q16: exec.modulator().dopamine_rpe,
                    expected_q16: self
                        .critic
                        .map(|c| [c.expected_q16[usize::from(stimulus)]; 2]),
                    value_q16: None,
                    held,
                    drawn,
                });
            }
        };
        let outcome_q16 = if positive {
            self.reward_q16
        } else {
            self.reward_q16.saturating_neg()
        };
        // The critic takes the stimulus's expectation from the outcome's reward and moves it
        // (ADR-0107); without one the outcome's reward is delivered itself.
        let (delivered_q16, expected_q16) = match self.critic.as_mut() {
            Some(critic) => {
                let (error, before, after) = critic.predict(stimulus, outcome_q16);
                (error, Some([before, after]))
            }
            None => (outcome_q16, None),
        };
        let signal_q16 = exec.reward(delivered_q16);
        // Under the engine's critic (ADR-0131) the modulator received the reward less the
        // engine's value, or the reward whole where a punishment met a value below zero while
        // that is set (ADR-0155), which the executor read at this reward; unset, it reads none.
        let (reward_q16, value_q16) = match exec.prediction() {
            Some(prediction) => (prediction.received_q16, Some(prediction.value_q16)),
            None => (delivered_q16, None),
        };
        Ok(Outcome {
            trial,
            stimulus,
            counts,
            selection,
            correct,
            reward_q16,
            signal_q16,
            expected_q16,
            value_q16,
            held,
            drawn,
        })
    }
}

/// The engine's value between ticks (ADR-0131), as its next reward will read it: its critic's
/// value of each unit's weight and its count since the previous reward, in unit order, the
/// composition of `Executor::reward`. None on an engine without the critic.
fn engine_value<const CAP: usize>(exec: &Executor<CAP>) -> Option<i32> {
    let critic = exec.critic()?;
    Some(
        critic.value_q16(
            exec.units()
                .iter()
                .zip(exec.features())
                .map(|(unit, &count)| (unit.value_weight, count)),
        ),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::executor::{Config, Prediction};
    use cortex_core::{STP_MAX, STP_U, THRESHOLD_BASE};
    use cortex_neuromod::{DOPAMINE_TAU_SHIFT, NeuromodulatorState, ValueCritic};

    const ONE: i32 = MODULATION_ONE_Q16;
    /// The replay drive's message (ADR-0038): two of 1.25 fire an armed unit once.
    const CUE_Q16: i32 = 0x0001_4000;
    const TICKS: u32 = 64;
    const REWARD: i32 = 0x4000;

    fn set(first: u32, len: u32) -> Set {
        Set::contiguous(first, len)
    }

    /// A lattice of `len` units, one every `stride` from `first`.
    fn lattice(first: u32, len: u32, stride: u32) -> Set {
        Set {
            first,
            period: stride,
            mask: 1,
            count: len,
        }
    }

    fn stimulus(first: u32) -> Stimulus {
        Stimulus {
            set: set(first, 4),
            messages: 2,
            efficacy_q16: CUE_Q16,
            cancel: None,
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
            window: Window::whole(TICKS),
            seed: 0,
            reward_q16: REWARD,
            answers: [0, 1],
            feedback,
            delivery: Delivery::Global,
            critic: None,
            hold: None,
            exploration: Exploration::Unset,
        }
    }

    /// Cues every unit of `set` through the injector, as a stimulus would.
    fn cue(exec: &Executor<8>, set: Set) {
        let inject = exec.injector();
        for unit in set.units() {
            for _ in 0..2 {
                inject.inject(unit, spike_message(CUE_Q16, false)).unwrap();
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
        assert_eq!(s.len(), 4);
        assert!(!s.is_empty() && set(5, 0).is_empty());
        assert_eq!(s.units().collect::<Vec<u32>>(), [4, 5, 6, 7]);
        assert_eq!(set(5, 0).units().count(), 0);
        assert_eq!(set(5, 0).end(), 5, "an empty set ends where it starts");
    }

    /// The periodic shape (ADR-0065): a lattice names one unit every stride, a mask names
    /// several offsets of one period, and two masks over one period interleave without
    /// sharing a unit.
    #[test]
    fn a_periodic_set_names_exactly_the_units_of_its_pattern() {
        let a = lattice(0, 3, 20);
        assert_eq!(a.units().collect::<Vec<u32>>(), [0, 20, 40]);
        assert_eq!((a.len(), a.per_period(), a.end()), (3, 1, 41));
        assert!(a.contains(0) && a.contains(20) && a.contains(40));
        assert!(!a.contains(1) && !a.contains(19) && !a.contains(21) && !a.contains(60));
        let b = lattice(11, 3, 20);
        assert_eq!(b.units().collect::<Vec<u32>>(), [11, 31, 51]);
        assert!(!a.overlaps(&b) && !b.overlaps(&a));
        // Odd offsets but 11, and even offsets but 0, over the same period: the two
        // readouts of brief 029's geometry, disjoint from the lattices and from each other.
        let odd = Set {
            first: 0,
            period: 20,
            mask: 0xAA2AA,
            count: 3,
        };
        let even = Set {
            first: 0,
            period: 20,
            mask: 0x55554,
            count: 3,
        };
        assert_eq!(
            odd.units().take(10).collect::<Vec<u32>>(),
            [1, 3, 5, 7, 9, 13, 15, 17, 19, 21]
        );
        assert_eq!(
            even.units().take(10).collect::<Vec<u32>>(),
            [2, 4, 6, 8, 10, 12, 14, 16, 18, 22]
        );
        assert_eq!((odd.len(), even.len()), (27, 27));
        assert_eq!((odd.per_period(), even.per_period()), (9, 9));
        assert_eq!((odd.end(), even.end()), (60, 59));
        assert!(odd.contains(59) && !odd.contains(60) && !odd.contains(51));
        assert!(even.contains(58) && !even.contains(59) && !even.contains(40));
        for (x, y) in [
            (&a, &odd),
            (&a, &even),
            (&b, &odd),
            (&b, &even),
            (&odd, &even),
        ] {
            assert!(!x.overlaps(y) && !y.overlaps(x), "{x:?} {y:?}");
        }
        assert!(odd.overlaps(&set(0, 2)), "a run over unit 1");
        assert!(!odd.overlaps(&set(0, 1)), "a run of unit 0 alone");
        assert!(a.overlaps(&lattice(20, 1, 7)), "one unit in common");
        assert!(!a.overlaps(&lattice(21, 4, 20)));
        assert!(
            lattice(0, 4, 6).overlaps(&lattice(3, 4, 9)),
            "12 is on both lattices"
        );
        assert!(
            !lattice(0, 2, 6).overlaps(&lattice(3, 2, 9)),
            "one period short of 12"
        );
        // The rule's bounds.
        assert!(a.is_well_formed() && odd.is_well_formed() && set(5, 0).is_well_formed());
        let malformed = [
            Set { period: 0, ..a },
            Set { period: 33, ..a },
            Set { mask: 0, ..a },
            Set {
                period: 2,
                mask: 0b100,
                ..a
            },
            Set {
                period: 20,
                mask: 1 << 20,
                ..a
            },
        ];
        for s in malformed {
            assert!(!s.is_well_formed(), "{s:?}");
            assert_eq!(s.units().count(), 0, "a refused set is not walked: {s:?}");
        }
        let widest = Set {
            first: 0,
            period: 32,
            mask: u32::MAX,
            count: 2,
        };
        assert!(widest.is_well_formed(), "every offset of a period of 32");
        assert_eq!((widest.len(), widest.end()), (64, 64));
        assert_eq!(widest.units().count(), 64);
        assert!(
            Set {
                period: 2,
                mask: 0b11,
                ..a
            }
            .is_well_formed()
        );
        assert!(
            !Set { period: 0, ..a }.contains(0),
            "no period names nothing"
        );
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
        t.stimuli[0].set = Set {
            period: 0,
            ..t.stimuli[0].set
        };
        assert_eq!(t.check(&exec), Err(TaskError::MalformedSet), "no period");
        let mut t = ok;
        t.readout = Readout::new([
            Set {
                period: 2,
                mask: 0b100,
                ..set(4, 2)
            },
            set(12, 4),
        ]);
        assert_eq!(
            t.check(&exec),
            Err(TaskError::MalformedSet),
            "a mask bit at the period would name a unit of the next period twice"
        );
        let mut t = ok;
        t.readout = Readout::new([
            Set {
                mask: 0,
                ..set(4, 4)
            },
            set(12, 4),
        ]);
        assert_eq!(t.check(&exec), Err(TaskError::MalformedSet), "no bit");
        let mut t = ok;
        t.window = Window { from: 0, ticks: 0 };
        assert_eq!(t.check(&exec), Err(TaskError::EmptyWindow));
        let mut t = ok;
        t.window = Window {
            from: TICKS - 8,
            ticks: 8,
        };
        assert_eq!(t.check(&exec), Ok(()), "a window that ends with the trial");
        t.window = Window {
            from: TICKS - 8,
            ticks: 9,
        };
        assert_eq!(t.check(&exec), Err(TaskError::WindowOutsideTrial));
        t.window = Window {
            from: TICKS,
            ticks: 1,
        };
        assert_eq!(t.check(&exec), Err(TaskError::WindowOutsideTrial));
        t.window = Window {
            from: u32::MAX,
            ticks: u32::MAX,
        };
        assert_eq!(
            t.check(&exec),
            Err(TaskError::WindowOutsideTrial),
            "the end is widened, not wrapped"
        );
        let mut t = ok;
        t.ticks = 2 * MIN_INTERVAL_TICKS;
        t.window = Window::whole(t.ticks);
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
        t.window = Window::whole(1);
        assert_eq!(
            t.check(&wide),
            Err(TaskError::CountBeyondWidth),
            "32 768 units at one spike each reach the width"
        );
        t.readout = Readout::new([set(4, 4), set(16, 32_767)]);
        assert_eq!(t.check(&wide), Ok(()), "one fewer fits");
        // A lattice's count is its units', not its span's: 10 923 units at three spikes
        // each reach the width and 10 922 do not.
        t.ticks = 2 * MIN_INTERVAL_TICKS + 1;
        t.window = Window::whole(t.ticks);
        t.readout = Readout::new([set(4, 4), lattice(16, 10_923, 4)]);
        assert_eq!(t.check(&wide), Err(TaskError::CountBeyondWidth));
        t.readout = Readout::new([set(4, 4), lattice(16, 10_922, 4)]);
        assert_eq!(t.check(&wide), Ok(()));
        t.readout = Readout::new([set(4, 4), set(16, 32_767)]);
        t.ticks = 51;
        t.window = Window::whole(51);
        assert_eq!(
            t.check(&wide),
            Err(TaskError::CountBeyondWidth),
            "two spikes per unit double the bound"
        );
        t.window = Window { from: 1, ticks: 50 };
        assert_eq!(
            t.check(&wide),
            Ok(()),
            "the bound is the window's: one refractory interval, one spike per unit"
        );
        t.window = Window { from: 0, ticks: 51 };
        assert_eq!(t.check(&wide), Err(TaskError::CountBeyondWidth));
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
        // The cancel's refusals (ADR-0076), at their edges.
        let cancel = Cancel {
            offset: 1,
            ticks: 1,
            messages: 1,
            efficacy_q16: -1,
        };
        let with = |c: Cancel| {
            let mut t = ok;
            t.stimuli[0].cancel = Some(c);
            t
        };
        assert_eq!(
            with(cancel).check(&exec),
            Ok(()),
            "the earliest cancel: one tick after the injection's"
        );
        assert_eq!(
            with(Cancel {
                offset: 0,
                ..cancel
            })
            .check(&exec),
            Err(TaskError::CancelAtInjection)
        );
        assert_eq!(
            with(Cancel {
                offset: TICKS - 2,
                ..cancel
            })
            .check(&exec),
            Ok(()),
            "lands on the trial's last tick"
        );
        assert_eq!(
            with(Cancel {
                offset: TICKS - 1,
                ..cancel
            })
            .check(&exec),
            Err(TaskError::CancelOutsideTrial),
            "would land after the trial's last tick"
        );
        assert_eq!(
            with(Cancel {
                offset: TICKS - 3,
                ticks: 2,
                ..cancel
            })
            .check(&exec),
            Ok(()),
            "two ticks, the last landing on the trial's last"
        );
        assert_eq!(
            with(Cancel {
                offset: TICKS - 3,
                ticks: 3,
                ..cancel
            })
            .check(&exec),
            Err(TaskError::CancelOutsideTrial)
        );
        assert_eq!(
            with(Cancel {
                offset: u32::MAX,
                ticks: u32::MAX,
                ..cancel
            })
            .check(&exec),
            Err(TaskError::CancelOutsideTrial),
            "the end is widened, not wrapped"
        );
        assert_eq!(
            with(Cancel {
                efficacy_q16: 0,
                ..cancel
            })
            .check(&exec),
            Err(TaskError::CancelNotNegative),
            "zero is not negative"
        );
        assert_eq!(
            with(Cancel {
                efficacy_q16: 1,
                ..cancel
            })
            .check(&exec),
            Err(TaskError::CancelNotNegative)
        );
        assert_eq!(
            with(Cancel {
                efficacy_q16: i32::MIN,
                ..cancel
            })
            .check(&exec),
            Ok(()),
            "the most negative is a cancel; the message clamps it"
        );
        assert_eq!(
            with(Cancel {
                messages: 0,
                ..cancel
            })
            .check(&exec),
            Err(TaskError::EmptyCancel)
        );
        assert_eq!(
            with(Cancel { ticks: 0, ..cancel }).check(&exec),
            Err(TaskError::EmptyCancel)
        );
        let mut t = ok;
        t.stimuli[1].cancel = Some(Cancel {
            offset: 0,
            ..cancel
        });
        assert_eq!(
            t.check(&exec),
            Err(TaskError::CancelAtInjection),
            "the other stimulus's cancel is checked too"
        );
        let mut exec = network(1, ONE / 2);
        let mut t = with(Cancel {
            efficacy_q16: 0,
            ..cancel
        });
        assert_eq!(t.trial(&mut exec, 0), Err(TaskError::CancelNotNegative));
        assert!(exec.is_quiescent(), "a refused cancel injects nothing");
        // The critic's refusal (ADR-0107), at its edges: an expectation of either stimulus
        // beyond the reward's magnitude, whatever the shift.
        let critic = |expected_q16: [i32; 2]| Task {
            critic: Some(Critic {
                expected_q16,
                shift: 5,
            }),
            ..ok
        };
        assert_eq!(
            critic([REWARD, -REWARD]).check(&exec),
            Ok(()),
            "at the bound, either sign"
        );
        assert_eq!(
            critic([REWARD + 1, 0]).check(&exec),
            Err(TaskError::ExpectationBeyondReward)
        );
        assert_eq!(
            critic([0, -REWARD - 1]).check(&exec),
            Err(TaskError::ExpectationBeyondReward),
            "the second stimulus's too"
        );
        assert_eq!(
            critic([i32::MIN, 0]).check(&exec),
            Err(TaskError::ExpectationBeyondReward),
            "the most negative, whose magnitude is past the width"
        );
        assert_eq!(
            Task {
                critic: Some(Critic::new(u32::MAX)),
                ..ok
            }
            .check(&exec),
            Ok(()),
            "any shift runs"
        );
        let withheld = Task {
            reward_q16: 0,
            feedback: Feedback::Withheld,
            ..ok
        };
        assert_eq!(
            Task {
                critic: Some(Critic::new(5)),
                ..withheld
            }
            .check(&exec),
            Ok(()),
            "a magnitude of zero bounds the expectations at zero"
        );
        assert_eq!(
            Task {
                critic: Some(Critic {
                    expected_q16: [0, 1],
                    shift: 5
                }),
                ..withheld
            }
            .check(&exec),
            Err(TaskError::ExpectationBeyondReward)
        );
        assert_eq!(
            Task {
                reward_q16: -1,
                ..critic([0, 0])
            }
            .check(&exec),
            Err(TaskError::NegativeReward),
            "the magnitude is refused first"
        );
        let mut t = critic([0, REWARD + 1]);
        assert_eq!(
            t.trial(&mut exec, 0),
            Err(TaskError::ExpectationBeyondReward)
        );
        assert!(exec.is_quiescent(), "a refused critic injects nothing");
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
        mirrored.answers = [1, 0];
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
        // The window's count (ADR-0065) passes over the entries after the window and stops
        // at the first before it: the same train, read over its sub-windows.
        assert_eq!(
            readout.count_window(&train, start, 8),
            [3, 3],
            "the whole trial: the trial's count"
        );
        assert_eq!(
            readout.count_window(&train, start, 7),
            [3, 2],
            "the last tick left out, the entry there passed over"
        );
        assert_eq!(
            readout.count_window(&train, start + 3, 2),
            [1, 1],
            "ticks 3 and 4 after the start: one entry each side of the wrap"
        );
        assert_eq!(
            readout.count_window(&train, 3, 1),
            [0, 1],
            "the last tick alone: set 1's entry at unit 13"
        );
        assert_eq!(
            readout.count_window(&train, start - 1, 1),
            [1, 1],
            "the tick before"
        );
        assert_eq!(
            readout.count_window(&train, start - 9, 8),
            [0, 0],
            "a window before every entry"
        );
        assert_eq!(
            readout.count_window(&train, 4, 8),
            [0, 0],
            "a window after every entry"
        );
        assert_eq!(readout.count_window(&[], start, 8), [0, 0]);
        assert_eq!(
            readout.count_window(&train, start, 0),
            [0, 0],
            "a window of no ticks counts nothing"
        );
    }

    /// A trial whose window is a sub-window of the trial counts the spikes in it and no
    /// other: a readout cued before the trial fires within the first ticks, inside a window
    /// that opens at the trial's first tick and outside one that opens later.
    #[test]
    fn a_trial_counts_its_window_of_the_trial_alone() {
        let mut exec = network(1, ONE / 2);
        let mut t = task(Feedback::Answer);
        t.window = Window {
            from: 0,
            ticks: TICKS / 2,
        };
        cue(&exec, t.readout.sets()[0]);
        let early = t.trial(&mut exec, 1).unwrap();
        assert_eq!(
            early.counts,
            [4, 0],
            "the cued readout fires in the first half"
        );
        assert_eq!(early.selection, Some(0));
        exec.run(400);
        t.window = Window {
            from: TICKS / 2,
            ticks: TICKS / 2,
        };
        cue(&exec, t.readout.sets()[0]);
        let late = t.trial(&mut exec, 1).unwrap();
        assert_eq!(
            late.counts,
            [0, 0],
            "the same spikes fall before a window that opens at the trial's midpoint"
        );
        assert_eq!(late.selection, None);
        assert!(!late.correct, "a tie is an error");
        // The whole trial reads them, as ADR-0059's readout did.
        exec.run(400);
        t.window = Window::whole(TICKS);
        cue(&exec, t.readout.sets()[0]);
        let whole = t.trial(&mut exec, 1).unwrap();
        assert_eq!(whole.counts, [4, 0]);
    }

    /// A stimulus over a lattice injects into the lattice's units and no other: the train
    /// of a trial holds exactly those units' spikes.
    #[test]
    fn a_stimulus_over_a_lattice_fires_the_lattice_s_units() {
        let mut exec = network(1, ONE / 2);
        let mut t = task(Feedback::Answer);
        // Stimulus 1 on every fourth unit from 8 (8 and 12), readout 1 on 9, 13, 14 and 15.
        t.stimuli[1].set = lattice(8, 2, 4);
        t.readout = Readout::new([
            set(4, 4),
            Set {
                first: 8,
                period: 4,
                mask: 0b1110,
                count: 2,
            },
        ]);
        assert_eq!(t.check(&exec), Ok(()));
        let trial = (0..8u64).find(|&k| t.stimulus_at(k) == 1).unwrap();
        let outcome = t.trial(&mut exec, trial).unwrap();
        assert_eq!(outcome.stimulus, 1);
        assert_eq!(
            outcome.counts,
            [0, 0],
            "no synapses: the readouts hold nothing"
        );
        let fired: Vec<u32> = exec.train().iter().map(|&(_, u)| u).collect();
        assert_eq!(fired, [8, 12], "the lattice's units, once each");
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
        assert_eq!(
            (first.expected_q16, heads.expected_q16, none.expected_q16),
            (None, None, None),
            "no critic, no expectation"
        );
    }

    /// The critic's rule at its edges (ADR-0107): the error against an expectation at either
    /// end of the bound, twice the reward from the far end, the arithmetic shift's floor on
    /// either side of zero, saturation at the width, and a shift of zero, at the width and past
    /// it; the other stimulus's expectation never moves.
    #[test]
    fn the_critic_s_error_and_update_at_their_edges() {
        const R: i32 = ONE;
        let at = |expected: i32| Critic {
            expected_q16: [expected, 7],
            shift: 5,
        };
        assert_eq!(
            Critic::new(5),
            Critic {
                expected_q16: [0; 2],
                shift: 5
            }
        );
        let mut c = at(R);
        assert_eq!(c.predict(0, R), (0, R, R), "a reward expected is no error");
        let mut c = at(-R);
        assert_eq!(
            c.predict(0, R),
            (2 * R, -R, -R + R / 16),
            "from −r, an error of 2r moves the expectation up by a sixteenth of r"
        );
        let mut c = at(R);
        assert_eq!(c.predict(0, -R), (-2 * R, R, R - R / 16));
        assert_eq!(
            c.expected_q16,
            [R - R / 16, 7],
            "the other stimulus's stays"
        );
        let mut c = at(-R);
        assert_eq!(c.predict(0, -R), (0, -R, -R));
        let mut c = Critic {
            expected_q16: [7, -R],
            shift: 5,
        };
        assert_eq!(
            c.predict(1, R),
            (2 * R, -R, -R + R / 16),
            "stimulus 1's own"
        );
        assert_eq!(c.expected_q16[0], 7);
        // The step is the floor of the error over 32: below zero one LSB down from −1 to −32,
        // two from −33; above it nothing up to 31, one from 32.
        for (error, step) in [
            (-65, -3),
            (-64, -2),
            (-33, -2),
            (-32, -1),
            (-31, -1),
            (-1, -1),
            (0, 0),
            (1, 0),
            (31, 0),
            (32, 1),
            (63, 1),
            (64, 2),
        ] {
            let mut c = Critic::new(5);
            assert_eq!(c.predict(0, error), (error, 0, step), "{error}");
        }
        // Saturation: the error at the width, and the step within it.
        let mut c = Critic {
            expected_q16: [-i32::MAX, 0],
            shift: 5,
        };
        assert_eq!(
            c.predict(0, i32::MAX),
            (i32::MAX, -i32::MAX, -i32::MAX + (i32::MAX >> 5))
        );
        let mut c = Critic {
            expected_q16: [i32::MAX, 0],
            shift: 5,
        };
        assert_eq!(
            c.predict(0, -i32::MAX),
            (i32::MIN, i32::MAX, i32::MAX + (i32::MIN >> 5))
        );
        // A shift of zero takes the whole error; at the width and past it the floor is none
        // above zero and one LSB below it.
        let mut c = Critic::new(0);
        assert_eq!(c.predict(0, -R), (-R, 0, -R));
        let mut c = Critic::new(30);
        assert_eq!(c.predict(0, i32::MAX), (i32::MAX, 0, 1));
        for shift in [31, 32, 33, 64, u32::MAX] {
            let mut c = Critic::new(shift);
            assert_eq!(c.predict(1, R), (R, 0, 0), "{shift}");
            assert_eq!(c.predict(1, -R), (-R, 0, -1), "{shift}");
            assert_eq!(c.predict(1, i32::MIN), (i32::MIN + 1, -1, -2), "{shift}");
        }
    }

    /// The critic on a trial (ADR-0107): the reward delivered is the outcome's less the
    /// presented stimulus's expectation — a tie an error — the modulator receives it as the
    /// oracle does, and the expectation moves by the error shifted by five while the other
    /// stimulus's stays; at an expectation of zero the trial is the trial without a critic in
    /// every field but the expectation's; withheld, nothing moves and the outcome reads the
    /// presented stimulus's expectation twice; shuffled, the coin's reward less it.
    #[test]
    fn a_trial_under_the_critic_delivers_the_error_and_moves_the_expectation() {
        let mut exec = network(1, ONE / 2);
        let mut plain_exec = network(1, ONE / 2);
        let mut oracle = NeuromodulatorState::new();
        let mut t = Task {
            critic: Some(Critic::new(5)),
            ..task(Feedback::Answer)
        };
        let mut plain = task(Feedback::Answer);
        let s = t.stimulus_at(0);
        let same = (1..16u64).find(|&k| t.stimulus_at(k) == s).unwrap();
        let other = (1..16u64).find(|&k| t.stimulus_at(k) != s).unwrap();
        let mut expected = [0i32; 2];
        // Trial 0, no readout cued: a tie, an error, against an expectation of zero.
        let tie = t.trial(&mut exec, 0).unwrap();
        let bare = plain.trial(&mut plain_exec, 0).unwrap();
        assert_eq!(tie.selection, None);
        assert_eq!(
            (tie.reward_q16, tie.expected_q16),
            (-REWARD, Some([0, -REWARD / 32]))
        );
        assert_eq!(
            Outcome {
                expected_q16: None,
                ..tie
            },
            bare,
            "at an expectation of zero the critic delivers the outcome's reward"
        );
        assert_eq!(oracle.reward(-REWARD), tie.signal_q16);
        expected[usize::from(s)] = -REWARD / 32;
        assert_eq!(t.critic.map(|c| c.expected_q16), Some(expected));
        // The same stimulus, its answer cued: correct, the error r + r/32, and the
        // expectation up by the error's thirty-second rounded down, −512 + 528.
        exec.run(400 - TICKS as u64);
        for _ in 0..(400 - TICKS) {
            oracle.decay_dopamine(DOPAMINE_TAU_SHIFT);
        }
        cue(&exec, t.readout.sets()[usize::from(t.answer(s))]);
        let right = t.trial(&mut exec, same).unwrap();
        for _ in 0..TICKS {
            oracle.decay_dopamine(DOPAMINE_TAU_SHIFT);
        }
        assert!(right.correct);
        assert_eq!(
            (right.reward_q16, right.expected_q16),
            (REWARD + REWARD / 32, Some([-REWARD / 32, 16]))
        );
        assert_eq!(oracle.reward(right.reward_q16), right.signal_q16);
        assert_eq!(exec.modulator().dopamine_rpe, right.signal_q16);
        expected[usize::from(s)] = 16;
        assert_eq!(t.critic.map(|c| c.expected_q16), Some(expected));
        // The other stimulus, a tie: its own expectation, and the first's stays.
        exec.run(400 - TICKS as u64);
        for _ in 0..(400 - TICKS) {
            oracle.decay_dopamine(DOPAMINE_TAU_SHIFT);
        }
        let next = t.trial(&mut exec, other).unwrap();
        for _ in 0..TICKS {
            oracle.decay_dopamine(DOPAMINE_TAU_SHIFT);
        }
        assert_eq!(next.selection, None);
        assert_eq!(
            (next.reward_q16, next.expected_q16),
            (-REWARD, Some([0, -REWARD / 32]))
        );
        assert_eq!(oracle.reward(-REWARD), next.signal_q16);
        expected[usize::from(s ^ 1)] = -REWARD / 32;
        assert_eq!(t.critic.map(|c| c.expected_q16), Some(expected));
        // Withheld: no reward, each stimulus's expectation read twice and none moved.
        let mut exec = network(1, ONE);
        let held = Critic {
            expected_q16: [3, -5],
            shift: 5,
        };
        let mut w = Task {
            critic: Some(held),
            ..task(Feedback::Withheld)
        };
        for k in [0, other] {
            let v = held.expected_q16[usize::from(w.stimulus_at(k))];
            let none = w.trial(&mut exec, k).unwrap();
            assert_eq!(
                (none.reward_q16, none.expected_q16),
                (0, Some([v, v])),
                "trial {k}"
            );
        }
        assert_eq!(w.critic, Some(held));
        assert!(exec.modulator().is_at_rest());
        // Shuffled: the coin's reward less the expectation.
        let mut exec = network(1, ONE / 2);
        let mut sh = Task {
            critic: Some(Critic {
                expected_q16: [100, 100],
                shift: 5,
            }),
            ..task(Feedback::Shuffled)
        };
        let coin = sh.coin_at(0);
        let (error, after) = if coin {
            (REWARD - 100, 100 + ((REWARD - 100) >> 5))
        } else {
            (-REWARD - 100, 100 + ((-REWARD - 100) >> 5))
        };
        let flipped = sh.trial(&mut exec, 0).unwrap();
        assert_eq!(
            (flipped.reward_q16, flipped.expected_q16, flipped.signal_q16),
            (error, Some([100, after]), error)
        );
    }

    /// `network` with the engine's critic set (ADR-0131): a step of 2^-9 and a scale of 2^-2.
    fn valued(baseline_q16: i32) -> Executor<8> {
        valued_whole(baseline_q16, false)
    }

    /// `valued` with the whole punishment set or unset (ADR-0155).
    fn valued_whole(baseline_q16: i32, whole_punishment: bool) -> Executor<8> {
        let mut exec = Executor::<8>::new(Config {
            units: 16,
            injector_capacity: 64,
            train_capacity: spikes_per_unit(TICKS).saturating_mul(16) as usize,
            modulation_baseline_q16: baseline_q16,
            critic: Some(ValueCritic { shift: 9, scale: 2 }),
            whole_punishment,
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

    /// The engine's critic under a task (ADR-0131): a task that carries a critic of its own is
    /// refused on an engine that carries one, before anything is injected. A task without one
    /// delivers the outcome's reward and the engine takes its value from it: the trial records
    /// the error the modulator received and the value, the executor's reading. At weights of
    /// zero the value is zero and the trial is the trial on an engine without the critic in every
    /// field but the value; after it, every unit that fired carries a weight, so the next trial's
    /// value is those weights times the spikes since the reward, over four, and the error the
    /// reward less it. Withheld, there is no value and no error, and the spikes wait for the next
    /// reward.
    #[test]
    fn a_task_under_the_engine_s_critic_delivers_the_reward_and_records_the_value_and_the_error() {
        let mut exec = valued(ONE / 2);
        let mut own = Task {
            critic: Some(Critic::new(5)),
            ..task(Feedback::Answer)
        };
        assert_eq!(own.check(&exec), Err(TaskError::TwoCritics));
        assert_eq!(own.trial(&mut exec, 0), Err(TaskError::TwoCritics));
        assert!(
            exec.is_quiescent() && exec.ticks() == 0,
            "a refusal injects nothing"
        );
        assert_eq!(
            Task {
                critic: Some(Critic::new(5)),
                ..task(Feedback::Answer)
            }
            .check(&network(1, ONE / 2)),
            Ok(()),
            "the task's own critic on an engine without one"
        );
        let mut plain_exec = network(1, ONE / 2);
        let mut t = task(Feedback::Answer);
        let mut plain = task(Feedback::Answer);
        let weights = |exec: &Executor<8>| -> Vec<i16> {
            exec.units().iter().map(|u| u.value_weight).collect()
        };
        // The spikes of each unit in the train since `since`.
        let since = |exec: &mut Executor<8>, since: u64| -> Vec<u32> {
            let mut counts = vec![0u32; 16];
            for &(tick, unit) in exec.train() {
                if u64::from(tick) >= since {
                    counts[unit as usize] = counts[unit as usize].saturating_add(1);
                }
            }
            counts
        };
        // Trial 0, no readout cued: a tie, the reward −r, against a value of zero.
        let tie = t.trial(&mut exec, 0).unwrap();
        let bare = plain.trial(&mut plain_exec, 0).unwrap();
        assert_eq!(tie.selection, None);
        assert_eq!((tie.reward_q16, tie.value_q16), (-REWARD, Some(0)));
        assert_eq!(
            Outcome {
                value_q16: None,
                ..tie
            },
            bare,
            "at weights of zero the engine's critic delivers the outcome's reward"
        );
        assert_eq!(bare.value_q16, None, "no critic, no value");
        assert_eq!(
            exec.prediction(),
            Some(Prediction {
                value_q16: 0,
                error_q16: -REWARD,
                received_q16: -REWARD
            })
        );
        let fired = since(&mut exec, 0);
        let stimulus = t.stimuli[usize::from(tie.stimulus)].set;
        for unit in 0..16u32 {
            assert_eq!(
                fired[unit as usize] > 0,
                stimulus.contains(unit),
                "unit {unit}: the stimulus fired and nothing else"
            );
        }
        let moved: Vec<i16> = fired
            .iter()
            .map(|&c| (-i64::from(REWARD) * i64::from(c)).div_euclid(512) as i16)
            .collect();
        assert_eq!(weights(&exec), moved, "−r over 512 per spike");
        assert!(exec.features().iter().all(|&c| c == 0));
        // The same stimulus with its answer cued: correct, against the value its units now
        // carry.
        let rewarded_at = exec.ticks();
        exec.run(400 - TICKS as u64);
        let s = tie.stimulus;
        let same = (1..16u64).find(|&k| t.stimulus_at(k) == s).unwrap();
        cue(&exec, t.readout.sets()[usize::from(t.answer(s))]);
        let before = weights(&exec);
        let right = t.trial(&mut exec, same).unwrap();
        assert!(right.correct);
        let counts = since(&mut exec, rewarded_at);
        let value = before
            .iter()
            .zip(&counts)
            .map(|(&w, &c)| i64::from(w) * i64::from(c))
            .sum::<i64>()
            .div_euclid(4) as i32;
        assert!(
            value < 0,
            "{value}: the stimulus predicts the punishment it met"
        );
        assert_eq!(
            (right.reward_q16, right.value_q16),
            (REWARD - value, Some(value))
        );
        let moved: Vec<i16> = before
            .iter()
            .zip(&counts)
            .map(|(&w, &c)| {
                (i64::from(w) + (i64::from(REWARD - value) * i64::from(c)).div_euclid(512)) as i16
            })
            .collect();
        assert_eq!(weights(&exec), moved);
        // Withheld, past the refractory window: no value, no error, nothing moved, and the
        // trial's spikes wait.
        exec.run(400 - TICKS as u64);
        let reading = exec.prediction();
        let before = weights(&exec);
        let mut w = task(Feedback::Withheld);
        let none = w.trial(&mut exec, 0).unwrap();
        assert_eq!((none.reward_q16, none.value_q16), (0, None));
        assert_eq!(exec.prediction(), reading, "no reward, no reading");
        assert_eq!(weights(&exec), before);
        assert!(
            exec.features().iter().any(|&c| c > 0),
            "the stimulus's spikes wait for the next reward"
        );
    }

    /// The whole punishment under a task (ADR-0155): the same trials on two engines that carry
    /// the critic, one with the parameter unset and one with it set. A first tie is punished
    /// against a value of zero, and the two trials are one. The same stimulus then ties again,
    /// against the value its units now carry, below zero: unset, the trial records the error,
    /// softer than the reward; set, it records the reward, which is what the modulator
    /// received — the two signals part by exactly what the two records part by — while the
    /// engine's reading names the error beside it. The weights move by the error on both
    /// engines alike. A rewarded trial against a value below zero then records the error on
    /// both: nothing changes where the reward is at or above zero.
    #[test]
    fn a_task_records_a_whole_punishment_as_what_the_modulator_received() {
        let mut engines = [valued_whole(ONE / 2, false), valued_whole(ONE / 2, true)];
        assert_eq!(
            [engines[0].whole_punishment(), engines[1].whole_punishment()],
            [false, true]
        );
        let mut tasks = [task(Feedback::Answer), task(Feedback::Answer)];
        let weights = |exec: &Executor<8>| -> Vec<i16> {
            exec.units().iter().map(|u| u.value_weight).collect()
        };
        // Trial 0, no readout cued: a tie, the reward −r against a value of zero.
        let first = [0usize, 1].map(|k| tasks[k].trial(&mut engines[k], 0).unwrap());
        assert_eq!(
            first[0], first[1],
            "against a value of zero the two are one trial"
        );
        assert_eq!(
            (first[0].selection, first[0].reward_q16, first[0].value_q16),
            (None, -REWARD, Some(0))
        );
        // The same stimulus again, uncued: a tie against the value its units now carry.
        let s = first[0].stimulus;
        let same = (1..16u64).find(|&k| tasks[0].stimulus_at(k) == s).unwrap();
        for exec in engines.iter_mut() {
            exec.run(400 - TICKS as u64);
        }
        let before = weights(&engines[0]);
        assert_eq!(weights(&engines[1]), before, "one history so far");
        let second = [0usize, 1].map(|k| tasks[k].trial(&mut engines[k], same).unwrap());
        let value = second[0].value_q16.expect("the engine's value");
        assert!(
            value < 0 && value > -REWARD,
            "{value}: the stimulus predicts the punishment it met"
        );
        let error = -REWARD - value;
        for (k, received) in [error, -REWARD].into_iter().enumerate() {
            assert_eq!(
                (second[k].selection, second[k].correct, second[k].value_q16),
                (None, false, Some(value)),
                "engine {k}: a tie against one value"
            );
            assert_eq!(
                second[k].reward_q16, received,
                "engine {k}: the trial records what the modulator received"
            );
            assert_eq!(
                engines[k].prediction(),
                Some(Prediction {
                    value_q16: value,
                    error_q16: error,
                    received_q16: received
                }),
                "engine {k}: the engine's reading names the error beside it"
            );
            assert_eq!(
                second[k].signal_q16,
                engines[k].modulator().dopamine_rpe,
                "engine {k}"
            );
        }
        assert_eq!(
            second[1].signal_q16 - second[0].signal_q16,
            value,
            "the signals part by what the value would have softened"
        );
        let moved: Vec<i16> = before
            .iter()
            .zip(engines[0].units())
            .map(|(&w, unit)| {
                assert!(unit.value_weight <= w, "a punishment moves a weight down");
                unit.value_weight
            })
            .collect();
        assert_ne!(moved, before, "the units that fired moved");
        assert_eq!(
            weights(&engines[1]),
            moved,
            "the weights move by the error on both engines alike"
        );
        // The same stimulus with its answer cued: a reward against a value below zero is the
        // error on both.
        let third = (same.saturating_add(1)..64u64)
            .find(|&k| tasks[0].stimulus_at(k) == s)
            .unwrap();
        let rewarded = [0usize, 1].map(|k| {
            engines[k].run(400 - TICKS as u64);
            cue(
                &engines[k],
                tasks[k].readout.sets()[usize::from(tasks[k].answer(s))],
            );
            tasks[k].trial(&mut engines[k], third).unwrap()
        });
        let value = rewarded[0].value_q16.expect("the engine's value");
        assert!(value < 0, "{value}: still below zero");
        for (k, outcome) in rewarded.iter().enumerate() {
            assert!(outcome.correct, "engine {k}");
            assert_eq!(
                (outcome.reward_q16, outcome.value_q16),
                (REWARD - value, Some(value)),
                "engine {k}: a reward at or above zero is received as its error"
            );
            assert_eq!(
                engines[k].prediction(),
                Some(Prediction {
                    value_q16: value,
                    error_q16: REWARD - value,
                    received_q16: REWARD - value
                }),
                "engine {k}"
            );
        }
        assert_eq!(weights(&engines[0]), weights(&engines[1]));
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
            cancel: None,
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

    /// The delivery (ADR-0068): under the addressed one a trial leaves exactly the presented
    /// stimulus's units as the sources and the selected readout's units as the targets, no
    /// target at a tie, whatever the feedback; under the global one every unit on both sides,
    /// whatever was addressed before. The executor's refusal is the task's by name.
    #[test]
    fn the_addressed_delivery_addresses_the_stimulus_onto_the_selected_readout_and_none_at_a_tie() {
        let mut exec = network(2, ONE / 2);
        let mut t = task(Feedback::Answer);
        t.delivery = Delivery::Addressed;
        let addressed = |exec: &Executor<8>| -> (Vec<u32>, Vec<u32>) {
            (
                (0..16).filter(|&u| exec.is_source(u)).collect(),
                (0..16).filter(|&u| exec.is_target(u)).collect(),
            )
        };
        let stimulus_units = |t: &Task, trial: u64| -> Vec<u32> {
            t.stimuli[usize::from(t.stimulus_at(trial))]
                .set
                .units()
                .collect()
        };
        assert_eq!(
            exec.addressed_counts(),
            (16, 16),
            "every unit, both sides, before the first trial"
        );
        // Trial 0 ties: the stimulus's units are the sources, nothing is a target.
        let tie = t.trial(&mut exec, 0).unwrap();
        assert_eq!(tie.selection, None);
        assert_eq!(addressed(&exec), (stimulus_units(&t, 0), vec![]));
        assert_eq!(stimulus_units(&t, 0).len(), 4);
        // Readout 0 cued and selected: the stimulus's four onto readout 0's four.
        exec.run(400);
        cue(&exec, t.readout.sets()[0]);
        let one = t.trial(&mut exec, 1).unwrap();
        assert_eq!(one.selection, Some(0));
        assert_eq!(addressed(&exec), (stimulus_units(&t, 1), vec![4, 5, 6, 7]));
        // Readout 1 cued and selected: onto readout 1's four.
        exec.run(400);
        cue(&exec, t.readout.sets()[1]);
        let two = t.trial(&mut exec, 2).unwrap();
        assert_eq!(two.selection, Some(1));
        assert_eq!(
            addressed(&exec),
            (stimulus_units(&t, 2), vec![12, 13, 14, 15])
        );
        // The two stimuli of trials 1 and 2 differ at this seed, so the source side moved.
        assert_ne!(stimulus_units(&t, 1), stimulus_units(&t, 2));
        // Withheld feedback writes the set too: the set is the outcome's, not the reward's.
        t.feedback = Feedback::Withheld;
        exec.run(400);
        cue(&exec, t.readout.sets()[0]);
        let withheld = t.trial(&mut exec, 3).unwrap();
        assert_eq!((withheld.selection, withheld.reward_q16), (Some(0), 0));
        assert_eq!(addressed(&exec), (stimulus_units(&t, 3), vec![4, 5, 6, 7]));
        // Both cued, a tie: no target.
        exec.run(400);
        cue(&exec, t.readout.sets()[0]);
        cue(&exec, t.readout.sets()[1]);
        let both = t.trial(&mut exec, 4).unwrap();
        assert_eq!(both.selection, None);
        assert_eq!(addressed(&exec), (stimulus_units(&t, 4), vec![]));
        // The global delivery addresses every unit again, whatever was addressed before.
        t.delivery = Delivery::Global;
        exec.run(400);
        cue(&exec, t.readout.sets()[1]);
        let global = t.trial(&mut exec, 5).unwrap();
        assert_eq!(global.selection, Some(1));
        assert_eq!(exec.addressed_counts(), (16, 16));
        // The executor's refusal surfaces by name; `check` holds every readout inside the
        // arena, so a trial never meets it.
        assert_eq!(
            TaskError::from(AddressError::NoSuchUnit),
            TaskError::Address(AddressError::NoSuchUnit)
        );
        assert_eq!(exec.address([16u32], [4u32]), Err(AddressError::NoSuchUnit));
        assert_eq!(exec.address([0u32], [16u32]), Err(AddressError::NoSuchUnit));
        let mut outside = t;
        outside.delivery = Delivery::Addressed;
        outside.readout = Readout::new([set(4, 4), set(13, 4)]);
        assert_eq!(outside.trial(&mut exec, 6), Err(TaskError::SetOutsideArena));
    }

    /// Sixteen armed units without synapses, as [`network`], with the engine's critic and its
    /// window of `window` ticks (ADR-0131, ADR-0134).
    fn windowed_network(window: u16) -> Executor<8> {
        let mut exec = Executor::<8>::new(Config {
            workers: 2,
            units: 16,
            injector_capacity: 64,
            train_capacity: spikes_per_unit(TICKS).saturating_mul(16) as usize,
            modulation_baseline_q16: ONE / 2,
            critic: Some(ValueCritic { shift: 9, scale: 2 }),
            critic_window_ticks: window,
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

    /// The drawn delivery (ADR-0139): a trial leaves as the sources exactly the units that fired
    /// fewer ticks than the window after it opened — read here from the train, whatever the
    /// stimulus presented — and as the targets the selected readout's units, none at a tie,
    /// whatever the feedback; with the feedback withheld the counts stand, and the sources are
    /// the units whose count is not zero. A window that closed before the trial draws no source;
    /// one that spans the ticks between two trials draws a cued readout's units beside the
    /// stimulus's. The task refuses the delivery before any tick on an engine without the
    /// critic or without its window.
    #[test]
    fn the_drawn_delivery_addresses_the_units_the_critic_counted_onto_the_selected_readout() {
        let sources =
            |exec: &Executor<8>| -> Vec<u32> { (0..16).filter(|&u| exec.is_source(u)).collect() };
        let targets =
            |exec: &Executor<8>| -> Vec<u32> { (0..16).filter(|&u| exec.is_target(u)).collect() };
        // The units that fired fewer than `window` ticks after `opened`, from the train.
        let counted = |exec: &mut Executor<8>, opened: u64, window: u16| -> Vec<u32> {
            let mut units: Vec<u32> = exec
                .train()
                .iter()
                .filter(|&&(tick, _)| {
                    u64::from(tick)
                        .checked_sub(opened)
                        .is_some_and(|d| d < u64::from(window))
                })
                .map(|&(_, unit)| unit)
                .collect();
            units.sort_unstable();
            units.dedup();
            units
        };
        let stimulus_units = |t: &Task, trial: u64| -> Vec<u32> {
            t.stimuli[usize::from(t.stimulus_at(trial))]
                .set
                .units()
                .collect()
        };
        let mut t = task(Feedback::Answer);
        t.delivery = Delivery::Drawn;
        // A window of thirty ticks: the stimulus's volley, some twelve ticks on, and nothing
        // after it.
        let mut exec = windowed_network(30);
        let opened = exec.window_opened();
        let tie = t.trial(&mut exec, 0).unwrap();
        assert_eq!(tie.selection, None);
        let drawn = counted(&mut exec, opened, 30);
        assert_eq!(
            drawn,
            stimulus_units(&t, 0),
            "the window counted the volley"
        );
        assert_eq!((sources(&exec), targets(&exec)), (drawn, vec![]));
        // The ticks between two trials close the window: a cued readout selected, no source.
        exec.run(400);
        cue(&exec, t.readout.sets()[0]);
        let opened = exec.window_opened();
        let closed = t.trial(&mut exec, 1).unwrap();
        assert_eq!(closed.selection, Some(0));
        assert_eq!(counted(&mut exec, opened, 30), Vec::<u32>::new());
        assert_eq!(
            (sources(&exec), targets(&exec)),
            (vec![], vec![4, 5, 6, 7]),
            "the window closed before the trial: no source, whatever was presented"
        );
        // A window that spans the ticks between two trials: the cued readout's units are drawn
        // beside the stimulus's, and the feedback withheld leaves the counts standing.
        let mut exec = windowed_network(1_000);
        t.trial(&mut exec, 0).unwrap();
        exec.run(400);
        cue(&exec, t.readout.sets()[1]);
        t.feedback = Feedback::Withheld;
        let opened = exec.window_opened();
        let withheld = t.trial(&mut exec, 2).unwrap();
        assert_eq!((withheld.selection, withheld.reward_q16), (Some(1), 0));
        let drawn = counted(&mut exec, opened, 1_000);
        let mut expected: Vec<u32> = stimulus_units(&t, 2);
        expected.extend([12, 13, 14, 15]);
        expected.sort_unstable();
        assert_eq!(
            drawn, expected,
            "the stimulus's volley and the cued readout"
        );
        assert_eq!(
            (sources(&exec), targets(&exec)),
            (drawn.clone(), vec![12, 13, 14, 15])
        );
        let nonzero: Vec<u32> = (0..16)
            .filter(|&u| exec.features()[u as usize] != 0)
            .collect();
        assert_eq!(
            nonzero, drawn,
            "withheld, the counts stand: the sources are theirs"
        );
        // Refused before any tick without the critic, and without its window.
        let mut none = network(2, ONE / 2);
        assert_eq!(
            t.trial(&mut none, 3),
            Err(TaskError::Address(AddressError::NoCritic))
        );
        assert_eq!(none.ticks(), 0, "no tick ran");
        let mut unwindowed = windowed_network(0);
        assert_eq!(
            t.check(&unwindowed),
            Err(TaskError::Address(AddressError::NoWindow))
        );
        assert_eq!(
            t.trial(&mut unwindowed, 3),
            Err(TaskError::Address(AddressError::NoWindow))
        );
        assert_eq!(unwindowed.ticks(), 0, "no tick ran");
        assert_eq!(
            unwindowed.addressed_counts(),
            (16, 16),
            "the set as at birth"
        );
    }

    /// The hold's rule at its edges (ADR-0144): due before the tick the window closes at and
    /// before every `every`-th tick after it, below `until`; never before the close, never at
    /// `until` or after it, and never with no cadence.
    #[test]
    fn a_hold_is_due_on_its_cadence_from_the_close_and_at_no_other_tick() {
        let h = Hold {
            until: 48,
            every: 8,
            messages: 1,
            efficacy_q16: -ONE,
        };
        let due =
            |h: Hold, close: u32| -> Vec<u32> { (0..64).filter(|&k| h.is_due(close, k)).collect() };
        assert_eq!(due(h, 16), vec![16, 24, 32, 40]);
        assert!(!h.is_due(16, 48), "at `until`, though on the cadence");
        assert_eq!(due(Hold { until: 49, ..h }, 16), vec![16, 24, 32, 40, 48]);
        assert_eq!(due(Hold { until: 41, ..h }, 16), vec![16, 24, 32, 40]);
        assert_eq!(due(Hold { until: 40, ..h }, 16), vec![16, 24, 32]);
        assert_eq!(
            due(Hold { until: 17, ..h }, 16),
            vec![16],
            "the close alone"
        );
        assert_eq!(due(Hold { until: 16, ..h }, 16), Vec::<u32>::new());
        assert_eq!(
            due(Hold { every: 1, ..h }, 16),
            (16..48).collect::<Vec<u32>>(),
            "a cadence of one: every tick from the close below `until`"
        );
        assert_eq!(
            due(Hold { every: 0, ..h }, 16),
            Vec::<u32>::new(),
            "no cadence, never due"
        );
        assert_eq!(due(Hold { every: 64, ..h }, 16), vec![16]);
        assert_eq!(due(h, 48), Vec::<u32>::new(), "a close at `until`");
        assert_eq!(due(h, 56), Vec::<u32>::new(), "a close after `until`");
        let top = Hold {
            until: u32::MAX,
            every: 1,
            ..h
        };
        assert!(top.is_due(u32::MAX - 1, u32::MAX - 1) && !top.is_due(u32::MAX - 1, u32::MAX));
        assert!(!top.is_due(u32::MAX, u32::MAX - 1), "before the close");
    }

    /// What the gate holds (ADR-0144): after a selection the hold's messages go into every unit
    /// of the channel whose net output is above zero, the one not selected, and into no other
    /// unit; at a tie, and before any selection, into none. An injector that refuses a message
    /// stops the hold there.
    #[test]
    fn the_gate_holds_the_channel_not_selected_and_neither_at_a_tie() {
        let hold = Hold {
            until: 48,
            every: 8,
            messages: 3,
            efficacy_q16: -ONE,
        };
        // The basal potentials two ticks after a delivery: the tick that drains the ring and
        // the tick that integrates what it drained.
        let landed = |counts: Option<[u32; 2]>| -> (Option<u8>, [i32; 2], u32, Vec<i32>, u64) {
            let mut exec = network(1, ONE / 2);
            let mut r = Readout::new([set(4, 4), set(12, 4)]);
            let selection = counts.and_then(|c| r.select(c));
            let outputs = r.channels().map(|c| c.gpi_snr_inhibition);
            let sent = r.hold(&exec.injector(), &hold).unwrap();
            exec.run(2);
            let basal = exec.units().iter().map(|u| u.v_basal).collect();
            (selection, outputs, sent, basal, exec.delivered())
        };
        let into = |held: core::ops::Range<usize>| -> Vec<i32> {
            (0..16)
                .map(|u| if held.contains(&u) { -3 * ONE } else { 0 })
                .collect()
        };
        assert_eq!(
            landed(Some([3, 1])),
            (Some(0), [-2 * ONE, 2 * ONE], 12, into(12..16), 12),
            "readout 0 selected: three messages into each unit of readout 1"
        );
        assert_eq!(
            landed(Some([1, 3])),
            (Some(1), [2 * ONE, -2 * ONE], 12, into(4..8), 12),
            "readout 1 selected: into readout 0"
        );
        assert_eq!(
            landed(Some([2, 2])),
            (None, [0, 0], 0, into(0..0), 0),
            "a tie: both outputs zero, nothing delivered"
        );
        assert_eq!(
            landed(None),
            (None, [0, 0], 0, into(0..0), 0),
            "before a selection the channels are at rest"
        );
        // Sixty-four slots in the ring: seventeen messages into each of four units is four
        // too many, and the hold stops where the ring refuses.
        let exec = network(1, ONE / 2);
        let mut r = Readout::new([set(4, 4), set(12, 4)]);
        assert_eq!(r.select([3, 1]), Some(0));
        let wide = Hold {
            messages: 17,
            ..hold
        };
        assert_eq!(r.hold(&exec.injector(), &wide), Err(InjectError::Full));
        assert_eq!(
            r.hold(
                &network(1, ONE / 2).injector(),
                &Hold {
                    messages: 16,
                    ..hold
                }
            ),
            Ok(64)
        );
    }

    /// Every refusal of a hold (ADR-0144), each at its edge, and a trial with a refused hold
    /// runs no tick.
    #[test]
    fn every_refusal_of_a_hold_is_named() {
        let exec = network(1, ONE / 2);
        let mut ok = task(Feedback::Answer);
        ok.window = Window { from: 4, ticks: 12 };
        let hold = Hold {
            until: 48,
            every: 8,
            messages: 1,
            efficacy_q16: -1,
        };
        let with = |hold: Hold| Task {
            hold: Some(hold),
            ..ok
        };
        assert_eq!(ok.check(&exec), Ok(()));
        assert_eq!(with(hold).check(&exec), Ok(()));
        assert_eq!(
            with(Hold {
                messages: 0,
                ..hold
            })
            .check(&exec),
            Err(TaskError::EmptyHold)
        );
        assert_eq!(
            with(Hold { every: 0, ..hold }).check(&exec),
            Err(TaskError::EmptyHold)
        );
        assert_eq!(
            with(Hold {
                efficacy_q16: 0,
                ..hold
            })
            .check(&exec),
            Err(TaskError::HoldNotNegative)
        );
        assert_eq!(
            with(Hold {
                efficacy_q16: ONE,
                ..hold
            })
            .check(&exec),
            Err(TaskError::HoldNotNegative)
        );
        // The window closes at tick 16: a hold that ends there has no tick due, one that
        // ends a tick later has the close itself.
        assert_eq!(
            with(Hold { until: 16, ..hold }).check(&exec),
            Err(TaskError::HoldBeforeClose)
        );
        assert_eq!(
            with(Hold { until: 0, ..hold }).check(&exec),
            Err(TaskError::HoldBeforeClose)
        );
        assert_eq!(with(Hold { until: 17, ..hold }).check(&exec), Ok(()));
        // The last message lands on tick `until` at the latest, which must be inside the
        // trial's sixty-four.
        assert_eq!(
            with(Hold {
                until: TICKS - 1,
                ..hold
            })
            .check(&exec),
            Ok(())
        );
        assert_eq!(
            with(Hold {
                until: TICKS,
                ..hold
            })
            .check(&exec),
            Err(TaskError::HoldOutsideTrial)
        );
        assert_eq!(
            with(Hold {
                until: u32::MAX,
                ..hold
            })
            .check(&exec),
            Err(TaskError::HoldOutsideTrial)
        );
        // A window that is the whole trial closes with it: no hold fits.
        let mut whole = with(Hold {
            until: TICKS,
            ..hold
        });
        whole.window = Window::whole(TICKS);
        assert_eq!(whole.check(&exec), Err(TaskError::HoldBeforeClose));
        // The refusals in their order: an empty hold before its sign, its sign before its span.
        assert_eq!(
            with(Hold {
                until: 0,
                every: 0,
                messages: 0,
                efficacy_q16: 0
            })
            .check(&exec),
            Err(TaskError::EmptyHold)
        );
        assert_eq!(
            with(Hold {
                until: 0,
                efficacy_q16: 0,
                ..hold
            })
            .check(&exec),
            Err(TaskError::HoldNotNegative)
        );
        // A window refused is refused before the hold is read against it.
        let mut outside = with(hold);
        outside.window = Window {
            from: TICKS,
            ticks: 1,
        };
        assert_eq!(outside.check(&exec), Err(TaskError::WindowOutsideTrial));
        let mut exec = network(1, ONE / 2);
        assert_eq!(
            with(Hold {
                until: TICKS,
                ..hold
            })
            .trial(&mut exec, 0),
            Err(TaskError::HoldOutsideTrial)
        );
        assert_eq!((exec.ticks(), exec.delivered()), (0, 0), "no tick ran");
    }

    /// The fields a trial can move on a unit of these networks.
    fn fields(exec: &Executor<8>) -> Vec<(i32, i32, i32, u16, u32)> {
        exec.units()
            .iter()
            .map(|u| {
                (
                    u.v_soma,
                    u.v_basal,
                    u.v_thresh,
                    u.refractory_ticks,
                    u.last_soma_spike_tick,
                )
            })
            .collect()
    }

    /// A task with no hold runs the trial it ran before the hold existed (ADR-0144): `trial`
    /// against the trial as ADR-0139 left it, written out here as the oracle — the train read
    /// once after the last tick and the selection made there — on twin engines over eight
    /// trials under a drive, a cancel, a sub-window, the critic's window and each delivery
    /// before the released one, a readout cued before some of them: the same outcome, the same
    /// units' fields, the same train, the same messages drained, the same addressed set and the
    /// same signal after every trial.
    #[test]
    fn a_trial_with_no_hold_is_the_trial_before_the_hold() {
        fn before(t: &mut Task, exec: &mut Executor<8>, trial: u64) -> Outcome {
            t.check(exec).unwrap();
            let start = exec.ticks();
            let stimulus = t.stimulus_at(trial);
            let inject = exec.injector();
            let presented = t.stimuli[usize::from(stimulus)];
            presented.inject(&inject).unwrap();
            for k in 0..t.ticks {
                presented.cancel_at(&inject, k).unwrap();
                t.drive.step(&inject, exec.ticks()).unwrap();
                exec.tick();
            }
            let counts = t.readout.count_window(
                exec.train(),
                (start as u32).wrapping_add(t.window.from),
                t.window.ticks,
            );
            let selection = t.readout.select(counts);
            let correct = selection == Some(t.answer(stimulus));
            let selected = selection.map(|r| t.readout.sets()[usize::from(r)]);
            match t.delivery {
                Delivery::Global => exec.address_all(),
                Delivery::Addressed => exec
                    .address(
                        presented.set.units(),
                        selected.iter().flat_map(|set| set.units()),
                    )
                    .unwrap(),
                Delivery::Drawn => exec
                    .address_drawn(selected.iter().flat_map(|set| set.units()))
                    .unwrap(),
                Delivery::Released => unreachable!("the oracle is of the trial before it"),
            }
            let delivered_q16 = if correct {
                t.reward_q16
            } else {
                t.reward_q16.saturating_neg()
            };
            let signal_q16 = exec.reward(delivered_q16);
            let prediction = exec.prediction().expect("the engine's critic");
            Outcome {
                trial,
                stimulus,
                counts,
                selection,
                correct,
                reward_q16: prediction.error_q16,
                signal_q16,
                expected_q16: None,
                value_q16: Some(prediction.value_q16),
                held: 0,
                drawn: false,
            }
        }
        for delivery in [Delivery::Global, Delivery::Addressed, Delivery::Drawn] {
            let mut t = task(Feedback::Answer);
            t.delivery = delivery;
            t.window = Window { from: 4, ticks: 20 };
            t.drive = Drive {
                every: 1,
                messages: 2,
                efficacy_q16: 0x2000,
                units: 16,
                seed: 5,
            };
            for stimulus in t.stimuli.iter_mut() {
                stimulus.cancel = Some(Cancel {
                    offset: 30,
                    ticks: 3,
                    messages: 1,
                    efficacy_q16: -ONE,
                });
            }
            assert_eq!(t.hold, None);
            let mut oracle = t;
            let mut now = windowed_network(40);
            let mut then = windowed_network(40);
            let mut selections = Vec::new();
            for trial in 0..8u64 {
                // A readout cued before three of the trials, so that the run holds each
                // selection and a tie.
                let cued = [None, Some(0), None, Some(1), Some(1), None, Some(0), None];
                if let Some(r) = cued[trial as usize] {
                    cue(&now, t.readout.sets()[r]);
                    cue(&then, t.readout.sets()[r]);
                }
                let outcome = t.trial(&mut now, trial).unwrap();
                assert_eq!(
                    outcome,
                    before(&mut oracle, &mut then, trial),
                    "{delivery:?} trial {trial}"
                );
                assert_eq!(fields(&now), fields(&then), "{delivery:?} trial {trial}");
                assert_eq!(now.train().to_vec(), then.train().to_vec());
                assert_eq!(
                    (
                        now.delivered(),
                        now.addressed_counts(),
                        now.modulator().dopamine_rpe,
                        now.features().to_vec(),
                    ),
                    (
                        then.delivered(),
                        then.addressed_counts(),
                        then.modulator().dopamine_rpe,
                        then.features().to_vec(),
                    ),
                    "{delivery:?} trial {trial}"
                );
                assert_eq!(
                    (0..16).map(|u| now.is_source(u)).collect::<Vec<bool>>(),
                    (0..16).map(|u| then.is_source(u)).collect::<Vec<bool>>()
                );
                assert_eq!(
                    t.readout, oracle.readout,
                    "the channels as the gate left them"
                );
                selections.push(outcome.selection);
            }
            assert!(
                [None, Some(0), Some(1)]
                    .iter()
                    .all(|s| selections.contains(s)),
                "{delivery:?}: each selection and a tie among {selections:?}"
            );
        }
    }

    /// The hold on the engine (ADR-0144), sixteen armed units without synapses at a gain of
    /// 1.0, the window the trial's first ticks and a hold of two messages at the bound every
    /// eight ticks from the close below tick 56.
    ///
    /// - A cued readout fires inside the window and is selected at the close: the hold's
    ///   messages — four times, four units, two messages — go into the other readout's units,
    ///   whose potentials after the trial are those of the membrane rule stepped alone with
    ///   −4.0 landing on the tick after each tick the hold was due at, and on no other; every
    ///   other unit, the train and the outcome but its count of the hold's messages are those
    ///   of the same trial with no hold.
    /// - The selection is the one the trial's end reads: the counts are the train's over the
    ///   window after the trial, with the window closing on the tick after the cued readout's
    ///   spike as with it closing later; a window that closes on the spike's own tick does not
    ///   hold it, the trial is a tie with the hold as without it, and nothing is delivered.
    #[test]
    fn a_hold_selects_at_the_window_s_close_and_holds_the_channel_not_selected() {
        let hold = Hold {
            until: 56,
            every: 8,
            messages: 2,
            efficacy_q16: -0x0002_0000,
        };
        // One trial from a fresh network with `cued` cued before it, under a window of the
        // trial's first `window` ticks: the outcome, the units' fields, the train and the
        // messages drained.
        type Ran = (
            Outcome,
            Vec<(i32, i32, i32, u16, u32)>,
            Vec<(u32, u32)>,
            u64,
        );
        let run = |cued: Option<usize>, window: u32, hold: Option<Hold>| -> Ran {
            let mut exec = network(2, ONE / 2);
            let mut t = task(Feedback::Answer);
            t.window = Window {
                from: 0,
                ticks: window,
            };
            t.hold = hold;
            if let Some(r) = cued {
                cue(&exec, t.readout.sets()[r]);
            }
            let outcome = t.trial(&mut exec, 0).unwrap();
            let read = t
                .readout
                .count_window(exec.train(), t.window.from, t.window.ticks);
            assert_eq!(
                outcome.counts, read,
                "the counts are the train's at the end"
            );
            let train = exec.train().to_vec();
            (outcome, fields(&exec), train, exec.delivered())
        };
        // The tick the cued readout's units fire on, read from a trial with no hold.
        let (bare, _, train, drained) = run(Some(0), 24, None);
        assert_eq!((bare.selection, bare.held, drained), (Some(0), 0, 16));
        let spikes: Vec<u32> = train
            .iter()
            .filter(|&&(_, unit)| (4..8).contains(&unit))
            .map(|&(tick, _)| tick)
            .collect();
        let at = spikes[0];
        assert_eq!(spikes, vec![at; 4], "the cued readout fires together, once");
        assert!(at < 16, "inside the windows below");
        // The membrane rule stepped alone: an armed unit at rest taking −4.0 on the tick after
        // each tick the hold is due at under a window that closes at `close`.
        let alone = |close: u32| -> (i32, i32, i32, u16, u32) {
            let mut unit = cortex_core::DendriticSuperNeuron::new(12);
            unit.v_thresh = THRESHOLD_BASE;
            for tick in 0..TICKS {
                let landing = tick.checked_sub(1).is_some_and(|k| hold.is_due(close, k));
                unit.integrate(if landing { -0x0004_0000 } else { 0 }, 0, tick);
            }
            (
                unit.v_soma,
                unit.v_basal,
                unit.v_thresh,
                unit.refractory_ticks,
                unit.last_soma_spike_tick,
            )
        };
        for close in [at.saturating_add(1), 24] {
            for (cued, held_units) in [(0usize, 12..16usize), (1, 4..8)] {
                let (without, fields_without, train_without, drained_without) =
                    run(Some(cued), close, None);
                let (with, fields_with, train_with, drained_with) =
                    run(Some(cued), close, Some(hold));
                let times = (close..hold.until).step_by(8).count() as u32;
                assert_eq!(
                    (with.selection, with.held),
                    (Some(cued as u8), times.saturating_mul(8)),
                    "close {close}: selected at the close, and the other readout held"
                );
                assert_eq!(
                    with,
                    Outcome {
                        held: with.held,
                        ..without
                    },
                    "close {close}: the outcome the trial's end reads"
                );
                assert_eq!(train_with, train_without, "no held unit fired either way");
                assert_eq!(
                    drained_with,
                    drained_without.saturating_add(u64::from(with.held))
                );
                for (u, (held, bare)) in fields_with.iter().zip(&fields_without).enumerate() {
                    if held_units.contains(&u) {
                        assert_eq!(*held, alone(close), "close {close} unit {u}: held");
                        assert!(held.1 < 0 && bare.1 == 0, "unit {u} below rest");
                    } else {
                        assert_eq!(held, bare, "close {close} unit {u}: not reached");
                    }
                }
            }
        }
        // A window that closes on the spike's own tick does not hold the spike: a tie, with
        // the hold as without it, and nothing delivered.
        let (without, fields_without, train_without, drained_without) = run(Some(0), at, None);
        let (with, fields_with, train_with, drained_with) = run(Some(0), at, Some(hold));
        assert_eq!((without.selection, without.counts), (None, [0, 0]));
        assert_eq!(with, without);
        assert_eq!(
            (fields_with, train_with, drained_with, with.held),
            (fields_without, train_without, drained_without, 0)
        );
        // No cue at all: a tie, and nothing delivered.
        let (tie, _, _, drained) = run(None, 24, Some(hold));
        assert_eq!((tie.selection, tie.held, drained), (None, 0, 8));
    }

    /// The released delivery (ADR-0144): a trial leaves as the sources the units the critic's
    /// window counted, as the drawn delivery does, and as the targets every unit, whatever was
    /// selected, a tie included; refused before any tick on an engine without the critic or
    /// without its window.
    #[test]
    fn the_released_delivery_addresses_the_units_the_critic_counted_onto_every_unit() {
        let sources =
            |exec: &Executor<8>| -> Vec<u32> { (0..16).filter(|&u| exec.is_source(u)).collect() };
        let targets =
            |exec: &Executor<8>| -> Vec<u32> { (0..16).filter(|&u| exec.is_target(u)).collect() };
        let counted = |exec: &Executor<8>| -> Vec<u32> {
            (0..16)
                .filter(|&u| exec.features()[u as usize] != 0)
                .collect()
        };
        let every: Vec<u32> = (0..16).collect();
        let mut t = task(Feedback::Withheld);
        t.delivery = Delivery::Released;
        // Withheld, so that the counts the address was drawn from still stand after the trial.
        let mut exec = windowed_network(30);
        let tie = t.trial(&mut exec, 0).unwrap();
        let presented: Vec<u32> = t.stimuli[usize::from(tie.stimulus)].set.units().collect();
        assert_eq!(tie.selection, None);
        assert_eq!(counted(&exec), presented, "the window counted the volley");
        assert_eq!(
            (sources(&exec), targets(&exec)),
            (presented.clone(), every.clone()),
            "a tie: the drawn sources onto every unit"
        );
        // The drawn delivery from the same state: the same sources, and at a tie no target.
        let mut drawn = t;
        drawn.delivery = Delivery::Drawn;
        let mut beside = windowed_network(30);
        drawn.trial(&mut beside, 0).unwrap();
        assert_eq!(
            (sources(&beside), targets(&beside)),
            (presented, vec![]),
            "the drawn delivery's sources, and its targets none at a tie"
        );
        // A window that spans the ticks between two trials: a cued readout is selected and
        // drawn beside the stimulus's units, and every unit is a target still.
        let mut exec = windowed_network(1_000);
        t.trial(&mut exec, 0).unwrap();
        exec.run(400);
        cue(&exec, t.readout.sets()[1]);
        let selected = t.trial(&mut exec, 2).unwrap();
        assert_eq!(selected.selection, Some(1));
        let drawn_units = counted(&exec);
        assert!(drawn_units.contains(&12) && drawn_units.len() < 16);
        assert_eq!((sources(&exec), targets(&exec)), (drawn_units, every));
        // Rewarded, the reward zeroes the counts after the address is written.
        t.feedback = Feedback::Answer;
        let mut exec = windowed_network(30);
        let rewarded = t.trial(&mut exec, 0).unwrap();
        assert_eq!(
            (sources(&exec).len(), targets(&exec).len(), counted(&exec)),
            (4, 16, vec![])
        );
        assert_eq!(rewarded.reward_q16, -REWARD, "a tie is punished");
        // Refused before any tick without the critic, and without its window.
        let mut none = network(2, ONE / 2);
        assert_eq!(
            t.check(&none),
            Err(TaskError::Address(AddressError::NoCritic))
        );
        assert_eq!(
            t.trial(&mut none, 3),
            Err(TaskError::Address(AddressError::NoCritic))
        );
        assert_eq!(none.ticks(), 0, "no tick ran");
        let mut unwindowed = windowed_network(0);
        assert_eq!(
            t.trial(&mut unwindowed, 3),
            Err(TaskError::Address(AddressError::NoWindow))
        );
        assert_eq!(
            (unwindowed.ticks(), unwindowed.addressed_counts()),
            (0, (16, 16)),
            "no tick ran, and the set is as at birth"
        );
        // The deliveries that draw nothing are not refused there.
        for delivery in [Delivery::Global, Delivery::Addressed] {
            t.delivery = delivery;
            assert_eq!(t.check(&network(2, ONE / 2)), Ok(()), "{delivery:?}");
        }
    }

    /// The cancel's rule at its edges (ADR-0076): due from its offset for its ticks, widened
    /// at the top of the tick space, and never with no tick.
    #[test]
    fn a_cancel_is_due_at_its_ticks_and_at_no_other() {
        let c = Cancel {
            offset: 201,
            ticks: 10,
            messages: 6,
            efficacy_q16: -ONE,
        };
        assert_eq!(c.end(), 211);
        assert!(!c.is_due(0) && !c.is_due(200));
        assert!(c.is_due(201) && c.is_due(205) && c.is_due(210));
        assert!(!c.is_due(211) && !c.is_due(u32::MAX));
        let one = Cancel { ticks: 1, ..c };
        assert_eq!(one.end(), 202);
        assert!(!one.is_due(200) && one.is_due(201) && !one.is_due(202));
        let top = Cancel {
            offset: u32::MAX,
            ticks: u32::MAX,
            ..c
        };
        assert_eq!(top.end(), 0x1_FFFF_FFFE, "widened, not wrapped");
        assert!(top.is_due(u32::MAX) && !top.is_due(u32::MAX - 1));
        let none = Cancel { ticks: 0, ..c };
        assert_eq!(none.end(), 201);
        assert!(!none.is_due(201) && !none.is_due(200), "no tick is due");
    }

    /// A stimulus with no cancel injects what it injected before the cancel existed
    /// (ADR-0076): `inject` against the loop ADR-0059 wrote, message for message through the
    /// ring and tick for tick on the units, and `cancel_at` nothing at any tick, so a trial's
    /// injections are the first injection's and the drive's, as they were.
    #[test]
    fn a_stimulus_with_no_cancel_injects_what_it_injected_before() {
        let traced = || {
            let mut exec = Executor::<8>::new(Config {
                workers: 1,
                units: 16,
                injector_capacity: 64,
                trace_capacity: 64,
                train_capacity: spikes_per_unit(TICKS).saturating_mul(16) as usize,
                modulation_baseline_q16: ONE / 2,
                ..Config::default()
            })
            .unwrap();
            for unit in exec.units_mut() {
                unit.v_thresh = THRESHOLD_BASE;
                unit.stp_u_rel = STP_U;
                unit.stp_r_ves = STP_MAX;
            }
            exec
        };
        let s = stimulus(0);
        assert_eq!(s.cancel, None);
        let mut now = traced();
        let mut before = traced();
        assert_eq!(s.inject(&now.injector()), Ok(8));
        // `Stimulus::inject` as ADR-0059 wrote it: the oracle.
        let inject = before.injector();
        let message = spike_message(s.efficacy_q16, false);
        let mut sent = 0u32;
        for unit in s.set.units() {
            for _ in 0..s.messages {
                inject.inject(unit, message).unwrap();
                sent = sent.saturating_add(1);
            }
        }
        assert_eq!(sent, 8);
        let fields = |exec: &Executor<8>| -> Vec<(i32, i32, i32, u16, u32)> {
            exec.units()
                .iter()
                .map(|u| {
                    (
                        u.v_soma,
                        u.v_basal,
                        u.v_thresh,
                        u.refractory_ticks,
                        u.last_soma_spike_tick,
                    )
                })
                .collect()
        };
        for k in 0..TICKS {
            assert_eq!(
                s.cancel_at(&now.injector(), k),
                Ok(0),
                "tick {k}: no cancel, nothing due"
            );
            now.tick();
            before.tick();
            assert_eq!(fields(&now), fields(&before), "tick {k}");
        }
        assert_eq!(now.train(), before.train());
        assert_eq!(now.delivered(), 8);
        let now = now.shutdown();
        let before = before.shutdown();
        assert_eq!(
            now[0].delivered, before[0].delivered,
            "the same messages, in order"
        );
        assert_eq!(now[0].delivered.len(), 8);
        // A trial with no cancel injects the stimulus and nothing else: eight messages, as
        // ADR-0059's trial injected.
        let mut exec = network(1, ONE / 2);
        let mut t = task(Feedback::Answer);
        t.trial(&mut exec, 0).unwrap();
        assert_eq!(exec.delivered(), 8);
    }

    /// The cancel on the engine (ADR-0076; the numbers computed apart from the tree by an
    /// oracle over the membrane rule first and held to the engine here). At a gain of 1.0 four
    /// messages of 1.25 fire an armed unit at rest at 4, at 205 and at 406 ticks: once on the
    /// drive and again at the end of each refractory window, from what the basal compartment
    /// still holds. A cancel of six messages at the bound (−2.0 each) injected at offset 204
    /// lands on tick 205, the first tick the unit integrates again, and the unit fires once;
    /// injected one tick earlier it lands inside the window, which drops it, and the unit
    /// fires three times; one tick later it lands after the second spike, inside the next
    /// window, and the unit fires three times; five messages on time leave the unit its second
    /// spike and take its third; a cancel over three ticks from 203 lands on 205 among them.
    #[test]
    fn a_cancel_lands_on_the_tick_after_its_offset_and_the_refractory_window_drops_one_inside_it() {
        const PROBE: u32 = 1024;
        let run = |cancel: Option<Cancel>| -> Vec<Vec<u32>> {
            let mut exec = Executor::<8>::new(Config {
                workers: 1,
                units: 16,
                injector_capacity: 256,
                train_capacity: spikes_per_unit(PROBE).saturating_mul(16) as usize,
                modulation_baseline_q16: ONE,
                ..Config::default()
            })
            .unwrap();
            for unit in exec.units_mut() {
                unit.v_thresh = THRESHOLD_BASE;
                unit.stp_u_rel = STP_U;
                unit.stp_r_ves = STP_MAX;
            }
            let mut t = task(Feedback::Withheld);
            t.ticks = PROBE;
            t.window = Window::whole(PROBE);
            t.stimuli = [
                Stimulus {
                    set: set(0, 4),
                    messages: 4,
                    efficacy_q16: CUE_Q16,
                    cancel,
                },
                Stimulus {
                    set: set(8, 4),
                    messages: 4,
                    efficacy_q16: CUE_Q16,
                    cancel,
                },
            ];
            let trial = (0..8u64).find(|&k| t.stimulus_at(k) == 0).unwrap();
            let start = exec.ticks() as u32;
            t.trial(&mut exec, trial).unwrap();
            let train: Vec<(u32, u32)> = exec.train().to_vec();
            (0..4u32)
                .map(|u| {
                    train
                        .iter()
                        .filter(|&&(_, unit)| unit == u)
                        .map(|&(tick, _)| tick.wrapping_sub(start))
                        .collect()
                })
                .collect()
        };
        let bound = Cancel {
            offset: 204,
            ticks: 1,
            messages: 6,
            efficacy_q16: -0x0002_0000,
        };
        assert_eq!(run(None), vec![vec![4, 205, 406]; 4], "no cancel");
        assert_eq!(run(Some(bound)), vec![vec![4]; 4], "on time");
        assert_eq!(
            run(Some(Cancel {
                offset: 203,
                ..bound
            })),
            vec![vec![4, 205, 406]; 4],
            "inside the window: dropped"
        );
        assert_eq!(
            run(Some(Cancel {
                offset: 205,
                ..bound
            })),
            vec![vec![4, 205, 406]; 4],
            "after the second spike: inside the next window"
        );
        assert_eq!(
            run(Some(Cancel {
                messages: 5,
                ..bound
            })),
            vec![vec![4, 205]; 4],
            "five under-cancel the second spike and take the third"
        );
        assert_eq!(
            run(Some(Cancel {
                offset: 203,
                ticks: 3,
                ..bound
            })),
            vec![vec![4]; 4],
            "a span over 203..206 lands on 205 among its ticks"
        );
    }

    /// The two draws of a trial against an oracle written apart from the tree: SplitMix64's
    /// finaliser (Steele, Lea and Flood 2014) implemented in another language, at seed 27, the
    /// learning harness's. Its low bits overlap the trials', so a draw from `seed | trial`
    /// differs from one from `seed ^ trial`; at seed zero the two agree, which is why the test
    /// above cannot tell them apart and why the shuffled run was the only test that could
    /// (ADR-0061). Bit `k` of each mask is trial `k`.
    #[test]
    fn the_stimulus_and_the_coin_of_the_first_sixteen_trials_at_seed_27() {
        let t = Task {
            seed: 27,
            ..task(Feedback::Shuffled)
        };
        let (mut stimuli, mut coins) = (0u16, 0u16);
        for k in 0..16u32 {
            stimuli |= u16::from(t.stimulus_at(u64::from(k))).wrapping_shl(k);
            coins |= u16::from(t.coin_at(u64::from(k))).wrapping_shl(k);
        }
        assert_eq!(stimuli, 0x4c04, "bit 0 of mix64(27 ^ k)");
        assert_eq!(coins, 0x6050, "bit 32 of mix64(27 ^ k)");
    }

    /// The misleading coin against the same oracle written apart from the tree (ADR-0148): bits
    /// 48, 49 and 50 of SplitMix64's finaliser of the seed and the trial's index all zero, over
    /// the first sixty-four trials at the unit tests' seed and at the learning harness's. Bit
    /// `k` of each mask is trial `k`; eight of sixty-four at both.
    #[test]
    fn the_misleading_coin_of_the_first_sixty_four_trials_at_seeds_0_and_27() {
        assert_eq!(MISLEADING_BITS, 7 << 48, "bits 48, 49 and 50");
        for (seed, mask) in [
            (0u64, 0x0104_0000_030c_0005u64),
            (27, 0x0000_0208_0a00_030c),
        ] {
            let t = Task {
                seed,
                ..task(Feedback::SevenInEight)
            };
            let mut read = 0u64;
            for k in 0..64u32 {
                read |= u64::from(t.misleading_at(u64::from(k))).wrapping_shl(k);
            }
            assert_eq!(read, mask, "seed {seed}: bits 48 to 50 of mix64(seed ^ k)");
            assert_eq!(read.count_ones(), 8, "seed {seed}");
        }
    }

    /// The coin's share and what it does not read (ADR-0148), at the learning harness's seed
    /// over 4 096 trials, every count the apart oracle's: 513 misleading, one in eight, and
    /// among the trials of each stimulus and each side of the shuffled control's coin between a
    /// tenth and a sixth, so the coin tells neither draw.
    #[test]
    fn the_misleading_coin_is_one_trial_in_eight_whatever_the_other_two_draws() {
        let t = Task {
            seed: 27,
            ..task(Feedback::SevenInEight)
        };
        let mut all = [[0u32; 2]; 2];
        let mut misleading = [[0u32; 2]; 2];
        for trial in 0..4096u64 {
            let s = usize::from(t.stimulus_at(trial));
            let c = usize::from(t.coin_at(trial));
            all[s][c] += 1;
            misleading[s][c] += u32::from(t.misleading_at(trial));
            assert_eq!(
                t.misleading_at(trial),
                t.misleading_at(trial),
                "the same draw twice"
            );
        }
        assert_eq!(all, [[1039, 991], [1051, 1015]], "[stimulus][coin]");
        assert_eq!(misleading, [[150, 122], [118, 123]], "[stimulus][coin]");
        assert_eq!(misleading.iter().flatten().sum::<u32>(), 513);
        for (m, n) in misleading.iter().flatten().zip(all.iter().flatten()) {
            assert!(m * 10 >= *n && m * 6 <= *n, "{m} of {n}");
        }
        let other = Task { seed: 1, ..t };
        let differ = (0..4096u64)
            .filter(|&k| other.misleading_at(k) != t.misleading_at(k))
            .count();
        assert!(differ > 512, "another seed, another coin: {differ}");
    }

    /// A reward right seven times in eight (ADR-0147, ADR-0148): over the first eight trials at
    /// the unit tests' seed — the coin misleading at trials 0 and 2, one of each stimulus — and
    /// for each outcome, the answer's readout cued (correct), the other's (wrong) and none (a
    /// tie), the trial is the trial under the answer's feedback on a twin engine in every field
    /// where the coin is not misleading, and where it is, in every field but the reward and the
    /// signal, which are the opposite: a correct selection punished, a wrong one and a tie
    /// rewarded. What the trial records as correct is the selection, and the addressed set, the
    /// units and the train are the outcome's, whatever the reward. Under the engine's critic
    /// the misleading reward is what the engine takes its value from.
    #[test]
    fn the_reward_s_sign_is_the_outcome_s_exactly_where_the_coin_is_not_misleading() {
        let probe = task(Feedback::SevenInEight);
        let mut seen = [[false; 3]; 2];
        let mut stimuli = [[false; 2]; 2];
        for trial in 0..8u64 {
            let misleading = probe.misleading_at(trial);
            assert_eq!(misleading, trial == 0 || trial == 2, "trial {trial}");
            let stimulus = probe.stimulus_at(trial);
            stimuli[usize::from(misleading)][usize::from(stimulus)] = true;
            let answer = probe.answer(stimulus);
            for (kind, cued) in [Some(answer), Some(answer ^ 1), None]
                .into_iter()
                .enumerate()
            {
                let mut exec = network(1, ONE / 2);
                let mut twin = network(1, ONE / 2);
                let mut t = task(Feedback::SevenInEight);
                let mut plain = task(Feedback::Answer);
                t.delivery = Delivery::Addressed;
                plain.delivery = Delivery::Addressed;
                if let Some(r) = cued {
                    cue(&exec, t.readout.sets()[usize::from(r)]);
                    cue(&twin, t.readout.sets()[usize::from(r)]);
                }
                let got = t.trial(&mut exec, trial).unwrap();
                let truth = plain.trial(&mut twin, trial).unwrap();
                let case = format!("trial {trial} cued {cued:?}");
                assert_eq!(got.selection, cued, "{case}");
                assert_eq!(got.correct, kind == 0, "{case}: correct is the selection");
                let sign = if got.correct != misleading {
                    REWARD
                } else {
                    -REWARD
                };
                assert_eq!((got.reward_q16, got.signal_q16), (sign, sign), "{case}");
                assert_eq!(
                    truth.reward_q16,
                    if truth.correct { REWARD } else { -REWARD },
                    "{case}"
                );
                let expected = if misleading {
                    Outcome {
                        reward_q16: -truth.reward_q16,
                        signal_q16: -truth.signal_q16,
                        ..truth
                    }
                } else {
                    truth
                };
                assert_eq!(got, expected, "{case}");
                assert_eq!(
                    (exec.addressed_counts(), fields(&exec), exec.delivered()),
                    (twin.addressed_counts(), fields(&twin), twin.delivered()),
                    "{case}: the address and the units are the outcome's"
                );
                assert_eq!(exec.train().to_vec(), twin.train().to_vec(), "{case}");
                assert_eq!(
                    exec.modulator().dopamine_rpe,
                    if misleading {
                        -twin.modulator().dopamine_rpe
                    } else {
                        twin.modulator().dopamine_rpe
                    },
                    "{case}"
                );
                seen[usize::from(misleading)][kind] = true;
            }
        }
        assert_eq!(seen, [[true; 3]; 2], "each outcome, misleading and not");
        assert_eq!(stimuli, [[true; 2]; 2], "each stimulus, misleading and not");
        // Under the engine's critic (ADR-0131), at weights of zero: the value zero, and the
        // error the misleading reward itself.
        for (trial, cued) in [(0u64, true), (0, false), (1, true), (1, false)] {
            let mut exec = valued(ONE / 2);
            let mut t = task(Feedback::SevenInEight);
            let answer = t.answer(t.stimulus_at(trial));
            if cued {
                cue(&exec, t.readout.sets()[usize::from(answer)]);
            }
            let got = t.trial(&mut exec, trial).unwrap();
            assert_eq!(got.correct, cued, "trial {trial}");
            let sign = if cued != t.misleading_at(trial) {
                REWARD
            } else {
                -REWARD
            };
            assert_eq!(
                (got.reward_q16, got.value_q16, exec.prediction()),
                (
                    sign,
                    Some(0),
                    Some(Prediction {
                        value_q16: 0,
                        error_q16: sign,
                        received_q16: sign
                    })
                ),
                "trial {trial} cued {cued}"
            );
        }
        // A reward of no magnitude and a reward at the ceiling are refused as under the
        // answer's feedback.
        let mut none = task(Feedback::SevenInEight);
        none.reward_q16 = 0;
        assert_eq!(none.check(&network(1, ONE / 2)), Err(TaskError::NoReward));
        assert_eq!(
            task(Feedback::SevenInEight).check(&network(1, ONE)),
            Err(TaskError::RewardAtCeiling)
        );
    }

    /// Under the three feedbacks before it a trial is the trial it was (ADR-0148): `trial`
    /// against the trial as ADR-0144 left it, written out here as the oracle with the three
    /// feedbacks it had — the answer, the coin read as bit 32 of the draw, and the reward
    /// withheld — on twin engines over eight trials under a drive, a cancel and a sub-window,
    /// a readout cued before some of them: once under the task's critic with the addressed
    /// delivery, once under the engine's critic and its window with the drawn one. The same
    /// outcome, the same units' fields, the same train, the same messages drained, the same
    /// addressed set, the same signal, the same counts and the same expectations after every
    /// trial.
    #[test]
    fn a_trial_under_the_three_feedbacks_before_is_the_trial_it_was() {
        fn before(t: &mut Task, exec: &mut Executor<8>, trial: u64) -> Outcome {
            t.check(exec).unwrap();
            assert_eq!(t.hold, None, "the oracle is of a trial with no hold");
            let start = exec.ticks();
            let stimulus = (mix64(t.seed ^ trial) & 1) as u8;
            let inject = exec.injector();
            let presented = t.stimuli[usize::from(stimulus)];
            presented.inject(&inject).unwrap();
            for k in 0..t.ticks {
                presented.cancel_at(&inject, k).unwrap();
                t.drive.step(&inject, exec.ticks()).unwrap();
                exec.tick();
            }
            let counts = t.readout.count_window(
                exec.train(),
                (start as u32).wrapping_add(t.window.from),
                t.window.ticks,
            );
            let selection = t.readout.select(counts);
            let correct = selection == Some(t.answer(stimulus));
            let selected = selection.map(|r| t.readout.sets()[usize::from(r)]);
            match t.delivery {
                Delivery::Global => exec.address_all(),
                Delivery::Addressed => exec
                    .address(
                        presented.set.units(),
                        selected.iter().flat_map(|set| set.units()),
                    )
                    .unwrap(),
                Delivery::Drawn => exec
                    .address_drawn(selected.iter().flat_map(|set| set.units()))
                    .unwrap(),
                Delivery::Released => unreachable!("the test runs the two deliveries above"),
            }
            let positive = match t.feedback {
                Feedback::Answer => correct,
                Feedback::Shuffled => (mix64(t.seed ^ trial) >> 32) & 1 == 1,
                Feedback::Withheld => {
                    return Outcome {
                        trial,
                        stimulus,
                        counts,
                        selection,
                        correct,
                        reward_q16: 0,
                        signal_q16: exec.modulator().dopamine_rpe,
                        expected_q16: t.critic.map(|c| [c.expected_q16[usize::from(stimulus)]; 2]),
                        value_q16: None,
                        held: 0,
                        drawn: false,
                    };
                }
                Feedback::SevenInEight => unreachable!("the oracle is of the trial before it"),
            };
            let outcome_q16 = if positive {
                t.reward_q16
            } else {
                t.reward_q16.saturating_neg()
            };
            let (delivered_q16, expected_q16) = match t.critic.as_mut() {
                Some(critic) => {
                    let (error, before, after) = critic.predict(stimulus, outcome_q16);
                    (error, Some([before, after]))
                }
                None => (outcome_q16, None),
            };
            let signal_q16 = exec.reward(delivered_q16);
            let (reward_q16, value_q16) = match exec.prediction() {
                Some(prediction) => (prediction.error_q16, Some(prediction.value_q16)),
                None => (delivered_q16, None),
            };
            Outcome {
                trial,
                stimulus,
                counts,
                selection,
                correct,
                reward_q16,
                signal_q16,
                expected_q16,
                value_q16,
                held: 0,
                drawn: false,
            }
        }
        for feedback in [Feedback::Answer, Feedback::Shuffled, Feedback::Withheld] {
            for engine_s in [false, true] {
                let mut t = task(feedback);
                t.window = Window { from: 4, ticks: 20 };
                t.drive = Drive {
                    every: 1,
                    messages: 2,
                    efficacy_q16: 0x2000,
                    units: 16,
                    seed: 5,
                };
                for stimulus in t.stimuli.iter_mut() {
                    stimulus.cancel = Some(Cancel {
                        offset: 30,
                        ticks: 3,
                        messages: 1,
                        efficacy_q16: -ONE,
                    });
                }
                let (mut now, mut then) = if engine_s {
                    t.delivery = Delivery::Drawn;
                    (windowed_network(40), windowed_network(40))
                } else {
                    t.delivery = Delivery::Addressed;
                    t.critic = Some(Critic::new(5));
                    (network(2, ONE / 2), network(2, ONE / 2))
                };
                let mut oracle = t;
                let case = format!("{feedback:?}, the engine's critic {engine_s}");
                let mut rewards = Vec::new();
                for trial in 0..8u64 {
                    let cued = [None, Some(0), None, Some(1), Some(1), None, Some(0), None];
                    if let Some(r) = cued[trial as usize] {
                        cue(&now, t.readout.sets()[r]);
                        cue(&then, t.readout.sets()[r]);
                    }
                    let outcome = t.trial(&mut now, trial).unwrap();
                    assert_eq!(
                        outcome,
                        before(&mut oracle, &mut then, trial),
                        "{case} trial {trial}"
                    );
                    assert_eq!(fields(&now), fields(&then), "{case} trial {trial}");
                    assert_eq!(now.train().to_vec(), then.train().to_vec(), "{case}");
                    assert_eq!(
                        (
                            now.delivered(),
                            now.addressed_counts(),
                            now.modulator().dopamine_rpe,
                            now.features().to_vec(),
                            now.prediction(),
                            now.units()
                                .iter()
                                .map(|u| u.value_weight)
                                .collect::<Vec<i16>>(),
                        ),
                        (
                            then.delivered(),
                            then.addressed_counts(),
                            then.modulator().dopamine_rpe,
                            then.features().to_vec(),
                            then.prediction(),
                            then.units()
                                .iter()
                                .map(|u| u.value_weight)
                                .collect::<Vec<i16>>(),
                        ),
                        "{case} trial {trial}"
                    );
                    assert_eq!(
                        (0..16).map(|u| now.is_source(u)).collect::<Vec<bool>>(),
                        (0..16).map(|u| then.is_source(u)).collect::<Vec<bool>>(),
                        "{case} trial {trial}"
                    );
                    assert_eq!((t.critic, t.readout), (oracle.critic, oracle.readout));
                    rewards.push(outcome.reward_q16.signum());
                }
                match feedback {
                    Feedback::Withheld => assert!(rewards.iter().all(|&r| r == 0), "{case}"),
                    _ => assert!(
                        rewards.contains(&1) && rewards.contains(&-1),
                        "{case}: a reward of each sign among {rewards:?}"
                    ),
                }
            }
        }
    }

    // --------------------------------------------- three answers (ADR-0151, ADR-0152)

    /// Twenty-four armed units without synapses, as [`network`], for a readout of three:
    /// stimuli at `[0, 4)` and `[8, 12)`, readouts at `[4, 8)`, `[12, 16)` and `[16, 20)`, and
    /// four units in no set; with the engine's critic and its window when `window` is given.
    fn wide(workers: usize, baseline_q16: i32, window: Option<u16>) -> Executor<8> {
        let mut exec = Executor::<8>::new(Config {
            workers,
            units: 24,
            injector_capacity: 64,
            train_capacity: spikes_per_unit(TICKS).saturating_mul(24) as usize,
            modulation_baseline_q16: baseline_q16,
            critic: window.map(|_| ValueCritic { shift: 9, scale: 2 }),
            critic_window_ticks: window.unwrap_or(0),
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

    /// [`task`] with a third readout at `[16, 20)`, on [`wide`]'s network.
    fn task3(feedback: Feedback) -> Task<3> {
        Task {
            stimuli: [stimulus(0), stimulus(8)],
            readout: Readout::new([set(4, 4), set(12, 4), set(16, 4)]),
            drive: Drive {
                every: 0,
                messages: 0,
                efficacy_q16: 0,
                units: 24,
                seed: 0,
            },
            ticks: TICKS,
            window: Window::whole(TICKS),
            seed: 0,
            reward_q16: REWARD,
            answers: [0, 1],
            feedback,
            delivery: Delivery::Global,
            critic: None,
            hold: None,
            exploration: Exploration::Unset,
        }
    }

    /// What a selection leaves on a channel: its direct, indirect and hyperdirect drives, its
    /// net output and its flag.
    fn gate(c: &BasalGangliaChannelState) -> (i32, i32, i32, i32, u32) {
        (
            c.striatal_d1_drive,
            c.striatal_d2_drive,
            c.stn_hyperdirect_drive,
            c.gpi_snr_inhibition,
            c.selected_flag,
        )
    }

    /// The selection among three (ADR-0151, ADR-0152): the channel whose count is above both
    /// others, each channel's indirect drive the largest of the other two and not their sum,
    /// and none where the largest is shared, by two or by all; among one channel, any count
    /// above zero; among none, nothing; and among the most a readout holds, the last.
    #[test]
    fn a_selection_among_three_is_the_largest_count_alone_and_none_where_it_is_shared() {
        let mut r = Readout::new([set(4, 4), set(12, 4), set(16, 4)]);
        assert_eq!(r.channels().each_ref().map(|c| c.channel_id), [0, 1, 2]);
        assert_eq!(r.select([0, 0, 0]), None);
        assert_eq!(
            r.channels().each_ref().map(gate),
            [(0, 0, 0, 0, 0); 3],
            "every output zero: nothing released"
        );
        assert_eq!(r.select([5, 3, 4]), Some(0));
        assert_eq!(
            r.channels().each_ref().map(gate),
            [
                (5 * ONE, 4 * ONE, 0, -ONE, 1),
                (3 * ONE, 5 * ONE, 0, 2 * ONE, 0),
                (4 * ONE, 5 * ONE, 0, ONE, 0),
            ],
            "each channel against the largest of the other two"
        );
        assert_eq!(r.select([3, 5, 4]), Some(1));
        assert_eq!(r.select([3, 4, 5]), Some(2));
        assert_eq!(
            r.channels().each_ref().map(gate),
            [
                (3 * ONE, 5 * ONE, 0, 2 * ONE, 0),
                (4 * ONE, 5 * ONE, 0, ONE, 0),
                (5 * ONE, 4 * ONE, 0, -ONE, 1),
            ]
        );
        assert_eq!(r.select([1, 0, 0]), Some(0), "one spike above the others");
        assert_eq!(r.select([0, 1, 0]), Some(1));
        assert_eq!(r.select([0, 0, 1]), Some(2));
        assert_eq!(
            r.select([10, 6, 6]),
            Some(0),
            "above each of the others, though below the two together"
        );
        assert_eq!(r.select([6, 10, 6]), Some(1));
        assert_eq!(r.select([6, 6, 10]), Some(2));
        // The largest shared by two: nothing selected, and the third's output above zero.
        assert_eq!(r.select([4, 4, 1]), None);
        assert_eq!(
            r.channels().each_ref().map(gate),
            [
                (4 * ONE, 4 * ONE, 0, 0, 0),
                (4 * ONE, 4 * ONE, 0, 0, 0),
                (ONE, 4 * ONE, 0, 3 * ONE, 0),
            ]
        );
        assert_eq!(r.select([4, 1, 4]), None);
        assert_eq!(r.select([1, 4, 4]), None);
        assert_eq!(r.select([4, 4, 4]), None, "shared by all");
        // At the width, as between two.
        assert_eq!(r.select([32_767, 32_766, 32_766]), Some(0));
        assert_eq!(r.select([32_766, 32_766, 32_767]), Some(2));
        assert_eq!(r.select([5, 32_768, 32_767]), Some(1));
        assert_eq!(
            r.select([32_769, 5, 32_768]),
            None,
            "beyond the width two counts tie: the mis-read `check` refuses"
        );
        // One channel has no other: its indirect drive is zero, and any count selects it.
        let mut one = Readout::new([set(4, 4)]);
        assert_eq!(one.select([0]), None);
        assert_eq!(one.select([1]), Some(0));
        assert_eq!(gate(&one.channels()[0]), (ONE, 0, 0, -ONE, 1));
        // No channel selects nothing.
        let mut none: Readout<0> = Readout::new([]);
        assert_eq!(none.select([]), None);
        assert_eq!(none.count_window(&[(0, 4)], 0, 8), []);
        // The most channels a readout holds: their ids are their indices, and the last is
        // selected where its count alone is the largest.
        let mut many = Readout::new([set(0, 1); MAX_CHANNELS]);
        assert_eq!(MAX_CHANNELS, 64);
        for (k, channel) in many.channels().iter().enumerate() {
            assert_eq!(channel.channel_id as usize, k);
        }
        let mut counts = [1u32; MAX_CHANNELS];
        assert_eq!(many.select(counts), None);
        counts[63] = 2;
        assert_eq!(many.select(counts), Some(63));
        counts[0] = 2;
        assert_eq!(many.select(counts), None, "the first shares it");
        counts[31] = 3;
        assert_eq!(many.select(counts), Some(31));
    }

    /// A trial among three counts each set and selects by the rule: on [`wide`]'s network a
    /// cued readout fires its four units, the third among them, and the outcome is correct
    /// where the selection is the stimulus's answer, any readout's index.
    #[test]
    fn a_trial_among_three_selects_the_cued_readout_and_judges_it_by_the_answer() {
        let mut exec = wide(2, ONE / 2, None);
        let mut t = task3(Feedback::Answer);
        assert_eq!(t.check(&exec), Ok(()));
        let tie = t.trial(&mut exec, 0).unwrap();
        assert_eq!((tie.counts, tie.selection), ([0, 0, 0], None));
        assert!(!tie.correct);
        assert_eq!(tie.reward_q16, -REWARD, "a tie is an error, punished");
        for (trial, cued, answers) in [
            (1u64, 2usize, [2u8, 2]),
            (2, 2, [0, 1]),
            (3, 0, [0, 0]),
            (4, 1, [2, 0]),
            (5, 1, [1, 1]),
        ] {
            exec.run(400);
            cue(&exec, t.readout.sets()[cued]);
            t.answers = answers;
            let outcome = t.trial(&mut exec, trial).unwrap();
            let mut counts = [0u32; 3];
            counts[cued] = 4;
            assert_eq!(outcome.counts, counts, "trial {trial}");
            assert_eq!(outcome.selection, Some(cued as u8), "trial {trial}");
            let answer = answers[usize::from(outcome.stimulus)];
            assert_eq!(t.answer(outcome.stimulus), answer);
            assert_eq!(
                outcome.correct,
                usize::from(answer) == cued,
                "trial {trial}"
            );
            assert_eq!(
                outcome.reward_q16.signum(),
                if outcome.correct { 1 } else { -1 },
                "trial {trial}"
            );
        }
        // Two cued, the third silent: the largest is shared and nothing is selected.
        exec.run(400);
        cue(&exec, t.readout.sets()[0]);
        cue(&exec, t.readout.sets()[2]);
        let shared = t.trial(&mut exec, 6).unwrap();
        assert_eq!((shared.counts, shared.selection), ([4, 0, 4], None));
        assert!(!shared.correct);
    }

    /// The mapping's flip (ADR-0151, ADR-0152): every answer to the next readout and the last
    /// to the first. Among three the two arms' four mappings come round in three flips; among
    /// two each answer moves to the other, the flag negated; among one nothing moves; and an
    /// answer beyond the readouts moves to the first.
    #[test]
    fn a_flip_moves_every_answer_to_the_next_readout_and_the_last_to_the_first() {
        let mut three = task3(Feedback::Answer);
        let mut seen = vec![three.answers];
        for _ in 0..3 {
            three.flip();
            seen.push(three.answers);
        }
        assert_eq!(seen, [[0, 1], [1, 2], [2, 0], [0, 1]]);
        three.answers = [1, 0];
        let mut seen = vec![three.answers];
        for _ in 0..3 {
            three.flip();
            seen.push(three.answers);
        }
        assert_eq!(seen, [[1, 0], [2, 1], [0, 2], [1, 0]]);
        three.answers = [2, 2];
        three.flip();
        assert_eq!(three.answers, [0, 0], "the last to the first, both");
        three.answers = [3, u8::MAX];
        three.flip();
        assert_eq!(three.answers, [0, 0], "beyond the readouts: to the first");
        // Among two: the flag negated, and negated back.
        let mut two = task(Feedback::Answer);
        assert_eq!(two.answers, [0, 1]);
        two.flip();
        assert_eq!(two.answers, [1, 0]);
        assert_eq!((two.answer(0), two.answer(1)), (1, 0));
        two.flip();
        assert_eq!(two.answers, [0, 1]);
        two.answers = [1, 1];
        two.flip();
        assert_eq!(two.answers, [0, 0]);
        two.answers = [2, 0];
        two.flip();
        assert_eq!(
            two.answers,
            [0, 1],
            "an answer `check` refuses moves to the first"
        );
        // Among one there is no next.
        let mut one = Task {
            stimuli: [stimulus(0), stimulus(8)],
            readout: Readout::new([set(4, 4)]),
            drive: two.drive,
            ticks: TICKS,
            window: Window::whole(TICKS),
            seed: 0,
            reward_q16: REWARD,
            answers: [0, 0],
            feedback: Feedback::Answer,
            delivery: Delivery::Global,
            critic: None,
            hold: None,
            exploration: Exploration::Unset,
        };
        assert_eq!(one.check(&network(1, ONE / 2)), Ok(()));
        one.flip();
        assert_eq!(one.answers, [0, 0]);
    }

    /// Every refusal of `check` with three readouts, by name (ADR-0152): each of the task's
    /// refusals met by a task of three, the third readout's set read as the other two are —
    /// empty, malformed, outside the arena, sharing a unit with each other set, beyond the
    /// width — and the new one, an answer that names no readout among the three, either
    /// stimulus's, at the first index past them; among two, the first index past two.
    #[test]
    fn every_refusal_is_named_with_three_readouts() {
        type Change = fn(&mut Task<3>);
        const CANCEL: Cancel = Cancel {
            offset: 1,
            ticks: 1,
            messages: 1,
            efficacy_q16: -1,
        };
        const HOLD: Hold = Hold {
            until: 48,
            every: 8,
            messages: 1,
            efficacy_q16: -1,
        };
        fn third(set: Set) -> Readout<3> {
            Readout::new([Set::contiguous(4, 4), Set::contiguous(12, 4), set])
        }
        fn early(t: &mut Task<3>) {
            t.window = Window { from: 0, ticks: 16 };
        }
        let exec = wide(1, ONE / 2, None);
        let ok = task3(Feedback::Answer);
        assert_eq!(ok.check(&exec), Ok(()));
        let cases: [(Change, TaskError); 33] = [
            (|t| t.stimuli[1].messages = 0, TaskError::NoStimulus),
            (|t| t.readout = third(set(16, 0)), TaskError::EmptySet),
            (
                |t| {
                    t.readout = third(Set {
                        period: 0,
                        ..set(16, 4)
                    })
                },
                TaskError::MalformedSet,
            ),
            (
                |t| {
                    t.readout = third(Set {
                        period: 2,
                        mask: 0b100,
                        ..set(16, 2)
                    })
                },
                TaskError::MalformedSet,
            ),
            (
                |t| t.readout = third(set(21, 4)),
                TaskError::SetOutsideArena,
            ),
            (|t| t.readout = third(set(3, 1)), TaskError::SetsOverlap),
            (|t| t.readout = third(set(11, 1)), TaskError::SetsOverlap),
            (|t| t.readout = third(set(7, 1)), TaskError::SetsOverlap),
            (|t| t.readout = third(set(15, 1)), TaskError::SetsOverlap),
            (|t| t.ticks = 0, TaskError::NoTicks),
            (
                |t| {
                    t.stimuli[0].cancel = Some(Cancel {
                        messages: 0,
                        ..CANCEL
                    })
                },
                TaskError::EmptyCancel,
            ),
            (
                |t| {
                    t.stimuli[1].cancel = Some(Cancel {
                        offset: 0,
                        ..CANCEL
                    })
                },
                TaskError::CancelAtInjection,
            ),
            (
                |t| {
                    t.stimuli[0].cancel = Some(Cancel {
                        offset: TICKS - 1,
                        ..CANCEL
                    })
                },
                TaskError::CancelOutsideTrial,
            ),
            (
                |t| {
                    t.stimuli[0].cancel = Some(Cancel {
                        efficacy_q16: 0,
                        ..CANCEL
                    })
                },
                TaskError::CancelNotNegative,
            ),
            (
                |t| t.window = Window { from: 0, ticks: 0 },
                TaskError::EmptyWindow,
            ),
            (
                |t| {
                    t.window = Window {
                        from: TICKS - 8,
                        ticks: 9,
                    }
                },
                TaskError::WindowOutsideTrial,
            ),
            (
                |t| {
                    early(t);
                    t.hold = Some(Hold {
                        messages: 0,
                        ..HOLD
                    })
                },
                TaskError::EmptyHold,
            ),
            (
                |t| {
                    early(t);
                    t.hold = Some(Hold {
                        efficacy_q16: 0,
                        ..HOLD
                    })
                },
                TaskError::HoldNotNegative,
            ),
            (
                |t| {
                    early(t);
                    t.hold = Some(Hold { until: 16, ..HOLD })
                },
                TaskError::HoldBeforeClose,
            ),
            (
                |t| {
                    early(t);
                    t.hold = Some(Hold {
                        until: TICKS,
                        ..HOLD
                    })
                },
                TaskError::HoldOutsideTrial,
            ),
            (
                |t| t.ticks = 2 * MIN_INTERVAL_TICKS + 1,
                TaskError::TrainTooSmall,
            ),
            (|t| t.reward_q16 = -1, TaskError::NegativeReward),
            (|t| t.reward_q16 = 0, TaskError::NoReward),
            (
                |t| {
                    t.critic = Some(Critic {
                        expected_q16: [0, REWARD + 1],
                        shift: 5,
                    })
                },
                TaskError::ExpectationBeyondReward,
            ),
            (
                |t| t.delivery = Delivery::Drawn,
                TaskError::Address(AddressError::NoCritic),
            ),
            (
                |t| t.delivery = Delivery::Released,
                TaskError::Address(AddressError::NoCritic),
            ),
            (|t| t.answers = [3, 0], TaskError::AnswerOutsideReadout),
            (|t| t.answers = [0, 3], TaskError::AnswerOutsideReadout),
            (|t| t.answers = [3, 3], TaskError::AnswerOutsideReadout),
            (|t| t.answers = [2, 4], TaskError::AnswerOutsideReadout),
            (
                |t| t.answers = [u8::MAX, 2],
                TaskError::AnswerOutsideReadout,
            ),
            (
                // The sets are read before the answers, and the answers before the trial's
                // length.
                |t| {
                    t.answers = [0, 3];
                    t.readout = third(set(16, 0))
                },
                TaskError::EmptySet,
            ),
            (
                |t| {
                    t.answers = [0, 3];
                    t.ticks = 0
                },
                TaskError::AnswerOutsideReadout,
            ),
        ];
        for (k, (change, refusal)) in cases.iter().enumerate() {
            let mut t = ok;
            change(&mut t);
            assert_eq!(t.check(&exec), Err(*refusal), "case {k}");
            let mut fresh = wide(1, ONE / 2, None);
            assert_eq!(t.trial(&mut fresh, 0), Err(*refusal), "case {k}");
            assert_eq!(
                (fresh.ticks(), fresh.delivered()),
                (0, 0),
                "case {k}: no tick ran"
            );
            assert!(fresh.is_quiescent(), "case {k}: nothing injected");
        }
        // Each cancel and each hold above is refused for the one field changed.
        let mut t = ok;
        t.stimuli[0].cancel = Some(CANCEL);
        early(&mut t);
        t.hold = Some(HOLD);
        assert_eq!(t.check(&exec), Ok(()));
        // Every answer that names a readout among the three runs, the same for both stimuli
        // too: a mapping is the caller's.
        for a in 0..3u8 {
            for b in 0..3u8 {
                let mut t = ok;
                t.answers = [a, b];
                assert_eq!(t.check(&exec), Ok(()), "{a} {b}");
                assert_eq!((t.answer(0), t.answer(1)), (a, b));
            }
        }
        // The refusals that need another engine.
        assert_eq!(
            ok.check(&wide(1, ONE, None)),
            Err(TaskError::RewardAtCeiling)
        );
        let valued = wide(1, ONE / 2, Some(30));
        assert_eq!(
            Task {
                critic: Some(Critic::new(5)),
                ..ok
            }
            .check(&valued),
            Err(TaskError::TwoCritics)
        );
        let mut drawn = ok;
        drawn.delivery = Delivery::Drawn;
        assert_eq!(drawn.check(&valued), Ok(()));
        assert_eq!(
            drawn.check(&wide(1, ONE / 2, Some(0))),
            Err(TaskError::Address(AddressError::NoWindow))
        );
        let big = Executor::<8>::new(Config {
            units: 1 << 16,
            injector_capacity: 64,
            train_capacity: 1 << 22,
            modulation_baseline_q16: ONE / 2,
            ..Config::default()
        })
        .unwrap();
        let mut t = ok;
        t.ticks = 1;
        t.window = Window::whole(1);
        t.readout = third(set(16, 32_768));
        assert_eq!(
            t.check(&big),
            Err(TaskError::CountBeyondWidth),
            "the third readout's count is bounded as the other two's"
        );
        t.readout = third(set(16, 32_767));
        assert_eq!(t.check(&big), Ok(()), "one fewer fits");
        // Among two readouts the first index past them is two.
        let narrow = network(1, ONE / 2);
        let two = task(Feedback::Answer);
        for (answers, refused) in [
            ([0u8, 1], false),
            ([1, 0], false),
            ([0, 0], false),
            ([1, 1], false),
            ([2, 1], true),
            ([0, 2], true),
            ([u8::MAX, 0], true),
        ] {
            let t = Task { answers, ..two };
            assert_eq!(
                t.check(&narrow),
                if refused {
                    Err(TaskError::AnswerOutsideReadout)
                } else {
                    Ok(())
                },
                "{answers:?}"
            );
        }
    }

    /// Under each delivery, what a trial among three addresses (ADR-0152): the selected
    /// readout's units as the targets when the third channel is selected, as when another is;
    /// no target where the largest count is shared, by two with the third silent or by all;
    /// under the released delivery every unit a target whatever was selected, and under the
    /// global one every unit on both sides.
    #[test]
    fn each_delivery_addresses_the_third_readout_when_it_is_selected_and_none_at_a_tie() {
        let addressed = |exec: &Executor<8>| -> (Vec<u32>, Vec<u32>) {
            (
                (0..24).filter(|&u| exec.is_source(u)).collect(),
                (0..24).filter(|&u| exec.is_target(u)).collect(),
            )
        };
        let stimulus_units = |t: &Task<3>, trial: u64| -> Vec<u32> {
            t.stimuli[usize::from(t.stimulus_at(trial))]
                .set
                .units()
                .collect()
        };
        // The trials: which readouts are cued, and the selection they leave.
        let trials: [(&[usize], Option<u8>); 6] = [
            (&[], None),
            (&[2], Some(2)),
            (&[0, 2], None),
            (&[1], Some(1)),
            (&[0, 1, 2], None),
            (&[0], Some(0)),
        ];
        let units_of = |r: u8| -> Vec<u32> {
            let first = [4u32, 12, 16][usize::from(r)];
            (first..first.saturating_add(4)).collect()
        };
        // The addressed delivery: the presented stimulus onto the selected readout.
        let mut exec = wide(2, ONE / 2, None);
        let mut t = task3(Feedback::Answer);
        t.delivery = Delivery::Addressed;
        assert_eq!(exec.addressed_counts(), (24, 24), "before the first trial");
        for (trial, &(cued, selection)) in trials.iter().enumerate() {
            exec.run(400);
            for &r in cued {
                cue(&exec, t.readout.sets()[r]);
            }
            let outcome = t.trial(&mut exec, trial as u64).unwrap();
            assert_eq!(outcome.selection, selection, "addressed, trial {trial}");
            assert_eq!(
                addressed(&exec),
                (
                    stimulus_units(&t, trial as u64),
                    selection.map_or(vec![], units_of)
                ),
                "addressed, trial {trial}"
            );
        }
        // The global delivery: every unit, both sides, whatever was selected.
        t.delivery = Delivery::Global;
        for (trial, &(cued, selection)) in trials.iter().enumerate() {
            exec.run(400);
            for &r in cued {
                cue(&exec, t.readout.sets()[r]);
            }
            let outcome = t.trial(&mut exec, trial as u64).unwrap();
            assert_eq!(outcome.selection, selection, "global, trial {trial}");
            assert_eq!(exec.addressed_counts(), (24, 24), "global, trial {trial}");
        }
        // The drawn delivery and the released one: the sources the units the window counted —
        // the first trial's volley, then none, the window closed between the trials — and the
        // targets the selected readout's units, or every unit.
        for (delivery, released) in [(Delivery::Drawn, false), (Delivery::Released, true)] {
            let mut exec = wide(2, ONE / 2, Some(30));
            let mut t = task3(Feedback::Answer);
            t.delivery = delivery;
            for (trial, &(cued, selection)) in trials.iter().enumerate() {
                if trial > 0 {
                    exec.run(400);
                }
                for &r in cued {
                    cue(&exec, t.readout.sets()[r]);
                }
                let outcome = t.trial(&mut exec, trial as u64).unwrap();
                assert_eq!(outcome.selection, selection, "{delivery:?}, trial {trial}");
                let sources = if trial == 0 {
                    stimulus_units(&t, 0)
                } else {
                    vec![]
                };
                let targets = if released {
                    (0..24).collect()
                } else {
                    selection.map_or(vec![], units_of)
                };
                assert_eq!(
                    addressed(&exec),
                    (sources, targets),
                    "{delivery:?}, trial {trial}"
                );
            }
        }
    }

    /// The hold among three is the gate's reading (ADR-0144, ADR-0152): its messages go into
    /// every unit of each channel whose net output the selection left above zero — both of the
    /// channels not selected; the third alone where two share the largest count, though
    /// nothing was selected; and none where all three share it.
    #[test]
    fn the_gate_holds_every_channel_below_the_largest_count_among_three() {
        let hold = Hold {
            until: 56,
            every: 8,
            messages: 2,
            efficacy_q16: -ONE,
        };
        // Due before ticks 24, 32, 40 and 48: four times.
        let times = (0..TICKS).filter(|&k| hold.is_due(24, k)).count() as u32;
        assert_eq!(times, 4);
        for (cued, selection, channels_held) in [
            (&[0usize][..], Some(0u8), 2u32),
            (&[2], Some(2), 2),
            (&[0, 1], None, 1),
            (&[1, 2], None, 1),
            (&[0, 1, 2], None, 0),
            (&[], None, 0),
        ] {
            let mut exec = wide(1, 0, None);
            let mut t = task3(Feedback::Withheld);
            t.reward_q16 = 0;
            t.window = Window { from: 0, ticks: 24 };
            t.hold = Some(hold);
            for &r in cued {
                cue(&exec, t.readout.sets()[r]);
            }
            let outcome = t.trial(&mut exec, 0).unwrap();
            assert_eq!(outcome.selection, selection, "{cued:?}");
            let expected = times
                .saturating_mul(hold.messages)
                .saturating_mul(4)
                .saturating_mul(channels_held);
            assert_eq!(outcome.held, expected, "{cued:?}");
            let held: Vec<bool> = t
                .readout
                .channels()
                .iter()
                .map(|c| c.gpi_snr_inhibition > 0)
                .collect();
            assert_eq!(
                held.iter().filter(|&&h| h).count() as u32,
                channels_held,
                "{cued:?}"
            );
            assert!(
                cued.iter().all(|&r| !held[r]),
                "{cued:?}: a channel at the largest count is not held"
            );
            exec.tick();
            assert_eq!(
                exec.delivered(),
                8u64.saturating_add((cued.len() as u64).saturating_mul(8))
                    .saturating_add(u64::from(expected)),
                "{cued:?}: the ring drained the stimulus, the cues and the hold"
            );
        }
    }

    /// Under two readouts a trial is the trial it was and a flip the flip it was (ADR-0152):
    /// `trial` and `flip` against the task as ADR-0148 left it, written out here as the oracle
    /// — two channels composed by hand, each one's own count its direct drive and the other's
    /// its indirect, the mapping a flag negated at a flip, the counts a filter over the train
    /// and the hold a walk over the two channels — on twin engines over twelve trials with a
    /// flip before the fifth and the ninth, from either mapping, under a drive, a cancel and a
    /// sub-window, a readout cued before some of them: under each feedback, with the task's
    /// critic and the addressed delivery, the engine's and the drawn one, the engine's and the
    /// released one under a hold, and no critic under the global one. The same outcome, the
    /// same channels, the same answers, the same units' fields, the same train, the same
    /// messages drained, the same addressed set, the same signal, the same counts, the same
    /// expectations and the same weights after every trial.
    #[test]
    fn a_trial_under_two_readouts_is_the_trial_it_was_and_a_flip_the_flip_it_was() {
        /// What the oracle holds in the task's place: the two channels, the flag and the
        /// critic.
        struct Before {
            channels: [BasalGangliaChannelState; 2],
            mirrored: bool,
            critic: Option<Critic>,
        }
        /// The two counts over the window that opens at `opens`, and the selection from them
        /// as the two-channel composition made it.
        fn read(
            o: &mut Before,
            sets: &[Set; 2],
            exec: &mut Executor<8>,
            opens: u32,
            ticks: u32,
        ) -> ([u32; 2], Option<u8>) {
            let counts = [0usize, 1].map(|r| {
                exec.train()
                    .iter()
                    .filter(|&&(tick, unit)| {
                        tick.wrapping_sub(opens) < ticks && sets[r].contains(unit)
                    })
                    .count() as u32
            });
            let drives = [count_q16(counts[0]), count_q16(counts[1])];
            let mut selected = [false; 2];
            for (k, channel) in o.channels.iter_mut().enumerate() {
                let other = if k == 0 { 1 } else { 0 };
                channel.striatal_d1_drive = drives[k];
                channel.striatal_d2_drive = drives[other];
                channel.stn_hyperdirect_drive = 0;
                selected[k] = channel.compute_gating();
            }
            let selection = match selected {
                [true, false] => Some(0),
                [false, true] => Some(1),
                _ => None,
            };
            (counts, selection)
        }
        fn before(o: &mut Before, t: &Task, exec: &mut Executor<8>, trial: u64) -> Outcome {
            let start = exec.ticks();
            let draw = mix64(t.seed ^ trial);
            let stimulus = (draw & 1) as u8;
            let inject = exec.injector();
            let presented = t.stimuli[usize::from(stimulus)];
            presented.inject(&inject).unwrap();
            let sets = *t.readout.sets();
            let opens = (start as u32).wrapping_add(t.window.from);
            let close = t.window.from.wrapping_add(t.window.ticks);
            let mut decided: Option<([u32; 2], Option<u8>)> = None;
            let mut held = 0u32;
            for k in 0..t.ticks {
                presented.cancel_at(&inject, k).unwrap();
                if let Some(hold) = t.hold.filter(|hold| hold.is_due(close, k)) {
                    if decided.is_none() {
                        decided = Some(read(o, &sets, exec, opens, t.window.ticks));
                    }
                    let message = spike_message(hold.efficacy_q16, false);
                    for (set, channel) in sets.iter().zip(o.channels.iter()) {
                        if channel.gpi_snr_inhibition > 0 {
                            for unit in set.units() {
                                for _ in 0..hold.messages {
                                    inject.inject(unit, message).unwrap();
                                    held = held.saturating_add(1);
                                }
                            }
                        }
                    }
                }
                t.drive.step(&inject, exec.ticks()).unwrap();
                exec.tick();
            }
            let (counts, selection) = match decided {
                Some(made) => made,
                None => read(o, &sets, exec, opens, t.window.ticks),
            };
            let answer = if o.mirrored { stimulus ^ 1 } else { stimulus };
            let correct = selection == Some(answer);
            let selected = selection.map(|r| sets[usize::from(r)]);
            match t.delivery {
                Delivery::Global => exec.address_all(),
                Delivery::Addressed => exec
                    .address(
                        presented.set.units(),
                        selected.iter().flat_map(|set| set.units()),
                    )
                    .unwrap(),
                Delivery::Drawn => exec
                    .address_drawn(selected.iter().flat_map(|set| set.units()))
                    .unwrap(),
                Delivery::Released => exec.address_drawn(0..16u32).unwrap(),
            }
            let positive = match t.feedback {
                Feedback::Answer => correct,
                Feedback::Shuffled => (draw >> 32) & 1 == 1,
                Feedback::SevenInEight => correct != (draw & MISLEADING_BITS == 0),
                Feedback::Withheld => {
                    return Outcome {
                        trial,
                        stimulus,
                        counts,
                        selection,
                        correct,
                        reward_q16: 0,
                        signal_q16: exec.modulator().dopamine_rpe,
                        expected_q16: o.critic.map(|c| [c.expected_q16[usize::from(stimulus)]; 2]),
                        value_q16: None,
                        held,
                        drawn: false,
                    };
                }
            };
            let outcome_q16 = if positive {
                t.reward_q16
            } else {
                t.reward_q16.saturating_neg()
            };
            let (delivered_q16, expected_q16) = match o.critic.as_mut() {
                Some(critic) => {
                    let (error, before, after) = critic.predict(stimulus, outcome_q16);
                    (error, Some([before, after]))
                }
                None => (outcome_q16, None),
            };
            let signal_q16 = exec.reward(delivered_q16);
            let (reward_q16, value_q16) = match exec.prediction() {
                Some(prediction) => (prediction.error_q16, Some(prediction.value_q16)),
                None => (delivered_q16, None),
            };
            Outcome {
                trial,
                stimulus,
                counts,
                selection,
                correct,
                reward_q16,
                signal_q16,
                expected_q16,
                value_q16,
                held,
                drawn: false,
            }
        }
        #[derive(Clone, Copy, Debug)]
        enum Mode {
            TaskCritic,
            Drawn,
            ReleasedHeld,
            Global,
        }
        let mut selections = Vec::new();
        let mut held_in_all = 0u32;
        for feedback in [
            Feedback::Answer,
            Feedback::Shuffled,
            Feedback::Withheld,
            Feedback::SevenInEight,
        ] {
            for mode in [
                Mode::TaskCritic,
                Mode::Drawn,
                Mode::ReleasedHeld,
                Mode::Global,
            ] {
                for mirrored in [false, true] {
                    let mut t = task(feedback);
                    t.window = Window { from: 4, ticks: 20 };
                    t.drive = Drive {
                        every: 1,
                        messages: 2,
                        efficacy_q16: 0x2000,
                        units: 16,
                        seed: 5,
                    };
                    for stimulus in t.stimuli.iter_mut() {
                        stimulus.cancel = Some(Cancel {
                            offset: 30,
                            ticks: 3,
                            messages: 1,
                            efficacy_q16: -ONE,
                        });
                    }
                    t.answers = if mirrored { [1, 0] } else { [0, 1] };
                    let (mut now, mut then) = match mode {
                        Mode::TaskCritic => {
                            t.delivery = Delivery::Addressed;
                            t.critic = Some(Critic::new(5));
                            (network(2, ONE / 2), network(2, ONE / 2))
                        }
                        Mode::Drawn => {
                            t.delivery = Delivery::Drawn;
                            (windowed_network(40), windowed_network(40))
                        }
                        Mode::ReleasedHeld => {
                            t.delivery = Delivery::Released;
                            t.hold = Some(Hold {
                                until: 48,
                                every: 8,
                                messages: 1,
                                efficacy_q16: -ONE,
                            });
                            (windowed_network(40), windowed_network(40))
                        }
                        Mode::Global => {
                            t.delivery = Delivery::Global;
                            (network(2, ONE / 2), network(2, ONE / 2))
                        }
                    };
                    let mut oracle = Before {
                        channels: *t.readout.channels(),
                        mirrored,
                        critic: t.critic,
                    };
                    let case = format!("{feedback:?}, {mode:?}, mirrored {mirrored}");
                    for trial in 0..12u64 {
                        if trial == 4 || trial == 8 {
                            t.flip();
                            oracle.mirrored = !oracle.mirrored;
                        }
                        let cued = [
                            None,
                            Some(0),
                            None,
                            Some(1),
                            Some(1),
                            None,
                            Some(0),
                            None,
                            Some(0),
                            Some(1),
                            None,
                            Some(1),
                        ];
                        if let Some(r) = cued[trial as usize] {
                            cue(&now, t.readout.sets()[r]);
                            cue(&then, t.readout.sets()[r]);
                        }
                        let outcome = t.trial(&mut now, trial).unwrap();
                        assert_eq!(
                            outcome,
                            before(&mut oracle, &t, &mut then, trial),
                            "{case} trial {trial}"
                        );
                        assert_eq!(
                            (*t.readout.channels(), t.critic, t.answers),
                            (
                                oracle.channels,
                                oracle.critic,
                                if oracle.mirrored { [1, 0] } else { [0, 1] }
                            ),
                            "{case} trial {trial}"
                        );
                        assert_eq!(
                            (t.answer(0), t.answer(1)),
                            if oracle.mirrored { (1, 0) } else { (0, 1) }
                        );
                        assert_eq!(fields(&now), fields(&then), "{case} trial {trial}");
                        assert_eq!(now.train().to_vec(), then.train().to_vec(), "{case}");
                        assert_eq!(
                            (
                                now.delivered(),
                                now.addressed_counts(),
                                now.modulator().dopamine_rpe,
                                now.features().to_vec(),
                                now.prediction(),
                                now.units()
                                    .iter()
                                    .map(|u| u.value_weight)
                                    .collect::<Vec<i16>>(),
                            ),
                            (
                                then.delivered(),
                                then.addressed_counts(),
                                then.modulator().dopamine_rpe,
                                then.features().to_vec(),
                                then.prediction(),
                                then.units()
                                    .iter()
                                    .map(|u| u.value_weight)
                                    .collect::<Vec<i16>>(),
                            ),
                            "{case} trial {trial}"
                        );
                        assert_eq!(
                            (0..16)
                                .map(|u| (now.is_source(u), now.is_target(u)))
                                .collect::<Vec<(bool, bool)>>(),
                            (0..16)
                                .map(|u| (then.is_source(u), then.is_target(u)))
                                .collect::<Vec<(bool, bool)>>(),
                            "{case} trial {trial}"
                        );
                        selections.push((outcome.selection, outcome.correct));
                        held_in_all = held_in_all.saturating_add(outcome.held);
                    }
                }
            }
        }
        // The runs held each selection, a tie, a correct trial and a wrong one, and a hold
        // that delivered.
        for selection in [None, Some(0), Some(1)] {
            assert!(selections.iter().any(|&(s, _)| s == selection));
        }
        assert!(selections.iter().any(|&(_, c)| c) && selections.iter().any(|&(_, c)| !c));
        assert!(held_in_all > 0);
    }

    // ------------------------------------------- the exploration (ADR-0162, ADR-0163)

    /// A task of `N` readouts, one unit each from unit 16, with the exploration set, whose
    /// draws are read and whose trial is never run.
    fn drawing<const N: usize>(seed: u64) -> Task<N> {
        Task {
            stimuli: [stimulus(0), stimulus(8)],
            readout: Readout::new(core::array::from_fn(|k| {
                set(16u32.saturating_add(k as u32), 1)
            })),
            drive: Drive {
                every: 0,
                messages: 0,
                efficacy_q16: 0,
                units: 16,
                seed: 0,
            },
            ticks: TICKS,
            window: Window::whole(TICKS),
            seed,
            reward_q16: REWARD,
            answers: [0, 0],
            feedback: Feedback::Answer,
            delivery: Delivery::Global,
            critic: None,
            hold: None,
            exploration: Exploration::ValueGated,
        }
    }

    /// [`task`] with the exploration set.
    fn exploring(feedback: Feedback) -> Task {
        Task {
            exploration: Exploration::ValueGated,
            ..task(feedback)
        }
    }

    /// Every unit's value weight written `weight`, as an image that carries them would hold
    /// them: the engine's value of a trial is then `weight` times the spikes its critic
    /// counted, over four at the tests' scale.
    fn weigh(exec: &mut Executor<8>, weight: i16) {
        for unit in exec.units_mut() {
            unit.value_weight = weight;
        }
    }

    /// The exploration's two draws against the oracle written apart from the tree (ADR-0163):
    /// bits 16 to 31 of SplitMix64's finaliser of the seed and the trial's index, the coin,
    /// over the first eight trials, and bits 33 to 47 dealt among two, three and four channels
    /// as the floor of the draw times the channels over 32 768, over the first sixteen, at the
    /// unit tests' seed and at the learning harness's.
    #[test]
    fn the_exploration_s_coin_and_channel_of_the_first_trials_at_seeds_0_and_27() {
        assert_eq!(EXPLORATION_COIN_BITS, 0xFFFF << 16, "bits 16 to 31");
        assert_eq!(EXPLORATION_CHANNEL_BITS, 0x7FFF << 33, "bits 33 to 47");
        type Pins = (u64, [u16; 8], [u8; 16], [u8; 16], [u8; 16]);
        let pins: [Pins; 2] = [
            (
                0,
                [
                    0x7b1d, 0x8902, 0x1c97, 0xdb01, 0xe233, 0xa389, 0xadef, 0x5932,
                ],
                [1, 0, 0, 0, 1, 0, 1, 1, 0, 0, 0, 0, 1, 0, 1, 1],
                [1, 0, 0, 0, 2, 0, 1, 2, 0, 0, 0, 1, 2, 0, 2, 1],
                [2, 0, 0, 0, 3, 0, 2, 3, 1, 1, 0, 1, 3, 0, 3, 2],
            ),
            (
                27,
                [
                    0x5981, 0x0f55, 0xf0c7, 0x00a8, 0x879f, 0xf272, 0x8801, 0xd49f,
                ],
                [0, 1, 1, 1, 1, 0, 0, 0, 0, 1, 0, 1, 1, 1, 0, 0],
                [0, 2, 2, 2, 1, 1, 0, 1, 1, 2, 1, 1, 2, 2, 0, 1],
                [0, 3, 3, 3, 2, 1, 1, 1, 1, 2, 1, 2, 3, 3, 0, 1],
            ),
        ];
        for (seed, coins, two, three, four) in pins {
            let trials = |n: u64| 0..n;
            assert_eq!(
                trials(8)
                    .map(|k| drawing::<2>(seed).exploration_coin_at(k))
                    .collect::<Vec<u16>>(),
                coins,
                "seed {seed}: bits 16 to 31 of mix64(seed ^ k)"
            );
            assert_eq!(
                trials(8)
                    .map(|k| drawing::<3>(seed).exploration_coin_at(k))
                    .collect::<Vec<u16>>(),
                coins,
                "seed {seed}: the coin is the trial's, whatever the channels"
            );
            assert_eq!(
                trials(16)
                    .map(|k| drawing::<2>(seed).drawn_at(k))
                    .collect::<Vec<u8>>(),
                two,
                "seed {seed}: bits 33 to 47 among two"
            );
            assert_eq!(
                trials(16)
                    .map(|k| drawing::<3>(seed).drawn_at(k))
                    .collect::<Vec<u8>>(),
                three,
                "seed {seed}: among three"
            );
            assert_eq!(
                trials(16)
                    .map(|k| drawing::<4>(seed).drawn_at(k))
                    .collect::<Vec<u8>>(),
                four,
                "seed {seed}: among four"
            );
            assert!(
                trials(16).all(|k| drawing::<1>(seed).drawn_at(k) == 0),
                "seed {seed}: among one, the one"
            );
        }
        // At the learning harness's seed over 4 096 trials, every count the apart oracle's: the
        // coins below a quarter, a half and three quarters of the width, and each channel among
        // three by the stimulus presented, so the two draws tell neither each other nor the
        // stimulus.
        let t = drawing::<3>(27);
        let mut below = [0u32; 3];
        let mut among = [[0u32; 3]; 2];
        for trial in 0..4096u64 {
            let coin = t.exploration_coin_at(trial);
            for (count, quarter) in below.iter_mut().zip([0x4000u16, 0x8000, 0xC000]) {
                *count += u32::from(coin < quarter);
            }
            among[usize::from(t.stimulus_at(trial))][usize::from(t.drawn_at(trial))] += 1;
        }
        assert_eq!(below, [1046, 2078, 3055], "of 4 096");
        assert_eq!(
            among,
            [[707, 657, 666], [670, 674, 722]],
            "[stimulus][channel]"
        );
    }

    /// The coin against the value (ADR-0162, ADR-0163): no coin draws at a value at or above
    /// zero; one LSB below it exactly the first coins do, as many as the reward's magnitude
    /// leaves of the width, rounded up, and never none; at minus the magnitude and beyond it
    /// every coin does; and between, the coins that draw are the first
    /// $\lceil \text{below} \cdot 2^{16} / r \rceil$, counted over every coin against that hand
    /// rule. A magnitude that is not above zero draws at no coin.
    #[test]
    fn a_coin_draws_exactly_below_the_value_s_part_below_zero_over_the_reward() {
        let draws = Exploration::draws;
        // At a reward of 1.0 a coin is one LSB of the value: the coins below the part below
        // zero draw, and no other.
        for value in [0, 1, 100, ONE, i32::MAX] {
            assert!(
                !draws(0, value, ONE) && !draws(u16::MAX, value, ONE),
                "{value}: at or above zero, no coin"
            );
        }
        assert!(
            draws(0, -1, ONE) && !draws(1, -1, ONE),
            "one LSB below: one coin"
        );
        assert!(draws(99, -100, ONE) && !draws(100, -100, ONE));
        assert!(
            draws(0x7FFF, -ONE / 2, ONE) && !draws(0x8000, -ONE / 2, ONE),
            "at minus a half, half the coins"
        );
        assert!(
            draws(0xFFFE, -ONE + 1, ONE) && !draws(0xFFFF, -ONE + 1, ONE),
            "one LSB above minus the reward: every coin but the last"
        );
        for value in [-ONE, -ONE - 1, -2 * ONE, i32::MIN + 1, i32::MIN] {
            assert!(
                draws(0, value, ONE) && draws(u16::MAX, value, ONE),
                "{value}: at minus the reward and beyond, every coin"
            );
        }
        // At the unit tests' reward, a quarter: one LSB of the value is four coins.
        assert!(draws(3, -1, REWARD) && !draws(4, -1, REWARD));
        assert!(draws(u16::MAX, -REWARD, REWARD) && !draws(u16::MAX, -REWARD + 1, REWARD));
        assert!(draws(0xFFFB, -REWARD + 1, REWARD), "the last four do not");
        // Where the magnitude is not a power of two the count is rounded up: at three, one LSB
        // below zero is a third, 21 846 coins of 65 536; and above 1.0 one LSB is still a coin.
        assert!(draws(21_845, -1, 3) && !draws(21_846, -1, 3));
        assert!(draws(43_690, -2, 3) && !draws(43_691, -2, 3));
        assert!(draws(u16::MAX, -3, 3) && draws(u16::MAX, -4, 3));
        assert!(draws(0, -1, 3 * ONE) && !draws(1, -1, 3 * ONE));
        assert!(draws(0, -1, i32::MAX) && !draws(1, -1, i32::MAX));
        assert!(
            draws(u16::MAX, i32::MIN, i32::MAX) && draws(u16::MAX, -i32::MAX, i32::MAX),
            "at the width"
        );
        assert!(
            draws(u16::MAX, -2_147_450_880, i32::MAX) && !draws(u16::MAX, -2_147_450_879, i32::MAX),
            "the last coin's edge at the widest magnitude: 65 535 parts of 65 536"
        );
        // Every coin, against the hand rule's count.
        for reward in [1, 3, 1000, REWARD, ONE, 3 * ONE, i32::MAX] {
            for value in [
                1,
                0,
                -1,
                -2,
                -reward / 3,
                -reward / 2,
                -reward + 1,
                -reward,
                i32::MIN,
            ] {
                let below = (-i64::from(value)).clamp(0, i64::from(reward)) as u64;
                let hand = (below * 65_536).div_ceil(reward as u64);
                let drawn = (0..=u16::MAX)
                    .filter(|&coin| draws(coin, value, reward))
                    .count() as u64;
                assert_eq!(drawn, hand, "value {value} reward {reward}");
                assert!(
                    (0..=u16::MAX)
                        .all(|coin| draws(coin, value, reward) == (u64::from(coin) < hand)),
                    "value {value} reward {reward}: the first coins, in order"
                );
            }
        }
        // No magnitude, or one below zero, which `check` refuses: no coin at any value.
        for reward in [0, -1, -ONE, i32::MIN] {
            for value in [i32::MIN, -ONE, -1, 0, 1, i32::MAX] {
                assert!(
                    !draws(0, value, reward) && !draws(u16::MAX, value, reward),
                    "value {value} reward {reward}"
                );
            }
        }
    }

    /// The draw among the channels (ADR-0163): over every draw of fifteen bits the channel is
    /// the floor of the draw times the channels over 32 768, by a hand rule, so the draws are
    /// dealt in order — the first to the first channel, the last to the last — into runs whose
    /// lengths differ by at most one: equal among one, two, four and sixty-four, and 10 923,
    /// 10 923 and 10 922 among three. A draw's sixteenth bit is not read.
    #[test]
    fn a_drawn_channel_is_each_of_the_channels_with_the_same_chance_to_within_one_draw() {
        let channel = Exploration::channel;
        for (channels, expected) in [
            (1usize, vec![32_768u32]),
            (2, vec![16_384; 2]),
            (3, vec![10_923, 10_923, 10_922]),
            (4, vec![8_192; 4]),
            (5, vec![6_554, 6_554, 6_553, 6_554, 6_553]),
            (64, vec![512; 64]),
        ] {
            let mut counts = vec![0u32; channels];
            let mut last = 0u8;
            for draw in 0..=0x7FFFu16 {
                let c = channel(draw, channels);
                assert_eq!(
                    usize::from(c),
                    usize::from(draw) * channels / 32_768,
                    "{draw} among {channels}"
                );
                assert!(c >= last, "{draw} among {channels}: in order");
                assert_eq!(
                    channel(draw | 0x8000, channels),
                    c,
                    "the sixteenth bit is not read"
                );
                counts[usize::from(c)] += 1;
                last = c;
            }
            assert_eq!(counts, expected, "among {channels}");
            assert_eq!(
                (channel(0, channels), usize::from(channel(0x7FFF, channels))),
                (0, channels - 1),
                "among {channels}: the first and the last"
            );
            let (low, high) = (
                counts.iter().min().copied().unwrap(),
                counts.iter().max().copied().unwrap(),
            );
            assert!(high - low <= 1, "among {channels}: within one draw");
        }
        assert_eq!(channel(0x7FFF, 0), 0, "no channel");
        // The runs' edges among three, by hand: a third of 32 768 is 10 922 and two thirds.
        assert_eq!(
            [0u16, 10_922, 10_923, 21_845, 21_846, 32_767].map(|d| channel(d, 3)),
            [0, 0, 1, 1, 2, 2]
        );
    }

    /// The exploration's refusals (ADR-0163): on an engine without the critic, whose value it
    /// would read; with a hold, which acts on the gate's reading before the value is whole;
    /// and with a reward's magnitude of zero, over which its probability is taken — each
    /// before anything is injected, and none with the exploration unset, where every task is
    /// refused or run as it was.
    #[test]
    fn every_refusal_of_the_exploration_is_named() {
        let plain = network(1, ONE / 2);
        let valued_exec = valued(ONE / 2);
        for feedback in [
            Feedback::Answer,
            Feedback::Shuffled,
            Feedback::Withheld,
            Feedback::SevenInEight,
        ] {
            assert_eq!(
                exploring(feedback).check(&plain),
                Err(TaskError::ExplorationWithoutCritic),
                "{feedback:?}"
            );
            assert_eq!(
                exploring(feedback).check(&valued_exec),
                Ok(()),
                "{feedback:?}"
            );
            assert_eq!(task(feedback).check(&plain), Ok(()), "{feedback:?}: unset");
            assert_eq!(task(feedback).check(&valued_exec), Ok(()), "{feedback:?}");
        }
        let mut exec = network(1, ONE / 2);
        assert_eq!(
            exploring(Feedback::Answer).trial(&mut exec, 0),
            Err(TaskError::ExplorationWithoutCritic)
        );
        assert!(
            exec.is_quiescent() && exec.ticks() == 0,
            "a refusal injects nothing"
        );
        // A hold: refused with the exploration, a task that runs without it.
        let held = |exploration: Exploration| Task {
            window: Window { from: 4, ticks: 12 },
            hold: Some(Hold {
                until: 48,
                every: 8,
                messages: 1,
                efficacy_q16: -1,
            }),
            exploration,
            ..task(Feedback::Answer)
        };
        assert_eq!(held(Exploration::Unset).check(&valued_exec), Ok(()));
        assert_eq!(
            held(Exploration::ValueGated).check(&valued_exec),
            Err(TaskError::ExplorationWithHold)
        );
        assert_eq!(
            held(Exploration::ValueGated).check(&plain),
            Err(TaskError::ExplorationWithoutCritic),
            "the critic is read first"
        );
        // No magnitude: with the reward withheld a task of no magnitude runs, and the
        // exploration is refused on it; under a feedback that delivers the reward it is the
        // exploration's refusal that is read first.
        let none = |exploration: Exploration, feedback: Feedback| Task {
            reward_q16: 0,
            exploration,
            ..task(feedback)
        };
        assert_eq!(
            none(Exploration::Unset, Feedback::Withheld).check(&valued_exec),
            Ok(())
        );
        assert_eq!(
            none(Exploration::ValueGated, Feedback::Withheld).check(&valued_exec),
            Err(TaskError::ExplorationWithoutReward)
        );
        assert_eq!(
            none(Exploration::ValueGated, Feedback::Answer).check(&valued_exec),
            Err(TaskError::ExplorationWithoutReward)
        );
        assert_eq!(
            none(Exploration::Unset, Feedback::Answer).check(&valued_exec),
            Err(TaskError::NoReward)
        );
        let one = Task {
            reward_q16: 1,
            ..exploring(Feedback::Withheld)
        };
        assert_eq!(one.check(&valued_exec), Ok(()), "one LSB is a magnitude");
        // The refusals before it are read before it.
        let below = Task {
            reward_q16: -1,
            ..exploring(Feedback::Answer)
        };
        assert_eq!(below.check(&valued_exec), Err(TaskError::NegativeReward));
        let own = Task {
            critic: Some(Critic::new(5)),
            ..exploring(Feedback::Answer)
        };
        assert_eq!(own.check(&valued_exec), Err(TaskError::TwoCritics));
        assert_eq!(
            own.check(&plain),
            Err(TaskError::ExplorationWithoutCritic),
            "the task's own critic is not the engine's"
        );
        assert_eq!(
            exploring(Feedback::Answer).check(&valued(ONE)),
            Err(TaskError::RewardAtCeiling)
        );
    }

    /// With the exploration unset a trial is the trial it was (ADR-0163): `trial` against the
    /// trial as ADR-0155 left it, written out here as the oracle with the four feedbacks and
    /// the four deliveries it had, on twin engines over eight trials under a drive, a cancel
    /// and a sub-window, a readout cued before some of them — once under the task's critic
    /// with the addressed delivery, and under the engine's critic and its window with every
    /// delivery, every value weight written so far below zero that a set exploration's coin
    /// would draw wherever the critic counted a spike. The same outcome, the same units'
    /// fields, the same train, the same messages drained, the same addressed set, the same
    /// signal, counts, reading and weights after every trial; and no trial drawn.
    #[test]
    fn with_the_exploration_unset_a_trial_is_the_trial_it_was() {
        fn before(t: &mut Task, exec: &mut Executor<8>, trial: u64) -> Outcome {
            t.check(exec).unwrap();
            assert_eq!(t.hold, None, "the oracle is of a trial with no hold");
            let start = exec.ticks();
            let draw = mix64(t.seed ^ trial);
            let stimulus = (draw & 1) as u8;
            let inject = exec.injector();
            let presented = t.stimuli[usize::from(stimulus)];
            presented.inject(&inject).unwrap();
            for k in 0..t.ticks {
                presented.cancel_at(&inject, k).unwrap();
                t.drive.step(&inject, exec.ticks()).unwrap();
                exec.tick();
            }
            let counts = t.readout.count_window(
                exec.train(),
                (start as u32).wrapping_add(t.window.from),
                t.window.ticks,
            );
            let selection = t.readout.select(counts);
            let correct = selection == Some(t.answer(stimulus));
            let selected = selection.map(|r| t.readout.sets()[usize::from(r)]);
            match t.delivery {
                Delivery::Global => exec.address_all(),
                Delivery::Addressed => exec
                    .address(
                        presented.set.units(),
                        selected.iter().flat_map(|set| set.units()),
                    )
                    .unwrap(),
                Delivery::Drawn => exec
                    .address_drawn(selected.iter().flat_map(|set| set.units()))
                    .unwrap(),
                Delivery::Released => exec.address_drawn(0..16).unwrap(),
            }
            let positive = match t.feedback {
                Feedback::Answer => correct,
                Feedback::Shuffled => (draw >> 32) & 1 == 1,
                Feedback::SevenInEight => correct != ((draw >> 48) & 7 == 0),
                Feedback::Withheld => {
                    return Outcome {
                        trial,
                        stimulus,
                        counts,
                        selection,
                        correct,
                        reward_q16: 0,
                        signal_q16: exec.modulator().dopamine_rpe,
                        expected_q16: t.critic.map(|c| [c.expected_q16[usize::from(stimulus)]; 2]),
                        value_q16: None,
                        held: 0,
                        drawn: false,
                    };
                }
            };
            let outcome_q16 = if positive {
                t.reward_q16
            } else {
                t.reward_q16.saturating_neg()
            };
            let (delivered_q16, expected_q16) = match t.critic.as_mut() {
                Some(critic) => {
                    let (error, before, after) = critic.predict(stimulus, outcome_q16);
                    (error, Some([before, after]))
                }
                None => (outcome_q16, None),
            };
            let signal_q16 = exec.reward(delivered_q16);
            let (reward_q16, value_q16) = match exec.prediction() {
                Some(prediction) => (prediction.received_q16, Some(prediction.value_q16)),
                None => (delivered_q16, None),
            };
            Outcome {
                trial,
                stimulus,
                counts,
                selection,
                correct,
                reward_q16,
                signal_q16,
                expected_q16,
                value_q16,
                held: 0,
                drawn: false,
            }
        }
        // The task's critic with the addressed delivery, then the engine's with each delivery.
        let cases = [
            (false, Delivery::Addressed),
            (true, Delivery::Global),
            (true, Delivery::Addressed),
            (true, Delivery::Drawn),
            (true, Delivery::Released),
        ];
        let mut would_draw = 0u32;
        for feedback in [
            Feedback::Answer,
            Feedback::Shuffled,
            Feedback::Withheld,
            Feedback::SevenInEight,
        ] {
            for (engine_s, delivery) in cases {
                let mut t = task(feedback);
                assert_eq!(t.exploration, Exploration::Unset);
                t.window = Window { from: 4, ticks: 20 };
                t.drive = Drive {
                    every: 1,
                    messages: 2,
                    efficacy_q16: 0x2000,
                    units: 16,
                    seed: 5,
                };
                for stimulus in t.stimuli.iter_mut() {
                    stimulus.cancel = Some(Cancel {
                        offset: 30,
                        ticks: 3,
                        messages: 1,
                        efficacy_q16: -ONE,
                    });
                }
                t.delivery = delivery;
                let (mut now, mut then) = if engine_s {
                    let mut pair = (windowed_network(40), windowed_network(40));
                    weigh(&mut pair.0, i16::MIN);
                    weigh(&mut pair.1, i16::MIN);
                    pair
                } else {
                    t.critic = Some(Critic::new(5));
                    (network(2, ONE / 2), network(2, ONE / 2))
                };
                let mut oracle = t;
                let case = format!("{feedback:?}, the engine's critic {engine_s}, {delivery:?}");
                for trial in 0..8u64 {
                    let cued = [None, Some(0), None, Some(1), Some(1), None, Some(0), None];
                    if let Some(r) = cued[trial as usize] {
                        cue(&now, t.readout.sets()[r]);
                        cue(&then, t.readout.sets()[r]);
                    }
                    let outcome = t.trial(&mut now, trial).unwrap();
                    assert_eq!(
                        outcome,
                        before(&mut oracle, &mut then, trial),
                        "{case} trial {trial}"
                    );
                    assert!(
                        !outcome.drawn,
                        "{case} trial {trial}: unset, nothing is drawn"
                    );
                    assert_eq!(fields(&now), fields(&then), "{case} trial {trial}");
                    assert_eq!(now.train().to_vec(), then.train().to_vec(), "{case}");
                    assert_eq!(
                        (
                            now.delivered(),
                            now.addressed_counts(),
                            now.modulator().dopamine_rpe,
                            now.features().to_vec(),
                            now.prediction(),
                            now.units()
                                .iter()
                                .map(|u| u.value_weight)
                                .collect::<Vec<i16>>(),
                        ),
                        (
                            then.delivered(),
                            then.addressed_counts(),
                            then.modulator().dopamine_rpe,
                            then.features().to_vec(),
                            then.prediction(),
                            then.units()
                                .iter()
                                .map(|u| u.value_weight)
                                .collect::<Vec<i16>>(),
                        ),
                        "{case} trial {trial}"
                    );
                    assert_eq!(
                        (0..16)
                            .map(|u| (now.is_source(u), now.is_target(u)))
                            .collect::<Vec<(bool, bool)>>(),
                        (0..16)
                            .map(|u| (then.is_source(u), then.is_target(u)))
                            .collect::<Vec<(bool, bool)>>(),
                        "{case} trial {trial}"
                    );
                    assert_eq!((t.critic, t.readout), (oracle.critic, oracle.readout));
                    // Where a set exploration would have drawn: the value the engine read below
                    // zero and the trial's coin below its part.
                    if let Some(value) = outcome.value_q16 {
                        would_draw += u32::from(Exploration::draws(
                            t.exploration_coin_at(trial),
                            value,
                            t.reward_q16,
                        ));
                    }
                }
            }
        }
        assert_eq!(
            would_draw, 96,
            "a set exploration would have drawn at every trial of the engine's critic under a reward delivered: three feedbacks, four deliveries, eight trials"
        );
    }

    /// The trials of the exploration's tests on [`valued`]'s network, one a fresh engine: every
    /// value weight written `weight`, the readouts of `cued` cued, then `t`'s trial `trial`.
    /// Returns the outcome, the engine, and the value by hand — `weight` times the spikes the
    /// train holds, over four, the floor.
    fn explored(
        t: &mut Task,
        weight: i16,
        cued: &[usize],
        trial: u64,
    ) -> (Outcome, Executor<8>, i32) {
        let mut exec = valued(ONE / 2);
        weigh(&mut exec, weight);
        for &r in cued {
            cue(&exec, t.readout.sets()[r]);
        }
        let outcome = t.trial(&mut exec, trial).unwrap();
        let spikes = exec.train().len() as i64;
        let value = i64::from(weight).saturating_mul(spikes).div_euclid(4) as i32;
        (outcome, exec, value)
    }

    /// Set, nothing changes where the value is at or above zero (ADR-0162, ADR-0163): over the
    /// first eight trials at the unit tests' seed, under each feedback, with every weight at
    /// zero — a value of zero — and at the positive rail, and with no readout cued, the
    /// answer's and the other's, the trial with the exploration set is the trial with it unset
    /// on a twin engine in every field of the outcome and of the engine, the selection the
    /// gate's and nothing drawn.
    #[test]
    fn with_the_exploration_set_nothing_changes_where_the_value_is_at_or_above_zero() {
        let mut values = [0u32; 2];
        for feedback in [
            Feedback::Answer,
            Feedback::Shuffled,
            Feedback::Withheld,
            Feedback::SevenInEight,
        ] {
            for weight in [0i16, 1, i16::MAX] {
                for trial in 0..8u64 {
                    let answer =
                        usize::from(task(feedback).answer(task(feedback).stimulus_at(trial)));
                    for cued in [vec![], vec![answer], vec![answer ^ 1], vec![0, 1]] {
                        let mut set = exploring(feedback);
                        let mut unset = task(feedback);
                        set.delivery = Delivery::Addressed;
                        unset.delivery = Delivery::Addressed;
                        let (got, mut exec, value) = explored(&mut set, weight, &cued, trial);
                        let (was, mut twin, _) = explored(&mut unset, weight, &cued, trial);
                        let case =
                            format!("{feedback:?} weight {weight} trial {trial} cued {cued:?}");
                        assert!(value >= 0, "{case}: {value}");
                        values[usize::from(value > 0)] += 1;
                        assert_eq!(got, was, "{case}");
                        assert!(!got.drawn, "{case}");
                        assert_eq!(
                            got.selection,
                            match cued.as_slice() {
                                [r] => Some(*r as u8),
                                _ => None,
                            },
                            "{case}: the gate's"
                        );
                        if feedback != Feedback::Withheld {
                            assert_eq!(got.value_q16, Some(value), "{case}");
                        }
                        assert_eq!(
                            (
                                fields(&exec),
                                exec.delivered(),
                                exec.addressed_counts(),
                                exec.modulator().dopamine_rpe,
                                exec.prediction(),
                                exec.features().to_vec(),
                            ),
                            (
                                fields(&twin),
                                twin.delivered(),
                                twin.addressed_counts(),
                                twin.modulator().dopamine_rpe,
                                twin.prediction(),
                                twin.features().to_vec(),
                            ),
                            "{case}"
                        );
                        assert_eq!(exec.train().to_vec(), twin.train().to_vec(), "{case}");
                        assert_eq!(
                            (0..16).map(|u| exec.is_target(u)).collect::<Vec<bool>>(),
                            (0..16).map(|u| twin.is_target(u)).collect::<Vec<bool>>(),
                            "{case}"
                        );
                        assert_eq!(set.readout, unset.readout, "{case}: the gate's reading");
                    }
                }
            }
        }
        assert!(
            values[0] > 0 && values[1] > 0,
            "a value of zero and values above it: {values:?}"
        );
    }

    /// A selection is drawn exactly where the trial's coin is below the value's part below zero
    /// (ADR-0162, ADR-0163). Over the first eight trials at the unit tests' seed, each on a
    /// fresh engine whose every weight is written so that the value the engine reads at the
    /// trial's end lies one LSB to either side of the coin's edge: at the reward of a quarter a
    /// coin `c` draws at a value of `-(c / 4 + 1)` and not at `-(c / 4)`. With no readout cued,
    /// the drawn channel's, the other's and both: where the coin draws the selection is the
    /// trial's drawn channel whatever the gate read, the trial is correct where that channel is
    /// the stimulus's answer, and the reward's sign is that outcome's; where it does not the
    /// trial is the trial with the exploration unset. Either way the value recorded is the one
    /// the task read, the one the reward was taken against, and the readout's channels hold the
    /// gate's own reading.
    #[test]
    fn a_selection_is_drawn_exactly_where_the_coin_is_below_the_value_s_part_below_zero() {
        let probe = exploring(Feedback::Answer);
        let mut seen = [[false; 2]; 4];
        let mut both = [[false; 2]; 2];
        for trial in 0..8u64 {
            let coin = probe.exploration_coin_at(trial);
            let channel = probe.drawn_at(trial);
            let stimulus = probe.stimulus_at(trial);
            let answer = probe.answer(stimulus);
            both[usize::from(stimulus)][usize::from(channel == answer)] = true;
            // The stimulus's four units fire once each and a cued readout's units carry the
            // same weight, so the value is the weight times the spikes over four; the edge is
            // read on the value itself.
            let quarter = i32::from(coin / 4);
            for (kind, cued) in [
                vec![],
                vec![usize::from(channel)],
                vec![usize::from(channel ^ 1)],
                vec![0, 1],
            ]
            .into_iter()
            .enumerate()
            {
                let spikes = 4 + 4 * cued.len() as i32;
                let gate = match cued.as_slice() {
                    [r] => Some(*r as u8),
                    _ => None,
                };
                for draws in [false, true] {
                    // The largest weight whose value is at or below the edge to be reached: a
                    // value of `-(quarter + 1)` or below draws, one of `-quarter` or above
                    // does not.
                    let edge = if draws { -(quarter + 1) } else { -quarter };
                    let weight = if draws {
                        (edge * 4).div_euclid(spikes)
                    } else {
                        -((-edge * 4).div_euclid(spikes))
                    };
                    let weight = i16::try_from(weight).unwrap();
                    let mut set = exploring(Feedback::Answer);
                    let mut unset = task(Feedback::Answer);
                    let (got, mut exec, value) = explored(&mut set, weight, &cued, trial);
                    let (was, _, _) = explored(&mut unset, weight, &cued, trial);
                    let case = format!(
                        "trial {trial} coin {coin:#x} cued {cued:?} weight {weight} value {value}"
                    );
                    assert_eq!(exec.train().len() as i32, spikes, "{case}");
                    assert!(
                        if draws { value <= edge } else { value >= edge },
                        "{case}: the value lies on its side of the edge"
                    );
                    assert_eq!(
                        Exploration::draws(coin, value, REWARD),
                        draws,
                        "{case}: the coin against the value by the rule"
                    );
                    assert_eq!(
                        (got.value_q16, exec.prediction().map(|p| p.value_q16)),
                        (Some(value), Some(value)),
                        "{case}: the value read is the one the reward was taken against"
                    );
                    assert_eq!(got.drawn, draws, "{case}");
                    assert_eq!(was.selection, gate, "{case}: unset, the gate's");
                    assert!(!was.drawn, "{case}");
                    if draws {
                        let correct = channel == answer;
                        let reward = if correct { REWARD } else { -REWARD };
                        assert_eq!(
                            got,
                            Outcome {
                                selection: Some(channel),
                                correct,
                                reward_q16: reward - value,
                                signal_q16: reward - value,
                                drawn: true,
                                ..was
                            },
                            "{case}: the trial's drawn channel, judged and rewarded as a selection"
                        );
                    } else {
                        assert_eq!(got, was, "{case}: the trial with the exploration unset");
                    }
                    assert_eq!(
                        set.readout, unset.readout,
                        "{case}: the channels hold the gate's own reading"
                    );
                    assert_eq!(
                        set.readout
                            .channels()
                            .each_ref()
                            .map(|c| c.selected_flag == 1),
                        [gate == Some(0), gate == Some(1)],
                        "{case}"
                    );
                    seen[kind][usize::from(draws)] = true;
                }
            }
        }
        assert_eq!(seen, [[true; 2]; 4], "each cue, drawn and not");
        assert_eq!(
            both, [[true; 2]; 2],
            "each stimulus, its drawn channel its answer and not"
        );
    }

    /// Under each feedback a drawn selection is judged and rewarded as any selection (ADR-0163):
    /// with every weight at the negative rail, where every coin draws, over the first eight
    /// trials at the unit tests' seed and with no readout cued, the answer's and the other's.
    /// The selection is the trial's drawn channel; correct is that channel against the
    /// stimulus's answer; the reward's sign is the outcome's under the answer's feedback, the
    /// coin's under the shuffled one, the outcome's unless the coin is misleading under the
    /// seven in eight, and none where it is withheld, where the trial records the draw and no
    /// value.
    #[test]
    fn a_drawn_selection_is_judged_and_rewarded_as_any_selection_under_each_feedback() {
        for feedback in [
            Feedback::Answer,
            Feedback::Shuffled,
            Feedback::Withheld,
            Feedback::SevenInEight,
        ] {
            let mut signs = [false; 2];
            for trial in 0..8u64 {
                let probe = exploring(feedback);
                let stimulus = probe.stimulus_at(trial);
                let answer = probe.answer(stimulus);
                let channel = probe.drawn_at(trial);
                for cued in [
                    vec![],
                    vec![usize::from(answer)],
                    vec![usize::from(answer ^ 1)],
                ] {
                    let mut t = exploring(feedback);
                    let (got, exec, value) = explored(&mut t, i16::MIN, &cued, trial);
                    let case = format!("{feedback:?} trial {trial} cued {cued:?}");
                    assert!(
                        value <= -REWARD,
                        "{case}: {value}, at or beyond minus the reward"
                    );
                    assert_eq!(
                        (got.drawn, got.selection, got.correct, got.stimulus),
                        (true, Some(channel), channel == answer, stimulus),
                        "{case}"
                    );
                    let positive = match feedback {
                        Feedback::Answer => Some(got.correct),
                        Feedback::Shuffled => Some(probe.coin_at(trial)),
                        Feedback::SevenInEight => Some(got.correct != probe.misleading_at(trial)),
                        Feedback::Withheld => None,
                    };
                    match positive {
                        Some(positive) => {
                            let reward = if positive { REWARD } else { -REWARD };
                            assert_eq!(
                                (got.reward_q16, got.value_q16),
                                (reward.saturating_sub(value), Some(value)),
                                "{case}"
                            );
                            signs[usize::from(positive)] = true;
                        }
                        None => {
                            assert_eq!((got.reward_q16, got.value_q16), (0, None), "{case}");
                            assert_eq!(exec.prediction(), None, "{case}: no reward, no reading");
                        }
                    }
                }
            }
            if feedback != Feedback::Withheld {
                assert_eq!(signs, [true; 2], "{feedback:?}: a reward of each sign");
            }
        }
    }

    /// Under each delivery, what is addressed at a drawn selection (ADR-0163), among three
    /// readouts on [`wide`]'s network with the critic's window, every weight at the negative
    /// rail so that every coin draws, each trial on a fresh engine: where the gate selected
    /// another channel than the drawn one, and where it selected none — nothing cued, and two
    /// cued, the largest count shared. Under the addressed delivery the sources are the
    /// presented stimulus's units and the targets the drawn readout's; under the drawn one the
    /// sources are the units the critic's window counted and the targets the drawn readout's;
    /// under the released one those sources and every unit a target; under the global one every
    /// unit on both sides. The same trials with the exploration unset address the gate's
    /// selection, none at a tie.
    #[test]
    fn under_each_delivery_a_drawn_selection_addresses_the_drawn_readout() {
        let addressed = |exec: &Executor<8>| -> (Vec<u32>, Vec<u32>) {
            (
                (0..24).filter(|&u| exec.is_source(u)).collect(),
                (0..24).filter(|&u| exec.is_target(u)).collect(),
            )
        };
        let units_of = |r: u8| -> Vec<u32> {
            let first = [4u32, 12, 16][usize::from(r)];
            (first..first + 4).collect()
        };
        let probe = Task {
            exploration: Exploration::ValueGated,
            ..task3(Feedback::Answer)
        };
        let mut drawn_channels = [false; 3];
        let mut gates = [false; 2];
        for delivery in [
            Delivery::Global,
            Delivery::Addressed,
            Delivery::Drawn,
            Delivery::Released,
        ] {
            for trial in 0..8u64 {
                let channel = probe.drawn_at(trial);
                drawn_channels[usize::from(channel)] = true;
                let other = usize::from(channel + 1) % 3;
                let third = usize::from(channel + 2) % 3;
                for (cued, gate) in [
                    (vec![], None),
                    (vec![other], Some(other as u8)),
                    (vec![third], Some(third as u8)),
                    (vec![other, third], None),
                    (vec![0, 1, 2], None),
                ] {
                    let stimulus_units: Vec<u32> = probe.stimuli
                        [usize::from(probe.stimulus_at(trial))]
                    .set
                    .units()
                    .collect();
                    // The units the window counted: the volley, the stimulus's and the cued
                    // readouts', in unit order.
                    let mut counted = stimulus_units.clone();
                    for &r in &cued {
                        counted.extend(units_of(r as u8));
                    }
                    counted.sort_unstable();
                    let run = |exploration: Exploration| {
                        let mut exec = wide(2, ONE / 2, Some(30));
                        for unit in exec.units_mut() {
                            unit.value_weight = i16::MIN;
                        }
                        let mut t = Task {
                            delivery,
                            exploration,
                            ..task3(Feedback::Answer)
                        };
                        for &r in &cued {
                            cue(&exec, t.readout.sets()[r]);
                        }
                        let outcome = t.trial(&mut exec, trial).unwrap();
                        (outcome, addressed(&exec))
                    };
                    let case = format!("{delivery:?} trial {trial} cued {cued:?}");
                    let (got, set) = run(Exploration::ValueGated);
                    let (was, unset) = run(Exploration::Unset);
                    assert_eq!(
                        (got.drawn, got.selection, was.drawn, was.selection),
                        (true, Some(channel), false, gate),
                        "{case}"
                    );
                    assert_ne!(
                        got.selection, was.selection,
                        "{case}: another than the gate's"
                    );
                    gates[usize::from(gate.is_some())] = true;
                    let all: Vec<u32> = (0..24).collect();
                    let (sources, drawn_targets, gated_targets) = match delivery {
                        Delivery::Global => (all.clone(), all.clone(), all.clone()),
                        Delivery::Addressed => (
                            stimulus_units,
                            units_of(channel),
                            gate.map_or(vec![], units_of),
                        ),
                        Delivery::Drawn => {
                            (counted, units_of(channel), gate.map_or(vec![], units_of))
                        }
                        Delivery::Released => (counted, all.clone(), all.clone()),
                    };
                    assert_eq!(set, (sources.clone(), drawn_targets), "{case}: drawn");
                    assert_eq!(unset, (sources, gated_targets), "{case}: unset");
                }
            }
        }
        assert_eq!(
            drawn_channels, [true; 3],
            "each channel drawn at some trial"
        );
        assert_eq!(gates, [true; 2], "the gate selected another, and none");
    }
}

/// The lattice property (ADR-0030): over seeded trains and seeded set pairs the selection is
/// the sign of the count difference, none exactly at equal counts; over seeded periodic sets
/// the units a set walks are exactly the ones its membership test names (ADR-0065).
#[cfg(test)]
mod prop {
    use super::*;
    use crate::executor::Config;
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../testkit/prop.rs"
    ));

    /// A seeded well-formed set: a period in `1..=32`, a mask of bits below it, a count
    /// below eight, a first unit below 200.
    fn seeded_set(lcg: &mut Lcg) -> Set {
        let period = lcg.below(MAX_PERIOD).saturating_add(1);
        let width = if period == 32 {
            u32::MAX
        } else {
            (1u32 << period).wrapping_sub(1)
        };
        let mask = lcg.next_u32() & width;
        Set {
            first: lcg.below(200),
            period,
            mask: if mask == 0 { 1 } else { mask },
            count: lcg.below(8),
        }
    }

    #[test]
    fn a_set_s_units_are_exactly_the_ones_it_names() {
        let mut lcg = Lcg::new(29);
        for _ in 0..2000 {
            let s = seeded_set(&mut lcg);
            assert!(s.is_well_formed(), "{s:?}");
            let units: Vec<u32> = s.units().collect();
            // Ascending and distinct: each unit named once.
            for pair in units.windows(2) {
                assert!(pair[0] < pair[1], "{s:?}: {units:?}");
            }
            assert_eq!(units.len() as u64, s.len(), "{s:?}");
            // The membership test over the whole range agrees with the walk.
            let named: Vec<u32> = (0..600).filter(|&u| s.contains(u)).collect();
            assert_eq!(named, units, "{s:?}");
            let end = units
                .last()
                .map_or(u64::from(s.first), |&u| u64::from(u) + 1);
            assert_eq!(s.end(), end, "{s:?}");
            assert_eq!(s.is_empty(), units.is_empty());
            // Overlap is a shared unit.
            let o = seeded_set(&mut lcg);
            let shared = o.units().any(|u| units.contains(&u));
            assert_eq!(s.overlaps(&o), shared, "{s:?} {o:?}");
            assert_eq!(o.overlaps(&s), shared);
        }
    }

    /// The misleading coin over the lattice (ADR-0148): for every seed and every trial built
    /// from the `u32` lattice's words, and over seeded pairs, the coin is the hand rule — the
    /// draw's second byte from the top, which holds bits 48 to 55, with its low three bits all
    /// zero — and it is the rule of a draw with the stimulus's bit and the shuffled coin's
    /// flipped, so it reads neither. Both sides of the coin occur among the seeded pairs, the
    /// misleading one 502 times in 4 096, about one in eight.
    #[test]
    fn the_misleading_coin_is_the_hand_rule_over_the_lattice() {
        let hand = |draw: u64| draw.to_be_bytes()[1] % 8 == 0;
        let stimulus = |first: u32| Stimulus {
            set: Set::contiguous(first, 4),
            messages: 2,
            efficacy_q16: 0x0001_4000,
            cancel: None,
        };
        // A task whose draws are read and whose trial is never run.
        let drawing = Task {
            stimuli: [stimulus(0), stimulus(8)],
            readout: Readout::new([Set::contiguous(4, 4), Set::contiguous(12, 4)]),
            drive: Drive {
                every: 0,
                messages: 0,
                efficacy_q16: 0,
                units: 16,
                seed: 0,
            },
            ticks: 64,
            window: Window::whole(64),
            seed: 0,
            reward_q16: 0x4000,
            answers: [0, 1],
            feedback: Feedback::SevenInEight,
            delivery: Delivery::Global,
            critic: None,
            hold: None,
            exploration: Exploration::Unset,
        };
        let one = |seed: u64, trial: u64| {
            let t = Task { seed, ..drawing };
            let draw = mix64(seed ^ trial);
            assert_eq!(t.misleading_at(trial), hand(draw), "{seed:#x} {trial:#x}");
            assert_eq!(
                hand(draw),
                hand(draw ^ 0x0000_0001_0000_0001),
                "{seed:#x} {trial:#x}: neither the stimulus's bit nor the shuffled coin's"
            );
            // The two draws beside it are the ones they were.
            assert_eq!(t.stimulus_at(trial), (draw % 2) as u8);
            assert_eq!(t.coin_at(trial), (draw >> 32) % 2 == 1);
            hand(draw)
        };
        let words = |high: u32, low: u32| u64::from(high) << 32 | u64::from(low);
        for &a in U32_LATTICE.iter() {
            for &b in U32_LATTICE.iter() {
                for &c in U32_LATTICE.iter() {
                    one(words(a, b), words(b, c));
                    one(words(c, a), u64::from(b));
                    one(u64::from(a), words(c, b));
                }
            }
        }
        let mut lcg = Lcg::new(41);
        let mut misleading = 0u32;
        for _ in 0..4096 {
            misleading += u32::from(one(lcg.next_u64(), lcg.next_u64()));
        }
        assert_eq!(misleading, 502, "about one in eight of 4 096");
        // Each of the three bits alone, and every other bit set, by the mask itself.
        for bit in 0..64u32 {
            let draw = 1u64 << bit;
            assert_eq!(
                draw & MISLEADING_BITS == 0,
                !(48..=50).contains(&bit),
                "bit {bit}"
            );
            assert_eq!(hand(draw), !(48..=50).contains(&bit), "bit {bit}");
        }
    }

    /// A cancel injects at its ticks and at no other (ADR-0076): over seeded cancels, before
    /// every tick of a trial `cancel_at` sends the set's units times the messages when the tick
    /// is within the cancel's ticks from its offset, by a hand rule, and nothing otherwise; the
    /// ring drains what was sent and no more; a stimulus with no cancel sends nothing at any
    /// tick; and the rule over the lattice of offsets, ticks and tick counts.
    #[test]
    fn a_cancel_injects_at_its_ticks_and_at_no_other() {
        let mut lcg = Lcg::new(31);
        let cue = 0x0001_4000;
        for _ in 0..64 {
            let offset = lcg.below(40).saturating_add(1);
            let ticks = lcg.below(6).saturating_add(1);
            let messages = lcg.below(3).saturating_add(1);
            let cancel = Cancel {
                offset,
                ticks,
                messages,
                efficacy_q16: (lcg.below(0x0002_0000) as i32)
                    .saturating_neg()
                    .saturating_sub(1),
            };
            let set = Set::contiguous(lcg.below(12), lcg.below(4).saturating_add(1));
            let with = Stimulus {
                set,
                messages: 2,
                efficacy_q16: cue,
                cancel: Some(cancel),
            };
            let without = Stimulus {
                cancel: None,
                ..with
            };
            let mut exec = Executor::<8>::new(Config {
                workers: 1,
                units: 16,
                injector_capacity: 64,
                modulation_baseline_q16: 0,
                ..Config::default()
            })
            .unwrap();
            let inject = exec.injector();
            let mut expected = 0u32;
            for k in 0..48u32 {
                let due = k.checked_sub(offset).is_some_and(|d| d < ticks);
                assert_eq!(cancel.is_due(k), due, "{cancel:?} at {k}");
                let count = if due {
                    (set.len() as u32).saturating_mul(messages)
                } else {
                    0
                };
                assert_eq!(with.cancel_at(&inject, k), Ok(count), "{cancel:?} at {k}");
                assert_eq!(without.cancel_at(&inject, k), Ok(0));
                expected = expected.saturating_add(count);
                exec.tick();
            }
            exec.tick();
            assert_eq!(
                exec.delivered(),
                u64::from(expected),
                "{cancel:?}: the ring drained what was sent"
            );
        }
        for &k in U32_LATTICE.iter() {
            for &offset in U32_LATTICE.iter() {
                for &ticks in U32_LATTICE.iter() {
                    let c = Cancel {
                        offset,
                        ticks,
                        messages: 1,
                        efficacy_q16: -1,
                    };
                    assert_eq!(
                        c.is_due(k),
                        k.checked_sub(offset).is_some_and(|d| d < ticks),
                        "{c:?} at {k}"
                    );
                    assert_eq!(c.end(), u64::from(offset).saturating_add(u64::from(ticks)));
                }
            }
        }
    }

    /// A hold delivers at its times and at no other (ADR-0144): over seeded holds and windows,
    /// a trial with a readout cued delivers the hold's messages into the other readout's four
    /// units once for every tick from the window's close below `until` on the cadence, by a
    /// hand rule, and a trial with none cued delivers nothing; the ring drains the stimulus's
    /// messages, the cue's and the hold's and no more; and the rule over the lattice of closes,
    /// ends, cadences and ticks.
    #[test]
    fn a_hold_delivers_at_its_times_and_at_no_other() {
        let mut lcg = Lcg::new(37);
        let cue = spike_message(0x0001_4000, false);
        for round in 0..48u32 {
            let close = lcg.below(24).saturating_add(20);
            let until = close
                .saturating_add(1)
                .saturating_add(lcg.below(62u32.saturating_sub(close)));
            let every = lcg.below(12).saturating_add(1);
            let messages = lcg.below(3).saturating_add(1);
            let hold = Hold {
                until,
                every,
                messages,
                efficacy_q16: (lcg.below(0x0002_0000) as i32)
                    .saturating_neg()
                    .saturating_sub(1),
            };
            let mut exec = Executor::<8>::new(Config {
                workers: 1,
                units: 16,
                injector_capacity: 64,
                train_capacity: 64,
                modulation_baseline_q16: 0,
                ..Config::default()
            })
            .unwrap();
            for unit in exec.units_mut() {
                unit.v_thresh = cortex_core::THRESHOLD_BASE;
            }
            let stimulus = |first: u32| Stimulus {
                set: Set::contiguous(first, 4),
                messages: 2,
                efficacy_q16: 0x0001_4000,
                cancel: None,
            };
            let mut t = Task {
                stimuli: [stimulus(0), stimulus(8)],
                readout: Readout::new([Set::contiguous(4, 4), Set::contiguous(12, 4)]),
                drive: Drive {
                    every: 0,
                    messages: 0,
                    efficacy_q16: 0,
                    units: 16,
                    seed: 0,
                },
                ticks: 64,
                window: Window {
                    from: 0,
                    ticks: close,
                },
                seed: u64::from(round),
                reward_q16: 0,
                answers: [0, 1],
                feedback: Feedback::Withheld,
                delivery: Delivery::Global,
                critic: None,
                hold: Some(hold),
                exploration: Exploration::Unset,
            };
            // A readout cued in two rounds of three, each readout in turn.
            let cued = [Some(0usize), Some(1), None][(round % 3) as usize];
            let mut cues = 0u64;
            if let Some(r) = cued {
                let inject = exec.injector();
                for unit in t.readout.sets()[r].units() {
                    for _ in 0..2 {
                        inject.inject(unit, cue).unwrap();
                        cues = cues.saturating_add(1);
                    }
                }
            }
            let times = (0..64u32)
                .filter(|&k| {
                    k >= close && k < until && k.wrapping_sub(close).checked_rem(every) == Some(0)
                })
                .count() as u32;
            let outcome = t.trial(&mut exec, 0).unwrap();
            assert_eq!(outcome.selection, cued.map(|r| r as u8), "{hold:?}");
            let expected = if cued.is_some() {
                times.saturating_mul(messages).saturating_mul(4)
            } else {
                0
            };
            assert!(cued.is_none() || times >= 1, "the close itself is due");
            assert_eq!(outcome.held, expected, "{hold:?} closing at {close}");
            assert_eq!(
                exec.delivered(),
                8u64.saturating_add(cues)
                    .saturating_add(u64::from(expected)),
                "{hold:?}: the ring drained what was sent"
            );
        }
        for &k in U32_LATTICE.iter() {
            for &close in U32_LATTICE.iter() {
                for &until in U32_LATTICE.iter() {
                    for every in [0u32, 1, 2, 3, 8, 255, 256, u32::MAX] {
                        let h = Hold {
                            until,
                            every,
                            messages: 1,
                            efficacy_q16: -1,
                        };
                        let since = k.checked_sub(close);
                        assert_eq!(
                            h.is_due(close, k),
                            k < until && since.is_some_and(|d| d.checked_rem(every) == Some(0)),
                            "{h:?} closing at {close}, at {k}"
                        );
                    }
                }
            }
        }
    }

    /// The critic over the lattice (ADR-0107): for every reward magnitude of the `i32` lattice
    /// at or above zero, either sign, every expectation of the lattice within the magnitude
    /// and the bound's two ends, and shifts from zero to past the width, the error is the
    /// difference taken wide and clamped to the width, the expectation after is the one
    /// before plus the floor of the error over $2^{\text{shift}}$ taken by `div_euclid` in
    /// `i64` and not by a shift, and it lies between the one before and the reward, so within
    /// the bound; and seeded walks of signed rewards from seeded expectations keep every
    /// expectation within its bound at every step.
    #[test]
    fn the_expectation_moves_toward_the_reward_and_stays_within_it() {
        const SHIFTS: [u32; 10] = [0, 1, 4, 5, 6, 30, 31, 32, 62, u32::MAX];
        let one = |reward: i32, expected: i32, shift: u32| {
            let mut c = Critic {
                expected_q16: [!expected, expected],
                shift,
            };
            let (error, before, after) = c.predict(1, reward);
            let wide = i64::from(reward)
                .saturating_sub(i64::from(expected))
                .clamp(i64::from(i32::MIN), i64::from(i32::MAX));
            let divisor = 1i64.checked_shl(shift.min(62)).unwrap();
            let moved = i64::from(expected).saturating_add(wide.div_euclid(divisor));
            let case = format!("{reward} {expected} {shift}");
            assert_eq!(i64::from(error), wide, "{case}");
            assert_eq!(before, expected, "{case}");
            assert_eq!(i64::from(after), moved, "{case}");
            assert!(
                (expected.min(reward)..=expected.max(reward)).contains(&after),
                "{case}: toward the reward and never past it: {after}"
            );
            assert_eq!(c.expected_q16, [!expected, after], "{case}");
            after
        };
        for &magnitude in I32_LATTICE.iter().filter(|&&m| m >= 0) {
            for reward in [magnitude, magnitude.saturating_neg()] {
                let ends = [magnitude, magnitude.saturating_neg()];
                for &expected in I32_LATTICE
                    .iter()
                    .filter(|v| v.unsigned_abs() <= magnitude.unsigned_abs())
                    .chain(ends.iter())
                {
                    for &shift in &SHIFTS {
                        let after = one(reward, expected, shift);
                        assert!(after.unsigned_abs() <= magnitude.unsigned_abs());
                    }
                }
            }
        }
        let mut lcg = Lcg::new(37);
        for _ in 0..256 {
            let magnitude = lcg.i32_edge_biased().saturating_abs();
            let span = u64::from(magnitude.unsigned_abs())
                .saturating_mul(2)
                .saturating_add(1);
            let mut expected = (lcg.next_u64().checked_rem(span).unwrap() as i64)
                .saturating_sub(i64::from(magnitude)) as i32;
            let shift = lcg.pick(&SHIFTS);
            for _ in 0..64 {
                let reward = if lcg.below(2) == 0 {
                    magnitude
                } else {
                    magnitude.saturating_neg()
                };
                expected = one(reward, expected, shift);
                assert!(
                    expected.unsigned_abs() <= magnitude.unsigned_abs(),
                    "{magnitude} {shift}: {expected}"
                );
            }
        }
    }

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
                Set::contiguous(a_first, a_len),
                Set::contiguous(b_first, b_len),
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
            assert_eq!(
                readout.count_window(&train, start, ticks),
                expected,
                "the whole trial as a window is the trial's count"
            );
            // A sub-window of the trial: the filter over it, with entries after it in the
            // train.
            let from = lcg.below(ticks);
            let len = lcg.below(ticks.wrapping_sub(from)).saturating_add(1);
            let window_start = start.wrapping_add(from);
            let expected = [
                train
                    .iter()
                    .filter(|&&(t, u)| {
                        t.wrapping_sub(window_start) < len && readout.sets()[0].contains(u)
                    })
                    .count() as u32,
                train
                    .iter()
                    .filter(|&&(t, u)| {
                        t.wrapping_sub(window_start) < len && readout.sets()[1].contains(u)
                    })
                    .count() as u32,
            ];
            assert_eq!(
                readout.count_window(&train, window_start, len),
                expected,
                "{from} {len}"
            );
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
        let mut readout = Readout::new([Set::contiguous(0, 1), Set::contiguous(1, 1)]);
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

    /// The selection over the lattice of counts (ADR-0151, ADR-0152). Among three, for every
    /// triple of the lattice — within the width, at it and beyond it — and over seeded triples
    /// of small counts, where ties are common: the hand rule, the one channel whose count as
    /// a drive is above both others', or none where the largest is shared; and every
    /// channel's drives, output and flag as the rule leaves them. Among two, for every pair of
    /// the `u32` lattice: the rule in place before ADR-0152 written out — each channel's own
    /// count its direct drive and the other's its indirect — the same selection and the same
    /// two channels, field by field.
    #[test]
    fn the_selection_is_the_largest_count_alone_over_the_lattice() {
        const COUNTS: [u32; 9] = [0, 1, 2, 3, 100, 32_766, 32_767, 32_768, u32::MAX];
        // A count as its drive, by hand: one a spike up to the width's last whole count, and
        // the width beyond it.
        let drive = |count: u32| -> i64 {
            if count <= 32_767 {
                i64::from(count) * 65_536
            } else {
                i64::from(i32::MAX)
            }
        };
        let mut three = Readout::new([
            Set::contiguous(0, 1),
            Set::contiguous(1, 1),
            Set::contiguous(2, 1),
        ]);
        let mut hand = |counts: [u32; 3]| -> Option<u8> {
            let d = counts.map(drive);
            let expected = if d[0] > d[1] && d[0] > d[2] {
                Some(0)
            } else if d[1] > d[0] && d[1] > d[2] {
                Some(1)
            } else if d[2] > d[0] && d[2] > d[1] {
                Some(2)
            } else {
                None
            };
            assert_eq!(three.select(counts), expected, "{counts:?}");
            let others = [d[1].max(d[2]), d[0].max(d[2]), d[0].max(d[1])];
            for (k, channel) in three.channels().iter().enumerate() {
                assert_eq!(
                    (
                        i64::from(channel.striatal_d1_drive),
                        i64::from(channel.striatal_d2_drive),
                        channel.stn_hyperdirect_drive,
                        i64::from(channel.gpi_snr_inhibition),
                        channel.selected_flag,
                    ),
                    (
                        d[k],
                        others[k],
                        0,
                        others[k] - d[k],
                        u32::from(expected == Some(k as u8)),
                    ),
                    "{counts:?} channel {k}"
                );
            }
            expected
        };
        let mut seen = [0u32; 4];
        for &a in &COUNTS {
            for &b in &COUNTS {
                for &c in &COUNTS {
                    let selected = hand([a, b, c]);
                    seen[selected.map_or(3, usize::from)] += 1;
                }
            }
        }
        assert_eq!(
            seen,
            [189, 189, 189, 162],
            "each channel as often, and a shared largest in 162 of 729"
        );
        let mut lcg = Lcg::new(43);
        let mut ties = 0u32;
        for _ in 0..4096 {
            let counts = [lcg.below(6), lcg.below(6), lcg.below(6)];
            ties += u32::from(hand(counts).is_none());
        }
        assert!(
            (800..1150).contains(&ties),
            "about 51 in 216 of 4 096: {ties}"
        );
        // Among two: the composition before ADR-0152, on channels of its own.
        let mut two = Readout::new([Set::contiguous(0, 1), Set::contiguous(1, 1)]);
        let mut before = *two.channels();
        for &a in U32_LATTICE.iter().chain(COUNTS.iter()) {
            for &b in U32_LATTICE.iter().chain(COUNTS.iter()) {
                let drives = [drive(a) as i32, drive(b) as i32];
                let mut selected = [false; 2];
                for (k, channel) in before.iter_mut().enumerate() {
                    let other = if k == 0 { 1 } else { 0 };
                    channel.striatal_d1_drive = drives[k];
                    channel.striatal_d2_drive = drives[other];
                    channel.stn_hyperdirect_drive = 0;
                    selected[k] = channel.compute_gating();
                }
                let expected = match selected {
                    [true, false] => Some(0),
                    [false, true] => Some(1),
                    _ => None,
                };
                assert_eq!(two.select([a, b]), expected, "{a} {b}");
                assert_eq!(*two.channels(), before, "{a} {b}");
            }
        }
    }

    /// The flip over every mapping (ADR-0152): among one, two, three, four and the most
    /// readouts a task can hold, for every pair of answers below the number of readouts, each
    /// answer after a flip is the next index by a hand rule, the remainder of one more over
    /// the number; as many flips as there are readouts bring the mapping back; and `check`'s
    /// refusal of an answer is the index at or beyond the number, for every `u8`.
    #[test]
    fn a_flip_is_the_next_readout_by_the_remainder_over_every_mapping() {
        fn over<const N: usize>() {
            let stimulus = |first: u32| Stimulus {
                set: Set::contiguous(first, 1),
                messages: 1,
                efficacy_q16: 0x0001_4000,
                cancel: None,
            };
            // The readouts' sets one unit each from unit 2, the stimuli at units 0 and 1.
            let sets: [Set; N] = core::array::from_fn(|k| Set::contiguous(k as u32 + 2, 1));
            let exec = Executor::<8>::new(Config {
                workers: 1,
                units: 66,
                injector_capacity: 64,
                train_capacity: 132,
                modulation_baseline_q16: 0,
                ..Config::default()
            })
            .unwrap();
            let task = |answers: [u8; 2]| Task {
                stimuli: [stimulus(0), stimulus(1)],
                readout: Readout::new(sets),
                drive: Drive {
                    every: 0,
                    messages: 0,
                    efficacy_q16: 0,
                    units: 66,
                    seed: 0,
                },
                ticks: 64,
                window: Window::whole(64),
                seed: 0,
                reward_q16: 0,
                answers,
                feedback: Feedback::Withheld,
                delivery: Delivery::Global,
                critic: None,
                hold: None,
                exploration: Exploration::Unset,
            };
            let next = |a: u8| (usize::from(a) + 1).checked_rem(N).unwrap() as u8;
            for a in 0..N as u8 {
                for b in 0..N as u8 {
                    let mut t = task([a, b]);
                    t.flip();
                    assert_eq!(t.answers, [next(a), next(b)], "{N}: {a} {b}");
                    assert!(
                        t.answers.iter().all(|&answer| usize::from(answer) < N),
                        "{N}: a flip keeps the answers inside"
                    );
                    for _ in 1..N {
                        t.flip();
                    }
                    assert_eq!(t.answers, [a, b], "{N}: as many flips as readouts");
                }
            }
            for a in 0..=u8::MAX {
                let refused = usize::from(a) >= N;
                for answers in [[a, 0], [0, a]] {
                    assert_eq!(
                        task(answers).check(&exec),
                        if refused {
                            Err(TaskError::AnswerOutsideReadout)
                        } else {
                            Ok(())
                        },
                        "{N}: {answers:?}"
                    );
                }
                if refused {
                    let mut t = task([a, a]);
                    t.flip();
                    assert_eq!(t.answers, [0, 0], "{N}: {a} moves to the first readout");
                }
            }
        }
        over::<1>();
        over::<2>();
        over::<3>();
        over::<4>();
        over::<64>();
    }

    /// The exploration's draws over the lattice (ADR-0163): for every seed and every trial
    /// built from the `u32` lattice's words, and over seeded pairs, the coin is the hand rule —
    /// the draw's fifth and sixth bytes from the top, which hold bits 31 to 16 — and the
    /// channel's draw the third and fourth, bits 47 to 32, without the lowest, dealt among two,
    /// three and four channels by the division; each is the rule of a draw with the
    /// stimulus's bit, the shuffled coin's and the misleading coin's three flipped, so it reads
    /// none of them, and the three draws beside them are the ones they were. Over the lattice
    /// of values and magnitudes and seeded coins, a coin draws by a hand rule in `i128`; and
    /// over seeded draws among one to sixty-four channels, the channel is the division's,
    /// below the channels.
    #[test]
    fn the_exploration_s_draws_are_the_hand_rules_over_the_lattice() {
        let coin = |draw: u64| {
            let b = draw.to_be_bytes();
            u16::from_be_bytes([b[4], b[5]])
        };
        let field = |draw: u64| {
            let b = draw.to_be_bytes();
            u16::from_be_bytes([b[2], b[3]]) / 2
        };
        let among = |draw: u64, channels: u32| (u32::from(field(draw)) * channels / 32_768) as u8;
        let stimulus = |first: u32| Stimulus {
            set: Set::contiguous(first, 4),
            messages: 2,
            efficacy_q16: 0x0001_4000,
            cancel: None,
        };
        // Tasks whose draws are read and whose trials are never run, of two, three and four
        // readouts.
        fn drawing<const N: usize>(stimuli: [Stimulus; 2], seed: u64) -> Task<N> {
            Task {
                stimuli,
                readout: Readout::new(core::array::from_fn(|k| {
                    Set::contiguous(16 + 4 * k as u32, 4)
                })),
                drive: Drive {
                    every: 0,
                    messages: 0,
                    efficacy_q16: 0,
                    units: 16,
                    seed: 0,
                },
                ticks: 64,
                window: Window::whole(64),
                seed,
                reward_q16: 0x4000,
                answers: [0, 0],
                feedback: Feedback::SevenInEight,
                delivery: Delivery::Global,
                critic: None,
                hold: None,
                exploration: Exploration::ValueGated,
            }
        }
        let stimuli = [stimulus(0), stimulus(8)];
        let others = MISLEADING_BITS | 0x0000_0001_0000_0001;
        let one = |seed: u64, trial: u64| {
            let (two, three, four) = (
                drawing::<2>(stimuli, seed),
                drawing::<3>(stimuli, seed),
                drawing::<4>(stimuli, seed),
            );
            let draw = mix64(seed ^ trial);
            assert_eq!(
                two.exploration_coin_at(trial),
                coin(draw),
                "{seed:#x} {trial:#x}"
            );
            assert_eq!(three.exploration_coin_at(trial), coin(draw));
            assert_eq!(
                (
                    two.drawn_at(trial),
                    three.drawn_at(trial),
                    four.drawn_at(trial)
                ),
                (among(draw, 2), among(draw, 3), among(draw, 4)),
                "{seed:#x} {trial:#x}"
            );
            assert_eq!(
                (coin(draw), field(draw)),
                (coin(draw ^ others), field(draw ^ others)),
                "{seed:#x} {trial:#x}: no bit of the stimulus's, the shuffled coin's or the misleading coin's"
            );
            assert_eq!(
                (coin(draw), field(draw)),
                (
                    coin(draw ^ EXPLORATION_CHANNEL_BITS),
                    field(draw ^ EXPLORATION_COIN_BITS)
                ),
                "{seed:#x} {trial:#x}: neither reads the other's bits"
            );
            // The three draws beside them are the ones they were.
            assert_eq!(two.stimulus_at(trial), (draw % 2) as u8);
            assert_eq!(two.coin_at(trial), (draw >> 32) % 2 == 1);
            assert_eq!(two.misleading_at(trial), draw.to_be_bytes()[1] % 8 == 0);
            (coin(draw), among(draw, 3))
        };
        let words = |high: u32, low: u32| u64::from(high) << 32 | u64::from(low);
        for &a in U32_LATTICE.iter() {
            for &b in U32_LATTICE.iter() {
                for &c in U32_LATTICE.iter() {
                    one(words(a, b), words(b, c));
                    one(words(c, a), u64::from(b));
                    one(u64::from(a), words(c, b));
                }
            }
        }
        let mut lcg = Lcg::new(43);
        let mut below_half = 0u32;
        let mut channels = [0u32; 3];
        for _ in 0..4096 {
            let (c, r) = one(lcg.next_u64(), lcg.next_u64());
            below_half += u32::from(c < 0x8000);
            channels[usize::from(r)] += 1;
        }
        assert!(
            (1_900..=2_200).contains(&below_half),
            "{below_half}: about half the coins below half the width"
        );
        assert!(
            channels.iter().all(|n| (1_250..=1_480).contains(n)),
            "{channels:?}: about a third each"
        );
        // Each bit alone, by the masks themselves.
        for bit in 0..64u32 {
            let draw = 1u64 << bit;
            assert_eq!(
                draw & EXPLORATION_COIN_BITS != 0,
                (16..=31).contains(&bit),
                "bit {bit}"
            );
            assert_eq!(
                draw & EXPLORATION_CHANNEL_BITS != 0,
                (33..=47).contains(&bit),
                "bit {bit}"
            );
            assert_eq!(coin(draw) != 0, (16..=31).contains(&bit), "bit {bit}");
            assert_eq!(field(draw) != 0, (33..=47).contains(&bit), "bit {bit}");
        }
        // The coin against the value: the hand rule in `i128`, over the lattice of values and
        // magnitudes with the coin's edges, and over a seeded walk.
        let hand = |coin: u16, value: i32, reward: i32| {
            let below = if value < 0 {
                (-i128::from(value)).min(i128::from(reward))
            } else {
                0
            };
            reward > 0 && i128::from(coin) * i128::from(reward) < below * 65_536
        };
        let mut drawn = [0u32; 2];
        for &value in I32_LATTICE.iter() {
            for &reward in I32_LATTICE.iter() {
                for c in [
                    0u16, 1, 3, 4, 0x3FFF, 0x4000, 0x7FFF, 0x8000, 0xFFFE, 0xFFFF,
                ] {
                    let got = Exploration::draws(c, value, reward);
                    assert_eq!(got, hand(c, value, reward), "{c} {value} {reward}");
                    drawn[usize::from(got)] += 1;
                }
            }
        }
        for _ in 0..20_000 {
            let (c, value, reward) = (lcg.next_u16(), lcg.i32_edge_biased(), lcg.i32_edge_biased());
            let got = Exploration::draws(c, value, reward);
            assert_eq!(got, hand(c, value, reward), "{c} {value} {reward}");
            drawn[usize::from(got)] += 1;
            // Within a reward of 1.0 the coin is the value's part below zero itself.
            let near = lcg.next_i32() % 0x0002_0000;
            assert_eq!(
                Exploration::draws(c, near, 0x0001_0000),
                near < 0 && i64::from(c) < i64::from(-near).min(0x0001_0000),
                "{c} {near}"
            );
        }
        assert!(
            drawn[0] > 1_000 && drawn[1] > 1_000,
            "both sides: {drawn:?}"
        );
        // The channel: the division's, below the channels, for every number a readout holds.
        for _ in 0..20_000 {
            let draw = lcg.next_u16();
            let n = lcg.below(MAX_CHANNELS as u32) as usize + 1;
            let got = Exploration::channel(draw, n);
            assert_eq!(
                usize::from(got),
                usize::from(draw % 0x8000) * n / 32_768,
                "{draw} among {n}"
            );
            assert!(usize::from(got) < n, "{draw} among {n}");
        }
    }
}
