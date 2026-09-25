//! The executor: a fixed pool of workers that run the turns, the fan-out and the deliveries of
//! whitepaper R-1 for every unit, tick by tick (ADR-0023).
//!
//! A tick has three phases separated by barriers, so that the references a worker holds into
//! the shared arenas never alias another worker's:
//!
//! 1. **Turns.** Each worker pops units from its deque, stealing from the others when it is
//!    empty; for each, it claims the turn (`begin_turn`), drains the mailbox, orders the batch
//!    by message value (§8.3), scales the two compartment sums by the tick's synaptic gain
//!    (ADR-0036), integrates, steps the short-term plasticity if the unit fired, ends the
//!    turn, and keeps the unit on the active set for the next tick while it is not at rest.
//!    Only the turn holder references the unit, exclusively. At the end of the phase the
//!    worker publishes how many of its units fired.
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
//!    shared. Units woken by a push are queued for the next tick, as is the active set.
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
use crate::deque::{self, Local, Steal, Stealer};
use crate::discovery::Discovery;
use crate::episode::{COINCIDENCE_TICKS, DISCOVERY_WINDOW, DiscoverError, DiscoverReport};
use crate::image::{ImageError, WriteAheadLog};
use crate::injector::{self, Injector};
use crate::pool::Pools;
use crate::store::{Induction, TermError};
use cortex_affect::InteroceptiveState;
use cortex_core::{
    BASAL_LEAK_SHIFT, CHAIN_END, Cadence, DendriticSuperNeuron, FlatTimingWheel,
    ISTDP_PERIOD_MAX_TICKS, ISTDP_PERIOD_MIN_TICKS, ISTDP_TARGET_PERIOD_TICKS, MODULATION_ONE_Q16,
    NO_SPIKE_ON_RECORD, PlasticDelta, Polarity, REFRACTORY_TICKS, SYNAPSES_PER_BLOCK, SynapseBlock,
    THRESHOLD_BASE, WorkerWheel, istdp_alpha_q1_15, message_efficacy_q16, message_is_apical,
    message_is_synaptic, spike_message, synapse_token, synaptic_message, token_block, token_slot,
};
use cortex_ethics::EthicalEvaluationGate;
use cortex_executive::{
    AMENDMENT_PROPOSED, PARAM_SWEEP_BUDGET, PARAM_SWEEP_QUIET_TICKS, PolicyAmendment, spec_of,
};
use cortex_hippocampus::{Episode, HippocampalAttractorState, PATTERN_MAX, RIPPLE_SHIFT};
use cortex_homeostasis::{
    ACTIVITY_BIN_SHIFT, ACTIVITY_WINDOW_SHIFT, CONTROL_STEP_MAX_Q0_16, GAIN_ONE_Q16,
    HomeostaticDrivePool, SLEEP_SHIFT_MAX, STAGE_AWAKE, STAGE_REM, STAGE_SWS,
};
use cortex_neuromod::{DOPAMINE_TAU_SHIFT, NeuromodulatorState};
use cortex_reasoning::{Compaction, InductionState, SEARCH_SHIFT_MAX, TermNode};
use std::collections::VecDeque;
use std::path::Path;
use std::sync::Arc;
use std::sync::atomic::{
    AtomicBool, AtomicI32, AtomicI64, AtomicU32, AtomicU64, AtomicUsize, Ordering,
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
    /// Slots per worker deque; 0 means one per unit, which cannot fill.
    pub deque_capacity: usize,
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
    /// stays at 1.0 and the dynamics are the reference ones. For an engine built from an image,
    /// the image's step and gain outrank this one: they change what the run does (§8.3).
    pub control_step_q0_16: u16,
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
    /// consolidates under `clamp(baseline + dopamine, -1, 1)` (`cortex-neuromod`'s
    /// `signed_modulation`), so that below zero its weight moves against its trace's sign and
    /// the trace is spent by as much (`SynapseBlock::consolidate_signed`); every other synapse
    /// consolidates as before — an unaddressed one under the baseline alone, an inhibitory one
    /// under the addressed modulation at no less than zero, or under its own baseline while
    /// that is set. Unset (the default), every synapse consolidates as before, bit for bit.
    /// For an engine built from an image, the image's outranks this one: it changes what the
    /// run does, so it is part of the image (§8.3).
    pub signed_gate: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            workers: 1,
            units: 1,
            blocks: 0,
            deltas: 0,
            nodes_per_worker: 64,
            deque_capacity: 0,
            injector_capacity: 64,
            trace_capacity: 0,
            amendments: 0,
            modulation_baseline_q16: MODULATION_ONE_Q16,
            control_step_q0_16: 0,
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
        }
    }
}

