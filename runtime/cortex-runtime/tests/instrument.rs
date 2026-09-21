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

#![deny(clippy::arithmetic_side_effects)]

use cortex_connectome::{
    CortexFileHeader, Prior, SECTION_HOMEOSTASIS, SectionEntry, crc64, ring_distance,
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
    assert_eq!(
        exec.addressed_counts(),
        (units as usize, units as usize),
        "every unit a source and a target before the first trial, whatever the delivery"
    );
    let mut task = task(shape, cancel, units, feedback, mirrored, delivery);
    task.check(&exec).expect("the task fits the executor");
    let [a, b, r0, r1] = geometry(units, rotation(units));
    // The stimulus sets counted as a readout would count them: the same rule, the other
    // two sets.
    let stimuli = Readout::new([a, b]);
    lead_in(&mut exec, &task.drive, lead_in_windows);
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
        let outcome = task.trial(&mut exec, trial as u64).expect("a trial runs");
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
        observe(&mut exec, trial, start, &outcome);
        sequence.push(
            i32::from(outcome.stimulus)
                | i32::from(outcome.selection.map_or(3, |r| r)) << 1
                | i32::from(outcome.correct) << 3,
        );
        if trial.wrapping_add(1) % BLOCK == 0 {
            let (inhibitory, excitatory) = weights_by_polarity(&exec);
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
                    [coupling(&exec, a, r0), coupling(&exec, a, r1)],
                    [coupling(&exec, b, r0), coupling(&exec, b, r1)],
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
        }
    }

    /// The synapses from a stimulus unit onto a readout unit, from the arena, in the walk's
    /// order; a stimulus unit is excitatory, as the geometry holds.
    fn enumerate(&mut self, exec: &Engine) {
        let [a, b, r0, r1] = self.sets;
        for unit in exec.units() {
            let id = unit.id as u32;
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
                self.synapses.push(Replayed {
                    block_idx: s.block_idx as usize,
                    slot: usize::from(s.slot),
                    source: id,
                    target: s.target,
                    stimulus,
                    readout,
                    delay: u32::from(s.delay_ticks),
                    magnitude: i32::from(s.weight_q1_15).max(0),
                    stamp: NO_SPIKE_ON_RECORD,
                    trace: 0,
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
        // target's last spike, each classed by that spike, then the stamp.
        let mut terms = [[[0i64; 4]; 2]; 2];
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
            }
        }
        // The record: (a), each slot held to the oracle and each weight to the prior's.
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
                "trial {trial}: the weight of {}→{} is frozen",
                syn.source,
                syn.target
            );
            let into = &mut sums[syn.stimulus][syn.readout];
            *into = into.saturating_add(i64::from(e));
        }
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
    let frozen = at_gain(&p, config(units, 2, 0), gain);
    let sums = weights_by_polarity(&frozen);
    let mut composer = Composer::new(units);
    let mut counted: Vec<Counted> = Vec::with_capacity(trials);
    let (blocks, trace) = run_behind(
        0,
        shape,
        cancel,
        units,
        2,
        gain,
        0,
        Feedback::Withheld,
        false,
        Delivery::Global,
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
/// the oracle, and the pick rule at its edges; no whole run.
#[test]
fn the_probes_of_the_two_injections_at_1024_units_and_the_rules_over_their_tables() {
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
}

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
}
