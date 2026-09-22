//! Brief 029's instrument, its calibration (ADR-0065) and the measurement through it
//! (ADR-0066): the same rule (ADR-0032), the same selection (`cortex-basal-ganglia` through
//! ADR-0059's task), the same four controls and the same seeds as brief 027's measurement
//! (ADR-0060), with the four changes ADR-0060 named and did not build: a stimulus whose
//! units are spaced beyond the prior's local window so that each fires once, a readout that
//! counts the ticks in which the stimulus's local synapses land, a trial of one time constant
//! of the dopamine signal so that one outcome gates one trial's traces, and a gain the
//! calibration picks from two candidates. Before any reward is given, the calibration shows
//! the readout sees the stimulus at all: the weights frozen (the modulation baseline at zero,
//! no reward), the two readouts' spikes in the window after the volley against the window of
//! the same length before the injection, over sixty-four trials, a pass at fifty-six of them.
//! Every constant is written here before the first rewarded run, derived by a rule from the
//! prior's parameters or picked by the calibration from the candidates listed here; nothing
//! is chosen after a rewarded run.
//!
//! Every number is the engine's own, pinned from one run and held on every worker count and
//! every architecture. Every full run and every calibration is the weekly job's `exhaustive`
//! test; the pull request's gate runs the geometry against the census, the rewarded run's
//! first block at 256 units and the criterion over the pinned tables (ADR-0061). What the
//! numbers decide, and at what scale, is stated in ADR-0066 and in whitepaper §11.1.
//!
//! Brief 031 (ADR-0070) asks where 256 units settle: the executor of the rewarded runs at
//! 256 units run under the drive alone for eighty windows, the sums by polarity read per
//! window, brief 026's clause as an integer rule over the table, and a lead-in in windows
//! derived from it and not chosen; then the rewarded run again behind that lead-in, under
//! a criterion written before the run (the calibration's measure at least fifty-six of
//! sixty-four in every block). The settling and the run behind the lead-in are weekly
//! `exhaustive` tests; the gate runs the first four windows of the settling and the rules
//! over the pinned tables.
//!
//! Brief 032 (ADR-0072) asks what the trace is made of: the eligibility on a stimulus's
//! synapses into a readout, read from the record with the weights frozen, split by what
//! paired it through an oracle that replays the pair rule over the executor's own train and
//! must agree with the record at every reading; then a ladder of gains below the present
//! one, each rung read by the calibration's measure and by the trace's sign, both written
//! before the run, the first rung that passes both the gain a rewarded run would use. The
//! composition and the ladder are weekly `exhaustive` tests; the gate runs the first eight
//! trials of the composition and the rules over the pinned tables.
//!
//! Brief 034 (ADR-0076) asks for two injections: F-46's stimulus kept whole and a cancel, a
//! negative basal message into the same units inside the trial, so that a unit fires its
//! volley spike and not again at the end of its refractory window. The cancel's offset, its
//! span and its size are derived by an integer oracle over the membrane rule and the volley's
//! census, checked against the engine's own probe through the task, and read over frozen runs
//! by ADR-0074's measure, the sight and the sign. The candidates are a weekly `exhaustive`
//! test; the gate runs the probes, the rules over the pinned tables and the first eight trials
//! of the candidate the rules pick.
//!
//! Brief 035 (ADR-0077), step 2 of H-12's stopping rule, asks for the background side: the
//! network settled before the task with the gain held, then the criticality controller on at
//! ADR-0055's step, each alone and in that order, each a lead-in of whole windows under the
//! drive alone until ADR-0055's criterion holds or a bound, run quiet, saved as an image with
//! the modulation baseline at zero, and read over a frozen run of the task by three measures
//! written first — ADR-0074's that the stimulus still fires once, the sight and the sign — the
//! first candidate passing all three being the configuration and none meaning no rewarded run
//! and the rule's third step. The two candidates are weekly `exhaustive` tests; the gate runs
//! the rules over the pinned tables and the first two windows of the settled lead-in.
//!
//! Brief 036 (ADR-0079) runs H-13 as ADR-0078 wrote it: ADR-0077's settled image, held to its
//! tables before any rewarded run; the reward withheld over 512 trials, then the assignment and
//! the mirrored assignment from the same image, the reward after every trial delivered to the
//! synapses from the presented stimulus onto its assigned readout whatever the selection, the
//! oracle replaying the consolidation and held to the record's weights and traces at every
//! trial; the criterion — each assigned pair's coupling above the image's, and the assigned
//! readout's paired count against the withheld arm at least 80 of the last 128, in both
//! rewarded arms — as integer rules written before the run. The three arms are one weekly
//! `exhaustive` test; the gate runs the rules at their edges and the delivery's reach over the
//! first eight trials.
//!
//! Brief 037 (ADR-0081) runs H-14 as ADR-0080 wrote it: ADR-0077's settled image, held to its
//! tables, and a frozen block from it held to ADR-0077's frozen run before any rewarded run;
//! then the task as ADR-0059 and ADR-0068 built it — `Delivery::Addressed`, the reward reaching
//! the synapses from the presented stimulus onto the readout the engine selected, its sign the
//! outcome's — in the assignment, the mirrored assignment and, as a reading of lock-in, the
//! shuffled reward, each 1 536 trials from the one image, the oracle replaying the
//! consolidation under the pair the task addressed and the signal its reward left; the
//! criterion — the correct selections over the last 128 trials at least 80 in both rewarded
//! arms — as an integer rule written before the run, and ADR-0080's derivation (a wrong
//! selection's pair never consolidates) read after every run as an assertion beside the
//! verdict. The three arms are one weekly `exhaustive` test; the gate runs the rules at their
//! edges and the delivery over the first eight trials.

#![deny(clippy::arithmetic_side_effects)]

use cortex_connectome::{
    CortexFileHeader, Prior, SECTION_HOMEOSTASIS, SECTION_MODULATOR, SectionEntry, crc64,
    ring_distance,
};
use cortex_core::{
    DendriticSuperNeuron, ELIGIBILITY_TAU_SHIFT, FLAG_INHIBITORY, MODULATION_ONE_Q16,
    NO_SPIKE_ON_RECORD, REFRACTORY_TICKS, STDP_A_MINUS_Q1_15, STDP_A_PLUS_Q1_15, STDP_TAU_SHIFT,
    THRESHOLD_BASE, message_efficacy_q16, spike_message, stp_decay_factor_q16,
};
use cortex_homeostasis::{ACTIVITY_BIN_SHIFT, ACTIVITY_WINDOW_SHIFT, HomeostaticDrivePool};
use cortex_neuromod::DOPAMINE_TAU_SHIFT;
use cortex_runtime::{
    Cancel, Config, Delivery, Drive, Executor, Feedback, Image, Outcome, Readout, Set, Stimulus,
    Task, TaskError, Window, blocks_for, run_driven, spikes_per_unit, synthesize,
};

include!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../testkit/prop.rs"
));

type Engine = Executor<2048>;

const ONE: i32 = MODULATION_ONE_Q16;

// ------------------------------------------- written before the run (ADR-0065, ADR-0066)

/// A trial: $2^{14}$ ticks, 164 ms simulated, one time constant of the dopamine signal, so
/// that the signal one outcome left has decayed to $e^{-1}$ of itself at the next trial's
/// end and one outcome gates one trial's traces; a quarter of the eligibility trace's window
/// of $2^{16}$ ticks, so every pairing of the trial is pending when its reward comes.
const TRIAL_TICKS: u32 = 1 << DOPAMINE_TAU_SHIFT;
const _: () = assert!(TRIAL_TICKS == 1 << 14);
/// A block: sixty-four trials.
const BLOCK: usize = 64;
/// A run: 512 trials, eight blocks, $2^{23}$ ticks at either size.
const TRIALS: usize = 8 * BLOCK;
/// The criterion counts the last two blocks, 128 trials.
const LAST_BLOCKS: usize = 2;
/// The modulation with the dopamine signal at rest in the rewarded runs and under the
/// shuffled reward, brief 027's: half of every pending trace consolidates at a presynaptic
/// spike with no reward; a reward carries the next trial's consolidation toward the ceiling
/// and a punishment toward the floor.
const BASELINE_Q16: i32 = 0x8000;
/// The reward's magnitude, 1.0, brief 027's: the width of the modulation, signed by the
/// outcome.
const REWARD_Q16: i32 = ONE;
/// The stimulus: two messages of 1.25 into every unit of the set, the replay drive's
/// (ADR-0038), which fires a unit at its base threshold once.
const STIMULUS_Q16: i32 = 0x0001_4000;
const STIMULUS_MESSAGES: u32 = 2;
/// The seed the trials' stimuli and the shuffled coin are drawn from, brief 027's.
const SEED: u64 = 27;

/// The prior's local window: a local synapse reaches a unit within this many places on the
/// ring (ADR-0044's prior, below).
const PRIOR_WINDOW: u32 = 8;
/// The geometry, a rule of the prior's window and of its inhibitory rule. The ring is read
/// in periods of twenty places from a rotation: stimulus A is the first place of every
/// period and stimulus B the twelfth, so that every stimulus unit is at least nine places
/// from every other (beyond the window: no unit fires twice through a local synapse, and
/// neither stimulus drives the other through one) and, the period being a multiple of five
/// and the rotation not three or four more than one, no stimulus unit is one the prior makes
/// inhibitory (every fifth unit, from the fifth); readout 0 is the odd places but the twelfth
/// and readout 1 the even places but the first, nine each, so that each stimulus unit has
/// four units of either readout on either side of it, at mirrored distances, and every unit
/// of a period is in exactly one set. The places from the last whole period to the ring's
/// end, and from the ring's start to the rotation, are in no set.
const PERIOD: u32 = 20;
const A_OFFSET: u32 = 0;
const B_OFFSET: u32 = 11;
const R0_MASK: u32 = 0xAA2AA;
const R1_MASK: u32 = 0x55554;
const _: () =
    assert!(B_OFFSET - A_OFFSET > PRIOR_WINDOW && PERIOD + A_OFFSET - B_OFFSET > PRIOR_WINDOW);
const _: () = assert!(PERIOD % 5 == 0 && (A_OFFSET + 1) % 5 != 0 && (B_OFFSET + 1) % 5 != 0);
const _: () = assert!(((1 << A_OFFSET) | (1 << B_OFFSET) | R0_MASK | R1_MASK) == (1 << PERIOD) - 1);
const _: () = assert!(((1 << A_OFFSET) & R0_MASK) == 0 && ((1 << B_OFFSET) & R0_MASK) == 0);
const _: () = assert!(((1 << A_OFFSET) & R1_MASK) == 0 && ((1 << B_OFFSET) & R1_MASK) == 0);
const _: () = assert!((R0_MASK & R1_MASK) == 0 && R0_MASK.count_ones() == R1_MASK.count_ones());
/// The rotation at each size: the first place from which the pattern's four excitatory
/// couplings, read from the prior's census before any run, are equal within ten per cent
/// and none zero (`the_geometry_holds_against_the_census_at_both_sizes` holds it, and that
/// every smaller rotation fails). At 256 units it is 17, eleven whole periods; at 1 024 it
/// is 0, fifty-one.
const ROTATION_256: u32 = 17;
const ROTATION_1024: u32 = 0;
/// The readout window, a rule of the prior's local delay band (1 to 3 ms, 100 to 300 ticks):
/// it opens `delay_min` ticks after the stimulus is injected, before which no local synapse
/// of the volley can have landed, and closes `2 × delay_max` after it, leaving the volley's
/// own latency and the readout unit's as much again as the band's longest delay; the far
/// band (1 400 ticks on) lies outside it. The same window for both readouts and every trial.
const WINDOW: Window = Window {
    from: 100,
    ticks: 500,
};
/// The lead-in before the first trial: one window under the drive with no stimulus, so that
/// the first trial has a window before its injection.
const LEAD_IN: u32 = WINDOW.ticks;
/// The calibration's candidate gains, in the order they are tried: 1.75, then 2.0
/// (ADR-0044's table: five to six times quieter at 1.75).
const GAINS: [u32; 2] = [0x0001_C000, 0x0002_0000];
/// The calibration's pass mark: trials in which the window after the volley held more
/// readout spikes than the window before the injection, of sixty-four.
const SEEN_MIN: u32 = 56;
/// The gain the calibration picked at each size, the first candidate that passed
/// (`CALIBRATION_256`, `CALIBRATION_1024` below): 2.0 at 256 units, where 1.75 saw the
/// stimulus in 50 trials of 64; 1.75 at 1 024, where it saw it in 62.
const GAIN_256: u32 = 0x0002_0000;
const GAIN_1024: u32 = 0x0001_C000;
/// The criterion's counts over the last 128 trials (ADR-0066): the rewarded run's correct
/// trials at least 80, in both assignments, at both sizes (by noise about once in 337 per
/// run); a control's at most 76 (exceeded by noise about once in 74 per control);
/// `the_criterion_reads_as_written` computes both from the binomial's tail in integers.
const REWARDED_MIN: u32 = 80;
const CONTROL_MAX: u32 = 76;

// ------------------------------------------------- written before the run (brief 031)

/// A window of the population tally: $2^{12 + 5}$ ticks, the cadence at which ADR-0055
/// read the day and at which the settling is read here.
const WINDOW_TICKS: u64 = 1 << (ACTIVITY_BIN_SHIFT + ACTIVITY_WINDOW_SHIFT);
const _: () = assert!(WINDOW_TICKS == 1 << 17);
/// The settling measurement's length: eighty windows, the length ADR-0055 gave 1 024 units;
/// a run of the task is sixty-four.
const SETTLING_WINDOWS: u64 = 80;
/// Brief 026's clause, unchanged (ADR-0055): a window satisfies it when each of the last
/// four windows up to and including it moved the excitatory sum by less than two per cent
/// of the sum before those four, the prior's sum standing before the first window. The
/// lead-in is the smallest window that satisfies it, or `SETTLING_WINDOWS` when none does
/// up to the bound; derived, never chosen.
const SETTLING_CLAUSE_WINDOWS: usize = 4;
const SETTLING_CLAUSE_PER_CENT: u64 = 2;
/// The lead-in in windows, derived from `SETTLING_256` by the rule above and not chosen:
/// the clause first holds at the ninth window (the last four moving the sum by 1.82, 1.79,
/// 1.58 and 1.44 per cent of the fifth's; the sum 0.818 of the prior's), so the run behind
/// the lead-in starts its first trial nine windows in. The gate holds the constant to the
/// rule over the pinned table; it was committed before the first run behind it.
const LEAD_IN_WINDOWS: u64 = 9;

// ------------------------------------------------------------------------- the network

/// The prior of ADR-0044 at `units`: a fifth inhibitory at the rail, 32 synapses per unit, a
/// window of eight, a quarter rewired, local delays of 1 to 3 ms and far ones of 14 to
/// 25.6 ms, excitatory weights in [6 000, 12 000], seed 22.
fn prior(units: u32) -> Prior {
    Prior {
        units,
        inhibitory_every: 5,
        synapses_per_unit: 32,
        window: PRIOR_WINDOW,
        rewire_q0_8: 64,
        delay_min: 100,
        delay_max: 300,
        far_delay_min: 1400,
        far_delay_max: 2559,
        weight_min: 6000,
        weight_max: 12000,
        inhibitory_gain_q4_4: 255,
        apical_q0_8: 0,
        seed: 22,
    }
}

/// The drive of ADR-0044: every tick, one message of 0.125 per 128 units into units drawn
/// by the tick.
fn drive(units: u32) -> Drive {
    Drive {
        every: 1,
        messages: units / 128,
        efficacy_q16: 0x2000,
        units,
        seed: 3,
    }
}

/// The executor for a run: the gain held (`control_step_q0_16` 0), no sleep, a train that
/// holds the most spikes a trial can produce, no arena and no store.
fn config(units: u32, workers: usize, baseline_q16: i32) -> Config {
    Config {
        workers,
        units: units as usize,
        blocks: blocks_for(&prior(units)) as usize,
        nodes_per_worker: 1 << 16,
        injector_capacity: 1 << 12,
        train_capacity: (units as usize).saturating_mul(spikes_per_unit(TRIAL_TICKS) as usize),
        modulation_baseline_q16: baseline_q16,
        control_step_q0_16: 0,
        sleep_shift: 0,
        ..Config::default()
    }
}

/// The image of `exec` with its homeostasis record patched, decoded under `config`.
fn reload_with(exec: &Engine, config: Config, patch: impl Fn(&mut HomeostaticDrivePool)) -> Engine {
    let mut img = Image::encode(exec).expect("quiescent");
    let header = CortexFileHeader::decode(img[0..64].try_into().unwrap());
    for at in (64..).step_by(64).take(header.section_count as usize) {
        let mut entry = SectionEntry::decode(img[at..][..64].try_into().unwrap());
        if entry.kind == SECTION_HOMEOSTASIS {
            let (offset, length) = (entry.offset as usize, entry.length as usize);
            let mut pool = HomeostaticDrivePool::decode((&img[offset..][..64]).try_into().unwrap());
            patch(&mut pool);
            img[offset..][..length].copy_from_slice(&pool.encode());
            entry.crc64 = crc64(&img[offset..][..length]);
            img[at..][..64].copy_from_slice(&entry.encode());
            break;
        }
    }
    Image::decode::<2048>(&img, config).expect("a well-formed record")
}

/// The network at `gain`, synthesized from `p` and reloaded with the gain in its record.
fn at_gain(p: &Prior, config: Config, gain: u32) -> Engine {
    let mut exec = Engine::new(config.clone()).unwrap();
    let (u, b) = exec.arenas_mut();
    synthesize(u, b, p).unwrap();
    reload_with(&exec, config, |h| h.synaptic_gain_q16 = gain)
}

// ---------------------------------------------------------------------------- the task

/// The four sets of the geometry at `units` from `rotation`: stimulus A, stimulus B,
/// readout 0, readout 1, over as many whole periods as fit from the rotation.
fn geometry(units: u32, rotation: u32) -> [Set; 4] {
    let count = units.saturating_sub(rotation) / PERIOD;
    let set = |mask: u32| Set {
        first: rotation,
        period: PERIOD,
        mask,
        count,
    };
    [
        set(1 << A_OFFSET),
        set(1 << B_OFFSET),
        set(R0_MASK),
        set(R1_MASK),
    ]
}

/// The rotation at `units`, as written above.
fn rotation(units: u32) -> u32 {
    match units {
        256 => ROTATION_256,
        1024 => ROTATION_1024,
        _ => panic!("no rotation is written for {units} units"),
    }
}

/// The gain at `units`, as the calibration picked it.
fn gain(units: u32) -> u32 {
    match units {
        256 => GAIN_256,
        1024 => GAIN_1024,
        _ => panic!("no gain is written for {units} units"),
    }
}

/// A stimulus's shape: the messages into every unit of the set and each message's efficacy
/// (brief 033; `SHAPE_F46` is the shape every run before it used).
type Shape = (u32, i32);
/// The shape of ADR-0065's stimulus, F-46's: two messages of 1.25.
const SHAPE_F46: Shape = (STIMULUS_MESSAGES, STIMULUS_Q16);

/// The task at `units` with the stimulus of `shape` and, when one is given, its cancel
/// (brief 034; every run before it passes `None`, so its task is the task it was).
fn task(
    shape: Shape,
    cancel: Option<Cancel>,
    units: u32,
    feedback: Feedback,
    mirrored: bool,
    delivery: Delivery,
) -> Task {
    let [a, b, r0, r1] = geometry(units, rotation(units));
    let stimulus = |set| Stimulus {
        set,
        messages: shape.0,
        efficacy_q16: shape.1,
        cancel,
    };
    Task {
        stimuli: [stimulus(a), stimulus(b)],
        readout: Readout::new([r0, r1]),
        drive: drive(units),
        ticks: TRIAL_TICKS,
        window: WINDOW,
        seed: SEED,
        reward_q16: REWARD_Q16,
        mirrored,
        feedback,
        delivery,
    }
}

// -------------------------------------------------------------------------- the readings

/// The weights over the arena by polarity: the sum of the inhibitory magnitudes and the sum
/// of the excitatory weights.
fn weights_by_polarity(exec: &Engine) -> (i64, i64) {
    let mut inhibitory = 0i64;
    let mut excitatory = 0i64;
    for unit in exec.units() {
        let inhibitory_unit = unit.flags & FLAG_INHIBITORY != 0;
        for s in unit.fan_out(exec.blocks()) {
            let w = s.weight_q1_15 as i64;
            if inhibitory_unit {
                inhibitory = inhibitory.wrapping_add(w.wrapping_neg());
            } else {
                excitatory = excitatory.wrapping_add(w);
            }
        }
    }
    (inhibitory, excitatory)
}

/// The excitatory coupling from `from` into `into`: the sum of the weights of the synapses an
/// excitatory unit of `from` sends to a unit of `into`.
fn coupling(exec: &Engine, from: Set, into: Set) -> i64 {
    let mut sum = 0i64;
    for unit in exec.units() {
        if unit.flags & FLAG_INHIBITORY != 0 || !from.contains(unit.id as u32) {
            continue;
        }
        for s in unit.fan_out(exec.blocks()) {
            if into.contains(s.target) {
                sum = sum.wrapping_add(s.weight_q1_15 as i64);
            }
        }
    }
    sum
}

/// The four couplings of a geometry on `exec`: A→R0, A→R1, B→R0, B→R1.
fn couplings(exec: &Engine, sets: &[Set; 4]) -> [i64; 4] {
    let [a, b, r0, r1] = *sets;
    [
        coupling(exec, a, r0),
        coupling(exec, a, r1),
        coupling(exec, b, r0),
        coupling(exec, b, r1),
    ]
}

/// (c): the four couplings equal within ten per cent (the largest at most eleven tenths of
/// the smallest) and none zero.
fn balanced(c: &[i64; 4]) -> bool {
    let min = c.iter().copied().min().unwrap_or(0);
    let max = c.iter().copied().max().unwrap_or(0);
    min > 0 && max.saturating_mul(10) <= min.saturating_mul(11)
}

/// One block: the correct trials of sixty-four; the trials that presented stimulus A; the
/// readouts' spikes in the window after the volley summed over the block, by the stimulus
/// presented (`[stimulus][readout]`); the presented stimulus set's own spikes in the ticks
/// before the window opens (the volley), summed over the block by stimulus; the stimulus
/// sets' own spikes over the whole trial, summed over the block; the readouts' spikes in the
/// window before the injection, summed over the block; the trials in which the window after
/// the volley held more readout spikes than the window before it (the calibration's
/// measure); the inhibitory and the excitatory sum over the arena after the block; the
/// modulator's signal after the block's last reward; the excitatory coupling from each
/// stimulus set into each readout set after the block (`[stimulus][readout]`); and the
/// trials whose two counts tied, which select nothing and are errors (ADR-0059).
type Block = (
    u32,
    u32,
    [[u64; 2]; 2],
    [u64; 2],
    [u64; 2],
    [u64; 2],
    u32,
    i64,
    i64,
    i32,
    [[i64; 2]; 2],
    u32,
);

/// A run of `trials` trials on the prior at `units` at `gain` on `workers` workers under a
/// delivery of the dopamine term (ADR-0068; `Delivery::Global` is ADR-0066's form): the
/// blocks' readings, and the FNV-1a hash of every trial's `(stimulus, selection, correct)`,
/// the accuracy sequence in one number. The first trial is preceded by a lead-in of one
/// window under the drive: `run_behind` with no whole windows before it.
#[allow(clippy::too_many_arguments)]
fn run(
    units: u32,
    workers: usize,
    gain: u32,
    baseline_q16: i32,
    feedback: Feedback,
    mirrored: bool,
    delivery: Delivery,
    trials: usize,
) -> (Vec<Block>, u64) {
    run_behind(
        0,
        SHAPE_F46,
        None,
        units,
        workers,
        gain,
        baseline_q16,
        feedback,
        mirrored,
        delivery,
        trials,
        &mut |_, _, _, _| {},
    )
}

/// `run` behind a lead-in of `lead_in_windows` whole windows under the drive alone
/// (brief 031), before the instrument's own lead-in of one readout window, with the
/// stimulus of `shape` (brief 033; `run` passes `SHAPE_F46`) and its cancel (brief 034;
/// `run` passes `None`); the executor, the task and every constant are `run`'s, and the
/// windows, the shape and the cancel are the only differences. `observe` is called after
/// every trial with the executor, the trial's index, its first tick and its outcome
/// (brief 032's composition reads the arena and the train through it); it reads and never
/// writes, so a run that observes nothing is the run before it.
#[allow(clippy::too_many_arguments)]
fn run_behind(
    lead_in_windows: u64,
    shape: Shape,
    cancel: Option<Cancel>,
    units: u32,
    workers: usize,
    gain: u32,
    baseline_q16: i32,
    feedback: Feedback,
    mirrored: bool,
    delivery: Delivery,
    trials: usize,
    observe: &mut dyn FnMut(&mut Engine, usize, u32, &Outcome),
) -> (Vec<Block>, u64) {
    let p = prior(units);
    let mut exec = at_gain(&p, config(units, workers, baseline_q16), gain);
    assert_eq!(exec.homeostasis().synaptic_gain_q16, gain);
    assert_eq!(exec.modulation_baseline_q16(), baseline_q16);
    let task = task(shape, cancel, units, feedback, mirrored, delivery);
    lead_in(&mut exec, &task.drive, lead_in_windows);
    run_on(&mut exec, task, units, trials, observe)
}

/// `run_behind` from the executor's present state (brief 035): the instrument's own lead-in
/// of one readout window under the task's drive, then `trials` trials of `task`, read as
/// `run_behind` reads them. The executor is the caller's, so a lead-in, a settling or an
/// image the caller made stands before the first trial; `run_behind` builds the network and
/// runs its whole windows, then calls this.
fn run_on(
    exec: &mut Engine,
    mut task: Task,
    units: u32,
    trials: usize,
    observe: &mut dyn FnMut(&mut Engine, usize, u32, &Outcome),
) -> (Vec<Block>, u64) {
    assert_eq!(
        exec.addressed_counts(),
        (units as usize, units as usize),
        "every unit a source and a target before the first trial, whatever the delivery"
    );
    task.check(exec).expect("the task fits the executor");
    let [a, b, r0, r1] = geometry(units, rotation(units));
    // The stimulus sets counted as a readout would count them: the same rule, the other
    // two sets.
    let stimuli = Readout::new([a, b]);
    let inject = exec.injector();
    for _ in 0..LEAD_IN {
        task.drive
            .step(&inject, exec.ticks())
            .expect("the drive runs");
        exec.tick();
    }
    let mut blocks = Vec::new();
    let mut sequence: Vec<i32> = Vec::with_capacity(trials);
    let mut correct = 0u32;
    let mut a_trials = 0u32;
    let mut spikes = [[0u64; 2]; 2];
    let mut volley = [0u64; 2];
    let mut stimulus_spikes = [0u64; 2];
    let mut before_spikes = [0u64; 2];
    let mut seen = 0u32;
    let mut ties = 0u32;
    for trial in 0..trials {
        let overwritten = exec.train_overwritten();
        let start = exec.ticks() as u32;
        let before =
            task.readout
                .count_window(exec.train(), start.wrapping_sub(WINDOW.ticks), WINDOW.ticks);
        let outcome = task.trial(exec, trial as u64).expect("a trial runs");
        assert!(
            exec.train_overwritten().saturating_sub(overwritten)
                <= u64::from(spikes_per_unit(TRIAL_TICKS)).saturating_mul(u64::from(units)),
            "the ring let go no more than one trial's bound"
        );
        let in_stimuli = stimuli.count(exec.train(), start, TRIAL_TICKS);
        let in_volley = stimuli.count_window(exec.train(), start, WINDOW.from);
        correct = correct.saturating_add(u32::from(outcome.correct));
        a_trials = a_trials.saturating_add(u32::from(outcome.stimulus == 0));
        let s = usize::from(outcome.stimulus);
        spikes[s][0] = spikes[s][0].saturating_add(u64::from(outcome.counts[0]));
        spikes[s][1] = spikes[s][1].saturating_add(u64::from(outcome.counts[1]));
        volley[s] = volley[s].saturating_add(u64::from(in_volley[s]));
        stimulus_spikes[0] = stimulus_spikes[0].saturating_add(u64::from(in_stimuli[0]));
        stimulus_spikes[1] = stimulus_spikes[1].saturating_add(u64::from(in_stimuli[1]));
        before_spikes[0] = before_spikes[0].saturating_add(u64::from(before[0]));
        before_spikes[1] = before_spikes[1].saturating_add(u64::from(before[1]));
        let after_total = outcome.counts[0].saturating_add(outcome.counts[1]);
        let before_total = before[0].saturating_add(before[1]);
        seen = seen.saturating_add(u32::from(after_total > before_total));
        ties = ties.saturating_add(u32::from(outcome.selection.is_none()));
        observe(exec, trial, start, &outcome);
        sequence.push(
            i32::from(outcome.stimulus)
                | i32::from(outcome.selection.map_or(3, |r| r)) << 1
                | i32::from(outcome.correct) << 3,
        );
        if trial.wrapping_add(1) % BLOCK == 0 {
            let (inhibitory, excitatory) = weights_by_polarity(exec);
            blocks.push((
                correct,
                a_trials,
                spikes,
                volley,
                stimulus_spikes,
                before_spikes,
                seen,
                inhibitory,
                excitatory,
                exec.modulator().dopamine_rpe,
                [
                    [coupling(exec, a, r0), coupling(exec, a, r1)],
                    [coupling(exec, b, r0), coupling(exec, b, r1)],
                ],
                ties,
            ));
            correct = 0;
            a_trials = 0;
            spikes = [[0; 2]; 2];
            volley = [0; 2];
            stimulus_spikes = [0; 2];
            before_spikes = [0; 2];
            seen = 0;
            ties = 0;
        }
    }
    (blocks, fnv1a_64(&sequence))
}

/// Dumps a run, then holds it to its pinned table and, where one is pinned, its trace.
fn pinned(name: &str, blocks: &[Block], trace: u64, table: &[Block], pin: u64) {
    eprintln!(
        "DUMP {name} {blocks:?} trace {trace:#018x} curve {:?}",
        curve(blocks)
    );
    assert_eq!(blocks, table, "{name}");
    if pin != 0 {
        assert_eq!(trace, pin, "{name}: the accuracy sequence");
    }
}

/// The correct trials per block of a run, for the dump.
fn curve(blocks: &[Block]) -> Vec<u32> {
    blocks.iter().map(|b| b.0).collect()
}

// ------------------------------------------------------------- the calibration (ADR-0065)

/// The calibration's measure over one block: the readouts' spikes after the volley against
/// the spikes before the injection, by readout, and the trials in which the window after
/// held more.
fn measure(block: &Block) -> ([u64; 2], [u64; 2], u32) {
    let after = [
        block.2[0][0].saturating_add(block.2[1][0]),
        block.2[0][1].saturating_add(block.2[1][1]),
    ];
    (after, block.5, block.6)
}

/// A gain passes when the window after the volley held more readout spikes than the window
/// before it in at least `SEEN_MIN` of the block's trials, and each readout's total after
/// exceeds its total before.
fn calibrated(block: &Block) -> bool {
    let (after, before, seen) = measure(block);
    seen >= SEEN_MIN && after[0] > before[0] && after[1] > before[1]
}

/// The gain a size's calibration picks: the first candidate, in the candidates' order, that
/// passed; none when neither did.
fn picked(calibration: &[(Block, u64); 2]) -> Option<u32> {
    GAINS
        .iter()
        .zip(calibration.iter())
        .find(|(_, (block, _))| calibrated(block))
        .map(|(&gain, _)| gain)
}

/// One calibration run: sixty-four trials at `gain` with the modulation baseline at zero
/// and no reward, so that no weight of either polarity moves (asserted against the sums
/// before the run); the block's readings and the run's trace.
fn calibration(units: u32, gain: u32) -> (Block, u64) {
    let p = prior(units);
    let frozen = at_gain(&p, config(units, 2, 0), gain);
    let sums = weights_by_polarity(&frozen);
    let (blocks, trace) = run(
        units,
        2,
        gain,
        0,
        Feedback::Withheld,
        false,
        Delivery::Global,
        BLOCK,
    );
    let block = blocks[0];
    assert_eq!(
        (block.7, block.8),
        sums,
        "no weight moves at a modulation of zero"
    );
    (block, trace)
}

/// The calibration at 256 units, both candidates in the order tried: at 1.75 the window
/// after the volley held more readout spikes than the window before in 50 trials of 64
/// (148 and 181 against 54 and 51 over the block) and at 2.0 in 58 (481 and 516 against 231
/// and 249), so the gain at 256 units is 2.0. The presented set fires 10.9 spikes per
/// presentation in the volley for its eleven units.
const CALIBRATION_256: [(Block, u64); 2] = [
    (
        (
            20,
            34,
            [[80, 98], [68, 83]],
            [374, 329],
            [1300, 1174],
            [54, 51],
            50,
            53_475_744,
            58_968_247,
            0,
            [[1_487_772, 1_409_916], [1_458_815, 1_477_095]],
            21,
        ),
        0xa2cf2b82436c5cc3,
    ),
    (
        (
            25,
            34,
            [[250, 284], [231, 232]],
            [370, 325],
            [1995, 2017],
            [231, 249],
            58,
            53_475_744,
            58_968_247,
            0,
            [[1_487_772, 1_409_916], [1_458_815, 1_477_095]],
            11,
        ),
        0xee01046fee1077e9,
    ),
];
/// The calibration at 1 024 units: at 1.75 the window after held more in 62 trials of 64
/// (775 and 741 against 255 and 241), so the gain at 1 024 units is 1.75; 2.0, tried second,
/// would have passed too (63; 2 119 and 2 094 against 1 293 and 1 215). The presented set
/// fires 50.8 spikes per presentation in the volley for its fifty-one units.
const CALIBRATION_1024: [(Block, u64); 2] = [
    (
        (
            27,
            34,
            [[396, 378], [379, 363]],
            [1727, 1526],
            [6171, 5504],
            [255, 241],
            62,
            213_902_976,
            235_822_619,
            0,
            [[6_986_739, 7_212_710], [7_146_878, 7_257_384]],
            8,
        ),
        0xa84553d901278f4f,
    ),
    (
        (
            25,
            34,
            [[1063, 1050], [1056, 1044]],
            [1697, 1503],
            [10_291, 8904],
            [1293, 1215],
            63,
            213_902_976,
            235_822_619,
            0,
            [[6_986_739, 7_212_710], [7_146_878, 7_257_384]],
            9,
        ),
        0x92d20337e23f16cb,
    ),
];

#[test]
#[ignore]
fn the_calibration_at_256_units_exhaustive() {
    for (k, gain) in GAINS.iter().enumerate() {
        let (block, trace) = calibration(256, *gain);
        let (table, pin) = CALIBRATION_256[k];
        pinned(
            &format!("calibrate256 {gain:#x}"),
            &[block],
            trace,
            &[table],
            pin,
        );
    }
}

#[test]
#[ignore]
fn the_calibration_at_1024_units_exhaustive() {
    for (k, gain) in GAINS.iter().enumerate() {
        let (block, trace) = calibration(1024, *gain);
        let (table, pin) = CALIBRATION_1024[k];
        pinned(
            &format!("calibrate1024 {gain:#x}"),
            &[block],
            trace,
            &[table],
            pin,
        );
    }
}

/// The gains are the first candidates that passed, and the measure reads the pinned blocks
/// as the constants above say.
#[test]
fn the_picked_gains_are_the_first_candidates_that_passed() {
    assert_eq!(picked(&CALIBRATION_256), Some(GAIN_256));
    assert_eq!(picked(&CALIBRATION_1024), Some(GAIN_1024));
    assert_eq!(GAIN_256, GAINS[1], "2.0 at 256 units");
    assert_eq!(GAIN_1024, GAINS[0], "1.75 at 1 024 units");
    assert_eq!(measure(&CALIBRATION_256[0].0), ([148, 181], [54, 51], 50));
    assert!(
        !calibrated(&CALIBRATION_256[0].0),
        "50 of 64 is below the mark"
    );
    assert_eq!(measure(&CALIBRATION_256[1].0), ([481, 516], [231, 249], 58));
    assert!(calibrated(&CALIBRATION_256[1].0));
    assert_eq!(
        measure(&CALIBRATION_1024[0].0),
        ([775, 741], [255, 241], 62)
    );
    assert!(calibrated(&CALIBRATION_1024[0].0));
    assert_eq!(
        measure(&CALIBRATION_1024[1].0),
        ([2119, 2094], [1293, 1215], 63)
    );
    assert!(calibrated(&CALIBRATION_1024[1].0));
    // The volley: the presented set's spikes before the window opens, per presentation, are
    // one per unit of the set at both sizes and both gains, within two spikes (a unit the
    // background fired within its refractory window before the injection misses the
    // volley): 11 units, 374 and 370 over 34 A trials, 329 and 325 over 30 B trials; 51
    // units, 1 727 and 1 697, 1 526 and 1 503.
    for (units, table) in [(256u32, &CALIBRATION_256), (1024, &CALIBRATION_1024)] {
        let [a, _, _, _] = geometry(units, rotation(units));
        for (block, _) in table {
            let a_trials = u64::from(block.1);
            let b_trials = 64u64.saturating_sub(a_trials);
            let per_a = block.3[0].saturating_mul(10) / a_trials;
            let per_b = block.3[1].saturating_mul(10) / b_trials;
            let once = a.len().saturating_mul(10);
            assert!(
                per_a <= once && per_a >= once.saturating_sub(20),
                "{units}: {per_a} tenths per A presentation against {once}"
            );
            assert!(
                per_b <= once && per_b >= once.saturating_sub(20),
                "{units}: {per_b} tenths per B presentation against {once}"
            );
        }
    }
    // The pass mark's arithmetic at its edges.
    let mut block = CALIBRATION_256[1].0;
    block.6 = SEEN_MIN;
    assert!(calibrated(&block), "56 of 64 passes");
    block.6 = SEEN_MIN.wrapping_sub(1);
    assert!(!calibrated(&block), "55 does not");
    block.6 = 64;
    block.5 = [481, 249];
    assert!(
        !calibrated(&block),
        "a readout no higher after than before fails"
    );
    block.5 = [480, 515];
    assert!(calibrated(&block), "one spike above, each");
    // Neither passing picks none, and the second alone picks the second.
    let mut failed = [(block, 0), (block, 0)];
    failed[0].0.6 = 0;
    failed[1].0.6 = 0;
    assert_eq!(picked(&failed), None);
    let second = [failed[0], (block, 0)];
    assert_eq!(picked(&second), Some(GAINS[1]));
}

/// The geometry against the prior's census at both sizes, before any run: (a) no stimulus
/// unit within the prior's window of another and none inhibitory; (b) two readouts of equal
/// size, disjoint from every stimulus unit, each stimulus unit with as many units of the one
/// in its window as of the other; (c) the four couplings equal within ten per cent and none
/// zero, at the written rotation and at no smaller one; and the window as the rule of the
/// delay band derives it. The task fits the executor for every feedback and both
/// assignments.
#[test]
fn the_geometry_holds_against_the_census_at_both_sizes() {
    for (units, expected_count, expected_couplings) in [
        (
            256u32,
            11u32,
            [1_487_772i64, 1_409_916, 1_458_815, 1_477_095],
        ),
        (1024, 51, [6_986_739, 7_212_710, 7_146_878, 7_257_384]),
    ] {
        let p = prior(units);
        let exec = at_gain(&p, config(units, 1, 0), gain(units));
        let r = rotation(units);
        let sets = geometry(units, r);
        let [a, b, r0, r1] = sets;
        assert_eq!(a.count, expected_count, "{units}: whole periods from {r}");
        assert_eq!(
            (a.len(), b.len()),
            (u64::from(expected_count), u64::from(expected_count))
        );
        assert_eq!(r0.len(), r1.len(), "(b): equal readouts");
        assert_eq!(r0.len(), u64::from(expected_count).saturating_mul(9));
        // (a): every stimulus unit beyond the window of every other, and none inhibitory.
        let stimulus_units: Vec<u32> = a.units().chain(b.units()).collect();
        for (i, &x) in stimulus_units.iter().enumerate() {
            assert!(
                !p.is_inhibitory(x),
                "{units}: stimulus unit {x} is inhibitory"
            );
            for &y in stimulus_units.iter().skip(i.saturating_add(1)) {
                assert!(
                    ring_distance(units, x, y) > PRIOR_WINDOW,
                    "{units}: stimulus units {x} and {y} within the window"
                );
            }
        }
        // (b): each stimulus unit has as many units of either readout within its window,
        // four on either side in a whole period and never more.
        for &x in &stimulus_units {
            let within = |set: Set| {
                (0..units)
                    .filter(|&u| set.contains(u) && ring_distance(units, x, u) <= PRIOR_WINDOW)
                    .count()
            };
            assert_eq!(within(r0), within(r1), "{units}: stimulus unit {x}");
            assert!(
                within(r0) <= 8 && within(r0) >= 4,
                "{units}: {x} sees {}",
                within(r0)
            );
        }
        // Every unit from the rotation to the last whole period's end is in exactly one set.
        let pattern_end = r.saturating_add(a.count.saturating_mul(PERIOD));
        for u in r..pattern_end {
            let n = sets.iter().filter(|s| s.contains(u)).count();
            assert_eq!(n, 1, "{units}: unit {u} is in {n} sets");
        }
        for u in (0..r).chain(pattern_end..units) {
            assert!(sets.iter().all(|s| !s.contains(u)), "{units}: unit {u}");
        }
        // (c) at the written rotation, and at no smaller one.
        let c = couplings(&exec, &sets);
        assert_eq!(c, expected_couplings, "{units}");
        assert!(balanced(&c));
        for smaller in 0..r {
            assert!(
                !balanced(&couplings(&exec, &geometry(units, smaller))),
                "{units}: rotation {smaller} is balanced before {r}"
            );
        }
        // The window is the delay band's rule, and the far band lies outside it.
        assert_eq!(WINDOW.from, u32::from(p.delay_min));
        assert_eq!(WINDOW.end(), u64::from(p.delay_max).saturating_mul(2));
        assert!(WINDOW.end() < u64::from(p.far_delay_min));
        assert_eq!(LEAD_IN, WINDOW.ticks);
        // The task fits, for every feedback and both assignments.
        for feedback in [Feedback::Answer, Feedback::Shuffled] {
            let exec = Engine::new(config(units, 1, BASELINE_Q16)).unwrap();
            assert_eq!(
                task(SHAPE_F46, None, units, feedback, false, Delivery::Global).check(&exec),
                Ok(())
            );
            assert_eq!(
                task(SHAPE_F46, None, units, feedback, true, Delivery::Global).check(&exec),
                Ok(())
            );
        }
        let fixed = Engine::new(config(units, 1, ONE)).unwrap();
        assert_eq!(
            task(
                SHAPE_F46,
                None,
                units,
                Feedback::Withheld,
                false,
                Delivery::Global
            )
            .check(&fixed),
            Ok(())
        );
        assert_eq!(
            task(
                SHAPE_F46,
                None,
                units,
                Feedback::Answer,
                false,
                Delivery::Global
            )
            .check(&fixed),
            Err(TaskError::RewardAtCeiling)
        );
        let frozen = Engine::new(config(units, 1, 0)).unwrap();
        assert_eq!(
            task(
                SHAPE_F46,
                None,
                units,
                Feedback::Withheld,
                false,
                Delivery::Global
            )
            .check(&frozen),
            Ok(())
        );
    }
    // The balance rule at its edge: ten per cent is eleven tenths of the smallest.
    assert!(balanced(&[100, 110, 105, 100]));
    assert!(!balanced(&[100, 111, 105, 100]));
    assert!(!balanced(&[0, 110, 105, 100]), "none zero");
    assert!(!balanced(&[-1, 1, 1, 1]));
    assert_eq!(
        (TRIALS as u64).saturating_mul(u64::from(TRIAL_TICKS)),
        1 << 23,
        "a run is 2^23 ticks"
    );
}

// --------------------------------------------------------------- the criterion (ADR-0066)

/// The correct trials over the last `LAST_BLOCKS` blocks.
fn last_correct(blocks: &[Block]) -> u32 {
    blocks
        .iter()
        .rev()
        .take(LAST_BLOCKS)
        .fold(0u32, |sum, b| sum.saturating_add(b.0))
}

/// The criterion of ADR-0066, clause by clause: the rewarded run's correct trials over the
/// last 128 at least `REWARDED_MIN`, in both assignments; the shuffled reward's and the
/// fixed modulation's at most `CONTROL_MAX`; the accuracy sequence the same on one worker
/// and on four. `learned` is all five.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Verdict {
    rewarded: bool,
    mirrored: bool,
    shuffled: bool,
    fixed: bool,
    workers: bool,
    learned: bool,
}

fn verdict(
    rewarded: &[Block],
    mirrored: &[Block],
    shuffled: &[Block],
    fixed: &[Block],
    workers: bool,
) -> Verdict {
    let v = Verdict {
        rewarded: last_correct(rewarded) >= REWARDED_MIN,
        mirrored: last_correct(mirrored) >= REWARDED_MIN,
        shuffled: last_correct(shuffled) <= CONTROL_MAX,
        fixed: last_correct(fixed) <= CONTROL_MAX,
        workers,
        learned: false,
    };
    Verdict {
        learned: v.rewarded && v.mirrored && v.shuffled && v.fixed && v.workers,
        ..v
    }
}

/// Row `n` of Pascal's triangle, $\binom{n}{0} \ldots \binom{n}{n}$, in `u128` by the
/// additive rule, which never overflows for `n` at most 128 (the largest coefficient is
/// below $2^{127}$; the multiplicative form's product before its division would).
fn row(n: usize) -> Vec<u128> {
    let mut row = vec![0u128; n.saturating_add(1)];
    row[0] = 1;
    for r in 1..=n {
        for j in (1..=r).rev() {
            row[j] = row[j].saturating_add(row[j.wrapping_sub(1)]);
        }
    }
    row
}

/// $\sum_{j \ge k} \binom{n}{j}$ from the row: the numerator of the binomial's upper tail
/// at $p = \tfrac12$ over $2^n$.
fn tail(row: &[u128], k: usize) -> u128 {
    row.iter()
        .skip(k)
        .fold(0u128, |sum, &c| sum.saturating_add(c))
}

/// The criterion's arithmetic reads as written, and its rates are the binomial's: over 128
/// trials at chance, 80 or more by noise about once in 337 and 77 or more about once in 74
/// (`u128::MAX / tail`, within one of $2^{128} / \text{tail}$); brief 027's clauses over
/// two blocks of 64, a rise of ten about once in 21 and a rise of four about once in 3.
#[test]
fn the_criterion_reads_as_written() {
    let block = |correct: u32| -> Block {
        (
            correct,
            0,
            [[0; 2]; 2],
            [0; 2],
            [0; 2],
            [0; 2],
            0,
            0,
            0,
            0,
            [[0; 2]; 2],
            0,
        )
    };
    let eight = |a: u32, b: u32| -> Vec<Block> {
        let mut v = vec![block(32); 6];
        v.push(block(a));
        v.push(block(b));
        v
    };
    assert_eq!(last_correct(&eight(40, 40)), 80);
    assert_eq!(last_correct(&eight(50, 29)), 79);
    assert_eq!(last_correct(&[block(64)]), 64, "one block counts as itself");
    assert_eq!(last_correct(&[]), 0);
    let up = eight(40, 40);
    let short = eight(40, 39);
    let flat = eight(38, 38);
    let over = eight(38, 39);
    assert_eq!(
        verdict(&up, &up, &flat, &flat, true),
        Verdict {
            rewarded: true,
            mirrored: true,
            shuffled: true,
            fixed: true,
            workers: true,
            learned: true
        }
    );
    assert_eq!(
        verdict(&short, &up, &flat, &over, true),
        Verdict {
            rewarded: false,
            mirrored: true,
            shuffled: true,
            fixed: false,
            workers: true,
            learned: false
        },
        "79 of 128 fails the rewarded clause and 77 fails a control's"
    );
    assert!(
        !verdict(&up, &short, &flat, &flat, true).learned,
        "both assignments"
    );
    assert!(!verdict(&up, &up, &over, &flat, true).shuffled);
    assert!(!verdict(&up, &up, &flat, &flat, false).learned);
    // The oracle: the tails in integers, against the figures written before the run
    // (Python's `math.comb` computed the same numerators apart from the tree).
    let of_128 = row(128);
    let of_64 = row(64);
    assert_eq!(
        of_128[64],
        23_951_146_041_928_082_866_135_587_776_380_551_750
    );
    assert_eq!((of_128[0], of_128[128], of_128.len()), (1, 1, 129));
    assert_eq!(of_64[10], 151_473_214_816);
    assert_eq!(tail(&of_64, 0), 1 << 64, "the whole row is 2^64");
    let at_least_80 = tail(&of_128, REWARDED_MIN as usize);
    let above_76 = tail(&of_128, CONTROL_MAX.saturating_add(1) as usize);
    assert_eq!(
        at_least_80,
        1_008_121_664_319_070_533_864_957_670_526_556_173
    );
    assert_eq!(above_76, 4_548_753_949_735_880_922_667_241_324_957_483_213);
    assert_eq!(u128::MAX / at_least_80, 337, "the rewarded clause by noise");
    assert_eq!(
        u128::MAX / above_76,
        74,
        "a control's ceiling exceeded by noise"
    );
    // Brief 027's clauses, two independent blocks of 64: the rise's numerator over 2^128.
    let rise = |at_least: usize| -> u128 {
        let mut sum = 0u128;
        for first in 0..=64usize {
            for last in first.saturating_add(at_least)..=64 {
                sum = sum.saturating_add(of_64[first].saturating_mul(of_64[last]));
            }
        }
        sum
    };
    assert_eq!(
        u128::MAX / rise(10),
        21,
        "ADR-0060's rewarded clause by noise"
    );
    assert_eq!(
        u128::MAX / rise(4),
        3,
        "ADR-0060's control clause failed by noise"
    );
}

// ------------------------------------------------------------ the measurement (ADR-0066)

/// The 256-unit form at the gain the calibration picked (2.0): 512 trials, eight blocks.
/// Each run is its own weekly `exhaustive` test; the pinned tables below are the runs' as
/// the engine produced them, the criterion's test reads the tables, and the gate runs the
/// rewarded run's first block (ADR-0061).
const REWARDED_256: &[Block] = &[
    (
        28,
        34,
        [[202, 222], [200, 202]],
        [371, 326],
        [1885, 1873],
        [229, 233],
        57,
        52_361_380,
        48_281_155,
        -56_439,
        [[983_329, 938_855], [950_214, 997_274]],
        9,
    ),
    (
        27,
        31,
        [[139, 162], [148, 150]],
        [338, 355],
        [1745, 1851],
        [231, 205],
        46,
        51_528_899,
        43_234_480,
        -112_031,
        [[787_567, 740_290], [779_456, 794_820]],
        8,
    ),
    (
        26,
        31,
        [[140, 157], [135, 148]],
        [335, 361],
        [1682, 1745],
        [191, 217],
        44,
        50_566_801,
        40_014_833,
        87_150,
        [[683_704, 662_362], [670_201, 662_578]],
        11,
    ),
    (
        33,
        28,
        [[130, 112], [120, 137]],
        [304, 392],
        [1560, 1846],
        [196, 175],
        42,
        49_332_645,
        37_323_666,
        -96_335,
        [[604_442, 612_136], [575_761, 589_365]],
        9,
    ),
    (
        37,
        30,
        [[108, 120], [93, 139]],
        [327, 370],
        [1600, 1785],
        [185, 215],
        38,
        48_000_750,
        35_402_883,
        -61_863,
        [[562_351, 568_462], [538_010, 548_929]],
        7,
    ),
    (
        28,
        36,
        [[130, 141], [97, 112]],
        [395, 308],
        [1810, 1606],
        [188, 163],
        42,
        46_895_024,
        34_136_212,
        59_399,
        [[530_695, 539_297], [541_956, 542_158]],
        8,
    ),
    (
        22,
        30,
        [[107, 144], [104, 126]],
        [327, 369],
        [1591, 1787],
        [187, 211],
        36,
        45_746_830,
        33_229_310,
        -106_113,
        [[512_519, 532_004], [522_511, 519_005]],
        13,
    ),
    (
        31,
        32,
        [[130, 121], [101, 123]],
        [349, 350],
        [1589, 1711],
        [189, 222],
        31,
        44_541_544,
        32_394_170,
        18_921,
        [[484_090, 484_645], [510_708, 512_616]],
        6,
    ),
];
const TRACE_256: u64 = 0x4b0afbb563e291c3;
const MIRRORED_256: &[Block] = &[
    (
        32,
        34,
        [[188, 217], [192, 202]],
        [371, 324],
        [1896, 1869],
        [232, 238],
        56,
        52_376_380,
        47_896_122,
        88_856,
        [[943_706, 909_075], [949_340, 984_919]],
        4,
    ),
    (
        28,
        31,
        [[136, 153], [152, 149]],
        [338, 355],
        [1735, 1841],
        [228, 201],
        42,
        51_472_579,
        42_875_947,
        111_625,
        [[767_221, 723_455], [774_070, 774_193]],
        10,
    ),
    (
        28,
        31,
        [[133, 152], [123, 154]],
        [336, 362],
        [1688, 1737],
        [202, 223],
        41,
        50_366_424,
        39_555_619,
        -45_728,
        [[664_222, 642_121], [645_177, 656_215]],
        11,
    ),
    (
        23,
        28,
        [[118, 110], [121, 129]],
        [304, 393],
        [1561, 1844],
        [198, 171],
        42,
        49_400_426,
        37_397_711,
        96_213,
        [[598_989, 577_919], [568_964, 591_399]],
        10,
    ),
    (
        22,
        30,
        [[106, 125], [95, 135]],
        [327, 371],
        [1609, 1804],
        [182, 218],
        39,
        48_294_322,
        35_912_003,
        29_766,
        [[562_735, 541_973], [543_314, 571_182]],
        11,
    ),
    (
        27,
        36,
        [[131, 143], [96, 115]],
        [395, 308],
        [1809, 1604],
        [192, 170],
        40,
        47_141_791,
        34_590_493,
        -34_870,
        [[533_267, 529_352], [549_299, 560_169]],
        9,
    ),
    (
        29,
        30,
        [[110, 141], [109, 127]],
        [327, 370],
        [1594, 1792],
        [182, 219],
        41,
        45_844_271,
        33_506_486,
        -110_453,
        [[515_210, 519_713], [533_129, 544_933]],
        16,
    ),
    (
        24,
        32,
        [[130, 124], [104, 131]],
        [349, 350],
        [1601, 1708],
        [195, 214],
        31,
        44_667_876,
        32_681_564,
        -68_369,
        [[497_659, 496_302], [526_086, 510_303]],
        12,
    ),
];
const SHUFFLED_256: &[Block] = &[
    (
        28,
        34,
        [[207, 219], [190, 193]],
        [371, 325],
        [1910, 1894],
        [236, 231],
        55,
        52_441_684,
        48_874_413,
        105_959,
        [[973_866, 951_500], [963_381, 1_010_598]],
        6,
    ),
    (
        30,
        31,
        [[137, 156], [154, 145]],
        [338, 354],
        [1748, 1860],
        [238, 205],
        44,
        51_609_561,
        43_747_881,
        -47_354,
        [[773_547, 744_582], [797_319, 813_200]],
        5,
    ),
    (
        27,
        31,
        [[134, 156], [130, 157]],
        [335, 362],
        [1689, 1750],
        [198, 219],
        46,
        50_604_594,
        40_110_815,
        49_150,
        [[674_526, 657_540], [667_126, 675_087]],
        10,
    ),
    (
        29,
        28,
        [[124, 115], [123, 128]],
        [304, 392],
        [1558, 1848],
        [200, 173],
        39,
        49_384_153,
        37_445_419,
        111_552,
        [[585_396, 587_847], [563_554, 592_656]],
        11,
    ),
    (
        35,
        30,
        [[106, 117], [100, 134]],
        [327, 372],
        [1614, 1799],
        [178, 210],
        33,
        47_910_985,
        35_396_856,
        -25_029,
        [[552_938, 547_701], [539_215, 554_443]],
        10,
    ),
    (
        28,
        36,
        [[131, 141], [99, 114]],
        [395, 307],
        [1814, 1602],
        [189, 167],
        43,
        46_925_247,
        34_313_813,
        19_035,
        [[534_125, 533_462], [548_288, 541_510]],
        9,
    ),
    (
        22,
        30,
        [[110, 140], [103, 131]],
        [327, 370],
        [1593, 1789],
        [186, 213],
        36,
        45_431_383,
        33_170_249,
        94_524,
        [[512_328, 524_382], [522_604, 525_876]],
        12,
    ),
    (
        29,
        32,
        [[133, 124], [103, 126]],
        [349, 350],
        [1593, 1705],
        [191, 221],
        32,
        44_085_735,
        32_302_095,
        -47_100,
        [[486_908, 490_839], [516_995, 502_421]],
        6,
    ),
];
const FIXED_256: &[Block] = &[
    (
        26,
        34,
        [[195, 215], [185, 185]],
        [370, 326],
        [1874, 1853],
        [215, 224],
        55,
        52_040_170,
        46_218_375,
        0,
        [[909_382, 872_278], [880_088, 923_029]],
        8,
    ),
    (
        25,
        31,
        [[130, 144], [146, 142]],
        [339, 353],
        [1723, 1796],
        [232, 194],
        44,
        50_726_380,
        40_594_139,
        0,
        [[701_740, 676_871], [720_576, 715_831]],
        7,
    ),
    (
        23,
        31,
        [[127, 141], [121, 145]],
        [335, 362],
        [1653, 1721],
        [197, 212],
        42,
        49_220_999,
        37_260_901,
        0,
        [[613_603, 599_581], [604_417, 601_551]],
        15,
    ),
    (
        29,
        28,
        [[117, 100], [118, 126]],
        [305, 392],
        [1537, 1826],
        [200, 173],
        40,
        47_596_723,
        34_891_795,
        0,
        [[548_693, 551_456], [525_831, 541_873]],
        13,
    ),
    (
        35,
        30,
        [[98, 122], [93, 130]],
        [327, 370],
        [1579, 1773],
        [180, 211],
        35,
        45_893_073,
        33_420_974,
        0,
        [[521_736, 527_691], [516_554, 528_002]],
        9,
    ),
    (
        31,
        36,
        [[130, 135], [96, 111]],
        [395, 308],
        [1791, 1594],
        [184, 164],
        40,
        44_332_392,
        32_325_074,
        0,
        [[509_015, 504_647], [525_571, 534_969]],
        7,
    ),
    (
        23,
        30,
        [[104, 138], [101, 125]],
        [327, 370],
        [1586, 1779],
        [179, 211],
        39,
        42_588_464,
        31_447_208,
        0,
        [[489_771, 508_936], [505_183, 508_030]],
        11,
    ),
    (
        28,
        32,
        [[127, 124], [104, 122]],
        [349, 350],
        [1578, 1702],
        [191, 220],
        31,
        40_908_307,
        30_846_947,
        0,
        [[478_579, 479_598], [500_762, 491_982]],
        8,
    ),
];
const VERDICT_256: Verdict = Verdict {
    rewarded: false,
    mirrored: false,
    shuffled: true,
    fixed: true,
    workers: true,
    learned: false,
};

/// The gate's run: the rewarded run's first block at 256 units, sixty-four trials of $2^{14}$
/// ticks, held to the first row of the table the full run pinned; no number is pinned twice.
#[test]
fn the_first_block_of_the_recalibrated_rewarded_run_at_256_units() {
    let (blocks, trace) = run(
        256,
        2,
        GAIN_256,
        BASELINE_Q16,
        Feedback::Answer,
        false,
        Delivery::Global,
        BLOCK,
    );
    pinned(
        "instrument256 first block",
        &blocks,
        trace,
        &REWARDED_256[..1],
        0,
    );
}

#[test]
#[ignore]
fn the_recalibrated_rewarded_run_at_256_units_on_four_workers_exhaustive() {
    let (blocks, trace) = run(
        256,
        4,
        GAIN_256,
        BASELINE_Q16,
        Feedback::Answer,
        false,
        Delivery::Global,
        TRIALS,
    );
    pinned(
        "instrument256 rewarded",
        &blocks,
        trace,
        REWARDED_256,
        TRACE_256,
    );
}

/// The second worker count: the same run, block for block and trial for trial (the table
/// and the trace were pinned from four workers).
#[test]
#[ignore]
fn the_recalibrated_rewarded_run_at_256_units_on_one_worker_is_the_same_run_exhaustive() {
    let (blocks, trace) = run(
        256,
        1,
        GAIN_256,
        BASELINE_Q16,
        Feedback::Answer,
        false,
        Delivery::Global,
        TRIALS,
    );
    pinned(
        "instrument256 rewarded-1",
        &blocks,
        trace,
        REWARDED_256,
        TRACE_256,
    );
}

#[test]
#[ignore]
fn the_recalibrated_mirrored_assignment_at_256_units_exhaustive() {
    let (blocks, trace) = run(
        256,
        2,
        GAIN_256,
        BASELINE_Q16,
        Feedback::Answer,
        true,
        Delivery::Global,
        TRIALS,
    );
    pinned("instrument256 mirrored", &blocks, trace, MIRRORED_256, 0);
}

#[test]
#[ignore]
fn the_recalibrated_shuffled_reward_at_256_units_exhaustive() {
    let (blocks, trace) = run(
        256,
        2,
        GAIN_256,
        BASELINE_Q16,
        Feedback::Shuffled,
        false,
        Delivery::Global,
        TRIALS,
    );
    pinned("instrument256 shuffled", &blocks, trace, SHUFFLED_256, 0);
}

#[test]
#[ignore]
fn the_recalibrated_fixed_modulation_at_256_units_exhaustive() {
    let (blocks, trace) = run(
        256,
        2,
        GAIN_256,
        ONE,
        Feedback::Withheld,
        false,
        Delivery::Global,
        TRIALS,
    );
    pinned("instrument256 fixed", &blocks, trace, FIXED_256, 0);
}

/// The criterion over the pinned tables at 256 units, clause by clause; the worker clause is
/// the test above that holds one worker to four workers' table.
#[test]
fn the_criterion_at_256_units_through_the_instrument_as_written() {
    let v = verdict(REWARDED_256, MIRRORED_256, SHUFFLED_256, FIXED_256, true);
    eprintln!("DUMP instrument256 verdict {v:?}");
    assert_eq!(v, VERDICT_256);
}

/// The 1 024-unit form at the gain the calibration picked (1.75): 512 trials, eight blocks.
const REWARDED_1024: &[Block] = &[
    (
        27,
        34,
        [[376, 361], [361, 349]],
        [1727, 1525],
        [6142, 5474],
        [249, 229],
        61,
        210_427_900,
        230_916_261,
        -84_933,
        [[6_493_095, 6_799_725], [6_705_098, 6_817_017]],
        8,
    ),
    (
        21,
        31,
        [[304, 329], [359, 344]],
        [1572, 1676],
        [5654, 5889],
        [241, 255],
        64,
        208_008_451,
        227_482_898,
        -112_151,
        [[6_255_219, 6_502_025], [6_421_477, 6_496_745]],
        4,
    ),
    (
        27,
        31,
        [[327, 343], [299, 302]],
        [1577, 1679],
        [5619, 5851],
        [248, 237],
        62,
        204_824_565,
        223_771_696,
        -109_231,
        [[5_913_232, 6_181_071], [6_093_045, 6_244_583]],
        5,
    ),
    (
        31,
        28,
        [[282, 255], [320, 326]],
        [1423, 1827],
        [5115, 6243],
        [265, 220],
        63,
        201_295_600,
        220_527_358,
        109_063,
        [[5_665_304, 5_991_844], [5_802_861, 5_971_959]],
        6,
    ),
    (
        30,
        30,
        [[263, 247], [274, 286]],
        [1530, 1732],
        [5471, 5951],
        [224, 225],
        60,
        197_825_311,
        217_575_437,
        80_911,
        [[5_416_069, 5_799_363], [5_507_726, 5_662_839]],
        11,
    ),
    (
        26,
        36,
        [[272, 248], [248, 218]],
        [1832, 1425],
        [6301, 5097],
        [234, 250],
        58,
        194_611_651,
        215_165_041,
        -88_442,
        [[5_125_532, 5_489_094], [5_363_502, 5_494_672]],
        9,
    ),
    (
        25,
        30,
        [[251, 243], [298, 274]],
        [1525, 1729],
        [5346, 5926],
        [252, 232],
        58,
        191_506_629,
        212_911_869,
        92_059,
        [[4_919_506, 5_340_292], [5_119_722, 5_273_076]],
        7,
    ),
    (
        24,
        32,
        [[234, 278], [258, 256]],
        [1629, 1629],
        [5689, 5619],
        [248, 253],
        57,
        188_702_707,
        211_017_969,
        29_991,
        [[4_784_180, 5_152_084], [4_924_049, 5_108_595]],
        5,
    ),
];
const TRACE_1024: u64 = 0xd35f65f1e45140ab;
const MIRRORED_1024: &[Block] = &[
    (
        34,
        34,
        [[364, 359], [361, 340]],
        [1727, 1525],
        [6129, 5475],
        [253, 233],
        60,
        209_792_108,
        230_207_898,
        69_101,
        [[6_420_795, 6_725_625], [6_705_954, 6_784_935]],
        5,
    ),
    (
        39,
        31,
        [[299, 316], [351, 332]],
        [1572, 1676],
        [5644, 5867],
        [243, 260],
        64,
        205_654_739,
        225_315_860,
        112_093,
        [[6_116_317, 6_331_028], [6_270_822, 6_303_448]],
        5,
    ),
    (
        30,
        31,
        [[321, 344], [296, 296]],
        [1578, 1679],
        [5608, 5842],
        [253, 243],
        62,
        202_241_102,
        221_698_313,
        27_863,
        [[5_747_309, 6_012_548], [5_971_815, 6_065_383]],
        7,
    ),
    (
        22,
        28,
        [[277, 244], [306, 312]],
        [1423, 1829],
        [5111, 6237],
        [272, 214],
        63,
        199_439_026,
        219_139_194,
        -109_934,
        [[5_538_155, 5_837_081], [5_715_329, 5_843_072]],
        10,
    ),
    (
        22,
        30,
        [[262, 238], [285, 281]],
        [1530, 1732],
        [5469, 5949],
        [217, 228],
        60,
        196_872_179,
        216_905_494,
        -80_948,
        [[5_283_881, 5_631_537], [5_538_397, 5_659_971]],
        11,
    ),
    (
        32,
        36,
        [[261, 244], [239, 217]],
        [1833, 1425],
        [6299, 5093],
        [235, 259],
        58,
        193_408_520,
        214_290_049,
        88_442,
        [[5_030_693, 5_362_796], [5_350_038, 5_410_356]],
        5,
    ),
    (
        32,
        30,
        [[243, 242], [290, 276]],
        [1525, 1730],
        [5341, 5917],
        [247, 235],
        58,
        189_512_145,
        211_851_286,
        -92_093,
        [[4_795_672, 5_195_167], [5_114_786, 5_172_675]],
        7,
    ),
    (
        33,
        32,
        [[238, 277], [250, 251]],
        [1628, 1629],
        [5658, 5611],
        [231, 260],
        59,
        185_500_296,
        209_518_496,
        -91_027,
        [[4_615_099, 4_989_680], [4_897_248, 4_997_023]],
        8,
    ),
];
const SHUFFLED_1024: &[Block] = &[
    (
        27,
        34,
        [[368, 361], [367, 345]],
        [1727, 1524],
        [6142, 5473],
        [256, 231],
        61,
        210_639_650,
        231_216_688,
        105_959,
        [[6_490_716, 6_773_268], [6_749_565, 6_837_479]],
        5,
    ),
    (
        22,
        31,
        [[314, 328], [355, 337]],
        [1572, 1676],
        [5660, 5887],
        [237, 259],
        64,
        207_699_748,
        227_567_247,
        -47_354,
        [[6_216_739, 6_416_894], [6_417_147, 6_491_039]],
        5,
    ),
    (
        26,
        31,
        [[324, 334], [296, 306]],
        [1578, 1679],
        [5633, 5862],
        [254, 241],
        61,
        204_324_583,
        223_701_566,
        49_150,
        [[5_852_301, 6_108_536], [6_078_916, 6_213_713]],
        10,
    ),
    (
        32,
        28,
        [[285, 248], [315, 320]],
        [1423, 1828],
        [5115, 6243],
        [272, 223],
        62,
        200_484_804,
        220_236_635,
        111_552,
        [[5_607_337, 5_887_031], [5_729_530, 5_913_131]],
        6,
    ),
    (
        33,
        30,
        [[270, 248], [271, 282]],
        [1530, 1732],
        [5461, 5947],
        [225, 230],
        60,
        196_108_503,
        216_619_182,
        -25_029,
        [[5_313_039, 5_642_233], [5_412_583, 5_585_270]],
        11,
    ),
    (
        28,
        36,
        [[268, 240], [238, 217]],
        [1833, 1425],
        [6303, 5095],
        [234, 257],
        58,
        193_029_377,
        214_275_489,
        19_035,
        [[5_056_613, 5_375_196], [5_260_653, 5_391_522]],
        9,
    ),
    (
        27,
        30,
        [[244, 238], [283, 275]],
        [1525, 1730],
        [5342, 5922],
        [246, 231],
        58,
        188_598_008,
        211_483_797,
        94_524,
        [[4_807_940, 5_187_464], [5_001_768, 5_123_320]],
        3,
    ),
    (
        28,
        32,
        [[247, 274], [248, 255]],
        [1629, 1629],
        [5671, 5609],
        [235, 254],
        59,
        184_707_688,
        209_162_019,
        -47_100,
        [[4_643_423, 4_974_975], [4_780_585, 4_952_133]],
        7,
    ),
];
const FIXED_1024: &[Block] = &[
    (
        26,
        34,
        [[363, 357], [362, 335]],
        [1727, 1525],
        [6119, 5467],
        [252, 232],
        61,
        208_089_383,
        228_225_119,
        0,
        [[6_289_430, 6_627_747], [6_574_171, 6_675_503]],
        4,
    ),
    (
        21,
        31,
        [[291, 312], [339, 332]],
        [1572, 1677],
        [5611, 5844],
        [244, 260],
        64,
        202_727_661,
        222_337_377,
        0,
        [[5_940_067, 6_166_206], [6_097_651, 6_173_941]],
        7,
    ),
    (
        29,
        31,
        [[307, 333], [278, 285]],
        [1578, 1679],
        [5569, 5819],
        [250, 242],
        62,
        197_291_101,
        217_293_045,
        0,
        [[5_522_296, 5_796_889], [5_684_375, 5_847_779]],
        4,
    ),
    (
        36,
        28,
        [[265, 232], [282, 300]],
        [1424, 1829],
        [5062, 6201],
        [269, 221],
        61,
        191_556_498,
        213_234_844,
        0,
        [[5_234_146, 5_571_742], [5_315_193, 5_520_576]],
        5,
    ),
    (
        30,
        30,
        [[245, 234], [263, 263]],
        [1530, 1732],
        [5415, 5901],
        [212, 221],
        60,
        185_885_652,
        209_732_113,
        0,
        [[4_937_632, 5_351_981], [4_999_732, 5_205_343]],
        10,
    ),
    (
        28,
        36,
        [[251, 231], [207, 196]],
        [1833, 1427],
        [6246, 5055],
        [245, 255],
        56,
        180_206_849,
        206_642_589,
        0,
        [[4_662_813, 5_017_097], [4_804_684, 4_946_144]],
        8,
    ),
    (
        29,
        30,
        [[227, 231], [277, 274]],
        [1527, 1730],
        [5291, 5871],
        [250, 239],
        57,
        174_305_265,
        203_905_215,
        0,
        [[4_425_440, 4_821_311], [4_547_152, 4_695_690]],
        5,
    ),
    (
        31,
        32,
        [[231, 265], [223, 238]],
        [1629, 1629],
        [5630, 5579],
        [242, 258],
        56,
        168_442_172,
        201_552_838,
        0,
        [[4_243_523, 4_607_215], [4_305_115, 4_514_524]],
        3,
    ),
];
const VERDICT_1024: Verdict = Verdict {
    rewarded: false,
    mirrored: false,
    shuffled: true,
    fixed: true,
    workers: true,
    learned: false,
};

#[test]
#[ignore]
fn the_recalibrated_rewarded_run_at_1024_units_on_four_workers_exhaustive() {
    let (blocks, trace) = run(
        1024,
        4,
        GAIN_1024,
        BASELINE_Q16,
        Feedback::Answer,
        false,
        Delivery::Global,
        TRIALS,
    );
    pinned(
        "instrument1024 rewarded",
        &blocks,
        trace,
        REWARDED_1024,
        TRACE_1024,
    );
}

#[test]
#[ignore]
fn the_recalibrated_rewarded_run_at_1024_units_on_one_worker_is_the_same_run_exhaustive() {
    let (blocks, trace) = run(
        1024,
        1,
        GAIN_1024,
        BASELINE_Q16,
        Feedback::Answer,
        false,
        Delivery::Global,
        TRIALS,
    );
    pinned(
        "instrument1024 rewarded-1",
        &blocks,
        trace,
        REWARDED_1024,
        TRACE_1024,
    );
}

#[test]
#[ignore]
fn the_recalibrated_mirrored_assignment_at_1024_units_exhaustive() {
    let (blocks, trace) = run(
        1024,
        2,
        GAIN_1024,
        BASELINE_Q16,
        Feedback::Answer,
        true,
        Delivery::Global,
        TRIALS,
    );
    pinned("instrument1024 mirrored", &blocks, trace, MIRRORED_1024, 0);
}

#[test]
#[ignore]
fn the_recalibrated_shuffled_reward_at_1024_units_exhaustive() {
    let (blocks, trace) = run(
        1024,
        2,
        GAIN_1024,
        BASELINE_Q16,
        Feedback::Shuffled,
        false,
        Delivery::Global,
        TRIALS,
    );
    pinned("instrument1024 shuffled", &blocks, trace, SHUFFLED_1024, 0);
}

#[test]
#[ignore]
fn the_recalibrated_fixed_modulation_at_1024_units_exhaustive() {
    let (blocks, trace) = run(
        1024,
        2,
        GAIN_1024,
        ONE,
        Feedback::Withheld,
        false,
        Delivery::Global,
        TRIALS,
    );
    pinned("instrument1024 fixed", &blocks, trace, FIXED_1024, 0);
}

/// The criterion over the pinned tables at 1 024 units.
#[test]
fn the_criterion_at_1024_units_through_the_instrument_as_written() {
    let v = verdict(
        REWARDED_1024,
        MIRRORED_1024,
        SHUFFLED_1024,
        FIXED_1024,
        true,
    );
    eprintln!("DUMP instrument1024 verdict {v:?}");
    assert_eq!(v, VERDICT_1024);
}

// ------------------------------------------- the addressed delivery (ADR-0068, ADR-0069)
//
// Written before the run. The same instrument, the same controls, the same seeds and the
// same counts as ADR-0066; the one variable is where the dopamine term reaches: under
// `Delivery::Addressed` the synapses onto the readout the engine selected, none at a tie
// (the addressed set is the outcome's, written by the task between the trial's last tick and
// the reward; the presynaptic side does not narrow it, ADR-0068), under `Delivery::Global`
// every synapse alike, ADR-0066's form, which reruns under this round's code as the
// comparison the claim rests on. The criterion is at 1 024 units; 256 units is one reading
// under the rule below.

/// The rule for a size that stops seeing, written before the run: a size whose calibration
/// measure (the trials of a block in which the window after the volley held more readout
/// spikes than the window before the injection) falls below `SEEN_MIN` in any block before
/// the criterion's window is recorded as not measured, never as a pass or a fail. ADR-0066
/// read 256 units below the mark from its second block and 1 024 units above it throughout.
fn sees_through(blocks: &[Block]) -> bool {
    let before_window = blocks.len().saturating_sub(LAST_BLOCKS);
    blocks
        .iter()
        .take(before_window)
        .all(|block| block.6 >= SEEN_MIN)
}

/// The criterion under the addressed delivery (ADR-0069), ADR-0066's clause by clause with
/// the global form beside the controls: the addressed rewarded run's correct trials over the
/// last 128 at least `REWARDED_MIN`, in both assignments; the addressed shuffled reward's,
/// the fixed modulation's and the global form's at most `CONTROL_MAX`; the addressed
/// rewarded run's sequence the same on one worker and on four. `learned` is all six.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct AddressedVerdict {
    rewarded: bool,
    mirrored: bool,
    shuffled: bool,
    fixed: bool,
    global: bool,
    workers: bool,
    learned: bool,
}

fn addressed_verdict(
    rewarded: &[Block],
    mirrored: &[Block],
    shuffled: &[Block],
    fixed: &[Block],
    global: &[Block],
    workers: bool,
) -> AddressedVerdict {
    let v = AddressedVerdict {
        rewarded: last_correct(rewarded) >= REWARDED_MIN,
        mirrored: last_correct(mirrored) >= REWARDED_MIN,
        shuffled: last_correct(shuffled) <= CONTROL_MAX,
        fixed: last_correct(fixed) <= CONTROL_MAX,
        global: last_correct(global) <= CONTROL_MAX,
        workers,
        learned: false,
    };
    AddressedVerdict {
        learned: v.rewarded && v.mirrored && v.shuffled && v.fixed && v.global && v.workers,
        ..v
    }
}

/// The addressed criterion and the sees-through rule read as written, at their edges.
#[test]
fn the_addressed_criterion_and_the_sees_through_rule_read_as_written() {
    let block = |correct: u32, seen: u32| -> Block {
        (
            correct,
            0,
            [[0; 2]; 2],
            [0; 2],
            [0; 2],
            [0; 2],
            seen,
            0,
            0,
            0,
            [[0; 2]; 2],
            0,
        )
    };
    let eight = |a: u32, b: u32| -> Vec<Block> {
        let mut v = vec![block(32, 64); 6];
        v.push(block(a, 64));
        v.push(block(b, 64));
        v
    };
    let up = eight(40, 40);
    let short = eight(40, 39);
    let flat = eight(38, 38);
    let over = eight(38, 39);
    let all = AddressedVerdict {
        rewarded: true,
        mirrored: true,
        shuffled: true,
        fixed: true,
        global: true,
        workers: true,
        learned: true,
    };
    assert_eq!(addressed_verdict(&up, &up, &flat, &flat, &flat, true), all);
    assert_eq!(
        addressed_verdict(&short, &up, &flat, &over, &flat, true),
        AddressedVerdict {
            rewarded: false,
            fixed: false,
            learned: false,
            ..all
        },
        "79 of 128 fails the rewarded clause and 77 fails a control's"
    );
    assert_eq!(
        addressed_verdict(&up, &up, &flat, &flat, &over, true),
        AddressedVerdict {
            global: false,
            learned: false,
            ..all
        },
        "the global form is a control: 77 fails it"
    );
    assert_eq!(
        addressed_verdict(&up, &up, &flat, &flat, &up, true),
        AddressedVerdict {
            global: false,
            learned: false,
            ..all
        },
        "a global form that learned would fail the addressed claim's comparison"
    );
    assert!(!addressed_verdict(&up, &short, &flat, &flat, &flat, true).learned);
    assert!(!addressed_verdict(&up, &up, &over, &flat, &flat, true).shuffled);
    assert!(!addressed_verdict(&up, &up, &flat, &flat, &flat, false).learned);
    // The sees-through rule: every block before the criterion's window at the mark or
    // above; a block below it before the window is a reading not measured, a block below it
    // inside the window is not the rule's concern; a run shorter than the window sees.
    assert!(sees_through(&eight(32, 32)));
    let mut lost_early = eight(32, 32);
    lost_early[0].6 = SEEN_MIN.wrapping_sub(1);
    assert!(!sees_through(&lost_early), "55 in the first block");
    let mut lost_sixth = eight(32, 32);
    lost_sixth[5].6 = SEEN_MIN.wrapping_sub(1);
    assert!(
        !sees_through(&lost_sixth),
        "55 in the sixth, the last before the window"
    );
    let mut at_mark = eight(32, 32);
    at_mark[5].6 = SEEN_MIN;
    assert!(sees_through(&at_mark), "56 is the mark");
    let mut lost_in_window = eight(32, 32);
    lost_in_window[6].6 = 0;
    lost_in_window[7].6 = 0;
    assert!(
        sees_through(&lost_in_window),
        "the window's own blocks are not the rule's"
    );
    assert!(sees_through(&[]), "nothing before the window");
    assert!(sees_through(&[block(32, 0), block(32, 0)]));
    assert!(!sees_through(&[block(32, 0), block(32, 64), block(32, 64)]));
    // ADR-0066's runs under the rule: 1 024 units sees through every run, 256 units none.
    for table in [REWARDED_1024, MIRRORED_1024, SHUFFLED_1024, FIXED_1024] {
        assert!(sees_through(table));
    }
    for table in [REWARDED_256, MIRRORED_256, SHUFFLED_256, FIXED_256] {
        assert!(!sees_through(table));
    }
    assert_eq!(
        REWARDED_256.iter().map(|b| b.6).collect::<Vec<u32>>(),
        [57, 46, 44, 42, 38, 42, 36, 31],
        "ADR-0066's reading at 256 units: below the mark from the second block"
    );
}

// ------------------------------------------------- the measurement, addressed (ADR-0069)

/// The addressed rewarded run at 1 024 units, the gain 1.75: 512 trials, eight blocks, on
/// four workers; the same run on one worker. The other runs at 1 024 units and the one
/// reading at 256 follow. Every table is the engine's own, pinned from one run.
const ADDRESSED_1024: &[Block] = &[
    (
        24,
        34,
        [[363, 359], [363, 339]],
        [1727, 1525],
        [6138, 5469],
        [251, 232],
        61,
        209840321,
        230036999,
        -91922,
        [[6352193, 6767883], [6695954, 6741722]],
        8,
    ),
    (
        20,
        31,
        [[299, 320], [350, 328]],
        [1572, 1676],
        [5636, 5869],
        [240, 263],
        64,
        206116890,
        225361098,
        -112151,
        [[6034948, 6364842], [6315559, 6274589]],
        8,
    ),
    (
        29,
        31,
        [[324, 345], [291, 300]],
        [1577, 1679],
        [5601, 5840],
        [251, 238],
        62,
        202335976,
        221198844,
        -87406,
        [[5638657, 6057918], [5949954, 5968162]],
        5,
    ),
    (
        30,
        28,
        [[270, 248], [301, 317]],
        [1423, 1828],
        [5098, 6224],
        [273, 220],
        62,
        198336847,
        217793631,
        87077,
        [[5374260, 5860103], [5611568, 5676560]],
        9,
    ),
    (
        32,
        30,
        [[251, 243], [275, 276]],
        [1530, 1732],
        [5453, 5935],
        [219, 225],
        59,
        194479928,
        214719302,
        80936,
        [[5091101, 5671035], [5322333, 5360730]],
        9,
    ),
    (
        25,
        36,
        [[253, 247], [230, 206]],
        [1833, 1425],
        [6291, 5083],
        [239, 260],
        58,
        190556391,
        212037088,
        -88451,
        [[4808504, 5382082], [5139875, 5115799]],
        6,
    ),
    (
        29,
        30,
        [[245, 243], [278, 272]],
        [1525, 1730],
        [5328, 5911],
        [250, 233],
        57,
        186505360,
        209547846,
        92165,
        [[4577601, 5220432], [4896016, 4867378]],
        3,
    ),
    (
        27,
        32,
        [[226, 274], [246, 239]],
        [1628, 1629],
        [5657, 5602],
        [228, 244],
        58,
        182495646,
        207400983,
        31411,
        [[4416757, 5031874], [4698487, 4700314]],
        5,
    ),
];
const ADDRESSED_TRACE_1024: u64 = 0x35238a0892c87363;
const ADDRESSED_MIRRORED_1024: &[Block] = &[
    (
        31,
        34,
        [[365, 358], [361, 342]],
        [1727, 1525],
        [6137, 5469],
        [251, 232],
        60,
        209843510,
        229951437,
        84733,
        [[6387363, 6666469], [6639144, 6784676]],
        7,
    ),
    (
        35,
        31,
        [[298, 312], [343, 332]],
        [1572, 1677],
        [5632, 5873],
        [239, 263],
        64,
        206124863,
        225217502,
        112093,
        [[6070946, 6240702], [6199346, 6327555]],
        9,
    ),
    (
        30,
        31,
        [[322, 345], [286, 296]],
        [1577, 1679],
        [5597, 5839],
        [250, 240],
        62,
        202353296,
        221015892,
        27863,
        [[5691518, 5897219], [5810546, 6047872]],
        4,
    ),
    (
        22,
        28,
        [[272, 237], [295, 316]],
        [1423, 1828],
        [5095, 6229],
        [272, 219],
        63,
        198360989,
        217641444,
        -109934,
        [[5439822, 5687188], [5466490, 5769290]],
        9,
    ),
    (
        22,
        30,
        [[259, 233], [273, 271]],
        [1530, 1732],
        [5451, 5935],
        [218, 227],
        59,
        194505947,
        214630001,
        -80939,
        [[5191866, 5472155], [5180974, 5503674]],
        11,
    ),
    (
        31,
        36,
        [[261, 237], [225, 213]],
        [1833, 1425],
        [6293, 5083],
        [236, 255],
        58,
        190597661,
        211950344,
        86813,
        [[4947506, 5168378], [4997364, 5266694]],
        7,
    ),
    (
        33,
        30,
        [[239, 235], [285, 274]],
        [1525, 1730],
        [5323, 5910],
        [245, 236],
        56,
        186553543,
        209439531,
        -92165,
        [[4711272, 4994859], [4732740, 5016482]],
        6,
    ),
    (
        32,
        32,
        [[236, 269], [240, 240]],
        [1628, 1629],
        [5652, 5592],
        [230, 248],
        58,
        182553563,
        207266033,
        -91027,
        [[4543951, 4792801], [4519381, 4854166]],
        6,
    ),
];
const ADDRESSED_SHUFFLED_1024: &[Block] = &[
    (
        25,
        34,
        [[365, 358], [363, 341]],
        [1727, 1525],
        [6138, 5469],
        [251, 231],
        61,
        209836061,
        230015138,
        105959,
        [[6367139, 6743062], [6659622, 6760931]],
        7,
    ),
    (
        21,
        31,
        [[299, 319], [344, 328]],
        [1572, 1677],
        [5633, 5872],
        [240, 263],
        64,
        206117930,
        225293328,
        -47354,
        [[6048682, 6332423], [6244718, 6296918]],
        8,
    ),
    (
        31,
        31,
        [[325, 347], [289, 299]],
        [1577, 1679],
        [5601, 5841],
        [251, 240],
        62,
        202332944,
        221115510,
        49150,
        [[5651943, 6023395], [5860626, 6001721]],
        3,
    ),
    (
        33,
        28,
        [[270, 246], [296, 317]],
        [1423, 1829],
        [5096, 6226],
        [271, 220],
        62,
        198344261,
        217709708,
        111552,
        [[5396395, 5818960], [5517275, 5713211]],
        7,
    ),
    (
        33,
        30,
        [[257, 244], [272, 276]],
        [1530, 1732],
        [5454, 5940],
        [219, 225],
        60,
        194494267,
        214590843,
        -25029,
        [[5131871, 5597278], [5224073, 5413314]],
        9,
    ),
    (
        25,
        36,
        [[256, 247], [228, 212]],
        [1833, 1425],
        [6290, 5079],
        [238, 254],
        58,
        190568856,
        211952745,
        19035,
        [[4865707, 5317038], [5048221, 5173250]],
        6,
    ),
    (
        26,
        30,
        [[244, 238], [285, 272]],
        [1525, 1730],
        [5325, 5911],
        [251, 235],
        57,
        186507189,
        209458063,
        94524,
        [[4625870, 5132276], [4795081, 4939273]],
        5,
    ),
    (
        27,
        32,
        [[228, 274], [240, 240]],
        [1629, 1629],
        [5658, 5600],
        [231, 251],
        57,
        182504612,
        207261475,
        -47100,
        [[4458442, 4929297], [4581524, 4760296]],
        5,
    ),
];
const ADDRESSED_256: &[Block] = &[
    (
        29,
        34,
        [[194, 214], [193, 196]],
        [371, 325],
        [1889, 1855],
        [230, 239],
        55,
        52318657,
        47323437,
        -110011,
        [[913694, 936764], [945767, 950517]],
        6,
    ),
    (
        21,
        31,
        [[132, 159], [145, 135]],
        [337, 354],
        [1722, 1819],
        [233, 198],
        44,
        51222726,
        41858984,
        -112167,
        [[711414, 712805], [753918, 729555]],
        7,
    ),
    (
        22,
        31,
        [[133, 151], [121, 150]],
        [335, 362],
        [1661, 1745],
        [196, 223],
        43,
        49988841,
        38571473,
        19763,
        [[619518, 639091], [630845, 612368]],
        14,
    ),
    (
        28,
        28,
        [[119, 114], [116, 127]],
        [305, 392],
        [1551, 1834],
        [192, 163],
        40,
        48645459,
        36201873,
        -98136,
        [[554662, 587762], [547639, 555299]],
        13,
    ),
    (
        38,
        30,
        [[105, 117], [92, 131]],
        [327, 371],
        [1593, 1787],
        [176, 214],
        34,
        47196507,
        34606708,
        69209,
        [[529727, 551881], [532705, 533423]],
        7,
    ),
    (
        29,
        36,
        [[124, 137], [95, 115]],
        [395, 308],
        [1797, 1595],
        [192, 170],
        38,
        45894396,
        33421695,
        31485,
        [[508055, 527113], [531193, 537457]],
        8,
    ),
    (
        22,
        30,
        [[107, 138], [105, 129]],
        [327, 370],
        [1590, 1786],
        [186, 216],
        38,
        44392493,
        32468169,
        24959,
        [[495266, 522135], [509156, 518023]],
        12,
    ),
    (
        33,
        32,
        [[134, 126], [100, 127]],
        [349, 350],
        [1592, 1702],
        [190, 215],
        31,
        42961889,
        31729687,
        68263,
        [[479084, 488171], [505446, 502122]],
        6,
    ),
];

#[test]
#[ignore]
fn the_addressed_rewarded_run_at_1024_units_on_four_workers_exhaustive() {
    let (blocks, trace) = run(
        1024,
        4,
        GAIN_1024,
        BASELINE_Q16,
        Feedback::Answer,
        false,
        Delivery::Addressed,
        TRIALS,
    );
    pinned(
        "addressed1024 rewarded",
        &blocks,
        trace,
        ADDRESSED_1024,
        ADDRESSED_TRACE_1024,
    );
}

#[test]
#[ignore]
fn the_addressed_rewarded_run_at_1024_units_on_one_worker_is_the_same_run_exhaustive() {
    let (blocks, trace) = run(
        1024,
        1,
        GAIN_1024,
        BASELINE_Q16,
        Feedback::Answer,
        false,
        Delivery::Addressed,
        TRIALS,
    );
    pinned(
        "addressed1024 rewarded-1",
        &blocks,
        trace,
        ADDRESSED_1024,
        ADDRESSED_TRACE_1024,
    );
}

#[test]
#[ignore]
fn the_addressed_mirrored_assignment_at_1024_units_exhaustive() {
    let (blocks, trace) = run(
        1024,
        2,
        GAIN_1024,
        BASELINE_Q16,
        Feedback::Answer,
        true,
        Delivery::Addressed,
        TRIALS,
    );
    pinned(
        "addressed1024 mirrored",
        &blocks,
        trace,
        ADDRESSED_MIRRORED_1024,
        0,
    );
}

#[test]
#[ignore]
fn the_addressed_shuffled_reward_at_1024_units_exhaustive() {
    let (blocks, trace) = run(
        1024,
        2,
        GAIN_1024,
        BASELINE_Q16,
        Feedback::Shuffled,
        false,
        Delivery::Addressed,
        TRIALS,
    );
    pinned(
        "addressed1024 shuffled",
        &blocks,
        trace,
        ADDRESSED_SHUFFLED_1024,
        0,
    );
}

/// The fixed modulation under the addressed delivery: no signal, so the two modulations are
/// one number and the run is ADR-0066's fixed-modulation run, held to its table; no number
/// is pinned twice.
#[test]
#[ignore]
fn the_fixed_modulation_under_the_addressed_delivery_at_1024_units_is_the_global_form_exhaustive() {
    let (blocks, trace) = run(
        1024,
        2,
        GAIN_1024,
        ONE,
        Feedback::Withheld,
        false,
        Delivery::Addressed,
        TRIALS,
    );
    pinned("addressed1024 fixed", &blocks, trace, FIXED_1024, 0);
}

/// The one reading at 256 units: the addressed rewarded run, under the sees-through rule.
#[test]
#[ignore]
fn the_addressed_rewarded_run_at_256_units_exhaustive() {
    let (blocks, trace) = run(
        256,
        2,
        GAIN_256,
        BASELINE_Q16,
        Feedback::Answer,
        false,
        Delivery::Addressed,
        TRIALS,
    );
    pinned("addressed256 rewarded", &blocks, trace, ADDRESSED_256, 0);
}

/// The criterion's outcome at 1 024 units under the addressed delivery, as the engine
/// produced it: the addressed rewarded run reads 56 correct of the last 128 and the mirrored
/// assignment 65, against the 80 the clause asks for; the addressed shuffled reward 53, the
/// fixed modulation 60 and the global form 49, at or below 76; the run the same on one worker
/// and on four. The rewarded clause fails at 1 024 units with the global form reproduced.
const ADDRESSED_VERDICT_1024: AddressedVerdict = AddressedVerdict {
    rewarded: false,
    mirrored: false,
    shuffled: true,
    fixed: true,
    global: true,
    workers: true,
    learned: false,
};
/// The reading at 256 units: the calibration's measure is 55 of 64 in the first block, below
/// the mark, so the size is not measured under the rule written before the run, as
/// ADR-0066's run was not.
const ADDRESSED_256_SEES: bool = false;
/// The gate's run: the addressed rewarded run's first block at 256 units, sixty-four trials
/// of $2^{14}$ ticks, held to the first row of the table the full run pinned; no number is
/// pinned twice (ADR-0061).
#[test]
fn the_first_block_of_the_addressed_rewarded_run_at_256_units() {
    let (blocks, trace) = run(
        256,
        2,
        GAIN_256,
        BASELINE_Q16,
        Feedback::Answer,
        false,
        Delivery::Addressed,
        BLOCK,
    );
    pinned(
        "addressed256 first block",
        &blocks,
        trace,
        &ADDRESSED_256[..1],
        0,
    );
}

/// The criterion at 1 024 units under the addressed delivery, over the pinned tables, clause
/// by clause; the global form is ADR-0066's rewarded run, rerun under this round's code by
/// its own weekly test and held to its table; the worker clause is the test above that holds
/// one worker to four workers' table.
#[test]
fn the_criterion_at_1024_units_under_the_addressed_delivery_as_written() {
    let v = addressed_verdict(
        ADDRESSED_1024,
        ADDRESSED_MIRRORED_1024,
        ADDRESSED_SHUFFLED_1024,
        FIXED_1024,
        REWARDED_1024,
        true,
    );
    eprintln!("DUMP addressed1024 verdict {v:?}");
    assert_eq!(v, ADDRESSED_VERDICT_1024);
    assert!(
        sees_through(ADDRESSED_1024),
        "1 024 units sees through the run"
    );
    assert!(sees_through(ADDRESSED_MIRRORED_1024));
    assert!(sees_through(ADDRESSED_SHUFFLED_1024));
}

/// The reading at 256 units under the sees-through rule: recorded as not measured when the
/// calibration's measure falls below the mark before the criterion's window, as ADR-0066's
/// run did; otherwise as a reading of the clause.
#[test]
fn the_reading_at_256_units_under_the_addressed_delivery() {
    let sees = sees_through(ADDRESSED_256);
    eprintln!(
        "DUMP addressed256 sees {sees} last {} seen {:?}",
        last_correct(ADDRESSED_256),
        ADDRESSED_256.iter().map(|b| b.6).collect::<Vec<u32>>()
    );
    assert_eq!(sees, ADDRESSED_256_SEES);
}

// ------------------------------------------------ where 256 units settle (brief 031)

/// One window of the settling measurement: the population's spikes over the window, read
/// from the executor's train; the sum of the inhibitory magnitudes and the sum of the
/// excitatory weights over the arena after it; and the fraction of units at the inhibitory
/// rule's target over it (ADR-0057's rule at the image's period, Q16.16).
type SettlingWindow = (u64, i64, i64, u32);

/// Runs `windows` whole windows under `drive` from the executor's clock: brief 031's
/// lead-in, in windows, which precedes the instrument's own lead-in of one readout window
/// (`LEAD_IN`); a countdown, so it ends by construction. Not `settle`, which in
/// `reference.rs` runs quiet until the network is quiescent.
fn lead_in(exec: &mut Engine, drive: &Drive, windows: u64) {
    for _ in 0..windows {
        let until = exec.ticks().wrapping_add(WINDOW_TICKS);
        run_driven(exec, drive, until).expect("the drive runs");
    }
}

/// The population's spikes over `[from, to)` and the fraction of units at the target over
/// it, read from the train, which must hold the whole window: a unit is at the target when
/// its spikes in the window are within a factor of two of the target's count per window,
/// `WINDOW_TICKS / period` (six at the default period of 20 000 ticks), inclusive both
/// ways (ADR-0057). The run's ticks stay below the stamp's width, so the stamp is the tick.
fn spikes_and_at_target(exec: &mut Engine, from: u64, to: u64) -> (u64, u32) {
    let units = exec.units().len();
    let period = exec.istdp_target_period_ticks();
    // The period is at least `ISTDP_PERIOD_MIN_TICKS`, so the quotient exists.
    let target = WINDOW_TICKS.checked_div(u64::from(period)).unwrap_or(0) as u32;
    let mut counts = vec![0u32; units];
    let mut spikes = 0u64;
    for &(tick, unit) in exec.train() {
        let tick = u64::from(tick);
        if tick >= from && tick < to {
            spikes = spikes.saturating_add(1);
            if let Some(count) = counts.get_mut(unit as usize) {
                *count = count.saturating_add(1);
            }
        }
    }
    let at_target = counts
        .iter()
        .filter(|&&count| count.saturating_mul(2) >= target && count <= target.saturating_mul(2))
        .count() as u64;
    // At most the unit count, which is below 2^16: the shift cannot wrap.
    let fraction = (at_target << 16).checked_div(units as u64).unwrap_or(0) as u32;
    (spikes, fraction)
}

/// The excitatory sums of a settling table, window by window.
fn excitatory_sums(table: &[SettlingWindow]) -> Vec<i64> {
    table.iter().map(|w| w.2).collect()
}

/// Brief 026's clause over the excitatory sums of a settling table, `prior` the sum before
/// the first window: the smallest window (from one) at which each of the last
/// `SETTLING_CLAUSE_WINDOWS` windows up to and including it moved the sum by less than
/// `SETTLING_CLAUSE_PER_CENT` per cent of the sum before those windows; none when no window
/// of the table does. Integers throughout: a move of `m` against a reference of `r` is under
/// `p` per cent when `100 m < p r`.
fn settled_at(prior: i64, sums: &[i64]) -> Option<usize> {
    let sum_after = |window: usize| -> Option<i64> {
        window
            .checked_sub(1)
            .map_or(Some(prior), |k| sums.get(k).copied())
    };
    (SETTLING_CLAUSE_WINDOWS..=sums.len()).find(|&window| {
        let first = window.wrapping_sub(SETTLING_CLAUSE_WINDOWS);
        let Some(reference) = sum_after(first) else {
            return false;
        };
        let bound = u64::try_from(reference)
            .unwrap_or(0)
            .saturating_mul(SETTLING_CLAUSE_PER_CENT);
        (first.wrapping_add(1)..=window).all(|k| {
            match (sum_after(k), sum_after(k.wrapping_sub(1))) {
                (Some(now), Some(before)) => now.abs_diff(before).saturating_mul(100) < bound,
                _ => false,
            }
        })
    })
}

/// The lead-in the rule derives from a settling table of `bound` windows: the window at
/// which the clause first holds, or the bound.
fn derived_lead_in(prior: i64, table: &[SettlingWindow], bound: u64) -> u64 {
    settled_at(prior, &excitatory_sums(table)).map_or(bound, |window| window as u64)
}

/// The settling measurement (brief 031): the executor of the task at `units`, as `run`
/// builds it for the rewarded runs (the prior of ADR-0044 at seed 22, the gain the
/// calibration picked held through the image, the modulation baseline 0.5, no controller,
/// no sleep, the inhibitory period at its default), run under the drive alone, no task and
/// no stimulus, for `windows` whole windows from the first tick; per window the reading
/// above. The sums before the first window are the prior's, as the calibration pinned them.
fn settling(units: u32, windows: u64) -> Vec<SettlingWindow> {
    let p = prior(units);
    let mut exec = at_gain(&p, config(units, 2, BASELINE_Q16), gain(units));
    assert_eq!(exec.homeostasis().synaptic_gain_q16, gain(units));
    assert_eq!(exec.modulation_baseline_q16(), BASELINE_Q16);
    let drive = drive(units);
    let mut out = Vec::new();
    for _ in 0..windows {
        let from = exec.ticks();
        let held = exec.train().len() as u64;
        let overwritten = exec.train_overwritten();
        lead_in(&mut exec, &drive, 1);
        let to = exec.ticks();
        // The ring lets go of its oldest entries once full (about 27 windows in at this
        // size): the window is held whole when nothing let go was younger than it, that
        // is, no more than the ring held before it.
        assert!(
            exec.train_overwritten().wrapping_sub(overwritten) <= held,
            "the train held the window"
        );
        let (spikes, at_target) = spikes_and_at_target(&mut exec, from, to);
        let (inhibitory, excitatory) = weights_by_polarity(&exec);
        out.push((spikes, inhibitory, excitatory, at_target));
    }
    out
}

/// The prior's sums at 256 units before any window, as the calibration pinned them with
/// the weights frozen: (inhibitory, excitatory).
const PRIOR_SUMS_256: (i64, i64) = (CALIBRATION_256[1].0.7, CALIBRATION_256[1].0.8);

/// The settling at 256 units over eighty windows, pinned from one run.
const SETTLING_256: &[SettlingWindow] = &[
    (3_090, 53_264_642, 56_693_908, 38_656),
    (2_506, 53_098_805, 55_376_488, 50_944),
    (2_656, 52_941_813, 53_987_339, 47_104),
    (2_595, 52_826_485, 52_729_916, 48_128),
    (2_563, 52_694_975, 51_628_294, 51_200),
    (2_487, 52_522_141, 50_691_256, 50_944),
    (2_481, 52_382_720, 49_769_277, 50_688),
    (2_453, 52_253_156, 48_953_681, 51_968),
    (2_352, 52_143_204, 48_210_448, 52_224),
    (2_480, 52_044_866, 47_404_654, 52_480),
    (2_371, 51_896_212, 46_688_151, 53_248),
    (2_421, 51_755_304, 46_025_327, 50_688),
    (2_294, 51_580_011, 45_439_030, 55_808),
    (2_338, 51_412_291, 44_905_736, 54_016),
    (2_225, 51_213_362, 44_393_555, 55_040),
    (2_352, 51_077_296, 43_862_956, 54_784),
    (2_232, 50_936_006, 43_438_627, 55_296),
    (2_241, 50_793_923, 43_008_702, 56_832),
    (2_182, 50_609_680, 42_622_827, 58_112),
    (2_270, 50_460_955, 42_216_613, 56_320),
    (2_231, 50_287_690, 41_762_576, 55_296),
    (2_183, 50_113_080, 41_407_926, 56_064),
    (2_228, 49_954_210, 41_091_863, 56_320),
    (2_199, 49_811_562, 40_715_016, 57_856),
    (2_197, 49_657_873, 40_371_087, 54_784),
    (2_104, 49_464_144, 40_011_682, 58_112),
    (2_236, 49_319_257, 39_669_924, 57_856),
    (2_182, 49_077_710, 39_389_616, 56_064),
    (2_122, 48_881_959, 39_101_242, 57_856),
    (2_187, 48_710_409, 38_820_609, 56_320),
    (2_102, 48_542_690, 38_573_708, 57_344),
    (2_139, 48_373_885, 38_321_850, 58_368),
    (2_180, 48_226_961, 38_092_697, 58_112),
    (2_193, 48_052_772, 37_819_065, 57_344),
    (2_033, 47_864_074, 37_591_169, 57_600),
    (2_066, 47_670_482, 37_413_730, 58_880),
    (2_148, 47_480_584, 37_211_645, 56_064),
    (2_033, 47_255_921, 37_019_781, 57_600),
    (2_180, 47_074_219, 36_819_756, 58_112),
    (2_067, 46_845_222, 36_630_250, 59_136),
    (2_092, 46_615_556, 36_462_159, 57_856),
    (2_070, 46_462_487, 36_282_041, 58_880),
    (2_110, 46_322_331, 36_112_512, 59_648),
    (2_098, 46_128_940, 35_962_763, 59_904),
    (2_132, 45_967_418, 35_836_423, 57_344),
    (2_069, 45_774_994, 35_700_717, 59_392),
    (2_122, 45_591_778, 35_540_372, 58_368),
    (2_127, 45_405_859, 35_376_910, 57_856),
    (2_053, 45_176_056, 35_242_937, 57_856),
    (2_049, 45_012_321, 35_103_513, 59_904),
    (2_105, 44_829_653, 34_983_990, 58_880),
    (2_079, 44_638_991, 34_866_758, 59_136),
    (1_986, 44_419_882, 34_779_473, 58_368),
    (2_108, 44_222_708, 34_629_361, 58_368),
    (2_025, 44_027_980, 34_503_692, 59_904),
    (2_024, 43_793_594, 34_409_536, 58_368),
    (2_114, 43_617_556, 34_285_231, 59_136),
    (2_028, 43_423_068, 34_171_957, 58_624),
    (2_034, 43_202_652, 34_074_752, 58_368),
    (2_065, 43_029_689, 34_010_524, 59_136),
    (2_111, 42_835_647, 33_899_485, 59_648),
    (1_965, 42_623_357, 33_826_291, 59_392),
    (2_090, 42_474_661, 33_741_523, 57_088),
    (2_037, 42_257_106, 33_632_752, 60_416),
    (2_058, 42_044_259, 33_529_204, 57_856),
    (1_982, 41_827_754, 33_470_054, 58_112),
    (1_952, 41_606_594, 33_381_882, 59_136),
    (2_000, 41_402_480, 33_285_922, 58_112),
    (2_027, 41_193_201, 33_148_561, 58_624),
    (2_054, 40_997_922, 33_082_612, 59_648),
    (2_055, 40_824_662, 33_027_666, 60_160),
    (2_039, 40_611_438, 32_950_616, 57_088),
    (2_078, 40_403_489, 32_834_325, 57_856),
    (2_118, 40_201_031, 32_787_524, 59_136),
    (2_145, 40_021_536, 32_740_288, 58_880),
    (2_056, 39_824_386, 32_725_565, 59_136),
    (2_058, 39_613_863, 32_669_906, 58_112),
    (2_087, 39_401_641, 32_618_796, 56_832),
    (2_035, 39_243_885, 32_595_157, 59_392),
    (2_024, 39_053_140, 32_515_504, 56_832),
];

#[test]
#[ignore]
fn the_settling_at_256_units_exhaustive() {
    let table = settling(256, SETTLING_WINDOWS);
    eprintln!(
        "DUMP settling256 {table:?} settled {:?} lead-in {}",
        settled_at(PRIOR_SUMS_256.1, &excitatory_sums(&table)),
        derived_lead_in(PRIOR_SUMS_256.1, &table, SETTLING_WINDOWS)
    );
    assert_eq!(table.as_slice(), SETTLING_256, "settling256");
}

/// The criterion behind the lead-in (brief 031), written before the run: the calibration's
/// measure (the trials of a block in which the window after the volley held more readout
/// spikes than the window before the injection) at least `SEEN_MIN` of 64 in every block of
/// the run; false for no blocks. Holding, 256 units is usable for this task behind the
/// lead-in and a later round may carry a criterion there; failing, it is not, and the block
/// at which the measure crosses is the reading. The correct trials are a reading, never a
/// clause: the run measures the instrument, not learning.
fn holds_through(blocks: &[Block]) -> bool {
    !blocks.is_empty() && blocks.iter().all(|block| block.6 >= SEEN_MIN)
}

/// The gate's test (ADR-0061's class; brief 031): the clause at its edges against tables
/// written by hand, the lead-in as the rule derives it from the pinned settling table, and
/// the first four windows of the settling, run and held to the first four rows of the table
/// the weekly run pinned; no number is pinned twice.
#[test]
fn the_first_four_windows_of_the_settling_at_256_units_and_the_rules_over_its_tables() {
    // The clause at its edges: four moves of exactly two per cent of the prior's sum fail
    // it (the comparison is strict) and four of just under pass it at the fourth window; a
    // first window's move of three per cent fails the fourth window, and the fifth, whose
    // reference is the first window's sum, holds; a rise is a move; a table shorter than
    // the clause's windows has no window.
    assert_eq!(settled_at(10_000, &[9_800, 9_600, 9_400, 9_200]), None);
    assert_eq!(settled_at(10_000, &[9_801, 9_602, 9_403, 9_204]), Some(4));
    assert_eq!(
        settled_at(10_000, &[9_700, 9_600, 9_500, 9_400, 9_300]),
        Some(5)
    );
    assert_eq!(
        settled_at(10_000, &[10_199, 10_398, 10_597, 10_796]),
        Some(4)
    );
    assert_eq!(settled_at(10_000, &[10_200, 10_398, 10_597, 10_796]), None);
    assert_eq!(settled_at(10_000, &[9_999, 9_998, 9_997]), None);
    assert_eq!(settled_at(10_000, &[]), None);
    // A table on which the clause never holds derives the bound.
    let falling = [
        (0, 0, 9_000, 0),
        (0, 0, 8_000, 0),
        (0, 0, 7_000, 0),
        (0, 0, 6_000, 0),
    ];
    assert_eq!(settled_at(10_000, &excitatory_sums(&falling)), None);
    assert_eq!(derived_lead_in(10_000, &falling, 4), 4);
    // The lead-in is derived, not chosen: the ninth window of the pinned table.
    assert_eq!(SETTLING_256.len() as u64, SETTLING_WINDOWS);
    assert_eq!(
        settled_at(PRIOR_SUMS_256.1, &excitatory_sums(SETTLING_256)),
        Some(9)
    );
    assert_eq!(
        derived_lead_in(PRIOR_SUMS_256.1, SETTLING_256, SETTLING_WINDOWS),
        LEAD_IN_WINDOWS
    );
    // The criterion behind the lead-in at its edges over ADR-0066's pinned runs: 1 024
    // units held the measure at 57 to 64 of 64 through the run and 256 fell to 46 in the
    // second block; no blocks hold nothing.
    assert!(holds_through(REWARDED_1024));
    assert!(!holds_through(REWARDED_256));
    assert!(!holds_through(&[]));
    // The criterion over the pinned run behind the lead-in, as the engine produced it.
    assert_eq!(holds_through(LEAD_IN_256), LEAD_IN_HOLDS);
    assert_eq!(LEAD_IN_256.len(), TRIALS / BLOCK);
    eprintln!(
        "DUMP leadin256 holds {LEAD_IN_HOLDS} seen {:?}",
        LEAD_IN_256.iter().map(|b| b.6).collect::<Vec<u32>>()
    );
    // The first four windows, run.
    let table = settling(256, SETTLING_CLAUSE_WINDOWS as u64);
    eprintln!("DUMP settling256 first four {table:?}");
    assert_eq!(
        table.as_slice(),
        &SETTLING_256[..SETTLING_CLAUSE_WINDOWS],
        "settling256 first four"
    );
}

/// The rewarded run at 256 units behind the lead-in (brief 031): ADR-0066's rewarded run in
/// the task's order — the same prior, gain, geometry, window, trial, block, baseline,
/// reward, seeds and four workers — with `LEAD_IN_WINDOWS` whole windows under the drive
/// alone before the instrument's own lead-in. The run without them is
/// `the_recalibrated_rewarded_run_at_256_units_on_four_workers_exhaustive`, held to
/// `REWARDED_256`, ADR-0066's table, so the windows are the only difference between the
/// two. Pinned from one run, with the trace of every trial's stimulus, selection and
/// outcome.
const LEAD_IN_256: &[Block] = &[
    (
        36,
        34,
        [[189, 184], [125, 172]],
        [370, 324],
        [1860, 1778],
        [219, 213],
        50,
        51_087_300,
        42_302_966,
        86_187,
        [[914_350, 828_681], [866_868, 893_382]],
        7,
    ),
    (
        29,
        31,
        [[151, 144], [138, 158]],
        [338, 363],
        [1709, 1783],
        [197, 216],
        44,
        50_089_181,
        39_078_798,
        103_842,
        [[760_717, 707_823], [706_612, 734_776]],
        12,
    ),
    (
        30,
        31,
        [[128, 124], [131, 133]],
        [337, 359],
        [1696, 1746],
        [189, 180],
        43,
        48_848_769,
        36_580_685,
        -43_878,
        [[635_297, 610_438], [618_391, 626_872]],
        14,
    ),
    (
        32,
        28,
        [[94, 112], [99, 140]],
        [306, 392],
        [1542, 1833],
        [181, 205],
        35,
        47_558_929,
        34_948_120,
        39_078,
        [[574_253, 570_099], [567_596, 572_557]],
        6,
    ),
    (
        26,
        30,
        [[106, 118], [125, 132]],
        [328, 371],
        [1604, 1819],
        [189, 172],
        41,
        46_405_804,
        33_764_237,
        -39_813,
        [[538_332, 547_718], [551_144, 564_469]],
        12,
    ),
    (
        30,
        36,
        [[122, 147], [80, 115]],
        [394, 305],
        [1798, 1599],
        [191, 219],
        34,
        45_123_980,
        32_828_499,
        43_056,
        [[501_695, 500_964], [538_489, 547_462]],
        11,
    ),
    (
        21,
        30,
        [[103, 120], [131, 126]],
        [329, 371],
        [1537, 1796],
        [176, 218],
        36,
        44_206_626,
        32_233_480,
        -112_225,
        [[496_818, 477_874], [530_668, 535_936]],
        13,
    ),
    (
        29,
        32,
        [[111, 126], [101, 116]],
        [350, 349],
        [1648, 1684],
        [194, 197],
        36,
        42_882_344,
        31_666_316,
        27_149,
        [[489_599, 486_545], [527_159, 509_825]],
        7,
    ),
];
const LEAD_IN_TRACE_256: u64 = 0x379636313802f88d;

/// The criterion's outcome behind the lead-in, as the engine produced it: the calibration's
/// measure is 50 of 64 in the first block, below the mark from the start, and 34 to 44
/// after it, so it holds in no block of the run; 256 units is not usable for this task
/// behind the lead-in the rule derived. The gate reads the rule over the pinned table.
const LEAD_IN_HOLDS: bool = false;

#[test]
#[ignore]
fn the_recalibrated_rewarded_run_at_256_units_behind_the_lead_in_exhaustive() {
    let (blocks, trace) = run_behind(
        LEAD_IN_WINDOWS,
        SHAPE_F46,
        None,
        256,
        4,
        GAIN_256,
        BASELINE_Q16,
        Feedback::Answer,
        false,
        Delivery::Global,
        TRIALS,
        &mut |_, _, _, _| {},
    );
    eprintln!(
        "DUMP leadin256 holds {} seen {:?}",
        holds_through(&blocks),
        blocks.iter().map(|b| b.6).collect::<Vec<u32>>()
    );
    pinned(
        "instrument256 rewarded behind the lead-in",
        &blocks,
        trace,
        LEAD_IN_256,
        LEAD_IN_TRACE_256,
    );
}

// ------------------------------------------ what the trace is made of (brief 032)
//
// Written before the run. Three decisions (ADR-0066, ADR-0068, ADR-0069) reasoned from one
// sentence about the eligibility trace on a stimulus's synapses into a readout — "a readout
// unit's background spike within the pair window before the volley is depression, and the
// background's are the more" — and nothing in the tree held a number for it. The
// composition reads it at 1 024 units, in the calibration's configuration (the weights
// frozen: the modulation baseline at zero and no reward, so the traces fill and decay and
// are never written into a weight), over sixty-four trials, per trial: (a) the record's
// summed `eligibility_q1_15` over the synapses from each stimulus set onto each readout
// set; (b) the terms that entered those traces since the previous reading, split by what
// paired them and by their sign; (c) the readouts' spikes in the pair window before the
// volley's arrival and in the pair window after it. The terms are an oracle's: the pair
// rule of `cortex-core` (ADR-0022, ADR-0032, ADR-0049, ADR-0055) replayed over the
// executor's own train, synapse by synapse, whose trace must equal the record's at every
// reading, so that the split is of the numbers the engine computed and of nothing else.
//
// The split, a rule of `STDP_TAU_SHIFT` and the volley's arrival: the rule enters a term at
// a presynaptic spike for the target's last somatic spike $q$, a potentiation when $q$
// followed the block's previous presynaptic spike and a depression when $q$ preceded this
// one; a term is **the volley's** when $q$ lies within one pair window ($2^{11}$ ticks) from
// the arrival of that synapse's volley message at the readout — the presented stimulus
// unit's volley spike plus the synapse's own delay — and **the background's** otherwise.
// The census (c) reads the readouts around the readout window's opening, the trial's first
// tick plus the prior's `delay_min`, before which no local message of the volley can have
// landed (ADR-0065's rule).
//
// Then the ladder: the gain, the one handle the calibration turns (ADR-0065), tried
// downward from the present in a fixed order, each rung a frozen run of sixty-four trials
// read by two measures that see no selection, reward or outcome — *sight*, ADR-0065's,
// unchanged, and *sign*, this round's: the presented stimulus's summed eligibility onto
// both readouts positive in at least fifty-six trials of sixty-four. A rung passes only
// when both pass; the first that does is the gain, and if none does there is no rewarded
// run. The expectation, written first: the background's terms are the product of two
// rates, the stimulus units' and the readouts', and the volley's the product of one and a
// response, so a quieter gain shrinks the background's faster than the volley's; whether a
// rung reaches net potentiation before the sight fails is the measurement.

/// The pair window, one STDP time constant: $2^{11}$ ticks (20.48 ms).
const PAIR_WINDOW: u32 = 1 << STDP_TAU_SHIFT;
const _: () = assert!(PAIR_WINDOW == 2048);
/// The pair window after the readout window's opening lies inside the trial.
const _: () = assert!(WINDOW.from + PAIR_WINDOW < TRIAL_TICKS);
/// The ladder of gains, in the order tried and no other: 1.75 (the present), 1.5, 1.25
/// and 1.0.
const LADDER: [u32; 4] = [0x0001_C000, 0x0001_8000, 0x0001_4000, 0x0001_0000];
const _: () = assert!(LADDER[0] == GAIN_1024 && LADDER[0] == GAINS[0]);
const _: () = assert!(LADDER[0] > LADDER[1] && LADDER[1] > LADDER[2] && LADDER[2] > LADDER[3]);
/// The sign calibration's pass mark: trials of sixty-four in which the presented stimulus's
/// summed eligibility onto both readouts, read from the record after the trial, is
/// positive; as strict as the sight's.
const SIGN_MIN: u32 = 56;
const _: () = assert!(SIGN_MIN == SEEN_MIN);
/// The excitatory depression's reference magnitude, $2^{13}$ (ADR-0055), as the shift that
/// divides by it in the oracle below; the gate holds the oracle to the amounts the rule
/// documents at the reference, at the rail and at the magnitude below which it rounds to
/// nothing, and every reading holds the oracle to the record.
const DEPRESSION_REFERENCE_SHIFT: u32 = 13;
const _: () = assert!(1 << DEPRESSION_REFERENCE_SHIFT == 0x2000);
/// The trials the gate runs of the composition at the present gain, held to the first rows
/// of the pinned table: eight, $2^{17}$ ticks at 1 024 units.
const GATE_TRIALS: usize = 8;

// ------------------------------------------------------------------------- the oracle

/// The pair rule's window at a distance, $A (1 - 2^{-11})^{\Delta t}$ rounded to nearest
/// (`cortex-core`'s `window`, written a second time as the oracle).
fn pair_window(amplitude_q1_15: i16, delta_ticks: u32) -> i16 {
    let factor = i64::from(stp_decay_factor_q16(delta_ticks, STDP_TAU_SHIFT));
    (i64::from(amplitude_q1_15)
        .saturating_mul(factor)
        .saturating_add(0x8000)
        >> 16) as i16
}

/// An excitatory depression at a magnitude (ADR-0055): `amount × magnitude / 2^13`, rounded
/// to nearest (`cortex-core`'s `depression_at`).
fn depression_at(amount_q1_15: i16, magnitude: i32) -> i16 {
    (i64::from(amount_q1_15)
        .saturating_mul(i64::from(magnitude))
        .saturating_add(1 << (DEPRESSION_REFERENCE_SHIFT - 1))
        >> DEPRESSION_REFERENCE_SHIFT) as i16
}

/// A trace decayed by a Q16.16 factor below 1.0, rounded to nearest and by at least one LSB
/// toward zero for a trace that is not zero (`cortex-core`'s `decayed`).
fn decayed(trace: i16, factor_q16: i64) -> i16 {
    if trace == 0 {
        return 0;
    }
    let magnitude = i64::from(trace).abs();
    let kept = (magnitude.saturating_mul(factor_q16).saturating_add(0x8000) >> 16)
        .min(magnitude.saturating_sub(1));
    kept.saturating_mul(i64::from(trace).signum()) as i16
}

/// Whether a readout unit's spike at `q` is the volley's: within one pair window from the
/// arrival of a volley message over a synapse of delay `delay`, `volleys` the presynaptic
/// unit's volley spikes in tick order. Ticks compare as numbers: the run is far below the
/// stamp's width.
fn volleyed(volleys: &[u32], delay: u32, q: u32) -> bool {
    let at = volleys.partition_point(|&v| v.saturating_add(delay) <= q);
    at.checked_sub(1)
        .and_then(|k| volleys.get(k))
        .is_some_and(|&v| q < v.saturating_add(delay).saturating_add(PAIR_WINDOW))
}

/// One synapse from a stimulus unit onto a readout unit as the oracle replays it: where it
/// is in the arena, its ends and the sets they are in, its delay, its magnitude (frozen),
/// and the block's presynaptic stamp and the slot's trace as the rule would hold them.
struct Replayed {
    block_idx: usize,
    slot: usize,
    source: u32,
    target: u32,
    stimulus: usize,
    readout: usize,
    delay: u32,
    magnitude: i32,
    stamp: u32,
    trace: i16,
}

/// One trial's reading of the trace's composition: the stimulus presented; (a) the record's
/// summed eligibility after the trial over the synapses from each stimulus set onto each
/// readout set, `[stimulus][readout]`; (b) the terms that entered those traces since the
/// previous reading (for the first trial, since the run's first tick), by stimulus and
/// readout, as the volley's potentiation, the volley's depression, the background's
/// potentiation and the background's depression, signed as they entered; (c) the spikes in
/// the pair window before the readout window's opening and in the pair window after it, of
/// readout 0, of readout 1 and of the presented stimulus set (whose before is the volley
/// and whose after is what its units fire beyond it), `[set][before, after]`.
type Composed = (u8, [[i64; 2]; 2], [[[i64; 4]; 2]; 2], [[u32; 2]; 3]);

/// The pinned row of a trial: the stimulus presented, the presented stimulus's summed
/// eligibility onto both readouts after the trial (the sign measure reads its sign), and its
/// four classes of terms since the previous reading over both readouts.
type Row = (u8, i64, [i64; 4]);

/// A block of the composition: the sums after the block's last trial; the block's terms by
/// stimulus, readout and class; the census summed over the block; the trials in which the
/// presented stimulus's sum onto both readouts was positive (the sign measure); and the
/// FNV-1a hash of every trial's whole reading.
type Composition = ([[i64; 2]; 2], [[[i64; 4]; 2]; 2], [[u64; 2]; 3], u32, u64);

/// The reader of the composition: the executor's train collected per unit since the run's
/// first tick, the volley spikes of the stimulus units, the synapses from the stimulus sets
/// onto the readout sets with their replayed traces, and the readings.
struct Composer {
    sets: [Set; 4],
    readout: Readout,
    stimuli: Readout,
    spikes: Vec<Vec<u32>>,
    volleys: Vec<Vec<u32>>,
    /// The presented set's volley spikes by their tick after the trial's first, over the run
    /// (brief 034): index `k` counts the units whose volley spike fell on trial tick `k`, for
    /// every `k` before the readout window opens.
    volley_ticks: Vec<u64>,
    synapses: Vec<Replayed>,
    cursor: u32,
    out: Vec<Composed>,
    /// The delivery as the oracle replays it (brief 036; the task's own since brief 037): the
    /// pair the last trial's delivery addressed and the signal its reward left, so that the
    /// oracle consolidates where and by as much as the engine does; none in a frozen run,
    /// where nothing consolidates and the weights are held to the record as they were.
    taught: Option<Taught>,
    /// What each trial consolidated into the weights, `[stimulus][readout]`, by the oracle
    /// (brief 036); zero in a frozen run.
    transferred: Vec<[[i64; 2]; 2]>,
    /// The reward the task itself delivered at the trial's end, before the reading
    /// (brief 037): the record's signal at the reading is the course's end plus it. Zero
    /// where the harness rewards after the reading (brief 036) or nothing rewards.
    rewarded: i32,
}

impl Composer {
    fn new(units: u32) -> Self {
        let sets = geometry(units, rotation(units));
        let [a, b, r0, r1] = sets;
        Self {
            sets,
            readout: Readout::new([r0, r1]),
            stimuli: Readout::new([a, b]),
            spikes: vec![Vec::new(); units as usize],
            volleys: vec![Vec::new(); units as usize],
            volley_ticks: vec![0; WINDOW.from as usize],
            synapses: Vec::new(),
            cursor: 0,
            out: Vec::new(),
            taught: None,
            transferred: Vec::new(),
            rewarded: 0,
        }
    }

    /// The synapses from a stimulus unit onto a readout unit, from the arena, in the walk's
    /// order, each with its block's presynaptic stamp and its slot's trace as the record holds
    /// them, and every unit's last spike on record seeded as the first entry of its list
    /// (brief 035): on a fresh network the stamps and the traces are none and zero and no unit
    /// has spiked, so the oracle starts as it started; on a network a lead-in left, it starts
    /// where the engine is. A stimulus unit is excitatory, as the geometry holds.
    fn enumerate(&mut self, exec: &Engine) {
        let [a, b, r0, r1] = self.sets;
        let blocks = exec.blocks();
        for unit in exec.units() {
            let id = unit.id as u32;
            if unit.last_soma_spike_tick != NO_SPIKE_ON_RECORD {
                if let Some(list) = self.spikes.get_mut(id as usize) {
                    list.push(unit.last_soma_spike_tick);
                }
            }
            let stimulus = if a.contains(id) {
                0
            } else if b.contains(id) {
                1
            } else {
                continue;
            };
            assert_eq!(unit.flags & FLAG_INHIBITORY, 0, "stimulus unit {id}");
            for s in unit.fan_out(exec.blocks()) {
                let readout = if r0.contains(s.target) {
                    0
                } else if r1.contains(s.target) {
                    1
                } else {
                    continue;
                };
                let block = blocks
                    .get(s.block_idx as usize)
                    .expect("a block of the arena");
                self.synapses.push(Replayed {
                    block_idx: s.block_idx as usize,
                    slot: usize::from(s.slot),
                    source: id,
                    target: s.target,
                    stimulus,
                    readout,
                    delay: u32::from(s.delay_ticks),
                    magnitude: i32::from(s.weight_q1_15).max(0),
                    stamp: block.last_spike_tick,
                    trace: block.eligibility_q1_15[usize::from(s.slot)],
                });
            }
        }
        assert!(!self.synapses.is_empty());
    }

    /// The synapses from each stimulus set onto each readout set, `[stimulus][readout]`.
    fn counts(&self) -> [[u32; 2]; 2] {
        let mut counts = [[0u32; 2]; 2];
        for syn in &self.synapses {
            let into = &mut counts[syn.stimulus][syn.readout];
            *into = into.saturating_add(1);
        }
        counts
    }

    /// After a trial: the train since the last reading collected, the presented set's
    /// volleys noted, the oracle advanced over the stimulus units' spikes since the last
    /// reading with every term classed, the record read and held to the oracle, the census
    /// counted.
    fn observe(&mut self, exec: &mut Engine, trial: usize, start: u32, outcome: &Outcome) {
        if self.synapses.is_empty() {
            self.enumerate(exec);
        }
        let end = start.wrapping_add(TRIAL_TICKS);
        let cursor = self.cursor;
        // The train since the last reading. The ring holds a trial's most spikes, so what
        // it let go is older than the reading before this one; asserted.
        let overwritten = exec.train_overwritten();
        let train = exec.train();
        assert!(
            overwritten == 0 || train.first().is_some_and(|&(t, _)| t <= cursor),
            "trial {trial}: the train held every spike since the last reading"
        );
        for &(tick, unit) in train {
            if tick < cursor {
                continue;
            }
            if tick >= end {
                break;
            }
            if let Some(list) = self.spikes.get_mut(unit as usize) {
                list.push(tick);
            }
        }
        // The presented set's volleys: each unit's first spike before the readout window
        // opens; a unit the background fired within its refractory window before the
        // injection has none this trial.
        let presented = self.sets[usize::from(outcome.stimulus)];
        for s in presented.units() {
            let Some(list) = self.spikes.get(s as usize) else {
                continue;
            };
            let at = list.partition_point(|&t| t < start);
            if let Some(&v) = list.get(at) {
                if v < start.wrapping_add(WINDOW.from) {
                    if let Some(volleys) = self.volleys.get_mut(s as usize) {
                        volleys.push(v);
                    }
                    if let Some(count) = self.volley_ticks.get_mut(v.wrapping_sub(start) as usize) {
                        *count = count.saturating_add(1);
                    }
                }
            }
        }
        // The oracle over the stimulus units' spikes since the last reading, synapse by
        // synapse: the block's decay since its stamp, then the rule's two terms for the
        // target's last spike, each classed by that spike, then the stamp; then, under the
        // taught delivery (brief 036), the consolidation at that spike under the signal the
        // executor published at its tick where the synapse is addressed — its source in the
        // stimulus the last trial presented and its target in that stimulus's assigned
        // readout — and under nothing otherwise, the baseline being zero.
        let mut terms = [[[0i64; 4]; 2]; 2];
        let mut transferred = [[0i64; 2]; 2];
        let course = self.taught.as_ref().map(|t| signal_course(t.signal));
        let addressed = self.taught.as_ref().and_then(|t| t.addressed);
        let Self {
            synapses,
            spikes,
            volleys,
            ..
        } = self;
        for syn in synapses.iter_mut() {
            let (Some(pre), Some(post), Some(volleys)) = (
                spikes.get(syn.source as usize),
                spikes.get(syn.target as usize),
                volleys.get(syn.source as usize),
            ) else {
                panic!("a synapse's end is outside the arena");
            };
            let from = pre.partition_point(|&t| t < cursor);
            for &t in pre.iter().skip(from) {
                let elapsed = t.wrapping_sub(syn.stamp);
                if elapsed != 0 {
                    let factor = i64::from(stp_decay_factor_q16(elapsed, ELIGIBILITY_TAU_SHIFT));
                    syn.trace = decayed(syn.trace, factor);
                }
                let at = post.partition_point(|&q| q <= t);
                if let Some(&q) = at.checked_sub(1).and_then(|k| post.get(k)) {
                    let since_post = t.wrapping_sub(q) as i32;
                    let class = if volleyed(volleys, syn.delay, q) {
                        0
                    } else {
                        2
                    };
                    let into = &mut terms[syn.stimulus][syn.readout];
                    if syn.stamp != NO_SPIKE_ON_RECORD {
                        let post_after_prev = q.wrapping_sub(syn.stamp) as i32;
                        if post_after_prev > 0 && since_post >= 0 {
                            let pot = pair_window(STDP_A_PLUS_Q1_15, post_after_prev as u32);
                            syn.trace = syn.trace.saturating_add(pot);
                            into[class] = into[class].saturating_add(i64::from(pot));
                        }
                    }
                    if since_post > 0 {
                        let dep = depression_at(
                            pair_window(STDP_A_MINUS_Q1_15, since_post as u32),
                            syn.magnitude,
                        );
                        syn.trace = syn.trace.saturating_sub(dep);
                        let k = class.wrapping_add(1);
                        into[k] = into[k].saturating_sub(i64::from(dep));
                    }
                }
                syn.stamp = t;
                if let Some(course) = &course {
                    // An addressed synapse's spike is inside the trial: the addressed pair is
                    // written at a trial's end, so the lead-in's spikes before the first
                    // trial, the only ones before `start` the oracle replays, are never
                    // addressed.
                    let modulation = if addressed == Some((syn.stimulus, syn.readout)) {
                        course
                            .get(t.wrapping_sub(start) as usize)
                            .copied()
                            .expect("an addressed synapse's spike is inside the trial")
                    } else {
                        0
                    };
                    let (trace, magnitude, absorbed) =
                        consolidated(syn.trace, syn.magnitude, modulation);
                    syn.trace = trace;
                    syn.magnitude = magnitude;
                    let into = &mut transferred[syn.stimulus][syn.readout];
                    *into = into.saturating_add(i64::from(absorbed));
                }
            }
        }
        // The record: (a), each slot's trace held to the oracle and each weight to the
        // oracle's magnitude — the prior's in a frozen run, the consolidated one under the
        // taught delivery.
        let blocks = exec.blocks();
        let mut sums = [[0i64; 2]; 2];
        for syn in synapses.iter() {
            let block = blocks.get(syn.block_idx).expect("a block of the arena");
            let e = block.eligibility_q1_15[syn.slot];
            assert_eq!(
                e, syn.trace,
                "trial {trial}: the record's trace of {}→{} is the oracle's",
                syn.source, syn.target
            );
            assert_eq!(
                i32::from(block.weights_q1_15[syn.slot]),
                syn.magnitude,
                "trial {trial}: the record's weight of {}→{} is the oracle's",
                syn.source,
                syn.target
            );
            let into = &mut sums[syn.stimulus][syn.readout];
            *into = into.saturating_add(i64::from(e));
        }
        // The signal at the trial's end, held to its course (brief 036), plus the reward the
        // task itself delivered before this reading (brief 037; zero otherwise); at rest in
        // a frozen run.
        let end_signal = course.as_ref().map_or(0, |c| signal_end(c));
        assert_eq!(
            exec.modulator().dopamine_rpe,
            end_signal.saturating_add(self.rewarded),
            "trial {trial}: the record's signal at the trial's end is the course's"
        );
        self.transferred.push(transferred);
        // The census around the readout window's opening: the readouts and the presented
        // set, counted as a readout counts.
        let arrival = start.wrapping_add(WINDOW.from);
        let opening = arrival.wrapping_sub(PAIR_WINDOW);
        let before = self
            .readout
            .count_window(exec.train(), opening, PAIR_WINDOW);
        let after = self
            .readout
            .count_window(exec.train(), arrival, PAIR_WINDOW);
        let s = usize::from(outcome.stimulus);
        let own_before = self
            .stimuli
            .count_window(exec.train(), opening, PAIR_WINDOW)[s];
        let own_after = self
            .stimuli
            .count_window(exec.train(), arrival, PAIR_WINDOW)[s];
        self.out.push((
            outcome.stimulus,
            sums,
            terms,
            [
                [before[0], after[0]],
                [before[1], after[1]],
                [own_before, own_after],
            ],
        ));
        self.cursor = end;
    }
}

// ----------------------------------------------------------------------- the readings

/// A trial's pinned row.
fn trial_row(t: &Composed) -> Row {
    let s = usize::from(t.0);
    let sum = t.1[s][0].saturating_add(t.1[s][1]);
    let mut terms = [0i64; 4];
    for readout in &t.2[s] {
        for (k, term) in readout.iter().enumerate() {
            terms[k] = terms[k].saturating_add(*term);
        }
    }
    (t.0, sum, terms)
}

/// The sign measure of a trial: the presented stimulus's summed eligibility onto both
/// readouts, read from the record after the trial, is positive.
fn positive(t: &Composed) -> bool {
    trial_row(t).1 > 0
}

/// The FNV-1a hash of every trial's whole reading, each number as its `i32` words.
fn hash_of(trials: &[Composed]) -> u64 {
    let mut words: Vec<i32> = Vec::new();
    let mut wide = |x: i64| {
        words.push(x as i32);
        words.push((x >> 32) as i32);
    };
    for t in trials {
        wide(i64::from(t.0));
        for readouts in &t.1 {
            for &sum in readouts {
                wide(sum);
            }
        }
        for readouts in &t.2 {
            for classes in readouts {
                for &term in classes {
                    wide(term);
                }
            }
        }
        for readout in &t.3 {
            for &count in readout {
                wide(i64::from(count));
            }
        }
    }
    fnv1a_64(&words)
}

/// A block's composition from its trials' readings.
fn composition(trials: &[Composed]) -> Composition {
    let sums = trials.last().map_or([[0; 2]; 2], |t| t.1);
    let mut terms = [[[0i64; 4]; 2]; 2];
    let mut census = [[0u64; 2]; 3];
    let mut signs = 0u32;
    for t in trials {
        for (s, readouts) in t.2.iter().enumerate() {
            for (r, classes) in readouts.iter().enumerate() {
                for (k, term) in classes.iter().enumerate() {
                    terms[s][r][k] = terms[s][r][k].saturating_add(*term);
                }
            }
        }
        for (r, counts) in t.3.iter().enumerate() {
            for (k, count) in counts.iter().enumerate() {
                census[r][k] = census[r][k].saturating_add(u64::from(*count));
            }
        }
        signs = signs.saturating_add(u32::from(positive(t)));
    }
    (sums, terms, census, signs, hash_of(trials))
}

/// A rung passes when both measures pass: the sight (`calibrated`, ADR-0065's) and the
/// sign, at least `SIGN_MIN` trials of the block positive.
fn passes(block: &Block, composition: &Composition) -> bool {
    calibrated(block) && composition.3 >= SIGN_MIN
}

/// The gain the ladder picks: the first rung, in the ladder's order, that passes both
/// measures; none when no rung does.
fn ladder_pick(rungs: &[(Block, u64, Composition)]) -> Option<u32> {
    LADDER
        .iter()
        .zip(rungs.iter())
        .find(|(_, (block, _, composition))| passes(block, composition))
        .map(|(&gain, _)| gain)
}

/// One frozen run of `trials` trials at `gain`, read through the composer: the calibration's
/// run (the modulation baseline at zero, no reward, two workers), so that at the present
/// gain over a block it is `CALIBRATION_1024[0]`'s run bit for bit, with the composition
/// read after every trial. The sums by polarity are asserted unchanged after a whole block,
/// as the calibration asserts them.
fn compose(units: u32, gain: u32, trials: usize) -> (Vec<Block>, u64, Vec<Composed>) {
    let (blocks, trace, trials, _) = compose_counting(units, gain, trials);
    (blocks, trace, trials)
}

/// `compose`, with the count of synapses from each stimulus set onto each readout set.
fn compose_counting(
    units: u32,
    gain: u32,
    trials: usize,
) -> (Vec<Block>, u64, Vec<Composed>, [[u32; 2]; 2]) {
    let (blocks, trace, trials, counts, _, _) =
        compose_shaped(SHAPE_F46, None, units, gain, trials);
    (blocks, trace, trials, counts)
}

/// A trial's readout counts as the task read them (brief 033): the stimulus presented and
/// the spikes of each readout set in the readout window.
type Counted = (u8, [u32; 2]);
/// A frozen run read under a shape: the sight's blocks and trace, the composed trials, the
/// synapses from each stimulus set onto each readout set, every trial's counts, and the
/// presented set's volley spikes by their tick after the trial's first (brief 034).
type Shaped = (
    Vec<Block>,
    u64,
    Vec<Composed>,
    [[u32; 2]; 2],
    Vec<Counted>,
    Vec<u64>,
);

/// `compose_counting` with the stimulus of `shape` (brief 033) and its cancel (brief 034),
/// and every trial's readout counts beside the composition; under `SHAPE_F46` with no cancel
/// it is `compose_counting`.
fn compose_shaped(
    shape: Shape,
    cancel: Option<Cancel>,
    units: u32,
    gain: u32,
    trials: usize,
) -> Shaped {
    let p = prior(units);
    let mut exec = at_gain(&p, config(units, 2, 0), gain);
    assert_eq!(exec.homeostasis().synaptic_gain_q16, gain);
    compose_on(&mut exec, shape, cancel, units, trials)
}

/// `compose_shaped` on an executor the caller made (brief 035): the modulation baseline
/// asserted zero, the sums by polarity read before the run and asserted unchanged after every
/// block, the composer's oracle seeded from the record before the first trial (a settled
/// network carries a stamp and a trace on every block and a last spike on every unit; a fresh
/// one none, so `compose_shaped` reads as it read), and the calibration's task run by
/// `run_on`.
fn compose_on(
    exec: &mut Engine,
    shape: Shape,
    cancel: Option<Cancel>,
    units: u32,
    trials: usize,
) -> Shaped {
    assert_eq!(exec.modulation_baseline_q16(), 0, "the weights are frozen");
    let sums = weights_by_polarity(exec);
    let mut composer = Composer::new(units);
    composer.enumerate(exec);
    // The oracle replays the train from the executor's clock: what the record already
    // holds — the seeded stamps, traces and last spikes — is where it starts, not what it
    // replays (a seeded last spike of a stimulus unit replayed as a presynaptic spike would
    // enter the record's last pairing twice). On a fresh engine the clock is at zero.
    composer.cursor = exec.ticks() as u32;
    let mut counted: Vec<Counted> = Vec::with_capacity(trials);
    let task = task(
        shape,
        cancel,
        units,
        Feedback::Withheld,
        false,
        Delivery::Global,
    );
    let (blocks, trace) = run_on(
        exec,
        task,
        units,
        trials,
        &mut |exec, trial, start, outcome| {
            composer.observe(exec, trial, start, outcome);
            assert_eq!(
                outcome.selection,
                selected(outcome.counts),
                "trial {trial}: the selection is the sign of the count difference"
            );
            counted.push((outcome.stimulus, outcome.counts));
        },
    );
    for block in &blocks {
        assert_eq!(
            (block.7, block.8),
            sums,
            "no weight moves at a modulation of zero"
        );
    }
    assert_eq!(composer.out.len(), trials);
    let counts = composer.counts();
    (
        blocks,
        trace,
        composer.out,
        counts,
        counted,
        composer.volley_ticks,
    )
}

/// Dumps a composed run: the sight's blocks and trace, the rows and the composition.
fn dump_composition(name: &str, blocks: &[Block], trace: u64, trials: &[Composed]) {
    let rows: Vec<Row> = trials.iter().map(trial_row).collect();
    eprintln!("DUMP {name} sight {blocks:?} trace {trace:#018x}");
    eprintln!("DUMP {name} rows {rows:?}");
    eprintln!("DUMP {name} composition {:?}", composition(trials));
}

/// Holds a composition's rows and, where a whole block was run, its block to their pinned
/// tables.
fn pinned_composition(name: &str, trials: &[Composed], rows: &[Row], block: Option<&Composition>) {
    let read: Vec<Row> = trials.iter().map(trial_row).collect();
    assert_eq!(read, rows, "{name}: the rows");
    if let Some(block) = block {
        assert_eq!(&composition(trials), block, "{name}: the block");
    }
}

// ------------------------------------------------------------ the measurement (ADR-0072)

/// The composition and the ladder at 1 024 units, rung by rung in the ladder's order, each
/// a frozen run of sixty-four trials: the sight's block and trace (at the present gain,
/// `CALIBRATION_1024[0]`'s), and the composition. Pinned from one run each.
const LADDER_1024: [(Block, u64, Composition); 4] = [
    (
        (
            27,
            34,
            [[396, 378], [379, 363]],
            [1727, 1526],
            [6171, 5504],
            [255, 241],
            62,
            213902976,
            235822619,
            0,
            [[6986739, 7212710], [7146878, 7257384]],
            8,
        ),
        0xa84553d901278f4f,
        (
            [[-51652, -55823], [-42894, -46658]],
            [
                [
                    [453094, -273496, 372055, -1330229],
                    [461454, -231820, 401769, -1314171],
                ],
                [
                    [433249, -239333, 364647, -1196198],
                    [415580, -234738, 381002, -1226100],
                ],
            ],
            [[1038, 2265], [1035, 2270], [3358, 6737]],
            1,
            11256068980148820643,
        ),
    ),
    (
        (
            25,
            34,
            [[63, 56], [48, 52]],
            [1734, 1529],
            [4463, 3966],
            [15, 22],
            49,
            213902976,
            235822619,
            0,
            [[6986739, 7212710], [7146878, 7257384]],
            17,
        ),
        0xd1a3a314d585fdcb,
        (
            [[7354, 5844], [991, 1416]],
            [
                [
                    [72257, -21448, 18293, -57382],
                    [82430, -17452, 21756, -62247],
                ],
                [
                    [65958, -16157, 21609, -47132],
                    [69522, -25426, 26529, -66552],
                ],
            ],
            [[57, 218], [73, 223], [3273, 5045]],
            45,
            2876531214492687737,
        ),
    ),
    (
        (
            4,
            34,
            [[2, 4], [2, 3]],
            [1734, 1530],
            [3487, 3078],
            [1, 1],
            10,
            213902976,
            235822619,
            0,
            [[6986739, 7212710], [7146878, 7257384]],
            54,
        ),
        0xf4bd9f85758e39e5,
        (
            [[88, 1709], [-173, 0]],
            [
                [[1770, 0, 352, -240], [5303, 0, 1574, -3447]],
                [[1854, 0, 387, -1426], [3449, 0, 1468, -1509]],
            ],
            [[1, 4], [4, 10], [3264, 3300]],
            32,
            13527546449650777006,
        ),
    ),
    (
        (
            0,
            34,
            [[0, 0], [0, 0]],
            [1734, 1530],
            [2527, 2251],
            [0, 0],
            0,
            213902976,
            235822619,
            0,
            [[6986739, 7212710], [7146878, 7257384]],
            64,
        ),
        0x420a6b796a3a6725,
        (
            [[0, 0], [0, 0]],
            [[[0, 0, 0, 0], [0, 0, 0, 0]], [[0, 0, 0, 0], [0, 0, 0, 0]]],
            [[0, 0], [0, 0], [3264, 1514]],
            0,
            1431228405596711687,
        ),
    ),
];
/// The rows of every rung's sixty-four trials, in the ladder's order.
const LADDER_ROWS_1024: [[Row; BLOCK]; 4] = [
    [
        (0, 4351, [39500, -27437, 11767, -19732]),
        (0, -15516, [38260, -7400, 19473, -71119]),
        (1, -88961, [33439, -41843, 27108, -92702]),
        (0, -52656, [23321, -14896, 23194, -74132]),
        (0, -54695, [25052, -3478, 20577, -56279]),
        (0, -85952, [19395, -7961, 13421, -69410]),
        (0, -99666, [14580, -5212, 13206, -55800]),
        (0, -96614, [18256, -6071, 14928, -48693]),
        (0, -110919, [17971, -11849, 18427, -58165]),
        (0, -140667, [15004, -9144, 24106, -87411]),
        (1, -44149, [49532, -38665, 27111, -58618]),
        (1, -32438, [34244, -9136, 22037, -47435]),
        (0, -130937, [30841, -28673, 29700, -86403]),
        (0, -118955, [40124, -13949, 25172, -67092]),
        (1, -71524, [24891, -22383, 23803, -74969]),
        (0, -108754, [23135, -21497, 20767, -58341]),
        (1, -72153, [22889, -14786, 20027, -59292]),
        (0, -124145, [20755, -7924, 21387, -91940]),
        (0, -145336, [29612, -6560, 16774, -87912]),
        (0, -175808, [17222, -7982, 14883, -86923]),
        (1, -68593, [22659, -24894, 27624, -59189]),
        (0, -153711, [22209, -13532, 16795, -72094]),
        (1, -75206, [41016, -20391, 21560, -83711]),
        (1, -85629, [25851, -10566, 21384, -63575]),
        (1, -130549, [19720, -15426, 20149, -89478]),
        (0, -104593, [21604, -19623, 18608, -61928]),
        (1, -133222, [17934, -16143, 18076, -76288]),
        (1, -140469, [26468, -7051, 13245, -69780]),
        (1, -157654, [8799, -6243, 15362, -66741]),
        (0, -99453, [33339, -22427, 20578, -85729]),
        (0, -109293, [24563, -7415, 20110, -70334]),
        (0, -128654, [27249, -20342, 16562, -67343]),
        (0, -136254, [21014, -13555, 12936, -59530]),
        (0, -151754, [16014, -5581, 13485, -73383]),
        (1, -73945, [36302, -35583, 18756, -54837]),
        (1, -85516, [39430, -10148, 19286, -79086]),
        (1, -98720, [18841, -3718, 13412, -60167]),
        (0, -113908, [30167, -34057, 23889, -67695]),
        (1, -107873, [25423, -17176, 29961, -84463]),
        (1, -120074, [24040, -6977, 20537, -73873]),
        (0, -88213, [23899, -21466, 20667, -54839]),
        (1, -113369, [18896, -12367, 15187, -54332]),
        (0, -79959, [28922, -15956, 26612, -68976]),
        (1, -108306, [26201, -20299, 20537, -65219]),
        (0, -80853, [21452, -12803, 18927, -59854]),
        (1, -99837, [30574, -21208, 16234, -63947]),
        (0, -104501, [20796, -12145, 20667, -70120]),
        (0, -109353, [27229, -16497, 19386, -56995]),
        (0, -113572, [18445, -7885, 14379, -53999]),
        (1, -86353, [36676, -29568, 30257, -83035]),
        (1, -105370, [23500, -11405, 21915, -72470]),
        (0, -112699, [26420, -24359, 24130, -72766]),
        (1, -110437, [20577, -9843, 25166, -84533]),
        (1, -115894, [17875, -5522, 14712, -55546]),
        (0, -104534, [29291, -27705, 21471, -77413]),
        (1, -133071, [14379, -11138, 13785, -77839]),
        (1, -152763, [30120, -10947, 16608, -85417]),
        (1, -167995, [17365, -4765, 14076, -78093]),
        (0, -89200, [30425, -22783, 22368, -75092]),
        (1, -139386, [17483, -11394, 18783, -65170]),
        (0, -101184, [30678, -17096, 21999, -81083]),
        (0, -117893, [19497, -10056, 16616, -67623]),
        (1, -91000, [25961, -14107, 21364, -58421]),
        (1, -89552, [21619, -10378, 16549, -47788]),
    ],
    [
        (0, 1042, [2581, -1557, 295, -277]),
        (0, 7859, [8098, -482, 2177, -2930]),
        (1, -3563, [1869, -1685, 937, -4718]),
        (0, -4571, [2566, -743, 443, -11660]),
        (0, -1523, [2151, -3, 742, -736]),
        (0, -3000, [0, 0, 107, -1874]),
        (0, -1594, [3872, -1163, 1584, -3467]),
        (0, -1550, [0, 0, 303, -554]),
        (0, -740, [3979, -1029, 702, -3120]),
        (0, -3095, [2037, -2, 1967, -6425]),
        (1, 5803, [12132, -4415, 2076, -3761]),
        (1, 9089, [4614, 0, 1296, -1410]),
        (0, -4841, [3971, -4975, 1527, -3894]),
        (0, 1471, [5782, 0, 1467, -1984]),
        (1, 1277, [5187, -7036, 1480, -2605]),
        (0, 110, [1820, -996, 1142, -2798]),
        (1, 8505, [7395, -1741, 4123, -2007]),
        (0, -7592, [1097, 0, 371, -9188]),
        (0, 1979, [7395, -3, 2322, -1793]),
        (0, 1800, [3632, -386, 1500, -4484]),
        (1, 8636, [4996, -30, 1976, -1193]),
        (0, 3815, [2860, 0, 768, -776]),
        (1, 10504, [9782, -4443, 2952, -3672]),
        (1, 11948, [5172, -781, 2630, -3156]),
        (1, 9364, [1230, -1197, 324, -195]),
        (0, 659, [3298, -1057, 1203, -4071]),
        (1, -2064, [2205, -1836, 2574, -10598]),
        (1, 1790, [2648, 0, 1281, -347]),
        (1, 496, [1773, -1301, 1583, -2845]),
        (0, 6274, [10485, -2527, 1266, -3230]),
        (0, 3599, [9263, -402, 1558, -11482]),
        (0, 1818, [5167, -3782, 1083, -3441]),
        (0, 920, [4521, 0, 501, -5456]),
        (0, 3753, [2909, 0, 1650, -1411]),
        (1, 1715, [2754, -816, 1271, -1535]),
        (1, 3848, [7990, -1430, 2154, -6128]),
        (1, 4964, [3528, 0, 1482, -2906]),
        (0, 2552, [3957, -768, 1098, -2711]),
        (1, -2710, [2428, 0, 1118, -9212]),
        (1, -6988, [1840, -1304, 1955, -7158]),
        (0, 3645, [5460, -1057, 1205, -3545]),
        (1, -6168, [2937, 0, 934, -5727]),
        (0, 7845, [7759, -1941, 930, -1000]),
        (1, -3718, [2185, -1750, 1686, -2094]),
        (0, 2381, [4317, -3933, 1326, -3878]),
        (1, 3299, [8438, -1323, 1890, -3363]),
        (0, 613, [3222, -1839, 1661, -3680]),
        (0, 3985, [6792, -1291, 1314, -3183]),
        (0, 3121, [3786, -552, 877, -4009]),
        (1, 2632, [9649, -3956, 2007, -6111]),
        (1, 5313, [3642, -739, 1736, -1334]),
        (0, -2165, [4089, -3318, 818, -5173]),
        (1, 4233, [1638, -14, 1182, -1668]),
        (1, 5435, [3515, -75, 751, -2126]),
        (0, 2381, [4092, 0, 714, -1502]),
        (1, -1400, [1308, 0, 1297, -7221]),
        (1, 1687, [6271, -613, 920, -3794]),
        (1, -893, [2503, -737, 187, -4047]),
        (0, 5603, [3656, 0, 1809, -552]),
        (1, -2616, [2889, -2836, 1350, -3426]),
        (0, 13544, [12577, -2858, 1211, -652]),
        (0, 13104, [6573, -2236, 1929, -3371]),
        (1, 1372, [6757, -1525, 1887, -4582]),
        (1, 2407, [4955, 0, 1034, -4435]),
    ],
    [
        (0, 0, [0, 0, 0, 0]),
        (0, 0, [0, 0, 0, 0]),
        (1, 0, [0, 0, 0, 0]),
        (0, 1122, [812, 0, 314, 0]),
        (0, 872, [0, 0, 0, 0]),
        (0, 678, [0, 0, 0, 0]),
        (0, -897, [0, 0, 0, -1427]),
        (0, -698, [0, 0, 0, 0]),
        (0, -600, [0, 0, 1, -57]),
        (0, -468, [0, 0, 0, 0]),
        (1, -1, [0, 0, 0, -2]),
        (1, 0, [0, 0, 0, 0]),
        (0, -203, [0, 0, 70, -51]),
        (0, 707, [867, 0, 0, 0]),
        (1, 0, [0, 0, 0, 0]),
        (0, 428, [0, 0, 0, 0]),
        (1, 0, [0, 0, 0, 0]),
        (0, -801, [590, 0, 296, -1945]),
        (0, -629, [0, 0, 0, 0]),
        (0, -492, [0, 0, 0, 0]),
        (1, 0, [0, 0, 0, 0]),
        (0, -302, [0, 0, 0, 0]),
        (1, 2446, [2284, 0, 571, -402]),
        (1, 1902, [0, 0, 0, 0]),
        (1, 1478, [0, 0, 0, 0]),
        (0, -110, [0, 0, 3, 0]),
        (1, 844, [0, 0, 299, -344]),
        (1, 1817, [897, 0, 309, -40]),
        (1, 1411, [0, 0, 0, 0]),
        (0, -42, [0, 0, 0, 0]),
        (0, -37, [0, 0, 0, 0]),
        (0, -198, [0, 0, 2, -169]),
        (0, -99, [0, 0, 56, -2]),
        (0, 967, [786, 0, 262, 0]),
        (1, 846, [268, 0, 268, 0]),
        (1, 1417, [873, 0, 2, -106]),
        (1, 1090, [0, 0, 0, 0]),
        (0, 354, [0, 0, 0, 0]),
        (1, 655, [0, 0, 0, 0]),
        (1, -302, [0, 0, 0, -794]),
        (0, 148, [0, 0, 0, -22]),
        (1, -176, [0, 0, 21, 0]),
        (0, 90, [0, 0, 0, 0]),
        (1, -114, [0, 0, 0, 0]),
        (0, 55, [0, 0, 0, 0]),
        (1, -79, [0, 0, 0, -3]),
        (0, 2096, [1734, 0, 339, 0]),
        (0, 1624, [0, 0, 0, 0]),
        (0, 1256, [0, 0, 0, 0]),
        (1, -32, [0, 0, 0, 0]),
        (1, 1274, [981, 0, 327, 0]),
        (0, 1741, [1180, 0, 0, -14]),
        (1, 807, [0, 0, 44, 0]),
        (1, 624, [0, 0, 0, 0]),
        (0, 812, [0, 0, 0, 0]),
        (1, 375, [0, 0, 0, 0]),
        (1, -941, [0, 0, 14, -1244]),
        (1, -734, [0, 0, 0, 0]),
        (0, 291, [0, 0, 0, 0]),
        (1, -447, [0, 0, 0, 0]),
        (0, 165, [0, 0, 0, 0]),
        (0, 1797, [1104, 0, 583, 0]),
        (1, -214, [0, 0, 0, 0]),
        (1, -173, [0, 0, 0, 0]),
    ],
    [
        (0, 0, [0, 0, 0, 0]),
        (0, 0, [0, 0, 0, 0]),
        (1, 0, [0, 0, 0, 0]),
        (0, 0, [0, 0, 0, 0]),
        (0, 0, [0, 0, 0, 0]),
        (0, 0, [0, 0, 0, 0]),
        (0, 0, [0, 0, 0, 0]),
        (0, 0, [0, 0, 0, 0]),
        (0, 0, [0, 0, 0, 0]),
        (0, 0, [0, 0, 0, 0]),
        (1, 0, [0, 0, 0, 0]),
        (1, 0, [0, 0, 0, 0]),
        (0, 0, [0, 0, 0, 0]),
        (0, 0, [0, 0, 0, 0]),
        (1, 0, [0, 0, 0, 0]),
        (0, 0, [0, 0, 0, 0]),
        (1, 0, [0, 0, 0, 0]),
        (0, 0, [0, 0, 0, 0]),
        (0, 0, [0, 0, 0, 0]),
        (0, 0, [0, 0, 0, 0]),
        (1, 0, [0, 0, 0, 0]),
        (0, 0, [0, 0, 0, 0]),
        (1, 0, [0, 0, 0, 0]),
        (1, 0, [0, 0, 0, 0]),
        (1, 0, [0, 0, 0, 0]),
        (0, 0, [0, 0, 0, 0]),
        (1, 0, [0, 0, 0, 0]),
        (1, 0, [0, 0, 0, 0]),
        (1, 0, [0, 0, 0, 0]),
        (0, 0, [0, 0, 0, 0]),
        (0, 0, [0, 0, 0, 0]),
        (0, 0, [0, 0, 0, 0]),
        (0, 0, [0, 0, 0, 0]),
        (0, 0, [0, 0, 0, 0]),
        (1, 0, [0, 0, 0, 0]),
        (1, 0, [0, 0, 0, 0]),
        (1, 0, [0, 0, 0, 0]),
        (0, 0, [0, 0, 0, 0]),
        (1, 0, [0, 0, 0, 0]),
        (1, 0, [0, 0, 0, 0]),
        (0, 0, [0, 0, 0, 0]),
        (1, 0, [0, 0, 0, 0]),
        (0, 0, [0, 0, 0, 0]),
        (1, 0, [0, 0, 0, 0]),
        (0, 0, [0, 0, 0, 0]),
        (1, 0, [0, 0, 0, 0]),
        (0, 0, [0, 0, 0, 0]),
        (0, 0, [0, 0, 0, 0]),
        (0, 0, [0, 0, 0, 0]),
        (1, 0, [0, 0, 0, 0]),
        (1, 0, [0, 0, 0, 0]),
        (0, 0, [0, 0, 0, 0]),
        (1, 0, [0, 0, 0, 0]),
        (1, 0, [0, 0, 0, 0]),
        (0, 0, [0, 0, 0, 0]),
        (1, 0, [0, 0, 0, 0]),
        (1, 0, [0, 0, 0, 0]),
        (1, 0, [0, 0, 0, 0]),
        (0, 0, [0, 0, 0, 0]),
        (1, 0, [0, 0, 0, 0]),
        (0, 0, [0, 0, 0, 0]),
        (0, 0, [0, 0, 0, 0]),
        (1, 0, [0, 0, 0, 0]),
        (1, 0, [0, 0, 0, 0]),
    ],
];
/// The gain the ladder picked, as `ladder_pick` reads the pinned tables; none when no rung
/// passed both measures.
const GAIN_PICKED_1024: Option<u32> = None;
/// The synapses from each stimulus set onto each readout set at 1 024 units,
/// `[stimulus][readout]`, as the composer walks them: the census behind the couplings.
const SYNAPSES_1024: [[u32; 2]; 2] = [[775, 806], [798, 809]];

/// The composition at the present gain: rung 1 of the ladder, `CALIBRATION_1024[0]`'s run
/// with the trace read after every trial.
#[test]
#[ignore]
fn the_composition_at_1024_units_exhaustive() {
    let (blocks, trace, trials, counts, counted, volley_ticks) =
        compose_shaped(SHAPE_F46, None, 1024, LADDER[0], BLOCK);
    dump_composition("composition1024", &blocks, trace, &trials);
    eprintln!(
        "DUMP composition1024 volley ticks {:?}",
        census_of(&volley_ticks)
    );
    assert_eq!(
        census_of(&volley_ticks),
        VOLLEY_TICKS_1024.to_vec(),
        "composition1024: the volley's ticks"
    );
    dump_requires("composition1024", &counted, false, OFFSET_MARK_64);
    let (block, pin, composed) = &LADDER_1024[0];
    pinned("composition1024 sight", &blocks, trace, &[*block], *pin);
    pinned_composition(
        "composition1024",
        &trials,
        &LADDER_ROWS_1024[0],
        Some(composed),
    );
    assert_eq!(counts, SYNAPSES_1024);
    assert_eq!(
        counted.as_slice(),
        COUNTED_1024,
        "composition1024: the counts"
    );
}

/// The ladder below the present gain: rungs 2 to 4, each a frozen run read by both
/// measures; every rung is run and dumped before any is held to its table, so that one
/// rung's failure still shows the others' readings.
#[test]
#[ignore]
fn the_ladder_below_the_present_gain_at_1024_units_exhaustive() {
    let runs: Vec<(Vec<Block>, u64, Vec<Composed>)> = LADDER
        .iter()
        .skip(1)
        .map(|&gain| compose(1024, gain, BLOCK))
        .collect();
    for (k, (blocks, trace, trials)) in runs.iter().enumerate() {
        let gain = LADDER[k.wrapping_add(1)];
        dump_composition(&format!("ladder1024 {gain:#x}"), blocks, *trace, trials);
    }
    for (k, (blocks, trace, trials)) in runs.iter().enumerate() {
        let rung = k.wrapping_add(1);
        let gain = LADDER[rung];
        let (block, pin, composed) = &LADDER_1024[rung];
        let name = format!("ladder1024 {gain:#x}");
        pinned(&format!("{name} sight"), blocks, *trace, &[*block], *pin);
        pinned_composition(&name, trials, &LADDER_ROWS_1024[rung], Some(composed));
    }
}

/// The gate's test (ADR-0061's class): the oracle's arithmetic and the class rule at their
/// edges, the measures and the pick at theirs, the pick over the pinned ladder as written,
/// and the first eight trials of the composition at the present gain, run and held to the
/// first eight rows of the pinned table; no number is pinned twice.
#[test]
fn the_first_eight_trials_of_the_composition_at_1024_units_and_the_rules_over_its_tables() {
    // The window: the amplitude at no distance, $e^{-1}$ of it at one time constant, and
    // nothing far away; the depression at the reference magnitude is the window's amount,
    // four times it at the rail and nothing below a magnitude of twelve (ADR-0055); a
    // decayed trace loses at least one LSB and reaches zero exactly.
    assert_eq!(pair_window(STDP_A_PLUS_Q1_15, 0), STDP_A_PLUS_Q1_15);
    assert_eq!(pair_window(STDP_A_MINUS_Q1_15, 0), STDP_A_MINUS_Q1_15);
    assert_eq!(pair_window(STDP_A_PLUS_Q1_15, PAIR_WINDOW), 120);
    assert_eq!(pair_window(STDP_A_MINUS_Q1_15, PAIR_WINDOW), 126);
    assert_eq!(pair_window(STDP_A_PLUS_Q1_15, 1 << 16), 0);
    assert_eq!(
        depression_at(STDP_A_MINUS_Q1_15, 0x2000),
        STDP_A_MINUS_Q1_15
    );
    assert_eq!(depression_at(STDP_A_MINUS_Q1_15, i32::from(i16::MAX)), 1376);
    assert_eq!(depression_at(STDP_A_MINUS_Q1_15, 12), 1);
    assert_eq!(depression_at(STDP_A_MINUS_Q1_15, 11), 0);
    assert_eq!(decayed(0, 0xFFFF), 0);
    assert_eq!(decayed(1, 0xFFFF), 0, "one LSB toward zero");
    assert_eq!(decayed(-1, 0xFFFF), 0);
    assert_eq!(decayed(1000, 0xFFFF), 999);
    assert_eq!(decayed(-1000, 0x8000), -500);
    assert_eq!(
        decayed(
            1000,
            i64::from(stp_decay_factor_q16(1000, ELIGIBILITY_TAU_SHIFT))
        ),
        985,
        "as the block's own test reads it"
    );
    // The class rule at its edges: a spike at the arrival is the volley's, one at the
    // window's last tick is, one at the window's end is the background's, as is one before
    // the arrival and one with no volley on record; the latest volley is the one read.
    let volleys = [1000u32, 20_000];
    assert!(volleyed(&volleys, 200, 1200));
    assert!(volleyed(&volleys, 200, 1200 + PAIR_WINDOW - 1));
    assert!(!volleyed(&volleys, 200, 1200 + PAIR_WINDOW));
    assert!(!volleyed(&volleys, 200, 1199));
    assert!(!volleyed(&[], 200, 1200));
    assert!(volleyed(&volleys, 100, 20_100));
    assert!(!volleyed(&volleys, 100, 20_099));
    assert!(!volleyed(&volleys, 100, 20_100 + PAIR_WINDOW));
    // The measures and the pick at their edges over readings written by hand.
    let composed = |stimulus: u8, sum: i64| -> Composed {
        let s = usize::from(stimulus);
        let mut sums = [[0i64; 2]; 2];
        sums[s][0] = sum;
        sums[s][1] = 1;
        (stimulus, sums, [[[0; 4]; 2]; 2], [[0; 2]; 3])
    };
    assert!(positive(&composed(0, 0)), "1 is positive");
    assert!(!positive(&composed(1, -1)), "0 is not");
    assert!(!positive(&composed(1, -2)));
    let trials: Vec<Composed> = (0..64)
        .map(|k| composed(0, if k < 56 { 5 } else { -5 }))
        .collect();
    let block = composition(&trials);
    assert_eq!(block.3, 56);
    assert_eq!(block.0, [[-5, 1], [0, 0]], "the sums are the last trial's");
    let mut sight = CALIBRATION_1024[0].0;
    assert!(calibrated(&sight));
    assert!(passes(&sight, &block), "56 signs and 62 seen pass");
    let mut short = trials.clone();
    short[0] = composed(0, -5);
    let below = composition(&short);
    assert_eq!(below.3, 55);
    assert!(!passes(&sight, &below), "55 signs fail");
    sight.6 = SEEN_MIN.wrapping_sub(1);
    assert!(!passes(&sight, &block), "55 seen fail with the sign passed");
    let blind = (sight, 0, block);
    let seeing = (CALIBRATION_1024[0].0, 0, block);
    let wrong = (CALIBRATION_1024[0].0, 0, below);
    assert_eq!(
        ladder_pick(&[seeing, seeing, seeing, seeing]),
        Some(LADDER[0])
    );
    assert_eq!(
        ladder_pick(&[blind, seeing, wrong, seeing]),
        Some(LADDER[1])
    );
    assert_eq!(ladder_pick(&[blind, wrong, wrong, seeing]), Some(LADDER[3]));
    assert_eq!(ladder_pick(&[blind, wrong, blind, wrong]), None);
    assert_eq!(ladder_pick(&[]), None);
    // The rows and the block agree: a block's signs are its rows' positive sums, and the
    // first rung's sight is the calibration's run.
    for (k, (block, pin, composed)) in LADDER_1024.iter().enumerate() {
        let signs = LADDER_ROWS_1024[k].iter().filter(|r| r.1 > 0).count() as u32;
        assert_eq!(signs, composed.3, "rung {k}: the signs are the rows'");
        eprintln!(
            "DUMP ladder1024 rung {k} gain {:#x} seen {} signs {} passes {}",
            LADDER[k],
            block.6,
            composed.3,
            passes(block, composed)
        );
        if k == 0 {
            assert_eq!(
                (*block, *pin),
                CALIBRATION_1024[0],
                "rung 1 is the calibration"
            );
        }
    }
    assert_eq!(
        ladder_pick(&LADDER_1024),
        GAIN_PICKED_1024,
        "the pick as written"
    );
    // The first eight trials, run; the synapses from each stimulus set onto each readout
    // set are the census's.
    let (blocks, trace, trials, counts) = compose_counting(1024, LADDER[0], GATE_TRIALS);
    dump_composition("composition1024 first eight", &blocks, trace, &trials);
    eprintln!("DUMP composition1024 synapses {counts:?}");
    assert_eq!(counts, SYNAPSES_1024);
    assert!(blocks.is_empty(), "no whole block");
    pinned_composition(
        "composition1024 first eight",
        &trials,
        &LADDER_ROWS_1024[0][..GATE_TRIALS],
        None,
    );
}

// ------------------------------------------ a stimulus that fires once (brief 033)
//
// Written before the run. F-46 (ADR-0072): the stimulus of ADR-0065 fires each of its
// units about three times within one pair window of the volley, once before the readout
// window opens and 2.06 more inside the pair window after it, and the pair rule enters a
// depression at every presynaptic spike. The executor scales every input of a unit by the
// tick's synaptic gain (ADR-0036; `scaled` in `executor.rs`), the injected stimulus
// included, so at the present gain of 1.75 the two messages of 1.25 arrive as 4.375 of
// basal drive, above the 3.0 at which ADR-0038 read a second spike. This round changes the
// stimulus and nothing else. The candidates below, in this order and no other, are each
// read by the measure below over one frozen run of sixty-four trials at the present gain,
// the calibration's run with the shape the one difference; the measure reads no selection,
// no reward and no outcome. The first candidate that passes is the stimulus; if none passes
// there is no rewarded run and the candidates are recorded with their censuses. Under the
// chosen stimulus the composition is read again by ADR-0072's oracle at no extra cost, and
// both calibrations, the sight (ADR-0065's) and the sign (ADR-0072's), are read from the
// same run at the gain 1.75, which does not move.
//
// What the membrane rule says of one message, computed apart from the tree before any run
// (an oracle over `integrate` in `membrane.rs` on a unit at rest at the base threshold with
// no other input; a Hypothesis about the instrument, whose units carry the drive's standing
// potential besides): a basal drive of $D$ after the gain fires the unit once when $D$ is
// between about 1.13 and 1.53, twice from 1.53 (the second spike about 200 ticks after the
// first, at the end of the refractory window, from what the basal compartment, whose time
// constant is 512 ticks, still holds), three times from 2.27 and four from 3.38; so 4.375
// is four spikes at rest, and F-46's three are that under the instrument's inhibition.
// Candidate (a), one message at the unit's threshold, is 1.75 after the gain: two spikes at
// rest, at 13 and 214 ticks. Candidate (b), one message of 1.25, is 2.19: two spikes at
// rest, at 9 and 210. The gate holds the engine's own unit at rest to these on the
// instrument's network at the gain (`the_candidates_rules_over_their_tables_...`).

/// Candidate (a): one message at the unit's threshold, `THRESHOLD_BASE` (1.0), the value
/// the record's threshold gives and no other; 1.75 of basal drive after the gain.
const CANDIDATE_A: Shape = (1, THRESHOLD_BASE);
/// Candidate (b): one message of 1.25, half of F-46's stimulus; 2.19 after the gain.
const CANDIDATE_B: Shape = (1, STIMULUS_Q16);
/// Candidate (c), the shape this round argues for, written after (a) and (b) were read and
/// before it was measured (ADR-0074): one message of 1.125, the midpoint of (a) and (b),
/// 1.97 after the gain. (a) read the volley short by 2.4 units of 51 and 0.28 spikes per
/// unit after the opening; (b) read the volley whole and 0.70 after; so the after clause's
/// tenth lies below the drive at which the volley clause first holds, if the after-count
/// is monotone in the drive between them, and (c) reads whether it is. The expectation,
/// written first: the volley at about 0.97 per unit and about 0.5 spikes per unit after,
/// so (c) fails the after clause and no one-message shape passes both.
const CANDIDATE_C: Shape = (1, 0x0001_2000);
/// The candidates in the order tried and no other.
const CANDIDATES: [Shape; 3] = [CANDIDATE_A, CANDIDATE_B, CANDIDATE_C];
const _: () = assert!(CANDIDATES[0].0 == 1 && CANDIDATES[1].0 == 1 && CANDIDATES[2].0 == 1);
const _: () = assert!(CANDIDATES[1].1 == SHAPE_F46.1 && SHAPE_F46.0 == 2);
const _: () = assert!(CANDIDATE_C.1 * 2 == CANDIDATE_A.1 + CANDIDATE_B.1);
/// The measure's second clause: the presented set's spikes in the pair window after the
/// readout window's opening, per unit per presentation, at most this many tenths of a
/// spike (a tenth), against the 2.06 F-46 records.
const AFTER_MAX_TENTHS: u64 = 1;
/// The volley clause's tolerance, ADR-0065's: one spike per unit within two spikes of the
/// set, in tenths of a spike per presentation.
const VOLLEY_TOLERANCE_TENTHS: u64 = 20;
/// The criterion's mark over the calibration's sixty-four trials, for the offset of
/// Deliverable D read there: the rewarded clause's 80 of 128 at the same proportion, 40.
const OFFSET_MARK_64: u32 = REWARDED_MIN * BLOCK as u32 / (LAST_BLOCKS * BLOCK) as u32;
const _: () = assert!(OFFSET_MARK_64 == 40);
/// The ticks the probe of a unit at rest runs after one message: one pair window.
const PROBE_TICKS: u32 = PAIR_WINDOW;

/// The selection as the task's readout makes it (ADR-0059; the property in `task.rs`): the
/// sign of the count difference, none at equal counts.
fn selected(counts: [u32; 2]) -> Option<u8> {
    match counts[0].cmp(&counts[1]) {
        core::cmp::Ordering::Greater => Some(0),
        core::cmp::Ordering::Less => Some(1),
        core::cmp::Ordering::Equal => None,
    }
}

/// The volley clause: over the block, the presented set's spikes before the readout window
/// opens are one per unit within `VOLLEY_TOLERANCE_TENTHS` tenths per presentation, for
/// each stimulus (ADR-0065's reading of the calibration, kept as it was read there).
fn volley_once(units: u32, block: &Block) -> bool {
    let [a, _, _, _] = geometry(units, rotation(units));
    let once = a.len().saturating_mul(10);
    let a_trials = u64::from(block.1);
    let b_trials = (BLOCK as u64).saturating_sub(a_trials);
    let per = |spikes: u64, trials: u64| spikes.saturating_mul(10).checked_div(trials);
    [per(block.3[0], a_trials), per(block.3[1], b_trials)]
        .iter()
        .all(|per| {
            per.is_some_and(|per| {
                per <= once && per >= once.saturating_sub(VOLLEY_TOLERANCE_TENTHS)
            })
        })
}

/// The after clause: over the block, the presented set's spikes in the pair window after
/// the readout window's opening are at most `AFTER_MAX_TENTHS` tenths per unit per
/// presentation.
fn after_quiet(units: u32, composition: &Composition) -> bool {
    let [a, _, _, _] = geometry(units, rotation(units));
    let after = composition.2[2][1];
    after.saturating_mul(10)
        <= (BLOCK as u64)
            .saturating_mul(a.len())
            .saturating_mul(AFTER_MAX_TENTHS)
}

/// A candidate passes when it fires once: the volley clause and the after clause both.
fn fires_once(units: u32, block: &Block, composition: &Composition) -> bool {
    volley_once(units, block) && after_quiet(units, composition)
}

/// The stimulus the round picks: the first candidate, in the candidates' order, whose
/// frozen run fires once; none when none does.
fn candidate_pick(runs: &[(Block, u64, Composition)]) -> Option<Shape> {
    CANDIDATES
        .iter()
        .zip(runs.iter())
        .find(|(_, (block, _, composition))| fires_once(1024, block, composition))
        .map(|(&shape, _)| shape)
}

/// One message of `shape` into unit 0 of the instrument's network at 1 024 units, at rest
/// at the gain, with no drive: the ticks at which unit 0 fires within `PROBE_TICKS`, read
/// from the train. The engine's own reading of what the oracle above computed.
fn probe(shape: Shape) -> Vec<u32> {
    let p = prior(1024);
    let mut exec = at_gain(&p, config(1024, 1, 0), GAIN_1024);
    assert!(exec.is_quiescent());
    let inject = exec.injector();
    for _ in 0..shape.0 {
        inject
            .inject(0, spike_message(shape.1, false))
            .expect("the ring has room");
    }
    let start = exec.ticks() as u32;
    for _ in 0..PROBE_TICKS {
        exec.tick();
    }
    exec.train()
        .iter()
        .filter(|&&(_, unit)| unit == 0)
        .map(|&(tick, _)| tick.wrapping_sub(start))
        .collect()
}

// --------------------------------------------- what the criterion requires (deliverable D)

/// The answer readout of a stimulus, as `Task::answer` has it.
fn answer_of(stimulus: u8, mirrored: bool) -> usize {
    usize::from(if mirrored { stimulus ^ 1 } else { stimulus })
}

/// (a) The instrument's bias, per stimulus: the sum over the trials that presented it of
/// the answer readout's count less the other readout's, and those trials, as an integer
/// ratio `(difference, trials)`; a negative difference favours the readout that is not the
/// answer.
fn bias(counted: &[Counted], mirrored: bool) -> [(i64, u32); 2] {
    let mut out = [(0i64, 0u32); 2];
    for &(stimulus, counts) in counted {
        let answer = answer_of(stimulus, mirrored);
        let other = answer ^ 1;
        let difference = i64::from(counts[answer]).saturating_sub(i64::from(counts[other]));
        let into = &mut out[usize::from(stimulus)];
        into.0 = into.0.saturating_add(difference);
        into.1 = into.1.saturating_add(1);
    }
    out
}

/// The correct trials of `counted` when `delta` is added to the answer readout's count on
/// every trial, the selection as `Task` makes it and a tie an error.
fn correct_with(counted: &[Counted], mirrored: bool, delta: u32) -> u32 {
    counted.iter().fold(0u32, |sum, &(stimulus, counts)| {
        let answer = answer_of(stimulus, mirrored);
        let mut shifted = counts;
        shifted[answer] = shifted[answer].saturating_add(delta);
        sum.saturating_add(u32::from(selected(shifted) == Some(answer as u8)))
    })
}

/// (b) The offset the criterion needs: the smallest whole `delta` such that adding it to
/// the answer readout's count on every trial carries the correct trials to `mark`, found by
/// a scan from zero to one past the largest count difference, at which every trial is
/// correct; none when `mark` exceeds the trials, which no scan reaches.
fn offset(counted: &[Counted], mirrored: bool, mark: u32) -> Option<u32> {
    let widest = counted
        .iter()
        .fold(0u32, |w, &(_, counts)| w.max(counts[0].abs_diff(counts[1])));
    (0..=widest.saturating_add(1)).find(|&delta| correct_with(counted, mirrored, delta) >= mark)
}

/// (c) The difference a block holds, per stimulus, from its summed counts: the answer
/// readout's spikes over the block's trials that presented the stimulus less the other
/// readout's, and those trials, as `(difference, trials)`; what the delivery moved is this
/// in a run's last block against its first.
fn block_bias(block: &Block, mirrored: bool) -> [(i64, u32); 2] {
    let a_trials = block.1;
    let b_trials = (BLOCK as u32).saturating_sub(a_trials);
    let of = |stimulus: u8, trials: u32| {
        let answer = answer_of(stimulus, mirrored);
        let s = usize::from(stimulus);
        (
            (block.2[s][answer] as i64).saturating_sub(block.2[s][answer ^ 1] as i64),
            trials,
        )
    };
    [of(0, a_trials), of(1, b_trials)]
}

/// The dump of Deliverable D's readings over a run's per-trial counts: the bias and the
/// offset at `mark`.
fn dump_requires(name: &str, counted: &[Counted], mirrored: bool, mark: u32) {
    eprintln!(
        "DUMP {name} requires bias {:?} offset {:?} at {mark} of {} counted {counted:?}",
        bias(counted, mirrored),
        offset(counted, mirrored, mark),
        counted.len()
    );
}

// ------------------------------------------------------------ the measurement (brief 033)

/// The candidates at 1 024 units, in the candidates' order, each a frozen run of
/// sixty-four trials at the present gain read by the sight, the sign and the measure that
/// picks: the sight's block and trace, and the composition. Pinned from one run each.
const ONCE_1024: [(Block, u64, Composition); 3] = [
    (
        (
            32,
            34,
            [[352, 327], [330, 338]],
            [1651, 1461],
            [3012, 2709],
            [257, 239],
            63,
            213902976,
            235822619,
            0,
            [[6986739, 7212710], [7146878, 7257384]],
            6,
        ),
        0x42c488fa8370cd67,
        (
            [[-11202, -17464], [-9421, -6182]],
            [
                [
                    [280098, -80914, 354009, -736327],
                    [265688, -60844, 389347, -713873],
                ],
                [
                    [244715, -57031, 313006, -632692],
                    [254062, -67362, 367226, -660112],
                ],
            ],
            [[1059, 1807], [1049, 1832], [3217, 911]],
            2,
            3075011394120061531,
        ),
    ),
    (
        (
            31,
            34,
            [[390, 374], [374, 378]],
            [1715, 1514],
            [3822, 3392],
            [257, 232],
            63,
            213902976,
            235822619,
            0,
            [[6986739, 7212710], [7146878, 7257384]],
            9,
        ),
        0xd39ec55203c68a8b,
        (
            [[-15256, -25370], [-14720, -15719]],
            [
                [
                    [355325, -102799, 350608, -873999],
                    [332868, -88384, 387923, -876730],
                ],
                [
                    [313081, -97315, 336729, -781233],
                    [299550, -96603, 380003, -781616],
                ],
            ],
            [[1049, 2037], [1042, 2052], [3332, 2277]],
            1,
            1732380210409406099,
        ),
    ),
    (
        (
            30,
            34,
            [[365, 350], [353, 350]],
            [1702, 1497],
            [3381, 3026],
            [253, 237],
            62,
            213902976,
            235822619,
            0,
            [[6986739, 7212710], [7146878, 7257384]],
            6,
        ),
        0xef44f6eee8436967,
        (
            [[-13221, -22593], [-6238, -10911]],
            [
                [
                    [311063, -93437, 357001, -806596],
                    [295797, -75992, 392519, -792305],
                ],
                [
                    [277828, -68645, 321778, -688156],
                    [270162, -83202, 373574, -710051],
                ],
            ],
            [[1045, 1905], [1038, 1936], [3304, 1502]],
            0,
            11588833636741486556,
        ),
    ),
];
/// The rows of every candidate's sixty-four trials, in the candidates' order.
const ONCE_ROWS_1024: [[Row; BLOCK]; 3] = [
    [
        (0, -5859, [13299, -8565, 10063, -20768]),
        (0, -80, [17533, -1124, 20316, -32032]),
        (1, -37841, [12659, -11371, 23933, -42364]),
        (0, -7194, [13770, -1206, 19032, -32056]),
        (0, 5495, [18915, -865, 19854, -27580]),
        (0, -2870, [15274, -3024, 12988, -31779]),
        (0, -8467, [10889, -1780, 11702, -26942]),
        (0, -11109, [12195, -1475, 16840, -33377]),
        (0, -9986, [18469, -7843, 17086, -27104]),
        (0, -20111, [13679, -2925, 26231, -49717]),
        (1, -18567, [11696, -7698, 23306, -26119]),
        (1, -3867, [24621, -5200, 13953, -23043]),
        (0, -26768, [12385, -6371, 24000, -40404]),
        (0, -13380, [26698, -8359, 27509, -38315]),
        (1, -14258, [9657, -4454, 17550, -30166]),
        (0, -13904, [15442, -10663, 13673, -24744]),
        (1, -12039, [10475, -3108, 18660, -29928]),
        (0, -31957, [10135, -2260, 24264, -51216]),
        (0, -41530, [15806, -5346, 12121, -38996]),
        (0, -45919, [13301, -3126, 14978, -37270]),
        (1, -16644, [3978, -1038, 18557, -25170]),
        (0, -46986, [13865, -7049, 15776, -41262]),
        (1, -15411, [15285, -2907, 18179, -40940]),
        (1, -6363, [18775, -2029, 17928, -29074]),
        (1, -23266, [21199, -10212, 19998, -46791]),
        (0, -34435, [4875, -5982, 16903, -26332]),
        (1, -26599, [17408, -6938, 12301, -36943]),
        (1, -24583, [16458, -1785, 12804, -31574]),
        (1, -23486, [11234, -2878, 13851, -26373]),
        (0, -28708, [18012, -8327, 20519, -36370]),
        (0, -25385, [12513, -1297, 16065, -30688]),
        (0, -25396, [15439, -6154, 14513, -30272]),
        (0, -23607, [20560, -3115, 13602, -35392]),
        (0, -28015, [17339, -2355, 13707, -38599]),
        (1, -24247, [4190, -3331, 16384, -30790]),
        (1, -19042, [20990, -2674, 12950, -31869]),
        (1, -5713, [22413, -978, 13212, -24795]),
        (0, -30833, [5722, -2025, 23931, -39541]),
        (1, -5285, [18290, -5486, 30517, -36747]),
        (1, 1873, [21808, -2670, 15869, -30160]),
        (0, -17157, [8605, -4161, 24569, -29835]),
        (1, -13901, [14028, -4255, 15376, -31263]),
        (0, -16165, [20894, -7886, 20443, -35867]),
        (1, -18952, [7962, -2174, 15959, -30174]),
        (0, -10768, [11568, -2968, 24702, -36736]),
        (1, -17621, [16807, -6727, 16148, -32566]),
        (0, -31069, [5071, -933, 12161, -35531]),
        (0, -13281, [23135, -4722, 21150, -26667]),
        (0, -6737, [20073, -5132, 13784, -25006]),
        (1, -13504, [16431, -10506, 28357, -33677]),
        (1, -15278, [15851, -4285, 16392, -32640]),
        (0, -19168, [5222, -2217, 21403, -33989]),
        (1, -9027, [15696, -386, 20498, -35827]),
        (1, -8105, [14958, -3392, 16168, -30489]),
        (0, -21872, [9452, -2685, 21860, -37361]),
        (1, -22163, [10569, -1959, 14265, -34054]),
        (1, -21159, [14012, -1906, 14918, -30457]),
        (1, -24423, [20034, -4903, 16179, -39597]),
        (0, -29715, [7794, -2789, 24615, -42715]),
        (1, -10399, [12497, -2618, 15564, -23628]),
        (0, -26717, [16760, -3760, 19729, -36944]),
        (0, -24133, [18095, -3269, 15365, -33388]),
        (1, -15497, [5055, -2579, 22602, -32091]),
        (1, -15603, [12947, -3945, 13088, -25706]),
    ],
    [
        (0, -1761, [21400, -11325, 8348, -20237]),
        (0, -3205, [19862, -483, 25230, -46990]),
        (1, -48589, [14342, -12429, 31620, -58757]),
        (0, -7949, [17296, -2604, 22164, -42677]),
        (0, 8274, [27187, -2966, 21497, -31863]),
        (0, -5297, [17301, -2806, 13492, -39957]),
        (0, -15065, [12593, -2364, 11182, -32304]),
        (0, -21320, [14792, -4377, 14656, -36080]),
        (0, -27850, [14361, -4670, 18457, -37287]),
        (0, -45317, [16566, -2350, 23670, -62636]),
        (1, -23043, [19357, -10400, 24329, -34788]),
        (1, -14251, [31210, -15476, 15324, -27971]),
        (0, -41153, [17098, -5552, 23176, -48436]),
        (0, -22087, [36761, -9501, 30468, -46620]),
        (1, -24460, [11915, -5428, 19456, -38388]),
        (0, -26712, [17030, -11679, 16127, -33615]),
        (1, -15212, [20328, -6889, 20355, -36154]),
        (0, -43953, [14462, -2195, 21532, -57715]),
        (0, -58370, [21420, -4168, 13475, -55044]),
        (0, -69675, [15785, -3783, 13723, -49432]),
        (1, -22296, [13043, -13322, 23137, -30726]),
        (0, -67218, [15405, -4960, 18442, -52716]),
        (1, -19768, [19568, -4520, 20889, -46929]),
        (1, -14267, [22524, -4946, 17872, -34581]),
        (1, -38852, [21220, -9483, 22298, -60272]),
        (0, -48158, [13031, -8629, 14333, -36635]),
        (1, -42282, [17300, -12050, 18173, -44727]),
        (1, -46657, [18908, -3374, 13730, -42444]),
        (1, -55012, [10243, -1544, 13554, -41051]),
        (0, -47100, [22329, -14914, 18289, -49802]),
        (0, -40766, [20917, -2680, 17986, -40123]),
        (0, -51419, [17556, -8616, 14468, -43760]),
        (0, -46379, [24110, -5898, 11403, -36884]),
        (0, -57402, [17999, -4978, 13208, -48714]),
        (1, -27358, [10412, -6483, 18235, -32285]),
        (1, -24030, [29387, -4533, 15039, -44161]),
        (1, -21067, [18303, -989, 14164, -33851]),
        (0, -51411, [13405, -11527, 21977, -49536]),
        (1, -26993, [17516, -4887, 26931, -45437]),
        (1, -22758, [23997, -3597, 19427, -41895]),
        (0, -30494, [11936, -6470, 21278, -32379]),
        (1, -37763, [18769, -8813, 13628, -38200]),
        (0, -22329, [26016, -11823, 23340, -41438]),
        (1, -40296, [13420, -4477, 19599, -44430]),
        (0, -21352, [16497, -5742, 24742, -43554]),
        (1, -32234, [24946, -13729, 17431, -37791]),
        (0, -41067, [10317, -1022, 12869, -44886]),
        (0, -34055, [21130, -3979, 22414, -40963]),
        (0, -34689, [20667, -6512, 12586, -34442]),
        (1, -28431, [17865, -11750, 30342, -43289]),
        (1, -33006, [20005, -4976, 19857, -45699]),
        (0, -45935, [9591, -5238, 20496, -45990]),
        (1, -21329, [18947, -1894, 19991, -40644]),
        (1, -24428, [16899, -5206, 12851, -33406]),
        (0, -44646, [12858, -4364, 20125, -48230]),
        (1, -38084, [12325, -2747, 12414, -42495]),
        (1, -34346, [21793, -1950, 17114, -42080]),
        (1, -41986, [20659, -4286, 19220, -51439]),
        (0, -42278, [11249, -5160, 23658, -48872]),
        (1, -29033, [16360, -4062, 16411, -34266]),
        (0, -42542, [22343, -4700, 20311, -49086]),
        (0, -40677, [17512, -3148, 15877, -39500]),
        (1, -30158, [9220, -4085, 19686, -36523]),
        (1, -30439, [15417, -5592, 13296, -29833]),
    ],
    [
        (0, -6113, [15928, -9026, 8047, -21194]),
        (0, -11945, [20835, -3424, 19311, -44218]),
        (1, -42295, [12599, -11634, 25107, -48030]),
        (0, -16307, [15284, -2536, 22021, -36884]),
        (0, -3619, [20318, -874, 18226, -30130]),
        (0, -13950, [15829, -3301, 13833, -37274]),
        (0, -19089, [11025, -1751, 11727, -29174]),
        (0, -19067, [13191, -1523, 14775, -32188]),
        (0, -18309, [17900, -7211, 17653, -29188]),
        (0, -33332, [14154, -2391, 25759, -57727]),
        (1, -22590, [12415, -9235, 23084, -29753]),
        (1, -7734, [25800, -5990, 13965, -24389]),
        (0, -29152, [19170, -7859, 22648, -41497]),
        (0, -17295, [30408, -8383, 29153, -44779]),
        (1, -18419, [8943, -3461, 17956, -34494]),
        (0, -18640, [15895, -9689, 14070, -29915]),
        (1, -14774, [12097, -3142, 21672, -34851]),
        (0, -35011, [11454, -766, 21724, -51737]),
        (0, -42262, [23945, -5201, 15059, -48476]),
        (0, -51165, [15334, -3758, 14280, -42983]),
        (1, -22627, [10502, -10705, 21378, -28195]),
        (0, -51484, [11942, -2781, 16837, -45089]),
        (1, -19506, [17439, -4481, 20172, -43468]),
        (1, -16215, [19512, -6439, 16998, -31045]),
        (1, -34567, [19939, -9175, 20620, -51101]),
        (0, -37507, [10515, -8618, 14074, -28331]),
        (1, -33248, [16878, -5852, 12327, -39212]),
        (1, -36858, [16768, -3280, 12923, -37787]),
        (1, -33269, [15750, -3146, 14108, -31168]),
        (0, -33428, [20580, -12935, 21428, -41746]),
        (0, -28403, [17266, -2262, 21083, -38259]),
        (0, -35821, [18136, -9714, 12137, -34621]),
        (0, -31243, [23143, -7517, 13152, -32376]),
        (0, -38559, [17588, -3815, 13992, -43129]),
        (1, -24170, [5423, -5602, 18683, -30249]),
        (1, -20431, [24452, -4681, 14878, -36846]),
        (1, -14899, [18277, -978, 13038, -29076]),
        (0, -39889, [11796, -11406, 26932, -43043]),
        (1, -19311, [16290, -6875, 26905, -38089]),
        (1, -14318, [23135, -3658, 17806, -37291]),
        (0, -25667, [9204, -5291, 22132, -30292]),
        (1, -25201, [13420, -3687, 15538, -33049]),
        (0, -20407, [16317, -3528, 20675, -38364]),
        (1, -24194, [12085, -2546, 19232, -36375]),
        (0, -20817, [13719, -4315, 22548, -40000]),
        (1, -20776, [18693, -6517, 16258, -34527]),
        (0, -39642, [9496, -2161, 13475, -42009]),
        (0, -29222, [19088, -3970, 21251, -33802]),
        (0, -25329, [20714, -6071, 12140, -28287]),
        (1, -16932, [16161, -9982, 29865, -38025]),
        (1, -22798, [17220, -5524, 20112, -41425]),
        (0, -31911, [6088, -1882, 21521, -38950]),
        (1, -13437, [17966, -907, 18675, -37303]),
        (1, -12973, [15826, -3653, 15503, -31546]),
        (0, -34414, [11576, -5422, 21443, -43426]),
        (1, -32019, [11378, -2653, 13216, -43047]),
        (1, -24573, [19438, -1105, 16226, -33595]),
        (1, -29734, [18864, -3660, 17749, -43812]),
        (0, -32527, [7750, -2252, 25047, -44905]),
        (1, -13483, [14537, -4002, 17136, -28140]),
        (0, -28575, [21894, -5374, 20424, -39690]),
        (0, -32072, [13984, -2422, 15730, -39414]),
        (1, -16857, [11749, -4340, 21095, -34411]),
        (1, -17149, [15365, -4936, 12978, -27479]),
    ],
];
/// Every trial's readout counts under every candidate, in the candidates' order.
const ONCE_COUNTED_1024: [[Counted; BLOCK]; 3] = [
    [
        (0, [6, 10]),
        (0, [14, 12]),
        (1, [15, 23]),
        (0, [13, 9]),
        (0, [5, 5]),
        (0, [5, 9]),
        (0, [7, 4]),
        (0, [10, 5]),
        (0, [12, 13]),
        (0, [6, 4]),
        (1, [12, 15]),
        (1, [10, 13]),
        (0, [15, 11]),
        (0, [15, 12]),
        (1, [14, 25]),
        (0, [10, 10]),
        (1, [10, 10]),
        (0, [5, 9]),
        (0, [12, 13]),
        (0, [15, 12]),
        (1, [16, 14]),
        (0, [8, 4]),
        (1, [11, 14]),
        (1, [5, 4]),
        (1, [8, 14]),
        (0, [7, 7]),
        (1, [9, 12]),
        (1, [10, 4]),
        (1, [11, 7]),
        (0, [13, 12]),
        (0, [9, 13]),
        (0, [10, 12]),
        (0, [16, 12]),
        (0, [6, 5]),
        (1, [14, 12]),
        (1, [18, 12]),
        (1, [12, 6]),
        (0, [14, 10]),
        (1, [12, 15]),
        (1, [5, 10]),
        (0, [13, 8]),
        (1, [13, 6]),
        (0, [16, 12]),
        (1, [12, 12]),
        (0, [9, 5]),
        (1, [14, 10]),
        (0, [8, 10]),
        (0, [8, 15]),
        (0, [8, 5]),
        (1, [11, 10]),
        (1, [7, 10]),
        (0, [7, 12]),
        (1, [10, 8]),
        (1, [6, 9]),
        (0, [13, 16]),
        (1, [9, 13]),
        (1, [14, 12]),
        (1, [10, 7]),
        (0, [9, 11]),
        (1, [10, 12]),
        (0, [13, 11]),
        (0, [15, 9]),
        (1, [11, 8]),
        (1, [11, 11]),
    ],
    [
        (0, [8, 12]),
        (0, [12, 11]),
        (1, [24, 33]),
        (0, [15, 13]),
        (0, [6, 6]),
        (0, [5, 9]),
        (0, [8, 4]),
        (0, [11, 6]),
        (0, [12, 12]),
        (0, [5, 6]),
        (1, [15, 16]),
        (1, [10, 15]),
        (0, [16, 14]),
        (0, [14, 12]),
        (1, [17, 28]),
        (0, [14, 10]),
        (1, [12, 9]),
        (0, [5, 13]),
        (0, [10, 17]),
        (0, [15, 13]),
        (1, [19, 17]),
        (0, [8, 4]),
        (1, [10, 16]),
        (1, [7, 5]),
        (1, [8, 15]),
        (0, [11, 15]),
        (1, [8, 11]),
        (1, [10, 4]),
        (1, [10, 6]),
        (0, [18, 15]),
        (0, [10, 14]),
        (0, [9, 12]),
        (0, [16, 10]),
        (0, [8, 3]),
        (1, [18, 18]),
        (1, [17, 9]),
        (1, [13, 6]),
        (0, [19, 13]),
        (1, [13, 19]),
        (1, [6, 10]),
        (0, [15, 9]),
        (1, [15, 7]),
        (0, [19, 13]),
        (1, [15, 15]),
        (0, [11, 4]),
        (1, [13, 12]),
        (0, [6, 10]),
        (0, [7, 14]),
        (0, [10, 7]),
        (1, [13, 13]),
        (1, [10, 11]),
        (0, [11, 18]),
        (1, [13, 8]),
        (1, [8, 8]),
        (0, [13, 17]),
        (1, [12, 12]),
        (1, [13, 13]),
        (1, [10, 6]),
        (0, [13, 13]),
        (1, [11, 13]),
        (0, [14, 11]),
        (0, [16, 14]),
        (1, [12, 9]),
        (1, [12, 14]),
    ],
    [
        (0, [7, 11]),
        (0, [15, 13]),
        (1, [18, 28]),
        (0, [13, 10]),
        (0, [5, 6]),
        (0, [5, 9]),
        (0, [7, 4]),
        (0, [10, 7]),
        (0, [12, 13]),
        (0, [5, 5]),
        (1, [14, 15]),
        (1, [10, 13]),
        (0, [16, 14]),
        (0, [15, 14]),
        (1, [15, 26]),
        (0, [12, 10]),
        (1, [10, 10]),
        (0, [6, 13]),
        (0, [12, 16]),
        (0, [15, 12]),
        (1, [17, 15]),
        (0, [8, 4]),
        (1, [9, 14]),
        (1, [7, 4]),
        (1, [8, 15]),
        (0, [9, 10]),
        (1, [8, 11]),
        (1, [10, 5]),
        (1, [12, 6]),
        (0, [14, 14]),
        (0, [9, 13]),
        (0, [9, 13]),
        (0, [16, 10]),
        (0, [6, 3]),
        (1, [14, 12]),
        (1, [19, 10]),
        (1, [12, 6]),
        (0, [15, 11]),
        (1, [12, 18]),
        (1, [5, 10]),
        (0, [13, 8]),
        (1, [15, 6]),
        (0, [16, 11]),
        (1, [15, 13]),
        (0, [9, 4]),
        (1, [13, 11]),
        (0, [7, 10]),
        (0, [8, 15]),
        (0, [9, 6]),
        (1, [12, 10]),
        (1, [7, 11]),
        (0, [9, 13]),
        (1, [11, 8]),
        (1, [9, 9]),
        (0, [13, 16]),
        (1, [12, 13]),
        (1, [13, 11]),
        (1, [11, 6]),
        (0, [13, 11]),
        (1, [11, 11]),
        (0, [11, 11]),
        (0, [16, 10]),
        (1, [12, 10]),
        (1, [12, 13]),
    ],
];

/// The stimulus the round picked, as `candidate_pick` reads the pinned tables: none, no
/// candidate having fired every unit once and added under a tenth of a spike after.
const SHAPE_PICKED_1024: Option<Shape> = None;
/// The ticks at which unit 0 of the instrument's network at 1 024 units, at rest at the
/// gain 1.75 with no drive, fires within one pair window after one injection of each
/// shape (`probe`): F-46's two messages of 1.25 fire it twice, at the end of the
/// refractory window from what the basal compartment still holds; (a) not at all; (b)
/// once; (c) not at all. The engine's own reading of what one message does to a unit at
/// rest, pinned; the executor scales an injected message by the synaptic gain (F-47).
const PROBED_1024: [(Shape, &[u32]); 4] = [
    (SHAPE_F46, &[5, 206]),
    (CANDIDATE_A, &[]),
    (CANDIDATE_B, &[22]),
    (CANDIDATE_C, &[]),
];
/// Every trial's readout counts of ADR-0072's composition run, F-46's stimulus at 1 024
/// units and the gain 1.75 with the weights frozen (`the_composition_at_1024_units_exhaustive`,
/// `CALIBRATION_1024[0]`'s run): Deliverable D's readings of the instrument as it stands.
const COUNTED_1024: &[Counted] = &[
    (0, [16, 19]),
    (0, [11, 12]),
    (1, [29, 31]),
    (0, [14, 9]),
    (0, [2, 3]),
    (0, [5, 9]),
    (0, [5, 5]),
    (0, [9, 5]),
    (0, [9, 12]),
    (0, [6, 6]),
    (1, [25, 24]),
    (1, [6, 8]),
    (0, [18, 18]),
    (0, [13, 8]),
    (1, [15, 31]),
    (0, [15, 8]),
    (1, [11, 9]),
    (0, [7, 13]),
    (0, [10, 12]),
    (0, [12, 11]),
    (1, [24, 20]),
    (0, [12, 7]),
    (1, [14, 15]),
    (1, [5, 4]),
    (1, [8, 12]),
    (0, [10, 12]),
    (1, [9, 12]),
    (1, [8, 4]),
    (1, [11, 5]),
    (0, [19, 18]),
    (0, [8, 14]),
    (0, [9, 11]),
    (0, [14, 10]),
    (0, [8, 3]),
    (1, [26, 26]),
    (1, [15, 8]),
    (1, [10, 4]),
    (0, [20, 18]),
    (1, [15, 15]),
    (1, [6, 7]),
    (0, [17, 8]),
    (1, [16, 7]),
    (0, [15, 15]),
    (1, [13, 12]),
    (0, [12, 3]),
    (1, [13, 15]),
    (0, [9, 11]),
    (0, [8, 13]),
    (0, [8, 6]),
    (1, [11, 15]),
    (1, [7, 7]),
    (0, [10, 19]),
    (1, [13, 8]),
    (1, [5, 6]),
    (0, [16, 18]),
    (1, [12, 12]),
    (1, [9, 12]),
    (1, [7, 5]),
    (0, [20, 21]),
    (1, [12, 13]),
    (0, [14, 12]),
    (0, [15, 9]),
    (1, [14, 8]),
    (1, [10, 8]),
];

/// A candidate's frozen run: the sight's blocks and trace, the composed trials and every
/// trial's counts.
type CandidateRun = (Vec<Block>, u64, Vec<Composed>, Vec<Counted>);

/// The candidates, each run and dumped before any is held to its table, so that one
/// candidate's failure still shows the others' readings.
#[test]
#[ignore]
fn the_candidate_stimuli_at_1024_units_exhaustive() {
    let runs: Vec<CandidateRun> = CANDIDATES
        .iter()
        .map(|&shape| {
            let (blocks, trace, trials, counts, counted, _) =
                compose_shaped(shape, None, 1024, GAIN_1024, BLOCK);
            assert_eq!(counts, SYNAPSES_1024);
            (blocks, trace, trials, counted)
        })
        .collect();
    for (k, (blocks, trace, trials, counted)) in runs.iter().enumerate() {
        let shape = CANDIDATES[k];
        let name = format!("once1024 {k} {shape:?}");
        dump_composition(&name, blocks, *trace, trials);
        dump_requires(&name, counted, false, OFFSET_MARK_64);
        let composed = composition(trials);
        eprintln!(
            "DUMP {name} volley {} after {} fires_once {} sight {} sign {}",
            volley_once(1024, &blocks[0]),
            after_quiet(1024, &composed),
            fires_once(1024, &blocks[0], &composed),
            calibrated(&blocks[0]),
            composed.3
        );
    }
    for (k, (blocks, trace, trials, counted)) in runs.iter().enumerate() {
        let shape = CANDIDATES[k];
        let name = format!("once1024 {k} {shape:?}");
        let (block, pin, composed) = &ONCE_1024[k];
        pinned(&format!("{name} sight"), blocks, *trace, &[*block], *pin);
        pinned_composition(&name, trials, &ONCE_ROWS_1024[k], Some(composed));
        assert_eq!(
            counted.as_slice(),
            &ONCE_COUNTED_1024[k],
            "{name}: the counts"
        );
    }
}

/// The gate's test (ADR-0061's class): the selection rule, the two clauses, the measure
/// and the pick at their edges over readings written by hand; Deliverable D's three
/// readings at theirs and over the pinned tables; the pick over the pinned candidates as
/// written; the engine's probe of one message into a unit at rest, held to its table; and
/// the first eight trials of candidate (a), run and held to the first eight rows and
/// counts of its pinned tables; no number is pinned twice.
#[test]
fn the_first_eight_trials_of_candidate_a_at_1024_units_and_the_rules_over_its_tables() {
    // The selection is the sign of the count difference, none at equal counts.
    assert_eq!(selected([3, 2]), Some(0));
    assert_eq!(selected([2, 3]), Some(1));
    assert_eq!(selected([3, 3]), None);
    assert_eq!(selected([0, 0]), None);
    // The volley clause at its edges: 51 units, 34 A trials and 30 B trials; one per unit
    // within two spikes per presentation, in tenths.
    let mut block = CALIBRATION_1024[0].0;
    assert!(
        volley_once(1024, &block),
        "ADR-0065's calibration fires once in the volley"
    );
    block.3 = [34 * 51, 30 * 51];
    assert!(volley_once(1024, &block), "exactly one per unit");
    block.3 = [34 * 51 - 68, 30 * 51 - 60];
    assert!(
        volley_once(1024, &block),
        "two spikes short of the set, each"
    );
    block.3 = [34 * 51 - 69, 30 * 51 - 60];
    assert!(
        !volley_once(1024, &block),
        "a tenth more than two short on A fails"
    );
    block.3 = [34 * 51, 30 * 51 - 61];
    assert!(!volley_once(1024, &block), "on B");
    block.3 = [34 * 51 + 4, 30 * 51];
    assert!(!volley_once(1024, &block), "more than one per unit fails");
    block.1 = 0;
    block.3 = [0, 64 * 51];
    assert!(
        !volley_once(1024, &block),
        "no A trial: no reading, no pass"
    );
    block.1 = 64;
    block.3 = [64 * 51, 0];
    assert!(!volley_once(1024, &block), "no B trial");
    // The after clause at its edges: at most a tenth per unit per presentation over the
    // block, 326 spikes of 64 × 51 × 0.1 = 326.4.
    let after = |spikes: u64| -> Composition {
        (
            [[0; 2]; 2],
            [[[0; 4]; 2]; 2],
            [[0, 0], [0, 0], [0, spikes]],
            0,
            0,
        )
    };
    assert!(after_quiet(1024, &after(326)));
    assert!(!after_quiet(1024, &after(327)));
    assert!(after_quiet(1024, &after(0)));
    // The measure is both clauses; the pick is the first candidate that passes.
    let seen = CALIBRATION_1024[0].0;
    assert!(fires_once(1024, &seen, &after(326)));
    assert!(!fires_once(1024, &seen, &after(327)));
    let mut short = seen;
    short.3 = [34 * 51 - 69, 30 * 51];
    assert!(!fires_once(1024, &short, &after(0)));
    let passing = (seen, 0u64, after(0));
    let loud = (seen, 0u64, after(327));
    let missing = (short, 0u64, after(0));
    assert_eq!(
        candidate_pick(&[passing, passing, passing]),
        Some(CANDIDATE_A)
    );
    assert_eq!(candidate_pick(&[loud, passing, loud]), Some(CANDIDATE_B));
    assert_eq!(candidate_pick(&[missing, loud, passing]), Some(CANDIDATE_C));
    assert_eq!(candidate_pick(&[loud, missing, loud]), None);
    assert_eq!(candidate_pick(&[]), None);
    // Deliverable D's rules at their edges over rows written by hand.
    let rows: [Counted; 6] = [
        (0, [10, 8]),
        (0, [7, 7]),
        (0, [5, 9]),
        (1, [8, 10]),
        (1, [12, 12]),
        (1, [9, 4]),
    ];
    assert_eq!(
        bias(&rows, false),
        [(-2, 3), (-3, 3)],
        "A +2 +0 −4; B +2 +0 −5"
    );
    assert_eq!(
        bias(&rows, true),
        [(2, 3), (3, 3)],
        "mirrored: the answers swap"
    );
    assert_eq!(bias(&[], false), [(0, 0), (0, 0)]);
    assert_eq!(
        correct_with(&rows, false, 0),
        2,
        "10>8 and 8<10: A's first and B's first"
    );
    assert_eq!(
        correct_with(&rows, false, 1),
        4,
        "the ties go to the answer"
    );
    assert_eq!(
        correct_with(&rows, false, 4),
        4,
        "5+4 ties 9 and 4+4 is still below 9: two errors"
    );
    assert_eq!(correct_with(&rows, false, 5), 5, "4+5 ties 9: one error");
    assert_eq!(correct_with(&rows, false, 6), 6);
    assert_eq!(offset(&rows, false, 2), Some(0));
    assert_eq!(offset(&rows, false, 3), Some(1));
    assert_eq!(offset(&rows, false, 4), Some(1));
    assert_eq!(offset(&rows, false, 5), Some(5));
    assert_eq!(offset(&rows, false, 6), Some(6));
    assert_eq!(
        offset(&rows, false, 7),
        None,
        "seven of six is out of reach"
    );
    assert_eq!(offset(&[], false, 0), Some(0));
    assert_eq!(offset(&[], false, 1), None);
    let mut with_a_trials = CALIBRATION_1024[0].0;
    with_a_trials.1 = 34;
    with_a_trials.2 = [[396, 378], [379, 363]];
    assert_eq!(
        block_bias(&with_a_trials, false),
        [(18, 34), (-16, 30)],
        "ADR-0065's calibration: readout 0 leads on either stimulus"
    );
    assert_eq!(block_bias(&with_a_trials, true), [(-18, 34), (16, 30)]);
    // Over the pinned tables: the candidates' readings as the constants say, the pick as
    // written, and ADR-0069's addressed run and the fixed modulation, eighth block
    // against first.
    for (k, (block, _, composed)) in ONCE_1024.iter().enumerate() {
        let shape = CANDIDATES[k];
        let rows = &ONCE_ROWS_1024[k];
        let counted = &ONCE_COUNTED_1024[k];
        let signs = rows.iter().filter(|r| r.1 > 0).count() as u32;
        assert_eq!(signs, composed.3, "candidate {k}: the signs are the rows'");
        assert_eq!(
            counted.iter().filter(|c| c.0 == 0).count() as u32,
            block.1,
            "candidate {k}: the A trials"
        );
        let [a, _, _, _] = geometry(1024, ROTATION_1024);
        eprintln!(
            "DUMP once1024 {k} {shape:?} volley {:?} after {} per unit {} tenths seen {} signs {} fires_once {} bias {:?} offset {:?}",
            block.3,
            composed.2[2][1],
            composed.2[2][1] * 10 / (64 * a.len()),
            block.6,
            composed.3,
            fires_once(1024, block, composed),
            bias(counted, false),
            offset(counted, false, OFFSET_MARK_64)
        );
        assert!(calibrated(block), "candidate {k}: the sight passes");
        assert!(composed.3 < SIGN_MIN, "candidate {k}: the sign fails");
        assert!(!fires_once(1024, block, composed), "candidate {k}");
    }
    assert!(
        volley_once(1024, &ONCE_1024[1].0),
        "(b) fires the volley whole"
    );
    assert!(volley_once(1024, &ONCE_1024[2].0), "(c) too");
    assert!(
        !volley_once(1024, &ONCE_1024[0].0),
        "(a) misses units of the volley"
    );
    assert_eq!(
        candidate_pick(&ONCE_1024),
        SHAPE_PICKED_1024,
        "the pick as written"
    );
    assert_eq!(
        [
            offset(&ONCE_COUNTED_1024[0], false, OFFSET_MARK_64),
            offset(&ONCE_COUNTED_1024[1], false, OFFSET_MARK_64),
            offset(&ONCE_COUNTED_1024[2], false, OFFSET_MARK_64),
        ],
        [Some(2), Some(1), Some(3)],
        "the offset the criterion needs over the candidates' calibrations"
    );
    assert_eq!(
        (
            block_bias(&ADDRESSED_1024[7], false),
            block_bias(&ADDRESSED_1024[0], false),
            block_bias(&FIXED_1024[7], false),
            block_bias(&FIXED_1024[0], false),
        ),
        (
            [(-48, 32), (-7, 32)],
            [(4, 34), (-24, 30)],
            [(-34, 32), (15, 32)],
            [(6, 34), (-27, 30)],
        ),
        "ADR-0069's addressed rewarded run and the fixed modulation, last block and first"
    );
    // The engine's probe of one message into a unit at rest, held to its table.
    for (shape, ticks) in PROBED_1024 {
        let read = probe(shape);
        eprintln!("DUMP probe1024 {shape:?} -> {read:?}");
        assert_eq!(
            read.as_slice(),
            ticks,
            "one message of {shape:?} into a unit at rest"
        );
    }
    // The first eight trials of candidate (a), run.
    let (blocks, trace, trials, counts, counted, _) =
        compose_shaped(CANDIDATE_A, None, 1024, GAIN_1024, GATE_TRIALS);
    dump_composition("once1024 0 first eight", &blocks, trace, &trials);
    assert_eq!(counts, SYNAPSES_1024);
    assert!(blocks.is_empty(), "no whole block");
    pinned_composition(
        "once1024 0 first eight",
        &trials,
        &ONCE_ROWS_1024[0][..GATE_TRIALS],
        None,
    );
    assert_eq!(counted.as_slice(), &ONCE_COUNTED_1024[0][..GATE_TRIALS]);
}

// ------------------------------------------------ two injections (brief 034)
//
// Written before the run. ADR-0074 named the shape that neither weakens the response nor
// leaves the stimulus units their spikes at the end of the refractory window: F-46's drive,
// then a message that cancels what the basal compartment holds when the window ends. Brief
// 034 asks for it and nothing else: a `Cancel` on the task's stimulus (`task.rs`), a
// negative basal message into the presented set injected inside the trial; the offset
// `REFRACTORY_TICKS` from the trial's first tick, "not searched"; the cancel's size from
// three candidates in a fixed order — the residual the drive leaves in the basal
// compartment of a unit at rest, twice it, and the bound one message carries — each
// derived after the gain (F-47) by an integer oracle from `BASAL_LEAK_SHIFT` and
// `REFRACTORY_TICKS` and probed on a unit at rest before it is used; the first candidate
// passing the probe and ADR-0074's two clauses over a frozen run is the stimulus.
//
// The tree says two things the brief's arithmetic did not (principle 1: the repository
// wins). *First*, the membrane rule drops an input that lands inside the refractory window
// (`integrate` in `membrane.rs`: `basal_in` is zero while `refractory_ticks` is above
// zero; ADR-0018 chose that so that a burst of input cannot fire the unit the tick the
// window ends). A unit that fires its volley spike on trial tick `k` drops every input on
// ticks `k + 1 ..= k + 200` and integrates again on `k + 201`; F-46's drive lands on tick
// one, the earliest volley spike is on tick one, so the earliest tick any volley unit can
// integrate again is 202, and the brief's cancel — injected at offset 200, landing on tick
// 201 — lands inside the window of every unit that fired in the volley and is dropped.
// *Second*, what fires the unit again is not the basal residual but the soma, which the
// coupling has pulled toward half of the residual through the whole window: on the first
// tick the unit integrates again the soma stands above the threshold already, and a cancel
// that lands then must pull it back below the threshold within that one tick through the
// coupling's sixteenth, so it must take the basal compartment well below rest rather than
// to it. The oracle below (`alone`: `cortex-core`'s `integrate` stepped alone on one unit,
// with the executor's scaling by the gain and its timing, a message injected before trial
// tick `k` landing on tick `k + 1`) reads for a unit at rest: F-46's drive fires it on tick
// 5 and again on 206; the basal residual on tick 205 is 2.94 after the gain; a cancel on
// tick 206 must be at least 7.41 after the gain to leave the unit its one spike, 2.5 times
// the residual; and a cancel on any tick from 201 to 205 is dropped whatever its size.
// So the brief's three candidates at the brief's offset are all dropped, and at the tick
// the unit integrates again the residual and twice the residual and one message at the
// bound (−3.5 after the gain) are all too small. They are probed below as written, and
// the probe is the record.
//
// What the engine's rule gives instead, derived and not searched. *The offset* is
// `REFRACTORY_TICKS + 1`, 201: the cancel lands on tick 202, the first tick a unit that
// fired on tick one integrates again. *The span*: the volley's spikes fall on several
// ticks, because every unit carries the drive's standing potential and the drive adds the
// same 4.375 to each, so one tick of cancel serves the units freed on that tick and no
// other; the cancel is one message per tick over as many consecutive ticks as the volley
// spreads, read from the census of ADR-0072's composition run (the calibration's, F-46's
// stimulus, no cancel; `VOLLEY_TICKS_1024`, the presented set's volley spikes by their
// tick after the trial's first): ticks 1 to 9, so the span is 9 and the cancel lands on
// ticks 202 to 210, every unit freed on one of them receiving it on that tick and on every
// later tick of the span. A cancel that lands after a unit's first free tick is dropped in
// its next window if it fired, and takes the basal compartment further below rest if it
// did not; ADR-0074's Context said what erring large costs — the stimulus units' own later
// firing, which is F-46's excess — and nothing else within the trial, the basal
// compartment returning to rest within a few thousand ticks of a trial of 16 384. *The
// size*, in messages at the bound per tick (`spike_message` clamps one message at −2.0,
// −3.5 after the gain, so a larger cancel is more messages): (i) the least count that
// leaves a unit at rest firing once, by the oracle, 3 (−10.5 after the gain; 2 leaves it
// its second spike); (ii) the least count that leaves a unit at the most standing
// potential it can carry without firing — the basal compartment one LSB below twice the
// threshold and the soma one LSB below it — firing once, by the oracle, 6, which is twice
// (i); (iii) twice (ii), 12, for the standing states the oracle does not model (a
// synapse's message landing on the same tick), the candidate that cannot under-cancel and
// so tests whether the mechanism works at all, apart from its size. In this order and no
// other; each probed on a unit at rest through the task before any run (`probe_task`,
// `PROBED_CANCEL_1024`, in the gate, and held to the oracle); then the pick, ADR-0074's
// measure unchanged (`fires_once`: the volley one spike per unit within two before the
// readout window opens, at most a tenth of a spike per unit in the pair window after it),
// over one frozen run of sixty-four trials at 1 024 units and the gain 1.75, the
// calibration's run with the cancel the one difference, the modulation baseline at zero and
// no reward, so no weight moves; the first candidate passing the probe and both clauses is
// the stimulus; none, and there is no rewarded run. The sight (ADR-0065's) and the sign
// (ADR-0072's) are read from the same run; the criterion runs only when both pass.
//
// The expectations, written before the run. *The brief's*: the volley's potentiation stays
// near ADR-0072's 17.2 per synapse per presentation, because the first injection is F-46's
// and the response it evokes is F-46's; the volley's depression falls toward zero, because
// the stimulus units do not fire again to pair the response as depression; and the terms
// reach ADR-0072's +5 per synapse per trial, named as the Hypothesis it is. *This round's,
// from the pair rule*: the rule enters a potentiation at the presynaptic spike for the
// target's last spike only, and the spike at which the response was entered under F-46's
// stimulus was the stimulus unit's own spike at the end of its window, 201 ticks after the
// volley; with that spike gone the response is entered at the unit's next spike, its next
// presentation two trials later on average or a background spike before it, and only where
// no background spike of the readout unit (0.28 per trial) displaced the response as its
// last spike. So the volley's potentiation falls below 17.2 by about the displaced share
// (a Hypothesis: to about 10), and part of it moves into later trials' rows; the volley's
// depression falls toward zero; the background's depression falls to about a third of 26.8,
// one presynaptic spike per presentation where there were three; the background's
// potentiation stays near 7.5; the terms sum net positive, between +5 and +10; the sign
// turns positive once the standing trace has integrated a few trials of that flow; and the
// sight stays at 62 or 63, the readouts' response being F-46's.

/// The cancel's message: the most negative efficacy one message carries, −2.0
/// (`spike_message`'s clamp, held below), −3.5 after the gain of 1.75.
const CANCEL_MESSAGE_Q16: i32 = -0x0002_0000;
/// The brief's offset: `REFRACTORY_TICKS` from the trial's first tick, landing on tick
/// 201, inside the window of every unit that fired in the volley.
const BRIEF_OFFSET: u32 = REFRACTORY_TICKS as u32;
/// The cancel's offset as the engine's rule gives it: `REFRACTORY_TICKS + 1`, landing on
/// tick 202, the first tick a unit that fired on tick one integrates again.
const CANCEL_OFFSET: u32 = REFRACTORY_TICKS as u32 + 1;
const _: () = assert!(BRIEF_OFFSET == 200 && CANCEL_OFFSET == 201);
/// The presented set's volley spikes by their tick after the trial's first, over the
/// sixty-four trials of ADR-0072's composition run (F-46's stimulus, no cancel; 3 253 of
/// 64 × 51), as `(tick, units)`; every other tick before the readout window opens holds
/// none. Pinned from that run, which `the_composition_at_1024_units_exhaustive` holds.
const VOLLEY_TICKS_1024: [(u32, u64); 9] = [
    (1, 142),
    (2, 944),
    (3, 1613),
    (4, 484),
    (5, 49),
    (6, 12),
    (7, 4),
    (8, 4),
    (9, 1),
];
/// The cancel's span in ticks, derived from `VOLLEY_TICKS_1024` by `span_of` and not
/// chosen: the last tick a volley spike fell on, so that the cancel lands on the first free
/// tick of every unit of the volley. The gate holds the constant to the rule.
const CANCEL_TICKS: u32 = 9;
/// The most messages the oracle scans to for a least count.
const LEAST_SCAN: u32 = 32;
/// The most standing potential a unit carries without firing, for the oracle: the basal
/// compartment one LSB below twice the threshold (the soma's fixed point is half of it)
/// and the soma one LSB below the threshold.
const EXTREME_STANDING: (i32, i32) = (2 * THRESHOLD_BASE - 1, THRESHOLD_BASE - 1);
/// Candidate (i): the least count of messages at the bound per tick that leaves a unit at
/// rest firing once, by the oracle.
const CANCEL_ONCE_AT_REST: u32 = 3;
/// Candidate (ii): the least count that leaves a unit at `EXTREME_STANDING` firing once, by
/// the oracle.
const CANCEL_AT_THE_EXTREME: u32 = 6;
/// The candidates, as counts of messages at the bound per tick, in the order tried and no
/// other: (i), (ii) and twice (ii).
const CANCELS: [u32; 3] = [
    CANCEL_ONCE_AT_REST,
    CANCEL_AT_THE_EXTREME,
    2 * CANCEL_AT_THE_EXTREME,
];
const _: () = assert!(CANCELS[0] < CANCELS[1] && CANCELS[1] < CANCELS[2]);

/// The cancel of `messages` messages at the bound per tick over the derived span from the
/// derived offset.
const fn cancel_of(messages: u32) -> Cancel {
    Cancel {
        offset: CANCEL_OFFSET,
        ticks: CANCEL_TICKS,
        messages,
        efficacy_q16: CANCEL_MESSAGE_Q16,
    }
}

/// The span the census gives: the last tick a volley spike fell on; zero for no spike.
fn span_of(volley_ticks: &[(u32, u64)]) -> u32 {
    volley_ticks
        .iter()
        .filter(|&&(_, units)| units > 0)
        .map(|&(tick, _)| tick)
        .max()
        .unwrap_or(0)
}

/// The nonzero entries of a run's volley-tick census, as the constant holds them.
fn census_of(volley_ticks: &[u64]) -> Vec<(u32, u64)> {
    volley_ticks
        .iter()
        .enumerate()
        .filter(|&(_, &units)| units > 0)
        .map(|(tick, &units)| (tick as u32, units))
        .collect()
}

// ------------------------------------------------------------------------- the oracle

/// A turn's sum under the tick's gain, as the executor scales it (`scaled` in
/// `executor.rs`, ADR-0036; the injected messages among the sum, F-47): `sum × gain`,
/// Q16.16, rounded to nearest and clamped to the width; written a second time as the
/// oracle's.
fn scaled_q16(sum: i32, gain_q16: u32) -> i32 {
    (i64::from(sum)
        .saturating_mul(i64::from(gain_q16))
        .saturating_add(0x8000)
        >> 16)
        .clamp(i64::from(i32::MIN), i64::from(i32::MAX)) as i32
}

/// What `messages` messages of `efficacy_q16` sum to in a unit's batch: each clamped as
/// `spike_message` clamps it, then summed, clamped to the width.
fn batch_q16(messages: u32, efficacy_q16: i32) -> i32 {
    i64::from(message_efficacy_q16(spike_message(efficacy_q16, false)))
        .saturating_mul(i64::from(messages))
        .clamp(i64::from(i32::MIN), i64::from(i32::MAX)) as i32
}

/// The membrane rule stepped alone: `cortex-core`'s `integrate` on one unit at the base
/// threshold with no synapse and no drive, from `standing` basal and somatic potentials,
/// under `shape`'s messages landing on tick one and `cancel`'s on the tick after each
/// offset it is due at (the executor's timing: a message injected before trial tick `k`
/// lands on tick `k + 1`), each tick's batch scaled by `gain` as the executor scales it.
/// Returns the ticks the unit fires on within `ticks`, and its basal potential after every
/// tick.
fn alone(
    standing: (i32, i32),
    shape: Shape,
    cancel: Option<Cancel>,
    gain: u32,
    ticks: u32,
) -> (Vec<u32>, Vec<i32>) {
    let mut unit = DendriticSuperNeuron::new(0);
    unit.v_thresh = THRESHOLD_BASE;
    unit.v_basal = standing.0;
    unit.v_soma = standing.1;
    let mut fires = Vec::new();
    let mut basal = Vec::with_capacity(ticks as usize);
    for k in 0..ticks {
        let mut sum = if k == 0 {
            batch_q16(shape.0, shape.1)
        } else {
            0
        };
        if let Some(c) = cancel {
            if c.is_due(k) {
                sum = sum.saturating_add(batch_q16(c.messages, c.efficacy_q16));
            }
        }
        let now = k.saturating_add(1);
        if unit.integrate(scaled_q16(sum, gain), 0, now) {
            fires.push(now);
        }
        basal.push(unit.v_basal);
    }
    (fires, basal)
}

/// The residual: the basal potential, after the gain, that `shape` alone leaves in a unit
/// at rest after trial tick `tick`, by the oracle; zero for a tick before the first.
fn residual(shape: Shape, gain: u32, tick: u32) -> i32 {
    let (_, basal) = alone((0, 0), shape, None, gain, tick);
    basal.last().copied().unwrap_or(0)
}

/// The least count of messages at the bound per tick, as one cancel over the derived span
/// from the derived offset, that leaves a unit with `standing` potentials firing exactly
/// once within one pair window of F-46's drive, by the oracle; none up to `LEAST_SCAN`.
fn least_cancel(standing: (i32, i32), gain: u32) -> Option<u32> {
    (1..=LEAST_SCAN).find(|&n| {
        alone(standing, SHAPE_F46, Some(cancel_of(n)), gain, PROBE_TICKS)
            .0
            .len()
            == 1
    })
}

/// The brief's three candidates as it wrote them, one injection each at `offset` over
/// `ticks`: (i) the residual, one message carrying what F-46's drive leaves in the basal
/// compartment of a unit at rest after tick `residual_tick` divided by the gain; (ii) twice
/// it, two such messages; (iii) the bound, one message at −2.0.
fn brief_cancels(offset: u32, ticks: u32, residual_tick: u32) -> [Cancel; 3] {
    let after_gain = i64::from(residual(SHAPE_F46, GAIN_1024, residual_tick));
    let before_gain = (after_gain << 16)
        .checked_div(i64::from(GAIN_1024))
        .unwrap_or(0);
    let of = |messages: u32, efficacy_q16: i64| Cancel {
        offset,
        ticks,
        messages,
        efficacy_q16: efficacy_q16.clamp(i64::from(i32::MIN), i64::from(i32::MAX)) as i32,
    };
    [
        of(1, before_gain.saturating_neg()),
        of(2, before_gain.saturating_neg()),
        of(1, i64::from(CANCEL_MESSAGE_Q16)),
    ]
}

/// `probe` through the task (the composition the runs use, on one unit): the instrument's
/// network at 1 024 units at rest at the gain 1.75 with no drive, a task whose stimulus A
/// is unit 0 alone with `shape` and `cancel`, one trial of `PROBE_TICKS` ticks, and the
/// ticks after the trial's first on which unit 0 fires, read from the train.
fn probe_task(shape: Shape, cancel: Option<Cancel>) -> Vec<u32> {
    let p = prior(1024);
    let mut exec = at_gain(&p, config(1024, 1, 0), GAIN_1024);
    assert!(exec.is_quiescent());
    let stimulus = |first| Stimulus {
        set: Set::contiguous(first, 1),
        messages: shape.0,
        efficacy_q16: shape.1,
        cancel,
    };
    let mut t = Task {
        stimuli: [stimulus(0), stimulus(1)],
        readout: Readout::new([Set::contiguous(2, 1), Set::contiguous(3, 1)]),
        drive: Drive {
            every: 0,
            messages: 0,
            efficacy_q16: 0,
            units: 1024,
            seed: 0,
        },
        ticks: PROBE_TICKS,
        window: Window::whole(PROBE_TICKS),
        seed: SEED,
        reward_q16: 0,
        mirrored: false,
        feedback: Feedback::Withheld,
        delivery: Delivery::Global,
    };
    let trial = (0..8u64)
        .find(|&k| t.stimulus_at(k) == 0)
        .expect("a trial presents A");
    let start = exec.ticks() as u32;
    t.trial(&mut exec, trial).expect("the probe runs");
    exec.train()
        .iter()
        .filter(|&&(_, unit)| unit == 0)
        .map(|&(tick, _)| tick.wrapping_sub(start))
        .collect()
}

/// The cancel the round picks: the first candidate, in the candidates' order, whose probe
/// passed (`probed[k]`: a unit at rest fires once) and whose frozen run fires once by
/// ADR-0074's measure; none when none does.
fn cancel_pick(probed: &[bool], runs: &[(Block, u64, Composition)]) -> Option<u32> {
    CANCELS
        .iter()
        .zip(probed.iter())
        .zip(runs.iter())
        .find(|&((_, &probed), (block, _, composition))| {
            probed && fires_once(1024, block, composition)
        })
        .map(|((&messages, _), _)| messages)
}

// ------------------------------------------------------------- the probes (brief 034)

/// The probes, each F-46's drive into unit 0 of the instrument's network at rest at the
/// gain 1.75 with a cancel, through the task: the ticks unit 0 fires on within one pair
/// window, pinned from the engine and held to the oracle. The brief's three at the brief's
/// offset land inside the window and are dropped; at the derived span they land on tick 206
/// for a unit at rest and are too small; two messages at the bound are one below the least;
/// the three candidates leave the unit its one spike.
const PROBED_CANCEL_1024: [(&str, Option<Cancel>, &[u32]); 11] = [
    ("F-46's drive alone", None, &[5, 206]),
    (
        "the brief's (i), the residual, at the brief's offset",
        Some(Cancel {
            offset: BRIEF_OFFSET,
            ticks: 1,
            messages: 1,
            efficacy_q16: -111_082,
        }),
        &[5, 206],
    ),
    (
        "the brief's (ii), twice the residual, at the brief's offset",
        Some(Cancel {
            offset: BRIEF_OFFSET,
            ticks: 1,
            messages: 2,
            efficacy_q16: -111_082,
        }),
        &[5, 206],
    ),
    (
        "the brief's (iii), the bound, at the brief's offset",
        Some(Cancel {
            offset: BRIEF_OFFSET,
            ticks: 1,
            messages: 1,
            efficacy_q16: CANCEL_MESSAGE_Q16,
        }),
        &[5, 206],
    ),
    (
        "the brief's (i) at the derived span",
        Some(Cancel {
            offset: CANCEL_OFFSET,
            ticks: CANCEL_TICKS,
            messages: 1,
            efficacy_q16: -110_004,
        }),
        &[5, 206],
    ),
    (
        "the brief's (ii) at the derived span",
        Some(Cancel {
            offset: CANCEL_OFFSET,
            ticks: CANCEL_TICKS,
            messages: 2,
            efficacy_q16: -110_004,
        }),
        &[5, 206],
    ),
    (
        "the brief's (iii) at the derived span",
        Some(Cancel {
            offset: CANCEL_OFFSET,
            ticks: CANCEL_TICKS,
            messages: 1,
            efficacy_q16: CANCEL_MESSAGE_Q16,
        }),
        &[5, 206],
    ),
    (
        "two at the bound, one below the least",
        Some(cancel_of(2)),
        &[5, 206],
    ),
    ("(i) three at the bound", Some(cancel_of(CANCELS[0])), &[5]),
    ("(ii) six at the bound", Some(cancel_of(CANCELS[1])), &[5]),
    (
        "(iii) twelve at the bound",
        Some(cancel_of(CANCELS[2])),
        &[5],
    ),
];
/// The residual after the gain on tick 200 and on tick 205, by the oracle: what stands
/// when the brief's cancel would land on tick 201, and when the derived span's message
/// lands on tick 206 for a unit at rest.
const RESIDUAL_200_Q16: i32 = 194_395;
const RESIDUAL_205_Q16: i32 = 192_507;

// ---------------------------------------------------------- the measurement (brief 034)

/// The gate's test (ADR-0061's class): the message's clamp, the oracle's scaling at its
/// edges, the oracle on a unit at rest and at the extreme held to the constants (the
/// residuals, the least counts), the span rule over the pinned census, the brief's three
/// cancels as the oracle sizes them, every probe through the task held to its table and to
/// the oracle, the pick rule at its edges and over the pinned candidates as written, both
/// calibrations and Deliverable D's readings over the pinned tables, and the first eight
/// trials of the picked candidate, run and held to the first eight rows and counts of its
/// tables; no number is pinned twice.
#[test]
fn the_first_eight_trials_of_the_two_injections_at_1024_units_and_the_rules_over_their_tables() {
    // One message carries at most −2.0: the clamp is the bound.
    assert_eq!(
        message_efficacy_q16(spike_message(CANCEL_MESSAGE_Q16, false)),
        CANCEL_MESSAGE_Q16
    );
    assert_eq!(
        message_efficacy_q16(spike_message(i32::MIN, false)),
        CANCEL_MESSAGE_Q16,
        "below the bound, clamped"
    );
    assert_eq!(batch_q16(3, i32::MIN), 3 * CANCEL_MESSAGE_Q16);
    assert_eq!(batch_q16(2, STIMULUS_Q16), 2 * STIMULUS_Q16);
    // The scaling: exact at 1.0, F-46's 4.375 at 1.75, the bound's −3.5, the width held.
    assert_eq!(scaled_q16(2 * STIMULUS_Q16, ONE as u32), 2 * STIMULUS_Q16);
    assert_eq!(
        scaled_q16(2 * STIMULUS_Q16, GAIN_1024),
        0x0004_6000,
        "4.375"
    );
    assert_eq!(
        scaled_q16(CANCEL_MESSAGE_Q16, GAIN_1024),
        -0x0003_8000,
        "−3.5"
    );
    assert_eq!(scaled_q16(i32::MAX, GAIN_1024), i32::MAX);
    assert_eq!(scaled_q16(i32::MIN, GAIN_1024), i32::MIN);
    assert_eq!(scaled_q16(0, GAIN_1024), 0);
    // The oracle on a unit at rest: ADR-0074's probe, the residuals, the least count.
    let (fires, _) = alone((0, 0), SHAPE_F46, None, GAIN_1024, PROBE_TICKS);
    assert_eq!(fires, [5, 206], "F-46's drive fires a unit at rest twice");
    assert_eq!(residual(SHAPE_F46, GAIN_1024, 200), RESIDUAL_200_Q16);
    assert_eq!(residual(SHAPE_F46, GAIN_1024, 205), RESIDUAL_205_Q16);
    assert_eq!(
        residual(SHAPE_F46, GAIN_1024, 0),
        0,
        "before the drive lands"
    );
    assert_eq!(least_cancel((0, 0), GAIN_1024), Some(CANCEL_ONCE_AT_REST));
    assert_eq!(
        least_cancel(EXTREME_STANDING, GAIN_1024),
        Some(CANCEL_AT_THE_EXTREME)
    );
    assert_eq!(
        alone(
            (0, 0),
            SHAPE_F46,
            Some(cancel_of(2)),
            GAIN_1024,
            PROBE_TICKS
        )
        .0,
        [5, 206],
        "two at the bound leave the second spike"
    );
    let (extreme, _) = alone(EXTREME_STANDING, SHAPE_F46, None, GAIN_1024, PROBE_TICKS);
    assert_eq!(
        extreme,
        [1, 202, 403],
        "at the extreme the unit fires on the drive's tick"
    );
    assert_eq!(
        alone(
            EXTREME_STANDING,
            SHAPE_F46,
            Some(cancel_of(5)),
            GAIN_1024,
            PROBE_TICKS
        )
        .0,
        [1, 202],
        "five leave the extreme its second spike"
    );
    // A cancel that lands inside the window is dropped whatever its size.
    for offset in [BRIEF_OFFSET, 202, 204] {
        let inside = Cancel {
            offset,
            ticks: 1,
            messages: LEAST_SCAN,
            efficacy_q16: CANCEL_MESSAGE_Q16,
        };
        assert_eq!(
            alone((0, 0), SHAPE_F46, Some(inside), GAIN_1024, PROBE_TICKS).0,
            [5, 206],
            "offset {offset}: inside the window"
        );
    }
    // The span rule over the pinned census, and the census's sum.
    assert_eq!(
        span_of(&VOLLEY_TICKS_1024),
        CANCEL_TICKS,
        "the span as derived"
    );
    assert_eq!(span_of(&[]), 0);
    assert_eq!(
        span_of(&[(3, 0), (2, 1)]),
        2,
        "a tick of no unit does not count"
    );
    assert_eq!(
        VOLLEY_TICKS_1024
            .iter()
            .map(|&(_, units)| units)
            .sum::<u64>(),
        3253
    );
    let mut census = vec![0u64; WINDOW.from as usize];
    for &(tick, units) in &VOLLEY_TICKS_1024 {
        census[tick as usize] = units;
    }
    assert_eq!(census_of(&census), VOLLEY_TICKS_1024.to_vec());
    assert_eq!(cancel_of(3).end(), 210, "lands on 202 to 210");
    // The brief's three cancels as the oracle sizes them.
    assert_eq!(
        brief_cancels(BRIEF_OFFSET, 1, 200).map(|c| (c.messages, c.efficacy_q16)),
        [(1, -111_082), (2, -111_082), (1, CANCEL_MESSAGE_Q16)],
        "the residual on tick 200, 2.966 after the gain, is 1.695 before it"
    );
    assert_eq!(
        brief_cancels(CANCEL_OFFSET, CANCEL_TICKS, 205).map(|c| (c.messages, c.efficacy_q16)),
        [(1, -110_004), (2, -110_004), (1, CANCEL_MESSAGE_Q16)]
    );
    // The probes through the task, each held to its table and to the oracle.
    for (name, cancel, ticks) in PROBED_CANCEL_1024 {
        let read = probe_task(SHAPE_F46, cancel);
        eprintln!("DUMP probe1024 cancel {name}: {cancel:?} -> {read:?}");
        assert_eq!(read.as_slice(), ticks, "{name}");
        assert_eq!(
            alone((0, 0), SHAPE_F46, cancel, GAIN_1024, PROBE_TICKS).0,
            ticks,
            "{name}: the oracle"
        );
    }
    assert_eq!(
        probe_task(SHAPE_F46, None).as_slice(),
        PROBED_1024[0].1,
        "the task's probe is the injector's (ADR-0074)"
    );
    // The pick at its edges: the first candidate whose probe passed and whose run fires
    // once.
    let seen = CALIBRATION_1024[0].0;
    let after = |spikes: u64| -> Composition {
        (
            [[0; 2]; 2],
            [[[0; 4]; 2]; 2],
            [[0, 0], [0, 0], [0, spikes]],
            0,
            0,
        )
    };
    let passing = (seen, 0u64, after(0));
    let loud = (seen, 0u64, after(327));
    assert_eq!(
        cancel_pick(&[true, true, true], &[passing, passing, passing]),
        Some(CANCELS[0])
    );
    assert_eq!(
        cancel_pick(&[false, true, true], &[passing, passing, passing]),
        Some(CANCELS[1]),
        "a failed probe is skipped"
    );
    assert_eq!(
        cancel_pick(&[true, true, true], &[loud, loud, passing]),
        Some(CANCELS[2])
    );
    assert_eq!(cancel_pick(&[true, true, true], &[loud, loud, loud]), None);
    assert_eq!(
        cancel_pick(&[false, false, false], &[passing, passing, passing]),
        None
    );
    assert_eq!(cancel_pick(&[], &[]), None);
    // Over the pinned tables: the candidates' readings as the constants say, the pick as
    // written, the calibrations, and Deliverable D's readings.
    let probed: Vec<bool> = CANCELS
        .iter()
        .map(|&messages| {
            PROBED_CANCEL_1024
                .iter()
                .any(|&(_, cancel, ticks)| cancel == Some(cancel_of(messages)) && ticks.len() == 1)
        })
        .collect();
    assert_eq!(probed, [true, true, true], "every candidate's probe passed");
    let [a, _, _, _] = geometry(1024, ROTATION_1024);
    for (k, (block, _, composed)) in CANCELLED_1024.iter().enumerate() {
        let rows = &CANCELLED_ROWS_1024[k];
        let counted = &CANCELLED_COUNTED_1024[k];
        let signs = rows.iter().filter(|r| r.1 > 0).count() as u32;
        assert_eq!(signs, composed.3, "candidate {k}: the signs are the rows'");
        assert_eq!(
            counted.iter().filter(|c| c.0 == 0).count() as u32,
            block.1,
            "candidate {k}: the A trials"
        );
        assert_eq!(
            CANCELLED_CENSUS_1024[k]
                .iter()
                .map(|&(_, units)| units)
                .sum::<u64>(),
            block.3[0].saturating_add(block.3[1]),
            "candidate {k}: the census is the volley"
        );
        assert_eq!(span_of(CANCELLED_CENSUS_1024[k]), CANCEL_TICKS);
        eprintln!(
            "DUMP cancelled1024 {k} {} volley {:?} after {} per unit {} tenths seen {} signs {} fires_once {} passes {} bias {:?} offset {:?}",
            CANCELS[k],
            block.3,
            composed.2[2][1],
            composed.2[2][1] * 10 / (64 * a.len()),
            block.6,
            composed.3,
            fires_once(1024, block, composed),
            passes(block, composed),
            bias(counted, false),
            offset(counted, false, OFFSET_MARK_64)
        );
        assert!(calibrated(block), "candidate {k}: the sight passes");
        assert!(composed.3 < SIGN_MIN, "candidate {k}: the sign fails");
        assert!(
            volley_once(1024, block),
            "candidate {k}: the volley is whole"
        );
    }
    assert!(
        !after_quiet(1024, &CANCELLED_1024[0].2),
        "(i) leaves the units the least count at rest does not cover their second spike"
    );
    assert_eq!(
        CANCELLED_1024[1].2.2[2][1], 0,
        "(ii): no spike after the opening"
    );
    assert_eq!(CANCELLED_1024[2].2.2[2][1], 0, "(iii): none");
    assert_eq!(
        cancel_pick(&probed, &CANCELLED_1024),
        CANCEL_PICKED_1024,
        "the pick as written"
    );
    let picked = CANCEL_PICKED_1024.expect("a candidate is picked");
    let k = CANCELS.iter().position(|&n| n == picked).unwrap();
    assert_eq!(k, 1, "candidate (ii)");
    assert_eq!(
        passes(&CANCELLED_1024[k].0, &CANCELLED_1024[k].2),
        CANCEL_CALIBRATED_1024,
        "both calibrations under the picked cancel, as written"
    );
    assert_eq!((CANCELLED_1024[k].0.6, CANCELLED_1024[k].2.3), (62, 13));
    assert_eq!(
        [
            offset(&CANCELLED_COUNTED_1024[0], false, OFFSET_MARK_64),
            offset(&CANCELLED_COUNTED_1024[1], false, OFFSET_MARK_64),
            offset(&CANCELLED_COUNTED_1024[2], false, OFFSET_MARK_64),
        ],
        [Some(2), Some(1), Some(1)],
        "the offset the criterion needs over the candidates' calibrations"
    );
    assert_eq!(
        bias(&CANCELLED_COUNTED_1024[k], false),
        [(6, 34), (3, 30)],
        "the picked cancel's bias, toward the answer on both stimuli"
    );
    // The first eight trials of the picked candidate, run.
    let (blocks, trace, trials, counts, counted, _) = compose_shaped(
        SHAPE_F46,
        Some(cancel_of(picked)),
        1024,
        GAIN_1024,
        GATE_TRIALS,
    );
    dump_composition("cancelled1024 1 first eight", &blocks, trace, &trials);
    assert_eq!(counts, SYNAPSES_1024);
    assert!(blocks.is_empty(), "no whole block");
    pinned_composition(
        "cancelled1024 1 first eight",
        &trials,
        &CANCELLED_ROWS_1024[k][..GATE_TRIALS],
        None,
    );
    assert_eq!(
        counted.as_slice(),
        &CANCELLED_COUNTED_1024[k][..GATE_TRIALS]
    );
}

/// The candidates at 1 024 units, in the candidates' order, each a frozen run of sixty-four
/// trials at the present gain with F-46's stimulus and its cancel, read by the sight, the
/// sign and the measure that picks: the sight's block and trace, and the composition. Pinned
/// from one run each.
const CANCELLED_1024: [(Block, u64, Composition); 3] = [
    (
        (
            35,
            34,
            [[414, 400], [413, 422]],
            [1727, 1526],
            [4047, 3601],
            [258, 234],
            63,
            213902976,
            235822619,
            0,
            [[6986739, 7212710], [7146878, 7257384]],
            3,
        ),
        0xa64f571232e8df09,
        (
            [[-11702, -23587], [-15667, -12495]],
            [
                [
                    [338616, -36496, 356813, -936028],
                    [344026, -40337, 398293, -930256],
                ],
                [
                    [306535, -30943, 329496, -817392],
                    [301502, -27026, 372513, -817929],
                ],
            ],
            [[1046, 2146], [1044, 2190], [3356, 2717]],
            2,
            14119637557841610480,
        ),
    ),
    (
        (
            34,
            34,
            [[321, 315], [307, 310]],
            [1726, 1525],
            [2549, 2293],
            [262, 239],
            62,
            213902976,
            235822619,
            0,
            [[6986739, 7212710], [7146878, 7257384]],
            6,
        ),
        0xeeb4acda1476b4c5,
        (
            [[-6937, -10356], [-2789, 1625]],
            [
                [
                    [211499, -6013, 326747, -631916],
                    [223967, -6680, 362378, -608124],
                ],
                [
                    [192145, -3500, 295516, -553067],
                    [204182, -5113, 332692, -560717],
                ],
            ],
            [[1071, 1638], [1069, 1679], [3359, 0]],
            13,
            7428265973369681826,
        ),
    ),
    (
        (
            34,
            34,
            [[320, 315], [307, 311]],
            [1726, 1525],
            [2532, 2285],
            [262, 239],
            62,
            213902976,
            235822619,
            0,
            [[6986739, 7212710], [7146878, 7257384]],
            6,
        ),
        0xeeb4acda1476b4c5,
        (
            [[-6551, -10787], [-2591, 1724]],
            [
                [
                    [210722, -4730, 320551, -624435],
                    [223647, -4574, 357606, -602920],
                ],
                [
                    [192084, -2943, 292620, -552306],
                    [203781, -4060, 337358, -565371],
                ],
            ],
            [[1072, 1636], [1071, 1679], [3359, 0]],
            13,
            268213348253458448,
        ),
    ),
];
/// The rows of every candidate's sixty-four trials, in the candidates' order.
const CANCELLED_ROWS_1024: [[Row; BLOCK]; 3] = [
    [
        (0, -568, [12182, -768, 7944, -19895]),
        (0, -15067, [17670, -2543, 28054, -58764]),
        (1, -48593, [8113, -2519, 25385, -60310]),
        (0, -18340, [18891, -3411, 27630, -48722]),
        (0, -9508, [24035, -1568, 21122, -39006]),
        (0, -21635, [19043, -1404, 12973, -44990]),
        (0, -29976, [14379, -1543, 11148, -37162]),
        (0, -36213, [9733, -724, 15943, -39361]),
        (0, -38963, [13515, -1795, 15829, -36533]),
        (0, -53322, [18573, -2340, 23590, -63897]),
        (1, -14748, [18476, -3437, 24670, -34373]),
        (1, 1056, [34403, -9810, 17788, -30895]),
        (0, -50552, [15388, -3554, 22683, -52031]),
        (0, -23802, [37429, -2787, 27509, -45369]),
        (1, -21220, [10167, -2249, 16799, -40166]),
        (0, -21781, [15995, -4978, 15752, -35781]),
        (1, -12202, [17572, -787, 20953, -37924]),
        (0, -43949, [17667, -541, 23102, -66758]),
        (0, -57467, [20805, -1185, 13740, -56140]),
        (0, -71554, [16007, -2248, 13720, -53750]),
        (1, -10066, [7503, -193, 22354, -30743]),
        (0, -62813, [11754, -124, 16002, -46342]),
        (1, -8163, [21612, -1501, 20505, -53527]),
        (1, 325, [25571, -186, 20160, -39134]),
        (1, -27536, [17354, -2959, 20592, -61492]),
        (0, -44165, [7105, -1076, 16246, -38400]),
        (1, -36062, [10823, -1603, 15642, -46270]),
        (1, -42478, [18475, -781, 11812, -44162]),
        (1, -42379, [13030, -1609, 15164, -36422]),
        (0, -46672, [18906, -4313, 18490, -56356]),
        (0, -34985, [24682, -1002, 18099, -40836]),
        (0, -38668, [19237, -4259, 15233, -41678]),
        (0, -36378, [22972, -2877, 13760, -41916]),
        (0, -47866, [16916, -1305, 12825, -47905]),
        (1, -24830, [8693, -1693, 14041, -31805]),
        (1, -27229, [28136, -2348, 14398, -49692]),
        (1, -17911, [27556, -426, 16728, -39476]),
        (0, -49106, [7480, -2661, 20727, -48578]),
        (1, -27877, [15735, -1880, 22396, -44509]),
        (1, -20207, [29297, -2677, 18780, -43701]),
        (0, -27239, [12245, -3892, 21700, -35111]),
        (1, -24149, [16027, -1465, 14910, -35721]),
        (0, -20808, [21772, -4125, 20111, -42346]),
        (1, -31278, [15691, -5980, 19678, -46325]),
        (0, -12231, [19303, -1764, 25713, -42994]),
        (1, -27709, [14670, -1063, 16366, -40048]),
        (0, -34163, [12423, -729, 17824, -47131]),
        (0, -27530, [20591, -3030, 21149, -39041]),
        (0, -29790, [19367, -3565, 13285, -37107]),
        (1, -33608, [10567, -3656, 29739, -53168]),
        (1, -35523, [23235, -3681, 19913, -48837]),
        (0, -42649, [7714, -457, 20781, -49921]),
        (1, -23678, [20601, -911, 18925, -41351]),
        (1, -26450, [16639, -874, 14494, -39191]),
        (0, -39424, [14336, -2045, 19456, -51934]),
        (1, -43802, [11065, -775, 14541, -50931]),
        (1, -42310, [24929, -234, 17709, -50282]),
        (1, -49195, [19417, -156, 15327, -51847]),
        (0, -40543, [13428, -2549, 18062, -45991]),
        (1, -40486, [13835, -1501, 17136, -43892]),
        (0, -31669, [25201, -4058, 22714, -49677]),
        (0, -34393, [17242, -1613, 15433, -42833]),
        (1, -31656, [9029, -886, 20035, -37121]),
        (1, -28162, [14939, -128, 14064, -32500]),
    ],
    [
        (0, -6963, [6892, -371, 6150, -19649]),
        (0, -3987, [11614, -468, 28988, -38786]),
        (1, -33007, [3582, -79, 19021, -33911]),
        (0, -3502, [11502, -360, 16615, -28525]),
        (0, 3265, [13804, -220, 14552, -22387]),
        (0, -2586, [13284, -518, 12147, -29407]),
        (0, -1546, [11967, -470, 10398, -21270]),
        (0, -3000, [11297, -450, 16869, -30257]),
        (0, -2593, [10414, -1370, 14433, -22384]),
        (0, -10731, [11644, -210, 21163, -41122]),
        (1, -9577, [7588, -275, 21225, -20671]),
        (1, 3425, [16442, -684, 16203, -20957]),
        (0, -12080, [7007, -338, 23176, -30568]),
        (0, 10764, [22015, -278, 23524, -25103]),
        (1, -10023, [5559, -352, 13888, -25911]),
        (0, 2781, [7749, -288, 15551, -27979]),
        (1, 571, [13000, -763, 20231, -29571]),
        (0, -12008, [13424, -445, 19413, -41303]),
        (0, -16552, [11927, -105, 11260, -30157]),
        (0, -18621, [14079, -71, 12829, -31933]),
        (1, -4987, [3643, -60, 17461, -20098]),
        (0, -14620, [10973, -77, 14442, -28376]),
        (1, -2373, [10231, -502, 19851, -33537]),
        (1, 8639, [15586, -37, 16235, -21811]),
        (1, 966, [13440, -72, 17564, -35318]),
        (0, -16383, [3752, -391, 17011, -23938]),
        (1, -6027, [12644, -228, 9103, -27845]),
        (1, -6734, [12541, -141, 11041, -25653]),
        (1, -10176, [7097, -48, 11674, -24011]),
        (0, -24464, [9604, -1311, 17512, -32589]),
        (0, -18648, [13128, -82, 16846, -29934]),
        (0, -15854, [10487, -126, 12812, -23812]),
        (0, -3860, [20893, -361, 13867, -26827]),
        (0, -301, [15429, -144, 13843, -25853]),
        (1, -12236, [3475, -565, 13812, -24105]),
        (1, -16367, [14637, -222, 9489, -31093]),
        (1, -1150, [23946, -13, 10430, -22049]),
        (0, -20913, [3037, -1022, 18739, -34024]),
        (1, -2476, [9764, -390, 26627, -28099]),
        (1, 5129, [19679, -671, 15146, -26857]),
        (0, -8226, [4253, -24, 20340, -22052]),
        (1, -3066, [10658, -496, 13102, -20436]),
        (0, -4401, [12909, -812, 16116, -25859]),
        (1, -1368, [7774, -258, 14156, -20628]),
        (0, -3169, [6680, -86, 20125, -29361]),
        (1, 723, [12373, -411, 15729, -25399]),
        (0, -14467, [7145, -162, 12994, -27216]),
        (0, 2299, [18344, -423, 19839, -23641]),
        (0, 6442, [14917, -10, 11962, -21811]),
        (1, -522, [8723, -431, 28367, -27680]),
        (1, -1468, [11603, -321, 15689, -28452]),
        (0, -8908, [5089, -179, 20427, -31225]),
        (1, 5190, [15363, -359, 17525, -26824]),
        (1, 3041, [13266, -458, 13980, -28909]),
        (0, -11583, [6309, -110, 20558, -29569]),
        (1, -15503, [9078, -14, 12544, -34100]),
        (1, -18260, [12935, -84, 13025, -31660]),
        (1, -16460, [15610, -58, 14680, -32215]),
        (0, -9627, [8651, -738, 20035, -26265]),
        (1, -8530, [10257, -244, 12699, -25761]),
        (0, -13671, [12389, -561, 17925, -30486]),
        (0, -12158, [12737, -112, 13148, -28440]),
        (1, -4647, [5586, -260, 21347, -24995]),
        (1, -1164, [10524, -116, 10675, -18246]),
    ],
    [
        (0, -6963, [6892, -371, 6150, -19649]),
        (0, -2217, [11545, -160, 28610, -36833]),
        (1, -33006, [3582, -79, 19021, -33910]),
        (0, -2948, [11350, -360, 16222, -28525]),
        (0, 3848, [13842, -217, 14626, -22358]),
        (0, -1801, [13284, -518, 12105, -29035]),
        (0, -1157, [11967, -470, 10121, -21270]),
        (0, -2723, [11297, -450, 16869, -30256]),
        (0, -1430, [8673, -145, 13906, -20368]),
        (0, -8373, [13139, -151, 19935, -39896]),
        (1, -9606, [7588, -275, 21226, -20677]),
        (1, 2497, [15530, -205, 18413, -23607]),
        (0, -12267, [7007, -338, 23250, -31898]),
        (0, 10393, [21592, -278, 23830, -25100]),
        (1, -9711, [5559, -352, 13948, -25907]),
        (0, 2636, [7749, -288, 15551, -27977]),
        (1, 743, [12999, -763, 20232, -29552]),
        (0, -12062, [13424, -445, 19413, -41301]),
        (0, -16625, [11927, -105, 11260, -30157]),
        (0, -18657, [14079, -71, 12829, -31933]),
        (1, -4902, [3643, -60, 17461, -20095]),
        (0, -14648, [10973, -77, 14444, -28376]),
        (1, -2318, [10231, -502, 19851, -33537]),
        (1, 8679, [15586, -37, 16235, -21811]),
        (1, 1001, [13440, -72, 17564, -35318]),
        (0, -15735, [3767, -344, 16169, -22433]),
        (1, -6006, [12644, -228, 9103, -27842]),
        (1, -6709, [12541, -141, 11044, -25653]),
        (1, -10160, [7097, -48, 11674, -24011]),
        (0, -24149, [9604, -1311, 17514, -32589]),
        (0, -18236, [13000, -36, 16817, -29700]),
        (0, -15517, [10615, -126, 12749, -23810]),
        (0, -1056, [20604, -124, 14253, -24293]),
        (0, -716, [15678, -143, 10376, -25766]),
        (1, -13647, [3475, -565, 13018, -24105]),
        (1, -17426, [14637, -219, 9489, -31091]),
        (1, -1915, [23948, -13, 10434, -22049]),
        (0, -18812, [2558, -291, 18499, -31818]),
        (1, -2232, [9764, -390, 27894, -28263]),
        (1, 5121, [19221, -490, 15145, -26905]),
        (0, -7602, [4254, -24, 20284, -22052]),
        (1, -3111, [10801, -198, 12534, -20245]),
        (0, -4239, [12381, -320, 16116, -25852]),
        (1, -1254, [7685, -50, 13564, -20005]),
        (0, -2994, [7208, -86, 19721, -29361]),
        (1, 1257, [12175, -107, 15592, -25040]),
        (0, -14331, [7145, -162, 12994, -27216]),
        (0, 2401, [18344, -423, 19839, -23641]),
        (0, 6520, [14917, -10, 11965, -21809]),
        (1, -173, [8859, -431, 28408, -27680]),
        (1, -1223, [11603, -321, 15689, -28450]),
        (0, -8861, [5089, -179, 20427, -31225]),
        (1, 5455, [15363, -359, 17525, -26822]),
        (1, 1795, [13266, -458, 12576, -28892]),
        (0, -12593, [6309, -110, 19522, -29569]),
        (1, -16251, [9078, -14, 12544, -34100]),
        (1, -18844, [12937, -84, 13025, -31660]),
        (1, -16918, [15610, -58, 14680, -32215]),
        (0, -9563, [8651, -737, 19807, -25554]),
        (1, -8690, [9741, -107, 12371, -24802]),
        (0, -13895, [12389, -322, 16302, -29495]),
        (0, -12390, [12737, -112, 13142, -28440]),
        (1, -4316, [5586, -260, 21347, -24994]),
        (1, -867, [10524, -116, 10676, -18245]),
    ],
];
/// Every trial's readout counts under each candidate, in the candidates' order.
const CANCELLED_COUNTED_1024: [[Counted; BLOCK]; 3] = [
    [
        (0, [10, 15]),
        (0, [14, 13]),
        (1, [28, 37]),
        (0, [15, 13]),
        (0, [6, 6]),
        (0, [5, 9]),
        (0, [6, 5]),
        (0, [10, 6]),
        (0, [12, 13]),
        (0, [4, 6]),
        (1, [21, 21]),
        (1, [11, 14]),
        (0, [18, 16]),
        (0, [15, 9]),
        (1, [18, 31]),
        (0, [14, 12]),
        (1, [16, 10]),
        (0, [5, 13]),
        (0, [10, 15]),
        (0, [13, 13]),
        (1, [22, 21]),
        (0, [11, 5]),
        (1, [15, 21]),
        (1, [6, 4]),
        (1, [8, 15]),
        (0, [11, 15]),
        (1, [9, 12]),
        (1, [12, 5]),
        (1, [11, 6]),
        (0, [21, 19]),
        (0, [10, 16]),
        (0, [10, 13]),
        (0, [16, 10]),
        (0, [9, 3]),
        (1, [22, 23]),
        (1, [21, 8]),
        (1, [14, 7]),
        (0, [22, 15]),
        (1, [15, 21]),
        (1, [7, 12]),
        (0, [17, 10]),
        (1, [17, 7]),
        (0, [17, 15]),
        (1, [16, 15]),
        (0, [13, 4]),
        (1, [13, 16]),
        (0, [8, 11]),
        (0, [8, 14]),
        (0, [10, 8]),
        (1, [11, 14]),
        (1, [9, 10]),
        (0, [12, 20]),
        (1, [12, 9]),
        (1, [8, 10]),
        (0, [16, 19]),
        (1, [15, 12]),
        (1, [10, 14]),
        (1, [9, 7]),
        (0, [17, 16]),
        (1, [12, 14]),
        (0, [14, 13]),
        (0, [15, 10]),
        (1, [13, 12]),
        (1, [12, 14]),
    ],
    [
        (0, [6, 9]),
        (0, [15, 12]),
        (1, [17, 22]),
        (0, [10, 7]),
        (0, [4, 5]),
        (0, [5, 9]),
        (0, [6, 4]),
        (0, [9, 4]),
        (0, [11, 13]),
        (0, [8, 5]),
        (1, [13, 15]),
        (1, [10, 12]),
        (0, [14, 11]),
        (0, [13, 11]),
        (1, [12, 25]),
        (0, [9, 9]),
        (1, [8, 9]),
        (0, [5, 13]),
        (0, [10, 14]),
        (0, [12, 11]),
        (1, [13, 9]),
        (0, [8, 5]),
        (1, [9, 14]),
        (1, [5, 5]),
        (1, [9, 12]),
        (0, [5, 6]),
        (1, [7, 9]),
        (1, [9, 4]),
        (1, [11, 7]),
        (0, [13, 10]),
        (0, [9, 13]),
        (0, [8, 13]),
        (0, [16, 9]),
        (0, [6, 2]),
        (1, [10, 12]),
        (1, [20, 11]),
        (1, [9, 6]),
        (0, [10, 9]),
        (1, [13, 13]),
        (1, [4, 12]),
        (0, [12, 7]),
        (1, [12, 5]),
        (0, [13, 12]),
        (1, [13, 11]),
        (0, [9, 4]),
        (1, [12, 7]),
        (0, [8, 8]),
        (0, [8, 17]),
        (0, [8, 6]),
        (1, [8, 9]),
        (1, [7, 9]),
        (0, [6, 12]),
        (1, [11, 7]),
        (1, [6, 9]),
        (0, [11, 15]),
        (1, [9, 13]),
        (1, [10, 10]),
        (1, [9, 5]),
        (0, [10, 11]),
        (1, [13, 13]),
        (0, [10, 9]),
        (0, [14, 10]),
        (1, [10, 6]),
        (1, [8, 9]),
    ],
    [
        (0, [6, 9]),
        (0, [15, 12]),
        (1, [17, 22]),
        (0, [10, 7]),
        (0, [3, 5]),
        (0, [5, 9]),
        (0, [6, 4]),
        (0, [9, 4]),
        (0, [11, 13]),
        (0, [8, 5]),
        (1, [13, 15]),
        (1, [10, 12]),
        (0, [14, 11]),
        (0, [13, 11]),
        (1, [12, 25]),
        (0, [9, 9]),
        (1, [8, 9]),
        (0, [5, 13]),
        (0, [10, 14]),
        (0, [12, 11]),
        (1, [13, 9]),
        (0, [8, 5]),
        (1, [9, 14]),
        (1, [5, 5]),
        (1, [9, 12]),
        (0, [5, 6]),
        (1, [7, 9]),
        (1, [9, 4]),
        (1, [11, 7]),
        (0, [13, 10]),
        (0, [9, 13]),
        (0, [8, 13]),
        (0, [16, 9]),
        (0, [6, 2]),
        (1, [10, 12]),
        (1, [20, 11]),
        (1, [9, 6]),
        (0, [10, 9]),
        (1, [13, 13]),
        (1, [4, 12]),
        (0, [12, 7]),
        (1, [12, 5]),
        (0, [13, 12]),
        (1, [13, 11]),
        (0, [9, 4]),
        (1, [12, 8]),
        (0, [8, 8]),
        (0, [8, 17]),
        (0, [8, 6]),
        (1, [8, 9]),
        (1, [7, 9]),
        (0, [6, 12]),
        (1, [11, 7]),
        (1, [6, 9]),
        (0, [11, 15]),
        (1, [9, 13]),
        (1, [10, 10]),
        (1, [9, 5]),
        (0, [10, 11]),
        (1, [13, 13]),
        (0, [10, 9]),
        (0, [14, 10]),
        (1, [10, 6]),
        (1, [8, 9]),
    ],
];
/// The presented set's volley spikes by tick under each candidate, in the candidates'
/// order: the volley is F-46's, the cancel landing after it, and the census differs from
/// `VOLLEY_TICKS_1024` by the few units the previous trial's cancel left elsewhere.
const CANCELLED_CENSUS_1024: [&[(u32, u64)]; 3] = [
    &[
        (1, 142),
        (2, 943),
        (3, 1613),
        (4, 485),
        (5, 48),
        (6, 13),
        (7, 4),
        (8, 3),
        (9, 2),
    ],
    &[
        (1, 142),
        (2, 942),
        (3, 1611),
        (4, 486),
        (5, 50),
        (6, 11),
        (7, 4),
        (8, 4),
        (9, 1),
    ],
    &[
        (1, 142),
        (2, 944),
        (3, 1609),
        (4, 486),
        (5, 50),
        (6, 11),
        (7, 4),
        (8, 4),
        (9, 1),
    ],
];
/// The cancel the rules pick over the pinned tables: candidate (ii), six messages at the
/// bound per tick, the first whose probe passed and whose run fires once — the volley whole
/// (1 726 and 1 525 of 34 × 51 and 30 × 51) and no spike of the presented set in the pair
/// window after the readout window opens, against F-46's 2.06 per unit; (i) leaves 0.83 per
/// unit, the units whose standing potential the least count at rest does not cover.
const CANCEL_PICKED_1024: Option<u32> = Some(CANCEL_AT_THE_EXTREME);
/// Whether the picked cancel's run passes both calibrations at 1.75: the sight passes (62 of
/// 64) and the sign does not (13 of 64), so there is no rewarded run.
const CANCEL_CALIBRATED_1024: bool = false;

/// A candidate's frozen run with its cancel: the sight's blocks and trace, the composed
/// trials, every trial's counts and the volley-tick census.
type CancelledRun = (Vec<Block>, u64, Vec<Composed>, Vec<Counted>, Vec<u64>);

/// The candidates, each a frozen run of sixty-four trials with its cancel, run and dumped
/// before any is held to its table, so that one candidate's failure still shows the
/// others' readings.
#[test]
#[ignore]
fn the_two_injections_at_1024_units_exhaustive() {
    let runs: Vec<CancelledRun> = CANCELS
        .iter()
        .map(|&messages| {
            let (blocks, trace, trials, counts, counted, volley_ticks) =
                compose_shaped(SHAPE_F46, Some(cancel_of(messages)), 1024, GAIN_1024, BLOCK);
            assert_eq!(counts, SYNAPSES_1024);
            (blocks, trace, trials, counted, volley_ticks)
        })
        .collect();
    for (k, (blocks, trace, trials, counted, volley_ticks)) in runs.iter().enumerate() {
        let name = format!("cancelled1024 {k} {}", CANCELS[k]);
        dump_composition(&name, blocks, *trace, trials);
        dump_requires(&name, counted, false, OFFSET_MARK_64);
        let composed = composition(trials);
        eprintln!(
            "DUMP {name} volley {} after {} fires_once {} sight {} sign {} passes {} census {:?}",
            volley_once(1024, &blocks[0]),
            after_quiet(1024, &composed),
            fires_once(1024, &blocks[0], &composed),
            calibrated(&blocks[0]),
            composed.3,
            passes(&blocks[0], &composed),
            census_of(volley_ticks)
        );
    }
    for (k, (blocks, trace, trials, counted, volley_ticks)) in runs.iter().enumerate() {
        let name = format!("cancelled1024 {k} {}", CANCELS[k]);
        let (block, pin, composed) = &CANCELLED_1024[k];
        pinned(&format!("{name} sight"), blocks, *trace, &[*block], *pin);
        pinned_composition(&name, trials, &CANCELLED_ROWS_1024[k], Some(composed));
        assert_eq!(
            counted.as_slice(),
            &CANCELLED_COUNTED_1024[k],
            "{name}: the counts"
        );
        assert_eq!(
            census_of(volley_ticks),
            CANCELLED_CENSUS_1024[k].to_vec(),
            "{name}: the volley's ticks"
        );
    }
}

// ------------------------------------------------ the background side (brief 035)
//
// Step 2 of H-12's stopping rule (whitepaper §11.1): the background, in its two named
// settings, each tried alone and in a fixed order — the network settled before the task with
// the gain held at 1.75, then the criticality controller on at ADR-0055's step of an eighth —
// and picked by three measures that read no selection, reward or outcome: ADR-0074's measure
// that the stimulus still fires once, ADR-0065's sight and ADR-0072's sign, all unchanged. The
// stimulus is ADR-0076's pick, its efficacies pinned under both candidates (under the
// controller the drive it delivers follows the gain, F-47, which is part of what that
// candidate is); the rule, the geometry, the readout window, the trial and the seeds are
// unchanged. The candidates are never combined, and the order is fixed so that no preference
// chooses.
//
// The lead-in, per candidate: the instrument's executor at 1 024 units (the prior of ADR-0044
// at seed 22, the gain 1.75 in the image, the modulation baseline 0.5 as the rewarded runs
// have it, no sleep, the inhibitory period at its default) under the drive of ADR-0044 alone,
// no task, whole windows of $2^{17}$ ticks read per window — the population's spikes, the two
// readout sets' spikes, the sums by polarity, the fraction at the target, the gain and the
// estimate — until ADR-0055's criterion holds or the bound is reached. The criterion, as an
// integer rule written before the run (`settled_within`): a window $W$ is settled when each of
// the sixteen windows up to and including $W$ is within 0.75 per cent of the sum after the
// window sixteen before $W$ (the prior's sum for $W = 16$), $10\,000\,|E_k - E_{W-16}| <
// 75\,E_{W-16}$, strict, which is at least as strict as ADR-0055's reading ("every one of the
// last sixteen windows is within 0.75 per cent of the sixty-fourth", this rule at $W = 80$).
// Not brief 026's clause: ADR-0070 read that one holding while the sum still fell by a third.
// The lead-in is the first such $W$; the bound is eighty windows (ADR-0055's day), and a
// lead-in unsettled at the eightieth runs on to 160, the cost stated in the ADR; unsettled at
// 160, the lead-in is 160 and the ADR says the sum was still moving and at what rate. The rule
// is evaluated after every window, so the lead-in stops at $W$ and the frozen run starts from
// the state after it: a `for` over the larger bound, left when the rule holds.
//
// The settled engine is then run quiet without the drive until it is quiescent (`quiet`, a
// countdown to one window of ticks; the ticks it took and what it changed in the sums are
// read, not assumed) — encoding requires it — and saved as an image whose modulator record
// carries a baseline of zero (`frozen_image`, `frozen_from`), the one patch, so that the
// frozen run's weights do not move (asserted after every block); the homeostasis record is
// left as the lead-in left it, so under the controller the gain it reached and its step are
// carried and the controller still regulates through the frozen run. The clock resumes where
// the image was written (ADR-0033), so every stamp keeps its meaning; the train is not in the
// image, so the composer's oracle is seeded from the record before the first trial (each
// block's stamp and trace, each unit's last spike; `Composer::enumerate`) and is held to the
// record at every trial, as ADR-0072 holds it.
//
// The three measures, each over the frozen run of sixty-four trials (`compose_on` on the
// decoded engine, ADR-0076's stimulus): `fires_once` (the volley one per unit within two
// before the readout window opens, at most a tenth of a spike per unit in the pair window
// after it), `calibrated` (the sight, 56 of 64) and the sign (56 of 64). A candidate passes
// when all three do; the first that passes is the configuration (`background_pick`); none,
// and there is no rewarded run and the rule's step 3 applies. Deliverable D's readings and the
// composition are read from the same run under each candidate.
//
// The prediction for the controller, written before it ran, a Hypothesis from ADR-0053 and
// ADR-0055 on the lattice prior from a gain of 2.0 (`CONTROLLER_PREDICTED`, read by
// `controller_read`): (i) the gain after the lead-in's last window is above 1.75; (ii) the
// gain's course is not settled by the rule above; (iii) the two readout sets fire more in the
// lead-in's last window than under the settled network in its last; (iv) the candidate fails
// at least one of the three measures. The settled network carries no prediction: whether
// settling the weights escapes the squeeze ADR-0070 and ADR-0072's ladder found — a quieter
// background and a weaker propagation of the stimulus — is what its run measures.

/// The prior's sums at 1 024 units before any window, as the calibration pinned them with the
/// weights frozen: (inhibitory, excitatory).
const PRIOR_SUMS_1024: (i64, i64) = (CALIBRATION_1024[0].0.7, CALIBRATION_1024[0].0.8);
const _: () = assert!(PRIOR_SUMS_1024.0 == 213_902_976 && PRIOR_SUMS_1024.1 == 235_822_619);
/// ADR-0055's criterion: sixteen windows, each within 75 per ten thousand (0.75 per cent) of
/// the sum after the window sixteen before.
const SETTLED_WINDOWS: usize = 16;
const SETTLED_PER_MYRIAD: u64 = 75;
/// The lead-in's bound, eighty windows (the length ADR-0055 gave 1 024 units), and the bound
/// a lead-in unsettled at the eightieth runs on to.
const LEAD_IN_BOUND: u64 = 80;
const LEAD_IN_EXTENDED: u64 = 160;
const _: () =
    assert!(LEAD_IN_EXTENDED == 2 * LEAD_IN_BOUND && LEAD_IN_BOUND > SETTLED_WINDOWS as u64);
/// The controller's step, ADR-0055's eighth (Q0.16), the step already measured on this network
/// size and not searched.
const CONTROL_STEP_EIGHTH: u16 = 0x2000;
const _: () = assert!(CONTROL_STEP_EIGHTH as u32 * 8 == 1 << 16);
/// The candidates, as the controller's step each runs under, in the order tried and no other:
/// the settled network (the gain held, step 0, as every run of the instrument), then the
/// controller.
const BACKGROUNDS: [u16; 2] = [0, CONTROL_STEP_EIGHTH];
/// The quiet run's bound: one window of ticks.
const QUIET_BOUND: u64 = WINDOW_TICKS;
/// The windows the gate runs of the settled lead-in, held to the first rows of its table:
/// two, $2^{18}$ ticks at 1 024 units.
const GATE_WINDOWS: u64 = 2;
const _: () = assert!(GATE_WINDOWS < SETTLED_WINDOWS as u64);

/// One window of a lead-in (brief 035): the population's spikes; the two readout sets' spikes;
/// the inhibitory and the excitatory sum over the arena after the window; the fraction of
/// units at the target (Q16.16, ADR-0057); and the gain and the estimate as the window's
/// regulation left them.
type LeadInWindow = (u64, [u64; 2], i64, i64, u32, u32, u32);

/// ADR-0055's criterion as a rule over the excitatory sums of a lead-in, `prior` the sum
/// before the first window: the smallest window (from one) at which each of the last
/// `SETTLED_WINDOWS` windows up to and including it is within `SETTLED_PER_MYRIAD` per ten
/// thousand of the sum after the window `SETTLED_WINDOWS` before it (the prior's for the
/// sixteenth); none when no window of the table is. Integers throughout: a sum `s` is within
/// `p` per myriad of a reference `r` when `10 000 |s − r| < p r`, strict either way.
fn settled_within(prior: i64, sums: &[i64]) -> Option<usize> {
    let sum_after = |window: usize| -> Option<i64> {
        window
            .checked_sub(1)
            .map_or(Some(prior), |k| sums.get(k).copied())
    };
    (SETTLED_WINDOWS..=sums.len()).find(|&window| {
        let first = window.wrapping_sub(SETTLED_WINDOWS);
        let Some(reference) = sum_after(first) else {
            return false;
        };
        let bound = u64::try_from(reference)
            .unwrap_or(0)
            .saturating_mul(SETTLED_PER_MYRIAD);
        (first.wrapping_add(1)..=window).all(|k| match sum_after(k) {
            Some(sum) => sum.abs_diff(reference).saturating_mul(10_000) < bound,
            None => false,
        })
    })
}

/// The excitatory sums of a lead-in table, window by window.
fn lead_in_sums(table: &[LeadInWindow]) -> Vec<i64> {
    table.iter().map(|w| w.3).collect()
}

/// The candidate's executor at 1 024 units (brief 035): the instrument's network at the gain
/// 1.75 with the modulation baseline 0.5 (the rewarded runs') and the controller's step `step`
/// in its homeostasis record; at 0 the gain is held, as in every run of the instrument.
fn candidate(units: u32, step: u16) -> Engine {
    let p = prior(units);
    let cfg = Config {
        control_step_q0_16: step,
        ..config(units, 2, BASELINE_Q16)
    };
    let exec = at_gain(&p, cfg, gain(units));
    assert_eq!(exec.homeostasis().synaptic_gain_q16, gain(units));
    assert_eq!(exec.homeostasis().control_step_q0_16, step);
    assert_eq!(exec.modulation_baseline_q16(), BASELINE_Q16);
    exec
}

/// The two readout sets' spikes over `[from, to)`, read from the train, which must hold the
/// whole window (the caller asserts it). The lead-in's ticks stay below the stamp's width, so
/// the stamp is the tick.
fn readout_spikes(exec: &mut Engine, readouts: [Set; 2], from: u64, to: u64) -> [u64; 2] {
    let mut counts = [0u64; 2];
    for &(tick, unit) in exec.train() {
        let tick = u64::from(tick);
        if tick >= from && tick < to {
            for (count, set) in counts.iter_mut().zip(readouts.iter()) {
                if set.contains(unit) {
                    *count = count.saturating_add(1);
                }
            }
        }
    }
    counts
}

/// One window of a lead-in under `drive`, run and read: the population's spikes and the
/// fraction at the target over it, the readout sets' spikes, the sums after it, and the gain
/// and the estimate as the window's regulation left them; the train asserted to have held
/// the window whole, as `settling` asserts it.
fn lead_in_window(exec: &mut Engine, drive: &Drive, readouts: [Set; 2]) -> LeadInWindow {
    let from = exec.ticks();
    let held = exec.train().len() as u64;
    let overwritten = exec.train_overwritten();
    lead_in(exec, drive, 1);
    let to = exec.ticks();
    assert!(
        exec.train_overwritten().wrapping_sub(overwritten) <= held,
        "the train held the window"
    );
    let (spikes, at_target) = spikes_and_at_target(exec, from, to);
    let readout = readout_spikes(exec, readouts, from, to);
    let (inhibitory, excitatory) = weights_by_polarity(exec);
    let h = exec.homeostasis();
    (
        spikes,
        readout,
        inhibitory,
        excitatory,
        at_target,
        h.synaptic_gain_q16,
        h.branching_ratio_q16,
    )
}

/// A candidate's lead-in (brief 035): whole windows under the drive alone from the executor's
/// clock, read per window, until ADR-0055's criterion holds over the table so far or `bound`
/// windows have run; the table's length is the lead-in. A `for` over the bound, left when the
/// rule holds: it ends by construction. `prior` is the excitatory sum before the first window.
fn lead_in_until_settled(
    exec: &mut Engine,
    units: u32,
    prior: i64,
    bound: u64,
) -> Vec<LeadInWindow> {
    let drive = drive(units);
    let [_, _, r0, r1] = geometry(units, rotation(units));
    let mut table = Vec::new();
    for _ in 0..bound {
        table.push(lead_in_window(exec, &drive, [r0, r1]));
        if settled_within(prior, &lead_in_sums(&table)).is_some() {
            break;
        }
    }
    table
}

/// Runs `exec` quiet, without the drive, until it is quiescent — no unit holds a message, no
/// token is in flight, the injector is empty: the state an image is written from — and
/// returns the ticks it took. A countdown from `QUIET_BOUND`, so it ends by construction; a
/// network still active at the bound fails the test, which is a reading of its own.
fn quiet(exec: &mut Engine) -> u64 {
    let mut ticks = 0u64;
    for _ in 0..QUIET_BOUND {
        if exec.is_quiescent() {
            return ticks;
        }
        exec.tick();
        ticks = ticks.wrapping_add(1);
    }
    assert!(
        exec.is_quiescent(),
        "the network fell quiet within {QUIET_BOUND} ticks"
    );
    ticks
}

/// The image of `exec`, quiescent, with its modulator record's baseline patched to zero, the
/// one change, so that an engine decoded from it consolidates nothing: the frozen form of the
/// settled network. Every other section is as the lead-in left it, the homeostasis record's
/// gain and step among them.
fn frozen_image(exec: &Engine) -> Vec<u8> {
    let mut img = Image::encode(exec).expect("quiescent");
    let header = CortexFileHeader::decode(img[0..64].try_into().unwrap());
    for at in (64..).step_by(64).take(header.section_count as usize) {
        let mut entry = SectionEntry::decode(img[at..][..64].try_into().unwrap());
        if entry.kind == SECTION_MODULATOR {
            let (offset, length) = (entry.offset as usize, entry.length as usize);
            img[offset..][16..20].copy_from_slice(&0i32.to_le_bytes());
            entry.crc64 = crc64(&img[offset..][..length]);
            img[at..][..64].copy_from_slice(&entry.encode());
            return img;
        }
    }
    panic!("the image holds a modulator section");
}

/// The frozen engine from a settled candidate's image, decoded under the calibration's
/// configuration (two workers, a train that holds a trial); the image's baseline, gain and
/// step outrank the configuration's (§8.3), and the baseline is asserted zero.
fn frozen_from(image: &[u8], units: u32) -> Engine {
    let exec = Image::decode::<2048>(image, config(units, 2, 0)).expect("a well-formed record");
    assert_eq!(exec.modulation_baseline_q16(), 0, "the weights are frozen");
    exec
}

/// A candidate's frozen run (brief 035): the composition's run of `trials` trials with
/// ADR-0076's stimulus — F-46's drive with the cancel it picked — on the frozen engine, the
/// composer seeded from the record; the sight's blocks and trace, the composed trials, every
/// trial's counts and the volley-tick census.
fn background_run(exec: &mut Engine, units: u32, trials: usize) -> CancelledRun {
    let picked = CANCEL_PICKED_1024.expect("ADR-0076 picked a cancel");
    let (blocks, trace, trials, counts, counted, volley_ticks) =
        compose_on(exec, SHAPE_F46, Some(cancel_of(picked)), units, trials);
    assert_eq!(counts, SYNAPSES_1024);
    (blocks, trace, trials, counted, volley_ticks)
}

/// A candidate passes when all three measures do: the stimulus still fires once (ADR-0074's
/// `fires_once`), the sight (ADR-0065's `calibrated`) and the sign (ADR-0072's, `SIGN_MIN`),
/// the last two `passes`.
fn background_passes(block: &Block, composition: &Composition) -> bool {
    fires_once(1024, block, composition) && passes(block, composition)
}

/// The configuration the round picks: the first candidate, in the candidates' order, whose
/// frozen run passes all three measures, as its step; none when none does.
fn background_pick(runs: &[(Block, u64, Composition)]) -> Option<u16> {
    BACKGROUNDS
        .iter()
        .zip(runs.iter())
        .find(|(_, (block, _, composition))| background_passes(block, composition))
        .map(|(&step, _)| step)
}

/// The prediction for the controller, read over its lead-in, the settled network's and its
/// frozen run (the four clauses above, in order): true where the clause holds.
fn controller_read(
    controller: &[LeadInWindow],
    settled: &[LeadInWindow],
    run: &(Block, u64, Composition),
) -> [bool; 4] {
    let last = controller
        .last()
        .expect("a window of the controller's lead-in");
    let settled_last = settled.last().expect("a window of the settled lead-in");
    let gains: Vec<i64> = controller.iter().map(|w| i64::from(w.5)).collect();
    [
        last.5 > GAIN_1024,
        settled_within(i64::from(GAIN_1024), &gains).is_none(),
        last.1[0].saturating_add(last.1[1]) > settled_last.1[0].saturating_add(settled_last.1[1]),
        !background_passes(&run.0, &run.2),
    ]
}

/// The prediction, written before the controller ran: every clause holds.
const CONTROLLER_PREDICTED: [bool; 4] = [true; 4];

/// A candidate's readings: the lead-in (dumped and held to its table), the quiet run's ticks
/// and the sums after it, the frozen image and its engine (the sums, the gain and the step
/// carried across, asserted), and the frozen run read by the three measures, dumped before
/// anything is held to its table so that a failure still shows the readings.
fn background_candidate(k: usize, name: &str) {
    let step = BACKGROUNDS[k];
    let mut exec = candidate(1024, step);
    assert_eq!(weights_by_polarity(&exec), PRIOR_SUMS_1024);
    let table = lead_in_until_settled(&mut exec, 1024, PRIOR_SUMS_1024.1, LEAD_IN_EXTENDED);
    let settled = settled_within(PRIOR_SUMS_1024.1, &lead_in_sums(&table));
    eprintln!(
        "DUMP {name} lead-in {table:?} settled {settled:?} windows {}",
        table.len()
    );
    let ticks = quiet(&mut exec);
    let quieted = weights_by_polarity(&exec);
    let (gain_after, estimate_after) = (
        exec.homeostasis().synaptic_gain_q16,
        exec.homeostasis().branching_ratio_q16,
    );
    eprintln!(
        "DUMP {name} quiet {ticks} sums {quieted:?} gain {gain_after:#x} estimate {estimate_after:#x}"
    );
    let image = frozen_image(&exec);
    let mut frozen = frozen_from(&image, 1024);
    assert_eq!(
        weights_by_polarity(&frozen),
        quieted,
        "{name}: the image carries the weights"
    );
    assert_eq!(
        frozen.homeostasis().synaptic_gain_q16,
        gain_after,
        "{name}: and the gain"
    );
    assert_eq!(
        frozen.homeostasis().control_step_q0_16,
        step,
        "{name}: and the step"
    );
    let (blocks, trace, trials, counted, volley_ticks) = background_run(&mut frozen, 1024, BLOCK);
    dump_composition(name, &blocks, trace, &trials);
    dump_requires(name, &counted, false, OFFSET_MARK_64);
    let composed = composition(&trials);
    eprintln!(
        "DUMP {name} volley {} after {} fires_once {} sight {} sign {} passes {} census {:?} gain after the run {:#x}",
        volley_once(1024, &blocks[0]),
        after_quiet(1024, &composed),
        fires_once(1024, &blocks[0], &composed),
        calibrated(&blocks[0]),
        composed.3,
        background_passes(&blocks[0], &composed),
        census_of(&volley_ticks),
        frozen.homeostasis().synaptic_gain_q16
    );
    assert_eq!(
        table.as_slice(),
        BACKGROUND_LEAD_IN_1024[k],
        "{name}: the lead-in"
    );
    assert_eq!((ticks, quieted), QUIET_1024[k], "{name}: the quiet run");
    let (block, pin, pinned_composed) = &BACKGROUND_1024[k];
    pinned(&format!("{name} sight"), &blocks, trace, &[*block], *pin);
    pinned_composition(
        name,
        &trials,
        &BACKGROUND_ROWS_1024[k],
        Some(pinned_composed),
    );
    assert_eq!(
        counted.as_slice(),
        &BACKGROUND_COUNTED_1024[k],
        "{name}: the counts"
    );
    assert_eq!(
        census_of(&volley_ticks),
        BACKGROUND_CENSUS_1024[k].to_vec(),
        "{name}: the volley's ticks"
    );
}

/// The settled network at 1 024 units (brief 035, the first candidate): the lead-in until
/// ADR-0055's criterion holds or the bound, the quiet run, the frozen image and the frozen run
/// read by the three measures.
#[test]
#[ignore]
fn the_settled_network_at_1024_units_exhaustive() {
    background_candidate(0, "settled1024");
}

/// The controller at 1 024 units (brief 035, the second candidate): as the first, with the
/// controller's step of an eighth regulating through the lead-in and the frozen run.
#[test]
#[ignore]
fn the_controller_at_1024_units_exhaustive() {
    background_candidate(1, "controller1024");
}

// ---------------------------------------------------------- the measurement (brief 035)

/// The candidates' lead-ins at 1 024 units, in the candidates' order, each pinned from one
/// run: the settled network's under the gain held at 1.75, the controller's under its step.
const BACKGROUND_LEAD_IN_1024: [&[LeadInWindow]; 2] = [
    &[
        (
            2446,
            [1085, 1121],
            213359105,
            235275227,
            27968,
            114688,
            18782,
        ),
        (
            2329,
            [1001, 1079],
            212880571,
            234948592,
            25984,
            114688,
            14600,
        ),
        (
            2306,
            [1043, 1031],
            212399982,
            234652372,
            26176,
            114688,
            14377,
        ),
        (2338, [1012, 1092], 211856927, 234369738, 25472, 114688, 0),
        (
            2388,
            [1059, 1077],
            211341639,
            234055848,
            27456,
            114688,
            7401,
        ),
        (2198, [1003, 986], 210858800, 233802827, 24512, 114688, 0),
        (2418, [1153, 1029], 210394382, 233441558, 27072, 114688, 0),
        (2319, [1073, 1009], 209931362, 233165658, 25344, 114688, 105),
        (2282, [986, 1071], 209440186, 232893632, 25536, 114688, 0),
        (
            2462,
            [1091, 1133],
            208973256,
            232558620,
            29184,
            114688,
            5648,
        ),
        (
            2247,
            [1001, 1019],
            208467654,
            232328675,
            24960,
            114688,
            7433,
        ),
        (2258, [1037, 1001], 208020347, 232055777, 25664, 114688, 0),
        (2331, [1055, 1031], 207572956, 231757255, 26368, 114688, 0),
        (2228, [982, 998], 207115120, 231511854, 24448, 114688, 5733),
        (2257, [1030, 995], 206623333, 231305112, 24384, 114688, 0),
        (2218, [986, 1012], 206197397, 231084619, 24320, 114688, 0),
        (
            2314,
            [1097, 1005],
            205748364,
            230786158,
            25792,
            114688,
            24506,
        ),
        (2217, [1045, 953], 205245258, 230560266, 23872, 114688, 0),
        (2292, [1050, 995], 204814608, 230327218, 25152, 114688, 0),
        (2223, [1023, 981], 204324017, 230129233, 23936, 114688, 0),
        (2378, [1087, 1040], 203855587, 229866069, 27072, 114688, 0),
        (
            2338,
            [1075, 1046],
            203411889,
            229586949,
            25600,
            114688,
            2043,
        ),
        (2341, [1069, 1061], 202912718, 229297053, 26816, 114688, 0),
        (
            2434,
            [1105, 1074],
            202384172,
            229013299,
            27328,
            114688,
            2184,
        ),
        (2258, [1066, 994], 201871721, 228798127, 24384, 114688, 3666),
        (
            2425,
            [1051, 1124],
            201387646,
            228534605,
            27712,
            114688,
            4483,
        ),
        (2179, [1011, 953], 200914490, 228364797, 24000, 114688, 0),
        (
            2318,
            [1036, 1037],
            200449709,
            228083637,
            24960,
            114688,
            8966,
        ),
        (
            2302,
            [1066, 1003],
            199963385,
            227894845,
            25664,
            114688,
            3284,
        ),
        (2212, [1044, 946], 199429909, 227689854, 23424, 114688, 7665),
        (2170, [1002, 959], 198906485, 227509818, 22272, 114688, 1878),
        (2182, [1003, 958], 198411019, 227340644, 24256, 114688, 0),
        (2146, [979, 970], 197909595, 227186565, 22976, 114688, 7880),
        (
            2308,
            [1060, 1011],
            197434974,
            226971255,
            24640,
            114688,
            2754,
        ),
        (
            2278,
            [1071, 984],
            196951252,
            226725904,
            23680,
            114688,
            16963,
        ),
        (2287, [1004, 1029], 196471084, 226521063, 25472, 114688, 0),
        (
            2296,
            [1002, 1043],
            195996948,
            226276661,
            24768,
            114688,
            10937,
        ),
        (
            2272,
            [1047, 991],
            195520803,
            226139285,
            24768,
            114688,
            10562,
        ),
        (2170, [1005, 964], 195038724, 225961883, 23104, 114688, 0),
        (
            2235,
            [1015, 1004],
            194538450,
            225783990,
            23104,
            114688,
            3315,
        ),
        (
            2177,
            [1011, 952],
            194062819,
            225632134,
            23616,
            114688,
            21198,
        ),
        (2337, [1058, 1067], 193508354, 225425960, 25408, 114688, 0),
        (
            2184,
            [1003, 971],
            193007964,
            225306353,
            23552,
            114688,
            26251,
        ),
        (
            2322,
            [1061, 1009],
            192501639,
            225116804,
            25024,
            114688,
            13699,
        ),
        (2160, [959, 977], 192036815, 224981683, 23296, 114688, 0),
        (2322, [1036, 996], 191524728, 224803168, 25600, 114688, 0),
        (2226, [1005, 999], 190996372, 224638213, 24192, 114688, 6352),
        (
            2298,
            [1009, 1044],
            190524522,
            224460111,
            25152,
            114688,
            29567,
        ),
        (2156, [972, 996], 190026675, 224347762, 23040, 114688, 0),
        (
            2387,
            [1115, 1017],
            189485926,
            224144929,
            26624,
            114688,
            14870,
        ),
        (
            2301,
            [1070, 1041],
            188968391,
            223943378,
            25664,
            114688,
            6017,
        ),
        (
            2266,
            [1079, 950],
            188473664,
            223785560,
            25152,
            114688,
            16091,
        ),
        (2245, [1008, 1003], 187989918, 223654378, 24960, 114688, 0),
        (2331, [1092, 1030], 187455344, 223498782, 25344, 114688, 0),
        (2143, [934, 985], 186924546, 223358080, 23360, 114688, 12832),
        (2213, [1048, 949], 186382434, 223204338, 23552, 114688, 0),
        (2221, [1014, 988], 185874694, 223065266, 24256, 114688, 0),
        (2132, [942, 979], 185383311, 222930288, 21312, 114688, 3869),
        (
            2295,
            [1040, 1023],
            184847230,
            222768360,
            24576,
            114688,
            13702,
        ),
        (2269, [1066, 983], 184318047, 222600675, 25088, 114688, 0),
        (
            2274,
            [1019, 1020],
            183803826,
            222441039,
            25088,
            114688,
            8503,
        ),
        (2239, [982, 1060], 183265633, 222297784, 24064, 114688, 6517),
        (2268, [1072, 983], 182749471, 222175583, 23936, 114688, 0),
        (2175, [970, 962], 182295585, 222063480, 23936, 114688, 18948),
        (2265, [1038, 1001], 181805389, 221900759, 24064, 114688, 0),
        (2252, [981, 1074], 181274039, 221752910, 23872, 114688, 9820),
        (2341, [1086, 1028], 180720644, 221623015, 25792, 114688, 0),
        (
            2299,
            [997, 1039],
            180214418,
            221440596,
            24512,
            114688,
            12643,
        ),
        (
            2273,
            [1045, 1005],
            179699585,
            221307497,
            24512,
            114688,
            13074,
        ),
        (2146, [944, 990], 179221133, 221198135, 23488, 114688, 0),
        (
            2254,
            [1039, 1004],
            178765721,
            221061424,
            24000,
            114688,
            5897,
        ),
        (
            2353,
            [1114, 1023],
            178258430,
            220881300,
            25408,
            114688,
            10902,
        ),
        (2245, [1024, 994], 177714283, 220767515, 24896, 114688, 0),
        (2324, [1043, 1051], 177171735, 220621883, 25280, 114688, 0),
        (2171, [971, 983], 176650812, 220507957, 22336, 114688, 14958),
        (
            2236,
            [1012, 1008],
            176161823,
            220394857,
            23488,
            114688,
            24187,
        ),
        (2340, [1082, 1023], 175687942, 220244766, 24960, 114688, 0),
        (
            2268,
            [1024, 1019],
            175149296,
            220147269,
            24384,
            114688,
            2861,
        ),
        (2256, [987, 1033], 174652364, 220019391, 24640, 114688, 6780),
        (2350, [1031, 1092], 174132530, 219887364, 26752, 114688, 0),
        (2142, [972, 958], 173657844, 219801483, 22720, 114688, 0),
        (2316, [1090, 988], 173132774, 219687064, 25728, 114688, 2299),
        (2246, [995, 1034], 172568625, 219579664, 24512, 114688, 2931),
        (2191, [1003, 954], 172043426, 219442039, 22592, 114688, 1070),
        (2182, [995, 984], 171499818, 219362408, 23232, 114688, 0),
        (
            2327,
            [1049, 1046],
            171027986,
            219230487,
            26304,
            114688,
            1971,
        ),
        (2143, [989, 955], 170528888, 219152474, 23360, 114688, 17859),
        (
            2154,
            [1001, 963],
            170018373,
            219058704,
            23424,
            114688,
            14469,
        ),
        (2322, [1102, 982], 169495824, 218925984, 24896, 114688, 0),
        (2203, [1003, 976], 168963177, 218845890, 23872, 114688, 0),
        (2222, [1018, 973], 168441628, 218734468, 25216, 114688, 0),
        (
            2200,
            [1015, 981],
            167898617,
            218650268,
            22400,
            114688,
            11728,
        ),
        (2314, [1068, 1036], 167365910, 218578023, 25152, 114688, 0),
        (
            2282,
            [992, 1060],
            166885026,
            218451597,
            23872,
            114688,
            18953,
        ),
        (2345, [1047, 1071], 166364950, 218357692, 25856, 114688, 0),
        (
            2308,
            [1013, 1063],
            165876154,
            218243354,
            25792,
            114688,
            19823,
        ),
    ],
    &[
        (
            2446,
            [1085, 1121],
            213359105,
            235275227,
            27968,
            124915,
            18782,
        ),
        (
            7918,
            [3537, 3585],
            212541010,
            231406417,
            59328,
            134144,
            26802,
        ),
        (
            13799,
            [6186, 6233],
            212528054,
            221515784,
            29056,
            145230,
            22209,
        ),
        (
            22710,
            [10131, 10283],
            213492369,
            200140518,
            3136,
            156301,
            25570,
        ),
        (
            31858,
            [14333, 14267],
            213860116,
            170962350,
            256,
            170792,
            16929,
        ),
        (
            44560,
            [20173, 19960],
            213901887,
            140387729,
            0,
            149443,
            1048576,
        ),
        (21427, [9694, 9608], 213883155, 134890027, 3264, 168123, 0),
        (
            39315,
            [17836, 17507],
            213901376,
            123374728,
            0,
            147108,
            1048576,
        ),
        (18653, [8304, 8462], 213851779, 121423030, 6912, 165497, 0),
        (
            35730,
            [16010, 16125],
            213893808,
            116130444,
            0,
            144810,
            1048576,
        ),
        (
            16385,
            [7399, 7409],
            213815082,
            115480001,
            15552,
            162684,
            822,
        ),
        (32277, [14544, 14484], 213887766, 112918474, 64, 183020, 0),
        (
            53795,
            [24254, 24082],
            213902976,
            109806549,
            0,
            160143,
            1048576,
        ),
        (29537, [13384, 13160], 213902485, 109065895, 128, 180161, 0),
        (
            50278,
            [22542, 22637],
            213902826,
            107997709,
            0,
            157641,
            1048576,
        ),
        (26891, [12164, 12030], 213900308, 107724148, 704, 177346, 0),
        (
            47253,
            [21447, 21111],
            213902800,
            107179569,
            0,
            155178,
            1048576,
        ),
        (24621, [11101, 11027], 213896026, 107237752, 1536, 174575, 0),
        (
            44229,
            [19956, 19844],
            213902151,
            106894660,
            0,
            152753,
            1048576,
        ),
        (22438, [10035, 10096], 213884837, 107042522, 1920, 171847, 0),
        (
            41132,
            [18546, 18363],
            213899497,
            106446783,
            0,
            150366,
            1048576,
        ),
        (20553, [9288, 9238], 213863194, 106843122, 5120, 169162, 0),
        (
            38212,
            [17342, 17062],
            213896154,
            106317651,
            64,
            148017,
            1048576,
        ),
        (
            18703,
            [8384, 8388],
            213848562,
            106610509,
            7488,
            165982,
            1907,
        ),
        (
            35176,
            [15817, 15869],
            213892917,
            106373443,
            0,
            145234,
            1048576,
        ),
        (16501, [7513, 7326], 213791779, 106698857, 15104, 163388, 0),
        (32224, [14521, 14484], 213879964, 106417189, 192, 183812, 0),
        (
            54398,
            [24474, 24402],
            213902976,
            107134111,
            0,
            160836,
            1048576,
        ),
        (30113, [13617, 13510], 213901363, 106880282, 256, 180941, 0),
        (
            51196,
            [23101, 23003],
            213902976,
            106750721,
            0,
            158323,
            1048576,
        ),
        (
            27393,
            [12471, 12243],
            213895007,
            106847917,
            512,
            174939,
            10510,
        ),
        (
            44269,
            [19933, 19985],
            213902110,
            106686159,
            0,
            153072,
            1048576,
        ),
        (
            22722,
            [10233, 10226],
            213893833,
            106831478,
            1472,
            170634,
            5387,
        ),
        (
            39885,
            [18003, 17862],
            213902676,
            106628922,
            0,
            149305,
            1048576,
        ),
        (
            19757,
            [8951, 8831],
            213866726,
            106938202,
            4672,
            161976,
            21036,
        ),
    ],
];
/// The quiet runs: the ticks each took and the sums by polarity after it.
const QUIET_1024: [(u64, (i64, i64)); 2] = [
    (2500, (165876268, 218243354)),
    (2541, (213866726, 106938185)),
];
/// The candidates' frozen runs, each of sixty-four trials with ADR-0076's stimulus on the
/// frozen engine: the sight's block and trace, and the composition. Pinned from one run each.
const BACKGROUND_1024: [(Block, u64, Composition); 2] = [
    (
        (
            29,
            34,
            [[330, 323], [315, 314]],
            [1728, 1525],
            [2491, 2252],
            [258, 256],
            62,
            165876268,
            218243354,
            0,
            [[6249552, 6698611], [6584205, 6815470]],
            8,
        ),
        0x60f6be387fbce9cd,
        (
            [[581, 11652], [6687, 10983]],
            [
                [
                    [214620, -5235, 318258, -506972],
                    [239131, -5599, 344531, -493105],
                ],
                [
                    [227460, -5372, 281672, -462205],
                    [208207, -4723, 325571, -442474],
                ],
            ],
            [[1100, 1646], [1009, 1645], [3350, 0]],
            53,
            5817133415553407245,
        ),
    ),
    (
        (
            33,
            34,
            [[2350, 2341], [1962, 1984]],
            [1646, 1458],
            [14811, 13425],
            [4034, 3942],
            51,
            213866726,
            106938185,
            0,
            [[3149340, 3346376], [3184652, 3239633]],
            0,
        ),
        0x4f57a261e13951af,
        (
            [[12794, -3720], [16587, 4124]],
            [
                [
                    [642005, -263256, 13907673, -14109941],
                    [673222, -274308, 14247814, -14654024],
                ],
                [
                    [527047, -194836, 12652268, -12809414],
                    [522192, -192938, 12744444, -12996071],
                ],
            ],
            [[16366, 16789], [16287, 16797], [4774, 14]],
            47,
            7693778306876544192,
        ),
    ),
];
/// The candidates' rows, trial by trial.
const BACKGROUND_ROWS_1024: [[Row; BLOCK]; 2] = [
    [
        (0, 7598, [1697, -8, 23569, -17210]),
        (0, -2383, [12050, -769, 17987, -36979]),
        (1, -6090, [2763, -39, 14205, -21100]),
        (0, -2456, [10353, -667, 16060, -25683]),
        (0, -454, [10021, -441, 12026, -20435]),
        (0, -7759, [10709, -621, 11043, -28055]),
        (0, -13372, [16659, -179, 9072, -32647]),
        (0, -4528, [14866, -125, 15795, -25648]),
        (0, 7348, [14792, -446, 17526, -19882]),
        (0, 18601, [14335, -633, 13605, -14830]),
        (1, -16059, [5376, -374, 12930, -25513]),
        (1, 3652, [17725, -90, 16035, -17233]),
        (0, 838, [3032, -45, 16627, -21107]),
        (0, 11626, [15775, -208, 12297, -16731]),
        (1, 14348, [14753, -145, 21871, -22674]),
        (0, 13840, [12707, -2, 14192, -21079]),
        (1, 14547, [9785, -191, 15872, -23401]),
        (0, 12148, [5710, -1, 19740, -27635]),
        (0, 21765, [16159, -442, 17086, -19234]),
        (0, 18891, [14052, -129, 12440, -24473]),
        (1, 1630, [4067, -329, 14056, -18700]),
        (0, 21586, [12416, -188, 17817, -17128]),
        (1, 1870, [12471, -203, 15991, -25751]),
        (1, 6831, [13141, -328, 17304, -25694]),
        (1, 8685, [14022, -37, 14031, -25093]),
        (0, -1825, [9281, -847, 16360, -24261]),
        (1, 10727, [10223, -119, 18597, -19081]),
        (1, 9842, [17057, -344, 17005, -31877]),
        (1, 23169, [14179, -230, 18195, -18838]),
        (0, 2036, [4048, -70, 19625, -18346]),
        (0, 7338, [20601, -262, 15013, -30099]),
        (0, 17078, [16564, -150, 12818, -16691]),
        (0, 29797, [21735, -945, 16592, -20087]),
        (0, 26264, [13869, -439, 13450, -25618]),
        (1, 5460, [8616, -1488, 21987, -22567]),
        (1, 23470, [20368, -1033, 16905, -17795]),
        (1, 20850, [14229, -544, 8680, -19107]),
        (0, 8943, [4587, -258, 15659, -21148]),
        (1, 4248, [9720, -363, 11899, -26114]),
        (1, 14096, [23196, -234, 17524, -29648]),
        (0, 8751, [10497, -199, 17832, -18697]),
        (1, 17298, [14462, -134, 21218, -23447]),
        (0, 4308, [6965, -92, 12816, -21525]),
        (1, 17165, [10962, -90, 13347, -19118]),
        (0, 5683, [7132, -100, 15969, -20550]),
        (1, 12999, [7860, -311, 20594, -28997]),
        (0, 11964, [16034, -354, 16135, -21761]),
        (0, 20776, [15499, -147, 12502, -15772]),
        (0, 23776, [15176, -100, 19430, -27498]),
        (1, 10871, [8451, -308, 16730, -18977]),
        (1, 22312, [17288, -19, 16691, -19084]),
        (0, 10455, [5341, -300, 13620, -15470]),
        (1, 11288, [7590, -403, 12849, -24619]),
        (1, 15428, [22215, -454, 13109, -28758]),
        (0, -4526, [7711, -228, 20506, -35729]),
        (1, 12125, [11515, -157, 18751, -25445]),
        (1, 19383, [20084, -988, 12989, -21716]),
        (1, 24423, [14030, -65, 15041, -19726]),
        (0, -14641, [6796, -684, 19670, -30921]),
        (1, 21547, [10958, -244, 14464, -17882]),
        (0, 4513, [14280, -747, 24128, -21376]),
        (0, 10174, [15309, -8, 13316, -22090]),
        (1, 16525, [3891, -34, 18755, -16728]),
        (1, 17670, [18524, -796, 10017, -22968]),
    ],
    [
        (0, 44457, [41310, -12664, 303198, -280213]),
        (0, 4856, [32926, -11724, 281258, -329500]),
        (1, 28762, [35654, -11312, 318589, -318262]),
        (0, 13555, [39475, -11681, 351155, -362635]),
        (0, -9970, [25102, -9615, 319931, -359727]),
        (0, 3145, [33947, -9526, 333761, -341822]),
        (0, 11807, [35164, -15605, 325208, -335736]),
        (0, 28690, [42639, -14684, 418734, -424101]),
        (0, -438, [37420, -20221, 655938, -695581]),
        (0, -11255, [47019, -28156, 674473, -702759]),
        (1, 22202, [46168, -23536, 596325, -606274]),
        (1, 3014, [43350, -26442, 559120, -588673]),
        (0, -27063, [48245, -25345, 683938, -701975]),
        (0, -19827, [41384, -21174, 679390, -697402]),
        (1, -14134, [34868, -18245, 611865, -651672]),
        (0, 6947, [43667, -20612, 520837, -547932]),
        (1, 15531, [33184, -9974, 268441, -267920]),
        (0, 22988, [42840, -13990, 276868, -283072]),
        (0, 24776, [34474, -12330, 253419, -267297]),
        (0, 41112, [40180, -10437, 291305, -294435]),
        (1, 69577, [34223, -11560, 280732, -261934]),
        (0, 30121, [36344, -10032, 250125, -273647]),
        (1, 39310, [33433, -9469, 254400, -270780]),
        (1, 28220, [29308, -8914, 269061, -290683]),
        (1, 2837, [34984, -13493, 486518, -530919]),
        (0, 1050, [43272, -22792, 633249, -661129]),
        (1, 10898, [38740, -21475, 595824, -595859]),
        (1, 8950, [32588, -14441, 573223, -589113]),
        (1, 652, [33570, -15444, 554967, -575704]),
        (0, -5436, [33746, -14998, 611306, -654101]),
        (0, -14314, [46019, -24284, 614972, -646385]),
        (0, -15552, [41772, -22818, 604617, -629006]),
        (0, 36457, [28813, -4813, 249217, -222771]),
        (0, 29976, [32159, -6668, 201484, -224853]),
        (1, 16888, [34335, -9526, 207884, -215842]),
        (1, 11741, [23758, -4975, 185856, -210694]),
        (1, 21128, [35206, -7000, 200362, -213759]),
        (0, 19544, [30941, -8594, 249339, -266035]),
        (1, 29402, [39215, -12439, 241432, -237906]),
        (1, 18073, [24888, -5827, 225371, -249617]),
        (0, 12791, [35286, -17530, 513150, -532752]),
        (1, 10148, [45838, -23111, 530044, -559041]),
        (0, -18330, [37182, -20633, 524820, -548585]),
        (1, 31835, [46327, -19151, 548826, -549900]),
        (0, -26268, [44939, -19644, 624392, -658698]),
        (1, -3075, [37748, -17449, 516165, -549997]),
        (0, -17063, [42285, -19616, 546826, -571892]),
        (0, -25142, [42269, -21205, 438371, -472470]),
        (0, 19395, [33662, -6920, 238011, -222311]),
        (1, 3963, [28956, -6689, 147362, -168935]),
        (1, 21213, [30048, -5625, 199133, -203399]),
        (0, 39283, [35838, -10442, 243176, -242211]),
        (1, 30672, [26789, -5443, 186459, -197518]),
        (1, 41903, [30350, -5713, 179932, -184886]),
        (0, 46473, [36898, -11455, 198187, -217338]),
        (1, 33953, [31243, -9343, 226559, -242897]),
        (1, -9209, [36680, -14170, 414419, -475550]),
        (1, 25249, [34009, -14686, 466932, -448246]),
        (0, 4367, [43365, -17540, 461881, -480280]),
        (1, 24932, [37499, -15973, 413099, -400065]),
        (0, -19862, [36437, -15989, 475830, -523456]),
        (0, 1475, [44992, -23826, 526682, -525175]),
        (1, -2006, [29289, -9188, 448902, -488990]),
        (1, 20711, [44266, -17158, 411226, -413452]),
    ],
];
/// Every trial's readout counts under each candidate.
const BACKGROUND_COUNTED_1024: [[Counted; BLOCK]; 2] = [
    [
        (0, [3, 3]),
        (0, [13, 12]),
        (1, [10, 10]),
        (0, [1, 10]),
        (0, [6, 9]),
        (0, [14, 7]),
        (0, [16, 15]),
        (0, [15, 10]),
        (0, [9, 11]),
        (0, [5, 8]),
        (1, [16, 14]),
        (1, [12, 14]),
        (0, [6, 7]),
        (0, [8, 11]),
        (1, [16, 19]),
        (0, [5, 7]),
        (1, [11, 12]),
        (0, [10, 8]),
        (0, [12, 8]),
        (0, [8, 12]),
        (1, [9, 12]),
        (0, [7, 15]),
        (1, [10, 15]),
        (1, [7, 8]),
        (1, [5, 5]),
        (0, [9, 9]),
        (1, [7, 9]),
        (1, [14, 10]),
        (1, [12, 11]),
        (0, [12, 13]),
        (0, [9, 10]),
        (0, [10, 15]),
        (0, [9, 9]),
        (0, [9, 13]),
        (1, [19, 10]),
        (1, [8, 5]),
        (1, [8, 7]),
        (0, [9, 9]),
        (1, [13, 18]),
        (1, [24, 13]),
        (0, [9, 6]),
        (1, [5, 9]),
        (0, [8, 3]),
        (1, [6, 16]),
        (0, [13, 11]),
        (1, [12, 5]),
        (0, [17, 15]),
        (0, [7, 9]),
        (0, [11, 8]),
        (1, [7, 16]),
        (1, [15, 5]),
        (0, [13, 4]),
        (1, [8, 9]),
        (1, [4, 6]),
        (0, [17, 11]),
        (1, [11, 8]),
        (1, [11, 11]),
        (1, [9, 15]),
        (0, [12, 6]),
        (1, [9, 7]),
        (0, [9, 10]),
        (0, [9, 9]),
        (1, [16, 8]),
        (1, [1, 7]),
    ],
    [
        (0, [45, 36]),
        (0, [60, 58]),
        (1, [59, 61]),
        (0, [57, 68]),
        (0, [59, 47]),
        (0, [66, 51]),
        (0, [69, 55]),
        (0, [56, 60]),
        (0, [77, 82]),
        (0, [86, 91]),
        (1, [100, 83]),
        (1, [106, 105]),
        (0, [104, 97]),
        (0, [94, 91]),
        (1, [88, 101]),
        (0, [87, 107]),
        (1, [43, 42]),
        (0, [53, 55]),
        (0, [55, 62]),
        (0, [49, 53]),
        (1, [51, 43]),
        (0, [55, 51]),
        (1, [56, 68]),
        (1, [43, 42]),
        (1, [102, 80]),
        (0, [90, 79]),
        (1, [91, 81]),
        (1, [72, 80]),
        (1, [86, 82]),
        (0, [75, 82]),
        (0, [92, 101]),
        (0, [99, 81]),
        (0, [52, 51]),
        (0, [51, 49]),
        (1, [39, 53]),
        (1, [50, 41]),
        (1, [53, 43]),
        (0, [48, 52]),
        (1, [51, 36]),
        (1, [49, 54]),
        (0, [81, 76]),
        (1, [78, 75]),
        (0, [83, 88]),
        (1, [83, 86]),
        (0, [90, 92]),
        (1, [81, 88]),
        (0, [85, 83]),
        (0, [81, 78]),
        (0, [48, 45]),
        (1, [42, 58]),
        (1, [51, 40]),
        (0, [35, 56]),
        (1, [47, 45]),
        (1, [58, 32]),
        (0, [40, 41]),
        (1, [39, 51]),
        (1, [74, 76]),
        (1, [64, 94]),
        (0, [69, 81]),
        (1, [77, 82]),
        (0, [70, 65]),
        (0, [89, 77]),
        (1, [60, 83]),
        (1, [69, 79]),
    ],
];
/// The volley-tick census under each candidate.
const BACKGROUND_CENSUS_1024: [&[(u32, u64)]; 2] = [
    &[
        (1, 158),
        (2, 982),
        (3, 1578),
        (4, 497),
        (5, 31),
        (6, 6),
        (7, 1),
    ],
    &[
        (0, 2),
        (1, 1174),
        (2, 1571),
        (3, 330),
        (4, 24),
        (5, 1),
        (6, 1),
        (94, 1),
    ],
];

/// Where ADR-0055's criterion first held, per candidate: the settled network at the
/// ninety-sixth window — past the eightieth, so the lead-in ran on under the extended bound —
/// with the excitatory sum at 0.925 of the prior's; the controller at the thirty-fifth, at
/// 0.453 of the prior's, the fixed point ADR-0055's day read (0.452), the gain swinging
/// between 2.2 and 2.8 every window and the rate with it.
const SETTLED_AT_1024: [Option<usize>; 2] = [Some(96), Some(35)];
/// The three measures under each candidate, as read over the pinned tables: the stimulus
/// still fires once, the sight, the sign. The settled network fires once and sees (62 of 64)
/// and its sign reads 53 of 64, three short of the mark; the controller fires the volley
/// short (48.4 of 51 per presentation, the units its background had within their refractory
/// window at the injection), is not seen (51) and its sign reads 47.
const BACKGROUND_MEASURES_1024: [[bool; 3]; 2] = [[true, true, false], [false, false, false]];
/// The configuration the rules pick over the pinned tables: none. No candidate passes all
/// three measures, so there is no rewarded run and H-12's stopping rule reaches its step 3.
const BACKGROUND_PICKED_1024: Option<u16> = None;
/// The prediction for the controller, as read: every clause held — the gain 2.47 after the
/// lead-in and 2.31 after the run, never settled, the readout sets firing 8.6 times more in
/// the lead-in's last window than under the settled network, and all three measures failed.
const CONTROLLER_READ: [bool; 4] = [true; 4];
/// Deliverable D under each candidate: the offset the criterion needs over the frozen run's
/// sixty-four trials at 40, and the instrument's bias per stimulus, `(difference, trials)`.
const OFFSETS_1024: [Option<u32>; 2] = [Some(2), Some(3)];
const BIASES_1024: [[(i64, u32); 2]; 2] = [[(7, 34), (-1, 30)], [(9, 34), (22, 30)]];

/// The gate's test (ADR-0061's class; brief 035): ADR-0055's criterion as a rule, at its
/// edges over tables written by hand; the pick and the prediction's clauses at theirs; then,
/// over the pinned tables, the lead-ins' lengths, the quiet runs, the measures, the pick and
/// the prediction as written; and the first two windows of the settled lead-in, run and held
/// to the first two rows of its table. No number is pinned twice.
#[test]
fn the_first_two_windows_of_the_settled_lead_in_at_1024_units_and_the_rules_over_their_tables() {
    // The criterion at its edges: sixteen windows at the prior's sum settle at the
    // sixteenth; one of them at exactly 0.75 per cent off does not (the comparison is
    // strict), one LSB inside does; a rise is a move; a table shorter than sixteen has no
    // window; and a window whose reference is a window's sum and not the prior's holds where
    // the prior's does not.
    let flat = [10_000i64; 16];
    assert_eq!(settled_within(10_000, &flat), Some(16));
    let mut edge = flat;
    edge[7] = 10_075;
    assert_eq!(
        settled_within(10_000, &edge),
        None,
        "exactly 0.75 per cent is not within"
    );
    edge[7] = 10_074;
    assert_eq!(settled_within(10_000, &edge), Some(16));
    edge[7] = 9_926;
    assert_eq!(
        settled_within(10_000, &edge),
        Some(16),
        "0.74 per cent below"
    );
    edge[7] = 9_925;
    assert_eq!(settled_within(10_000, &edge), None, "0.75 per cent below");
    edge[7] = 10_076;
    assert_eq!(settled_within(10_000, &edge), None, "a rise is a move");
    assert_eq!(
        settled_within(10_000, &flat[..15]),
        None,
        "fifteen windows have no window"
    );
    assert_eq!(settled_within(10_000, &[]), None);
    let mut later = [9_000i64; 17];
    later[0] = 9_000;
    assert_eq!(
        settled_within(10_000, &later),
        Some(17),
        "the sixteenth fails against the prior, the seventeenth holds against the first"
    );
    let mut falling: Vec<i64> = (0..80i64)
        .map(|k| 10_000i64.saturating_sub(k.saturating_mul(50)))
        .collect();
    assert_eq!(
        settled_within(10_000, &falling),
        None,
        "a fall of fifty a window, above the tolerance at every window"
    );
    falling.extend([6_050i64; 16]);
    assert_eq!(
        settled_within(10_000, &falling),
        Some(96),
        "then flat: the sixteenth flat window, against the last of the fall"
    );
    assert_eq!(
        lead_in_sums(&[(0, [0; 2], 1, 2, 0, 0, 0), (0, [0; 2], 3, 4, 0, 0, 0)]),
        [2, 4]
    );
    // The pick: the first candidate passing all three measures, in the candidates' order;
    // none when none does. Over readings written by hand from the pinned tables' shapes: a
    // block that fires the volley whole and sees, with a composition whose after-count is
    // quiet and whose signs are at the mark.
    let sees: Block = (
        0,
        34,
        [[800, 800], [800, 800]],
        [1_734, 1_530],
        [0; 2],
        [300, 300],
        SEEN_MIN,
        0,
        0,
        0,
        [[0; 2]; 2],
        0,
    );
    let quiet_signed: Composition = ([[0; 2]; 2], [[[0; 4]; 2]; 2], [[0; 2]; 3], SIGN_MIN, 0);
    let mut loud = quiet_signed;
    loud.2[2][1] = 64 * 51 / 10 + 1;
    let mut unsigned = quiet_signed;
    unsigned.3 = SIGN_MIN - 1;
    let mut blind = sees;
    blind.6 = SEEN_MIN - 1;
    assert!(background_passes(&sees, &quiet_signed));
    assert!(!background_passes(&sees, &loud), "the after clause");
    assert!(!background_passes(&sees, &unsigned), "the sign");
    assert!(!background_passes(&blind, &quiet_signed), "the sight");
    assert_eq!(
        background_pick(&[(sees, 0, quiet_signed), (sees, 0, quiet_signed)]),
        Some(0),
        "the settled network first"
    );
    assert_eq!(
        background_pick(&[(sees, 0, unsigned), (sees, 0, quiet_signed)]),
        Some(CONTROL_STEP_EIGHTH),
        "the controller when the settled network fails"
    );
    assert_eq!(
        background_pick(&[(sees, 0, unsigned), (blind, 0, quiet_signed)]),
        None
    );
    assert_eq!(background_pick(&[]), None);
    // The prediction's clauses at their edges: a gain at 1.75 is not above it; a course of
    // sixteen equal gains is settled; readouts equal are not more; a run that passes fails
    // the fourth.
    let window =
        |gain: u32, readouts: [u64; 2]| -> LeadInWindow { (0, readouts, 0, 0, 0, gain, 0) };
    let at_gain: Vec<LeadInWindow> = (0..16).map(|_| window(GAIN_1024, [10, 10])).collect();
    let above: Vec<LeadInWindow> = vec![window(GAIN_1024 + 1, [11, 10])];
    assert_eq!(
        controller_read(&at_gain, &at_gain, &(sees, 0, quiet_signed)),
        [false, false, false, false]
    );
    assert_eq!(
        controller_read(&above, &at_gain, &(sees, 0, unsigned)),
        [true, true, true, true]
    );
    assert_eq!(CONTROLLER_PREDICTED, [true; 4]);
    // Over the pinned tables. Each lead-in is the window at which the criterion first held,
    // as the constant says, or the bound; the settled network's gain stayed at 1.75 through
    // its lead-in and the controller's rose above it; each quiet run stayed within a window
    // and left the sums where the constant says.
    for (k, table) in BACKGROUND_LEAD_IN_1024.iter().enumerate() {
        let settled = settled_within(PRIOR_SUMS_1024.1, &lead_in_sums(table));
        assert_eq!(
            settled, SETTLED_AT_1024[k],
            "candidate {k}: where the criterion held"
        );
        assert_eq!(
            table.len() as u64,
            settled.map_or(LEAD_IN_EXTENDED, |w| w as u64),
            "candidate {k}: the lead-in is the first settled window, or the bound"
        );
        assert!(
            QUIET_1024[k].0 < QUIET_BOUND,
            "candidate {k}: quiet within a window"
        );
        let last = table.last().expect("a window");
        eprintln!(
            "DUMP background1024 {k} windows {} settled {settled:?} last {last:?} quiet {:?}",
            table.len(),
            QUIET_1024[k]
        );
    }
    assert!(
        SETTLED_AT_1024[0].is_some_and(|w| w as u64 > LEAD_IN_BOUND),
        "the settled lead-in ran on past the eightieth window under the extended bound"
    );
    assert!(
        SETTLED_AT_1024[1].is_some_and(|w| w as u64 <= LEAD_IN_BOUND),
        "the controller's settled within the eightieth"
    );
    assert!(
        BACKGROUND_LEAD_IN_1024[0].iter().all(|w| w.5 == GAIN_1024),
        "the settled network's gain is held"
    );
    assert!(
        BACKGROUND_LEAD_IN_1024[1].iter().all(|w| w.5 > GAIN_1024),
        "the controller's gain is above 1.75 after every window"
    );
    // The frozen runs: the signs are the rows', the A trials the counts', the three measures
    // and the pick as written, the prediction as read, and Deliverable D's readings.
    for (k, (block, _, composed)) in BACKGROUND_1024.iter().enumerate() {
        let rows = &BACKGROUND_ROWS_1024[k];
        let counted = &BACKGROUND_COUNTED_1024[k];
        let signs = rows.iter().filter(|r| r.1 > 0).count() as u32;
        assert_eq!(signs, composed.3, "candidate {k}: the signs are the rows'");
        assert_eq!(
            counted.iter().filter(|c| c.0 == 0).count() as u32,
            block.1,
            "candidate {k}: the A trials"
        );
        eprintln!(
            "DUMP background1024 {k} volley {:?} after {} seen {} signs {} fires_once {} sight {} sign {} passes {} bias {:?} offset {:?}",
            block.3,
            composed.2[2][1],
            block.6,
            composed.3,
            fires_once(1024, block, composed),
            calibrated(block),
            composed.3 >= SIGN_MIN,
            background_passes(block, composed),
            bias(counted, false),
            offset(counted, false, OFFSET_MARK_64)
        );
        assert_eq!(
            [
                fires_once(1024, block, composed),
                calibrated(block),
                composed.3 >= SIGN_MIN,
            ],
            BACKGROUND_MEASURES_1024[k],
            "candidate {k}: the three measures as written"
        );
    }
    assert_eq!(
        background_pick(&BACKGROUND_1024),
        BACKGROUND_PICKED_1024,
        "the pick as written"
    );
    assert_eq!(
        controller_read(
            BACKGROUND_LEAD_IN_1024[1],
            BACKGROUND_LEAD_IN_1024[0],
            &BACKGROUND_1024[1]
        ),
        CONTROLLER_READ,
        "the prediction as read"
    );
    assert_eq!(
        [
            offset(&BACKGROUND_COUNTED_1024[0], false, OFFSET_MARK_64),
            offset(&BACKGROUND_COUNTED_1024[1], false, OFFSET_MARK_64),
        ],
        OFFSETS_1024,
        "the offset the criterion needs under each candidate"
    );
    assert_eq!(
        [
            bias(&BACKGROUND_COUNTED_1024[0], false),
            bias(&BACKGROUND_COUNTED_1024[1], false),
        ],
        BIASES_1024,
        "the instrument's bias under each candidate"
    );
    // The first two windows of the settled lead-in, run and held to the first two rows of
    // its table.
    let mut exec = candidate(1024, BACKGROUNDS[0]);
    assert_eq!(weights_by_polarity(&exec), PRIOR_SUMS_1024);
    let table = lead_in_until_settled(&mut exec, 1024, PRIOR_SUMS_1024.1, GATE_WINDOWS);
    eprintln!("DUMP settled1024 first two {table:?}");
    assert_eq!(
        table.as_slice(),
        &BACKGROUND_LEAD_IN_1024[0][..GATE_WINDOWS as usize],
        "the first two windows of the settled lead-in"
    );
}

// ------------------------------------------------ written before the run (brief 036)

/// H-13's network (ADR-0078): ADR-0077's settled candidate, `BACKGROUNDS[SETTLED]`, the gain
/// held at 1.75 and the controller off; its image built as `background_candidate` builds it
/// and held to ADR-0077's tables step by step before any rewarded run.
const SETTLED: usize = 0;
const _: () = assert!(BACKGROUNDS[SETTLED] == 0);
/// The arms of H-13, in the order run and no other: the reward withheld — the frozen run of
/// `TRIALS` trials from the image, whose first block is the calibration and whose every trial
/// is the reference a rewarded arm's trial is paired with — then the assignment (A onto
/// readout 0, B onto readout 1), then the mirrored assignment (A onto 1, B onto 0).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Arm {
    Withheld,
    Assignment,
    Mirrored,
}
const ARMS: [Arm; 3] = [Arm::Withheld, Arm::Assignment, Arm::Mirrored];
/// The rewarded arms in `ARMS`'s order, the criterion's `[assignment, mirrored]`.
const REWARDED_ARMS: [Arm; 2] = [Arm::Assignment, Arm::Mirrored];
/// ADR-0078's prediction for the couplings clause, a Hypothesis written before the run: it
/// holds in both rewarded arms, since ADR-0077's trace is positive after 53 trials of 64 and
/// every pair's sum after its frozen block is positive. No prediction is written for the
/// response clause.
const COUPLINGS_PREDICTED: [bool; 2] = [true, true];
/// The dopamine signal under a reward of `REWARD_Q16` after every trial, by the two rules
/// alone (`NeuromodulatorState::reward`, saturating, and `decay_dopamine` by
/// `DOPAMINE_TAU_SHIFT` once per tick), computed apart from the tree before the run and held
/// here to the oracle below, which every run holds to the record: after the first reward 1.0
/// and at the first trial's end 0.458; the per-trial map's fixed point 1.712 after a reward and
/// 0.712 at a trial's end, reached within eleven trials; from it the addressed modulation is at
/// the ceiling for 9 694 ticks of the trial's 16 384 and never below 0.712. ADR-0078's
/// arithmetic from a decay of exactly $2^{-14}$ per tick said about 1.58 and 0.58: the rule's
/// step is the floor of that fraction, so below every power of two the signal decays more
/// slowly than the exponent, and the reading is the record's.
const SIGNAL_END_FIRST_Q16: i32 = 30_036;
const SIGNAL_AFTER_FIXED_Q16: i32 = 112_227;
const SIGNAL_END_FIXED_Q16: i32 = 46_691;
const SIGNAL_FIXED_WITHIN_TRIALS: usize = 11;
const SIGNAL_CEILING_TICKS_FIXED: u32 = 9_694;
const _: () = assert!(SIGNAL_AFTER_FIXED_Q16 == SIGNAL_END_FIXED_Q16 + ONE);
const _: () = assert!(SIGNAL_FIXED_WITHIN_TRIALS < BLOCK);

// ------------------------------------------------------------- the oracle (brief 036)

/// The dopamine signal one tick on, as the executor decays it after publishing each tick's
/// modulations (`decay_dopamine` with `DOPAMINE_TAU_SHIFT`, ADR-0032): toward zero by the
/// floor of $2^{-14}$ of itself and by at least one LSB, so that it reaches rest exactly;
/// written a second time as the oracle's.
fn decayed_signal(signal: i32) -> i32 {
    let d = i64::from(signal);
    let step = (d.abs() >> DOPAMINE_TAU_SHIFT).max(1).min(d.abs());
    d.saturating_sub(d.signum().saturating_mul(step)) as i32
}

/// The signal's course over one trial from `first`, the signal at the trial's first tick:
/// entry `k` is the signal the trial's `k`-th tick publishes its modulation from, and entry
/// `TRIAL_TICKS` the signal at the trial's end, `TRIAL_TICKS` decays on.
fn signal_course(first: i32) -> Vec<i32> {
    let mut course = Vec::with_capacity(TRIAL_TICKS as usize);
    let mut signal = first;
    for _ in 0..TRIAL_TICKS {
        course.push(signal);
        signal = decayed_signal(signal);
    }
    course.push(signal);
    course
}

/// The signal at the trial's end, the course's last entry.
fn signal_end(course: &[i32]) -> i32 {
    course.last().copied().unwrap_or(0)
}

/// The ticks of a course at which the modulation is at the ceiling, the signal at or above
/// 1.0.
fn ceiling_ticks(course: &[i32]) -> u32 {
    course
        .iter()
        .take(TRIAL_TICKS as usize)
        .filter(|&&s| s >= ONE)
        .count() as u32
}

/// The consolidation of one excitatory slot at a presynaptic spike, as `cortex-core`'s
/// `consolidate` moves it (ADR-0032, ADR-0049): `round(|trace| × m)` of the trace, signed as
/// the trace, into the magnitude, clamped to $[0, 2^{15})$, and what the magnitude absorbed
/// taken out of the trace, `m` clamped to $[0, 1]$. Returns the trace after, the magnitude
/// after and the amount absorbed; written a second time as the oracle's.
fn consolidated(trace: i16, magnitude: i32, modulation_q16: i32) -> (i16, i32, i32) {
    let m = i64::from(modulation_q16.clamp(0, ONE));
    let trace = i32::from(trace);
    let amount = (i64::from(trace)
        .abs()
        .saturating_mul(m)
        .saturating_add(0x8000)
        >> 16) as i32;
    let transfer = amount.saturating_mul(trace.signum());
    let before = magnitude.max(0);
    let after = before
        .saturating_add(transfer)
        .clamp(0, i32::from(i16::MAX));
    let absorbed = after.saturating_sub(before);
    (trace.saturating_sub(absorbed) as i16, after, absorbed)
}

/// The delivery as the oracle replays it (brief 036): the pair the last trial's delivery
/// addressed — the stimulus presented and its assigned readout under the taught delivery,
/// the readout the engine selected under the task's own (brief 037), none at a tie — and the
/// signal that delivery's reward left, which is the signal at the next trial's first tick;
/// before the first delivery none and zero.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Taught {
    addressed: Option<(usize, usize)>,
    signal: i32,
}

// ---------------------------------------------------------- the criterion (brief 036)

/// One trial's reading under an arm (brief 036): the stimulus presented; the readouts'
/// counts in the task's window, as the task read them; the selection; the trace consolidated
/// into the weights over the trial, `[stimulus][readout]`, by the oracle; and the dopamine
/// signal at the trial's end and after the delivery's reward (the same, in the withheld arm).
type TaughtTrial = (u8, [u32; 2], Option<u8>, [[i64; 2]; 2], i32, i32);

/// One block of an arm (brief 036): the paired tallies against the withheld arm over the
/// block — the trials in which the presented stimulus's assigned readout counted more than at
/// the same trial of the withheld arm, and the trials in which the other readout did — the
/// selections of the assigned readout, the ties, the trace consolidated into the weights over
/// the block, `[stimulus][readout]`, and the signal at the block's last trial's end and after
/// its reward.
type TaughtBlock = ([u32; 2], u32, u32, [[i64; 2]; 2], i32, i32);

/// Whether an arm rewards, and whether it mirrors the assignment.
fn rewards(arm: Arm) -> bool {
    arm != Arm::Withheld
}

fn mirrors(arm: Arm) -> bool {
    arm == Arm::Mirrored
}

/// The assigned pairs of an assignment, `(stimulus, readout)` for A and for B.
fn assigned_pairs(mirrored: bool) -> [(usize, usize); 2] {
    [(0, answer_of(0, mirrored)), (1, answer_of(1, mirrored))]
}

/// A readout set of the geometry by its index.
fn readout_set(sets: &[Set; 4], readout: usize) -> Set {
    if readout == 0 { sets[2] } else { sets[3] }
}

/// The paired comparison of a rewarded arm's trial with the withheld arm's (ADR-0078): for
/// the presented stimulus's assigned readout, then for the other readout, whether the arm's
/// count exceeds the withheld arm's at the same trial. A tie is not more.
fn paired(rewarded: &TaughtTrial, withheld: &TaughtTrial, mirrored: bool) -> [bool; 2] {
    assert_eq!(
        rewarded.0, withheld.0,
        "the same trial presents the same stimulus"
    );
    let assigned = answer_of(rewarded.0, mirrored);
    let other = assigned ^ 1;
    [
        rewarded.1[assigned] > withheld.1[assigned],
        rewarded.1[other] > withheld.1[other],
    ]
}

/// An arm's blocks from its trials and the withheld arm's, `BLOCK` trials each: the paired
/// tallies, the selections of the assigned readout, the ties, the consolidation summed and
/// the last trial's signals. The withheld arm against itself tallies nothing, since no count
/// exceeds itself.
fn taught_blocks(
    read: &[TaughtTrial],
    withheld: &[TaughtTrial],
    mirrored: bool,
) -> Vec<TaughtBlock> {
    assert_eq!(read.len(), withheld.len(), "the arms run the same trials");
    read.chunks(BLOCK)
        .zip(withheld.chunks(BLOCK))
        .map(|(mine, theirs)| {
            let mut tallies = [0u32; 2];
            let mut selections = 0u32;
            let mut ties = 0u32;
            let mut transferred = [[0i64; 2]; 2];
            for (r, w) in mine.iter().zip(theirs.iter()) {
                let more = paired(r, w, mirrored);
                tallies[0] = tallies[0].saturating_add(u32::from(more[0]));
                tallies[1] = tallies[1].saturating_add(u32::from(more[1]));
                let assigned = answer_of(r.0, mirrored) as u8;
                selections = selections.saturating_add(u32::from(r.2 == Some(assigned)));
                ties = ties.saturating_add(u32::from(r.2.is_none()));
                for (s, readouts) in r.3.iter().enumerate() {
                    for (k, &amount) in readouts.iter().enumerate() {
                        transferred[s][k] = transferred[s][k].saturating_add(amount);
                    }
                }
            }
            let last = mine.last().expect("a block holds a trial");
            (tallies, selections, ties, transferred, last.4, last.5)
        })
        .collect()
}

/// The response clause's count: the assigned readout's paired tally over the last
/// `LAST_BLOCKS` blocks.
fn last_paired(blocks: &[TaughtBlock]) -> u32 {
    blocks
        .iter()
        .rev()
        .take(LAST_BLOCKS)
        .fold(0u32, |sum, b| sum.saturating_add(b.0[0]))
}

/// The selections of the assigned readout over the last `LAST_BLOCKS` blocks, a reading
/// beside the response clause and no clause.
fn last_selected(blocks: &[TaughtBlock]) -> u32 {
    blocks
        .iter()
        .rev()
        .take(LAST_BLOCKS)
        .fold(0u32, |sum, b| sum.saturating_add(b.1))
}

/// The couplings clause: each assigned pair's excitatory coupling sum after the arm's last
/// block above the image's, strict; false for an arm of no block.
fn couplings_rose(image: &[[i64; 2]; 2], blocks: &[Block], mirrored: bool) -> bool {
    blocks.last().is_some_and(|last| {
        assigned_pairs(mirrored)
            .iter()
            .all(|&(s, r)| last.10[s][r] > image[s][r])
    })
}

/// H-13's criterion (ADR-0078), clause by clause in each rewarded arm, `[assignment,
/// mirrored]`: the couplings (`couplings_rose`) and the response (`last_paired` at least
/// `REWARDED_MIN`). `yes` is all four.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Direction {
    couplings: [bool; 2],
    response: [bool; 2],
    yes: bool,
}

fn direction(image: &[[i64; 2]; 2], arms: [(&[Block], &[TaughtBlock]); 2]) -> Direction {
    let couplings = [
        couplings_rose(image, arms[0].0, mirrors(REWARDED_ARMS[0])),
        couplings_rose(image, arms[1].0, mirrors(REWARDED_ARMS[1])),
    ];
    let response = [
        last_paired(arms[0].1) >= REWARDED_MIN,
        last_paired(arms[1].1) >= REWARDED_MIN,
    ];
    Direction {
        couplings,
        response,
        yes: couplings[0] && couplings[1] && response[0] && response[1],
    }
}

// ---------------------------------------------------------------- the run (brief 036)

/// An arm's run: the blocks, the accuracy-sequence trace, the composed trials, every trial's
/// reading, the volley-tick census over the run, and the first block's trace and census
/// (the calibration's, held to ADR-0077's).
type TaughtRun = (
    Vec<Block>,
    u64,
    Vec<Composed>,
    Vec<TaughtTrial>,
    Vec<u64>,
    (u64, Vec<u64>),
);

/// The word `run_on` hashes for a trial, `(stimulus, selection, correct)`, written a second
/// time so that a block of a longer run can be held to the trace of a run of that block.
fn sequence_word(outcome: &Outcome) -> i32 {
    i32::from(outcome.stimulus)
        | i32::from(outcome.selection.map_or(3, |r| r)) << 1
        | i32::from(outcome.correct) << 3
}

/// The FNV-1a hash of every trial's reading, each number as its `i32` words.
fn taught_hash(read: &[TaughtTrial]) -> u64 {
    let mut words: Vec<i32> = Vec::new();
    let mut wide = |x: i64| {
        words.push(x as i32);
        words.push((x >> 32) as i32);
    };
    for t in read {
        wide(i64::from(t.0));
        wide(i64::from(t.1[0]));
        wide(i64::from(t.1[1]));
        wide(t.2.map_or(3, i64::from));
        for readouts in &t.3 {
            for &amount in readouts {
                wide(amount);
            }
        }
        wide(i64::from(t.4));
        wide(i64::from(t.5));
    }
    fnv1a_64(&words)
}

/// An arm's run from the frozen engine (brief 036): `trials` trials of the calibration's task
/// with ADR-0076's stimulus, the addressed delivery and the reward withheld by the task, the
/// composer seeded from the record and replaying the consolidation; after every trial of a
/// rewarded arm the taught delivery — the addressed set rewritten, between the trial's last
/// tick and the reward, to the synapses from the presented stimulus's units onto its assigned
/// readout's units, whatever the selection, then `REWARD_Q16` into the signal — and after
/// every trial of the withheld arm nothing, the task's own addressing standing with the signal
/// at rest. At every trial the record's traces and weights are held to the oracle and the
/// signal at the trial's end to its course; after every block of the withheld arm the sums by
/// polarity are asserted unchanged.
fn taught_run(exec: &mut Engine, arm: Arm, units: u32, trials: usize) -> TaughtRun {
    assert_eq!(
        units, 1024,
        "the cancel and the synapse counts below are pinned at 1 024 units"
    );
    assert_eq!(
        exec.modulation_baseline_q16(),
        0,
        "the baseline is the image's zero"
    );
    assert_eq!(
        exec.modulator().dopamine_rpe,
        0,
        "the signal is at rest before the first trial"
    );
    let sums = weights_by_polarity(exec);
    let sets = geometry(units, rotation(units));
    let mut composer = Composer::new(units);
    composer.enumerate(exec);
    composer.cursor = exec.ticks() as u32;
    if rewards(arm) {
        composer.taught = Some(Taught {
            addressed: None,
            signal: 0,
        });
    }
    let mirrored = mirrors(arm);
    let picked = CANCEL_PICKED_1024.expect("ADR-0076 picked a cancel");
    let task = task(
        SHAPE_F46,
        Some(cancel_of(picked)),
        units,
        Feedback::Withheld,
        mirrored,
        Delivery::Addressed,
    );
    let mut read: Vec<TaughtTrial> = Vec::with_capacity(trials);
    let mut sequence: Vec<i32> = Vec::with_capacity(trials);
    let mut first_block: (u64, Vec<u64>) = (0, Vec::new());
    let (blocks, trace) = run_on(
        exec,
        task,
        units,
        trials,
        &mut |exec, trial, start, outcome| {
            composer.observe(exec, trial, start, outcome);
            assert_eq!(
                outcome.selection,
                selected(outcome.counts),
                "trial {trial}: the selection is the sign of the count difference"
            );
            let end = exec.modulator().dopamine_rpe;
            assert_eq!(
                outcome.signal_q16, end,
                "trial {trial}: the task read the signal at the trial's end"
            );
            let transferred = composer.transferred.last().copied().unwrap_or([[0; 2]; 2]);
            let after = if rewards(arm) {
                let stimulus = usize::from(outcome.stimulus);
                let assigned = answer_of(outcome.stimulus, mirrored);
                let targets = readout_set(&sets, assigned);
                exec.address(sets[stimulus].units(), targets.units())
                    .expect("the sets are inside the arena");
                assert_eq!(
                    exec.addressed_counts(),
                    (sets[stimulus].len() as usize, targets.len() as usize),
                    "trial {trial}: the addressed set is the presented stimulus onto its assigned readout"
                );
                let after = exec.reward(REWARD_Q16);
                composer.taught = Some(Taught {
                    addressed: Some((stimulus, assigned)),
                    signal: after,
                });
                after
            } else {
                assert_eq!(
                    end, 0,
                    "trial {trial}: the withheld arm's signal stays at rest"
                );
                end
            };
            sequence.push(sequence_word(outcome));
            read.push((
                outcome.stimulus,
                outcome.counts,
                outcome.selection,
                transferred,
                end,
                after,
            ));
            if sequence.len() == BLOCK {
                first_block = (fnv1a_64(&sequence), composer.volley_ticks.clone());
            }
        },
    );
    assert_eq!(fnv1a_64(&sequence), trace, "the sequence is the run's");
    if !rewards(arm) {
        for block in &blocks {
            assert_eq!(
                (block.7, block.8),
                sums,
                "no weight moves with the reward withheld"
            );
        }
    }
    assert_eq!(composer.out.len(), trials);
    assert_eq!(composer.counts(), SYNAPSES_1024);
    (
        blocks,
        trace,
        composer.out,
        read,
        composer.volley_ticks,
        first_block,
    )
}

/// Every weight of the arena in the arena's order, block by block: what the reach assertion
/// compares bit for bit.
fn weights_of(exec: &Engine) -> Vec<Vec<i16>> {
    exec.blocks()
        .iter()
        .map(|b| b.weights_q1_15.to_vec())
        .collect()
}

/// The reach of a delivery over the arena (brief 036): the synapses whose weight differs
/// from `before`, counted inside `pairs` — a synapse from the stimulus set onto the readout
/// set a pair names — and outside them. Every occupied slot is walked once, through its
/// unit's chain.
fn reach(exec: &Engine, before: &[Vec<i16>], units: u32, pairs: &[(usize, usize)]) -> (u64, u64) {
    let sets = geometry(units, rotation(units));
    let mut inside = 0u64;
    let mut outside = 0u64;
    for unit in exec.units() {
        let id = unit.id as u32;
        for s in unit.fan_out(exec.blocks()) {
            let was = before
                .get(s.block_idx as usize)
                .and_then(|b| b.get(usize::from(s.slot)))
                .copied();
            if was == Some(s.weight_q1_15) {
                continue;
            }
            let in_pair = pairs.iter().any(|&(stimulus, readout)| {
                sets[stimulus].contains(id) && readout_set(&sets, readout).contains(s.target)
            });
            if in_pair {
                inside = inside.saturating_add(1);
            } else {
                outside = outside.saturating_add(1);
            }
        }
    }
    (inside, outside)
}

/// The settled image (brief 036): ADR-0077's settled candidate built as `background_candidate`
/// builds it, each step held to ADR-0077's pinned tables — the lead-in to its table and its
/// length, the quiet run to its ticks and sums, the image's sums, gain and step carried — and
/// its bytes, the one image every arm decodes. A mismatch stops the round here, before any
/// rewarded run (H-13's stopping rule, step 2).
fn settled_image(name: &str) -> Vec<u8> {
    let step = BACKGROUNDS[SETTLED];
    let mut exec = candidate(1024, step);
    assert_eq!(weights_by_polarity(&exec), PRIOR_SUMS_1024);
    let table = lead_in_until_settled(&mut exec, 1024, PRIOR_SUMS_1024.1, LEAD_IN_EXTENDED);
    let settled = settled_within(PRIOR_SUMS_1024.1, &lead_in_sums(&table));
    eprintln!(
        "DUMP {name} lead-in settled {settled:?} windows {} last {:?}",
        table.len(),
        table.last()
    );
    assert_eq!(
        table.as_slice(),
        BACKGROUND_LEAD_IN_1024[SETTLED],
        "{name}: the lead-in is ADR-0077's"
    );
    assert_eq!(
        settled, SETTLED_AT_1024[SETTLED],
        "{name}: the criterion holds where ADR-0077 read it"
    );
    let ticks = quiet(&mut exec);
    let quieted = weights_by_polarity(&exec);
    eprintln!("DUMP {name} quiet {ticks} sums {quieted:?}");
    assert_eq!(
        (ticks, quieted),
        QUIET_1024[SETTLED],
        "{name}: the quiet run is ADR-0077's"
    );
    let image = frozen_image(&exec);
    let frozen = frozen_from(&image, 1024);
    assert_eq!(
        weights_by_polarity(&frozen),
        quieted,
        "{name}: the image carries the weights"
    );
    assert_eq!(
        frozen.homeostasis().synaptic_gain_q16,
        GAIN_1024,
        "{name}: and the gain"
    );
    assert_eq!(
        frozen.homeostasis().control_step_q0_16,
        step,
        "{name}: and the step"
    );
    image
}

/// The calibration (brief 036): the withheld arm's first block held to ADR-0077's frozen run
/// of the settled candidate — the sight's block and trace, the rows and the composition (the
/// stimulus firing once, the sight 62, the sign 53 of 64), the counts and the volley's census
/// — before any rewarded run. The first block of a run of `TRIALS` trials from the image is
/// that run of `BLOCK` trials, the inputs being the same.
fn calibration_holds(name: &str, run: &TaughtRun) {
    let (blocks, _, trials, read, _, (first_trace, first_census)) = run;
    let (block, pin, composed) = &BACKGROUND_1024[SETTLED];
    assert_eq!(
        blocks.first(),
        Some(block),
        "{name}: the first block is ADR-0077's"
    );
    assert_eq!(
        *first_trace, *pin,
        "{name}: the first block's sequence is ADR-0077's"
    );
    let first = trials.get(..BLOCK).expect("a block of trials");
    pinned_composition(name, first, &BACKGROUND_ROWS_1024[SETTLED], Some(composed));
    let counted: Vec<Counted> = read.iter().take(BLOCK).map(|t| (t.0, t.1)).collect();
    assert_eq!(
        counted.as_slice(),
        &BACKGROUND_COUNTED_1024[SETTLED],
        "{name}: the counts are ADR-0077's"
    );
    assert_eq!(
        census_of(first_census),
        BACKGROUND_CENSUS_1024[SETTLED].to_vec(),
        "{name}: the volley's ticks are ADR-0077's"
    );
    let read_measures = [
        fires_once(1024, block, composed),
        calibrated(block),
        composed.3 >= SIGN_MIN,
    ];
    assert_eq!(
        read_measures, BACKGROUND_MEASURES_1024[SETTLED],
        "{name}: the three measures as ADR-0077 read them"
    );
}

/// Dumps an arm's run: the sight's blocks and trace, the rows, the composition per block, the
/// taught blocks, every trial's reading with its hash, and the census.
fn dump_direction(name: &str, run: &TaughtRun, taught: &[TaughtBlock]) {
    let (blocks, trace, trials, read, volley_ticks, _) = run;
    eprintln!(
        "DUMP {name} sight {blocks:?} trace {trace:#018x} curve {:?}",
        curve(blocks)
    );
    let rows: Vec<Row> = trials.iter().map(trial_row).collect();
    eprintln!("DUMP {name} rows {rows:?}");
    let compositions: Vec<Composition> = trials.chunks(BLOCK).map(composition).collect();
    eprintln!("DUMP {name} compositions {compositions:?}");
    eprintln!("DUMP {name} taught {taught:?}");
    eprintln!("DUMP {name} read {read:?} hash {:#018x}", taught_hash(read));
    eprintln!("DUMP {name} census {:?}", census_of(volley_ticks));
}

/// Holds an arm's run to its pinned tables: the blocks and the trace, the composition per
/// block, the taught blocks, the readings' hash and the census.
fn pinned_direction(name: &str, k: usize, run: &TaughtRun, taught: &[TaughtBlock]) {
    let (blocks, trace, trials, read, volley_ticks, _) = run;
    pinned(
        &format!("{name} sight"),
        blocks,
        *trace,
        DIRECTION_BLOCKS_1024[k],
        DIRECTION_TRACES_1024[k],
    );
    let compositions: Vec<Composition> = trials.chunks(BLOCK).map(composition).collect();
    assert_eq!(
        compositions.as_slice(),
        DIRECTION_COMPOSITIONS_1024[k],
        "{name}: the composition per block"
    );
    assert_eq!(
        taught, DIRECTION_TAUGHT_1024[k],
        "{name}: the taught blocks"
    );
    assert_eq!(
        taught_hash(read),
        DIRECTION_READ_1024[k],
        "{name}: the readings"
    );
    assert_eq!(
        census_of(volley_ticks),
        DIRECTION_CENSUS_1024[k].to_vec(),
        "{name}: the volley's ticks"
    );
}

/// H-13 at 1 024 units (brief 036): the settled image, held to ADR-0077 step by step; the
/// reward withheld over `TRIALS` trials, its first block the calibration, held to ADR-0077's
/// frozen run before any rewarded run; then the assignment and the mirrored assignment, each
/// from the one image, each held after its run to the delivery's reach — every synapse outside
/// the arm's two assigned pairs as the image holds it, bit for bit, and a synapse inside moved
/// — and every arm dumped before anything is held to its table, then held; the criterion's
/// verdict computed by the rules and held to the constant written from it.
#[test]
#[ignore]
fn the_rewards_direction_at_1024_units_exhaustive() {
    let name = "direction1024";
    let image = settled_image(name);
    let sets = geometry(1024, ROTATION_1024);
    let mut runs: Vec<TaughtRun> = Vec::with_capacity(ARMS.len());
    let mut image_couplings = [[0i64; 2]; 2];
    for (k, &arm) in ARMS.iter().enumerate() {
        let arm_name = format!("{name} {arm:?}");
        let mut exec = frozen_from(&image, 1024);
        let before = weights_of(&exec);
        let couplings_before = [
            [
                coupling(&exec, sets[0], sets[2]),
                coupling(&exec, sets[0], sets[3]),
            ],
            [
                coupling(&exec, sets[1], sets[2]),
                coupling(&exec, sets[1], sets[3]),
            ],
        ];
        if k == 0 {
            image_couplings = couplings_before;
            assert_eq!(
                image_couplings, BACKGROUND_1024[SETTLED].0.10,
                "the image's couplings are the frozen block's"
            );
        } else {
            assert_eq!(
                couplings_before, image_couplings,
                "{arm_name}: the same image"
            );
        }
        let run = taught_run(&mut exec, arm, 1024, TRIALS);
        let pairs: Vec<(usize, usize)> = if rewards(arm) {
            assigned_pairs(mirrors(arm)).to_vec()
        } else {
            Vec::new()
        };
        let (inside, outside) = reach(&exec, &before, 1024, &pairs);
        eprintln!(
            "DUMP {arm_name} reach inside {inside} outside {outside} signal after the run {:#x}",
            exec.modulator().dopamine_rpe
        );
        if k == 0 {
            calibration_holds(&arm_name, &run);
            eprintln!("DUMP {name} calibration holds: ADR-0077's settled candidate reproduced");
        }
        assert_eq!(
            outside, 0,
            "{arm_name}: every synapse outside the assigned pairs is the image's"
        );
        if rewards(arm) {
            assert!(inside > 0, "{arm_name}: the delivery reached a synapse");
        } else {
            assert_eq!(inside, 0, "{arm_name}: no synapse moved");
        }
        runs.push(run);
    }
    let withheld = &runs[0].3;
    let mut tables: Vec<Vec<TaughtBlock>> = Vec::with_capacity(ARMS.len());
    for (k, &arm) in ARMS.iter().enumerate() {
        let arm_name = format!("{name} {arm:?}");
        let taught = taught_blocks(&runs[k].3, withheld, mirrors(arm));
        dump_direction(&arm_name, &runs[k], &taught);
        tables.push(taught);
    }
    let verdict = direction(
        &image_couplings,
        [(&runs[1].0, &tables[1]), (&runs[2].0, &tables[2])],
    );
    let read_couplings = [
        couplings_rose(&image_couplings, &runs[1].0, mirrors(REWARDED_ARMS[0])),
        couplings_rose(&image_couplings, &runs[2].0, mirrors(REWARDED_ARMS[1])),
    ];
    eprintln!(
        "DUMP {name} verdict {verdict:?} couplings predicted {COUPLINGS_PREDICTED:?} read {read_couplings:?} paired {:?} selected {:?} image couplings {image_couplings:?}",
        [last_paired(&tables[1]), last_paired(&tables[2])],
        [last_selected(&tables[1]), last_selected(&tables[2])]
    );
    for (k, &arm) in ARMS.iter().enumerate() {
        pinned_direction(&format!("{name} {arm:?}"), k, &runs[k], &tables[k]);
    }
    assert_eq!(verdict, DIRECTION_1024, "the verdict as written");
}

// ----------------------------------------------------------- the measurement (brief 036)

/// The three arms at 1 024 units, in `ARMS`'s order, each pinned from one run: the sight's
/// blocks and trace, the composition per block, the taught blocks, the readings' hash and the
/// volley's census.
const DIRECTION_BLOCKS_1024: [&[Block]; 3] = [
    &[
        (
            29,
            34,
            [[330, 323], [315, 314]],
            [1728, 1525],
            [2491, 2252],
            [258, 256],
            62,
            165876268,
            218243354,
            0,
            [[6249552, 6698611], [6584205, 6815470]],
            8,
        ),
        (
            29,
            31,
            [[284, 309], [327, 332]],
            [1574, 1677],
            [2397, 2398],
            [242, 228],
            64,
            165876268,
            218243354,
            0,
            [[6249552, 6698611], [6584205, 6815470]],
            7,
        ),
        (
            25,
            31,
            [[287, 327], [335, 340]],
            [1570, 1678],
            [2384, 2380],
            [237, 226],
            63,
            165876268,
            218243354,
            0,
            [[6249552, 6698611], [6584205, 6815470]],
            5,
        ),
        (
            24,
            28,
            [[249, 336], [351, 377]],
            [1427, 1829],
            [2258, 2565],
            [247, 251],
            64,
            165876268,
            218243354,
            0,
            [[6249552, 6698611], [6584205, 6815470]],
            6,
        ),
        (
            29,
            30,
            [[283, 338], [326, 358]],
            [1524, 1724],
            [2332, 2390],
            [245, 255],
            63,
            165876268,
            218243354,
            0,
            [[6249552, 6698611], [6584205, 6815470]],
            8,
        ),
        (
            28,
            36,
            [[344, 368], [288, 307]],
            [1829, 1424],
            [2649, 2110],
            [263, 239],
            64,
            165876268,
            218243354,
            0,
            [[6249552, 6698611], [6584205, 6815470]],
            7,
        ),
        (
            30,
            30,
            [[282, 333], [358, 346]],
            [1520, 1728],
            [2310, 2432],
            [244, 257],
            63,
            165876268,
            218243354,
            0,
            [[6249552, 6698611], [6584205, 6815470]],
            4,
        ),
        (
            29,
            32,
            [[306, 340], [316, 337]],
            [1629, 1628],
            [2433, 2362],
            [253, 238],
            64,
            165876268,
            218243354,
            0,
            [[6249552, 6698611], [6584205, 6815470]],
            2,
        ),
    ],
    &[
        (
            32,
            34,
            [[339, 322], [315, 322]],
            [1728, 1525],
            [2491, 2255],
            [258, 257],
            62,
            165876268,
            218355944,
            112227,
            [[6284035, 6698611], [6584205, 6893577]],
            6,
        ),
        (
            32,
            31,
            [[316, 314], [334, 354]],
            [1574, 1677],
            [2399, 2398],
            [243, 228],
            64,
            165876268,
            218474361,
            112227,
            [[6330824, 6698611], [6584205, 6965205]],
            7,
        ),
        (
            27,
            31,
            [[324, 331], [344, 374]],
            [1570, 1678],
            [2379, 2384],
            [240, 226],
            64,
            165876268,
            218645864,
            112227,
            [[6401794, 6698611], [6584205, 7065738]],
            7,
        ),
        (
            34,
            28,
            [[313, 339], [359, 454]],
            [1427, 1829],
            [2262, 2567],
            [251, 251],
            64,
            165876268,
            218794658,
            112227,
            [[6439245, 6698611], [6584205, 7177081]],
            4,
        ),
        (
            43,
            30,
            [[376, 351], [330, 480]],
            [1524, 1724],
            [2330, 2391],
            [251, 258],
            63,
            165876268,
            219110202,
            112227,
            [[6545900, 6698611], [6584205, 7385970]],
            4,
        ),
        (
            49,
            36,
            [[507, 376], [296, 433]],
            [1829, 1424],
            [2648, 2115],
            [263, 240],
            64,
            165876268,
            219435608,
            112227,
            [[6735165, 6698611], [6584205, 7522111]],
            5,
        ),
        (
            46,
            30,
            [[436, 344], [374, 519]],
            [1520, 1728],
            [2311, 2430],
            [250, 259],
            64,
            165876268,
            219696028,
            112227,
            [[6838841, 6698611], [6584205, 7678855]],
            7,
        ),
        (
            60,
            32,
            [[553, 377], [334, 542]],
            [1629, 1628],
            [2435, 2371],
            [259, 236],
            64,
            165876268,
            219923242,
            112227,
            [[6988811, 6698611], [6584205, 7756099]],
            1,
        ),
    ],
    &[
        (
            28,
            34,
            [[330, 329], [323, 315]],
            [1728, 1525],
            [2491, 2251],
            [258, 256],
            62,
            165876268,
            218391844,
            112227,
            [[6249552, 6786836], [6644470, 6815470]],
            5,
        ),
        (
            33,
            31,
            [[285, 334], [363, 339]],
            [1574, 1677],
            [2396, 2398],
            [241, 229],
            64,
            165876268,
            218618093,
            112227,
            [[6249552, 6903534], [6754021, 6815470]],
            7,
        ),
        (
            41,
            31,
            [[291, 373], [382, 349]],
            [1570, 1678],
            [2379, 2378],
            [238, 228],
            63,
            165876268,
            218825745,
            112227,
            [[6249552, 7012368], [6852839, 6815470]],
            5,
        ),
        (
            44,
            28,
            [[250, 403], [438, 391]],
            [1427, 1829],
            [2255, 2565],
            [246, 251],
            64,
            165876268,
            219036885,
            112227,
            [[6249552, 7092753], [6983594, 6815470]],
            8,
        ),
        (
            45,
            30,
            [[286, 447], [443, 378]],
            [1524, 1724],
            [2336, 2388],
            [248, 252],
            64,
            165876268,
            219344217,
            112227,
            [[6249552, 7266213], [7117466, 6815470]],
            3,
        ),
        (
            49,
            36,
            [[345, 534], [426, 326]],
            [1829, 1424],
            [2650, 2115],
            [265, 242],
            64,
            165876268,
            219670172,
            112227,
            [[6249552, 7445611], [7264023, 6815470]],
            2,
        ),
        (
            60,
            30,
            [[279, 510], [540, 376]],
            [1520, 1728],
            [2311, 2429],
            [245, 264],
            64,
            165876268,
            219927313,
            112227,
            [[6249552, 7580045], [7386730, 6815470]],
            1,
        ),
        (
            55,
            32,
            [[314, 564], [502, 359]],
            [1629, 1628],
            [2432, 2363],
            [261, 238],
            64,
            165876268,
            220183131,
            112227,
            [[6249552, 7723009], [7499584, 6815470]],
            4,
        ),
    ],
];
const DIRECTION_TRACES_1024: [u64; 3] =
    [0x6882536eb2efe02b, 0x8efcd277e7eac46b, 0x9d97c7cae502242b];
const DIRECTION_COMPOSITIONS_1024: [&[Composition]; 3] = [
    &[
        (
            [[581, 11652], [6687, 10983]],
            [
                [
                    [214620, -5235, 318258, -506972],
                    [239131, -5599, 344531, -493105],
                ],
                [
                    [227460, -5372, 281672, -462205],
                    [208207, -4723, 325571, -442474],
                ],
            ],
            [[1100, 1646], [1009, 1645], [3350, 0]],
            53,
            5817133415553407245,
        ),
        (
            [[-2901, 6687], [-1382, -3770]],
            [
                [
                    [183614, -4848, 311320, -490757],
                    [242170, -5630, 322780, -451060],
                ],
                [
                    [236853, -6402, 299222, -460036],
                    [223791, -5576, 312100, -482400],
                ],
            ],
            [[988, 1567], [1008, 1609], [3354, 0]],
            50,
            16045209168826706265,
        ),
        (
            [[-8529, 8394], [18417, 12490]],
            [
                [
                    [201505, -5725, 319226, -470946],
                    [226135, -4528, 337902, -464682],
                ],
                [
                    [214340, -5449, 288415, -438090],
                    [252654, -7017, 344915, -481233],
                ],
            ],
            [[965, 1585], [930, 1606], [3354, 1]],
            54,
            7381233627641188654,
        ),
        (
            [[-8212, 3329], [6909, 13514]],
            [
                [
                    [163052, -7806, 315500, -477929],
                    [216104, -5256, 314809, -483961],
                ],
                [
                    [260874, -4891, 319335, -490489],
                    [278227, -5749, 335746, -527326],
                ],
            ],
            [[1041, 1639], [1071, 1662], [3367, 0]],
            49,
            11210138435166375508,
        ),
        (
            [[2216, 9174], [16698, 13359]],
            [
                [
                    [200618, -6379, 316154, -462527],
                    [245142, -8272, 335595, -449909],
                ],
                [
                    [239104, -5315, 296632, -443489],
                    [276001, -5892, 322267, -456969],
                ],
            ],
            [[1038, 1616], [951, 1673], [3352, 0]],
            60,
            3247137162712829221,
        ),
        (
            [[-310, 3153], [-1956, -2020]],
            [
                [
                    [259432, -4791, 329634, -484625],
                    [260748, -6118, 356080, -504942],
                ],
                [
                    [192265, -2994, 254817, -388012],
                    [201502, -2863, 294264, -434048],
                ],
            ],
            [[938, 1676], [981, 1667], [3363, 0]],
            59,
            764532996799968190,
        ),
        (
            [[10166, 3088], [-3968, 3394]],
            [
                [
                    [197984, -5827, 311656, -462568],
                    [234859, -3039, 299910, -466501],
                ],
                [
                    [238872, -3262, 312584, -479852],
                    [254346, -5494, 342900, -498446],
                ],
            ],
            [[1013, 1528], [969, 1622], [3347, 0]],
            54,
            3404416904895450140,
        ),
        (
            [[-4233, 9490], [7933, 13205]],
            [
                [
                    [214989, -4418, 306165, -478033],
                    [254123, -6916, 344424, -493629],
                ],
                [
                    [226391, -6585, 299274, -464476],
                    [241971, -6434, 344037, -491916],
                ],
            ],
            [[1032, 1580], [1014, 1649], [3351, 0]],
            60,
            8384656864237310999,
        ),
    ],
    &[
        (
            [[-2159, 11652], [6724, -50]],
            [
                [
                    [222686, -5012, 326708, -510884],
                    [236897, -5160, 344403, -493906],
                ],
                [
                    [227047, -5372, 282402, -465650],
                    [211228, -4704, 332351, -447995],
                ],
            ],
            [[1098, 1660], [1010, 1647], [3350, 0]],
            52,
            15242842349370337392,
        ),
        (
            [[1289, 7520], [883, -5170]],
            [
                [
                    [209268, -5369, 335220, -499522],
                    [244199, -5637, 324581, -451118],
                ],
                [
                    [240871, -6615, 300756, -463032],
                    [233261, -5968, 327502, -491359],
                ],
            ],
            [[990, 1615], [1007, 1645], [3354, 0]],
            52,
            15161987032918355244,
        ),
        (
            [[-7369, 8554], [17442, -199]],
            [
                [
                    [225730, -5467, 340137, -491260],
                    [225916, -4555, 340027, -464627],
                ],
                [
                    [219994, -5227, 287965, -447821],
                    [268017, -7735, 387010, -510767],
                ],
            ],
            [[971, 1641], [931, 1663], [3355, 1]],
            54,
            12712124857853946605,
        ),
        (
            [[-5086, 2885], [6708, 1190]],
            [
                [
                    [212679, -10548, 353697, -507251],
                    [218571, -5278, 314641, -488634],
                ],
                [
                    [264108, -5018, 323876, -496454],
                    [320858, -7394, 393784, -571004],
                ],
            ],
            [[1043, 1726], [1072, 1760], [3368, 0]],
            54,
            2347284019407618795,
        ),
        (
            [[4090, 9527], [12727, 4091]],
            [
                [
                    [263069, -8385, 367856, -499793],
                    [251652, -8434, 340335, -456498],
                ],
                [
                    [235427, -5365, 294849, -447070],
                    [364159, -9322, 408897, -490701],
                ],
            ],
            [[1048, 1734], [958, 1843], [3353, 0]],
            61,
            17973165208132369960,
        ),
        (
            [[3244, 5792], [-166, 202]],
            [
                [
                    [362049, -8754, 414695, -543810],
                    [265462, -6661, 361742, -508102],
                ],
                [
                    [193534, -3770, 260130, -398269],
                    [294653, -4635, 410216, -529188],
                ],
            ],
            [[951, 1867], [991, 1825], [3362, 0]],
            58,
            12760567381758701610,
        ),
        (
            [[5022, 1195], [-3965, 11646]],
            [
                [
                    [286405, -7195, 442672, -579224],
                    [236352, -3903, 302596, -471787],
                ],
                [
                    [241506, -3495, 313518, -483735],
                    [352785, -11737, 484994, -586159],
                ],
            ],
            [[1029, 1731], [981, 1839], [3347, 0]],
            60,
            14319541334190284163,
        ),
        (
            [[-406, 8589], [8471, 7293]],
            [
                [
                    [357039, -8838, 471902, -582284],
                    [267838, -7070, 356029, -510220],
                ],
                [
                    [227444, -6241, 306978, -478929],
                    [349639, -10148, 492308, -620043],
                ],
            ],
            [[1055, 1875], [1017, 1935], [3351, 0]],
            63,
            4876945349722782605,
        ),
    ],
    &[
        (
            [[280, 267], [153, 11321]],
            [
                [
                    [215607, -5251, 318582, -508157],
                    [244605, -5592, 349888, -496497],
                ],
                [
                    [232779, -5568, 287688, -464250],
                    [207641, -4723, 324585, -440479],
                ],
            ],
            [[1101, 1650], [1009, 1653], [3350, 0]],
            51,
            16165550050043189585,
        ),
        (
            [[-2280, 2226], [-3554, -2959]],
            [
                [
                    [183300, -4848, 311636, -490238],
                    [251707, -6126, 350215, -461911],
                ],
                [
                    [270359, -7956, 320375, -473439],
                    [226900, -5697, 314243, -483162],
                ],
            ],
            [[989, 1599], [1010, 1638], [3353, 0]],
            40,
            1530281515048583347,
        ),
        (
            [[-8039, 6367], [-326, 12290]],
            [
                [
                    [202378, -5694, 321371, -470989],
                    [251723, -5986, 388798, -504770],
                ],
                [
                    [244625, -5877, 317756, -456529],
                    [254007, -7163, 349159, -488123],
                ],
            ],
            [[968, 1640], [933, 1660], [3354, 1]],
            56,
            5775214775904447558,
        ),
        (
            [[-8476, 5486], [3902, 14083]],
            [
                [
                    [165573, -7727, 307729, -473651],
                    [254389, -5936, 375716, -528019],
                ],
                [
                    [328199, -6648, 368180, -528284],
                    [282253, -6261, 336037, -532614],
                ],
            ],
            [[1042, 1744], [1073, 1747], [3368, 0]],
            48,
            5296857175929719305,
        ),
        (
            [[2001, 2298], [212, 13050]],
            [
                [
                    [203104, -6491, 314139, -470606],
                    [325451, -10408, 412148, -493927],
                ],
                [
                    [324382, -6606, 381839, -514974],
                    [283986, -5419, 327066, -461616],
                ],
            ],
            [[1052, 1733], [956, 1803], [3353, 0]],
            59,
            14476312193724415126,
        ),
        (
            [[714, -149], [-3924, -1511]],
            [
                [
                    [254419, -5538, 333426, -489755],
                    [366402, -9501, 489018, -604056],
                ],
                [
                    [286221, -3709, 350529, -458423],
                    [214244, -2622, 307808, -438111],
                ],
            ],
            [[954, 1831], [991, 1862], [3364, 0]],
            61,
            10057758972030701247,
        ),
        (
            [[10922, 3415], [-2041, 5108]],
            [
                [
                    [190208, -5545, 315035, -467795],
                    [342947, -5721, 442854, -577751],
                ],
                [
                    [351637, -7971, 425258, -584227],
                    [269699, -5410, 352932, -505103],
                ],
            ],
            [[1021, 1715], [983, 1813], [3347, 0]],
            61,
            10970552946675300088,
        ),
        (
            [[-4760, 2049], [7027, 13146]],
            [
                [
                    [214984, -4114, 305122, -482973],
                    [357147, -10379, 506542, -630902],
                ],
                [
                    [345663, -9761, 439680, -552550],
                    [247490, -6031, 350668, -500826],
                ],
            ],
            [[1048, 1778], [1024, 1914], [3351, 0]],
            61,
            8614354661603916758,
        ),
    ],
];
const DIRECTION_TAUGHT_1024: [&[TaughtBlock]; 3] = [
    &[
        ([0, 0], 29, 8, [[0, 0], [0, 0]], 0, 0),
        ([0, 0], 29, 7, [[0, 0], [0, 0]], 0, 0),
        ([0, 0], 25, 5, [[0, 0], [0, 0]], 0, 0),
        ([0, 0], 24, 6, [[0, 0], [0, 0]], 0, 0),
        ([0, 0], 29, 8, [[0, 0], [0, 0]], 0, 0),
        ([0, 0], 28, 7, [[0, 0], [0, 0]], 0, 0),
        ([0, 0], 30, 4, [[0, 0], [0, 0]], 0, 0),
        ([0, 0], 29, 2, [[0, 0], [0, 0]], 0, 0),
    ],
    &[
        ([17, 2], 32, 6, [[34483, 0], [0, 78107]], 46691, 112227),
        ([36, 9], 32, 7, [[46789, 0], [0, 71628]], 46691, 112227),
        ([44, 12], 27, 7, [[70970, 0], [0, 100533]], 46691, 112227),
        ([57, 15], 34, 4, [[37451, 0], [0, 111343]], 46691, 112227),
        ([63, 21], 43, 4, [[106655, 0], [0, 208889]], 46691, 112227),
        ([61, 21], 49, 5, [[189265, 0], [0, 136141]], 46691, 112227),
        ([64, 25], 46, 7, [[103676, 0], [0, 156744]], 46691, 112227),
        ([64, 34], 60, 1, [[149970, 0], [0, 77244]], 46691, 112227),
    ],
    &[
        ([15, 2], 28, 5, [[0, 88225], [60265, 0]], 46691, 112227),
        ([38, 6], 33, 7, [[0, 116698], [109551, 0]], 46691, 112227),
        ([52, 13], 41, 5, [[0, 108834], [98818, 0]], 46691, 112227),
        ([56, 14], 44, 8, [[0, 80385], [130755, 0]], 46691, 112227),
        ([61, 21], 45, 3, [[0, 173460], [133872, 0]], 46691, 112227),
        ([63, 24], 49, 2, [[0, 179398], [146557, 0]], 46691, 112227),
        ([64, 21], 60, 1, [[0, 134434], [122707, 0]], 46691, 112227),
        ([63, 24], 55, 4, [[0, 142964], [112854, 0]], 46691, 112227),
    ],
];
const DIRECTION_READ_1024: [u64; 3] = [0xb82310568dfe19e2, 0xae96ac18cddd3cb2, 0x47e927406cb50d3c];
const DIRECTION_CENSUS_1024: [&[(u32, u64)]; 3] = [
    &[
        (1, 1059),
        (2, 8070),
        (3, 12698),
        (4, 3908),
        (5, 224),
        (6, 40),
        (7, 9),
        (8, 4),
        (9, 1),
        (11, 1),
    ],
    &[
        (1, 1059),
        (2, 8067),
        (3, 12699),
        (4, 3912),
        (5, 220),
        (6, 41),
        (7, 10),
        (8, 4),
        (9, 1),
        (10, 1),
    ],
    &[
        (1, 1058),
        (2, 8059),
        (3, 12700),
        (4, 3913),
        (5, 228),
        (6, 41),
        (7, 9),
        (8, 4),
        (9, 1),
        (11, 1),
    ],
];
/// The verdict, as the rules compute it over the pinned tables, written from the run: both
/// clauses hold in both rewarded arms. **H-13 is yes** for this network, this taught delivery,
/// this regime and this rule.
const DIRECTION_1024: Direction = Direction {
    couplings: [true, true],
    response: [true, true],
    yes: true,
};
/// ADR-0078's prediction for the couplings, as read: it held in both arms.
const COUPLINGS_READ_1024: [bool; 2] = [true, true];
/// The assigned readout's paired tally over the last 128 trials, per rewarded arm: 128 of
/// 128 in the assignment and 127 in the mirrored (one tie), against the mark of 80.
const PAIRED_1024: [u32; 2] = [128, 127];
/// The other readout's paired tally over the last 128 trials, a reading: 59 and 45, with 62
/// and 71 ties — the delivery never reaches those synapses, so what rises there is the
/// network's.
const OTHER_PAIRED_1024: [u32; 2] = [59, 45];
/// The selections of the assigned readout over the last 128 trials, a reading beside 80 that
/// reopens nothing of H-12: 106 and 115.
const SELECTED_1024: [u32; 2] = [106, 115];
/// The image's couplings, `[stimulus][readout]`, the frozen block's: A→R0 6 249 552, A→R1
/// 6 698 611, B→R0 6 584 205, B→R1 6 815 470.
const IMAGE_COUPLINGS_1024: [[i64; 2]; 2] = [[6_249_552, 6_698_611], [6_584_205, 6_815_470]];
const _: () = assert!(
    IMAGE_COUPLINGS_1024[0][0] == BACKGROUND_1024[0].0.10[0][0]
        && IMAGE_COUPLINGS_1024[0][1] == BACKGROUND_1024[0].0.10[0][1]
        && IMAGE_COUPLINGS_1024[1][0] == BACKGROUND_1024[0].0.10[1][0]
        && IMAGE_COUPLINGS_1024[1][1] == BACKGROUND_1024[0].0.10[1][1]
);
/// The reach of each arm's delivery, `(inside the assigned pairs, outside)`, as read: none
/// under the withheld arm; every one of the assignment's 1 584 synapses (775 of A→R0 and 809
/// of B→R1) and 1 603 of the mirrored assignment's 1 604 (806 of A→R1 and 798 of B→R0)
/// moved, and no synapse outside the pairs in any arm.
const REACH_1024: [(u64, u64); 3] = [(0, 0), (1_584, 0), (1_603, 0)];
const _: () = assert!(
    REACH_1024[1].0 as u32 == SYNAPSES_1024[0][0] + SYNAPSES_1024[1][1]
        && REACH_1024[2].0 as u32 + 1 == SYNAPSES_1024[0][1] + SYNAPSES_1024[1][0]
);

/// The gate's test (ADR-0061's class; brief 036): the signal's rule and the consolidation's
/// rule at their edges, held to the figures computed apart from the tree; the criterion's
/// rules at their edges over tables written by hand — 80 and 79 of the last 128, a tie
/// against, a clause failing in one arm only; the pairs of each assignment; and the taught
/// delivery's reach over the first `GATE_TRIALS` trials on the instrument's network at 1 024
/// units — the record's traces, weights and signal held to the oracle at every trial, every
/// synapse outside the assigned pairs unmoved bit for bit, and a synapse inside moved. No
/// whole run, and nothing else added to the gate.
#[test]
fn the_first_eight_trials_of_the_taught_delivery_at_1024_units_and_the_rules_over_their_tables() {
    // The signal's rule at its edges: rest stays; one LSB either side reaches rest; the step
    // is the floor of the fraction and at least one LSB; the width's ends.
    assert_eq!(decayed_signal(0), 0);
    assert_eq!(decayed_signal(1), 0);
    assert_eq!(decayed_signal(-1), 0);
    assert_eq!(decayed_signal(2), 1);
    assert_eq!(decayed_signal(16_383), 16_382, "below 2^14 one LSB");
    assert_eq!(decayed_signal(16_384), 16_383);
    assert_eq!(decayed_signal(ONE), ONE - 4, "at 1.0 four LSB");
    assert_eq!(decayed_signal(-ONE), 4 - ONE);
    assert_eq!(decayed_signal(i32::MAX), i32::MAX - (i32::MAX >> 14));
    assert_eq!(decayed_signal(i32::MIN), i32::MIN + (1 << 17));
    // The course from 1.0, and the per-trial map's fixed point, as computed apart from the
    // tree.
    let from_one = signal_course(ONE);
    assert_eq!(from_one.len(), TRIAL_TICKS as usize + 1);
    assert_eq!(from_one[0], ONE);
    assert_eq!(signal_end(&from_one), SIGNAL_END_FIRST_Q16);
    assert_eq!(
        ceiling_ticks(&from_one),
        1,
        "from 1.0 the first tick alone is at the ceiling"
    );
    assert_eq!(signal_end(&[]), 0);
    let mut signal = 0i32;
    let mut reached = None;
    for trial in 0..BLOCK {
        let after = signal.saturating_add(REWARD_Q16);
        signal = signal_end(&signal_course(after));
        if reached.is_none() && (after, signal) == (SIGNAL_AFTER_FIXED_Q16, SIGNAL_END_FIXED_Q16) {
            reached = Some(trial);
        }
    }
    assert_eq!(
        reached,
        Some(SIGNAL_FIXED_WITHIN_TRIALS - 1),
        "the fixed point within eleven trials"
    );
    let fixed = signal_course(SIGNAL_AFTER_FIXED_Q16);
    assert_eq!(signal_end(&fixed), SIGNAL_END_FIXED_Q16, "and it is fixed");
    assert_eq!(ceiling_ticks(&fixed), SIGNAL_CEILING_TICKS_FIXED);
    assert!(
        fixed.iter().all(|&s| s >= SIGNAL_END_FIXED_Q16),
        "never below the end"
    );
    // The consolidation's rule at its edges: nothing at zero; the whole trace at 1.0; half
    // at 0.5, rounded to nearest; a negative trace beyond the magnitude clamps at zero and
    // keeps the rest; the rail absorbs nothing; a modulation above 1.0 is 1.0; a negative
    // magnitude reads as zero.
    assert_eq!(consolidated(100, 5_000, 0), (100, 5_000, 0));
    assert_eq!(consolidated(100, 5_000, ONE), (0, 5_100, 100));
    assert_eq!(consolidated(-100, 5_000, ONE), (0, 4_900, -100));
    assert_eq!(
        consolidated(3, 5_000, 0x8000),
        (1, 5_002, 2),
        "1.5 rounds to 2"
    );
    assert_eq!(consolidated(-3, 5_000, 0x8000), (-1, 4_998, -2));
    assert_eq!(
        consolidated(1, 5_000, 0x7FFF),
        (1, 5_000, 0),
        "just under a half rounds down"
    );
    assert_eq!(consolidated(-6_000, 5_000, ONE), (-1_000, 0, -5_000));
    assert_eq!(
        consolidated(100, i32::from(i16::MAX), ONE),
        (100, i32::from(i16::MAX), 0)
    );
    assert_eq!(consolidated(100, 5_000, ONE + 1), (0, 5_100, 100));
    assert_eq!(consolidated(100, 5_000, -1), (100, 5_000, 0));
    assert_eq!(consolidated(100, -7, ONE), (0, 100, 100));
    assert_eq!(consolidated(0, 5_000, ONE), (0, 5_000, 0));
    // The pairs and the paired comparison.
    assert_eq!(assigned_pairs(false), [(0, 0), (1, 1)]);
    assert_eq!(assigned_pairs(true), [(0, 1), (1, 0)]);
    assert_eq!(REWARDED_ARMS, [ARMS[1], ARMS[2]]);
    assert!(!rewards(Arm::Withheld) && rewards(Arm::Assignment) && rewards(Arm::Mirrored));
    assert!(!mirrors(Arm::Withheld) && !mirrors(Arm::Assignment) && mirrors(Arm::Mirrored));
    let trial = |stimulus: u8, counts: [u32; 2]| -> TaughtTrial {
        (stimulus, counts, selected(counts), [[0; 2]; 2], 0, 0)
    };
    assert_eq!(
        paired(&trial(0, [5, 5]), &trial(0, [5, 5]), false),
        [false, false],
        "a tie is not more"
    );
    assert_eq!(
        paired(&trial(0, [6, 5]), &trial(0, [5, 5]), false),
        [true, false]
    );
    assert_eq!(
        paired(&trial(0, [5, 6]), &trial(0, [5, 5]), false),
        [false, true]
    );
    assert_eq!(
        paired(&trial(0, [5, 6]), &trial(0, [5, 5]), true),
        [true, false],
        "mirrored: A's readout is 1"
    );
    assert_eq!(
        paired(&trial(1, [6, 5]), &trial(1, [5, 5]), true),
        [true, false]
    );
    assert_eq!(
        paired(&trial(1, [6, 5]), &trial(1, [5, 5]), false),
        [false, true]
    );
    assert_eq!(
        paired(&trial(0, [4, 4]), &trial(0, [5, 5]), false),
        [false, false]
    );
    // The blocks from trials written by hand: two blocks, the assigned readout more in every
    // trial of the first and in none of the second, the other readout more in one.
    let withheld: Vec<TaughtTrial> = (0..2 * BLOCK)
        .map(|k| trial((k % 2) as u8, [10, 10]))
        .collect();
    let mut mine: Vec<TaughtTrial> = withheld.clone();
    for (k, t) in mine.iter_mut().enumerate() {
        let assigned = answer_of(t.0, false);
        if k < BLOCK {
            t.1[assigned] = 11;
        }
        if k == BLOCK {
            t.1[assigned ^ 1] = 11;
        }
        t.2 = selected(t.1);
        t.3 = [[1, 2], [3, 4]];
        t.4 = k as i32;
        t.5 = t.4 + 1;
    }
    let blocks = taught_blocks(&mine, &withheld, false);
    assert_eq!(blocks.len(), 2);
    assert_eq!(
        blocks[0],
        (
            [BLOCK as u32, 0],
            BLOCK as u32,
            0,
            [[64, 128], [192, 256]],
            63,
            64
        )
    );
    assert_eq!(
        blocks[1],
        (
            [0, 1],
            0,
            BLOCK as u32 - 1,
            [[64, 128], [192, 256]],
            127,
            128
        ),
        "the other readout's tally, the ties, the last trial's signals"
    );
    assert_eq!(
        taught_blocks(&withheld, &withheld, false),
        vec![([0, 0], 0, 64, [[0; 2]; 2], 0, 0); 2],
        "the withheld arm against itself tallies nothing"
    );
    assert_eq!(taught_blocks(&[], &[], true), Vec::<TaughtBlock>::new());
    // The criterion at its edges over tables written by hand: the couplings strict, the
    // tally at 80 and at 79, a clause failing in one arm only.
    let image = [[100i64, 200], [300, 400]];
    let block_with = |couplings: [[i64; 2]; 2]| -> Block {
        (
            0,
            0,
            [[0; 2]; 2],
            [0; 2],
            [0; 2],
            [0; 2],
            0,
            0,
            0,
            0,
            couplings,
            0,
        )
    };
    let taught_with = |tallies: [[u32; 2]; 2]| -> Vec<TaughtBlock> {
        let mut v = vec![([0u32; 2], 0, 0, [[0; 2]; 2], 0, 0); 6];
        v.push((tallies[0], 0, 0, [[0; 2]; 2], 0, 0));
        v.push((tallies[1], 0, 0, [[0; 2]; 2], 0, 0));
        v
    };
    let up = [block_with([[101, 200], [300, 401]])];
    let up_mirrored = [block_with([[100, 201], [301, 400]])];
    let flat = [block_with(image)];
    let one_short = [block_with([[101, 200], [300, 400]])];
    assert!(couplings_rose(&image, &up, false));
    assert!(
        !couplings_rose(&image, &up, true),
        "the mirrored pairs are the other two"
    );
    assert!(couplings_rose(&image, &up_mirrored, true));
    assert!(!couplings_rose(&image, &flat, false), "equal is not above");
    assert!(!couplings_rose(&image, &one_short, false), "both pairs");
    assert!(!couplings_rose(&image, &[], false));
    let eighty = taught_with([[40, 3], [40, 5]]);
    let seventy_nine = taught_with([[40, 3], [39, 5]]);
    assert_eq!(last_paired(&eighty), REWARDED_MIN);
    assert_eq!(last_paired(&seventy_nine), REWARDED_MIN - 1);
    assert_eq!(last_paired(&[]), 0);
    assert_eq!(last_selected(&taught_with([[0, 0], [0, 0]])), 0);
    let mut selects = taught_with([[0, 0], [0, 0]]);
    selects[6].1 = 30;
    selects[7].1 = 31;
    assert_eq!(last_selected(&selects), 61);
    assert_eq!(
        direction(&image, [(&up, &eighty), (&up_mirrored, &eighty)]),
        Direction {
            couplings: [true, true],
            response: [true, true],
            yes: true
        }
    );
    assert_eq!(
        direction(&image, [(&up, &seventy_nine), (&up_mirrored, &eighty)]),
        Direction {
            couplings: [true, true],
            response: [false, true],
            yes: false
        },
        "79 of the last 128 fails the response clause in the assignment"
    );
    assert_eq!(
        direction(&image, [(&up, &eighty), (&flat, &eighty)]),
        Direction {
            couplings: [true, false],
            response: [true, true],
            yes: false
        },
        "the couplings clause failing in the mirrored arm alone"
    );
    assert_eq!(
        direction(&image, [(&up, &eighty), (&up, &eighty)]),
        Direction {
            couplings: [true, false],
            response: [true, true],
            yes: false
        },
        "the mirrored arm is read by its own pairs"
    );
    assert_eq!(COUPLINGS_PREDICTED, [true, true]);
    // The taught delivery's reach over the first trials on the instrument's network at 1 024
    // units: the oracle held at every trial inside `taught_run`; then the arena against the
    // weights before, outside the assigned pairs bit for bit and inside moved.
    let p = prior(1024);
    let mut exec = at_gain(&p, config(1024, 2, 0), GAIN_1024);
    let before = weights_of(&exec);
    let run = taught_run(&mut exec, Arm::Assignment, 1024, GATE_TRIALS);
    let (blocks, trace, trials, read, _, _) = &run;
    assert!(blocks.is_empty(), "eight trials are no whole block");
    assert_eq!(trials.len(), GATE_TRIALS);
    eprintln!("DUMP taught1024 first eight trace {trace:#018x} read {read:?}");
    let (inside, outside) = reach(&exec, &before, 1024, &assigned_pairs(false));
    eprintln!("DUMP taught1024 first eight reach inside {inside} outside {outside}");
    assert_eq!(
        outside, 0,
        "every synapse outside the assigned pairs is as it was"
    );
    assert!(inside > 0, "the delivery reached a synapse inside them");
    let (moved_inside, moved_outside) = reach(&exec, &before, 1024, &[]);
    assert_eq!(
        (moved_inside, moved_outside),
        (0, inside),
        "the same synapses, counted outside no pair"
    );
    assert_eq!(
        read[0].4, 0,
        "the signal at the first trial's end is at rest"
    );
    assert_eq!(read[0].5, REWARD_Q16, "and the first reward is 1.0");
    assert_eq!(read[1].4, SIGNAL_END_FIRST_Q16);
    assert!(
        read.iter().all(|t| t.5 == t.4.saturating_add(REWARD_Q16)),
        "every reward adds 1.0"
    );
    assert_eq!(
        read[0].3, [[0; 2]; 2],
        "nothing consolidates before the first delivery"
    );
    assert!(
        read.iter().any(|t| t.3 != [[0; 2]; 2]),
        "and something after it"
    );
    assert!(
        read.iter().all(|t| {
            let unassigned = [(0usize, 1usize), (1, 0)];
            unassigned.iter().all(|&(s, r)| t.3[s][r] == 0)
        }),
        "nothing consolidates outside the assigned pairs"
    );
    // Over the pinned tables: the verdict as written, by the rules; the prediction as read;
    // the tallies, the selections and the reach the constants state; every arm's tables of
    // eight blocks with the same trials presenting A and the sight holding in every block;
    // the withheld arm's tables at rest; in a rewarded arm the unaddressed pairs' couplings
    // the image's in every block, the assigned pairs' rising block by block, nothing
    // consolidated outside the pairs, and the signals at the fixed point from the first
    // block on.
    let verdict = direction(
        &IMAGE_COUPLINGS_1024,
        [
            (DIRECTION_BLOCKS_1024[1], DIRECTION_TAUGHT_1024[1]),
            (DIRECTION_BLOCKS_1024[2], DIRECTION_TAUGHT_1024[2]),
        ],
    );
    assert_eq!(verdict, DIRECTION_1024, "the verdict as written");
    assert!(verdict.yes, "H-13 is yes");
    assert_eq!(
        [
            couplings_rose(&IMAGE_COUPLINGS_1024, DIRECTION_BLOCKS_1024[1], false),
            couplings_rose(&IMAGE_COUPLINGS_1024, DIRECTION_BLOCKS_1024[2], true),
        ],
        COUPLINGS_READ_1024,
        "the couplings clause as read"
    );
    assert_eq!(
        COUPLINGS_READ_1024, COUPLINGS_PREDICTED,
        "the prediction held"
    );
    let last_other = |k: usize| -> u32 {
        DIRECTION_TAUGHT_1024[k]
            .iter()
            .rev()
            .take(LAST_BLOCKS)
            .map(|b| b.0[1])
            .sum()
    };
    assert_eq!(
        [
            last_paired(DIRECTION_TAUGHT_1024[1]),
            last_paired(DIRECTION_TAUGHT_1024[2]),
        ],
        PAIRED_1024,
        "the assigned readout's paired tally"
    );
    assert_eq!(
        [last_other(1), last_other(2)],
        OTHER_PAIRED_1024,
        "the other readout's"
    );
    assert_eq!(
        [
            last_selected(DIRECTION_TAUGHT_1024[1]),
            last_selected(DIRECTION_TAUGHT_1024[2]),
        ],
        SELECTED_1024,
        "the selections of the assigned readout"
    );
    assert_eq!(REACH_1024[0], (0, 0));
    let withheld = DIRECTION_BLOCKS_1024[0];
    for (k, &arm) in ARMS.iter().enumerate() {
        let blocks = DIRECTION_BLOCKS_1024[k];
        let taught = DIRECTION_TAUGHT_1024[k];
        let compositions = DIRECTION_COMPOSITIONS_1024[k];
        assert_eq!(blocks.len(), TRIALS / BLOCK, "{arm:?}: eight blocks");
        assert_eq!(taught.len(), TRIALS / BLOCK);
        assert_eq!(compositions.len(), TRIALS / BLOCK);
        assert_ne!(DIRECTION_TRACES_1024[k], 0);
        assert_ne!(DIRECTION_READ_1024[k], 0);
        assert!(!DIRECTION_CENSUS_1024[k].is_empty());
        let pairs = assigned_pairs(mirrors(arm));
        let mut last = IMAGE_COUPLINGS_1024;
        for (j, block) in blocks.iter().enumerate() {
            assert_eq!(
                block.1, withheld[j].1,
                "{arm:?} block {j}: the trials presenting A"
            );
            assert!(block.6 >= SEEN_MIN, "{arm:?} block {j}: the sight holds");
            assert!(taught[j].0[0] <= BLOCK as u32 && taught[j].0[1] <= BLOCK as u32);
            assert!(compositions[j].3 <= BLOCK as u32);
            for s in 0..2 {
                for r in 0..2 {
                    let assigned = pairs.contains(&(s, r)) && rewards(arm);
                    if assigned {
                        assert!(
                            block.10[s][r] > last[s][r],
                            "{arm:?} block {j}: the assigned pair {s}→{r} rose"
                        );
                        assert!(
                            taught[j].3[s][r] > 0,
                            "{arm:?} block {j}: and consolidated potentiation"
                        );
                    } else {
                        assert_eq!(
                            block.10[s][r], IMAGE_COUPLINGS_1024[s][r],
                            "{arm:?} block {j}: the pair {s}→{r} is the image's"
                        );
                        assert_eq!(taught[j].3[s][r], 0);
                    }
                }
            }
            last = block.10;
            if rewards(arm) {
                assert_eq!(
                    block.9, SIGNAL_AFTER_FIXED_Q16,
                    "{arm:?} block {j}: the signal"
                );
                assert_eq!(
                    (taught[j].4, taught[j].5),
                    (SIGNAL_END_FIXED_Q16, SIGNAL_AFTER_FIXED_Q16)
                );
            } else {
                assert_eq!(block.9, 0);
                assert_eq!(taught[j], ([0, 0], block.0, block.11, [[0; 2]; 2], 0, 0));
                assert_eq!(
                    (block.7, block.8),
                    QUIET_1024[SETTLED].1,
                    "the sums at rest"
                );
            }
        }
    }
}

// ------------------------------------------------ written before the run (brief 037)

/// H-14's run (ADR-0080): three of the instrument's runs, twenty-four blocks, the length
/// derived from H-13's blocks before any run and not this round's to move.
const REINFORCED_TRIALS: usize = 3 * TRIALS;
const _: () = assert!(REINFORCED_TRIALS == 1_536 && REINFORCED_TRIALS / BLOCK == 24);
/// The arms of H-14 (ADR-0080), in the order run and no other, every one `Delivery::Addressed`
/// from the one image — the task as ADR-0059 and ADR-0068 built it, the reward reaching the
/// synapses from the presented stimulus onto the readout the engine selected: the assignment
/// (`Feedback::Answer`, A's answer readout 0 and B's readout 1), the mirrored assignment
/// (`Feedback::Answer`, `mirrored`), and the shuffled reward (`Feedback::Shuffled`, the
/// reward's sign a coin the stimulus does not read), a reading of lock-in and no clause.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Earned {
    Assignment,
    Mirrored,
    Shuffled,
}
const EARNED_ARMS: [Earned; 3] = [Earned::Assignment, Earned::Mirrored, Earned::Shuffled];
/// The rewarded arms in `EARNED_ARMS`'s order, the criterion's `[assignment, mirrored]`.
const EARNED_REWARDED: [Earned; 2] = [Earned::Assignment, Earned::Mirrored];
/// ADR-0080's prediction, a Hypothesis written before the run: H-14 is yes.
const REINFORCED_PREDICTED: bool = true;
/// ADR-0080's Hypothesis for the course, from arithmetic on H-13's blocks and not held to
/// the run: the selection passes `CROSSING_MARK` per block (40 of 64, the criterion's rate
/// over one block) by about trial `CROSSING_PREDICTED_BY_TRIAL` in both assignments, the
/// mirrored first. Read beside the run, never asserted.
const CROSSING_MARK: u32 = OFFSET_MARK_64;
const _: () = assert!(CROSSING_MARK == 40);
const CROSSING_PREDICTED_BY_TRIAL: usize = 900;
const MIRRORED_CROSSES_FIRST_PREDICTED: bool = true;
const _: () = assert!(REINFORCED_PREDICTED && MIRRORED_CROSSES_FIRST_PREDICTED);
const _: () = assert!(CROSSING_PREDICTED_BY_TRIAL < REINFORCED_TRIALS - LAST_BLOCKS * BLOCK);
/// The four stimulus–readout pairs, `(stimulus, readout)`: the shuffled arm's reach, since a
/// reward without information addresses whichever readout the engine selected.
const ALL_PAIRS: [(usize, usize); 4] = [(0, 0), (0, 1), (1, 0), (1, 1)];

// ------------------------------------------------------------- the readings (brief 037)

/// One trial's reading under an arm (brief 037): the stimulus presented; the readouts'
/// counts in the task's window, as the task read them; the selection; whether it was
/// correct (the stimulus's answer; a tie is not); the reward the task delivered, signed; the
/// trace consolidated into the weights over the trial, `[stimulus][readout]`, by the oracle;
/// and the dopamine signal at the trial's end, before the reward, and after it.
type EarnedTrial = (u8, [u32; 2], Option<u8>, bool, i32, [[i64; 2]; 2], i32, i32);

/// One block of an arm (brief 037): the selections per stimulus, `[stimulus][readout 0,
/// readout 1, tie]`; the positive rewards; the trace consolidated into the weights over the
/// block, `[stimulus][readout]`; and the signal at the block's last trial's end and after
/// its reward.
type EarnedBlock = ([[u32; 3]; 2], u32, [[i64; 2]; 2], i32, i32);

/// Where an arm's reward takes its sign from: the answer, or the shuffled coin.
fn feedback_of(arm: Earned) -> Feedback {
    if arm == Earned::Shuffled {
        Feedback::Shuffled
    } else {
        Feedback::Answer
    }
}

/// Whether an arm mirrors the assignment.
fn earned_mirrors(arm: Earned) -> bool {
    arm == Earned::Mirrored
}

/// True for an arm whose reward carries the answer, the criterion's two.
fn answers(arm: Earned) -> bool {
    arm != Earned::Shuffled
}

/// The pairs an arm's delivery can reach: the two answer pairs where the reward carries the
/// answer — ADR-0080's derivation, a wrong selection's pair spending the next trial under a
/// signal below zero and consolidating nothing — and all four under the shuffled reward.
fn reachable_pairs(arm: Earned) -> Vec<(usize, usize)> {
    if answers(arm) {
        assigned_pairs(earned_mirrors(arm)).to_vec()
    } else {
        ALL_PAIRS.to_vec()
    }
}

/// The selections per stimulus over `read`: to readout 0, to readout 1, and the ties.
fn splits(read: &[EarnedTrial]) -> [[u32; 3]; 2] {
    let mut out = [[0u32; 3]; 2];
    for t in read {
        let into = &mut out[usize::from(t.0)][t.2.map_or(2, usize::from)];
        *into = into.saturating_add(1);
    }
    out
}

/// The selections per stimulus over the last `LAST_BLOCKS` blocks, the criterion's window.
fn last_splits(read: &[EarnedTrial]) -> [[u32; 3]; 2] {
    let from = read.len().saturating_sub(LAST_BLOCKS.saturating_mul(BLOCK));
    splits(read.get(from..).unwrap_or(&[]))
}

/// An arm's blocks from its trials, `BLOCK` trials each: the splits, the positive rewards,
/// the consolidation summed and the last trial's signals.
fn earned_blocks(read: &[EarnedTrial]) -> Vec<EarnedBlock> {
    read.chunks(BLOCK)
        .map(|mine| {
            let mut positive = 0u32;
            let mut transferred = [[0i64; 2]; 2];
            for t in mine {
                positive = positive.saturating_add(u32::from(t.4 > 0));
                for (s, readouts) in t.5.iter().enumerate() {
                    for (k, &amount) in readouts.iter().enumerate() {
                        transferred[s][k] = transferred[s][k].saturating_add(amount);
                    }
                }
            }
            let last = mine.last().expect("a block holds a trial");
            (splits(mine), positive, transferred, last.6, last.7)
        })
        .collect()
}

/// Where the selection first passes `CROSSING_MARK` per block, as the trial at that block's
/// end; none when no block does.
fn crossing(blocks: &[Block]) -> Option<usize> {
    blocks
        .iter()
        .position(|b| b.0 >= CROSSING_MARK)
        .map(|k| k.saturating_add(1).saturating_mul(BLOCK))
}

/// The lock-in reading (ADR-0080): per stimulus, the shuffled arm goes to one readout over
/// the last 128 trials as often as a rewarded arm goes to that stimulus's answer — the larger
/// of its two readouts' counts at least the smaller of the two rewarded arms' correct counts
/// for the stimulus (the same trials present the same stimuli in every arm). A reading, not a
/// clause.
fn locked_in(shuffled: [[u32; 3]; 2], rewarded: [[[u32; 3]; 2]; 2]) -> [bool; 2] {
    let mut out = [false; 2];
    for (s, into) in out.iter_mut().enumerate() {
        let dominant = shuffled[s][0].max(shuffled[s][1]);
        let answered = EARNED_REWARDED
            .iter()
            .zip(rewarded.iter())
            .map(|(&arm, split)| split[s][answer_of(s as u8, earned_mirrors(arm))])
            .min()
            .unwrap_or(0);
        *into = dominant >= answered;
    }
    out
}

/// ADR-0080's derivation read over an arm's trials, clause by clause: the signal at every
/// trial's end at most the fixed point's (`SIGNAL_END_FIXED_Q16`, 0.712); after every
/// negative reward the signal below zero; and nothing consolidated in the trial after a
/// negative reward, the addressed pair — a wrong selection's, or none at a tie — spending
/// it under a signal below zero. Every clause true on every arm is the assertion; a false
/// one is a finding against the derivation, reported beside the verdict.
fn derivation(read: &[EarnedTrial]) -> [bool; 3] {
    let bounded = read.iter().all(|t| t.6 <= SIGNAL_END_FIXED_Q16);
    let negative = read.iter().filter(|t| t.4 < 0).all(|t| t.7 < 0);
    let nothing = read
        .windows(2)
        .filter(|w| w[0].4 < 0)
        .all(|w| w[1].5 == [[0; 2]; 2]);
    [bounded, negative, nothing]
}

// ------------------------------------------------------------ the criterion (brief 037)

/// H-14's criterion (ADR-0080), per rewarded arm `[assignment, mirrored]`: the correct
/// selections over the last `LAST_BLOCKS` blocks at least `REWARDED_MIN` — `last_correct`,
/// ADR-0066's rule over the same shape, the task counting a trial correct only when the
/// selection is the stimulus's answer, a tie not. `yes` is both.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Reinforced {
    correct: [bool; 2],
    yes: bool,
}

fn reinforced(arms: [&[Block]; 2]) -> Reinforced {
    let correct = [
        last_correct(arms[0]) >= REWARDED_MIN,
        last_correct(arms[1]) >= REWARDED_MIN,
    ];
    Reinforced {
        correct,
        yes: correct[0] && correct[1],
    }
}

// ---------------------------------------------------------------- the run (brief 037)

/// An arm's run: the blocks, the accuracy-sequence trace, the composed trials, every trial's
/// reading and the volley-tick census over the run.
type EarnedRun = (Vec<Block>, u64, Vec<Composed>, Vec<EarnedTrial>, Vec<u64>);

/// The FNV-1a hash of every trial's reading, each number as its `i32` words.
fn earned_hash(read: &[EarnedTrial]) -> u64 {
    let mut words: Vec<i32> = Vec::new();
    let mut wide = |x: i64| {
        words.push(x as i32);
        words.push((x >> 32) as i32);
    };
    for t in read {
        wide(i64::from(t.0));
        wide(i64::from(t.1[0]));
        wide(i64::from(t.1[1]));
        wide(t.2.map_or(3, i64::from));
        wide(i64::from(t.3));
        wide(i64::from(t.4));
        for readouts in &t.5 {
            for &amount in readouts {
                wide(amount);
            }
        }
        wide(i64::from(t.6));
        wide(i64::from(t.7));
    }
    fnv1a_64(&words)
}

/// An arm's run from the frozen engine (brief 037): `trials` trials of the calibration's task
/// with ADR-0076's stimulus, `Delivery::Addressed` and the arm's feedback — the task itself
/// writing the addressed set after every trial to the presented stimulus's units onto the
/// selected readout's units, none at a tie, and delivering the reward, `REWARD_Q16` signed by
/// the outcome or by the coin — the composer seeded from the record and replaying the
/// consolidation under the pair the task addressed and the signal its reward left. At every
/// trial the record's traces and weights are held to the oracle and the record's signal, read
/// after the task's reward, to the course's end plus that reward; the task's contract is
/// asserted trial by trial — the selection the sign of the count difference, correct the
/// answer and a tie not, the reward's sign the outcome's, the addressed set the presented
/// stimulus onto the selected readout, the signal the task read the record's. Nothing of
/// ADR-0080's derivation is asserted here: it is read after the run (`derivation`) so that a
/// failure is reported beside the verdict and not in place of it.
fn earned_run(exec: &mut Engine, arm: Earned, units: u32, trials: usize) -> EarnedRun {
    assert_eq!(
        units, 1024,
        "the cancel and the synapse counts below are pinned at 1 024 units"
    );
    assert_eq!(
        exec.modulation_baseline_q16(),
        0,
        "the baseline is the image's zero"
    );
    assert_eq!(
        exec.modulator().dopamine_rpe,
        0,
        "the signal is at rest before the first trial"
    );
    let sets = geometry(units, rotation(units));
    let mut composer = Composer::new(units);
    composer.enumerate(exec);
    composer.cursor = exec.ticks() as u32;
    composer.taught = Some(Taught {
        addressed: None,
        signal: 0,
    });
    let mirrored = earned_mirrors(arm);
    let feedback = feedback_of(arm);
    let picked = CANCEL_PICKED_1024.expect("ADR-0076 picked a cancel");
    let task = task(
        SHAPE_F46,
        Some(cancel_of(picked)),
        units,
        feedback,
        mirrored,
        Delivery::Addressed,
    );
    // The task is `Copy`: a copy reads the coin the run's own task drew.
    let probe = task;
    let mut read: Vec<EarnedTrial> = Vec::with_capacity(trials);
    let (blocks, trace) = run_on(
        exec,
        task,
        units,
        trials,
        &mut |exec, trial, start, outcome| {
            // The task rewarded at the trial's end, before this reading: the oracle holds the
            // record's signal to the course's end plus it.
            composer.rewarded = outcome.reward_q16;
            composer.observe(exec, trial, start, outcome);
            assert_eq!(
                outcome.selection,
                selected(outcome.counts),
                "trial {trial}: the selection is the sign of the count difference"
            );
            let answer = answer_of(outcome.stimulus, mirrored) as u8;
            assert_eq!(
                outcome.correct,
                outcome.selection == Some(answer),
                "trial {trial}: correct is the answer, a tie not"
            );
            let positive = match feedback {
                Feedback::Answer => outcome.correct,
                Feedback::Shuffled => probe.coin_at(trial as u64),
                Feedback::Withheld => unreachable!("every arm of H-14 rewards"),
            };
            let expected = if positive {
                REWARD_Q16
            } else {
                REWARD_Q16.saturating_neg()
            };
            assert_eq!(
                outcome.reward_q16, expected,
                "trial {trial}: the reward's sign is the outcome's"
            );
            let stimulus = usize::from(outcome.stimulus);
            let targets = outcome
                .selection
                .map_or(0, |r| readout_set(&sets, usize::from(r)).len() as usize);
            assert_eq!(
                exec.addressed_counts(),
                (sets[stimulus].len() as usize, targets),
                "trial {trial}: the addressed set is the presented stimulus onto the selected readout, onto none at a tie"
            );
            let after = exec.modulator().dopamine_rpe;
            assert_eq!(
                outcome.signal_q16, after,
                "trial {trial}: the task read the signal after its reward"
            );
            let end = after.saturating_sub(outcome.reward_q16);
            let transferred = composer.transferred.last().copied().unwrap_or([[0; 2]; 2]);
            composer.taught = Some(Taught {
                addressed: outcome.selection.map(|r| (stimulus, usize::from(r))),
                signal: after,
            });
            read.push((
                outcome.stimulus,
                outcome.counts,
                outcome.selection,
                outcome.correct,
                outcome.reward_q16,
                transferred,
                end,
                after,
            ));
        },
    );
    assert_eq!(composer.out.len(), trials);
    assert_eq!(composer.counts(), SYNAPSES_1024);
    (blocks, trace, composer.out, read, composer.volley_ticks)
}

/// Dumps an arm's run: the sight's blocks and trace, the rows, the composition per block, the
/// earned blocks, every trial's reading with its hash, the census, the crossing and the last
/// blocks' splits.
fn dump_earned(name: &str, run: &EarnedRun, earned: &[EarnedBlock]) {
    let (blocks, trace, trials, read, volley_ticks) = run;
    eprintln!(
        "DUMP {name} sight {blocks:?} trace {trace:#018x} curve {:?}",
        curve(blocks)
    );
    let rows: Vec<Row> = trials.iter().map(trial_row).collect();
    eprintln!("DUMP {name} rows {rows:?}");
    let compositions: Vec<Composition> = trials.chunks(BLOCK).map(composition).collect();
    eprintln!("DUMP {name} compositions {compositions:?}");
    eprintln!("DUMP {name} earned {earned:?}");
    eprintln!("DUMP {name} read {read:?} hash {:#018x}", earned_hash(read));
    eprintln!("DUMP {name} census {:?}", census_of(volley_ticks));
    eprintln!(
        "DUMP {name} crossing {:?} last splits {:?} last correct {}",
        crossing(blocks),
        last_splits(read),
        last_correct(blocks)
    );
}

/// Holds an arm's run to its pinned tables: the blocks and the trace, the composition per
/// block, the earned blocks, the readings' hash and the census.
fn pinned_earned(name: &str, k: usize, run: &EarnedRun, earned: &[EarnedBlock]) {
    let (blocks, trace, trials, read, volley_ticks) = run;
    pinned(
        &format!("{name} sight"),
        blocks,
        *trace,
        REINFORCED_BLOCKS_1024[k],
        REINFORCED_TRACES_1024[k],
    );
    let compositions: Vec<Composition> = trials.chunks(BLOCK).map(composition).collect();
    assert_eq!(
        compositions.as_slice(),
        REINFORCED_COMPOSITIONS_1024[k],
        "{name}: the composition per block"
    );
    assert_eq!(
        earned, REINFORCED_EARNED_1024[k],
        "{name}: the earned blocks"
    );
    assert_eq!(
        earned_hash(read),
        REINFORCED_READ_1024[k],
        "{name}: the readings"
    );
    assert_eq!(
        census_of(volley_ticks),
        REINFORCED_CENSUS_1024[k].to_vec(),
        "{name}: the volley's ticks"
    );
}

/// H-14 at 1 024 units (brief 037): the settled image, held to ADR-0077 step by step; the
/// calibration — a frozen block from the image, the reward withheld, held to ADR-0077's
/// frozen run — before any rewarded run (H-14's stopping rule, step 2); then the assignment,
/// the mirrored assignment and the shuffled reward, each `REINFORCED_TRIALS` trials from the
/// one image under the task's own delivery, each dumped and the verdict computed by the rules
/// before anything is held; then the assertion — every synapse outside the pairs the arm's
/// delivery can reach as the image holds it, bit for bit, a synapse inside moved, ADR-0080's
/// derivation true on every arm and the wrong pairs' couplings the image's in every block of
/// the rewarded arms — and the pinned tables.
#[test]
#[ignore]
fn the_reinforced_form_at_1024_units_exhaustive() {
    let name = "reinforced1024";
    let image = settled_image(name);
    {
        let mut frozen = frozen_from(&image, 1024);
        let calibration = taught_run(&mut frozen, Arm::Withheld, 1024, BLOCK);
        calibration_holds(&format!("{name} calibration"), &calibration);
        eprintln!("DUMP {name} calibration holds: ADR-0077's settled candidate reproduced");
    }
    let sets = geometry(1024, ROTATION_1024);
    let mut runs: Vec<EarnedRun> = Vec::with_capacity(EARNED_ARMS.len());
    let mut reaches = [(0u64, 0u64); 3];
    for (k, &arm) in EARNED_ARMS.iter().enumerate() {
        let arm_name = format!("{name} {arm:?}");
        let mut exec = frozen_from(&image, 1024);
        let before = weights_of(&exec);
        let couplings_before = [
            [
                coupling(&exec, sets[0], sets[2]),
                coupling(&exec, sets[0], sets[3]),
            ],
            [
                coupling(&exec, sets[1], sets[2]),
                coupling(&exec, sets[1], sets[3]),
            ],
        ];
        assert_eq!(
            couplings_before, IMAGE_COUPLINGS_1024,
            "{arm_name}: the same image"
        );
        let run = earned_run(&mut exec, arm, 1024, REINFORCED_TRIALS);
        let (inside, outside) = reach(&exec, &before, 1024, &reachable_pairs(arm));
        eprintln!(
            "DUMP {arm_name} reach inside {inside} outside {outside} signal after the run {:#x}",
            exec.modulator().dopamine_rpe
        );
        reaches[k] = (inside, outside);
        runs.push(run);
    }
    // Every arm dumped and the verdict computed before anything is held.
    let mut tables: Vec<Vec<EarnedBlock>> = Vec::with_capacity(EARNED_ARMS.len());
    for (k, &arm) in EARNED_ARMS.iter().enumerate() {
        let earned = earned_blocks(&runs[k].3);
        dump_earned(&format!("{name} {arm:?}"), &runs[k], &earned);
        tables.push(earned);
    }
    let verdict = reinforced([&runs[0].0, &runs[1].0]);
    let last = [
        last_splits(&runs[0].3),
        last_splits(&runs[1].3),
        last_splits(&runs[2].3),
    ];
    let locked = locked_in(last[2], [last[0], last[1]]);
    let crossed = [
        crossing(&runs[0].0),
        crossing(&runs[1].0),
        crossing(&runs[2].0),
    ];
    let derivations = [
        derivation(&runs[0].3),
        derivation(&runs[1].3),
        derivation(&runs[2].3),
    ];
    eprintln!(
        "DUMP {name} verdict {verdict:?} predicted {REINFORCED_PREDICTED} correct last {:?} crossed {crossed:?} predicted by {CROSSING_PREDICTED_BY_TRIAL} mirrored first {MIRRORED_CROSSES_FIRST_PREDICTED} splits last {last:?} locked {locked:?} derivation {derivations:?} reach {reaches:?}",
        [last_correct(&runs[0].0), last_correct(&runs[1].0)]
    );
    // The assertion (ADR-0080's derivation), after the verdict and beside it.
    for (k, &arm) in EARNED_ARMS.iter().enumerate() {
        assert_eq!(
            reaches[k].1, 0,
            "{arm:?}: every synapse outside the pairs the delivery can reach is the image's"
        );
        assert!(reaches[k].0 > 0, "{arm:?}: the delivery reached a synapse");
        assert_eq!(derivations[k], [true; 3], "{arm:?}: ADR-0080's derivation");
        if answers(arm) {
            let pairs = assigned_pairs(earned_mirrors(arm));
            for (j, block) in runs[k].0.iter().enumerate() {
                for &(s, r) in &ALL_PAIRS {
                    if !pairs.contains(&(s, r)) {
                        assert_eq!(
                            block.10[s][r], IMAGE_COUPLINGS_1024[s][r],
                            "{arm:?} block {j}: the wrong pair {s}→{r} is the image's"
                        );
                        assert_eq!(tables[k][j].2[s][r], 0);
                    }
                }
            }
        }
    }
    for (k, &arm) in EARNED_ARMS.iter().enumerate() {
        pinned_earned(&format!("{name} {arm:?}"), k, &runs[k], &tables[k]);
    }
    assert_eq!(verdict, REINFORCED_1024, "the verdict as written");
    assert_eq!(
        [last_correct(&runs[0].0), last_correct(&runs[1].0)],
        CORRECT_LAST_1024
    );
    assert_eq!(crossed, CROSSED_1024);
    assert_eq!(last, SPLITS_LAST_1024);
    assert_eq!(locked, LOCKED_1024);
    assert_eq!(reaches, REINFORCED_REACH_1024);
    assert_eq!(derivations, DERIVATION_1024);
}

// ----------------------------------------------------------- the measurement (brief 037)

/// The three arms at 1 024 units, in `EARNED_ARMS`'s order, each pinned from one run: the
/// sight's blocks and trace, the composition per block, the earned blocks, the readings' hash
/// and the volley's census.
const REINFORCED_BLOCKS_1024: [&[Block]; 3] = [&[], &[], &[]];
const REINFORCED_TRACES_1024: [u64; 3] = [0; 3];
const REINFORCED_COMPOSITIONS_1024: [&[Composition]; 3] = [&[], &[], &[]];
const REINFORCED_EARNED_1024: [&[EarnedBlock]; 3] = [&[], &[], &[]];
const REINFORCED_READ_1024: [u64; 3] = [0; 3];
const REINFORCED_CENSUS_1024: [&[(u32, u64)]; 3] = [&[], &[], &[]];
/// The verdict, as the rules compute it over the pinned tables; written from the run.
const REINFORCED_1024: Reinforced = Reinforced {
    correct: [false, false],
    yes: false,
};
/// The correct selections over the last 128 trials, per rewarded arm, against `REWARDED_MIN`.
const CORRECT_LAST_1024: [u32; 2] = [0; 2];
/// Where each arm's selection first passed 40 of 64 per block, as read, beside ADR-0080's
/// estimate of about trial 900, the mirrored first.
const CROSSED_1024: [Option<usize>; 3] = [None; 3];
/// The selections per stimulus over the last 128 trials, per arm, `[stimulus][readout 0,
/// readout 1, tie]`.
const SPLITS_LAST_1024: [[[u32; 3]; 2]; 3] = [[[0; 3]; 2]; 3];
/// The lock-in reading of the shuffled arm, per stimulus.
const LOCKED_1024: [bool; 2] = [false; 2];
/// The reach of each arm's delivery, `(inside the pairs it can reach, outside)`.
const REINFORCED_REACH_1024: [(u64, u64); 3] = [(0, 0); 3];
/// ADR-0080's derivation as read on each arm, clause by clause.
const DERIVATION_1024: [[bool; 3]; 3] = [[false; 3]; 3];

/// The gate's test (ADR-0061's class; brief 037): the arms, their feedback and their pairs;
/// the criterion's rule at its edges over tables written by hand — 80 and 79 of the last
/// 128, one arm failing, an arm of no block; the readings' rules (the crossing, the splits,
/// the earned blocks, the lock-in, the derivation) over trials written by hand; and the
/// task's own delivery over the first `GATE_TRIALS` trials on the instrument's network at
/// 1 024 units with the baseline at zero — the record's traces, weights and signal held to
/// the oracle at every trial, every synapse outside the answer pairs (the wrong pairs among
/// them) unmoved bit for bit, the derivation true, a synapse inside moved. No whole run, and
/// nothing else added to the gate. The settled network's lead-in of ninety-six windows is the
/// weekly test's; the rules the derivation rests on read the same on any network with the
/// baseline at zero.
#[test]
fn the_first_eight_trials_of_the_reinforced_form_at_1024_units_and_the_rules_over_their_tables() {
    // The arms, their feedback and their pairs.
    assert_eq!(EARNED_REWARDED, [EARNED_ARMS[0], EARNED_ARMS[1]]);
    assert_eq!(feedback_of(Earned::Assignment), Feedback::Answer);
    assert_eq!(feedback_of(Earned::Mirrored), Feedback::Answer);
    assert_eq!(feedback_of(Earned::Shuffled), Feedback::Shuffled);
    assert!(
        !earned_mirrors(Earned::Assignment)
            && earned_mirrors(Earned::Mirrored)
            && !earned_mirrors(Earned::Shuffled)
    );
    assert!(answers(Earned::Assignment) && answers(Earned::Mirrored) && !answers(Earned::Shuffled));
    assert_eq!(reachable_pairs(Earned::Assignment), vec![(0, 0), (1, 1)]);
    assert_eq!(reachable_pairs(Earned::Mirrored), vec![(0, 1), (1, 0)]);
    assert_eq!(reachable_pairs(Earned::Shuffled), ALL_PAIRS.to_vec());
    assert_eq!(REINFORCED_TRIALS, 3 * TRIALS);
    assert_eq!(REINFORCED_TRIALS / BLOCK, 24);
    assert_eq!(CROSSING_MARK, REWARDED_MIN / LAST_BLOCKS as u32);
    // The criterion at its edges over tables written by hand: 80 and 79 of the last 128,
    // one arm failing, an arm of no block.
    let block_with = |correct: u32| -> Block {
        (
            correct,
            0,
            [[0; 2]; 2],
            [0; 2],
            [0; 2],
            [0; 2],
            0,
            0,
            0,
            0,
            [[0; 2]; 2],
            0,
        )
    };
    let blocks_with = |last: [u32; 2]| -> Vec<Block> {
        let mut v = vec![block_with(0); 6];
        v.push(block_with(last[0]));
        v.push(block_with(last[1]));
        v
    };
    let eighty = blocks_with([40, 40]);
    let seventy_nine = blocks_with([40, 39]);
    assert_eq!(last_correct(&eighty), REWARDED_MIN);
    assert_eq!(last_correct(&seventy_nine), REWARDED_MIN - 1);
    assert_eq!(
        reinforced([&eighty, &eighty]),
        Reinforced {
            correct: [true, true],
            yes: true
        }
    );
    assert_eq!(
        reinforced([&seventy_nine, &eighty]),
        Reinforced {
            correct: [false, true],
            yes: false
        },
        "79 of the last 128 fails the assignment"
    );
    assert_eq!(
        reinforced([&eighty, &seventy_nine]),
        Reinforced {
            correct: [true, false],
            yes: false
        },
        "and the mirrored arm"
    );
    assert_eq!(
        reinforced([&[], &eighty]),
        Reinforced {
            correct: [false, true],
            yes: false
        },
        "an arm of no block"
    );
    // The crossing.
    assert_eq!(crossing(&[]), None);
    assert_eq!(crossing(&blocks_with([39, 39])), None);
    assert_eq!(crossing(&[block_with(40)]), Some(BLOCK));
    assert_eq!(
        crossing(&[block_with(39), block_with(40), block_with(64)]),
        Some(2 * BLOCK)
    );
    assert_eq!(crossing(&blocks_with([40, 39])), Some(7 * BLOCK));
    // The splits, the earned blocks and the lock-in over trials written by hand.
    let trial = |stimulus: u8, selection: Option<u8>, reward: i32| -> EarnedTrial {
        let correct = selection == Some(stimulus);
        (
            stimulus,
            [0; 2],
            selection,
            correct,
            reward,
            [[0; 2]; 2],
            0,
            reward,
        )
    };
    assert_eq!(splits(&[]), [[0; 3]; 2]);
    assert_eq!(
        splits(&[
            trial(0, Some(0), ONE),
            trial(0, Some(1), -ONE),
            trial(0, None, -ONE),
            trial(1, Some(1), ONE),
        ]),
        [[1, 1, 1], [0, 1, 0]]
    );
    let mut three_blocks: Vec<EarnedTrial> = Vec::with_capacity(3 * BLOCK);
    for k in 0..3 * BLOCK {
        let stimulus = (k % 2) as u8;
        let selection = if k < BLOCK {
            Some(stimulus ^ 1)
        } else {
            Some(stimulus)
        };
        three_blocks.push(trial(
            stimulus,
            selection,
            if k < BLOCK { -ONE } else { ONE },
        ));
    }
    assert_eq!(
        last_splits(&three_blocks),
        [[64, 0, 0], [0, 64, 0]],
        "the first block is outside the last 128"
    );
    assert_eq!(
        last_splits(&three_blocks[..BLOCK]),
        [[0, 32, 0], [32, 0, 0]]
    );
    assert_eq!(last_splits(&[]), [[0; 3]; 2]);
    let mut with_transfer = three_blocks.clone();
    with_transfer[BLOCK].5 = [[1, 2], [3, 4]];
    with_transfer[BLOCK + 1].5 = [[10, 20], [30, 40]];
    with_transfer[2 * BLOCK - 1].6 = 7;
    with_transfer[2 * BLOCK - 1].7 = 8;
    let earned = earned_blocks(&with_transfer);
    assert_eq!(earned.len(), 3);
    assert_eq!(
        earned[0],
        ([[0, 32, 0], [32, 0, 0]], 0, [[0; 2]; 2], 0, -ONE)
    );
    assert_eq!(
        earned[1],
        ([[32, 0, 0], [0, 32, 0]], 64, [[11, 22], [33, 44]], 7, 8),
        "the splits, the positive rewards, the transfer summed, the last trial's signals"
    );
    assert_eq!(
        earned[2],
        ([[32, 0, 0], [0, 32, 0]], 64, [[0; 2]; 2], 0, ONE)
    );
    assert_eq!(earned_blocks(&[]), Vec::<EarnedBlock>::new());
    let rewarded = [[[45, 5, 0], [5, 45, 0]], [[5, 45, 0], [45, 5, 0]]];
    assert_eq!(
        locked_in([[40, 10, 0], [5, 45, 0]], rewarded),
        [false, true],
        "A at 40 of 50 is under the rewarded arms' 45; B at 45 is as often"
    );
    assert_eq!(locked_in([[46, 4, 0], [46, 4, 0]], rewarded), [true, true]);
    assert_eq!(
        locked_in([[44, 0, 6], [0, 44, 6]], rewarded),
        [false, false],
        "a tie goes to no readout"
    );
    assert_eq!(
        locked_in(
            [[45, 5, 0], [0, 0, 0]],
            [[[45, 5, 0], [0, 0, 0]], [[5, 46, 0], [0, 0, 0]]]
        ),
        [true, true],
        "the smaller of the two rewarded arms' counts"
    );
    // The derivation's rule over trials written by hand.
    assert_eq!(derivation(&[]), [true; 3]);
    let mut course = vec![
        trial(0, Some(0), ONE),
        trial(1, Some(0), -ONE),
        trial(1, Some(1), ONE),
    ];
    course[0].6 = 0;
    course[0].7 = ONE;
    course[1].6 = SIGNAL_END_FIRST_Q16;
    course[1].7 = SIGNAL_END_FIRST_Q16 - ONE;
    course[2].6 = -1;
    course[2].7 = ONE - 1;
    course[1].5 = [[5, 0], [0, 0]];
    assert_eq!(
        derivation(&course),
        [true; 3],
        "a transfer after a positive reward is not the clause's"
    );
    let mut over = course.clone();
    over[1].6 = SIGNAL_END_FIXED_Q16 + 1;
    assert_eq!(derivation(&over), [false, true, true]);
    let mut still_positive = course.clone();
    still_positive[1].7 = 0;
    assert_eq!(derivation(&still_positive), [true, false, true]);
    let mut moved = course.clone();
    moved[2].5 = [[0, 0], [1, 0]];
    assert_eq!(
        derivation(&moved),
        [true, true, false],
        "a transfer in the trial after a negative reward"
    );
    // The task's own delivery over the first trials on the instrument's network at 1 024
    // units with the baseline at zero: the oracle held at every trial inside `earned_run`;
    // then the arena against the weights before, outside the answer pairs bit for bit and
    // inside moved, and the derivation read over the trials.
    let p = prior(1024);
    let mut exec = at_gain(&p, config(1024, 2, 0), GAIN_1024);
    let before = weights_of(&exec);
    let run = earned_run(&mut exec, Earned::Assignment, 1024, GATE_TRIALS);
    let (blocks, trace, trials, read, _) = &run;
    assert!(blocks.is_empty(), "eight trials are no whole block");
    assert_eq!(trials.len(), GATE_TRIALS);
    eprintln!("DUMP reinforced1024 first eight trace {trace:#018x} read {read:?}");
    let (inside, outside) = reach(&exec, &before, 1024, &assigned_pairs(false));
    eprintln!("DUMP reinforced1024 first eight reach inside {inside} outside {outside}");
    assert_eq!(
        outside, 0,
        "every synapse outside the answer pairs, the wrong pairs among them, is as it was"
    );
    assert!(inside > 0, "the delivery reached a synapse inside them");
    assert_eq!(
        derivation(read),
        [true; 3],
        "ADR-0080's derivation over the trials"
    );
    assert_eq!(
        read[0].6, 0,
        "the signal at the first trial's end is at rest"
    );
    assert_eq!(
        read[0].5, [[0; 2]; 2],
        "nothing consolidates before the first delivery"
    );
    assert!(
        read.iter().all(|t| t.4.abs() == REWARD_Q16),
        "every reward is 1.0 in magnitude"
    );
    assert!(
        read.iter().all(|t| t.7 == t.6.saturating_add(t.4)),
        "the signal after the reward is the end's plus it"
    );
    assert!(
        read.iter()
            .all(|t| t.3 == (t.2 == Some(answer_of(t.0, false) as u8))),
        "correct is the answer, a tie not"
    );
    assert!(
        read.iter().all(|t| (t.4 > 0) == t.3),
        "the reward's sign is the outcome's"
    );
    assert!(
        read.iter().any(|t| !t.3),
        "a selection was wrong within eight trials"
    );
    assert!(
        read.iter().all(|t| t.5[0][1] == 0 && t.5[1][0] == 0),
        "nothing consolidates on the wrong pairs"
    );
}
