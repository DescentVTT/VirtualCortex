//! Brief 027's measurement (ADR-0060): the reward path of three-factor plasticity closed on a
//! behaviour the engine reads back from its own spike train, on the reference network of
//! ADR-0044 with the gain held at 2.0 (no controller, no sleep), through the task of
//! `cortex-runtime::task` (ADR-0059). Two stimuli, each a quarter of the ring; two readouts,
//! the other two quarters, each adjacent to both stimuli, so that the anatomy favours neither
//! assignment in expectation; a trial of $2^{12}$ ticks (41 ms, one estimator bin); a block of
//! sixty-four trials; the modulation baseline at 0.5 and a reward of ±1.0, so that the
//! outcome of a trial carries the next trial's consolidation to the ceiling or to the floor.
//! The rewarded run and its four controls (a shuffled reward, a fixed modulation with no
//! reward, the mirrored assignment, a second worker count) run under one harness, each as
//! its own test; per block the accuracy, the spikes per readout set by the stimulus
//! presented, the stimulus quarters' own spikes, the weight sums by polarity, the modulator's
//! signal and the excitatory coupling from each stimulus quarter into each readout quarter,
//! so that a curve that does not move can be told from a network that fell silent. The
//! criterion is ADR-0060's, written before the run and applied as written; its outcome is
//! pinned here as the engine's, whatever it is.
//!
//! Every number is the engine's own, pinned from one run and held on every worker count and
//! every architecture, as the determinism pin is. Every full run, at 256 units and at 1 024,
//! is the weekly job's `exhaustive` test; the pull request's gate runs the rewarded run's first
//! block at 256 units and the criterion over the pinned tables (ADR-0061: a run in the gate is
//! paid again by every runtime mutant of the weekly sweep). What the numbers decide, and at
//! what scale, is stated in ADR-0060 and in whitepaper §11.1.

#![deny(clippy::arithmetic_side_effects)]

use cortex_connectome::{CortexFileHeader, Prior, SECTION_HOMEOSTASIS, SectionEntry, crc64};
use cortex_core::{FLAG_INHIBITORY, MODULATION_ONE_Q16};
use cortex_homeostasis::HomeostaticDrivePool;
use cortex_runtime::{
    Config, Delivery, Drive, Executor, Feedback, Image, Readout, Set, Stimulus, Task, TaskError,
    Window, blocks_for, spikes_per_unit, synthesize,
};

include!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../testkit/prop.rs"
));

type Engine = Executor<2048>;

const ONE: i32 = MODULATION_ONE_Q16;

// ------------------------------------------------------------------ written before the run

/// A trial: $2^{12}$ ticks, 41.0 ms simulated, one estimator bin (ADR-0036); inside the
/// eligibility window of $2^{16}$ ticks and four times the dopamine signal's time constant.
const TRIAL_TICKS: u32 = 1 << 12;
/// A block: sixty-four trials.
const BLOCK: usize = 64;
/// The modulation with the dopamine signal at rest: half of every pending trace consolidates
/// at a presynaptic spike with no reward; a reward carries the next trial's consolidation to
/// the ceiling and a punishment to the floor (0.5 ± 0.78 one trial on, the signal decaying by
/// $2^{-14}$ per tick).
const BASELINE_Q16: i32 = 0x8000;
/// The reward's magnitude, 1.0: the whole width of the modulation, signed by the outcome.
const REWARD_Q16: i32 = ONE;
/// The stimulus: two messages of 1.25 into every unit of the set, the replay drive's
/// (ADR-0038), which fires a unit at its base threshold exactly once.
const STIMULUS_Q16: i32 = 0x0001_4000;
const STIMULUS_MESSAGES: u32 = 2;
/// The synaptic gain, held: 2.0, where the reference network fires at 7 to 10 Hz under the
/// drive at 256 units (ADR-0044, ADR-0055).
const GAIN_Q16: u32 = 0x0002_0000;
/// The seed the trials' stimuli and the shuffled coin are drawn from.
const SEED: u64 = 27;
/// The criterion's two thresholds, as trials of a block: a rise of 0.15 is 9.6 trials of
/// sixty-four, so at least ten; a difference below 0.05 is below 3.2, so at most three.
const RISE_PER_CENT: i64 = 15;
const CONTROL_PER_CENT: i64 = 5;

// ------------------------------------------------------------------------- the network

