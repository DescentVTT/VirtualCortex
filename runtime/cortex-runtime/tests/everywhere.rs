//! Brief 038 (ADR-0083) runs H-15 as ADR-0082 wrote it: H-14's configuration with the
//! modulation baseline at 0.5 instead of zero — brief 027's `BASELINE_Q16`, the baseline
//! ADR-0077's lead-in settled the network under and every rewarded run of H-12 used — every
//! other constant H-14's, so that every synapse consolidates half of what it pairs and a
//! reward still has room. The settled engine is built as ADR-0077, ADR-0079 and ADR-0081
//! build it and held to ADR-0077's tables; its image with the baseline patched to zero is the
//! calibration's, a frozen block from it held to ADR-0077's frozen run before any rewarded
//! run; its image as encoded, the baseline 0.5, is the arms'. Three arms of 1 536 trials from
//! that image under the task as built (`Delivery::Addressed`): the assignment and the
//! mirrored assignment (`Feedback::Answer`), and the reward withheld (`Feedback::Withheld`),
//! a reading of what the unrewarded consolidation does alone. ADR-0079's oracle consolidates
//! every stimulus–readout synapse under the baseline plus the signal where the synapse is
//! addressed and under the baseline alone elsewhere, held to the record at every trial. The
//! criterion — the correct selections over the last 128 trials at least 80 in both rewarded
//! arms — the assertion (the signal at every trial's end between the fixed points ±0.712, the
//! wrong pair's modulation after a wrong selection at most 0.2125, the answer pair's after a
//! correct one at least 0.7875) and ADR-0082's two predicted readings (the withheld arm
//! drifting toward readout 1, the stimulus whose answer is readout 0 deciding each rewarded
//! arm) are integer rules written before the run. The three arms are one weekly `exhaustive`
//! test; the gate runs the rules at their edges and eight trials under the baseline.
//!
//! The harness is `tests/instrument.rs`'s, shared as one module and not copied; this binary
//! is named so that `scripts/exhaustive-shard.sh`'s round robin over the sorted binaries
//! leaves `instrument` alone in its shard (ADR-0073, ADR-0082).

#![deny(clippy::arithmetic_side_effects)]

// The harness — the network, the task, the readings, the oracle, every rule and every pinned
// table of ADR-0065 to ADR-0081 — is `tests/instrument.rs`'s module, compiled into this binary
// as well. What this binary does not call is that binary's, so the module's unused items and
// imports are allowed here and nowhere else in this file.
#[allow(dead_code, unused_imports)]
#[path = "instrument/harness.rs"]
mod harness;
use harness::*;

// ------------------------------------------- written before the run (ADR-0082)

/// H-15's baseline: brief 027's `BASELINE_Q16`, 0.5, the baseline `candidate` settled the
/// network under and every rewarded run of H-12 used; the one constant moved from H-14's
/// zero. The executor's default, 1.0 (`Config::default`, ADR-0022's rule), is where a reward
/// adds nothing and `Task::check` refuses one.
const EVERYWHERE_BASELINE_Q16: i32 = BASELINE_Q16;
const _: () = assert!(EVERYWHERE_BASELINE_Q16 == ONE / 2);
/// The trials of an arm, H-14's: three of the instrument's runs, twenty-four blocks.
const EVERYWHERE_TRIALS: usize = REINFORCED_TRIALS;
const _: () = assert!(EVERYWHERE_TRIALS == 1_536 && EVERYWHERE_TRIALS / BLOCK == 24);

/// The arms of H-15 (ADR-0082), in the order run and no other, every one `Delivery::Addressed`
/// from the one image at 0.5: the assignment (`Feedback::Answer`, A's answer readout 0 and B's
/// readout 1), the mirrored assignment (`Feedback::Answer`, `mirrored`), and the reward
/// withheld (`Feedback::Withheld`: no reward, the signal at rest, every synapse consolidating
/// under the baseline alone), a reading bounded by no clause. The shuffled reward is not run.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Everywhere {
    Assignment,
    Mirrored,
    Withheld,
}
const EVERYWHERE_ARMS: [Everywhere; 3] = [
    Everywhere::Assignment,
    Everywhere::Mirrored,
    Everywhere::Withheld,
];
/// The rewarded arms in `EVERYWHERE_ARMS`'s order, the criterion's `[assignment, mirrored]`.
const EVERYWHERE_REWARDED: [Everywhere; 2] = [Everywhere::Assignment, Everywhere::Mirrored];
/// The withheld arm's index in `EVERYWHERE_ARMS`.
const WITHHELD: usize = 2;

/// Where an arm's reward takes its sign from: the answer, or nothing.
fn everywhere_feedback(arm: Everywhere) -> Feedback {
    if arm == Everywhere::Withheld {
        Feedback::Withheld
    } else {
        Feedback::Answer
    }
}

/// Whether an arm mirrors the assignment.
fn everywhere_mirrors(arm: Everywhere) -> bool {
    arm == Everywhere::Mirrored
}

/// True for an arm whose reward carries the answer, the criterion's two.
fn everywhere_rewards(arm: Everywhere) -> bool {
    arm != Everywhere::Withheld
}

// ---------------------------------------------------- the assertion's shape (ADR-0082)

/// The signal's floor at a trial's end, the fixed point of a punishment every trial:
/// `decay_dopamine` reads the magnitude only, so the two fixed points are symmetric and the
/// floor is the negative of ADR-0079's `SIGNAL_END_FIXED_Q16` (0.712).
const SIGNAL_END_FLOOR_Q16: i32 = -SIGNAL_END_FIXED_Q16;
/// The modulation the addressed pair — a wrong selection's — meets at the first tick of the
/// trial after a punishment, at most: the baseline plus the signal after a reward of −1.0
/// from the highest end, 0.5 + 0.712 − 1.0 (`Modulations::of`: the baseline plus the signal,
/// saturating, clamped to $[0, 1]$); 13 923 in Q16.16, 0.2125, ADR-0082's 0.212 to three
/// decimals.
const WRONG_AT_MOST_Q16: i32 = EVERYWHERE_BASELINE_Q16 + SIGNAL_END_FIXED_Q16 - ONE;
/// The modulation the addressed pair — the answer's — meets at the first tick of the trial
/// after a reward, at least: 0.5 − 0.712 + 1.0; 51 613, 0.7875, ADR-0082's 0.788 to three
/// decimals.
const ANSWER_AT_LEAST_Q16: i32 = EVERYWHERE_BASELINE_Q16 - SIGNAL_END_FIXED_Q16 + ONE;
const _: () = assert!(WRONG_AT_MOST_Q16 == 13_923 && ANSWER_AT_LEAST_Q16 == 51_613);
const _: () = assert!(
    WRONG_AT_MOST_Q16 < EVERYWHERE_BASELINE_Q16 && EVERYWHERE_BASELINE_Q16 < ANSWER_AT_LEAST_Q16
);
/// ADR-0082's decimals are the bounds rounded to the nearest thousandth.
const _: () = assert!(
    (WRONG_AT_MOST_Q16 as i64 * 2000 + ONE as i64) / (2 * ONE as i64) == 212
        && (ANSWER_AT_LEAST_Q16 as i64 * 2000 + ONE as i64) / (2 * ONE as i64) == 788
);

/// ADR-0082's assertion as a rule over an arm's trials, clause by clause: the signal at
/// every trial's end between the fixed points (`SIGNAL_END_FLOOR_Q16` and
/// `SIGNAL_END_FIXED_Q16`); after every negative reward the modulation the addressed pair —
/// the wrong one — meets at the next trial's first tick, the baseline plus the signal after
/// the reward, at most `WRONG_AT_MOST_Q16`; after every positive reward the modulation the
/// answer's pair meets there at least `ANSWER_AT_LEAST_Q16`. The oracle holds the record's
/// weights to the consolidation under the baseline plus the signal's course at every spike of
/// every trial (`Composer::observe`), so the modulation the rule reads is the one the record
/// consolidated under. Every clause true on both rewarded arms is the assertion; a false one
/// is a finding against ADR-0082's derivation, reported beside the verdict and not in place
/// of it.
fn bounds(read: &[EarnedTrial]) -> [bool; 3] {
    let within = read
        .iter()
        .all(|t| SIGNAL_END_FLOOR_Q16 <= t.6 && t.6 <= SIGNAL_END_FIXED_Q16);
    let wrong = read
        .iter()
        .filter(|t| t.4 < 0)
        .all(|t| t.7.saturating_add(EVERYWHERE_BASELINE_Q16) <= WRONG_AT_MOST_Q16);
    let answer = read
        .iter()
        .filter(|t| t.4 > 0)
        .all(|t| t.7.saturating_add(EVERYWHERE_BASELINE_Q16) >= ANSWER_AT_LEAST_Q16);
    [within, wrong, answer]
}

// ------------------------------------------ the predicted readings' shape (ADR-0082)

/// The withheld arm's drift, per stimulus: whether each coupling ended above the image's
/// (`[stimulus][readout]`); whether the coupling onto readout 1 rose faster than the one onto
/// readout 0, in integers, `end[s][1] · image[s][0] > end[s][0] · image[s][1]`; and whether
/// the stimulus selected readout 1 more often than readout 0 over the last 128 trials.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Drift {
    rose: [[bool; 2]; 2],
    faster_onto_r1: [bool; 2],
    toward_r1: [bool; 2],
}

/// The drift read from the withheld arm: `image` the image's couplings, `end` the couplings
/// after the last block, `last` the selections per stimulus over the last 128 trials.
fn drift(image: &[[i64; 2]; 2], end: &[[i64; 2]; 2], last: [[u32; 3]; 2]) -> Drift {
    let mut d = Drift {
        rose: [[false; 2]; 2],
        faster_onto_r1: [false; 2],
        toward_r1: [false; 2],
    };
    for (s, rose) in d.rose.iter_mut().enumerate() {
        for (r, into) in rose.iter_mut().enumerate() {
            *into = end[s][r] > image[s][r];
        }
        d.faster_onto_r1[s] =
            end[s][1].saturating_mul(image[s][0]) > end[s][0].saturating_mul(image[s][1]);
        d.toward_r1[s] = last[s][1] > last[s][0];
    }
    d
}

/// ADR-0082's predicted reading (a), a Hypothesis written before the run and never asserted:
/// in the withheld arm every coupling rises, the two onto readout 1 rise faster, and the
/// selection drifts toward readout 1 for both stimuli (from ADR-0077's terms, +0.4, +1.6,
/// +0.8 and +1.7 per synapse per trial on A→R0, A→R1, B→R0 and B→R1, and H-14's shuffled
/// arm).
const DRIFT_PREDICTED: Drift = Drift {
    rose: [[true; 2]; 2],
    faster_onto_r1: [true; 2],
    toward_r1: [true; 2],
};

/// Where each stimulus's selection first reaches its answer at the crossing's rate
/// (`CROSSING_MARK`, 40 of 64): the trial at the end of the first block in which the
/// stimulus's correct selections are at least 40 of 64 of its presentations — in integers,
/// `correct × 64 ≥ 40 × presented`, over a block that presented it at all; none when no
/// block has it. `[stimulus A, stimulus B]`.
fn reached(blocks: &[Block], earned: &[EarnedBlock], mirrored: bool) -> [Option<usize>; 2] {
    let mut out = [None; 2];
    for (s, into) in out.iter_mut().enumerate() {
        *into = blocks
            .iter()
            .zip(earned.iter())
            .position(|(b, e)| {
                let presented = if s == 0 {
                    b.1
                } else {
                    (BLOCK as u32).saturating_sub(b.1)
                };
                let correct = e.0[s][answer_of(s as u8, mirrored)];
                presented > 0
                    && correct.saturating_mul(BLOCK as u32)
                        >= CROSSING_MARK.saturating_mul(presented)
            })
            .map(|k| k.saturating_add(1).saturating_mul(BLOCK));
    }
    out
}

/// The stimulus whose answer is readout 0 under an assignment: A when not mirrored, B when.
fn r0_stimulus(mirrored: bool) -> usize {
    usize::from(mirrored)
}

/// True when the stimulus whose answer is readout 0 reached its answer later than the other
/// or not at all.
fn r0_later(reached: [Option<usize>; 2], mirrored: bool) -> bool {
    let s = r0_stimulus(mirrored);
    match (reached[s], reached[s ^ 1]) {
        (None, _) => true,
        (Some(_), None) => false,
        (Some(mine), Some(other)) => mine > other,
    }
}

/// ADR-0082's predicted reading (b), a Hypothesis written before the run and never asserted:
/// in each rewarded arm, `[assignment, mirrored]`, the stimulus whose answer is readout 0 —
/// A in the assignment, B in the mirrored — reaches its answer later than the other or not
/// at all, and is where the verdict is decided.
const R0_LATER_PREDICTED: [bool; 2] = [true, true];

// ---------------------------------------------------------------- the run (brief 038)

/// The settled engine decoded from its image at 0.5: under the calibration's configuration
/// with the baseline zero, so that the image's baseline is shown to outrank the
/// configuration's (§8.3); the baseline asserted 0.5.
fn settled_from(image: &[u8], units: u32) -> Engine {
    let exec = Image::decode::<2048>(image, config(units, 2, 0)).expect("a well-formed record");
    assert_eq!(
        exec.modulation_baseline_q16(),
        EVERYWHERE_BASELINE_Q16,
        "the image carries the baseline 0.5"
    );
    exec
}