/// The engine's policy (ADR-0031): the parameters its rules take that it may amend by itself,
/// each an entry of `cortex_executive::REGISTRY`. Read between ticks by the clock sweep;
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

/// Why an addressing is refused (ADR-0068).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AddressError {
    /// A unit of the addressed set is outside the arena.
    NoSuchUnit,
}

/// The modulations of a tick (ADR-0068, ADR-0086): the one an addressed synapse consolidates
/// under, `clamp(baseline + dopamine, 0, 1)` (`cortex-neuromod`'s `modulation`, ADR-0032);
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
/// (`cortex-neuromod`'s `signed_modulation`), which an addressed excitatory synapse
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
    /// signed gate: the rule is `cortex-neuromod`'s in the first two, once with the signal as
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

/// The injector payload that asks for a turn without a message.
pub const ACTIVATE: u32 = u32::MAX;

/// Aborts the process: inside the tick loop a violated invariant is a bug (whitepaper §8.9).
#[cold]
fn abort(message: &str) -> ! {
    eprintln!("cortex-runtime: invariant violated: {message}");
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
    stealers: Box<[Stealer]>,
    barrier: SpinBarrier,
    injector: Injector,
    delivered: Box<[AtomicU64]>,
    /// Per worker, the turns it has served so far (ADR-0097): stored beside `delivered` at the
    /// end of the deliveries phase, summed by `Executor::turns` between ticks.
    turns: Box<[AtomicU64]>,
    now: AtomicU32,
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
}

