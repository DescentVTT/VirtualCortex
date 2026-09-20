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

#![deny(clippy::arithmetic_side_effects)]

use cortex_connectome::{
    CortexFileHeader, Prior, SECTION_HOMEOSTASIS, SectionEntry, crc64, ring_distance,
};
use cortex_core::{FLAG_INHIBITORY, MODULATION_ONE_Q16};
use cortex_homeostasis::{ACTIVITY_BIN_SHIFT, ACTIVITY_WINDOW_SHIFT, HomeostaticDrivePool};
use cortex_neuromod::DOPAMINE_TAU_SHIFT;
use cortex_runtime::{
    Config, Delivery, Drive, Executor, Feedback, Image, Readout, Set, Stimulus, Task, TaskError,
    Window, blocks_for, run_driven, spikes_per_unit, synthesize,
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

fn task(units: u32, feedback: Feedback, mirrored: bool, delivery: Delivery) -> Task {
    let [a, b, r0, r1] = geometry(units, rotation(units));
    let stimulus = |set| Stimulus {
        set,
        messages: STIMULUS_MESSAGES,
        efficacy_q16: STIMULUS_Q16,
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
/// window under the drive.
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
    let p = prior(units);
    let mut exec = at_gain(&p, config(units, workers, baseline_q16), gain);
    assert_eq!(exec.homeostasis().synaptic_gain_q16, gain);
    assert_eq!(exec.modulation_baseline_q16(), baseline_q16);
    assert_eq!(
        exec.addressed_counts(),
        (units as usize, units as usize),
        "every unit a source and a target before the first trial, whatever the delivery"
    );
    let mut task = task(units, feedback, mirrored, delivery);
    task.check(&exec).expect("the task fits the executor");
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
                task(units, feedback, false, Delivery::Global).check(&exec),
                Ok(())
            );
            assert_eq!(
                task(units, feedback, true, Delivery::Global).check(&exec),
                Ok(())
            );
        }
        let fixed = Engine::new(config(units, 1, ONE)).unwrap();
        assert_eq!(
            task(units, Feedback::Withheld, false, Delivery::Global).check(&fixed),
            Ok(())
        );
        assert_eq!(
            task(units, Feedback::Answer, false, Delivery::Global).check(&fixed),
            Err(TaskError::RewardAtCeiling)
        );
        let frozen = Engine::new(config(units, 1, 0)).unwrap();
        assert_eq!(
            task(units, Feedback::Withheld, false, Delivery::Global).check(&frozen),
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