/// The two images of the one settled engine: ADR-0077's frozen image, the baseline patched to
/// zero, for the calibration, held as `settled_image` holds it; and the image as encoded, the
/// baseline 0.5 the lead-in settled under, for the arms — decoded and asserted to carry the
/// baseline, the sums, the gain and the step, and to resume the clock where the image was
/// written. The two differ in the modulator section's baseline word and that section's CRC
/// and nowhere else.
fn settled_images(name: &str) -> (Vec<u8>, Vec<u8>) {
    let (exec, quieted) = settled_engine(name);
    let zero = frozen_image_checked(name, &exec, quieted);
    let half = Image::encode(&exec).expect("quiescent");
    let decoded = settled_from(&half, 1024);
    assert_eq!(
        weights_by_polarity(&decoded),
        quieted,
        "{name}: the 0.5 image carries the weights"
    );
    assert_eq!(
        decoded.homeostasis().synaptic_gain_q16,
        GAIN_1024,
        "{name}: and the gain"
    );
    assert_eq!(
        decoded.homeostasis().control_step_q0_16,
        BACKGROUNDS[SETTLED],
        "{name}: and the step"
    );
    assert_eq!(
        decoded.ticks(),
        exec.ticks(),
        "{name}: the clock resumes where the image was written"
    );
    assert_eq!(zero.len(), half.len(), "{name}: one image, two baselines");
    let differing = zero.iter().zip(half.iter()).filter(|(a, b)| a != b).count();
    assert!(
        differing > 0 && differing <= 12,
        "{name}: the images differ in the baseline word and the section's CRC: {differing} bytes"
    );
    eprintln!(
        "DUMP {name} images {} bytes, {differing} differ, written at tick {}",
        half.len(),
        exec.ticks()
    );
    (zero, half)
}

/// An arm's run from the settled engine at 0.5: `earned_run_under` with the arm's feedback and
/// assignment at `EVERYWHERE_BASELINE_Q16`, the oracle consolidating every stimulus–readout
/// synapse under the baseline plus the signal where the synapse is addressed and under the
/// baseline alone elsewhere, held to the record at every trial; under the withheld arm the
/// reward is zero and the signal stays at rest, asserted trial by trial.
fn everywhere_run(exec: &mut Engine, arm: Everywhere, trials: usize) -> EarnedRun {
    earned_run_under(
        exec,
        everywhere_feedback(arm),
        everywhere_mirrors(arm),
        1024,
        trials,
        EVERYWHERE_BASELINE_Q16,
    )
}

/// Holds an arm's run to its pinned tables: the blocks and the trace, the composition per
/// block, the earned blocks, the readings' hash and the census.
fn pinned_everywhere(name: &str, k: usize, run: &EarnedRun, earned: &[EarnedBlock]) {
    let (blocks, trace, trials, read, volley_ticks) = run;
    pinned(
        &format!("{name} sight"),
        blocks,
        *trace,
        EVERYWHERE_BLOCKS_1024[k],
        EVERYWHERE_TRACES_1024[k],
    );
    let compositions: Vec<Composition> = trials.chunks(BLOCK).map(composition).collect();
    assert_eq!(
        compositions.as_slice(),
        EVERYWHERE_COMPOSITIONS_1024[k],
        "{name}: the composition per block"
    );
    assert_eq!(
        earned, EVERYWHERE_EARNED_1024[k],
        "{name}: the earned blocks"
    );
    assert_eq!(
        earned_hash(read),
        EVERYWHERE_READ_1024[k],
        "{name}: the readings"
    );
    assert_eq!(
        census_of(volley_ticks),
        EVERYWHERE_CENSUS_1024[k].to_vec(),
        "{name}: the volley's ticks"
    );
}

/// H-15 at 1 024 units (brief 038): the settled engine, held to ADR-0077 step by step, and its
/// two images; the calibration — a frozen block from the zero image, the reward withheld,
/// held to ADR-0077's frozen run — before any rewarded run (H-15's stopping rule, step 2);
/// then the assignment, the mirrored assignment and the reward withheld, each
/// `EVERYWHERE_TRIALS` trials from the 0.5 image under the task's own delivery, each dumped and
/// the verdict and every reading computed by the rules before anything is held; then the
/// assertion — ADR-0082's bounds true on both rewarded arms, and every arm's delivery having
/// moved synapses inside the four pairs and outside them, since every synapse consolidates —
/// and the pinned tables.
#[test]
#[ignore]
fn plasticity_everywhere_at_1024_units_exhaustive() {
    let name = "everywhere1024";
    let (zero, half) = settled_images(name);
    {
        let mut frozen = frozen_from(&zero, 1024);
        let calibration = taught_run(&mut frozen, Arm::Withheld, 1024, BLOCK);
        calibration_holds(&format!("{name} calibration"), &calibration);
        eprintln!("DUMP {name} calibration holds: ADR-0077's settled candidate reproduced");
    }
    let sets = geometry(1024, ROTATION_1024);
    let mut runs: Vec<EarnedRun> = Vec::with_capacity(EVERYWHERE_ARMS.len());
    let mut reaches = [(0u64, 0u64); 3];
    for (k, &arm) in EVERYWHERE_ARMS.iter().enumerate() {
        let arm_name = format!("{name} {arm:?}");
        let mut exec = settled_from(&half, 1024);
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
        let run = everywhere_run(&mut exec, arm, EVERYWHERE_TRIALS);
        let (inside, outside) = reach(&exec, &before, 1024, &ALL_PAIRS);
        eprintln!(
            "DUMP {arm_name} reach inside {inside} outside {outside} sums after the run {:?} signal {:#x}",
            weights_by_polarity(&exec),
            exec.modulator().dopamine_rpe
        );
        reaches[k] = (inside, outside);
        runs.push(run);
    }
    // Every arm dumped and the verdict and the readings computed before anything is held.
    let mut tables: Vec<Vec<EarnedBlock>> = Vec::with_capacity(EVERYWHERE_ARMS.len());
    for (k, &arm) in EVERYWHERE_ARMS.iter().enumerate() {
        let earned = earned_blocks(&runs[k].3);
        dump_earned(&format!("{name} {arm:?}"), &runs[k], &earned);
        tables.push(earned);
    }
    let verdict = reinforced([&runs[0].0, &runs[1].0]);
    let correct_last = [last_correct(&runs[0].0), last_correct(&runs[1].0)];
    let last = [
        last_splits(&runs[0].3),
        last_splits(&runs[1].3),
        last_splits(&runs[2].3),
    ];
    let crossed = [
        crossing(&runs[0].0),
        crossing(&runs[1].0),
        crossing(&runs[2].0),
    ];
    let bounds_read = [bounds(&runs[0].3), bounds(&runs[1].3), bounds(&runs[2].3)];
    let end_withheld = runs[WITHHELD]
        .0
        .last()
        .map(|b| b.10)
        .expect("the withheld arm has a block");
    let drift_read = drift(&IMAGE_COUPLINGS_1024, &end_withheld, last[WITHHELD]);
    let reached_read = [
        reached(&runs[0].0, &tables[0], false),
        reached(&runs[1].0, &tables[1], true),
    ];
    let r0_later_read = [
        r0_later(reached_read[0], false),
        r0_later(reached_read[1], true),
    ];
    eprintln!(
        "DUMP {name} verdict {verdict:?} correct last {correct_last:?} crossed {crossed:?} splits last {last:?} bounds {bounds_read:?} drift {drift_read:?} predicted {DRIFT_PREDICTED:?} reached {reached_read:?} r0 later {r0_later_read:?} predicted {R0_LATER_PREDICTED:?} reach {reaches:?}"
    );
    // The assertion (ADR-0082's bounds), after the verdict and beside it; and the reach of a
    // baseline under which every synapse consolidates.
    for (k, &arm) in EVERYWHERE_ARMS.iter().enumerate() {
        if everywhere_rewards(arm) {
            assert_eq!(bounds_read[k], [true; 3], "{arm:?}: ADR-0082's assertion");
        }
        assert!(
            reaches[k].0 > 0 && reaches[k].1 > 0,
            "{arm:?}: every synapse consolidates, inside the four pairs and outside them"
        );
    }
    for (k, &arm) in EVERYWHERE_ARMS.iter().enumerate() {
        pinned_everywhere(&format!("{name} {arm:?}"), k, &runs[k], &tables[k]);
    }
    assert_eq!(verdict, EVERYWHERE_1024, "the verdict as written");
    assert_eq!(correct_last, CORRECT_LAST_EVERYWHERE_1024);
    assert_eq!(crossed, CROSSED_EVERYWHERE_1024);
    assert_eq!(last, SPLITS_LAST_EVERYWHERE_1024);
    assert_eq!(bounds_read, BOUNDS_1024);
    assert_eq!(drift_read, DRIFT_1024);
    assert_eq!(reached_read, REACHED_1024);
    assert_eq!(r0_later_read, R0_LATER_1024);
    assert_eq!(reaches, EVERYWHERE_REACH_1024);
}

// ----------------------------------------------------------- the measurement (brief 038)