/// One worker's private state.
struct Worker<const CAP: usize> {
    id: usize,
    local: Local,
    wheel: Box<FlatTimingWheel<CAP>>,
    steal_from: usize,
    batch: Vec<u32>,
    due: Vec<u32>,
    spiked: Vec<(u32, u8, u8)>,
    next_tick: Vec<u32>,
    trace: Vec<u32>,
    spike_trace: Vec<(u32, u32)>,
    trace_dropped: u64,
    delivered: u64,
    /// Turns this worker has served (ADR-0097).
    turns: u64,
    /// Descendants this worker fired this tick (ADR-0054).
    descended: u32,
    in_flight: i64,
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
}

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
        let workers = config.workers;
        // A unit fires at most once per tick, so one slot per unit holds a tick's spikes.
        let fired_slots = if config.train_capacity == 0 {
            0
        } else {
            config.units
        };
        let deque_capacity = if config.deque_capacity == 0 {
            config.units
        } else {
            config.deque_capacity
        };
        let (locals, stealers): (Vec<Local>, Vec<Stealer>) =
            (0..workers).map(|_| deque::new(deque_capacity)).unzip();
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
            stealers: stealers.into_boxed_slice(),
            barrier: SpinBarrier::new(workers),
            injector: Injector::new(config.injector_capacity),
            delivered: (0..workers).map(|_| AtomicU64::new(0)).collect(),
            turns: (0..workers).map(|_| AtomicU64::new(0)).collect(),
            now: AtomicU32::new(0),
            modulation: AtomicI32::new(config.modulation_baseline_q16),
            modulation_at_rest: AtomicI32::new(config.modulation_baseline_q16),
            modulation_inhibitory: AtomicI32::new(INHIBITORY_UNSET),
            sources: (0..config.units).map(|_| AtomicBool::new(true)).collect(),
            targets: (0..config.units).map(|_| AtomicBool::new(true)).collect(),
            gain: AtomicU32::new(GAIN_ONE_Q16),
            spikes: (0..workers).map(|_| AtomicU32::new(0)).collect(),
            descendants: (0..workers).map(|_| AtomicU32::new(0)).collect(),
            istdp_alpha: AtomicI32::new(i32::from(istdp_alpha_q1_15(
                config.istdp_target_period_ticks,
            ))),
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
        let mut states: Vec<Worker<CAP>> = locals
            .into_iter()
            .enumerate()
            .map(|(id, local)| Worker {
                id,
                local,
                wheel: wheels.remove(0),
                steal_from: next_in_ring(id, workers),
                batch: Vec::with_capacity(total_nodes),
                due: Vec::with_capacity(CAP),
                spiked: Vec::with_capacity(config.units),
                next_tick: Vec::with_capacity(config.units),
                trace: Vec::with_capacity(config.trace_capacity),
                spike_trace: Vec::with_capacity(config.trace_capacity),
                trace_dropped: 0,
                delivered: 0,
                turns: 0,
                in_flight: 0,
                descended: 0,
            })
            .collect();
        let worker0 = states.remove(0);
        let threads = states
            .into_iter()
            .map(|mut worker| {
                let shared = Arc::clone(&shared);
                thread::Builder::new()
                    .name(format!("cortex-worker-{}", worker.id))
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
            homeostasis: HomeostaticDrivePool {
                control_step_q0_16: config.control_step_q0_16,
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
        })
    }

    /// The target period of the inhibitory rule (ADR-0053), in ticks: the configuration's,
    /// or the image's for an engine built from one.
    pub fn istdp_target_period_ticks(&self) -> u32 {
        self.istdp_target_period_ticks
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

    /// A reward-prediction error into the dopamine signal, between ticks (ADR-0032): an input,
    /// like an injection, so a run that replays its rewards at the same ticks is the same run.
    /// The next tick's fan-out consolidates under the raised modulation; the signal then decays
    /// by `DOPAMINE_TAU_SHIFT` per tick. Returns the signal.
    pub fn reward(&mut self, reward_prediction_error_q16: i32) -> i32 {
        self.modulator.reward(reward_prediction_error_q16)
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
    /// ticks, before anything reads a stamp against it.
    pub(crate) fn resume_clock(&mut self, tick: u64) {
        self.tick = tick;
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
    /// `cortex-homeostasis`, read between ticks.
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
        let signal_q16 = self.reward(report.reward_total_q16);
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
    /// are masks on the tick, so a loaded engine continues the window it was written in.
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
                self.homeostasis
                    .regulate(saturation_ceiling(self.shared.units.len()));
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

    /// Worker threads, including the caller's.
    pub fn workers(&self) -> usize {
        self.shared.barrier.parties()
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

    /// Turns served by every worker so far (ADR-0097): one for each unit a worker took from a
    /// deque and ran, so the difference across a tick is the tick's active set — the units
    /// that were not at rest after the tick before or that a message or an activation woke
    /// (ADR-0023). A count kept beside `delivered`; it changes nothing the engine does.
    pub fn turns(&self) -> u64 {
        self.shared
            .turns
            .iter()
            .map(|n| n.load(Ordering::Relaxed))
            .sum()
    }

    /// The unit arena, between ticks.
    pub fn units(&self) -> &[DendriticSuperNeuron] {
        // SAFETY: `&self` excludes `tick` (which takes `&mut self`); between ticks every worker
        // thread is spinning at the barrier and holds no reference into an arena.
        unsafe { self.shared.units.as_slice() }
    }

    /// The unit arena, exclusively, between ticks: for wiring a network before it runs.
    pub fn units_mut(&mut self) -> &mut [DendriticSuperNeuron] {
        // SAFETY: as in `units`, with `&mut self` excluding every other reference this
        // executor hands out.
        unsafe { self.shared.units.as_mut_slice() }
    }

    /// The unit and synapse arenas together, exclusively, between ticks: for a rule that
    /// writes both, such as the synthesis of a network from a prior (ADR-0044).
    pub fn arenas_mut(&mut self) -> (&mut [DendriticSuperNeuron], &mut [SynapseBlock]) {
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

    /// Between ticks: queues `units` for a turn on the next tick without a message (the loader
    /// wakes every unit that is not at rest).
    pub(crate) fn wake_now(&mut self, units: &[u32]) {
        for &unit in units {
            self.worker0.wake(&self.shared, unit);
        }
    }

    /// One fine tick: the three phases on every worker, this thread running as worker 0.
    pub fn tick(&mut self) {
        let now = self.tick as u32;
        self.shared.now.store(now, Ordering::Relaxed);
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
        // The gain this tick's turns scale by (ADR-0036), likewise before the barrier.
        self.shared
            .gain
            .store(self.homeostasis.synaptic_gain_q16, Ordering::Relaxed);
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
        self.merge_spikes(now);
        // A clock wraps by name (§8.1): the dynamics already see it as `tick as u32`.
        self.tick = self.tick.wrapping_add(1);
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

/// A unit at rest has nothing to integrate: every potential zero, no window running, the
/// threshold at or below its base. It leaves the active set until a message wakes it.
pub(crate) fn at_rest(u: &DendriticSuperNeuron) -> bool {
    u.v_soma == 0
        && u.v_basal == 0
        && u.v_apical == 0
        && u.refractory_ticks == 0
        && u.bac_plateau_ticks == 0
        && u.v_thresh <= THRESHOLD_BASE
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
        }
    }

    // ---------------------------------------------------------------- phase 1: turns

    fn phase_turns(&mut self, shared: &Shared, now: u32) {
        let gain = shared.gain.load(Ordering::Relaxed);
        self.descended = 0;
        loop {
            let unit = match self.local.pop() {
                Some(unit) => unit,
                None => match self.steal(shared) {
                    Some(unit) => unit,
                    None => break,
                },
            };
            self.turn(shared, unit, now, gain);
            self.turns = self.turns.saturating_add(1);
        }
        // The units this worker fired this tick, for the population tally (ADR-0036), and
        // how many of them were descendants (ADR-0054); counts below the unit count, which
        // `Executor::new` bounded below `u32::MAX`.
        shared.spikes[self.id].store(self.spiked.len() as u32, Ordering::Relaxed);
        shared.descendants[self.id].store(self.descended, Ordering::Relaxed);
    }

    fn steal(&mut self, shared: &Shared) -> Option<u32> {
        let n = shared.stealers.len();
        for _ in 0..n {
            let victim = self.steal_from;
            self.steal_from = next_in_ring(victim, n);
            if victim == self.id {
                continue;
            }
            loop {
                match shared.stealers[victim].steal() {
                    Steal::Success(unit) => return Some(unit),
                    Steal::Empty => break,
                    Steal::Retry => continue,
                }
            }
        }
        None
    }

    fn turn(&mut self, shared: &Shared, unit: u32, now: u32, gain: u32) {
        // SAFETY (phase 1): this worker took `unit` from a deque, where it was put by the one
        // `try_schedule` that moved its gate to scheduled, so no other worker holds it; no
        // phase-1 code references another unit, and phases 2 and 3 have ended at the barrier.
        let Some(u) = (unsafe { shared.units.get_mut(unit as usize) }) else {
            abort("a queued unit index is outside the arena");
        };
        if !u.begin_turn() {
            abort("a queued unit was not scheduled");
        }
        self.batch.clear();
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
        if u.integrate(basal, apical, now) {
            let elapsed = if previous_spike == NO_SPIKE_ON_RECORD {
                u32::MAX
            } else {
                now.wrapping_sub(previous_spike)
            };
            let (release_u, release_r) = u.step_stp(elapsed);
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
        if u.end_turn() {
            // A message arrived during the turn. No push overlaps a turn in this executor, so
            // this cannot happen; the rule of ADR-0017 is kept as defence in depth.
            if self.local.push(unit).is_err() {
                abort("the deque is full");
            }
        } else if !at_rest(u) && u.try_schedule() {
            push_bounded(&mut self.next_tick, unit, "more active units than units");
        }
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
                block.step_stdp_all(now, posts, polarity, istdp_alpha);
                consolidate_each(block, &per_slot, polarity);
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
                    self.wake(shared, unit);
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
        for i in 0..self.next_tick.len() {
            if self.local.push(self.next_tick[i]).is_err() {
                abort("the deque is full");
            }
        }
        self.next_tick.clear();
        shared.delivered[self.id].store(self.delivered, Ordering::Relaxed);
        shared.turns[self.id].store(self.turns, Ordering::Relaxed);
        shared.in_flight[self.id].store(self.in_flight, Ordering::Relaxed);
    }

    /// Phases 2 and 3: a message into a unit's mailbox, and the unit onto this worker's deque
    /// if its gate was idle.
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
        if u.try_schedule() && self.local.push(target).is_err() {
            abort("the deque is full");
        }
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

    /// Phase 3: a turn without a message.
    fn wake(&mut self, shared: &Shared, unit: u32) {
        // SAFETY (phase 3): as in `deliver`.
        let Some(u) = (unsafe { shared.units.get(unit as usize) }) else {
            abort("an activation names a unit outside the arena");
        };
        if u.try_schedule() && self.local.push(unit).is_err() {
            abort("the deque is full");
        }
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
                blocks: cortex_core::MAX_TOKEN_BLOCK as usize + 2,
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

    /// Two armed units fired by fourteen strong messages each, `rounds` times, 400 ticks
    /// apart; returns the executor after the last round and the spikes its workers traced.
    fn fired(train_capacity: usize, rounds: u32) -> (Executor<8>, Vec<(u32, u32)>) {
        use cortex_core::{STP_MAX, STP_U, synaptic_efficacy_q16};
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
                "the rule is cortex-neuromod's"
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
    /// -1, 1)` — `cortex-neuromod`'s `signed_modulation` — and every other slot under what it
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
                "the rule is cortex-neuromod's"
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
