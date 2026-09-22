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
const EVERYWHERE_BLOCKS_1024: [&[Block]; 3] = [&[], &[], &[]];
const EVERYWHERE_TRACES_1024: [u64; 3] = [0; 3];
const EVERYWHERE_COMPOSITIONS_1024: [&[Composition]; 3] = [&[], &[], &[]];
const EVERYWHERE_EARNED_1024: [&[EarnedBlock]; 3] = [&[], &[], &[]];
const EVERYWHERE_READ_1024: [u64; 3] = [0; 3];
const EVERYWHERE_CENSUS_1024: [&[(u32, u64)]; 3] = [&[], &[], &[]];
/// The verdict, by the rule committed first over the pinned tables.
const EVERYWHERE_1024: Reinforced = Reinforced {
    correct: [false, false],
    yes: false,
};
/// The correct selections over the last 128 trials, per rewarded arm, against `REWARDED_MIN`.
const CORRECT_LAST_EVERYWHERE_1024: [u32; 2] = [0; 2];
/// Where each arm's selection first passed 40 of 64 per block, as read, beside H-14's 448 and
/// 384; the withheld arm's correct trials read the assignment's answers.
const CROSSED_EVERYWHERE_1024: [Option<usize>; 3] = [None; 3];
/// The selections per stimulus over the last 128 trials, per arm, `[stimulus][readout 0,
/// readout 1, tie]`.
const SPLITS_LAST_EVERYWHERE_1024: [[[u32; 3]; 2]; 3] = [[[0; 3]; 2]; 3];
/// ADR-0082's assertion as read on each arm, clause by clause; the withheld arm's a reading.
const BOUNDS_1024: [[bool; 3]; 3] = [[false; 3]; 3];
/// The withheld arm's drift, as read, beside `DRIFT_PREDICTED`.
const DRIFT_1024: Drift = Drift {
    rose: [[false; 2]; 2],
    faster_onto_r1: [false; 2],
    toward_r1: [false; 2],
};
/// Where each stimulus reached its answer in each rewarded arm, as read.
const REACHED_1024: [[Option<usize>; 2]; 2] = [[None; 2]; 2];
/// Whether the stimulus whose answer is readout 0 reached its answer later in each rewarded
/// arm, as read, beside `R0_LATER_PREDICTED`.
const R0_LATER_1024: [bool; 2] = [false; 2];
/// The reach of each arm's run, `(inside the four pairs, outside)`, as read.
const EVERYWHERE_REACH_1024: [(u64, u64); 3] = [(0, 0); 3];

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
}