/// The three arms at 1 024 units, in `EVERYWHERE_ARMS`'s order, each pinned from one run: the
/// sight's blocks and trace, the composition per block, the earned blocks, the readings' hash
/// and the volley's census.
const EVERYWHERE_BLOCKS_1024: [&[Block]; 3] = [
    &[
        (
            31,
            34,
            [[336, 328], [317, 320]],
            [1728, 1525],
            [2473, 2249],
            [258, 253],
            62,
            161643622,
            216931996,
            41361,
            [[6280714, 6744904], [6602976, 6892503]],
            6,
        ),
        (
            32,
            31,
            [[332, 340], [363, 382]],
            [1573, 1677],
            [2360, 2364],
            [243, 228],
            64,
            157246031,
            215973948,
            -20945,
            [[6337882, 6840476], [6674573, 6967618]],
            6,
        ),
        (
            27,
            31,
            [[349, 366], [390, 404]],
            [1573, 1680],
            [2329, 2335],
            [254, 226],
            64,
            152744034,
            215070539,
            -93985,
            [[6424055, 6921565], [6725530, 7078468]],
            4,
        ),
        (
            32,
            28,
            [[345, 399], [453, 517]],
            [1427, 1829],
            [2186, 2505],
            [253, 259],
            64,
            148020804,
            214123601,
            101885,
            [[6483990, 6978852], [6829294, 7211492]],
            4,
        ),
        (
            33,
            30,
            [[401, 444], [436, 538]],
            [1525, 1725],
            [2282, 2333],
            [253, 253],
            64,
            143338153,
            213426889,
            93851,
            [[6606371, 7094054], [6923209, 7457259]],
            7,
        ),
        (
            37,
            36,
            [[553, 514], [417, 485]],
            [1831, 1427],
            [2553, 2041],
            [276, 242],
            64,
            138488504,
            212929507,
            -85160,
            [[6806540, 7229611], [6998861, 7589357]],
            5,
        ),
        (
            38,
            30,
            [[493, 503], [540, 603]],
            [1521, 1729],
            [2212, 2346],
            [275, 269],
            64,
            133659366,
            212433864,
            46741,
            [[6938587, 7327047], [7095776, 7719605]],
            4,
        ),
        (
            39,
            32,
            [[604, 592], [502, 613]],
            [1629, 1628],
            [2311, 2263],
            [271, 256],
            64,
            128646090,
            212102746,
            -21487,
            [[7086149, 7461565], [7182540, 7808263]],
            6,
        ),
        (
            36,
            34,
            [[697, 659], [585, 640]],
            [1729, 1526],
            [2444, 2128],
            [268, 240],
            64,
            123609020,
            212001627,
            41898,
            [[7271829, 7662403], [7297159, 7963112]],
            5,
        ),
        (
            35,
            28,
            [[567, 622], [655, 803]],
            [1424, 1832],
            [2162, 2444],
            [278, 234],
            64,
            118590466,
            211772715,
            97105,
            [[7335820, 7795699], [7429591, 8131829]],
            6,
        ),
        (
            41,
            33,
            [[792, 831], [620, 774]],
            [1675, 1579],
            [2328, 2190],
            [256, 265],
            64,
            113692418,
            211564647,
            58745,
            [[7415313, 7970165], [7512925, 8296572]],
            4,
        ),
        (
            30,
            32,
            [[743, 823], [745, 832]],
            [1626, 1629],
            [2324, 2259],
            [294, 284],
            64,
            108749460,
            211202858,
            -101795,
            [[7526911, 8118488], [7613515, 8458012]],
            7,
        ),
        (
            36,
            32,
            [[801, 884], [735, 919]],
            [1632, 1627],
            [2275, 2226],
            [272, 230],
            64,
            104018282,
            211026666,
            108537,
            [[7627895, 8254194], [7699451, 8646820]],
            1,
        ),
        (
            41,
            33,
            [[971, 964], [738, 928]],
            [1677, 1579],
            [2358, 2221],
            [295, 278],
            64,
            99492219,
            210745539,
            82949,
            [[7790485, 8426253], [7775219, 8817304]],
            6,
        ),
        (
            32,
            37,
            [[1121, 1168], [775, 922]],
            [1883, 1375],
            [2520, 2009],
            [283, 309],
            64,
            95135042,
            210439627,
            -98216,
            [[7975943, 8584693], [7833198, 8941853]],
            6,
        ),
        (
            40,
            35,
            [[1151, 1205], [862, 1021]],
            [1781, 1473],
            [2410, 2124],
            [290, 304],
            64,
            91215687,
            209979301,
            -29717,
            [[8146456, 8747504], [7957851, 9063417]],
            3,
        ),
        (
            37,
            33,
            [[1187, 1226], [917, 1089]],
            [1679, 1572],
            [2343, 2116],
            [288, 279],
            64,
            87392819,
            209710067,
            59377,
            [[8316658, 8905955], [8057339, 9190196]],
            7,
        ),
        (
            34,
            38,
            [[1347, 1396], [860, 983]],
            [1933, 1323],
            [2565, 1903],
            [324, 301],
            64,
            83832826,
            209366740,
            89800,
            [[8503728, 9076767], [8126698, 9263562]],
            4,
        ),
        (
            34,
            31,
            [[1147, 1260], [1075, 1237]],
            [1579, 1681],
            [2222, 2276],
            [285, 317],
            64,
            80513859,
            209253923,
            89639,
            [[8595828, 9228921], [8277575, 9405140]],
            3,
        ),
        (
            37,
            34,
            [[1358, 1433], [1033, 1224]],
            [1728, 1527],
            [2292, 2158],
            [331, 312],
            64,
            77558140,
            208938061,
            -21675,
            [[8722480, 9347412], [8357616, 9541278]],
            3,
        ),
        (
            32,
            30,
            [[1192, 1318], [1199, 1415]],
            [1524, 1730],
            [2116, 2299],
            [311, 285],
            64,
            74747482,
            209099505,
            24277,
            [[8823949, 9439195], [8533344, 9702831]],
            3,
        ),
        (
            40,
            29,
            [[1204, 1325], [1336, 1629]],
            [1473, 1781],
            [2085, 2349],
            [321, 283],
            64,
            72243193,
            209109087,
            -18953,
            [[8927435, 9564355], [8658701, 9952371]],
            3,
        ),
        (
            38,
            32,
            [[1389, 1522], [1285, 1557]],
            [1630, 1630],
            [2250, 2258],
            [371, 317],
            64,
            70109418,
            208871232,
            -93348,
            [[9030844, 9684859], [8770465, 10079289]],
            1,
        ),
        (
            40,
            29,
            [[1328, 1471], [1506, 1751]],
            [1471, 1779],
            [2069, 2333],
            [330, 293],
            64,
            68197381,
            208801743,
            112117,
            [[9136672, 9786768], [8875690, 10212976]],
            1,
        ),
    ],
    &[
        (
            29,
            34,
            [[333, 334], [318, 321]],
            [1728, 1525],
            [2473, 2249],
            [259, 253],
            62,
            161633939,
            216983778,
            -41361,
            [[6273865, 6790022], [6629631, 6874067]],
            5,
        ),
        (
            27,
            31,
            [[328, 350], [364, 368]],
            [1574, 1677],
            [2356, 2364],
            [241, 230],
            64,
            157214904,
            216051340,
            -89071,
            [[6313866, 6916675], [6727007, 6913858]],
            9,
        ),
        (
            35,
            31,
            [[338, 376], [393, 395]],
            [1573, 1680],
            [2326, 2332],
            [255, 225],
            64,
            152691930,
            215173851,
            93985,
            [[6361921, 7046634], [6808755, 7002504]],
            2,
        ),
        (
            33,
            28,
            [[324, 431], [456, 496]],
            [1427, 1829],
            [2187, 2507],
            [252, 261],
            64,
            147928191,
            214255194,
            -103213,
            [[6408063, 7150258], [6951931, 7071014]],
            4,
        ),
        (
            31,
            30,
            [[390, 487], [457, 489]],
            [1525, 1725],
            [2279, 2329],
            [257, 255],
            64,
            143221711,
            213475118,
            -93859,
            [[6498549, 7342624], [7057929, 7196495]],
            3,
        ),
        (
            37,
            36,
            [[518, 589], [437, 435]],
            [1831, 1427],
            [2545, 2038],
            [271, 241],
            64,
            138318066,
            213025955,
            89124,
            [[6664076, 7553484], [7185127, 7301696]],
            1,
        ),
        (
            38,
            30,
            [[465, 576], [575, 535]],
            [1521, 1729],
            [2212, 2348],
            [269, 269],
            64,
            133419360,
            212588770,
            92539,
            [[6765491, 7708501], [7304299, 7439970]],
            6,
        ),
        (
            42,
            32,
            [[570, 677], [540, 554]],
            [1629, 1628],
            [2314, 2262],
            [272, 257],
            64,
            128325351,
            212271589,
            94721,
            [[6893860, 7915651], [7430081, 7529561]],
            2,
        ),
        (
            44,
            34,
            [[646, 768], [620, 570]],
            [1729, 1526],
            [2444, 2127],
            [267, 242],
            64,
            123247044,
            212246333,
            89202,
            [[7080680, 8214761], [7574557, 7661569]],
            2,
        ),
        (
            42,
            28,
            [[532, 724], [698, 723]],
            [1424, 1832],
            [2158, 2441],
            [274, 234],
            64,
            118222515,
            211960171,
            34376,
            [[7148715, 8405825], [7713871, 7775472]],
            3,
        ),
        (
            38,
            33,
            [[752, 946], [649, 689]],
            [1675, 1579],
            [2325, 2193],
            [256, 260],
            64,
            113266359,
            211696235,
            -21151,
            [[7226716, 8596068], [7810904, 7872032]],
            5,
        ),
        (
            43,
            32,
            [[695, 930], [762, 756]],
            [1626, 1629],
            [2323, 2265],
            [295, 289],
            64,
            108326651,
            211301525,
            102438,
            [[7319632, 8753255], [7892092, 7996760]],
            4,
        ),
        (
            42,
            32,
            [[754, 1035], [775, 806]],
            [1632, 1627],
            [2272, 2227],
            [265, 233],
            64,
            103568322,
            211107052,
            -108531,
            [[7400409, 8939244], [7999594, 8117444]],
            5,
        ),
        (
            41,
            33,
            [[880, 1130], [762, 793]],
            [1677, 1579],
            [2361, 2220],
            [288, 279],
            64,
            98985282,
            210719326,
            -20049,
            [[7523413, 9131978], [8087896, 8238666]],
            2,
        ),
        (
            44,
            37,
            [[1022, 1352], [789, 795]],
            [1883, 1375],
            [2520, 2007],
            [277, 294],
            64,
            94621556,
            210465126,
            104728,
            [[7707244, 9337636], [8188070, 8297227]],
            4,
        ),
        (
            47,
            35,
            [[1072, 1398], [894, 875]],
            [1781, 1474],
            [2402, 2130],
            [282, 303],
            64,
            90671419,
            210131219,
            63061,
            [[7861820, 9554422], [8321953, 8380336]],
            2,
        ),
        (
            47,
            33,
            [[1083, 1405], [949, 947]],
            [1679, 1572],
            [2345, 2116],
            [295, 278],
            64,
            86919505,
            209998227,
            -58221,
            [[8038735, 9727887], [8398695, 8486525]],
            2,
        ),
        (
            52,
            38,
            [[1296, 1657], [894, 850]],
            [1933, 1323],
            [2560, 1910],
            [322, 306],
            64,
            83472562,
            209732009,
            -74377,
            [[8270096, 9940199], [8481763, 8551023]],
            4,
        ),
        (
            50,
            31,
            [[1133, 1476], [1144, 1082]],
            [1579, 1681],
            [2223, 2284],
            [290, 311],
            64,
            80276936,
            209693462,
            101457,
            [[8392572, 10130061], [8661742, 8660293]],
            2,
        ),
        (
            47,
            34,
            [[1330, 1739], [1105, 1084]],
            [1728, 1527],
            [2283, 2153],
            [339, 300],
            64,
            77358025,
            209485965,
            103089,
            [[8530742, 10326156], [8771755, 8768709]],
            2,
        ),
        (
            47,
            30,
            [[1217, 1597], [1312, 1290]],
            [1525, 1730],
            [2108, 2300],
            [311, 283],
            64,
            74528492,
            209713901,
            107553,
            [[8666968, 10518459], [8974869, 8926297]],
            2,
        ),
        (
            47,
            29,
            [[1203, 1617], [1448, 1439]],
            [1473, 1781],
            [2081, 2354],
            [324, 273],
            64,
            72075792,
            209593598,
            102743,
            [[8780001, 10682351], [9117802, 9136948]],
            3,
        ),
        (
            49,
            32,
            [[1397, 1820], [1413, 1365]],
            [1630, 1630],
            [2256, 2258],
            [371, 321],
            64,
            69985623,
            209384628,
            94747,
            [[8890274, 10803181], [9222875, 9276762]],
            2,
        ),
        (
            57,
            29,
            [[1321, 1718], [1628, 1537]],
            [1471, 1778],
            [2066, 2346],
            [336, 300],
            64,
            68086606,
            209276730,
            82894,
            [[8990118, 10932676], [9309336, 9410658]],
            0,
        ),
    ],
    &[
        (
            31,
            34,
            [[336, 330], [318, 320]],
            [1728, 1525],
            [2472, 2249],
            [258, 253],
            62,
            161643050,
            216959731,
            0,
            [[6278705, 6768486], [6619221, 6885530]],
            5,
        ),
        (
            28,
            31,
            [[328, 344], [363, 374]],
            [1574, 1677],
            [2358, 2364],
            [242, 229],
            64,
            157235820,
            216012736,
            0,
            [[6332530, 6882078], [6704545, 6945215]],
            10,
        ),
        (
            28,
            31,
            [[344, 373], [390, 402]],
            [1573, 1680],
            [2327, 2335],
            [255, 225],
            64,
            152724727,
            215132032,
            0,
            [[6396625, 6989737], [6770455, 7052693]],
            3,
        ),
        (
            29,
            28,
            [[337, 416], [455, 514]],
            [1427, 1829],
            [2192, 2507],
            [254, 258],
            64,
            147983147,
            214220986,
            0,
            [[6447175, 7071472], [6898561, 7159224]],
            6,
        ),
        (
            33,
            30,
            [[393, 470], [449, 519]],
            [1525, 1725],
            [2285, 2331],
            [254, 256],
            64,
            143304942,
            213454842,
            0,
            [[6553746, 7223566], [6992089, 7346719]],
            3,
        ),
        (
            31,
            36,
            [[534, 549], [431, 458]],
            [1831, 1427],
            [2549, 2041],
            [275, 244],
            64,
            138430941,
            212986571,
            0,
            [[6737951, 7391232], [7094575, 7461926]],
            5,
        ),
        (
            29,
            30,
            [[474, 554], [565, 592]],
            [1521, 1729],
            [2213, 2346],
            [271, 271],
            64,
            133533271,
            212532306,
            0,
            [[6839371, 7529403], [7196595, 7603903]],
            7,
        ),
        (
            26,
            32,
            [[577, 651], [527, 598]],
            [1629, 1628],
            [2309, 2262],
            [268, 260],
            64,
            128480142,
            212224935,
            0,
            [[6977537, 7696998], [7304517, 7687433]],
            6,
        ),
        (
            25,
            34,
            [[669, 722], [605, 605]],
            [1729, 1526],
            [2444, 2129],
            [270, 248],
            64,
            123420317,
            212145470,
            0,
            [[7159802, 7940591], [7443375, 7798452]],
            3,
        ),
        (
            25,
            28,
            [[547, 688], [682, 772]],
            [1424, 1832],
            [2161, 2443],
            [273, 237],
            64,
            118370820,
            211941343,
            0,
            [[7213337, 8148287], [7580184, 7935178]],
            4,
        ),
        (
            34,
            33,
            [[763, 887], [639, 758]],
            [1675, 1579],
            [2329, 2197],
            [254, 265],
            64,
            113399521,
            211767241,
            0,
            [[7295979, 8327260], [7670505, 8095522]],
            5,
        ),
        (
            19,
            32,
            [[706, 882], [756, 804]],
            [1626, 1629],
            [2327, 2266],
            [291, 288],
            64,
            108409269,
            211347535,
            0,
            [[7395825, 8486252], [7753744, 8248672]],
            7,
        ),
        (
            26,
            32,
            [[766, 961], [769, 884]],
            [1632, 1627],
            [2274, 2223],
            [273, 239],
            64,
            103653912,
            211079857,
            0,
            [[7476087, 8624739], [7856251, 8385011]],
            9,
        ),
        (
            29,
            33,
            [[903, 1046], [763, 858]],
            [1677, 1579],
            [2361, 2219],
            [284, 275],
            64,
            99088397,
            210735099,
            0,
            [[7581397, 8764071], [7949696, 8535406]],
            3,
        ),
        (
            24,
            37,
            [[1051, 1262], [783, 860]],
            [1883, 1375],
            [2514, 2013],
            [280, 304],
            64,
            94732376,
            210436317,
            0,
            [[7775055, 8942092], [8014977, 8619210]],
            2,
        ),
        (
            26,
            35,
            [[1088, 1301], [885, 951]],
            [1781, 1473],
            [2408, 2125],
            [284, 303],
            64,
            90829939,
            210109241,
            0,
            [[7946572, 9176043], [8138072, 8746202]],
            2,
        ),
        (
            24,
            33,
            [[1123, 1311], [930, 1027]],
            [1679, 1572],
            [2342, 2115],
            [287, 276],
            64,
            87026080,
            209864605,
            0,
            [[8129883, 9321924], [8218913, 8844208]],
            6,
        ),
        (
            19,
            38,
            [[1298, 1552], [864, 909]],
            [1933, 1323],
            [2563, 1905],
            [326, 305],
            64,
            83575578,
            209546255,
            0,
            [[8314439, 9506763], [8269567, 8899074]],
            7,
        ),
        (
            17,
            31,
            [[1111, 1378], [1092, 1161]],
            [1579, 1681],
            [2221, 2284],
            [290, 313],
            64,
            80358926,
            209419539,
            0,
            [[8435506, 9685626], [8381412, 9038122]],
            3,
        ),
        (
            23,
            34,
            [[1321, 1590], [1034, 1150]],
            [1728, 1527],
            [2293, 2157],
            [335, 306],
            64,
            77489304,
            209079510,
            0,
            [[8553262, 9820014], [8453184, 9196986]],
            2,
        ),
        (
            26,
            30,
            [[1186, 1473], [1185, 1327]],
            [1525, 1730],
            [2106, 2305],
            [304, 285],
            64,
            74606958,
            209211155,
            0,
            [[8675799, 9980623], [8612085, 9336884]],
            0,
        ),
        (
            27,
            29,
            [[1183, 1474], [1331, 1483]],
            [1473, 1781],
            [2090, 2353],
            [327, 272],
            64,
            72038890,
            209082951,
            0,
            [[8775430, 10135085], [8735761, 9509600]],
            2,
        ),
        (
            23,
            32,
            [[1387, 1725], [1281, 1405]],
            [1630, 1630],
            [2250, 2261],
            [371, 316],
            64,
            69872749,
            208867306,
            0,
            [[8902133, 10268735], [8841290, 9618213]],
            2,
        ),
        (
            24,
            29,
            [[1315, 1589], [1534, 1610]],
            [1471, 1778],
            [2064, 2342],
            [323, 284],
            64,
            67918377,
            208718619,
            0,
            [[9017306, 10358182], [8969340, 9767116]],
            5,
        ),
    ],
];
const EVERYWHERE_TRACES_1024: [u64; 3] =
    [0x4ea242ce12c94d61, 0x2277e7a62fb088af, 0xac188c15cb76a9a3];
