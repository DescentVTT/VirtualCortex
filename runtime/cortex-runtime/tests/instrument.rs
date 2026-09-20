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
use cortex_homeostasis::HomeostaticDrivePool;
use cortex_neuromod::DOPAMINE_TAU_SHIFT;
use cortex_runtime::{
    Config, Drive, Executor, Feedback, Image, Readout, Set, Stimulus, Task, TaskError, Window,
    blocks_for, spikes_per_unit, synthesize,
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

fn task(units: u32, feedback: Feedback, mirrored: bool) -> Task {
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
/// modulator's signal after the block's last reward; and the excitatory coupling from each
/// stimulus set into each readout set after the block (`[stimulus][readout]`).
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
);

/// A run of `trials` trials on the prior at `units` at `gain` on `workers` workers: the
/// blocks' readings, and the FNV-1a hash of every trial's `(stimulus, selection, correct)`,
/// the accuracy sequence in one number. The first trial is preceded by a lead-in of one
/// window under the drive.
fn run(
    units: u32,
    workers: usize,
    gain: u32,
    baseline_q16: i32,
    feedback: Feedback,
    mirrored: bool,
    trials: usize,
) -> (Vec<Block>, u64) {
    let p = prior(units);
    let mut exec = at_gain(&p, config(units, workers, baseline_q16), gain);
    assert_eq!(exec.homeostasis().synaptic_gain_q16, gain);
    assert_eq!(exec.modulation_baseline_q16(), baseline_q16);
    let mut task = task(units, feedback, mirrored);
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
            ));
            correct = 0;
            a_trials = 0;
            spikes = [[0; 2]; 2];
            volley = [0; 2];
            stimulus_spikes = [0; 2];
            before_spikes = [0; 2];
            seen = 0;
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
    let (blocks, trace) = run(units, 2, gain, 0, Feedback::Withheld, false, BLOCK);
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
            assert_eq!(task(units, feedback, false).check(&exec), Ok(()));
            assert_eq!(task(units, feedback, true).check(&exec), Ok(()));
        }
        let fixed = Engine::new(config(units, 1, ONE)).unwrap();
        assert_eq!(task(units, Feedback::Withheld, false).check(&fixed), Ok(()));
        assert_eq!(
            task(units, Feedback::Answer, false).check(&fixed),
            Err(TaskError::RewardAtCeiling)
        );
        let frozen = Engine::new(config(units, 1, 0)).unwrap();
        assert_eq!(
            task(units, Feedback::Withheld, false).check(&frozen),
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