/// The prior of ADR-0044 at `units`: a fifth inhibitory at the rail, 32 synapses per unit, a
/// window of eight, a quarter rewired, local delays of 1 to 3 ms and far ones of 14 to
/// 25.6 ms, excitatory weights in [6 000, 12 000].
fn prior(units: u32) -> Prior {
    Prior {
        units,
        inhibitory_every: 5,
        synapses_per_unit: 32,
        window: 8,
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

/// The ring in four quarters: stimulus A, readout 0, stimulus B, readout 1, in that order, so
/// that each readout touches both stimuli on the ring at the same window.
fn quarters(units: u32) -> [Set; 4] {
    let len = units / 4;
    [
        Set::contiguous(0, len),
        Set::contiguous(len, len),
        Set::contiguous(len.saturating_mul(2), len),
        Set::contiguous(len.saturating_mul(3), len),
    ]
}

fn task(units: u32, feedback: Feedback, mirrored: bool) -> Task {
    let [a, r0, b, r1] = quarters(units);
    let stimulus = |set| Stimulus {
        set,
        messages: STIMULUS_MESSAGES,
        efficacy_q16: STIMULUS_Q16,
        cancel: None,
    };
    Task {
        stimuli: [stimulus(a), stimulus(b)],
        readout: Readout::new([r0, r1]),
        drive: drive(units),
        ticks: TRIAL_TICKS,
        window: Window::whole(TRIAL_TICKS),
        seed: SEED,
        reward_q16: REWARD_Q16,
        mirrored,
        feedback,
        delivery: Delivery::Global,
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

/// One block: the correct trials of sixty-four; the trials that presented stimulus A; the
/// trials' spikes in each readout set summed over the block, by the stimulus presented
/// (`[stimulus][readout]`); the spikes in each stimulus quarter summed over the block; the
/// inhibitory and the excitatory sum over the arena after the block; the modulator's signal
/// after the block's last reward; and the excitatory coupling from each stimulus quarter into
/// each readout quarter after the block (`[stimulus][readout]`).
type Block = (
    u32,
    u32,
    [[u64; 2]; 2],
    [u64; 2],
    i64,
    i64,
    i32,
    [[i64; 2]; 2],
);

/// A run of `trials` trials on the prior at `units` on `workers` workers: the blocks'
/// readings, and the FNV-1a hash of every trial's `(stimulus, selection, correct)`, the
/// accuracy sequence in one number.
fn run(
    units: u32,
    workers: usize,
    baseline_q16: i32,
    feedback: Feedback,
    mirrored: bool,
    trials: usize,
) -> (Vec<Block>, u64) {
    let p = prior(units);
    let mut exec = at_gain(&p, config(units, workers, baseline_q16), GAIN_Q16);
    assert_eq!(exec.homeostasis().synaptic_gain_q16, GAIN_Q16);
    let mut task = task(units, feedback, mirrored);
    task.check(&exec).expect("the task fits the executor");
    let [a, r0, b, r1] = quarters(units);
    // The stimulus quarters counted as a readout would count them: the same rule, the
    // other two sets.
    let stimuli = Readout::new([a, b]);
    let mut blocks = Vec::new();
    let mut sequence: Vec<i32> = Vec::with_capacity(trials);
    let mut correct = 0u32;
    let mut a_trials = 0u32;
    let mut spikes = [[0u64; 2]; 2];
    let mut stimulus_spikes = [0u64; 2];
    for trial in 0..trials {
        let overwritten = exec.train_overwritten();
        let start = exec.ticks() as u32;
        let outcome = task.trial(&mut exec, trial as u64).expect("a trial runs");
        assert!(
            exec.train_overwritten().saturating_sub(overwritten)
                <= u64::from(spikes_per_unit(TRIAL_TICKS)).saturating_mul(u64::from(units)),
            "the ring let go no more than one trial's bound"
        );
        let in_stimuli = stimuli.count(exec.train(), start, TRIAL_TICKS);
        correct = correct.saturating_add(u32::from(outcome.correct));
        a_trials = a_trials.saturating_add(u32::from(outcome.stimulus == 0));
        let s = usize::from(outcome.stimulus);
        spikes[s][0] = spikes[s][0].saturating_add(u64::from(outcome.counts[0]));
        spikes[s][1] = spikes[s][1].saturating_add(u64::from(outcome.counts[1]));
        stimulus_spikes[0] = stimulus_spikes[0].saturating_add(u64::from(in_stimuli[0]));
        stimulus_spikes[1] = stimulus_spikes[1].saturating_add(u64::from(in_stimuli[1]));
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
                stimulus_spikes,
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
            stimulus_spikes = [0; 2];
        }
    }
    (blocks, fnv1a_64(&sequence))
}

// ------------------------------------------------------------------------- the criterion

/// The last block's correct trials minus the first block's.
fn rise(blocks: &[Block]) -> i64 {
    let first = blocks.first().map_or(0, |b| i64::from(b.0));
    let last = blocks.last().map_or(0, |b| i64::from(b.0));
    last.saturating_sub(first)
}

/// `rise / BLOCK >= per_cent / 100`, in integers.
fn at_least(rise: i64, per_cent: i64) -> bool {
    rise.saturating_mul(100) >= per_cent.saturating_mul(BLOCK as i64)
}

/// The criterion of ADR-0060, clause by clause: the rewarded run rises by at least 0.15; the
/// shuffled reward and the fixed modulation each rise by less than 0.05; the mirrored
/// assignment's rewarded run rises by at least 0.15 too; the accuracy sequence is the same on
/// one worker and on four. `learned` is all five.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Verdict {
    rewarded: bool,
    shuffled: bool,
    fixed: bool,
    mirrored: bool,
    workers: bool,
    learned: bool,
}

fn verdict(
    rewarded: &[Block],
    shuffled: &[Block],
    fixed: &[Block],
    mirrored: &[Block],
    workers: bool,
) -> Verdict {
    let v = Verdict {
        rewarded: at_least(rise(rewarded), RISE_PER_CENT),
        shuffled: !at_least(rise(shuffled), CONTROL_PER_CENT),
        fixed: !at_least(rise(fixed), CONTROL_PER_CENT),
        mirrored: at_least(rise(mirrored), RISE_PER_CENT),
        workers,
        learned: false,
    };
    Verdict {
        learned: v.rewarded && v.shuffled && v.fixed && v.mirrored && v.workers,
        ..v
    }
}

/// The correct trials per block of a run, for the dump.
fn curve(blocks: &[Block]) -> Vec<u32> {
    blocks.iter().map(|b| b.0).collect()
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

// ------------------------------------------------------------------------------ the runs

/// The 256-unit form: 512 trials (eight blocks, sixteen windows, 21 s simulated). Each run is
/// its own test, so that the test harness runs them beside one another, and since ADR-0061
/// each is a weekly `exhaustive` test; the pinned tables below are the runs' as the engine
/// produced them, the criterion's test reads the tables, and the gate runs the first block.
const TRIALS_256: usize = 8 * BLOCK;
const REWARDED_256: &[Block] = &[
    (
        35,
        34,
        [[774, 749], [565, 597]],
        [7739, 6974],
        53_054_109,
        56_099_340,
        181_724,
        [[1_017_638, 919_760], [763_781, 890_716]],
    ),
    (
        27,
        31,
        [[568, 688], [666, 604]],
        [7226, 7504],
        52_742_869,
        53_703_743,
        -12_289,
        [[832_999, 738_710], [594_945, 680_481]],
    ),
    (
        30,
        31,
        [[607, 614], [687, 594]],
        [7091, 7499],
        52_428_833,
        52_341_764,
        -65_536,
        [[686_961, 609_696], [478_664, 559_714]],
    ),
    (
        34,
        28,
        [[554, 536], [654, 670]],
        [6525, 8102],
        52_065_911,
        50_588_804,
        -51_077,
        [[587_003, 533_621], [395_025, 460_482]],
    ),
    (
        32,
        30,
        [[553, 553], [612, 594]],
        [6881, 7716],
        51_707_694,
        49_379_633,
        -94_256,
        [[518_773, 494_543], [366_999, 423_831]],
    ),
    (
        22,
        36,
        [[635, 655], [517, 481]],
        [8127, 6634],
        51_486_463,
        48_995_859,
        -201_181,
        [[528_218, 474_027], [363_808, 423_706]],
    ),
    (
        31,
        30,
        [[506, 551], [556, 617]],
        [6953, 7640],
        51_085_520,
        47_762_055,
        31_922,
        [[508_986, 439_725], [339_325, 391_082]],
    ),
    (
        32,
        32,
        [[580, 505], [593, 571]],
        [7285, 7281],
        50_679_642,
        47_308_553,
        25_769,
        [[512_931, 431_707], [345_152, 400_704]],
    ),
];
const TRACE_256: u64 = 0x9ca6f87d15955aad;
const SHUFFLED_256: &[Block] = &[
    (
        39,
        34,
        [[795, 792], [581, 621]],
        [7747, 6996],
        53_172_749,
        57_165_668,
        179_398,
        [[1_104_470, 1_016_553], [839_338, 954_353]],
    ),
    (
        23,
        31,
        [[579, 702], [675, 645]],
        [7228, 7507],
        52_940_957,
        54_888_825,
        -117_018,
        [[900_925, 823_193], [669_977, 778_710]],
    ),
    (
        33,
        31,
        [[625, 604], [686, 604]],
        [7093, 7514],
        52_590_093,
        53_087_000,
        45_070,
        [[740_080, 673_276], [502_337, 598_095]],
    ),
    (
        33,
        28,
        [[549, 531], [651, 674]],
        [6524, 8099],
        52_254_232,
        51_399_499,
        248_012,
        [[631_239, 582_219], [408_605, 481_761]],
    ),
    (
        34,
        30,
        [[563, 545], [635, 611]],
        [6876, 7720],
        51_882_933,
        50_034_016,
        76_713,
        [[538_743, 516_779], [374_596, 428_581]],
    ),
    (
        24,
        36,
        [[645, 672], [517, 486]],
        [8134, 6631],
        51_562_393,
        48_896_976,
        -97_355,
        [[533_349, 496_619], [364_695, 418_765]],
    ),
    (
        30,
        30,
        [[501, 536], [563, 607]],
        [6955, 7641],
        51_078_358,
        47_797_438,
        186_686,
        [[516_965, 449_809], [340_904, 386_234]],
    ),
    (
        30,
        32,
        [[560, 508], [587, 580]],
        [7275, 7269],
        50_667_546,
        47_298_135,
        -79_289,
        [[495_152, 443_678], [330_274, 398_149]],
    ),
];
const FIXED_256: &[Block] = &[
    (
        29,
        34,
        [[720, 755], [602, 594]],
        [7752, 6990],
        53_005_503,
        55_810_960,
        0,
        [[960_090, 915_921], [762_137, 858_800]],
    ),
    (
        23,
        31,
        [[602, 683], [631, 620]],
        [7212, 7496],
        52_574_079,
        53_033_508,
        0,
        [[767_548, 705_079], [565_011, 634_411]],
    ),
    (
        28,
        31,
        [[597, 605], [662, 565]],
        [7083, 7491],
        52_139_864,
        51_369_273,
        0,
        [[637_754, 577_539], [440_105, 521_100]],
    ),
    (
        32,
        28,
        [[519, 521], [627, 647]],
        [6519, 8088],
        51_702_010,
        49_663_089,
        0,
        [[556_881, 517_127], [370_001, 434_937]],
    ),
    (
        31,
        30,
        [[536, 546], [612, 601]],
        [6871, 7714],
        51_287_929,
        48_486_196,
        0,
        [[494_152, 477_662], [354_761, 404_518]],
    ),
    (
        22,
        36,
        [[617, 649], [501, 464]],
        [8114, 6641],
        50_786_250,
        47_813_826,
        0,
        [[488_595, 465_693], [352_160, 392_145]],
    ),
    (
        31,
        30,
        [[490, 539], [542, 589]],
        [6945, 7640],
        50_288_994,
        46_479_676,
        0,
        [[480_517, 430_597], [330_447, 372_290]],
    ),
    (
        31,
        32,
        [[554, 496], [571, 576]],
        [7287, 7280],
        49_793_404,
        46_007_067,
        0,
        [[480_786, 420_744], [335_402, 388_917]],
    ),
];
const MIRRORED_256: &[Block] = &[
    (
        30,
        34,
        [[757, 753], [581, 586]],
        [7735, 6978],
        53_135_570,
        56_203_477,
        -179_608,
        [[1_037_465, 941_482], [817_462, 920_032]],
    ),
    (
        39,
        31,
        [[576, 682], [669, 607]],
        [7218, 7505],
        52_676_251,
        53_530_239,
        39_801,
        [[799_404, 720_942], [589_005, 665_271]],
    ),
    (
        33,
        31,
        [[609, 601], [671, 589]],
        [7090, 7496],
        52_317_258,
        51_866_765,
        71_685,
        [[661_522, 603_476], [455_746, 548_659]],
    ),
    (
        30,
        28,
        [[539, 521], [648, 653]],
        [6523, 8087],
        51_995_648,
        50_400_524,
        58_879,
        [[574_547, 544_853], [387_834, 454_164]],
    ),
    (
        30,
        30,
        [[548, 552], [630, 603]],
        [6875, 7723],
        51_690_387,
        49_304_852,
        13_472,
        [[505_702, 499_233], [367_211, 415_255]],
    ),
    (
        35,
        36,
        [[639, 651], [497, 484]],
        [8117, 6635],
        51_215_945,
        48_476_270,
        65_536,
        [[490_823, 478_634], [363_147, 397_441]],
    ),
    (
        26,
        30,
        [[499, 557], [564, 611]],
        [6953, 7644],
        50_894_815,
        47_353_070,
        -57_343,
        [[487_586, 449_508], [339_352, 379_097]],
    ),
    (
        30,
        32,
        [[554, 499], [594, 580]],
        [7281, 7288],
        50_516_239,
        47_267_128,
        -35_099,
        [[462_070, 447_700], [329_291, 382_975]],
    ),
];
const VERDICT_256: Verdict = Verdict {
    rewarded: false,
    shuffled: true,
    fixed: true,
    mirrored: false,
    workers: true,
    learned: false,
};

/// The gate's form since ADR-0061: the rewarded run's first block, sixty-four trials, held to
/// the first row of the table the full run pinned. The trials do not read how many follow, so
/// the first block of a short run is the first block of the long one; the loop runs end to end
/// on every pull request at an eighth of one full run's ticks, and the full runs are weekly.
#[test]
fn the_first_block_of_the_rewarded_run_at_256_units() {
    let (blocks, trace) = run(256, 2, BASELINE_Q16, Feedback::Answer, false, BLOCK);
    pinned(
        "learn256 first block",
        &blocks,
        trace,
        &REWARDED_256[..1],
        0,
    );
}

#[test]
#[ignore]
fn the_rewarded_run_at_256_units_on_four_workers_exhaustive() {
    let (blocks, trace) = run(256, 4, BASELINE_Q16, Feedback::Answer, false, TRIALS_256);
    pinned("learn256 rewarded", &blocks, trace, REWARDED_256, TRACE_256);
}

/// The second worker count: the same run, block for block and trial for trial (the table
/// and the trace were pinned from four workers).
#[test]
#[ignore]
fn the_rewarded_run_at_256_units_on_one_worker_is_the_same_run_exhaustive() {
    let (blocks, trace) = run(256, 1, BASELINE_Q16, Feedback::Answer, false, TRIALS_256);
    pinned(
        "learn256 rewarded-1",
        &blocks,
        trace,
        REWARDED_256,
        TRACE_256,
    );
}

#[test]
#[ignore]
fn the_shuffled_reward_at_256_units_exhaustive() {
    let (blocks, trace) = run(256, 2, BASELINE_Q16, Feedback::Shuffled, false, TRIALS_256);
    pinned("learn256 shuffled", &blocks, trace, SHUFFLED_256, 0);
}

#[test]
#[ignore]
fn the_fixed_modulation_at_256_units_exhaustive() {
    let (blocks, trace) = run(256, 2, ONE, Feedback::Withheld, false, TRIALS_256);
    pinned("learn256 fixed", &blocks, trace, FIXED_256, 0);
}

#[test]
#[ignore]
fn the_mirrored_assignment_at_256_units_exhaustive() {
    let (blocks, trace) = run(256, 2, BASELINE_Q16, Feedback::Answer, true, TRIALS_256);
    pinned("learn256 mirrored", &blocks, trace, MIRRORED_256, 0);
}

/// The criterion over the pinned tables, clause by clause; the worker clause is the test
/// above that holds one worker to four workers' table.
#[test]
fn the_criterion_at_256_units_as_written() {
    let v = verdict(REWARDED_256, SHUFFLED_256, FIXED_256, MIRRORED_256, true);
    eprintln!("DUMP learn256 verdict {v:?}");
    assert_eq!(v, VERDICT_256);
}

/// The weekly job's form: 2 048 trials (thirty-two blocks, sixty-four windows, 84 s
/// simulated) at 1 024 units.
const TRIALS_1024: usize = 32 * BLOCK;
const REWARDED_1024: &[Block] = &[
    (
        30,
        34,
        [[3193, 3211], [2667, 2635]],
        [31_065, 28_253],
        212_717_312,
        226_147_801,
        -7813,
        [[2_830_472, 2_756_758], [2_882_786, 2_747_900]],
    ),
    (
        24,
        31,
        [[2593, 2694], [2647, 2515]],
        [28_616, 30_069],
        212_079_301,
        219_248_538,
        -104_290,
        [[2_411_848, 2_310_215], [2_310_710, 2_244_441]],
    ),
    (
        31,
        31,
        [[2414, 2497], [2583, 2603]],
        [28_566, 30_081],
        211_068_235,
        213_167_750,
        -154_216,
        [[1_988_147, 1_907_411], [1_855_534, 1_788_356]],
    ),
    (
        39,
        28,
        [[2155, 2067], [2707, 2835]],
        [26_096, 32_388],
        209_753_361,
        206_635_911,
        64_069,
        [[1_675_561, 1_637_048], [1_574_000, 1_457_587]],
    ),
    (
        34,
        30,
        [[2269, 2131], [2562, 2549]],
        [27_617, 30_948],
        208_499_346,
        201_867_861,
        -65_536,
        [[1_516_223, 1_484_242], [1_462_166, 1_344_218]],
    ),
    (
        23,
        36,
        [[2570, 2601], [2158, 2008]],
        [32_237, 26_319],
        207_798_158,
        199_075_367,
        -187_725,
        [[1_422_931, 1_422_286], [1_424_174, 1_316_346]],
    ),
    (
        35,
        30,
        [[2192, 2200], [2403, 2427]],
        [27_625, 30_667],
        206_502_591,
        198_031_516,
        259_346,
        [[1_420_952, 1_372_359], [1_317_849, 1_240_364]],
    ),
    (
        30,
        32,
        [[2265, 2164], [2308, 2323]],
        [29_274, 29_298],
        205_390_855,
        194_893_434,
        -66_471,
        [[1_368_107, 1_335_181], [1_297_653, 1_228_550]],
    ),
    (
        30,
        34,
        [[2437, 2470], [2037, 2067]],
        [30_671, 27_655],
        204_136_858,
        194_084_139,
        7253,
        [[1_356_256, 1_293_970], [1_297_586, 1_198_323]],
    ),
    (
        33,
        28,
        [[1945, 1932], [2512, 2556]],
        [26_243, 32_287],
        203_016_649,
        191_877_771,
        34_660,
        [[1_323_489, 1_284_127], [1_239_898, 1_194_196]],
    ),
    (
        39,
        33,
        [[2401, 2294], [2179, 2170]],
        [29_994, 28_325],
        201_692_127,
        189_468_813,
        -166_805,
        [[1_325_021, 1_285_018], [1_248_642, 1_177_331]],
    ),
    (
        33,
        32,
        [[2183, 2152], [2268, 2257]],
        [29_205, 29_370],
        200_378_688,
        187_785_124,
        10_496,
        [[1_321_723, 1_293_058], [1_253_632, 1_154_124]],
    ),
    (
        34,
        32,
        [[2159, 2089], [2191, 2238]],
        [29_344, 29_188],
        199_164_613,
        186_988_107,
        68_566,
        [[1_291_588, 1_282_273], [1_251_752, 1_198_257]],
    ),
    (
        28,
        33,
        [[2344, 2263], [2201, 2065]],
        [29_978, 28_300],
        198_178_140,
        185_054_791,
        -58_402,
        [[1_305_880, 1_274_715], [1_252_731, 1_196_430]],
    ),
    (
        25,
        37,
        [[2485, 2540], [1904, 1827]],
        [32_942, 25_370],
        197_409_705,
        183_604_188,
        33_616,
        [[1_306_256, 1_293_635], [1_264_753, 1_208_796]],
    ),
    (
        31,
        35,
        [[2333, 2314], [1976, 1980]],
        [31_334, 26_788],
        196_292_892,
        183_581_240,
        -92_101,
        [[1_295_773, 1_274_867], [1_231_265, 1_207_590]],
    ),
    (
        28,
        33,
        [[2145, 2208], [2090, 2039]],
        [30_016, 28_505],
        195_247_692,
        182_032_749,
        71_791,
        [[1_298_049, 1_286_411], [1_274_367, 1_209_392]],
    ),
    (
        32,
        38,
        [[2556, 2665], [1749, 1842]],
        [33_714, 24_608],
        194_210_617,
        180_889_183,
        -16_482,
        [[1_281_388, 1_258_755], [1_267_500, 1_227_683]],
    ),
    (
        33,
        31,
        [[2100, 2080], [2158, 2278]],
        [28_449, 29_915],
        193_169_116,
        181_296_462,
        877,
        [[1_277_146, 1_268_818], [1_250_119, 1_214_727]],
    ),
    (
        31,
        34,
        [[2396, 2330], [1990, 2087]],
        [30_704, 27_576],
        192_436_667,
        179_420_571,
        -28_570,
        [[1_301_547, 1_286_084], [1_238_404, 1_209_496]],
    ),
    (
        31,
        30,
        [[2064, 1965], [2247, 2311]],
        [27_526, 30_741],
        191_590_765,
        179_475_702,
        39_881,
        [[1_286_154, 1_276_882], [1_236_533, 1_211_162]],
    ),
    (
        31,
        29,
        [[1954, 1909], [2412, 2462]],
        [26_704, 31_516],
        190_792_281,
        178_829_623,
        -47_135,
        [[1_289_293, 1_277_497], [1_256_559, 1_224_222]],
    ),
    (
        25,
        32,
        [[2058, 2204], [2150, 2126]],
        [29_111, 29_112],
        190_251_959,
        177_189_717,
        3817,
        [[1_296_523, 1_280_241], [1_253_869, 1_221_555]],
    ),
    (
        33,
        29,
        [[1924, 1960], [2368, 2354]],
        [26_751, 31_554],
        189_617_733,
        176_104_385,
        -172_450,
        [[1_313_438, 1_265_614], [1_224_718, 1_205_265]],
    ),
    (
        33,
        29,
        [[2026, 1967], [2338, 2265]],
        [26_961, 31_320],
        188_853_320,
        177_436_564,
        -2728,
        [[1_322_552, 1_274_697], [1_217_529, 1_183_879]],
    ),
    (
        36,
        33,
        [[2227, 2204], [2011, 2043]],
        [29_950, 28_368],
        188_152_284,
        176_420_069,
        134_695,
        [[1_307_718, 1_273_654], [1_242_920, 1_193_008]],
    ),
    (
        27,
        36,
        [[2254, 2476], [1857, 1786]],
        [32_261, 26_181],
        187_560_575,
        174_817_796,
        -58_402,
        [[1_303_254, 1_274_675], [1_272_559, 1_194_288]],
    ),
    (
        31,
        32,
        [[2079, 2048], [2055, 2080]],
        [29_236, 29_055],
        186_853_802,
        174_889_184,
        -90_305,
        [[1_306_832, 1_290_653], [1_278_512, 1_213_766]],
    ),
    (
        25,
        34,
        [[2198, 2227], [2020, 1926]],
        [30_687, 27_657],
        186_437_265,
        174_693_921,
        65_536,
        [[1_310_881, 1_273_170], [1_274_176, 1_210_343]],
    ),
    (
        26,
        34,
        [[2138, 2369], [1955, 2042]],
        [30_589, 27_699],
        185_866_385,
        174_458_052,
        -348,
        [[1_286_177, 1_270_478], [1_286_546, 1_210_435]],
    ),
    (
        28,
        35,
        [[2342, 2346], [1904, 1848]],
        [31_476, 26_857],
        185_363_802,
        173_889_681,
        -56_631,
        [[1_277_564, 1_263_995], [1_268_963, 1_211_966]],
    ),
    (
        23,
        29,
        [[1847, 2010], [2342, 2205]],
        [26_824, 31_601],
        184_893_686,
        174_687_832,
        -28_865,
        [[1_285_516, 1_270_784], [1_269_523, 1_201_564]],
    ),
];
const TRACE_1024: u64 = 0x783de2769ceb3d33;
const SHUFFLED_1024: &[Block] = &[
    (
        32,
        34,
        [[3239, 3246], [2710, 2712]],
        [31_075, 28_261],
        213_035_382,
        229_169_172,
        179_398,
        [[3_081_382, 3_047_008], [3_049_980, 2_942_684]],
    ),
    (
        26,
        31,
        [[2642, 2712], [2666, 2543]],
        [28_630, 30_092],
        212_282_703,
        222_041_651,
        -117_018,
        [[2_492_661, 2_458_019], [2_474_559, 2_408_337]],
    ),
    (
        29,
        31,
        [[2421, 2516], [2669, 2632]],
        [28_536, 30_048],
        211_257_595,
        214_390_155,
        45_070,
        [[2_012_132, 1_991_737], [1_914_763, 1_843_984]],
    ),
    (
        37,
        28,
        [[2157, 2071], [2729, 2809]],
        [26_100, 32_392],
        210_063_788,
        208_262_390,
        248_012,
        [[1_714_279, 1_720_977], [1_631_302, 1_502_643]],
    ),
    (
        29,
        30,
        [[2293, 2149], [2570, 2560]],
        [27_596, 30_982],
        208_678_358,
        203_621_823,
        76_713,
        [[1_546_018, 1_532_411], [1_477_565, 1_355_882]],
    ),
    (
        29,
        36,
        [[2591, 2604], [2113, 2022]],
        [32_239, 26_332],
        207_762_374,
        199_602_814,
        -97_355,
        [[1_441_654, 1_468_003], [1_409_096, 1_313_527]],
    ),
    (
        33,
        30,
        [[2161, 2181], [2410, 2443]],
        [27_641, 30_682],
        206_307_221,
        197_610_014,
        186_686,
        [[1_421_337, 1_390_438], [1_308_135, 1_240_062]],
    ),
    (
        33,
        32,
        [[2245, 2132], [2274, 2321]],
        [29_257, 29_293],
        205_061_869,
        194_520_786,
        -79_289,
        [[1_376_897, 1_342_618], [1_293_005, 1_231_448]],
    ),
    (
        31,
        34,
        [[2419, 2453], [2036, 2059]],
        [30_694, 27_678],
        203_618_399,
        192_637_182,
        -128_409,
        [[1_354_221, 1_301_915], [1_291_139, 1_209_642]],
    ),
    (
        30,
        28,
        [[1952, 1965], [2481, 2533]],
        [26_257, 32_267],
        202_386_059,
        190_540_017,
        203_153,
        [[1_325_254, 1_294_287], [1_243_414, 1_201_989]],
    ),
    (
        37,
        33,
        [[2359, 2265], [2161, 2190]],
        [30_019, 28_323],
        201_046_742,
        188_143_349,
        -73_729,
        [[1_328_128, 1_289_242], [1_253_000, 1_179_289]],
    ),
    (
        36,
        32,
        [[2182, 2138], [2249, 2247]],
        [29_187, 29_366],
        199_734_842,
        186_686_569,
        -76_999,
        [[1_313_674, 1_290_644], [1_250_234, 1_160_649]],
    ),
    (
        36,
        32,
        [[2162, 2126], [2188, 2233]],
        [29_335, 29_193],
        198_529_460,
        186_328_384,
        -65_536,
        [[1_287_229, 1_280_280], [1_244_481, 1_201_954]],
    ),
    (
        32,
        33,
        [[2332, 2248], [2178, 2048]],
        [29_984, 28_335],
        197_448_728,
        184_162_131,
        -146_422,
        [[1_287_843, 1_288_294], [1_255_209, 1_186_341]],
    ),
    (
        27,
        37,
        [[2445, 2488], [1894, 1824]],
        [32_941, 25_383],
        196_143_761,
        182_683_019,
        13_943,
        [[1_269_074, 1_306_053], [1_268_180, 1_207_779]],
    ),
    (
        38,
        35,
        [[2337, 2295], [1948, 1973]],
        [31_349, 26_778],
        195_167_359,
        182_402_796,
        -97_905,
        [[1_276_849, 1_278_501], [1_257_021, 1_220_915]],
    ),
    (
        28,
        33,
        [[2124, 2195], [2121, 2064]],
        [29_986, 28_563],
        194_213_587,
        181_358_665,
        146_477,
        [[1_284_567, 1_292_819], [1_285_553, 1_217_674]],
    ),
    (
        33,
        38,
        [[2560, 2685], [1741, 1831]],
        [33_685, 24_594],
        193_195_365,
        180_073_888,
        -78_074,
        [[1_262_963, 1_259_973], [1_274_125, 1_237_504]],
    ),
    (
        31,
        31,
        [[2089, 2092], [2171, 2246]],
        [28_483, 29_913],
        192_296_742,
        180_485_037,
        -96_611,
        [[1_262_121, 1_262_276], [1_261_904, 1_217_898]],
    ),
    (
        30,
        34,
        [[2349, 2322], [1977, 2053]],
        [30_697, 27_571],
        191_424_472,
        179_287_376,
        63_488,
        [[1_273_397, 1_282_435], [1_248_874, 1_211_485]],
    ),
    (
        34,
        30,
        [[2087, 1992], [2252, 2317]],
        [27_515, 30_746],
        190_491_282,
        179_310_813,
        28_971,
        [[1_272_609, 1_278_828], [1_242_062, 1_213_401]],
    ),
    (
        27,
        29,
        [[1972, 1902], [2426, 2436]],
        [26_705, 31_517],
        189_779_429,
        178_046_417,
        -43_979,
        [[1_277_628, 1_277_515], [1_264_182, 1_217_667]],
    ),
    (
        26,
        32,
        [[2071, 2175], [2151, 2120]],
        [29_119, 29_127],
        189_022_017,
        176_988_235,
        -27_403,
        [[1_291_795, 1_279_411], [1_259_130, 1_220_136]],
    ),
    (
        31,
        29,
        [[1917, 1949], [2372, 2320]],
        [26_722, 31_536],
        188_365_898,
        175_116_824,
        -23_915,
        [[1_319_435, 1_279_056], [1_233_889, 1_192_288]],
    ),
    (
        31,
        29,
        [[2009, 1942], [2353, 2281]],
        [26_945, 31_327],
        187_750_582,
        176_508_631,
        145_610,
        [[1_314_890, 1_268_900], [1_245_434, 1_178_769]],
    ),
    (
        33,
        33,
        [[2208, 2229], [1993, 2033]],
        [29_975, 28_369],
        187_109_383,
        175_908_120,
        -28_865,
        [[1_298_481, 1_283_564], [1_250_280, 1_179_243]],
    ),
    (
        24,
        36,
        [[2264, 2460], [1846, 1786]],
        [32_270, 26_149],
        186_361_534,
        174_145_166,
        -65_536,
        [[1_305_068, 1_296_238], [1_277_738, 1_196_451]],
    ),
    (
        33,
        32,
        [[2104, 2050], [2024, 2081]],
        [29_257, 29_082],
        185_829_108,
        174_191_780,
        -11_539,
        [[1_308_370, 1_292_809], [1_292_254, 1_218_731]],
    ),
    (
        28,
        34,
        [[2220, 2209], [2019, 1923]],
        [30_691, 27_661],
        185_194_331,
        174_203_073,
        -123_762,
        [[1_311_150, 1_286_418], [1_270_466, 1_208_024]],
    ),
    (
        28,
        34,
        [[2159, 2350], [1953, 2028]],
        [30_607, 27_703],
        184_541_868,
        173_979_106,
        61_672,
        [[1_285_954, 1_279_500], [1_296_773, 1_206_380]],
    ),
    (
        28,
        35,
        [[2336, 2356], [1894, 1854]],
        [31_477, 26_873],
        183_963_121,
        173_360_959,
        65_536,
        [[1_259_493, 1_278_788], [1_286_207, 1_201_440]],
    ),
    (
        26,
        29,
        [[1849, 2005], [2344, 2218]],
        [26_838, 31_587],
        183_441_761,
        173_949_072,
        -111_947,
        [[1_267_395, 1_282_285], [1_276_107, 1_194_891]],
    ),
];
const FIXED_1024: &[Block] = &[
    (
        33,
        34,
        [[3204, 3116], [2628, 2644]],
        [31_055, 28_244],
        212_510_923,
        223_725_731,
        0,
        [[2_680_190, 2_633_469], [2_767_711, 2_638_022]],
    ),
    (
        29,
        31,
        [[2527, 2633], [2614, 2462]],
        [28_628, 30_066],
        211_181_170,
        213_883_247,
        0,
        [[2_096_488, 2_041_884], [2_020_985, 1_961_786]],
    ),
    (
        29,
        31,
        [[2301, 2368], [2485, 2531]],
        [28_521, 30_042],
        209_807_328,
        207_189_331,
        0,
        [[1_753_884, 1_694_096], [1_660_216, 1_609_970]],
    ),
    (
        32,
        28,
        [[2073, 2025], [2639, 2670]],
        [26_064, 32_371],
        208_358_657,
        201_729_758,
        0,
        [[1_544_648, 1_520_161], [1_462_267, 1_359_209]],
    ),
    (
        31,
        30,
        [[2222, 2082], [2530, 2471]],
        [27_596, 30_970],
        206_861_037,
        197_874_819,
        0,
        [[1_437_449, 1_417_953], [1_381_178, 1_282_791]],
    ),
    (
        26,
        36,
        [[2472, 2531], [2078, 1965]],
        [32_231, 26_306],
        205_360_537,
        194_719_794,
        0,
        [[1_363_562, 1_368_216], [1_329_901, 1_250_730]],
    ),
    (
        35,
        30,
        [[2118, 2134], [2326, 2359]],
        [27_611, 30_657],
        203_782_408,
        192_468_458,
        0,
        [[1_368_854, 1_327_129], [1_260_972, 1_206_925]],
    ),
    (
        35,
        32,
        [[2206, 2096], [2218, 2286]],
        [29_267, 29_287],
        202_239_203,
        189_615_040,
        0,
        [[1_327_478, 1_288_389], [1_268_454, 1_220_374]],
    ),
    (
        27,
        34,
        [[2352, 2402], [2013, 2006]],
        [30_665, 27_662],
        200_629_597,
        189_051_189,
        0,
        [[1_293_873, 1_260_982], [1_274_501, 1_206_510]],
    ),
    (
        32,
        28,
        [[1900, 1871], [2457, 2471]],
        [26_235, 32_268],
        199_151_767,
        186_492_223,
        0,
        [[1_288_989, 1_258_774], [1_217_162, 1_208_629]],
    ),
    (
        32,
        33,
        [[2323, 2280], [2117, 2136]],
        [30_017, 28_315],
        197_790_167,
        184_449_899,
        0,
        [[1_303_878, 1_267_859], [1_240_929, 1_181_143]],
    ),
    (
        34,
        32,
        [[2120, 2094], [2204, 2205]],
        [29_180, 29_368],
        196_249_775,
        183_213_988,
        0,
        [[1_293_931, 1_278_414], [1_245_948, 1_161_345]],
    ),
    (
        32,
        32,
        [[2086, 2080], [2136, 2184]],
        [29_348, 29_212],
        194_874_857,
        182_989_186,
        0,
        [[1_272_005, 1_271_663], [1_237_913, 1_197_770]],
    ),
    (
        28,
        33,
        [[2298, 2217], [2150, 2019]],
        [29_996, 28_306],
        193_597_903,
        181_048_265,
        0,
        [[1_264_154, 1_275_089], [1_263_254, 1_190_258]],
    ),
    (
        27,
        37,
        [[2416, 2482], [1841, 1820]],
        [32_955, 25_415],
        192_356_592,
        179_385_590,
        0,
        [[1_267_783, 1_304_450], [1_267_540, 1_204_252]],
    ),
    (
        37,
        35,
        [[2283, 2271], [1894, 1963]],
        [31_335, 26_768],
        191_126_423,
        178_515_644,
        0,
        [[1_273_218, 1_271_200], [1_263_511, 1_203_295]],
    ),
    (
        26,
        33,
        [[2105, 2187], [2067, 2002]],
        [29_971, 28_543],
        190_052_793,
        177_958_933,
        0,
        [[1_275_086, 1_290_871], [1_287_426, 1_204_346]],
    ),
    (
        30,
        38,
        [[2472, 2645], [1717, 1792]],
        [33_705, 24_593],
        189_037_831,
        176_873_458,
        0,
        [[1_260_168, 1_256_942], [1_273_789, 1_220_848]],
    ),
    (
        34,
        31,
        [[2058, 2055], [2151, 2253]],
        [28_472, 29_919],
        188_110_255,
        177_479_384,
        0,
        [[1_264_974, 1_245_713], [1_257_864, 1_201_764]],
    ),
    (
        33,
        34,
        [[2310, 2285], [1964, 2032]],
        [30_721, 27_594],
        187_258_247,
        176_788_816,
        0,
        [[1_265_840, 1_262_765], [1_249_191, 1_197_888]],
    ),
    (
        27,
        30,
        [[2041, 1958], [2230, 2258]],
        [27_518, 30_745],
        186_327_575,
        176_088_837,
        0,
        [[1_258_640, 1_272_208], [1_243_077, 1_196_992]],
    ),
    (
        30,
        29,
        [[1921, 1875], [2406, 2391]],
        [26_732, 31_536],
        185_522_418,
        175_108_835,
        0,
        [[1_261_171, 1_279_924], [1_266_020, 1_214_528]],
    ),
    (
        24,
        32,
        [[2029, 2179], [2127, 2113]],
        [29_117, 29_134],
        184_692_809,
        174_646_816,
        0,
        [[1_278_797, 1_283_696], [1_263_462, 1_212_215]],
    ),
    (
        28,
        29,
        [[1866, 1947], [2338, 2319]],
        [26_728, 31_540],
        183_969_146,
        172_934_575,
        0,
        [[1_293_118, 1_289_524], [1_230_983, 1_193_340]],
    ),
    (
        33,
        29,
        [[2009, 1927], [2326, 2270]],
        [26_975, 31_328],
        183_162_412,
        174_428_019,
        0,
        [[1_293_162, 1_262_568], [1_232_183, 1_177_266]],
    ),
    (
        37,
        33,
        [[2204, 2192], [1970, 2021]],
        [29_982, 28_391],
        182_450_884,
        173_386_132,
        0,
        [[1_286_445, 1_271_275], [1_248_249, 1_174_795]],
    ),
    (
        26,
        36,
        [[2250, 2440], [1810, 1786]],
        [32_282, 26_153],
        181_578_647,
        171_755_511,
        0,
        [[1_293_089, 1_273_303], [1_279_479, 1_198_352]],
    ),
    (
        29,
        32,
        [[2062, 2046], [2034, 2064]],
        [29_234, 29_074],
        180_742_485,
        172_625_359,
        0,
        [[1_305_409, 1_287_965], [1_279_187, 1_209_027]],
    ),
    (
        25,
        34,
        [[2186, 2211], [2006, 1898]],
        [30_687, 27_673],
        180_033_245,
        171_700_303,
        0,
        [[1_302_159, 1_267_313], [1_271_024, 1_205_710]],
    ),
    (
        28,
        34,
        [[2151, 2342], [1923, 2015]],
        [30_611, 27_699],
        179_344_912,
        171_821_272,
        0,
        [[1_278_917, 1_272_288], [1_292_915, 1_196_940]],
    ),
    (
        31,
        35,
        [[2311, 2355], [1901, 1867]],
        [31_485, 26_874],
        178_620_098,
        171_711_948,
        0,
        [[1_260_515, 1_273_588], [1_286_550, 1_192_251]],
    ),
    (
        26,
        29,
        [[1834, 2000], [2315, 2213]],
        [26_854, 31_602],
        177_848_790,
        172_392_217,
        0,
        [[1_278_836, 1_277_259], [1_267_306, 1_172_352]],
    ),
];
const MIRRORED_1024: &[Block] = &[
    (
        33,
        34,
        [[3246, 3191], [2610, 2597]],
        [31_070, 28_246],
        212_716_633,
        225_717_278,
        -64_683,
        [[2_755_088, 2_735_584], [2_959_273, 2_810_698]],
    ),
    (
        37,
        31,
        [[2566, 2690], [2617, 2494]],
        [28_627, 30_062],
        211_486_595,
        215_589_389,
        107_105,
        [[2_149_740, 2_098_733], [2_118_835, 2_050_808]],
    ),
    (
        33,
        31,
        [[2334, 2461], [2510, 2568]],
        [28_486, 30_047],
        210_386_683,
        209_630_537,
        151_024,
        [[1_806_218, 1_764_597], [1_760_742, 1_677_947]],
    ),
    (
        29,
        28,
        [[2080, 2074], [2668, 2798]],
        [26_060, 32_373],
        209_375_702,
        204_651_428,
        -34_239,
        [[1_598_179, 1_603_301], [1_557_114, 1_423_335]],
    ),
    (
        29,
        30,
        [[2284, 2123], [2560, 2508]],
        [27_590, 30_946],
        208_254_233,
        200_846_408,
        110_883,
        [[1_493_133, 1_486_341], [1_435_294, 1_327_269]],
    ),
    (
        40,
        36,
        [[2521, 2578], [2139, 1994]],
        [32_224, 26_300],
        206_868_287,
        198_244_582,
        188_783,
        [[1_386_486, 1_400_769], [1_366_795, 1_280_519]],
    ),
    (
        31,
        30,
        [[2166, 2153], [2368, 2406]],
        [27_630, 30_667],
        205_694_640,
        195_835_446,
        -72_437,
        [[1_372_420, 1_371_678], [1_305_470, 1_228_274]],
    ),
    (
        28,
        32,
        [[2262, 2132], [2248, 2313]],
        [29_231, 29_259],
        204_583_428,
        193_295_811,
        26_937,
        [[1_342_516, 1_322_176], [1_281_122, 1_223_553]],
    ),
    (
        32,
        34,
        [[2405, 2426], [2059, 2064]],
        [30_670, 27_663],
        203_176_421,
        192_526_619,
        -33_971,
        [[1_313_233, 1_284_082], [1_286_939, 1_205_596]],
    ),
    (
        32,
        28,
        [[1899, 1913], [2501, 2546]],
        [26_232, 32_246],
        202_133_497,
        190_248_371,
        -25_048,
        [[1_297_854, 1_284_985], [1_241_751, 1_204_857]],
    ),
    (
        31,
        33,
        [[2372, 2286], [2181, 2186]],
        [30_009, 28_328],
        200_977_073,
        188_119_446,
        79_099,
        [[1_310_717, 1_272_088], [1_254_388, 1_174_195]],
    ),
    (
        31,
        32,
        [[2191, 2154], [2253, 2239]],
        [29_193, 29_372],
        199_765_628,
        186_738_184,
        -132_515,
        [[1_302_743, 1_287_750], [1_266_973, 1_165_631]],
    ),
    (
        28,
        32,
        [[2154, 2107], [2183, 2222]],
        [29_334, 29_203],
        198_719_396,
        186_839_076,
        -112_941,
        [[1_287_293, 1_290_126], [1_250_807, 1_194_365]],
    ),
    (
        28,
        33,
        [[2346, 2253], [2167, 2076]],
        [29_991, 28_346],
        197_619_551,
        184_752_761,
        46_065,
        [[1_272_694, 1_284_482], [1_267_985, 1_188_136]],
    ),
    (
        36,
        37,
        [[2474, 2513], [1881, 1829]],
        [32_961, 25_377],
        196_300_981,
        183_011_816,
        68_010,
        [[1_272_253, 1_306_652], [1_281_505, 1_198_572]],
    ),
    (
        28,
        35,
        [[2338, 2297], [1969, 1964]],
        [31_334, 26_807],
        195_390_414,
        181_924_732,
        63_147,
        [[1_262_363, 1_295_968], [1_280_422, 1_208_338]],
    ),
    (
        39,
        33,
        [[2110, 2235], [2141, 2045]],
        [29_986, 28_547],
        194_183_983,
        181_597_461,
        -63_929,
        [[1_270_561, 1_304_521], [1_301_185, 1_206_491]],
    ),
    (
        28,
        38,
        [[2548, 2702], [1733, 1820]],
        [33_714, 24_574],
        193_377_851,
        179_845_688,
        -12_608,
        [[1_272_188, 1_271_848], [1_289_655, 1_228_241]],
    ),
    (
        36,
        31,
        [[2073, 2086], [2202, 2281]],
        [28_474, 29_913],
        192_334_790,
        180_803_413,
        42_313,
        [[1_274_773, 1_276_169], [1_264_372, 1_211_858]],
    ),
    (
        30,
        34,
        [[2344, 2314], [1966, 2045]],
        [30_713, 27_573],
        191_532_723,
        179_470_579,
        10_589,
        [[1_273_199, 1_291_655], [1_244_052, 1_204_793]],
    ),
    (
        30,
        30,
        [[2079, 2004], [2262, 2295]],
        [27_509, 30_732],
        190_802_973,
        178_991_279,
        -74_799,
        [[1_274_604, 1_288_381], [1_239_124, 1_195_774]],
    ),
    (
        33,
        29,
        [[1952, 1920], [2403, 2443]],
        [26_723, 31_516],
        190_054_637,
        177_893_203,
        74_747,
        [[1_280_155, 1_289_011], [1_262_044, 1_209_303]],
    ),
    (
        35,
        32,
        [[2055, 2168], [2141, 2141]],
        [29_108, 29_125],
        189_280_475,
        177_397_636,
        -22_489,
        [[1_285_297, 1_280_646], [1_262_223, 1_209_010]],
    ),
    (
        33,
        29,
        [[1915, 1983], [2368, 2329]],
        [26_722, 31_538],
        188_679_900,
        175_509_765,
        210_611,
        [[1_301_944, 1_289_942], [1_226_697, 1_191_409]],
    ),
    (
        28,
        29,
        [[2008, 1935], [2363, 2267]],
        [26_964, 31_334],
        188_037_134,
        176_513_460,
        -32_865,
        [[1_299_047, 1_280_964], [1_238_103, 1_178_580]],
    ),
    (
        30,
        33,
        [[2230, 2248], [2021, 2036]],
        [29_968, 28_383],
        187_452_040,
        175_690_234,
        -229_447,
        [[1_288_210, 1_283_935], [1_252_632, 1_174_747]],
    ),
    (
        36,
        36,
        [[2267, 2469], [1853, 1806]],
        [32_278, 26_160],
        186_679_374,
        174_195_893,
        61_638,
        [[1_291_945, 1_281_008], [1_298_382, 1_196_153]],
    ),
    (
        28,
        32,
        [[2097, 2054], [2030, 2077]],
        [29_223, 29_074],
        186_024_448,
        174_831_714,
        -41_418,
        [[1_309_254, 1_293_137], [1_292_534, 1_215_352]],
    ),
    (
        36,
        34,
        [[2196, 2212], [2014, 1913]],
        [30_679, 27_668],
        185_312_888,
        174_093_980,
        -86_255,
        [[1_317_118, 1_276_736], [1_276_222, 1_211_880]],
    ),
    (
        36,
        34,
        [[2142, 2374], [1976, 2038]],
        [30_595, 27_714],
        184_714_764,
        174_093_263,
        65_536,
        [[1_283_784, 1_283_789], [1_301_973, 1_207_789]],
    ),
    (
        32,
        35,
        [[2326, 2342], [1900, 1857]],
        [31_489, 26_872],
        184_057_785,
        173_436_161,
        -24_247,
        [[1_270_709, 1_284_558], [1_288_686, 1_209_249]],
    ),
    (
        36,
        29,
        [[1858, 2009], [2351, 2212]],
        [26_851, 31_622],
        183_429_080,
        174_291_447,
        12_177,
        [[1_276_932, 1_286_137], [1_276_277, 1_192_475]],
    ),
];
const VERDICT_1024: Verdict = Verdict {
    rewarded: false,
    shuffled: true,
    fixed: true,
    mirrored: false,
    workers: true,
    learned: false,
};

#[test]
#[ignore]
fn the_rewarded_run_at_1024_units_on_four_workers_exhaustive() {
    let (blocks, trace) = run(1024, 4, BASELINE_Q16, Feedback::Answer, false, TRIALS_1024);
    pinned(
        "learn1024 rewarded",
        &blocks,
        trace,
        REWARDED_1024,
        TRACE_1024,
    );
}

#[test]
#[ignore]
fn the_rewarded_run_at_1024_units_on_one_worker_is_the_same_run_exhaustive() {
    let (blocks, trace) = run(1024, 1, BASELINE_Q16, Feedback::Answer, false, TRIALS_1024);
    pinned(
        "learn1024 rewarded-1",
        &blocks,
        trace,
        REWARDED_1024,
        TRACE_1024,
    );
}

#[test]
#[ignore]
fn the_shuffled_reward_at_1024_units_exhaustive() {
    let (blocks, trace) = run(
        1024,
        2,
        BASELINE_Q16,
        Feedback::Shuffled,
        false,
        TRIALS_1024,
    );
    pinned("learn1024 shuffled", &blocks, trace, SHUFFLED_1024, 0);
}

#[test]
#[ignore]
fn the_fixed_modulation_at_1024_units_exhaustive() {
    let (blocks, trace) = run(1024, 2, ONE, Feedback::Withheld, false, TRIALS_1024);
    pinned("learn1024 fixed", &blocks, trace, FIXED_1024, 0);
}

#[test]
#[ignore]
fn the_mirrored_assignment_at_1024_units_exhaustive() {
    let (blocks, trace) = run(1024, 2, BASELINE_Q16, Feedback::Answer, true, TRIALS_1024);
    pinned("learn1024 mirrored", &blocks, trace, MIRRORED_1024, 0);
}

#[test]
#[ignore]
fn the_criterion_at_1024_units_as_written_exhaustive() {
    let v = verdict(
        REWARDED_1024,
        SHUFFLED_1024,
        FIXED_1024,
        MIRRORED_1024,
        true,
    );
    eprintln!("DUMP learn1024 verdict {v:?}");
    assert_eq!(v, VERDICT_1024);
}

/// The task fits the executor at both sizes, the quarters tile the ring, and the criterion's
/// arithmetic reads the thresholds as written (ten of sixty-four is a rise of 0.15; three is
/// below 0.05, four is not).
#[test]
fn the_task_fits_the_reference_network_and_the_criterion_reads_as_written() {
    for units in [256u32, 1024] {
        let [a, r0, b, r1] = quarters(units);
        assert_eq!(a.end(), u64::from(r0.first));
        assert_eq!(r0.end(), u64::from(b.first));
        assert_eq!(b.end(), u64::from(r1.first));
        assert_eq!(
            r1.end(),
            u64::from(units),
            "the four quarters tile the ring"
        );
        let exec = Engine::new(config(units, 1, BASELINE_Q16)).unwrap();
        for feedback in [Feedback::Answer, Feedback::Shuffled] {
            assert_eq!(task(units, feedback, false).check(&exec), Ok(()));
            assert_eq!(task(units, feedback, true).check(&exec), Ok(()));
        }
        let fixed = Engine::new(config(units, 1, ONE)).unwrap();
        assert_eq!(task(units, Feedback::Withheld, false).check(&fixed), Ok(()));
        assert_eq!(
            task(units, Feedback::Answer, false).check(&fixed),
            Err(TaskError::RewardAtCeiling),
            "the rewarded run needs room above the baseline"
        );
    }
    let block = |correct: u32| -> Block { (correct, 0, [[0; 2]; 2], [0; 2], 0, 0, 0, [[0; 2]; 2]) };
    assert_eq!(rise(&[block(30), block(40)]), 10);
    assert_eq!(rise(&[block(40), block(35), block(30)]), -10);
    assert_eq!(rise(&[]), 0);
    assert!(at_least(10, RISE_PER_CENT), "ten of sixty-four is 0.156");
    assert!(!at_least(9, RISE_PER_CENT), "nine is 0.141");
    assert!(
        !at_least(3, CONTROL_PER_CENT),
        "three of sixty-four is 0.047"
    );
    assert!(at_least(4, CONTROL_PER_CENT), "four is 0.0625");
    assert!(!at_least(-64, CONTROL_PER_CENT), "a fall is below any rise");
    let up = [block(32), block(42)];
    let flat = [block(32), block(35)];
    let drift = [block(32), block(36)];
    assert_eq!(
        verdict(&up, &flat, &flat, &up, true),
        Verdict {
            rewarded: true,
            shuffled: true,
            fixed: true,
            mirrored: true,
            workers: true,
            learned: true
        }
    );
    assert_eq!(
        verdict(&up, &drift, &flat, &up, true),
        Verdict {
            rewarded: true,
            shuffled: false,
            fixed: true,
            mirrored: true,
            workers: true,
            learned: false
        },
        "a control that rises by four of sixty-four fails its clause"
    );
    assert!(
        !verdict(&up, &flat, &flat, &flat, true).learned,
        "the mirrored assignment must rise too"
    );
    assert!(!verdict(&up, &flat, &flat, &up, false).learned);
    assert!(!verdict(&flat, &flat, &flat, &flat, true).rewarded);
}