const EVERYWHERE_COMPOSITIONS_1024: [&[Composition]; 3] = [
    &[
        (
            [[2176, 7941], [5919, 778]],
            [
                [
                    [224538, -4914, 320745, -501794],
                    [239807, -5113, 348072, -489571],
                ],
                [
                    [227060, -5534, 291909, -468747],
                    [208909, -5367, 333896, -451694],
                ],
            ],
            [[1093, 1656], [1015, 1656], [3350, 0]],
            51,
            17196297825170112305,
        ),
        (
            [[1012, 1589], [-883, -1523]],
            [
                [
                    [218062, -5841, 328219, -478061],
                    [252943, -5748, 346007, -454551],
                ],
                [
                    [265712, -6539, 305401, -455239],
                    [249814, -5345, 322155, -481200],
                ],
            ],
            [[993, 1661], [1017, 1703], [3349, 0]],
            55,
            1212051118818355656,
        ),
        (
            [[-1681, 2399], [5693, 5231]],
            [
                [
                    [238617, -5634, 333919, -472157],
                    [250533, -5793, 370472, -482410],
                ],
                [
                    [244828, -4630, 298663, -447038],
                    [279872, -7203, 372825, -477961],
                ],
            ],
            [[995, 1723], [946, 1757], [3348, 1]],
            59,
            11921686182627400426,
        ),
        (
            [[-246, 3457], [4557, 3319]],
            [
                [
                    [220550, -8172, 337499, -474014],
                    [241650, -5376, 353005, -485286],
                ],
                [
                    [324320, -7743, 342798, -502418],
                    [345698, -7693, 389815, -549702],
                ],
            ],
            [[1056, 1903], [1091, 1930], [3361, 1]],
            61,
            15817898145558516207,
        ),
        (
            [[624, 8888], [8124, 4939]],
            [
                [
                    [272825, -7944, 384559, -510721],
                    [309692, -8587, 388984, -479218],
                ],
                [
                    [303720, -6336, 346211, -484612],
                    [391210, -9046, 411093, -474863],
                ],
            ],
            [[1077, 1900], [985, 2020], [3345, 0]],
            62,
            17298715596551695689,
        ),
        (
            [[6006, 6192], [2046, 6160]],
            [
                [
                    [384096, -8034, 413422, -531138],
                    [344556, -8214, 432873, -538171],
                ],
                [
                    [274444, -2905, 300941, -413271],
                    [306626, -3282, 386034, -493250],
                ],
            ],
            [[989, 2072], [1001, 2061], [3354, 0]],
            62,
            5249377351726160199,
        ),
        (
            [[5838, 3406], [7964, 19426]],
            [
                [
                    [301838, -8028, 432733, -531584],
                    [341810, -6296, 392529, -523946],
                ],
                [
                    [336377, -6033, 381943, -513289],
                    [379244, -10395, 458019, -539640],
                ],
            ],
            [[1086, 1983], [1011, 2131], [3337, 0]],
            64,
            1681742037761486053,
        ),
        (
            [[-102, 4477], [5379, 14248]],
            [
                [
                    [364403, -8878, 439421, -545369],
                    [363564, -6901, 438054, -554455],
                ],
                [
                    [338035, -7314, 373195, -507341],
                    [370067, -9968, 441852, -564715],
                ],
            ],
            [[1085, 2180], [1059, 2297], [3340, 0]],
            64,
            7868494497109307788,
        ),
        (
            [[13749, 13001], [228, 15256]],
            [
                [
                    [431409, -9206, 500786, -540975],
                    [461384, -10462, 536917, -591297],
                ],
                [
                    [331507, -5759, 380308, -490051],
                    [368934, -6462, 461189, -519656],
                ],
            ],
            [[1064, 2381], [1069, 2425], [3331, 0]],
            64,
            16226574772168389382,
        ),
        (
            [[17790, 12540], [8671, 23419]],
            [
                [
                    [343564, -5394, 459403, -576639],
                    [370261, -9256, 482433, -516860],
                ],
                [
                    [398525, -9299, 443386, -570982],
                    [472121, -11439, 521384, -596261],
                ],
            ],
            [[1118, 2314], [1077, 2488], [3354, 0]],
            64,
            14645007438732400877,
        ),
        (
            [[17184, 24189], [13983, 27021]],
            [
                [
                    [414167, -10103, 507148, -644190],
                    [460740, -8202, 514386, -592059],
                ],
                [
                    [353995, -6741, 357869, -504920],
                    [431156, -8274, 492435, -561135],
                ],
            ],
            [[1045, 2517], [1028, 2794], [3334, 0]],
            64,
            17616817441726762925,
        ),
        (
            [[11936, 12162], [18766, 30019]],
            [
                [
                    [425764, -10715, 508771, -619920],
                    [469495, -16905, 574812, -672046],
                ],
                [
                    [389144, -11381, 455490, -576169],
                    [469917, -12203, 556029, -574582],
                ],
            ],
            [[1164, 2665], [1103, 2833], [3342, 1]],
            64,
            3484181159429439860,
        ),
        (
            [[11166, 18129], [18435, 28376]],
            [
                [
                    [450408, -11558, 539507, -632712],
                    [522550, -13450, 546917, -602916],
                ],
                [
                    [408101, -9607, 458232, -609465],
                    [527461, -14889, 555672, -619788],
                ],
            ],
            [[1154, 2774], [1066, 3020], [3342, 0]],
            64,
            605281634970738369,
        ),
        (
            [[32713, 29292], [15497, 29207]],
            [
                [
                    [511944, -17004, 602455, -654893],
                    [526112, -18319, 622374, -703770],
                ],
                [
                    [396219, -10123, 467997, -576931],
                    [536406, -17036, 586692, -602639],
                ],
            ],
            [[1185, 2984], [1143, 3209], [3350, 0]],
            64,
            13901754270263793814,
        ),
        (
            [[28682, 29213], [17753, 32379]],
            [
                [
                    [633841, -14640, 626583, -722021],
                    [645126, -13368, 680386, -762196],
                ],
                [
                    [373554, -11996, 434136, -564013],
                    [484574, -16570, 620545, -652492],
                ],
            ],
            [[1151, 3211], [1166, 3326], [3341, 0]],
            64,
            10247938232656116723,
        ),
        (
            [[36803, 36855], [13879, 36599]],
            [
                [
                    [632008, -15446, 630031, -673138],
                    [651633, -18486, 702084, -745485],
                ],
                [
                    [401311, -9118, 501718, -560217],
                    [501161, -9581, 605491, -641559],
                ],
            ],
            [[1194, 3446], [1173, 3599], [3350, 1]],
            64,
            1112885541000463402,
        ),
        (
            [[32661, 25599], [15880, 36872]],
            [
                [
                    [624786, -16907, 628767, -666297],
                    [652739, -14331, 709725, -761195],
                ],
                [
                    [425774, -8832, 472201, -596875],
                    [557684, -14750, 633417, -700453],
                ],
            ],
            [[1137, 3486], [1139, 3718], [3344, 0]],
            64,
            6546253704213923618,
        ),
        (
            [[36255, 36549], [24060, 52717]],
            [
                [
                    [746740, -19934, 706183, -754579],
                    [797356, -20892, 805536, -862595],
                ],
                [
                    [388194, -12394, 456573, -585194],
                    [488315, -14602, 597799, -664710],
                ],
            ],
            [[1305, 3713], [1223, 3826], [3345, 0]],
            64,
            731565006732446392,
        ),
        (
            [[34355, 40571], [23896, 47435]],
            [
                [
                    [592036, -14169, 615450, -672168],
                    [659573, -20497, 728879, -737796],
                ],
                [
                    [568506, -15579, 567508, -658211],
                    [661273, -18671, 710709, -750875],
                ],
            ],
            [[1187, 3808], [1189, 3934], [3330, 0]],
            64,
            15960198310059701328,
        ),
        (
            [[35166, 57598], [17499, 47210]],
            [
                [
                    [706652, -15857, 637696, -711483],
                    [713208, -20599, 735615, -758483],
                ],
                [
                    [474735, -14553, 538040, -679374],
                    [646749, -19374, 672808, -713537],
                ],
            ],
            [[1317, 4035], [1265, 4158], [3319, 0]],
            64,
            1977767949079865260,
        ),
        (
            [[47543, 51695], [25700, 40245]],
            [
                [
                    [621322, -14366, 644494, -673227],
                    [649101, -14078, 706228, -720196],
                ],
                [
                    [601175, -19616, 626677, -647462],
                    [747081, -22433, 747962, -796228],
                ],
            ],
            [[1255, 4030], [1217, 4359], [3347, 0]],
            64,
            1767764977576629575,
        ),
        (
            [[44406, 50250], [40675, 47957]],
            [
                [
                    [588772, -21961, 697188, -703663],
                    [631477, -15664, 724924, -733854],
                ],
                [
                    [661423, -19953, 629740, -737343],
                    [860485, -21808, 749397, -754798],
                ],
            ],
            [[1275, 4342], [1157, 4670], [3323, 0]],
            64,
            11062377075487004992,
        ),
        (
            [[35830, 76019], [17883, 36408]],
            [
                [
                    [683454, -16878, 727727, -817976],
                    [759634, -24108, 816102, -852579],
                ],
                [
                    [587639, -16859, 611437, -742493],
                    [747950, -24495, 745589, -816617],
                ],
            ],
            [[1413, 4505], [1290, 4823], [3352, 0]],
            64,
            9145738838034621121,
        ),
        (
            [[44321, 52212], [36639, 49717]],
            [
                [
                    [638153, -13918, 665600, -750427],
                    [695917, -17296, 755251, -800747],
                ],
                [
                    [662804, -14279, 661980, -816721],
                    [874912, -24607, 822835, -910516],
                ],
            ],
            [[1322, 4761], [1288, 4945], [3336, 1]],
            64,
            2121837591356787042,
        ),
    ],
    &[
        (
            [[1224, 1678], [84, 1056]],
            [
                [
                    [221710, -5259, 321631, -501930],
                    [245740, -5696, 351022, -490629],
                ],
                [
                    [229082, -5518, 291611, -469878],
                    [209526, -5320, 333453, -447101],
                ],
            ],
            [[1095, 1656], [1016, 1668], [3350, 0]],
            49,
            4875818070468884952,
        ),
        (
            [[70, 2000], [-1200, -403]],
            [
                [
                    [213535, -5241, 328445, -471165],
                    [254483, -5803, 357488, -460545],
                ],
                [
                    [270810, -6704, 306849, -464545],
                    [241979, -5084, 314593, -482165],
                ],
            ],
            [[995, 1661], [1017, 1696], [3349, 0]],
            53,
            16635308884347259713,
        ),
        (
            [[-2314, 1170], [4445, 10045]],
            [
                [
                    [233931, -5578, 315210, -465886],
                    [260693, -6043, 378096, -486366],
                ],
                [
                    [246967, -5459, 299867, -451529],
                    [270110, -7057, 368955, -476422],
                ],
            ],
            [[995, 1717], [946, 1740], [3348, 1]],
            59,
            6308149380219909452,
        ),
        (
            [[-9, 4388], [5203, 18367]],
            [
                [
                    [212040, -8393, 326393, -462792],
                    [264900, -6160, 370265, -500762],
                ],
                [
                    [333274, -8134, 351975, -507453],
                    [332186, -7538, 374798, -541094],
                ],
            ],
            [[1053, 1882], [1096, 1940], [3360, 1]],
            60,
            2227008261100309101,
        ),
        (
            [[-323, 4642], [5282, 19115]],
            [
                [
                    [260544, -8079, 364551, -493159],
                    [341832, -10264, 415257, -493922],
                ],
                [
                    [317151, -6219, 351128, -499174],
                    [355030, -6860, 371985, -460595],
                ],
            ],
            [[1077, 1902], [988, 2009], [3344, 0]],
            63,
            16752537024635391542,
        ),
        (
            [[7281, 8209], [1622, 5765]],
            [
                [
                    [362195, -7479, 393247, -505748],
                    [389052, -9826, 480559, -559948],
                ],
                [
                    [282995, -2806, 317553, -428758],
                    [275996, -2094, 355158, -457639],
                ],
            ],
            [[986, 2060], [999, 2097], [3353, 0]],
            62,
            10201695642310798502,
        ),
        (
            [[6858, 2201], [3528, 10784]],
            [
                [
                    [291350, -6978, 407767, -510497],
                    [375181, -8325, 442442, -567124],
                ],
                [
                    [351949, -6819, 396790, -545454],
                    [345649, -9210, 423474, -517301],
                ],
            ],
            [[1087, 2010], [1016, 2119], [3337, 0]],
            64,
            9057864359552435503,
        ),
        (
            [[-355, 5877], [4742, 16899]],
            [
                [
                    [350394, -7708, 406263, -530164],
                    [408741, -8967, 495083, -614212],
                ],
                [
                    [356720, -8371, 385288, -514747],
                    [346648, -8475, 422389, -540143],
                ],
            ],
            [[1084, 2189], [1070, 2309], [3339, 0]],
            64,
            2729510492604696917,
        ),
        (
            [[7667, 17112], [1518, 21120]],
            [
                [
                    [404772, -9460, 470004, -510371],
                    [515194, -11409, 598811, -639398],
                ],
                [
                    [346439, -6296, 402801, -505726],
                    [344053, -6179, 435992, -510027],
                ],
            ],
            [[1069, 2368], [1076, 2455], [3332, 0]],
            64,
            13292565573362117768,
        ),
        (
            [[15293, 19104], [10581, 15642]],
            [
                [
                    [320817, -4524, 438638, -551106],
                    [415001, -9969, 541237, -568200],
                ],
                [
                    [425792, -9822, 470549, -584086],
                    [436115, -11112, 491222, -567370],
                ],
            ],
            [[1109, 2297], [1077, 2521], [3353, 0]],
            64,
            4472911491304782609,
        ),
        (
            [[14236, 17766], [14070, 24708]],
            [
                [
                    [385084, -10020, 495591, -623608],
                    [509079, -10948, 588373, -630706],
                ],
                [
                    [381410, -6572, 381854, -520839],
                    [391974, -6796, 444868, -515708],
                ],
            ],
            [[1044, 2489], [1023, 2834], [3333, 0]],
            64,
            12143305719222569001,
        ),
        (
            [[10517, 12625], [20578, 22191]],
            [
                [
                    [388373, -10182, 488759, -602683],
                    [548285, -19378, 654522, -738806],
                ],
                [
                    [402753, -13274, 486277, -617956],
                    [423415, -9716, 512587, -552877],
                ],
            ],
            [[1168, 2667], [1109, 2855], [3343, 1]],
            64,
            14258887745567619028,
        ),
        (
            [[10704, 24846], [22233, 34361]],
            [
                [
                    [420773, -10966, 503343, -601196],
                    [601098, -16635, 642907, -687099],
                ],
                [
                    [433891, -9584, 484054, -637107],
                    [472462, -12781, 507990, -586086],
                ],
            ],
            [[1153, 2755], [1068, 3042], [3341, 0]],
            64,
            13482222028571616623,
        ),
        (
            [[33912, 32498], [16434, 27186]],
            [
                [
                    [470245, -13427, 567746, -623092],
                    [609264, -22079, 723548, -789855],
                ],
                [
                    [429997, -10647, 504714, -619383],
                    [464463, -13580, 522032, -560288],
                ],
            ],
            [[1183, 2905], [1141, 3247], [3350, 0]],
            64,
            4083150327106594389,
        ),
        (
            [[26719, 41224], [16212, 28738]],
            [
                [
                    [589566, -13454, 590075, -687307],
                    [750184, -17017, 790148, -853255],
                ],
                [
                    [401338, -13295, 474057, -581487],
                    [417490, -14172, 542002, -592674],
                ],
            ],
            [[1141, 3112], [1164, 3395], [3342, 0]],
            64,
            4346513988771814212,
        ),
        (
            [[28992, 50506], [18123, 25840]],
            [
                [
                    [577987, -14008, 586417, -640237],
                    [764592, -22036, 810321, -830910],
                ],
                [
                    [429461, -8822, 536179, -582829],
                    [438819, -7480, 554786, -606673],
                ],
            ],
            [[1196, 3392], [1170, 3650], [3350, 0]],
            64,
            16003831220237787369,
        ),
        (
            [[27130, 32356], [18255, 30288]],
            [
                [
                    [565739, -13110, 613382, -648004],
                    [744514, -17621, 827646, -853869],
                ],
                [
                    [448407, -10727, 501304, -623057],
                    [494809, -10278, 552553, -608390],
                ],
            ],
            [[1123, 3413], [1131, 3749], [3344, 0]],
            64,
            1953721645472014172,
        ),
        (
            [[29525, 46191], [26931, 47420]],
            [
                [
                    [725534, -18739, 688951, -724640],
                    [924094, -24582, 898560, -958622],
                ],
                [
                    [415858, -14583, 498310, -619668],
                    [424411, -13652, 543981, -598799],
                ],
            ],
            [[1297, 3684], [1227, 3950], [3344, 0]],
            64,
            12246359953320232664,
        ),
        (
            [[30440, 48162], [21491, 44234]],
            [
                [
                    [585821, -14042, 610530, -662364],
                    [760792, -24422, 816197, -793387],
                ],
                [
                    [603604, -16469, 599385, -693332],
                    [588428, -17512, 637179, -681093],
                ],
            ],
            [[1193, 3904], [1183, 4037], [3330, 0]],
            64,
            17404118533540291430,
        ),
        (
            [[40600, 65672], [20059, 35331]],
            [
                [
                    [699205, -14346, 638113, -699377],
                    [854860, -24221, 837681, -810874],
                ],
                [
                    [527146, -15101, 573636, -712047],
                    [583140, -18511, 605908, -665370],
                ],
            ],
            [[1317, 4137], [1234, 4302], [3319, 0]],
            64,
            15084375480375239195,
        ),
        (
            [[46454, 39398], [30103, 33751]],
            [
                [
                    [629634, -13436, 635067, -638149],
                    [773358, -19307, 800961, -832992],
                ],
                [
                    [661231, -22809, 655268, -668726],
                    [681720, -19900, 659538, -708865],
                ],
            ],
            [[1229, 4230], [1242, 4532], [3346, 0]],
            64,
            18100069406202911844,
        ),
        (
            [[49772, 59370], [43914, 59618]],
            [
                [
                    [583579, -22508, 696259, -673795],
                    [742371, -21026, 825706, -797969],
                ],
                [
                    [728028, -23812, 691471, -778605],
                    [768834, -18215, 696356, -705465],
                ],
            ],
            [[1264, 4511], [1149, 4767], [3321, 0]],
            64,
            10898570306860397167,
        ),
        (
            [[35967, 72662], [26462, 29258]],
            [
                [
                    [666989, -17179, 728667, -804443],
                    [870913, -30880, 949205, -984505],
                ],
                [
                    [673699, -18888, 668628, -792344],
                    [668185, -18731, 682183, -769778],
                ],
            ],
            [[1419, 4693], [1303, 4908], [3351, 0]],
            64,
            16208986338346559754,
        ),
        (
            [[45752, 73847], [41376, 37614]],
            [
                [
                    [622974, -14267, 671246, -722419],
                    [806987, -21730, 853923, -876839],
                ],
                [
                    [733691, -16123, 710190, -879049],
                    [779284, -19732, 761065, -852370],
                ],
            ],
            [[1340, 4840], [1295, 4968], [3338, 1]],
            64,
            7406428007107404268,
        ),
    ],
    &[
        (
            [[1954, 4710], [2002, 915]],
            [
                [
                    [224522, -4914, 322020, -501639],
                    [240990, -5239, 349101, -489980],
                ],
                [
                    [228825, -5513, 291793, -469211],
                    [208802, -5357, 333348, -447389],
                ],
            ],
            [[1093, 1657], [1017, 1657], [3350, 0]],
            51,
            14391910066059119395,
        ),
        (
            [[1186, 1459], [-821, -587]],
            [
                [
                    [218612, -5617, 329888, -473432],
                    [254980, -5806, 354189, -458885],
                ],
                [
                    [269532, -6702, 307754, -463172],
                    [245127, -5126, 319576, -481467],
                ],
            ],
            [[995, 1664], [1019, 1703], [3349, 0]],
            54,
            7156570894980421545,
        ),
        (
            [[-2021, 1316], [4746, 8351]],
            [
                [
                    [234327, -5553, 322756, -469913],
                    [257069, -6065, 370756, -483063],
                ],
                [
                    [245513, -4854, 299715, -453578],
                    [278305, -7161, 374991, -473374],
                ],
            ],
            [[997, 1720], [947, 1751], [3347, 1]],
            59,
            4871129838240990066,
        ),
        (
            [[-75, 4087], [4171, 9744]],
            [
                [
                    [218598, -8520, 331496, -472794],
                    [255293, -5767, 361939, -496971],
                ],
                [
                    [328948, -7983, 350881, -504076],
                    [340044, -7682, 386203, -545852],
                ],
            ],
            [[1054, 1901], [1090, 1942], [3360, 1]],
            60,
            11222430077264760200,
        ),
        (
            [[110, 6983], [6306, 10665]],
            [
                [
                    [266911, -7451, 380591, -509699],
                    [331470, -10116, 407254, -496233],
                ],
                [
                    [307987, -6269, 348353, -492747],
                    [372174, -8171, 392198, -466382],
                ],
            ],
            [[1079, 1897], [983, 2021], [3345, 0]],
            63,
            16345487289156515632,
        ),
        (
            [[7589, 7814], [1667, 6432]],
            [
                [
                    [371519, -7657, 402529, -520608],
                    [369567, -8636, 457333, -542255],
                ],
                [
                    [281950, -2836, 307338, -417714],
                    [290504, -3101, 368141, -476197],
                ],
            ],
            [[985, 2062], [999, 2071], [3352, 0]],
            64,
            14470464270366016443,
        ),
        (
            [[6277, 3030], [5886, 13941]],
            [
                [
                    [289944, -7995, 419085, -522020],
                    [371519, -6949, 426847, -551848],
                ],
                [
                    [345284, -5745, 387133, -529651],
                    [369701, -9610, 449278, -535970],
                ],
            ],
            [[1088, 1996], [1017, 2162], [3337, 0]],
            64,
            3734294405486344508,
        ),
        (
            [[274, 7250], [6580, 18291]],
            [
                [
                    [349908, -7870, 425389, -525618],
                    [394667, -8018, 472076, -585452],
                ],
                [
                    [351133, -7983, 383540, -510691],
                    [364634, -9915, 439846, -562514],
                ],
            ],
            [[1084, 2184], [1070, 2325], [3340, 0]],
            64,
            8976855206667300178,
        ),
        (
            [[9678, 13650], [320, 18229]],
            [
                [
                    [411619, -9290, 479057, -531279],
                    [490220, -10659, 562882, -626225],
                ],
                [
                    [347118, -5858, 397853, -507288],
                    [356846, -6445, 445246, -522533],
                ],
            ],
            [[1070, 2360], [1082, 2443], [3333, 0]],
            64,
            5841962401737437325,
        ),
        (
            [[14887, 13444], [9229, 19557]],
            [
                [
                    [326562, -5380, 438642, -555901],
                    [400884, -9915, 514456, -549792],
                ],
                [
                    [415096, -9434, 457643, -574739],
                    [451755, -10510, 504356, -577986],
                ],
            ],
            [[1106, 2327], [1081, 2542], [3354, 0]],
            64,
            17834068858660187897,
        ),
        (
            [[13495, 19028], [16408, 24448]],
            [
                [
                    [396804, -10613, 498207, -625704],
                    [482163, -10114, 561185, -638679],
                ],
                [
                    [374623, -6846, 378119, -513309],
                    [424677, -7850, 479918, -542455],
                ],
            ],
            [[1045, 2520], [1029, 2838], [3334, 0]],
            64,
            10385622438990969836,
        ),
        (
            [[10030, 12292], [20186, 26127]],
            [
                [
                    [396332, -10040, 496095, -608175],
                    [514827, -18005, 626181, -711397],
                ],
                [
                    [400842, -12283, 464361, -591753],
                    [451843, -11345, 539741, -564167],
                ],
            ],
            [[1152, 2646], [1103, 2890], [3345, 1]],
            64,
            16520975604103118048,
        ),
        (
            [[11043, 24459], [21171, 36093]],
            [
                [
                    [431681, -11048, 520437, -616065],
                    [562070, -15323, 598327, -655504],
                ],
                [
                    [427064, -9946, 465491, -617014],
                    [504584, -14517, 534174, -610491],
                ],
            ],
            [[1162, 2766], [1078, 3066], [3344, 0]],
            64,
            16870112787992976138,
        ),
        (
            [[33063, 36241], [15345, 26958]],
            [
                [
                    [469667, -12587, 570698, -635717],
                    [559988, -19376, 667547, -746585],
                ],
                [
                    [418128, -10829, 488716, -597951],
                    [498494, -14645, 559029, -582493],
                ],
            ],
            [[1178, 2908], [1138, 3246], [3350, 0]],
            64,
            11358985968739326916,
        ),
        (
            [[34359, 33324], [17551, 31094]],
            [
                [
                    [612638, -14432, 597652, -680003],
                    [693021, -15122, 729168, -798518],
                ],
                [
                    [386592, -11395, 445372, -576081],
                    [450656, -15181, 589332, -624873],
                ],
            ],
            [[1133, 3135], [1173, 3356], [3340, 0]],
            64,
            12980382693589736749,
        ),
        (
            [[27904, 43719], [16131, 32993]],
            [
                [
                    [589930, -14650, 593050, -633536],
                    [708081, -22221, 757921, -770981],
                ],
                [
                    [420618, -9360, 527686, -580560],
                    [482968, -9002, 585181, -624088],
                ],
            ],
            [[1188, 3415], [1166, 3622], [3350, 0]],
            64,
            8009413439086147504,
        ),
        (
            [[27399, 31734], [16652, 38165]],
            [
                [
                    [582147, -15483, 613195, -645552],
                    [699750, -16155, 768048, -816291],
                ],
                [
                    [434929, -8901, 478539, -599189],
                    [525197, -13083, 591624, -649074],
                ],
            ],
            [[1124, 3406], [1147, 3726], [3343, 0]],
            64,
            6312102842356963653,
        ),
        (
            [[31554, 38196], [27663, 47584]],
            [
                [
                    [724941, -20136, 690375, -739148],
                    [874281, -23931, 849668, -920794],
                ],
                [
                    [387020, -13983, 469942, -599164],
                    [457027, -15172, 556353, -634655],
                ],
            ],
            [[1305, 3653], [1233, 3891], [3344, 0]],
            64,
            8071051274674109483,
        ),
        (
            [[30699, 45903], [27438, 47586]],
            [
                [
                    [574331, -13624, 618820, -660222],
                    [718328, -21082, 779904, -757523],
                ],
                [
                    [570585, -15989, 573146, -681191],
                    [613593, -17685, 685329, -714279],
                ],
            ],
            [[1202, 3796], [1191, 3966], [3331, 0]],
            64,
            4512556398921558735,
        ),
        (
            [[34972, 67314], [17293, 41472]],
            [
                [
                    [687623, -15184, 620367, -714019],
                    [787089, -22029, 792406, -795225],
                ],
                [
                    [475812, -14469, 533818, -682028],
                    [615974, -19413, 645391, -692091],
                ],
            ],
            [[1320, 4017], [1260, 4232], [3318, 0]],
            64,
            9511367075768714685,
        ),
        (
            [[47309, 42952], [28391, 36469]],
            [
                [
                    [617489, -12742, 633490, -638466],
                    [718274, -17451, 749936, -759699],
                ],
                [
                    [597425, -19827, 627479, -653382],
                    [706785, -20930, 719813, -782009],
                ],
            ],
            [[1242, 4008], [1228, 4410], [3344, 0]],
            64,
            15532973765467117604,
        ),
        (
            [[52943, 56447], [39421, 52828]],
            [
                [
                    [587447, -22110, 701613, -678854],
                    [688266, -17550, 782377, -754270],
                ],
                [
                    [659289, -20301, 637973, -738334],
                    [785543, -18234, 722578, -737575],
                ],
            ],
            [[1268, 4311], [1153, 4660], [3322, 0]],
            64,
            5370203420150183073,
        ),
        (
            [[34697, 76328], [18983, 31756]],
            [
                [
                    [687814, -17383, 735014, -798742],
                    [825237, -26476, 896349, -924670],
                ],
                [
                    [588441, -16155, 607653, -746356],
                    [701832, -19069, 708945, -784157],
                ],
            ],
            [[1407, 4535], [1285, 4848], [3351, 0]],
            64,
            9107765927647604979,
        ),
        (
            [[41505, 64839], [34060, 44677]],
            [
                [
                    [633075, -13460, 668244, -735320],
                    [747683, -18995, 797537, -834194],
                ],
                [
                    [681129, -14212, 672869, -831050],
                    [814092, -20818, 788395, -868792],
                ],
            ],
            [[1318, 4742], [1273, 4890], [3335, 1]],
            64,
            10966691613784855157,
        ),
    ],
];
const EVERYWHERE_EARNED_1024: [&[EarnedBlock]; 3] = [
    &[
        (
            [[15, 15, 4], [12, 16, 2]],
            31,
            [[31162, 46293], [18771, 77033]],
            -24175,
            41361,
        ),
        (
            [[13, 13, 5], [13, 19, 1]],
            32,
            [[57168, 95572], [71597, 75115]],
            44591,
            -20945,
        ),
        (
            [[12, 18, 1], [15, 15, 3]],
            27,
            [[86173, 81089], [50957, 110850]],
            -28449,
            -93985,
        ),
        (
            [[8, 18, 2], [10, 24, 2]],
            32,
            [[59935, 57287], [103764, 133024]],
            36349,
            101885,
        ),
        (
            [[8, 17, 5], [7, 25, 2]],
            33,
            [[122381, 115202], [93915, 245767]],
            28315,
            93851,
        ),
        (
            [[19, 12, 5], [10, 18, 0]],
            37,
            [[200169, 135557], [75652, 132098]],
            -19624,
            -85160,
        ),
        (
            [[15, 15, 0], [7, 23, 4]],
            38,
            [[132047, 97436], [96915, 130248]],
            -18795,
            46741,
        ),
        (
            [[13, 14, 5], [5, 26, 1]],
            39,
            [[147562, 134518], [86764, 88658]],
            44049,
            -21487,
        ),
        (
            [[17, 15, 2], [8, 19, 3]],
            36,
            [[185680, 200838], [114619, 154849]],
            -23638,
            41898,
        ),
        (
            [[7, 17, 4], [6, 28, 2]],
            35,
            [[63991, 133296], [132432, 168717]],
            31569,
            97105,
        ),
        (
            [[12, 18, 3], [1, 29, 1]],
            41,
            [[79493, 174466], [83334, 164743]],
            -6791,
            58745,
        ),
        (
            [[8, 19, 5], [8, 22, 2]],
            30,
            [[111598, 148323], [100590, 161440]],
            -36259,
            -101795,
        ),
        (
            [[8, 23, 1], [4, 28, 0]],
            36,
            [[100984, 135706], [85936, 188808]],
            43001,
            108537,
        ),
        (
            [[14, 15, 4], [2, 27, 2]],
            41,
            [[162590, 172059], [75768, 170484]],
            17413,
            82949,
        ),
        (
            [[12, 21, 4], [5, 20, 2]],
            32,
            [[185458, 158440], [57979, 124549]],
            -32680,
            -98216,
        ),
        (
            [[17, 18, 0], [3, 23, 3]],
            40,
            [[170513, 162811], [124653, 121564]],
            35819,
            -29717,
        ),
        (
            [[11, 17, 5], [3, 26, 2]],
            37,
            [[170202, 158451], [99488, 126779]],
            -6159,
            59377,
        ),
        (
            [[13, 21, 4], [5, 21, 0]],
            34,
            [[187070, 170812], [69359, 73366]],
            24264,
            89800,
        ),
        (
            [[9, 21, 1], [6, 25, 2]],
            34,
            [[92100, 152154], [150877, 141578]],
            24103,
            89639,
        ),
        (
            [[13, 20, 1], [4, 24, 2]],
            37,
            [[126652, 118491], [80041, 136138]],
            43861,
            -21675,
        ),
        (
            [[4, 24, 2], [5, 28, 1]],
            32,
            [[101469, 91783], [175728, 161553]],
            -41259,
            24277,
        ),
        (
            [[8, 19, 2], [2, 32, 1]],
            40,
            [[103486, 125160], [125357, 249540]],
            46583,
            -18953,
        ),
        (
            [[8, 23, 1], [2, 30, 0]],
            38,
            [[103409, 120504], [111764, 126918]],
            -27812,
            -93348,
        ),
        (
            [[7, 21, 1], [2, 33, 0]],
            40,
            [[105828, 101909], [105225, 133687]],
            46581,
            112117,
        ),
    ],
    &[
        (
            [[14, 17, 3], [12, 16, 2]],
            29,
            [[24313, 91411], [45426, 58597]],
            24175,
            -41361,
        ),
        (
            [[12, 14, 5], [13, 16, 4]],
            27,
            [[40001, 126653], [97376, 39791]],
            -23535,
            -89071,
        ),
        (
            [[11, 19, 1], [16, 16, 1]],
            35,
            [[48055, 129959], [81748, 88646]],
            28449,
            93985,
        ),
        (
            [[6, 21, 1], [12, 21, 3]],
            33,
            [[46142, 103624], [143176, 68510]],
            -37677,
            -103213,
        ),
        (
            [[8, 20, 2], [11, 22, 1]],
            31,
            [[90486, 192366], [105998, 125481]],
            -28323,
            -93859,
        ),
        (
            [[12, 23, 1], [14, 14, 0]],
            37,
            [[165527, 210860], [127198, 105201]],
            23588,
            89124,
        ),
        (
            [[8, 19, 3], [19, 12, 3]],
            38,
            [[101415, 155017], [119172, 138274]],
            27003,
            92539,
        ),
        (
            [[6, 26, 0], [16, 14, 2]],
            42,
            [[128369, 207150], [125782, 89591]],
            29185,
            94721,
        ),
        (
            [[9, 24, 1], [20, 9, 1]],
            44,
            [[186820, 299110], [144476, 132008]],
            23666,
            89202,
        ),
        (
            [[2, 25, 1], [17, 17, 2]],
            42,
            [[68035, 191064], [139314, 113903]],
            -31160,
            34376,
        ),
        (
            [[5, 26, 2], [12, 16, 3]],
            38,
            [[78001, 190243], [97033, 96560]],
            44385,
            -21151,
        ),
        (
            [[0, 30, 2], [13, 17, 2]],
            43,
            [[92916, 157187], [81188, 124728]],
            36902,
            102438,
        ),
        (
            [[1, 31, 0], [11, 16, 5]],
            42,
            [[80777, 185989], [107502, 120684]],
            -42995,
            -108531,
        ),
        (
            [[2, 29, 2], [12, 19, 0]],
            41,
            [[123004, 192734], [88302, 121222]],
            45487,
            -20049,
        ),
        (
            [[2, 33, 2], [11, 14, 2]],
            44,
            [[183831, 205658], [100174, 58561]],
            39192,
            104728,
        ),
        (
            [[2, 31, 2], [16, 13, 0]],
            47,
            [[154576, 216786], [133883, 83109]],
            -2475,
            63061,
        ),
        (
            [[1, 31, 1], [16, 14, 1]],
            47,
            [[176915, 173465], [76742, 106189]],
            7315,
            -58221,
        ),
        (
            [[1, 35, 2], [17, 7, 2]],
            52,
            [[231361, 212312], [83068, 64498]],
            -8841,
            -74377,
        ),
        (
            [[2, 29, 0], [21, 10, 2]],
            50,
            [[122476, 189862], [179979, 109270]],
            35921,
            101457,
        ),
        (
            [[1, 32, 1], [15, 14, 1]],
            47,
            [[138170, 196095], [110013, 108416]],
            37553,
            103089,
        ),
        (
            [[0, 30, 0], [17, 15, 2]],
            47,
            [[136226, 192303], [203114, 157588]],
            42017,
            107553,
        ),
        (
            [[0, 29, 0], [18, 14, 3]],
            47,
            [[113033, 163892], [142933, 210651]],
            37207,
            102743,
        ),
        (
            [[0, 32, 0], [17, 13, 2]],
            49,
            [[110273, 120830], [105073, 139814]],
            29211,
            94747,
        ),
        (
            [[1, 28, 0], [29, 6, 0]],
            57,
            [[99844, 129495], [86461, 133896]],
            17358,
            82894,
        ),
    ],
    &[
        (
            [[15, 16, 3], [12, 16, 2]],
            0,
            [[29153, 69875], [35016, 70060]],
            0,
            0,
        ),
        (
            [[12, 13, 6], [13, 16, 4]],
            0,
            [[53825, 113592], [85324, 59685]],
            0,
            0,
        ),
        (
            [[12, 18, 1], [15, 16, 2]],
            0,
            [[64095, 107659], [65910, 107478]],
            0,
            0,
        ),
        (
            [[6, 20, 2], [9, 23, 4]],
            0,
            [[50550, 81735], [128106, 106531]],
            0,
            0,
        ),
        (
            [[8, 20, 2], [8, 25, 1]],
            0,
            [[106571, 152094], [93528, 187495]],
            0,
            0,
        ),
        (
            [[15, 17, 4], [11, 16, 1]],
            0,
            [[184205, 167666], [102486, 115207]],
            0,
            0,
        ),
        (
            [[11, 16, 3], [12, 18, 4]],
            0,
            [[101420, 138171], [102020, 141977]],
            0,
            0,
        ),
        (
            [[7, 23, 2], [9, 19, 4]],
            0,
            [[138166, 167595], [107922, 83530]],
            0,
            0,
        ),
        (
            [[11, 22, 1], [14, 14, 2]],
            0,
            [[182265, 243593], [138858, 111019]],
            0,
            0,
        ),
        (
            [[3, 24, 1], [11, 22, 3]],
            0,
            [[53535, 207696], [136809, 136726]],
            0,
            0,
        ),
        (
            [[7, 24, 2], [1, 27, 3]],
            0,
            [[82642, 178973], [90321, 160344]],
            0,
            0,
        ),
        (
            [[0, 25, 7], [13, 19, 0]],
            0,
            [[99846, 158992], [83239, 153150]],
            0,
            0,
        ),
        (
            [[4, 28, 0], [1, 22, 9]],
            0,
            [[80262, 138487], [102507, 136339]],
            0,
            0,
        ),
        (
            [[6, 25, 2], [7, 23, 1]],
            0,
            [[105310, 139332], [93445, 150395]],
            0,
            0,
        ),
        (
            [[5, 31, 1], [7, 19, 1]],
            0,
            [[193658, 178021], [65281, 83804]],
            0,
            0,
        ),
        (
            [[6, 28, 1], [8, 20, 1]],
            0,
            [[171517, 233951], [123095, 126992]],
            0,
            0,
        ),
        (
            [[4, 28, 1], [6, 20, 5]],
            0,
            [[183311, 145881], [80841, 98006]],
            0,
            0,
        ),
        (
            [[4, 31, 3], [7, 15, 4]],
            0,
            [[184556, 184839], [50654, 54866]],
            0,
            0,
        ),
        (
            [[1, 29, 1], [15, 16, 2]],
            0,
            [[121067, 178863], [111845, 139048]],
            0,
            0,
        ),
        (
            [[3, 29, 2], [10, 20, 0]],
            0,
            [[117756, 134388], [71772, 158864]],
            0,
            0,
        ),
        (
            [[0, 30, 0], [8, 26, 0]],
            0,
            [[122537, 160609], [158901, 139898]],
            0,
            0,
        ),
        (
            [[1, 28, 0], [7, 26, 2]],
            0,
            [[99631, 154462], [123676, 172716]],
            0,
            0,
        ),
        (
            [[2, 30, 0], [9, 21, 2]],
            0,
            [[126703, 133650], [105529, 108613]],
            0,
            0,
        ),
        (
            [[1, 27, 1], [8, 23, 4]],
            0,
            [[115173, 89447], [128050, 148903]],
            0,
            0,
        ),
    ],
];
const EVERYWHERE_READ_1024: [u64; 3] = [0xf549d4fe37934d76, 0xcf09be179021adbe, 0x9249fc186898a4f6];
const EVERYWHERE_CENSUS_1024: [&[(u32, u64)]; 3] = [
    &[
        (0, 1),
        (1, 2962),
        (2, 23806),
        (3, 38557),
        (4, 12004),
        (5, 617),
        (6, 111),
        (7, 32),
        (8, 15),
        (9, 4),
        (10, 1),
        (11, 4),
    ],
    &[
        (0, 1),
        (1, 2966),
        (2, 23787),
        (3, 38556),
        (4, 12001),
        (5, 646),
        (6, 103),
        (7, 32),
        (8, 16),
        (9, 4),
        (10, 1),
        (11, 3),
    ],
    &[
        (0, 1),
        (1, 2965),
        (2, 23787),
        (3, 38556),
        (4, 12013),
        (5, 630),
        (6, 106),
        (7, 32),
        (8, 16),
        (9, 5),
        (10, 1),
        (11, 3),
    ],
];
/// The verdict, by the rule committed first over the pinned tables: the assignment fails the
/// mark and the mirrored assignment passes it, so H-15 is no.
const EVERYWHERE_1024: Reinforced = Reinforced {
    correct: [false, true],
    yes: false,
};
/// The correct selections over the last 128 trials, per rewarded arm, against `REWARDED_MIN`:
/// 78 of 128 in the assignment, two short of the mark, and 106 in the mirrored assignment.
const CORRECT_LAST_EVERYWHERE_1024: [u32; 2] = [78, 106];
/// Where each arm's selection first passed 40 of 64 per block, as read, beside H-14's 448 and
/// 384: the assignment at trial 704 (the eleventh block, at 41, a mark it reaches again in
/// four later blocks and never holds), the mirrored assignment at 512 (the eighth, at 42,
/// from where it climbs to 57); the withheld arm, whose correct trials read the assignment's
/// answers, never.
const CROSSED_EVERYWHERE_1024: [Option<usize>; 3] = [Some(704), Some(512), None];
/// The selections per stimulus over the last 128 trials, per arm, `[stimulus][readout 0,
/// readout 1, tie]`: in the assignment A, whose answer is readout 0, went to readout 1 in 44
/// of 61 and to its answer in 15, while B went to its answer, readout 1, in 63 of 67; in the
/// mirrored assignment A went to its answer, readout 1, in 60 of 61 and B, whose answer is
/// readout 0, to it in 46 of 67; with the reward withheld A went to readout 1 in 57 of 61 and
/// B in 44 of 67.
const SPLITS_LAST_EVERYWHERE_1024: [[[u32; 3]; 2]; 3] = [
    [[15, 44, 2], [4, 63, 0]],
    [[1, 60, 0], [46, 19, 2]],
    [[3, 57, 1], [17, 44, 6]],
];
/// ADR-0082's assertion as read on each arm, clause by clause — the signal at every trial's
/// end between −0.712 and 0.712, the wrong pair's modulation after a punishment at most
/// 0.2125, the answer pair's after a reward at least 0.7875 — true on both rewarded arms; the
/// withheld arm's, with no reward and the signal at rest, a reading.
const BOUNDS_1024: [[bool; 3]; 3] = [[true, true, true], [true, true, true], [true, true, true]];
/// The withheld arm's drift, as read, beside `DRIFT_PREDICTED`, which it equals: every
/// coupling rose (to 1.443, 1.546, 1.362 and 1.433 of the image's on A→R0, A→R1, B→R0 and
/// B→R1), the two onto readout 1 faster, and both stimuli selected readout 1 over the last 128
/// trials.
const DRIFT_1024: Drift = Drift {
    rose: [[true, true], [true, true]],
    faster_onto_r1: [true, true],
    toward_r1: [true, true],
};
/// Where each stimulus reached its answer at the crossing's rate in each rewarded arm, as
/// read, `[A, B]`: in the assignment A never and B at trial 256; in the mirrored assignment
/// A at 256 and B at 576.
const REACHED_1024: [[Option<usize>; 2]; 2] = [[None, Some(256)], [Some(256), Some(576)]];
/// Whether the stimulus whose answer is readout 0 reached its answer later than the other or
/// not at all, in each rewarded arm, as read, beside `R0_LATER_PREDICTED`, which it equals:
/// A in the assignment not at all, B in the mirrored assignment later.
const R0_LATER_1024: [bool; 2] = [true, true];
/// The reach of each arm's run, `(inside the four pairs, outside)`, as read: every one of the
/// four pairs' 3 188 synapses moved in every arm, and 29 537 to 29 539 outside them — all but
/// 43 of the arena's 32 768 — since every synapse consolidates under the baseline.
const EVERYWHERE_REACH_1024: [(u64, u64); 3] = [(3188, 29537), (3188, 29539), (3188, 29537)];
const _: () = assert!(
    EVERYWHERE_REACH_1024[0].0 as u32
        == SYNAPSES_1024[0][0] + SYNAPSES_1024[0][1] + SYNAPSES_1024[1][0] + SYNAPSES_1024[1][1]
        && EVERYWHERE_REACH_1024[1].0 == EVERYWHERE_REACH_1024[0].0
        && EVERYWHERE_REACH_1024[2].0 == EVERYWHERE_REACH_1024[0].0
);

