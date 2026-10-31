//! The executor: a fixed pool of workers that run the turns, the fan-out and the deliveries of
//! whitepaper R-1 for every unit, tick by tick (ADR-0023).
//!
//! A tick has three phases separated by barriers, so that the references a worker holds into
//! the shared arenas never alias another worker's:
//!
//! 1. **Turns.** Each worker owns a fixed, contiguous range of the unit arena and serves, in
//!    unit order, the units of its range the schedule names: one bit per unit, set by the
//!    unit's owner when its turn left it awake and by a push or an activation that found it
//!    idle (ADR-0100). For each, the worker drains the mailbox if it holds mail, orders the
//!    batch by message value (§8.3), scales the two compartment sums by the tick's synaptic
//!    gain (ADR-0036), integrates (under the image's slow current for a unit marked for it,
//!    ADR-0123), steps the short-term plasticity if the unit fired (under the image's class for
//!    a unit marked facilitating, ADR-0114), and keeps the unit on the
//!    schedule for the next tick while it is not at rest. Only the owner references the unit,
//!    exclusively, and no push happens in the phase, so the turn takes no claim. At the end of
//!    the phase the worker publishes how many of its units fired.
//! 2. **Fan-out.** For each unit that fired in phase 1, the worker that ran it walks its chain:
//!    per block, STDP against the targets' last spikes (settled, since every turn has ended)
//!    into the eligibility traces, the traces consolidated into the weights under the tick's
//!    modulation (ADR-0032), the release under the unit's factors, then each synapse into the
//!    target's mailbox now (delay 0) or into this worker's wheel (`synapse_token`). Blocks are
//!    referenced exclusively by the worker that owns the spiking unit; units only shared.
//! 3. **Deliveries.** Each worker advances its wheel and pushes the due tokens' stored releases
//!    into the targets' mailboxes as spike messages; worker 0 also drains the injector and,
//!    during slow-wave sleep on the ripple's cadence, delivers the replay drive to every unit
//!    of the episode the coordinator chose before the tick (ADR-0038). Blocks and units only
//!    shared. A unit a push wakes is marked on the schedule for the next tick.
//!
//! A message pushed in phase 2 or 3 of tick $t$ is integrated in phase 1 of tick $t + 1$, so a
//! zero-delay synapse and a one-tick one arrive together; a delay $d$ scheduled in phase 2 is
//! due at $t + d$. Between ticks the coordinator merges the units the workers fired, in unit
//! order, into its own bounded train (ADR-0050), sums the workers' spike counts into the
//! homeostasis record's open bin, closes the bin on its cadence, regulates the gain on the
//! window's and steps the sleep stage (ADR-0035, ADR-0036, ADR-0037), and, awake on the
//! search's cadence, runs the discovery loop over its own clause store (ADR-0052): the search
//! from its cursor, the committed rewards into the modulator, the coincidence before the
//! reward tagged from its own train and bound to the invented predicate; before a tick's
//! first barrier it decides the ripple (ADR-0038): a schedule that is a function of the tick,
//! so it adds no barrier and runs at the same ticks on every worker count. Nothing allocates
//! after [`Executor::new`], nothing blocks but the spin barrier, and the only system call in
//! the loop is the barrier's yield.

use crate::arena::Arena;
use crate::barrier::SpinBarrier;
use crate::discovery::Discovery;
use crate::episode::{COINCIDENCE_TICKS, DISCOVERY_WINDOW, DiscoverError, DiscoverReport};
use crate::image::{ImageError, WriteAheadLog};
use crate::injector::{self, Injector};
use crate::pool::Pools;
use crate::store::{Induction, TermError};
use menteion_affect::InteroceptiveState;
use menteion_core::{
    BASAL_LEAK_SHIFT, CHAIN_END, Cadence, DendriticSuperNeuron, DepressionExponent,
    FLAG_FACILITATING, FLAG_SLOW, FlatTimingWheel, GateState, ISTDP_PERIOD_MAX_TICKS,
    ISTDP_PERIOD_MIN_TICKS, ISTDP_TARGET_PERIOD_TICKS, MODULATION_ONE_Q16, NO_SPIKE_ON_RECORD,
    PlasticDelta, Polarity, REFRACTORY_TICKS, SYNAPSES_PER_BLOCK, SlowCurrent, StpClass,
    SynapseBlock, THRESHOLD_BASE, WorkerWheel, istdp_alpha_q1_15, message_efficacy_q16,
    message_is_apical, message_is_synaptic, spike_message, synapse_token, synaptic_message,
    token_block, token_slot,
};
use menteion_ethics::EthicalEvaluationGate;
use menteion_executive::{
    AMENDMENT_PROPOSED, PARAM_SWEEP_BUDGET, PARAM_SWEEP_QUIET_TICKS, PolicyAmendment, spec_of,
};
use menteion_hippocampus::{Episode, HippocampalAttractorState, PATTERN_MAX, RIPPLE_SHIFT};
use menteion_homeostasis::{
    ACTIVITY_BIN_SHIFT, ACTIVITY_COUNT_MAX, ACTIVITY_WINDOW_BINS, ACTIVITY_WINDOW_SHIFT,
    CONTROL_STEP_MAX_Q0_16, GAIN_MAX_Q16, GAIN_MIN_Q16, GAIN_ONE_Q16, HomeostaticDrivePool,
    SLEEP_SHIFT_MAX, STAGE_AWAKE, STAGE_REM, STAGE_SWS,
};
use menteion_neuromod::{DOPAMINE_TAU_SHIFT, NeuromodulatorState, ValueCritic};
use menteion_reasoning::{Compaction, InductionState, SEARCH_SHIFT_MAX, TermNode};
use std::cell::Cell;
use std::collections::VecDeque;
use std::path::Path;
use std::sync::Arc;
use std::sync::atomic::{
    AtomicBool, AtomicI32, AtomicI64, AtomicU8, AtomicU32, AtomicU64, AtomicUsize, Ordering,
};
use std::thread::{self, JoinHandle};

/// How the executor is sized. Every capacity is allocated once in [`Executor::new`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Config {
    /// Worker threads, at least one. Worker 0 is the thread that calls [`Executor::tick`].
    pub workers: usize,
    /// Units in the arena.
    pub units: usize,
    /// Synapse blocks in the arena.
    pub blocks: usize,
    /// Tier-2 plastic deltas in the arena (stored and read; applying them is Specified).
    pub deltas: usize,
    /// Mailbox nodes per worker: the most messages one worker can have in flight at once. A
    /// tick takes at most one wheel slot's tokens plus the zero-delay synapses of the units
    /// that fired, and, on worker 0, one injector ring's worth and one ripple's worth
    /// (`REPLAY_MESSAGES × PATTERN_MAX`, 24, while the ledger has room; ADR-0038); nodes come
    /// back the tick after. Refused below one ripple's worth when the ledger has room.
    pub nodes_per_worker: usize,
    /// Entries of the injector ring (rounded up to a power of two).
    pub injector_capacity: usize,
    /// Delivered messages and spikes each worker records, for tests and reports; 0 records
    /// nothing.
    pub trace_capacity: usize,
    /// Room for policy amendments (ADR-0031) beyond those an image holds; 0 leaves the engine
    /// unable to propose one.
    pub amendments: usize,
    /// The modulation of three-factor plasticity with the dopamine signal at rest (ADR-0032):
    /// the fraction of each synapse's eligibility trace consolidated into the weight at a
    /// presynaptic spike, Q16.16 in $[0, 1]$. At 1.0 (the default) the rule is ADR-0022's; a
    /// lower baseline leaves the pairings pending for a reward to consolidate. For an engine
    /// built from an image, the image's baseline outranks this one: it changes what the run
    /// does, so it is part of the image (§8.3).
    pub modulation_baseline_q16: i32,
    /// The control step $\kappa$ of criticality control (ADR-0036), Q0.16 at most
    /// `CONTROL_STEP_MAX_Q0_16` (0.5): once per window the synaptic gain moves by
    /// $1 - \kappa\,\operatorname{clamp}(\hat\sigma - 1, -1, 1)$. At 0 (the default) the gain
    /// stays where it started, 1.0 by default (`synaptic_gain_q16`), and at 1.0 the dynamics are
    /// the reference ones. For an engine built from an image,
    /// the image's step and gain outrank this one: they change what the run does (§8.3).
    pub control_step_q0_16: u16,
    /// The synaptic gain an engine starts with (ADR-0196), Q16.16 within the controller's own
    /// bounds, `GAIN_MIN_Q16` to `GAIN_MAX_Q16` (0.25 to 4): [`Executor::new`] puts it in the
    /// homeostasis record, whose gain every turn scales its sums by and the controller moves
    /// while its step is not zero. At 1.0 (`GAIN_ONE_Q16`, the default) the engine is the one it
    /// was before ADR-0196, bit for bit. Refused outside the bounds. For an engine built from an
    /// image, the image's gain outranks this one, as its step does (§8.3).
    pub synaptic_gain_q16: u32,
    /// The sleep shift $k$ of ADR-0037, at most `SLEEP_SHIFT_MAX` (15): the sleep pressure's
    /// time constant is $2^k$ windows awake and $2^{k-2}$ asleep. At 0 (the default) the
    /// pressure and the stage stay where they are: the engine never sleeps and the dynamics
    /// are the reference ones. For an engine built from an image, the image's shift, stage
    /// and pressure outrank this one (§8.3).
    pub sleep_shift: u8,
    /// Room for episodes tagged into the ledger (ADR-0038) beyond those an image holds; 0
    /// leaves the engine unable to tag one.
    pub episodes: usize,
    /// The spikes the executor's own train keeps (ADR-0050): after every tick the coordinator
    /// merges the units every worker fired, in unit order, into a ring of this many
    /// `(tick, unit)` entries, the oldest let go when it is full and counted; read between
    /// ticks by [`Executor::train`]. 0 keeps none and adds nothing to the tick.
    pub train_capacity: usize,
    /// Nodes of the engine's term arena (ADR-0052) beyond those an image holds; 0 (the
    /// default) leaves the engine without an arena, a store or a search.
    pub terms: usize,
    /// Slots of the engine's clause store beyond those an image holds; refused without an
    /// arena.
    pub clauses: usize,
    /// The search's cadence inside the tick, $2^{\text{shift}}$ ticks, while awake (ADR-0052);
    /// 0 (the default) never searches inside the tick, and a caller may still search between
    /// ticks. For an engine built from an image, the image's outranks this one (§8.3).
    pub search_shift: u8,
    /// Attempts one search may spend (ADR-0045); the image's outranks this one.
    pub search_budget: u32,
    /// The REM ripples an invention's episode survives (ADR-0048); at 0 a rewarded search
    /// tags nothing and is counted as untagged. The image's outranks this one.
    pub discovery_tag: u8,
    /// The target period of the inhibitory rule (ADR-0049, ADR-0053), in ticks: the rate an
    /// inhibitory synapse's target is driven toward, as the depression per presynaptic spike
    /// `istdp_alpha_q1_15(period)`; 20 000 (the default) is 5 Hz at the fine tick. Refused
    /// outside `[ISTDP_PERIOD_MIN_TICKS, ISTDP_PERIOD_MAX_TICKS]`. For an engine built from
    /// an image, the image's outranks this one: it changes what the run does (§8.3).
    pub istdp_target_period_ticks: u32,
    /// The inhibitory baseline (ADR-0085, ADR-0086): while set, the modulation every slot of
    /// an inhibitory block consolidates under, `clamp(inhibitory baseline, 0, 1)`, whether or
    /// not the synapse is addressed — the dopamine term never reaches it — while every
    /// excitatory synapse consolidates under `modulation_baseline_q16` and the signal as
    /// before; unset (the default), every synapse consolidates as before, bit for bit.
    /// Q16.16 in [0, 1] when set; refused outside it. For an engine built from an image, the
    /// image's outranks this one: it changes what the run does, so it is part of the image
    /// (§8.3).
    pub inhibitory_baseline_q16: Option<i32>,
    /// The signed gate (ADR-0093, ADR-0094): while set, an addressed excitatory synapse
    /// consolidates under `clamp(baseline + dopamine, -1, 1)` (`menteion-neuromod`'s
    /// `signed_modulation`), so that below zero its weight moves against its trace's sign and
    /// the trace is spent by as much (`SynapseBlock::consolidate_signed`); every other synapse
    /// consolidates as before — an unaddressed one under the baseline alone, an inhibitory one
    /// under the addressed modulation at no less than zero, or under its own baseline while
    /// that is set. Unset (the default), every synapse consolidates as before, bit for bit.
    /// For an engine built from an image, the image's outranks this one: it changes what the
    /// run does, so it is part of the image (§8.3).
    pub signed_gate: bool,
    /// The class of short-term plasticity (ADR-0113, ADR-0114): while set, a unit marked
    /// `FLAG_FACILITATING` steps its short-term state under the class's constants
    /// (`DendriticSuperNeuron::step_stp_class`) and every other unit under ADR-0019's
    /// (`step_stp`); unset (the default), every unit steps under ADR-0019's, marked or not, bit
    /// for bit. Refused unless the rule resolves it (`StpClass::is_valid`). Each worker holds it
    /// from `Executor::new`, so the tick reads no word for it. For an engine built from an image,
    /// the image's outranks this one: it changes what the run does, so it is part of the image
    /// (§8.3).
    pub stp_class: Option<StpClass>,
    /// The slow current (ADR-0122, ADR-0123): while set, a unit marked `FLAG_SLOW` integrates
    /// under its constants (`DendriticSuperNeuron::integrate_slow`), its slow potential taking
    /// the positive efficacies of the synaptic messages that land in its basal compartment,
    /// scaled by the gain; every other unit integrates under `integrate`. Unset (the default),
    /// every unit integrates under `integrate`, marked or not, bit for bit. Refused unless the
    /// rule resolves it (`SlowCurrent::is_valid`). Each worker holds it from `Executor::new`,
    /// so the tick reads no word for it. For an engine built from an image, the image's
    /// outranks this one: it changes what the run does, so it is part of the image (§8.3).
    pub slow_current: Option<SlowCurrent>,
    /// The critic of the engine's own (ADR-0130, ADR-0131): while set, [`Executor::reward`]
    /// takes the value of the reward — each unit's value weight times its spikes since the
    /// previous reward, counted as the coordinator merges them into the train, summed and scaled
    /// (`menteion-neuromod`'s `ValueCritic`) — from the reward, delivers the error to the
    /// modulator in the reward's place and moves every weight by the delta rule, between ticks.
    /// Unset (the default), `reward` is the reward into the modulator, as before, bit for bit.
    /// Refused unless the rule resolves it (`ValueCritic::is_valid`) and without a train to count
    /// the spikes from. For an engine built from an image, the image's outranks this one: it
    /// changes what the run does, so it is part of the image (§8.3).
    pub critic: Option<ValueCritic>,
    /// The critic's window (ADR-0133, ADR-0134), in ticks: while it is not zero, a unit's spike
    /// counts toward its feature only when its tick is fewer than this many ticks after the
    /// previous reward the critic took — after the engine's start, its build or its load,
    /// before the first — so that the value is read from what the network was given after its
    /// last outcome. Zero (the default) counts every spike since the previous reward, the critic
    /// of ADR-0131 bit for bit. Refused while the critic is unset. For an engine built from an
    /// image, the image's outranks this one, and the loader refuses a window that is not the
    /// shortest delay of any synapse the image carries ([`crate::shortest_delay`]).
    pub critic_window_ticks: u16,
    /// The whole punishment (ADR-0154, ADR-0155): while set, a reward below zero that meets a
    /// value below zero reaches the modulator whole (`menteion-neuromod`'s
    /// `ValueCritic::received_q16`) where the critic's error is softer; every other reward
    /// reaches it as the error, and the critic's weights move by the error in every case.
    /// Unset (the default), the modulator receives the error at every reward, the critic of
    /// ADR-0131 bit for bit. Refused while the critic is unset. For an engine built from an
    /// image, the image's outranks this one: it changes what the run does, so it is part of the
    /// image (§8.3).
    pub whole_punishment: bool,
    /// The rate's target (ADR-0178, ADR-0180), a period in ticks: a unit's mean interval at the
    /// rate the controller holds the population to. While it is not zero the gain moves once a
    /// window by the window's count over the target's — the units times the window's ticks over
    /// this period, to nearest ([`Executor::rate_target_per_window`]) — in the estimate's place
    /// (`HomeostaticDrivePool::regulate_rate`): the saturation rule is not consulted, and the
    /// estimate is kept as a reading. Zero (the default) is unset: the controller is ADR-0036's,
    /// bit for bit. Refused when the count is no spike a window or more than a window's bins
    /// count. The step is `control_step_q0_16`, for either rule. For an engine built from an
    /// image, the image's outranks this one: it changes what the run does, so it is part of the
    /// image (§8.3).
    pub rate_target_period_ticks: u32,
    /// The read of the rate's target (ADR-0188, ADR-0189), its span in windows: while it is not
    /// zero the engine holds its gain for this many windows from the first it runs, neither rule
    /// moving it whatever the step (`HomeostaticDrivePool::hold`), and sums the counts of the
    /// last sixteen. At the span's last window their mean, to nearest, is the target: the period
    /// becomes the units times the window's ticks over it, to nearest, and the rate's rule moves
    /// the gain from the next window at `control_step_q0_16`, against that period's count, as
    /// though the period had been `rate_target_period_ticks` (`Executor::target_read`). A mean
    /// whose period the rate's target refuses ends the read with no target, the gain held from
    /// there on. Zero (the default) is unset: the controller is the one before ADR-0189, bit for
    /// bit. Refused below the sixteen windows it sums, and beside a period already set: an image
    /// has one target. For an engine built from an image, the image's read, under way or none,
    /// outranks this one: it changes what the run does, so it is part of the image (§8.3).
    pub rate_read_windows: u8,
    /// The inhibitory rule's target by the same read (ADR-0192, ADR-0194): while set beside a
    /// read of the rate's target, the read's last window sets the inhibitory rule's target period
    /// to the period it sets the rate's to, through the path an image's period takes
    /// (`set_istdp_target_period`), so that both rules run against the read from the next window;
    /// a mean whose period the inhibitory rule refuses, outside `[ISTDP_PERIOD_MIN_TICKS,
    /// ISTDP_PERIOD_MAX_TICKS]`, then ends the read refused. The flag is the read's: it is spent
    /// when the read ends. Unset (the default), a read sets the rate's target alone, as before
    /// ADR-0194, bit for bit. Refused without a read. For an engine built from an image, the
    /// image's, beside its read under way, outranks this one: it changes what the run does, so it
    /// is part of the image (§8.3).
    pub istdp_target_read: bool,
    /// The probe of the step (ADR-0200, ADR-0202): while set beside a read of the rate's target,
    /// the read's end starts a probe in the rule's place. For sixteen windows the record's gain
    /// $g_0$ is held and the workers are given it times $1 - 1/64$, $1 + 1/64$, $1 + 1/64$,
    /// $1 - 1/64$, four, eight and four windows ([`Executor::gain_given_q16`]); the probe sums the
    /// higher windows' counts less the lower ones', $D$. At its last window the step becomes
    /// $\kappa = 4\,096\,c^{*}/D$ in Q0.16, $c^{*}$ the read's target count, which is
    /// $\lambda / S$ with $\lambda = 1/4$ and the steepness $S = 4D/c^{*}$, and the rate's rule
    /// runs from the next window at it. A probe whose $D$ is not above zero, or whose $\kappa$
    /// passes `CONTROL_STEP_MAX_Q0_16`, ends refused, the gain held from there on, as a refused
    /// read ends. The flag is the probe's: it is spent when the probe ends, or when the read ends
    /// refused. Unset (the default), a read ends in the rule at `control_step_q0_16`, as before
    /// ADR-0202, bit for bit. Refused without a read. For an engine built from an image, the
    /// image's, beside its read or its probe under way, outranks this one (§8.3).
    pub step_probe: bool,
    /// The kept addresses (ADR-0207, ADR-0208): the oldest age, in trials, a reward may name.
    /// While it is not zero, a trial's close ([`Executor::keep_address`]) keeps the critic's
    /// counts, whose units not zero are the address's sources, and the targets given, as the
    /// newest entry, then zeroes the counts and reopens the critic's window as a reward does; and
    /// a reward of age $a$ ([`Executor::reward_kept`]) writes the address of the entry kept $a$
    /// closes before the newest, reads the value and moves the critic's weights from that entry's
    /// counts, and spends it. The engine holds this many entries beside the newest. Zero (the
    /// default) keeps none, and every run is the run it was, bit for bit. Refused above
    /// `KEPT_ADDRESSES_MAX`, the eligibility trace's τ in trials, and without the critic or its
    /// window, from which the address is drawn (ADR-0139). Not in the image, as the counts are
    /// not: for an engine built from an image, the configuration's.
    pub kept_addresses: u8,
    /// The delay read (ADR-0216, ADR-0217): while set, a reward given without its age
    /// ([`Executor::reward_untold`]) is read by a value critic per kept age, each predicting
    /// from and learning on the counts of the trial its age closes before the newest, and the
    /// engine consolidates it at the age in use: zero until another age's score leads for
    /// `DELAY_RUN` rewards running, then that age, once and for good. Unset (the default), every
    /// run is the run it was, bit for bit; a reward that names its age
    /// ([`Executor::reward_kept`]) is ADR-0208's, set or not. Refused without the kept
    /// addresses, which need the critic and its window. Not in the image, as the entries are
    /// not: for an engine built from an image, the configuration's.
    pub reads_its_delay: bool,
    /// The exponent of every excitatory pairing's depression (ADR-0226, ADR-0228). At
    /// [`DepressionExponent::One`] (the default) it is ADR-0055's rule, and every run is the run
    /// it was, bit for bit. Any other is refused beside a read of the rate's target, and on an
    /// image with a read or a probe under way: the derivation stays H-42's (ADR-0232). Not in the
    /// image: for an engine built from an image, the configuration's.
    pub depression_exponent: DepressionExponent,
}

/// The oldest age a reward may name (ADR-0207): four trials, the eligibility trace's τ of
/// $2^{16}$ ticks over a trial of $2^{14}$.
pub const KEPT_ADDRESSES_MAX: u8 = 4;

/// The rewards an age's score sums (ADR-0216): the magnitudes of its critic's errors at its
/// last 64 rewards. Scores are compared once every age has as many.
pub const DELAY_WINDOW: usize = 64;

/// The rewards running at which one age other than zero must lead for the engine to move to it
/// (ADR-0216).
pub const DELAY_RUN: u8 = 64;

/// The lead's shift (ADR-0216): an age leads when its score is below every other age's by at
/// least `DELAY_WINDOW` times a thirty-second of the reward's magnitude, $64 \cdot |r| / 2^5$,
/// the mean error a thirty-second of the reward lower over the window.
pub const DELAY_LEAD_SHIFT: u32 = 5;

/// The ages a delay read can hold a critic for: zero to the oldest an engine keeps.
const DELAY_AGES: usize = KEPT_ADDRESSES_MAX as usize + 1;

impl Default for Config {
    fn default() -> Self {
        Self {
            workers: 1,
            units: 1,
            blocks: 0,
            deltas: 0,
            nodes_per_worker: 64,
            injector_capacity: 64,
            trace_capacity: 0,
            amendments: 0,
            modulation_baseline_q16: MODULATION_ONE_Q16,
            control_step_q0_16: 0,
            synaptic_gain_q16: GAIN_ONE_Q16,
            sleep_shift: 0,
            episodes: 0,
            train_capacity: 0,
            terms: 0,
            clauses: 0,
            search_shift: 0,
            search_budget: 0,
            discovery_tag: 0,
            istdp_target_period_ticks: ISTDP_TARGET_PERIOD_TICKS,
            inhibitory_baseline_q16: None,
            signed_gate: false,
            stp_class: None,
            slow_current: None,
            critic: None,
            critic_window_ticks: 0,
            whole_punishment: false,
            rate_target_period_ticks: 0,
            rate_read_windows: 0,
            istdp_target_read: false,
            step_probe: false,
            kept_addresses: 0,
            reads_its_delay: false,
            depression_exponent: DepressionExponent::One,
        }
    }
}

impl Config {
    /// The engine's default for an image no derivation has settled (ADR-0196, ADR-0205):
    /// `Config::default()` with six fields set, each what H-41 or H-42 read of the network's own
    /// regime on three networks at 1 024 units (ADR-0195, ADR-0204), and no other:
    /// - the modulation at rest 0.5, every rewarded run's lead-in's since brief 027;
    /// - the gain 1.75, the calibration's first candidate, calibrated on every network derived;
    /// - the controller's step a sixteenth (H-39, H-40, H-41), which no window reads while the
    ///   probe is set, and which H-42's runs carried;
    /// - the read of the rate's target over 128 windows (H-40, H-41);
    /// - the read's flag, so that its end sets the inhibitory rule's target too (H-41);
    /// - the probe of the step, so that the rate's rule runs at the step the engine makes from
    ///   its own loop in the sixteenth's place (H-42).
    ///
    /// An engine born under it holds its gain at 1.75 for 128 windows, sets both targets from
    /// the mean count of the last sixteen, probes its loop for sixteen windows more and runs the
    /// rate's rule at the step the probe made from the 145th. The sizing is the caller's, by
    /// struct update, and the learning configuration stays unset. `Config::default()` stays the
    /// bare engine, every rule unset, that every pin runs.
    pub fn reads_its_targets() -> Self {
        Self {
            modulation_baseline_q16: 0x8000,
            synaptic_gain_q16: 0x0001_C000,
            control_step_q0_16: 0x1000,
            rate_read_windows: 128,
            istdp_target_read: true,
            step_probe: true,
            ..Self::default()
        }
    }

    /// The configuration an engine runs a settled image under (ADR-0232): the engine's default
    /// with `depression_exponent` at [`DepressionExponent::Quarter`], and no other field. An image
    /// outranks every value the default sets, so the exponent is the one field it adds to a
    /// decoded engine; the sizing is the caller's, by struct update, `train_capacity` among it. No
    /// engine is born under it: the read it carries is refused beside the exponent.
    ///
    /// The exponent alone is not what was measured. H-46 ran it with the zero sum, whose groups
    /// no configuration names: the host reads each group's value once
    /// ([`Executor::coupling_sum`]), keeps it and sets the groups at each load
    /// ([`Executor::set_zero_sum`]). An engine under this name with no groups set runs a state no
    /// run has read, and nothing refuses it. The bound is the zero sum's and not this name's.
    pub fn runs_a_settled_image() -> Self {
        Self {
            depression_exponent: DepressionExponent::Quarter,
            ..Self::reads_its_targets()
        }
    }
}

/// The engine's policy (ADR-0031): the parameters its rules take that it may amend by itself,
/// each an entry of `menteion_executive::REGISTRY`. Read between ticks by the clock sweep;
/// changed only by [`Executor::commit`], after the amendment's trial, or by the loader from
/// the committed amendments an image holds.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Policy {
    /// Ticks a unit must have been quiet before [`Executor::sweep_by_policy`] evicts it
    /// (`PARAM_SWEEP_QUIET_TICKS`).
    pub sweep_quiet_ticks: u32,
    /// The most units one [`Executor::sweep_by_policy`] evicts (`PARAM_SWEEP_BUDGET`).
    pub sweep_budget: u32,
}

impl Default for Policy {
    /// Quiet for 10 000 ticks (100 ms at the fine tick), at most 1 024 units per sweep.
    fn default() -> Self {
        Self {
            sweep_quiet_ticks: 10_000,
            sweep_budget: 1_024,
        }
    }
}

impl Policy {
    /// The live value of a registered parameter; `None` for one the registry does not name.
    pub fn value(&self, parameter: u16) -> Option<i32> {
        match parameter {
            PARAM_SWEEP_QUIET_TICKS => Some(self.sweep_quiet_ticks.min(i32::MAX as u32) as i32),
            PARAM_SWEEP_BUDGET => Some(self.sweep_budget.min(i32::MAX as u32) as i32),
            _ => None,
        }
    }

    /// Sets a registered parameter to a value within its bounds; refused, with nothing
    /// changed, for an unregistered parameter or a value outside them.
    pub fn set(&mut self, parameter: u16, value: i32) -> bool {
        if !spec_of(parameter).is_some_and(|s| s.holds(value)) {
            return false;
        }
        match parameter {
            PARAM_SWEEP_QUIET_TICKS => self.sweep_quiet_ticks = value as u32,
            PARAM_SWEEP_BUDGET => self.sweep_budget = value as u32,
            _ => return false,
        }
        true
    }
}

/// Why an amendment operation is refused (ADR-0031).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AmendError {
    /// The index names no amendment.
    NoSuchAmendment,
    /// The arena has no room for another proposal (`Config::amendments`).
    ArenaFull,
    /// The veto gate was evaluated on another proposal: its `proposal_action_id` is not this
    /// amendment's id.
    WrongProposal,
    /// The amendment is not proposed, so the veto gate has nothing to admit.
    NotProposed,
    /// The amendment is not admitted, so there is nothing to trial.
    NotAdmitted,
    /// The amendment is not trialled with every gate passed.
    NotCommittable,
    /// A later proposal for the same parameter was committed first: the arena is the log the
    /// loader replays in order, so commits to one parameter happen in index order.
    Superseded,
    /// The live value is no longer the one the trial started from: another commit came
    /// between, and the trial did not test this one on top of it.
    Stale,
}

/// Why a configuration is refused.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConfigError {
    /// `workers` is zero.
    NoWorkers,
    /// `units` is zero.
    NoUnits,
    /// `nodes_per_worker` is zero.
    NoNodes,
    /// More units than a mailbox payload or a `u32` index can name.
    TooManyUnits,
    /// More blocks than a synapse token can name (`MAX_TOKEN_BLOCK + 1`; finding F-23).
    TooManyBlocks,
    /// `modulation_baseline_q16` is outside $[0, 1]$ (ADR-0032).
    ModulationOutOfRange,
    /// `control_step_q0_16` is above `CONTROL_STEP_MAX_Q0_16` (ADR-0036).
    ControlStepOutOfRange,
    /// `synaptic_gain_q16` is outside the controller's bounds, `GAIN_MIN_Q16` to `GAIN_MAX_Q16`
    /// (ADR-0196).
    GainOutOfRange,
    /// `sleep_shift` is above `SLEEP_SHIFT_MAX` (ADR-0037).
    SleepShiftOutOfRange,
    /// More episodes than a `u32` index can name (ADR-0038).
    TooManyEpisodes,
    /// Fewer mailbox nodes per worker than one ripple's deliveries take on worker 0
    /// (`REPLAY_MESSAGES × PATTERN_MAX`, 24) while the ledger has room (ADR-0038): a ripple
    /// would exhaust the pool and abort the process.
    TooFewNodes,
    /// More term nodes or clause slots than a `u32` index can name (ADR-0052).
    TooManyTerms,
    /// A clause store without a term arena (`clauses` above zero with `terms` zero).
    ClausesWithoutArena,
    /// `search_shift` is above `SEARCH_SHIFT_MAX` (ADR-0052).
    SearchShiftOutOfRange,
    /// `istdp_target_period_ticks` is outside `[ISTDP_PERIOD_MIN_TICKS,
    /// ISTDP_PERIOD_MAX_TICKS]` (ADR-0053).
    IstdpPeriodOutOfRange,
    /// `inhibitory_baseline_q16` is set outside [0, 1] (ADR-0086).
    InhibitoryBaselineOutOfRange,
    /// `stp_class` is set to a class the rule does not resolve: a $U$ of zero, or a shift
    /// outside 1 to 16 (ADR-0114).
    StpClassOutOfRange,
    /// `slow_current` is set to constants the rule does not resolve: a leak shift outside 1 to
    /// 16, an input shift above 16, or voltages outside `0 < V_lo < V_hi <= THRESHOLD_BASE`
    /// (ADR-0123).
    SlowCurrentOutOfRange,
    /// `critic` is set to constants the rule does not resolve: a shift above
    /// `CRITIC_SHIFT_MAX` or a scale above `CRITIC_SCALE_MAX` (ADR-0131).
    CriticOutOfRange,
    /// `critic` is set with no train (`train_capacity` zero): the critic's features are the
    /// spikes the coordinator merges into the train (ADR-0131).
    CriticWithoutTrain,
    /// `critic_window_ticks` is set while `critic` is not: the window gates the critic's
    /// features and nothing else (ADR-0134).
    WindowWithoutCritic,
    /// `whole_punishment` is set while `critic` is not: it is a rule of what the modulator
    /// receives at the critic's reading and of nothing else (ADR-0155).
    WholePunishmentWithoutCritic,
    /// `rate_target_period_ticks` is set to a period whose count for a window of this many
    /// units is zero, a target of no spike, or above what a window's bins count (ADR-0180).
    /// Since ADR-0189 also what a read of the target that ended on such a count raises, at the
    /// write of its image.
    RateTargetOutOfRange,
    /// `rate_read_windows` is set below the sixteen windows the read sums (ADR-0189).
    RateReadOutOfRange,
    /// `rate_read_windows` is set beside a period already set in `rate_target_period_ticks`, or
    /// an image carries a read under way beside a period: an image has one target (ADR-0188,
    /// ADR-0189).
    RateReadBesideTarget,
    /// `istdp_target_read` is set without a read of the rate's target (`rate_read_windows` zero),
    /// or an image carries it beside no read under way: the inhibitory target is taken from the
    /// read (ADR-0192, ADR-0194).
    IstdpReadWithoutRead,
    /// `step_probe` is set without a read of the rate's target (`rate_read_windows` zero), or an
    /// image carries it beside no read and no probe under way, or a probe beside no target the
    /// read made: the probe follows the read (ADR-0200, ADR-0202).
    StepProbeWithoutRead,
    /// `kept_addresses` is above `KEPT_ADDRESSES_MAX` (ADR-0208).
    KeptAddressesOutOfRange,
    /// `kept_addresses` is set without the critic or without its window: the address kept is
    /// the one drawn from the window's counts (ADR-0139, ADR-0208).
    KeptAddressesWithoutWindow,
    /// `reads_its_delay` is set without `kept_addresses`: each age's critic reads the counts its
    /// age's entry kept (ADR-0216, ADR-0217).
    DelayReadWithoutKept,
    /// `depression_exponent` is other than one beside a read of the rate's target, or an image
    /// carries a read or a probe under way under such an exponent: the derivation stays H-42's,
    /// and no run has read one at another exponent (ADR-0232).
    ExponentBesideRead,
}

/// The read of the rate's target (ADR-0188, ADR-0189), between ticks: `Executor::target_read`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TargetRead {
    /// Under way: `left` windows of the read still to close, at least one, the gain held through
    /// each; `sum` the spikes of those of its last sixteen windows closed so far.
    Reading { left: u8, sum: u64 },
    /// The probe of the step after the read (ADR-0200, ADR-0202), under way: `left` of its sixteen
    /// windows still to close, at least one, the record's gain held through each and the workers
    /// given it moved by a sixty-fourth (`Executor::gain_given_q16`); `sum` the counts of its
    /// windows closed so far at the higher gain less those at the lower. The read's target is set
    /// beside it, and the rule waits for the probe's step.
    Probing { left: u8, sum: i64 },
    /// Ended on a mean whose period the rate's target refuses: no target, and the gain held from
    /// there on, as through the read. The writer refuses an image of the engine with
    /// `ConfigError::RateTargetOutOfRange`, so no image carries it. Since ADR-0202 also a probe
    /// that made no step: the read's target is kept as a reading, the gain is held from there on
    /// and no rule runs, and the writer refuses its image with
    /// `ConfigError::ControlStepOutOfRange`.
    Refused,
}

/// What the critic read at a reward (ADR-0131): the value, the error the weights moved by, and
/// what the modulator received in the reward's place (ADR-0155).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Prediction {
    /// The value, Q16.16: the weights times the units' spikes since the previous reward,
    /// scaled.
    pub value_q16: i32,
    /// The reward less the value, saturating, Q16.16: the critic's error, which moved its
    /// weights.
    pub error_q16: i32,
    /// What the modulator received, Q16.16: the error, or, while the whole punishment is set
    /// (ADR-0155), the reward where the reward and the value were both below zero.
    pub received_q16: i32,
}

/// Why an episode could not be tagged (ADR-0038).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TagError {
    /// The ledger has no room for another episode (`Config::episodes`).
    LedgerFull,
    /// A unit of the pattern is outside the arena.
    NoSuchUnit,
    /// The pattern is one `Episode::tag` refuses: no unit, more than `PATTERN_MAX`, a unit
    /// twice, or a priority of zero.
    InvalidPattern,
    /// No episode at that index of the ledger (ADR-0052).
    NoSuchEpisode,
    /// The episode is bound to a symbol already, or the symbol is none (ADR-0052).
    Bound,
}

/// Why an injection is refused.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InjectError {
    /// The unit index is outside the arena.
    NoSuchUnit,
    /// The payload is [`ACTIVATE`], which is reserved.
    ReservedPayload,
    /// The ring is full; try again after a tick.
    Full,
}

/// Why an addressing is refused (ADR-0068, ADR-0139).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AddressError {
    /// A unit of the addressed set is outside the arena.
    NoSuchUnit,
    /// The address drawn while the critic is unset (ADR-0139): the engine keeps no count to
    /// draw its sources from.
    NoCritic,
    /// The address drawn while the critic's window is unset (ADR-0139): the counts are every
    /// spike since the previous reward, the whole network's, which is the global delivery's
    /// reach and not an address.
    NoWindow,
    /// A trial's close kept, or a reward of an age given, while `Config::kept_addresses` is unset
    /// (ADR-0208): the engine keeps no address.
    NotKept,
    /// A reward of an age above `Config::kept_addresses` (ADR-0208): the engine keeps no entry
    /// that old.
    AgeNotKept,
    /// A reward of an age at which no address is kept (ADR-0208): fewer trials have closed since
    /// the engine's start, its build or its load, or that trial's reward was given already.
    NoKeptAddress,
    /// A reward given without its age while `Config::reads_its_delay` is unset (ADR-0217): the
    /// engine reads no age to give it at.
    DelayUnread,
}

/// One trial's address kept (ADR-0208): the critic's counts as the trial's close found them,
/// whose units not zero are the address's sources, the targets given by unit, whether its
/// reward is still to come, and whether a close has written it since the engine's start, its
/// build or its load (ADR-0217): a spent entry's counts are still its trial's.
#[derive(Clone, Debug, PartialEq, Eq)]
struct KeptEntry {
    counts: Vec<u32>,
    targets: Vec<bool>,
    live: bool,
    closed: bool,
}

/// One age's score (ADR-0216, ADR-0217): the magnitudes of its critic's errors at its last
/// `DELAY_WINDOW` rewards, a ring, the slot the next is written at, how many it holds, and
/// their sum.
#[derive(Clone, Debug, PartialEq, Eq)]
struct AgeScore {
    errors: [u32; DELAY_WINDOW],
    next: usize,
    held: usize,
    sum: u64,
}

impl AgeScore {
    const fn new() -> Self {
        Self {
            errors: [0; DELAY_WINDOW],
            next: 0,
            held: 0,
            sum: 0,
        }
    }

    /// An error's magnitude recorded, the oldest of a full window let go.
    fn record(&mut self, magnitude: u32) {
        if let Some(slot) = self.errors.get_mut(self.next) {
            if self.held == DELAY_WINDOW {
                self.sum = self.sum.saturating_sub(u64::from(*slot));
            } else {
                self.held = self.held.saturating_add(1);
            }
            *slot = magnitude;
            self.sum = self.sum.saturating_add(u64::from(magnitude));
        }
        self.next = self
            .next
            .wrapping_add(1)
            .checked_rem(DELAY_WINDOW)
            .unwrap_or(0);
    }

    /// The score, once the window is full: the sum of its `DELAY_WINDOW` magnitudes.
    fn score(&self) -> Option<u64> {
        (self.held == DELAY_WINDOW).then_some(self.sum)
    }
}

/// The age whose score leads (ADR-0216): the age whose score is below every other age's by at
/// least `lead`, and so strictly the lowest; none until every age has a score, and none where
/// no age leads by as much.
fn delay_leader(scores: &[Option<u64>], lead: u64) -> Option<u8> {
    (0..scores.len())
        .find(|&age| {
            let Some(low) = scores.get(age).copied().flatten() else {
                return false;
            };
            scores
                .iter()
                .enumerate()
                .filter(|&(other, _)| other != age)
                .all(|(_, &score)| {
                    score
                        .and_then(|s| s.checked_sub(low))
                        .is_some_and(|gap| gap > 0 && gap >= lead)
                })
        })
        .map(|age| age as u8)
}

/// The run after a reward read from age zero (ADR-0216): one more where the same age other than
/// zero leads again, one where another such age leads, and none where age zero or no age leads.
fn delay_run_after(run: Option<(u8, u8)>, leader: Option<u8>) -> Option<(u8, u8)> {
    match (leader, run) {
        (Some(age), Some((leading, running))) if age != 0 && age == leading => {
            Some((age, running.saturating_add(1)))
        }
        (Some(age), _) if age != 0 => Some((age, 1)),
        _ => None,
    }
}

/// The delay read (ADR-0216, ADR-0217): the age the engine consolidates at, whether its first
/// reward has been read, every age's critic but the one in use — whose weights are the units',
/// the engine's critic — each age's score, and the age other than zero leading at the last
/// rewards with how many running.
#[derive(Clone, Debug, PartialEq, Eq)]
struct DelayRead {
    in_use: u8,
    started: bool,
    weights: Vec<Vec<i16>>,
    scores: Vec<AgeScore>,
    run: Option<(u8, u8)>,
}

/// The modulations of a tick (ADR-0068, ADR-0086): the one an addressed synapse consolidates
/// under, `clamp(baseline + dopamine, 0, 1)` (`menteion-neuromod`'s `modulation`, ADR-0032);
/// the one every other synapse consolidates under, the same rule with the signal at rest,
/// `clamp(baseline, 0, 1)`; and, when the inhibitory baseline is set, the one every slot of
/// an inhibitory block consolidates under, `clamp(inhibitory baseline, 0, 1)`, addressed or
/// not — the dopamine term never reaches it (ADR-0085). A synapse is addressed when its
/// presynaptic unit is an addressed source and its target an addressed target. The
/// addressing reaches the dopamine term only. With the signal at rest the first two are one
/// number, so a run with no reward is the same run whatever is addressed; with every unit a
/// source and a target only the first is read, so a run that addresses every unit is the run
/// before the addressing existed, bit for bit; with the inhibitory baseline unset the third
/// is never read, so every run before ADR-0086 is the run it was, bit for bit. While the
/// signed gate is set (ADR-0094) the first is `clamp(baseline + dopamine, -1, 1)`
/// (`menteion-neuromod`'s `signed_modulation`), which an addressed excitatory synapse
/// consolidates under and an addressed inhibitory one at no less than zero — the first as it
/// is unset — so the sign reaches no synapse but an addressed excitatory one; unset, the first
/// is never below zero and the floor changes nothing, so every run before ADR-0094 is the run
/// it was, bit for bit.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Modulations {
    /// The modulation of an addressed synapse: in $[0, 1]$ while the signed gate is unset, in
    /// $[-1, 1]$ while it is set, where an inhibitory slot takes it at no less than zero.
    pub addressed: i32,
    pub at_rest: i32,
    /// The modulation of every slot of an inhibitory block while the inhibitory baseline is
    /// set; none while it is unset, where an inhibitory slot's is `addressed` or `at_rest`
    /// as an excitatory one's is.
    pub inhibitory: Option<i32>,
}

impl Modulations {
    /// The modulations from the modulator, the baseline, the inhibitory baseline and the
    /// signed gate: the rule is `menteion-neuromod`'s in the first two, once with the signal as
    /// it stands — `modulation`, or `signed_modulation` while the signed gate is set — and
    /// once at rest; the third is the inhibitory baseline clamped to $[0, 1]$, the signal
    /// never entering it, and none when the baseline is unset.
    pub fn of(
        modulator: &NeuromodulatorState,
        baseline_q16: i32,
        inhibitory_baseline_q16: Option<i32>,
        signed_gate: bool,
    ) -> Self {
        Self {
            addressed: if signed_gate {
                modulator.signed_modulation(baseline_q16)
            } else {
                modulator.modulation(baseline_q16)
            },
            at_rest: NeuromodulatorState::new().modulation(baseline_q16),
            inhibitory: inhibitory_baseline_q16.map(|b| b.clamp(0, MODULATION_ONE_Q16)),
        }
    }

    /// The modulation a synapse consolidates under: `inhibitory` for a slot of an inhibitory
    /// block while the inhibitory baseline is set, whatever the addressing; otherwise
    /// `addressed` when the synapse is addressed — at no less than zero for an inhibitory
    /// slot, so that the signed gate reaches none (ADR-0094) — and `at_rest` when it is not.
    pub const fn for_synapse(&self, addressed: bool, polarity: Polarity) -> i32 {
        match (polarity, self.inhibitory, addressed) {
            (Polarity::Inhibitory, Some(modulation), _) => modulation,
            (_, _, false) => self.at_rest,
            (Polarity::Excitatory, _, true) => self.addressed,
            // A range pattern, not a comparison: at zero the floor and the value are one
            // number, which no test could tell from a bound.
            (Polarity::Inhibitory, None, true) => match self.addressed {
                i32::MIN..0 => 0,
                modulation => modulation,
            },
        }
    }
}

/// The word the coordinator publishes for the inhibitory modulation (ADR-0086): the
/// modulation itself while the inhibitory baseline is set — within $[0, 1]$, as
/// `Modulations::of` clamps it — and `INHIBITORY_UNSET`, below zero, while it is unset. One
/// atomic beside the other two, read once per tick per worker.
const INHIBITORY_UNSET: i32 = -1;

const _: () = assert!(INHIBITORY_UNSET < 0);

fn inhibitory_word(modulation: Option<i32>) -> i32 {
    modulation.unwrap_or(INHIBITORY_UNSET)
}

/// The modulation a published word names: none for a word below zero.
fn inhibitory_of_word(word: i32) -> Option<i32> {
    if word < 0 { None } else { Some(word) }
}

/// [`SynapseBlock::consolidate_signed`] for every slot in order, each under its own modulation
/// (ADR-0068, ADR-0094). A modulation at or above zero is `consolidate`'s, bit for bit, so under
/// one such modulation for every slot it is `consolidate_all`, call for call, which
/// `consolidate_each_is_consolidate_signed_slot_by_slot_and_consolidate_all_above_zero` holds
/// with the previous call as its oracle; only an addressed excitatory slot under the signed
/// gate is ever given one below zero (`Modulations::for_synapse`). An empty slot's modulation
/// is not read, since neither rule moves anything there.
fn consolidate_each(
    block: &mut SynapseBlock,
    modulations: &[i32; SYNAPSES_PER_BLOCK],
    polarity: Polarity,
) {
    for (slot, &modulation) in modulations.iter().enumerate() {
        block.consolidate_signed(slot, modulation, polarity);
    }
}

/// The most groups a zero sum holds (ADR-0228): one bit each in `Shared::zero_sum_touched`.
pub const ZERO_SUM_GROUPS_MAX: usize = 64;

/// The fraction every synapse of a zero sum starts at (ADR-0226): a half, in $2^{-32}$ LSB, so
/// that a synapse's rounding over all its rounds is within half an LSB of the exact quotients'
/// either way, but for the $2^{-32}$ LSB each round's quotient is truncated to.
const ZERO_SUM_HALF: u32 = 1 << 31;

/// An excitatory weight scaled by `value / sum` with its fraction carried (ADR-0226):
/// $\lfloor w V / S + f \rfloor$ within `[0, i16::MAX]`, the fraction left over kept in
/// `fraction` for the synapse's next round, $f$ in $2^{-32}$ LSB. The product is an `i128`, since
/// $S$ at the arena's $2^{28}$ synapses at the rail is past $2^{42}$ and $w V 2^{32}$ past an
/// `i64`. Nothing moves at a sum of zero, whose weights are all zero.
fn carried_weight(weight: i16, value: i64, sum: i64, fraction: &mut u32) -> i16 {
    let scaled = i128::from(weight)
        .saturating_mul(i128::from(value))
        .wrapping_shl(32)
        .checked_div(i128::from(sum));
    let Some(scaled) = scaled else {
        return weight;
    };
    let carried = scaled.saturating_add(i128::from(*fraction));
    *fraction = carried as u32;
    carried.wrapping_shr(32).clamp(0, i128::from(i16::MAX)) as i16
}

/// The synapses of the chains `heads` start (encoded as a unit stores its first block) onto a unit
/// `targets` marks: their weights summed, and each one's block and slot in `at`, which is
/// scratch. A group's sum is a walk of its sources' chains alone, since the tree keeps a synapse
/// with its presynaptic unit. A round of the zero sum scales from this sum, and
/// `Executor::coupling_sum` reads it.
fn zero_sum_walk(
    blocks: &[SynapseBlock],
    heads: &[u32],
    targets: &[bool],
    at: &mut Vec<(u32, u8)>,
) -> i64 {
    at.clear();
    let mut sum = 0i64;
    for &head in heads {
        for s in SynapseBlock::fan_out(blocks, head) {
            if targets.get(s.target as usize).copied().unwrap_or(false) {
                sum = sum.saturating_add(i64::from(s.weight_q1_15));
                at.push((s.block_idx, s.slot));
            }
        }
    }
    sum
}

/// One group's round of the zero sum (ADR-0226): every synapse `zero_sum_walk` finds scaled to
/// `value` by `carried_weight`, which leaves a sum of zero. `fractions` are the synapses'
/// fractions, one for each slot of the arena in its order; `at` is scratch.
fn zero_sum_round(
    blocks: &mut [SynapseBlock],
    heads: &[u32],
    value: i64,
    targets: &[bool],
    fractions: &mut [u32],
    at: &mut Vec<(u32, u8)>,
) {
    let sum = zero_sum_walk(blocks, heads, targets, at);
    if sum == value {
        return;
    }
    for &(block_idx, slot) in at.iter() {
        let place = (block_idx as usize)
            .saturating_mul(SYNAPSES_PER_BLOCK)
            .saturating_add(slot as usize);
        let weight = blocks
            .get_mut(block_idx as usize)
            .and_then(|block| block.weights_q1_15.get_mut(slot as usize));
        if let (Some(w), Some(fraction)) = (weight, fractions.get_mut(place)) {
            *w = carried_weight(*w, value, sum, fraction);
        }
    }
}

/// The injector payload that asks for a turn without a message.
pub const ACTIVATE: u32 = u32::MAX;

/// Aborts the process: inside the tick loop a violated invariant is a bug (whitepaper §8.9).
#[cold]
fn abort(message: &str) -> ! {
    eprintln!("menteion-runtime: invariant violated: {message}");
    std::process::abort()
}

/// The bin of population activity closes every $2^{12}$ ticks and the window is regulated
/// every $2^{17}$ (ADR-0036), on cadences that are masks on the tick (ADR-0035).
const BIN_CADENCE: Cadence = match Cadence::new(ACTIVITY_BIN_SHIFT, 0) {
    Some(c) => c,
    None => panic!("the bin shift is below the clock's width"),
};
const WINDOW_CADENCE: Cadence =
    match Cadence::new(ACTIVITY_BIN_SHIFT.saturating_add(ACTIVITY_WINDOW_SHIFT), 0) {
        Some(c) => c,
        None => panic!("the window shift is below the clock's width"),
    };
/// A bin that is at least the wheel's horizon sees every direct descendant of its spikes in
/// itself or the next bin (ADR-0036); a shorter one would read a delayed network as
/// sub-critical and the controller would raise the gain without bound.
const _: () = assert!(BIN_CADENCE.period() >= WorkerWheel::horizon_ticks());
/// A replay event every $2^{11}$ ticks (ADR-0038), on a cadence that is a mask on the tick.
const RIPPLE_CADENCE: Cadence = match Cadence::new(RIPPLE_SHIFT, 0) {
    Some(c) => c,
    None => panic!("the ripple shift is below the clock's width"),
};
/// A ripple is longer than the refractory window, and at least four basal time constants, so
/// that successive drives do not sum in the dendrite and a unit replayed at every ripple fires
/// once at every ripple (ADR-0038; at two constants the second drive fires it twice).
const _: () = assert!(RIPPLE_CADENCE.period() > REFRACTORY_TICKS as u64);
const _: () = assert!(RIPPLE_CADENCE.period() >= 4 << BASAL_LEAK_SHIFT);
/// The episodes one ripple considers before it gives up: the hand walks the ledger round
/// robin, skipping spent episodes, and a ripple that finds none within this many delivers
/// nothing (ADR-0038); the walk is bounded so that a ledger of spent episodes costs a ripple
/// nothing more than this.
const RIPPLE_SCAN: usize = 16;
/// The replay drive (ADR-0038): `REPLAY_MESSAGES` messages of `REPLAY_DRIVE_Q16` each into
/// the basal compartment of every unit of the episode, two and a half times the threshold's
/// base in all: the middle of the band that fires a unit at its base threshold exactly once,
/// about twelve ticks on (below about 2.1 the soma, which settles near half the basal
/// potential, never reaches the threshold; from 3.0 what the drive leaves in the dendrite
/// after the refractory window fires the unit a second time; ADR-0018).
const REPLAY_DRIVE_Q16: i32 = 0x0001_4000;
const REPLAY_MESSAGES: usize = 2;
const REPLAY_MESSAGE: u32 = spike_message(REPLAY_DRIVE_Q16, false);
/// The mailbox nodes one ripple takes on worker 0: a message per unit of the widest pattern,
/// `REPLAY_MESSAGES` times; `Executor::new` refuses a pool below it while the ledger has room.
const RIPPLE_NODES: usize = REPLAY_MESSAGES * PATTERN_MAX;
const _: () = assert!(REPLAY_DRIVE_Q16 * REPLAY_MESSAGES as i32 == THRESHOLD_BASE * 5 / 2);
/// The activity above which a window is read as saturated (ADR-0036): one spike per unit per
/// bin on average (about 24 Hz per unit at the fine tick). A population
/// firing that often no longer forms the branching process the estimator's slope reads, so
/// the ceiling reads such a window as supercritical instead; the units' own short-term
/// depression keeps a network well below it in every run of the exit test.
fn saturation_ceiling(units: usize) -> u32 {
    units.max(1).min(u32::MAX as usize) as u32
}

/// The most spikes a window's bins count: thirty-two bins at the cap, 536 870 880. A literal,
/// so that the bound is nothing a mutant can touch; the assertion ties it to the record's
/// constants.
const RATE_TARGET_MAX: u32 = 536_870_880;
const _: () = assert!(RATE_TARGET_MAX == ACTIVITY_WINDOW_BINS as u32 * ACTIVITY_COUNT_MAX);

/// The rate's target as a window's count (ADR-0180): none while the period is zero, which is
/// unset; otherwise the spikes `units` units firing once a period make in a window, the units
/// times the window's ticks over the period, rounded to nearest from the exact quotient, a half
/// up. Refused where that is no spike — a period longer than twice the units times the window
/// names no rate the window can be held to — and where it is more than a window's bins count.
fn rate_target_of(units: usize, period_ticks: u32) -> Result<Option<u32>, ConfigError> {
    if period_ticks == 0 {
        return Ok(None);
    }
    let period = u64::from(period_ticks);
    let count = (units as u64)
        .saturating_mul(WINDOW_CADENCE.period())
        .saturating_add(period >> 1)
        .checked_div(period)
        .unwrap_or(0);
    match u32::try_from(count) {
        Ok(count @ 1..=RATE_TARGET_MAX) => Ok(Some(count)),
        _ => Err(ConfigError::RateTargetOutOfRange),
    }
}

/// The windows at the end of a read whose counts are summed (ADR-0188): sixteen, ADR-0055's,
/// as a shift so that their mean is the sum shifted.
pub(crate) const READ_SUMMED_SHIFT: u32 = 4;
pub(crate) const READ_SUMMED_WINDOWS: u8 = 1 << READ_SUMMED_SHIFT;
const _: () = assert!(READ_SUMMED_WINDOWS == 16);

/// The rate's target a read makes of its sum (ADR-0188, ADR-0189): the mean of its last sixteen
/// windows' counts, the sum over sixteen to nearest with a half going up; the period, the units
/// times the window's ticks over that mean, to nearest with a half going up, as `rate_target_of`
/// rounds the inverse; and the window's count of that period, `rate_target_of`'s, which the rule
/// then holds the windows to, so that the engine after the read is the engine its image loads.
/// The two counts are one below the square root of the units times the window's ticks, 11 585 a
/// window at 1 024 units. Refused at a mean of no spike, a period the width does not hold or one
/// of no tick, and where `rate_target_of` refuses the period.
fn read_target_of(units: usize, sum: u64) -> Result<(u32, u32), ConfigError> {
    let mean = sum.saturating_add(u64::from(READ_SUMMED_WINDOWS >> 1)) >> READ_SUMMED_SHIFT;
    let period = (units as u64)
        .saturating_mul(WINDOW_CADENCE.period())
        .saturating_add(mean >> 1)
        .checked_div(mean);
    match period.map(u32::try_from) {
        Some(Ok(period @ 1..)) => match rate_target_of(units, period)? {
            Some(count) => Ok((period, count)),
            None => Err(ConfigError::RateTargetOutOfRange),
        },
        _ => Err(ConfigError::RateTargetOutOfRange),
    }
}

/// A window's close under a read (ADR-0189): what follows a read with `left` windows still to
/// close and `sum` of its last sixteen summed so far, the window having counted `spikes` with
/// the gain held.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ReadStep {
    /// The read goes on: one window fewer left, the count summed where the window was one of
    /// the last sixteen.
    Reading { left: u8, sum: u64 },
    /// The window was the read's last: the period the target is set to, and its count.
    Target { period: u32, count: u32 },
    /// The window was the read's last, and its mean's period is refused.
    Refused,
}

/// `ReadStep` of a window's close: the window's spikes summed when it is one of the read's last
/// sixteen — the windows left, this one among them, at most sixteen — and, where it was the
/// last, the target `read_target_of` makes of the sum for `units` units. A read always has a
/// window left to close, so a `left` of zero is read as the last, as one is. Where the read sets
/// the inhibitory rule's target as well (`inhibitory`, ADR-0194), a period that rule refuses,
/// outside `[ISTDP_PERIOD_MIN_TICKS, ISTDP_PERIOD_MAX_TICKS]`, is refused: the bounds are a range
/// pattern, so no comparison is left for a mutant to move.
fn read_step(left: u8, sum: u64, spikes: u64, units: usize, inhibitory: bool) -> ReadStep {
    let sum = if left <= READ_SUMMED_WINDOWS {
        sum.saturating_add(spikes)
    } else {
        sum
    };
    match left.checked_sub(1) {
        Some(left @ 1..) => ReadStep::Reading { left, sum },
        _ => match (read_target_of(units, sum), inhibitory) {
            (Ok((period, count)), false)
            | (Ok((period @ ISTDP_PERIOD_MIN_TICKS..=ISTDP_PERIOD_MAX_TICKS, count)), true) => {
                ReadStep::Target { period, count }
            }
            _ => ReadStep::Refused,
        },
    }
}

/// The most a read's sum holds with `left` windows still to close (ADR-0189): a window's count
/// at most `RATE_TARGET_MAX` for each of its last sixteen windows already closed, sixteen less
/// `left` of them, none while sixteen or more are left. The loader refuses a sum above it, which
/// is a record the writer never produces.
pub(crate) fn read_sum_max(left: u8) -> u64 {
    u64::from(READ_SUMMED_WINDOWS.saturating_sub(left)).saturating_mul(u64::from(RATE_TARGET_MAX))
}

/// The probe's windows (ADR-0200): four at the lower gain, eight at the higher and four at the
/// lower, which `TargetRead::Probing`'s `left` counts down from sixteen. The order cancels a
/// count that drifts at a constant rate through them.
pub(crate) const PROBE_WINDOWS: u8 = 16;
/// The probe's windows at either gain before the higher ones, and the higher ones (ADR-0200):
/// the higher are those with five to twelve left (`probe_raises`).
const PROBE_LOWER_FIRST: u8 = 4;
const PROBE_HIGHER: u8 = 8;
const _: () = assert!(PROBE_WINDOWS == 2 * PROBE_HIGHER && PROBE_HIGHER == 2 * PROBE_LOWER_FIRST);
/// The probe's move of the gain as a shift (ADR-0200): a sixty-fourth either way, its two
/// factors and the half the product rounds by.
const PROBE_SHIFT: u32 = 6;
const PROBE_UP: u64 = 65;
const PROBE_DOWN: u64 = 63;
const PROBE_HALF: u64 = 32;
const _: () = assert!(
    PROBE_UP == (1 << PROBE_SHIFT) + 1
        && PROBE_DOWN == (1 << PROBE_SHIFT) - 1
        && PROBE_HALF == 1 << (PROBE_SHIFT - 1)
);
/// λ of ADR-0200 as a shift: a quarter.
const PROBE_LAMBDA_SHIFT: u32 = 2;
/// The step the probe makes as a shift of the target's count over its sum (ADR-0200): with eight
/// windows a side, $2^3$, and the gain moved by $2 \cdot 2^{-6}$ between them, the steepness is
/// $S = D \cdot 2^{6 - 1 - 3} / c^{*} = 4D/c^{*}$, and $\kappa = \lambda / S$ in Q0.16 is
/// $2^{16 - 2 - 2}\,c^{*}/D$, ADR-0200's $4\,096\,c^{*}/D$.
const PROBE_STEP_SHIFT: u32 = 12;
const _: () = assert!(
    PROBE_STEP_SHIFT == 16 - PROBE_LAMBDA_SHIFT - (PROBE_SHIFT - 1 - 3)
        && PROBE_HIGHER as u32 == 1 << 3
);

/// Whether the probe's window with `left` windows still to close, itself among them, runs at the
/// higher gain (ADR-0200): the eight between its first four and its last four. A range pattern,
/// so no comparison is left for a mutant to move.
fn probe_raises(left: u8) -> bool {
    matches!(left, 5..=12)
}
const _: () = assert!(
    5 == PROBE_WINDOWS - PROBE_LOWER_FIRST - PROBE_HIGHER + 1
        && 12 == PROBE_WINDOWS - PROBE_LOWER_FIRST
);

/// The gain the workers are given through the probe's window with `left` windows still to close
/// (ADR-0200, ADR-0202): `gain`, the record's $g_0$, which the probe holds, times $65/64$ in a
/// window that raises it and $63/64$ in the others, to nearest with a half going up. The product
/// may pass the controller's bounds by a sixty-fourth of them; the record's gain never does.
fn probe_gain_q16(gain: u32, left: u8) -> u32 {
    let factor = if probe_raises(left) {
        PROBE_UP
    } else {
        PROBE_DOWN
    };
    let given = u64::from(gain)
        .saturating_mul(factor)
        .saturating_add(PROBE_HALF)
        >> PROBE_SHIFT;
    u32::try_from(given).unwrap_or(u32::MAX)
}

/// The step a probe's sum makes against the read's target count (ADR-0200, ADR-0202):
/// $\kappa = 4\,096\,c^{*}/D$ in Q0.16, to nearest with a half going up. None where the sum is
/// not above zero — the higher gain counted no more than the lower, a steepness the probe cannot
/// read — and where $\kappa$ passes `CONTROL_STEP_MAX_Q0_16`, a steepness below a half. The sum
/// is held to no comparison: a sum below zero is no `u64`, and a sum of zero divides nothing.
fn probe_step_of(count: u32, sum: i64) -> Option<u16> {
    let sum = u64::try_from(sum).ok()?;
    let step = (u64::from(count) << PROBE_STEP_SHIFT)
        .saturating_add(sum >> 1)
        .checked_div(sum)?;
    match u16::try_from(step) {
        Ok(step @ 0..=CONTROL_STEP_MAX_Q0_16) => Some(step),
        _ => None,
    }
}

/// A window's close under the probe (ADR-0202): what follows a probe with `left` windows still to
/// close and `sum` so far, the window having counted `spikes`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ProbeStep {
    /// The probe goes on: one window fewer left, the count added or taken.
    Probing { left: u8, sum: i64 },
    /// The window was the probe's last: the step it made.
    Step(u16),
    /// The window was the probe's last, and its sum makes no step.
    Refused,
}

/// `ProbeStep` of a window's close: the window's spikes added to the sum where the window ran at
/// the higher gain and taken from it where it ran at the lower, and, where it was the last, the
/// step `probe_step_of` makes of the sum against `count`, the read's target count. A probe always
/// has a window left to close, so a `left` of zero is read as the last, as one is.
fn probe_step(left: u8, sum: i64, spikes: u64, count: u32) -> ProbeStep {
    // A window's count is below 2^30, so it fits; it saturates by name past that.
    let spikes = i64::try_from(spikes).unwrap_or(i64::MAX);
    let sum = if probe_raises(left) {
        sum.saturating_add(spikes)
    } else {
        sum.saturating_sub(spikes)
    };
    match left.checked_sub(1) {
        Some(left @ 1..) => ProbeStep::Probing { left, sum },
        _ => probe_step_of(count, sum).map_or(ProbeStep::Refused, ProbeStep::Step),
    }
}

/// The least and the most a probe's sum holds with `left` windows still to close (ADR-0202):
/// each window's count at most `RATE_TARGET_MAX`, taken from the sum in each lower window closed
/// and added in each higher one; zero and zero while none is closed. The loader refuses a sum
/// outside them, which is a record the writer never produces.
pub(crate) fn probe_sum_bounds(left: u8) -> (i64, i64) {
    let closed = PROBE_WINDOWS.saturating_sub(left);
    let higher = closed.saturating_sub(PROBE_LOWER_FIRST).min(PROBE_HIGHER);
    let lower = closed.saturating_sub(higher);
    let max = i64::from(RATE_TARGET_MAX);
    (
        i64::from(lower).saturating_mul(max).saturating_neg(),
        i64::from(higher).saturating_mul(max),
    )
}

/// The two compartment sums under the tick's gain: `sum × gain`, Q16.16, rounded to nearest
/// and clamped to the width; exact at a gain of 1.0 (ADR-0036).
#[inline]
fn scaled(sum: i32, gain_q16: u32) -> i32 {
    ((sum as i64)
        .saturating_mul(gain_q16 as i64)
        .saturating_add(0x8000)
        >> 16)
        .clamp(i32::MIN as i64, i32::MAX as i64) as i32
}

/// The partition of the unit arena among the workers (ADR-0100): the share, $\lceil N / W
/// \rceil$ units a worker, and the stride, the words of the schedule a worker's region takes —
/// a share's words rounded up to eight, a cache line, so that no two owners' words share one.
fn partition(units: usize, workers: usize) -> (usize, usize) {
    let share = units.div_ceil(workers.max(1));
    (share, share.div_ceil(64).next_multiple_of(8))
}

/// Worker `id`'s range (ADR-0100): its first unit `id·share`, the words of the schedule its
/// units take, and its first word `id·stride`. The range ends at `(id + 1)·share` or at the
/// arena's end; a worker past the arena owns none and takes no word.
fn range_of(id: usize, units: usize, share: usize, stride: usize) -> (u32, usize, usize) {
    let first = id.saturating_mul(share).min(units);
    let end = id.saturating_add(1).saturating_mul(share).min(units);
    // Below the unit count, which `Executor::new` bounds below `u32::MAX`.
    (
        first as u32,
        end.saturating_sub(first).div_ceil(64),
        id.saturating_mul(stride),
    )
}

/// `Vec::push` that never grows: exceeding the capacity is a sizing bug, not an allocation.
#[inline]
fn push_bounded<T>(v: &mut Vec<T>, x: T, what: &str) {
    if v.len() == v.capacity() {
        abort(what);
    }
    v.push(x);
}

/// The slot after `i` on a ring of `n` slots: `(i + 1) % n` by name. `i` is below `n`, so the
/// sum cannot wrap; `n` is at least one wherever a ring is walked (a worker count or a unit
/// count, both refused at zero by [`Executor::new`]), so the fallback is never taken.
#[inline]
fn next_in_ring(i: usize, n: usize) -> usize {
    i.wrapping_add(1).checked_rem(n).unwrap_or(0)
}

struct Shared {
    units: Arena<DendriticSuperNeuron>,
    blocks: Arena<SynapseBlock>,
    deltas: Arena<PlasticDelta>,
    /// Units whose record is in the write-ahead log (ADR-0024); set and cleared between ticks.
    evicted: Box<[AtomicBool]>,
    /// Units that received a message while evicted; re-hydrated after the tick.
    needs_rehydration: Box<[AtomicBool]>,
    rehydration_pending: AtomicUsize,
    in_flight: Box<[AtomicI64]>,
    pools: Pools,
    /// The schedule (ADR-0100): one bit per unit, set while the next tick serves the unit.
    /// Worker `w` owns the units `[w·share, min((w+1)·share, N))`, and their bits are its
    /// region of `stride` words from word `w·stride`, in unit order. The owner loads and stores
    /// its words in the turns phase, where no push happens; a push or an activation sets a bit
    /// with a fetch-or in phases 2 and 3, and the coordinator between ticks; the barriers order
    /// the two.
    scheduled: Box<[AtomicU64]>,
    /// The units asleep (ADR-0224), one bit per unit in the schedule's layout: set while the unit
    /// is not at rest and off the schedule, `quiet_cannot_fire` having admitted it at the end of
    /// its last turn, its gate recorded scheduled as per-tick service records it and its state
    /// that of the tick in `since`. The owner loads and stores its words in the turns phase, and
    /// the coordinator between ticks; a marker reads them in phases 2 and 3.
    asleep: Box<[AtomicU64]>,
    /// Per unit, the first tick an asleep unit was not served, whose state it holds.
    since: Box<[AtomicU64]>,
    /// Per-tick service, the tests' reference (ADR-0224): set between ticks by
    /// `Executor::per_tick`, read by every worker at the start of its turns phase.
    per_tick: AtomicBool,
    /// Units per worker, $\lceil N / W \rceil$.
    share: usize,
    /// Words of the schedule per worker: a share's words rounded up to eight, a cache line, so
    /// that no two owners' words share a line.
    stride: usize,
    barrier: SpinBarrier,
    injector: Injector,
    delivered: Box<[AtomicU64]>,
    /// Per worker, the turns it has served so far (ADR-0097): stored beside `delivered` at the
    /// end of the deliveries phase, summed by `Executor::turns` between ticks.
    turns: Box<[AtomicU64]>,
    /// Per worker, the steps its turns caught up so far (ADR-0224), stored beside `turns`.
    caught_up: Box<[AtomicU64]>,
    now: AtomicU32,
    /// The tick `now` is, unwrapped: stored beside it, read by a turn that catches a unit up.
    tick: AtomicU64,
    /// The modulation this tick's fan-out consolidates with (ADR-0032): stored by the
    /// coordinator before the tick's first barrier, read by every worker after it. Since
    /// ADR-0068 it is the modulation of an addressed synapse; since ADR-0094 it is below zero
    /// only while the signed gate is set.
    modulation: AtomicI32,
    /// The modulation of a synapse that is not addressed (ADR-0068): the same rule with the
    /// dopamine signal at rest, stored and read beside `modulation`.
    modulation_at_rest: AtomicI32,
    /// The inhibitory modulation's word (ADR-0086): the modulation every slot of an
    /// inhibitory block consolidates under while the inhibitory baseline is set, or
    /// `INHIBITORY_UNSET` while it is unset; stored and read beside `modulation`.
    modulation_inhibitory: AtomicI32,
    /// The addressed set (ADR-0068) as two flags per unit, a source and a target: a synapse
    /// is addressed when its presynaptic unit is a source and its target a target. Written
    /// between ticks by the coordinator, read by every worker in the fan-out phase, the
    /// source once per spiked unit and the target once per slot. Every unit is both until
    /// `Executor::address` narrows them.
    sources: Box<[AtomicBool]>,
    targets: Box<[AtomicBool]>,
    /// The synaptic gain this tick's turns scale their sums by (ADR-0036): stored by the
    /// coordinator before the tick's first barrier, read by every worker after it.
    gain: AtomicU32,
    /// Per worker, the units that fired in its turns phase this tick: stored at the end of the
    /// phase, summed by the coordinator after the tick (ADR-0036).
    spikes: Box<[AtomicU32]>,
    /// Descendants each worker fired this tick (ADR-0054), beside its spikes.
    descendants: Box<[AtomicU32]>,
    /// The inhibitory rule's depression per spike at the engine's target period (ADR-0053),
    /// read by every worker in the fan-out phase.
    istdp_alpha: AtomicI32,
    /// The excitatory depression's exponent (ADR-0228): the configuration's, read by every
    /// worker in the fan-out phase.
    depression_exponent: DepressionExponent,
    /// The zero sum (ADR-0226, ADR-0228), in no image: each unit's group as a held source, its
    /// index plus one, zero elsewhere; whether each unit is a held target; and the groups
    /// touched this tick, one bit each. The first two are written between ticks by
    /// `Executor::set_zero_sum`; a group's bit is set by any worker whose consolidation moved
    /// one of the group's synapses onto a target in the fan-out phase, and taken by the
    /// coordinator after the tick.
    zero_sum_source: Box<[AtomicU8]>,
    zero_sum_target: Box<[AtomicBool]>,
    zero_sum_touched: AtomicU64,
    /// The units that fired this tick, one slot per unit, appended by every worker at the
    /// spike through `fired_len` and taken by the coordinator after the tick into the train
    /// (ADR-0050); empty when the executor keeps no train.
    fired: Box<[AtomicU32]>,
    fired_len: AtomicUsize,
    /// The episodic ledger (ADR-0038): written between ticks by the coordinator (a tag, a
    /// replay's count, a REM ripple's depotentiation), read by worker 0 in phase 3.
    episodes: Arena<Episode>,
    /// The episode worker 0 replays in this tick's phase 3, as index + 1, or 0 for none:
    /// stored by the coordinator before the tick's first barrier (ADR-0038).
    replay: AtomicU32,
    stop: AtomicBool,
}

impl Shared {
    /// `unit`'s group as a held source of the zero sum (ADR-0228), its index plus one; zero for
    /// any other unit, and for one outside the arena, which no spike names.
    fn zero_sum_group(&self, unit: u32) -> u8 {
        self.zero_sum_source
            .get(unit as usize)
            .map_or(0, |group| group.load(Ordering::Relaxed))
    }

    /// True when a slot of `block` holds a weight other than `before`'s onto a held target of
    /// the zero sum (ADR-0228).
    fn moved_onto_a_target(
        &self,
        block: &SynapseBlock,
        before: &[i16; SYNAPSES_PER_BLOCK],
    ) -> bool {
        (0..SYNAPSES_PER_BLOCK).any(|slot| {
            block.weights_q1_15[slot] != before[slot]
                && block.target(slot).is_some_and(|t| {
                    self.zero_sum_target
                        .get(t as usize)
                        .is_some_and(|flag| flag.load(Ordering::Relaxed))
                })
        })
    }

    /// True when `unit` is an addressed source (ADR-0068); false for a unit outside the
    /// arena, which no spike names.
    fn is_source(&self, unit: u32) -> bool {
        self.sources
            .get(unit as usize)
            .is_some_and(|flag| flag.load(Ordering::Relaxed))
    }

    /// True when `unit` is an addressed target (ADR-0068); false for a unit outside the
    /// arena, which no synapse the loader admitted targets.
    fn is_target(&self, unit: u32) -> bool {
        self.targets
            .get(unit as usize)
            .is_some_and(|flag| flag.load(Ordering::Relaxed))
    }

    /// The worker that owns `unit` and alone serves it (ADR-0100): `unit / share`; none for a
    /// unit outside the arena.
    fn owner(&self, unit: u32) -> Option<usize> {
        if unit as usize >= self.units.len() {
            return None;
        }
        (unit as usize).checked_div(self.share)
    }

    /// The word of the schedule that holds `unit`'s bit, and the bit (ADR-0100): the unit's
    /// place in its owner's range, `unit − owner·share`, is bit `place mod 64` of the owner's
    /// word `place / 64`. None for a unit outside the arena. The tests' view of `word_of`.
    #[cfg(test)]
    fn place(&self, unit: u32) -> Option<(&AtomicU64, u64)> {
        let (word, bit) = self.word_of(unit)?;
        Some((self.scheduled.get(word)?, bit))
    }

    /// The index of the word that holds `unit`'s bit in a map laid out as the schedule is, the
    /// schedule and the asleep map, and the bit; none for a unit outside the arena.
    fn word_of(&self, unit: u32) -> Option<(usize, u64)> {
        let owner = self.owner(unit)?;
        // Below the share: `owner·share` is at most `unit`, which `owner` is the quotient of.
        let at = (unit as usize).wrapping_sub(owner.wrapping_mul(self.share));
        let word = owner.wrapping_mul(self.stride).wrapping_add(at >> 6);
        Some((word, 1u64 << (at & 63)))
    }

    /// The unit of bit `bit` of the word `word` of a map laid out as the schedule is (ADR-0100):
    /// bit `b` of word `w·stride + k` is unit `w·share + 64k + b`.
    fn unit_at(&self, word: usize, bit: u32) -> u32 {
        let owner = word.checked_div(self.stride).unwrap_or(0);
        let k = word.wrapping_sub(owner.wrapping_mul(self.stride));
        // Below the unit count, which `Executor::new` bounds below `u32::MAX`: a bit is set only
        // for a unit of the arena.
        owner
            .wrapping_mul(self.share)
            .wrapping_add(k << 6)
            .wrapping_add(bit as usize) as u32
    }

    /// Marks `unit` for a turn at the next tick (ADR-0100), in phases 2 and 3 after a push or
    /// at an activation, and between ticks: a unit already scheduled is left as it is; an idle
    /// one has its gate byte recorded scheduled and its bit set. Two markers that both find the
    /// unit idle store the same byte and set the same bit, so no claim is needed. An asleep
    /// unit's gate is recorded scheduled, as per-tick service records it while the unit is not
    /// at rest, and its bit is clear (ADR-0224): it is marked as an idle one is.
    fn mark(&self, unit: u32, u: &DendriticSuperNeuron) {
        let Some((word, bit)) = self.word_of(unit) else {
            abort("a marked unit is outside the arena");
        };
        let asleep = self
            .asleep
            .get(word)
            .is_some_and(|asleep| asleep.load(Ordering::Relaxed) & bit == bit);
        if u.gate() == Some(GateState::Scheduled) && !asleep {
            return;
        }
        u.set_gate(GateState::Scheduled);
        let Some(scheduled) = self.scheduled.get(word) else {
            abort("a marked unit is outside the arena");
        };
        scheduled.fetch_or(bit, Ordering::Relaxed);
    }
}

/// One worker's private state.
struct Worker<const CAP: usize> {
    id: usize,
    /// The first unit of the worker's range (ADR-0100).
    first: u32,
    /// The words of the schedule the range takes, from `region`.
    words: usize,
    /// The worker's first word of the schedule.
    region: usize,
    wheel: Box<FlatTimingWheel<CAP>>,
    batch: Vec<u32>,
    due: Vec<u32>,
    spiked: Vec<(u32, u8, u8)>,
    trace: Vec<u32>,
    spike_trace: Vec<(u32, u32)>,
    trace_dropped: u64,
    delivered: u64,
    /// Turns this worker has served (ADR-0097).
    turns: u64,
    /// Steps this worker's turns have caught up (ADR-0224).
    caught_up: u64,
    /// Descendants this worker fired this tick (ADR-0054).
    descended: u32,
    in_flight: i64,
    /// The class a marked unit steps under (ADR-0114): the executor's, copied at
    /// `Executor::new` and never written after, so the turn reads the worker's own field.
    stp_class: Option<StpClass>,
    /// The slow current a marked unit integrates under (ADR-0123): the executor's, copied at
    /// `Executor::new` and never written after, as `stp_class` is.
    slow_current: Option<SlowCurrent>,
}

/// What a worker kept.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct WorkerReport {
    /// The messages the worker drained, in the order it drained them, up to the trace capacity.
    pub delivered: Vec<u32>,
    /// `(unit, tick)` of every spike the worker ran, up to the trace capacity.
    pub spikes: Vec<(u32, u32)>,
    /// Trace entries dropped because the capacity was reached.
    pub dropped: u64,
    /// Messages the worker drained, whether or not traced.
    pub delivered_count: u64,
    /// Turns the worker served (ADR-0097): its own units', since each worker serves the range
    /// of the arena it owns (ADR-0100).
    pub turns: u64,
}

/// A handle producers use to inject from outside the tick loop. Clone it freely; it is `Send`
/// and `Sync`.
#[derive(Clone)]
pub struct Inject(Arc<Shared>);

impl Inject {
    /// Queues `payload` for `unit`'s mailbox; it is integrated on the tick after the one that
    /// drains the ring. `payload` is a spike message (`spike_message`) or any value the unit's
    /// batch order can sort.
    pub fn inject(&self, unit: u32, payload: u32) -> Result<(), InjectError> {
        if unit as usize >= self.0.units.len() {
            return Err(InjectError::NoSuchUnit);
        }
        if payload == ACTIVATE {
            return Err(InjectError::ReservedPayload);
        }
        self.0
            .injector
            .push(unit, payload)
            .map_err(|injector::Full| InjectError::Full)
    }

    /// Queues a turn for `unit` without a message: it integrates on the tick after the drain.
    pub fn activate(&self, unit: u32) -> Result<(), InjectError> {
        if unit as usize >= self.0.units.len() {
            return Err(InjectError::NoSuchUnit);
        }
        self.0
            .injector
            .push(unit, ACTIVATE)
            .map_err(|injector::Full| InjectError::Full)
    }
}

/// The executor. `CAP` is the tokens one wheel slot holds (2 048 in production, ADR-0013).
pub struct Executor<const CAP: usize> {
    shared: Arc<Shared>,
    worker0: Worker<CAP>,
    threads: Vec<JoinHandle<Worker<CAP>>>,
    tick: u64,
    log: Option<WriteAheadLog>,
    hand: usize,
    evictions: u64,
    rehydrations: u64,
    policy: Policy,
    amendments: Vec<PolicyAmendment>,
    /// `Config::amendments` plus what an image held: the arena's size, which `Vec::capacity`
    /// only bounds from below.
    amendment_capacity: usize,
    /// The engine's modulator record (ADR-0032): one for the engine until macro-columns exist.
    modulator: NeuromodulatorState,
    modulation_baseline_q16: i32,
    /// The inhibitory baseline (ADR-0086): the configuration's, or the image's; none while
    /// unset.
    inhibitory_baseline_q16: Option<i32>,
    /// The signed gate (ADR-0094): the configuration's, or the image's.
    signed_gate: bool,
    /// The class of short-term plasticity (ADR-0114): the configuration's, or the image's.
    stp_class: Option<StpClass>,
    /// The slow current (ADR-0123): the configuration's, or the image's.
    slow_current: Option<SlowCurrent>,
    /// The critic (ADR-0131): the configuration's, or the image's.
    critic: Option<ValueCritic>,
    /// Each unit's spikes since the previous reward (ADR-0131), counted as the coordinator
    /// merges a tick's spikes into the train; one per unit while the critic is set, none while
    /// it is unset.
    features: Vec<u32>,
    /// What the critic read at the last reward; none before one, or while the critic is unset.
    prediction: Option<Prediction>,
    /// The critic's window (ADR-0134): the configuration's, or the image's; zero while unset.
    critic_window_ticks: u16,
    /// The tick the critic's window opened at: the previous reward's the critic took, or the
    /// engine's start, its build or its load, before the first.
    window_opened: u64,
    /// The whole punishment (ADR-0155): the configuration's, or the image's.
    whole_punishment: bool,
    /// The rate's target (ADR-0180), a period in ticks: the configuration's, or the image's;
    /// zero while unset.
    rate_target_period_ticks: u32,
    /// The window's count of that period for this engine's units; none while unset, where the
    /// controller regulates by its estimate.
    rate_target: Option<u32>,
    /// The read of the rate's target (ADR-0189): under way from the configuration's span or the
    /// image's progress, refused, or none — unset, or ended in the period above.
    target_read: Option<TargetRead>,
    /// Whether the read under way sets the inhibitory rule's target at its end as well (ADR-0194):
    /// the configuration's or the image's, set only beside a read under way and spent when the
    /// read ends.
    istdp_target_read: bool,
    /// Whether the read under way is followed by the probe of the step, or the probe is under way
    /// (ADR-0202): the configuration's or the image's, set only beside a read or a probe under
    /// way and spent when the probe ends or the read ends refused.
    step_probe: bool,
    /// The kept addresses (ADR-0208): the configuration's oldest age a reward may name, zero
    /// while unset.
    kept_addresses: u8,
    /// The entries kept, a ring of `kept_addresses` and one more while set, none while unset,
    /// whose newest is at `kept_newest`.
    kept: Vec<KeptEntry>,
    kept_newest: usize,
    /// Entries a trial's close wrote over while their reward was still to come.
    kept_let_go: u64,
    /// The delay read (ADR-0217): the configuration's, none while unset.
    delay: Option<DelayRead>,
    /// The engine's homeostasis record (ADR-0036, ADR-0037): the population tally, the
    /// branching-ratio estimator's window, the synaptic gain, the sleep pressure and the
    /// stage; one for the engine until macro-columns exist.
    homeostasis: HomeostaticDrivePool,
    /// The engine's hippocampal record (ADR-0038): the ledger's length and its hand.
    hippocampus: HippocampalAttractorState,
    /// `Config::episodes` plus what an image held: the ledger arena's size.
    episode_capacity: usize,
    /// Slow-wave replays delivered so far.
    replays: u64,
    /// REM ripples that lowered an episode's tag so far.
    depotentiations: u64,
    /// The executor's own spike train (ADR-0050): `(tick, unit)` in tick order and unit order
    /// within a tick, the last `train_capacity` spikes.
    train: VecDeque<(u32, u32)>,
    train_capacity: usize,
    /// Spikes the ring let go because it was full.
    train_overwritten: u64,
    /// The tick's spikes as the coordinator sorts them, one slot per unit.
    merge: Vec<u32>,
    /// The engine's term arena, clause store, affect state and induction record (ADR-0052),
    /// with the loop's counters.
    induction: Induction,
    /// The search's cadence inside the tick, from the induction record's shift; `None` never
    /// searches.
    search_cadence: Option<Cadence>,
    /// The inhibitory rule's target period (ADR-0053): the configuration's, or the image's.
    istdp_target_period_ticks: u32,
    /// Descendants tallied so far (ADR-0054).
    descendants: u64,
    /// Steps caught up between ticks, at a read or a write of the unit arena (ADR-0224).
    read_steps: Cell<u64>,
    /// True from a tick, which may leave units asleep, until a read of the unit arena has
    /// brought them up to date (ADR-0224). A `Cell`, so that the executor is not `Sync`: a read
    /// under `&self` writes the arena, and no other thread holds `&self` (the assertion below).
    unsettled: Cell<bool>,
    /// The zero sum (ADR-0226, ADR-0228), in no image, empty while unset: each group's sources'
    /// chains and its held value; each unit, whether it is a held target; each slot of the
    /// synapse arena's fraction, in the arena's order; and the rounds' scratch.
    zero_sum_groups: Vec<(Vec<u32>, i64)>,
    zero_sum_targets: Vec<bool>,
    zero_sum_fractions: Vec<u32>,
    zero_sum_at: Vec<(u32, u8)>,
}

// The executor is not `Sync` (ADR-0224): `Executor::units` writes the asleep units' records
// under `&self`, which is sound only while no other thread holds `&self`. This fails to compile
// if the executor ever were `Sync`, the pattern of `static_assertions::assert_not_impl_any!`:
// with a second implementation for `Sync` types, the call's trait parameter is ambiguous.
const _: fn() = || {
    trait NotSync<A> {
        fn of() {}
    }
    impl<T: ?Sized> NotSync<()> for T {}
    struct IsSync;
    impl<T: ?Sized + Sync> NotSync<IsSync> for T {}
    let _ = <Executor<1> as NotSync<_>>::of;
};

/// The cadence of a search shift: none at zero.
fn search_cadence_of(shift: u8) -> Option<Cadence> {
    if shift == 0 {
        None
    } else {
        Cadence::new(u32::from(shift), 0)
    }
}

impl<const CAP: usize> Executor<CAP> {
    /// Allocates every arena, pool, deque, wheel and buffer once, spawns `workers - 1` threads
    /// and parks them at the tick barrier. Nothing allocates after this returns.
    pub fn new(config: Config) -> Result<Self, ConfigError> {
        if config.workers == 0 {
            return Err(ConfigError::NoWorkers);
        }
        if config.units == 0 {
            return Err(ConfigError::NoUnits);
        }
        if config.nodes_per_worker == 0 {
            return Err(ConfigError::NoNodes);
        }
        if config.units >= u32::MAX as usize {
            return Err(ConfigError::TooManyUnits);
        }
        // The loader's bound, one function unit-tested at its edge (finding F-23).
        if crate::image::too_many_blocks(config.blocks as u64) {
            return Err(ConfigError::TooManyBlocks);
        }
        if !(0..=MODULATION_ONE_Q16).contains(&config.modulation_baseline_q16) {
            return Err(ConfigError::ModulationOutOfRange);
        }
        if config.control_step_q0_16 > CONTROL_STEP_MAX_Q0_16 {
            return Err(ConfigError::ControlStepOutOfRange);
        }
        if !(GAIN_MIN_Q16..=GAIN_MAX_Q16).contains(&config.synaptic_gain_q16) {
            return Err(ConfigError::GainOutOfRange);
        }
        if config.sleep_shift > SLEEP_SHIFT_MAX {
            return Err(ConfigError::SleepShiftOutOfRange);
        }
        if config.episodes >= u32::MAX as usize {
            return Err(ConfigError::TooManyEpisodes);
        }
        if config.episodes > 0 && config.nodes_per_worker < RIPPLE_NODES {
            return Err(ConfigError::TooFewNodes);
        }
        if config.terms >= u32::MAX as usize || config.clauses >= u32::MAX as usize {
            return Err(ConfigError::TooManyTerms);
        }
        if config.clauses > 0 && config.terms == 0 {
            return Err(ConfigError::ClausesWithoutArena);
        }
        if config.search_shift > SEARCH_SHIFT_MAX {
            return Err(ConfigError::SearchShiftOutOfRange);
        }
        if !(ISTDP_PERIOD_MIN_TICKS..=ISTDP_PERIOD_MAX_TICKS)
            .contains(&config.istdp_target_period_ticks)
        {
            return Err(ConfigError::IstdpPeriodOutOfRange);
        }
        if config
            .inhibitory_baseline_q16
            .is_some_and(|b| !(0..=MODULATION_ONE_Q16).contains(&b))
        {
            return Err(ConfigError::InhibitoryBaselineOutOfRange);
        }
        if config.stp_class.is_some_and(|c| !c.is_valid()) {
            return Err(ConfigError::StpClassOutOfRange);
        }
        if config.slow_current.is_some_and(|c| !c.is_valid()) {
            return Err(ConfigError::SlowCurrentOutOfRange);
        }
        if config.critic.is_some_and(|c| !c.is_valid()) {
            return Err(ConfigError::CriticOutOfRange);
        }
        if config.critic.is_some() && config.train_capacity == 0 {
            return Err(ConfigError::CriticWithoutTrain);
        }
        if config.critic.is_none() && config.critic_window_ticks != 0 {
            return Err(ConfigError::WindowWithoutCritic);
        }
        if config.critic.is_none() && config.whole_punishment {
            return Err(ConfigError::WholePunishmentWithoutCritic);
        }
        if config.kept_addresses > KEPT_ADDRESSES_MAX {
            return Err(ConfigError::KeptAddressesOutOfRange);
        }
        if config.kept_addresses != 0
            && (config.critic.is_none() || config.critic_window_ticks == 0)
        {
            return Err(ConfigError::KeptAddressesWithoutWindow);
        }
        if config.reads_its_delay && config.kept_addresses == 0 {
            return Err(ConfigError::DelayReadWithoutKept);
        }
        let rate_target = rate_target_of(config.units, config.rate_target_period_ticks)?;
        // The read (ADR-0189): unset at zero; refused short of the windows it sums, and beside a
        // period, which is another target.
        let target_read = match (config.rate_read_windows, config.rate_target_period_ticks) {
            (0, _) => None,
            (1..READ_SUMMED_WINDOWS, _) => return Err(ConfigError::RateReadOutOfRange),
            (left, 0) => Some(TargetRead::Reading { left, sum: 0 }),
            _ => return Err(ConfigError::RateReadBesideTarget),
        };
        // The inhibitory target by the read (ADR-0194): refused without a read to take it from,
        // after the read's own refusals.
        if let (true, None) = (config.istdp_target_read, target_read) {
            return Err(ConfigError::IstdpReadWithoutRead);
        }
        // The probe of the step (ADR-0202): refused without a read to follow, after the read's
        // own refusals and the inhibitory flag's.
        if let (true, None) = (config.step_probe, target_read) {
            return Err(ConfigError::StepProbeWithoutRead);
        }
        // An exponent but one beside a read (ADR-0232), after the read's own refusals: a lead-in
        // with no read stays legal at any exponent.
        if config.depression_exponent != DepressionExponent::One && target_read.is_some() {
            return Err(ConfigError::ExponentBesideRead);
        }
        let workers = config.workers;
        // A unit fires at most once per tick, so one slot per unit holds a tick's spikes.
        let fired_slots = if config.train_capacity == 0 {
            0
        } else {
            config.units
        };
        let (share, stride) = partition(config.units, workers);
        let shared = Arc::new(Shared {
            units: Arena::from_vec(
                (0..config.units as u64)
                    .map(DendriticSuperNeuron::new)
                    .collect(),
            ),
            blocks: Arena::from_vec(vec![SynapseBlock::new(); config.blocks]),
            deltas: Arena::from_vec(vec![PlasticDelta::default(); config.deltas]),
            evicted: (0..config.units).map(|_| AtomicBool::new(false)).collect(),
            needs_rehydration: (0..config.units).map(|_| AtomicBool::new(false)).collect(),
            rehydration_pending: AtomicUsize::new(0),
            in_flight: (0..workers).map(|_| AtomicI64::new(0)).collect(),
            pools: Pools::new(workers, config.nodes_per_worker),
            scheduled: (0..workers.saturating_mul(stride))
                .map(|_| AtomicU64::new(0))
                .collect(),
            asleep: (0..workers.saturating_mul(stride))
                .map(|_| AtomicU64::new(0))
                .collect(),
            since: (0..config.units).map(|_| AtomicU64::new(0)).collect(),
            per_tick: AtomicBool::new(false),
            share,
            stride,
            barrier: SpinBarrier::new(workers),
            injector: Injector::new(config.injector_capacity),
            delivered: (0..workers).map(|_| AtomicU64::new(0)).collect(),
            turns: (0..workers).map(|_| AtomicU64::new(0)).collect(),
            caught_up: (0..workers).map(|_| AtomicU64::new(0)).collect(),
            now: AtomicU32::new(0),
            tick: AtomicU64::new(0),
            modulation: AtomicI32::new(config.modulation_baseline_q16),
            modulation_at_rest: AtomicI32::new(config.modulation_baseline_q16),
            modulation_inhibitory: AtomicI32::new(INHIBITORY_UNSET),
            sources: (0..config.units).map(|_| AtomicBool::new(true)).collect(),
            targets: (0..config.units).map(|_| AtomicBool::new(true)).collect(),
            gain: AtomicU32::new(config.synaptic_gain_q16),
            spikes: (0..workers).map(|_| AtomicU32::new(0)).collect(),
            descendants: (0..workers).map(|_| AtomicU32::new(0)).collect(),
            istdp_alpha: AtomicI32::new(i32::from(istdp_alpha_q1_15(
                config.istdp_target_period_ticks,
            ))),
            depression_exponent: config.depression_exponent,
            zero_sum_source: (0..config.units).map(|_| AtomicU8::new(0)).collect(),
            zero_sum_target: (0..config.units).map(|_| AtomicBool::new(false)).collect(),
            zero_sum_touched: AtomicU64::new(0),
            fired: (0..fired_slots).map(|_| AtomicU32::new(0)).collect(),
            fired_len: AtomicUsize::new(0),
            episodes: Arena::from_vec(vec![Episode::default(); config.episodes]),
            replay: AtomicU32::new(0),
            stop: AtomicBool::new(false),
        });
        // Saturates for a pool that would not fit the address space; `Pools::new` refused that
        // above (a capacity overflow) before this sizes a buffer.
        let total_nodes = workers.saturating_mul(config.nodes_per_worker);
        let mut wheels = build_wheels::<CAP>(workers);
        let mut states: Vec<Worker<CAP>> = (0..workers)
            .map(|id| (id, range_of(id, config.units, share, stride)))
            .map(|(id, (first, words, region))| Worker {
                id,
                first,
                words,
                region,
                wheel: wheels.remove(0),
                batch: Vec::with_capacity(total_nodes),
                due: Vec::with_capacity(CAP),
                spiked: Vec::with_capacity(config.units),
                trace: Vec::with_capacity(config.trace_capacity),
                spike_trace: Vec::with_capacity(config.trace_capacity),
                trace_dropped: 0,
                delivered: 0,
                turns: 0,
                caught_up: 0,
                in_flight: 0,
                descended: 0,
                stp_class: config.stp_class,
                slow_current: config.slow_current,
            })
            .collect();
        let worker0 = states.remove(0);
        let threads = states
            .into_iter()
            .map(|mut worker| {
                let shared = Arc::clone(&shared);
                thread::Builder::new()
                    .name(format!("menteion-worker-{}", worker.id))
                    .stack_size(1 << 20)
                    .spawn(move || {
                        worker.run(&shared);
                        worker
                    })
                    .expect("spawn a worker thread")
            })
            .collect();
        Ok(Self {
            shared,
            worker0,
            threads,
            tick: 0,
            log: None,
            hand: 0,
            evictions: 0,
            rehydrations: 0,
            policy: Policy::default(),
            amendments: Vec::with_capacity(config.amendments),
            amendment_capacity: config.amendments,
            modulator: NeuromodulatorState::new(),
            modulation_baseline_q16: config.modulation_baseline_q16,
            inhibitory_baseline_q16: config.inhibitory_baseline_q16,
            signed_gate: config.signed_gate,
            stp_class: config.stp_class,
            slow_current: config.slow_current,
            critic: config.critic,
            features: if config.critic.is_some() {
                vec![0; config.units]
            } else {
                Vec::new()
            },
            prediction: None,
            critic_window_ticks: config.critic_window_ticks,
            window_opened: 0,
            whole_punishment: config.whole_punishment,
            rate_target_period_ticks: config.rate_target_period_ticks,
            rate_target,
            target_read,
            istdp_target_read: config.istdp_target_read,
            step_probe: config.step_probe,
            kept_addresses: config.kept_addresses,
            // The newest entry and as many before it as a reward's age may reach, each sized
            // here; none while unset.
            kept: (0..if config.kept_addresses == 0 {
                0
            } else {
                usize::from(config.kept_addresses).saturating_add(1)
            })
                .map(|_| KeptEntry {
                    counts: vec![0; config.units],
                    targets: vec![false; config.units],
                    live: false,
                    closed: false,
                })
                .collect(),
            kept_newest: 0,
            kept_let_go: 0,
            // A critic for every age but zero's, whose weights are the units'; each sized here and
            // started from the units' weights at the rule's first reward.
            delay: config.reads_its_delay.then(|| DelayRead {
                in_use: 0,
                started: false,
                weights: (0..=config.kept_addresses)
                    .map(|age| {
                        if age == 0 {
                            Vec::new()
                        } else {
                            vec![0; config.units]
                        }
                    })
                    .collect(),
                scores: (0..=config.kept_addresses)
                    .map(|_| AgeScore::new())
                    .collect(),
                run: None,
            }),
            // The gain the configuration starts the engine at (ADR-0196), 1.0 by default.
            homeostasis: HomeostaticDrivePool {
                control_step_q0_16: config.control_step_q0_16,
                synaptic_gain_q16: config.synaptic_gain_q16,
                sleep_shift: config.sleep_shift,
                ..HomeostaticDrivePool::new()
            },
            hippocampus: HippocampalAttractorState::new(),
            episode_capacity: config.episodes,
            replays: 0,
            depotentiations: 0,
            train: VecDeque::with_capacity(config.train_capacity),
            train_capacity: config.train_capacity,
            train_overwritten: 0,
            merge: Vec::with_capacity(fired_slots),
            induction: Induction::new(
                config.terms,
                config.clauses,
                config.search_shift,
                config.search_budget,
                config.discovery_tag,
            ),
            search_cadence: search_cadence_of(config.search_shift),
            istdp_target_period_ticks: config.istdp_target_period_ticks,
            descendants: 0,
            read_steps: Cell::new(0),
            unsettled: Cell::new(false),
            zero_sum_groups: Vec::new(),
            zero_sum_targets: Vec::new(),
            zero_sum_fractions: Vec::new(),
            zero_sum_at: Vec::new(),
        })
    }

    /// The target period of the inhibitory rule (ADR-0053), in ticks: the configuration's,
    /// or the image's for an engine built from one.
    pub fn istdp_target_period_ticks(&self) -> u32 {
        self.istdp_target_period_ticks
    }

    /// The exponent of every excitatory pairing's depression (ADR-0228), the configuration's.
    pub fn depression_exponent(&self) -> DepressionExponent {
        self.shared.depression_exponent
    }

    /// Holds each of `groups`, its excitatory source units and its value, at a zero sum over
    /// `targets` (ADR-0226, ADR-0228), between ticks, from the next tick: after every tick whose
    /// consolidations moved one of a group's synapses onto a target, the coordinator walks the
    /// group's chains, sums those synapses' weights and scales each back to the value, with its
    /// fraction carried (`zero_sum_round`). Every fraction starts at a half, here, whenever the
    /// groups are set. No groups unsets it, and every run is the run it was, bit for bit. Refused
    /// whole, nothing changed, for more than `ZERO_SUM_GROUPS_MAX` groups, a value below zero, a
    /// unit outside the arena, a source that is not excitatory, or a source named twice. It is in
    /// no image, so an engine decoded from one starts unset.
    pub fn set_zero_sum(&mut self, groups: &[(&[u32], i64)], targets: &[u32]) -> bool {
        if groups.len() > ZERO_SUM_GROUPS_MAX {
            return false;
        }
        let units = self.shared.zero_sum_source.len();
        let mut source = vec![0u8; units];
        let mut held = Vec::with_capacity(groups.len());
        for (g, &(sources, value)) in groups.iter().enumerate() {
            if value < 0 {
                return false;
            }
            // Below `ZERO_SUM_GROUPS_MAX`, so the index plus one fits.
            let index = (g as u8).wrapping_add(1);
            let Some(heads) = self.zero_sum_heads(sources, index, &mut source) else {
                return false;
            };
            held.push((heads, value));
        }
        let Some(target) = self.zero_sum_marks(targets) else {
            return false;
        };
        let on = !groups.is_empty();
        for (flag, &index) in self.shared.zero_sum_source.iter().zip(&source) {
            flag.store(index, Ordering::Relaxed);
        }
        for (flag, &is) in self.shared.zero_sum_target.iter().zip(&target) {
            flag.store(on && is, Ordering::Relaxed);
        }
        self.zero_sum_groups = held;
        self.zero_sum_targets = if on { target } else { Vec::new() };
        self.zero_sum_fractions = if on {
            let slots = self.blocks().len().saturating_mul(SYNAPSES_PER_BLOCK);
            vec![ZERO_SUM_HALF; slots]
        } else {
            Vec::new()
        };
        true
    }

    /// How many groups the zero sum holds (ADR-0228), none while unset.
    pub fn zero_sum_groups(&self) -> usize {
        self.zero_sum_groups.len()
    }

    /// The chains of `sources`, each marked `index` in `source`: none for a unit outside the
    /// arena, a source that is not excitatory, or a unit `source` marks already.
    fn zero_sum_heads(&self, sources: &[u32], index: u8, source: &mut [u8]) -> Option<Vec<u32>> {
        let mut heads = Vec::with_capacity(sources.len());
        for &unit in sources {
            let u = self.units().get(unit as usize)?;
            if Polarity::of_flags(u.flags) != Polarity::Excitatory {
                return None;
            }
            // A unit's chain is set when the network is made and never moves.
            heads.push(u.synapse_slab_idx);
            match source.get_mut(unit as usize) {
                Some(slot @ 0) => *slot = index,
                _ => return None,
            }
        }
        Some(heads)
    }

    /// Each unit of the arena, whether `targets` names it: none for a unit outside the arena.
    fn zero_sum_marks(&self, targets: &[u32]) -> Option<Vec<bool>> {
        let mut target = vec![false; self.shared.zero_sum_source.len()];
        for &unit in targets {
            *target.get_mut(unit as usize)? = true;
        }
        Some(target)
    }

    /// The sum of the synapses from `sources` onto `targets`, between ticks (ADR-0232): the sum a
    /// round of the zero sum scales from (`zero_sum_walk`). A host reads each group's value here
    /// once, from the image the derivation wrote, keeps it, and gives it to
    /// [`Executor::set_zero_sum`] at each load, since no image holds it. None for what that call
    /// refuses of a group: a unit outside the arena, a source that is not excitatory, or a source
    /// named twice.
    pub fn coupling_sum(&self, sources: &[u32], targets: &[u32]) -> Option<i64> {
        let mut source = vec![0u8; self.shared.zero_sum_source.len()];
        let heads = self.zero_sum_heads(sources, 1, &mut source)?;
        let targets = self.zero_sum_marks(targets)?;
        Some(zero_sum_walk(
            self.blocks(),
            &heads,
            &targets,
            &mut Vec::new(),
        ))
    }

    /// After a tick's phases, while the workers wait at the barrier (ADR-0228): every group this
    /// tick's consolidations touched, held at its value by one round of `zero_sum_round`. The
    /// synapse arena alone: a read of the unit arena here would bring the asleep units up to
    /// date before the clock moves (ADR-0224). Nothing while unset, or in a tick that touched
    /// none.
    fn hold_zero_sum(&mut self) {
        let touched = self.shared.zero_sum_touched.swap(0, Ordering::Relaxed);
        if touched == 0 {
            return;
        }
        let groups = std::mem::take(&mut self.zero_sum_groups);
        let targets = std::mem::take(&mut self.zero_sum_targets);
        let mut fractions = std::mem::take(&mut self.zero_sum_fractions);
        let mut at = std::mem::take(&mut self.zero_sum_at);
        let blocks = self.blocks_mut();
        for (g, (heads, value)) in groups.iter().enumerate() {
            if touched.wrapping_shr(g as u32) & 1 != 0 {
                zero_sum_round(blocks, heads, *value, &targets, &mut fractions, &mut at);
            }
        }
        self.zero_sum_groups = groups;
        self.zero_sum_targets = targets;
        self.zero_sum_fractions = fractions;
        self.zero_sum_at = at;
    }

    /// The loader's: the target period an image holds, which outranks the configuration's
    /// (the image defines the run, §8.3). Refused outside the bounds, as `new` refuses it;
    /// the depression the workers read is published at once.
    pub(crate) fn set_istdp_target_period(&mut self, period_ticks: u32) -> bool {
        if !(ISTDP_PERIOD_MIN_TICKS..=ISTDP_PERIOD_MAX_TICKS).contains(&period_ticks) {
            return false;
        }
        self.istdp_target_period_ticks = period_ticks;
        self.shared.istdp_alpha.store(
            i32::from(istdp_alpha_q1_15(period_ticks)),
            Ordering::Relaxed,
        );
        true
    }

    /// Descendants so far (ADR-0054): the spikes that came within `CAUSAL_LATENCY_TICKS` of
    /// a synapse's message reaching the unit, summed by the tally; beside the population's
    /// spikes, the in-loop reading of the branching ratio, the oracle's first-generation
    /// rule without its counterfactual.
    pub fn descendants(&self) -> u64 {
        self.descendants
    }

    /// The engine's modulator record (ADR-0032): one for the engine until macro-columns exist,
    /// read between ticks.
    pub fn modulator(&self) -> &NeuromodulatorState {
        &self.modulator
    }

    /// The modulation with the dopamine signal at rest, from the configuration (ADR-0032).
    pub fn modulation_baseline_q16(&self) -> i32 {
        self.modulation_baseline_q16
    }

    /// The inhibitory baseline (ADR-0086), from the configuration or the image; none while
    /// unset, where every synapse consolidates as before ADR-0086.
    pub fn inhibitory_baseline_q16(&self) -> Option<i32> {
        self.inhibitory_baseline_q16
    }

    /// The signed gate (ADR-0094), from the configuration or the image: while set, an
    /// addressed excitatory synapse consolidates under `clamp(baseline + dopamine, -1, 1)`;
    /// unset, every synapse consolidates as before ADR-0094.
    pub fn signed_gate(&self) -> bool {
        self.signed_gate
    }

    /// The class of short-term plasticity (ADR-0114), from the configuration or the image: while
    /// set, a unit marked `FLAG_FACILITATING` steps under it; none while unset, where every unit
    /// steps under ADR-0019's constants as before ADR-0114.
    pub fn stp_class(&self) -> Option<StpClass> {
        self.stp_class
    }

    /// The slow current (ADR-0123), from the configuration or the image: while set, a unit marked
    /// `FLAG_SLOW` integrates under it; none while unset, where every unit integrates under
    /// `integrate` as before ADR-0123.
    pub fn slow_current(&self) -> Option<SlowCurrent> {
        self.slow_current
    }

    /// The critic (ADR-0131), from the configuration or the image: while set, `reward` takes
    /// its value from the reward; none while unset, where `reward` is as before ADR-0131.
    pub fn critic(&self) -> Option<ValueCritic> {
        self.critic
    }

    /// Each unit's spikes since the previous reward (ADR-0131), the critic's features, in unit
    /// order; empty while the critic is unset.
    pub fn features(&self) -> &[u32] {
        &self.features
    }

    /// What the critic read at the last reward (ADR-0131): the value, the error and what the
    /// modulator received (ADR-0155); none before the first reward, or while the critic is
    /// unset.
    pub fn prediction(&self) -> Option<Prediction> {
        self.prediction
    }

    /// The critic's window (ADR-0134), in ticks, from the configuration or the image: while it
    /// is not zero, a spike counts toward the critic's features only within this many ticks
    /// after the window opened; zero while unset, where every spike since the previous reward
    /// counts, as before ADR-0134.
    pub fn critic_window_ticks(&self) -> u16 {
        self.critic_window_ticks
    }

    /// The tick the critic's window opened at (ADR-0134): the previous reward's the critic took,
    /// or the engine's start before the first — zero for an engine built from a configuration,
    /// the image's tick for one loaded from an image.
    pub fn window_opened(&self) -> u64 {
        self.window_opened
    }

    /// The whole punishment (ADR-0155), from the configuration or the image: while set, a
    /// reward below zero that meets a value below zero reaches the modulator whole; unset, the
    /// modulator receives the critic's error at every reward, as before ADR-0155.
    pub fn whole_punishment(&self) -> bool {
        self.whole_punishment
    }

    /// The rate's target (ADR-0180), a period in ticks, from the configuration or the image:
    /// while it is not zero the controller holds the population's count to it; zero while
    /// unset, where the controller regulates by its estimate, as before ADR-0180.
    pub fn rate_target_period_ticks(&self) -> u32 {
        self.rate_target_period_ticks
    }

    /// The spikes a window holds at the rate's target (ADR-0180): this engine's units times
    /// the window's ticks over the target's period, to nearest; none while the target is unset.
    pub fn rate_target_per_window(&self) -> Option<u32> {
        self.rate_target
    }

    /// The read of the rate's target (ADR-0188, ADR-0189), between ticks: under way, with the
    /// windows it has left and the sum of its last sixteen so far; refused; or none — unset, or
    /// ended in a target, which `rate_target_period_ticks` and `rate_target_per_window` then hold
    /// as though the configuration had set them.
    pub fn target_read(&self) -> Option<TargetRead> {
        self.target_read
    }

    /// The loader's: a read under way an image holds (ADR-0189), `left` windows still to close
    /// and `sum` of its last sixteen so far, which outranks the configuration's (§8.3). Refused
    /// beside a period, as `new` refuses the configuration's: an image has one target.
    pub(crate) fn set_target_read(&mut self, left: u8, sum: u64) -> Result<(), ConfigError> {
        if self.rate_target_period_ticks != 0 {
            return Err(ConfigError::RateReadBesideTarget);
        }
        self.target_read = Some(TargetRead::Reading { left, sum });
        Ok(())
    }

    /// The inhibitory rule's target by the read (ADR-0192, ADR-0194), between ticks: whether the
    /// read under way sets that rule's period beside the rate's at its last window. False while
    /// unset, and once the read has ended, the flag spent with it: the period the read set is then
    /// `istdp_target_period_ticks`.
    pub fn istdp_target_read(&self) -> bool {
        self.istdp_target_read
    }

    /// The loader's: the flag an image carries beside its read under way (ADR-0194), which
    /// outranks the configuration's (§8.3). Refused beside no read under way, as `new` refuses
    /// the configuration's without a read.
    pub(crate) fn set_istdp_target_read(&mut self) -> Result<(), ConfigError> {
        let Some(TargetRead::Reading { .. }) = self.target_read else {
            return Err(ConfigError::IstdpReadWithoutRead);
        };
        self.istdp_target_read = true;
        Ok(())
    }

    /// The probe of the step (ADR-0200, ADR-0202), between ticks: whether the read under way is
    /// followed by the probe, or the probe is under way (`TargetRead::Probing`). False while
    /// unset, and once the probe has ended, the flag spent with it: the step it made is then the
    /// record's `control_step_q0_16`.
    pub fn step_probe(&self) -> bool {
        self.step_probe
    }

    /// The gain the workers are given at the next tick (ADR-0202): the record's
    /// `synaptic_gain_q16`, or, while the probe is under way, the record's moved by the probe's
    /// sixty-fourth for the window its windows left name. The tick publishes this, and nothing
    /// else, to every worker.
    pub fn gain_given_q16(&self) -> u32 {
        match self.target_read {
            Some(TargetRead::Probing { left, .. }) => {
                probe_gain_q16(self.homeostasis.synaptic_gain_q16, left)
            }
            _ => self.homeostasis.synaptic_gain_q16,
        }
    }

    /// The loader's: the probe's flag an image carries (ADR-0202), which outranks the
    /// configuration's (§8.3), beside the read under way it follows, or, where `probe` gives one,
    /// with the probe under way, `left` windows still to close and `sum` so far, beside the
    /// target the read made. Refused beside no read under way and no probe, and a probe beside no
    /// target, as `new` refuses the configuration's flag without a read.
    pub(crate) fn set_step_probe(&mut self, probe: Option<(u8, i64)>) -> Result<(), ConfigError> {
        match (probe, self.target_read, self.rate_target) {
            (None, Some(TargetRead::Reading { .. }), _) => {}
            (Some((left, sum)), None, Some(_)) => {
                self.target_read = Some(TargetRead::Probing { left, sum });
            }
            _ => return Err(ConfigError::StepProbeWithoutRead),
        }
        self.step_probe = true;
        Ok(())
    }

    /// A reward into the dopamine signal, between ticks (ADR-0032): an input, like an
    /// injection, so a run that replays its rewards at the same ticks is the same run. The next
    /// tick's fan-out consolidates under the raised modulation; the signal then decays by
    /// `DOPAMINE_TAU_SHIFT` per tick. Returns the signal.
    ///
    /// With the critic unset the argument is the reward-prediction error and goes to the
    /// modulator itself, as before ADR-0131. With the critic set it is the reward (ADR-0130,
    /// ADR-0131): the value is the units' value weights times their spikes since the previous
    /// reward, the error the reward less the value is what the modulator receives, every
    /// unit's weight then moves by the error times its spikes, shifted, and every count starts
    /// again from zero. The weights are written here, between ticks, where `&mut self` excludes
    /// the tick and every worker waits at the barrier holding no reference into the arena
    /// (axiom A3). The critic's window opens again at this tick (ADR-0134): the spikes of the
    /// next tick are the first after the reward.
    ///
    /// While the whole punishment is set (ADR-0154, ADR-0155), what the modulator receives is
    /// `ValueCritic::received_q16` of the reward and the value: the reward itself where both
    /// are below zero, the error everywhere else. The weights move by the error in every case,
    /// so the value holds the expected reward as it did.
    pub fn reward(&mut self, reward_q16: i32) -> i32 {
        let Some(critic) = self.critic else {
            return self.modulator.reward(reward_q16);
        };
        let mut features = std::mem::take(&mut self.features);
        let signal = self.read_reward(critic, reward_q16, &features);
        features.fill(0);
        self.features = features;
        self.window_opened = self.tick;
        signal
    }

    /// The critic's reading at a reward (ADR-0131, ADR-0155), of the counts given, between ticks:
    /// the value of the units' weights times the counts, the error the reward less the value,
    /// every weight moved by the error times its count, and what the modulator receives — the
    /// error, or while the whole punishment is set the reward where both are below zero —
    /// recorded and sent. Returns the signal. `reward` gives it the live counts and
    /// `reward_kept` an entry's (ADR-0208), so that a kept reward at age zero is a reward, bit for
    /// bit.
    fn read_reward(&mut self, critic: ValueCritic, reward_q16: i32, counts: &[u32]) -> i32 {
        let units = self.units_mut();
        let value_q16 = critic.value_q16(
            units
                .iter()
                .zip(counts)
                .map(|(unit, &count)| (unit.value_weight, count)),
        );
        let error_q16 = ValueCritic::error_q16(reward_q16, value_q16);
        for (unit, &count) in units.iter_mut().zip(counts) {
            unit.value_weight = critic.step(unit.value_weight, error_q16, count);
        }
        let received_q16 = if self.whole_punishment {
            ValueCritic::received_q16(reward_q16, value_q16)
        } else {
            error_q16
        };
        self.prediction = Some(Prediction {
            value_q16,
            error_q16,
            received_q16,
        });
        self.modulator.reward(received_q16)
    }

    /// The kept addresses (ADR-0207, ADR-0208), from the configuration: the oldest age a reward
    /// may name; zero while unset, where the engine keeps no address.
    pub fn kept_addresses(&self) -> u8 {
        self.kept_addresses
    }

    /// The entries a trial's close wrote over while their reward was still to come (ADR-0208):
    /// a reward that would have named one finds no address there.
    pub fn kept_let_go(&self) -> u64 {
        self.kept_let_go
    }

    /// The ring's slot of the entry kept `age` closes before the newest (ADR-0208). Refused while
    /// no address is kept and above the oldest age kept.
    fn kept_slot(&self, age: u8) -> Result<usize, AddressError> {
        if self.kept_addresses == 0 {
            return Err(AddressError::NotKept);
        }
        if age > self.kept_addresses {
            return Err(AddressError::AgeNotKept);
        }
        // The newest is below the ring's length and the age at most one less, so the sum is above
        // the age and below twice the length.
        let len = self.kept.len();
        self.kept_newest
            .wrapping_add(len)
            .wrapping_sub(usize::from(age))
            .checked_rem(len)
            .ok_or(AddressError::NotKept)
    }

    /// The address kept at `age` (ADR-0208), between ticks: the critic's counts the trial's close
    /// found, whose units not zero are its sources, and its targets by unit. None while no
    /// address is kept, above the oldest age kept, and where none is kept at that age: fewer
    /// trials have closed, or its reward was given.
    pub fn kept_at(&self, age: u8) -> Option<(&[u32], &[bool])> {
        let slot = self.kept_slot(age).ok()?;
        self.kept
            .get(slot)
            .filter(|entry| entry.live)
            .map(|entry| (entry.counts.as_slice(), entry.targets.as_slice()))
    }

    /// Between ticks: a trial's close, its address kept (ADR-0207, ADR-0208). The critic's counts
    /// — the spikes its window admitted since it last opened — and the units of `targets` become
    /// the newest entry, the oldest written over; the counts start again from zero and the
    /// critic's window opens again at this tick, as at a reward. The addressed set, the weights
    /// and the signal are not touched: the address is written by the reward that names the trial
    /// ([`Executor::reward_kept`]). An entry written over while its reward was still to come is
    /// counted ([`Executor::kept_let_go`]). Refused as `address_drawn` is, while no address is
    /// kept, and whole for a target outside the arena, nothing changed. Called between a trial's
    /// last tick and its reward in place of `address_drawn`, the counts are that trial's window.
    pub fn keep_address<T>(&mut self, targets: T) -> Result<(), AddressError>
    where
        T: IntoIterator<Item = u32>,
        T::IntoIter: Clone,
    {
        self.draws()?;
        let newest = self.kept_slot(0)?;
        let targets = targets.into_iter();
        if targets
            .clone()
            .any(|unit| unit as usize >= self.shared.targets.len())
        {
            return Err(AddressError::NoSuchUnit);
        }
        let slot = newest
            .wrapping_add(1)
            .checked_rem(self.kept.len())
            .ok_or(AddressError::NotKept)?;
        let entry = self.kept.get_mut(slot).ok_or(AddressError::NotKept)?;
        if entry.live {
            self.kept_let_go = self.kept_let_go.saturating_add(1);
        }
        for (kept, count) in entry.counts.iter_mut().zip(self.features.iter_mut()) {
            *kept = *count;
            *count = 0;
        }
        entry.targets.fill(false);
        for unit in targets {
            if let Some(flag) = entry.targets.get_mut(unit as usize) {
                *flag = true;
            }
        }
        entry.live = true;
        entry.closed = true;
        self.kept_newest = slot;
        self.window_opened = self.tick;
        Ok(())
    }

    /// A reward for the trial whose address was kept `age` closes before the newest, between ticks
    /// (ADR-0207, ADR-0208). The entry's address becomes the addressed set — its sources the units
    /// whose kept count is not zero, its targets the units it kept — as `address_drawn` would have
    /// written it at that trial's close; the critic reads the value from the entry's counts, moves
    /// the weights by the error times them and sends what the modulator receives, as `reward` does
    /// from the live counts; and the entry is spent. The live counts and the critic's window are
    /// not touched. Returns the signal. At age zero, after a close, it is `address_drawn` and then
    /// `reward` at that close, bit for bit. Refused as `address_drawn` is, while no address is
    /// kept, above the oldest age kept and where none is kept at that age, nothing changed.
    pub fn reward_kept(&mut self, reward_q16: i32, age: u8) -> Result<i32, AddressError> {
        self.draws()?;
        let slot = self.kept_slot(age)?;
        let Some(critic) = self.critic else {
            return Err(AddressError::NoCritic);
        };
        let entry = self
            .kept
            .get_mut(slot)
            .filter(|entry| entry.live)
            .ok_or(AddressError::NoKeptAddress)?;
        entry.live = false;
        let counts = std::mem::take(&mut entry.counts);
        for (flag, &count) in self.shared.sources.iter().zip(&counts) {
            flag.store(count != 0, Ordering::Relaxed);
        }
        for (flag, &target) in self.shared.targets.iter().zip(&entry.targets) {
            flag.store(target, Ordering::Relaxed);
        }
        let signal = self.read_reward(critic, reward_q16, &counts);
        if let Some(entry) = self.kept.get_mut(slot) {
            entry.counts = counts;
        }
        Ok(signal)
    }

    /// A reward given without its age, between ticks (ADR-0216, ADR-0217). Each age's critic, from
    /// zero to the oldest kept, reads the value of its weights times the counts of the trial its
    /// age closes before the newest, spent or not, takes the error, records its magnitude in its
    /// score and moves its weights by the error times those counts; an age no close has reached
    /// yet reads nothing. The rule starts at its first such reward: every age's critic starts
    /// there from the engine's weights. While the age in use is zero, one other age leading
    /// (`delay_leader`, by `DELAY_WINDOW` times the reward's magnitude over $2^5$) at
    /// `DELAY_RUN` rewards running moves the engine to it, its critic becoming the engine's, and
    /// the engine never moves again. Then the age in use's entry, where it is live, is the
    /// reward's: its address written as `reward_kept` writes it, that age's error sent — or the
    /// reward where the whole punishment takes it — and the entry spent; returns the signal.
    /// Where it is not live the reward reaches no address and no modulator, and returns none.
    /// The live counts and the window are not touched. Refused while the rule is unset, nothing
    /// changed.
    pub fn reward_untold(&mut self, reward_q16: i32) -> Result<Option<i32>, AddressError> {
        let Some(mut delay) = self.delay.take() else {
            return Err(AddressError::DelayUnread);
        };
        let result = self.read_untold(&mut delay, reward_q16);
        self.delay = Some(delay);
        result
    }

    /// `reward_untold` with the rule's state lent out of the executor.
    fn read_untold(
        &mut self,
        delay: &mut DelayRead,
        reward_q16: i32,
    ) -> Result<Option<i32>, AddressError> {
        let Some(critic) = self.critic else {
            return Err(AddressError::NoCritic);
        };
        if !delay.started {
            let units = self.units();
            for weights in &mut delay.weights {
                for (weight, unit) in weights.iter_mut().zip(units) {
                    *weight = unit.value_weight;
                }
            }
            delay.started = true;
        }
        // Every age's critic, from the counts its age's entry holds.
        let mut read = [None; DELAY_AGES];
        for age in 0..=self.kept_addresses {
            let slot = self.kept_slot(age)?;
            let Some(entry) = self.kept.get_mut(slot).filter(|entry| entry.closed) else {
                continue;
            };
            let counts = std::mem::take(&mut entry.counts);
            let reading = if age == delay.in_use {
                let units = self.units_mut();
                let value_q16 = critic.value_q16(
                    units
                        .iter()
                        .zip(&counts)
                        .map(|(unit, &count)| (unit.value_weight, count)),
                );
                let error_q16 = ValueCritic::error_q16(reward_q16, value_q16);
                for (unit, &count) in units.iter_mut().zip(&counts) {
                    unit.value_weight = critic.step(unit.value_weight, error_q16, count);
                }
                Some((value_q16, error_q16))
            } else {
                delay.weights.get_mut(usize::from(age)).map(|weights| {
                    let value_q16 =
                        critic.value_q16(weights.iter().copied().zip(counts.iter().copied()));
                    let error_q16 = ValueCritic::error_q16(reward_q16, value_q16);
                    for (weight, &count) in weights.iter_mut().zip(&counts) {
                        *weight = critic.step(*weight, error_q16, count);
                    }
                    (value_q16, error_q16)
                })
            };
            if let Some(entry) = self.kept.get_mut(slot) {
                entry.counts = counts;
            }
            if let (Some((_, error_q16)), Some(score)) =
                (reading, delay.scores.get_mut(usize::from(age)))
            {
                score.record(error_q16.unsigned_abs());
            }
            if let Some(at) = read.get_mut(usize::from(age)) {
                *at = reading;
            }
        }
        // The move (ADR-0216): from age zero, once, to an age other than zero that has led at
        // `DELAY_RUN` rewards running.
        if delay.in_use == 0 {
            let mut scores = [None; DELAY_AGES];
            for (at, score) in scores.iter_mut().zip(&delay.scores) {
                *at = score.score();
            }
            let lead = u64::from(reward_q16.unsigned_abs()).saturating_mul(DELAY_WINDOW as u64)
                >> DELAY_LEAD_SHIFT;
            let ages = usize::from(self.kept_addresses).saturating_add(1);
            delay.run = delay_run_after(
                delay.run,
                delay_leader(scores.get(..ages).unwrap_or(&[]), lead),
            );
            if let Some((age, DELAY_RUN)) = delay.run {
                if let Some(mut weights) =
                    delay.weights.get_mut(usize::from(age)).map(std::mem::take)
                {
                    for (unit, weight) in self.units_mut().iter_mut().zip(weights.iter_mut()) {
                        core::mem::swap(&mut unit.value_weight, weight);
                    }
                    if let Some(zero) = delay.weights.first_mut() {
                        *zero = weights;
                    }
                    delay.in_use = age;
                }
            }
        }
        // The age in use's entry, where it is live, takes the reward.
        let slot = self.kept_slot(delay.in_use)?;
        let Some(entry) = self.kept.get_mut(slot).filter(|entry| entry.live) else {
            return Ok(None);
        };
        let Some((value_q16, error_q16)) = read.get(usize::from(delay.in_use)).copied().flatten()
        else {
            return Ok(None);
        };
        entry.live = false;
        for (flag, &count) in self.shared.sources.iter().zip(&entry.counts) {
            flag.store(count != 0, Ordering::Relaxed);
        }
        for (flag, &target) in self.shared.targets.iter().zip(&entry.targets) {
            flag.store(target, Ordering::Relaxed);
        }
        let received_q16 = if self.whole_punishment {
            ValueCritic::received_q16(reward_q16, value_q16)
        } else {
            error_q16
        };
        self.prediction = Some(Prediction {
            value_q16,
            error_q16,
            received_q16,
        });
        Ok(Some(self.modulator.reward(received_q16)))
    }

    /// Whether the engine reads the delay of a reward given without its age (ADR-0217), from the
    /// configuration.
    pub fn reads_its_delay(&self) -> bool {
        self.delay.is_some()
    }

    /// The age a reward given without its age is consolidated at (ADR-0217): zero until the
    /// move, the age moved to after it; none while the rule is unset.
    pub fn delay_age(&self) -> Option<u8> {
        self.delay.as_ref().map(|delay| delay.in_use)
    }

    /// The score of `age` (ADR-0216, ADR-0217): the sum of the magnitudes of its critic's errors
    /// at its last `DELAY_WINDOW` rewards, once it has read as many; none before, above the
    /// oldest age kept and while the rule is unset.
    pub fn delay_score(&self, age: u8) -> Option<u64> {
        self.delay
            .as_ref()?
            .scores
            .get(usize::from(age))
            .and_then(AgeScore::score)
    }

    /// The age other than zero that led at the last rewards read from age zero, and at how many
    /// running (ADR-0216, ADR-0217): `DELAY_RUN` at the move, and as it stood then from there on;
    /// none where no such age led at the last, and while the rule is unset.
    pub fn delay_run(&self) -> Option<(u8, u8)> {
        self.delay.as_ref()?.run
    }

    /// The weight of `unit` in `age`'s critic (ADR-0217): the unit's own value weight for the
    /// age in use, whose critic is the engine's, and for every age before the rule's first
    /// reward; none above the oldest age kept, outside the arena and while the rule is unset.
    pub fn delay_weight(&self, age: u8, unit: usize) -> Option<i16> {
        let delay = self.delay.as_ref()?;
        if age > self.kept_addresses {
            return None;
        }
        if age == delay.in_use || !delay.started {
            return self.units().get(unit).map(|u| u.value_weight);
        }
        delay.weights.get(usize::from(age))?.get(unit).copied()
    }

    /// Between ticks: the addressed set becomes the synapses from a unit of `sources` onto a
    /// unit of `targets`, exactly (ADR-0068). From the next tick an addressed synapse
    /// consolidates under `clamp(baseline + dopamine, 0, 1)` and every other synapse under
    /// the baseline alone, until the set is written again; the dopamine signal itself is one
    /// for the engine and is not addressed. Every unit is a source and a target until this is
    /// called, which is the rule before the addressing existed. A set that names a unit
    /// outside the arena, on either side, is refused whole, and the set stands as it was. An
    /// input between ticks, like a reward: a run that replays its addressings at the same
    /// ticks is the same run.
    pub fn address<S, T>(&mut self, sources: S, targets: T) -> Result<(), AddressError>
    where
        S: IntoIterator<Item = u32>,
        S::IntoIter: Clone,
        T: IntoIterator<Item = u32>,
        T::IntoIter: Clone,
    {
        let sources = sources.into_iter();
        let targets = targets.into_iter();
        let len = self.shared.sources.len();
        if sources.clone().any(|unit| unit as usize >= len)
            || targets.clone().any(|unit| unit as usize >= len)
        {
            return Err(AddressError::NoSuchUnit);
        }
        Self::write_side(&self.shared.sources, sources);
        Self::write_side(&self.shared.targets, targets);
        Ok(())
    }

    /// One side of the addressed set: every flag cleared, then the units named set. Every
    /// unit is below the slice's length, which `address` held before it wrote either side.
    fn write_side(side: &[AtomicBool], units: impl Iterator<Item = u32>) {
        for flag in side {
            flag.store(false, Ordering::Relaxed);
        }
        for unit in units {
            if let Some(flag) = side.get(unit as usize) {
                flag.store(true, Ordering::Relaxed);
            }
        }
    }

    /// Whether the address can be drawn (ADR-0139): the critic set and its window set. Refused
    /// otherwise, the critic named first, since the configuration and the loader never set a
    /// window without it.
    pub fn draws(&self) -> Result<(), AddressError> {
        if self.critic.is_none() {
            Err(AddressError::NoCritic)
        } else if self.critic_window_ticks == 0 {
            Err(AddressError::NoWindow)
        } else {
            Ok(())
        }
    }

    /// Between ticks: the address drawn by the engine (ADR-0139). The sources are the units
    /// whose critic count is not zero — the spikes the critic's window admitted since the
    /// previous reward the critic took, or since the engine's start before the first — and the
    /// targets exactly the units of `targets`. From the next tick it is the addressed set, read
    /// as the set `address` writes; the counts, the weights and the signal are not touched.
    /// Refused while the critic or its window is unset (`draws`), and whole for a target outside
    /// the arena, the set standing as it was either way. Called between a trial's last tick and
    /// its reward, the counts are that trial's window.
    pub fn address_drawn<T>(&mut self, targets: T) -> Result<(), AddressError>
    where
        T: IntoIterator<Item = u32>,
        T::IntoIter: Clone,
    {
        self.draws()?;
        let targets = targets.into_iter();
        if targets
            .clone()
            .any(|unit| unit as usize >= self.shared.targets.len())
        {
            return Err(AddressError::NoSuchUnit);
        }
        for (unit, flag) in self.shared.sources.iter().enumerate() {
            flag.store(
                self.features.get(unit).is_some_and(|&count| count != 0),
                Ordering::Relaxed,
            );
        }
        Self::write_side(&self.shared.targets, targets);
        Ok(())
    }

    /// Between ticks: every unit a source and a target, the rule before ADR-0068 and the
    /// state a new executor starts in.
    pub fn address_all(&mut self) {
        for flag in self.shared.sources.iter().chain(self.shared.targets.iter()) {
            flag.store(true, Ordering::Relaxed);
        }
    }

    /// True when `unit` is an addressed source (ADR-0068); false outside the arena.
    pub fn is_source(&self, unit: u32) -> bool {
        self.shared.is_source(unit)
    }

    /// True when `unit` is an addressed target (ADR-0068); false outside the arena.
    pub fn is_target(&self, unit: u32) -> bool {
        self.shared.is_target(unit)
    }

    /// The addressed sources and the addressed targets, counted (ADR-0068), between ticks.
    pub fn addressed_counts(&self) -> (usize, usize) {
        let count = |side: &[AtomicBool]| {
            side.iter()
                .filter(|flag| flag.load(Ordering::Relaxed))
                .count()
        };
        (count(&self.shared.sources), count(&self.shared.targets))
    }

    /// The loader's: the modulator an image holds.
    pub(crate) fn set_modulator(&mut self, modulator: NeuromodulatorState) {
        self.modulator = modulator;
    }

    /// The loader's: the baseline an image holds, which outranks the configuration's (the
    /// image defines the run, §8.3). Refused outside $[0, 1]$, as `new` refuses it.
    pub(crate) fn set_modulation_baseline(&mut self, baseline_q16: i32) -> bool {
        if !(0..=MODULATION_ONE_Q16).contains(&baseline_q16) {
            return false;
        }
        self.modulation_baseline_q16 = baseline_q16;
        true
    }

    /// The loader's: the inhibitory baseline an image holds, set or unset, which outranks the
    /// configuration's (§8.3). Refused when set outside [0, 1], as `new` refuses it, the
    /// baseline standing as it was.
    pub(crate) fn set_inhibitory_baseline(&mut self, baseline_q16: Option<i32>) -> bool {
        if baseline_q16.is_some_and(|b| !(0..=MODULATION_ONE_Q16).contains(&b)) {
            return false;
        }
        self.inhibitory_baseline_q16 = baseline_q16;
        true
    }

    /// The loader's: the signed gate an image holds, set or unset, which outranks the
    /// configuration's (§8.3).
    pub(crate) fn set_signed_gate(&mut self, set: bool) {
        self.signed_gate = set;
    }

    /// The loader's: the clock resumes at the tick the image was written (ADR-0033), between
    /// ticks, before anything reads a stamp against it. The critic counts from the load, so its
    /// window opens there (ADR-0134).
    pub(crate) fn resume_clock(&mut self, tick: u64) {
        self.tick = tick;
        self.window_opened = tick;
        self.shared.now.store(tick as u32, Ordering::Relaxed);
    }

    /// The engine's homeostasis record (ADR-0036): the spikes of the open bin, the estimator's
    /// window, the last branching-ratio estimate, the control step and the synaptic gain; one
    /// for the engine until macro-columns exist, read between ticks.
    pub fn homeostasis(&self) -> &HomeostaticDrivePool {
        &self.homeostasis
    }

    /// The bins the homeostasis record's window holds at tick `tick`, as the tally leaves
    /// them: the bins closed since the window began. An image whose record says otherwise
    /// was not written by this executor.
    pub(crate) fn window_bins_at(tick: u64) -> u8 {
        let bins_per_window = WINDOW_CADENCE.period() >> ACTIVITY_BIN_SHIFT;
        ((tick >> ACTIVITY_BIN_SHIFT) & bins_per_window.wrapping_sub(1)) as u8
    }

    /// The loader's: the homeostasis record an image holds, whose gain and step outrank the
    /// configuration's (the image defines the run, §8.3). Refused for a record that is not
    /// well formed, as the rules would never leave it.
    pub(crate) fn set_homeostasis(&mut self, pool: HomeostaticDrivePool) -> bool {
        if !pool.is_well_formed() {
            return false;
        }
        self.homeostasis = pool;
        true
    }

    /// The sleep stage (ADR-0037): `STAGE_AWAKE`, `STAGE_SWS` or `STAGE_REM` of
    /// `menteion-homeostasis`, read between ticks.
    pub fn sleep_stage(&self) -> u8 {
        self.homeostasis.sleep_stage
    }

    /// An input between ticks (ADR-0037): whatever the stage, the engine is awake from the
    /// next tick, its pressure kept. Returns whether it was asleep.
    pub fn wake(&mut self) -> bool {
        self.homeostasis.wake()
    }

    /// The engine's hippocampal record (ADR-0038): the ledger's length and the hand the next
    /// ripple starts from, read between ticks.
    pub fn hippocampus(&self) -> &HippocampalAttractorState {
        &self.hippocampus
    }

    /// The ledger (ADR-0038): every episode tagged so far, in order, read between ticks.
    pub fn episodes(&self) -> &[Episode] {
        // SAFETY: `&self` between ticks; every worker is parked at the barrier, and phase 3's
        // reader is worker 0, which is this thread.
        let all = unsafe { self.shared.episodes.as_slice() };
        &all[..(self.hippocampus.episodes as usize).min(all.len())]
    }

    /// Room left in the ledger for tagged episodes.
    pub fn episode_room(&self) -> usize {
        // The ledger never holds more than its capacity (`tag_episode`, `load_episode`).
        self.episode_capacity
            .saturating_sub(self.hippocampus.episodes as usize)
    }

    /// Slow-wave replays delivered so far (ADR-0038).
    pub fn replays(&self) -> u64 {
        self.replays
    }

    /// REM ripples that lowered an episode's tag so far (ADR-0038).
    pub fn depotentiations(&self) -> u64 {
        self.depotentiations
    }

    /// The executor's own spike train (ADR-0050), between ticks: the last
    /// `Config::train_capacity` spikes as `(tick, unit)`, in tick order and unit order
    /// within a tick, whichever worker ran the unit, so that the train is the same on every
    /// worker count. A ring made contiguous, so the call may move its entries; nothing
    /// allocates. Empty for an executor that keeps no train.
    pub fn train(&mut self) -> &[(u32, u32)] {
        self.train.make_contiguous()
    }

    /// The spikes the train let go because the ring was full.
    pub fn train_overwritten(&self) -> u64 {
        self.train_overwritten
    }

    /// `Config::train_capacity`.
    pub fn train_capacity(&self) -> usize {
        self.train_capacity
    }

    // ------------------------------------------------ the term arena and the store (ADR-0052)

    /// The engine's term arena (ADR-0052): the nodes in use, in order, read between ticks.
    pub fn terms(&self) -> &[TermNode] {
        self.induction.terms()
    }

    /// The engine's clause store: the arena indices of its clauses, in order.
    pub fn clauses(&self) -> &[u32] {
        self.induction.clauses()
    }

    /// The engine's affect state (ADR-0043): primed to the store's description length, the
    /// valence rule reading an invention's drop.
    pub fn affect(&self) -> &InteroceptiveState {
        &self.induction.affect
    }

    /// The engine's induction record (ADR-0052): the arena's cursor and counters, the
    /// store's length, the search's cursor, budget, cadence and tag.
    pub fn induction(&self) -> &InductionState {
        &self.induction.record
    }

    /// Searches run so far, inside the tick and between ticks.
    pub fn searches(&self) -> u64 {
        self.induction.searches
    }

    /// Inventions committed by them.
    pub fn inventions(&self) -> u64 {
        self.induction.inventions
    }

    /// Rewarded searches whose moment the ledger or the train refused to tag: the reward
    /// stood, no episode was bound.
    pub fn untagged(&self) -> u64 {
        self.induction.untagged
    }

    /// Searches that ended in an error of their own (the store or the arena full, a bound,
    /// a malformed store), their commits before it standing.
    pub fn search_failures(&self) -> u64 {
        self.induction.failures
    }

    /// A compaction of the engine's arena between ticks (ADR-0056): the nodes the store does
    /// not reach are reclaimed, the store's indices and the induction record's cursor moved,
    /// the last search's discoveries cleared (their indices moved); the store reads the same
    /// node for node, so the affect state and the search's cursor stand. The same runs
    /// inside the tick at every entry into slow-wave sleep. `NoArena` for an engine without
    /// one; `Malformed` if the rule refuses, which the engine's own arena, bottom-up by
    /// construction, cannot make it do.
    pub fn compact(&mut self) -> Result<Compaction, TermError> {
        self.induction.compact()
    }

    /// Compactions run, between ticks and at slow-wave onsets (ADR-0056).
    pub fn compactions(&self) -> u64 {
        self.induction.compactions
    }

    /// Nodes the compactions reclaimed.
    pub fn reclaimed(&self) -> u64 {
        self.induction.reclaimed
    }

    /// The compaction at a slow-wave onset (ADR-0056; whitepaper §8.8's glymphatic row):
    /// once per entry into the stage, between the window's regulation and the next tick.
    /// An engine without an arena reclaims nothing; the engine's own arena cannot make the
    /// rule refuse.
    fn compact_at_onset(&mut self) {
        match self.induction.compact() {
            Ok(_) | Err(TermError::NoArena) => {}
            Err(_) => abort("the engine's own arena refused a compaction"),
        }
    }

    /// The last search's commits, in order (ADR-0045): the invention, the store's length
    /// before and after it, its valence and its reward; the first is the one the rewarded
    /// moment's episode is bound to. Empty after a search that ended in an error.
    pub fn discoveries(&self) -> &[Discovery] {
        self.induction.discoveries()
    }

    /// An input between ticks (ADR-0052): `node` into the arena, its index returned. Refused
    /// for an engine without an arena, a full arena, and a node that is not well formed, is
    /// empty, names a child at or beyond the arena's cursor (a host builds bottom-up) or is a
    /// variable outside the binding table. A variable moves the record's `next_variable`
    /// above its number. Like an injection, a term is part of the trace: a run that asserts
    /// the same terms at the same ticks is the same run.
    /// The index returned names the node until the next compaction (ADR-0056): a
    /// slow-wave onset inside the tick, or [`compact`](Self::compact) between ticks, moves
    /// every node the store does not reach, so a host builds a clause and asserts it
    /// before letting the ticks cross an onset, or holds an index it will not use.
    pub fn term(&mut self, node: TermNode) -> Result<u32, TermError> {
        self.induction.term(node)
    }

    /// An input between ticks (ADR-0052): the clause `head ← body` into the arena and its
    /// index into the store, the affect state primed to the store's new length; returns the
    /// clause's index. Refused as `term` refuses, for a full store, for a body longer than
    /// `MAX_BODY`, and for a clause whose size cannot be measured.
    /// The index returned, and every index the clause names, stand until the next
    /// compaction (ADR-0056), when the store's nodes move; the store's positions and the
    /// record's cursor do not.
    pub fn assert_clause(&mut self, head: u32, body: &[u32]) -> Result<u32, TermError> {
        self.induction.assert_clause(head, body)
    }

    /// An input between ticks (ADR-0052): the episode at `index` bound to `symbol`, the id of
    /// what its pattern stands for. Refused for an index the ledger does not hold, a symbol
    /// of zero and an episode already bound (`Episode::bind`).
    pub fn bind_episode(&mut self, index: u32, symbol: u32) -> Result<(), TagError> {
        if index >= self.hippocampus.episodes {
            return Err(TagError::NoSuchEpisode);
        }
        // SAFETY: `&mut self` between ticks; every worker is parked at the barrier.
        let Some(episode) = (unsafe { self.shared.episodes.get_mut(index as usize) }) else {
            abort("the ledger's length exceeds its arena");
        };
        if episode.bind(symbol) {
            Ok(())
        } else {
            Err(TagError::Bound)
        }
    }

    /// The discovery loop between ticks (ADR-0052; the composition ADR-0050 ran over a
    /// caller's store): one search over the engine's own store from the record's cursor with
    /// the record's budget (ADR-0045), the committed rewards' total into the modulator when
    /// positive (ADR-0043), then the pattern active in the ripple before now (its densest
    /// basal time constant, from the executor's own train) tagged once with the record's tag
    /// and bound to the first commit's predicate (ADR-0048); a search of two commits binds
    /// the one episode to the first and reports both. The search's refusals as it gives
    /// them, its commits before one standing; the ledger's or the train's refusal of the
    /// moment as `Tag`, the reward the modulator's by then. The same loop runs inside the
    /// tick on the record's cadence while the engine is awake, its refusals counted
    /// (`untagged`, `search_failures`) and never returned.
    /// The report's indices name nodes until the next compaction (ADR-0056), which clears
    /// the last search's discoveries for that reason.
    pub fn discover(&mut self) -> Result<DiscoverReport, DiscoverError> {
        self.induce()
    }

    fn induce(&mut self) -> Result<DiscoverReport, DiscoverError> {
        self.induction.searches = self.induction.searches.saturating_add(1);
        let report = match self.induction.search() {
            Ok(report) => report,
            Err(e) => {
                self.induction.failures = self.induction.failures.saturating_add(1);
                return Err(DiscoverError::Discovery(e));
            }
        };
        self.induction.inventions = self
            .induction
            .inventions
            .saturating_add(u64::from(report.commits));
        if report.reward_total_q16 <= 0 {
            return Ok(DiscoverReport {
                search: report,
                signal_q16: self.modulator.dopamine_rpe,
                tagged: None,
            });
        }
        // The committed rewards are the engine's own (ADR-0043), not a reward it receives: they
        // go to the modulator itself, whether or not the critic is set (ADR-0131).
        let signal_q16 = self.modulator.reward(report.reward_total_q16);
        let at = self.tick as u32;
        let tag = self.induction.record.tag;
        let predicate = self
            .induction
            .first_invention()
            .map_or(0, |d| d.invention.predicate);
        let tagged = crate::episode::tag_burst_in(
            self,
            at.saturating_sub(DISCOVERY_WINDOW),
            at,
            COINCIDENCE_TICKS,
            tag,
        )
        .and_then(|(burst, episode, _, _)| {
            self.bind_episode(episode, predicate)?;
            Ok((episode, burst))
        });
        match tagged {
            Ok(tagged) => Ok(DiscoverReport {
                search: report,
                signal_q16,
                tagged: Some(tagged),
            }),
            Err(e) => {
                self.induction.untagged = self.induction.untagged.saturating_add(1);
                Err(DiscoverError::Tag(e))
            }
        }
    }

    /// After the tally, between ticks (ADR-0052): the loop on its cadence while awake.
    fn search_on_cadence(&mut self) {
        let Some(cadence) = self.search_cadence else {
            return;
        };
        if self.homeostasis.sleep_stage != STAGE_AWAKE || !cadence.is_due(self.tick) {
            return;
        }
        // The refusals are counted by `induce`; inside the tick there is no caller to
        // return them to.
        let _ = self.induce();
    }

    /// The loader's: a node from an image, appended at the arena's cursor; refused as `term`
    /// refuses (so no loaded arena holds a forward reference or a cycle).
    pub(crate) fn load_term(&mut self, node: TermNode) -> bool {
        self.induction.load_term(node)
    }

    /// The loader's: a store index from an image; refused for an index at or beyond the
    /// cursor, a node that is not a clause, an index the store holds already or a full store.
    pub(crate) fn load_clause(&mut self, index: u32) -> bool {
        self.induction.load_clause(index)
    }

    /// The loader's: the induction record an image holds, after its arena and store; its
    /// cadence, budget and tag outrank the configuration's (§8.3). Refused for a record that
    /// is not well formed or does not describe what was loaded.
    pub(crate) fn set_induction(&mut self, record: InductionState) -> bool {
        if !self.induction.set_record(record) {
            return false;
        }
        self.search_cadence = search_cadence_of(record.search_shift);
        true
    }

    /// The loader's: the affect state an image holds. Refused for a state that is not well
    /// formed or not primed to the loaded store's length.
    pub(crate) fn set_affect(&mut self, state: InteroceptiveState) -> bool {
        self.induction.set_affect(state)
    }

    /// The arena's and the store's capacities: what the image held plus the configuration's
    /// room.
    pub fn term_capacity(&self) -> (usize, usize) {
        self.induction.capacity()
    }

    /// Whether the tick being merged counts toward the critic's features (ADR-0134): always
    /// while the window is unset; while it is set, when the tick is fewer than the window's
    /// length after the tick the window opened at. The clock only moves forward from the
    /// opening, so the difference is the ticks since.
    fn in_window(&self) -> bool {
        self.critic_window_ticks == 0
            || self.tick.wrapping_sub(self.window_opened) < u64::from(self.critic_window_ticks)
    }

    /// After a tick, between ticks (ADR-0050): the units the workers fired this tick, sorted,
    /// into the ring at `now`, the oldest let go when the ring is full. Nothing for an
    /// executor that keeps no train.
    fn merge_spikes(&mut self, now: u32) {
        if self.train_capacity == 0 {
            return;
        }
        let fired = self.shared.fired_len.swap(0, Ordering::Relaxed);
        self.merge.clear();
        for slot in self.shared.fired.iter().take(fired) {
            self.merge.push(slot.load(Ordering::Relaxed));
        }
        self.merge.sort_unstable();
        // The critic's features (ADR-0131): each merged spike counted against its unit, as the
        // train receives it, so the count is the unit's spikes since the previous reward however
        // many the ring has let go since. Unset, there is no count. With the window set
        // (ADR-0134), only a tick fewer than its length after the window opened counts; `now` is
        // `self.tick`'s low word, and the comparison takes the clock itself.
        if self.in_window() {
            for &unit in &self.merge {
                if let Some(count) = self.features.get_mut(unit as usize) {
                    *count = count.saturating_add(1);
                }
            }
        }
        for &unit in &self.merge {
            if self.train.len() >= self.train_capacity {
                self.train.pop_front();
                self.train_overwritten = self.train_overwritten.saturating_add(1);
            }
            self.train.push_back((now, unit));
        }
    }

    /// An input between ticks (ADR-0038): appends an episode of `units`, tagged at this tick
    /// with `priority` REM ripples to survive, and returns its index in the ledger. Refused
    /// for a full ledger, a unit outside the arena, or a pattern `Episode::tag` refuses;
    /// nothing changes then. Like an injection or a reward, a tag is part of the trace: a
    /// run that replays its tags at the same ticks is the same run.
    pub fn tag_episode(&mut self, units: &[u32], priority: u8) -> Result<u32, TagError> {
        if self.episode_room() == 0 {
            return Err(TagError::LedgerFull);
        }
        if units
            .iter()
            .any(|&unit| unit as usize >= self.shared.units.len())
        {
            return Err(TagError::NoSuchUnit);
        }
        let episode =
            Episode::tag(self.tick as u32, units, priority).ok_or(TagError::InvalidPattern)?;
        let index = self.hippocampus.episodes as usize;
        // SAFETY: `&mut self` between ticks; every worker is parked at the barrier.
        let Some(slot) = (unsafe { self.shared.episodes.get_mut(index) }) else {
            abort("the ledger's length exceeds its arena");
        };
        *slot = episode;
        // Below the capacity, which `new` bounded below `u32::MAX`: never `None`.
        self.hippocampus.append().ok_or(TagError::LedgerFull)
    }

    /// The loader's: an episode from an image, already well formed, appended at the ledger's
    /// end. Refused when the arena is full or the pattern names a unit outside the arena.
    pub(crate) fn load_episode(&mut self, episode: Episode) -> bool {
        if self.episode_room() == 0
            || episode
                .pattern()
                .iter()
                .any(|&unit| unit as usize >= self.shared.units.len())
        {
            return false;
        }
        let index = self.hippocampus.episodes as usize;
        // SAFETY: between ticks; every worker is parked at the barrier.
        let Some(slot) = (unsafe { self.shared.episodes.get_mut(index) }) else {
            return false;
        };
        *slot = episode;
        self.hippocampus.append().is_some()
    }

    /// The loader's: the hippocampal record an image holds. Refused for a record that is not
    /// well formed or whose length is not the episodes loaded before it.
    pub(crate) fn set_hippocampus(&mut self, record: HippocampalAttractorState) -> bool {
        if !record.is_well_formed() || record.episodes != self.hippocampus.episodes {
            return false;
        }
        self.hippocampus = record;
        true
    }

    /// Before a tick's first barrier (ADR-0038): on the ripple's cadence, in slow-wave sleep
    /// the hand walks the ledger to the first episode that is not spent, within
    /// `RIPPLE_SCAN`, counts its replay and returns its index + 1 for worker 0 to deliver in
    /// phase 3; in REM the same walk lowers that episode's tag and returns 0; awake, off the
    /// cadence, or with no episode to find, 0.
    fn ripple(&mut self) -> u32 {
        if !RIPPLE_CADENCE.is_due(self.tick) {
            return 0;
        }
        let stage = self.homeostasis.sleep_stage;
        if stage != STAGE_SWS && stage != STAGE_REM {
            return 0;
        }
        // SAFETY: before the tick's first barrier; every worker is parked at it.
        let episodes = unsafe { self.shared.episodes.as_mut_slice() };
        for _ in 0..RIPPLE_SCAN {
            let Some(index) = self.hippocampus.next_hand() else {
                return 0;
            };
            let Some(episode) = episodes.get_mut(index as usize) else {
                abort("the ledger's hand names an episode outside its arena");
            };
            if episode.is_spent() {
                continue;
            }
            if stage == STAGE_SWS {
                episode.replay();
                self.replays = self.replays.saturating_add(1);
                // Below the ledger's length, which is below `u32::MAX`: index + 1 fits.
                return index.wrapping_add(1);
            }
            episode.depotentiate();
            self.depotentiations = self.depotentiations.saturating_add(1);
            return 0;
        }
        0
    }

    /// After a tick, between ticks (ADR-0036, ADR-0037): the workers' spike counts into the
    /// open bin; on the bin's cadence the bin closes into the window; on the window's cadence
    /// the gain is regulated, the window cleared and the sleep stage stepped. The cadences
    /// are masks on the tick, so a loaded engine continues the window it was written in. The
    /// gain is regulated by the window's count over the rate's target while one is set
    /// (ADR-0180), and by the estimate under the saturation rule while none is; while a read of
    /// the target is under way or refused, or the probe of the step after it is under way, it is
    /// held (ADR-0189, ADR-0202, `read_close`).
    fn tally(&mut self) {
        let spikes = self
            .shared
            .spikes
            .iter()
            .fold(0u32, |sum, s| sum.saturating_add(s.load(Ordering::Relaxed)));
        let descendants = self
            .shared
            .descendants
            .iter()
            .fold(0u32, |sum, d| sum.saturating_add(d.load(Ordering::Relaxed)));
        self.descendants = self.descendants.saturating_add(u64::from(descendants));
        self.homeostasis.count_activity(spikes);
        if BIN_CADENCE.is_due(self.tick) {
            if self.homeostasis.close_bin().is_none() {
                abort("the homeostasis window was full before its cadence regulated it");
            }
            if WINDOW_CADENCE.is_due(self.tick) {
                if let Some(read) = self.target_read {
                    self.read_close(read);
                } else {
                    match self.rate_target {
                        Some(target) => self.homeostasis.regulate_rate(target),
                        None => self
                            .homeostasis
                            .regulate(saturation_ceiling(self.shared.units.len())),
                    };
                }
                let before = self.homeostasis.sleep_stage;
                self.homeostasis.step_sleep();
                // The slow-wave onset (ADR-0056): the arena's garbage is reclaimed once per
                // entry into the stage, the reclamation §8.8's glymphatic row places there.
                if before != STAGE_SWS && self.homeostasis.sleep_stage == STAGE_SWS {
                    self.compact_at_onset();
                }
            }
        }
    }

    /// A window's close under the read (ADR-0189): the gain held, the window's count read and
    /// the window cleared (`HomeostaticDrivePool::hold`), the estimate kept as a reading; then
    /// the read moved on by `read_step` — one window fewer, the count summed where it is one of
    /// the last sixteen — and at its last window the target set, the period and its count, for the
    /// next window's rule, or the read refused. A refused read holds the gain at every window.
    /// Where the read sets the inhibitory rule's target as well (ADR-0194), its last window sets
    /// that rule's period to the same period, published to the workers for the next tick, and a
    /// period that rule refuses refuses the read; either end spends the flag. Where the probe of
    /// the step follows the read (ADR-0202), a read that made its target starts the probe in the
    /// rule's place, and a window under the probe is the probe's (`probe_close`); a read refused
    /// spends the probe's flag too.
    fn read_close(&mut self, read: TargetRead) {
        let spikes = self.homeostasis.hold();
        if let TargetRead::Probing { left, sum } = read {
            self.probe_close(left, sum, spikes);
            return;
        }
        let TargetRead::Reading { left, sum } = read else {
            return;
        };
        let inhibitory = self.istdp_target_read;
        self.target_read = match read_step(left, sum, spikes, self.shared.units.len(), inhibitory) {
            ReadStep::Reading { left, sum } => Some(TargetRead::Reading { left, sum }),
            ReadStep::Target { period, count } => {
                self.rate_target_period_ticks = period;
                self.rate_target = Some(count);
                // `read_step` gave a period the inhibitory rule accepts where the flag is set.
                if inhibitory && !self.set_istdp_target_period(period) {
                    abort("the inhibitory rule refused the period the read's step accepted for it");
                }
                self.istdp_target_read = false;
                // The probe from the next window where its flag is set (ADR-0202); else the rule.
                self.step_probe.then_some(TargetRead::Probing {
                    left: PROBE_WINDOWS,
                    sum: 0,
                })
            }
            ReadStep::Refused => {
                self.istdp_target_read = false;
                self.step_probe = false;
                Some(TargetRead::Refused)
            }
        };
    }

    /// A window's close under the probe of the step (ADR-0200, ADR-0202), the gain already held
    /// and the window's count read by `read_close`: the probe moved on by `probe_step` — one
    /// window fewer, the count added where the window ran at the higher gain and taken where it
    /// ran at the lower — and at its last window the step it made set in the record, for the
    /// rate's rule from the next window, the record's gain the read's all along; or the probe
    /// refused, the gain held from there on and the read's target kept as a reading. Either end
    /// spends the flag.
    fn probe_close(&mut self, left: u8, sum: i64, spikes: u64) {
        // The read set the target before the probe began, so it is there.
        let count = self.rate_target.unwrap_or(0);
        self.target_read = match probe_step(left, sum, spikes, count) {
            ProbeStep::Probing { left, sum } => Some(TargetRead::Probing { left, sum }),
            ProbeStep::Step(step) => {
                self.homeostasis.control_step_q0_16 = step;
                self.step_probe = false;
                None
            }
            ProbeStep::Refused => {
                self.step_probe = false;
                Some(TargetRead::Refused)
            }
        };
    }

    /// Worker threads, including the caller's.
    pub fn workers(&self) -> usize {
        self.shared.barrier.parties()
    }

    /// The worker that serves `unit` (ADR-0100): each worker owns a fixed, contiguous range of
    /// the arena, $\lceil N / W \rceil$ units, the last range shorter and a worker past the
    /// arena none, and alone runs the turns of its range's units. None for a unit outside the
    /// arena.
    pub fn owner(&self, unit: u32) -> Option<usize> {
        self.shared.owner(unit)
    }

    /// Ticks run so far.
    pub fn ticks(&self) -> u64 {
        self.tick
    }

    /// A handle for producers outside the loop.
    pub fn injector(&self) -> Inject {
        Inject(Arc::clone(&self.shared))
    }

    /// Messages drained by every worker so far.
    pub fn delivered(&self) -> u64 {
        self.shared
            .delivered
            .iter()
            .map(|d| d.load(Ordering::Relaxed))
            .sum()
    }

    /// Turns served by every worker so far (ADR-0097): one for each unit a worker served, so the
    /// difference across a tick is the units the schedule named (ADR-0100) — the units that
    /// were not at rest after the tick before, or that a message or an activation woke
    /// (ADR-0023), and that were not asleep (ADR-0224). An asleep unit takes no turn: the steps
    /// it is caught up by are [`caught_up`](Self::caught_up)'s, and the two together are
    /// per-tick service's turns, the active set ADR-0097 measured. A count kept beside
    /// `delivered`; it changes nothing the engine does.
    pub fn turns(&self) -> u64 {
        self.shared
            .turns
            .iter()
            .map(|n| n.load(Ordering::Relaxed))
            .sum()
    }

    /// Steps caught up so far (ADR-0224): the quiet ticks in which per-tick service would have
    /// served a unit that was asleep, brought up to date at its next turn by its worker, or
    /// between ticks by a read or a write of the unit arena. Once a read has brought every
    /// asleep unit up to date, this and [`turns`](Self::turns) together are per-tick service's
    /// turns. A count; it changes nothing the engine does.
    pub fn caught_up(&self) -> u64 {
        self.shared
            .caught_up
            .iter()
            .map(|n| n.load(Ordering::Relaxed))
            .sum::<u64>()
            .saturating_add(self.read_steps.get())
    }

    /// This executor serving every unit that is not at rest at every tick, as the executor did
    /// before ADR-0224: per-tick service, the tests' reference, which no configuration and no
    /// image sets. For an executor just built or loaded; every asleep unit is brought up to date
    /// and put back on the schedule first.
    pub fn per_tick(mut self) -> Self {
        self.wake_asleep();
        self.shared.per_tick.store(true, Ordering::Relaxed);
        self
    }

    /// The unit arena, between ticks. Every asleep unit is brought up to date first, so that
    /// every record is per-tick service's (ADR-0224).
    pub fn units(&self) -> &[DendriticSuperNeuron] {
        self.settle();
        // SAFETY: `&self` excludes `tick` (which takes `&mut self`); between ticks every worker
        // thread is spinning at the barrier and holds no reference into an arena. `settle` has
        // written the arena, if at all, before this reference is made.
        unsafe { self.shared.units.as_slice() }
    }

    /// The unit arena, exclusively, between ticks: for wiring a network before it runs. Every
    /// asleep unit is brought up to date and put back on the schedule first, since a write may
    /// take it out of the condition it sleeps under (ADR-0224).
    pub fn units_mut(&mut self) -> &mut [DendriticSuperNeuron] {
        self.wake_asleep();
        // SAFETY: as in `units`, with `&mut self` excluding every other reference this
        // executor hands out.
        unsafe { self.shared.units.as_mut_slice() }
    }

    /// Between ticks, under `&self`: if a tick has run since the asleep units were last brought
    /// up to date, every one of them is, from the tick it was left at to the next tick's, and
    /// sleeps on from there (ADR-0224). One at rest by then leaves the asleep map, its gate
    /// recorded idle unless a message or an activation has marked it since, as per-tick service
    /// left it.
    ///
    /// This is the one write of the unit arena under `&self`. ADR-0023's invariant, that a `&mut`
    /// to a record never overlaps another reference to it, holds for it (ADR-0225): no worker
    /// references the arena between ticks; no other thread holds `&self`, the executor not being
    /// `Sync`; and no reference this executor handed out is alive, since only a tick sets
    /// `unsettled`, a tick takes `&mut self`, which ends every such reference, and every reference
    /// made after it is made by `units` after this has cleared the flag.
    fn settle(&self) {
        if self.unsettled.replace(false) {
            self.bring_up_to_date(false);
        }
    }

    /// Between ticks, under `&mut self`: every asleep unit brought up to date and put back on the
    /// schedule (ADR-0224), so that a write of the arena finds none asleep.
    fn wake_asleep(&mut self) {
        self.unsettled.set(false);
        self.bring_up_to_date(true);
    }

    /// Every asleep unit brought from the tick it was left at to the next tick's, as per-tick
    /// service would have left it with no input (`quiet_catch_up`), and its steps counted. One
    /// at rest by then leaves the asleep map, its gate recorded idle unless a message or an
    /// activation has marked it; one not at rest is put back on the schedule under `wake`, and
    /// otherwise sleeps on from the next tick. Called by `settle` and `wake_asleep` alone.
    fn bring_up_to_date(&self, wake: bool) {
        let next = self.tick;
        let mut steps = 0u64;
        for (w, word) in self.shared.asleep.iter().enumerate() {
            let asleep = word.load(Ordering::Relaxed);
            if asleep == 0 {
                continue;
            }
            let Some(scheduled) = self.shared.scheduled.get(w) else {
                abort("the asleep map runs past the schedule");
            };
            let mut left = asleep;
            let mut keep = 0u64;
            for _ in 0..asleep.count_ones() {
                let bit = left.trailing_zeros();
                left &= left.wrapping_sub(1); // the lowest set bit, brought up to date now
                let mask = 1u64 << bit;
                let unit = self.shared.unit_at(w, bit);
                // SAFETY: between ticks, with `settle`'s argument or under `&mut self`
                // (`wake_asleep`): no other reference to any record of the arena is alive, so
                // this is the only one to `unit`'s, dropped before the next.
                let Some(u) = (unsafe { self.shared.units.get_mut(unit as usize) }) else {
                    abort("an asleep unit is outside the arena");
                };
                let Some(since) = self.shared.since.get(unit as usize) else {
                    abort("an asleep unit is outside the arena");
                };
                let gap = u32::try_from(next.wrapping_sub(since.load(Ordering::Relaxed)));
                // Past `u32::MAX` ticks every unit is at rest: `quiet_catch_up`'s rest, at its
                // longest gap.
                steps = steps.saturating_add(u64::from(u.quiet_catch_up(gap.unwrap_or(u32::MAX))));
                if membrane_at_rest(u) {
                    if scheduled.load(Ordering::Relaxed) & mask == 0 {
                        u.set_gate(GateState::Idle);
                    }
                } else if wake {
                    scheduled.fetch_or(mask, Ordering::Relaxed);
                } else {
                    since.store(next, Ordering::Relaxed);
                    // A bit not yet in `keep`: the sum is the union.
                    keep = keep.wrapping_add(mask);
                }
            }
            word.store(keep, Ordering::Relaxed);
        }
        self.read_steps
            .set(self.read_steps.get().saturating_add(steps));
    }

    /// The unit and synapse arenas together, exclusively, between ticks: for a rule that
    /// writes both, such as the synthesis of a network from a prior (ADR-0044). Every asleep
    /// unit as in `units_mut`.
    pub fn arenas_mut(&mut self) -> (&mut [DendriticSuperNeuron], &mut [SynapseBlock]) {
        self.wake_asleep();
        // SAFETY: as in `units_mut`; the two arenas are distinct allocations, so the two
        // exclusive slices do not overlap.
        unsafe {
            (
                self.shared.units.as_mut_slice(),
                self.shared.blocks.as_mut_slice(),
            )
        }
    }

    /// The synapse arena, between ticks.
    pub fn blocks(&self) -> &[SynapseBlock] {
        // SAFETY: as in `units`.
        unsafe { self.shared.blocks.as_slice() }
    }

    /// The synapse arena, exclusively, between ticks.
    pub fn blocks_mut(&mut self) -> &mut [SynapseBlock] {
        // SAFETY: as in `units_mut`.
        unsafe { self.shared.blocks.as_mut_slice() }
    }

    /// The Tier-2 delta arena, between ticks.
    pub fn deltas(&self) -> &[PlasticDelta] {
        // SAFETY: as in `units`.
        unsafe { self.shared.deltas.as_slice() }
    }

    /// The Tier-2 delta arena, exclusively, between ticks.
    pub fn deltas_mut(&mut self) -> &mut [PlasticDelta] {
        // SAFETY: as in `units_mut`.
        unsafe { self.shared.deltas.as_mut_slice() }
    }

    /// True between ticks when no unit holds a message, no token is in flight and the injector
    /// ring holds nothing not yet drained: the state an image can be written from (whitepaper
    /// §8.7; a pair still in the ring is not in any record, so an image written over it would
    /// lose it, ADR-0028).
    pub fn is_quiescent(&self) -> bool {
        self.units().iter().all(|u| u.mailbox_is_empty())
            && self.tokens_in_flight() == 0
            && self.shared.injector.is_empty()
    }

    /// Tokens scheduled in the wheels and not yet delivered.
    pub fn tokens_in_flight(&self) -> i64 {
        self.shared
            .in_flight
            .iter()
            .map(|n| n.load(Ordering::Relaxed))
            .sum()
    }

    /// Attaches a fresh write-ahead log at `path`, which the clock sweep evicts into and
    /// re-hydration reads from (ADR-0024). Between ticks; allocates the log's index once.
    pub fn attach_log(&mut self, path: &Path) -> Result<(), ImageError> {
        self.log = Some(WriteAheadLog::create(path, self.shared.units.len())?);
        Ok(())
    }

    /// The attached log.
    pub fn log(&self) -> Option<&WriteAheadLog> {
        self.log.as_ref()
    }

    /// True when `unit`'s record is in the log and its slot holds only its id, its last spike
    /// stamp, its gate and its mailbox.
    pub fn is_evicted(&self, unit: u32) -> bool {
        self.shared
            .evicted
            .get(unit as usize)
            .is_some_and(|e| e.load(Ordering::Relaxed))
    }

    /// Units evicted so far.
    pub fn evictions(&self) -> u64 {
        self.evictions
    }

    /// Units re-hydrated so far.
    pub fn rehydrations(&self) -> u64 {
        self.rehydrations
    }

    /// The clock sweep of axiom A5 (§8.6), between ticks: the hand walks up to one full turn of
    /// the unit arena and evicts at most `budget` units that are idle, unscheduled, at rest and
    /// quiet for at least `quiet_ticks` since their last spike. An evicted unit's 64 bytes go to
    /// the log and its slot keeps only its id and its last spike stamp; a message to it
    /// re-hydrates it after the tick
    /// that delivered the message. Returns the number evicted.
    pub fn sweep(&mut self, quiet_ticks: u32, budget: usize) -> Result<usize, ImageError> {
        // Rest is read as per-tick service leaves it (ADR-0224).
        self.settle();
        let Some(log) = self.log.as_mut() else {
            return Err(ImageError::NoLog);
        };
        let now = self.tick as u32;
        let len = self.shared.units.len();
        // SAFETY: `&mut self` between ticks; every worker is parked at the barrier.
        let units = unsafe { self.shared.units.as_mut_slice() };
        let mut evicted = 0usize;
        for _ in 0..len {
            if evicted >= budget {
                break;
            }
            let i = self.hand;
            self.hand = next_in_ring(i, len);
            if self.shared.evicted[i].load(Ordering::Relaxed) {
                continue;
            }
            let unit = &mut units[i];
            if !unit.is_image_ready() || !at_rest(unit) || unit.ticks_since_spike(now) < quiet_ticks
            {
                continue;
            }
            // A unit that fired since the previous reward is the critic's to read and move at
            // the next one, in its slot: it stays (ADR-0131). Unset, there is no count.
            if self.features.get(i).is_some_and(|&count| count != 0) {
                continue;
            }
            log.append(i as u32, &unit.encode())?;
            // The slot keeps what other units read of it: its id and its last spike stamp,
            // which STDP pairs against (ADR-0022).
            let mut empty = DendriticSuperNeuron::new(unit.id);
            empty.last_soma_spike_tick = unit.last_soma_spike_tick;
            *unit = empty;
            self.shared.evicted[i].store(true, Ordering::Relaxed);
            evicted = evicted.saturating_add(1);
        }
        self.evictions = self.evictions.saturating_add(evicted as u64);
        Ok(evicted)
    }

    /// The clock sweep under the live policy (ADR-0031): [`Executor::sweep`] with the policy's
    /// quiet bound and budget.
    pub fn sweep_by_policy(&mut self) -> Result<usize, ImageError> {
        let policy = self.policy;
        self.sweep(policy.sweep_quiet_ticks, policy.sweep_budget as usize)
    }

    // ------------------------------------------------------------ amendments (ADR-0031)

    /// The live policy.
    pub fn policy(&self) -> Policy {
        self.policy
    }

    /// The amendment arena: every proposal so far, in order, whatever became of it.
    pub fn amendments(&self) -> &[PolicyAmendment] {
        &self.amendments
    }

    /// Slots left for proposals.
    pub fn amendment_room(&self) -> usize {
        // The arena never holds more than its capacity (`propose`, `load_amendment`).
        self.amendment_capacity
            .saturating_sub(self.amendments.len())
    }

    /// Proposes, at this tick, to change `parameter` from its live value to `proposed_value`,
    /// judged by `objective` with a gain of at least `min_gain`; the bounds gate runs in
    /// `PolicyAmendment::propose`. Returns the arena index; every proposal takes a slot, a
    /// rejected one as the record of its rejection. Refused when the arena is full.
    pub fn propose(
        &mut self,
        parameter: u16,
        proposed_value: i32,
        objective: u8,
        min_gain: u16,
    ) -> Result<usize, AmendError> {
        if self.amendments.len() >= self.amendment_capacity {
            return Err(AmendError::ArenaFull);
        }
        let index = self.amendments.len();
        // The id is the index plus one (zero is no record, ADR-0031); an index the id width
        // cannot name above is a proposal the arena cannot hold.
        let id = (index as u32).checked_add(1).ok_or(AmendError::ArenaFull)?;
        let current = self.policy.value(parameter).unwrap_or(0);
        self.amendments.push(PolicyAmendment::propose(
            id,
            self.tick as u32,
            parameter,
            current,
            proposed_value,
            objective,
            min_gain,
        ));
        Ok(index)
    }

    /// The veto gate's verdict on the proposal at `index`: the gate must have been evaluated on
    /// this amendment (`proposal_action_id` is its id), and the amendment is admitted only by
    /// a permitting verdict; a gate not yet evaluated is closed (ADR-0028). Returns whether
    /// the amendment is now admitted; refused for an amendment that is not proposed.
    pub fn admit(
        &mut self,
        index: usize,
        gate: &EthicalEvaluationGate,
    ) -> Result<bool, AmendError> {
        let a = self
            .amendments
            .get_mut(index)
            .ok_or(AmendError::NoSuchAmendment)?;
        if gate.proposal_action_id != a.amendment_id {
            return Err(AmendError::WrongProposal);
        }
        if a.status != AMENDMENT_PROPOSED {
            return Err(AmendError::NotProposed);
        }
        Ok(a.admit(gate.is_permitted()))
    }

    /// The trial's way in: records a trial's result on the amendment at `index`
    /// (`PolicyAmendment::record_trial`). Crate-private, so that a verdict enters the arena
    /// only through a trial the runtime ran ([`crate::trial::run`]).
    pub(crate) fn record_trial(
        &mut self,
        index: usize,
        ticks: u32,
        baseline_hash: u64,
        candidate_hash: u64,
        baseline_cost: u32,
        candidate_cost: u32,
    ) -> bool {
        self.amendments.get_mut(index).is_some_and(|a| {
            a.record_trial(
                ticks,
                baseline_hash,
                candidate_hash,
                baseline_cost,
                candidate_cost,
            )
        })
    }

    /// Commits the trialled amendment at `index` into the live policy, between ticks. Refused
    /// when the amendment is not committable; when a later proposal for the same parameter was
    /// committed first (the arena is the log the loader replays in index order, so commits to
    /// one parameter keep that order); or when the live value is no longer the one the trial
    /// started from (another commit came between: the trial did not test this change on top
    /// of that one).
    pub fn commit(&mut self, index: usize) -> Result<(), AmendError> {
        let tick = self.tick as u32;
        let a = *self
            .amendments
            .get(index)
            .ok_or(AmendError::NoSuchAmendment)?;
        if !a.may_commit() {
            return Err(AmendError::NotCommittable);
        }
        if self.amendments[index.saturating_add(1)..]
            .iter()
            .any(|later| later.is_committed() && later.parameter == a.parameter)
        {
            return Err(AmendError::Superseded);
        }
        if self.policy.value(a.parameter) != Some(a.current_value) {
            return Err(AmendError::Stale);
        }
        let a = &mut self.amendments[index];
        if !self.policy.set(a.parameter, a.proposed_value) {
            return Err(AmendError::NotCommittable);
        }
        a.commit(tick);
        Ok(())
    }

    /// A trial fork's policy: the runtime's own way to put the candidate value in place, not a
    /// public way around [`Executor::commit`].
    pub(crate) fn set_policy_value(&mut self, parameter: u16, value: i32) -> bool {
        self.policy.set(parameter, value)
    }

    /// The loader's way in: an amendment record from an image, already well-formed, appended
    /// in the image's order; a committed one is replayed into the policy, and is refused when
    /// its starting value is not the policy's at that point in the replay.
    pub(crate) fn load_amendment(&mut self, a: PolicyAmendment) -> bool {
        if self.amendments.len() >= self.amendment_capacity {
            return false;
        }
        if a.is_committed() {
            if self.policy.value(a.parameter) != Some(a.current_value) {
                return false;
            }
            if !self.policy.set(a.parameter, a.proposed_value) {
                return false;
            }
        }
        self.amendments.push(a);
        true
    }

    /// After a tick: every evicted unit that received a message gets its plain fields back from
    /// the log, keeping the gate and the mailbox the delivery left in its slot.
    fn rehydrate_pending(&mut self) {
        if self.shared.rehydration_pending.swap(0, Ordering::AcqRel) == 0 {
            return;
        }
        let Some(log) = self.log.as_ref() else {
            abort("a message reached an evicted unit with no log attached");
        };
        // SAFETY: between ticks; every worker is parked at the barrier.
        let units = unsafe { self.shared.units.as_mut_slice() };
        for (i, flag) in self.shared.needs_rehydration.iter().enumerate() {
            if !flag.swap(false, Ordering::AcqRel) {
                continue;
            }
            let Ok(record) = log.read(i as u32) else {
                abort("the log does not hold the record of an evicted unit");
            };
            let cold = DendriticSuperNeuron::decode(&record);
            units[i].restore_plain_fields(&cold);
            self.shared.evicted[i].store(false, Ordering::Relaxed);
            self.rehydrations = self.rehydrations.saturating_add(1);
        }
    }

    /// Between ticks: marks `units` for a turn on the next tick without a message (the loader
    /// wakes every unit that is not at rest).
    pub(crate) fn wake_now(&mut self, units: &[u32]) {
        for &unit in units {
            Worker::<CAP>::wake(&self.shared, unit);
        }
    }

    /// One fine tick: the three phases on every worker, this thread running as worker 0.
    pub fn tick(&mut self) {
        let now = self.tick as u32;
        self.shared.now.store(now, Ordering::Relaxed);
        self.shared.tick.store(self.tick, Ordering::Relaxed);
        // The modulations this tick's fan-out consolidates with, from the signal as it
        // stands (the addressed units' with the signal, every other's at rest; ADR-0032,
        // ADR-0068; the inhibitory blocks' under their own baseline while it is set,
        // ADR-0086; the addressed one signed while the signed gate is set, ADR-0094); then the
        // signal decays by one tick. All before the barrier that starts the tick, so every
        // worker reads the same values.
        let modulations = Modulations::of(
            &self.modulator,
            self.modulation_baseline_q16,
            self.inhibitory_baseline_q16,
            self.signed_gate,
        );
        self.shared
            .modulation
            .store(modulations.addressed, Ordering::Relaxed);
        self.shared
            .modulation_at_rest
            .store(modulations.at_rest, Ordering::Relaxed);
        self.shared
            .modulation_inhibitory
            .store(inhibitory_word(modulations.inhibitory), Ordering::Relaxed);
        self.modulator.decay_dopamine(DOPAMINE_TAU_SHIFT);
        // The gain this tick's turns scale by (ADR-0036), likewise before the barrier: the
        // record's, or under the probe of the step the probe's of it (ADR-0202).
        self.shared
            .gain
            .store(self.gain_given_q16(), Ordering::Relaxed);
        // The episode this tick's phase 3 replays, if a ripple is due in slow-wave sleep
        // (ADR-0038), likewise.
        let replay = self.ripple();
        self.shared.replay.store(replay, Ordering::Relaxed);
        self.shared.barrier.wait();
        self.worker0.phase_turns(&self.shared, now);
        self.shared.barrier.wait();
        self.worker0.phase_fan_out(&self.shared, now);
        self.shared.barrier.wait();
        self.worker0.phase_deliveries(&self.shared);
        self.shared.barrier.wait();
        // The zero sum (ADR-0228), nothing while unset.
        self.hold_zero_sum();
        self.merge_spikes(now);
        // A clock wraps by name (§8.1): the dynamics already see it as `tick as u32`.
        self.tick = self.tick.wrapping_add(1);
        // The tick may have left units asleep: a read brings them up to date first (ADR-0224).
        self.unsettled.set(true);
        self.rehydrate_pending();
        self.tally();
        self.search_on_cadence();
    }

    /// `ticks` fine ticks.
    pub fn run(&mut self, ticks: u64) {
        for _ in 0..ticks {
            self.tick();
        }
    }

    /// Stops the workers, joins them, and returns what each kept, worker 0 first.
    pub fn shutdown(mut self) -> Vec<WorkerReport> {
        let threads = std::mem::take(&mut self.threads);
        self.stop_workers();
        let mut reports = vec![self.worker0.report()];
        for handle in threads {
            let mut worker = handle.join().expect("a worker thread ended by panicking");
            reports.push(worker.report());
        }
        reports
    }

    fn stop_workers(&self) {
        if !self.shared.stop.swap(true, Ordering::AcqRel) {
            self.shared.barrier.wait();
        }
    }
}

impl<const CAP: usize> Drop for Executor<CAP> {
    fn drop(&mut self) {
        self.stop_workers();
        for handle in self.threads.drain(..) {
            let _ = handle.join();
        }
    }
}

/// Builds the wheels on a thread whose stack holds several: a production wheel is 4 MB,
/// `Box::new` builds it on the stack and copies it on its way into the box, and the debug
/// profile materialises more than two copies (a Linux build overflowed at twice the wheel,
/// finding F-34, the first time a test constructed the production geometry), so the
/// reservation is eight wheels and a megabyte, virtual memory committed only as touched.
fn build_wheels<const CAP: usize>(count: usize) -> Vec<Box<FlatTimingWheel<CAP>>> {
    let bytes = std::mem::size_of::<FlatTimingWheel<CAP>>();
    thread::Builder::new()
        .stack_size(bytes.saturating_mul(8).saturating_add(1 << 20))
        .spawn(move || {
            (0..count)
                .map(|_| Box::new(FlatTimingWheel::<CAP>::new()))
                .collect()
        })
        .expect("spawn the wheel builder")
        .join()
        .expect("the wheel builder ended by panicking")
}

/// A unit at rest has nothing to integrate: every potential zero, the slow potential among them
/// (ADR-0123), no window running, the threshold at or below its base. It leaves the active set
/// until a message wakes it; the loader wakes and the sweep keeps every unit that is not.
pub(crate) fn at_rest(u: &DendriticSuperNeuron) -> bool {
    membrane_at_rest(u) && u.v_slow == 0
}

/// `at_rest` but for the slow potential, which is zero in every unit not marked for the slow
/// current: the turn reads the slow potential for a marked unit only, in that unit's own arm.
fn membrane_at_rest(u: &DendriticSuperNeuron) -> bool {
    u.v_soma == 0
        && u.v_basal == 0
        && u.v_apical == 0
        && u.refractory_ticks == 0
        && u.bac_plateau_ticks == 0
        && u.v_thresh <= THRESHOLD_BASE
}

/// Where a unit's turn leaves it (ADR-0100, ADR-0224).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Left {
    /// At rest: off the schedule until a message or an activation marks it.
    AtRest,
    /// Not at rest: on the schedule for the next tick.
    Awake,
    /// Not at rest and unable to fire with no input (`quiet_cannot_fire`): off the schedule and
    /// asleep until a message or an activation marks it, or the arena is read or written.
    Asleep,
}

/// A marked unit's slow input (ADR-0122): the positive efficacies of the synapses' messages in
/// the batch that land in the basal compartment, the excitatory synapses' releases, summed and
/// saturating. The drive's, the injector's and the replay's messages, an inhibitory synapse's
/// and an apical one's give nothing.
fn slow_input(batch: &[u32]) -> i32 {
    batch
        .iter()
        .filter(|&&m| message_is_synaptic(m) && !message_is_apical(m))
        .map(|&m| message_efficacy_q16(m).max(0))
        .fold(0, i32::saturating_add)
}

impl<const CAP: usize> Worker<CAP> {
    /// A spawned worker's life: the phases of every tick until told to stop.
    fn run(&mut self, shared: &Shared) {
        loop {
            shared.barrier.wait();
            if shared.stop.load(Ordering::Acquire) {
                return;
            }
            let now = shared.now.load(Ordering::Relaxed);
            self.phase_turns(shared, now);
            shared.barrier.wait();
            self.phase_fan_out(shared, now);
            shared.barrier.wait();
            self.phase_deliveries(shared);
            shared.barrier.wait();
        }
    }

    fn report(&mut self) -> WorkerReport {
        WorkerReport {
            delivered: std::mem::take(&mut self.trace),
            spikes: std::mem::take(&mut self.spike_trace),
            dropped: self.trace_dropped,
            delivered_count: self.delivered,
            turns: self.turns,
        }
    }

    // ---------------------------------------------------------------- phase 1: turns

    /// The sweep (ADR-0100): this worker's region of the schedule, word by word, and in each
    /// word the set bits from the lowest, so the units of its range in increasing index. No
    /// other worker writes the region in this phase, since no push happens in it, so a word is
    /// loaded and stored back with the bits of the units that stay awake. The asleep map's word
    /// is the owner's too in this phase (ADR-0224): the units the word serves leave it, and those
    /// that fall asleep join it.
    fn phase_turns(&mut self, shared: &Shared, now: u32) {
        let gain = shared.gain.load(Ordering::Relaxed);
        // Per-tick service, the tests' reference, puts no unit to sleep (ADR-0224).
        let sleeps = !shared.per_tick.load(Ordering::Relaxed);
        self.descended = 0;
        for w in 0..self.words {
            let at = self.region.wrapping_add(w);
            let Some(word) = shared.scheduled.get(at) else {
                abort("a worker's region runs past the schedule");
            };
            let mut due = word.load(Ordering::Relaxed);
            if due == 0 {
                continue;
            }
            let Some(asleep_word) = shared.asleep.get(at) else {
                abort("a worker's region runs past the asleep map");
            };
            let asleep = asleep_word.load(Ordering::Relaxed);
            let mut sleeping = asleep & !due;
            // The range's unit of the word's bit 0: within the range, below the unit count.
            let base = self.first.wrapping_add((w as u32) << 6);
            let mut awake = 0u64;
            for _ in 0..due.count_ones() {
                let bit = due.trailing_zeros();
                due &= due.wrapping_sub(1); // the lowest set bit, served now, cleared
                let mask = 1u64 << bit;
                let unit = base.wrapping_add(bit);
                // A bit not yet in `awake` or `sleeping`: the sum is the union.
                match self.turn(shared, unit, now, gain, asleep & mask != 0, sleeps) {
                    Left::Awake => awake = awake.wrapping_add(mask),
                    Left::Asleep => sleeping = sleeping.wrapping_add(mask),
                    Left::AtRest => {}
                }
                self.turns = self.turns.saturating_add(1);
            }
            word.store(awake, Ordering::Relaxed);
            asleep_word.store(sleeping, Ordering::Relaxed);
        }
        // The units this worker fired this tick, for the population tally (ADR-0036), and
        // how many of them were descendants (ADR-0054); counts below the unit count, which
        // `Executor::new` bounded below `u32::MAX`.
        shared.spikes[self.id].store(self.spiked.len() as u32, Ordering::Relaxed);
        shared.descendants[self.id].store(self.descended, Ordering::Relaxed);
    }

    /// One unit's turn, and where it leaves the unit. An asleep unit is first brought from the
    /// tick it was left at to this one, as per-tick service would have served it with no input,
    /// and the turn is then the turn it was (ADR-0224).
    fn turn(
        &mut self,
        shared: &Shared,
        unit: u32,
        now: u32,
        gain: u32,
        asleep: bool,
        sleeps: bool,
    ) -> Left {
        // SAFETY (phase 1): `unit` is a set bit of this worker's region of the schedule, and so
        // in this worker's range, which no other worker's range overlaps: the ranges partition
        // the arena and are fixed in `Executor::new` (ADR-0100). In phase 1 a worker references
        // only its own range's units, one at a time, and no push happens; phases 2 and 3 have
        // ended at the barrier.
        let Some(u) = (unsafe { shared.units.get_mut(unit as usize) }) else {
            abort("a scheduled unit index is outside the arena");
        };
        if asleep {
            let Some(since) = shared.since.get(unit as usize) else {
                abort("an asleep unit is outside the arena");
            };
            let gap = shared
                .tick
                .load(Ordering::Relaxed)
                .wrapping_sub(since.load(Ordering::Relaxed));
            // Past `u32::MAX` ticks every unit is at rest: `quiet_catch_up`'s rest, at its
            // longest gap.
            let steps = u.quiet_catch_up(u32::try_from(gap).unwrap_or(u32::MAX));
            self.caught_up = self.caught_up.saturating_add(u64::from(steps));
        }
        self.batch.clear();
        // A unit with no mail takes no swap: no push happens in this phase, so an empty head
        // stays empty through the turn (ADR-0100).
        if !u.mailbox_is_empty() {
            for (node, payload) in u.mailbox_drain(shared.pools.nodes()) {
                push_bounded(
                    &mut self.batch,
                    payload,
                    "a mailbox held more nodes than exist",
                );
                shared.pools.free(node);
                self.delivered = self.delivered.saturating_add(1);
                if self.trace.len() < self.trace.capacity() {
                    self.trace.push(payload);
                } else if self.trace.capacity() > 0 {
                    self.trace_dropped = self.trace_dropped.saturating_add(1);
                }
            }
        }
        // §8.3: the batch is applied in message order, never in arrival order.
        self.batch.sort_unstable();
        let (mut basal, mut apical) = (0i32, 0i32);
        let mut synaptic = false;
        for &message in &self.batch {
            let efficacy = message_efficacy_q16(message);
            if message_is_apical(message) {
                apical = apical.saturating_add(efficacy);
            } else {
                basal = basal.saturating_add(efficacy);
            }
            synaptic |= message_is_synaptic(message);
        }
        // A synapse's message reached the unit this tick (ADR-0054): the stamp the
        // descendant rule reads at the unit's next spikes.
        if synaptic {
            u.note_synaptic_input(now);
        }
        // The tick's synaptic gain on every input of the unit (ADR-0036): the whitepaper's
        // rescaling of every weight, as one factor per turn.
        let (basal, apical) = (scaled(basal, gain), scaled(apical, gain));
        let previous_spike = u.last_soma_spike_tick;
        // A marked unit under the image's slow current, its slow input scaled by the gain as its
        // basal input is, and every other under `integrate` (ADR-0123); with no slow current every
        // unit takes `integrate`, marked or not. A marked unit's slow potential keeps it awake.
        let (fired, slow_awake, slow) = match self.slow_current {
            Some(current) if u.flags & FLAG_SLOW != 0 => {
                let slow = scaled(slow_input(&self.batch), gain);
                let fired = u.integrate_slow(basal, apical, slow, now, current);
                (fired, u.v_slow != 0, true)
            }
            _ => (u.integrate(basal, apical, now), false, false),
        };
        if fired {
            let elapsed = if previous_spike == NO_SPIKE_ON_RECORD {
                u32::MAX
            } else {
                now.wrapping_sub(previous_spike)
            };
            // A marked unit under the image's class, every other under ADR-0019's (ADR-0114);
            // with no class every unit takes `step_stp`, marked or not.
            let (release_u, release_r) = match self.stp_class {
                Some(class) if u.flags & FLAG_FACILITATING != 0 => u.step_stp_class(elapsed, class),
                _ => u.step_stp(elapsed),
            };
            if u.is_descendant(now) {
                // Below the unit count, as the spikes are.
                self.descended = self.descended.wrapping_add(1);
            }
            push_bounded(
                &mut self.spiked,
                (unit, release_u, release_r),
                "more spikes than units in one tick",
            );
            if self.spike_trace.len() < self.spike_trace.capacity() {
                self.spike_trace.push((unit, now));
            } else if self.spike_trace.capacity() > 0 {
                self.trace_dropped = self.trace_dropped.saturating_add(1);
            }
            // The tick's spikes for the coordinator's train (ADR-0050): one slot per unit,
            // taken in any order and sorted after the tick.
            if !shared.fired.is_empty() {
                let at = shared.fired_len.fetch_add(1, Ordering::Relaxed);
                let Some(slot) = shared.fired.get(at) else {
                    abort("more spikes than units in one tick");
                };
                slot.store(unit, Ordering::Relaxed);
            }
        }
        // The schedule's record (ADR-0100): the unit stays on it while it is not at rest and
        // leaves it at rest, until a message or an activation marks it again.
        let awake = slow_awake || !membrane_at_rest(u);
        u.set_gate(if awake {
            GateState::Scheduled
        } else {
            GateState::Idle
        });
        if !awake {
            return Left::AtRest;
        }
        // A unit not at rest that cannot fire with no input leaves the schedule, asleep from the
        // next tick, its gate recorded scheduled as above (ADR-0224). A unit under the slow
        // current never sleeps.
        if sleeps && !slow && u.quiet_cannot_fire() {
            let Some(since) = shared.since.get(unit as usize) else {
                abort("a unit put to sleep is outside the arena");
            };
            since.store(
                shared.tick.load(Ordering::Relaxed).wrapping_add(1),
                Ordering::Relaxed,
            );
            return Left::Asleep;
        }
        Left::Awake
    }

    // ---------------------------------------------------------------- phase 2: fan-out

    fn phase_fan_out(&mut self, shared: &Shared, now: u32) {
        let modulations = Modulations {
            addressed: shared.modulation.load(Ordering::Relaxed),
            at_rest: shared.modulation_at_rest.load(Ordering::Relaxed),
            inhibitory: inhibitory_of_word(shared.modulation_inhibitory.load(Ordering::Relaxed)),
        };
        // The inhibitory rule's depression per spike at the engine's target period
        // (ADR-0053), published by the coordinator; within `i16`, as `istdp_alpha_q1_15`
        // bounds it.
        let istdp_alpha = shared.istdp_alpha.load(Ordering::Relaxed) as i16;
        // The excitatory depression's exponent (ADR-0228), the rule's unless the configuration
        // says otherwise.
        let exponent = shared.depression_exponent;
        for k in 0..self.spiked.len() {
            let (unit, release_u, release_r) = self.spiked[k];
            // SAFETY (phase 2): every turn ended at the barrier, so no `&mut` to any unit
            // exists; units are only read in this phase.
            let Some(pre) = (unsafe { shared.units.get(unit as usize) }) else {
                abort("a spiked unit index is outside the arena");
            };
            // The block's polarity is its presynaptic unit's (ADR-0049): every rule below
            // moves a weight within that half of the width.
            let polarity = Polarity::of_flags(pre.flags);
            // Whether the chain's synapses are from an addressed source (ADR-0068), read once
            // for the whole chain; the target side is read per slot below.
            let from_source = shared.is_source(unit);
            // The unit's group as a held source of the zero sum (ADR-0228), its index plus one,
            // zero while unset and for every other unit.
            let zero_sum_group = shared.zero_sum_group(unit);
            let mut next = pre.synapse_slab_idx;
            let mut remaining = shared.blocks.len();
            while next != CHAIN_END && remaining > 0 {
                // Index + 1 encoded: `next` is not `CHAIN_END` (0), and `remaining` is not 0.
                let block_idx = next.wrapping_sub(1) as usize;
                remaining = remaining.wrapping_sub(1);
                // SAFETY (phase 2): block `block_idx` is in the chain of `unit`, whose spike
                // this worker alone runs (a unit fires at most once per tick and lands in one
                // worker's list), so this is the only reference to the block: no other worker
                // walks this chain, and blocks are not read until phase 3.
                let Some(block) = (unsafe { shared.blocks.get_mut(block_idx) }) else {
                    break;
                };
                let posts: [u32; SYNAPSES_PER_BLOCK] = core::array::from_fn(|slot| {
                    block
                        .target(slot)
                        // SAFETY (phase 2): as above, units are only read in this phase.
                        .and_then(|t| unsafe { shared.units.get(t as usize) })
                        .map_or(NO_SPIKE_ON_RECORD, |t| t.last_soma_spike_tick)
                });
                // The modulation per slot (ADR-0068): the addressed one for a synapse from an
                // addressed source onto an addressed target, the one at rest for every other;
                // for every slot of an inhibitory block the inhibitory one while the
                // inhibitory baseline is set (ADR-0086); the addressed one below zero only for
                // an excitatory slot under the signed gate (ADR-0094); an empty slot's is never
                // read.
                let per_slot: [i32; SYNAPSES_PER_BLOCK] = core::array::from_fn(|slot| {
                    modulations.for_synapse(
                        from_source && block.target(slot).is_some_and(|t| shared.is_target(t)),
                        polarity,
                    )
                });
                block.step_stdp_all_shaped(now, posts, polarity, istdp_alpha, exponent);
                let before = block.weights_q1_15;
                consolidate_each(block, &per_slot, polarity);
                // The unit's group touched where this consolidation moved one of its synapses
                // onto a held target, for the coordinator to hold after the tick (ADR-0228).
                if zero_sum_group != 0 && shared.moved_onto_a_target(block, &before) {
                    // The group's index, below `ZERO_SUM_GROUPS_MAX`.
                    let bit = 1u64.wrapping_shl(u32::from(zero_sum_group.wrapping_sub(1)));
                    shared.zero_sum_touched.fetch_or(bit, Ordering::Relaxed);
                }
                let released = block.release_all(release_u, release_r);
                for (slot, &efficacy) in released.iter().enumerate() {
                    let Some(target) = block.target(slot) else {
                        continue;
                    };
                    let delay = block.delays_ticks[slot];
                    if delay == 0 {
                        let message = synaptic_message(efficacy, block.is_apical(slot));
                        self.deliver(shared, target, message);
                    } else {
                        let Some(token) = synapse_token(block_idx as u32, slot as u8) else {
                            abort("a block index exceeds the token width (finding F-23)");
                        };
                        if self.wheel.schedule(delay as u32, token).is_err() {
                            abort("a delay the loader should have rejected, or a full wheel slot");
                        }
                        self.in_flight = self.in_flight.saturating_add(1);
                    }
                }
                next = block.next_encoded();
            }
        }
        self.spiked.clear();
    }

    // ---------------------------------------------------------------- phase 3: deliveries

    fn phase_deliveries(&mut self, shared: &Shared) {
        self.due.clear();
        self.due.extend_from_slice(self.wheel.advance());
        self.in_flight = self.in_flight.saturating_sub(self.due.len() as i64);
        for i in 0..self.due.len() {
            let token = self.due[i];
            let slot = token_slot(token) as usize;
            // SAFETY (phase 3): blocks are written only in phase 2, which ended at the
            // barrier; every reference in this phase is shared.
            let Some(block) = (unsafe { shared.blocks.get(token_block(token) as usize) }) else {
                abort("a scheduled token names a block outside the arena");
            };
            let Some(target) = block.target(slot) else {
                continue;
            };
            let message = synaptic_message(block.last_release_q16[slot], block.is_apical(slot));
            self.deliver(shared, target, message);
        }
        if self.id == 0 {
            // At most one ring's worth per tick, so that the nodes a tick can take from pool 0
            // are bounded by the ring's capacity however fast the producers refill it.
            let mut budget = shared.injector.capacity();
            while budget > 0 {
                let Some((unit, payload)) = shared.injector.pop() else {
                    break;
                };
                budget = budget.wrapping_sub(1); // not 0: the loop's guard
                if payload == ACTIVATE {
                    Self::wake(shared, unit);
                } else {
                    self.deliver(shared, unit, payload);
                }
            }
            // The ripple (ADR-0038): the replay drive to every unit of the episode the
            // coordinator chose, in the pattern's order, so that they fire together at the
            // next tick and phase 2 pairs every synapse among them as potentiation.
            let replay = shared.replay.load(Ordering::Relaxed);
            if replay != 0 {
                // SAFETY (phase 3): the ledger is written only between ticks by the
                // coordinator; every reference in a phase is shared.
                let Some(episode) =
                    (unsafe { shared.episodes.get(replay.wrapping_sub(1) as usize) })
                else {
                    abort("a ripple names an episode outside the ledger");
                };
                for &unit in episode.pattern() {
                    for _ in 0..REPLAY_MESSAGES {
                        self.deliver(shared, unit, REPLAY_MESSAGE);
                    }
                }
            }
        }
        shared.delivered[self.id].store(self.delivered, Ordering::Relaxed);
        shared.turns[self.id].store(self.turns, Ordering::Relaxed);
        shared.caught_up[self.id].store(self.caught_up, Ordering::Relaxed);
        shared.in_flight[self.id].store(self.in_flight, Ordering::Relaxed);
    }

    /// Phases 2 and 3: a message into a unit's mailbox, and the unit marked on the schedule
    /// for the next tick if it was idle (ADR-0100).
    fn deliver(&mut self, shared: &Shared, target: u32, message: u32) {
        let Some(node) = shared.pools.alloc(self.id) else {
            abort("the worker's mailbox node pool is exhausted (size nodes_per_worker, §8.9)");
        };
        // SAFETY (phases 2 and 3): no `&mut` to any unit exists in these phases; the mailbox
        // and the gate are atomics behind a shared reference.
        let Some(u) = (unsafe { shared.units.get(target as usize) }) else {
            abort("a synapse targets a unit outside the arena");
        };
        if !u.mailbox_push(shared.pools.nodes(), node, message) {
            abort("a mailbox push was refused");
        }
        shared.mark(target, u);
        Self::note_evicted(shared, target);
    }

    /// A message or a wake reached an evicted unit: the coordinator re-hydrates it after the
    /// tick, before the turn that drains the message.
    fn note_evicted(shared: &Shared, unit: u32) {
        if shared.evicted[unit as usize].load(Ordering::Relaxed)
            && !shared.needs_rehydration[unit as usize].swap(true, Ordering::AcqRel)
        {
            shared.rehydration_pending.fetch_add(1, Ordering::AcqRel);
        }
    }

    /// Phase 3, and between ticks from `Executor::wake_now`: a turn without a message.
    fn wake(shared: &Shared, unit: u32) {
        // SAFETY (phase 3): as in `deliver`. Between ticks `wake_now` holds `&mut` to the
        // executor, which excludes every reference it hands out, and every worker is parked at
        // the barrier.
        let Some(u) = (unsafe { shared.units.get(unit as usize) }) else {
            abort("an activation names a unit outside the arena");
        };
        shared.mark(unit, u);
        Self::note_evicted(shared, unit);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_configuration_is_validated() {
        let ok = Config {
            units: 4,
            ..Config::default()
        };
        assert!(Executor::<8>::new(ok.clone()).is_ok());
        assert_eq!(
            Executor::<8>::new(Config {
                workers: 0,
                ..ok.clone()
            })
            .err(),
            Some(ConfigError::NoWorkers)
        );
        assert_eq!(
            Executor::<8>::new(Config {
                units: 0,
                ..ok.clone()
            })
            .err(),
            Some(ConfigError::NoUnits)
        );
        assert_eq!(
            Executor::<8>::new(Config {
                nodes_per_worker: 0,
                ..ok.clone()
            })
            .err(),
            Some(ConfigError::NoNodes)
        );
        assert_eq!(
            Executor::<8>::new(Config {
                blocks: menteion_core::MAX_TOKEN_BLOCK as usize + 2,
                ..ok.clone()
            })
            .err(),
            Some(ConfigError::TooManyBlocks)
        );
        // The term arena and the store (ADR-0052): a store needs an arena, the counts stay
        // below the width, the shift below the cadence's width.
        assert_eq!(
            Executor::<8>::new(Config {
                clauses: 1,
                ..ok.clone()
            })
            .err(),
            Some(ConfigError::ClausesWithoutArena)
        );
        assert_eq!(
            Executor::<8>::new(Config {
                terms: u32::MAX as usize,
                ..ok.clone()
            })
            .err(),
            Some(ConfigError::TooManyTerms)
        );
        assert_eq!(
            Executor::<8>::new(Config {
                terms: 1,
                clauses: u32::MAX as usize,
                ..ok.clone()
            })
            .err(),
            Some(ConfigError::TooManyTerms)
        );
        assert_eq!(
            Executor::<8>::new(Config {
                search_shift: SEARCH_SHIFT_MAX + 1,
                ..ok.clone()
            })
            .err(),
            Some(ConfigError::SearchShiftOutOfRange)
        );
        // The inhibitory rule's target period (ADR-0053): within its bounds, at them.
        for period in [ISTDP_PERIOD_MIN_TICKS - 1, ISTDP_PERIOD_MAX_TICKS + 1, 0] {
            assert_eq!(
                Executor::<8>::new(Config {
                    istdp_target_period_ticks: period,
                    ..ok.clone()
                })
                .err(),
                Some(ConfigError::IstdpPeriodOutOfRange),
                "{period}"
            );
        }
        for period in [ISTDP_PERIOD_MIN_TICKS, ISTDP_PERIOD_MAX_TICKS] {
            let at = Executor::<8>::new(Config {
                istdp_target_period_ticks: period,
                ..ok.clone()
            })
            .unwrap();
            assert_eq!(at.istdp_target_period_ticks(), period);
            assert_eq!(
                at.shared.istdp_alpha.load(Ordering::Relaxed),
                i32::from(istdp_alpha_q1_15(period)),
                "the depression the workers read"
            );
        }
        assert_eq!(
            Executor::<8>::new(ok.clone())
                .unwrap()
                .istdp_target_period_ticks(),
            ISTDP_TARGET_PERIOD_TICKS
        );
        let with_store = Executor::<8>::new(Config {
            terms: 4,
            clauses: 2,
            search_shift: SEARCH_SHIFT_MAX,
            search_budget: 3,
            discovery_tag: 7,
            ..ok
        })
        .unwrap();
        assert_eq!(with_store.term_capacity(), (4, 2));
        assert_eq!(
            (
                with_store.induction().search_shift,
                with_store.induction().search_budget,
                with_store.induction().tag
            ),
            (SEARCH_SHIFT_MAX, 3, 7)
        );
        assert!(with_store.terms().is_empty() && with_store.clauses().is_empty());
        assert_eq!(
            (
                with_store.searches(),
                with_store.inventions(),
                with_store.untagged(),
                with_store.search_failures()
            ),
            (0, 0, 0, 0)
        );
        assert_eq!(search_cadence_of(0), None, "a shift of zero is no cadence");
        assert_eq!(
            search_cadence_of(4).map(|c| c.period()),
            Some(16),
            "a shift of four is every sixteen ticks"
        );
    }

    #[test]
    fn an_injection_is_validated_and_a_unit_at_rest_leaves_the_active_set() {
        let mut exec = Executor::<8>::new(Config {
            units: 2,
            trace_capacity: 8,
            ..Config::default()
        })
        .unwrap();
        let inject = exec.injector();
        assert_eq!(inject.inject(2, 1), Err(InjectError::NoSuchUnit));
        assert_eq!(
            inject.inject(0, ACTIVATE),
            Err(InjectError::ReservedPayload)
        );
        assert_eq!(inject.activate(5), Err(InjectError::NoSuchUnit));
        assert_eq!(inject.inject(1, spike_message(0x100, false)), Ok(()));
        exec.tick();
        assert_eq!(
            exec.delivered(),
            0,
            "drained at the end of the tick, integrated next"
        );
        exec.tick();
        assert_eq!(exec.delivered(), 1);
        assert_eq!(exec.units()[1].v_basal, 0x100);
        // The unit leaks to rest and then stops integrating: the potential stays zero and the
        // report shows no further work.
        exec.run(4000);
        assert_eq!(exec.units()[1].v_basal, 0);
        let reports = exec.shutdown();
        assert_eq!(reports.len(), 1);
        assert_eq!(reports[0].delivered, vec![spike_message(0x100, false)]);
        assert_eq!(reports[0].delivered_count, 1);
        assert!(reports[0].spikes.is_empty());
    }

    /// Axiom A3's partition (ADR-0100): for every arena and worker count of a grid, the ranges
    /// are contiguous from unit 0, at most a share each, and cover the arena exactly; each
    /// worker's region is the stride from `id·stride` and its words hold its range; and on an
    /// executor every unit has one owner, whose range holds it, and one place in the schedule,
    /// the bit of its position in that range. Outside the arena, neither.
    #[test]
    fn the_ranges_partition_the_arena_and_the_places_partition_the_schedule() {
        const UNITS: [usize; 12] = [1, 2, 3, 5, 63, 64, 65, 129, 512, 513, 1000, 4097];
        const WORKERS: [usize; 7] = [1, 2, 3, 4, 7, 8, 16];
        for units in UNITS {
            for workers in WORKERS {
                let (share, stride) = partition(units, workers);
                assert_eq!(share, units.div_ceil(workers), "{units}/{workers}");
                assert_eq!(
                    stride.checked_rem(8),
                    Some(0),
                    "a whole cache line of words"
                );
                assert!(
                    stride.saturating_mul(64) >= share
                        && stride.saturating_sub(8).saturating_mul(64) < share,
                    "the least whole line that holds a share: {units}/{workers}"
                );
                let mut next = 0usize;
                for id in 0..workers {
                    let (first, words, region) = range_of(id, units, share, stride);
                    let end = next.saturating_add(share).min(units);
                    assert_eq!(first as usize, next, "{units}/{workers} worker {id}");
                    assert_eq!(words, end.saturating_sub(next).div_ceil(64));
                    assert!(words <= stride);
                    assert_eq!(region, id.saturating_mul(stride));
                    next = end;
                }
                assert_eq!(next, units, "{units}/{workers}: the ranges cover the arena");
            }
        }
        for units in [1usize, 3, 65, 130] {
            for workers in [1usize, 2, 3, 4] {
                let exec = Executor::<8>::new(Config {
                    workers,
                    units,
                    ..Config::default()
                })
                .unwrap();
                let (share, stride) = partition(units, workers);
                let shared = &exec.shared;
                for unit in 0..units as u32 {
                    let owner = exec.owner(unit).expect("an owner");
                    let (first, words, region) = range_of(owner, units, share, stride);
                    let at = (unit as usize).wrapping_sub(first as usize);
                    assert!(first <= unit && at < share, "{units}/{workers} unit {unit}");
                    let (word, bit) = shared.place(unit).expect("a place");
                    let index = shared
                        .scheduled
                        .iter()
                        .position(|w| core::ptr::eq(w, word))
                        .unwrap();
                    assert!(index >= region && index < region.wrapping_add(words));
                    assert_eq!(
                        (index.wrapping_sub(region), bit),
                        (at >> 6, 1u64 << (at & 63)),
                        "{units}/{workers} unit {unit}"
                    );
                }
                assert_eq!(exec.owner(units as u32), None);
                assert!(shared.place(units as u32).is_none());
            }
        }
    }

    /// Two armed units fired by fourteen strong messages each, `rounds` times, 400 ticks
    /// apart; returns the executor after the last round and the spikes its workers traced.
    fn fired(train_capacity: usize, rounds: u32) -> (Executor<8>, Vec<(u32, u32)>) {
        use menteion_core::{STP_MAX, STP_U, synaptic_efficacy_q16};
        let mut exec = Executor::<8>::new(Config {
            units: 2,
            nodes_per_worker: 64,
            trace_capacity: 64,
            train_capacity,
            ..Config::default()
        })
        .unwrap();
        for unit in exec.units_mut() {
            unit.v_thresh = THRESHOLD_BASE;
            unit.stp_u_rel = STP_U;
            unit.stp_r_ves = STP_MAX;
        }
        let inject = exec.injector();
        let strong = spike_message(synaptic_efficacy_q16(i16::MAX, STP_U, STP_MAX), false);
        for _ in 0..rounds {
            for unit in [1, 0] {
                for _ in 0..14 {
                    inject.inject(unit, strong).unwrap();
                }
            }
            exec.run(400);
        }
        let traced: Vec<(u32, u32)> = exec
            .worker0
            .spike_trace
            .iter()
            .map(|&(u, t)| (t, u))
            .collect();
        (exec, traced)
    }

    #[test]
    fn the_train_keeps_the_last_spikes_in_tick_and_unit_order_and_counts_what_it_let_go() {
        // Three rounds fire both units three times: six spikes, of which a ring of four
        // keeps the last four in tick order, both units of a tick in unit order although
        // unit 1 was injected first, and counts the two it let go.
        let (mut exec, traced) = fired(4, 3);
        assert_eq!(traced.len(), 6, "{traced:?}");
        let mut sorted = traced.clone();
        sorted.sort_unstable();
        assert_eq!(exec.train(), &sorted[2..], "the last four, sorted");
        assert_eq!(exec.train_overwritten(), 2);
        assert_eq!(exec.train_capacity(), 4);
        let last = exec.train().to_vec();
        assert_eq!(last[0].1, 0, "unit 0 before unit 1 at the same tick");
        assert_eq!(last[0].0, last[1].0);
        // A ring the size of the run keeps every spike and lets none go.
        let (mut exec, traced) = fired(6, 3);
        let mut sorted = traced;
        sorted.sort_unstable();
        assert_eq!(exec.train(), &sorted[..]);
        assert_eq!(exec.train_overwritten(), 0);
        // No train: nothing kept, nothing counted, and the tick has no slot to fill.
        let (mut exec, traced) = fired(0, 3);
        assert_eq!(traced.len(), 6);
        assert!(exec.train().is_empty());
        assert_eq!((exec.train_overwritten(), exec.train_capacity()), (0, 0));
    }

    /// The addressed set (ADR-0068): every unit a source and a target at birth; an addressing
    /// is exactly the units named on each side, refused whole for a unit outside the arena on
    /// either side with both sides left as they were, empty on a side asked for no unit;
    /// `address_all` is the birth state again.
    #[test]
    fn an_addressing_is_exact_and_refused_whole_outside_the_arena() {
        let mut exec = Executor::<8>::new(Config {
            units: 4,
            ..Config::default()
        })
        .unwrap();
        let sides = |exec: &Executor<8>| {
            (
                (0..5).map(|u| exec.is_source(u)).collect::<Vec<bool>>(),
                (0..5).map(|u| exec.is_target(u)).collect::<Vec<bool>>(),
            )
        };
        let all = vec![true, true, true, true, false];
        assert_eq!(
            exec.addressed_counts(),
            (4, 4),
            "every unit, both sides, at birth"
        );
        assert_eq!(sides(&exec), (all.clone(), all.clone()));
        assert!(
            !exec.is_source(u32::MAX) && !exec.is_target(u32::MAX),
            "outside the arena is neither"
        );
        assert_eq!(exec.address([0u32], [1u32, 3]), Ok(()));
        assert_eq!(exec.addressed_counts(), (1, 2));
        let narrowed = (
            vec![true, false, false, false, false],
            vec![false, true, false, true, false],
        );
        assert_eq!(sides(&exec), narrowed);
        assert_eq!(
            exec.address([0u32], [1u32, 4]),
            Err(AddressError::NoSuchUnit),
            "refused whole for a target outside"
        );
        assert_eq!(sides(&exec), narrowed, "as it was");
        assert_eq!(
            exec.address([4u32], [1u32]),
            Err(AddressError::NoSuchUnit),
            "and for a source outside"
        );
        assert_eq!(sides(&exec), narrowed);
        assert_eq!(
            exec.address([0u32, u32::MAX], core::iter::empty()),
            Err(AddressError::NoSuchUnit)
        );
        assert_eq!(sides(&exec), narrowed);
        assert_eq!(exec.address([2u32], core::iter::empty()), Ok(()));
        assert_eq!(exec.addressed_counts(), (1, 0), "a side of no unit");
        assert_eq!(
            sides(&exec),
            (vec![false, false, true, false, false], vec![false; 5])
        );
        assert_eq!(
            exec.address([2u32, 2], [2u32, 2]),
            Ok(()),
            "a unit named twice is addressed once"
        );
        assert_eq!(exec.addressed_counts(), (1, 1));
        exec.address_all();
        assert_eq!(exec.addressed_counts(), (4, 4));
        assert_eq!(sides(&exec), (all.clone(), all.clone()));
        assert_eq!(exec.address([4u32], [4u32]), Err(AddressError::NoSuchUnit));
        assert_eq!(exec.addressed_counts(), (4, 4), "as it was: every unit");
        assert_eq!(
            exec.address(core::iter::empty(), core::iter::empty()),
            Ok(())
        );
        assert_eq!(exec.addressed_counts(), (0, 0));
    }

    /// The inhibitory baseline (ADR-0086): unset by default; refused when set outside
    /// $[0, 1]$, at both edges, by `new` and by the loader's setter, which leaves the baseline
    /// as it was; accepted at the edges and unset again.
    #[test]
    fn the_inhibitory_baseline_is_unset_by_default_and_refused_when_set_outside_the_unit_interval()
    {
        let ok = Config {
            units: 2,
            ..Config::default()
        };
        assert_eq!(ok.inhibitory_baseline_q16, None);
        let unset = Executor::<8>::new(ok.clone()).unwrap();
        assert_eq!(unset.inhibitory_baseline_q16(), None);
        assert_eq!(
            unset.shared.modulation_inhibitory.load(Ordering::Relaxed),
            INHIBITORY_UNSET
        );
        for outside in [-1, MODULATION_ONE_Q16 + 1, i32::MIN, i32::MAX] {
            assert_eq!(
                Executor::<8>::new(Config {
                    inhibitory_baseline_q16: Some(outside),
                    ..ok.clone()
                })
                .err(),
                Some(ConfigError::InhibitoryBaselineOutOfRange),
                "{outside}"
            );
        }
        for inside in [0, 1, 0x8000, MODULATION_ONE_Q16 - 1, MODULATION_ONE_Q16] {
            let set = Executor::<8>::new(Config {
                inhibitory_baseline_q16: Some(inside),
                ..ok.clone()
            })
            .unwrap();
            assert_eq!(set.inhibitory_baseline_q16(), Some(inside), "{inside}");
        }
        let mut exec = Executor::<8>::new(ok).unwrap();
        assert!(exec.set_inhibitory_baseline(Some(0x8000)));
        assert_eq!(exec.inhibitory_baseline_q16(), Some(0x8000));
        for outside in [-1, MODULATION_ONE_Q16 + 1] {
            assert!(!exec.set_inhibitory_baseline(Some(outside)), "{outside}");
            assert_eq!(
                exec.inhibitory_baseline_q16(),
                Some(0x8000),
                "refused whole: the baseline stands"
            );
        }
        assert!(exec.set_inhibitory_baseline(Some(0)));
        assert_eq!(
            exec.inhibitory_baseline_q16(),
            Some(0),
            "set at zero is set"
        );
        assert!(exec.set_inhibitory_baseline(Some(MODULATION_ONE_Q16)));
        assert_eq!(exec.inhibitory_baseline_q16(), Some(MODULATION_ONE_Q16));
        assert!(exec.set_inhibitory_baseline(None));
        assert_eq!(exec.inhibitory_baseline_q16(), None, "unset again");
        // The tick publishes the word the workers read: the modulation while set, the
        // sentinel while unset.
        assert!(exec.set_inhibitory_baseline(Some(0x4000)));
        exec.tick();
        assert_eq!(
            exec.shared.modulation_inhibitory.load(Ordering::Relaxed),
            0x4000
        );
        assert!(exec.set_inhibitory_baseline(Some(0)));
        exec.tick();
        assert_eq!(
            exec.shared.modulation_inhibitory.load(Ordering::Relaxed),
            0,
            "set at zero publishes zero, which is not the sentinel"
        );
        assert!(exec.set_inhibitory_baseline(None));
        exec.tick();
        assert_eq!(
            exec.shared.modulation_inhibitory.load(Ordering::Relaxed),
            INHIBITORY_UNSET
        );
    }

    /// The signed gate (ADR-0094): unset by default and set by the configuration and by the
    /// loader's setter; the tick publishes the addressed modulation below zero under a
    /// punishment only while it is set — the signal as it stands, clamped at −1.0 — and zero
    /// while it is unset, the modulation at rest zero either way at the gate's baseline.
    #[test]
    fn the_signed_gate_is_unset_by_default_and_publishes_a_modulation_below_zero_only_while_set() {
        let ok = Config {
            units: 2,
            modulation_baseline_q16: 0,
            ..Config::default()
        };
        assert!(!ok.signed_gate);
        assert!(!Config::default().signed_gate, "unset by default");
        let set = Executor::<8>::new(Config {
            signed_gate: true,
            ..ok.clone()
        })
        .unwrap();
        assert!(set.signed_gate(), "set by the configuration");
        let mut exec = Executor::<8>::new(ok).unwrap();
        assert!(!exec.signed_gate());
        let published = |exec: &Executor<8>| {
            (
                exec.shared.modulation.load(Ordering::Relaxed),
                exec.shared.modulation_at_rest.load(Ordering::Relaxed),
            )
        };
        // Unset: a punishment publishes zero, the gate's floor.
        assert_eq!(exec.reward(-MODULATION_ONE_Q16), -MODULATION_ONE_Q16);
        exec.tick();
        assert_eq!(published(&exec), (0, 0), "unset: clamped at zero");
        // Set: the signal as it stands, below zero.
        exec.set_signed_gate(true);
        assert!(exec.signed_gate());
        let signal = exec.modulator().dopamine_rpe;
        assert!(signal < 0 && signal > -MODULATION_ONE_Q16, "{signal}");
        exec.tick();
        assert_eq!(published(&exec), (signal, 0), "set: the signal below zero");
        // A second punishment carries the signal below −1.0, and the modulation stops there.
        exec.reward(-MODULATION_ONE_Q16);
        assert!(exec.modulator().dopamine_rpe < -MODULATION_ONE_Q16);
        exec.tick();
        assert_eq!(
            published(&exec),
            (-MODULATION_ONE_Q16, 0),
            "set: clamped at −1.0"
        );
        // A reward above the floor publishes the same modulation set or unset.
        exec.reward(2 * MODULATION_ONE_Q16);
        let signal = exec.modulator().dopamine_rpe;
        assert!(signal > 0 && signal < MODULATION_ONE_Q16, "{signal}");
        exec.tick();
        assert_eq!(published(&exec), (signal, 0), "set, above zero");
        exec.set_signed_gate(false);
        let signal = exec.modulator().dopamine_rpe;
        exec.tick();
        assert_eq!(published(&exec), (signal, 0), "unset, above zero");
        // Unset again: a punishment publishes zero.
        exec.reward(-3 * MODULATION_ONE_Q16);
        exec.tick();
        assert_eq!(published(&exec), (0, 0), "unset again");
    }

    /// The class of short-term plasticity (ADR-0114): unset by default; refused by `new` at
    /// every edge the rule does not resolve and taken at every edge it does. In one run under
    /// a class, the marked unit's factors step under it and the unmarked unit's under ADR-0019's
    /// constants, each held to the step on a copy of its factors from rest, spike by spike, with
    /// the intervals the executor's own train read; with no class the marked unit steps as the
    /// unmarked one does, bit for bit.
    #[test]
    fn a_marked_unit_steps_under_the_class_and_an_unmarked_one_under_adr_0019_s_constants() {
        use menteion_core::{STP_MAX, STP_U, synaptic_efficacy_q16};
        assert_eq!(Config::default().stp_class, None, "unset by default");
        let class = |u, tau_f_shift, tau_d_shift| StpClass {
            u,
            tau_f_shift,
            tau_d_shift,
        };
        let with = |stp_class| Config {
            units: 2,
            nodes_per_worker: 64,
            train_capacity: 64,
            stp_class,
            ..Config::default()
        };
        for bad in [
            class(0, 16, 13),
            class(51, 0, 13),
            class(51, 17, 13),
            class(51, 16, 0),
            class(51, 16, 17),
        ] {
            assert_eq!(
                Executor::<8>::new(with(Some(bad))).err(),
                Some(ConfigError::StpClassOutOfRange),
                "{bad:?}"
            );
        }
        for good in [class(1, 1, 1), class(255, 16, 16), class(26, 16, 13)] {
            let exec = Executor::<8>::new(with(Some(good))).unwrap();
            assert_eq!(exec.stp_class(), Some(good));
        }
        assert_eq!(Executor::<8>::new(with(None)).unwrap().stp_class(), None);
        // Both units at rest, unit 0 marked; each round fires both with fourteen strong
        // messages and runs the round's ticks, so the intervals between spikes vary.
        let rounds = [400u32, 1_000, 250, 5_000, 20_000, 300, 70_000, 600];
        let run = |stp_class: Option<StpClass>| {
            let mut exec = Executor::<8>::new(with(stp_class)).unwrap();
            for unit in exec.units_mut() {
                unit.v_thresh = THRESHOLD_BASE;
                unit.stp_u_rel = STP_U;
                unit.stp_r_ves = STP_MAX;
            }
            exec.units_mut()[0].flags |= FLAG_FACILITATING;
            let inject = exec.injector();
            let strong = spike_message(synaptic_efficacy_q16(i16::MAX, STP_U, STP_MAX), false);
            let mut train = Vec::new();
            for &ticks in &rounds {
                for unit in [0, 1] {
                    for _ in 0..14 {
                        inject.inject(unit, strong).unwrap();
                    }
                }
                exec.run(u64::from(ticks));
                train.extend_from_slice(exec.train());
                exec.train.clear();
            }
            let factors: Vec<(u8, u8)> = exec
                .units()
                .iter()
                .map(|u| (u.stp_u_rel, u.stp_r_ves))
                .collect();
            (train, factors)
        };
        // The factors a unit's spikes leave, by the step on a copy from rest: `class` for the
        // class's step, none for `step_stp`.
        let stepped = |train: &[(u32, u32)], unit: u32, class: Option<StpClass>| {
            let mut copy = DendriticSuperNeuron::new(0);
            copy.stp_u_rel = STP_U;
            copy.stp_r_ves = STP_MAX;
            let mut last: Option<u32> = None;
            let mut spikes = 0u32;
            for &(tick, _) in train.iter().filter(|&&(_, u)| u == unit) {
                let elapsed = last.map_or(u32::MAX, |l| tick.wrapping_sub(l));
                match class {
                    Some(c) => copy.step_stp_class(elapsed, c),
                    None => copy.step_stp(elapsed),
                };
                last = Some(tick);
                spikes = spikes.saturating_add(1);
            }
            ((copy.stp_u_rel, copy.stp_r_ves), spikes)
        };
        let set = class(26, 16, 13);
        let (train, factors) = run(Some(set));
        let (marked, marked_spikes) = stepped(&train, 0, Some(set));
        let (unmarked, unmarked_spikes) = stepped(&train, 1, None);
        assert!(
            marked_spikes >= 8 && unmarked_spikes >= 8,
            "every round fires both: {train:?}"
        );
        assert_eq!(factors, [marked, unmarked], "{train:?}");
        assert_ne!(marked, stepped(&train, 0, None).0, "the class tells");
        assert_ne!(unmarked, stepped(&train, 1, Some(set)).0);
        // No class: the mark is read by nothing, and both units step under ADR-0019's.
        let (train, factors) = run(None);
        assert_eq!(
            factors,
            [stepped(&train, 0, None).0, stepped(&train, 1, None).0]
        );
    }

    /// The slow current (ADR-0123): unset by default; refused by `new` at every edge the rule
    /// does not resolve and taken at every edge it does. In one run under it at a gain of 1.75,
    /// both units fed the same batches — excitatory and inhibitory synapses' messages into the
    /// basal compartment, a synapse's into the apical one, and injected messages into both — the
    /// marked unit is `integrate_slow` on a copy whose slow input is the positive efficacies of
    /// the synapses' basal messages alone, scaled by the gain, and the unmarked unit is
    /// `integrate` on a copy, tick by tick; with no slow current the marked unit is `integrate`'s
    /// too. A marked unit with a slow potential and nothing else stays on the schedule, leaking it,
    /// until it is zero; an unmarked unit's turn does not read it.
    #[test]
    fn a_marked_unit_takes_its_excitatory_synaptic_basal_input_into_its_slow_potential_and_an_unmarked_one_integrates_as_before()
     {
        use menteion_core::{FLAG_SLOW, SlowCurrent, synaptic_message};
        assert_eq!(Config::default().slow_current, None, "unset by default");
        let slow = |leak_shift, input_shift, v_lo_q16, v_hi_q16| SlowCurrent {
            leak_shift,
            input_shift,
            v_lo_q16,
            v_hi_q16,
        };
        let with = |slow_current| Config {
            units: 2,
            nodes_per_worker: 64,
            slow_current,
            ..Config::default()
        };
        for bad in [
            slow(0, 1, 0x4000, THRESHOLD_BASE),
            slow(17, 1, 0x4000, THRESHOLD_BASE),
            slow(6, 17, 0x4000, THRESHOLD_BASE),
            slow(6, 1, 0, THRESHOLD_BASE),
            slow(6, 1, 0x4000, 0x4000),
            slow(6, 1, 0x4000, THRESHOLD_BASE + 1),
        ] {
            assert_eq!(
                Executor::<8>::new(with(Some(bad))).err(),
                Some(ConfigError::SlowCurrentOutOfRange),
                "{bad:?}"
            );
        }
        for good in [
            slow(1, 0, 1, 2),
            slow(16, 16, 1, THRESHOLD_BASE),
            slow(13, 1, 28_561, THRESHOLD_BASE),
        ] {
            let exec = Executor::<8>::new(with(Some(good))).unwrap();
            assert_eq!(exec.slow_current(), Some(good));
        }
        assert_eq!(Executor::<8>::new(with(None)).unwrap().slow_current(), None);
        // The batch injected before tick `k`, into both units: a pattern of every kind of message.
        let batch = |k: u32| -> Vec<u32> {
            let at = |m: u32, a: u32, b: u32| (k.wrapping_mul(m) % b).wrapping_add(a) as i32;
            let mut out = Vec::new();
            if k % 3 != 2 {
                out.push(synaptic_message(at(97, 0x180, 0x300), false));
            }
            if k % 7 == 0 {
                out.push(synaptic_message(at(13, 0x40, 0x100), false));
            }
            if k % 5 == 1 {
                out.push(synaptic_message(-at(31, 0x80, 0x200), false));
            }
            if k % 4 == 3 {
                out.push(synaptic_message(at(53, 0x100, 0x400), true));
            }
            if k % 2 == 0 {
                out.push(spike_message(at(71, 0x100, 0x200), false));
            }
            if k % 6 == 5 {
                out.push(spike_message(at(19, 0x80, 0x200), true));
            }
            out
        };
        let gain = 0x1_C000u32;
        const TICKS: u32 = 3_000;
        let current = slow(6, 1, 0x4000, THRESHOLD_BASE);
        let run = |slow_current: Option<SlowCurrent>| {
            let mut exec = Executor::<8>::new(with(slow_current)).unwrap();
            exec.homeostasis.synaptic_gain_q16 = gain;
            let mut copies = [DendriticSuperNeuron::new(0), DendriticSuperNeuron::new(1)];
            for (unit, copy) in exec.units_mut().iter_mut().zip(copies.iter_mut()) {
                unit.v_thresh = THRESHOLD_BASE;
                copy.v_thresh = THRESHOLD_BASE;
            }
            exec.units_mut()[0].flags = FLAG_SLOW;
            copies[0].flags = FLAG_SLOW;
            let inject = exec.injector();
            let (mut pending, mut fired, mut highest) = (Vec::new(), [0u32; 2], 0i32);
            for k in 0..TICKS {
                let now = exec.ticks() as u32;
                let this = batch(k);
                for unit in [0, 1] {
                    for &m in &this {
                        inject.inject(unit, m).unwrap();
                    }
                }
                exec.tick();
                // What the tick integrated: the batch injected before the tick before it.
                let (mut basal, mut apical, mut input) = (0i32, 0i32, 0i32);
                for &m in &pending {
                    let e = message_efficacy_q16(m);
                    if message_is_apical(m) {
                        apical = apical.saturating_add(e);
                    } else {
                        basal = basal.saturating_add(e);
                        if message_is_synaptic(m) && e > 0 {
                            input = input.saturating_add(e);
                        }
                    }
                }
                let (basal, apical, input) = (
                    scaled(basal, gain),
                    scaled(apical, gain),
                    scaled(input, gain),
                );
                let marked = match slow_current {
                    Some(c) => copies[0].integrate_slow(basal, apical, input, now, c),
                    None => copies[0].integrate(basal, apical, now),
                };
                let unmarked = copies[1].integrate(basal, apical, now);
                for (i, (unit, copy)) in exec.units().iter().zip(copies.iter()).enumerate() {
                    assert_eq!(
                        (
                            unit.v_soma,
                            unit.v_basal,
                            unit.v_apical,
                            unit.v_thresh,
                            unit.refractory_ticks,
                            unit.last_soma_spike_tick,
                            unit.flags,
                            unit.v_slow,
                        ),
                        (
                            copy.v_soma,
                            copy.v_basal,
                            copy.v_apical,
                            copy.v_thresh,
                            copy.refractory_ticks,
                            copy.last_soma_spike_tick,
                            copy.flags,
                            copy.v_slow,
                        ),
                        "unit {i} at tick {k}"
                    );
                }
                fired[0] = fired[0].saturating_add(u32::from(marked));
                fired[1] = fired[1].saturating_add(u32::from(unmarked));
                highest = highest.max(exec.units()[0].v_slow);
                pending = this;
            }
            (fired, highest)
        };
        let (fired, highest) = run(Some(current));
        assert!(fired[1] > 10, "the batches fire the units: {fired:?}");
        assert!(fired[0] > fired[1], "the slow current tells: {fired:?}");
        assert!(highest > 0x4000, "the slow potential rose: {highest:#x}");
        let (fired, highest) = run(None);
        assert_eq!(
            (fired[0], highest),
            (fired[1], 0),
            "no slow current: {fired:?}"
        );
        // A marked unit with a slow potential and nothing else: on the schedule, leaking it by
        // 2^-6 a tick and at least one LSB, until it is zero within 200 ticks; the gate below 0.25 shut, so the soma
        // stays at rest. An unmarked unit's turn does not read the field.
        let mut exec = Executor::<8>::new(with(Some(current))).unwrap();
        for unit in exec.units_mut() {
            unit.v_thresh = THRESHOLD_BASE;
            unit.v_slow = 0x100;
        }
        exec.units_mut()[0].flags = FLAG_SLOW;
        let inject = exec.injector();
        inject.activate(0).unwrap();
        inject.activate(1).unwrap();
        exec.tick();
        let mut expected = 0x100i32;
        for t in 0..200 {
            exec.tick();
            expected = expected.saturating_sub((expected >> 6).max(1).min(expected));
            let marked = &exec.units()[0];
            assert_eq!(marked.v_slow, expected, "tick {t}");
            assert_eq!(marked.v_soma, 0, "the soma at rest");
            let schedule = if expected == 0 {
                GateState::Idle
            } else {
                GateState::Scheduled
            };
            assert_eq!(marked.gate(), Some(schedule), "tick {t}");
            assert_eq!(exec.units()[1].v_slow, 0x100, "untouched");
            assert_eq!(exec.units()[1].gate(), Some(GateState::Idle));
        }
        assert_eq!(expected, 0, "the slow potential leaked away");
    }

    /// The critic (ADR-0131): unset by default, with no count and no reading; `new` refuses
    /// constants the rule does not resolve and a critic with no train to count from, and takes
    /// every edge it resolves.
    #[test]
    fn the_critic_is_unset_by_default_and_refused_outside_what_its_rule_resolves_or_without_a_train()
     {
        assert_eq!(Config::default().critic, None, "unset by default");
        let exec = Executor::<8>::new(Config {
            units: 3,
            train_capacity: 8,
            ..Config::default()
        })
        .unwrap();
        assert_eq!(
            (exec.critic(), exec.features(), exec.prediction()),
            (None, &[][..], None)
        );
        let with = |shift, scale, train_capacity| {
            Executor::<8>::new(Config {
                units: 3,
                train_capacity,
                critic: Some(ValueCritic { shift, scale }),
                ..Config::default()
            })
        };
        for (shift, scale) in [(0, 0), (30, 14), (9, 2)] {
            let exec = with(shift, scale, 8).unwrap();
            assert_eq!(exec.critic(), Some(ValueCritic { shift, scale }));
            assert_eq!(exec.features(), &[0, 0, 0], "one count a unit, zero");
            assert_eq!(exec.prediction(), None);
        }
        for (shift, scale) in [(31, 0), (0, 15), (u8::MAX, u8::MAX)] {
            assert!(
                matches!(with(shift, scale, 8), Err(ConfigError::CriticOutOfRange)),
                "{shift} {scale}"
            );
        }
        assert!(matches!(
            with(9, 2, 0),
            Err(ConfigError::CriticWithoutTrain)
        ));
        assert!(with(9, 2, 1).is_ok(), "a train of one spike is a train");
    }

    /// Unset, a reward is the modulator's own (ADR-0131): over the lattice of rewards, between
    /// ticks of a network that fires, the signal is the oracle's modulator's after the same
    /// rewards and decays, bit for bit; no reading, no count, and a value weight written in a
    /// record by hand is read and moved by nothing.
    #[test]
    fn with_the_critic_unset_a_reward_is_the_modulator_s_bit_for_bit() {
        use menteion_core::{STP_MAX, STP_U, synaptic_efficacy_q16};
        let mut exec = Executor::<8>::new(Config {
            units: 2,
            nodes_per_worker: 64,
            train_capacity: 64,
            ..Config::default()
        })
        .unwrap();
        for unit in exec.units_mut() {
            unit.v_thresh = THRESHOLD_BASE;
            unit.stp_u_rel = STP_U;
            unit.stp_r_ves = STP_MAX;
            unit.value_weight = 0x1234;
        }
        let strong = spike_message(synaptic_efficacy_q16(i16::MAX, STP_U, STP_MAX), false);
        let mut oracle = NeuromodulatorState::new();
        let rewards = [i32::MIN, -0x1_0000, -1, 0, 1, 0x4000, 0x1_0000, i32::MAX];
        for &reward in &rewards {
            let inject = exec.injector();
            for _ in 0..14 {
                inject.inject(0, strong).unwrap();
            }
            for _ in 0..300 {
                exec.tick();
                oracle.decay_dopamine(DOPAMINE_TAU_SHIFT);
            }
            assert_eq!(exec.reward(reward), oracle.reward(reward), "{reward}");
            assert_eq!(exec.modulator(), &oracle);
            assert_eq!((exec.features(), exec.prediction()), (&[][..], None));
        }
        assert!(exec.train().len() >= rewards.len(), "the unit fired");
        assert!(
            exec.units().iter().all(|u| u.value_weight == 0x1234),
            "no weight read or moved"
        );
    }

    /// The critic's composition (ADR-0131), each number from an oracle written from the rule's
    /// text: four armed units kicked in patterns, twice in each window between two rewards, so
    /// that a unit fires more than once or not at all in a window and a window holds no spike. At
    /// every tick no weight moves; at every reward the features are exactly the train's spikes
    /// since the previous reward, the reading is the value — the weights times the counts, over
    /// four, floored — and the error, the signal moves by the error, every weight moves by the
    /// error times its count over 512, floored, and every count starts again from zero. With a
    /// ring that holds two spikes the features are still every spike since the previous reward,
    /// the worker's trace's, however many the ring let go.
    #[test]
    fn the_critic_s_features_are_the_spikes_since_the_previous_reward_and_its_weights_move_only_at_a_reward()
     {
        use menteion_core::{STP_MAX, STP_U, synaptic_efficacy_q16};
        let critic = ValueCritic { shift: 9, scale: 2 };
        let make = |train_capacity| {
            let mut exec = Executor::<8>::new(Config {
                units: 4,
                nodes_per_worker: 128,
                trace_capacity: 256,
                train_capacity,
                critic: Some(critic),
                ..Config::default()
            })
            .unwrap();
            for (unit, weight) in exec.units_mut().iter_mut().zip([100i16, -40, 7, 0]) {
                unit.v_thresh = THRESHOLD_BASE;
                unit.stp_u_rel = STP_U;
                unit.stp_r_ves = STP_MAX;
                unit.value_weight = weight;
            }
            exec
        };
        let strong = spike_message(synaptic_efficacy_q16(i16::MAX, STP_U, STP_MAX), false);
        let weights = |exec: &Executor<8>| -> Vec<i16> {
            exec.units().iter().map(|u| u.value_weight).collect()
        };
        let patterns: [&[u32]; 5] = [&[0, 1], &[2], &[], &[0, 3, 1], &[3]];
        let rewards = [0x1_0000, -0x1_0000, 0x4000, 0x1_0000, i32::MIN];
        let mut finals = Vec::new();
        for train_capacity in [64, 2] {
            let mut exec = make(train_capacity);
            let mut since = 0u64;
            let mut traced_before = 0usize;
            let mut most = 0u32;
            for (window, (pattern, &reward)) in patterns.iter().zip(&rewards).enumerate() {
                for _ in 0..2 {
                    let inject = exec.injector();
                    for &unit in *pattern {
                        for _ in 0..14 {
                            inject.inject(unit, strong).unwrap();
                        }
                    }
                    let before = weights(&exec);
                    for _ in 0..400 {
                        exec.tick();
                        assert_eq!(weights(&exec), before, "window {window}: a tick moves none");
                    }
                }
                // The spikes since the previous reward, from the worker's own trace, which holds
                // every one; and, where the ring holds them, from the train.
                let traced = &exec.worker0.spike_trace[traced_before..];
                traced_before = exec.worker0.spike_trace.len();
                let mut counts = [0u32; 4];
                for &(unit, tick) in traced {
                    assert!(u64::from(tick) >= since);
                    counts[unit as usize] = counts[unit as usize].saturating_add(1);
                }
                for (unit, &count) in counts.iter().enumerate() {
                    assert_eq!(
                        count > 0,
                        pattern.contains(&(unit as u32)),
                        "window {window}: unit {unit} fired as injected"
                    );
                }
                most = most.max(counts.iter().copied().max().unwrap_or(0));
                assert_eq!(exec.features(), &counts, "window {window}: the features");
                if train_capacity == 64 {
                    let mut from_train = [0u32; 4];
                    for &(tick, unit) in exec.train() {
                        if u64::from(tick) >= since {
                            from_train[unit as usize] = from_train[unit as usize].saturating_add(1);
                        }
                    }
                    assert_eq!(from_train, counts, "window {window}: the train's");
                }
                let before = weights(&exec);
                let exact: i64 = before
                    .iter()
                    .zip(&counts)
                    .map(|(&w, &c)| i64::from(w).saturating_mul(i64::from(c)))
                    .sum();
                let value = exact.div_euclid(4) as i32;
                let error = (i64::from(reward) - i64::from(value))
                    .clamp(i64::from(i32::MIN), i64::from(i32::MAX))
                    as i32;
                let signal = exec.modulator().dopamine_rpe;
                assert_eq!(
                    exec.reward(reward),
                    signal.saturating_add(error),
                    "window {window}: the modulator receives the error"
                );
                assert_eq!(
                    exec.prediction(),
                    Some(Prediction {
                        value_q16: value,
                        error_q16: error,
                        received_q16: error
                    }),
                    "window {window}"
                );
                let moved: Vec<i16> = before
                    .iter()
                    .zip(&counts)
                    .map(|(&w, &c)| {
                        (i64::from(w) + (i64::from(error) * i64::from(c)).div_euclid(512))
                            .clamp(i64::from(i16::MIN), i64::from(i16::MAX))
                            as i16
                    })
                    .collect();
                assert_eq!(weights(&exec), moved, "window {window}: the delta rule");
                assert_eq!(exec.features(), &[0; 4], "window {window}: counted afresh");
                since = exec.ticks();
            }
            assert!(most >= 2, "a unit fired more than once in a window");
            if train_capacity == 2 {
                assert!(exec.train_overwritten() > 0, "the ring let spikes go");
            }
            finals.push((weights(&exec), exec.modulator().dopamine_rpe));
        }
        assert_eq!(
            finals[0], finals[1],
            "the ring's size changes nothing the critic reads"
        );
    }

    /// The critic's window (ADR-0134): unset by default, where it reads zero and opens at the
    /// engine's start; refused while the critic is unset; taken at every length the field holds
    /// while the critic is set.
    #[test]
    fn the_window_is_unset_by_default_and_refused_without_the_critic() {
        assert_eq!(Config::default().critic_window_ticks, 0, "unset by default");
        let with = |critic, window| {
            Executor::<8>::new(Config {
                units: 3,
                train_capacity: 8,
                critic,
                critic_window_ticks: window,
                ..Config::default()
            })
        };
        let unset = with(None, 0).unwrap();
        assert_eq!((unset.critic_window_ticks(), unset.window_opened()), (0, 0));
        let critic = Some(ValueCritic { shift: 9, scale: 2 });
        assert_eq!(with(critic, 0).unwrap().critic_window_ticks(), 0);
        for window in [1, 100, u16::MAX] {
            let exec = with(critic, window).unwrap();
            assert_eq!(
                (exec.critic_window_ticks(), exec.window_opened()),
                (window, 0),
                "{window}: taken, open at the start"
            );
            assert!(
                matches!(with(None, window), Err(ConfigError::WindowWithoutCritic)),
                "{window}: refused without the critic"
            );
        }
    }

    /// The window's rule (ADR-0134), each number from an oracle written from the rule's text:
    /// four armed units kicked at different ticks after each reward, and before the first, so
    /// that their spikes fall at several distances from the window's opening. A first run with the
    /// window unset reads those distances and is ADR-0131's critic; then, at every length on both
    /// sides of every distance, a run of the same inputs, where the spikes are the first run's
    /// tick for tick, and at every reward the features are exactly the train's spikes fewer ticks
    /// after the opening than the length, the reading is the value and the error of those counts,
    /// every weight moves by them, and the window opens again at the reward's tick; before the
    /// first reward it opens at the engine's start.
    #[test]
    fn the_window_counts_only_the_spikes_within_its_length_after_the_previous_reward() {
        use menteion_core::{STP_MAX, STP_U, synaptic_efficacy_q16};
        let critic = ValueCritic { shift: 9, scale: 2 };
        let strong = spike_message(synaptic_efficacy_q16(i16::MAX, STP_U, STP_MAX), false);
        // After each reward, and before the first: (ticks after the opening, units kicked).
        let kicks: [&[(u64, &[u32])]; 4] = [
            &[(0, &[0]), (3, &[1, 2]), (40, &[3])],
            &[(0, &[1, 3]), (7, &[0]), (200, &[2])],
            &[(5, &[2]), (6, &[0, 1])],
            &[(0, &[0, 1, 2, 3]), (300, &[0, 1])],
        ];
        let rewards = [0x1_0000, -0x1_0000, 0x8000, 0x1_0000];
        let window_ticks = 400u64;
        // One run: every spike's (tick, unit), the opening before each reward, and at each reward
        // the features, the reading and the weights after.
        type Read = (
            Vec<(u32, u32)>,
            Vec<u64>,
            Vec<(Vec<u32>, Prediction, Vec<i16>)>,
        );
        let run = |window: u16| -> Read {
            let mut exec = Executor::<8>::new(Config {
                units: 4,
                nodes_per_worker: 128,
                train_capacity: 256,
                critic: Some(critic),
                critic_window_ticks: window,
                ..Config::default()
            })
            .unwrap();
            for (unit, weight) in exec.units_mut().iter_mut().zip([100i16, -40, 7, 0]) {
                unit.v_thresh = THRESHOLD_BASE;
                unit.stp_u_rel = STP_U;
                unit.stp_r_ves = STP_MAX;
                unit.value_weight = weight;
            }
            let mut openings = Vec::new();
            let mut readings = Vec::new();
            for (pattern, &reward) in kicks.iter().zip(&rewards) {
                let opened = exec.window_opened();
                assert_eq!(
                    opened,
                    exec.ticks(),
                    "the window opens where the last one closed"
                );
                openings.push(opened);
                for since in 0..window_ticks {
                    for &(at, units) in *pattern {
                        if at == since {
                            let inject = exec.injector();
                            for &unit in units {
                                for _ in 0..14 {
                                    inject.inject(unit, strong).unwrap();
                                }
                            }
                        }
                    }
                    exec.tick();
                }
                let features = exec.features().to_vec();
                exec.reward(reward);
                let weights = exec.units().iter().map(|u| u.value_weight).collect();
                readings.push((features, exec.prediction().unwrap(), weights));
                assert_eq!(
                    exec.window_opened(),
                    exec.ticks(),
                    "open again at the reward"
                );
            }
            assert_eq!(exec.train_overwritten(), 0, "the ring holds every spike");
            (exec.train().to_vec(), openings, readings)
        };
        let (train, openings, unset) = run(0);
        assert_eq!(
            openings,
            [0, 400, 800, 1200],
            "the start, then each reward's tick"
        );
        // The distances of the spikes from their window's opening, as the unset run reads them.
        let distance = |tick: u32| {
            let opened = openings
                .iter()
                .rev()
                .find(|&&o| o <= u64::from(tick))
                .unwrap();
            u64::from(tick) - opened
        };
        let mut distances: Vec<u64> = train.iter().map(|&(tick, _)| distance(tick)).collect();
        distances.sort_unstable();
        distances.dedup();
        assert!(
            distances.len() >= 4,
            "spikes at several distances: {distances:?}"
        );
        let oracle = |window: u64| -> Vec<(Vec<u32>, Prediction, Vec<i16>)> {
            let mut weights = [100i64, -40, 7, 0];
            (0..openings.len())
                .map(|k| {
                    let mut counts = vec![0u32; 4];
                    for &(tick, unit) in &train {
                        let opened = openings[k];
                        let t = u64::from(tick);
                        if t >= opened
                            && t < opened + window_ticks
                            && (window == 0 || t - opened < window)
                        {
                            counts[unit as usize] += 1;
                        }
                    }
                    let exact: i64 = weights
                        .iter()
                        .zip(&counts)
                        .map(|(&w, &c)| w * i64::from(c))
                        .sum();
                    let value = exact.div_euclid(4) as i32;
                    let error = (i64::from(rewards[k]) - i64::from(value))
                        .clamp(i64::from(i32::MIN), i64::from(i32::MAX))
                        as i32;
                    for (w, &c) in weights.iter_mut().zip(&counts) {
                        *w = (*w + (i64::from(error) * i64::from(c)).div_euclid(512))
                            .clamp(i64::from(i16::MIN), i64::from(i16::MAX));
                    }
                    (
                        counts,
                        Prediction {
                            value_q16: value,
                            error_q16: error,
                            received_q16: error,
                        },
                        weights.iter().map(|&w| w as i16).collect(),
                    )
                })
                .collect()
        };
        assert_eq!(
            unset,
            oracle(0),
            "unset, every spike since the reward: ADR-0131's"
        );
        let mut lengths: Vec<u64> = distances
            .iter()
            .flat_map(|&d| [d, d + 1])
            .filter(|&w| w >= 1)
            .collect();
        lengths.dedup();
        let mut excluded_some = false;
        for &window in &lengths {
            let (again, opened_again, windowed) = run(window as u16);
            assert_eq!(
                (again, opened_again),
                (train.clone(), openings.clone()),
                "{window}: the window changes no spike"
            );
            assert_eq!(windowed, oracle(window), "window {window}");
            excluded_some |= windowed.iter().zip(&unset).any(|(w, u)| w.0 != u.0);
        }
        assert!(excluded_some, "some length leaves a spike out");
    }

    /// The whole punishment (ADR-0155): unset by default; refused while the critic is unset;
    /// taken with the critic, with its window or without it.
    #[test]
    fn the_whole_punishment_is_unset_by_default_and_refused_without_the_critic() {
        assert!(!Config::default().whole_punishment, "unset by default");
        let with = |critic, window, whole| {
            Executor::<8>::new(Config {
                units: 3,
                train_capacity: 8,
                critic,
                critic_window_ticks: window,
                whole_punishment: whole,
                ..Config::default()
            })
        };
        assert!(!with(None, 0, false).unwrap().whole_punishment());
        let critic = Some(ValueCritic { shift: 9, scale: 2 });
        for window in [0, 100] {
            assert!(
                !with(critic, window, false).unwrap().whole_punishment(),
                "{window}: unset with the critic"
            );
            assert!(
                with(critic, window, true).unwrap().whole_punishment(),
                "{window}: taken with the critic"
            );
        }
        assert!(
            matches!(
                with(None, 0, true),
                Err(ConfigError::WholePunishmentWithoutCritic)
            ),
            "refused without the critic"
        );
    }

    /// The rate's target as a window's count (ADR-0180): unset at a period of zero; the units
    /// times the window's 131 072 ticks over the period, a half going up; refused at no spike
    /// and above what a window's bins count. H-38's period, 58 201 ticks, at the three sizes
    /// the tree runs, and the other networks' periods at theirs.
    #[test]
    fn the_rate_s_target_is_a_window_s_count_of_its_period_and_refused_outside_what_a_window_counts()
     {
        assert_eq!(WINDOW_CADENCE.period(), 131_072);
        assert_eq!(RATE_TARGET_MAX, 32 * 0x00FF_FFFF);
        for units in [1, 64, 1024, usize::MAX] {
            assert_eq!(rate_target_of(units, 0), Ok(None), "{units}: unset");
        }
        // 256 x 131 072 / 58 201 is 576.53, 1 024 times it 2 306.11 and 2 048 times it 4 612.21.
        assert_eq!(rate_target_of(256, 58_201), Ok(Some(577)));
        assert_eq!(rate_target_of(1024, 58_201), Ok(Some(2_306)));
        assert_eq!(rate_target_of(2048, 58_201), Ok(Some(4_612)));
        // Seeds 23 and 24 at 1 024 units, and seeds 22 and 23 at 2 048 (ADR-0167, ADR-0169).
        assert_eq!(rate_target_of(1024, 59_510), Ok(Some(2_255)));
        assert_eq!(rate_target_of(1024, 59_179), Ok(Some(2_268)));
        assert_eq!(rate_target_of(2048, 59_599), Ok(Some(4_504)));
        assert_eq!(rate_target_of(2048, 59_119), Ok(Some(4_541)));
        // A unit a tick is the window's ticks; a unit a window is one.
        assert_eq!(rate_target_of(1, 1), Ok(Some(131_072)));
        assert_eq!(rate_target_of(1, 131_072), Ok(Some(1)));
        assert_eq!(rate_target_of(3, 131_072), Ok(Some(3)));
        // A half goes up: one unit over two windows is half a spike, which is one; a tick
        // longer is no spike, and refused. Three units over two windows are one and a half.
        assert_eq!(rate_target_of(1, 262_144), Ok(Some(1)));
        assert_eq!(
            rate_target_of(1, 262_145),
            Err(ConfigError::RateTargetOutOfRange)
        );
        assert_eq!(rate_target_of(3, 262_144), Ok(Some(2)));
        assert_eq!(rate_target_of(1, 196_608), Ok(Some(1)), "two thirds");
        assert_eq!(rate_target_of(4, 196_608), Ok(Some(3)), "2.67");
        assert_eq!(
            rate_target_of(1, u32::MAX),
            Err(ConfigError::RateTargetOutOfRange)
        );
        assert_eq!(
            rate_target_of(16_384, u32::MAX),
            Ok(Some(1)),
            "exactly a half"
        );
        assert_eq!(
            rate_target_of(16_383, u32::MAX),
            Err(ConfigError::RateTargetOutOfRange)
        );
        // What a window's bins count: 4 095 units a tick fit it, 4 096 do not.
        assert_eq!(rate_target_of(4_095, 1), Ok(Some(536_739_840)));
        assert_eq!(
            rate_target_of(4_096, 1),
            Err(ConfigError::RateTargetOutOfRange)
        );
        assert_eq!(rate_target_of(8_191, 2), Ok(Some(536_805_376)));
        assert_eq!(
            rate_target_of(8_192, 2),
            Err(ConfigError::RateTargetOutOfRange)
        );
        // A count the width does not hold, and one the product saturates at.
        assert_eq!(
            rate_target_of(1 << 20, 1),
            Err(ConfigError::RateTargetOutOfRange)
        );
        assert_eq!(
            rate_target_of(usize::MAX, 1),
            Err(ConfigError::RateTargetOutOfRange)
        );
    }

    /// The rate's target in the configuration (ADR-0180): unset by default, where the engine
    /// holds no count; set, the engine holds the period and the window's count of it for its
    /// units; refused where the count is no spike or more than a window counts.
    #[test]
    fn the_rate_s_target_is_unset_by_default_and_the_engine_holds_its_count() {
        assert_eq!(Config::default().rate_target_period_ticks, 0);
        let with = |units, period| {
            Executor::<8>::new(Config {
                units,
                rate_target_period_ticks: period,
                ..Config::default()
            })
        };
        let unset = with(3, 0).unwrap();
        assert_eq!(
            (
                unset.rate_target_period_ticks(),
                unset.rate_target_per_window()
            ),
            (0, None)
        );
        let set = with(3, 1_000).unwrap();
        assert_eq!(
            (set.rate_target_period_ticks(), set.rate_target_per_window()),
            (1_000, Some(393)),
            "3 x 131 072 / 1 000 is 393.2"
        );
        let one = with(1, 262_144).unwrap();
        assert_eq!(one.rate_target_per_window(), Some(1));
        assert!(matches!(
            with(1, 262_145),
            Err(ConfigError::RateTargetOutOfRange)
        ));
        assert!(matches!(
            with(4_096, 1),
            Err(ConfigError::RateTargetOutOfRange)
        ));
        assert_eq!(
            with(4_095, 1).unwrap().rate_target_per_window(),
            Some(536_739_840)
        );
    }

    /// The target a read makes of its sum (ADR-0189): the mean of sixteen windows, a half going
    /// up; the period of that mean, a half going up; and the period's count. The three networks'
    /// derived counts at 1 024 units, each sixteen times; the least sum that is a spike; the
    /// period at the width's edge; the count of a period apart from its mean only past 11 601 a
    /// window at 1 024 units, and at one unit past 374; refused at no spike and at a period of
    /// no tick or past the width.
    #[test]
    fn a_read_s_target_is_its_mean_s_period_and_that_period_s_count() {
        let n = 1024usize;
        // 16 x 2 306: the period 134 217 728 / 2 306 = 58 203.70, and back to 2 306.
        assert_eq!(read_target_of(n, 16 * 2_306), Ok((58_204, 2_306)));
        assert_eq!(read_target_of(n, 16 * 2_255), Ok((59_520, 2_255)));
        assert_eq!(read_target_of(n, 16 * 2_268), Ok((59_179, 2_268)));
        // The mean to nearest, a half up: seven over sixteen is no spike, eight is one; 23 is
        // one, 24 two.
        assert_eq!(read_target_of(n, 0), Err(ConfigError::RateTargetOutOfRange));
        assert_eq!(read_target_of(n, 7), Err(ConfigError::RateTargetOutOfRange));
        assert_eq!(read_target_of(n, 8), Ok((134_217_728, 1)));
        assert_eq!(read_target_of(n, 23), Ok((134_217_728, 1)));
        assert_eq!(read_target_of(n, 24), Ok((67_108_864, 2)));
        // At 64 units, the target of `tests/criticality.rs`: 5 127 is a mean of 320, 5 128 of
        // 321, whose period is 26 133.
        assert_eq!(read_target_of(64, 16 * 320), Ok((26_214, 320)));
        assert_eq!(read_target_of(64, 16 * 320 + 7), Ok((26_214, 320)));
        assert_eq!(read_target_of(64, 16 * 320 + 8), Ok((26_133, 321)));
        // A spike a window's period on 32 767 units fits the width; on 32 768 it does not.
        assert_eq!(read_target_of(32_767, 8), Ok((4_294_836_224, 1)));
        assert_eq!(
            read_target_of(32_768, 8),
            Err(ConfigError::RateTargetOutOfRange)
        );
        // The period's count is the mean's to 11 601 a window at 1 024 units, and from 11 602 one
        // above it; at one unit, to 374, a mean of 375 a count of 374.
        assert_eq!(read_target_of(n, 16 * 11_601).map(|t| t.1), Ok(11_601));
        assert_eq!(read_target_of(n, 16 * 11_602), Ok((11_568, 11_603)));
        assert_eq!(read_target_of(1, 16 * 374).map(|t| t.1), Ok(374));
        assert_eq!(read_target_of(1, 16 * 375), Ok((350, 374)));
        // A unit a tick is a period of one; twice that is a period of a half, one; past it no
        // tick, refused.
        assert_eq!(read_target_of(1, 16 * 131_072), Ok((1, 131_072)));
        assert_eq!(read_target_of(1, 16 * 262_144 + 7), Ok((1, 131_072)));
        assert_eq!(
            read_target_of(1, 16 * 262_144 + 8),
            Err(ConfigError::RateTargetOutOfRange)
        );
        // The widest sum a read holds, at the cap in all sixteen windows, saturates nothing.
        assert_eq!(read_sum_max(0), 16 * u64::from(RATE_TARGET_MAX));
        assert_eq!(
            read_target_of(4_095, read_sum_max(0)),
            Ok((1, 536_739_840)),
            "a mean at the cap, a period of a tick, its count what 4 095 units make"
        );
        assert_eq!(
            read_target_of(usize::MAX, 16),
            Err(ConfigError::RateTargetOutOfRange)
        );
    }

    /// A window's close under a read (ADR-0189): the windows left one fewer and the count summed
    /// only where the window is one of the last sixteen — at sixteen left and under, not at
    /// seventeen — and at the last window the target of the sum, this window's count among it,
    /// or the refusal; a `left` of zero is the last. What a read holds is bounded by the windows
    /// of its last sixteen already closed.
    #[test]
    fn a_read_sums_its_last_sixteen_windows_and_makes_its_target_at_its_last() {
        let n = 1024usize;
        assert_eq!(
            read_step(128, 0, 2_000, n, false),
            ReadStep::Reading { left: 127, sum: 0 }
        );
        assert_eq!(
            read_step(17, 0, 2_000, n, false),
            ReadStep::Reading { left: 16, sum: 0 }
        );
        assert_eq!(
            read_step(16, 0, 2_000, n, false),
            ReadStep::Reading {
                left: 15,
                sum: 2_000
            }
        );
        assert_eq!(
            read_step(2, 14 * 2_306, 2_306, n, false),
            ReadStep::Reading {
                left: 1,
                sum: 15 * 2_306
            }
        );
        assert_eq!(
            read_step(1, 15 * 2_306, 2_306, n, false),
            ReadStep::Target {
                period: 58_204,
                count: 2_306
            }
        );
        assert_eq!(
            read_step(1, 15 * 2_306, 2_306 + 7, n, false),
            ReadStep::Target {
                period: 58_204,
                count: 2_306
            },
            "less than half a spike in the mean"
        );
        assert_eq!(
            read_step(1, 15 * 2_306, 2_306 + 8, n, false),
            ReadStep::Target {
                period: 58_178,
                count: 2_307
            },
            "a half goes up"
        );
        assert_eq!(read_step(1, 0, 7, n, false), ReadStep::Refused, "no spike");
        assert_eq!(
            read_step(0, 15 * 2_306, 2_306, n, false),
            read_step(1, 15 * 2_306, 2_306, n, false)
        );
        // The bound the loader holds a read to.
        assert_eq!(read_sum_max(255), 0);
        assert_eq!(read_sum_max(17), 0);
        assert_eq!(read_sum_max(16), 0);
        assert_eq!(read_sum_max(15), u64::from(RATE_TARGET_MAX));
        assert_eq!(read_sum_max(1), 15 * u64::from(RATE_TARGET_MAX));
    }

    /// The read in the configuration (ADR-0189): unset by default; set at sixteen windows and
    /// above, the engine's read under way with the span and nothing summed; refused below
    /// sixteen and beside a period, after the period's own refusal.
    #[test]
    fn the_read_is_unset_by_default_and_refused_short_of_sixteen_windows_and_beside_a_period() {
        assert_eq!(Config::default().rate_read_windows, 0);
        let with = |windows, period| {
            Executor::<8>::new(Config {
                units: 1024,
                rate_read_windows: windows,
                rate_target_period_ticks: period,
                ..Config::default()
            })
        };
        let unset = with(0, 0).unwrap();
        assert_eq!(unset.target_read(), None);
        let beside_none = with(0, 58_201).unwrap();
        assert_eq!(beside_none.target_read(), None);
        for windows in [16u8, 17, 128, u8::MAX] {
            let set = with(windows, 0).unwrap();
            assert_eq!(
                (
                    set.target_read(),
                    set.rate_target_period_ticks(),
                    set.rate_target_per_window()
                ),
                (
                    Some(TargetRead::Reading {
                        left: windows,
                        sum: 0
                    }),
                    0,
                    None
                ),
                "{windows} windows"
            );
        }
        for windows in [1u8, 15] {
            assert!(
                matches!(with(windows, 0), Err(ConfigError::RateReadOutOfRange)),
                "{windows}"
            );
        }
        assert!(matches!(
            with(128, 58_201),
            Err(ConfigError::RateReadBesideTarget)
        ));
        assert!(matches!(
            with(15, 58_201),
            Err(ConfigError::RateReadOutOfRange)
        ));
        assert!(matches!(
            with(128, u32::MAX),
            Err(ConfigError::RateTargetOutOfRange)
        ));
    }

    /// A read that sets the inhibitory rule's target as well (ADR-0194): its last window's target
    /// is the read's without the flag wherever that rule accepts the period, and refused where it
    /// does not — at 1 024 units a mean of 135 a window is a period of 994 205 and 134 one of
    /// 1 001 625; the rule's widest period, 1 000 000, a mean of 2 048 at 15 625 units, is
    /// accepted and the next mean down, a period of 1 000 489, refused; its shortest, 100, a mean
    /// of 32 768 at 25 units, accepted and 99 refused. A window before the last, and a read the
    /// rate's target refuses, are the read's either way.
    #[test]
    fn a_read_that_sets_the_inhibitory_target_refuses_a_period_that_rule_refuses() {
        // The last window of a read whose sixteen windows were `mean` each.
        let last = |units: usize, mean: u64, inhibitory: bool| {
            read_step(1, 15 * mean, mean, units, inhibitory)
        };
        let target = |period, count| ReadStep::Target { period, count };
        for (units, mean, period, count) in [
            (1024usize, 2_330u64, 57_604u32, 2_330u32),
            (1024, 135, 994_205, 135),
            (15_625, 2_048, 1_000_000, 2_048),
            (25, 32_768, 100, 32_768),
        ] {
            assert_eq!(
                last(units, mean, false),
                target(period, count),
                "{units} at {mean}"
            );
            assert_eq!(
                last(units, mean, true),
                target(period, count),
                "{units} at {mean}"
            );
        }
        for (units, mean, period, count) in [
            (1024usize, 134u64, 1_001_625u32, 134u32),
            (15_625, 2_047, 1_000_489, 2_047),
            (25, 33_099, 99, 33_099),
        ] {
            assert_eq!(
                last(units, mean, false),
                target(period, count),
                "{units} at {mean}"
            );
            assert_eq!(
                last(units, mean, true),
                ReadStep::Refused,
                "{units} at {mean}"
            );
        }
        assert_eq!(last(1024, 0, true), ReadStep::Refused, "no spike");
        assert_eq!(
            read_step(16, 0, 2_330, 1024, true),
            read_step(16, 0, 2_330, 1024, false),
            "a window before the last"
        );
        assert_eq!(
            read_step(0, 15 * 135, 135, 1024, true),
            last(1024, 135, true),
            "a left of zero is the last"
        );
    }

    /// The inhibitory target by the read in the configuration (ADR-0194): unset by default; set
    /// beside a read, the engine's flag set and the inhibitory period the configuration's until
    /// the read ends; refused without a read, after the read's own refusals.
    #[test]
    fn the_inhibitory_target_by_the_read_is_unset_by_default_and_refused_without_a_read() {
        assert!(!Config::default().istdp_target_read);
        let with = |windows, period, flag| {
            Executor::<8>::new(Config {
                units: 1024,
                rate_read_windows: windows,
                rate_target_period_ticks: period,
                istdp_target_read: flag,
                ..Config::default()
            })
        };
        let unset = with(128, 0, false).unwrap();
        assert!(!unset.istdp_target_read());
        let set = with(128, 0, true).unwrap();
        assert_eq!(
            (
                set.istdp_target_read(),
                set.target_read(),
                set.istdp_target_period_ticks()
            ),
            (
                true,
                Some(TargetRead::Reading { left: 128, sum: 0 }),
                ISTDP_TARGET_PERIOD_TICKS
            )
        );
        for period in [0u32, 58_201] {
            assert!(
                matches!(
                    with(0, period, true),
                    Err(ConfigError::IstdpReadWithoutRead)
                ),
                "no read, a period of {period}"
            );
        }
        assert!(matches!(
            with(15, 0, true),
            Err(ConfigError::RateReadOutOfRange)
        ));
        assert!(matches!(
            with(128, 58_201, true),
            Err(ConfigError::RateReadBesideTarget)
        ));
    }

    /// The probe's gain (ADR-0202), each number from an oracle in wider integers written from
    /// ADR-0200's text: the record's moved a sixty-fourth by the window its windows left name —
    /// down in the first four and the last four, up in the eight between, and down at any other
    /// count — to nearest with a half going up. At 1.75 the two are 1.75 x 63/64 and 1.75 x 65/64
    /// exactly; 16 416 x 63/64 is 16 159.5 and x 65/64 16 672.5, each going up, and one LSB either
    /// side of it goes to nearest; at the controller's bounds the product passes them by a
    /// sixty-fourth of them.
    #[test]
    fn the_probe_s_gain_is_the_record_s_moved_a_sixty_fourth_by_its_window() {
        let g = 0x0001_C000u32;
        for left in [16u8, 15, 14, 13, 4, 3, 2, 1, 0, 17, u8::MAX] {
            assert!(!probe_raises(left), "{left} left");
            assert_eq!(probe_gain_q16(g, left), 112_896, "{left} left");
        }
        for left in 5u8..=12 {
            assert!(probe_raises(left), "{left} left");
            assert_eq!(probe_gain_q16(g, left), 116_480, "{left} left");
        }
        for (gain, down, up) in [
            (0x4020u32, 16_160u32, 16_673u32),
            (0x401F, 16_159, 16_671),
            (0x4021, 16_160, 16_674),
            (GAIN_ONE_Q16, 64_512, 66_560),
            (GAIN_MIN_Q16, 16_128, 16_640),
            (GAIN_MAX_Q16, 258_048, 266_240),
        ] {
            assert_eq!(
                (probe_gain_q16(gain, 16), probe_gain_q16(gain, 12)),
                (down, up),
                "{gain:#x}"
            );
        }
    }

    /// A probe's windows and its step (ADR-0200, ADR-0202), each number by hand: a window's count
    /// is taken from the sum in the probe's first four windows and its last four and added in the
    /// eight between; at the last window the step is $4\,096\,c^{*}/D$ to nearest, a half going
    /// up — at $c^{*} = 2\,306$ and $D = 9\,224$, a steepness of 16, a sixty-fourth. Refused at a
    /// sum of zero and below; at the bound, 0x8000, a step, and one past it refused; a target of
    /// no spike a step of zero. A `left` of zero is the last, as one is. The bounds the loader
    /// holds a probe's sum to, by the windows closed so far.
    #[test]
    fn a_probe_sums_its_higher_windows_less_its_lower_and_makes_its_step_at_its_last() {
        let c = 2_306u32;
        let probing = |left, sum| ProbeStep::Probing { left, sum };
        assert_eq!(probe_step(16, 0, 1_730, c), probing(15, -1_730));
        assert_eq!(probe_step(13, -5_190, 1_733, c), probing(12, -6_923));
        assert_eq!(probe_step(12, -6_923, 2_887, c), probing(11, -4_036));
        assert_eq!(probe_step(5, 16_183, 2_894, c), probing(4, 19_077));
        assert_eq!(probe_step(4, 19_077, 1_742, c), probing(3, 17_335));
        assert_eq!(probe_step(2, 10_969, 1_744, c), probing(1, 9_225));
        assert_eq!(probe_step(1, 9_225 + 1, 2, c), ProbeStep::Step(1_024));
        assert_eq!(probe_step(0, 9_226, 2, c), probe_step(1, 9_226, 2, c));
        // The sum at the last window: zero and below refused, the bound a step, one past refused.
        assert_eq!(probe_step(1, 2_000, 2_000, c), ProbeStep::Refused, "D = 0");
        assert_eq!(probe_step(1, 0, 1, c), ProbeStep::Refused, "D = -1");
        assert_eq!(
            probe_step(1, i64::MIN, 1, c),
            ProbeStep::Refused,
            "the width"
        );
        for (count, sum, step) in [
            (2_306u32, 9_224i64, Some(1_024u16)),
            (2_306, 289, Some(32_683)),
            (2_306, 288, None),
            (8, 1, Some(0x8000)),
            (32_768, 4_096, Some(0x8000)),
            (32_769, 4_096, None),
            (320, 40, Some(0x8000)),
            (320, 39, None),
            (320, 320, Some(0x1000)),
            (3, 8_192, Some(2)),
            (3, 8_193, Some(1)),
            (0, 1, Some(0)),
            (2_306, 0, None),
            (2_306, -9_224, None),
        ] {
            assert_eq!(probe_step_of(count, sum), step, "{count} over {sum}");
        }
        // The loader's bounds, by the windows closed so far: none, the first four lower, then
        // the eight higher, then the last four lower.
        let m = i64::from(RATE_TARGET_MAX);
        for (left, bounds) in [
            (17u8, (0i64, 0i64)),
            (16, (0, 0)),
            (15, (-m, 0)),
            (12, (-2_147_483_520, 0)),
            (11, (-2_147_483_520, m)),
            (4, (-2_147_483_520, 4_294_967_040)),
            (3, (-2_684_354_400, 4_294_967_040)),
            (1, (-3_758_096_160, 4_294_967_040)),
            (0, (-4_294_967_040, 4_294_967_040)),
        ] {
            assert_eq!(probe_sum_bounds(left), bounds, "{left} left");
        }
    }

    /// The probe of the step in the configuration (ADR-0202): unset by default — the engine's
    /// default sets it since ADR-0205, held by that default's own test field by field; set beside
    /// a read, the engine's flag set beside the read under way and the workers given the record's
    /// gain; refused without a read, after the read's own refusals and the inhibitory flag's.
    #[test]
    fn the_probe_is_unset_by_default_and_refused_without_a_read() {
        assert!(!Config::default().step_probe);
        let with = |windows, period, inhibitory, probe| {
            Executor::<8>::new(Config {
                units: 1024,
                synaptic_gain_q16: 0x0001_C000,
                rate_read_windows: windows,
                rate_target_period_ticks: period,
                istdp_target_read: inhibitory,
                step_probe: probe,
                ..Config::default()
            })
        };
        assert!(!with(128, 0, true, false).unwrap().step_probe());
        let set = with(128, 0, true, true).unwrap();
        assert_eq!(
            (
                set.step_probe(),
                set.istdp_target_read(),
                set.target_read(),
                set.gain_given_q16()
            ),
            (
                true,
                true,
                Some(TargetRead::Reading { left: 128, sum: 0 }),
                0x0001_C000
            )
        );
        for period in [0u32, 58_201] {
            assert!(
                matches!(
                    with(0, period, false, true),
                    Err(ConfigError::StepProbeWithoutRead)
                ),
                "no read, a period of {period}"
            );
        }
        assert!(matches!(
            with(0, 0, true, true),
            Err(ConfigError::IstdpReadWithoutRead)
        ));
        assert!(matches!(
            with(15, 0, false, true),
            Err(ConfigError::RateReadOutOfRange)
        ));
        assert!(matches!(
            with(128, 58_201, false, true),
            Err(ConfigError::RateReadBesideTarget)
        ));
    }

    /// A window's close as `tally` makes it, with the record's open window holding `spikes` in
    /// one bin: what `read_close` reads of it under the read or the probe.
    fn close_holding(exec: &mut Executor<8>, spikes: u32) {
        exec.homeostasis.window_bins = 1;
        exec.homeostasis.first_activity = spikes;
        exec.homeostasis.last_activity = spikes;
        exec.homeostasis.sum_prev = 0;
        let read = exec.target_read.expect("a read or a probe under way");
        exec.read_close(read);
    }

    /// The probe as the engine runs it (ADR-0200, ADR-0202), window by window through
    /// `read_close`, each number by hand. A read of sixteen windows of 2 306 ends in its target,
    /// the period 58 204, and with the probe's flag the probe begins in the rule's place: sixteen
    /// windows left, nothing summed, the step the configuration's, the record's gain 1.75 and the
    /// workers given 1.75 x 63/64. Through the probe's windows — 1 730 and up by one in the four
    /// lower, 2 883 and up by one in the eight higher, then 1 742 and up — the sum is the higher
    /// less the lower, the drift cancelling to D = 9 224, the record's gain and step held and the
    /// workers' gain the window's; at the last the step becomes 1 024, a sixty-fourth, the flag is
    /// spent and the workers are given the record's gain again. Without the flag the read ends in
    /// the same target and no probe. A probe whose sum is zero ends refused: the gain held from
    /// there on, the target kept as a reading, the step the configuration's, the flag spent, and
    /// its image refused. A read refused spends the probe's flag, and no probe begins.
    #[test]
    fn a_probe_begins_at_the_read_s_end_and_sets_the_step_at_its_last_window_or_ends_refused() {
        let built = |probe: bool| {
            let mut exec = Executor::<8>::new(Config {
                units: 1024,
                synaptic_gain_q16: 0x0001_C000,
                control_step_q0_16: 0x1000,
                rate_read_windows: 16,
                step_probe: probe,
                ..Config::default()
            })
            .unwrap();
            exec.target_read = Some(TargetRead::Reading {
                left: 1,
                sum: 15 * 2_306,
            });
            close_holding(&mut exec, 2_306);
            exec
        };
        let unset = built(false);
        assert_eq!(
            (
                unset.target_read(),
                unset.rate_target_period_ticks(),
                unset.rate_target_per_window(),
                unset.step_probe()
            ),
            (None, 58_204, Some(2_306), false),
            "without the flag the rule from the next window"
        );
        let windows: Vec<(u32, bool)> = (0..16u32)
            .map(|k| match k {
                4..=11 => (k.wrapping_add(2_883), true),
                _ => (k.wrapping_add(1_730), false),
            })
            .collect();
        for (equal, ended) in [(false, None), (true, Some(TargetRead::Refused))] {
            let mut exec = built(true);
            assert_eq!(
                (
                    exec.target_read(),
                    exec.rate_target_period_ticks(),
                    exec.rate_target_per_window(),
                    exec.step_probe(),
                    exec.homeostasis().control_step_q0_16,
                    exec.gain_given_q16()
                ),
                (
                    Some(TargetRead::Probing { left: 16, sum: 0 }),
                    58_204,
                    Some(2_306),
                    true,
                    0x1000,
                    112_896
                ),
                "the probe in the rule's place"
            );
            let mut sum = 0i64;
            for (k, &(count, higher)) in windows.iter().enumerate() {
                let count = if equal { 2_306 } else { count };
                close_holding(&mut exec, count);
                sum = if higher {
                    sum.wrapping_add(i64::from(count))
                } else {
                    sum.wrapping_sub(i64::from(count))
                };
                let left = 15u8.wrapping_sub(k as u8);
                if left > 0 {
                    assert_eq!(
                        (
                            exec.target_read(),
                            exec.homeostasis().control_step_q0_16,
                            exec.homeostasis().synaptic_gain_q16,
                            exec.step_probe(),
                            exec.gain_given_q16()
                        ),
                        (
                            Some(TargetRead::Probing { left, sum }),
                            0x1000,
                            0x0001_C000,
                            true,
                            if (5..=12).contains(&left) {
                                116_480
                            } else {
                                112_896
                            }
                        ),
                        "window {k}"
                    );
                }
            }
            assert_eq!(sum, if equal { 0 } else { 9_224 }, "the drift cancelled");
            assert_eq!(
                (
                    exec.target_read(),
                    exec.homeostasis().control_step_q0_16,
                    exec.step_probe(),
                    exec.gain_given_q16(),
                    exec.rate_target_per_window()
                ),
                (
                    ended,
                    if equal { 0x1000 } else { 1_024 },
                    false,
                    0x0001_C000,
                    Some(2_306)
                ),
                "the probe's end"
            );
            if equal {
                assert!(matches!(
                    crate::Image::encode(&exec),
                    Err(crate::ImageError::Config(
                        ConfigError::ControlStepOutOfRange
                    ))
                ));
            }
        }
        // A read refused: no target, the probe's flag spent with it and no probe.
        let mut refused = built(true);
        refused.target_read = Some(TargetRead::Reading { left: 1, sum: 0 });
        refused.step_probe = true;
        close_holding(&mut refused, 0);
        assert_eq!(
            (refused.target_read(), refused.step_probe()),
            (Some(TargetRead::Refused), false)
        );
    }

    /// The gain the tick publishes to the workers is `gain_given_q16`'s (ADR-0202): on a silent
    /// engine of one unit set by hand to the probe's first window beside a target of one spike,
    /// the workers are given 1.0 x 63/64 at its first tick, and through the probe's sixteen
    /// windows each window's gain at its first tick and its last; the silent windows sum nothing,
    /// so the probe ends refused, the record's gain 1.0 given from there on and held.
    #[test]
    fn the_tick_gives_the_workers_the_probe_s_gain_through_its_windows() {
        let mut exec = Executor::<8>::new(Config {
            units: 1,
            control_step_q0_16: 0x2000,
            rate_read_windows: 16,
            step_probe: true,
            ..Config::default()
        })
        .unwrap();
        exec.target_read = Some(TargetRead::Probing { left: 16, sum: 0 });
        exec.rate_target = Some(1);
        let window = WINDOW_CADENCE.period();
        for left in (1..=16u8).rev() {
            let given = if (5..=12).contains(&left) {
                66_560
            } else {
                64_512
            };
            exec.tick();
            assert_eq!(
                exec.shared.gain.load(Ordering::Relaxed),
                given,
                "{left} left"
            );
            exec.run(window.wrapping_sub(2));
            assert_eq!(
                (
                    exec.shared.gain.load(Ordering::Relaxed),
                    exec.gain_given_q16()
                ),
                (given, given),
                "{left} left, the window's last tick to come"
            );
            exec.tick();
        }
        assert_eq!(exec.target_read(), Some(TargetRead::Refused));
        exec.run(window);
        assert_eq!(
            (
                exec.shared.gain.load(Ordering::Relaxed),
                exec.homeostasis().synaptic_gain_q16
            ),
            (GAIN_ONE_Q16, GAIN_ONE_Q16)
        );
    }

    /// The gain an engine starts with (ADR-0196): 1.0 by default, so that `Config::default()`
    /// builds the engine it built, its homeostasis record `HomeostaticDrivePool::new()`'s and the
    /// workers' gain 1.0; from either of the controller's bounds, 0.25 and 4, to the other, the
    /// record's gain and the workers' the configuration's, the step and the sleep shift beside it
    /// the configuration's; one LSB past either bound, zero and the width refused.
    #[test]
    fn the_gain_an_engine_starts_with_is_one_by_default_and_refused_outside_the_controller_s_bounds()
     {
        assert_eq!(Config::default().synaptic_gain_q16, 0x0001_0000);
        let bare = Executor::<8>::new(Config {
            units: 4,
            ..Config::default()
        })
        .unwrap();
        assert_eq!(bare.homeostasis(), &HomeostaticDrivePool::new());
        assert_eq!(bare.shared.gain.load(Ordering::Relaxed), 0x0001_0000);
        let with = |gain| {
            Executor::<8>::new(Config {
                units: 4,
                synaptic_gain_q16: gain,
                control_step_q0_16: 0x1000,
                sleep_shift: 3,
                ..Config::default()
            })
        };
        for gain in [
            0x4000u32,
            0x4001,
            0x0001_0000,
            0x0001_C000,
            0x0003_FFFF,
            0x0004_0000,
        ] {
            let exec = with(gain).unwrap();
            assert_eq!(
                *exec.homeostasis(),
                HomeostaticDrivePool {
                    control_step_q0_16: 0x1000,
                    synaptic_gain_q16: gain,
                    sleep_shift: 3,
                    ..HomeostaticDrivePool::new()
                },
                "{gain:#x}"
            );
            assert_eq!(exec.shared.gain.load(Ordering::Relaxed), gain, "{gain:#x}");
        }
        for gain in [0u32, 1, 0x3FFF, 0x0004_0001, u32::MAX] {
            assert!(
                matches!(with(gain), Err(ConfigError::GainOutOfRange)),
                "{gain:#x}"
            );
        }
    }

    /// The engine's default (ADR-0196, ADR-0205), field by field against `Config::default()`: the
    /// modulation at rest 0.5, the gain 1.75, the step a sixteenth, the read 128 windows, its
    /// flag set and the probe of the step set, each other than the bare engine's; and with those
    /// six put back, the bare engine, so that every other field — the sizing, the rate's target,
    /// the inhibitory period and the whole learning configuration — is `Config::default()`'s.
    #[test]
    fn reads_its_targets_is_the_default_with_adr_0196_s_five_values_the_probe_and_no_other() {
        let (default, bare) = (Config::reads_its_targets(), Config::default());
        assert_eq!(
            (
                default.modulation_baseline_q16,
                default.synaptic_gain_q16,
                default.control_step_q0_16,
                default.rate_read_windows,
                default.istdp_target_read
            ),
            (0x8000, 0x0001_C000, 0x1000, 128, true),
            "ADR-0196's five values"
        );
        assert!(default.step_probe, "ADR-0205's probe");
        assert_eq!(
            (
                bare.modulation_baseline_q16,
                bare.synaptic_gain_q16,
                bare.control_step_q0_16,
                bare.rate_read_windows,
                bare.istdp_target_read,
                bare.step_probe
            ),
            (0x0001_0000, 0x0001_0000, 0, 0, false, false),
            "the bare engine's, each other than the default's"
        );
        assert_eq!(
            Config {
                modulation_baseline_q16: bare.modulation_baseline_q16,
                synaptic_gain_q16: bare.synaptic_gain_q16,
                control_step_q0_16: bare.control_step_q0_16,
                rate_read_windows: bare.rate_read_windows,
                istdp_target_read: bare.istdp_target_read,
                step_probe: bare.step_probe,
                ..default.clone()
            },
            bare,
            "every other field the bare engine's"
        );
        assert_eq!(
            (
                default.rate_target_period_ticks,
                default.istdp_target_period_ticks,
                default.inhibitory_baseline_q16,
                default.signed_gate,
                default.critic,
                default.critic_window_ticks,
                default.whole_punishment
            ),
            (0, ISTDP_TARGET_PERIOD_TICKS, None, false, None, 0, false),
            "no target set, the inhibitory period the default's, the learning configuration unset"
        );
    }

    /// An engine born under the default (ADR-0196, ADR-0205): reading, 128 windows left and
    /// nothing summed, at the gain 1.75 and the step a sixteenth, its flag and the probe's set, no
    /// target and the inhibitory period the default's, at the modulation 0.5 with the learning
    /// configuration unset; its image, decoded under the bare engine's configuration, is the same
    /// engine, read back whole and written again byte for byte.
    #[test]
    fn an_engine_born_under_the_default_reads_at_1_75_with_its_flag_and_its_image_reads_back_alike()
    {
        type Readings = (
            Option<TargetRead>,
            bool,
            bool,
            u32,
            Option<u32>,
            u32,
            i32,
            HomeostaticDrivePool,
        );
        let readings = |exec: &Executor<8>| -> Readings {
            (
                exec.target_read(),
                exec.istdp_target_read(),
                exec.step_probe(),
                exec.rate_target_period_ticks(),
                exec.rate_target_per_window(),
                exec.istdp_target_period_ticks(),
                exec.modulation_baseline_q16(),
                *exec.homeostasis(),
            )
        };
        let born = Executor::<8>::new(Config {
            units: 1024,
            ..Config::reads_its_targets()
        })
        .unwrap();
        let expected: Readings = (
            Some(TargetRead::Reading { left: 128, sum: 0 }),
            true,
            true,
            0,
            None,
            ISTDP_TARGET_PERIOD_TICKS,
            0x8000,
            HomeostaticDrivePool {
                control_step_q0_16: 0x1000,
                synaptic_gain_q16: 0x0001_C000,
                ..HomeostaticDrivePool::new()
            },
        );
        assert_eq!(readings(&born), expected);
        assert_eq!(
            (
                born.inhibitory_baseline_q16(),
                born.signed_gate(),
                born.critic(),
                born.whole_punishment()
            ),
            (None, false, None, false),
            "the learning configuration unset"
        );
        let image = crate::Image::encode(&born).unwrap();
        let loaded = crate::Image::decode::<8>(&image, Config::default()).unwrap();
        assert_eq!(readings(&loaded), expected, "read back whole");
        assert_eq!(loaded.units().len(), 1024);
        assert!(
            crate::Image::encode(&loaded).unwrap() == image,
            "written again byte for byte"
        );
    }

    /// The whole punishment's composition (ADR-0155), each number from an oracle written from
    /// ADR-0154's text: two engines of four armed units, one with the parameter unset and one
    /// with it set, given the same kicks, the same weights written by hand before each window
    /// and the same rewards. At every reward of both the value is the weights times the counts,
    /// over four, floored; the prediction records it, the error — the reward less it — and what
    /// the modulator received; and every weight moves by the error times its count over 512,
    /// floored, in the two engines alike. Unset, the modulator receives the error at every
    /// reward, the critic of ADR-0131 bit for bit. Set, it receives the reward where the reward
    /// and the value are both below zero and the error everywhere else: the four cases by sign,
    /// a reward of zero, a value of zero, one LSB below zero on each side, and the width. Each
    /// engine's modulator is held to an oracle's, decayed tick by tick and given what the
    /// engine's rule says it receives.
    #[test]
    fn with_the_whole_punishment_set_a_punishment_under_a_value_below_zero_reaches_the_modulator_whole()
     {
        use menteion_core::{STP_MAX, STP_U, synaptic_efficacy_q16};
        let critic = ValueCritic { shift: 9, scale: 2 };
        let make = |whole| {
            let mut exec = Executor::<8>::new(Config {
                units: 4,
                nodes_per_worker: 128,
                train_capacity: 64,
                critic: Some(critic),
                whole_punishment: whole,
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
        let strong = spike_message(synaptic_efficacy_q16(i16::MAX, STP_U, STP_MAX), false);
        let weights = |exec: &Executor<8>| -> Vec<i16> {
            exec.units().iter().map(|u| u.value_weight).collect()
        };
        const R: i32 = 0x1_0000;
        // A case: the weights written before the window, the units kicked, the reward, and the
        // value's sign the weights are written for.
        let cases: [([i16; 4], &[u32], i32, i32); 13] = [
            // The four cases by sign.
            ([-20_000, -8, 5, 0], &[0, 1], -R, -1),
            ([20_000, 0, -5, 0], &[0], -R, 1),
            ([-20_000, 0, 5, -3], &[0, 3], R, -1),
            ([20_000, 8, -5, 0], &[0, 1], R, 1),
            // Zero on either side is not below zero.
            ([-20_000, 0, 0, 0], &[0], 0, -1),
            ([20_000, 0, 0, 0], &[0], 0, 1),
            ([20_000, -20_000, 9, -9], &[], -R, 0),
            ([0, 0, 0, 0], &[0, 1, 2, 3], -R, 0),
            // One LSB below zero on each side: the error is softer by the whole of it.
            ([-1, 0, 0, 0], &[0], -1, -1),
            ([8, 0, 0, 0], &[0], -1, 1),
            // The width, and beyond it.
            ([-20_000, 0, 0, 0], &[0], i32::MIN, -1),
            ([20_000, 0, 0, 0], &[0], i32::MIN, 1),
            ([-20_000, 0, 0, 0], &[0], i32::MAX, -1),
        ];
        let mut engines = [make(false), make(true)];
        let mut oracles = [NeuromodulatorState::new(); 2];
        let mut whole_cases = 0u32;
        for (case, &(written, kicked, reward, sign)) in cases.iter().enumerate() {
            for (exec, oracle) in engines.iter_mut().zip(oracles.iter_mut()) {
                // Armed again before each window, a thousand ticks long: a kick lands past the
                // refractory window of the last spike of the window before.
                for (unit, &weight) in exec.units_mut().iter_mut().zip(&written) {
                    unit.v_thresh = THRESHOLD_BASE;
                    unit.stp_u_rel = STP_U;
                    unit.stp_r_ves = STP_MAX;
                    unit.value_weight = weight;
                }
                let inject = exec.injector();
                for &unit in kicked {
                    for _ in 0..14 {
                        inject.inject(unit, strong).unwrap();
                    }
                }
                for _ in 0..1_000 {
                    exec.tick();
                    oracle.decay_dopamine(DOPAMINE_TAU_SHIFT);
                }
                assert_eq!(weights(exec), written, "case {case}: a tick moves none");
                assert_eq!(
                    exec.modulator(),
                    &*oracle,
                    "case {case}: decayed tick by tick"
                );
            }
            let counts = engines[0].features().to_vec();
            assert_eq!(engines[1].features(), &counts[..], "case {case}: one train");
            for (unit, &count) in counts.iter().enumerate() {
                assert_eq!(
                    count > 0,
                    kicked.contains(&(unit as u32)),
                    "case {case}: unit {unit} fired as kicked"
                );
            }
            let exact: i64 = written
                .iter()
                .zip(&counts)
                .map(|(&w, &c)| i64::from(w).saturating_mul(i64::from(c)))
                .sum();
            let value = exact.div_euclid(4) as i32;
            assert_eq!(
                value.signum(),
                sign,
                "case {case}: the value's sign, {value}"
            );
            let error = (i64::from(reward) - i64::from(value))
                .clamp(i64::from(i32::MIN), i64::from(i32::MAX)) as i32;
            // ADR-0154's rule, by hand: the reward where both are below zero.
            let whole = if reward < 0 && value < 0 {
                reward
            } else {
                error
            };
            assert_eq!(
                whole != error,
                reward < 0 && value < 0,
                "case {case}: softer exactly where both are below zero"
            );
            whole_cases = whole_cases.saturating_add(u32::from(whole != error));
            let moved: Vec<i16> = written
                .iter()
                .zip(&counts)
                .map(|(&w, &c)| {
                    (i64::from(w) + (i64::from(error) * i64::from(c)).div_euclid(512))
                        .clamp(i64::from(i16::MIN), i64::from(i16::MAX)) as i16
                })
                .collect();
            for ((exec, oracle), received) in engines
                .iter_mut()
                .zip(oracles.iter_mut())
                .zip([error, whole])
            {
                let set = exec.whole_punishment();
                assert_eq!(
                    exec.reward(reward),
                    oracle.reward(received),
                    "case {case} set {set}: the modulator receives {received}"
                );
                assert_eq!(exec.modulator(), &*oracle, "case {case} set {set}");
                assert_eq!(
                    exec.prediction(),
                    Some(Prediction {
                        value_q16: value,
                        error_q16: error,
                        received_q16: received
                    }),
                    "case {case} set {set}"
                );
                assert_eq!(
                    weights(exec),
                    moved,
                    "case {case} set {set}: the weights move by the error"
                );
                assert_eq!(
                    exec.features(),
                    &[0; 4],
                    "case {case} set {set}: counted afresh"
                );
            }
            if case == 0 {
                assert_ne!(
                    engines[0].modulator(),
                    engines[1].modulator(),
                    "the two engines' signals part at the first whole punishment"
                );
            }
        }
        assert_eq!(whole_cases, 3, "three of the cases meet both below zero");
    }

    /// The address drawn (ADR-0139): refused while the critic is unset and while its window is
    /// unset, the set standing as it was; with both set, the sources are exactly the units whose
    /// count is not zero — held to the train's spikes fewer ticks after the window's opening than
    /// its length, a unit that fired after the window closed not among them — and the targets
    /// exactly the units given, the arena's last unit taken and one past it refused whole, none
    /// at a tie; the drawing moves no count, no weight and no signal; after a reward every count
    /// is zero and the next drawing names no source; and `address_all` writes over it as over
    /// any set.
    #[test]
    fn the_address_drawn_is_the_units_the_window_counted_onto_the_units_given() {
        use menteion_core::{STP_MAX, STP_U, synaptic_efficacy_q16};
        let critic = Some(ValueCritic { shift: 9, scale: 2 });
        let make = |critic, window| {
            let mut exec = Executor::<8>::new(Config {
                units: 4,
                nodes_per_worker: 128,
                train_capacity: 64,
                critic,
                critic_window_ticks: window,
                ..Config::default()
            })
            .unwrap();
            for (unit, weight) in exec.units_mut().iter_mut().zip([100i16, -40, 7, 0]) {
                unit.v_thresh = THRESHOLD_BASE;
                unit.stp_u_rel = STP_U;
                unit.stp_r_ves = STP_MAX;
                unit.value_weight = weight;
            }
            exec
        };
        let sides = |exec: &Executor<8>| {
            (
                (0..5).map(|u| exec.is_source(u)).collect::<Vec<bool>>(),
                (0..5).map(|u| exec.is_target(u)).collect::<Vec<bool>>(),
            )
        };
        let every = (
            vec![true, true, true, true, false],
            vec![true, true, true, true, false],
        );
        // Refused without the critic, and with the critic and no window: the set as it was.
        for (critic, refusal) in [
            (None, AddressError::NoCritic),
            (critic, AddressError::NoWindow),
        ] {
            let mut exec = make(critic, 0);
            assert_eq!(exec.draws(), Err(refusal));
            assert_eq!(exec.address_drawn([1u32]), Err(refusal));
            assert_eq!(sides(&exec), every, "{refusal:?}: every unit, as at birth");
            exec.address([0u32], [1u32]).unwrap();
            assert_eq!(exec.address_drawn([2u32]), Err(refusal));
            assert_eq!(exec.addressed_counts(), (1, 1), "{refusal:?}: as it was");
        }
        // Both set: units 0 and 2 kicked at the window's opening, unit 1 after it closed.
        let window = 20u16;
        let mut exec = make(critic, window);
        assert_eq!(exec.draws(), Ok(()));
        let strong = spike_message(synaptic_efficacy_q16(i16::MAX, STP_U, STP_MAX), false);
        let kick = |exec: &Executor<8>, unit: u32| {
            let inject = exec.injector();
            for _ in 0..14 {
                inject.inject(unit, strong).unwrap();
            }
        };
        kick(&exec, 0);
        kick(&exec, 2);
        for t in 0..60 {
            if t == 30 {
                kick(&exec, 1);
            }
            exec.tick();
        }
        let opened = exec.window_opened();
        assert_eq!(opened, 0, "open at the engine's start");
        let counted: Vec<bool> = (0..5)
            .map(|u| {
                exec.train()
                    .iter()
                    .any(|&(tick, unit)| unit == u && u64::from(tick) - opened < u64::from(window))
            })
            .collect();
        assert_eq!(
            counted,
            [true, false, true, false, false],
            "the window counted 0 and 2"
        );
        assert!(
            exec.train().iter().any(|&(_, unit)| unit == 1),
            "unit 1 fired, after the window closed"
        );
        let features = exec.features().to_vec();
        assert_eq!(
            features.iter().map(|&c| c != 0).collect::<Vec<bool>>(),
            counted[..4],
            "the counts are the window's"
        );
        let (signal, weights) = (
            exec.modulator().dopamine_rpe,
            exec.units()
                .iter()
                .map(|u| u.value_weight)
                .collect::<Vec<i16>>(),
        );
        assert_eq!(exec.address_drawn([3u32, 1]), Ok(()));
        assert_eq!(
            sides(&exec),
            (counted.clone(), vec![false, true, false, true, false])
        );
        assert_eq!(exec.addressed_counts(), (2, 2));
        assert_eq!(
            (
                exec.features(),
                exec.modulator().dopamine_rpe,
                exec.units()
                    .iter()
                    .map(|u| u.value_weight)
                    .collect::<Vec<i16>>()
            ),
            (features.as_slice(), signal, weights),
            "the drawing moves no count, no signal and no weight"
        );
        // The arena's last unit taken as a target; one past it refused whole.
        assert_eq!(exec.address_drawn([3u32]), Ok(()));
        let last = (counted.clone(), vec![false, false, false, true, false]);
        assert_eq!(sides(&exec), last);
        assert_eq!(exec.address_drawn([0u32, 4]), Err(AddressError::NoSuchUnit));
        assert_eq!(sides(&exec), last, "refused whole, as it was");
        assert_eq!(exec.address_drawn(core::iter::empty()), Ok(()));
        assert_eq!(exec.addressed_counts(), (2, 0), "a tie: no target");
        // After a reward every count is zero, and the next drawing names no source.
        exec.reward(0x1_0000);
        assert_eq!(exec.features(), &[0; 4]);
        assert_eq!(exec.address_drawn([0u32, 1, 2, 3]), Ok(()));
        assert_eq!(
            sides(&exec),
            (vec![false; 5], vec![true, true, true, true, false])
        );
        exec.address_all();
        assert_eq!(sides(&exec), every);
    }

    /// The kept addresses (ADR-0208): unset by default, where no address is kept and both calls
    /// are refused; taken from one to four with the critic and its window, and refused above
    /// four and without either.
    #[test]
    fn the_kept_addresses_are_unset_by_default_and_refused_above_four_or_without_the_window() {
        assert_eq!(Config::default().kept_addresses, 0, "unset by default");
        assert_eq!(KEPT_ADDRESSES_MAX, 4);
        let critic = Some(ValueCritic { shift: 9, scale: 2 });
        let with = |kept, critic, window| {
            Executor::<8>::new(Config {
                units: 3,
                train_capacity: 8,
                critic,
                critic_window_ticks: window,
                kept_addresses: kept,
                ..Config::default()
            })
        };
        for (critic, window) in [(None, 0), (critic, 0), (critic, 20)] {
            let mut exec = with(0, critic, window).unwrap();
            assert_eq!(
                (exec.kept_addresses(), exec.kept_at(0), exec.kept_let_go()),
                (0, None, 0)
            );
            let refusal = exec.draws().err().unwrap_or(AddressError::NotKept);
            assert_eq!(exec.keep_address([0u32]), Err(refusal), "{refusal:?}");
            assert_eq!(exec.reward_kept(0x1_0000, 0), Err(refusal), "{refusal:?}");
            assert_eq!(exec.modulator().dopamine_rpe, 0, "{refusal:?}: no reward");
        }
        for kept in 1..=KEPT_ADDRESSES_MAX {
            let exec = with(kept, critic, 20).unwrap();
            assert_eq!(exec.kept_addresses(), kept);
            assert!(
                (0..=kept.saturating_add(1)).all(|age| exec.kept_at(age).is_none()),
                "{kept}: nothing kept before a close"
            );
        }
        for kept in [5, u8::MAX] {
            for (critic, window) in [(critic, 20), (None, 0)] {
                assert!(
                    matches!(
                        with(kept, critic, window),
                        Err(ConfigError::KeptAddressesOutOfRange)
                    ),
                    "{kept}"
                );
            }
        }
        for (critic, window) in [(None, 0), (critic, 0)] {
            assert!(matches!(
                with(1, critic, window),
                Err(ConfigError::KeptAddressesWithoutWindow)
            ));
        }
        assert!(matches!(
            with(1, None, 20),
            Err(ConfigError::WindowWithoutCritic)
        ));
    }

    /// A trial's close and a reward of an age (ADR-0208), each number from an oracle written from
    /// the rule's text, on four armed units under a critic whose value is the weights times the
    /// counts over four and whose weights move by the error times the count over 512. Before a
    /// close nothing is kept, and a reward of an age is refused, above two as never kept. A close
    /// keeps the window's counts and the targets given as the newest entry, zeroes the counts,
    /// opens the window again and touches no address, weight or signal; a target outside the
    /// arena is refused whole. A reward of age two after three closes writes the first close's
    /// address — its counted units onto its targets — reads the value of its counts, moves the
    /// weights by the error times them, sends the error, spends the entry and leaves the live
    /// counts and the window as they were; a second reward of that age is refused. A close that
    /// writes over a spent entry lets nothing go, and one that writes over an entry whose reward
    /// is still to come counts it.
    #[test]
    fn a_close_keeps_its_counts_and_targets_and_a_reward_of_its_age_writes_and_spends_them() {
        use menteion_core::{STP_MAX, STP_U, synaptic_efficacy_q16};
        let critic = ValueCritic { shift: 9, scale: 2 };
        let mut exec = Executor::<8>::new(Config {
            units: 4,
            nodes_per_worker: 128,
            train_capacity: 64,
            critic: Some(critic),
            critic_window_ticks: 20,
            kept_addresses: 2,
            ..Config::default()
        })
        .unwrap();
        for (unit, weight) in exec.units_mut().iter_mut().zip([100i16, -40, 7, 0]) {
            unit.v_thresh = THRESHOLD_BASE;
            unit.stp_u_rel = STP_U;
            unit.stp_r_ves = STP_MAX;
            unit.value_weight = weight;
        }
        let strong = spike_message(synaptic_efficacy_q16(i16::MAX, STP_U, STP_MAX), false);
        // A trial: the units of `pattern` kicked at its start, 400 ticks run.
        let trial = |exec: &mut Executor<8>, pattern: &[u32]| {
            let inject = exec.injector();
            for &unit in pattern {
                for _ in 0..14 {
                    inject.inject(unit, strong).unwrap();
                }
            }
            for _ in 0..400 {
                exec.tick();
            }
        };
        let weights = |exec: &Executor<8>| -> Vec<i16> {
            exec.units().iter().map(|u| u.value_weight).collect()
        };
        let sides = |exec: &Executor<8>| {
            (
                (0..4).map(|u| exec.is_source(u)).collect::<Vec<bool>>(),
                (0..4).map(|u| exec.is_target(u)).collect::<Vec<bool>>(),
            )
        };
        let every = (vec![true; 4], vec![true; 4]);
        assert_eq!(
            exec.reward_kept(0x1_0000, 0),
            Err(AddressError::NoKeptAddress)
        );
        assert_eq!(
            exec.reward_kept(0x1_0000, 2),
            Err(AddressError::NoKeptAddress)
        );
        assert_eq!(exec.reward_kept(0x1_0000, 3), Err(AddressError::AgeNotKept));
        assert_eq!(
            exec.reward_kept(0x1_0000, u8::MAX),
            Err(AddressError::AgeNotKept)
        );
        assert_eq!(
            (exec.modulator().dopamine_rpe, exec.prediction()),
            (0, None)
        );
        // Three closes, each a trial's window kept with its targets.
        let mut closes: Vec<(Vec<u32>, Vec<bool>)> = Vec::new();
        for (pattern, targets) in [
            (&[0u32, 2][..], &[3u32, 1][..]),
            (&[1], &[0]),
            (&[3, 0], &[]),
        ] {
            trial(&mut exec, pattern);
            let counts = exec.features().to_vec();
            assert_eq!(
                counts.iter().map(|&c| c != 0).collect::<Vec<bool>>(),
                (0..4u32)
                    .map(|u| pattern.contains(&u))
                    .collect::<Vec<bool>>(),
                "the window counted the pattern"
            );
            let before = (weights(&exec), exec.modulator().dopamine_rpe, sides(&exec));
            assert_eq!(exec.keep_address([0u32, 4]), Err(AddressError::NoSuchUnit));
            assert_eq!(exec.features(), &counts[..], "refused whole");
            assert_eq!(exec.keep_address(targets.iter().copied()), Ok(()));
            let kept: Vec<bool> = (0..4u32).map(|u| targets.contains(&u)).collect();
            assert_eq!(exec.kept_at(0), Some((&counts[..], &kept[..])));
            assert_eq!(exec.features(), &[0; 4], "counted afresh");
            assert_eq!(exec.window_opened(), exec.ticks(), "the window opens again");
            assert_eq!(
                (weights(&exec), exec.modulator().dopamine_rpe, sides(&exec)),
                before,
                "a close moves no weight, no signal and no address"
            );
            assert_eq!(exec.prediction(), None);
            closes.push((counts, kept));
        }
        for (age, (counts, kept)) in closes.iter().rev().enumerate() {
            assert_eq!(
                exec.kept_at(age as u8),
                Some((&counts[..], &kept[..])),
                "age {age}"
            );
        }
        assert_eq!(exec.kept_let_go(), 0);
        assert_eq!(sides(&exec), every, "no address written yet");
        // The live counts after the third close, which a reward of an age does not touch.
        trial(&mut exec, &[1, 2]);
        let live = exec.features().to_vec();
        assert!(live.iter().any(|&c| c != 0));
        let opened = exec.window_opened();
        // The reward of age two: the first close's.
        let (counts, kept) = closes[0].clone();
        let before = weights(&exec);
        let exact: i64 = before
            .iter()
            .zip(&counts)
            .map(|(&w, &c)| i64::from(w) * i64::from(c))
            .sum();
        let value = exact.div_euclid(4) as i32;
        let error = 0x1_0000 - value;
        let signal = exec.modulator().dopamine_rpe;
        assert_eq!(exec.reward_kept(0x1_0000, 2), Ok(signal + error));
        assert_eq!(
            exec.prediction(),
            Some(Prediction {
                value_q16: value,
                error_q16: error,
                received_q16: error
            })
        );
        assert_eq!(
            sides(&exec),
            (counts.iter().map(|&c| c != 0).collect(), kept.clone()),
            "the first close's address"
        );
        let moved: Vec<i16> = before
            .iter()
            .zip(&counts)
            .map(|(&w, &c)| {
                (i64::from(w) + (i64::from(error) * i64::from(c)).div_euclid(512)) as i16
            })
            .collect();
        assert_eq!(weights(&exec), moved, "the delta rule on the kept counts");
        assert_ne!(moved, before, "a weight moved");
        assert_eq!(
            (exec.features(), exec.window_opened()),
            (&live[..], opened),
            "the live counts and the window as they were"
        );
        assert_eq!(exec.kept_at(2), None, "spent");
        assert_eq!(exec.kept_at(1), Some((&closes[1].0[..], &closes[1].1[..])));
        assert_eq!(exec.kept_at(0), Some((&closes[2].0[..], &closes[2].1[..])));
        let after = (
            weights(&exec),
            exec.modulator().dopamine_rpe,
            sides(&exec),
            exec.prediction(),
        );
        assert_eq!(
            exec.reward_kept(-0x1_0000, 2),
            Err(AddressError::NoKeptAddress)
        );
        assert_eq!(
            (
                weights(&exec),
                exec.modulator().dopamine_rpe,
                sides(&exec),
                exec.prediction()
            ),
            after,
            "a spent entry is refused, nothing changed"
        );
        // A fourth close writes over the spent entry; a fifth over the second close's, whose
        // reward is still to come.
        assert_eq!(exec.keep_address([2u32]), Ok(()));
        assert_eq!(exec.kept_let_go(), 0, "the entry written over was spent");
        assert_eq!(exec.kept_at(1), Some((&closes[2].0[..], &closes[2].1[..])));
        assert_eq!(exec.kept_at(2), Some((&closes[1].0[..], &closes[1].1[..])));
        trial(&mut exec, &[0]);
        assert_eq!(exec.keep_address([1u32]), Ok(()));
        assert_eq!(
            exec.kept_let_go(),
            1,
            "the second close's reward never came"
        );
        assert_eq!(exec.kept_at(2), Some((&closes[2].0[..], &closes[2].1[..])));
        // A reward of age zero: the newest close's, the fifth.
        let (newest, targets) = exec
            .kept_at(0)
            .map(|(c, t)| (c.to_vec(), t.to_vec()))
            .unwrap();
        assert_eq!(
            newest.iter().map(|&c| c != 0).collect::<Vec<bool>>(),
            [true, false, false, false],
            "the fifth close kept unit 0's spikes"
        );
        let before = weights(&exec);
        let value = (i64::from(before[0]) * i64::from(newest[0])).div_euclid(4) as i32;
        let signal = exec.modulator().dopamine_rpe;
        assert_eq!(
            exec.reward_kept(-0x1_0000, 0),
            Ok(signal + (-0x1_0000 - value))
        );
        assert_eq!(sides(&exec), (vec![true, false, false, false], targets));
        assert_eq!(exec.kept_at(0), None);
    }
}

/// The lattice property (ADR-0030) of the addressing (ADR-0068): over the lattice and a
/// seeded walk of baselines and dopamine signals, the modulation of an addressed unit is
/// `clamp(baseline + dopamine, 0, 1)` and every other unit's is `clamp(baseline, 0, 1)`, one
/// number when the signal is at rest; and the consolidation slot by slot under one
/// modulation is `consolidate_all`, the previous call, bit for bit over seeded blocks.
#[cfg(test)]
mod prop {
    use super::*;
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../testkit/prop.rs"
    ));

    #[test]
    fn the_addressed_modulation_is_the_baseline_with_the_signal_and_the_other_the_baseline_alone() {
        let mut lcg = Lcg::new(0x68);
        let mut cases: Vec<(i32, i32)> = Vec::new();
        for &baseline in &I32_LATTICE {
            for &dopamine in &I32_LATTICE {
                cases.push((baseline, dopamine));
            }
        }
        for _ in 0..4_000 {
            cases.push((lcg.i32_edge_biased(), lcg.i32_edge_biased()));
        }
        let mut modulator = NeuromodulatorState::new();
        for (baseline, dopamine) in cases {
            modulator.dopamine_rpe = dopamine;
            let m = Modulations::of(&modulator, baseline, None, false);
            assert_eq!(
                m.addressed,
                baseline
                    .saturating_add(dopamine)
                    .clamp(0, MODULATION_ONE_Q16),
                "{baseline} {dopamine}"
            );
            assert_eq!(m.at_rest, baseline.clamp(0, MODULATION_ONE_Q16));
            assert_eq!(
                m.addressed,
                modulator.modulation(baseline),
                "the rule is menteion-neuromod's"
            );
            assert_eq!(m.for_synapse(true, Polarity::Excitatory), m.addressed);
            assert_eq!(m.for_synapse(false, Polarity::Excitatory), m.at_rest);
            if dopamine == 0 {
                assert_eq!(m.addressed, m.at_rest, "at rest the two are one number");
            }
        }
        // The harness's baseline, a reward and a dip: the addressed modulation moves, the
        // other stays at the baseline.
        modulator.dopamine_rpe = 0x4000;
        let m = Modulations::of(&modulator, 0x8000, None, false);
        assert_eq!(
            (
                m.for_synapse(true, Polarity::Excitatory),
                m.for_synapse(false, Polarity::Excitatory)
            ),
            (0xC000, 0x8000)
        );
        modulator.dopamine_rpe = -0x4000;
        let m = Modulations::of(&modulator, 0x8000, None, false);
        assert_eq!(
            (
                m.for_synapse(true, Polarity::Excitatory),
                m.for_synapse(false, Polarity::Excitatory)
            ),
            (0x4000, 0x8000)
        );
        modulator.dopamine_rpe = MODULATION_ONE_Q16;
        let m = Modulations::of(&modulator, 0x8000, None, false);
        assert_eq!(
            (
                m.for_synapse(true, Polarity::Excitatory),
                m.for_synapse(false, Polarity::Excitatory)
            ),
            (MODULATION_ONE_Q16, 0x8000),
            "the reward of brief 027 carries the addressed to the ceiling"
        );
        modulator.dopamine_rpe = -MODULATION_ONE_Q16;
        let m = Modulations::of(&modulator, 0x8000, None, false);
        assert_eq!(
            (
                m.for_synapse(true, Polarity::Excitatory),
                m.for_synapse(false, Polarity::Excitatory)
            ),
            (0, 0x8000),
            "and the punishment to the floor"
        );
    }

    /// A seeded block: three slots in four filled with a seeded target, weight and delay,
    /// a seeded trace in every slot, empty ones included.
    fn seeded_block(lcg: &mut Lcg) -> SynapseBlock {
        let mut block = SynapseBlock::new();
        for slot in 0..SYNAPSES_PER_BLOCK {
            if lcg.below(4) != 0 {
                let weight = if lcg.below(2) == 0 {
                    lcg.pick(&I16_LATTICE)
                } else {
                    lcg.next_i16()
                };
                assert!(block.set_synapse(
                    slot,
                    lcg.below(1_000),
                    weight,
                    lcg.below(2_560) as u16,
                    lcg.below(2) == 1
                ));
            }
            block.eligibility_q1_15[slot] = if lcg.below(2) == 0 {
                lcg.pick(&I16_LATTICE)
            } else {
                lcg.next_i16()
            };
        }
        block
    }

    /// Since ADR-0094 `consolidate_each` is `consolidate_signed` slot by slot; under one
    /// modulation at or above zero for every slot it is `consolidate_all`, the call before the
    /// signed gate existed, bit for bit, and under one below zero it is not.
    #[test]
    fn consolidate_each_is_consolidate_signed_slot_by_slot_and_consolidate_all_above_zero() {
        let mut lcg = Lcg::new(0x69);
        let mut below = 0u32;
        for round in 0..2_000u32 {
            let block = seeded_block(&mut lcg);
            let polarity = if lcg.below(2) == 0 {
                Polarity::Excitatory
            } else {
                Polarity::Inhibitory
            };
            let modulation = lcg.i32_edge_biased();
            let mut each = block;
            consolidate_each(&mut each, &[modulation; SYNAPSES_PER_BLOCK], polarity);
            let mut oracle = block;
            let weights = oracle.consolidate_all(modulation.max(0), polarity);
            if modulation >= 0 {
                assert_eq!(
                    each, oracle,
                    "round {round}: the previous call, bit for bit"
                );
                assert_eq!(each.weights_q1_15, weights);
            } else if each != oracle {
                below = below.saturating_add(1);
            }
            // Under a modulation per slot, every slot is its own `consolidate_signed`, in order.
            let per_slot: [i32; SYNAPSES_PER_BLOCK] =
                core::array::from_fn(|_| lcg.i32_edge_biased());
            let mut expected = block;
            for (slot, &m) in per_slot.iter().enumerate() {
                expected.consolidate_signed(slot, m, polarity);
            }
            let mut each = block;
            consolidate_each(&mut each, &per_slot, polarity);
            assert_eq!(each, expected, "round {round}: slot by slot");
        }
        assert!(
            below > 100,
            "below zero the signed rule moves what `consolidate_all` leaves: {below} rounds"
        );
    }

    /// The lattice property of the inhibitory baseline (ADR-0086): over the lattice and a
    /// seeded walk of baselines, dopamine signals and inhibitory baselines, while the
    /// inhibitory baseline is set every slot of an inhibitory block consolidates under
    /// `clamp(inhibitory baseline, 0, 1)` whatever the addressing and the signal, and every
    /// slot of an excitatory block under the two modulations as before; while it is unset
    /// the third modulation is none and an inhibitory slot's is an excitatory slot's; and the
    /// word the coordinator publishes round-trips, the sentinel for unset and the modulation
    /// itself — zero included — for set.
    #[test]
    fn an_inhibitory_block_consolidates_under_the_inhibitory_baseline_alone_while_it_is_set() {
        let mut lcg = Lcg::new(0x86);
        let mut cases: Vec<(i32, i32, i32)> = Vec::new();
        for &baseline in &I32_LATTICE {
            for &dopamine in &I32_LATTICE {
                for &inhibitory in &I32_LATTICE {
                    cases.push((baseline, dopamine, inhibitory));
                }
            }
        }
        for _ in 0..4_000 {
            cases.push((
                lcg.i32_edge_biased(),
                lcg.i32_edge_biased(),
                lcg.i32_edge_biased(),
            ));
        }
        let mut modulator = NeuromodulatorState::new();
        for (baseline, dopamine, inhibitory) in cases {
            modulator.dopamine_rpe = dopamine;
            let unset = Modulations::of(&modulator, baseline, None, false);
            let set = Modulations::of(&modulator, baseline, Some(inhibitory), false);
            let clamped = inhibitory.clamp(0, MODULATION_ONE_Q16);
            assert_eq!(unset.inhibitory, None, "{baseline} {dopamine} {inhibitory}");
            assert_eq!(set.inhibitory, Some(clamped));
            assert_eq!(
                (set.addressed, set.at_rest),
                (unset.addressed, unset.at_rest),
                "the inhibitory baseline moves neither of the other two"
            );
            for addressed in [false, true] {
                let excitatory = unset.for_synapse(addressed, Polarity::Excitatory);
                assert_eq!(
                    excitatory,
                    if addressed {
                        unset.addressed
                    } else {
                        unset.at_rest
                    }
                );
                assert_eq!(
                    unset.for_synapse(addressed, Polarity::Inhibitory),
                    excitatory,
                    "unset: an inhibitory slot's modulation is an excitatory slot's"
                );
                assert_eq!(
                    set.for_synapse(addressed, Polarity::Excitatory),
                    excitatory,
                    "set: an excitatory slot's is as before"
                );
                assert_eq!(
                    set.for_synapse(addressed, Polarity::Inhibitory),
                    clamped,
                    "set: an inhibitory slot's is the inhibitory baseline, clamped, whatever the addressing and the signal"
                );
            }
            assert_eq!(
                inhibitory_of_word(inhibitory_word(set.inhibitory)),
                set.inhibitory
            );
            assert_eq!(inhibitory_word(set.inhibitory), clamped);
        }
        assert_eq!(inhibitory_word(None), INHIBITORY_UNSET);
        assert_eq!(inhibitory_of_word(INHIBITORY_UNSET), None);
        assert_eq!(
            inhibitory_of_word(0),
            Some(0),
            "zero is set, not the sentinel"
        );
        assert_eq!(
            inhibitory_of_word(MODULATION_ONE_Q16),
            Some(MODULATION_ONE_Q16)
        );
        assert_eq!(inhibitory_of_word(i32::MIN), None);
        // The harness's baselines: the reward's gate at zero with the inhibitory baseline at
        // 0.5 (H-16), a reward and a punishment.
        for (dopamine, addressed_excitatory) in [
            (MODULATION_ONE_Q16, MODULATION_ONE_Q16),
            (-MODULATION_ONE_Q16, 0),
            (0x4000, 0x4000),
            (0, 0),
        ] {
            modulator.dopamine_rpe = dopamine;
            let m = Modulations::of(&modulator, 0, Some(0x8000), false);
            assert_eq!(
                (
                    m.for_synapse(true, Polarity::Excitatory),
                    m.for_synapse(false, Polarity::Excitatory),
                    m.for_synapse(true, Polarity::Inhibitory),
                    m.for_synapse(false, Polarity::Inhibitory),
                ),
                (addressed_excitatory, 0, 0x8000, 0x8000),
                "{dopamine}: the signal reaches the addressed excitatory slot and no inhibitory one"
            );
        }
    }

    /// The lattice property of the signed gate (ADR-0094): over the lattice and a seeded walk
    /// of baselines, dopamine signals and inhibitory baselines set and unset, while the signed
    /// gate is set an addressed excitatory slot consolidates under `clamp(baseline + dopamine,
    /// -1, 1)` — `menteion-neuromod`'s `signed_modulation` — and every other slot under what it
    /// consolidates under while the gate is unset: an unaddressed one at rest, an inhibitory
    /// one under its own baseline while that is set and under the addressed modulation at no
    /// less than zero while it is not. Unset, the addressed excitatory slot's is the signed
    /// one at its floor of zero, the rule before ADR-0094; and no modulation below zero
    /// reaches any slot but an addressed excitatory one under the set gate.
    #[test]
    fn only_an_addressed_excitatory_slot_consolidates_under_the_signed_modulation_while_the_gate_is_set()
     {
        let mut lcg = Lcg::new(0x94);
        let mut cases: Vec<(i32, i32, Option<i32>)> = Vec::new();
        for &baseline in &I32_LATTICE {
            for &dopamine in &I32_LATTICE {
                cases.push((baseline, dopamine, None));
                cases.push((baseline, dopamine, Some(0x8000)));
            }
        }
        for _ in 0..4_000 {
            let inhibitory = if lcg.below(2) == 0 {
                None
            } else {
                Some(lcg.i32_edge_biased())
            };
            cases.push((lcg.i32_edge_biased(), lcg.i32_edge_biased(), inhibitory));
        }
        let mut modulator = NeuromodulatorState::new();
        for (baseline, dopamine, inhibitory) in cases {
            modulator.dopamine_rpe = dopamine;
            let unset = Modulations::of(&modulator, baseline, inhibitory, false);
            let set = Modulations::of(&modulator, baseline, inhibitory, true);
            let signed = baseline
                .saturating_add(dopamine)
                .clamp(-MODULATION_ONE_Q16, MODULATION_ONE_Q16);
            assert_eq!(set.addressed, signed, "{baseline} {dopamine}");
            assert_eq!(
                set.addressed,
                modulator.signed_modulation(baseline),
                "the rule is menteion-neuromod's"
            );
            assert_eq!(
                (set.at_rest, set.inhibitory),
                (unset.at_rest, unset.inhibitory),
                "the signed gate moves neither of the other two"
            );
            for addressed in [false, true] {
                for polarity in [Polarity::Excitatory, Polarity::Inhibitory] {
                    let before = unset.for_synapse(addressed, polarity);
                    let after = set.for_synapse(addressed, polarity);
                    if addressed && polarity == Polarity::Excitatory {
                        assert_eq!(after, signed, "{baseline} {dopamine}: set, the signed one");
                        assert_eq!(
                            before,
                            signed.max(0),
                            "{baseline} {dopamine}: unset, the same number at its floor"
                        );
                    } else {
                        assert_eq!(
                            after, before,
                            "{baseline} {dopamine} {inhibitory:?} {addressed} {polarity:?}: every other slot as unset"
                        );
                        assert!(
                            after >= 0,
                            "no modulation below zero reaches any other slot"
                        );
                    }
                }
            }
        }
        // H-18's configuration (ADR-0093): the reward's gate at zero, the inhibitory baseline
        // at 0.5 and the signed gate set, under the signal after a punishment and at a trial's
        // end under a punishment every trial, at rest, and the mirror under a reward.
        for (dopamine, addressed_excitatory) in [
            (-112_227, -MODULATION_ONE_Q16),
            (-46_691, -46_691),
            (0, 0),
            (46_691, 46_691),
            (112_227, MODULATION_ONE_Q16),
        ] {
            modulator.dopamine_rpe = dopamine;
            let m = Modulations::of(&modulator, 0, Some(0x8000), true);
            assert_eq!(
                (
                    m.for_synapse(true, Polarity::Excitatory),
                    m.for_synapse(false, Polarity::Excitatory),
                    m.for_synapse(true, Polarity::Inhibitory),
                    m.for_synapse(false, Polarity::Inhibitory),
                ),
                (addressed_excitatory, 0, 0x8000, 0x8000),
                "{dopamine}: the signed modulation reaches the addressed excitatory slot alone"
            );
        }
        // The inhibitory baseline unset: an addressed inhibitory slot under a punishment
        // consolidates under zero, not below it; above zero it shares the excitatory slot's.
        modulator.dopamine_rpe = -MODULATION_ONE_Q16;
        let m = Modulations::of(&modulator, 0, None, true);
        assert_eq!(
            (
                m.for_synapse(true, Polarity::Excitatory),
                m.for_synapse(true, Polarity::Inhibitory),
                m.for_synapse(false, Polarity::Inhibitory),
            ),
            (-MODULATION_ONE_Q16, 0, 0)
        );
        modulator.dopamine_rpe = -1;
        let m = Modulations::of(&modulator, 0, None, true);
        assert_eq!(
            (
                m.for_synapse(true, Polarity::Excitatory),
                m.for_synapse(true, Polarity::Inhibitory)
            ),
            (-1, 0),
            "one LSB below zero"
        );
        modulator.dopamine_rpe = 0x4000;
        let m = Modulations::of(&modulator, 0, None, true);
        assert_eq!(
            (
                m.for_synapse(true, Polarity::Excitatory),
                m.for_synapse(true, Polarity::Inhibitory)
            ),
            (0x4000, 0x4000),
            "above zero, one number"
        );
    }
}

/// The lattice property (ADR-0030) of the gain an engine starts with (ADR-0196): over the
/// lattice, both bounds and their neighbours, and a seeded walk inside the bounds and across the
/// width, an engine is built exactly where an oracle written from ADR-0196's text accepts the
/// gain, 0.25 to 4 inclusive, its homeostasis record's gain and the workers' the configuration's;
/// everywhere else it is refused with `GainOutOfRange`.
#[cfg(test)]
mod gain_prop {
    use super::*;
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../testkit/prop.rs"
    ));

    #[test]
    fn an_engine_is_built_at_a_gain_exactly_where_the_controller_s_bounds_hold_it() {
        let mut rng = Lcg::new(0x0196);
        let mut gains: Vec<u32> = U32_LATTICE.to_vec();
        gains.extend([
            0x3FFF,
            0x4000,
            0x4001,
            0x0003_FFFF,
            0x0004_0000,
            0x0004_0001,
        ]);
        for round in 0..1_000u32 {
            gains.push(match round % 3 {
                0 => 0x4000u32.saturating_add(rng.below(0x0003_C001)),
                1 => rng.u32_edge_biased(),
                _ => rng.next_u32(),
            });
        }
        let (mut built, mut refused) = (0u32, 0u32);
        for &gain in &gains {
            let by_hand = (0x4000..=0x0004_0000).contains(&gain);
            match Executor::<8>::new(Config {
                units: 2,
                synaptic_gain_q16: gain,
                ..Config::default()
            }) {
                Ok(exec) => {
                    assert!(by_hand, "{gain:#x} built");
                    assert_eq!(exec.homeostasis().synaptic_gain_q16, gain);
                    assert_eq!(exec.shared.gain.load(Ordering::Relaxed), gain);
                    built = built.wrapping_add(1);
                }
                Err(error) => {
                    assert!(!by_hand, "{gain:#x} refused");
                    assert_eq!(error, ConfigError::GainOutOfRange, "{gain:#x}");
                    refused = refused.wrapping_add(1);
                }
            }
        }
        assert!(
            built > 300 && refused > 300,
            "the walk met both sides: {built} built, {refused} refused"
        );
    }
}

/// The lattice property (ADR-0030) of the read of the rate's target (ADR-0189): over spans,
/// units and windows' counts drawn on and between their edges, a read stepped window by window
/// by `read_step` from its configuration's span ends where an oracle written from ADR-0188's
/// text ends it — the mean of the last sixteen counts, the period of that mean and the count of
/// that period, each to nearest by doubling in wider integers, or the refusal — and holds, at
/// every window between, the windows left and the sum of those of its last sixteen closed, each
/// within the bound the loader holds a read to. Since ADR-0194 every other four rounds read with
/// the inhibitory target by the read set, where the oracle also refuses a target whose period the
/// inhibitory rule refuses, ADR-0192's text, and the walk meets both of that rule's bounds.
#[cfg(test)]
mod read_prop {
    use super::*;
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../testkit/prop.rs"
    ));

    /// ADR-0188's read in `i128`, from its text and not from the engine's code: none where any
    /// rounding gives no spike, no tick, a period past the width, or a count past a window's.
    fn read_by_hand(units: usize, counts: &[u64]) -> Option<(u32, u32)> {
        let last = &counts[counts.len().checked_sub(16)?..];
        let sum = last
            .iter()
            .fold(0i128, |s, &c| s.saturating_add(i128::from(c)));
        let mean = sum.saturating_mul(2).saturating_add(16).div_euclid(32);
        let ticks = i128::try_from(units)
            .unwrap_or(i128::MAX)
            .saturating_mul(131_072)
            .min(i128::from(u64::MAX));
        if mean == 0 {
            return None;
        }
        let period = ticks
            .saturating_mul(2)
            .saturating_add(mean)
            .div_euclid(mean.saturating_mul(2));
        if period == 0 || period > i128::from(u32::MAX) {
            return None;
        }
        let count = ticks
            .saturating_mul(2)
            .saturating_add(period)
            .div_euclid(period.saturating_mul(2));
        if count == 0 || count > 536_870_880 {
            return None;
        }
        Some((period as u32, count as u32))
    }

    /// ADR-0192's refusal beside it (ADR-0194): where the read sets the inhibitory rule's target
    /// as well, a target whose period that rule refuses, outside 100 to 1 000 000 ticks, refused.
    fn inhibitory_by_hand(target: Option<(u32, u32)>) -> Option<(u32, u32)> {
        target.filter(|&(period, _)| (100..=1_000_000).contains(&period))
    }

    #[test]
    fn a_read_stepped_window_by_window_ends_where_the_hand_rule_ends_it() {
        let mut rng = Lcg::new(0x0189);
        let (mut targets, mut refused) = (0u32, 0u32);
        // The rounds whose target the inhibitory rule alone refuses (ADR-0194), by either bound.
        let (mut wide, mut short) = (0u32, 0u32);
        for round in 0..4_000u32 {
            // The flag on every other four rounds (ADR-0194), so that each of the four spans' rules
            // meets it; the walk's draws are the ones they were.
            let inhibitory = (round / 4) % 2 == 1;
            let span: u8 = match round % 4 {
                0 => 16,
                1 => 17,
                2 => rng.pick(&[18u8, 31, 128, 255]),
                _ => 16u8.saturating_add(rng.below(240) as u8),
            };
            let units: usize = match round % 5 {
                0 => 1,
                1 => 1024,
                2 => rng.pick(&[64usize, 2048, 4_095, 32_767, 32_768]),
                _ => 1usize.saturating_add(rng.below(1 << 16) as usize),
            };
            let around = rng.pick(&[0u64, 1, 7, 8, 2_306, 11_602, 1 << 20]);
            let counts: Vec<u64> = (0..span)
                .map(|_| match rng.below(6) {
                    0 => u64::from(rng.pick(&U32_LATTICE)).min(u64::from(RATE_TARGET_MAX)),
                    1 => 0,
                    2 => u64::from(RATE_TARGET_MAX),
                    3 => around.saturating_add(u64::from(rng.below(3))),
                    _ => u64::from(rng.below(1 << 14)),
                })
                .collect();
            let (mut left, mut sum) = (span, 0u64);
            let mut end = None;
            for (k, &count) in counts.iter().enumerate() {
                match read_step(left, sum, count, units, inhibitory) {
                    ReadStep::Reading { left: l, sum: s } => {
                        assert_eq!(l, left.wrapping_sub(1), "round {round}, window {k}");
                        let closed = k.wrapping_add(1);
                        let first = usize::from(span).saturating_sub(16);
                        let by_hand: u64 = counts[first.min(closed)..closed].iter().sum();
                        assert_eq!(s, by_hand, "round {round}, window {k}: the last sixteen");
                        assert!(s <= read_sum_max(l), "round {round}, window {k}");
                        (left, sum) = (l, s);
                    }
                    step => {
                        assert_eq!(k.wrapping_add(1), usize::from(span), "round {round}");
                        end = Some(step);
                    }
                }
            }
            let by_hand = read_by_hand(units, &counts);
            let by_hand = if inhibitory {
                let kept = inhibitory_by_hand(by_hand);
                match (by_hand, kept) {
                    (Some((period, _)), None) if period > 1_000_000 => wide = wide.wrapping_add(1),
                    (Some(_), None) => short = short.wrapping_add(1),
                    _ => {}
                }
                kept
            } else {
                by_hand
            };
            let expected = match by_hand {
                Some((period, count)) => ReadStep::Target { period, count },
                None => ReadStep::Refused,
            };
            assert_eq!(
                end,
                Some(expected),
                "round {round}: {units} units, {counts:?}"
            );
            match expected {
                ReadStep::Target { .. } => targets = targets.wrapping_add(1),
                _ => refused = refused.wrapping_add(1),
            }
        }
        assert!(
            targets > 1_000 && refused > 100,
            "the walk met both ends: {targets} targets, {refused} refused"
        );
        assert!(
            wide > 10 && short > 100,
            "the walk met both of the inhibitory rule's bounds: {wide} past the widest, {short} short of the shortest"
        );
    }
}

/// The lattice property (ADR-0030) of the probe of the step (ADR-0202): over targets, windows'
/// counts and gains drawn on and between their edges, a probe stepped window by window by
/// `probe_step` from its start ends where an oracle written from ADR-0200's text ends it — the
/// eight higher windows' counts less the eight lower ones', in the order four lower, eight higher,
/// four lower, and the step $4\,096\,c^{*}/D$ to nearest by doubling in wider integers, refused at
/// $D \le 0$ and past 0x8000 — and holds, at every window between, the windows left and the sum so
/// far within the bounds the loader holds a probe to; and the gain the workers are given through
/// each window is the record's times $1 \mp 1/64$ to nearest by doubling. The walk meets a step
/// accepted, a sum not above zero and a step past its bound.
#[cfg(test)]
mod probe_prop {
    use super::*;
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../testkit/prop.rs"
    ));

    /// ADR-0200's step in `i128`, from its text and not from the engine's code.
    fn step_by_hand(count: u32, counts: &[u64]) -> Option<u16> {
        let d = counts.iter().enumerate().fold(0i128, |d, (k, &c)| {
            if (4..12).contains(&k) {
                d.saturating_add(i128::from(c))
            } else {
                d.saturating_sub(i128::from(c))
            }
        });
        if d <= 0 {
            return None;
        }
        let step = i128::from(count)
            .saturating_mul(4_096)
            .saturating_mul(2)
            .saturating_add(d)
            .div_euclid(d.saturating_mul(2));
        u16::try_from(step).ok().filter(|&step| step <= 0x8000)
    }

    /// ADR-0200's gain in `i128`: the record's times 65/64 in the windows five to twelve left and
    /// times 63/64 in the others, to nearest by doubling.
    fn gain_by_hand(gain: u32, left: u8) -> u32 {
        let factor = if (5..=12).contains(&left) { 65 } else { 63 };
        let given = i128::from(gain)
            .saturating_mul(factor)
            .saturating_mul(2)
            .saturating_add(64)
            .div_euclid(128);
        u32::try_from(given).unwrap_or(u32::MAX)
    }

    #[test]
    fn a_probe_stepped_window_by_window_ends_where_the_hand_rule_ends_it() {
        let mut rng = Lcg::new(0x0202);
        let (mut steps, mut not_above, mut past) = (0u32, 0u32, 0u32);
        for round in 0..4_000u32 {
            let count: u32 = match round % 4 {
                0 => 2_306,
                1 => rng.pick(&[0u32, 1, 8, 320, 32_768, 32_769, RATE_TARGET_MAX]),
                2 => rng.below(1 << 14),
                _ => rng.u32_edge_biased().min(RATE_TARGET_MAX),
            };
            let gain = match round % 3 {
                0 => 0x0001_C000,
                1 => rng.pick(&[GAIN_MIN_Q16, 0x4020, 0x401F, GAIN_ONE_Q16, GAIN_MAX_Q16]),
                _ => GAIN_MIN_Q16.saturating_add(rng.below(0x0003_C001)),
            };
            // A window's counts about the target, one side higher by a margin, or anywhere; none
            // above what a window's bins count.
            let lower = u64::from(count).saturating_sub(u64::from(rng.below(64)));
            let margin = u64::from(rng.pick(&[0u32, 1, 2, 5, 40, 1_153]));
            let counts: Vec<u64> = (0..16usize)
                .map(|k| {
                    let c = match rng.below(5) {
                        0 => u64::from(rng.pick(&U32_LATTICE)),
                        1 => 0,
                        _ if (4..12).contains(&k) => lower
                            .saturating_add(margin)
                            .saturating_add(u64::from(rng.below(3))),
                        _ => lower.saturating_add(u64::from(rng.below(3))),
                    };
                    c.min(u64::from(RATE_TARGET_MAX))
                })
                .collect();
            let (mut left, mut sum) = (PROBE_WINDOWS, 0i64);
            let mut end = None;
            for (k, &c) in counts.iter().enumerate() {
                assert_eq!(
                    probe_gain_q16(gain, left),
                    gain_by_hand(gain, left),
                    "round {round}, window {k}: {gain:#x}"
                );
                match probe_step(left, sum, c, count) {
                    ProbeStep::Probing { left: l, sum: s } => {
                        assert_eq!(l, left.wrapping_sub(1), "round {round}, window {k}");
                        let (least, most) = probe_sum_bounds(l);
                        assert!(
                            (least..=most).contains(&s),
                            "round {round}, window {k}: {s} within {least} and {most}"
                        );
                        (left, sum) = (l, s);
                    }
                    step => {
                        assert_eq!(k, 15, "round {round}");
                        end = Some(step);
                    }
                }
            }
            let expected = match step_by_hand(count, &counts) {
                Some(step) => {
                    steps = steps.wrapping_add(1);
                    ProbeStep::Step(step)
                }
                None => {
                    let higher: u64 = counts[4..12].iter().sum();
                    let lower: u64 = counts[..4].iter().chain(&counts[12..]).sum();
                    if higher > lower {
                        past = past.wrapping_add(1);
                    } else {
                        not_above = not_above.wrapping_add(1);
                    }
                    ProbeStep::Refused
                }
            };
            assert_eq!(
                end,
                Some(expected),
                "round {round}: target {count}, {counts:?}"
            );
        }
        assert!(
            steps > 500 && not_above > 500 && past > 100,
            "the walk met each end: {steps} steps, {not_above} not above zero, {past} past the bound"
        );
    }
}

/// The lattice property (ADR-0030) of the kept addresses (ADR-0208), two walks.
///
/// Against an oracle written from ADR-0208's text: over a seeded walk of trials — armed units
/// kicked and ticks run — closes with targets drawn from the arena and one past it, and rewards of
/// every age from zero to two past the oldest kept at rewards over the lattice, under critics,
/// windows, oldest ages and the whole punishment drawn, every call is the oracle's. The oracle
/// keeps a ring of the counts each close found and the targets it was given, refuses what the
/// text refuses, and at a reward reads the value and moves the weights from the entry the age
/// names by `ValueCritic`'s rule, writes that entry's address and spends it. After every call the
/// entry at every age, the entries let go, the weights, the addressed set, the reading and the
/// signal are the oracle's, and the live counts are zeroed by a close and untouched by a reward.
///
/// The twin: over a seeded walk of trials each closed and rewarded at once, an engine that keeps
/// addresses and rewards at age zero is the engine that draws and rewards, bit for bit — its
/// weights, counts, window, reading, signal and addressed set after every trial.
#[cfg(test)]
mod kept_prop {
    use super::*;
    use menteion_core::{STP_MAX, STP_U, synaptic_efficacy_q16};
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../testkit/prop.rs"
    ));

    const UNITS: u32 = 6;

    /// An engine of `UNITS` armed units under `config`, its value weights drawn.
    fn armed(config: Config, rng: &mut Lcg) -> Executor<8> {
        let mut exec = Executor::<8>::new(config).unwrap();
        for unit in exec.units_mut() {
            unit.v_thresh = THRESHOLD_BASE;
            unit.stp_u_rel = STP_U;
            unit.stp_r_ves = STP_MAX;
            unit.value_weight = match rng.below(3) {
                0 => rng.pick(&I16_LATTICE),
                _ => rng.next_i16() >> rng.below(12),
            };
        }
        exec
    }

    /// A configuration drawn: a critic the rule resolves, a window, the whole punishment or not,
    /// and `kept` addresses.
    fn drawn(rng: &mut Lcg, kept: u8) -> Config {
        Config {
            units: UNITS as usize,
            nodes_per_worker: 256,
            injector_capacity: 128,
            train_capacity: 256,
            critic: Some(ValueCritic {
                shift: rng.below(31) as u8,
                scale: rng.below(15) as u8,
            }),
            critic_window_ticks: rng.pick(&[1u16, 3, 20, 400, u16::MAX]),
            whole_punishment: rng.below(2) == 0,
            kept_addresses: kept,
            ..Config::default()
        }
    }

    /// A trial: each unit kicked with a chance of a half, then one to 120 ticks run.
    fn trial(exec: &mut Executor<8>, rng: &mut Lcg) {
        let strong = spike_message(synaptic_efficacy_q16(i16::MAX, STP_U, STP_MAX), false);
        let inject = exec.injector();
        for unit in 0..UNITS {
            if rng.below(2) == 0 {
                for _ in 0..14 {
                    inject.inject(unit, strong).unwrap();
                }
            }
        }
        for _ in 0..=rng.below(120) {
            exec.tick();
        }
    }

    fn reward_of(rng: &mut Lcg) -> i32 {
        match rng.below(3) {
            0 => rng.pick(&I32_LATTICE),
            1 => rng.pick(&[0x1_0000, -0x1_0000]),
            _ => rng.next_i32() >> rng.below(16),
        }
    }

    fn weights(exec: &Executor<8>) -> Vec<i16> {
        exec.units().iter().map(|u| u.value_weight).collect()
    }

    fn sides(exec: &Executor<8>) -> (Vec<bool>, Vec<bool>) {
        (
            (0..UNITS).map(|u| exec.is_source(u)).collect(),
            (0..UNITS).map(|u| exec.is_target(u)).collect(),
        )
    }

    /// The oracle's entry: the counts a close found, the targets it was given, and whether its
    /// reward is still to come.
    type Entry = (Vec<u32>, Vec<bool>, bool);

    #[test]
    fn every_close_and_every_reward_of_an_age_is_the_oracle_s() {
        let mut rng = Lcg::new(0x0208);
        let (mut at_zero, mut at_oldest, mut aged) = (0u32, 0u32, 0u32);
        let (mut spent, mut beyond, mut outside, mut let_go) = (0u32, 0u32, 0u32, 0u64);
        for walk in 0..96u32 {
            let kept = 1u8.saturating_add(rng.below(u32::from(KEPT_ADDRESSES_MAX)) as u8);
            let config = drawn(&mut rng, kept);
            let (critic, whole) = (config.critic.unwrap(), config.whole_punishment);
            let mut exec = armed(config, &mut rng);
            // The oracle: a ring of `kept + 1` entries and the newest's slot, the entries let go,
            // the weights and the addressed set.
            let len = usize::from(kept) + 1;
            let mut ring: Vec<Entry> =
                vec![(vec![0; UNITS as usize], vec![false; UNITS as usize], false); len];
            let mut newest = 0usize;
            let mut gone = 0u64;
            let mut held = weights(&exec);
            let mut address = (vec![true; UNITS as usize], vec![true; UNITS as usize]);
            for step in 0..48u32 {
                let at = format!("walk {walk} step {step}");
                match rng.below(3) {
                    0 => trial(&mut exec, &mut rng),
                    1 => {
                        let targets: Vec<u32> = (0..=UNITS)
                            .filter(|&u| match u {
                                UNITS => rng.below(8) == 0,
                                _ => rng.below(2) == 0,
                            })
                            .collect();
                        let counts = exec.features().to_vec();
                        let opened = exec.window_opened();
                        if targets.contains(&UNITS) {
                            outside = outside.wrapping_add(1);
                            assert_eq!(
                                exec.keep_address(targets.iter().copied()),
                                Err(AddressError::NoSuchUnit),
                                "{at}"
                            );
                            assert_eq!(
                                (exec.features(), exec.window_opened()),
                                (&counts[..], opened),
                                "{at}: refused whole"
                            );
                        } else {
                            assert_eq!(exec.keep_address(targets.iter().copied()), Ok(()), "{at}");
                            newest = (newest + 1) % len;
                            if ring[newest].2 {
                                gone += 1;
                            }
                            ring[newest] = (
                                counts,
                                (0..UNITS).map(|u| targets.contains(&u)).collect(),
                                true,
                            );
                            assert_eq!(exec.features(), &[0; UNITS as usize], "{at}");
                            assert_eq!(exec.window_opened(), exec.ticks(), "{at}");
                        }
                    }
                    _ => {
                        let age = rng.below(u32::from(kept) + 3) as u8;
                        let reward = reward_of(&mut rng);
                        let live = exec.features().to_vec();
                        let opened = exec.window_opened();
                        let mut modulator = *exec.modulator();
                        let reading = exec.prediction();
                        let slot = (newest + len - usize::from(age).min(len - 1)) % len;
                        let got = exec.reward_kept(reward, age);
                        if age > kept {
                            beyond = beyond.wrapping_add(1);
                            assert_eq!(got, Err(AddressError::AgeNotKept), "{at}");
                        } else if !ring[slot].2 {
                            spent = spent.wrapping_add(1);
                            assert_eq!(got, Err(AddressError::NoKeptAddress), "{at}");
                        } else {
                            match age {
                                0 => at_zero = at_zero.wrapping_add(1),
                                a if a == kept => at_oldest = at_oldest.wrapping_add(1),
                                _ => aged = aged.wrapping_add(1),
                            }
                            let (counts, targets, _) = ring[slot].clone();
                            ring[slot].2 = false;
                            let value =
                                critic.value_q16(held.iter().copied().zip(counts.iter().copied()));
                            let error = ValueCritic::error_q16(reward, value);
                            held = held
                                .iter()
                                .zip(&counts)
                                .map(|(&w, &c)| critic.step(w, error, c))
                                .collect();
                            let received = if whole {
                                ValueCritic::received_q16(reward, value)
                            } else {
                                error
                            };
                            assert_eq!(got, Ok(modulator.reward(received)), "{at}");
                            assert_eq!(
                                exec.prediction(),
                                Some(Prediction {
                                    value_q16: value,
                                    error_q16: error,
                                    received_q16: received
                                }),
                                "{at}"
                            );
                            address = (counts.iter().map(|&c| c != 0).collect(), targets);
                        }
                        if got.is_err() {
                            assert_eq!(exec.prediction(), reading, "{at}: refused, no reading");
                        }
                        assert_eq!(exec.modulator(), &modulator, "{at}: the signal");
                        assert_eq!(
                            (exec.features(), exec.window_opened()),
                            (&live[..], opened),
                            "{at}: the live counts and the window untouched"
                        );
                    }
                }
                assert_eq!(weights(&exec), held, "{at}: the weights");
                assert_eq!(sides(&exec), address, "{at}: the addressed set");
                assert_eq!(exec.kept_let_go(), gone, "{at}: let go");
                for age in 0..=kept.saturating_add(2) {
                    let expected = (age <= kept)
                        .then(|| &ring[(newest + len - usize::from(age)) % len])
                        .filter(|entry| entry.2)
                        .map(|entry| (entry.0.as_slice(), entry.1.as_slice()));
                    assert_eq!(exec.kept_at(age), expected, "{at}: age {age}");
                }
            }
            let_go += gone;
        }
        assert!(
            at_zero > 50
                && at_oldest > 50
                && aged > 50
                && spent > 50
                && beyond > 50
                && outside > 20
                && let_go > 20,
            "the walk met every case: {at_zero} at zero, {at_oldest} at the oldest, {aged} between, {spent} spent or never kept, {beyond} beyond, {outside} outside, {let_go} let go"
        );
    }

    #[test]
    fn a_reward_kept_at_age_zero_is_the_drawn_reward_bit_for_bit() {
        let mut rng = Lcg::new(0x2080);
        let mut moved = 0u32;
        for walk in 0..64u32 {
            let kept = 1u8.saturating_add(rng.below(u32::from(KEPT_ADDRESSES_MAX)) as u8);
            let config = drawn(&mut rng, kept);
            let twin = Config {
                kept_addresses: 0,
                ..config.clone()
            };
            let seed = rng.next_u64();
            let mut engines = [
                armed(twin, &mut Lcg::new(seed)),
                armed(config, &mut Lcg::new(seed)),
            ];
            for step in 0..32u32 {
                let at = format!("walk {walk} step {step}");
                let draw = rng.next_u64();
                for exec in &mut engines {
                    trial(exec, &mut Lcg::new(draw));
                }
                let targets: Vec<u32> = (0..UNITS).filter(|_| rng.below(2) == 0).collect();
                let reward = reward_of(&mut rng);
                let [drawing, keeping] = &mut engines;
                drawing.address_drawn(targets.iter().copied()).unwrap();
                let signal = drawing.reward(reward);
                keeping.keep_address(targets.iter().copied()).unwrap();
                assert_eq!(keeping.reward_kept(reward, 0), Ok(signal), "{at}");
                let state = |exec: &Executor<8>| {
                    (
                        weights(exec),
                        exec.features().to_vec(),
                        exec.window_opened(),
                        exec.prediction(),
                        *exec.modulator(),
                        sides(exec),
                        exec.ticks(),
                    )
                };
                assert_eq!(state(&engines[0]), state(&engines[1]), "{at}");
                if engines[0].prediction().is_some_and(|p| p.error_q16 != 0) {
                    moved = moved.wrapping_add(1);
                }
            }
        }
        assert!(moved > 1_000, "the rewards moved the reading: {moved}");
    }
}

/// The delay read (ADR-0216, ADR-0217): its refusals, its leader and its move by hand, and three
/// walks over the lattice.
///
/// By hand: unset by default, where a reward given without its age is refused and nothing
/// changes, and refused without the kept addresses; the leader at its edges; and on two armed
/// units whose reward follows unit 0's spikes a trial before, no move until every age has
/// `DELAY_WINDOW` rewards and one has led for `DELAY_RUN` running, then one move, to age one,
/// losing the one reward whose entry age zero spent, and none after, though age zero then leads.
///
/// Against an oracle written from ADR-0216's text: over a seeded walk of trials, closes, rewards
/// given without their age and rewards of an age, under critics, windows, oldest ages and the
/// whole punishment drawn, the rewards following a drawn age's counts, every call is the
/// oracle's. The oracle keeps a critic for every age, the engine's the age in use's; after every
/// call each age's weights, its score, the run, the age in use, the entries, the addressed set,
/// the reading and the signal are the oracle's.
///
/// The twins: over seeded walks too short for a move, an engine that reads its delay, given each
/// trial's reward without its age at its close, is the engine that draws and rewards, bit for
/// bit; and an engine given each reward at its age is the same engine with the rule unset, bit
/// for bit.
#[cfg(test)]
mod delay_prop {
    use super::*;
    use menteion_core::{STP_MAX, STP_U, synaptic_efficacy_q16};
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../testkit/prop.rs"
    ));

    const UNITS: u32 = 6;
    const ONE: i32 = 0x1_0000;

    /// An engine under `config`, its units armed and their value weights drawn.
    fn armed(config: Config, rng: &mut Lcg) -> Executor<8> {
        let mut exec = Executor::<8>::new(config).unwrap();
        for unit in exec.units_mut() {
            unit.v_thresh = THRESHOLD_BASE;
            unit.stp_u_rel = STP_U;
            unit.stp_r_ves = STP_MAX;
            unit.value_weight = match rng.below(3) {
                0 => rng.pick(&I16_LATTICE),
                _ => rng.next_i16() >> rng.below(12),
            };
        }
        exec
    }

    /// A configuration drawn: a critic the rule resolves, half the time one that learns within a
    /// walk; a window; the whole punishment or not; `kept` addresses; the rule or not.
    fn drawn(rng: &mut Lcg, kept: u8, reads: bool) -> Config {
        let critic = if rng.below(2) == 0 {
            ValueCritic {
                shift: 2u8.saturating_add(rng.below(8) as u8),
                scale: rng.below(3) as u8,
            }
        } else {
            ValueCritic {
                shift: rng.below(31) as u8,
                scale: rng.below(15) as u8,
            }
        };
        Config {
            units: UNITS as usize,
            nodes_per_worker: 256,
            injector_capacity: 128,
            train_capacity: 256,
            critic: Some(critic),
            critic_window_ticks: rng.pick(&[1u16, 3, 20, 400, u16::MAX]),
            whole_punishment: rng.below(2) == 0,
            kept_addresses: kept,
            reads_its_delay: reads,
            ..Config::default()
        }
    }

    /// A trial: the units of `kicked` kicked, then `ticks` ticks run.
    fn kicked(exec: &mut Executor<8>, kicked: &[bool], ticks: u32) {
        let strong = spike_message(synaptic_efficacy_q16(i16::MAX, STP_U, STP_MAX), false);
        let inject = exec.injector();
        for (unit, _) in kicked.iter().enumerate().filter(|&(_, &k)| k) {
            for _ in 0..14 {
                inject.inject(unit as u32, strong).unwrap();
            }
        }
        for _ in 0..ticks {
            exec.tick();
        }
    }

    /// A trial drawn: each unit kicked with a chance of a half, then one to 40 ticks run.
    fn trial(exec: &mut Executor<8>, rng: &mut Lcg) {
        let pattern: Vec<bool> = (0..UNITS).map(|_| rng.below(2) == 0).collect();
        let ticks = 1u32.saturating_add(rng.below(40));
        kicked(exec, &pattern, ticks);
    }

    fn reward_of(rng: &mut Lcg) -> i32 {
        match rng.below(3) {
            0 => rng.pick(&I32_LATTICE),
            1 => rng.pick(&[ONE, -ONE]),
            _ => rng.next_i32() >> rng.below(16),
        }
    }

    fn weights(exec: &Executor<8>) -> Vec<i16> {
        exec.units().iter().map(|u| u.value_weight).collect()
    }

    fn sides(exec: &Executor<8>) -> (Vec<bool>, Vec<bool>) {
        (
            (0..UNITS).map(|u| exec.is_source(u)).collect(),
            (0..UNITS).map(|u| exec.is_target(u)).collect(),
        )
    }

    /// Unset by default, the rule refuses a reward given without its age and changes nothing, and
    /// every reading of it is none; set, it is refused without the kept addresses, and taken at
    /// every oldest age from one to four, the age in use zero, no score and no run, every age's
    /// critic the engine's before its first reward, and a reward before any close reaching
    /// nothing.
    #[test]
    fn the_delay_read_is_unset_by_default_and_refused_without_the_kept_addresses() {
        assert!(!Config::default().reads_its_delay, "unset by default");
        assert_eq!(
            (DELAY_WINDOW, DELAY_RUN, DELAY_LEAD_SHIFT),
            (64, 64, 5),
            "ADR-0216's window, run and lead"
        );
        let critic = Some(ValueCritic { shift: 9, scale: 2 });
        let with = |kept, reads, critic, window| {
            Executor::<8>::new(Config {
                units: 3,
                train_capacity: 8,
                critic,
                critic_window_ticks: window,
                kept_addresses: kept,
                reads_its_delay: reads,
                ..Config::default()
            })
        };
        for (kept, critic, window) in [(0, None, 0), (0, critic, 20), (2, critic, 20)] {
            let mut exec = with(kept, false, critic, window).unwrap();
            assert!(!exec.reads_its_delay());
            assert_eq!(
                (exec.delay_age(), exec.delay_score(0), exec.delay_run()),
                (None, None, None)
            );
            assert_eq!(exec.delay_weight(0, 0), None);
            if kept != 0 {
                exec.keep_address([0u32]).unwrap();
            }
            let before = (weights(&exec), *exec.modulator(), exec.kept_at(0).is_some());
            assert_eq!(exec.reward_untold(ONE), Err(AddressError::DelayUnread));
            assert_eq!(
                (weights(&exec), *exec.modulator(), exec.kept_at(0).is_some()),
                before,
                "refused, nothing changed"
            );
        }
        for (critic, window) in [(None, 0), (critic, 0), (critic, 20)] {
            assert!(
                matches!(
                    with(0, true, critic, window),
                    Err(ConfigError::DelayReadWithoutKept)
                ),
                "{critic:?} {window}"
            );
        }
        for kept in 1..=KEPT_ADDRESSES_MAX {
            let mut exec = with(kept, true, critic, 20).unwrap();
            for (unit, weight) in exec.units_mut().iter_mut().zip([5i16, -7, 300]) {
                unit.value_weight = weight;
            }
            assert!(exec.reads_its_delay());
            assert_eq!((exec.delay_age(), exec.delay_run()), (Some(0), None));
            for age in 0..=kept {
                assert_eq!(exec.delay_score(age), None, "{kept} {age}");
                assert_eq!(
                    (0..3)
                        .map(|u| exec.delay_weight(age, u))
                        .collect::<Vec<_>>(),
                    [Some(5), Some(-7), Some(300)],
                    "{kept} {age}: the engine's before the first reward"
                );
            }
            assert_eq!(exec.delay_weight(0, 3), None, "outside the arena");
            assert_eq!(exec.delay_weight(kept.saturating_add(1), 0), None);
            assert_eq!(exec.delay_score(kept.saturating_add(1)), None);
            // Before any close no age reads, and the reward reaches nothing.
            assert_eq!(exec.reward_untold(ONE), Ok(None));
            assert_eq!(
                (exec.modulator().dopamine_rpe, exec.prediction()),
                (0, None)
            );
            assert_eq!(weights(&exec), [5, -7, 300], "no weight moved");
        }
        assert!(matches!(
            with(KEPT_ADDRESSES_MAX.saturating_add(1), true, critic, 20),
            Err(ConfigError::KeptAddressesOutOfRange)
        ));
    }

    /// The leader at its edges (ADR-0216): below every other score by the lead exactly leads, one
    /// short does not; a tie at the lowest is no leader, at a lead of zero too; no leader until
    /// every age has a score.
    #[test]
    fn an_age_leads_below_every_other_by_the_lead_and_none_before_every_score() {
        assert_eq!(delay_leader(&[Some(100), Some(90), Some(100)], 10), Some(1));
        assert_eq!(delay_leader(&[Some(100), Some(91), Some(100)], 10), None);
        assert_eq!(delay_leader(&[Some(100), Some(90), Some(99)], 10), None);
        assert_eq!(delay_leader(&[Some(5), Some(90), Some(99)], 10), Some(0));
        assert_eq!(delay_leader(&[Some(90), Some(90), Some(200)], 10), None);
        assert_eq!(delay_leader(&[Some(90), Some(90), Some(200)], 0), None);
        assert_eq!(delay_leader(&[Some(200), Some(90), Some(90)], 0), None);
        assert_eq!(delay_leader(&[Some(89), Some(90)], 0), Some(0));
        assert_eq!(delay_leader(&[Some(90), Some(89)], 1), Some(1));
        assert_eq!(delay_leader(&[Some(200), Some(90), None], 10), None);
        assert_eq!(delay_leader(&[None, Some(0)], 0), None);
        assert_eq!(delay_leader(&[Some(0), None], 0), None);
        assert_eq!(delay_leader(&[None], 0), None);
        assert_eq!(delay_leader(&[], 0), None);
        assert_eq!(delay_leader(&[Some(u64::MAX - 1), Some(u64::MAX)], 2), None);
        assert_eq!(
            delay_leader(&[Some(u64::MAX - 2), Some(u64::MAX)], 2),
            Some(0)
        );
        assert_eq!(
            delay_leader(&[Some(0), Some(u64::MAX), Some(1)], 1),
            Some(0),
            "one other at the lead's edge"
        );
    }

    /// The run at its edges (ADR-0216): the same age other than zero leading again adds one, to
    /// its width's end; another such age leading starts again at one; age zero leading, or no
    /// age, ends it, a run of 63 included.
    #[test]
    fn the_run_counts_one_age_leading_again_and_starts_again_at_another() {
        assert_eq!(delay_run_after(None, Some(1)), Some((1, 1)));
        assert_eq!(delay_run_after(None, Some(4)), Some((4, 1)));
        assert_eq!(delay_run_after(Some((1, 5)), Some(1)), Some((1, 6)));
        assert_eq!(
            delay_run_after(Some((2, 63)), Some(2)),
            Some((2, DELAY_RUN))
        );
        assert_eq!(
            delay_run_after(Some((1, u8::MAX)), Some(1)),
            Some((1, u8::MAX))
        );
        assert_eq!(
            delay_run_after(Some((1, 5)), Some(2)),
            Some((2, 1)),
            "another age"
        );
        assert_eq!(
            delay_run_after(Some((2, 63)), Some(1)),
            Some((1, 1)),
            "another age"
        );
        assert_eq!(delay_run_after(Some((2, 63)), Some(0)), None, "age zero");
        assert_eq!(delay_run_after(None, Some(0)), None, "age zero");
        assert_eq!(delay_run_after(Some((1, 5)), None), None, "no leader");
        assert_eq!(delay_run_after(None, None), None);
    }

    /// The move made once (ADR-0216). Two armed units under one kept address: unit 1 kicked at
    /// every trial and unit 0 by a coin, the reward at each close after the first the sign of
    /// the coin of the trial before. Age one's critic reads that trial's counts and learns the
    /// reward; age zero's reads an independent coin. No move comes before every age has
    /// `DELAY_WINDOW` rewards and age one has led at `DELAY_RUN` running; then the engine moves
    /// to age one at the reward that completes the run, whose entry age zero had spent, so that
    /// reward reaches nothing. Then the reward follows each trial's own coin, age zero's critic
    /// takes the lead, and the engine does not move again.
    #[test]
    fn the_engine_moves_once_to_the_age_that_leads_and_never_again() {
        let mut exec = Executor::<8>::new(Config {
            units: 2,
            nodes_per_worker: 128,
            train_capacity: 64,
            critic: Some(ValueCritic { shift: 4, scale: 0 }),
            critic_window_ticks: 400,
            kept_addresses: 1,
            reads_its_delay: true,
            ..Config::default()
        })
        .unwrap();
        for unit in exec.units_mut() {
            unit.v_thresh = THRESHOLD_BASE;
            unit.stp_u_rel = STP_U;
            unit.stp_r_ves = STP_MAX;
        }
        let earliest = DELAY_WINDOW + usize::from(DELAY_RUN) - 1;
        let mut rng = Lcg::new(0x0217);
        let mut coins: Vec<bool> = Vec::new();
        let mut moved_at = None;
        for t in 0..1_000usize {
            let coin = rng.below(2) == 0;
            coins.push(coin);
            kicked(&mut exec, &[coin, true], 400);
            exec.keep_address([0u32]).unwrap();
            let Some(&before) = t.checked_sub(1).and_then(|u| coins.get(u)) else {
                continue;
            };
            let got = exec.reward_untold(if before { ONE } else { -ONE });
            if exec.delay_age() == Some(1) {
                assert!(t >= earliest, "{t}: no move before {earliest} rewards");
                assert_eq!(exec.delay_run(), Some((1, DELAY_RUN)), "{t}");
                assert_eq!(got, Ok(None), "{t}: age one's entry spent at age zero");
                moved_at = Some(t);
                break;
            }
            assert_eq!(exec.delay_age(), Some(0), "{t}");
            assert!(matches!(got, Ok(Some(_))), "{t}");
            if t < DELAY_WINDOW {
                assert_eq!(
                    (exec.delay_score(0), exec.delay_score(1), exec.delay_run()),
                    (None, None, None),
                    "{t}: no score before {DELAY_WINDOW} rewards"
                );
            }
        }
        let moved_at = moved_at.expect("the engine moved to age one");
        assert!(moved_at < 400, "it moved early: {moved_at}");
        // From here the reward follows each trial's own coin, age zero's to read.
        let mut zero_led = 0u32;
        for t in 0..1_000usize {
            let coin = rng.below(2) == 0;
            kicked(&mut exec, &[coin, true], 400);
            exec.keep_address([0u32]).unwrap();
            let got = exec.reward_untold(if coin { ONE } else { -ONE });
            assert!(matches!(got, Ok(Some(_))), "{t}: age one's entry is live");
            assert_eq!(exec.delay_age(), Some(1), "{t}: never moves again");
            assert_eq!(exec.delay_run(), Some((1, DELAY_RUN)), "{t}: as it stood");
            if let (Some(zero), Some(one)) = (exec.delay_score(0), exec.delay_score(1)) {
                if zero.saturating_add(2 * ONE as u64) <= one {
                    zero_led = zero_led.saturating_add(1);
                }
            }
        }
        assert!(zero_led > 500, "age zero led by the lead: {zero_led}");
    }

    /// The oracle's entry: the counts a close found, the targets it was given, whether its reward
    /// is still to come, and whether a close wrote it.
    type Entry = (Vec<u32>, Vec<bool>, bool, bool);

    /// The oracle's score of an age: the sum of its last 64 errors' magnitudes, once it has 64.
    fn score_of(errors: &[u32]) -> Option<u64> {
        let last = errors.get(errors.len().checked_sub(64)?..)?;
        Some(
            last.iter()
                .fold(0u64, |sum, &e| sum.saturating_add(u64::from(e))),
        )
    }

    #[test]
    fn every_reward_given_without_its_age_is_the_oracle_s() {
        let mut rng = Lcg::new(0x0216);
        let (mut read_short, mut lost, mut told, mut broken) = (0u32, 0u32, 0u32, 0u32);
        let mut moves = [0u32; DELAY_AGES];
        for walk in 0..48u32 {
            let kept = 1u8.saturating_add(rng.below(u32::from(KEPT_ADDRESSES_MAX)) as u8);
            // Half the walks learnable: a unit kicked at every trial, 400 ticks a trial, a critic
            // that learns within the walk, and the reward following an age above zero.
            let learnable = walk % 2 == 0;
            let mut config = drawn(&mut rng, kept, true);
            if learnable {
                config.critic = Some(ValueCritic {
                    shift: 4u8.saturating_add(rng.below(2) as u8),
                    scale: 0,
                });
                config.critic_window_ticks = 400;
            }
            let (critic, whole) = (config.critic.unwrap(), config.whole_punishment);
            let due = if learnable {
                1usize.saturating_add(rng.below(u32::from(kept)) as usize)
            } else {
                rng.below(u32::from(kept) + 1) as usize
            };
            let mut exec = armed(config, &mut rng);
            let len = usize::from(kept) + 1;
            let blank: Entry = (
                vec![0; UNITS as usize],
                vec![false; UNITS as usize],
                false,
                false,
            );
            let mut ring: Vec<Entry> = vec![blank; len];
            let mut newest = 0usize;
            let mut gone = 0u64;
            let mut engine = weights(&exec);
            let mut side: Vec<Vec<i16>> = vec![engine.clone(); len];
            let mut started = false;
            let mut in_use = 0usize;
            let mut errors: Vec<Vec<u32>> = vec![Vec::new(); len];
            let mut run: Option<(u8, u8)> = None;
            let mut address = (vec![true; UNITS as usize], vec![true; UNITS as usize]);
            for step in 0..400u32 {
                let at = format!("walk {walk} step {step}");
                // A trial and its close, the targets drawn, one past the arena at times.
                if learnable {
                    let pattern: Vec<bool> =
                        (0..UNITS).map(|u| u == 1 || rng.below(2) == 0).collect();
                    kicked(&mut exec, &pattern, 400);
                } else {
                    trial(&mut exec, &mut rng);
                }
                let targets: Vec<u32> = (0..=UNITS)
                    .filter(|&u| match u {
                        UNITS => rng.below(32) == 0,
                        _ => rng.below(2) == 0,
                    })
                    .collect();
                let counts = exec.features().to_vec();
                if targets.contains(&UNITS) {
                    assert_eq!(
                        exec.keep_address(targets.iter().copied()),
                        Err(AddressError::NoSuchUnit),
                        "{at}"
                    );
                } else {
                    assert_eq!(exec.keep_address(targets.iter().copied()), Ok(()), "{at}");
                    newest = (newest + 1) % len;
                    if ring[newest].2 {
                        gone += 1;
                    }
                    ring[newest] = (
                        counts,
                        (0..UNITS).map(|u| targets.contains(&u)).collect(),
                        true,
                        true,
                    );
                }
                // Its reward, or two, and at times one of an age.
                for _ in 0..if rng.below(16) == 0 { 2 } else { 1 } {
                    let mut modulator = *exec.modulator();
                    let reading = exec.prediction();
                    let live = exec.features().to_vec();
                    let opened = exec.window_opened();
                    if rng.below(12) == 0 {
                        told += 1;
                        let age = rng.below(u32::from(kept) + 1) as usize;
                        let reward = reward_of(&mut rng);
                        let slot = (newest + len - age) % len;
                        let got = exec.reward_kept(reward, age as u8);
                        if ring[slot].2 {
                            ring[slot].2 = false;
                            let (counts, targets) = (ring[slot].0.clone(), ring[slot].1.clone());
                            let value = critic
                                .value_q16(engine.iter().copied().zip(counts.iter().copied()));
                            let error = ValueCritic::error_q16(reward, value);
                            engine = engine
                                .iter()
                                .zip(&counts)
                                .map(|(&w, &c)| critic.step(w, error, c))
                                .collect();
                            let received = if whole {
                                ValueCritic::received_q16(reward, value)
                            } else {
                                error
                            };
                            assert_eq!(got, Ok(modulator.reward(received)), "{at}: told");
                            address = (counts.iter().map(|&c| c != 0).collect(), targets);
                        } else {
                            assert_eq!(got, Err(AddressError::NoKeptAddress), "{at}: told");
                            assert_eq!(exec.prediction(), reading, "{at}: refused");
                        }
                    } else {
                        let due_entry = &ring[(newest + len - due) % len];
                        let reward = if rng.below(64) == 0 {
                            reward_of(&mut rng)
                        } else if due_entry.3 && due_entry.0[0] != 0 {
                            ONE
                        } else {
                            -ONE
                        };
                        let got = exec.reward_untold(reward);
                        if !started {
                            for weights in &mut side {
                                weights.clone_from(&engine);
                            }
                            started = true;
                        }
                        let mut read: Vec<Option<(i32, i32)>> = vec![None; len];
                        for age in 0..len {
                            let entry = &ring[(newest + len - age) % len];
                            if !entry.3 {
                                read_short += 1;
                                continue;
                            }
                            let w = if age == in_use {
                                &mut engine
                            } else {
                                &mut side[age]
                            };
                            let value =
                                critic.value_q16(w.iter().copied().zip(entry.0.iter().copied()));
                            let error = ValueCritic::error_q16(reward, value);
                            *w = w
                                .iter()
                                .zip(&entry.0)
                                .map(|(&w, &c)| critic.step(w, error, c))
                                .collect();
                            errors[age].push(error.unsigned_abs());
                            read[age] = Some((value, error));
                        }
                        if in_use == 0 {
                            // The leader from the text: every age scored, one strictly lowest and
                            // below every other by 64 times a thirty-second of the reward.
                            let scores: Option<Vec<u64>> =
                                errors.iter().map(|e| score_of(e)).collect();
                            let lead = u64::from(reward.unsigned_abs()) * 64 / 32;
                            let leader = scores.and_then(|scores| {
                                (0..len).find(|&a| {
                                    (0..len).filter(|&b| b != a).all(|b| {
                                        scores[b] > scores[a] && scores[b] - scores[a] >= lead
                                    })
                                })
                            });
                            let before = run;
                            run = match (leader, run) {
                                (Some(a), Some((b, n))) if a != 0 && a == usize::from(b) => {
                                    Some((b, n + 1))
                                }
                                (Some(a), _) if a != 0 => Some((a as u8, 1)),
                                _ => None,
                            };
                            if before.is_some() && run.is_none_or(|(_, n)| n == 1) {
                                broken += 1;
                            }
                            if let Some((a, 64)) = run {
                                moves[usize::from(a)] += 1;
                                side[0] = engine.clone();
                                engine = side[usize::from(a)].clone();
                                in_use = usize::from(a);
                            }
                        }
                        let slot = (newest + len - in_use) % len;
                        if ring[slot].2 {
                            ring[slot].2 = false;
                            let (value, error) = read[in_use].unwrap();
                            let received = if whole {
                                ValueCritic::received_q16(reward, value)
                            } else {
                                error
                            };
                            assert_eq!(got, Ok(Some(modulator.reward(received))), "{at}");
                            assert_eq!(
                                exec.prediction(),
                                Some(Prediction {
                                    value_q16: value,
                                    error_q16: error,
                                    received_q16: received
                                }),
                                "{at}"
                            );
                            address = (
                                ring[slot].0.iter().map(|&c| c != 0).collect(),
                                ring[slot].1.clone(),
                            );
                        } else {
                            lost += 1;
                            assert_eq!(got, Ok(None), "{at}: no live entry at the age in use");
                            assert_eq!(exec.prediction(), reading, "{at}: no reading");
                        }
                    }
                    assert_eq!(exec.modulator(), &modulator, "{at}: the signal");
                    assert_eq!(
                        (exec.features(), exec.window_opened()),
                        (&live[..], opened),
                        "{at}: the live counts and the window untouched"
                    );
                }
                assert_eq!(weights(&exec), engine, "{at}: the engine's critic");
                for age in 0..len {
                    let expected: Vec<Option<i16>> = (0..UNITS as usize)
                        .map(|u| {
                            Some(if age == in_use || !started {
                                engine[u]
                            } else {
                                side[age][u]
                            })
                        })
                        .collect();
                    assert_eq!(
                        (0..UNITS as usize)
                            .map(|u| exec.delay_weight(age as u8, u))
                            .collect::<Vec<_>>(),
                        expected,
                        "{at}: age {age}'s critic"
                    );
                    assert_eq!(
                        exec.delay_score(age as u8),
                        score_of(&errors[age]),
                        "{at}: age {age}'s score"
                    );
                    let entry = &ring[(newest + len - age) % len];
                    assert_eq!(
                        exec.kept_at(age as u8),
                        entry.2.then_some((entry.0.as_slice(), entry.1.as_slice())),
                        "{at}: age {age}'s entry"
                    );
                }
                assert_eq!(
                    (exec.delay_age(), exec.delay_run()),
                    (Some(in_use as u8), run),
                    "{at}: the age in use and the run"
                );
                assert_eq!(sides(&exec), address, "{at}: the addressed set");
                assert_eq!(exec.kept_let_go(), gone, "{at}: let go");
            }
        }
        assert!(
            read_short > 20
                && lost > 50
                && told > 200
                && moves.iter().skip(1).take(2).all(|&m| m > 0)
                && moves.iter().sum::<u32>() > 8
                && broken > 20
                && moves[0] == 0,
            "the walk met every case: {read_short} ages unread, {lost} rewards lost, {told} told, moves {moves:?}, {broken} runs broken"
        );
    }

    #[test]
    fn on_time_with_no_move_a_reward_given_without_its_age_is_the_drawn_reward_bit_for_bit() {
        let mut rng = Lcg::new(0x2170);
        let mut moved = 0u32;
        for walk in 0..48u32 {
            let kept = 1u8.saturating_add(rng.below(u32::from(KEPT_ADDRESSES_MAX)) as u8);
            let config = drawn(&mut rng, kept, true);
            let twin = Config {
                kept_addresses: 0,
                reads_its_delay: false,
                ..config.clone()
            };
            let seed = rng.next_u64();
            let mut engines = [
                armed(twin, &mut Lcg::new(seed)),
                armed(config, &mut Lcg::new(seed)),
            ];
            // Fewer rewards than a score and a run take: no move can come.
            for step in 0..usize::from(DELAY_RUN) + DELAY_WINDOW - 2 {
                let at = format!("walk {walk} step {step}");
                let draw = rng.next_u64();
                for exec in &mut engines {
                    trial(exec, &mut Lcg::new(draw));
                }
                let targets: Vec<u32> = (0..UNITS).filter(|_| rng.below(2) == 0).collect();
                let reward = reward_of(&mut rng);
                let [drawing, reading] = &mut engines;
                drawing.address_drawn(targets.iter().copied()).unwrap();
                let signal = drawing.reward(reward);
                reading.keep_address(targets.iter().copied()).unwrap();
                assert_eq!(reading.reward_untold(reward), Ok(Some(signal)), "{at}");
                assert_eq!(reading.delay_age(), Some(0), "{at}");
                let state = |exec: &Executor<8>| {
                    (
                        weights(exec),
                        exec.features().to_vec(),
                        exec.window_opened(),
                        exec.prediction(),
                        *exec.modulator(),
                        sides(exec),
                        exec.ticks(),
                    )
                };
                assert_eq!(state(&engines[0]), state(&engines[1]), "{at}");
                if engines[0].prediction().is_some_and(|p| p.error_q16 != 0) {
                    moved = moved.wrapping_add(1);
                }
            }
        }
        assert!(moved > 2_000, "the rewards moved the reading: {moved}");
    }

    #[test]
    fn a_reward_that_names_its_age_is_the_one_without_the_rule_bit_for_bit() {
        let mut rng = Lcg::new(0x0812);
        let (mut given, mut refused) = (0u32, 0u32);
        for walk in 0..48u32 {
            let kept = 1u8.saturating_add(rng.below(u32::from(KEPT_ADDRESSES_MAX)) as u8);
            let config = drawn(&mut rng, kept, true);
            let unset = Config {
                reads_its_delay: false,
                ..config.clone()
            };
            let seed = rng.next_u64();
            let mut engines = [
                armed(unset, &mut Lcg::new(seed)),
                armed(config, &mut Lcg::new(seed)),
            ];
            for step in 0..48u32 {
                let at = format!("walk {walk} step {step}");
                let draw = rng.next_u64();
                for exec in &mut engines {
                    trial(exec, &mut Lcg::new(draw));
                }
                let targets: Vec<u32> = (0..UNITS).filter(|_| rng.below(2) == 0).collect();
                let reward = reward_of(&mut rng);
                let age = rng.below(u32::from(kept) + 2) as u8;
                let got: Vec<Result<i32, AddressError>> = engines
                    .iter_mut()
                    .map(|exec| {
                        exec.keep_address(targets.iter().copied()).unwrap();
                        exec.reward_kept(reward, age)
                    })
                    .collect();
                assert_eq!(got[0], got[1], "{at}");
                if got[0].is_ok() {
                    given = given.wrapping_add(1);
                } else {
                    refused = refused.wrapping_add(1);
                }
                let state = |exec: &Executor<8>| {
                    (
                        weights(exec),
                        exec.features().to_vec(),
                        exec.window_opened(),
                        exec.prediction(),
                        *exec.modulator(),
                        sides(exec),
                        exec.kept_let_go(),
                        (0..=kept)
                            .map(|a| exec.kept_at(a).map(|(c, t)| (c.to_vec(), t.to_vec())))
                            .collect::<Vec<_>>(),
                    )
                };
                assert_eq!(state(&engines[0]), state(&engines[1]), "{at}");
                assert_eq!(engines[1].delay_age(), Some(0), "{at}");
            }
        }
        assert!(
            given > 1_000 && refused > 200,
            "{given} given, {refused} refused"
        );
    }
}

/// The zero sum at the stimulus and the depression's exponent on the engine (ADR-0226, ADR-0228).
/// By hand at the edges; the carried scaling over the lattice and a walk against an exact rational
/// oracle; the engine tick by tick against a twin on which the host applies that oracle, under
/// both services; and unset, the engine as it is, bit for bit.
#[cfg(test)]
mod zero_sum_prop {
    use super::*;
    use crate::image::Image;
    use menteion_core::{
        FLAG_INHIBITORY, ISTDP_ALPHA_Q1_15, STDP_A_PLUS_Q1_15, STP_MAX, STP_U,
        synaptic_efficacy_q16,
    };
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../testkit/prop.rs"
    ));

    /// Messages that make an armed unit at rest fire.
    const KICK: usize = 13;

    /// The exact rule (ADR-0226): the weight $\lfloor w V / S + f / 2^{32} \rfloor$ at most the
    /// rail, and the fraction left over truncated to $2^{-32}$ LSB, from the rational over the
    /// common denominator $S 2^{32}$. `sum` above zero, `weight` and `value` at or above zero.
    fn oracle(weight: i16, value: i64, sum: i64, fraction: u32) -> (i16, u32) {
        let (w, v, s, f) = (
            i128::from(weight),
            i128::from(value),
            i128::from(sum),
            i128::from(fraction),
        );
        let one = 1i128 << 32;
        let denominator = s.saturating_mul(one);
        let numerator = w
            .saturating_mul(v)
            .saturating_mul(one)
            .saturating_add(f.saturating_mul(s));
        let whole = numerator.div_euclid(denominator);
        let left = numerator.rem_euclid(denominator).div_euclid(s);
        (
            whole.min(i128::from(i16::MAX)) as i16,
            u32::try_from(left).expect("below one"),
        )
    }

    /// A weight's rounds by hand: a quotient just under one, a carry from just under one, the
    /// rail, a weight of zero, a sum of zero, and a product past an `i64`.
    #[test]
    fn a_carried_weight_by_hand_at_its_edges() {
        // 4 000 × 15 999 / 16 000 is 3 999.75: carried from a half it loses an LSB in the third
        // round, three quarters left over.
        let (mut w, mut f) = (4_000i16, ZERO_SUM_HALF);
        let mut out = Vec::new();
        for _ in 0..3 {
            w = carried_weight(w, 15_999, 16_000, &mut f);
            out.push(w);
        }
        assert_eq!((out, f), (vec![4_000, 4_000, 3_999], 0xC000_0000));
        // A carry from just under one: 4 000.25 and 0.9999….
        let mut f = u32::MAX;
        assert_eq!(carried_weight(4_000, 16_001, 16_000, &mut f), 4_001);
        assert_eq!(f, 0x3FFF_FFFF);
        // At the rail the weight stays there and the fraction is kept: 34 814.9375 and a half.
        let mut f = ZERO_SUM_HALF;
        assert_eq!(carried_weight(i16::MAX, 17_000, 16_000, &mut f), i16::MAX);
        assert_eq!(f, 0x7000_0000);
        // A weight of zero stays zero, its fraction as it was.
        let mut f = 0xC000_0000;
        assert_eq!(carried_weight(0, 16_001, 16_000, &mut f), 0);
        assert_eq!(f, 0xC000_0000);
        // A sum of zero moves nothing.
        let mut f = 0x1234_5678;
        assert_eq!(carried_weight(7, 16_000, 0, &mut f), 7);
        assert_eq!(f, 0x1234_5678);
        // Past an `i64`: the rail times 2^43 over 2^44 is 16 383.5, and a half carries it.
        let mut f = ZERO_SUM_HALF;
        assert_eq!(
            carried_weight(i16::MAX, 1i64 << 43, 1i64 << 44, &mut f),
            16_384
        );
        assert_eq!(f, 0);
    }

    /// The carried scaling over the lattice and a walk: the weight and the fraction are the exact
    /// rational's, at or above zero and never past the rail, at every width of a group's sum.
    #[test]
    fn every_carried_weight_is_the_exact_rational_s() {
        let check = |weight: i16, value: i64, sum: i64, fraction: u32| {
            let mut f = fraction;
            let w = carried_weight(weight, value, sum, &mut f);
            assert_eq!(
                (w, f),
                oracle(weight, value, sum, fraction),
                "{weight} {value} {sum} {fraction}"
            );
            assert!(w >= 0, "{weight} {value} {sum} {fraction}");
        };
        let weights: Vec<i16> = I16_LATTICE.iter().copied().filter(|&w| w >= 0).collect();
        let wide: Vec<i64> = I32_LATTICE
            .iter()
            .map(|&x| i64::from(x).saturating_abs())
            .chain([1 << 31, 1 << 40, 1 << 43, i64::MAX])
            .collect();
        for &weight in &weights {
            for &value in &wide {
                for &sum in wide.iter().filter(|&&s| s > 0) {
                    for &fraction in U32_LATTICE.iter() {
                        check(weight, value, sum, fraction);
                    }
                }
            }
        }
        let mut rng = Lcg::new(0x0228);
        for _ in 0..200_000 {
            let sum = 1i64.saturating_add(i64::from(rng.next_u32()) >> rng.below(16));
            // Near the sum, as a round's is, or anywhere.
            let value = match rng.below(2) {
                0 => sum.saturating_add(i64::from(rng.next_i16() >> 8)).max(0),
                _ => i64::from(rng.next_u32()),
            };
            check(rng.next_i16().max(0), value, sum, rng.next_u32());
        }
    }

    /// A hand-made arena: one chain of two blocks, 30 000, 12 000 and 100 onto target 5 and 6 000
    /// onto unit 7, and another chain's 10 000 onto target 5, with every fraction at a half.
    fn arena() -> (Vec<SynapseBlock>, Vec<bool>, Vec<u32>) {
        let mut blocks = vec![SynapseBlock::new(); 3];
        assert!(blocks[0].set_synapse(0, 5, 30_000, 0, false));
        assert!(blocks[0].set_synapse(1, 7, 6_000, 0, false));
        assert!(blocks[0].set_synapse(2, 5, 12_000, 0, false));
        assert!(blocks[0].link(1));
        assert!(blocks[1].set_synapse(3, 5, 100, 0, false));
        assert!(blocks[2].set_synapse(0, 5, 10_000, 0, false));
        let mut targets = vec![false; 8];
        targets[5] = true;
        (blocks, targets, vec![ZERO_SUM_HALF; 12])
    }

    fn weights_of(blocks: &[SynapseBlock]) -> Vec<i16> {
        blocks.iter().flat_map(|b| b.weights_q1_15).collect()
    }

    /// A group's round reads its sources' synapses onto the targets alone, scales each by the
    /// same quotient with its own fraction, either way, at most the rail, and leaves every other
    /// synapse and fraction; at its value, and at a sum of zero, nothing moves.
    #[test]
    fn a_round_scales_a_group_s_synapses_onto_the_targets_and_no_other() {
        let run = |value: i64| {
            let (mut blocks, targets, mut fractions) = arena();
            let mut at = Vec::new();
            zero_sum_round(&mut blocks, &[1], value, &targets, &mut fractions, &mut at);
            (weights_of(&blocks), fractions)
        };
        // The group's sum is 42 100. Held at 42 099, from a half: 29 999.29 + 0.5 stays below
        // 30 000, 11 999.71 + 0.5 and 99.998 + 0.5 carry one each.
        let (w, f) = run(42_099);
        assert_eq!(
            w,
            vec![29_999, 6_000, 12_000, 0, 0, 0, 0, 100, 10_000, 0, 0, 0]
        );
        assert_eq!(f[0], oracle(30_000, 42_099, 42_100, ZERO_SUM_HALF).1);
        assert_eq!(f[2], oracle(12_000, 42_099, 42_100, ZERO_SUM_HALF).1);
        assert_eq!(f[7], oracle(100, 42_099, 42_100, ZERO_SUM_HALF).1);
        for slot in [1, 3, 4, 5, 6, 8, 9, 10, 11] {
            assert_eq!(f[slot], ZERO_SUM_HALF, "slot {slot} is not the group's");
        }
        // Scaled up by 116 %, the first at the rail.
        let (w, _) = run(48_836);
        assert_eq!(
            (w[0], w[1], w[2], w[7], w[8]),
            (i16::MAX, 6_000, 13_920, 116, 10_000)
        );
        // At its value nothing moves, no fraction either.
        let (w, f) = run(42_100);
        assert_eq!(w, weights_of(&arena().0));
        assert_eq!(f, vec![ZERO_SUM_HALF; 12]);
        // A sum of zero is left.
        let (mut blocks, targets, mut fractions) = arena();
        blocks[0].weights_q1_15[0] = 0;
        blocks[0].weights_q1_15[2] = 0;
        blocks[1].weights_q1_15[3] = 0;
        let before = weights_of(&blocks);
        let mut at = Vec::new();
        zero_sum_round(&mut blocks, &[1], 500, &targets, &mut fractions, &mut at);
        assert_eq!(weights_of(&blocks), before);
        assert_eq!(fractions, vec![ZERO_SUM_HALF; 12]);
    }

    /// `coupling_sum` on the hand-made arena (ADR-0232), unit 0 on its first chain, unit 1 on its
    /// second, unit 2 inhibitory and unit 3 on none: each sum by hand, over one target and two,
    /// one chain and two; and what `set_zero_sum` refuses of a group, none.
    #[test]
    fn a_coupling_sum_is_the_sum_by_hand_and_refuses_what_a_group_refuses() {
        let mut exec = Executor::<64>::new(Config {
            units: 8,
            blocks: 3,
            ..Config::default()
        })
        .expect("a valid configuration");
        exec.blocks_mut().copy_from_slice(&arena().0);
        assert!(exec.units_mut()[0].set_first_block(0));
        assert!(exec.units_mut()[1].set_first_block(2));
        assert!(exec.units_mut()[2].set_first_block(2));
        exec.units_mut()[2].flags = FLAG_INHIBITORY;
        let sums: [(&[u32], &[u32], i64); 8] = [
            (&[0], &[5], 42_100),
            (&[0], &[7], 6_000),
            (&[0], &[5, 7], 48_100),
            (&[1], &[5], 10_000),
            (&[1], &[7], 0),
            (&[0, 1], &[5], 52_100),
            (&[1, 0], &[7, 5, 5], 58_100),
            (&[3], &[5], 0),
        ];
        for (sources, targets, sum) in sums {
            assert_eq!(
                exec.coupling_sum(sources, targets),
                Some(sum),
                "{sources:?} {targets:?}"
            );
        }
        assert_eq!(exec.coupling_sum(&[], &[5]), Some(0), "no source");
        assert_eq!(exec.coupling_sum(&[0], &[]), Some(0), "no target");
        let refused: [(&[u32], &[u32]); 5] = [
            (&[8], &[5]),
            (&[0, 8], &[5]),
            (&[0], &[8]),
            (&[2], &[5]),
            (&[0, 0], &[5]),
        ];
        for (sources, targets) in refused {
            assert_eq!(
                exec.coupling_sum(sources, targets),
                None,
                "{sources:?} {targets:?}"
            );
        }
    }

    /// `set_zero_sum` refuses whole, nothing changed, and every set starts the fractions again.
    #[test]
    fn a_zero_sum_is_set_whole_or_refused_and_its_fractions_start_at_a_half() {
        let mut exec = Executor::<64>::new(Config {
            units: 70,
            blocks: 2,
            ..Config::default()
        })
        .expect("a valid configuration");
        exec.units_mut()[1].flags = FLAG_INHIBITORY;
        assert_eq!(exec.zero_sum_groups(), 0, "unset by default");
        assert!(
            exec.set_zero_sum(&[(&[0], 1), (&[3], 0)], &[2]),
            "a value of zero is one"
        );
        let state = |e: &Executor<64>| {
            (
                e.zero_sum_groups.clone(),
                e.zero_sum_targets.clone(),
                e.zero_sum_fractions.len(),
                e.shared.zero_sum_group(0),
                e.shared.zero_sum_group(3),
                e.shared.zero_sum_group(2),
            )
        };
        assert_eq!(exec.zero_sum_groups(), 2);
        let set = state(&exec);
        assert_eq!((set.2, set.3, set.4, set.5), (8, 1, 2, 0));
        assert!(set.1[2] && !set.1[0], "the target alone");
        type Groups<'a> = &'a [(&'a [u32], i64)];
        let refused: [(Groups, &[u32]); 6] = [
            (&[(&[0, 99], 1)], &[2]),
            (&[(&[0, 1], 1)], &[2]),
            (&[(&[0], 1), (&[0], 1)], &[2]),
            (&[(&[0, 0], 1)], &[2]),
            (&[(&[0], -1)], &[2]),
            (&[(&[0], 1)], &[99]),
        ];
        for (groups, targets) in refused {
            assert!(
                !exec.set_zero_sum(groups, targets),
                "{groups:?} {targets:?}"
            );
            assert_eq!(state(&exec), set, "{groups:?} {targets:?}: refused whole");
        }
        // Every group has a bit: 64 of them, the most, are set, and 65 refused.
        let units: Vec<u32> = (2..68).collect();
        let many: Vec<(&[u32], i64)> = units
            .iter()
            .map(|u| (core::slice::from_ref(u), 1))
            .collect();
        assert!(!exec.set_zero_sum(&many[..65], &[69]));
        assert_eq!(state(&exec), set);
        assert!(exec.set_zero_sum(&many[..ZERO_SUM_GROUPS_MAX], &[69]));
        assert_eq!(exec.zero_sum_groups(), ZERO_SUM_GROUPS_MAX);
        assert_eq!(exec.shared.zero_sum_group(65), 64);
        // A set starts every fraction at a half, again.
        exec.zero_sum_fractions[3] = 7;
        assert!(exec.set_zero_sum(&many[..2], &[69]));
        assert_eq!(exec.zero_sum_fractions, vec![ZERO_SUM_HALF; 8]);
        // No groups unsets it, whatever targets it is given, and holds nothing.
        assert!(exec.set_zero_sum(&[], &[69]));
        assert_eq!(exec.zero_sum_groups(), 0);
        assert_eq!(exec.shared.zero_sum_group(2), 0);
        assert!(exec.zero_sum_targets.is_empty() && exec.zero_sum_fractions.is_empty());
        assert!(
            !exec.shared.zero_sum_target[69].load(Ordering::Relaxed),
            "no unit left a target"
        );
    }

    /// Every unit armed.
    fn arm(exec: &mut Executor<64>) {
        for unit in exec.units_mut() {
            unit.v_thresh = THRESHOLD_BASE;
            unit.stp_u_rel = STP_U;
            unit.stp_r_ves = STP_MAX;
        }
    }

    /// Unit 0 onto units 2, 3 and 4 and unit 1 onto units 2 and 5, each its own block, every
    /// weight 8 000, delays one tick; the modulation at rest 1.0, so that a spike consolidates
    /// its whole trace.
    fn engine(exponent: DepressionExponent) -> Executor<64> {
        let mut exec = Executor::<64>::new(Config {
            workers: 2,
            units: 6,
            blocks: 2,
            nodes_per_worker: 256,
            injector_capacity: 64,
            depression_exponent: exponent,
            ..Config::default()
        })
        .expect("a valid configuration");
        for (slot, target) in [2u32, 3, 4].into_iter().enumerate() {
            assert!(exec.blocks_mut()[0].set_synapse(slot, target, 8_000, 1, false));
        }
        for (slot, target) in [2u32, 5].into_iter().enumerate() {
            assert!(exec.blocks_mut()[1].set_synapse(slot, target, 8_000, 1, false));
        }
        arm(&mut exec);
        assert!(exec.units_mut()[0].set_first_block(0));
        assert!(exec.units_mut()[1].set_first_block(1));
        exec
    }

    /// Makes `unit` fire, and runs until it has.
    fn fire(exec: &mut Executor<64>, unit: u32) -> u32 {
        let before = exec.units()[unit as usize].last_soma_spike_tick;
        let inject = exec.injector();
        let strong = synaptic_efficacy_q16(i16::MAX, STP_U, STP_MAX);
        for _ in 0..KICK {
            inject.inject(unit, spike_message(strong, false)).unwrap();
        }
        for _ in 0..40 {
            exec.tick();
            let now = exec.units()[unit as usize].last_soma_spike_tick;
            if now != before {
                return now;
            }
        }
        panic!("unit {unit} did not fire");
    }

    /// A pairing: `pre`, `post` twenty ticks on, and `pre` again once it can fire, whose
    /// consolidation moves its synapse onto `post`, potentiated.
    fn pairing(exec: &mut Executor<64>, pre: u32, post: u32) {
        fire(exec, pre);
        exec.run(20);
        fire(exec, post);
        exec.run(4_000);
        fire(exec, pre);
        exec.run(4);
    }

    /// A group moved is scaled back after its tick, its synapses onto the targets alone; a group
    /// no consolidation touched is left whatever its value says; a move onto no target touches
    /// none, nor does a spike that moves nothing; and the bit taken leaves the next tick clean.
    #[test]
    fn a_group_is_held_after_a_tick_that_moved_it_and_no_other_is() {
        let sums = [16_000i64, 8_000];
        let held = |value: [i64; 2]| {
            let mut exec = engine(DepressionExponent::One);
            assert!(exec.set_zero_sum(&[(&[0], value[0]), (&[1], value[1])], &[2, 3]));
            exec
        };
        let weights = |exec: &Executor<64>| weights_of(exec.blocks());
        // The twin carries no zero sum, so its weights are the rule's.
        let mut twin = engine(DepressionExponent::One);
        pairing(&mut twin, 0, 2);
        let rule = weights(&twin);
        let rise = i64::from(rule[0]).saturating_sub(8_000);
        assert!(
            (1..=i64::from(STDP_A_PLUS_Q1_15)).contains(&rise),
            "the pairing moved the synapse onto unit 2: {rise}"
        );
        assert_eq!((rule[1], rule[2]), (8_000, 8_000));
        // Group 0 moved: its two synapses onto the targets scaled back to the sum it was; its
        // synapse onto unit 4 and group 1, held at more than its sum, left.
        let mut exec = held([sums[0], sums[1].saturating_add(99)]);
        pairing(&mut exec, 0, 2);
        let sum = sums[0].saturating_add(rise);
        let expected: Vec<i16> = [rule[0], rule[1]]
            .iter()
            .map(|&w| oracle(w, sums[0], sum, ZERO_SUM_HALF).0)
            .collect();
        let w = weights(&exec);
        assert_eq!(&w[..2], expected.as_slice());
        assert_eq!(&w[2..], &rule[2..], "no other synapse");
        assert_eq!(exec.shared.zero_sum_touched.load(Ordering::Relaxed), 0);
        assert_eq!(
            exec.zero_sum_fractions[0],
            oracle(rule[0], sums[0], sum, ZERO_SUM_HALF).1,
            "the fraction of slot 0 of block 0, kept"
        );
        // Group 1 moved alone, by its pairing: group 0, held away from its sum, left.
        let mut twin = engine(DepressionExponent::One);
        pairing(&mut twin, 1, 2);
        let rule = weights(&twin);
        let mut exec = held([sums[0].saturating_add(99), sums[1]]);
        pairing(&mut exec, 1, 2);
        let w = weights(&exec);
        assert_eq!(&w[..4], &rule[..4], "group 0 untouched");
        assert_eq!(
            w[4],
            oracle(rule[4], sums[1], i64::from(rule[4]), ZERO_SUM_HALF).0,
            "group 1's one synapse onto a target back at its value"
        );
        assert_eq!(w[5], rule[5], "its synapse onto no target");
        // A move onto no target alone: the pairing with unit 4 moves slot 2, and nothing is held
        // though the value is not the sum.
        let mut twin = engine(DepressionExponent::One);
        pairing(&mut twin, 0, 4);
        let rule = weights(&twin);
        assert!(rule[2] > 8_000, "the pairing moved the synapse onto unit 4");
        let mut exec = held([sums[0].saturating_add(99), sums[1]]);
        pairing(&mut exec, 0, 4);
        assert_eq!(weights(&exec), rule, "a move onto no target touches none");
        // A spike that moves nothing: no target has fired, so no slot has a trace.
        let mut exec = held([sums[0].saturating_add(99), sums[1].saturating_add(99)]);
        fire(&mut exec, 0);
        exec.run(4_000);
        fire(&mut exec, 0);
        exec.run(4);
        assert_eq!(
            weights(&exec),
            weights_of(engine(DepressionExponent::One).blocks())
        );
    }

    /// The top group's bit: the 64th group alone moved is the one scaled.
    #[test]
    fn the_last_group_has_its_own_bit() {
        const UNITS: usize = 66;
        let mut exec = Executor::<64>::new(Config {
            units: UNITS,
            blocks: 64,
            nodes_per_worker: 256,
            injector_capacity: 64,
            ..Config::default()
        })
        .expect("a valid configuration");
        let target = 65u32;
        for b in 0..64usize {
            assert!(exec.blocks_mut()[b].set_synapse(0, target, 8_000, 1, false));
        }
        arm(&mut exec);
        for b in 0..64u32 {
            assert!(exec.units_mut()[b as usize].set_first_block(b));
        }
        let units: Vec<u32> = (0..64).collect();
        let groups: Vec<(&[u32], i64)> = units
            .iter()
            .map(|u| (core::slice::from_ref(u), 8_100))
            .collect();
        assert!(exec.set_zero_sum(&groups, &[target]));
        pairing(&mut exec, 63, target);
        let w = weights_of(exec.blocks());
        assert_eq!(w[252], 8_100, "the 64th group, at its value");
        for b in 0..63usize {
            assert_eq!(w[b.saturating_mul(SYNAPSES_PER_BLOCK)], 8_000, "group {b}");
        }
    }

    const WALK_UNITS: usize = 8;
    const WALK_BLOCKS: usize = 5;
    /// A walk's network: units 0 and 1 in group A, unit 2 in group B, unit 3 excitatory in no
    /// group, unit 6 inhibitory; targets 4 and 5, unit 7 neither. Each is `(unit, first block)`.
    const WALK_FIRST: [(usize, u32); 5] = [(0, 0), (1, 2), (2, 3), (3, 4), (6, 1)];

    /// The walk's network under `config`, its weights from `seed`.
    fn walked(config: &Config, seed: u64, per_tick: bool) -> Executor<64> {
        let exec = Executor::<64>::new(config.clone()).expect("a valid configuration");
        let mut exec = if per_tick { exec.per_tick() } else { exec };
        let mut rng = Lcg::new(seed);
        for b in 0..WALK_BLOCKS {
            for (slot, target) in [4u32, 5, 7, 4].into_iter().enumerate() {
                let magnitude = 2_000i16.saturating_add((rng.next_u32() >> 19) as i16);
                let weight = if b == 1 {
                    0i16.saturating_sub(magnitude)
                } else {
                    magnitude
                };
                assert!(exec.blocks_mut()[b].set_synapse(slot, target, weight, 1, false));
            }
        }
        arm(&mut exec);
        exec.units_mut()[6].flags = FLAG_INHIBITORY;
        for (unit, block) in WALK_FIRST {
            assert!(exec.units_mut()[unit].set_first_block(block));
        }
        exec
    }

    /// The slots of `blocks` onto the walk's targets.
    fn walk_slots(exec: &Executor<64>, blocks: &[usize]) -> Vec<(usize, usize)> {
        let mut slots = Vec::new();
        for &b in blocks {
            for s in 0..SYNAPSES_PER_BLOCK {
                if matches!(exec.blocks()[b].target(s), Some(4 | 5)) {
                    slots.push((b, s));
                }
            }
        }
        slots
    }

    /// `slots`' weights on `exec`, summed.
    fn walk_sum(exec: &Executor<64>, slots: &[(usize, usize)]) -> i64 {
        slots.iter().fold(0i64, |sum, &(b, s)| {
            sum.saturating_add(i64::from(exec.blocks()[b].weights_q1_15[s]))
        })
    }

    /// The host's zero sum on a twin that holds none: after a tick that moved a group's synapse
    /// onto a target, the oracle at each such synapse, its fractions the host's own. Returns the
    /// rounds that scaled.
    fn host_round(
        twin: &mut Executor<64>,
        before: &[SynapseBlock],
        groups: &[(Vec<(usize, usize)>, i64); 2],
        fractions: &mut [u32],
    ) -> u32 {
        let mut scaled = 0u32;
        for (slots, value) in groups {
            let moved = slots
                .iter()
                .any(|&(b, s)| twin.blocks()[b].weights_q1_15[s] != before[b].weights_q1_15[s]);
            let sum = walk_sum(twin, slots);
            if !moved || sum == *value {
                continue;
            }
            scaled = scaled.saturating_add(1);
            for &(b, s) in slots {
                let place = b.saturating_mul(SYNAPSES_PER_BLOCK).saturating_add(s);
                let weight = twin.blocks()[b].weights_q1_15[s];
                let (w, f) = oracle(weight, *value, sum, fractions[place]);
                twin.blocks_mut()[b].weights_q1_15[s] = w;
                fractions[place] = f;
            }
        }
        scaled
    }

    /// Tick by tick over seeded walks, on one worker and two and under both services: the engine
    /// under the zero sum is the twin on which the host applies the exact rule, every record of
    /// the synapse arena after every tick; and an engine on which the groups were set and unset
    /// is the engine with none, bit for bit.
    #[test]
    fn over_a_walk_the_engine_s_zero_sum_is_the_host_s_exact_rule_tick_by_tick() {
        let strong = synaptic_efficacy_q16(i16::MAX, STP_U, STP_MAX);
        let mut scaled = 0u32;
        for walk in 0..12u64 {
            let mut rng = Lcg::new(0x0226_u64.wrapping_add(walk));
            let config = Config {
                workers: 1usize.saturating_add(walk as usize % 2),
                units: WALK_UNITS,
                blocks: WALK_BLOCKS,
                nodes_per_worker: 1_024,
                injector_capacity: 64,
                modulation_baseline_q16: rng.pick(&[0x4000, 0x8000, MODULATION_ONE_Q16]),
                ..Config::default()
            };
            let per_tick = walk % 3 == 0;
            let mut held = walked(&config, walk, per_tick);
            let mut twin = walked(&config, walk, per_tick);
            let mut unset = walked(&config, walk, per_tick);
            let mut plain = walked(&config, walk, per_tick);
            let a = walk_slots(&held, &[0, 2]);
            let b = walk_slots(&held, &[3]);
            let groups = [
                (a.clone(), walk_sum(&held, &a)),
                (b.clone(), walk_sum(&held, &b)),
            ];
            assert!(held.set_zero_sum(&[(&[0, 1], groups[0].1), (&[2], groups[1].1)], &[4, 5]));
            assert!(unset.set_zero_sum(&[(&[0], 1)], &[4]));
            assert!(unset.set_zero_sum(&[], &[]));
            let mut fractions = vec![ZERO_SUM_HALF; WALK_BLOCKS.saturating_mul(SYNAPSES_PER_BLOCK)];
            for tick in 0..4_000u32 {
                if rng.below(4) == 0 {
                    let unit = rng.below(WALK_UNITS as u32);
                    for exec in [&held, &twin, &unset, &plain] {
                        let inject = exec.injector();
                        for _ in 0..KICK {
                            inject.inject(unit, spike_message(strong, false)).unwrap();
                        }
                    }
                }
                let before: Vec<SynapseBlock> = twin.blocks().to_vec();
                held.tick();
                twin.tick();
                unset.tick();
                plain.tick();
                scaled =
                    scaled.saturating_add(host_round(&mut twin, &before, &groups, &mut fractions));
                assert_eq!(
                    held.blocks(),
                    twin.blocks(),
                    "walk {walk} tick {tick}: the host's exact rule"
                );
                assert_eq!(
                    held.zero_sum_fractions, fractions,
                    "walk {walk} tick {tick}"
                );
            }
            assert_eq!(
                held.coupling_sum(&[0, 1], &[4, 5]),
                Some(walk_sum(&held, &a)),
                "walk {walk}: the reader is the sum by hand"
            );
            assert_eq!(unset.blocks(), plain.blocks(), "walk {walk}: bit for bit");
            let spikes = |e: &Executor<64>| -> Vec<u32> {
                e.units().iter().map(|u| u.last_soma_spike_tick).collect()
            };
            assert_eq!(
                spikes(&unset),
                spikes(&plain),
                "walk {walk}: every unit too"
            );
        }
        assert!(scaled > 500, "the walks scaled: {scaled} rounds");
    }

    /// The configuration's exponent reaches every excitatory pairing; `One` is the default; and
    /// neither rule is in the image: an engine under either encodes the bytes of the engine
    /// without it, and one decoded takes its exponent from the configuration and no group.
    #[test]
    fn the_exponent_is_the_configuration_s_and_neither_rule_is_in_the_image() {
        assert_eq!(
            Config::default().depression_exponent,
            DepressionExponent::One
        );
        assert_eq!(
            Config::reads_its_targets().depression_exponent,
            DepressionExponent::One,
            "the derivation stays H-42's (ADR-0232)"
        );
        let mut weights = [0i16; 3];
        for (i, exponent) in [
            DepressionExponent::One,
            DepressionExponent::Half,
            DepressionExponent::Quarter,
        ]
        .into_iter()
        .enumerate()
        {
            let mut exec = engine(exponent);
            assert_eq!(exec.depression_exponent(), exponent);
            // Unit 2 fires, then unit 0: slot 0 of block 0 is depressed at its shaped magnitude.
            let mut block = exec.blocks()[0];
            let post = fire(&mut exec, 2);
            exec.run(10);
            let pre = fire(&mut exec, 0);
            let traces = block.step_stdp_all_shaped(
                pre,
                [post, 0, 0, 0],
                Polarity::Excitatory,
                ISTDP_ALPHA_Q1_15,
                exponent,
            );
            weights[i] = exec.blocks()[0].weights_q1_15[0];
            assert_eq!(
                weights[i],
                8_000i16.saturating_add(traces[0]),
                "{exponent:?}"
            );
        }
        // At 8 000, below the reference, a quarter depresses more than the rule, a half between.
        assert!(
            weights[2] < weights[0] && weights[2] <= weights[1] && weights[1] <= weights[0],
            "{weights:?}"
        );
        let plain = Image::encode(&engine(DepressionExponent::One)).expect("quiescent");
        let mut set = engine(DepressionExponent::Quarter);
        assert!(set.set_zero_sum(&[(&[0], 16_000)], &[2, 3]));
        assert_eq!(
            Image::encode(&set).expect("quiescent"),
            plain,
            "neither rule writes a byte"
        );
        let decoded = Image::decode::<64>(
            &plain,
            Config {
                workers: 1,
                nodes_per_worker: 256,
                depression_exponent: DepressionExponent::Half,
                ..Config::default()
            },
        )
        .expect("a well-formed image");
        assert_eq!(decoded.depression_exponent(), DepressionExponent::Half);
        assert_eq!(decoded.zero_sum_groups(), 0, "a loaded engine starts unset");
    }
}

/// The running configuration (ADR-0232): the name, field by field against the engine's default;
/// and its two refusals, `Executor::new`'s and `Image::decode`'s, over the lattice of
/// `testkit/prop.rs` against a rule written from that ADR's text, at every exponent.
#[cfg(test)]
mod running_prop {
    use super::*;
    use crate::image::{Image, ImageError};
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../testkit/prop.rs"
    ));

    const EXPONENTS: [DepressionExponent; 3] = [
        DepressionExponent::One,
        DepressionExponent::Half,
        DepressionExponent::Quarter,
    ];

    /// A period the rate's target takes at `UNITS`.
    const PERIOD: u32 = 58_201;
    const UNITS: usize = 64;

    /// A quarter's exponent where the default's is one, and with that put back, the default: the
    /// read, its flag and the probe are the default's, for an image to outrank, and the kept
    /// addresses, the delay read and the sleep stay unset.
    #[test]
    fn runs_a_settled_image_is_the_default_with_a_quarter_s_exponent_and_no_other() {
        let (running, default) = (Config::runs_a_settled_image(), Config::reads_its_targets());
        assert_eq!(
            (running.depression_exponent, default.depression_exponent),
            (DepressionExponent::Quarter, DepressionExponent::One)
        );
        assert_eq!(
            Config {
                depression_exponent: default.depression_exponent,
                ..running.clone()
            },
            default,
            "every other field the default's"
        );
        assert_eq!(
            (
                running.kept_addresses,
                running.reads_its_delay,
                running.sleep_shift
            ),
            (0, false, 0)
        );
    }

    /// `Executor::new`'s refusal: over the read's lattice and its edges, beside no period and
    /// beside one, each flag set and unset, at every exponent. An exponent but one is refused
    /// beside a read and nowhere else, after the read's own refusals; so no engine is born under
    /// the name, and with its read taken off one is.
    #[test]
    fn an_exponent_but_one_is_refused_beside_a_read_and_nowhere_else() {
        let rule = |windows: u8, period: u32, flags: (bool, bool), exponent: DepressionExponent| {
            let read = match (windows, period) {
                (0, _) => None,
                (1..16, _) => return Err(ConfigError::RateReadOutOfRange),
                (left, 0) => Some(TargetRead::Reading { left, sum: 0 }),
                _ => return Err(ConfigError::RateReadBesideTarget),
            };
            match (read, flags, exponent) {
                (None, (true, _), _) => Err(ConfigError::IstdpReadWithoutRead),
                (None, (_, true), _) => Err(ConfigError::StepProbeWithoutRead),
                (None, _, _) | (_, _, DepressionExponent::One) => Ok((read, exponent)),
                _ => Err(ConfigError::ExponentBesideRead),
            }
        };
        let (mut refused, mut legal) = (0u32, 0u32);
        for exponent in EXPONENTS {
            for windows in U8_LATTICE.into_iter().chain([15, 16]) {
                for period in [0, PERIOD] {
                    for flags in [(false, false), (true, false), (false, true), (true, true)] {
                        let made = Executor::<8>::new(Config {
                            units: UNITS,
                            rate_read_windows: windows,
                            rate_target_period_ticks: period,
                            istdp_target_read: flags.0,
                            step_probe: flags.1,
                            depression_exponent: exponent,
                            ..Config::default()
                        })
                        .map(|exec| (exec.target_read(), exec.depression_exponent()));
                        let expected = rule(windows, period, flags, exponent);
                        assert_eq!(made, expected, "{windows} {period} {flags:?} {exponent:?}");
                        refused = refused.saturating_add(u32::from(
                            expected == Err(ConfigError::ExponentBesideRead),
                        ));
                        legal = legal.saturating_add(u32::from(
                            exponent != DepressionExponent::One && expected.is_ok(),
                        ));
                    }
                }
            }
        }
        assert!(refused > 0 && legal > 0, "{refused} refused, {legal} legal");
        assert!(matches!(
            Executor::<8>::new(Config {
                units: UNITS,
                ..Config::runs_a_settled_image()
            }),
            Err(ConfigError::ExponentBesideRead)
        ));
        let no_read = Executor::<8>::new(Config {
            units: UNITS,
            rate_read_windows: 0,
            istdp_target_read: false,
            step_probe: false,
            ..Config::runs_a_settled_image()
        })
        .expect("a lead-in with no read stays legal at any exponent");
        assert_eq!(no_read.depression_exponent(), DepressionExponent::Quarter);
    }

    /// An engine with its period set or not, its image written once what is under way is laid on.
    fn imaged(period: u32, under_way: impl FnOnce(&mut Executor<8>)) -> Vec<u8> {
        let mut exec = Executor::<8>::new(Config {
            units: UNITS,
            rate_target_period_ticks: period,
            ..Config::default()
        })
        .expect("a valid configuration");
        under_way(&mut exec);
        Image::encode(&exec).expect("quiescent")
    }

    /// `image` at every exponent: refused by the ADR's own error at any but one, and at one the
    /// engine with `read` under way.
    fn refused_but_at_one(image: &[u8], read: TargetRead, at: &str) {
        for exponent in EXPONENTS {
            let decoded = Image::decode::<8>(
                image,
                Config {
                    depression_exponent: exponent,
                    ..Config::default()
                },
            );
            match (exponent, decoded) {
                (DepressionExponent::One, Ok(exec)) => {
                    assert_eq!(exec.target_read(), Some(read), "{at}");
                }
                (DepressionExponent::One, Err(e)) => panic!("{at}: {e:?}"),
                (_, Err(ImageError::Config(ConfigError::ExponentBesideRead))) => {}
                (_, Err(e)) => panic!("{at} at {exponent:?}: {e:?}"),
                (_, Ok(_)) => panic!("{at} at {exponent:?}: not refused"),
            }
        }
    }

    /// `Image::decode`'s refusal: a read under way over the lattice's spans and sums, each flag
    /// set and unset, and a probe under way at every window left over the lattice's sums, under
    /// every exponent. A settled image, its read over or never begun, loads at each, under the
    /// name too, which takes every other value from the image.
    #[test]
    fn an_image_with_a_read_or_a_probe_under_way_is_refused_at_any_exponent_but_one() {
        for left in U8_LATTICE.into_iter().filter(|&left| left != 0) {
            for sum in U32_LATTICE.map(|sum| u64::from(sum).min(read_sum_max(left))) {
                for flags in [(false, false), (true, false), (false, true), (true, true)] {
                    let image = imaged(0, |exec| {
                        exec.set_target_read(left, sum)
                            .expect("no period beside it");
                        if flags.0 {
                            exec.set_istdp_target_read().expect("a read under way");
                        }
                        if flags.1 {
                            exec.set_step_probe(None).expect("a read under way");
                        }
                    });
                    let read = TargetRead::Reading { left, sum };
                    refused_but_at_one(&image, read, &format!("{read:?} {flags:?}"));
                }
            }
        }
        for left in 1..=PROBE_WINDOWS {
            let (least, most) = probe_sum_bounds(left);
            for sum in I32_LATTICE.map(|sum| i64::from(sum).clamp(least, most)) {
                let image = imaged(PERIOD, |exec| {
                    exec.set_step_probe(Some((left, sum))).expect("a target");
                });
                let probe = TargetRead::Probing { left, sum };
                refused_but_at_one(&image, probe, &format!("{probe:?}"));
            }
        }
        for period in [0, PERIOD] {
            let image = imaged(period, |_| {});
            for exponent in EXPONENTS {
                let exec = Image::decode::<8>(
                    &image,
                    Config {
                        depression_exponent: exponent,
                        ..Config::default()
                    },
                )
                .expect("no read and no probe under way");
                assert_eq!(exec.depression_exponent(), exponent, "{period}");
            }
            let exec = Image::decode::<8>(&image, Config::runs_a_settled_image())
                .expect("the name's read is the image's to outrank");
            assert_eq!(
                (
                    exec.depression_exponent(),
                    exec.target_read(),
                    exec.step_probe(),
                    exec.istdp_target_read(),
                    exec.rate_target_period_ticks(),
                    *exec.homeostasis(),
                    exec.modulation_baseline_q16()
                ),
                (
                    DepressionExponent::Quarter,
                    None,
                    false,
                    false,
                    period,
                    HomeostaticDrivePool::new(),
                    MODULATION_ONE_Q16
                ),
                "{period}: every value but the exponent the image's"
            );
        }
    }
}