/// The gate's test (ADR-0061's class; brief 038): the arms and their feedback; the baseline
/// and the bounds as ADR-0082 derived them; the assertion's rule at its edges over trials
/// written by hand; the readings' rules (the drift, where a stimulus reaches its answer,
/// which stimulus is later) over tables written by hand; and eight trials under the baseline
/// at 0.5 on the instrument's network at 1 024 units, the assignment and the reward withheld
/// — the record's traces, weights and signal held to the oracle at every trial, the bounds
/// true, synapses moved inside the four pairs and outside them, the wrong pairs among those
/// that consolidated, and under the withheld arm no reward and the signal at rest. No whole
/// run, and nothing else added to the gate. The instrument's network and not the settled one,
/// as ADR-0081's gate ran it: the settled lead-in is the weekly test's cost, and the rules read
/// the same on any network at this baseline.
#[test]
fn the_first_eight_trials_of_plasticity_everywhere_at_1024_units_and_the_rules_over_their_tables() {
    // The arms, their feedback and their assignment; the baseline and the bounds.
    assert_eq!(
        EVERYWHERE_REWARDED,
        [EVERYWHERE_ARMS[0], EVERYWHERE_ARMS[1]]
    );
    assert_eq!(EVERYWHERE_ARMS[WITHHELD], Everywhere::Withheld);
    assert_eq!(
        everywhere_feedback(Everywhere::Assignment),
        Feedback::Answer
    );
    assert_eq!(everywhere_feedback(Everywhere::Mirrored), Feedback::Answer);
    assert_eq!(
        everywhere_feedback(Everywhere::Withheld),
        Feedback::Withheld
    );
    assert!(
        !everywhere_mirrors(Everywhere::Assignment)
            && everywhere_mirrors(Everywhere::Mirrored)
            && !everywhere_mirrors(Everywhere::Withheld)
    );
    assert!(
        everywhere_rewards(Everywhere::Assignment)
            && everywhere_rewards(Everywhere::Mirrored)
            && !everywhere_rewards(Everywhere::Withheld)
    );
    assert_eq!(EVERYWHERE_BASELINE_Q16, 0x8000);
    assert_eq!(EVERYWHERE_TRIALS, REINFORCED_TRIALS);
    assert_eq!(SIGNAL_END_FLOOR_Q16, -46_691);
    assert_eq!(
        WRONG_AT_MOST_Q16,
        EVERYWHERE_BASELINE_Q16
            .saturating_add(SIGNAL_END_FIXED_Q16)
            .saturating_sub(ONE)
    );
    assert_eq!(
        ANSWER_AT_LEAST_Q16,
        EVERYWHERE_BASELINE_Q16
            .saturating_sub(SIGNAL_END_FIXED_Q16)
            .saturating_add(ONE)
    );
    assert_eq!(
        r0_stimulus(false),
        0,
        "A's answer is readout 0 in the assignment"
    );
    assert_eq!(r0_stimulus(true), 1, "B's in the mirrored");
    assert_eq!(answer_of(r0_stimulus(false) as u8, false), 0);
    assert_eq!(answer_of(r0_stimulus(true) as u8, true), 0);
    // The assertion's rule at its edges over trials written by hand.
    let trial = |reward: i32, end: i32, after: i32| -> EarnedTrial {
        (
            0,
            [0; 2],
            Some(0),
            reward > 0,
            reward,
            [[0; 2]; 2],
            end,
            after,
        )
    };
    assert_eq!(bounds(&[]), [true; 3]);
    let at_rest = trial(0, 0, 0);
    assert_eq!(
        bounds(&[at_rest]),
        [true; 3],
        "no reward, the signal at rest"
    );
    let top = trial(ONE, SIGNAL_END_FIXED_Q16, SIGNAL_END_FIXED_Q16 + ONE);
    let floor = trial(-ONE, SIGNAL_END_FLOOR_Q16, SIGNAL_END_FLOOR_Q16 - ONE);
    assert_eq!(
        bounds(&[top, floor]),
        [true; 3],
        "the fixed points are inside the bounds"
    );
    assert_eq!(
        bounds(&[trial(ONE, SIGNAL_END_FIXED_Q16 + 1, 0)]),
        [false, true, false],
        "one LSB above the ceiling"
    );
    assert_eq!(
        bounds(&[trial(-ONE, SIGNAL_END_FLOOR_Q16 - 1, 0)]),
        [false, false, true],
        "one LSB below the floor"
    );
    let wrong_edge = trial(-ONE, SIGNAL_END_FIXED_Q16, SIGNAL_END_FIXED_Q16 - ONE);
    assert_eq!(
        bounds(&[wrong_edge]),
        [true; 3],
        "a punishment from the ceiling leaves the wrong pair at exactly 0.2125"
    );
    let mut wrong_over = wrong_edge;
    wrong_over.7 = SIGNAL_END_FIXED_Q16 - ONE + 1;
    assert_eq!(bounds(&[wrong_over]), [true, false, true]);
    let answer_edge = trial(ONE, SIGNAL_END_FLOOR_Q16, SIGNAL_END_FLOOR_Q16 + ONE);
    assert_eq!(
        bounds(&[answer_edge]),
        [true; 3],
        "a reward from the floor leaves the answer pair at exactly 0.7875"
    );
    let mut answer_under = answer_edge;
    answer_under.7 = SIGNAL_END_FLOOR_Q16 + ONE - 1;
    assert_eq!(bounds(&[answer_under]), [true, true, false]);
    let mut positive_low = trial(ONE, 0, 0);
    positive_low.7 = 0;
    assert_eq!(
        bounds(&[positive_low]),
        [true, true, false],
        "a positive reward's clause reads the signal after it"
    );
    assert_eq!(
        bounds(&[trial(0, 0, SIGNAL_END_FIXED_Q16 - ONE)]),
        [true; 3],
        "no reward, no clause on the signal after"
    );
    // The drift's rule over tables written by hand.
    let image = [[10, 10], [20, 10]];
    let d = drift(&image, &[[11, 12], [20, 11]], [[3, 4, 0], [5, 5, 1]]);
    assert_eq!(
        d,
        Drift {
            rose: [[true, true], [false, true]],
            faster_onto_r1: [true, true],
            toward_r1: [true, false],
        }
    );
    assert_eq!(
        drift(&image, &[[12, 12], [22, 11]], [[0, 0, 8], [0, 0, 0]]).faster_onto_r1,
        [false, false],
        "equal ratios and a smaller one are not faster"
    );
    assert_eq!(
        drift(&image, &[[10, 10], [20, 10]], [[0; 3]; 2]),
        Drift {
            rose: [[false; 2]; 2],
            faster_onto_r1: [false; 2],
            toward_r1: [false; 2],
        },
        "the image itself"
    );
    assert_eq!(DRIFT_PREDICTED.rose, [[true; 2]; 2]);
    // Where a stimulus reaches its answer, and which is later, over tables written by hand.
    let block_with = |a_trials: u32| -> Block {
        (
            0,
            a_trials,
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
    let earned_with = |a: [u32; 3], b: [u32; 3]| -> EarnedBlock { ([a, b], 0, [[0; 2]; 2], 0, 0) };
    assert_eq!(reached(&[], &[], false), [None; 2]);
    let blocks = [block_with(40), block_with(40), block_with(40)];
    let earned = [
        earned_with([24, 16, 0], [9, 15, 0]),
        earned_with([25, 15, 0], [9, 14, 1]),
        earned_with([40, 0, 0], [0, 24, 0]),
    ];
    assert_eq!(
        reached(&blocks, &earned, false),
        [Some(2 * BLOCK), Some(BLOCK)],
        "A at 25 of 40 is 40 of 64 exactly, B at 15 of 24 too; 24 of 40 is under"
    );
    assert_eq!(
        reached(&blocks, &earned, true),
        [None, None],
        "mirrored, the answers are the other readouts"
    );
    let mirrored_earned = [
        earned_with([16, 24, 0], [15, 9, 0]),
        earned_with([15, 25, 0], [14, 9, 1]),
        earned_with([0, 40, 0], [24, 0, 0]),
    ];
    assert_eq!(
        reached(&blocks, &mirrored_earned, true),
        [Some(2 * BLOCK), Some(BLOCK)]
    );
    assert_eq!(
        reached(
            &[block_with(64)],
            &[earned_with([64, 0, 0], [0, 0, 0])],
            false
        ),
        [Some(BLOCK), None],
        "a block that presented no B reaches nothing for B"
    );
    assert!(r0_later([None, None], false));
    assert!(r0_later([None, Some(64)], false));
    assert!(!r0_later([Some(64), None], false));
    assert!(r0_later([Some(128), Some(64)], false));
    assert!(!r0_later([Some(64), Some(128)], false));
    assert!(
        !r0_later([Some(64), Some(64)], false),
        "together is not later"
    );
    assert!(
        r0_later([Some(64), Some(128)], true),
        "mirrored, B is the readout-0 stimulus"
    );
    assert!(!r0_later([Some(128), Some(64)], true));
    assert_eq!(R0_LATER_PREDICTED, [true; 2]);
    // Eight trials under the baseline at 0.5 on the instrument's network at 1 024 units: the
    // assignment, the oracle held at every trial inside `earned_run_under`; then the arena
    // against the weights before, moved inside the four pairs and outside them, the wrong
    // pairs consolidating, the bounds true; then the reward withheld, no reward and the signal
    // at rest, every pair consolidating.
    let p = prior(1024);
    let mut exec = at_gain(&p, config(1024, 2, EVERYWHERE_BASELINE_Q16), GAIN_1024);
    let before = weights_of(&exec);
    let run = earned_run_under(
        &mut exec,
        Feedback::Answer,
        false,
        1024,
        GATE_TRIALS,
        EVERYWHERE_BASELINE_Q16,
    );
    let (blocks, trace, trials, read, _) = &run;
    assert!(blocks.is_empty(), "eight trials are no whole block");
    assert_eq!(trials.len(), GATE_TRIALS);
    eprintln!("DUMP everywhere1024 first eight trace {trace:#018x} read {read:?}");
    let (inside, outside) = reach(&exec, &before, 1024, &ALL_PAIRS);
    eprintln!("DUMP everywhere1024 first eight reach inside {inside} outside {outside}");
    assert!(inside > 0, "the pairs consolidated");
    assert!(
        outside > 0,
        "and so did the arena outside them: every synapse consolidates under the baseline"
    );
    assert_eq!(bounds(read), [true; 3], "ADR-0082's bounds over the trials");
    assert_eq!(
        read[0].6, 0,
        "the signal at the first trial's end is at rest"
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
        read.iter().any(|t| t.5[0][1] != 0 || t.5[1][0] != 0),
        "a wrong pair consolidated under the baseline"
    );
    assert!(
        read.iter().any(|t| t.5[0][0] != 0 || t.5[1][1] != 0),
        "and an answer pair"
    );
    let mut withheld = at_gain(&p, config(1024, 2, EVERYWHERE_BASELINE_Q16), GAIN_1024);
    let before = weights_of(&withheld);
    let run = earned_run_under(
        &mut withheld,
        Feedback::Withheld,
        false,
        1024,
        GATE_TRIALS,
        EVERYWHERE_BASELINE_Q16,
    );
    let (_, trace, _, read, _) = &run;
    eprintln!("DUMP everywhere1024 first eight withheld trace {trace:#018x} read {read:?}");
    let (inside, outside) = reach(&withheld, &before, 1024, &ALL_PAIRS);
    assert!(
        inside > 0 && outside > 0,
        "every synapse consolidates with no reward"
    );
    assert!(
        read.iter().all(|t| t.4 == 0 && t.6 == 0 && t.7 == 0),
        "no reward, the signal at rest"
    );
    assert_eq!(bounds(read), [true; 3]);
    assert!(
        read.iter()
            .any(|t| t.5.iter().flatten().any(|&amount| amount != 0)),
        "the pairs consolidated under the baseline alone"
    );
    // Over the pinned tables: the verdict as written, by the rules; the correct selections,
    // the crossings, the splits, the bounds, the drift, where each stimulus reached its
    // answer and which was later, and the reach as the constants state, the two predicted
    // readings beside what was read; each arm's twenty-four blocks with the same trials
    // presenting A; every coupling above the image's in every block of every arm; each
    // block's earned readings consistent with its sight's block — the correct trials the
    // answer's splits, the ties the splits' ties, the signal after the block's last reward
    // the earned block's — and the withheld arm's rewards zero and its signal at rest.
    let verdict = reinforced([EVERYWHERE_BLOCKS_1024[0], EVERYWHERE_BLOCKS_1024[1]]);
    assert_eq!(verdict, EVERYWHERE_1024, "the verdict as written");
    assert!(!verdict.yes, "H-15 is no");
    assert_eq!(
        [
            last_correct(EVERYWHERE_BLOCKS_1024[0]),
            last_correct(EVERYWHERE_BLOCKS_1024[1]),
        ],
        CORRECT_LAST_EVERYWHERE_1024
    );
    assert!(CORRECT_LAST_EVERYWHERE_1024[0] < REWARDED_MIN);
    assert!(CORRECT_LAST_EVERYWHERE_1024[1] >= REWARDED_MIN);
    assert_eq!(
        [
            crossing(EVERYWHERE_BLOCKS_1024[0]),
            crossing(EVERYWHERE_BLOCKS_1024[1]),
            crossing(EVERYWHERE_BLOCKS_1024[2]),
        ],
        CROSSED_EVERYWHERE_1024
    );
    let withheld_end = EVERYWHERE_BLOCKS_1024[WITHHELD]
        .last()
        .map(|b| b.10)
        .expect("the withheld arm has a block");
    assert_eq!(
        drift(
            &IMAGE_COUPLINGS_1024,
            &withheld_end,
            SPLITS_LAST_EVERYWHERE_1024[WITHHELD]
        ),
        DRIFT_1024
    );
    assert_eq!(DRIFT_1024, DRIFT_PREDICTED, "the drift as predicted");
    assert_eq!(
        [
            reached(EVERYWHERE_BLOCKS_1024[0], EVERYWHERE_EARNED_1024[0], false),
            reached(EVERYWHERE_BLOCKS_1024[1], EVERYWHERE_EARNED_1024[1], true),
        ],
        REACHED_1024
    );
    assert_eq!(
        [
            r0_later(REACHED_1024[0], false),
            r0_later(REACHED_1024[1], true)
        ],
        R0_LATER_1024
    );
    assert_eq!(
        R0_LATER_1024, R0_LATER_PREDICTED,
        "the readout-0 stimulus later or not at all, as predicted"
    );
    let assignment = EVERYWHERE_BLOCKS_1024[0];
    for (k, &arm) in EVERYWHERE_ARMS.iter().enumerate() {
        let blocks = EVERYWHERE_BLOCKS_1024[k];
        let earned = EVERYWHERE_EARNED_1024[k];
        let compositions = EVERYWHERE_COMPOSITIONS_1024[k];
        assert_eq!(
            blocks.len(),
            EVERYWHERE_TRIALS / BLOCK,
            "{arm:?}: twenty-four blocks"
        );
        assert_eq!(earned.len(), EVERYWHERE_TRIALS / BLOCK);
        assert_eq!(compositions.len(), EVERYWHERE_TRIALS / BLOCK);
        assert_ne!(EVERYWHERE_TRACES_1024[k], 0);
        assert_ne!(EVERYWHERE_READ_1024[k], 0);
        assert!(!EVERYWHERE_CENSUS_1024[k].is_empty());
        assert!(
            EVERYWHERE_REACH_1024[k].1 > 0,
            "{arm:?}: the arena moved outside the pairs"
        );
        assert_eq!(BOUNDS_1024[k], [true; 3], "{arm:?}: the bounds");
        let mirrored = everywhere_mirrors(arm);
        let last_two = earned
            .iter()
            .rev()
            .take(LAST_BLOCKS)
            .fold([[0u32; 3]; 2], |mut sum, b| {
                for (s, split) in b.0.iter().enumerate() {
                    for (r, &n) in split.iter().enumerate() {
                        sum[s][r] = sum[s][r].saturating_add(n);
                    }
                }
                sum
            });
        assert_eq!(
            last_two, SPLITS_LAST_EVERYWHERE_1024[k],
            "{arm:?}: the last blocks' splits"
        );
        for (j, block) in blocks.iter().enumerate() {
            assert_eq!(
                block.1, assignment[j].1,
                "{arm:?} block {j}: the trials presenting A"
            );
            let presented = [block.1, BLOCK as u32 - block.1];
            for (s, &n) in presented.iter().enumerate() {
                assert_eq!(
                    earned[j].0[s].iter().sum::<u32>(),
                    n,
                    "{arm:?} block {j}: every presentation of {s} selected or tied"
                );
            }
            assert_eq!(
                block.11,
                earned[j].0[0][2] + earned[j].0[1][2],
                "{arm:?} block {j}: the ties"
            );
            assert_eq!(
                block.9, earned[j].4,
                "{arm:?} block {j}: the signal after the last reward"
            );
            assert!(SIGNAL_END_FLOOR_Q16 <= earned[j].3 && earned[j].3 <= SIGNAL_END_FIXED_Q16);
            assert!(compositions[j].3 <= BLOCK as u32);
            for &(s, r) in &ALL_PAIRS {
                assert!(
                    block.10[s][r] > IMAGE_COUPLINGS_1024[s][r],
                    "{arm:?} block {j}: the coupling {s}→{r} is above the image's"
                );
            }
            if everywhere_rewards(arm) {
                let correct =
                    earned[j].0[0][answer_of(0, mirrored)] + earned[j].0[1][answer_of(1, mirrored)];
                assert_eq!(block.0, correct, "{arm:?} block {j}: the correct trials");
                assert_eq!(
                    earned[j].1, correct,
                    "{arm:?} block {j}: one positive reward per correct trial"
                );
            } else {
                assert_eq!(
                    block.0,
                    earned[j].0[0][0] + earned[j].0[1][1],
                    "{arm:?} block {j}: correct reads the assignment's answers"
                );
                assert_eq!(earned[j].1, 0, "{arm:?} block {j}: no reward");
                assert_eq!(
                    (earned[j].3, earned[j].4),
                    (0, 0),
                    "{arm:?} block {j}: the signal at rest"
                );
            }
        }
    }
}
