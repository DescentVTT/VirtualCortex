//! Brief 039 (ADR-0087) runs H-16 as ADR-0085 wrote it: H-14's configuration — ADR-0077's
//! settled image, the gain 1.75, the controller off, ADR-0076's stimulus, the task as built
//! under `Delivery::Addressed`, the modulation baseline at zero, 1 536 trials — with the one
//! constant moved: the inhibitory baseline of ADR-0086 set at 0.5, so that every synapse of
//! an inhibitory block consolidates half of what it pairs at each presynaptic spike, the
//! dopamine term never reaching it, while every excitatory synapse stays under the reward's
//! gate. The settled engine is built as ADR-0077, ADR-0079, ADR-0081 and ADR-0083 build it
//! and held to ADR-0077's tables; its frozen image — the baseline zero, the inhibitory
//! baseline unset — is the calibration's, a frozen block from it held to ADR-0077's frozen
//! run before any rewarded run; the same image with the inhibitory baseline's flag and value
//! written into the modulator section is the arms'. Three arms of 1 536 trials from that
//! image under the task as built: the assignment and the mirrored assignment
//! (`Feedback::Answer`), and the reward withheld (`Feedback::Withheld`), a reading bounded by
//! no clause. ADR-0079's oracle consolidates every stimulus–readout synapse — all excitatory
//! — under the signal where the synapse is addressed and under zero elsewhere, H-14's rule,
//! held to the record at every trial. The criterion — the correct selections over the last
//! 128 trials at least 80 in both rewarded arms — the assertion (every excitatory synapse
//! outside the two answer pairs ends a rewarded arm as the image holds it, bit for bit, and
//! every excitatory synapse ends the withheld arm so) and ADR-0085's two predicted readings
//! (the inhibitory sum falling in every block; the withheld arm's selection where the frozen
//! network's is) are integer rules written before the run. The three arms are one weekly
//! `exhaustive` test; the gate runs the rules at their edges and eight trials with the
//! inhibitory baseline set.
//!
//! Brief 040 runs H-17 here as ADR-0089 wrote it and ADR-0090 amended its assertion: H-16's
//! configuration over 3 072 trials with the answer's mapping flipped once, between the
//! 1 536th trial and the 1 537th, by the task's `mirrored` and nothing else
//! (`earned_run_flipped` on the shared harness), from the same inhibited image after the same
//! calibration. Two arms, each its own weekly `exhaustive` test: the assignment first and the
//! mirrored first. The first half of each is H-16's arm of its first mapping bit for bit and
//! is held to H-16's tables; the second half is pinned. The criterion's two clauses (it
//! learned, it revised), ADR-0090's assertion (the old answer's pairs held from the 1 537th
//! trial, the pair the 1 536th did not carry from the 1 536th, no excitatory synapse outside
//! the four pairs moved) and the readings of the measured need are integer rules written
//! before the run; the gate runs them at their edges and a few trials over a flip.
//!
//! Brief 041 runs H-18 here as ADR-0093 wrote it and ADR-0095 amended its stopping rule:
//! H-17's configuration with ADR-0094's signed gate set — an addressed excitatory synapse
//! consolidating under the signal clamped to [−1, 1], a punishment moving its weight against
//! its trace and spending the trace — over 4 608 trials, the flip where H-17's was, from the
//! same settled engine's image with the gate's flag written (`signed_images`), after the same
//! calibration. Two arms, each its own weekly `exhaustive` test, pinned whole: the gate acts
//! in the first half too, so nothing of H-16 is replicated. The oracle consolidates by
//! `consolidated_signed` (`earned_run_signed`) and is held to the record at every trial; the
//! criterion's two clauses, the assertion and the readings — among them what each punished
//! trial's consolidation did to the addressed pair's synapses — are integer rules written
//! before the run, and the gate runs them at their edges and a few trials over a punishment
//! with the gate set beside the same trials with it unset.
//!
//! Brief 046 runs H-19 here as ADR-0106 wrote it: H-18's configuration, schedule and arms with
//! ADR-0107's critic set in the task — an expected reward per stimulus, zero at the start, the
//! reward delivered the outcome's less it, the expectation then moved by a thirty-second of
//! that error — from H-18's one signed image, after the same calibration and H-18's first
//! block reproduced with the critic unset. Two arms, each its own weekly `exhaustive` test,
//! pinned whole with each stimulus's expectation per block. The oracle is fed the error each
//! trial delivered (`earned_run_predicted`) and held to the record at every trial, and the
//! task's error and expectations to the harness's critic; the criterion's three clauses (it
//! learned, it revised, it settled), the assertion and the readings are integer rules written
//! before the run, and the gate runs them at their edges and a few trials with the critic set
//! beside the same trials with it unset.
//!
//! The harness is `tests/instrument.rs`'s, shared as one module and not copied (ADR-0083);
//! since ADR-0084 the weekly shards take tests, not binaries, so this binary's name steers
//! nothing.

#![deny(clippy::arithmetic_side_effects)]

// The harness — the network, the task, the readings, the oracle, every rule and every pinned
// table of ADR-0065 to ADR-0083 — is `tests/instrument.rs`'s module, compiled into this binary
// as well. What this binary does not call is that binary's, so the module's unused items and
// imports are allowed here and nowhere else in this file.
#[allow(dead_code, unused_imports)]
#[path = "instrument/harness.rs"]
mod harness;
use harness::*;

// ------------------------------------------- written before the run (ADR-0085, ADR-0086)

/// H-16's modulation baseline: H-14's zero, the reward's gate, under which an excitatory
/// synapse consolidates nothing but what a reward reaches.
const GATE_BASELINE_Q16: i32 = 0;
/// H-16's inhibitory baseline (ADR-0085): 0.5, brief 027's `BASELINE_Q16`, the value under
/// which ADR-0077's lead-in settled the network and H-15 read the inhibitory rule's course —
/// the one constant moved from H-14's configuration, and a parameter the image carries
/// (ADR-0086).
const INHIBITORY_BASELINE_Q16: i32 = BASELINE_Q16;
const _: () = assert!(GATE_BASELINE_Q16 == 0 && INHIBITORY_BASELINE_Q16 == ONE / 2);
/// The trials of an arm, H-14's: three of the instrument's runs, twenty-four blocks.
const INHIBITION_TRIALS: usize = REINFORCED_TRIALS;
const _: () = assert!(INHIBITION_TRIALS == 1_536 && INHIBITION_TRIALS / BLOCK == 24);

/// The arms of H-16 (ADR-0085), in the order run and no other, every one `Delivery::Addressed`
/// from the one image with the inhibitory baseline set: the assignment (`Feedback::Answer`,
/// A's answer readout 0 and B's readout 1), the mirrored assignment (`Feedback::Answer`,
/// `mirrored`), and the reward withheld (`Feedback::Withheld`: no reward, the signal at rest,
/// no excitatory synapse consolidating and every inhibitory one under its baseline), a
/// reading bounded by no clause. The shuffled reward is not run.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Inhibition {
    Assignment,
    Mirrored,
    Withheld,
}
const INHIBITION_ARMS: [Inhibition; 3] = [
    Inhibition::Assignment,
    Inhibition::Mirrored,
    Inhibition::Withheld,
];
/// The rewarded arms in `INHIBITION_ARMS`'s order, the criterion's `[assignment, mirrored]`.
const INHIBITION_REWARDED: [Inhibition; 2] = [Inhibition::Assignment, Inhibition::Mirrored];
/// The withheld arm's index in `INHIBITION_ARMS`.
const WITHHELD: usize = 2;

/// Where an arm's reward takes its sign from: the answer, or nothing.
fn inhibition_feedback(arm: Inhibition) -> Feedback {
    if arm == Inhibition::Withheld {
        Feedback::Withheld
    } else {
        Feedback::Answer
    }
}

/// Whether an arm mirrors the assignment.
fn inhibition_mirrors(arm: Inhibition) -> bool {
    arm == Inhibition::Mirrored
}

/// True for an arm whose reward carries the answer, the criterion's two.
fn inhibition_rewards(arm: Inhibition) -> bool {
    arm != Inhibition::Withheld
}

/// The pairs an arm's delivery can reach with the gate at zero: the two answer pairs where
/// the reward carries the answer (ADR-0080's derivation, which the inhibitory baseline does
/// not touch: a wrong selection's pair spends the next trial under a signal below zero and
/// consolidates nothing), and none with the reward withheld, where no excitatory synapse
/// consolidates at all; the four pairs name where the withheld arm's reach is counted.
fn inhibition_pairs(arm: Inhibition) -> Vec<(usize, usize)> {
    if inhibition_rewards(arm) {
        assigned_pairs(inhibition_mirrors(arm)).to_vec()
    } else {
        ALL_PAIRS.to_vec()
    }
}

/// ADR-0085's prediction, a Hypothesis written before the run: H-16 is yes.
const INHIBITION_PREDICTED: bool = true;

// ---------------------------------------------------- the assertion's shape (ADR-0085)

/// The reach of a run over the arena by polarity: the synapses whose weight differs from the
/// image's, counted inside the pairs named and outside them, for the excitatory synapses and
/// for the inhibitory ones. Every occupied slot is walked once, through its unit's chain; a
/// block's polarity is its presynaptic unit's flag (ADR-0049).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct Reach {
    excitatory: (u64, u64),
    inhibitory: (u64, u64),
}

fn reach_by_polarity(
    exec: &Engine,
    before: &[Vec<i16>],
    units: u32,
    pairs: &[(usize, usize)],
) -> Reach {
    let sets = geometry(units, rotation(units));
    let mut out = Reach::default();
    for unit in exec.units() {
        let id = unit.id as u32;
        let inhibitory = unit.flags & FLAG_INHIBITORY != 0;
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
            let into = if inhibitory {
                &mut out.inhibitory
            } else {
                &mut out.excitatory
            };
            if in_pair {
                into.0 = into.0.saturating_add(1);
            } else {
                into.1 = into.1.saturating_add(1);
            }
        }
    }
    out
}

/// ADR-0085's assertion as a rule over an arm's reach, its pairs the ones `inhibition_pairs`
/// names: in a rewarded arm no excitatory synapse outside the two answer pairs moved; in the
/// withheld arm no excitatory synapse moved at all, inside the four pairs or outside them.
/// The inhibitory synapses are outside the clause: that they move is ADR-0085's rule, read
/// beside it. True on every arm is the assertion; false on one is a finding against
/// ADR-0080's derivation as ADR-0085 extends it, reported beside the verdict and not in
/// place of it.
fn excitatory_held(reach: &Reach, rewarded: bool) -> bool {
    reach.excitatory.1 == 0 && (rewarded || reach.excitatory.0 == 0)
}

// ------------------------------------------ the predicted readings' shape (ADR-0085)

/// ADR-0085's predicted reading (a): the arena's inhibitory sum falls in every block, as it
/// did under H-15 — each block's sum below the block's before, the first below the image's;
/// false for a run of no block.
fn falls_every_block(image: i64, blocks: &[Block]) -> bool {
    let mut previous = image;
    for block in blocks {
        if block.7 >= previous {
            return false;
        }
        previous = block.7;
    }
    !blocks.is_empty()
}

/// A Hypothesis written before the run and never asserted: the inhibitory sum falls in every
/// block of every arm, `[assignment, mirrored, withheld]`.
const FALLS_PREDICTED: [bool; 3] = [true; 3];

/// A sum as a fraction of the image's, in parts per ten thousand; zero against an image sum
/// of zero, which no image of this network has.
fn per_myriad(sum: i64, image: i64) -> i64 {
    sum.saturating_mul(10_000).checked_div(image).unwrap_or(0)
}

/// The readout each stimulus selected more often, `[A, B]`, from the selections per stimulus
/// (`[stimulus][readout 0, readout 1, tie]`): the sign of the two readouts' counts as
/// `selected` reads it, none at equal counts.
fn lean(splits: [[u32; 3]; 2]) -> [Option<u8>; 2] {
    [
        selected([splits[0][0], splits[0][1]]),
        selected([splits[1][0], splits[1][1]]),
    ]
}

/// The frozen network's lean: the readout each stimulus selected more often over ADR-0077's
/// frozen block of the settled candidate (`BACKGROUND_COUNTED_1024[SETTLED]`, the sixty-four
/// trials the calibration reproduces), each trial's selection the sign of its counts as the
/// task makes it.
fn frozen_lean() -> [Option<u8>; 2] {
    let mut splits = [[0u32; 3]; 2];
    for &(stimulus, counts) in &BACKGROUND_COUNTED_1024[SETTLED] {
        let into = &mut splits[usize::from(stimulus)][selected(counts).map_or(2, usize::from)];
        *into = into.saturating_add(1);
    }
    lean(splits)
}

/// The frozen network's lean as the pinned table gives it, computed by `frozen_lean` before
/// the run and held to it in the gate: readout 1 for both stimuli, by one selection for A
/// (14 to 15 with 5 ties) and three for B (12 to 15 with 3 ties).
const FROZEN_LEAN_1024: [Option<u8>; 2] = [Some(1), Some(1)];

/// ADR-0085's predicted reading (b), a Hypothesis written before the run and never asserted:
/// the withheld arm's excitatory weights stay the image's, so its selection stays where the
/// frozen network's is — the lean of its last 128 trials the frozen network's lean.
const LEAN_AS_FROZEN_PREDICTED: bool = true;

// ---------------------------------------------------------------- the run (brief 039)

/// The settled image with the inhibitory baseline set (ADR-0086): ADR-0077's frozen image
/// — the baseline patched to zero, the inhibitory baseline unset — with the modulator
/// section's flag at `[24]` set and its value at `[28..32)` written 0.5, the section
/// re-sealed; every other byte the frozen image's.
fn inhibited_image(zero: &[u8]) -> Vec<u8> {
    let mut img = zero.to_vec();
    let header = CortexFileHeader::decode(img[0..64].try_into().unwrap());
    for at in (64..).step_by(64).take(header.section_count as usize) {
        let mut entry = SectionEntry::decode(img[at..][..64].try_into().unwrap());
        if entry.kind == SECTION_MODULATOR {
            let (offset, length) = (entry.offset as usize, entry.length as usize);
            img[offset..][24] = 1;
            img[offset..][28..32].copy_from_slice(&INHIBITORY_BASELINE_Q16.to_le_bytes());
            entry.crc64 = crc64(&img[offset..][..length]);
            img[at..][..64].copy_from_slice(&entry.encode());
            return img;
        }
    }
    panic!("the image holds a modulator section");
}

/// The engine from the inhibited image, decoded under the calibration's configuration (two
/// workers, a train that holds a trial, the baseline zero, the inhibitory baseline unset):
/// the image's baseline, inhibitory baseline, gain and step outrank the configuration's
/// (§8.3); the baseline asserted zero and the inhibitory baseline 0.5.
fn inhibited_from(image: &[u8], units: u32) -> Engine {
    let exec = Image::decode::<2048>(image, config(units, 2, 0)).expect("a well-formed record");
    assert_eq!(exec.modulation_baseline_q16(), 0, "the gate");
    assert_eq!(
        exec.inhibitory_baseline_q16(),
        Some(INHIBITORY_BASELINE_Q16),
        "the image carries the inhibitory baseline"
    );
    exec
}

/// The two images of the one settled engine: ADR-0077's frozen image, held as
/// `settled_image` holds it, for the calibration; and the inhibited image for the arms —
/// decoded and asserted to carry the baseline zero, the inhibitory baseline 0.5, the sums,
/// the gain and the step, and to resume the clock where the image was written. The two
/// differ in the modulator section's flag byte, the one byte of the value that is not zero
/// and the section's CRC, and nowhere else.
fn inhibited_images(name: &str) -> (Vec<u8>, Vec<u8>) {
    let (exec, quieted) = settled_engine(name);
    let zero = frozen_image_checked(name, &exec, quieted);
    let inhibited = inhibited_image(&zero);
    let decoded = inhibited_from(&inhibited, 1024);
    assert_eq!(
        weights_by_polarity(&decoded),
        quieted,
        "{name}: the inhibited image carries the weights"
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
    assert_eq!(zero.len(), inhibited.len(), "{name}: one image, twice");
    let differing = zero
        .iter()
        .zip(inhibited.iter())
        .filter(|(a, b)| a != b)
        .count();
    assert!(
        differing > 0 && differing <= 10,
        "{name}: the images differ in the flag, the value's one byte and the section's CRC: {differing} bytes"
    );
    eprintln!(
        "DUMP {name} images {} bytes, {differing} differ, written at tick {}",
        inhibited.len(),
        exec.ticks()
    );
    (zero, inhibited)
}

/// An arm's run from the inhibited engine: `earned_run_under` with the arm's feedback and
/// assignment at the gate's zero, H-14's rule for every excitatory synapse — the oracle
/// consolidating every stimulus–readout synapse under the signal where the synapse is
/// addressed and under zero elsewhere, held to the record at every trial — while the
/// inhibitory synapses consolidate under their own baseline, which the oracle does not
/// model and the readings read from the record; under the withheld arm the reward is zero
/// and the signal stays at rest, asserted trial by trial.
fn inhibition_run(exec: &mut Engine, arm: Inhibition, trials: usize) -> EarnedRun {
    earned_run_under(
        exec,
        inhibition_feedback(arm),
        inhibition_mirrors(arm),
        1024,
        trials,
        GATE_BASELINE_Q16,
    )
}

/// The inhibitory sum after each block as a fraction of the image's, in parts per ten
/// thousand: the course a run's blocks hold, read beside H-15's.
fn course(image: i64, blocks: &[Block]) -> Vec<i64> {
    blocks.iter().map(|b| per_myriad(b.7, image)).collect()
}

/// Holds an arm's run to its pinned tables: the blocks and the trace, the composition per
/// block, the earned blocks, the readings' hash and the census.
fn pinned_inhibition(name: &str, k: usize, run: &EarnedRun, earned: &[EarnedBlock]) {
    let (blocks, trace, trials, read, volley_ticks) = run;
    pinned(
        &format!("{name} sight"),
        blocks,
        *trace,
        INHIBITION_BLOCKS_1024[k],
        INHIBITION_TRACES_1024[k],
    );
    let compositions: Vec<Composition> = trials.chunks(BLOCK).map(composition).collect();
    assert_eq!(
        compositions.as_slice(),
        INHIBITION_COMPOSITIONS_1024[k],
        "{name}: the composition per block"
    );
    assert_eq!(
        earned, INHIBITION_EARNED_1024[k],
        "{name}: the earned blocks"
    );
    assert_eq!(
        earned_hash(read),
        INHIBITION_READ_1024[k],
        "{name}: the readings"
    );
    assert_eq!(
        census_of(volley_ticks),
        INHIBITION_CENSUS_1024[k].to_vec(),
        "{name}: the volley's ticks"
    );
}

/// H-16 at 1 024 units (brief 039): the settled engine, held to ADR-0077 step by step, and
/// its two images; the calibration — a frozen block from the zero image, the inhibitory
/// baseline unset, the reward withheld, held to ADR-0077's frozen run — before any rewarded
/// run (H-16's stopping rule, step 2); then the assignment, the mirrored assignment and the
/// reward withheld, each `INHIBITION_TRIALS` trials from the inhibited image under the
/// task's own delivery, each dumped and the verdict and every reading computed by the rules
/// before anything is held; then the assertion — every excitatory synapse outside the pairs
/// the reward can reach the image's, bit for bit, on every arm — and the pinned tables.
#[test]
#[ignore]
fn inhibition_off_the_gate_at_1024_units_exhaustive() {
    let name = "inhibition1024";
    let (zero, inhibited) = inhibited_images(name);
    {
        let mut frozen = frozen_from(&zero, 1024);
        assert_eq!(
            frozen.inhibitory_baseline_q16(),
            None,
            "{name}: the calibration's image leaves the inhibitory baseline unset"
        );
        let calibration = taught_run(&mut frozen, Arm::Withheld, 1024, BLOCK);
        calibration_holds(&format!("{name} calibration"), &calibration);
        eprintln!("DUMP {name} calibration holds: ADR-0077's settled candidate reproduced");
    }
    let sets = geometry(1024, ROTATION_1024);
    let image_sums = QUIET_1024[SETTLED].1;
    let mut runs: Vec<EarnedRun> = Vec::with_capacity(INHIBITION_ARMS.len());
    let mut reaches = [Reach::default(); 3];
    let mut sums_after = [(0i64, 0i64); 3];
    for (k, &arm) in INHIBITION_ARMS.iter().enumerate() {
        let arm_name = format!("{name} {arm:?}");
        let mut exec = inhibited_from(&inhibited, 1024);
        assert_eq!(
            weights_by_polarity(&exec),
            image_sums,
            "{arm_name}: the image's sums"
        );
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
        let run = inhibition_run(&mut exec, arm, INHIBITION_TRIALS);
        let reach = reach_by_polarity(&exec, &before, 1024, &inhibition_pairs(arm));
        sums_after[k] = weights_by_polarity(&exec);
        eprintln!(
            "DUMP {arm_name} reach {reach:?} sums after the run {:?} signal {:#x}",
            sums_after[k],
            exec.modulator().dopamine_rpe
        );
        reaches[k] = reach;
        runs.push(run);
    }
    // Every arm dumped and the verdict and the readings computed before anything is held.
    let mut tables: Vec<Vec<EarnedBlock>> = Vec::with_capacity(INHIBITION_ARMS.len());
    for (k, &arm) in INHIBITION_ARMS.iter().enumerate() {
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
    let derivation_read = [
        derivation(&runs[0].3),
        derivation(&runs[1].3),
        derivation(&runs[2].3),
    ];
    let falls_read = [
        falls_every_block(image_sums.0, &runs[0].0),
        falls_every_block(image_sums.0, &runs[1].0),
        falls_every_block(image_sums.0, &runs[2].0),
    ];
    let courses = [
        course(image_sums.0, &runs[0].0),
        course(image_sums.0, &runs[1].0),
        course(image_sums.0, &runs[2].0),
    ];
    let lean_read = lean(last[WITHHELD]);
    let frozen = frozen_lean();
    let held = [
        excitatory_held(&reaches[0], true),
        excitatory_held(&reaches[1], true),
        excitatory_held(&reaches[2], false),
    ];
    eprintln!(
        "DUMP {name} verdict {verdict:?} predicted {INHIBITION_PREDICTED} correct last {correct_last:?} crossed {crossed:?} (H-14's {:?}) splits last {last:?} derivation {derivation_read:?} falls {falls_read:?} predicted {FALLS_PREDICTED:?} courses {courses:?} lean {lean_read:?} frozen {frozen:?} predicted as frozen {LEAN_AS_FROZEN_PREDICTED} held {held:?} reach {reaches:?} sums after {sums_after:?} image {image_sums:?}",
        &CROSSED_1024[..2]
    );
    // The assertion (ADR-0085's), after the verdict and beside it: every excitatory synapse
    // outside the pairs the reward can reach is the image's, on every arm.
    for (k, &arm) in INHIBITION_ARMS.iter().enumerate() {
        assert!(
            held[k],
            "{arm:?}: ADR-0085's assertion — every excitatory synapse outside the pairs the reward reaches ends the run as the image holds it: {:?}",
            reaches[k]
        );
    }
    for (k, &arm) in INHIBITION_ARMS.iter().enumerate() {
        pinned_inhibition(&format!("{name} {arm:?}"), k, &runs[k], &tables[k]);
    }
    assert_eq!(verdict, INHIBITION_1024, "the verdict as written");
    assert_eq!(correct_last, CORRECT_LAST_INHIBITION_1024);
    assert_eq!(crossed, CROSSED_INHIBITION_1024);
    assert_eq!(last, SPLITS_LAST_INHIBITION_1024);
    assert_eq!(derivation_read, DERIVATION_INHIBITION_1024);
    assert_eq!(falls_read, FALLS_1024);
    assert_eq!(lean_read, LEAN_1024);
    assert_eq!(frozen, FROZEN_LEAN_1024);
    assert_eq!(reaches, REACH_INHIBITION_1024);
    assert_eq!(sums_after, SUMS_AFTER_1024);
}

// ----------------------------------------------------------- the measurement (brief 039)

/// The three arms at 1 024 units, in `INHIBITION_ARMS`'s order, each pinned from one run:
/// the sight's blocks and trace, the composition per block, the earned blocks, the readings'
/// hash and the volley's census. Empty until the run: the constants above are committed
/// before the first rewarded run, and the tables after it.
const INHIBITION_BLOCKS_1024: [&[Block]; 3] = [
    &[
        (
            31,
            34,
            [[335, 321], [315, 321]],
            [1728, 1525],
            [2494, 2254],
            [258, 258],
            62,
            161656719,
            218307591,
            20605,
            [[6257422, 6698611], [6584205, 6871837]],
            8,
        ),
        (
            32,
            31,
            [[303, 314], [331, 347]],
            [1574, 1677],
            [2398, 2399],
            [241, 229],
            64,
            157389360,
            218367021,
            108546,
            [[6271975, 6698611], [6584205, 6916714]],
            5,
        ),
        (
            27,
            31,
            [[301, 330], [343, 370]],
            [1570, 1679],
            [2381, 2384],
            [239, 228],
            63,
            153111101,
            218464997,
            -34256,
            [[6313630, 6698611], [6584205, 6973035]],
            4,
        ),
        (
            29,
            28,
            [[271, 340], [357, 427]],
            [1427, 1829],
            [2255, 2568],
            [251, 253],
            64,
            148733892,
            218545219,
            103139,
            [[6315375, 6698611], [6584205, 7051512]],
            7,
        ),
        (
            35,
            30,
            [[310, 347], [328, 407]],
            [1524, 1724],
            [2326, 2393],
            [252, 259],
            63,
            144450836,
            218686064,
            93851,
            [[6361822, 6698611], [6584205, 7145910]],
            5,
        ),
        (
            39,
            36,
            [[404, 375], [298, 366]],
            [1829, 1424],
            [2655, 2113],
            [263, 243],
            64,
            140054228,
            218849014,
            46770,
            [[6461308, 6698611], [6584205, 7209374]],
            4,
        ),
        (
            38,
            30,
            [[350, 340], [382, 444]],
            [1520, 1728],
            [2322, 2432],
            [254, 258],
            64,
            135770434,
            219006559,
            46670,
            [[6536434, 6698611], [6584205, 7291793]],
            7,
        ),
        (
            42,
            32,
            [[419, 359], [325, 438]],
            [1628, 1628],
            [2446, 2369],
            [264, 242],
            64,
            131434745,
            219162131,
            -82047,
            [[6607456, 6698611], [6584205, 7376343]],
            7,
        ),
        (
            49,
            34,
            [[479, 363], [360, 457]],
            [1729, 1524],
            [2553, 2230],
            [261, 232],
            64,
            127124478,
            219435187,
            112153,
            [[6761685, 6698611], [6584205, 7495170]],
            2,
        ),
        (
            50,
            28,
            [[384, 307], [357, 583]],
            [1424, 1831],
            [2293, 2542],
            [250, 230],
            64,
            122889007,
            219658927,
            47376,
            [[6839889, 6698611], [6584205, 7640706]],
            1,
        ),
        (
            59,
            33,
            [[542, 379], [309, 542]],
            [1676, 1578],
            [2468, 2287],
            [235, 244],
            64,
            118650445,
            219883501,
            61055,
            [[6933393, 6698611], [6584205, 7771776]],
            1,
        ),
        (
            57,
            32,
            [[509, 364], [345, 563]],
            [1625, 1629],
            [2483, 2387],
            [255, 251],
            64,
            114310324,
            220111044,
            112227,
            [[7062900, 6698611], [6584205, 7869812]],
            2,
        ),
        (
            56,
            32,
            [[530, 350], [340, 609]],
            [1631, 1627],
            [2446, 2357],
            [252, 221],
            64,
            110242202,
            220319838,
            112203,
            [[7162490, 6698611], [6584205, 7979016]],
            3,
        ),
        (
            59,
            33,
            [[664, 385], [328, 591]],
            [1676, 1579],
            [2527, 2354],
            [270, 249],
            64,
            106119010,
            220582532,
            112219,
            [[7315271, 6698611], [6584205, 8088929]],
            2,
        ),
        (
            61,
            37,
            [[735, 423], [307, 586]],
            [1882, 1375],
            [2693, 2159],
            [261, 263],
            64,
            101919828,
            220819803,
            112225,
            [[7472448, 6698611], [6584205, 8169023]],
            1,
        ),
        (
            60,
            35,
            [[704, 432], [345, 634]],
            [1782, 1472],
            [2568, 2242],
            [251, 262],
            64,
            98017420,
            221068842,
            110207,
            [[7620624, 6698611], [6584205, 8269886]],
            0,
        ),
        (
            61,
            33,
            [[752, 417], [361, 680]],
            [1678, 1572],
            [2531, 2250],
            [256, 265],
            64,
            94166126,
            221328035,
            111555,
            [[7758534, 6698611], [6584205, 8391169]],
            2,
        ),
        (
            62,
            38,
            [[852, 458], [312, 617]],
            [1932, 1322],
            [2736, 2054],
            [265, 268],
            64,
            90259628,
            221569234,
            112227,
            [[7917931, 6698611], [6584205, 8472971]],
            1,
        ),
        (
            64,
            31,
            [[734, 384], [406, 794]],
            [1578, 1680],
            [2409, 2425],
            [253, 279],
            64,
            86628912,
            221804712,
            112227,
            [[8048314, 6698611], [6584205, 8578066]],
            0,
        ),
        (
            64,
            34,
            [[894, 455], [382, 810]],
            [1726, 1524],
            [2489, 2290],
            [292, 283],
            64,
            83110554,
            222099438,
            112227,
            [[8176148, 6698611], [6584205, 8744958]],
            0,
        ),
        (
            64,
            30,
            [[807, 412], [400, 930]],
            [1524, 1730],
            [2316, 2472],
            [288, 240],
            64,
            79520179,
            222380783,
            112227,
            [[8295135, 6698611], [6584205, 8907316]],
            0,
        ),
        (
            64,
            29,
            [[806, 362], [424, 1012]],
            [1471, 1781],
            [2310, 2490],
            [265, 255],
            64,
            76137852,
            222649565,
            112227,
            [[8380581, 6698611], [6584205, 9090652]],
            0,
        ),
        (
            64,
            32,
            [[881, 407], [377, 933]],
            [1627, 1628],
            [2447, 2416],
            [305, 260],
            64,
            72813343,
            222791682,
            112227,
            [[8407678, 6698611], [6584205, 9205672]],
            0,
        ),
        (
            64,
            29,
            [[867, 416], [467, 1058]],
            [1469, 1779],
            [2286, 2500],
            [265, 267],
            64,
            69725710,
            222954146,
            112227,
            [[8475315, 6698611], [6584205, 9300499]],
            0,
        ),
    ],
    &[
        (
            28,
            34,
            [[329, 326], [319, 315]],
            [1728, 1525],
            [2494, 2254],
            [258, 257],
            62,
            161663952,
            218322085,
            -41361,
            [[6249552, 6754599], [6606948, 6815470]],
            6,
        ),
        (
            31,
            31,
            [[285, 328], [338, 333]],
            [1574, 1677],
            [2396, 2401],
            [240, 229],
            64,
            157351628,
            218447206,
            -110127,
            [[6249552, 6833307], [6653361, 6815470]],
            6,
        ),
        (
            37,
            31,
            [[291, 358], [359, 347]],
            [1570, 1679],
            [2380, 2381],
            [237, 228],
            64,
            153016601,
            218590615,
            34260,
            [[6249552, 6915233], [6714844, 6815470]],
            7,
        ),
        (
            40,
            28,
            [[252, 381], [393, 382]],
            [1427, 1829],
            [2252, 2567],
            [248, 252],
            64,
            148539293,
            218712530,
            34826,
            [[6249552, 6978845], [6773147, 6815470]],
            7,
        ),
        (
            38,
            30,
            [[283, 418], [366, 361]],
            [1524, 1724],
            [2328, 2390],
            [250, 252],
            63,
            144140934,
            218936105,
            -81964,
            [[6249552, 7129171], [6846396, 6815470]],
            6,
        ),
        (
            45,
            36,
            [[352, 486], [360, 314]],
            [1828, 1424],
            [2647, 2110],
            [263, 242],
            64,
            139615041,
            219177856,
            110197,
            [[6249552, 7271418], [6945900, 6815470]],
            2,
        ),
        (
            52,
            30,
            [[283, 470], [453, 367]],
            [1520, 1728],
            [2318, 2427],
            [253, 264],
            64,
            135191810,
            219366321,
            -20865,
            [[6249552, 7377367], [7028416, 6815470]],
            4,
        ),
        (
            47,
            32,
            [[314, 507], [438, 353]],
            [1629, 1628],
            [2444, 2367],
            [264, 242],
            64,
            130715066,
            219578188,
            94752,
            [[6249552, 7511784], [7105866, 6815470]],
            3,
        ),
        (
            53,
            34,
            [[356, 552], [453, 337]],
            [1729, 1524],
            [2549, 2230],
            [261, 241],
            64,
            126150014,
            219906891,
            94749,
            [[6249552, 7737849], [7208504, 6815470]],
            3,
        ),
        (
            51,
            28,
            [[262, 528], [489, 418]],
            [1424, 1831],
            [2291, 2537],
            [251, 231],
            64,
            121703793,
            220154428,
            112227,
            [[6249552, 7889189], [7304701, 6815470]],
            0,
        ),
        (
            54,
            33,
            [[353, 703], [459, 353]],
            [1675, 1578],
            [2461, 2284],
            [235, 246],
            64,
            117249005,
            220439154,
            112227,
            [[6249552, 8060362], [7418254, 6815470]],
            1,
        ),
        (
            58,
            32,
            [[303, 680], [535, 384]],
            [1625, 1629],
            [2477, 2385],
            [251, 251],
            64,
            112711628,
            220726062,
            112203,
            [[6249552, 8268225], [7497299, 6815470]],
            1,
        ),
        (
            59,
            32,
            [[295, 750], [543, 376]],
            [1631, 1627],
            [2437, 2354],
            [254, 221],
            64,
            108462384,
            221026007,
            111749,
            [[6249552, 8467310], [7598159, 6815470]],
            3,
        ),
        (
            61,
            33,
            [[362, 821], [513, 339]],
            [1676, 1579],
            [2519, 2346],
            [259, 258],
            64,
            104079350,
            221256005,
            111555,
            [[6249552, 8618571], [7676896, 6815470]],
            1,
        ),
        (
            61,
            37,
            [[376, 960], [509, 330]],
            [1883, 1375],
            [2683, 2155],
            [259, 268],
            64,
            99564046,
            221499119,
            112227,
            [[6249552, 8805986], [7732595, 6815470]],
            1,
        ),
        (
            64,
            35,
            [[345, 959], [569, 360]],
            [1782, 1472],
            [2566, 2242],
            [255, 270],
            64,
            95480142,
            221750342,
            112227,
            [[6249552, 8952782], [7837022, 6815470]],
            0,
        ),
        (
            62,
            33,
            [[373, 971], [601, 329]],
            [1678, 1572],
            [2521, 2252],
            [253, 262],
            64,
            91518036,
            221986426,
            109938,
            [[6249552, 9099868], [7926020, 6815470]],
            1,
        ),
        (
            63,
            38,
            [[403, 1100], [557, 319]],
            [1932, 1322],
            [2728, 2054],
            [263, 261],
            64,
            87582825,
            222212780,
            110207,
            [[6249552, 9246527], [8005715, 6815470]],
            1,
        ),
        (
            61,
            31,
            [[333, 974], [712, 390]],
            [1578, 1680],
            [2401, 2418],
            [243, 290],
            64,
            83944503,
            222442113,
            112227,
            [[6249552, 9349684], [8131891, 6815470]],
            0,
        ),
        (
            64,
            34,
            [[382, 1103], [702, 402]],
            [1726, 1524],
            [2482, 2299],
            [287, 285],
            64,
            80491269,
            222685256,
            112227,
            [[6249552, 9465754], [8258964, 6815470]],
            0,
        ),
        (
            63,
            30,
            [[335, 1015], [759, 422]],
            [1523, 1730],
            [2295, 2480],
            [279, 238],
            64,
            77045960,
            222953380,
            112227,
            [[6249552, 9557724], [8435118, 6815470]],
            1,
        ),
        (
            64,
            29,
            [[284, 968], [839, 442]],
            [1471, 1781],
            [2307, 2497],
            [253, 255],
            64,
            73845853,
            223131644,
            112227,
            [[6249552, 9617028], [8554078, 6815470]],
            0,
        ),
        (
            64,
            32,
            [[341, 1087], [828, 401]],
            [1627, 1627],
            [2448, 2427],
            [296, 251],
            64,
            70725808,
            223360447,
            112227,
            [[6249552, 9690664], [8709245, 6815470]],
            0,
        ),
        (
            64,
            29,
            [[351, 1005], [1003, 429]],
            [1470, 1779],
            [2277, 2511],
            [278, 267],
            64,
            67783115,
            223533834,
            112227,
            [[6249552, 9743362], [8829934, 6815470]],
            0,
        ),
    ],
    &[
        (
            29,
            34,
            [[328, 322], [315, 314]],
            [1728, 1525],
            [2494, 2253],
            [258, 257],
            62,
            161672028,
            218243354,
            0,
            [[6249552, 6698611], [6584205, 6815470]],
            8,
        ),
        (
            29,
            31,
            [[285, 310], [328, 333]],
            [1574, 1677],
            [2398, 2400],
            [241, 229],
            64,
            157400416,
            218243354,
            0,
            [[6249552, 6698611], [6584205, 6815470]],
            7,
        ),
        (
            25,
            31,
            [[288, 328], [337, 343]],
            [1570, 1679],
            [2384, 2382],
            [237, 228],
            63,
            153125786,
            218243354,
            0,
            [[6249552, 6698611], [6584205, 6815470]],
            6,
        ),
        (
            23,
            28,
            [[251, 338], [351, 378]],
            [1427, 1829],
            [2258, 2566],
            [248, 253],
            64,
            148746838,
            218243354,
            0,
            [[6249552, 6698611], [6584205, 6815470]],
            8,
        ),
        (
            29,
            30,
            [[280, 342], [324, 356]],
            [1524, 1724],
            [2325, 2394],
            [248, 256],
            63,
            144470108,
            218243354,
            0,
            [[6249552, 6698611], [6584205, 6815470]],
            6,
        ),
        (
            28,
            36,
            [[348, 371], [292, 310]],
            [1828, 1424],
            [2652, 2109],
            [263, 241],
            64,
            140088908,
            218243354,
            0,
            [[6249552, 6698611], [6584205, 6815470]],
            7,
        ),
        (
            28,
            30,
            [[280, 334], [361, 348]],
            [1520, 1728],
            [2321, 2431],
            [249, 258],
            62,
            135854517,
            218243354,
            0,
            [[6249552, 6698611], [6584205, 6815470]],
            4,
        ),
        (
            26,
            32,
            [[309, 342], [318, 335]],
            [1629, 1628],
            [2442, 2368],
            [259, 242],
            64,
            131543574,
            218243354,
            0,
            [[6249552, 6698611], [6584205, 6815470]],
            6,
        ),
        (
            25,
            34,
            [[354, 343], [335, 322]],
            [1729, 1524],
            [2555, 2231],
            [256, 232],
            63,
            127252105,
            218243354,
            0,
            [[6249552, 6698611], [6584205, 6815470]],
            10,
        ),
        (
            31,
            28,
            [[256, 295], [333, 394]],
            [1424, 1831],
            [2297, 2536],
            [249, 224],
            64,
            123076929,
            218243354,
            0,
            [[6249552, 6698611], [6584205, 6815470]],
            4,
        ),
        (
            33,
            33,
            [[349, 363], [269, 329]],
            [1675, 1578],
            [2462, 2289],
            [238, 240],
            63,
            118901830,
            218243354,
            0,
            [[6249552, 6698611], [6584205, 6815470]],
            4,
        ),
        (
            34,
            32,
            [[293, 336], [309, 354]],
            [1625, 1629],
            [2471, 2391],
            [249, 243],
            63,
            114618072,
            218243354,
            0,
            [[6249552, 6698611], [6584205, 6815470]],
            2,
        ),
        (
            28,
            32,
            [[287, 332], [314, 341]],
            [1631, 1627],
            [2437, 2355],
            [250, 221],
            64,
            110595270,
            218243354,
            0,
            [[6249552, 6698611], [6584205, 6815470]],
            4,
        ),
        (
            37,
            33,
            [[357, 356], [290, 310]],
            [1676, 1579],
            [2523, 2349],
            [263, 250],
            63,
            106544286,
            218243354,
            0,
            [[6249552, 6698611], [6584205, 6815470]],
            6,
        ),
        (
            38,
            37,
            [[366, 384], [258, 317]],
            [1882, 1375],
            [2687, 2158],
            [251, 254],
            64,
            102373707,
            218243354,
            0,
            [[6249552, 6698611], [6584205, 6815470]],
            5,
        ),
        (
            28,
            35,
            [[320, 384], [299, 323]],
            [1782, 1472],
            [2561, 2233],
            [243, 258],
            64,
            98492554,
            218243354,
            0,
            [[6249552, 6698611], [6584205, 6815470]],
            5,
        ),
        (
            33,
            33,
            [[367, 367], [296, 299]],
            [1678, 1572],
            [2522, 2240],
            [250, 256],
            64,
            94649021,
            218243354,
            0,
            [[6249552, 6698611], [6584205, 6815470]],
            3,
        ),
        (
            28,
            38,
            [[387, 395], [273, 272]],
            [1932, 1322],
            [2728, 2056],
            [254, 249],
            62,
            90679509,
            218243354,
            0,
            [[6249552, 6698611], [6584205, 6815470]],
            3,
        ),
        (
            30,
            31,
            [[309, 326], [343, 348]],
            [1578, 1680],
            [2408, 2414],
            [239, 276],
            63,
            86944693,
            218243354,
            0,
            [[6249552, 6698611], [6584205, 6815470]],
            3,
        ),
        (
            28,
            34,
            [[365, 385], [312, 352]],
            [1726, 1524],
            [2474, 2287],
            [278, 278],
            63,
            83339971,
            218243354,
            0,
            [[6249552, 6698611], [6584205, 6815470]],
            4,
        ),
        (
            28,
            30,
            [[306, 343], [315, 388]],
            [1524, 1730],
            [2295, 2468],
            [270, 224],
            64,
            79601705,
            218243354,
            0,
            [[6249552, 6698611], [6584205, 6815470]],
            6,
        ),
        (
            31,
            29,
            [[268, 306], [333, 406]],
            [1472, 1781],
            [2287, 2495],
            [253, 241],
            64,
            76092101,
            218243354,
            0,
            [[6249552, 6698611], [6584205, 6815470]],
            5,
        ),
        (
            32,
            32,
            [[308, 324], [295, 327]],
            [1627, 1627],
            [2436, 2419],
            [291, 250],
            63,
            72659097,
            218243354,
            0,
            [[6249552, 6698611], [6584205, 6815470]],
            6,
        ),
        (
            28,
            29,
            [[325, 329], [379, 360]],
            [1470, 1780],
            [2273, 2495],
            [258, 253],
            64,
            69399752,
            218243354,
            0,
            [[6249552, 6698611], [6584205, 6815470]],
            9,
        ),
    ],
];
const INHIBITION_TRACES_1024: [u64; 3] =
    [0x6e6a4106db4a096f, 0x644d8c85291aa525, 0x9536fa34086e28a9];
const INHIBITION_COMPOSITIONS_1024: [&[Composition]; 3] = [
    &[
        (
            [[3681, 11801], [6251, 6236]],
            [
                [
                    [217847, -5236, 326869, -512465],
                    [237157, -5604, 347167, -496052],
                ],
                [
                    [226932, -5368, 282548, -466058],
                    [211083, -4709, 331119, -448374],
                ],
            ],
            [[1098, 1650], [1010, 1649], [3350, 0]],
            54,
            10136684507982153650,
        ),
        (
            [[-1163, 6831], [735, -5938]],
            [
                [
                    [193944, -4865, 316522, -492098],
                    [244821, -6149, 326296, -449425],
                ],
                [
                    [239260, -5762, 299870, -462522],
                    [231607, -5301, 321125, -492104],
                ],
            ],
            [[983, 1597], [1008, 1643], [3354, 0]],
            50,
            3823758468564717504,
        ),
        (
            [[-8556, 8867], [17469, 6327]],
            [
                [
                    [212202, -5390, 328374, -475351],
                    [226155, -4552, 338442, -472163],
                ],
                [
                    [220577, -5324, 294308, -446503],
                    [261064, -7164, 371354, -505792],
                ],
            ],
            [[968, 1621], [934, 1657], [3355, 1]],
            56,
            16608775463467884709,
        ),
        (
            [[-5374, 3026], [6515, 5606]],
            [
                [
                    [180627, -8301, 330289, -488447],
                    [219795, -5236, 320310, -491249],
                ],
                [
                    [265884, -4852, 322932, -499430],
                    [298199, -6547, 377496, -558165],
                ],
            ],
            [[1051, 1688], [1080, 1746], [3368, 1]],
            55,
            5294676886327485614,
        ),
        (
            [[3279, 10365], [13719, 4990]],
            [
                [
                    [219515, -6451, 329562, -475883],
                    [249801, -8194, 335761, -458360],
                ],
                [
                    [238487, -5108, 301268, -450917],
                    [306855, -5695, 370235, -478786],
                ],
            ],
            [[1056, 1642], [961, 1740], [3353, 0]],
            60,
            2837754288549394100,
        ),
        (
            [[-90, 3737], [1318, -1208]],
            [
                [
                    [297901, -5512, 363347, -505984],
                    [264013, -6614, 368644, -515408],
                ],
                [
                    [198770, -3442, 261688, -394561],
                    [240931, -3547, 363342, -482636],
                ],
            ],
            [[949, 1770], [996, 1767], [3362, 0]],
            60,
            8665197029292676556,
        ),
        (
            [[6041, 1106], [-3570, 5113]],
            [
                [
                    [246239, -7260, 367001, -514368],
                    [239877, -4158, 304400, -479234],
                ],
                [
                    [249858, -3549, 310467, -487454],
                    [311287, -9037, 424981, -555393],
                ],
            ],
            [[1034, 1656], [992, 1758], [3347, 0]],
            57,
            17953464242657262727,
        ),
        (
            [[-670, 8899], [7687, 1578]],
            [
                [
                    [286453, -6308, 384276, -539653],
                    [262357, -7346, 363337, -511072],
                ],
                [
                    [227442, -6275, 313138, -481052],
                    [295838, -7994, 434470, -554337],
                ],
            ],
            [[1061, 1743], [1029, 1815], [3350, 0]],
            62,
            3519876315893851600,
        ),
        (
            [[4679, 11085], [-8682, 8408]],
            [
                [
                    [334975, -6707, 421302, -527499],
                    [290131, -7392, 391331, -527104],
                ],
                [
                    [222871, -4577, 289241, -461860],
                    [306835, -6128, 445680, -512024],
                ],
            ],
            [[1027, 1852], [1025, 1875], [3349, 0]],
            61,
            7224582974094275592,
        ),
        (
            [[4432, 4527], [5018, 8056]],
            [
                [
                    [266831, -3811, 394945, -569691],
                    [216120, -4355, 348720, -450882],
                ],
                [
                    [233637, -4719, 303237, -503659],
                    [378583, -8976, 471496, -584813],
                ],
            ],
            [[1011, 1714], [1013, 1863], [3363, 0]],
            58,
            12117755371981971634,
        ),
        (
            [[23184, 14548], [11286, 10678]],
            [
                [
                    [329743, -9286, 464100, -629857],
                    [263515, -4162, 378898, -512866],
                ],
                [
                    [211110, -4261, 263597, -436528],
                    [347859, -7169, 443155, -540400],
                ],
            ],
            [[971, 1894], [959, 2020], [3352, 0]],
            60,
            7834365528043607326,
        ),
        (
            [[3288, 2101], [-5545, 14851]],
            [
                [
                    [343865, -10018, 460477, -642162],
                    [253219, -9704, 364088, -499732],
                ],
                [
                    [219922, -6133, 323469, -485360],
                    [352705, -9535, 540732, -601179],
                ],
            ],
            [[1045, 1946], [1022, 2004], [3362, 1]],
            63,
            16904083644830291195,
        ),
        (
            [[1204, 968], [13164, 18531]],
            [
                [
                    [358694, -11438, 496429, -648550],
                    [255667, -6445, 363316, -510459],
                ],
                [
                    [221230, -5065, 317284, -463595],
                    [390121, -11983, 485420, -607144],
                ],
            ],
            [[1052, 1926], [1003, 2007], [3365, 0]],
            64,
            8121628200619684161,
        ),
        (
            [[11936, 5433], [1162, 8757]],
            [
                [
                    [418038, -15226, 542520, -655894],
                    [256776, -6173, 378807, -537367],
                ],
                [
                    [217727, -4974, 308848, -503101],
                    [406202, -18748, 522445, -596936],
                ],
            ],
            [[1070, 2079], [1006, 2118], [3363, 0]],
            64,
            2889462022108040372,
        ),
        (
            [[19035, 7436], [4696, 19604]],
            [
                [
                    [492272, -15532, 583442, -749270],
                    [299305, -6458, 387904, -560146],
                ],
                [
                    [187610, -4437, 304888, -479781],
                    [352657, -12719, 553888, -602779],
                ],
            ],
            [[1052, 2159], [1056, 2051], [3357, 0]],
            61,
            17215442664201372233,
        ),
        (
            [[16445, 15313], [2427, 25484]],
            [
                [
                    [462977, -14609, 555272, -662848],
                    [295133, -6033, 372044, -533145],
                ],
                [
                    [205934, -5717, 311096, -455914],
                    [374214, -9921, 555667, -630271],
                ],
            ],
            [[1040, 2212], [1026, 2196], [3361, 1]],
            64,
            12844983025154492134,
        ),
        (
            [[13686, 4042], [8109, 24058]],
            [
                [
                    [451115, -15724, 608550, -769475],
                    [272711, -7842, 374385, -548947],
                ],
                [
                    [206843, -4284, 310215, -469805],
                    [421215, -8257, 592748, -684099],
                ],
            ],
            [[1012, 2227], [1063, 2255], [3362, 0]],
            62,
            16693408307266882969,
        ),
        (
            [[1847, 5974], [1298, 23012]],
            [
                [
                    [566562, -16636, 635207, -792137],
                    [335624, -9898, 374122, -555579],
                ],
                [
                    [191986, -5734, 280916, -459547],
                    [365497, -14263, 560392, -651349],
                ],
            ],
            [[1097, 2353], [1061, 2230], [3359, 0]],
            63,
            11970631268909868591,
        ),
        (
            [[17458, 9866], [10667, 20547]],
            [
                [
                    [468760, -16427, 561046, -691081],
                    [272828, -8876, 374149, -512520],
                ],
                [
                    [249567, -5525, 322297, -506477],
                    [489962, -15876, 660767, -760755],
                ],
            ],
            [[1032, 2299], [1069, 2288], [3350, 0]],
            63,
            5810584380575294427,
        ),
        (
            [[27402, 13457], [5725, 24606]],
            [
                [
                    [527665, -18889, 627846, -768084],
                    [306224, -8044, 369450, -552735],
                ],
                [
                    [227273, -6386, 341839, -496249],
                    [497006, -18354, 651243, -740746],
                ],
            ],
            [[1129, 2459], [1100, 2430], [3339, 0]],
            64,
            12117237804481946950,
        ),
        (
            [[19288, 17370], [9711, 21793]],
            [
                [
                    [494137, -14256, 621120, -749983],
                    [272124, -6288, 379450, -508704],
                ],
                [
                    [243019, -7621, 335801, -519651],
                    [583147, -18951, 705789, -817027],
                ],
            ],
            [[1096, 2326], [1042, 2577], [3370, 0]],
            63,
            12579986990051336563,
        ),
        (
            [[25824, 7064], [4527, 33338]],
            [
                [
                    [469606, -24523, 681820, -786091],
                    [250857, -6952, 383052, -518881],
                ],
                [
                    [277169, -5997, 322175, -538549],
                    [653038, -13718, 692830, -800752],
                ],
            ],
            [[1046, 2465], [1022, 2677], [3329, 0]],
            63,
            3688487099005689564,
        ),
        (
            [[18726, 15091], [3842, 25409]],
            [
                [
                    [524354, -15850, 674083, -911027],
                    [272390, -9340, 382469, -568775],
                ],
                [
                    [250351, -7567, 364910, -520171],
                    [575424, -20537, 705505, -814236],
                ],
            ],
            [[1186, 2453], [1105, 2598], [3373, 0]],
            63,
            5564842184275886180,
        ),
        (
            [[19616, 345], [6374, 20423]],
            [
                [
                    [509408, -12655, 645693, -810129],
                    [265068, -5912, 372427, -542221],
                ],
                [
                    [265194, -5607, 342077, -534658],
                    [659718, -19135, 787629, -960254],
                ],
            ],
            [[1109, 2571], [1118, 2722], [3358, 0]],
            64,
            4081771246160097736,
        ),
    ],
    &[
        (
            [[-555, 3681], [1578, 10457]],
            [
                [
                    [215183, -5251, 321609, -512486],
                    [241995, -5615, 348867, -499035],
                ],
                [
                    [229888, -5388, 285229, -465156],
                    [208076, -4781, 326125, -444424],
                ],
            ],
            [[1099, 1646], [1010, 1651], [3350, 0]],
            53,
            617215032958896972,
        ),
        (
            [[-2227, 6078], [545, -2920]],
            [
                [
                    [183197, -4860, 309600, -490032],
                    [247170, -5906, 341999, -456683],
                ],
                [
                    [249717, -5725, 310968, -467381],
                    [224904, -4882, 312328, -485456],
                ],
            ],
            [[987, 1576], [1009, 1630], [3354, 0]],
            47,
            2154768690161622433,
        ),
        (
            [[-7901, 5500], [9598, 11412]],
            [
                [
                    [205250, -5688, 321658, -465852],
                    [251306, -5680, 369620, -488802],
                ],
                [
                    [230435, -5682, 300604, -447793],
                    [251674, -7171, 343811, -486219],
                ],
            ],
            [[968, 1622], [935, 1653], [3353, 1]],
            56,
            8501028342152723512,
        ),
        (
            [[-4789, 3917], [551, 12999]],
            [
                [
                    [167073, -7726, 317201, -473103],
                    [240014, -5380, 361106, -517534],
                ],
                [
                    [293697, -6883, 338502, -510411],
                    [279519, -6275, 337394, -534753],
                ],
            ],
            [[1052, 1707], [1078, 1736], [3369, 1]],
            48,
            4957639914544827501,
        ),
        (
            [[2724, 1829], [9481, 13903]],
            [
                [
                    [203340, -6489, 314745, -468206],
                    [298690, -9305, 386307, -479936],
                ],
                [
                    [279775, -6783, 333049, -480349],
                    [279164, -5951, 322348, -464709],
                ],
            ],
            [[1061, 1662], [954, 1756], [3352, 0]],
            57,
            2024051208259761331,
        ),
        (
            [[1453, -294], [-2788, 1905]],
            [
                [
                    [263181, -5429, 333738, -483056],
                    [336762, -8738, 456799, -585257],
                ],
                [
                    [241743, -3033, 295685, -415367],
                    [211676, -2534, 305731, -436421],
                ],
            ],
            [[949, 1778], [996, 1828], [3363, 0]],
            58,
            4776257335360452350,
        ),
        (
            [[10186, 602], [-2099, 4703]],
            [
                [
                    [196372, -6704, 320937, -470489],
                    [326235, -4925, 413113, -569429],
                ],
                [
                    [301726, -3910, 358082, -537215],
                    [267599, -4892, 350342, -508916],
                ],
            ],
            [[1032, 1642], [996, 1803], [3345, 0]],
            59,
            3412930827303937360,
        ),
        (
            [[-3891, 2733], [4314, 14085]],
            [
                [
                    [217098, -4139, 313496, -494613],
                    [336571, -9488, 477638, -608244],
                ],
                [
                    [313469, -9194, 375676, -527064],
                    [245638, -6369, 353575, -505615],
                ],
            ],
            [[1062, 1753], [1035, 1865], [3351, 0]],
            56,
            11157594300242647548,
        ),
        (
            [[5694, 7360], [-5577, 16457]],
            [
                [
                    [256778, -5239, 334530, -474457],
                    [410994, -10596, 555098, -634852],
                ],
                [
                    [290346, -5717, 377958, -527280],
                    [236805, -3986, 342176, -460486],
                ],
            ],
            [[1038, 1801], [1043, 1935], [3351, 0]],
            61,
            14693511181922790663,
        ),
        (
            [[1394, 19780], [126, 8195]],
            [
                [
                    [188137, -2315, 284926, -478184],
                    [344339, -10218, 521517, -597662],
                ],
                [
                    [329913, -7382, 419968, -592635],
                    [274442, -6260, 362929, -513617],
                ],
            ],
            [[1013, 1724], [1025, 1914], [3363, 0]],
            59,
            13616744006546211686,
        ),
        (
            [[5814, 6596], [4029, 2970]],
            [
                [
                    [216576, -5139, 335708, -509436],
                    [448366, -9940, 558607, -664923],
                ],
                [
                    [317436, -7625, 357339, -505655],
                    [239111, -3489, 318633, -463734],
                ],
            ],
            [[965, 1801], [972, 2126], [3350, 0]],
            61,
            5196614687330326975,
        ),
        (
            [[-1054, 437], [9714, 11470]],
            [
                [
                    [208167, -7009, 323363, -507243],
                    [439134, -17053, 632178, -744476],
                ],
                [
                    [337678, -10539, 451957, -598345],
                    [244836, -4917, 375909, -498159],
                ],
            ],
            [[1056, 1896], [1037, 2115], [3361, 1]],
            61,
            10186451732980210463,
        ),
        (
            [[-7387, 12835], [4514, 24000]],
            [
                [
                    [194746, -7347, 336147, -522002],
                    [508798, -14581, 650369, -727346],
                ],
                [
                    [339143, -8582, 448121, -598447],
                    [255670, -5556, 328362, -491695],
                ],
            ],
            [[1057, 1907], [1019, 2209], [3363, 0]],
            64,
            11754507003513236159,
        ),
        (
            [[12635, 20449], [14192, 6860]],
            [
                [
                    [242534, -6428, 327546, -497776],
                    [518561, -20922, 723483, -836438],
                ],
                [
                    [344038, -9135, 448263, -578190],
                    [248717, -10041, 327968, -510099],
                ],
            ],
            [[1041, 1943], [1048, 2284], [3363, 0]],
            61,
            11341790919221532242,
        ),
        (
            [[13947, 26666], [14468, 4694]],
            [
                [
                    [255354, -8583, 350615, -523489],
                    [639400, -18089, 771073, -888458],
                ],
                [
                    [311015, -10639, 449993, -584774],
                    [203346, -5346, 336424, -469460],
                ],
            ],
            [[1034, 1923], [1077, 2324], [3355, 0]],
            62,
            10617922543238864153,
        ),
        (
            [[6580, 42844], [4509, 11096]],
            [
                [
                    [233758, -5960, 316962, -510654],
                    [617380, -21721, 783147, -851800],
                ],
                [
                    [337930, -9925, 471496, -558857],
                    [229429, -4819, 324710, -492572],
                ],
            ],
            [[1022, 2033], [1039, 2451], [3361, 1]],
            63,
            7664520144685478125,
        ),
        (
            [[8038, 22311], [16731, 3102]],
            [
                [
                    [227081, -6092, 356020, -500486],
                    [614784, -19130, 835537, -963608],
                ],
                [
                    [357392, -13402, 477154, -608138],
                    [225969, -4611, 341490, -517221],
                ],
            ],
            [[1006, 2022], [1047, 2443], [3362, 0]],
            60,
            3471537262751756204,
        ),
        (
            [[-9291, 32309], [13124, 13375]],
            [
                [
                    [282341, -5986, 354123, -543247],
                    [721144, -27372, 840470, -968644],
                ],
                [
                    [312672, -11183, 478894, -606352],
                    [206321, -5669, 315486, -478166],
                ],
            ],
            [[1091, 2089], [1085, 2559], [3360, 0]],
            61,
            11181226026674619894,
        ),
        (
            [[2550, 37613], [14326, 10817]],
            [
                [
                    [230177, -6545, 287879, -476310],
                    [589485, -26352, 811139, -878653],
                ],
                [
                    [457253, -13994, 527171, -647773],
                    [261238, -6047, 365318, -540455],
                ],
            ],
            [[1021, 2154], [1089, 2421], [3351, 0]],
            64,
            65605828888439063,
        ),
        (
            [[10469, 49108], [15127, 8126]],
            [
                [
                    [256782, -7392, 343749, -533253],
                    [650095, -25572, 818001, -878124],
                ],
                [
                    [420534, -14326, 567504, -705589],
                    [241897, -8963, 369314, -532148],
                ],
            ],
            [[1122, 2185], [1119, 2616], [3340, 0]],
            62,
            12446530812951043081,
        ),
        (
            [[1121, 25969], [19793, 8609]],
            [
                [
                    [213106, -4459, 310525, -487354],
                    [577567, -16834, 762552, -852566],
                ],
                [
                    [491872, -18854, 614491, -721747],
                    [281756, -6223, 365994, -547094],
                ],
            ],
            [[1066, 2207], [1050, 2597], [3373, 0]],
            63,
            8836340399555622940,
        ),
        (
            [[8891, 40587], [18312, 17519]],
            [
                [
                    [205708, -5988, 306957, -475600],
                    [564963, -19696, 792338, -898217],
                ],
                [
                    [531563, -18473, 607640, -762669],
                    [318340, -7802, 380985, -565871],
                ],
            ],
            [[1018, 2271], [1048, 2645], [3335, 0]],
            64,
            13338580351784501064,
        ),
        (
            [[-3996, 54283], [11015, 5911]],
            [
                [
                    [210276, -6238, 344727, -546902],
                    [629402, -29371, 854197, -959549],
                ],
                [
                    [515788, -15542, 638416, -766089],
                    [255491, -5538, 371255, -532188],
                ],
            ],
            [[1149, 2324], [1078, 2722], [3376, 0]],
            63,
            2094943425547524298,
        ),
        (
            [[2195, 31251], [21423, 13946]],
            [
                [
                    [230859, -4741, 318539, -526159],
                    [604079, -20817, 805058, -989898],
                ],
                [
                    [579863, -13872, 672472, -838964],
                    [296738, -6939, 418328, -605685],
                ],
            ],
            [[1123, 2552], [1147, 2607], [3362, 0]],
            61,
            6579011670422388899,
        ),
    ],
    &[
        (
            [[-455, 11796], [6255, 10343]],
            [
                [
                    [214196, -5235, 320901, -512591],
                    [238576, -5598, 347215, -497063],
                ],
                [
                    [227424, -5368, 281840, -464151],
                    [208181, -4722, 326203, -444151],
                ],
            ],
            [[1099, 1643], [1010, 1644], [3350, 0]],
            53,
            1457144849140171388,
        ),
        (
            [[-2228, 6803], [359, -2495]],
            [
                [
                    [183462, -4856, 310921, -490704],
                    [243322, -6131, 323739, -448906],
                ],
                [
                    [238222, -5636, 299565, -462753],
                    [224685, -4843, 313659, -485846],
                ],
            ],
            [[987, 1570], [1009, 1618], [3354, 0]],
            50,
            1808309301862196141,
        ),
        (
            [[-8739, 8920], [17187, 12507]],
            [
                [
                    [203190, -5722, 318850, -470000],
                    [226354, -4525, 337885, -471332],
                ],
                [
                    [215367, -5449, 290861, -441899],
                    [253212, -7030, 346812, -486589],
                ],
            ],
            [[969, 1594], [934, 1617], [3354, 1]],
            55,
            8743230943862918556,
        ),
        (
            [[-8564, 3127], [6776, 12887]],
            [
                [
                    [165826, -7768, 317417, -480794],
                    [218222, -5234, 316717, -487654],
                ],
                [
                    [261611, -4851, 321698, -496757],
                    [281055, -5784, 339615, -530195],
                ],
            ],
            [[1053, 1655], [1079, 1682], [3369, 1]],
            52,
            131948139373864433,
        ),
        (
            [[3129, 8710], [16559, 13334]],
            [
                [
                    [200829, -6325, 314941, -460150],
                    [246235, -7462, 334724, -452070],
                ],
                [
                    [238369, -5590, 298060, -449417],
                    [276531, -5898, 322886, -463031],
                ],
            ],
            [[1046, 1616], [955, 1675], [3351, 0]],
            59,
            5239439373465351536,
        ),
        (
            [[509, 3673], [-1190, 1262]],
            [
                [
                    [260264, -4634, 332157, -485547],
                    [260396, -6092, 363562, -509682],
                ],
                [
                    [192812, -2715, 256134, -392589],
                    [205896, -2664, 296182, -433362],
                ],
            ],
            [[943, 1689], [990, 1684], [3363, 0]],
            59,
            10105863207689781449,
        ),
        (
            [[10241, 1689], [-4672, 2416]],
            [
                [
                    [197894, -6482, 316153, -468812],
                    [238108, -3504, 304008, -480182],
                ],
                [
                    [244284, -3290, 312000, -485554],
                    [256810, -5779, 344623, -503538],
                ],
            ],
            [[1024, 1558], [985, 1649], [3346, 0]],
            55,
            17270951097485941793,
        ),
        (
            [[-4079, 9180], [6899, 13839]],
            [
                [
                    [215231, -4488, 309814, -484813],
                    [253226, -6898, 357052, -504526],
                ],
                [
                    [232639, -7146, 305791, -475349],
                    [240323, -6366, 350450, -498420],
                ],
            ],
            [[1055, 1607], [1031, 1667], [3351, 0]],
            60,
            5893953143060338035,
        ),
        (
            [[7454, 9999], [-8463, 16265]],
            [
                [
                    [256552, -5158, 329402, -470170],
                    [279421, -7462, 387768, -521146],
                ],
                [
                    [218881, -4763, 279474, -449946],
                    [230069, -3969, 333688, -455362],
                ],
            ],
            [[1021, 1671], [1023, 1697], [3349, 0]],
            61,
            12273719749151974976,
        ),
        (
            [[901, 5982], [4642, 8314]],
            [
                [
                    [191304, -2445, 285667, -478324],
                    [209520, -5073, 332876, -438722],
                ],
                [
                    [229962, -4586, 298541, -492789],
                    [265789, -5917, 350565, -498838],
                ],
            ],
            [[1004, 1548], [998, 1637], [3360, 0]],
            56,
            2088058403078080738,
        ),
        (
            [[5319, 13921], [9691, 2921]],
            [
                [
                    [216636, -5331, 330725, -510105],
                    [258044, -4497, 352011, -498649],
                ],
                [
                    [193265, -4121, 266259, -432096],
                    [231448, -3374, 315638, -453206],
                ],
            ],
            [[962, 1606], [944, 1746], [3352, 0]],
            59,
            9257469390611712180,
        ),
        (
            [[725, -541], [-7277, 10549]],
            [
                [
                    [209559, -6753, 320987, -498162],
                    [237880, -8751, 346107, -482221],
                ],
                [
                    [206840, -5575, 310685, -475853],
                    [229141, -5301, 363879, -484745],
                ],
            ],
            [[1031, 1653], [1005, 1700], [3363, 1]],
            60,
            465184960812400659,
        ),
        (
            [[-6849, -1807], [11189, 20139]],
            [
                [
                    [190099, -7544, 335822, -514855],
                    [241641, -6084, 348502, -497818],
                ],
                [
                    [212078, -5067, 310558, -458054],
                    [240798, -4847, 321699, -485945],
                ],
            ],
            [[1044, 1627], [997, 1696], [3364, 0]],
            55,
            10219717001124811129,
        ),
        (
            [[12758, 4658], [1552, 6144]],
            [
                [
                    [238650, -6867, 325061, -492946],
                    [243512, -6072, 368140, -521780],
                ],
                [
                    [211426, -5031, 291116, -478310],
                    [230607, -9471, 321780, -491500],
                ],
            ],
            [[1045, 1664], [1010, 1736], [3364, 0]],
            58,
            13475041344534734156,
        ),
        (
            [[11996, 7058], [5813, 6533]],
            [
                [
                    [254059, -8227, 347256, -524353],
                    [277672, -6612, 370432, -543938],
                ],
                [
                    [177530, -4151, 282669, -456443],
                    [202309, -4853, 321117, -438675],
                ],
            ],
            [[1018, 1631], [1035, 1658], [3356, 0]],
            52,
            6174409511473354739,
        ),
        (
            [[6782, 16575], [2142, 10711]],
            [
                [
                    [238404, -6184, 315871, -492628],
                    [269974, -5336, 352153, -508824],
                ],
                [
                    [191517, -5251, 294862, -429543],
                    [215816, -4794, 313525, -462757],
                ],
            ],
            [[994, 1686], [991, 1767], [3361, 1]],
            63,
            8721598446168445598,
        ),
        (
            [[5151, 4956], [2793, 3999]],
            [
                [
                    [228071, -6197, 349193, -503344],
                    [250057, -7395, 346966, -523497],
                ],
                [
                    [180294, -3915, 286575, -445549],
                    [212934, -3340, 317678, -495975],
                ],
            ],
            [[983, 1644], [1029, 1734], [3363, 0]],
            51,
            5079001991175175081,
        ),
        (
            [[-8938, 3707], [4781, 16445]],
            [
                [
                    [279670, -5310, 340088, -527824],
                    [299682, -9365, 341635, -528030],
                ],
                [
                    [178328, -5079, 264030, -431336],
                    [193859, -5384, 303375, -449492],
                ],
            ],
            [[1063, 1747], [1029, 1727], [3359, 0]],
            55,
            14447684897769946791,
        ),
        (
            [[1473, 6148], [10903, 11815]],
            [
                [
                    [220253, -6611, 284175, -458151],
                    [232025, -7349, 339014, -484326],
                ],
                [
                    [235136, -5353, 302021, -470842],
                    [249202, -5921, 349643, -503874],
                ],
            ],
            [[996, 1697], [1044, 1620], [3352, 0]],
            55,
            8882628811489822335,
        ),
        (
            [[8796, 8027], [6703, 7765]],
            [
                [
                    [251374, -7607, 325769, -519847],
                    [264054, -7627, 338274, -518120],
                ],
                [
                    [204309, -6186, 320365, -466463],
                    [230214, -8032, 345009, -496619],
                ],
            ],
            [[1095, 1712], [1070, 1780], [3338, 0]],
            53,
            17579281760734411741,
        ),
        (
            [[1422, 14861], [7840, 6625]],
            [
                [
                    [202768, -4338, 297033, -462600],
                    [232684, -5447, 336345, -458278],
                ],
                [
                    [223693, -7167, 313786, -482784],
                    [274997, -6056, 340811, -513090],
                ],
            ],
            [[1033, 1637], [999, 1794], [3372, 0]],
            55,
            3988361816129406275,
        ),
        (
            [[9547, 4361], [2157, 14678]],
            [
                [
                    [197401, -5407, 296186, -441797],
                    [213028, -5600, 336045, -468831],
                ],
                [
                    [238094, -5986, 302064, -506333],
                    [302473, -6625, 357281, -520926],
                ],
            ],
            [[1001, 1632], [986, 1819], [3331, 0]],
            59,
            17672966803251675441,
        ),
        (
            [[-8207, 8961], [3969, 4901]],
            [
                [
                    [198244, -5894, 321483, -517622],
                    [224915, -7918, 359272, -530072],
                ],
                [
                    [218625, -7687, 336908, -485273],
                    [234538, -4389, 362032, -510794],
                ],
            ],
            [[1115, 1601], [1055, 1750], [3371, 0]],
            54,
            3164547345086987163,
        ),
        (
            [[-577, 1070], [5000, 5985]],
            [
                [
                    [220346, -4318, 292683, -492114],
                    [226417, -5974, 330631, -501176],
                ],
                [
                    [238140, -4558, 321602, -494091],
                    [267284, -6286, 382833, -543029],
                ],
            ],
            [[1053, 1767], [1071, 1725], [3359, 0]],
            54,
            1857431938680788171,
        ),
    ],
];
const INHIBITION_EARNED_1024: [&[EarnedBlock]; 3] = [
    &[
        (
            [[15, 14, 5], [11, 16, 3]],
            31,
            [[7870, 0], [0, 56367]],
            -44931,
            20605,
        ),
        (
            [[16, 14, 1], [13, 16, 4]],
            32,
            [[14553, 0], [0, 44877]],
            43010,
            108546,
        ),
        (
            [[10, 19, 2], [14, 17, 2]],
            27,
            [[41655, 0], [0, 56321]],
            31280,
            -34256,
        ),
        (
            [[6, 18, 4], [10, 23, 3]],
            29,
            [[1745, 0], [0, 78477]],
            37603,
            103139,
        ),
        (
            [[12, 15, 3], [9, 23, 2]],
            35,
            [[46447, 0], [0, 94398]],
            28315,
            93851,
        ),
        (
            [[21, 12, 3], [9, 18, 1]],
            39,
            [[99486, 0], [0, 63464]],
            -18766,
            46770,
        ),
        (
            [[16, 12, 2], [7, 22, 5]],
            38,
            [[75126, 0], [0, 82419]],
            -18866,
            46670,
        ),
        (
            [[18, 9, 5], [6, 24, 2]],
            42,
            [[71022, 0], [0, 84550]],
            -16511,
            -82047,
        ),
        (
            [[27, 7, 0], [6, 22, 2]],
            49,
            [[154229, 0], [0, 118827]],
            46617,
            112153,
        ),
        (
            [[18, 9, 1], [4, 32, 0]],
            50,
            [[78204, 0], [0, 145536]],
            -18160,
            47376,
        ),
        (
            [[29, 3, 1], [1, 30, 0]],
            59,
            [[93504, 0], [0, 131070]],
            -4481,
            61055,
        ),
        (
            [[27, 4, 1], [1, 30, 1]],
            57,
            [[129507, 0], [0, 98036]],
            46691,
            112227,
        ),
        (
            [[26, 4, 2], [1, 30, 1]],
            56,
            [[99590, 0], [0, 109204]],
            46667,
            112203,
        ),
        (
            [[29, 3, 1], [0, 30, 1]],
            59,
            [[152781, 0], [0, 109913]],
            46683,
            112219,
        ),
        (
            [[36, 1, 0], [1, 25, 1]],
            61,
            [[157177, 0], [0, 80094]],
            46689,
            112225,
        ),
        (
            [[33, 2, 0], [2, 27, 0]],
            60,
            [[148176, 0], [0, 100863]],
            44671,
            110207,
        ),
        (
            [[32, 1, 0], [0, 29, 2]],
            61,
            [[137910, 0], [0, 121283]],
            46019,
            111555,
        ),
        (
            [[36, 1, 1], [0, 26, 0]],
            62,
            [[159397, 0], [0, 81802]],
            46691,
            112227,
        ),
        (
            [[31, 0, 0], [0, 33, 0]],
            64,
            [[130383, 0], [0, 105095]],
            46691,
            112227,
        ),
        (
            [[34, 0, 0], [0, 30, 0]],
            64,
            [[127834, 0], [0, 166892]],
            46691,
            112227,
        ),
        (
            [[30, 0, 0], [0, 34, 0]],
            64,
            [[118987, 0], [0, 162358]],
            46691,
            112227,
        ),
        (
            [[29, 0, 0], [0, 35, 0]],
            64,
            [[85446, 0], [0, 183336]],
            46691,
            112227,
        ),
        (
            [[32, 0, 0], [0, 32, 0]],
            64,
            [[27097, 0], [0, 115020]],
            46691,
            112227,
        ),
        (
            [[29, 0, 0], [0, 35, 0]],
            64,
            [[67637, 0], [0, 94827]],
            46691,
            112227,
        ),
    ],
    &[
        (
            [[14, 16, 4], [12, 16, 2]],
            28,
            [[0, 55988], [22743, 0]],
            24175,
            -41361,
        ),
        (
            [[10, 17, 4], [14, 17, 2]],
            31,
            [[0, 78708], [46413, 0]],
            -44591,
            -110127,
        ),
        (
            [[8, 21, 2], [16, 12, 5]],
            37,
            [[0, 81926], [61483, 0]],
            -31276,
            34260,
        ),
        (
            [[4, 23, 1], [17, 13, 6]],
            40,
            [[0, 63612], [58303, 0]],
            -30710,
            34826,
        ),
        (
            [[5, 23, 2], [15, 15, 4]],
            38,
            [[0, 150326], [73249, 0]],
            -16428,
            -81964,
        ),
        (
            [[6, 29, 1], [16, 11, 1]],
            45,
            [[0, 142247], [99504, 0]],
            44661,
            110197,
        ),
        (
            [[3, 27, 0], [25, 5, 4]],
            52,
            [[0, 105949], [82516, 0]],
            44671,
            -20865,
        ),
        (
            [[4, 27, 1], [20, 10, 2]],
            47,
            [[0, 134417], [77450, 0]],
            29216,
            94752,
        ),
        (
            [[3, 30, 1], [23, 5, 2]],
            53,
            [[0, 226065], [102638, 0]],
            29213,
            94749,
        ),
        (
            [[1, 27, 0], [24, 12, 0]],
            51,
            [[0, 151340], [96197, 0]],
            46691,
            112227,
        ),
        (
            [[3, 30, 0], [24, 6, 1]],
            54,
            [[0, 171173], [113553, 0]],
            46691,
            112227,
        ),
        (
            [[0, 32, 0], [26, 5, 1]],
            58,
            [[0, 207863], [79045, 0]],
            46667,
            112203,
        ),
        (
            [[0, 32, 0], [27, 2, 3]],
            59,
            [[0, 199085], [100860, 0]],
            46213,
            111749,
        ),
        (
            [[0, 33, 0], [28, 2, 1]],
            61,
            [[0, 151261], [78737, 0]],
            46019,
            111555,
        ),
        (
            [[0, 37, 0], [24, 2, 1]],
            61,
            [[0, 187415], [55699, 0]],
            46691,
            112227,
        ),
        (
            [[0, 35, 0], [29, 0, 0]],
            64,
            [[0, 146796], [104427, 0]],
            46691,
            112227,
        ),
        (
            [[0, 33, 0], [29, 1, 1]],
            62,
            [[0, 147086], [88998, 0]],
            44402,
            109938,
        ),
        (
            [[0, 38, 0], [25, 0, 1]],
            63,
            [[0, 146659], [79695, 0]],
            44671,
            110207,
        ),
        (
            [[0, 31, 0], [30, 3, 0]],
            61,
            [[0, 103157], [126176, 0]],
            46691,
            112227,
        ),
        (
            [[0, 34, 0], [30, 0, 0]],
            64,
            [[0, 116070], [127073, 0]],
            46691,
            112227,
        ),
        (
            [[0, 30, 0], [33, 0, 1]],
            63,
            [[0, 91970], [176154, 0]],
            46691,
            112227,
        ),
        (
            [[0, 29, 0], [35, 0, 0]],
            64,
            [[0, 59304], [118960, 0]],
            46691,
            112227,
        ),
        (
            [[0, 32, 0], [32, 0, 0]],
            64,
            [[0, 73636], [155167, 0]],
            46691,
            112227,
        ),
        (
            [[0, 29, 0], [35, 0, 0]],
            64,
            [[0, 52698], [120689, 0]],
            46691,
            112227,
        ),
    ],
    &[
        ([[14, 15, 5], [12, 15, 3]], 0, [[0, 0], [0, 0]], 0, 0),
        ([[13, 14, 4], [14, 16, 3]], 0, [[0, 0], [0, 0]], 0, 0),
        ([[8, 18, 5], [15, 17, 1]], 0, [[0, 0], [0, 0]], 0, 0),
        ([[5, 21, 2], [12, 18, 6]], 0, [[0, 0], [0, 0]], 0, 0),
        ([[10, 16, 4], [13, 19, 2]], 0, [[0, 0], [0, 0]], 0, 0),
        ([[13, 18, 5], [11, 15, 2]], 0, [[0, 0], [0, 0]], 0, 0),
        ([[12, 17, 1], [15, 16, 3]], 0, [[0, 0], [0, 0]], 0, 0),
        ([[11, 20, 1], [12, 15, 5]], 0, [[0, 0], [0, 0]], 0, 0),
        ([[15, 14, 5], [15, 10, 5]], 0, [[0, 0], [0, 0]], 0, 0),
        ([[10, 17, 1], [12, 21, 3]], 0, [[0, 0], [0, 0]], 0, 0),
        ([[12, 20, 1], [7, 21, 3]], 0, [[0, 0], [0, 0]], 0, 0),
        ([[12, 19, 1], [9, 22, 1]], 0, [[0, 0], [0, 0]], 0, 0),
        ([[13, 17, 2], [15, 15, 2]], 0, [[0, 0], [0, 0]], 0, 0),
        ([[20, 10, 3], [11, 17, 3]], 0, [[0, 0], [0, 0]], 0, 0),
        ([[18, 16, 3], [5, 20, 2]], 0, [[0, 0], [0, 0]], 0, 0),
        ([[11, 22, 2], [9, 17, 3]], 0, [[0, 0], [0, 0]], 0, 0),
        ([[19, 12, 2], [16, 14, 1]], 0, [[0, 0], [0, 0]], 0, 0),
        ([[18, 19, 1], [14, 10, 2]], 0, [[0, 0], [0, 0]], 0, 0),
        ([[13, 16, 2], [15, 17, 1]], 0, [[0, 0], [0, 0]], 0, 0),
        ([[13, 19, 2], [13, 15, 2]], 0, [[0, 0], [0, 0]], 0, 0),
        ([[9, 18, 3], [12, 19, 3]], 0, [[0, 0], [0, 0]], 0, 0),
        ([[8, 19, 2], [9, 23, 3]], 0, [[0, 0], [0, 0]], 0, 0),
        ([[16, 13, 3], [13, 16, 3]], 0, [[0, 0], [0, 0]], 0, 0),
        ([[15, 12, 2], [15, 13, 7]], 0, [[0, 0], [0, 0]], 0, 0),
    ],
];
const INHIBITION_READ_1024: [u64; 3] = [0x9125c69cd9000bff, 0xbb2bfa57ebdfaff0, 0xfbb9d0ae596907cf];
const INHIBITION_CENSUS_1024: [&[(u32, u64)]; 3] = [
    &[
        (1, 3202),
        (2, 24531),
        (3, 38063),
        (4, 11538),
        (5, 578),
        (6, 108),
        (7, 33),
        (8, 12),
        (9, 5),
        (10, 1),
        (11, 4),
    ],
    &[
        (1, 3189),
        (2, 24500),
        (3, 38119),
        (4, 11523),
        (5, 586),
        (6, 100),
        (7, 32),
        (8, 16),
        (9, 4),
        (10, 1),
        (11, 3),
        (13, 1),
    ],
    &[
        (1, 3193),
        (2, 24547),
        (3, 38094),
        (4, 11510),
        (5, 578),
        (6, 102),
        (7, 29),
        (8, 13),
        (9, 5),
        (11, 5),
    ],
];
/// The verdict, by the rule committed first, over the pinned tables.
const INHIBITION_1024: Reinforced = Reinforced {
    correct: [true, true],
    yes: true,
};
/// The correct selections over the last 128 trials, per rewarded arm, against `REWARDED_MIN`.
const CORRECT_LAST_INHIBITION_1024: [u32; 2] = [128, 128];
/// Where each arm's selection first passed 40 of 64 per block, as read, beside H-14's 448
/// and 384 (`CROSSED_1024`) and H-15's 704 and 512.
const CROSSED_INHIBITION_1024: [Option<usize>; 3] = [Some(512), Some(256), None];
/// The selections per stimulus over the last 128 trials, per arm, `[stimulus][readout 0,
/// readout 1, tie]`.
const SPLITS_LAST_INHIBITION_1024: [[[u32; 3]; 2]; 3] = [
    [[61, 0, 0], [0, 67, 0]],
    [[0, 61, 0], [67, 0, 0]],
    [[31, 25, 5], [28, 29, 10]],
];
/// ADR-0080's derivation as read on each arm, clause by clause, a reading beside the
/// assertion: the signal at every trial's end at most 0.712, below zero after every negative
/// reward, and nothing consolidated in the trial after one.
const DERIVATION_INHIBITION_1024: [[bool; 3]; 3] =
    [[true, true, true], [true, true, true], [true, true, true]];
/// Whether the inhibitory sum fell in every block of each arm, as read, beside
/// `FALLS_PREDICTED`.
const FALLS_1024: [bool; 3] = [true, true, true];
/// The withheld arm's lean over its last 128 trials, as read, beside `FROZEN_LEAN_1024`.
const LEAN_1024: [Option<u8>; 2] = [Some(0), Some(1)];
/// The reach of each arm's run by polarity, `(inside the pairs the reward can reach,
/// outside)` for the excitatory synapses and for the inhibitory ones, as read.
const REACH_INHIBITION_1024: [Reach; 3] = [
    Reach {
        excitatory: (1584, 0),
        inhibitory: (0, 6491),
    },
    Reach {
        excitatory: (1604, 0),
        inhibitory: (0, 6494),
    },
    Reach {
        excitatory: (0, 0),
        inhibitory: (0, 6497),
    },
];
/// The arena's sums by polarity after each arm's run, `(inhibitory, excitatory)`, against
/// the image's `QUIET_1024[SETTLED].1`.
const SUMS_AFTER_1024: [(i64, i64); 3] = [
    (69725710, 222954146),
    (67783115, 223533834),
    (69399752, 218243354),
];

/// The gate's test (ADR-0061's class; brief 039): the arms and their feedback; the constants
/// as ADR-0085 fixed them; the assertion's rule and the readings' rules at their edges over
/// reaches, blocks and splits written by hand; the frozen network's lean from the pinned
/// table; the reach by polarity on the instrument's network, nothing moved and then one
/// weight of each polarity moved by hand; and eight trials on the instrument's network at
/// 1 024 units with the inhibitory baseline set — the assignment, with the oracle held at
/// every trial inside `earned_run_under`, no excitatory synapse outside the answer pairs
/// moved and the inhibitory synapses moved; then the reward withheld, no excitatory synapse
/// moved and the inhibitory ones moved; then the reward withheld with the baseline unset,
/// nothing moved at all, today's rule. No whole run, and nothing else added to the gate. The
/// instrument's network and not the settled one, as ADR-0083's gate ran it: the settled
/// lead-in is the weekly test's cost, and the rules read the same on any network.
#[test]
fn the_first_eight_trials_of_inhibition_off_the_gate_at_1024_units_and_the_rules_over_their_tables()
{
    // The arms, their feedback, their assignment and their pairs; the constants.
    assert_eq!(
        INHIBITION_REWARDED,
        [INHIBITION_ARMS[0], INHIBITION_ARMS[1]]
    );
    assert_eq!(INHIBITION_ARMS[WITHHELD], Inhibition::Withheld);
    assert_eq!(
        inhibition_feedback(Inhibition::Assignment),
        Feedback::Answer
    );
    assert_eq!(inhibition_feedback(Inhibition::Mirrored), Feedback::Answer);
    assert_eq!(
        inhibition_feedback(Inhibition::Withheld),
        Feedback::Withheld
    );
    assert!(
        !inhibition_mirrors(Inhibition::Assignment)
            && inhibition_mirrors(Inhibition::Mirrored)
            && !inhibition_mirrors(Inhibition::Withheld)
    );
    assert!(
        inhibition_rewards(Inhibition::Assignment)
            && inhibition_rewards(Inhibition::Mirrored)
            && !inhibition_rewards(Inhibition::Withheld)
    );
    assert_eq!(
        inhibition_pairs(Inhibition::Assignment),
        vec![(0, 0), (1, 1)]
    );
    assert_eq!(inhibition_pairs(Inhibition::Mirrored), vec![(0, 1), (1, 0)]);
    assert_eq!(inhibition_pairs(Inhibition::Withheld), ALL_PAIRS.to_vec());
    assert_eq!(GATE_BASELINE_Q16, 0);
    assert_eq!(INHIBITORY_BASELINE_Q16, 0x8000);
    assert_eq!(INHIBITION_TRIALS, REINFORCED_TRIALS);
    assert_eq!([INHIBITION_PREDICTED, LEAN_AS_FROZEN_PREDICTED], [true; 2]);
    assert_eq!(FALLS_PREDICTED, [true; 3]);
    // The assertion's rule at its edges over reaches written by hand.
    let reach = |excitatory: (u64, u64), inhibitory: (u64, u64)| Reach {
        excitatory,
        inhibitory,
    };
    assert!(excitatory_held(&Reach::default(), true));
    assert!(excitatory_held(&Reach::default(), false));
    assert!(
        excitatory_held(&reach((5, 0), (0, 9)), true),
        "a rewarded arm's answer pairs may move"
    );
    assert!(
        !excitatory_held(&reach((5, 0), (0, 9)), false),
        "the withheld arm's may not"
    );
    assert!(
        !excitatory_held(&reach((0, 1), (0, 0)), true),
        "one excitatory synapse outside the pairs fails a rewarded arm"
    );
    assert!(!excitatory_held(&reach((0, 1), (0, 0)), false));
    assert!(
        excitatory_held(&reach((0, 0), (7, 3)), false),
        "the inhibitory synapses are outside the clause"
    );
    // The readings' rules at their edges: the fall over blocks written by hand, the fraction,
    // the lean over splits written by hand, and the frozen network's lean from the table.
    let block_with = |inhibitory: i64| -> Block {
        (
            0,
            0,
            [[0; 2]; 2],
            [0; 2],
            [0; 2],
            [0; 2],
            0,
            inhibitory,
            0,
            0,
            [[0; 2]; 2],
            0,
        )
    };
    assert!(!falls_every_block(100, &[]), "no block falls");
    assert!(falls_every_block(100, &[block_with(99)]));
    assert!(
        !falls_every_block(100, &[block_with(100)]),
        "level is not a fall"
    );
    assert!(!falls_every_block(100, &[block_with(101)]));
    assert!(falls_every_block(
        100,
        &[block_with(99), block_with(98), block_with(1)]
    ));
    assert!(!falls_every_block(100, &[block_with(99), block_with(99)]));
    assert!(!falls_every_block(
        100,
        &[block_with(99), block_with(98), block_with(98)]
    ));
    assert!(!falls_every_block(100, &[block_with(99), block_with(100)]));
    assert_eq!(per_myriad(100, 100), 10_000);
    assert_eq!(per_myriad(41, 100), 4_100);
    assert_eq!(per_myriad(165_876_268, 165_876_268), 10_000);
    assert_eq!(per_myriad(68_197_381, 165_876_268), 4_111, "H-15's 0.411");
    assert_eq!(per_myriad(1, 0), 0);
    assert_eq!(
        course(100, &[block_with(99), block_with(50)]),
        vec![9_900, 5_000]
    );
    assert_eq!(lean([[3, 4, 0], [5, 5, 1]]), [Some(1), None]);
    assert_eq!(lean([[4, 3, 9], [0, 1, 0]]), [Some(0), Some(1)]);
    assert_eq!(lean([[0; 3]; 2]), [None; 2]);
    assert_eq!(frozen_lean(), FROZEN_LEAN_1024, "the frozen network's lean");
    let mut frozen_splits = [[0u32; 3]; 2];
    for &(stimulus, counts) in &BACKGROUND_COUNTED_1024[SETTLED] {
        let into =
            &mut frozen_splits[usize::from(stimulus)][selected(counts).map_or(2, usize::from)];
        *into = into.saturating_add(1);
    }
    assert_eq!(
        frozen_splits,
        [[14, 15, 5], [12, 15, 3]],
        "the calibration's block by stimulus and readout"
    );
    assert_eq!(
        frozen_splits[0].iter().sum::<u32>() + frozen_splits[1].iter().sum::<u32>(),
        BLOCK as u32
    );
    // The reach by polarity on the instrument's network: nothing moved, then one excitatory
    // weight inside an answer pair and one inhibitory weight moved by hand.
    let p = prior(1024);
    let sets = geometry(1024, ROTATION_1024);
    let mut exec = at_gain(&p, config(1024, 2, GATE_BASELINE_Q16), GAIN_1024);
    let before = weights_of(&exec);
    assert_eq!(
        reach_by_polarity(&exec, &before, 1024, &ALL_PAIRS),
        Reach::default()
    );
    // The arena's synapses by polarity, the count the reach is read against: the settled
    // network is this network after its lead-in, with every synapse where the prior put it.
    let (inhibitory_synapses, excitatory_synapses) =
        exec.units()
            .iter()
            .fold((0u64, 0u64), |(inhibitory, excitatory), unit| {
                let count = unit.fan_out(exec.blocks()).count() as u64;
                if unit.flags & FLAG_INHIBITORY != 0 {
                    (inhibitory.saturating_add(count), excitatory)
                } else {
                    (inhibitory, excitatory.saturating_add(count))
                }
            });
    eprintln!(
        "DUMP inhibition1024 synapses inhibitory {inhibitory_synapses} excitatory {excitatory_synapses}"
    );
    assert!(inhibitory_synapses > 0 && excitatory_synapses > 0);
    let mut moved: Vec<(usize, usize)> = Vec::new();
    for unit in exec.units() {
        let id = unit.id as u32;
        let inhibitory = unit.flags & FLAG_INHIBITORY != 0;
        for s in unit.fan_out(exec.blocks()) {
            let wanted = if inhibitory {
                moved.len() == 1
            } else {
                moved.is_empty() && sets[0].contains(id) && sets[2].contains(s.target)
            };
            if wanted {
                moved.push((s.block_idx as usize, usize::from(s.slot)));
                break;
            }
        }
        if moved.len() == 2 {
            break;
        }
    }
    assert_eq!(moved.len(), 2, "one synapse of each polarity");
    for &(block, slot) in &moved {
        let w = &mut exec.blocks_mut()[block].weights_q1_15[slot];
        *w = w.saturating_sub(1);
    }
    assert_eq!(
        reach_by_polarity(&exec, &before, 1024, &[(0, 0)]),
        reach((1, 0), (0, 1)),
        "A→R0 inside, the inhibitory synapse outside"
    );
    assert_eq!(
        reach_by_polarity(&exec, &before, 1024, &[(0, 1)]),
        reach((0, 1), (0, 1)),
        "against the other pair, outside"
    );
    // Eight trials with the inhibitory baseline set on the instrument's network at 1 024
    // units: the assignment, the oracle held at every trial inside `earned_run_under`; then
    // the arena against the weights before — no excitatory synapse outside the answer pairs
    // moved, the inhibitory synapses moved — and ADR-0080's derivation true; then the reward
    // withheld, no excitatory synapse moved and the inhibitory ones moved; then the reward
    // withheld with the baseline unset, nothing moved: today's rule.
    let inhibited = || Config {
        inhibitory_baseline_q16: Some(INHIBITORY_BASELINE_Q16),
        ..config(1024, 2, GATE_BASELINE_Q16)
    };
    let mut exec = at_gain(&p, inhibited(), GAIN_1024);
    assert_eq!(
        exec.inhibitory_baseline_q16(),
        Some(INHIBITORY_BASELINE_Q16)
    );
    let before = weights_of(&exec);
    let run = inhibition_run(&mut exec, Inhibition::Assignment, GATE_TRIALS);
    let (blocks, trace, trials, read, _) = &run;
    assert!(blocks.is_empty(), "eight trials are no whole block");
    assert_eq!(trials.len(), GATE_TRIALS);
    eprintln!("DUMP inhibition1024 first eight trace {trace:#018x} read {read:?}");
    let reach_read = reach_by_polarity(
        &exec,
        &before,
        1024,
        &inhibition_pairs(Inhibition::Assignment),
    );
    eprintln!("DUMP inhibition1024 first eight reach {reach_read:?}");
    assert!(
        excitatory_held(&reach_read, true),
        "no excitatory synapse outside the answer pairs moved: {reach_read:?}"
    );
    assert!(
        reach_read.inhibitory.1 > 0,
        "the inhibitory synapses consolidated under their baseline: {reach_read:?}"
    );
    assert_eq!(
        reach_read.inhibitory.0, 0,
        "no inhibitory synapse is inside a pair: a stimulus unit is excitatory"
    );
    assert_eq!(derivation(read), [true; 3], "ADR-0080's derivation");
    assert!(
        read.iter().all(|t| t.4.abs() == REWARD_Q16),
        "every reward is 1.0 in magnitude"
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
        read.iter().all(|t| t.5[0][1] == 0 && t.5[1][0] == 0),
        "no wrong pair consolidated under the gate"
    );
    let mut withheld = at_gain(&p, inhibited(), GAIN_1024);
    let before = weights_of(&withheld);
    let run = inhibition_run(&mut withheld, Inhibition::Withheld, GATE_TRIALS);
    let (_, trace, _, read, _) = &run;
    eprintln!("DUMP inhibition1024 first eight withheld trace {trace:#018x} read {read:?}");
    let reach_read = reach_by_polarity(&withheld, &before, 1024, &ALL_PAIRS);
    eprintln!("DUMP inhibition1024 first eight withheld reach {reach_read:?}");
    assert!(
        excitatory_held(&reach_read, false),
        "no excitatory synapse moved with the reward withheld: {reach_read:?}"
    );
    assert!(
        reach_read.inhibitory.1 > 0,
        "and the inhibitory synapses still consolidated: {reach_read:?}"
    );
    assert!(
        read.iter().all(|t| t.4 == 0 && t.6 == 0 && t.7 == 0),
        "no reward, the signal at rest"
    );
    assert!(
        read.iter()
            .all(|t| t.5.iter().flatten().all(|&amount| amount == 0)),
        "the pairs consolidated nothing under the gate"
    );
    let mut unset = at_gain(&p, config(1024, 2, GATE_BASELINE_Q16), GAIN_1024);
    assert_eq!(unset.inhibitory_baseline_q16(), None);
    let before = weights_of(&unset);
    let run = inhibition_run(&mut unset, Inhibition::Withheld, GATE_TRIALS);
    let (_, trace, _, _, _) = &run;
    eprintln!("DUMP inhibition1024 first eight unset trace {trace:#018x}");
    assert_eq!(
        reach_by_polarity(&unset, &before, 1024, &ALL_PAIRS),
        Reach::default(),
        "unset, nothing moves with the reward withheld under the gate: today's rule"
    );
    // Over the pinned tables: the verdict as written, by the rules; the correct selections,
    // the crossings, the splits, the derivation, the fall, the lean, the reach and the sums
    // as the constants state, the two predicted readings beside what was read — the fall as
    // predicted, the lean not the frozen network's for A; each arm's twenty-four blocks with
    // the same trials presenting A; the inhibitory sum below the block before in every block
    // of every arm; in a rewarded arm the two wrong pairs' couplings the image's to the LSB
    // in every block and the answer pairs' above the block before; in the withheld arm every
    // coupling and the excitatory sum the image's in every block, its rewards zero and its
    // signal at rest; the excitatory sum's rise in a rewarded arm exactly the answer pairs';
    // and each block's earned readings consistent with its sight's block.
    let verdict = reinforced([INHIBITION_BLOCKS_1024[0], INHIBITION_BLOCKS_1024[1]]);
    assert_eq!(verdict, INHIBITION_1024, "the verdict as written");
    assert!(verdict.yes, "H-16 is yes");
    assert_eq!(verdict.yes, INHIBITION_PREDICTED, "as ADR-0085 predicted");
    assert_eq!(
        [
            last_correct(INHIBITION_BLOCKS_1024[0]),
            last_correct(INHIBITION_BLOCKS_1024[1]),
        ],
        CORRECT_LAST_INHIBITION_1024
    );
    assert!(
        CORRECT_LAST_INHIBITION_1024
            .iter()
            .all(|&correct| correct >= REWARDED_MIN)
    );
    assert_eq!(
        [
            crossing(INHIBITION_BLOCKS_1024[0]),
            crossing(INHIBITION_BLOCKS_1024[1]),
            crossing(INHIBITION_BLOCKS_1024[2]),
        ],
        CROSSED_INHIBITION_1024
    );
    let image = QUIET_1024[SETTLED].1;
    assert_eq!(
        [
            falls_every_block(image.0, INHIBITION_BLOCKS_1024[0]),
            falls_every_block(image.0, INHIBITION_BLOCKS_1024[1]),
            falls_every_block(image.0, INHIBITION_BLOCKS_1024[2]),
        ],
        FALLS_1024
    );
    assert_eq!(
        FALLS_1024, FALLS_PREDICTED,
        "the inhibitory sum fell in every block of every arm, as predicted"
    );
    assert_eq!(lean(SPLITS_LAST_INHIBITION_1024[WITHHELD]), LEAN_1024);
    assert_ne!(
        LEAN_1024, FROZEN_LEAN_1024,
        "the withheld arm's lean is not the frozen network's for A: the predicted reading did not hold as written, and is recorded so"
    );
    assert_eq!(LEAN_1024[1], FROZEN_LEAN_1024[1], "and is for B");
    assert_eq!(DERIVATION_INHIBITION_1024, [[true; 3]; 3]);
    let assignment = INHIBITION_BLOCKS_1024[0];
    for (k, &arm) in INHIBITION_ARMS.iter().enumerate() {
        let blocks = INHIBITION_BLOCKS_1024[k];
        let earned = INHIBITION_EARNED_1024[k];
        let compositions = INHIBITION_COMPOSITIONS_1024[k];
        let rewarded = inhibition_rewards(arm);
        let mirrored = inhibition_mirrors(arm);
        assert_eq!(
            blocks.len(),
            INHIBITION_TRIALS / BLOCK,
            "{arm:?}: twenty-four blocks"
        );
        assert_eq!(earned.len(), INHIBITION_TRIALS / BLOCK);
        assert_eq!(compositions.len(), INHIBITION_TRIALS / BLOCK);
        assert_ne!(INHIBITION_TRACES_1024[k], 0);
        assert_ne!(INHIBITION_READ_1024[k], 0);
        assert!(!INHIBITION_CENSUS_1024[k].is_empty());
        let reach = REACH_INHIBITION_1024[k];
        assert!(
            excitatory_held(&reach, rewarded),
            "{arm:?}: the assertion over the pinned reach"
        );
        assert!(
            reach.inhibitory.1 > 0 && reach.inhibitory.0 == 0,
            "{arm:?}: the inhibitory synapses moved, none inside a pair"
        );
        if rewarded {
            let pairs = assigned_pairs(mirrored);
            let synapses = u64::from(SYNAPSES_1024[pairs[0].0][pairs[0].1])
                .saturating_add(u64::from(SYNAPSES_1024[pairs[1].0][pairs[1].1]));
            assert_eq!(
                reach.excitatory.0, synapses,
                "{arm:?}: every synapse of the two answer pairs moved"
            );
        } else {
            assert_eq!(reach.excitatory, (0, 0));
        }
        let last = blocks.last().expect("a block");
        assert_eq!(
            SUMS_AFTER_1024[k],
            (last.7, last.8),
            "{arm:?}: the sums after the run are the last block's"
        );
        assert!(
            SUMS_AFTER_1024[k].0 < image.0,
            "{arm:?}: the inhibitory sum fell"
        );
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
            last_two, SPLITS_LAST_INHIBITION_1024[k],
            "{arm:?}: the last blocks' splits"
        );
        let mut previous_couplings = IMAGE_COUPLINGS_1024;
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
            assert!(earned[j].3 <= SIGNAL_END_FIXED_Q16);
            assert!(compositions[j].3 <= BLOCK as u32);
            for &(s, r) in &ALL_PAIRS {
                let answer = rewarded && answer_of(s as u8, mirrored) == r;
                if answer {
                    assert!(
                        block.10[s][r] > previous_couplings[s][r],
                        "{arm:?} block {j}: the answer pair {s}→{r} rose"
                    );
                } else {
                    assert_eq!(
                        block.10[s][r], IMAGE_COUPLINGS_1024[s][r],
                        "{arm:?} block {j}: the coupling {s}→{r} is the image's to the LSB"
                    );
                }
            }
            previous_couplings = block.10;
            if rewarded {
                let correct =
                    earned[j].0[0][answer_of(0, mirrored)] + earned[j].0[1][answer_of(1, mirrored)];
                assert_eq!(block.0, correct, "{arm:?} block {j}: the correct trials");
                assert_eq!(
                    earned[j].1, correct,
                    "{arm:?} block {j}: one positive reward per correct trial"
                );
                let pairs = assigned_pairs(mirrored);
                let rise = block.10[pairs[0].0][pairs[0].1]
                    .saturating_sub(IMAGE_COUPLINGS_1024[pairs[0].0][pairs[0].1])
                    .saturating_add(
                        block.10[pairs[1].0][pairs[1].1]
                            .saturating_sub(IMAGE_COUPLINGS_1024[pairs[1].0][pairs[1].1]),
                    );
                assert_eq!(
                    block.8,
                    image.1.saturating_add(rise),
                    "{arm:?} block {j}: the excitatory sum rose by exactly the answer pairs' rise"
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
                assert_eq!(
                    block.8, image.1,
                    "{arm:?} block {j}: the excitatory sum is the image's"
                );
            }
        }
    }
}

// =================================================================================== H-17

// ------------------------------------------ written before the run (ADR-0089, ADR-0090)

/// H-17's run (ADR-0089): two of H-16's, forty-eight blocks.
const REVERSAL_TRIALS: usize = 2 * INHIBITION_TRIALS;
const _: () = assert!(REVERSAL_TRIALS == 3_072 && REVERSAL_TRIALS / BLOCK == 48);
/// The blocks of an H-17 run.
const REVERSAL_BLOCKS: usize = REVERSAL_TRIALS / BLOCK;

/// The flip (ADR-0089): the index of the first trial under the second mapping, the 1 537th,
/// the flip falling between the 1 536th and it. The trials before it are H-16's arm of the
/// first mapping, bit for bit (ADR-0090).
const FLIP: usize = INHIBITION_TRIALS;
/// The index of the last trial under the first mapping, the 1 536th, whose delivery the
/// 1 537th consolidates (ADR-0090).
const LAST_BEFORE_FLIP: usize = FLIP - 1;
/// The first block under the second mapping, the twenty-fifth; the twenty-four before it are
/// H-16's.
const FLIP_BLOCK: usize = FLIP / BLOCK;
const _: () = assert!(FLIP == 1_536 && FLIP % BLOCK == 0 && FLIP_BLOCK == 24);
const _: () = assert!(FLIP_BLOCK >= LAST_BLOCKS && REVERSAL_BLOCKS - FLIP_BLOCK >= LAST_BLOCKS);

/// The arms of H-17 (ADR-0089), each its own weekly test (ADR-0088's floor is the longest
/// test): the assignment first — A onto readout 0 and B onto readout 1 for the first 1 536
/// trials, the mirrored mapping for the last 1 536 — and the mirrored first, the other way
/// round; both `Feedback::Answer` under the task's own delivery from the one inhibited image,
/// the flip `Task::mirrored` negated and nothing else. No withheld arm.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Reversal {
    AssignmentFirst,
    MirroredFirst,
}
const REVERSAL_ARMS: [Reversal; 2] = [Reversal::AssignmentFirst, Reversal::MirroredFirst];

/// The mapping an arm starts under, the task's `mirrored` before the flip; after it, the
/// other.
fn first_mapping(arm: Reversal) -> bool {
    arm == Reversal::MirroredFirst
}

/// The H-16 arm an H-17 arm's first half is, by its index in `INHIBITION_ARMS` (ADR-0090):
/// the assignment for the assignment first, the mirrored assignment for the mirrored first.
fn replicated_arm(arm: Reversal) -> usize {
    usize::from(first_mapping(arm))
}

/// ADR-0089's prediction, a Hypothesis written before the run: H-17 is no.
const REVERSAL_PREDICTED: bool = false;

// ------------------------------------------------------------ the criterion (ADR-0089)

/// Clause 1's count (ADR-0089): the correct selections over the last `LAST_BLOCKS` blocks
/// before the flip, trials 1 409 to 1 536, under the first mapping; zero for a run that does
/// not reach the flip.
fn correct_before(blocks: &[Block]) -> u32 {
    blocks.get(..FLIP_BLOCK).map_or(0, last_correct)
}

/// Clause 2's count: the correct selections over the last `LAST_BLOCKS` blocks of the run,
/// trials 2 945 to 3 072, under the second mapping; zero for a run of any other length.
fn correct_after(blocks: &[Block]) -> u32 {
    if blocks.len() == REVERSAL_BLOCKS {
        last_correct(blocks)
    } else {
        0
    }
}

/// H-17's criterion (ADR-0089), clause by clause per arm `[assignment first, mirrored
/// first]`: (1) it learned — `correct_before` at least `REWARDED_MIN`; (2) it revised —
/// `correct_after` at least `REWARDED_MIN`; a tie not correct, the task's count. `replicated`
/// is clause 1 in both arms, H-16's result reproduced; `yes` is all four. A run whose
/// `replicated` is false is a failure to replicate H-16 and answers nothing of H-17.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Revision {
    learned: [bool; 2],
    revised: [bool; 2],
    replicated: bool,
    yes: bool,
}

fn revision(arms: [&[Block]; 2]) -> Revision {
    let learned = arms.map(|blocks| correct_before(blocks) >= REWARDED_MIN);
    let revised = arms.map(|blocks| correct_after(blocks) >= REWARDED_MIN);
    let replicated = learned[0] && learned[1];
    Revision {
        learned,
        revised,
        replicated,
        yes: replicated && revised[0] && revised[1],
    }
}

// ------------------------------------------------ the assertion's shape (ADR-0090)

/// The weights and the four couplings at a trial's end: what the assertion compares at the
/// 1 536th and the 1 537th trials.
type Snapshot = (Vec<Vec<i16>>, [[i64; 2]; 2]);

/// The four stimulus–readout couplings of `exec`, `[stimulus][readout]`, as a block reads
/// them.
fn pair_couplings(exec: &Engine, sets: &[Set; 4]) -> [[i64; 2]; 2] {
    [
        [
            coupling(exec, sets[0], sets[2]),
            coupling(exec, sets[0], sets[3]),
        ],
        [
            coupling(exec, sets[1], sets[2]),
            coupling(exec, sets[1], sets[3]),
        ],
    ]
}

/// The pair the 1 536th trial's delivery carries into the 1 537th (ADR-0090): the stimulus it
/// presented onto the readout it selected, when it was rewarded; none otherwise.
fn carried_pair(last: &EarnedTrial) -> Option<(usize, usize)> {
    if last.4 > 0 {
        last.2.map(|r| (usize::from(last.0), usize::from(r)))
    } else {
        None
    }
}

/// The old answer's pairs the carry-over does not reach: both, when the 1 536th trial
/// carried none.
fn uncarried(old: [(usize, usize); 2], carried: Option<(usize, usize)>) -> Vec<(usize, usize)> {
    old.into_iter()
        .filter(|&pair| Some(pair) != carried)
        .collect()
}

/// ADR-0090's assertion read over a run, clause by clause, as the synapses that moved: (a) of
/// the two old answer pairs, from the end of the 1 537th trial to the run's end; (b) of the
/// old answer pair the carry-over does not reach, from the end of the 1 536th; (c) the
/// excitatory synapses outside the four stimulus–readout pairs, from the image's. Beside
/// them, a reading and no clause: the synapses of the old answer's pairs the 1 537th trial
/// moved.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct Held {
    after_carry: u64,
    uncarried: u64,
    outside: u64,
    carried: u64,
}

/// The assertion as a rule: (a), (b) and (c) each moved nothing. True on both arms is the
/// assertion; false on one is a finding against ADR-0080's derivation as ADR-0089 and
/// ADR-0090 extend it, reported beside the verdict and not in place of it.
fn reversal_held(held: &Held) -> bool {
    held.after_carry == 0 && held.uncarried == 0 && held.outside == 0
}

/// The assertion's reach over an arm's run: the executor at the run's end against the
/// image's weights and the weights at the ends of the 1 536th and the 1 537th trials, the
/// pairs named by the arm's first mapping and the 1 536th trial's reading; `carried` is the
/// synapses of the old answer's pairs that moved in the 1 537th trial, read at its end.
fn held_of(
    exec: &Engine,
    image: &[Vec<i16>],
    at_flip: &[Vec<i16>],
    at_carry: &[Vec<i16>],
    last: &EarnedTrial,
    arm: Reversal,
    carried: u64,
) -> Held {
    let old = assigned_pairs(first_mapping(arm));
    Held {
        after_carry: reach(exec, at_carry, 1024, &old).0,
        uncarried: reach(exec, at_flip, 1024, &uncarried(old, carried_pair(last))).0,
        outside: reach_by_polarity(exec, image, 1024, &ALL_PAIRS)
            .excitatory
            .1,
        carried,
    }
}

// --------------------------------------------- the readings' shape (ADR-0089): the need

/// The measured need after the flip (ADR-0089), per arm: the trials in which the engine
/// selected the readout that is the answer under the second mapping (the task's correct
/// count), the trials in which it selected the old answer, the ties, the rewards it earned,
/// and the trial at whose block's end the selection first passed `CROSSING_MARK` after the
/// flip, none when no block did.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct Need {
    selected_new: u32,
    selected_old: u32,
    ties: u32,
    rewards: u32,
    crossed: Option<usize>,
}

fn need(blocks: &[Block], earned: &[EarnedBlock]) -> Need {
    let after = blocks.get(FLIP_BLOCK..).unwrap_or(&[]);
    let selected_new = after.iter().fold(0u32, |sum, b| sum.saturating_add(b.0));
    let ties = after.iter().fold(0u32, |sum, b| sum.saturating_add(b.11));
    let trials = (after.len() as u32).saturating_mul(BLOCK as u32);
    Need {
        selected_new,
        selected_old: trials.saturating_sub(selected_new).saturating_sub(ties),
        ties,
        rewards: earned
            .get(FLIP_BLOCK..)
            .unwrap_or(&[])
            .iter()
            .fold(0u32, |sum, b| sum.saturating_add(b.1)),
        crossed: crossing(after).map(|t| t.saturating_add(FLIP)),
    }
}

/// The first trial after the flip, by index, that earned a reward; none when none did.
fn first_reward(read: &[EarnedTrial]) -> Option<usize> {
    read.iter()
        .enumerate()
        .skip(FLIP)
        .find(|(_, t)| t.4 > 0)
        .map(|(k, _)| k)
}

/// The counts' gap per block (ADR-0089): for each stimulus, the spikes over the block of the
/// readout that is its answer under the first mapping less the other readout's, `[A, B]` —
/// the lead the first mapping's learning built before the flip, and the lead the old answer
/// keeps after it.
fn gaps(blocks: &[Block], first: bool) -> Vec<[i64; 2]> {
    blocks
        .iter()
        .map(|b| {
            [0u8, 1].map(|s| {
                let old = answer_of(s, first);
                let split = b.2[usize::from(s)];
                (split[old] as i64).saturating_sub(split[old ^ 1] as i64)
            })
        })
        .collect()
}

/// The new answer's pairs' couplings at the run's end less at the flip, `[A, B]`: whether, and
/// by how much, the rewards the second mapping earned raised them; zero for a run that does
/// not pass the flip.
fn new_rise(blocks: &[Block], first: bool) -> [i64; 2] {
    let (Some(flip), Some(last)) = (
        blocks.get(FLIP_BLOCK - 1),
        blocks.get(FLIP_BLOCK..).and_then(|after| after.last()),
    ) else {
        return [0; 2];
    };
    [0u8, 1].map(|s| {
        let new = answer_of(s, !first);
        last.10[usize::from(s)][new].saturating_sub(flip.10[usize::from(s)][new])
    })
}

/// The blocks in which the stimulus fired once by ADR-0074's measure (`fires_once`), of a
/// run's blocks and their compositions.
fn once_blocks(blocks: &[Block], compositions: &[Composition]) -> u32 {
    blocks
        .iter()
        .zip(compositions.iter())
        .filter(|(b, c)| fires_once(1024, b, c))
        .count() as u32
}

// ------------------------------------------------------------------ the run (brief 040)

/// An image with its header's version written as `version` and the header resealed, every
/// other byte as it was (ADR-0095): the image a writer of that version would have produced
/// from the same records, where no record's bytes differ between the two versions.
fn with_version(image: &[u8], version: u32) -> Vec<u8> {
    let mut img = image.to_vec();
    let mut header = CortexFileHeader::decode(img[0..64].try_into().unwrap());
    header.version = version;
    header.crc64 = header.checksum();
    img[0..64].copy_from_slice(&header.encode());
    img
}

/// An arm's run from the inhibited engine (brief 040): `earned_run_flipped` under the
/// answer's feedback at the gate's zero, the arm's first mapping, the flip before the trial
/// of index `flip`, and `after` reading the executor at every trial's end — the oracle held
/// to the record at every trial and every trial's contract asserted under the mapping in
/// force at it.
fn reversal_run(
    exec: &mut Engine,
    arm: Reversal,
    trials: usize,
    flip: usize,
    after: &mut dyn FnMut(&Engine, usize),
) -> EarnedRun {
    earned_run_flipped(
        exec,
        Feedback::Answer,
        first_mapping(arm),
        1024,
        trials,
        GATE_BASELINE_Q16,
        Some(flip),
        after,
    )
}

/// One arm of H-17 at 1 024 units (brief 040): the settled engine held to ADR-0077 step by
/// step and its two images; the calibration — a frozen block from the zero image, the
/// inhibitory baseline unset, the reward withheld, held to ADR-0077's frozen run — before
/// any rewarded run (H-17's stopping rule, step 2); then the arm's 3 072 trials from the
/// inhibited image, the flip between the 1 536th and the 1 537th, the weights and the four
/// couplings read at the ends of those two trials; everything dumped and the clauses, the
/// assertion and the readings computed before anything is held; then the first half held to
/// H-16's arm (ADR-0090), the assertion, and the pinned tables of the second half.
fn reversal_arm(arm: Reversal) {
    let k = REVERSAL_ARMS
        .iter()
        .position(|&a| a == arm)
        .expect("an arm of H-17");
    let name = format!("reversal1024 {arm:?}");
    let (zero, inhibited) = inhibited_images(&name);
    let image_crc = crc64(&inhibited);
    {
        let mut frozen = frozen_from(&zero, 1024);
        assert_eq!(
            frozen.inhibitory_baseline_q16(),
            None,
            "{name}: the calibration's image leaves the inhibitory baseline unset"
        );
        let calibration = taught_run(&mut frozen, Arm::Withheld, 1024, BLOCK);
        calibration_holds(&format!("{name} calibration"), &calibration);
        eprintln!(
            "DUMP {name} calibration holds: ADR-0077's settled candidate reproduced; image crc {image_crc:#018x}"
        );
    }
    let sets = geometry(1024, ROTATION_1024);
    let image_sums = QUIET_1024[SETTLED].1;
    let mut exec = inhibited_from(&inhibited, 1024);
    assert_eq!(
        weights_by_polarity(&exec),
        image_sums,
        "{name}: the image's sums"
    );
    assert_eq!(
        pair_couplings(&exec, &sets),
        IMAGE_COUPLINGS_1024,
        "{name}: the same image"
    );
    let image = weights_of(&exec);
    let first = first_mapping(arm);
    let old = assigned_pairs(first);
    let mut at_flip: Option<Snapshot> = None;
    let mut at_carry: Option<Snapshot> = None;
    let mut carried = 0u64;
    let run = reversal_run(&mut exec, arm, REVERSAL_TRIALS, FLIP, &mut |exec, trial| {
        if trial == LAST_BEFORE_FLIP {
            at_flip = Some((weights_of(exec), pair_couplings(exec, &sets)));
        } else if trial == FLIP {
            let before = at_flip.as_ref().expect("the 1 536th trial's reading first");
            carried = reach(exec, &before.0, 1024, &old).0;
            at_carry = Some((weights_of(exec), pair_couplings(exec, &sets)));
        }
    });
    let (blocks, trace, trials, read, volley_ticks) = &run;
    let at_flip = at_flip.expect("the run reached the flip");
    let at_carry = at_carry.expect("and the trial after it");
    let earned = earned_blocks(read);
    let compositions: Vec<Composition> = trials.chunks(BLOCK).map(composition).collect();
    assert_eq!(blocks.len(), REVERSAL_BLOCKS, "{name}: forty-eight blocks");
    assert_eq!(read.len(), REVERSAL_TRIALS);
    // Everything dumped, and the clauses, the assertion and the readings computed, before
    // anything is held.
    dump_earned(&name, &run, &earned);
    eprintln!("DUMP {name} PIN blocks {:?}", &blocks[FLIP_BLOCK..]);
    eprintln!("DUMP {name} PIN trace {trace:#018x}");
    eprintln!(
        "DUMP {name} PIN compositions {:?}",
        &compositions[FLIP_BLOCK..]
    );
    eprintln!("DUMP {name} PIN earned {:?}", &earned[FLIP_BLOCK..]);
    eprintln!("DUMP {name} PIN read {:#018x}", earned_hash(&read[FLIP..]));
    eprintln!("DUMP {name} PIN census {:?}", census_of(volley_ticks));
    eprintln!("DUMP {name} PIN snapshots {:?}", (at_flip.1, at_carry.1));
    let last = &read[LAST_BEFORE_FLIP];
    let held = held_of(&exec, &image, &at_flip.0, &at_carry.0, last, arm, carried);
    let correct = [correct_before(blocks), correct_after(blocks)];
    let need_read = need(blocks, &earned);
    let first_rewarded = first_reward(read);
    let carried_read = carried_pair(last);
    let new_rise_read = new_rise(blocks, first);
    let once = once_blocks(blocks, &compositions);
    let falls = falls_every_block(image_sums.0, blocks);
    let derivation_read = derivation(read);
    let sums_after = weights_by_polarity(&exec);
    eprintln!(
        "DUMP {name} PIN readings correct {correct:?} held {held:?} need {need_read:?} first reward {first_rewarded:?} carried {carried_read:?} new rise {new_rise_read:?} once {once} falls {falls} derivation {derivation_read:?} image crc {image_crc:#018x}"
    );
    eprintln!(
        "DUMP {name} gaps {:?} course {:?} sums after {sums_after:?} image {image_sums:?} last splits {:?}",
        gaps(blocks, first),
        course(image_sums.0, blocks),
        last_splits(read)
    );
    // The first half is H-16's arm of the first mapping, bit for bit (ADR-0090): clause 1 is
    // H-16's 128, and a mismatch here is a failure to replicate H-16.
    let h16 = replicated_arm(arm);
    assert_eq!(
        &blocks[..FLIP_BLOCK],
        INHIBITION_BLOCKS_1024[h16],
        "{name}: the first half is H-16's arm, block by block"
    );
    assert_eq!(
        &compositions[..FLIP_BLOCK],
        INHIBITION_COMPOSITIONS_1024[h16],
        "{name}: and its composition"
    );
    assert_eq!(
        &earned[..FLIP_BLOCK],
        INHIBITION_EARNED_1024[h16],
        "{name}: and its earned blocks"
    );
    assert_eq!(
        earned_hash(&read[..FLIP]),
        INHIBITION_READ_1024[h16],
        "{name}: and its readings"
    );
    // The assertion (ADR-0090), after the dump and beside the verdict.
    assert!(
        reversal_held(&held),
        "{name}: ADR-0090's assertion — the old answer's pairs held from the 1 537th trial, the uncarried one from the 1 536th, no excitatory synapse outside the four pairs moved: {held:?}"
    );
    // The pinned tables of the second half, and the readings as the constants state.
    pinned(
        &format!("{name} sight"),
        &blocks[FLIP_BLOCK..],
        *trace,
        REVERSAL_BLOCKS_1024[k],
        REVERSAL_TRACES_1024[k],
    );
    assert_eq!(
        &compositions[FLIP_BLOCK..],
        REVERSAL_COMPOSITIONS_1024[k],
        "{name}: the composition per block"
    );
    assert_eq!(
        &earned[FLIP_BLOCK..],
        REVERSAL_EARNED_1024[k],
        "{name}: the earned blocks"
    );
    assert_eq!(
        earned_hash(&read[FLIP..]),
        REVERSAL_READ_1024[k],
        "{name}: the readings"
    );
    assert_eq!(
        census_of(volley_ticks),
        REVERSAL_CENSUS_1024[k].to_vec(),
        "{name}: the volley's ticks"
    );
    assert_eq!(image_crc, REVERSAL_IMAGE_CRC_1024, "{name}: the one image");
    assert_eq!(
        crc64(&with_version(&inhibited, 15)),
        REVERSAL_IMAGE_CRC_FORMAT_15_1024,
        "{name}: every byte but the header's version and seal is the image H-17 read (ADR-0095)"
    );
    assert_eq!((at_flip.1, at_carry.1), REVERSAL_SNAPSHOTS_1024[k]);
    assert_eq!(correct, CORRECT_REVERSAL_1024[k]);
    assert_eq!(held, HELD_1024[k]);
    assert_eq!(need_read, NEED_1024[k]);
    assert_eq!(first_rewarded, FIRST_REWARD_1024[k]);
    assert_eq!(carried_read, CARRIED_1024[k]);
    assert_eq!(new_rise_read, NEW_RISE_1024[k]);
    assert_eq!(once, ONCE_BLOCKS_REVERSAL_1024[k]);
    assert_eq!(falls, FALLS_REVERSAL_1024[k]);
    assert_eq!(derivation_read, DERIVATION_REVERSAL_1024[k]);
    assert_eq!(
        (sums_after, blocks.last().map(|b| (b.7, b.8))),
        (
            SUMS_AFTER_REVERSAL_1024[k],
            Some(SUMS_AFTER_REVERSAL_1024[k])
        ),
        "{name}: the sums after the run are the last block's"
    );
}

/// H-17's arm that starts from the assignment (brief 040): A onto readout 0 and B onto
/// readout 1 for 1 536 trials, then the mirrored mapping for 1 536.
#[test]
#[ignore]
fn the_assignment_reversed_from_the_assignment_at_1024_units_exhaustive() {
    reversal_arm(Reversal::AssignmentFirst);
}

/// H-17's arm that starts from the mirrored assignment (brief 040): A onto readout 1 and B
/// onto readout 0 for 1 536 trials, then the assignment for 1 536.
#[test]
#[ignore]
fn the_assignment_reversed_from_the_mirrored_assignment_at_1024_units_exhaustive() {
    reversal_arm(Reversal::MirroredFirst);
}

// ----------------------------------------------------------- the measurement (brief 040)

/// The two arms at 1 024 units, in `REVERSAL_ARMS`'s order, each pinned from one run over
/// its second half — the first half being H-16's arm, held to `INHIBITION_BLOCKS_1024` and
/// its companions: the sight's blocks from the twenty-fifth and the whole run's trace, the
/// composition and the earned blocks from the twenty-fifth, the hash of the readings from
/// the 1 537th trial, and the whole run's volley census. The constants above were committed
/// before the first rewarded run, and these tables after it.
const REVERSAL_BLOCKS_1024: [&[Block]; 2] = [
    &[
        (
            0,
            29,
            [[859, 382], [453, 1106]],
            [1476, 1782],
            [2351, 2510],
            [291, 265],
            64,
            66644319,
            222956116,
            -112227,
            [[8475315, 6698611], [6584205, 9302469]],
            0,
        ),
        (
            0,
            33,
            [[965, 434], [452, 990]],
            [1676, 1579],
            [2515, 2350],
            [259, 278],
            64,
            63658202,
            222956116,
            -112227,
            [[8475315, 6698611], [6584205, 9302469]],
            0,
        ),
        (
            0,
            36,
            [[1036, 470], [370, 911]],
            [1824, 1426],
            [2656, 2170],
            [249, 261],
            64,
            60944296,
            222956116,
            -112227,
            [[8475315, 6698611], [6584205, 9302469]],
            0,
        ),
        (
            1,
            32,
            [[926, 467], [450, 1021]],
            [1629, 1625],
            [2497, 2385],
            [263, 270],
            64,
            58279302,
            222959531,
            -112227,
            [[8475315, 6702026], [6584205, 9302469]],
            0,
        ),
        (
            0,
            34,
            [[983, 496], [396, 940]],
            [1730, 1524],
            [2545, 2288],
            [258, 287],
            64,
            55857459,
            222959531,
            -112227,
            [[8475315, 6702026], [6584205, 9302469]],
            0,
        ),
        (
            0,
            34,
            [[995, 423], [404, 921]],
            [1726, 1520],
            [2597, 2318],
            [280, 273],
            64,
            53454209,
            222959531,
            -112227,
            [[8475315, 6702026], [6584205, 9302469]],
            0,
        ),
        (
            0,
            35,
            [[1010, 464], [409, 935]],
            [1780, 1474],
            [2586, 2202],
            [321, 278],
            64,
            51178571,
            222959531,
            -112227,
            [[8475315, 6702026], [6584205, 9302469]],
            0,
        ),
        (
            0,
            29,
            [[845, 400], [441, 1117]],
            [1477, 1774],
            [2331, 2534],
            [276, 236],
            64,
            48989528,
            222959531,
            -112227,
            [[8475315, 6702026], [6584205, 9302469]],
            0,
        ),
        (
            0,
            32,
            [[927, 437], [406, 985]],
            [1626, 1626],
            [2432, 2399],
            [288, 270],
            64,
            46947937,
            222959531,
            -112227,
            [[8475315, 6702026], [6584205, 9302469]],
            0,
        ),
        (
            0,
            28,
            [[843, 416], [485, 1173]],
            [1427, 1829],
            [2254, 2585],
            [279, 270],
            64,
            44964975,
            222959531,
            -112227,
            [[8475315, 6702026], [6584205, 9302469]],
            0,
        ),
        (
            0,
            29,
            [[854, 410], [490, 1093]],
            [1471, 1781],
            [2313, 2556],
            [241, 287],
            64,
            43020089,
            222959531,
            -112227,
            [[8475315, 6702026], [6584205, 9302469]],
            0,
        ),
        (
            0,
            32,
            [[940, 440], [435, 1011]],
            [1622, 1627],
            [2444, 2415],
            [269, 287],
            64,
            41263652,
            222959531,
            -112227,
            [[8475315, 6702026], [6584205, 9302469]],
            0,
        ),
        (
            0,
            32,
            [[938, 428], [433, 1002]],
            [1627, 1622],
            [2486, 2390],
            [254, 251],
            64,
            39701449,
            222959531,
            -112227,
            [[8475315, 6702026], [6584205, 9302469]],
            0,
        ),
        (
            0,
            34,
            [[951, 431], [404, 960]],
            [1726, 1526],
            [2553, 2276],
            [263, 283],
            64,
            38154053,
            222959531,
            -112227,
            [[8475315, 6702026], [6584205, 9302469]],
            0,
        ),
        (
            0,
            32,
            [[938, 435], [436, 987]],
            [1627, 1622],
            [2519, 2383],
            [275, 302],
            64,
            36868984,
            222959531,
            -112227,
            [[8475315, 6702026], [6584205, 9302469]],
            0,
        ),
        (
            0,
            29,
            [[876, 373], [472, 1113]],
            [1475, 1777],
            [2355, 2520],
            [300, 295],
            64,
            35581507,
            222959531,
            -112227,
            [[8475315, 6702026], [6584205, 9302469]],
            0,
        ),
        (
            0,
            30,
            [[907, 420], [434, 1071]],
            [1524, 1730],
            [2417, 2544],
            [274, 298],
            64,
            34445626,
            222959531,
            -112227,
            [[8475315, 6702026], [6584205, 9302469]],
            0,
        ),
        (
            0,
            30,
            [[886, 387], [490, 1092]],
            [1526, 1730],
            [2399, 2496],
            [300, 260],
            64,
            33391918,
            222959531,
            -112227,
            [[8475315, 6702026], [6584205, 9302469]],
            0,
        ),
        (
            0,
            32,
            [[933, 447], [464, 1028]],
            [1629, 1630],
            [2429, 2402],
            [303, 254],
            64,
            32382419,
            222959531,
            -112227,
            [[8475315, 6702026], [6584205, 9302469]],
            0,
        ),
        (
            0,
            31,
            [[922, 427], [452, 1047]],
            [1578, 1678],
            [2405, 2442],
            [269, 286],
            64,
            31493941,
            222959531,
            -112227,
            [[8475315, 6702026], [6584205, 9302469]],
            0,
        ),
        (
            0,
            27,
            [[786, 357], [484, 1097]],
            [1371, 1879],
            [2263, 2692],
            [297, 249],
            64,
            30670768,
            222959531,
            -112227,
            [[8475315, 6702026], [6584205, 9302469]],
            0,
        ),
        (
            1,
            24,
            [[734, 332], [574, 1211]],
            [1218, 2029],
            [2105, 2735],
            [283, 252],
            64,
            29943622,
            222960156,
            -112227,
            [[8475315, 6702026], [6584830, 9302469]],
            0,
        ),
        (
            0,
            34,
            [[987, 461], [438, 1006]],
            [1726, 1524],
            [2607, 2298],
            [288, 292],
            64,
            29317768,
            222960156,
            -112227,
            [[8475315, 6702026], [6584830, 9302469]],
            0,
        ),
        (
            0,
            33,
            [[985, 431], [425, 1020]],
            [1679, 1577],
            [2549, 2361],
            [271, 272],
            64,
            28694627,
            222960156,
            -112227,
            [[8475315, 6702026], [6584830, 9302469]],
            0,
        ),
    ],
    &[
        (
            0,
            29,
            [[319, 1009], [975, 458]],
            [1476, 1781],
            [2345, 2516],
            [290, 280],
            64,
            64827559,
            223537098,
            -112227,
            [[6249552, 9743362], [8833198, 6815470]],
            0,
        ),
        (
            0,
            33,
            [[380, 1168], [918, 395]],
            [1676, 1579],
            [2513, 2357],
            [266, 280],
            64,
            61958504,
            223537098,
            -112227,
            [[6249552, 9743362], [8833198, 6815470]],
            0,
        ),
        (
            0,
            36,
            [[393, 1210], [819, 376]],
            [1825, 1426],
            [2672, 2165],
            [254, 275],
            64,
            59396423,
            223537098,
            -112227,
            [[6249552, 9743362], [8833198, 6815470]],
            0,
        ),
        (
            0,
            32,
            [[326, 1142], [950, 422]],
            [1629, 1625],
            [2488, 2382],
            [262, 272],
            64,
            56889683,
            223537098,
            -112227,
            [[6249552, 9743362], [8833198, 6815470]],
            0,
        ),
        (
            0,
            34,
            [[357, 1178], [845, 380]],
            [1730, 1524],
            [2550, 2287],
            [260, 300],
            64,
            54460824,
            223537098,
            -112227,
            [[6249552, 9743362], [8833198, 6815470]],
            0,
        ),
        (
            0,
            34,
            [[369, 1166], [862, 385]],
            [1726, 1521],
            [2596, 2312],
            [288, 276],
            64,
            52117267,
            223537098,
            -112227,
            [[6249552, 9743362], [8833198, 6815470]],
            0,
        ),
        (
            0,
            35,
            [[381, 1221], [869, 403]],
            [1780, 1474],
            [2575, 2189],
            [323, 280],
            64,
            49911379,
            223537098,
            -112227,
            [[6249552, 9743362], [8833198, 6815470]],
            0,
        ),
        (
            0,
            29,
            [[301, 998], [981, 478]],
            [1476, 1774],
            [2322, 2531],
            [280, 241],
            64,
            47785290,
            223537098,
            -112227,
            [[6249552, 9743362], [8833198, 6815470]],
            0,
        ),
        (
            0,
            32,
            [[347, 1111], [893, 413]],
            [1626, 1626],
            [2443, 2411],
            [296, 286],
            64,
            45777605,
            223537098,
            -112227,
            [[6249552, 9743362], [8833198, 6815470]],
            0,
        ),
        (
            0,
            28,
            [[316, 1005], [1013, 502]],
            [1427, 1829],
            [2255, 2571],
            [270, 276],
            64,
            43875367,
            223537098,
            -112227,
            [[6249552, 9743362], [8833198, 6815470]],
            0,
        ),
        (
            0,
            29,
            [[319, 1020], [1009, 444]],
            [1472, 1781],
            [2315, 2555],
            [234, 289],
            64,
            42030466,
            223537098,
            -112227,
            [[6249552, 9743362], [8833198, 6815470]],
            0,
        ),
        (
            0,
            32,
            [[344, 1096], [935, 402]],
            [1622, 1627],
            [2438, 2404],
            [277, 303],
            64,
            40382983,
            223537098,
            -112227,
            [[6249552, 9743362], [8833198, 6815470]],
            0,
        ),
        (
            0,
            32,
            [[362, 1058], [900, 420]],
            [1627, 1623],
            [2471, 2397],
            [247, 260],
            64,
            38849602,
            223537098,
            -112227,
            [[6249552, 9743362], [8833198, 6815470]],
            0,
        ),
        (
            1,
            34,
            [[329, 1165], [857, 408]],
            [1726, 1527],
            [2545, 2280],
            [264, 278],
            64,
            37354924,
            223537260,
            -112227,
            [[6249552, 9743362], [8833198, 6815632]],
            0,
        ),
        (
            0,
            32,
            [[353, 1117], [923, 374]],
            [1628, 1623],
            [2523, 2395],
            [270, 309],
            64,
            36036773,
            223537260,
            -112227,
            [[6249552, 9743362], [8833198, 6815632]],
            0,
        ),
        (
            0,
            29,
            [[344, 981], [977, 474]],
            [1475, 1777],
            [2364, 2520],
            [287, 294],
            64,
            34825278,
            223537260,
            -112227,
            [[6249552, 9743362], [8833198, 6815632]],
            0,
        ),
        (
            0,
            30,
            [[351, 1075], [960, 474]],
            [1524, 1730],
            [2401, 2529],
            [276, 303],
            64,
            33703913,
            223537260,
            -112227,
            [[6249552, 9743362], [8833198, 6815632]],
            0,
        ),
        (
            0,
            30,
            [[343, 1063], [984, 457]],
            [1525, 1730],
            [2393, 2511],
            [296, 267],
            64,
            32710272,
            223537260,
            -112227,
            [[6249552, 9743362], [8833198, 6815632]],
            0,
        ),
        (
            0,
            32,
            [[336, 1125], [964, 406]],
            [1629, 1630],
            [2420, 2407],
            [297, 259],
            64,
            31693411,
            223537260,
            -112227,
            [[6249552, 9743362], [8833198, 6815632]],
            0,
        ),
        (
            0,
            31,
            [[344, 1099], [965, 431]],
            [1578, 1677],
            [2384, 2421],
            [264, 285],
            64,
            30800162,
            223537260,
            -112227,
            [[6249552, 9743362], [8833198, 6815632]],
            0,
        ),
        (
            0,
            27,
            [[296, 933], [1016, 461]],
            [1371, 1879],
            [2259, 2685],
            [300, 258],
            64,
            30026365,
            223537260,
            -112227,
            [[6249552, 9743362], [8833198, 6815632]],
            0,
        ),
        (
            0,
            24,
            [[263, 849], [1153, 513]],
            [1218, 2030],
            [2095, 2719],
            [286, 252],
            64,
            29338419,
            223537260,
            -112227,
            [[6249552, 9743362], [8833198, 6815632]],
            0,
        ),
        (
            0,
            34,
            [[373, 1171], [912, 445]],
            [1727, 1525],
            [2590, 2289],
            [288, 295],
            64,
            28725576,
            223537260,
            -112227,
            [[6249552, 9743362], [8833198, 6815632]],
            1,
        ),
        (
            0,
            33,
            [[406, 1148], [872, 408]],
            [1679, 1577],
            [2550, 2364],
            [277, 268],
            64,
            28136783,
            223537260,
            -112227,
            [[6249552, 9743362], [8833198, 6815632]],
            0,
        ),
    ],
];
const REVERSAL_TRACES_1024: [u64; 2] = [0x64a1d9adad28586c, 0x742fba322cc8bd8a];
const REVERSAL_COMPOSITIONS_1024: [&[Composition]; 2] = [
    &[
        (
            [[18919, 6684], [14722, 35102]],
            [
                [
                    [472128, -15927, 715146, -918464],
                    [239268, -6348, 388174, -564461],
                ],
                [
                    [293631, -5685, 352834, -549704],
                    [693242, -12781, 772565, -970420],
                ],
            ],
            [[1151, 2575], [1102, 2682], [3375, 0]],
            64,
            18239405190732265563,
        ),
        (
            [[40850, 15611], [5658, 40275]],
            [
                [
                    [555693, -16963, 745801, -933142],
                    [301909, -6662, 411318, -602941],
                ],
                [
                    [261444, -7352, 345390, -499316],
                    [582009, -21147, 755959, -903518],
                ],
            ],
            [[1106, 2668], [1087, 2687], [3361, 0]],
            63,
            13731627527993758659,
        ),
        (
            [[48278, 10911], [1686, 22921]],
            [
                [
                    [652257, -23611, 728609, -900419],
                    [333957, -9046, 418052, -574680],
                ],
                [
                    [211788, -6505, 325593, -464660],
                    [511533, -23589, 672059, -781170],
                ],
            ],
            [[1025, 2656], [1025, 2648], [3348, 0]],
            64,
            5634900123172546301,
        ),
        (
            [[39715, 16830], [18453, 33403]],
            [
                [
                    [550797, -23064, 717790, -859593],
                    [283845, -9593, 418166, -578774],
                ],
                [
                    [263807, -8062, 353414, -527744],
                    [575837, -22214, 807732, -997473],
                ],
            ],
            [[1133, 2649], [1152, 2715], [3375, 0]],
            64,
            6530467253295352134,
        ),
        (
            [[42130, -1559], [-786, 19322]],
            [
                [
                    [642201, -20663, 772209, -952217],
                    [313136, -8550, 399515, -638119],
                ],
                [
                    [230692, -5945, 347890, -498205],
                    [566111, -18286, 719067, -774332],
                ],
            ],
            [[1081, 2672], [1154, 2749], [3361, 0]],
            63,
            5601181915929823667,
        ),
        (
            [[22292, 16128], [8670, 48891]],
            [
                [
                    [600987, -24974, 781078, -983748],
                    [307726, -8090, 420717, -607855],
                ],
                [
                    [217527, -4653, 347004, -550771],
                    [538037, -19843, 752081, -907166],
                ],
            ],
            [[1182, 2649], [1188, 2609], [3376, 0]],
            61,
            2965596168937369122,
        ),
        (
            [[55412, 19347], [-3992, 12411]],
            [
                [
                    [642822, -23031, 738142, -948850],
                    [326808, -8276, 409686, -575026],
                ],
                [
                    [236066, -6437, 287898, -494877],
                    [535097, -21859, 710456, -753335],
                ],
            ],
            [[1192, 2725], [1111, 2682], [3359, 0]],
            63,
            3704218712227885460,
        ),
        (
            [[16490, 140], [2863, 50704]],
            [
                [
                    [510567, -22178, 678925, -762527],
                    [243645, -7151, 393028, -510036],
                ],
                [
                    [261684, -6024, 377565, -592793],
                    [702832, -23795, 811693, -892700],
                ],
            ],
            [[1098, 2568], [1079, 2772], [3351, 0]],
            62,
            8066463595391088842,
        ),
        (
            [[18945, 10890], [15440, 60048]],
            [
                [
                    [582653, -18760, 713541, -899948],
                    [320058, -8112, 417809, -560900],
                ],
                [
                    [246099, -5221, 362006, -553845],
                    [607273, -16077, 736517, -840646],
                ],
            ],
            [[1117, 2668], [1122, 2733], [3363, 0]],
            64,
            3368212236771092905,
        ),
        (
            [[38707, 4878], [9410, 22023]],
            [
                [
                    [500441, -16951, 677169, -780062],
                    [265596, -6387, 367916, -549900],
                ],
                [
                    [293373, -7865, 398118, -567904],
                    [686225, -23541, 817730, -954435],
                ],
            ],
            [[1090, 2574], [1131, 2849], [3343, 0]],
            63,
            13282230175895344056,
        ),
        (
            [[-3312, 689], [5071, 53190]],
            [
                [
                    [544661, -20083, 683684, -763992],
                    [283508, -8349, 390582, -564925],
                ],
                [
                    [270038, -5624, 373127, -548796],
                    [677741, -17276, 827407, -955797],
                ],
            ],
            [[1103, 2592], [1094, 2795], [3350, 0]],
            64,
            321789689046400672,
        ),
        (
            [[26644, 10783], [25938, 62092]],
            [
                [
                    [564333, -21943, 709431, -897886],
                    [283456, -7075, 430768, -599388],
                ],
                [
                    [258982, -6994, 347777, -539095],
                    [592345, -21455, 781633, -898919],
                ],
            ],
            [[1095, 2724], [1127, 2724], [3361, 0]],
            62,
            988599341539021904,
        ),
        (
            [[34496, 13570], [9237, 46239]],
            [
                [
                    [587106, -19888, 708414, -850630],
                    [305035, -9406, 437616, -553574],
                ],
                [
                    [251648, -9141, 369528, -534412],
                    [614833, -25148, 741745, -884141],
                ],
            ],
            [[1080, 2736], [1070, 2744], [3341, 0]],
            64,
            763206292214527955,
        ),
        (
            [[18727, 8253], [1487, 49477]],
            [
                [
                    [572391, -19290, 747984, -960732],
                    [309112, -6476, 397135, -631065],
                ],
                [
                    [224730, -5806, 323496, -482499],
                    [554711, -28111, 735001, -825767],
                ],
            ],
            [[1060, 2635], [1147, 2682], [3359, 0]],
            64,
            9552617886000149780,
        ),
        (
            [[28602, 4815], [-378, 5265]],
            [
                [
                    [549983, -26231, 743216, -861129],
                    [295363, -7971, 383822, -607688],
                ],
                [
                    [241161, -8253, 354926, -534762],
                    [581818, -19963, 743905, -904714],
                ],
            ],
            [[1153, 2678], [1177, 2686], [3360, 0]],
            64,
            2643117370524646420,
        ),
        (
            [[55816, 17020], [-4608, 21307]],
            [
                [
                    [544623, -17842, 738797, -929457],
                    [278692, -5063, 397539, -594440],
                ],
                [
                    [293142, -6238, 361451, -593020],
                    [708969, -26610, 809295, -851350],
                ],
            ],
            [[1255, 2757], [1168, 2811], [3373, 0]],
            63,
            5135648070825893184,
        ),
        (
            [[33336, 9666], [-5384, 32858]],
            [
                [
                    [534956, -18790, 726693, -834289],
                    [290160, -9554, 413383, -577110],
                ],
                [
                    [255879, -6625, 373270, -600539],
                    [639719, -23396, 815089, -1016597],
                ],
            ],
            [[1123, 2633], [1105, 2707], [3363, 0]],
            63,
            5003414039608608493,
        ),
        (
            [[36024, 15712], [-5211, 36541]],
            [
                [
                    [535288, -19005, 711463, -890839],
                    [265368, -7157, 428472, -564799],
                ],
                [
                    [275305, -8179, 340467, -552700],
                    [658034, -30414, 782994, -914673],
                ],
            ],
            [[1130, 2728], [1055, 2840], [3352, 0]],
            62,
            1638651417222283484,
        ),
        (
            [[32601, 533], [-3221, 44386]],
            [
                [
                    [541547, -16602, 709488, -931005],
                    [282771, -8764, 403342, -592871],
                ],
                [
                    [257045, -4874, 353296, -554223],
                    [597906, -19557, 781199, -794351],
                ],
            ],
            [[1180, 2773], [1163, 2741], [3360, 0]],
            63,
            6647508489938429495,
        ),
        (
            [[26445, 14191], [193, 35373]],
            [
                [
                    [541455, -25250, 750783, -967528],
                    [268463, -7935, 393209, -594077],
                ],
                [
                    [269853, -6924, 357790, -565179],
                    [637600, -17942, 811277, -956630],
                ],
            ],
            [[1112, 2741], [1119, 2788], [3363, 0]],
            62,
            2010094659243249254,
        ),
        (
            [[40948, 12337], [-1294, 20352]],
            [
                [
                    [460044, -20681, 708027, -893457],
                    [247894, -7086, 431712, -558559],
                ],
                [
                    [303443, -9614, 387054, -589047],
                    [719720, -31716, 851023, -1000057],
                ],
            ],
            [[1153, 2572], [1103, 2778], [3361, 1]],
            62,
            3653367752991256096,
        ),
        (
            [[44229, 7000], [14111, 50066]],
            [
                [
                    [405690, -11653, 637956, -701664],
                    [212006, -3727, 342197, -523425],
                ],
                [
                    [346957, -6536, 380180, -629840],
                    [735991, -25026, 830457, -976692],
                ],
            ],
            [[1109, 2564], [1105, 2867], [3369, 0]],
            63,
            8620299176532337420,
        ),
        (
            [[17457, 546], [11568, 41640]],
            [
                [
                    [609283, -21894, 764446, -889131],
                    [311580, -10517, 459428, -623695],
                ],
                [
                    [230168, -6527, 345160, -566181],
                    [573426, -13652, 737866, -850362],
                ],
            ],
            [[1160, 2711], [1161, 2834], [3358, 0]],
            62,
            16418454760614602305,
        ),
        (
            [[24729, 3080], [3998, 32180]],
            [
                [
                    [556502, -19684, 733765, -929337],
                    [290927, -9209, 457903, -614151],
                ],
                [
                    [268034, -7202, 342257, -553052],
                    [594932, -19448, 761888, -891764],
                ],
            ],
            [[1120, 2785], [1114, 2835], [3358, 0]],
            64,
            13623711690227666666,
        ),
    ],
    &[
        (
            [[2547, 36047], [50083, 1067]],
            [
                [
                    [209859, -4630, 355202, -536633],
                    [550405, -24455, 840966, -1025513],
                ],
                [
                    [645574, -21057, 699794, -852306],
                    [315426, -6276, 391918, -584188],
                ],
            ],
            [[1144, 2515], [1136, 2644], [3371, 0]],
            63,
            18256501942296654891,
        ),
        (
            [[10499, 44150], [34060, 7354]],
            [
                [
                    [230408, -5921, 352223, -573774],
                    [669193, -23964, 878464, -1034556],
                ],
                [
                    [527149, -22699, 656225, -758725],
                    [263400, -8142, 366232, -544656],
                ],
            ],
            [[1117, 2494], [1107, 2796], [3360, 0]],
            63,
            4134716348427934996,
        ),
        (
            [[9032, 60029], [24374, -7863]],
            [
                [
                    [272193, -8526, 387800, -578382],
                    [744908, -35131, 956718, -1082335],
                ],
                [
                    [475533, -17431, 653926, -739602],
                    [226899, -6733, 353450, -505908],
                ],
            ],
            [[1034, 2435], [1061, 2874], [3344, 0]],
            64,
            14231736968411972125,
        ),
        (
            [[6815, 52111], [58985, 4086]],
            [
                [
                    [218194, -6809, 349424, -533083],
                    [673136, -37456, 893958, -967393],
                ],
                [
                    [552794, -21898, 681061, -808274],
                    [258416, -7458, 377658, -575363],
                ],
            ],
            [[1141, 2486], [1127, 2752], [3372, 0]],
            63,
            7103850760348990498,
        ),
        (
            [[2273, 30213], [16068, -1023]],
            [
                [
                    [241046, -6509, 360842, -580456],
                    [669052, -27002, 881534, -1134286],
                ],
                [
                    [511362, -15487, 675106, -784418],
                    [267645, -4982, 366948, -517147],
                ],
            ],
            [[1083, 2448], [1158, 2838], [3362, 0]],
            63,
            6170489367470436308,
        ),
        (
            [[4756, 31098], [53493, 8189]],
            [
                [
                    [240085, -7604, 377976, -572312],
                    [712675, -27307, 923360, -1042229],
                ],
                [
                    [489072, -17124, 715348, -917181],
                    [258844, -8162, 383911, -596666],
                ],
            ],
            [[1179, 2434], [1208, 2764], [3375, 0]],
            58,
            8504988899047351739,
        ),
        (
            [[-1595, 56016], [9346, -2074]],
            [
                [
                    [262152, -8638, 352353, -587621],
                    [753732, -29902, 892333, -1001369],
                ],
                [
                    [471322, -16223, 594063, -729671],
                    [255282, -6999, 343203, -473388],
                ],
            ],
            [[1212, 2419], [1131, 2799], [3356, 0]],
            64,
            13271942603368515662,
        ),
        (
            [[-1413, 22691], [27760, 18335]],
            [
                [
                    [213506, -8376, 334305, -511365],
                    [562696, -25517, 834876, -882681],
                ],
                [
                    [576439, -20796, 712301, -953085],
                    [328697, -9895, 415435, -561564],
                ],
            ],
            [[1099, 2472], [1092, 2675], [3349, 0]],
            62,
            14883485550201546392,
        ),
        (
            [[-909, 30020], [42346, 17612]],
            [
                [
                    [256048, -5856, 364451, -558418],
                    [685437, -32936, 894110, -1040755],
                ],
                [
                    [559718, -21401, 697182, -888274],
                    [244400, -5291, 370334, -542771],
                ],
            ],
            [[1123, 2479], [1164, 2757], [3361, 0]],
            64,
            10507392982355552357,
        ),
        (
            [[11423, 32166], [49180, 5522]],
            [
                [
                    [202754, -4390, 324110, -501536],
                    [553757, -26195, 833902, -993669],
                ],
                [
                    [627359, -19671, 709144, -828004],
                    [319147, -8358, 422997, -560278],
                ],
            ],
            [[1091, 2516], [1145, 2751], [3342, 0]],
            64,
            8081532133818735826,
        ),
        (
            [[-2040, 20693], [50922, 17870]],
            [
                [
                    [226444, -7126, 345696, -499640],
                    [628571, -24834, 818613, -953691],
                ],
                [
                    [600639, -21461, 710886, -783577],
                    [300938, -6823, 406524, -596019],
                ],
            ],
            [[1088, 2558], [1095, 2736], [3350, 0]],
            64,
            6416394061333116946,
        ),
        (
            [[-2035, 33040], [62755, 21439]],
            [
                [
                    [231255, -5703, 368054, -564029],
                    [658300, -25071, 912710, -1053525],
                ],
                [
                    [555672, -19547, 698471, -855691],
                    [247036, -5689, 384718, -559418],
                ],
            ],
            [[1141, 2555], [1161, 2770], [3360, 1]],
            64,
            2400334792378182366,
        ),
        (
            [[-487, 56680], [30130, 11133]],
            [
                [
                    [266972, -8318, 346163, -553666],
                    [663191, -30934, 842756, -968075],
                ],
                [
                    [533815, -19174, 670384, -802760],
                    [279022, -8247, 364199, -572148],
                ],
            ],
            [[1071, 2554], [1077, 2726], [3347, 0]],
            61,
            14824401001700262345,
        ),
        (
            [[4013, 25750], [35350, 16811]],
            [
                [
                    [233845, -6511, 356814, -558685],
                    [701923, -25824, 893068, -1103983],
                ],
                [
                    [508517, -15064, 640049, -730191],
                    [270238, -7633, 376449, -529209],
                ],
            ],
            [[1038, 2437], [1150, 2843], [3361, 0]],
            64,
            3422132670232395087,
        ),
        (
            [[-609, 26634], [21528, -4176]],
            [
                [
                    [227577, -8431, 375136, -570815],
                    [657933, -31440, 889102, -1098026],
                ],
                [
                    [525263, -23232, 665960, -798981],
                    [245225, -6082, 383746, -580756],
                ],
            ],
            [[1157, 2462], [1179, 2679], [3362, 0]],
            64,
            14586291169708630446,
        ),
        (
            [[4613, 59644], [12591, -602]],
            [
                [
                    [228032, -7873, 358973, -561689],
                    [601647, -17689, 878009, -1021572],
                ],
                [
                    [634222, -22825, 680545, -861944],
                    [318259, -8696, 401188, -568234],
                ],
            ],
            [[1218, 2646], [1181, 2725], [3373, 0]],
            63,
            5696346294444156466,
        ),
        (
            [[2272, 48681], [19480, 11479]],
            [
                [
                    [203445, -6757, 357284, -532885],
                    [620837, -31538, 899170, -1018537],
                ],
                [
                    [576932, -20498, 724921, -898140],
                    [298490, -8781, 408739, -595754],
                ],
            ],
            [[1114, 2527], [1099, 2771], [3361, 0]],
            64,
            12865238714337935998,
        ),
        (
            [[5594, 58000], [21889, 3254]],
            [
                [
                    [231078, -5073, 354134, -553866],
                    [622746, -22722, 896269, -960093],
                ],
                [
                    [614718, -26149, 666948, -834619],
                    [296229, -10572, 407025, -591192],
                ],
            ],
            [[1125, 2626], [1085, 2763], [3351, 0]],
            64,
            8042340439730067969,
        ),
        (
            [[-3323, 45401], [25605, 15334]],
            [
                [
                    [220556, -4795, 332744, -537588],
                    [662041, -28325, 845176, -1007940],
                ],
                [
                    [570305, -15649, 686369, -848070],
                    [274548, -6074, 401607, -546940],
                ],
            ],
            [[1169, 2573], [1174, 2736], [3358, 0]],
            63,
            16578320710874384730,
        ),
        (
            [[3309, 41226], [44546, 4318]],
            [
                [
                    [226572, -5739, 350432, -556651],
                    [623156, -21086, 850705, -977331],
                ],
                [
                    [587907, -21674, 666452, -783467],
                    [281203, -6328, 384967, -551124],
                ],
            ],
            [[1080, 2609], [1109, 2820], [3354, 0]],
            64,
            16674142310887634830,
        ),
        (
            [[14804, 59459], [33907, 11262]],
            [
                [
                    [196986, -7061, 338228, -514840],
                    [539797, -24776, 891147, -945135],
                ],
                [
                    [607689, -20158, 728568, -949689],
                    [323819, -8225, 422680, -606084],
                ],
            ],
            [[1165, 2546], [1115, 2664], [3361, 1]],
            60,
            3161411255725529859,
        ),
        (
            [[6993, 48393], [36285, 13815]],
            [
                [
                    [164981, -4179, 336663, -496257],
                    [470409, -13194, 754337, -874557],
                ],
                [
                    [702383, -23225, 680608, -905694],
                    [358754, -10401, 415918, -626717],
                ],
            ],
            [[1126, 2616], [1131, 2646], [3368, 0]],
            61,
            1677240541763115531,
        ),
        (
            [[1573, 19116], [52416, 8418]],
            [
                [
                    [233336, -8002, 393256, -575677],
                    [706116, -32163, 953516, -1119532],
                ],
                [
                    [504584, -19455, 651268, -790048],
                    [271348, -8915, 379233, -561805],
                ],
            ],
            [[1133, 2535], [1159, 2942], [3351, 0]],
            63,
            15535089194657908539,
        ),
        (
            [[4757, 32085], [16962, 9562]],
            [
                [
                    [259287, -7748, 371194, -575096],
                    [696974, -35706, 918127, -1062073],
                ],
                [
                    [535015, -16911, 644557, -876134],
                    [282053, -9581, 375708, -555999],
                ],
            ],
            [[1115, 2584], [1117, 2908], [3356, 0]],
            64,
            13158093212196169954,
        ),
    ],
];
const REVERSAL_EARNED_1024: [&[EarnedBlock]; 2] = [
    &[
        (
            [[29, 0, 0], [0, 35, 0]],
            0,
            [[0, 0], [0, 1970]],
            -46691,
            -112227,
        ),
        (
            [[33, 0, 0], [0, 31, 0]],
            0,
            [[0, 0], [0, 0]],
            -46691,
            -112227,
        ),
        (
            [[36, 0, 0], [0, 28, 0]],
            0,
            [[0, 0], [0, 0]],
            -46691,
            -112227,
        ),
        (
            [[31, 1, 0], [0, 32, 0]],
            1,
            [[0, 3415], [0, 0]],
            -46691,
            -112227,
        ),
        (
            [[34, 0, 0], [0, 30, 0]],
            0,
            [[0, 0], [0, 0]],
            -46691,
            -112227,
        ),
        (
            [[34, 0, 0], [0, 30, 0]],
            0,
            [[0, 0], [0, 0]],
            -46691,
            -112227,
        ),
        (
            [[35, 0, 0], [0, 29, 0]],
            0,
            [[0, 0], [0, 0]],
            -46691,
            -112227,
        ),
        (
            [[29, 0, 0], [0, 35, 0]],
            0,
            [[0, 0], [0, 0]],
            -46691,
            -112227,
        ),
        (
            [[32, 0, 0], [0, 32, 0]],
            0,
            [[0, 0], [0, 0]],
            -46691,
            -112227,
        ),
        (
            [[28, 0, 0], [0, 36, 0]],
            0,
            [[0, 0], [0, 0]],
            -46691,
            -112227,
        ),
        (
            [[29, 0, 0], [0, 35, 0]],
            0,
            [[0, 0], [0, 0]],
            -46691,
            -112227,
        ),
        (
            [[32, 0, 0], [0, 32, 0]],
            0,
            [[0, 0], [0, 0]],
            -46691,
            -112227,
        ),
        (
            [[32, 0, 0], [0, 32, 0]],
            0,
            [[0, 0], [0, 0]],
            -46691,
            -112227,
        ),
        (
            [[34, 0, 0], [0, 30, 0]],
            0,
            [[0, 0], [0, 0]],
            -46691,
            -112227,
        ),
        (
            [[32, 0, 0], [0, 32, 0]],
            0,
            [[0, 0], [0, 0]],
            -46691,
            -112227,
        ),
        (
            [[29, 0, 0], [0, 35, 0]],
            0,
            [[0, 0], [0, 0]],
            -46691,
            -112227,
        ),
        (
            [[30, 0, 0], [0, 34, 0]],
            0,
            [[0, 0], [0, 0]],
            -46691,
            -112227,
        ),
        (
            [[30, 0, 0], [0, 34, 0]],
            0,
            [[0, 0], [0, 0]],
            -46691,
            -112227,
        ),
        (
            [[32, 0, 0], [0, 32, 0]],
            0,
            [[0, 0], [0, 0]],
            -46691,
            -112227,
        ),
        (
            [[31, 0, 0], [0, 33, 0]],
            0,
            [[0, 0], [0, 0]],
            -46691,
            -112227,
        ),
        (
            [[27, 0, 0], [0, 37, 0]],
            0,
            [[0, 0], [0, 0]],
            -46691,
            -112227,
        ),
        (
            [[24, 0, 0], [1, 39, 0]],
            1,
            [[0, 0], [625, 0]],
            -46691,
            -112227,
        ),
        (
            [[34, 0, 0], [0, 30, 0]],
            0,
            [[0, 0], [0, 0]],
            -46691,
            -112227,
        ),
        (
            [[33, 0, 0], [0, 31, 0]],
            0,
            [[0, 0], [0, 0]],
            -46691,
            -112227,
        ),
    ],
    &[
        (
            [[0, 29, 0], [35, 0, 0]],
            0,
            [[0, 0], [3264, 0]],
            -46691,
            -112227,
        ),
        (
            [[0, 33, 0], [31, 0, 0]],
            0,
            [[0, 0], [0, 0]],
            -46691,
            -112227,
        ),
        (
            [[0, 36, 0], [28, 0, 0]],
            0,
            [[0, 0], [0, 0]],
            -46691,
            -112227,
        ),
        (
            [[0, 32, 0], [32, 0, 0]],
            0,
            [[0, 0], [0, 0]],
            -46691,
            -112227,
        ),
        (
            [[0, 34, 0], [30, 0, 0]],
            0,
            [[0, 0], [0, 0]],
            -46691,
            -112227,
        ),
        (
            [[0, 34, 0], [30, 0, 0]],
            0,
            [[0, 0], [0, 0]],
            -46691,
            -112227,
        ),
        (
            [[0, 35, 0], [29, 0, 0]],
            0,
            [[0, 0], [0, 0]],
            -46691,
            -112227,
        ),
        (
            [[0, 29, 0], [35, 0, 0]],
            0,
            [[0, 0], [0, 0]],
            -46691,
            -112227,
        ),
        (
            [[0, 32, 0], [32, 0, 0]],
            0,
            [[0, 0], [0, 0]],
            -46691,
            -112227,
        ),
        (
            [[0, 28, 0], [36, 0, 0]],
            0,
            [[0, 0], [0, 0]],
            -46691,
            -112227,
        ),
        (
            [[0, 29, 0], [35, 0, 0]],
            0,
            [[0, 0], [0, 0]],
            -46691,
            -112227,
        ),
        (
            [[0, 32, 0], [32, 0, 0]],
            0,
            [[0, 0], [0, 0]],
            -46691,
            -112227,
        ),
        (
            [[0, 32, 0], [32, 0, 0]],
            0,
            [[0, 0], [0, 0]],
            -46691,
            -112227,
        ),
        (
            [[0, 34, 0], [29, 1, 0]],
            1,
            [[0, 0], [0, 162]],
            -46691,
            -112227,
        ),
        (
            [[0, 32, 0], [32, 0, 0]],
            0,
            [[0, 0], [0, 0]],
            -46691,
            -112227,
        ),
        (
            [[0, 29, 0], [35, 0, 0]],
            0,
            [[0, 0], [0, 0]],
            -46691,
            -112227,
        ),
        (
            [[0, 30, 0], [34, 0, 0]],
            0,
            [[0, 0], [0, 0]],
            -46691,
            -112227,
        ),
        (
            [[0, 30, 0], [34, 0, 0]],
            0,
            [[0, 0], [0, 0]],
            -46691,
            -112227,
        ),
        (
            [[0, 32, 0], [32, 0, 0]],
            0,
            [[0, 0], [0, 0]],
            -46691,
            -112227,
        ),
        (
            [[0, 31, 0], [33, 0, 0]],
            0,
            [[0, 0], [0, 0]],
            -46691,
            -112227,
        ),
        (
            [[0, 27, 0], [37, 0, 0]],
            0,
            [[0, 0], [0, 0]],
            -46691,
            -112227,
        ),
        (
            [[0, 24, 0], [40, 0, 0]],
            0,
            [[0, 0], [0, 0]],
            -46691,
            -112227,
        ),
        (
            [[0, 34, 0], [29, 0, 1]],
            0,
            [[0, 0], [0, 0]],
            -46691,
            -112227,
        ),
        (
            [[0, 33, 0], [31, 0, 0]],
            0,
            [[0, 0], [0, 0]],
            -46691,
            -112227,
        ),
    ],
];
const REVERSAL_READ_1024: [u64; 2] = [0xfa696f8fb2545fd6, 0x5c13c015d378e9a5];
const REVERSAL_CENSUS_1024: [&[(u32, u64)]; 2] = [
    &[
        (0, 2),
        (1, 6459),
        (2, 49495),
        (3, 76118),
        (4, 22692),
        (5, 1104),
        (6, 185),
        (7, 50),
        (8, 16),
        (9, 8),
        (10, 1),
        (11, 4),
        (12, 1),
        (70, 1),
    ],
    &[
        (0, 4),
        (1, 6437),
        (2, 49444),
        (3, 76206),
        (4, 22680),
        (5, 1099),
        (6, 180),
        (7, 55),
        (8, 24),
        (9, 5),
        (10, 1),
        (11, 4),
        (13, 1),
        (71, 1),
    ],
];
/// The one inhibited image both arms decode, its CRC-64: the same bytes in both tests. Since
/// ADR-0094 the image is format 16, so its header's version and seal are not the bytes H-17
/// read, and ADR-0095 pins the image as it is now beside the CRC H-17 read.
const REVERSAL_IMAGE_CRC_1024: u64 = 0xd2965219775c394a;
/// The same image with its header's version written back to 15 and the header resealed: the
/// CRC-64 H-17 read (ADR-0091), so that every byte of the image but the version and the seal is
/// the image H-17 ran from (ADR-0095).
const REVERSAL_IMAGE_CRC_FORMAT_15_1024: u64 = 0xd5579c31308388ad;
/// The four couplings at the ends of the 1 536th and the 1 537th trials, `[stimulus][readout]`.
type Carry = ([[i64; 2]; 2], [[i64; 2]; 2]);
/// The couplings around the flip per arm, as read.
const REVERSAL_SNAPSHOTS_1024: [Carry; 2] = [
    (
        [[8475315, 6698611], [6584205, 9300499]],
        [[8475315, 6698611], [6584205, 9302469]],
    ),
    (
        [[6249552, 9743362], [8829934, 6815470]],
        [[6249552, 9743362], [8833198, 6815470]],
    ),
];
/// Clause 1's and clause 2's counts per arm, `[before the flip, the run's last 128]`,
/// against `REWARDED_MIN`.
const CORRECT_REVERSAL_1024: [[u32; 2]; 2] = [[128, 0], [128, 0]];
/// ADR-0090's assertion's reach per arm, as read.
const HELD_1024: [Held; 2] = [
    Held {
        after_carry: 0,
        uncarried: 0,
        outside: 0,
        carried: 69,
    },
    Held {
        after_carry: 0,
        uncarried: 0,
        outside: 0,
        carried: 68,
    },
];
/// The measured need per arm, as read.
const NEED_1024: [Need; 2] = [
    Need {
        selected_new: 2,
        selected_old: 1534,
        ties: 0,
        rewards: 2,
        crossed: None,
    },
    Need {
        selected_new: 1,
        selected_old: 1534,
        ties: 1,
        rewards: 1,
        crossed: None,
    },
];
/// The first trial after the flip that earned a reward, per arm, as read.
const FIRST_REWARD_1024: [Option<usize>; 2] = [Some(1762), Some(2373)];
/// The pair the 1 536th trial carried into the 1 537th, per arm, as read.
const CARRIED_1024: [Option<(usize, usize)>; 2] = [Some((1, 1)), Some((1, 0))];
/// The new answer's pairs' rise from the flip to the run's end, `[A, B]`, per arm, as read.
const NEW_RISE_1024: [[i64; 2]; 2] = [[3415, 625], [0, 162]];
/// The blocks, of forty-eight, in which the stimulus fired once, per arm, as read.
const ONCE_BLOCKS_REVERSAL_1024: [u32; 2] = [48, 48];
/// Whether the inhibitory sum fell in every block of the run, per arm, as read.
const FALLS_REVERSAL_1024: [bool; 2] = [true, true];
/// ADR-0080's derivation as read over each arm's whole run, clause by clause.
const DERIVATION_REVERSAL_1024: [[bool; 3]; 2] = [[true, true, true], [true, true, true]];
/// The arena's sums by polarity after each arm's run, `(inhibitory, excitatory)`.
const SUMS_AFTER_REVERSAL_1024: [(i64, i64); 2] = [(28694627, 222960156), (28136783, 223537260)];

/// The verdict, by the rule committed first, over the pinned tables: clause 1 held in both
/// arms, 128 of 128 before the flip, H-16 replicated; clause 2 in neither, 0 of the last 128
/// after it. H-17 is no, as ADR-0089 predicted.
const REVERSAL_1024: Revision = Revision {
    learned: [true, true],
    revised: [false, false],
    replicated: true,
    yes: false,
};

/// The gate's test (ADR-0061's class; brief 040): the arms and their mappings; the constants
/// as ADR-0089 fixed them; the criterion's two clauses at their edges over blocks written by
/// hand, a clause-1 failure among them; ADR-0090's assertion's rule and the pair the
/// carry-over names at their edges; the readings' rules over blocks written by hand; and a
/// few trials over a flip on the instrument's network at 1 024 units with the inhibitory
/// baseline set — the flipped run, the oracle held at every trial inside
/// `earned_run_flipped`, beside a run with no flip from the same network: every trial
/// before the flip the same, and at the flip the same stimulus, counts, selection, signal
/// and consolidation, the correctness the other mapping's and the reward's sign with it.
/// No whole run, and nothing else added to the gate.
#[test]
fn a_few_trials_over_the_flip_at_1024_units_and_the_rules_of_the_assignment_reversed() {
    // The arms, their mappings and the H-16 arms their first halves are; the constants.
    assert_eq!(
        REVERSAL_ARMS,
        [Reversal::AssignmentFirst, Reversal::MirroredFirst]
    );
    assert!(!first_mapping(Reversal::AssignmentFirst) && first_mapping(Reversal::MirroredFirst));
    assert_eq!(
        REVERSAL_ARMS.map(replicated_arm),
        [0, 1],
        "the first halves are H-16's assignment and mirrored assignment"
    );
    assert_eq!(INHIBITION_ARMS[0], Inhibition::Assignment);
    assert_eq!(INHIBITION_ARMS[1], Inhibition::Mirrored);
    assert!(!inhibition_mirrors(INHIBITION_ARMS[0]) && inhibition_mirrors(INHIBITION_ARMS[1]));
    assert_eq!(
        (
            REVERSAL_TRIALS,
            REVERSAL_BLOCKS,
            FLIP,
            LAST_BEFORE_FLIP,
            FLIP_BLOCK
        ),
        (3_072, 48, 1_536, 1_535, 24)
    );
    assert_eq!([REVERSAL_PREDICTED], [false], "ADR-0089 predicts no");
    // Every constant of H-17 restated unchanged: ADR-0065's window, trial, seed and gain,
    // ADR-0066's mark and window of the criterion, ADR-0076's stimulus and cancel, ADR-0077's
    // settled candidate, ADR-0080's length and reward, ADR-0085's two baselines.
    assert_eq!(
        (WINDOW.from, WINDOW.ticks, TRIAL_TICKS, SEED, GAIN_1024),
        (100, 500, 1 << 14, 27, 0x0001_C000)
    );
    assert_eq!((REWARDED_MIN, LAST_BLOCKS, BLOCK), (80, 2, 64));
    assert_eq!(SHAPE_F46, (2, 0x0001_4000));
    assert_eq!(CANCEL_PICKED_1024, Some(CANCEL_AT_THE_EXTREME));
    assert_eq!((SETTLED, BACKGROUNDS[SETTLED]), (0, 0));
    assert_eq!(INHIBITION_TRIALS, REINFORCED_TRIALS);
    assert_eq!((GATE_BASELINE_Q16, INHIBITORY_BASELINE_Q16), (0, 0x8000));
    assert_eq!(REWARD_Q16, ONE);
    // The criterion at its edges over blocks written by hand: clause 1 reads the last two
    // blocks before the flip and nothing after it; clause 2 the run's last two and nothing
    // before them; a tie is never counted, since the task's correct count is the rule's.
    let blocks_of = |correct: &[u32]| -> Vec<Block> {
        correct
            .iter()
            .map(|&c| {
                (
                    c,
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
                    (BLOCK as u32).saturating_sub(c),
                )
            })
            .collect()
    };
    let run_of = |before: [u32; 2], after: [u32; 2], rest: u32| -> Vec<Block> {
        let mut correct = vec![rest; REVERSAL_BLOCKS];
        correct[FLIP_BLOCK - 2] = before[0];
        correct[FLIP_BLOCK - 1] = before[1];
        correct[REVERSAL_BLOCKS - 2] = after[0];
        correct[REVERSAL_BLOCKS - 1] = after[1];
        blocks_of(&correct)
    };
    let edge = run_of([40, 40], [40, 40], 0);
    assert_eq!((correct_before(&edge), correct_after(&edge)), (80, 80));
    let v = revision([&edge, &edge]);
    assert_eq!(
        v,
        Revision {
            learned: [true; 2],
            revised: [true; 2],
            replicated: true,
            yes: true
        },
        "80 and 80 in both arms: yes"
    );
    let short_after = run_of([40, 40], [40, 39], 64);
    assert_eq!(correct_after(&short_after), 79);
    let v = revision([&edge, &short_after]);
    assert_eq!(
        v,
        Revision {
            learned: [true; 2],
            revised: [true, false],
            replicated: true,
            yes: false
        },
        "79 after the flip in one arm: no, whatever the blocks between"
    );
    let short_before = run_of([39, 40], [64, 64], 64);
    assert_eq!(correct_before(&short_before), 79);
    let v = revision([&short_before, &edge]);
    assert_eq!(
        v,
        Revision {
            learned: [false, true],
            revised: [true; 2],
            replicated: false,
            yes: false
        },
        "a clause-1 failure: not replicated, and no answer to H-17 whatever clause 2 reads"
    );
    let only_after = run_of([0, 0], [64, 64], 64);
    assert_eq!(
        correct_before(&only_after),
        0,
        "clause 1 reads no block after the flip"
    );
    let only_before = run_of([64, 64], [0, 0], 64);
    assert_eq!(
        correct_after(&only_before),
        0,
        "clause 2 reads no block before the run's last two"
    );
    assert_eq!(
        correct_before(&blocks_of(&[64; 23])),
        0,
        "a run short of the flip"
    );
    assert_eq!(
        correct_before(&blocks_of(&[64; 24])),
        128,
        "a run that reaches it"
    );
    assert_eq!(
        correct_after(&blocks_of(&[64; 47])),
        0,
        "a run short of 3 072"
    );
    assert_eq!(correct_after(&blocks_of(&[64; 49])), 0, "or past it");
    assert_eq!(correct_after(&blocks_of(&[64; 48])), 128);
    // ADR-0090's assertion's rule at its edges, and the pair the carry-over names.
    let held = |after_carry: u64, uncarried: u64, outside: u64, carried: u64| Held {
        after_carry,
        uncarried,
        outside,
        carried,
    };
    assert!(reversal_held(&Held::default()));
    assert!(
        reversal_held(&held(0, 0, 0, 170)),
        "the carry-over's moves are a reading, not a clause"
    );
    assert!(!reversal_held(&held(1, 0, 0, 0)), "(a)");
    assert!(!reversal_held(&held(0, 1, 0, 0)), "(b)");
    assert!(!reversal_held(&held(0, 0, 1, 0)), "(c)");
    let trial = |stimulus: u8, selection: Option<u8>, reward: i32| -> EarnedTrial {
        (
            stimulus,
            [0; 2],
            selection,
            reward > 0,
            reward,
            [[0; 2]; 2],
            0,
            0,
        )
    };
    assert_eq!(carried_pair(&trial(1, Some(1), ONE)), Some((1, 1)));
    assert_eq!(carried_pair(&trial(0, Some(1), ONE)), Some((0, 1)));
    assert_eq!(
        carried_pair(&trial(1, Some(0), ONE.saturating_neg())),
        None,
        "a punishment carries nothing"
    );
    assert_eq!(
        carried_pair(&trial(1, None, ONE.saturating_neg())),
        None,
        "nor a tie"
    );
    let old = assigned_pairs(false);
    assert_eq!(uncarried(old, Some((1, 1))), vec![(0, 0)]);
    assert_eq!(uncarried(old, Some((0, 0))), vec![(1, 1)]);
    assert_eq!(uncarried(old, None), vec![(0, 0), (1, 1)]);
    assert_eq!(uncarried(assigned_pairs(true), Some((1, 0))), vec![(0, 1)]);
    // The readings' rules over blocks written by hand: the need after the flip, the first
    // reward, the gaps and the new answer's rise.
    let mut hand = run_of([64, 64], [3, 1], 0);
    for (j, b) in hand.iter_mut().enumerate() {
        b.11 = if j >= FLIP_BLOCK { 2 } else { 0 };
    }
    hand[FLIP_BLOCK + 3].0 = 41;
    let earned_hand: Vec<EarnedBlock> = hand
        .iter()
        .map(|b| ([[0; 3]; 2], b.0, [[0; 2]; 2], 0, 0))
        .collect();
    assert_eq!(
        need(&hand, &earned_hand),
        Need {
            selected_new: 45,
            selected_old: 1_536 - 45 - 48,
            ties: 48,
            rewards: 45,
            crossed: Some(FLIP + 4 * BLOCK),
        },
        "the need reads the blocks after the flip and nothing before"
    );
    assert_eq!(
        need(&hand[..FLIP_BLOCK], &earned_hand[..FLIP_BLOCK]),
        Need::default(),
        "a run that stops at the flip needs nothing yet"
    );
    let mut read_hand = vec![trial(0, Some(0), ONE); REVERSAL_TRIALS];
    for t in read_hand.iter_mut().skip(FLIP) {
        *t = trial(0, Some(0), ONE.saturating_neg());
    }
    assert_eq!(
        first_reward(&read_hand),
        None,
        "the rewards before the flip are not read"
    );
    read_hand[FLIP + 700] = trial(0, Some(1), ONE);
    read_hand[FLIP + 900] = trial(1, Some(0), ONE);
    assert_eq!(first_reward(&read_hand), Some(FLIP + 700));
    read_hand[FLIP] = trial(1, Some(0), ONE);
    assert_eq!(first_reward(&read_hand), Some(FLIP), "the 1 537th counts");
    let mut gap_block = blocks_of(&[0])[0];
    gap_block.2 = [[30, 12], [11, 29]];
    assert_eq!(gaps(&[gap_block], false), vec![[18, 18]]);
    assert_eq!(gaps(&[gap_block], true), vec![[-18, -18]]);
    let mut rise = blocks_of(&[0; REVERSAL_BLOCKS]);
    rise[FLIP_BLOCK - 1].10 = [[100, 200], [300, 400]];
    rise[REVERSAL_BLOCKS - 1].10 = [[100, 207], [305, 400]];
    assert_eq!(
        new_rise(&rise, false),
        [7, 5],
        "A→R1 and B→R0 after the assignment"
    );
    assert_eq!(
        new_rise(&rise, true),
        [0, 0],
        "A→R0 and B→R1 after the mirrored"
    );
    assert_eq!(
        new_rise(&rise[..FLIP_BLOCK], false),
        [0; 2],
        "no block after the flip"
    );
    assert_eq!(once_blocks(&[], &[]), 0);
    // A few trials over a flip on the instrument's network at 1 024 units, the inhibitory
    // baseline set: the assignment first, flipped before the trial of index `GATE_FLIP`,
    // the oracle held at every trial inside `earned_run_flipped` and every trial's contract
    // asserted under the mapping in force; beside it the same network run to the flip's
    // trial with no flip. The trials before the flip are the same run; at the flip's trial
    // the stimulus, the counts, the selection, the signal at the trial's end and what the
    // trial consolidated are the same, and the correctness is the other mapping's, the
    // reward's sign with it; after it, every trial is judged under the other mapping.
    const GATE_FLIP: usize = GATE_TRIALS / 2;
    let p = prior(1024);
    let inhibited = || Config {
        inhibitory_baseline_q16: Some(INHIBITORY_BASELINE_Q16),
        ..config(1024, 2, GATE_BASELINE_Q16)
    };
    let mut flipped = at_gain(&p, inhibited(), GAIN_1024);
    let old = assigned_pairs(false);
    let mut snapshot: Option<Vec<Vec<i16>>> = None;
    let mut moved_at_flip = [0u64; 2];
    let run = reversal_run(
        &mut flipped,
        Reversal::AssignmentFirst,
        GATE_TRIALS,
        GATE_FLIP,
        &mut |exec, trial| {
            if trial.saturating_add(1) == GATE_FLIP {
                snapshot = Some(weights_of(exec));
            } else if trial == GATE_FLIP {
                let before = snapshot.as_ref().expect("the trial before the flip first");
                moved_at_flip = old.map(|pair| reach(exec, before, 1024, &[pair]).0);
            }
        },
    );
    let (blocks, trace, _, read, _) = &run;
    assert!(blocks.is_empty(), "a few trials are no whole block");
    assert_eq!(read.len(), GATE_TRIALS);
    let mut plain = at_gain(&p, inhibited(), GAIN_1024);
    let reference = earned_run_under(
        &mut plain,
        Feedback::Answer,
        false,
        1024,
        GATE_FLIP + 1,
        GATE_BASELINE_Q16,
    );
    let (_, _, _, unflipped, _) = &reference;
    eprintln!(
        "DUMP reversal1024 over the flip trace {trace:#018x} read {read:?} unflipped {unflipped:?} moved at the flip {moved_at_flip:?}"
    );
    assert_eq!(
        read[..GATE_FLIP],
        unflipped[..GATE_FLIP],
        "the trials before the flip are the run with no flip"
    );
    let (at, was) = (&read[GATE_FLIP], &unflipped[GATE_FLIP]);
    assert_eq!(
        (at.0, at.1, at.2, at.5, at.6),
        (was.0, was.1, was.2, was.5, was.6),
        "at the flip's trial the stimulus, the counts, the selection, what it consolidated and the signal at its end are the same"
    );
    assert!(
        at.2.is_some(),
        "the flip's trial selects a readout, so the mapping decides its correctness"
    );
    assert_eq!(at.3, !was.3, "and its correctness is the other mapping's");
    assert_eq!(
        (at.4, was.4),
        (
            if at.3 {
                REWARD_Q16
            } else {
                REWARD_Q16.saturating_neg()
            },
            if was.3 {
                REWARD_Q16
            } else {
                REWARD_Q16.saturating_neg()
            }
        ),
        "the reward's sign with it"
    );
    for (trial, t) in read.iter().enumerate() {
        let in_force = trial >= GATE_FLIP;
        assert_eq!(
            t.3,
            t.2 == Some(answer_of(t.0, in_force) as u8),
            "trial {trial}: correct under the mapping in force"
        );
        assert_eq!(
            t.4 > 0,
            t.3,
            "trial {trial}: the reward's sign is the outcome's"
        );
    }
    assert_eq!(
        derivation(read),
        [true; 3],
        "ADR-0080's derivation over the flip"
    );
    // ADR-0090 at the gate: the trial after the flip moves no old answer pair but the one the
    // trial before it carried, if it carried one.
    let carried = carried_pair(&read[GATE_FLIP - 1]);
    for (pair, &moved) in old.iter().zip(moved_at_flip.iter()) {
        if Some(*pair) != carried {
            assert_eq!(
                moved, 0,
                "the uncarried old answer pair {pair:?} held over the flip"
            );
        }
    }
    // And the carry-over itself, the defect ADR-0090 amends ADR-0089's assertion for: here the
    // trial before the flip was rewarded on A→R0, and the trial after it consolidated that
    // pair — 560 of its synapses moved — under the second mapping.
    assert_eq!(
        carried,
        Some((0, 0)),
        "the trial before the flip carried A→R0"
    );
    assert!(
        moved_at_flip[0] > 0 && read[GATE_FLIP].5[0][0] > 0,
        "and the first trial under the second mapping consolidated it: {moved_at_flip:?}"
    );
    // Over the pinned tables: each arm's whole run is H-16's arm of its first mapping followed
    // by the second half pinned here. The verdict as written, by the rule committed first; the
    // clauses' counts, the need and the readings as the constants state; and, block by block,
    // the tables consistent with one another, with ADR-0090's assertion and with the oracle's
    // consolidation — each coupling after a block is the one before it plus what the oracle
    // consolidated into that pair over the block.
    let image = QUIET_1024[SETTLED].1;
    let full: Vec<Vec<Block>> = REVERSAL_ARMS
        .iter()
        .enumerate()
        .map(|(k, &arm)| {
            [
                INHIBITION_BLOCKS_1024[replicated_arm(arm)],
                REVERSAL_BLOCKS_1024[k],
            ]
            .concat()
        })
        .collect();
    let earned_full: Vec<Vec<EarnedBlock>> = REVERSAL_ARMS
        .iter()
        .enumerate()
        .map(|(k, &arm)| {
            [
                INHIBITION_EARNED_1024[replicated_arm(arm)],
                REVERSAL_EARNED_1024[k],
            ]
            .concat()
        })
        .collect();
    let verdict = revision([&full[0], &full[1]]);
    assert_eq!(verdict, REVERSAL_1024, "the verdict as written");
    assert!(
        verdict.replicated,
        "clause 1 held in both arms: H-16 replicated"
    );
    assert_eq!(
        verdict.yes, REVERSAL_PREDICTED,
        "H-17 is no, as ADR-0089 predicted"
    );
    for (k, &arm) in REVERSAL_ARMS.iter().enumerate() {
        let blocks = &full[k];
        let earned = &earned_full[k];
        let first = first_mapping(arm);
        let h16 = replicated_arm(arm);
        assert_eq!(blocks.len(), REVERSAL_BLOCKS, "{arm:?}: forty-eight blocks");
        assert_eq!(earned.len(), REVERSAL_BLOCKS);
        assert_eq!(
            REVERSAL_COMPOSITIONS_1024[k].len(),
            REVERSAL_BLOCKS - FLIP_BLOCK
        );
        assert_ne!(REVERSAL_TRACES_1024[k], 0);
        assert_ne!(REVERSAL_READ_1024[k], 0);
        assert!(!REVERSAL_CENSUS_1024[k].is_empty());
        assert_eq!(
            [correct_before(blocks), correct_after(blocks)],
            CORRECT_REVERSAL_1024[k]
        );
        let need_read = need(blocks, earned);
        assert_eq!(need_read, NEED_1024[k], "{arm:?}: the need");
        assert_eq!(
            need_read
                .selected_new
                .saturating_add(need_read.selected_old)
                .saturating_add(need_read.ties),
            (REVERSAL_TRIALS - FLIP) as u32,
            "{arm:?}: every trial after the flip selected one readout or tied"
        );
        assert_eq!(
            need_read.rewards, need_read.selected_new,
            "{arm:?}: a reward for every selection of the new answer and none else"
        );
        assert_eq!(new_rise(blocks, first), NEW_RISE_1024[k]);
        assert_eq!(falls_every_block(image.0, blocks), FALLS_REVERSAL_1024[k]);
        assert_eq!(
            once_blocks(&blocks[..FLIP_BLOCK], INHIBITION_COMPOSITIONS_1024[h16]).saturating_add(
                once_blocks(&blocks[FLIP_BLOCK..], REVERSAL_COMPOSITIONS_1024[k])
            ),
            ONCE_BLOCKS_REVERSAL_1024[k]
        );
        assert!(
            reversal_held(&HELD_1024[k]) && HELD_1024[k].carried > 0,
            "{arm:?}: the assertion held, and the carry-over moved the carried pair"
        );
        assert_eq!(DERIVATION_REVERSAL_1024[k], [true; 3]);
        let last = blocks.last().expect("a block");
        assert_eq!(SUMS_AFTER_REVERSAL_1024[k], (last.7, last.8));
        // The first reward after the flip is in a block that earned one, and none before it.
        if let Some(t) = FIRST_REWARD_1024[k] {
            let at = t.checked_div(BLOCK).expect("a block");
            assert!(at >= FLIP_BLOCK && earned[at].1 > 0);
            assert!(earned[FLIP_BLOCK..at].iter().all(|b| b.1 == 0));
        }
        // The carry-over (ADR-0090): the 1 536th trial presented B and was rewarded on its
        // first mapping's answer, and only that pair moved in the 1 537th.
        let (at_flip, at_carry) = REVERSAL_SNAPSHOTS_1024[k];
        let carried = (1, answer_of(1, first));
        assert_eq!(CARRIED_1024[k], Some(carried));
        assert_eq!(
            at_flip,
            blocks[FLIP_BLOCK - 1].10,
            "the flip is at a block's end"
        );
        for &(s, r) in &ALL_PAIRS {
            if (s, r) == carried {
                assert_ne!(
                    at_carry[s][r], at_flip[s][r],
                    "{arm:?}: the carried pair moved"
                );
            } else {
                assert_eq!(at_carry[s][r], at_flip[s][r], "{arm:?}: {s}→{r} did not");
            }
        }
        let mut previous = IMAGE_COUPLINGS_1024;
        for (j, block) in blocks.iter().enumerate() {
            let after_flip = j >= FLIP_BLOCK;
            let in_force = first != after_flip;
            assert_eq!(
                block.1, full[0][j].1,
                "{arm:?} block {j}: the same trials present A in both arms"
            );
            let presented = [block.1, (BLOCK as u32).saturating_sub(block.1)];
            for (s, &n) in presented.iter().enumerate() {
                assert_eq!(
                    earned[j].0[s].iter().sum::<u32>(),
                    n,
                    "{arm:?} block {j}: every presentation of {s} selected or tied"
                );
            }
            assert_eq!(
                block.11,
                earned[j].0[0][2].saturating_add(earned[j].0[1][2]),
                "{arm:?} block {j}: the ties"
            );
            let correct = earned[j].0[0][answer_of(0, in_force)]
                .saturating_add(earned[j].0[1][answer_of(1, in_force)]);
            assert_eq!(
                block.0, correct,
                "{arm:?} block {j}: correct under the mapping in force"
            );
            assert_eq!(
                earned[j].1, correct,
                "{arm:?} block {j}: one reward per correct trial"
            );
            assert!(earned[j].3.abs() <= SIGNAL_END_FIXED_Q16);
            let mut rise = 0i64;
            for &(s, r) in &ALL_PAIRS {
                assert_eq!(
                    block.10[s][r],
                    previous[s][r].saturating_add(earned[j].2[s][r]),
                    "{arm:?} block {j}: {s}→{r} moved by what the oracle consolidated"
                );
                rise =
                    rise.saturating_add(block.10[s][r].saturating_sub(IMAGE_COUPLINGS_1024[s][r]));
                let old = r == answer_of(s as u8, first);
                if after_flip && old {
                    assert_eq!(
                        block.10[s][r], at_carry[s][r],
                        "{arm:?} block {j}: the old answer's pair {s}→{r} held from the 1 537th trial"
                    );
                }
                if !after_flip && !old {
                    assert_eq!(
                        block.10[s][r], IMAGE_COUPLINGS_1024[s][r],
                        "{arm:?} block {j}: the second mapping's pair {s}→{r} is the image's before the flip"
                    );
                }
            }
            assert_eq!(
                block.8,
                image.1.saturating_add(rise),
                "{arm:?} block {j}: the excitatory sum moved by the four pairs' moves and nothing else"
            );
            previous = block.10;
        }
    }
}

// =================================================================================== H-18

// ------------------------------------------ written before the run (ADR-0093, ADR-0095)

/// H-18's run (ADR-0093): three of H-16's, seventy-two blocks, the second mapping 3 072 trials
/// long, the length derived before any run from ADR-0091's readings.
const PUNISHED_TRIALS: usize = 3 * INHIBITION_TRIALS;
const _: () = assert!(PUNISHED_TRIALS == 4_608 && PUNISHED_TRIALS / BLOCK == 72);
/// The blocks of an H-18 run.
const PUNISHED_BLOCKS: usize = PUNISHED_TRIALS / BLOCK;
/// The flip is H-17's (`FLIP`, before the trial of index 1 536; ADR-0089): the second mapping
/// runs the rest, 3 072 trials in forty-eight blocks, and the criterion's two windows each lie
/// wholly under one mapping.
const _: () = assert!(PUNISHED_TRIALS - FLIP == 3_072 && PUNISHED_BLOCKS - FLIP_BLOCK == 48);
const _: () = assert!(FLIP_BLOCK >= LAST_BLOCKS && PUNISHED_BLOCKS - FLIP_BLOCK >= LAST_BLOCKS);

/// The arms of H-18 (ADR-0093): H-17's two, in their order, each its own weekly test — the
/// assignment first and the mirrored first — from the one signed image, the flip
/// `Task::mirrored` negated and nothing else.
const PUNISHED_ARMS: [Reversal; 2] = REVERSAL_ARMS;

/// ADR-0093 writes no prediction for the verdict.
const PUNISHED_PREDICTED: Option<bool> = None;

/// ADR-0093's three predicted readings, Hypotheses written before the run and never asserted:
/// (a) after the flip the old answer pair falls, block on block on average, until the
/// selection crosses (`old_falls`); (b) its fall is fastest in the first blocks after the flip
/// and slows as it falls (`fall_slows`); (c) in the first half the wrong pairs end below the
/// image's couplings, depressed while they were selected early (`wrong_below`). Each is
/// predicted true for both stimuli in both arms.
const OLD_FALLS_PREDICTED: bool = true;
const FALL_SLOWS_PREDICTED: bool = true;
const WRONG_BELOW_PREDICTED: bool = true;

/// The modulator section's byte that holds the signed gate's flag (ADR-0094), which the
/// signed image sets.
const SIGNED_GATE_BYTE: usize = 25;

// ------------------------------------------------------------ the criterion (ADR-0093)

/// Clause 2's count under H-18: the correct selections over the run's last `LAST_BLOCKS`
/// blocks, trials 4 481 to 4 608, under the second mapping; zero for a run of any other
/// length.
fn revised_count(blocks: &[Block]) -> u32 {
    if blocks.len() == PUNISHED_BLOCKS {
        last_correct(blocks)
    } else {
        0
    }
}

/// H-18's criterion (ADR-0093), clause by clause per arm `[assignment first, mirrored first]`:
/// (1) it learned — `correct_before`, H-17's count over the two blocks before the flip, trials
/// 1 409 to 1 536 under the first mapping, at least `REWARDED_MIN`; (2) it revised —
/// `revised_count` at least `REWARDED_MIN`; a tie not correct, the task's count. `yes` is all
/// four. The signed gate acts in the first half too, so the first half is not H-16's run and
/// nothing is replicated: a failure of clause 1 is a no about the signed gate.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Punished {
    learned: [bool; 2],
    revised: [bool; 2],
    yes: bool,
}

fn punished(arms: [&[Block]; 2]) -> Punished {
    let learned = arms.map(|blocks| correct_before(blocks) >= REWARDED_MIN);
    let revised = arms.map(|blocks| revised_count(blocks) >= REWARDED_MIN);
    Punished {
        learned,
        revised,
        yes: learned[0] && learned[1] && revised[0] && revised[1],
    }
}

// ------------------------------------------------ the assertion's shape (ADR-0093)

/// ADR-0093's assertion as a rule over an arm's reach from the image to the run's end: no
/// excitatory synapse outside the four stimulus–readout pairs moved. Beside it, the oracle —
/// extended to the signed branch — is held to the record's weights, traces and signal at
/// every trial inside `earned_run_signed`. True on both arms is the assertion; false on one is
/// a finding, reported beside the verdict and not in place of it.
fn punished_held(reach: &Reach) -> bool {
    reach.excitatory.1 == 0
}

// --------------------------------------------------- the readings' shape (ADR-0093)

/// The old answer pairs' couplings, `[A, B]` — each stimulus onto its answer under the first
/// mapping — at the end of every block from the last before the flip to the run's end.
fn old_course(blocks: &[Block], first: bool) -> Vec<[i64; 2]> {
    blocks
        .iter()
        .skip(FLIP_BLOCK - 1)
        .map(|b| [0u8, 1].map(|s| b.10[usize::from(s)][answer_of(s, first)]))
        .collect()
}

/// Per stimulus `[A, B]`, the first block after the flip, by index, in which the stimulus
/// selected its answer under the second mapping more often than its answer under the first
/// (the earned splits): where its selection crossed; none when no block did.
fn crossed_block(earned: &[EarnedBlock], first: bool) -> [Option<usize>; 2] {
    [0u8, 1].map(|s| {
        let (old, new) = (answer_of(s, first), answer_of(s, !first));
        earned
            .iter()
            .enumerate()
            .skip(FLIP_BLOCK)
            .find(|(_, b)| b.0[usize::from(s)][new] > b.0[usize::from(s)][old])
            .map(|(j, _)| j)
    })
}

/// Per stimulus, the first trial after the flip, by index, that selected the stimulus's
/// answer under the second mapping; none when none did.
fn first_new(read: &[EarnedTrial], first: bool) -> [Option<usize>; 2] {
    [0u8, 1].map(|s| {
        let new = answer_of(s, !first) as u8;
        read.iter()
            .enumerate()
            .skip(FLIP)
            .find(|(_, t)| t.0 == s && t.2 == Some(new))
            .map(|(k, _)| k)
    })
}

/// Per stimulus, the last block the predicted readings (a) and (b) read: the stimulus's
/// crossing block, or the run's last block when it never crossed; none for a run that does
/// not pass the flip.
fn span_end(blocks: &[Block], earned: &[EarnedBlock], first: bool) -> [Option<usize>; 2] {
    let last = blocks.len().checked_sub(1).filter(|&j| j >= FLIP_BLOCK);
    let crossed = crossed_block(earned, first);
    crossed.map(|c| last.map(|l| c.unwrap_or(l).min(l)))
}

/// ADR-0093's predicted reading (a) as a rule: per stimulus, the old answer pair's coupling at
/// the end of its span below its coupling at the flip, the end of the 1 536th trial — a fall
/// on average over the blocks from the flip until the selection crossed.
fn old_falls(blocks: &[Block], earned: &[EarnedBlock], first: bool) -> [bool; 2] {
    let ends = span_end(blocks, earned, first);
    [0usize, 1].map(|s| {
        let old = answer_of(s as u8, first);
        match (
            blocks.get(FLIP_BLOCK - 1),
            ends[s].and_then(|j| blocks.get(j)),
        ) {
            (Some(flip), Some(end)) => end.10[s][old] < flip.10[s][old],
            _ => false,
        }
    })
}

/// ADR-0093's predicted reading (b) as a rule: per stimulus, over the span's blocks after the
/// flip, the old answer pair's fall over the first half of them (rounded down) greater than
/// its fall over the rest; false for a span of fewer than two blocks after the flip.
fn fall_slows(blocks: &[Block], earned: &[EarnedBlock], first: bool) -> [bool; 2] {
    let ends = span_end(blocks, earned, first);
    [0usize, 1].map(|s| {
        let old = answer_of(s as u8, first);
        let Some(end) = ends[s] else {
            return false;
        };
        let after = end.saturating_add(1).saturating_sub(FLIP_BLOCK);
        if after < 2 {
            return false;
        }
        let mid = (FLIP_BLOCK - 1).saturating_add(after >> 1);
        let at = |j: usize| blocks.get(j).map_or(0, |b| b.10[s][old]);
        let early = at(FLIP_BLOCK - 1).saturating_sub(at(mid));
        let late = at(mid).saturating_sub(at(end));
        early > late
    })
}

/// ADR-0093's predicted reading (c) as a rule: per stimulus, the wrong pair's coupling — the
/// stimulus onto the readout that is not its answer under the first mapping — at the end of
/// the first half below the image's; false for a run that does not reach the flip.
fn wrong_below(blocks: &[Block], first: bool) -> [bool; 2] {
    let Some(flip) = blocks.get(FLIP_BLOCK - 1) else {
        return [false; 2];
    };
    [0usize, 1].map(|s| {
        let wrong = answer_of(s as u8, !first);
        flip.10[s][wrong] < IMAGE_COUPLINGS_1024[s][wrong]
    })
}

/// A block of what the consolidation did to the addressed pairs (brief 041), split by the
/// sign of the delivery each trial consolidated: `[after a reward, after a punishment]`, each
/// `([rose, fell, stayed], [raised, lowered])` summed over the block's trials.
type MovesBlock = [Moves; 2];

/// An arm's moves per block from its trials: a trial consolidates the previous trial's
/// delivery (ADR-0090), so trial `t`'s moves count under the sign of trial `t − 1`'s reward,
/// in trial `t`'s block; the first trial's, which consolidates nothing addressed, under
/// neither.
fn moves_blocks(read: &[EarnedTrial], moves: &[Moves]) -> Vec<MovesBlock> {
    let mut out = vec![[([0u32; 3], [0i64; 2]); 2]; read.len().div_ceil(BLOCK)];
    for (t, m) in moves.iter().enumerate().skip(1) {
        let Some(previous) = t.checked_sub(1).and_then(|p| read.get(p)) else {
            continue;
        };
        let side = match previous.4 {
            1.. => 0,
            0 => continue,
            _ => 1,
        };
        let Some(block) = t.checked_div(BLOCK).and_then(|j| out.get_mut(j)) else {
            continue;
        };
        add_moves(&mut block[side], m);
    }
    out
}

/// `into` plus `m`, count by count and amount by amount.
fn add_moves(into: &mut Moves, m: &Moves) {
    for (a, b) in into.0.iter_mut().zip(m.0.iter()) {
        *a = a.saturating_add(*b);
    }
    for (a, b) in into.1.iter_mut().zip(m.1.iter()) {
        *a = a.saturating_add(*b);
    }
}

/// The moves after a punishment (`side` 1) or after a reward (`side` 0), summed over the
/// blocks before the flip and over the blocks from it, `[before, after]`: the readings of how
/// often a punishment potentiated an addressed synapse, one whose trace was negative.
fn moves_totals(blocks: &[MovesBlock], side: usize) -> [Moves; 2] {
    let mut out = [([0u32; 3], [0i64; 2]); 2];
    for (j, block) in blocks.iter().enumerate() {
        add_moves(&mut out[usize::from(j >= FLIP_BLOCK)], &block[side]);
    }
    out
}

// ---------------------------------------------------------------- the run (brief 041)

/// The signed image (ADR-0094): the inhibited image with the modulator section's flag at
/// `[25]` set, the section re-sealed; every other byte the inhibited image's.
fn signed_image(inhibited: &[u8]) -> Vec<u8> {
    let mut img = inhibited.to_vec();
    let header = CortexFileHeader::decode(img[0..64].try_into().unwrap());
    for at in (64..).step_by(64).take(header.section_count as usize) {
        let mut entry = SectionEntry::decode(img[at..][..64].try_into().unwrap());
        if entry.kind == SECTION_MODULATOR {
            let (offset, length) = (entry.offset as usize, entry.length as usize);
            assert_eq!(img[offset..][SIGNED_GATE_BYTE], 0, "the gate unset before");
            img[offset..][SIGNED_GATE_BYTE] = 1;
            entry.crc64 = crc64(&img[offset..][..length]);
            img[at..][..64].copy_from_slice(&entry.encode());
            return img;
        }
    }
    panic!("the image holds a modulator section");
}

/// The engine from the signed image, decoded under the calibration's configuration: the
/// image's baseline, inhibitory baseline, signed gate, gain and step outrank the
/// configuration's (§8.3); the baseline asserted zero, the inhibitory baseline 0.5 and the
/// signed gate set.
fn signed_from(image: &[u8], units: u32) -> Engine {
    let exec = inhibited_from(image, units);
    assert!(exec.signed_gate(), "the image carries the signed gate");
    exec
}

/// The images of the one settled engine (brief 041): ADR-0077's frozen image for the
/// calibration, and the signed image for the arms — built from the inhibited image as H-16's
/// and H-17's tests build it, decoded and asserted to carry the gate, the sums, the gain and
/// the step, and to differ from the inhibited image in the flag byte and the section's CRC
/// and nowhere else.
fn signed_images(name: &str) -> (Vec<u8>, Vec<u8>) {
    let (zero, inhibited) = inhibited_images(name);
    let signed = signed_image(&inhibited);
    assert!(
        !inhibited_from(&inhibited, 1024).signed_gate(),
        "{name}: the inhibited image leaves the gate unset"
    );
    let decoded = signed_from(&signed, 1024);
    assert_eq!(
        weights_by_polarity(&decoded),
        QUIET_1024[SETTLED].1,
        "{name}: the signed image carries the weights"
    );
    assert_eq!(decoded.homeostasis().synaptic_gain_q16, GAIN_1024);
    assert_eq!(
        decoded.homeostasis().control_step_q0_16,
        BACKGROUNDS[SETTLED]
    );
    assert_eq!(signed.len(), inhibited.len(), "{name}: one image, twice");
    let differing = signed
        .iter()
        .zip(inhibited.iter())
        .filter(|(a, b)| a != b)
        .count();
    assert!(
        differing > 0 && differing <= 9,
        "{name}: the images differ in the flag and the section's CRC: {differing} bytes"
    );
    eprintln!(
        "DUMP {name} signed image {} bytes, {differing} differ from the inhibited",
        signed.len()
    );
    (zero, signed)
}

/// An arm's run from the signed engine (brief 041): `earned_run_signed` under the answer's
/// feedback at the gate's zero with the signed gate set, the arm's first mapping, the flip
/// before the trial of index `flip`, and `after` reading the executor at every trial's end —
/// the oracle, consolidating every addressed replayed synapse by `consolidated_signed`, held
/// to the record at every trial, and every trial's contract asserted under the mapping in
/// force at it.
fn punished_run(
    exec: &mut Engine,
    arm: Reversal,
    trials: usize,
    flip: usize,
    after: &mut dyn FnMut(&Engine, usize),
) -> (EarnedRun, Vec<Moves>) {
    earned_run_signed(
        exec,
        Feedback::Answer,
        first_mapping(arm),
        1024,
        trials,
        GATE_BASELINE_Q16,
        Some(flip),
        true,
        after,
    )
}

/// One arm of H-18 at 1 024 units (brief 041): the settled engine held to ADR-0077 step by
/// step and its images; the calibration — a frozen block from the zero image, the inhibitory
/// baseline and the signed gate unset, the reward withheld, held to ADR-0077's frozen run —
/// before any rewarded run (H-18's stopping rule, step 2); then the arm's 4 608 trials from
/// the signed image, the flip between the 1 536th and the 1 537th, the couplings read at the
/// end of the 1 537th; everything dumped and the clauses, the assertion and the readings
/// computed before anything is held; then the assertion, and the pinned tables.
fn punished_arm(arm: Reversal) {
    let k = PUNISHED_ARMS
        .iter()
        .position(|&a| a == arm)
        .expect("an arm of H-18");
    let name = format!("punished1024 {arm:?}");
    let (zero, signed) = signed_images(&name);
    let image_crc = crc64(&signed);
    {
        let mut frozen = frozen_from(&zero, 1024);
        assert_eq!(
            (frozen.inhibitory_baseline_q16(), frozen.signed_gate()),
            (None, false),
            "{name}: the calibration's image leaves the inhibitory baseline and the signed gate unset"
        );
        let calibration = taught_run(&mut frozen, Arm::Withheld, 1024, BLOCK);
        calibration_holds(&format!("{name} calibration"), &calibration);
        eprintln!(
            "DUMP {name} calibration holds: ADR-0077's settled candidate reproduced; image crc {image_crc:#018x}"
        );
    }
    let sets = geometry(1024, ROTATION_1024);
    let image_sums = QUIET_1024[SETTLED].1;
    let mut exec = signed_from(&signed, 1024);
    assert_eq!(
        weights_by_polarity(&exec),
        image_sums,
        "{name}: the image's sums"
    );
    assert_eq!(
        pair_couplings(&exec, &sets),
        IMAGE_COUPLINGS_1024,
        "{name}: the same image"
    );
    let image = weights_of(&exec);
    let first = first_mapping(arm);
    let mut at_carry: Option<[[i64; 2]; 2]> = None;
    let (run, moves) = punished_run(&mut exec, arm, PUNISHED_TRIALS, FLIP, &mut |exec, trial| {
        if trial == FLIP {
            at_carry = Some(pair_couplings(exec, &sets));
        }
    });
    let (blocks, trace, trials, read, volley_ticks) = &run;
    let at_carry = at_carry.expect("the run reached the trial after the flip");
    let earned = earned_blocks(read);
    let compositions: Vec<Composition> = trials.chunks(BLOCK).map(composition).collect();
    let moved = moves_blocks(read, &moves);
    assert_eq!(blocks.len(), PUNISHED_BLOCKS, "{name}: seventy-two blocks");
    assert_eq!(read.len(), PUNISHED_TRIALS);
    assert_eq!(moved.len(), PUNISHED_BLOCKS);
    // Everything dumped, and the clauses, the assertion and the readings computed, before
    // anything is held.
    dump_earned(&name, &run, &earned);
    eprintln!("DUMP {name} PIN blocks {blocks:?}");
    eprintln!("DUMP {name} PIN trace {trace:#018x}");
    eprintln!("DUMP {name} PIN compositions {compositions:?}");
    eprintln!("DUMP {name} PIN earned {earned:?}");
    eprintln!("DUMP {name} PIN read {:#018x}", earned_hash(read));
    eprintln!("DUMP {name} PIN census {:?}", census_of(volley_ticks));
    eprintln!("DUMP {name} PIN moves {moved:?}");
    eprintln!("DUMP {name} PIN carry {at_carry:?}");
    let reach = reach_by_polarity(&exec, &image, 1024, &ALL_PAIRS);
    let correct = [correct_before(blocks), revised_count(blocks)];
    let need_read = need(blocks, &earned);
    let first_rewarded = first_reward(read);
    let first_new_read = first_new(read, first);
    let crossed = crossed_block(&earned, first);
    let old_falls_read = old_falls(blocks, &earned, first);
    let fall_slows_read = fall_slows(blocks, &earned, first);
    let wrong_below_read = wrong_below(blocks, first);
    let punished_moves = moves_totals(&moved, 1);
    let rewarded_moves = moves_totals(&moved, 0);
    let new_rise_read = new_rise(blocks, first);
    let once = once_blocks(blocks, &compositions);
    let falls = falls_every_block(image_sums.0, blocks);
    let derivation_read = derivation(read);
    let sums_after = weights_by_polarity(&exec);
    eprintln!(
        "DUMP {name} PIN readings correct {correct:?} reach {reach:?} need {need_read:?} first reward {first_rewarded:?} first new {first_new_read:?} crossed {crossed:?} old falls {old_falls_read:?} (predicted {OLD_FALLS_PREDICTED}) slows {fall_slows_read:?} (predicted {FALL_SLOWS_PREDICTED}) wrong below {wrong_below_read:?} (predicted {WRONG_BELOW_PREDICTED}) punished moves {punished_moves:?} rewarded moves {rewarded_moves:?} new rise {new_rise_read:?} once {once} falls {falls} derivation {derivation_read:?} sums after {sums_after:?} image crc {image_crc:#018x}"
    );
    eprintln!(
        "DUMP {name} old course {:?} gaps {:?} course {:?} image {image_sums:?} last splits {:?}",
        old_course(blocks, first),
        gaps(blocks, first),
        course(image_sums.0, blocks),
        last_splits(read)
    );
    // The assertion (ADR-0093), after the dump and beside the verdict.
    assert!(
        punished_held(&reach),
        "{name}: ADR-0093's assertion — no excitatory synapse outside the four stimulus–readout pairs moved: {reach:?}"
    );
    // The pinned tables, and the readings as the constants state.
    pinned(
        &format!("{name} sight"),
        blocks,
        *trace,
        PUNISHED_BLOCKS_1024[k],
        PUNISHED_TRACES_1024[k],
    );
    assert_eq!(
        compositions.as_slice(),
        PUNISHED_COMPOSITIONS_1024[k],
        "{name}: the composition per block"
    );
    assert_eq!(
        earned.as_slice(),
        PUNISHED_EARNED_1024[k],
        "{name}: the earned blocks"
    );
    assert_eq!(
        earned_hash(read),
        PUNISHED_READ_1024[k],
        "{name}: the readings"
    );
    assert_eq!(
        census_of(volley_ticks),
        PUNISHED_CENSUS_1024[k].to_vec(),
        "{name}: the volley's ticks"
    );
    assert_eq!(
        moved.as_slice(),
        PUNISHED_MOVES_1024[k],
        "{name}: the moves per block"
    );
    assert_eq!(image_crc, PUNISHED_IMAGE_CRC_1024, "{name}: the one image");
    assert_eq!(at_carry, PUNISHED_CARRY_1024[k]);
    assert_eq!(correct, CORRECT_PUNISHED_1024[k]);
    assert_eq!(reach, REACH_PUNISHED_1024[k]);
    assert_eq!(need_read, NEED_PUNISHED_1024[k]);
    assert_eq!(first_rewarded, FIRST_REWARD_PUNISHED_1024[k]);
    assert_eq!(first_new_read, FIRST_NEW_1024[k]);
    assert_eq!(crossed, CROSSED_BLOCK_1024[k]);
    assert_eq!(old_falls_read, OLD_FALLS_1024[k]);
    assert_eq!(fall_slows_read, FALL_SLOWS_1024[k]);
    assert_eq!(wrong_below_read, WRONG_BELOW_1024[k]);
    assert_eq!(punished_moves, PUNISHED_MOVES_TOTAL_1024[k]);
    assert_eq!(rewarded_moves, REWARDED_MOVES_TOTAL_1024[k]);
    assert_eq!(new_rise_read, NEW_RISE_PUNISHED_1024[k]);
    assert_eq!(once, ONCE_BLOCKS_PUNISHED_1024[k]);
    assert_eq!(falls, FALLS_PUNISHED_1024[k]);
    assert_eq!(derivation_read, DERIVATION_PUNISHED_1024[k]);
    assert_eq!(
        (sums_after, blocks.last().map(|b| (b.7, b.8))),
        (
            SUMS_AFTER_PUNISHED_1024[k],
            Some(SUMS_AFTER_PUNISHED_1024[k])
        ),
        "{name}: the sums after the run are the last block's"
    );
}

/// H-18's arm that starts from the assignment (brief 041): A onto readout 0 and B onto
/// readout 1 for 1 536 trials, then the mirrored mapping for 3 072, the signed gate set.
#[test]
#[ignore]
fn the_punished_pair_from_the_assignment_at_1024_units_exhaustive() {
    punished_arm(Reversal::AssignmentFirst);
}

/// H-18's arm that starts from the mirrored assignment (brief 041): A onto readout 1 and B
/// onto readout 0 for 1 536 trials, then the assignment for 3 072, the signed gate set.
#[test]
#[ignore]
fn the_punished_pair_from_the_mirrored_assignment_at_1024_units_exhaustive() {
    punished_arm(Reversal::MirroredFirst);
}

/// The gate's test (ADR-0061's class; brief 041): the arms and the constants as ADR-0093 fixed
/// them; the criterion's two clauses at their edges over blocks written by hand, a clause-1
/// failure among them; the assertion's rule; the readings' rules over blocks and trials
/// written by hand; `with_version` on a small image (ADR-0095); and a few trials over a flip
/// on the instrument's network at 1 024 units with the inhibitory baseline set, the signed
/// gate set beside the same run with it unset — the oracle held at every trial inside
/// `earned_run_signed` in both — every trial the same up to the first punishment and the one
/// after it, in which the punished pair moves against its trace under the gate and not
/// without it, and no excitatory synapse outside the pairs the deliveries addressed moves.
/// No whole run, and nothing else added to the gate.
#[test]
fn a_few_trials_over_a_punishment_at_1024_units_and_the_rules_of_the_punished_pair() {
    // The arms and the constants.
    assert_eq!(
        PUNISHED_ARMS,
        [Reversal::AssignmentFirst, Reversal::MirroredFirst]
    );
    assert!(!first_mapping(Reversal::AssignmentFirst) && first_mapping(Reversal::MirroredFirst));
    assert_eq!(
        (PUNISHED_TRIALS, PUNISHED_BLOCKS, FLIP, FLIP_BLOCK),
        (4_608, 72, 1_536, 24)
    );
    assert_eq!(PUNISHED_PREDICTED, None, "ADR-0093 predicts no verdict");
    assert_eq!(
        [
            OLD_FALLS_PREDICTED,
            FALL_SLOWS_PREDICTED,
            WRONG_BELOW_PREDICTED
        ],
        [true; 3],
        "ADR-0093's three predicted readings"
    );
    assert_eq!(SIGNED_GATE_BYTE, 25, "ADR-0094's flag");
    // Every constant of H-18 restated unchanged: ADR-0065's window, trial, seed and gain,
    // ADR-0066's mark and window of the criterion, ADR-0076's stimulus and cancel, ADR-0077's
    // settled candidate, ADR-0080's reward, ADR-0085's two baselines, ADR-0089's flip.
    assert_eq!(
        (WINDOW.from, WINDOW.ticks, TRIAL_TICKS, SEED, GAIN_1024),
        (100, 500, 1 << 14, 27, 0x0001_C000)
    );
    assert_eq!((REWARDED_MIN, LAST_BLOCKS, BLOCK), (80, 2, 64));
    assert_eq!(SHAPE_F46, (2, 0x0001_4000));
    assert_eq!(CANCEL_PICKED_1024, Some(CANCEL_AT_THE_EXTREME));
    assert_eq!((SETTLED, BACKGROUNDS[SETTLED]), (0, 0));
    assert_eq!(INHIBITION_TRIALS, REINFORCED_TRIALS);
    assert_eq!((GATE_BASELINE_Q16, INHIBITORY_BASELINE_Q16), (0, 0x8000));
    assert_eq!(REWARD_Q16, ONE);
    assert_eq!((FLIP, LAST_BEFORE_FLIP), (INHIBITION_TRIALS, 1_535));
    // The criterion at its edges over blocks written by hand: clause 1 reads the two blocks
    // before the flip and nothing else; clause 2 the run's last two and nothing before them,
    // and only in a run of seventy-two blocks; a tie is never counted.
    let blocks_of = |correct: &[u32]| -> Vec<Block> {
        correct
            .iter()
            .map(|&c| {
                (
                    c,
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
                    (BLOCK as u32).saturating_sub(c),
                )
            })
            .collect()
    };
    let run_of = |before: [u32; 2], after: [u32; 2], rest: u32| -> Vec<Block> {
        let mut correct = vec![rest; PUNISHED_BLOCKS];
        correct[FLIP_BLOCK - 2] = before[0];
        correct[FLIP_BLOCK - 1] = before[1];
        correct[PUNISHED_BLOCKS - 2] = after[0];
        correct[PUNISHED_BLOCKS - 1] = after[1];
        blocks_of(&correct)
    };
    let edge = run_of([40, 40], [40, 40], 0);
    assert_eq!((correct_before(&edge), revised_count(&edge)), (80, 80));
    assert_eq!(
        punished([&edge, &edge]),
        Punished {
            learned: [true; 2],
            revised: [true; 2],
            yes: true
        },
        "80 and 80 in both arms: yes"
    );
    let short_after = run_of([40, 40], [40, 39], 64);
    assert_eq!(revised_count(&short_after), 79);
    assert_eq!(
        punished([&edge, &short_after]),
        Punished {
            learned: [true; 2],
            revised: [true, false],
            yes: false
        },
        "79 at the end in one arm: no, whatever the blocks between"
    );
    let short_before = run_of([39, 40], [64, 64], 64);
    assert_eq!(correct_before(&short_before), 79);
    assert_eq!(
        punished([&edge, &short_before]),
        Punished {
            learned: [true, false],
            revised: [true; 2],
            yes: false
        },
        "a clause-1 failure: a no about the signed gate, whatever clause 2 reads"
    );
    assert_eq!(
        correct_before(&run_of([0, 0], [64, 64], 64)),
        0,
        "clause 1 reads no block after the flip"
    );
    assert_eq!(
        revised_count(&run_of([64, 64], [0, 0], 64)),
        0,
        "clause 2 reads no block before the run's last two"
    );
    assert_eq!(
        revised_count(&blocks_of(&[64; 48])),
        0,
        "H-17's length is not H-18's"
    );
    assert_eq!(
        revised_count(&blocks_of(&[64; 71])),
        0,
        "a run short of 4 608"
    );
    assert_eq!(revised_count(&blocks_of(&[64; 73])), 0, "or past it");
    assert_eq!(revised_count(&blocks_of(&[64; 72])), 128);
    // The assertion's rule.
    assert!(punished_held(&Reach::default()));
    assert!(
        punished_held(&Reach {
            excitatory: (1_600, 0),
            inhibitory: (0, 6_000),
        }),
        "the four pairs and the inhibitory synapses move by the rules"
    );
    assert!(!punished_held(&Reach {
        excitatory: (0, 1),
        inhibitory: (0, 0),
    }));
    // The readings' rules over blocks written by hand, the assignment first: A's old answer
    // is readout 0 and its new one readout 1, B's the other way round.
    let mut hand = blocks_of(&[0; PUNISHED_BLOCKS]);
    for (j, b) in hand.iter_mut().enumerate() {
        // A→R0 (old) falls by 100 a block from the flip for eight blocks, by 20 for the
        // next eight, then stays; B→R1 (old) never falls; the wrong pairs of the first half,
        // A→R1 and B→R0, end it below and above the image's.
        let after = j.saturating_add(1).saturating_sub(FLIP_BLOCK) as i64;
        let fall = after
            .min(8)
            .saturating_mul(100)
            .saturating_add(after.saturating_sub(8).clamp(0, 8).saturating_mul(20));
        b.10 = [
            [
                IMAGE_COUPLINGS_1024[0][0].saturating_sub(fall),
                IMAGE_COUPLINGS_1024[0][1].saturating_sub(5),
            ],
            [
                IMAGE_COUPLINGS_1024[1][0].saturating_add(5),
                IMAGE_COUPLINGS_1024[1][1],
            ],
        ];
    }
    let mut earned_hand: Vec<EarnedBlock> = hand
        .iter()
        .map(|_| ([[32, 0, 0], [0, 32, 0]], 0, [[0; 2]; 2], 0, 0))
        .collect();
    // A crosses in the sixteenth block after the flip (index 39); B never.
    earned_hand[FLIP_BLOCK + 15].0[0] = [10, 22, 0];
    assert_eq!(
        crossed_block(&earned_hand, false),
        [Some(FLIP_BLOCK + 15), None]
    );
    assert_eq!(
        crossed_block(&earned_hand[..FLIP_BLOCK], false),
        [None; 2],
        "nothing crosses before the flip"
    );
    let mut tied = earned_hand.clone();
    tied[FLIP_BLOCK + 15].0[0] = [16, 16, 0];
    assert_eq!(
        crossed_block(&tied, false),
        [None; 2],
        "as many selections of each is no crossing"
    );
    assert_eq!(
        crossed_block(&earned_hand, true),
        [Some(FLIP_BLOCK); 2],
        "under the other first mapping the answers swap, and both stimuli already select the new one at the flip"
    );
    assert_eq!(
        span_end(&hand, &earned_hand, false),
        [Some(FLIP_BLOCK + 15), Some(PUNISHED_BLOCKS - 1)]
    );
    assert_eq!(
        span_end(&hand[..FLIP_BLOCK], &earned_hand, false),
        [None; 2]
    );
    assert_eq!(
        old_falls(&hand, &earned_hand, false),
        [true, false],
        "A's old pair fell to its crossing; B's never moved"
    );
    assert_eq!(
        fall_slows(&hand, &earned_hand, false),
        [true, false],
        "A: 800 over the first eight blocks against 160 over the next eight"
    );
    let mut steady = hand.clone();
    for (j, b) in steady.iter_mut().enumerate() {
        let after = j.saturating_add(1).saturating_sub(FLIP_BLOCK) as i64;
        b.10[0][0] = IMAGE_COUPLINGS_1024[0][0].saturating_sub(after.saturating_mul(50));
    }
    assert_eq!(
        (
            old_falls(&steady, &earned_hand, false),
            fall_slows(&steady, &earned_hand, false)
        ),
        ([true, false], [false, false]),
        "a steady fall is a fall and does not slow"
    );
    let mut at_once = earned_hand.clone();
    at_once[FLIP_BLOCK].0[0] = [0, 32, 0];
    assert_eq!(
        fall_slows(&hand, &at_once, false),
        [false, false],
        "a span of one block after the flip cannot read a slowing"
    );
    assert_eq!(
        wrong_below(&hand, false),
        [true, false],
        "A→R1 ended the first half below the image's, B→R0 above it"
    );
    assert_eq!(wrong_below(&hand[..FLIP_BLOCK - 1], false), [false; 2]);
    assert_eq!(
        old_course(&hand, false).first(),
        Some(&[IMAGE_COUPLINGS_1024[0][0], IMAGE_COUPLINGS_1024[1][1]]),
        "the course starts at the flip"
    );
    assert_eq!(
        old_course(&hand, false).len(),
        PUNISHED_BLOCKS - FLIP_BLOCK + 1
    );
    let trial = |stimulus: u8, selection: Option<u8>, reward: i32| -> EarnedTrial {
        (
            stimulus,
            [0; 2],
            selection,
            reward > 0,
            reward,
            [[0; 2]; 2],
            0,
            0,
        )
    };
    let mut read_hand = vec![trial(0, Some(0), ONE); PUNISHED_TRIALS];
    assert_eq!(
        first_new(&read_hand, false),
        [None; 2],
        "no new answer selected"
    );
    read_hand[FLIP - 1] = trial(0, Some(1), ONE.saturating_neg());
    read_hand[FLIP + 5] = trial(1, Some(0), ONE);
    read_hand[FLIP + 9] = trial(0, Some(1), ONE);
    assert_eq!(
        first_new(&read_hand, false),
        [Some(FLIP + 9), Some(FLIP + 5)],
        "the selections before the flip are not read"
    );
    // The moves per block: trial t counts under trial t − 1's reward, in trial t's block.
    let mut moves_hand = vec![([0u32; 3], [0i64; 2]); PUNISHED_TRIALS];
    moves_hand[0] = ([9, 9, 9], [99, -99]);
    moves_hand[FLIP] = ([3, 5, 800], [40, -70]);
    moves_hand[FLIP + 6] = ([1, 0, 0], [7, 0]);
    let blocks_moved = moves_blocks(&read_hand, &moves_hand);
    assert_eq!(blocks_moved.len(), PUNISHED_BLOCKS);
    assert_eq!(
        blocks_moved[0],
        [([0; 3], [0; 2]); 2],
        "the first trial consolidates nothing addressed"
    );
    assert_eq!(
        blocks_moved[FLIP_BLOCK],
        [([1, 0, 0], [7, 0]), ([3, 5, 800], [40, -70])],
        "the 1 537th trial under the 1 536th's punishment, the 1 543rd under the 1 542nd's reward"
    );
    assert_eq!(
        moves_totals(&blocks_moved, 1),
        [([0; 3], [0; 2]), ([3, 5, 800], [40, -70])]
    );
    assert_eq!(
        moves_totals(&blocks_moved, 0),
        [([0; 3], [0; 2]), ([1, 0, 0], [7, 0])]
    );
    let mut withheld = read_hand.clone();
    withheld[FLIP - 1].4 = 0;
    assert_eq!(
        moves_blocks(&withheld, &moves_hand)[FLIP_BLOCK][1],
        ([0; 3], [0; 2]),
        "a trial after no reward counts under neither sign"
    );
    // `with_version` (ADR-0095) on a small image: the current version is the image itself,
    // and another differs from it in the header's version and seal and nowhere else.
    let small = Image::encode(
        &Executor::<8>::new(Config {
            units: 2,
            blocks: 1,
            ..Config::default()
        })
        .unwrap(),
    )
    .unwrap();
    assert_eq!(
        with_version(&small, CortexFileHeader::FORMAT_VERSION),
        small
    );
    let older = with_version(&small, 15);
    let apart: Vec<usize> = older
        .iter()
        .zip(small.iter())
        .enumerate()
        .filter(|(_, (a, b))| a != b)
        .map(|(k, _)| k)
        .collect();
    assert!(
        !apart.is_empty()
            && apart
                .iter()
                .all(|&k| (8..12).contains(&k) || (48..56).contains(&k)),
        "only the version and the seal: {apart:?}"
    );
    assert_eq!(
        CortexFileHeader::decode(older[0..64].try_into().unwrap()).version,
        15
    );
    // A few trials over a flip on the instrument's network at 1 024 units, the inhibitory
    // baseline set, the assignment first, flipped before the trial of index `GATE_FLIP`: the
    // signed gate set, and beside it the same network with the gate unset. The oracle is held
    // at every trial inside `earned_run_signed` in both. Up to and including the first
    // punishment the two runs are one run; in the trial after it the punished pair moves
    // against its trace under the gate and stays without it.
    const GATE_FLIP: usize = GATE_TRIALS / 2;
    let p = prior(1024);
    let network = |signed_gate: bool| Config {
        inhibitory_baseline_q16: Some(INHIBITORY_BASELINE_Q16),
        signed_gate,
        ..config(1024, 2, GATE_BASELINE_Q16)
    };
    let mut signed_exec = at_gain(&p, network(true), GAIN_1024);
    assert!(signed_exec.signed_gate());
    let before = weights_of(&signed_exec);
    let (run, signed_moves) = punished_run(
        &mut signed_exec,
        Reversal::AssignmentFirst,
        GATE_TRIALS,
        GATE_FLIP,
        &mut |_, _| {},
    );
    let (blocks, trace, _, read, _) = &run;
    assert!(blocks.is_empty(), "a few trials are no whole block");
    assert_eq!((read.len(), signed_moves.len()), (GATE_TRIALS, GATE_TRIALS));
    let mut plain_exec = at_gain(&p, network(false), GAIN_1024);
    let (plain_run, plain_moves) = earned_run_signed(
        &mut plain_exec,
        Feedback::Answer,
        false,
        1024,
        GATE_TRIALS,
        GATE_BASELINE_Q16,
        Some(GATE_FLIP),
        false,
        &mut |_, _| {},
    );
    let plain = &plain_run.3;
    eprintln!(
        "DUMP punished1024 over a punishment trace {trace:#018x} read {read:?} moves {signed_moves:?} plain {plain:?} plain moves {plain_moves:?}"
    );
    let punished_at = read
        .iter()
        .take(GATE_TRIALS - 1)
        .position(|t| t.4 < 0 && t.2.is_some())
        .expect("a punished selection before the last trial");
    assert_eq!(
        read[..=punished_at],
        plain[..=punished_at],
        "up to the first punishment the two runs are one run"
    );
    let after = punished_at.saturating_add(1);
    let punished_pair = (
        usize::from(read[punished_at].0),
        usize::from(read[punished_at].2.expect("a selection")),
    );
    let (signed_after, plain_after) = (signed_moves[after], plain_moves[after]);
    assert!(
        signed_after.0[0].saturating_add(signed_after.0[1]) > 0,
        "under the gate the punished pair {punished_pair:?} moved in the trial after: {signed_after:?}"
    );
    assert_eq!(
        (plain_after.0[0], plain_after.0[1], plain_after.1),
        (0, 0, [0, 0]),
        "without it the punished pair stayed: {plain_after:?}"
    );
    let transferred = read[after].5[punished_pair.0][punished_pair.1];
    assert_eq!(
        transferred,
        signed_after.1[0].saturating_add(signed_after.1[1]),
        "what the oracle consolidated into the pair is what its synapses moved"
    );
    assert_eq!(plain[after].5[punished_pair.0][punished_pair.1], 0);
    // No excitatory synapse outside the pairs the deliveries addressed moved.
    let addressed: Vec<(usize, usize)> = read
        .iter()
        .take(GATE_TRIALS - 1)
        .filter(|t| t.4 != 0)
        .filter_map(|t| t.2.map(|r| (usize::from(t.0), usize::from(r))))
        .collect();
    let reach = reach_by_polarity(&signed_exec, &before, 1024, &addressed);
    assert_eq!(
        reach.excitatory.1, 0,
        "no excitatory synapse outside the addressed pairs moved: {reach:?}"
    );
    assert!(reach.excitatory.0 > 0, "the addressed pairs moved");
    for (t, r) in read.iter().enumerate() {
        let in_force = t >= GATE_FLIP;
        assert_eq!(
            r.3,
            r.2 == Some(answer_of(r.0, in_force) as u8),
            "trial {t}: correct under the mapping in force"
        );
    }
    let derivation_read = derivation(read);
    assert!(
        derivation_read[0] && derivation_read[1],
        "the signal's course is the rewards' whatever the gate: {derivation_read:?}"
    );
    assert!(
        !derivation_read[2],
        "under the gate a punished pair consolidates, which ADR-0080's third clause excludes"
    );
    // Over the pinned tables: the verdict as written, by the rule committed first; the readings
    // as the constants state; and, block by block, both arms' seventy-two blocks consistent with
    // one another, with the assertion and with the oracle — each coupling after a block the one
    // before it plus what the oracle consolidated into that pair over the block, the excitatory
    // sum the image's plus the four pairs' moves, and what the addressed pairs' synapses moved,
    // trial by trial, what the oracle consolidated.
    let image = QUIET_1024[SETTLED].1;
    let verdict = punished([PUNISHED_BLOCKS_1024[0], PUNISHED_BLOCKS_1024[1]]);
    assert_eq!(verdict, PUNISHED_1024, "the verdict as written");
    assert_eq!(PUNISHED_PREDICTED, None, "and no prediction to hold it to");
    assert_ne!(PUNISHED_IMAGE_CRC_1024, 0);
    for (k, &arm) in PUNISHED_ARMS.iter().enumerate() {
        let blocks = PUNISHED_BLOCKS_1024[k];
        let earned = PUNISHED_EARNED_1024[k];
        let moved = PUNISHED_MOVES_1024[k];
        let first = first_mapping(arm);
        assert_eq!(
            (
                blocks.len(),
                earned.len(),
                moved.len(),
                PUNISHED_COMPOSITIONS_1024[k].len()
            ),
            (
                PUNISHED_BLOCKS,
                PUNISHED_BLOCKS,
                PUNISHED_BLOCKS,
                PUNISHED_BLOCKS
            ),
            "{arm:?}: seventy-two blocks of each"
        );
        assert_ne!(PUNISHED_TRACES_1024[k], 0);
        assert_ne!(PUNISHED_READ_1024[k], 0);
        assert!(!PUNISHED_CENSUS_1024[k].is_empty());
        assert_eq!(
            [correct_before(blocks), revised_count(blocks)],
            CORRECT_PUNISHED_1024[k]
        );
        assert_eq!(
            [
                correct_before(blocks) >= REWARDED_MIN,
                revised_count(blocks) >= REWARDED_MIN
            ],
            [verdict.learned[k], verdict.revised[k]]
        );
        assert_eq!(
            need(blocks, earned),
            NEED_PUNISHED_1024[k],
            "{arm:?}: the need"
        );
        assert_eq!(crossed_block(earned, first), CROSSED_BLOCK_1024[k]);
        assert_eq!(old_falls(blocks, earned, first), OLD_FALLS_1024[k]);
        assert_eq!(fall_slows(blocks, earned, first), FALL_SLOWS_1024[k]);
        assert_eq!(wrong_below(blocks, first), WRONG_BELOW_1024[k]);
        assert_eq!(moves_totals(moved, 1), PUNISHED_MOVES_TOTAL_1024[k]);
        assert_eq!(moves_totals(moved, 0), REWARDED_MOVES_TOTAL_1024[k]);
        assert_eq!(new_rise(blocks, first), NEW_RISE_PUNISHED_1024[k]);
        assert_eq!(falls_every_block(image.0, blocks), FALLS_PUNISHED_1024[k]);
        assert_eq!(
            once_blocks(blocks, PUNISHED_COMPOSITIONS_1024[k]),
            ONCE_BLOCKS_PUNISHED_1024[k]
        );
        assert!(
            punished_held(&REACH_PUNISHED_1024[k]),
            "{arm:?}: the assertion held"
        );
        let last = blocks.last().expect("a block");
        assert_eq!(SUMS_AFTER_PUNISHED_1024[k], (last.7, last.8));
        let mut previous = IMAGE_COUPLINGS_1024;
        for (j, block) in blocks.iter().enumerate() {
            let in_force = first != (j >= FLIP_BLOCK);
            assert_eq!(
                block.1, PUNISHED_BLOCKS_1024[0][j].1,
                "{arm:?} block {j}: the same trials present A in both arms"
            );
            let presented = [block.1, (BLOCK as u32).saturating_sub(block.1)];
            for (s, &n) in presented.iter().enumerate() {
                assert_eq!(
                    earned[j].0[s].iter().sum::<u32>(),
                    n,
                    "{arm:?} block {j}: every presentation of {s} selected or tied"
                );
            }
            assert_eq!(
                block.11,
                earned[j].0[0][2].saturating_add(earned[j].0[1][2]),
                "{arm:?} block {j}: the ties"
            );
            let correct = earned[j].0[0][answer_of(0, in_force)]
                .saturating_add(earned[j].0[1][answer_of(1, in_force)]);
            assert_eq!(
                block.0, correct,
                "{arm:?} block {j}: correct under the mapping in force"
            );
            assert_eq!(
                earned[j].1, correct,
                "{arm:?} block {j}: one reward per correct trial"
            );
            assert!(earned[j].3.abs() <= SIGNAL_END_FIXED_Q16);
            let mut rise = 0i64;
            let mut consolidated = 0i64;
            for &(s, r) in &ALL_PAIRS {
                assert_eq!(
                    block.10[s][r],
                    previous[s][r].saturating_add(earned[j].2[s][r]),
                    "{arm:?} block {j}: {s}→{r} moved by what the oracle consolidated"
                );
                rise =
                    rise.saturating_add(block.10[s][r].saturating_sub(IMAGE_COUPLINGS_1024[s][r]));
                consolidated = consolidated.saturating_add(earned[j].2[s][r]);
            }
            assert_eq!(
                block.8,
                image.1.saturating_add(rise),
                "{arm:?} block {j}: the excitatory sum moved by the four pairs' moves and nothing else"
            );
            let moved_sum = moved[j]
                .iter()
                .flat_map(|m| m.1.iter())
                .fold(0i64, |sum, &a| sum.saturating_add(a));
            assert_eq!(
                moved_sum, consolidated,
                "{arm:?} block {j}: what the addressed synapses moved is what the oracle consolidated"
            );
            previous = block.10;
        }
    }
}

// ----------------------------------------------------------- the measurement (brief 041)

/// The two arms at 1 024 units, in `PUNISHED_ARMS`'s order, each pinned from one run: the
/// sight's blocks and the trace, the composition, the earned blocks and the moves per block,
/// the hash of the readings and the volley's census. Empty until the run: the constants above
/// are committed before the first rewarded run, and the tables after it.
const PUNISHED_BLOCKS_1024: [&[Block]; 2] = [
    &[
        (
            33,
            34,
            [[336, 320], [310, 321]],
            [1728, 1525],
            [2494, 2254],
            [260, 258],
            62,
            161665921,
            218231773,
            41361,
            [[6258441, 6647296], [6559366, 6871154]],
            6,
        ),
        (
            36,
            31,
            [[305, 297], [320, 344]],
            [1574, 1677],
            [2399, 2400],
            [240, 228],
            64,
            157418028,
            218218704,
            108579,
            [[6281801, 6617816], [6510180, 6913391]],
            2,
        ),
        (
            30,
            31,
            [[304, 320], [334, 370]],
            [1570, 1679],
            [2383, 2384],
            [239, 228],
            63,
            153165503,
            218244907,
            -34256,
            [[6327690, 6569834], [6474651, 6977216]],
            5,
        ),
        (
            35,
            28,
            [[281, 297], [337, 430]],
            [1427, 1829],
            [2254, 2569],
            [250, 257],
            64,
            148835411,
            218315295,
            111191,
            [[6333571, 6558795], [6461389, 7066024]],
            5,
        ),
        (
            39,
            30,
            [[314, 312], [316, 410]],
            [1524, 1724],
            [2326, 2391],
            [252, 258],
            62,
            144588124,
            218356776,
            93859,
            [[6383178, 6498939], [6419759, 7159384]],
            3,
        ),
        (
            47,
            36,
            [[416, 333], [283, 369]],
            [1829, 1424],
            [2652, 2114],
            [263, 243],
            64,
            140240329,
            218523729,
            106072,
            [[6508568, 6484266], [6404878, 7230501]],
            3,
        ),
        (
            43,
            30,
            [[362, 304], [357, 452]],
            [1520, 1728],
            [2320, 2438],
            [256, 257],
            64,
            135988811,
            218699739,
            46759,
            [[6607205, 6464099], [6384489, 7348430]],
            5,
        ),
        (
            53,
            32,
            [[458, 322], [310, 456]],
            [1628, 1628],
            [2449, 2369],
            [264, 243],
            64,
            131710736,
            218905376,
            -68005,
            [[6723116, 6460455], [6377151, 7449138]],
            3,
        ),
        (
            54,
            34,
            [[521, 328], [324, 470]],
            [1729, 1524],
            [2551, 2232],
            [261, 232],
            64,
            127446246,
            219173431,
            112153,
            [[6893093, 6446474], [6366661, 7571687]],
            3,
        ),
        (
            61,
            28,
            [[432, 276], [322, 605]],
            [1424, 1831],
            [2297, 2542],
            [251, 225],
            64,
            123238547,
            219434532,
            112227,
            [[7018551, 6428894], [6366400, 7725171]],
            0,
        ),
        (
            63,
            33,
            [[603, 329], [288, 556]],
            [1676, 1578],
            [2478, 2291],
            [236, 241],
            64,
            119046001,
            219689349,
            112227,
            [[7153674, 6428894], [6366295, 7844970]],
            0,
        ),
        (
            64,
            32,
            [[567, 327], [323, 574]],
            [1625, 1629],
            [2480, 2388],
            [259, 250],
            64,
            114753934,
            219904399,
            112227,
            [[7272515, 6428894], [6366295, 7941179]],
            0,
        ),
        (
            63,
            32,
            [[593, 304], [298, 605]],
            [1631, 1627],
            [2445, 2354],
            [255, 221],
            64,
            110755954,
            220140529,
            112227,
            [[7395605, 6428894], [6362961, 8057553]],
            0,
        ),
        (
            62,
            33,
            [[730, 334], [306, 599]],
            [1676, 1579],
            [2526, 2357],
            [270, 246],
            64,
            106701553,
            220373522,
            112227,
            [[7545481, 6428657], [6362961, 8140907]],
            1,
        ),
        (
            63,
            37,
            [[802, 375], [276, 598]],
            [1883, 1375],
            [2689, 2157],
            [258, 264],
            64,
            102545120,
            220603828,
            112227,
            [[7692838, 6428657], [6362011, 8224806]],
            0,
        ),
        (
            62,
            35,
            [[790, 391], [318, 644]],
            [1782, 1472],
            [2566, 2241],
            [249, 259],
            64,
            98661614,
            220868016,
            112227,
            [[7843582, 6428657], [6361667, 8338594]],
            1,
        ),
        (
            63,
            33,
            [[839, 380], [341, 701]],
            [1678, 1572],
            [2528, 2253],
            [252, 268],
            64,
            94831434,
            221146722,
            111555,
            [[7994991, 6428657], [6361667, 8465891]],
            1,
        ),
        (
            64,
            38,
            [[938, 425], [288, 643]],
            [1932, 1322],
            [2735, 2049],
            [265, 266],
            64,
            90964295,
            221412778,
            112227,
            [[8162523, 6428657], [6361667, 8564415]],
            0,
        ),
        (
            64,
            31,
            [[802, 344], [376, 812]],
            [1578, 1680],
            [2419, 2421],
            [257, 279],
            64,
            87326906,
            221636996,
            112227,
            [[8281668, 6428657], [6361667, 8669488]],
            0,
        ),
        (
            64,
            34,
            [[969, 404], [369, 820]],
            [1726, 1524],
            [2487, 2285],
            [293, 281],
            64,
            83752353,
            221911164,
            112227,
            [[8393905, 6428657], [6361667, 8831419]],
            0,
        ),
        (
            64,
            30,
            [[850, 382], [366, 934]],
            [1525, 1730],
            [2316, 2471],
            [292, 241],
            64,
            80111124,
            222131823,
            112227,
            [[8467488, 6428657], [6361667, 8978495]],
            0,
        ),
        (
            64,
            29,
            [[836, 312], [381, 1020]],
            [1471, 1781],
            [2310, 2492],
            [266, 256],
            64,
            76771634,
            222367606,
            112227,
            [[8538607, 6428657], [6361667, 9143159]],
            0,
        ),
        (
            64,
            32,
            [[913, 360], [349, 953]],
            [1627, 1628],
            [2445, 2414],
            [302, 259],
            64,
            73465303,
            222515150,
            112227,
            [[8566400, 6428657], [6361667, 9262910]],
            0,
        ),
        (
            64,
            29,
            [[897, 377], [438, 1068]],
            [1469, 1779],
            [2291, 2505],
            [267, 262],
            64,
            70370084,
            222661390,
            112227,
            [[8621458, 6428657], [6361667, 9354092]],
            0,
        ),
        (
            0,
            29,
            [[867, 328], [427, 1062]],
            [1476, 1781],
            [2347, 2513],
            [286, 263],
            64,
            67282030,
            221932465,
            -112227,
            [[8358336, 6428657], [6361667, 8888289]],
            0,
        ),
        (
            0,
            33,
            [[918, 375], [405, 848]],
            [1676, 1578],
            [2507, 2341],
            [259, 271],
            64,
            64184631,
            221286855,
            -112227,
            [[8016671, 6428657], [6361667, 8584344]],
            0,
        ),
        (
            0,
            36,
            [[854, 393], [324, 741]],
            [1823, 1426],
            [2651, 2179],
            [248, 264],
            64,
            61372725,
            220699234,
            -112227,
            [[7691486, 6428657], [6361667, 8321908]],
            0,
        ),
        (
            2,
            32,
            [[660, 377], [377, 728]],
            [1629, 1625],
            [2489, 2381],
            [263, 268],
            64,
            58594067,
            220284040,
            -112227,
            [[7488701, 6432456], [6361667, 8105700]],
            1,
        ),
        (
            5,
            34,
            [[590, 377], [315, 598]],
            [1730, 1524],
            [2537, 2287],
            [256, 284],
            64,
            55985464,
            219841945,
            18845,
            [[7293272, 6433413], [6361629, 7858115]],
            1,
        ),
        (
            3,
            34,
            [[547, 342], [303, 500]],
            [1726, 1521],
            [2589, 2311],
            [271, 261],
            64,
            53372682,
            219565612,
            -106166,
            [[7164155, 6433207], [6364790, 7707944]],
            3,
        ),
        (
            9,
            35,
            [[551, 366], [316, 465]],
            [1780, 1474],
            [2575, 2202],
            [314, 265],
            64,
            50947332,
            219331572,
            -111247,
            [[7042110, 6461160], [6367272, 7565514]],
            5,
        ),
        (
            8,
            29,
            [[400, 308], [338, 510]],
            [1477, 1774],
            [2322, 2522],
            [268, 229],
            64,
            48569960,
            219047534,
            -54599,
            [[6940212, 6467461], [6374972, 7369373]],
            5,
        ),
        (
            14,
            32,
            [[426, 312], [297, 420]],
            [1626, 1625],
            [2423, 2401],
            [280, 262],
            64,
            46319135,
            218856465,
            -111555,
            [[6839565, 6483123], [6377629, 7260632]],
            4,
        ),
        (
            15,
            28,
            [[380, 305], [377, 489]],
            [1427, 1829],
            [2239, 2565],
            [262, 257],
            64,
            44075304,
            218676117,
            -94518,
            [[6777881, 6488994], [6391789, 7121937]],
            2,
        ),
        (
            21,
            29,
            [[340, 306], [365, 430]],
            [1472, 1781],
            [2314, 2541],
            [235, 277],
            64,
            41956718,
            218554568,
            68099,
            [[6729412, 6516104], [6400998, 7012538]],
            2,
        ),
        (
            14,
            32,
            [[390, 319], [323, 365]],
            [1623, 1627],
            [2433, 2408],
            [264, 286],
            64,
            40012081,
            218443421,
            20865,
            [[6676707, 6522185], [6402522, 6946491]],
            10,
        ),
        (
            17,
            32,
            [[388, 311], [309, 369]],
            [1627, 1622],
            [2464, 2391],
            [245, 239],
            63,
            38192309,
            218361671,
            -57003,
            [[6609539, 6545992], [6413419, 6897205]],
            4,
        ),
        (
            27,
            34,
            [[355, 331], [299, 348]],
            [1726, 1526],
            [2541, 2264],
            [251, 263],
            63,
            36426722,
            218308205,
            25008,
            [[6584743, 6559796], [6425687, 6842463]],
            6,
        ),
        (
            32,
            32,
            [[337, 351], [315, 309]],
            [1628, 1622],
            [2511, 2390],
            [259, 284],
            60,
            34888216,
            218310857,
            90149,
            [[6563465, 6588928], [6434197, 6828751]],
            4,
        ),
        (
            23,
            29,
            [[334, 302], [333, 374]],
            [1475, 1777],
            [2343, 2507],
            [286, 286],
            62,
            33421101,
            218252961,
            68395,
            [[6536120, 6605629], [6452590, 6763106]],
            3,
        ),
        (
            27,
            30,
            [[325, 348], [341, 388]],
            [1524, 1730],
            [2403, 2528],
            [259, 284],
            64,
            32103500,
            218256446,
            -105874,
            [[6509670, 6647859], [6482039, 6721362]],
            6,
        ),
        (
            31,
            30,
            [[331, 326], [370, 370]],
            [1526, 1730],
            [2380, 2498],
            [289, 252],
            64,
            30886405,
            218231697,
            88943,
            [[6475379, 6695303], [6494209, 6671290]],
            2,
        ),
        (
            33,
            32,
            [[335, 388], [348, 330]],
            [1629, 1630],
            [2414, 2394],
            [287, 246],
            64,
            29675086,
            218267260,
            92923,
            [[6440173, 6767555], [6522742, 6641274]],
            5,
        ),
        (
            32,
            31,
            [[324, 387], [331, 332]],
            [1577, 1678],
            [2380, 2421],
            [258, 263],
            64,
            28576553,
            218271246,
            -83860,
            [[6421899, 6793935], [6542416, 6617480]],
            7,
        ),
        (
            39,
            27,
            [[280, 335], [397, 342]],
            [1371, 1879],
            [2251, 2677],
            [287, 236],
            64,
            27606879,
            218390287,
            98877,
            [[6415445, 6884516], [6606219, 6588591]],
            4,
        ),
        (
            47,
            24,
            [[245, 333], [505, 397]],
            [1218, 2030],
            [2091, 2718],
            [265, 237],
            63,
            26716332,
            218516023,
            111707,
            [[6417784, 6933702], [6689529, 6579492]],
            4,
        ),
        (
            37,
            34,
            [[368, 465], [359, 335]],
            [1726, 1524],
            [2598, 2294],
            [278, 286],
            63,
            25924968,
            218639574,
            78857,
            [[6407446, 7060397], [6712731, 6563484]],
            4,
        ),
        (
            44,
            33,
            [[346, 450], [373, 327]],
            [1679, 1577],
            [2541, 2355],
            [274, 257],
            64,
            25170764,
            218780747,
            -27153,
            [[6409238, 7175632], [6748301, 6552060]],
            5,
        ),
        (
            49,
            31,
            [[357, 501], [431, 304]],
            [1576, 1676],
            [2439, 2431],
            [292, 298],
            63,
            24489119,
            218918807,
            111031,
            [[6397836, 7261326], [6816597, 6547532]],
            9,
        ),
        (
            54,
            37,
            [[375, 582], [376, 251]],
            [1877, 1370],
            [2743, 2170],
            [264, 277],
            64,
            23799087,
            219087633,
            63075,
            [[6394653, 7397256], [6854100, 6546108]],
            3,
        ),
        (
            54,
            35,
            [[380, 595], [400, 295]],
            [1771, 1476],
            [2619, 2276],
            [269, 258],
            64,
            23232996,
            219317233,
            111967,
            [[6388936, 7565344], [6930336, 6537101]],
            1,
        ),
        (
            52,
            30,
            [[325, 505], [512, 373]],
            [1527, 1733],
            [2383, 2524],
            [268, 268],
            64,
            22605900,
            219501320,
            112153,
            [[6389075, 7674832], [7031377, 6510520]],
            2,
        ),
        (
            60,
            37,
            [[367, 640], [407, 257]],
            [1883, 1371],
            [2720, 2174],
            [272, 275],
            63,
            22069913,
            219794096,
            112227,
            [[6387022, 7903026], [7102035, 6506497]],
            1,
        ),
        (
            53,
            20,
            [[238, 456], [640, 464]],
            [1015, 2240],
            [1927, 2958],
            [281, 271],
            64,
            21646927,
            219988714,
            112153,
            [[6386305, 8010744], [7220478, 6475671]],
            2,
        ),
        (
            60,
            34,
            [[341, 721], [504, 310]],
            [1728, 1517],
            [2551, 2324],
            [262, 266],
            64,
            21248655,
            220258955,
            63075,
            [[6386305, 8226317], [7275857, 6474960]],
            3,
        ),
        (
            57,
            30,
            [[349, 755], [554, 373]],
            [1523, 1729],
            [2371, 2493],
            [249, 277],
            64,
            20795360,
            220461260,
            -18845,
            [[6386305, 8376537], [7349546, 6453356]],
            2,
        ),
        (
            64,
            25,
            [[286, 666], [685, 393]],
            [1274, 1982],
            [2194, 2729],
            [276, 273],
            64,
            20471711,
            220744978,
            112227,
            [[6386305, 8490488], [7519880, 6452789]],
            0,
        ),
        (
            62,
            26,
            [[299, 731], [680, 364]],
            [1318, 1933],
            [2181, 2690],
            [289, 282],
            64,
            20142291,
            220995092,
            112227,
            [[6386305, 8623114], [7641862, 6448295]],
            0,
        ),
        (
            63,
            35,
            [[354, 970], [589, 323]],
            [1779, 1476],
            [2669, 2268],
            [302, 262],
            64,
            19884104,
            221284834,
            112219,
            [[6386305, 8814511], [7740207, 6448295]],
            1,
        ),
        (
            64,
            35,
            [[335, 979], [616, 317]],
            [1782, 1477],
            [2645, 2246],
            [247, 286],
            64,
            19617681,
            221506279,
            112227,
            [[6386305, 8965138], [7811025, 6448295]],
            0,
        ),
        (
            64,
            31,
            [[339, 955], [677, 396]],
            [1575, 1677],
            [2428, 2462],
            [251, 257],
            64,
            19416114,
            221730478,
            112227,
            [[6386305, 9087470], [7912892, 6448295]],
            0,
        ),
        (
            63,
            35,
            [[387, 1021], [688, 335]],
            [1777, 1472],
            [2662, 2244],
            [288, 247],
            64,
            19239347,
            221930012,
            106166,
            [[6386305, 9200487], [8003074, 6444630]],
            0,
        ),
        (
            64,
            32,
            [[386, 1034], [807, 363]],
            [1626, 1625],
            [2500, 2375],
            [293, 275],
            64,
            19045697,
            222185582,
            112227,
            [[6386305, 9320272], [8138859, 6444630]],
            0,
        ),
        (
            64,
            34,
            [[389, 1059], [771, 346]],
            [1731, 1527],
            [2519, 2311],
            [316, 243],
            64,
            18917185,
            222420740,
            112227,
            [[6386305, 9388520], [8305769, 6444630]],
            0,
        ),
        (
            64,
            30,
            [[331, 1047], [858, 360]],
            [1525, 1728],
            [2397, 2505],
            [274, 284],
            64,
            18681247,
            222737007,
            112227,
            [[6386305, 9518046], [8492510, 6444630]],
            0,
        ),
        (
            64,
            36,
            [[392, 1234], [778, 324]],
            [1831, 1421],
            [2660, 2266],
            [282, 309],
            64,
            18602842,
            222938121,
            112227,
            [[6386305, 9649726], [8561944, 6444630]],
            0,
        ),
        (
            64,
            27,
            [[321, 984], [1020, 368]],
            [1372, 1885],
            [2271, 2699],
            [309, 267],
            64,
            18512000,
            223189090,
            112227,
            [[6386305, 9754546], [8708093, 6444630]],
            0,
        ),
        (
            64,
            34,
            [[366, 1252], [879, 351]],
            [1727, 1525],
            [2546, 2297],
            [262, 278],
            64,
            18284067,
            223437500,
            112227,
            [[6386305, 9904087], [8806962, 6444630]],
            0,
        ),
        (
            64,
            34,
            [[389, 1226], [992, 375]],
            [1727, 1527],
            [2574, 2292],
            [281, 301],
            64,
            18254210,
            223706857,
            112227,
            [[6386305, 10052512], [8927894, 6444630]],
            0,
        ),
        (
            64,
            30,
            [[367, 1229], [1136, 413]],
            [1528, 1730],
            [2387, 2480],
            [314, 298],
            64,
            18203634,
            224042256,
            112227,
            [[6386305, 10222280], [9093525, 6444630]],
            0,
        ),
        (
            64,
            29,
            [[333, 1204], [1121, 453]],
            [1472, 1777],
            [2408, 2610],
            [306, 276],
            64,
            18186620,
            224280500,
            112227,
            [[6386305, 10291651], [9262398, 6444630]],
            0,
        ),
        (
            64,
            31,
            [[372, 1289], [1173, 409]],
            [1573, 1673],
            [2449, 2490],
            [281, 277],
            64,
            18207853,
            224498537,
            112227,
            [[6386305, 10381931], [9390155, 6444630]],
            0,
        ),
    ],
    &[
        (
            28,
            34,
            [[327, 325], [320, 310]],
            [1728, 1525],
            [2495, 2252],
            [258, 257],
            62,
            161665379,
            218273594,
            -40571,
            [[6243051, 6745455], [6612268, 6777304]],
            7,
        ),
        (
            34,
            31,
            [[279, 331], [346, 323]],
            [1574, 1677],
            [2398, 2399],
            [240, 231],
            64,
            157383354,
            218361295,
            -109741,
            [[6231944, 6820846], [6671738, 6741251]],
            7,
        ),
        (
            37,
            31,
            [[278, 352], [363, 331]],
            [1570, 1679],
            [2383, 2380],
            [237, 230],
            64,
            153070740,
            218459113,
            34260,
            [[6200329, 6895479], [6737534, 6730255]],
            8,
        ),
        (
            43,
            28,
            [[244, 377], [401, 359]],
            [1427, 1829],
            [2254, 2564],
            [246, 249],
            64,
            148621270,
            218545631,
            34884,
            [[6197574, 6956021], [6806828, 6689692]],
            4,
        ),
        (
            45,
            30,
            [[267, 416], [374, 336]],
            [1524, 1724],
            [2331, 2391],
            [250, 252],
            63,
            144258824,
            218771146,
            -81964,
            [[6191420, 7111571], [6908430, 6664209]],
            4,
        ),
        (
            48,
            36,
            [[323, 484], [369, 273]],
            [1828, 1424],
            [2645, 2113],
            [263, 243],
            64,
            139747750,
            218977120,
            110197,
            [[6160128, 7248819], [7015027, 6657630]],
            3,
        ),
        (
            57,
            30,
            [[261, 465], [480, 331]],
            [1520, 1728],
            [2320, 2429],
            [251, 265],
            63,
            135374669,
            219172477,
            109471,
            [[6148748, 7351993], [7125502, 6650718]],
            3,
        ),
        (
            54,
            32,
            [[292, 514], [477, 323]],
            [1629, 1628],
            [2441, 2367],
            [262, 240],
            64,
            130907943,
            219436596,
            94752,
            [[6145768, 7496265], [7254276, 6644771]],
            0,
        ),
        (
            55,
            34,
            [[333, 556], [490, 306]],
            [1729, 1524],
            [2544, 2231],
            [262, 237],
            64,
            126373670,
            219797305,
            94752,
            [[6144490, 7735798], [7381237, 6640264]],
            2,
        ),
        (
            57,
            28,
            [[239, 531], [534, 388]],
            [1424, 1831],
            [2292, 2537],
            [248, 232],
            64,
            121892520,
            220089418,
            112227,
            [[6146522, 7898373], [7511669, 6637338]],
            0,
        ),
        (
            59,
            33,
            [[336, 698], [481, 312]],
            [1674, 1578],
            [2466, 2280],
            [231, 245],
            64,
            117421777,
            220322784,
            112227,
            [[6136682, 8055042], [7599631, 6635913]],
            2,
        ),
        (
            60,
            32,
            [[277, 679], [556, 333]],
            [1625, 1629],
            [2477, 2387],
            [250, 249],
            64,
            112850384,
            220574364,
            112203,
            [[6136682, 8251918], [7657470, 6632778]],
            2,
        ),
        (
            63,
            32,
            [[284, 726], [564, 325]],
            [1631, 1627],
            [2442, 2348],
            [255, 221],
            64,
            108572356,
            220817122,
            112003,
            [[6136682, 8413468], [7742009, 6629447]],
            0,
        ),
        (
            63,
            33,
            [[337, 797], [525, 300]],
            [1676, 1579],
            [2515, 2344],
            [260, 259],
            64,
            104200228,
            220994867,
            112227,
            [[6136682, 8536284], [7798214, 6628171]],
            0,
        ),
        (
            62,
            37,
            [[356, 932], [531, 307]],
            [1883, 1375],
            [2687, 2151],
            [259, 270],
            64,
            99681697,
            221233480,
            112227,
            [[6136682, 8712773], [7860338, 6628171]],
            2,
        ),
        (
            64,
            35,
            [[318, 939], [586, 320]],
            [1782, 1472],
            [2562, 2246],
            [249, 271],
            64,
            95599858,
            221488826,
            112227,
            [[6136682, 8875278], [7953179, 6628171]],
            0,
        ),
        (
            64,
            33,
            [[357, 955], [617, 293]],
            [1678, 1571],
            [2520, 2251],
            [252, 261],
            64,
            91646337,
            221716877,
            112227,
            [[6136682, 9024690], [8031818, 6628171]],
            0,
        ),
        (
            63,
            38,
            [[382, 1081], [563, 296]],
            [1932, 1322],
            [2725, 2051],
            [262, 259],
            64,
            87715953,
            221930125,
            110207,
            [[6136682, 9158499], [8112250, 6627178]],
            0,
        ),
        (
            64,
            31,
            [[307, 945], [740, 363]],
            [1578, 1680],
            [2408, 2417],
            [243, 291],
            64,
            84092565,
            222173239,
            112227,
            [[6136682, 9255564], [8258299, 6627178]],
            0,
        ),
        (
            64,
            34,
            [[369, 1064], [703, 362]],
            [1726, 1524],
            [2485, 2302],
            [286, 286],
            64,
            80647755,
            222425314,
            112227,
            [[6136682, 9371265], [8394673, 6627178]],
            0,
        ),
        (
            64,
            30,
            [[309, 986], [781, 382]],
            [1523, 1730],
            [2294, 2472],
            [280, 238],
            64,
            77148519,
            222686358,
            112227,
            [[6136682, 9463917], [8563065, 6627178]],
            0,
        ),
        (
            64,
            29,
            [[269, 930], [881, 399]],
            [1471, 1781],
            [2301, 2497],
            [257, 256],
            64,
            73949475,
            222877628,
            112227,
            [[6136682, 9512149], [8706103, 6627178]],
            0,
        ),
        (
            64,
            32,
            [[323, 1055], [869, 355]],
            [1627, 1628],
            [2452, 2426],
            [301, 252],
            64,
            70822053,
            223148132,
            112227,
            [[6136682, 9594755], [8894001, 6627178]],
            0,
        ),
        (
            64,
            29,
            [[318, 992], [1055, 388]],
            [1470, 1779],
            [2274, 2511],
            [278, 268],
            64,
            67880662,
            223333344,
            112227,
            [[6136682, 9650801], [9023167, 6627178]],
            0,
        ),
        (
            0,
            29,
            [[298, 967], [964, 418]],
            [1476, 1781],
            [2340, 2516],
            [291, 269],
            64,
            64897252,
            222589341,
            -112227,
            [[6136682, 9342640], [8587325, 6627178]],
            0,
        ),
        (
            0,
            33,
            [[356, 1021], [812, 352]],
            [1676, 1579],
            [2514, 2351],
            [263, 279],
            64,
            62001443,
            221964082,
            -112227,
            [[6136682, 8987256], [8317450, 6627178]],
            0,
        ),
        (
            0,
            36,
            [[361, 901], [662, 318]],
            [1824, 1426],
            [2665, 2166],
            [251, 274],
            64,
            59355450,
            221331204,
            -112227,
            [[6136682, 8578063], [8093765, 6627178]],
            0,
        ),
        (
            5,
            32,
            [[303, 766], [694, 367]],
            [1629, 1624],
            [2477, 2377],
            [259, 266],
            64,
            56774992,
            220802327,
            -110155,
            [[6136287, 8286211], [7855507, 6628806]],
            0,
        ),
        (
            2,
            34,
            [[312, 705], [549, 293]],
            [1730, 1524],
            [2543, 2284],
            [252, 293],
            64,
            54295705,
            220427331,
            -63075,
            [[6136434, 8102925], [7663650, 6628806]],
            0,
        ),
        (
            1,
            34,
            [[320, 590], [506, 302]],
            [1726, 1521],
            [2595, 2311],
            [275, 271],
            64,
            51855693,
            220203154,
            -63075,
            [[6136434, 7938314], [7604071, 6628819]],
            4,
        ),
        (
            4,
            35,
            [[338, 570], [519, 315]],
            [1780, 1474],
            [2575, 2193],
            [314, 273],
            64,
            49563821,
            219943428,
            -110207,
            [[6137172, 7774832], [7505408, 6630500]],
            4,
        ),
        (
            14,
            29,
            [[270, 476], [509, 377]],
            [1476, 1774],
            [2320, 2530],
            [264, 230],
            64,
            47330690,
            219821176,
            -92722,
            [[6150452, 7674740], [7444020, 6656448]],
            2,
        ),
        (
            14,
            32,
            [[313, 465], [462, 336]],
            [1626, 1627],
            [2440, 2407],
            [283, 267],
            64,
            45202130,
            219661034,
            24997,
            [[6166875, 7558658], [7375722, 6664263]],
            4,
        ),
        (
            15,
            28,
            [[291, 427], [489, 414]],
            [1427, 1829],
            [2249, 2572],
            [260, 262],
            64,
            43168908,
            219597977,
            -106112,
            [[6170312, 7529702], [7288211, 6714236]],
            7,
        ),
        (
            9,
            29,
            [[271, 395], [501, 360]],
            [1471, 1781],
            [2312, 2545],
            [229, 276],
            64,
            41129066,
            219434486,
            -109937,
            [[6177599, 7450819], [7177158, 6733394]],
            6,
        ),
        (
            14,
            32,
            [[305, 392], [431, 318]],
            [1622, 1627],
            [2429, 2407],
            [257, 286],
            64,
            39311371,
            219393631,
            -89873,
            [[6198792, 7417442], [7131475, 6750406]],
            1,
        ),
        (
            23,
            32,
            [[335, 369], [395, 357]],
            [1627, 1622],
            [2472, 2389],
            [244, 242],
            64,
            37602501,
            219325373,
            18961,
            [[6221207, 7366161], [7064027, 6778462]],
            4,
        ),
        (
            20,
            34,
            [[316, 386], [373, 334]],
            [1726, 1526],
            [2539, 2276],
            [253, 276],
            64,
            35940434,
            219283732,
            -49579,
            [[6227710, 7354399], [7004156, 6801951]],
            3,
        ),
        (
            21,
            32,
            [[342, 399], [387, 331]],
            [1628, 1622],
            [2503, 2392],
            [258, 289],
            62,
            34470608,
            219242850,
            -90233,
            [[6235578, 7326076], [6964418, 6821262]],
            8,
        ),
        (
            28,
            29,
            [[318, 341], [381, 418]],
            [1475, 1777],
            [2348, 2507],
            [283, 285],
            62,
            33076382,
            219299486,
            -106119,
            [[6242763, 7313863], [6936103, 6911241]],
            6,
        ),
        (
            28,
            30,
            [[323, 367], [391, 428]],
            [1524, 1730],
            [2404, 2527],
            [265, 279],
            64,
            31827639,
            219321802,
            -25839,
            [[6258956, 7278083], [6908302, 6980945]],
            8,
        ),
        (
            26,
            30,
            [[334, 341], [423, 432]],
            [1526, 1730],
            [2380, 2505],
            [298, 259],
            64,
            30671314,
            219383884,
            -101309,
            [[6298315, 7262515], [6888115, 7039423]],
            6,
        ),
        (
            27,
            32,
            [[345, 361], [383, 385]],
            [1629, 1630],
            [2409, 2396],
            [290, 249],
            64,
            29487604,
            219436937,
            -110467,
            [[6346883, 7236696], [6849309, 7108533]],
            5,
        ),
        (
            33,
            31,
            [[357, 352], [390, 426]],
            [1577, 1677],
            [2390, 2421],
            [258, 275],
            64,
            28455119,
            219532814,
            -44885,
            [[6381648, 7214854], [6843003, 7197793]],
            6,
        ),
        (
            31,
            27,
            [[312, 302], [421, 475]],
            [1371, 1879],
            [2260, 2680],
            [288, 243],
            64,
            27529161,
            219605864,
            -98259,
            [[6416348, 7174750], [6824751, 7294499]],
            9,
        ),
        (
            34,
            24,
            [[291, 283], [496, 546]],
            [1218, 2030],
            [2086, 2721],
            [276, 242],
            64,
            26688028,
            219691820,
            34808,
            [[6433102, 7173647], [6804665, 7384890]],
            5,
        ),
        (
            41,
            34,
            [[432, 382], [386, 512]],
            [1726, 1525],
            [2592, 2294],
            [285, 288],
            64,
            25966487,
            219840038,
            -19531,
            [[6495513, 7153418], [6810680, 7484911]],
            7,
        ),
        (
            49,
            33,
            [[466, 371], [377, 509]],
            [1679, 1577],
            [2552, 2360],
            [273, 260],
            64,
            25268059,
            220016885,
            112153,
            [[6609333, 7137722], [6795705, 7578609]],
            4,
        ),
        (
            47,
            31,
            [[474, 391], [402, 511]],
            [1576, 1676],
            [2449, 2430],
            [296, 305],
            63,
            24690657,
            220184223,
            62999,
            [[6681476, 7138607], [6789179, 7679445]],
            5,
        ),
        (
            48,
            37,
            [[511, 420], [340, 495]],
            [1877, 1369],
            [2738, 2164],
            [276, 286],
            63,
            24010893,
            220306571,
            -19073,
            [[6768345, 7135069], [6776236, 7731405]],
            3,
        ),
        (
            52,
            35,
            [[543, 406], [355, 538]],
            [1773, 1476],
            [2627, 2266],
            [265, 244],
            64,
            23444394,
            220438435,
            111465,
            [[6826121, 7125778], [6774298, 7816722]],
            4,
        ),
        (
            56,
            30,
            [[477, 337], [446, 690]],
            [1527, 1733],
            [2393, 2533],
            [263, 259],
            64,
            22912519,
            220725594,
            112203,
            [[6909540, 7114529], [6771976, 8034033]],
            2,
        ),
        (
            58,
            37,
            [[641, 404], [348, 603]],
            [1883, 1371],
            [2724, 2185],
            [281, 276],
            64,
            22377019,
            221057990,
            94727,
            [[7107886, 7115669], [6771976, 8166943]],
            3,
        ),
        (
            63,
            20,
            [[400, 257], [512, 1006]],
            [1014, 2239],
            [1922, 2955],
            [284, 272],
            64,
            21975016,
            221379938,
            112227,
            [[7126123, 7115669], [6771976, 8470654]],
            1,
        ),
        (
            63,
            34,
            [[626, 350], [405, 726]],
            [1728, 1517],
            [2562, 2325],
            [270, 267],
            64,
            21584692,
            221660334,
            110207,
            [[7266324, 7115669], [6771976, 8610849]],
            1,
        ),
        (
            61,
            30,
            [[643, 381], [413, 864]],
            [1523, 1729],
            [2401, 2490],
            [247, 279],
            64,
            21200132,
            221964398,
            112003,
            [[7392390, 7115881], [6771976, 8788635]],
            0,
        ),
        (
            64,
            25,
            [[560, 326], [501, 1002]],
            [1274, 1982],
            [2204, 2736],
            [287, 270],
            64,
            20908248,
            222187748,
            112227,
            [[7497070, 7115881], [6771976, 8907305]],
            0,
        ),
        (
            63,
            26,
            [[608, 334], [494, 990]],
            [1318, 1932],
            [2191, 2672],
            [293, 278],
            64,
            20551699,
            222399387,
            112227,
            [[7594560, 7116279], [6771976, 9021056]],
            0,
        ),
        (
            63,
            35,
            [[812, 436], [392, 838]],
            [1779, 1474],
            [2692, 2264],
            [314, 269],
            64,
            20294787,
            222664397,
            112003,
            [[7724588, 7116279], [6771976, 9156038]],
            1,
        ),
        (
            64,
            35,
            [[858, 430], [429, 855]],
            [1781, 1477],
            [2669, 2230],
            [254, 301],
            64,
            20017886,
            222859343,
            112227,
            [[7846762, 7116279], [6771976, 9228810]],
            0,
        ),
        (
            64,
            31,
            [[807, 389], [426, 1040]],
            [1575, 1678],
            [2434, 2455],
            [257, 267],
            64,
            19801044,
            223058153,
            112227,
            [[7927634, 7116279], [6771976, 9346748]],
            0,
        ),
        (
            64,
            35,
            [[931, 384], [426, 949]],
            [1777, 1472],
            [2677, 2246],
            [282, 248],
            64,
            19672687,
            223321384,
            112227,
            [[8070166, 7116279], [6771976, 9467447]],
            0,
        ),
        (
            64,
            32,
            [[927, 411], [487, 1034]],
            [1626, 1626],
            [2507, 2378],
            [306, 272],
            64,
            19471220,
            223521612,
            112227,
            [[8177699, 7116279], [6771976, 9560142]],
            0,
        ),
        (
            64,
            34,
            [[978, 431], [456, 1008]],
            [1731, 1526],
            [2530, 2322],
            [319, 257],
            64,
            19428396,
            223715170,
            112227,
            [[8251634, 7116279], [6771976, 9679765]],
            0,
        ),
        (
            64,
            30,
            [[907, 376], [478, 1146]],
            [1526, 1728],
            [2409, 2515],
            [275, 281],
            64,
            19269193,
            223938348,
            112227,
            [[8339012, 7116279], [6771976, 9815565]],
            0,
        ),
        (
            64,
            36,
            [[1065, 449], [398, 1029]],
            [1830, 1421],
            [2653, 2262],
            [292, 326],
            64,
            19193467,
            224088278,
            112227,
            [[8438169, 7116279], [6771976, 9866338]],
            0,
        ),
        (
            64,
            27,
            [[873, 369], [512, 1295]],
            [1372, 1886],
            [2277, 2695],
            [316, 294],
            64,
            19113505,
            224216656,
            112227,
            [[8459494, 7116279], [6771976, 9973391]],
            0,
        ),
        (
            64,
            34,
            [[1085, 464], [480, 1170]],
            [1727, 1525],
            [2552, 2308],
            [266, 283],
            64,
            18973695,
            224368680,
            112227,
            [[8505134, 7116279], [6771976, 10079775]],
            0,
        ),
        (
            64,
            34,
            [[1060, 403], [506, 1212]],
            [1727, 1527],
            [2569, 2299],
            [281, 303],
            64,
            18862281,
            224575800,
            112227,
            [[8608938, 7116279], [6771976, 10183091]],
            0,
        ),
        (
            64,
            30,
            [[1008, 424], [563, 1443]],
            [1528, 1729],
            [2386, 2471],
            [323, 297],
            64,
            18877999,
            224736748,
            112227,
            [[8652355, 7116279], [6771976, 10300622]],
            0,
        ),
        (
            64,
            29,
            [[978, 402], [555, 1478]],
            [1471, 1776],
            [2423, 2609],
            [313, 285],
            64,
            18908800,
            224937258,
            112227,
            [[8712846, 7116279], [6771976, 10440641]],
            0,
        ),
        (
            64,
            31,
            [[1064, 435], [531, 1423]],
            [1573, 1675],
            [2469, 2476],
            [268, 274],
            64,
            19024488,
            225117757,
            112227,
            [[8811410, 7116279], [6771976, 10522576]],
            0,
        ),
    ],
];
const PUNISHED_TRACES_1024: [u64; 2] = [0x987e1bcfabe50662, 0x7767c0e1a4be0c88];
const PUNISHED_COMPOSITIONS_1024: [&[Composition]; 2] = [
    &[
        (
            [[5083, 1452], [1048, 6274]],
            [
                [
                    [218960, -5422, 328102, -512755],
                    [233835, -5554, 342758, -494101],
                ],
                [
                    [221847, -5343, 279982, -466430],
                    [210428, -4716, 329476, -446848],
                ],
            ],
            [[1100, 1649], [1009, 1646], [3350, 0]],
            54,
            6816454107154483347,
        ),
        (
            [[-1170, 3461], [-3983, -6281]],
            [
                [
                    [197921, -5091, 317554, -490057],
                    [231529, -6088, 317984, -445139],
                ],
                [
                    [227041, -5382, 292204, -456713],
                    [231719, -5168, 320416, -491606],
                ],
            ],
            [[983, 1585], [1009, 1615], [3355, 0]],
            46,
            15360126570716355963,
        ),
        (
            [[-8482, 3883], [8884, 6344]],
            [
                [
                    [214712, -5396, 330259, -477357],
                    [218512, -4477, 324836, -452128],
                ],
                [
                    [212031, -5002, 285629, -440911],
                    [262507, -7279, 372500, -502954],
                ],
            ],
            [[969, 1611], [933, 1636], [3355, 1]],
            51,
            12729296111150967286,
        ),
        (
            [[-3271, -182], [2107, 457]],
            [
                [
                    [187927, -8359, 327924, -487689],
                    [187609, -4973, 291099, -473054],
                ],
                [
                    [255252, -4919, 310546, -486196],
                    [297972, -6611, 380722, -561402],
                ],
            ],
            [[1050, 1682], [1080, 1699], [3369, 1]],
            45,
            10905835993431037488,
        ),
        (
            [[3408, -1930], [13221, 4983]],
            [
                [
                    [221902, -6466, 329762, -482048],
                    [224264, -8380, 313292, -445067],
                ],
                [
                    [221127, -4294, 291020, -441237],
                    [307983, -5438, 371927, -478642],
                ],
            ],
            [[1053, 1639], [956, 1703], [3353, 0]],
            55,
            3905556344168364631,
        ),
        (
            [[755, 1906], [-1284, -862]],
            [
                [
                    [307742, -5581, 371121, -511079],
                    [234886, -5780, 339300, -493078],
                ],
                [
                    [181173, -3059, 253266, -384958],
                    [246819, -3575, 363352, -484534],
                ],
            ],
            [[948, 1759], [992, 1721], [3362, 0]],
            55,
            3807414228862553332,
        ),
        (
            [[6716, 2535], [-7841, 4964]],
            [
                [
                    [253326, -7388, 383920, -526335],
                    [210051, -4024, 279518, -459130],
                ],
                [
                    [231605, -3314, 298972, -477144],
                    [314741, -9694, 435169, -561836],
                ],
            ],
            [[1034, 1649], [984, 1738], [3347, 0]],
            49,
            8323201345631688737,
        ),
        (
            [[-4648, 5562], [7197, 3795]],
            [
                [
                    [305263, -7243, 403147, -548015],
                    [223539, -6612, 339214, -497220],
                ],
                [
                    [210925, -5995, 300593, -468332],
                    [307937, -8591, 442167, -563004],
                ],
            ],
            [[1059, 1775], [1032, 1812], [3350, 0]],
            60,
            7567570620453981585,
        ),
        (
            [[7303, 8779], [-8907, 8858]],
            [
                [
                    [364117, -7662, 447163, -543147],
                    [258609, -7593, 355996, -500917],
                ],
                [
                    [201695, -4155, 270186, -446270],
                    [311864, -6374, 451348, -515243],
                ],
            ],
            [[1028, 1874], [1019, 1870], [3349, 0]],
            58,
            15823680934957176910,
        ),
        (
            [[5405, 4649], [1424, 10290]],
            [
                [
                    [303916, -4321, 442730, -600990],
                    [183001, -4076, 322241, -437440],
                ],
                [
                    [207770, -4247, 286909, -490193],
                    [385652, -10682, 494189, -597691],
                ],
            ],
            [[1017, 1737], [1015, 1856], [3361, 0]],
            57,
            14882129310425703360,
        ),
        (
            [[220, 8887], [6153, 14002]],
            [
                [
                    [368756, -11229, 509786, -685655],
                    [226602, -3445, 343860, -493826],
                ],
                [
                    [194846, -4128, 254972, -425902],
                    [357200, -7875, 453709, -543816],
                ],
            ],
            [[978, 1924], [951, 1966], [3351, 0]],
            62,
            1830675931677302823,
        ),
        (
            [[4004, 1490], [-5928, 14515]],
            [
                [
                    [379642, -12435, 514052, -688467],
                    [220815, -8065, 337420, -471736],
                ],
                [
                    [205663, -5675, 302255, -469385],
                    [361544, -9879, 546225, -597927],
                ],
            ],
            [[1046, 1996], [1019, 1995], [3364, 1]],
            62,
            18130019961328043724,
        ),
        (
            [[1476, -2994], [9503, 18997]],
            [
                [
                    [391791, -14071, 547478, -704889],
                    [216332, -5507, 327286, -494108],
                ],
                [
                    [196238, -4452, 299083, -443960],
                    [392171, -12000, 490413, -611784],
                ],
            ],
            [[1054, 1945], [1006, 1980], [3365, 0]],
            64,
            6839063845750508798,
        ),
        (
            [[22677, 412], [480, 12017]],
            [
                [
                    [451083, -17604, 593841, -684895],
                    [230660, -6108, 346904, -512117],
                ],
                [
                    [195374, -4590, 295199, -488777],
                    [409577, -15261, 535590, -626213],
                ],
            ],
            [[1072, 2147], [1009, 2102], [3365, 0]],
            64,
            12561841272124388342,
        ),
        (
            [[24329, 837], [4869, 20068]],
            [
                [
                    [524143, -17932, 619058, -787917],
                    [257358, -6088, 350093, -530447],
                ],
                [
                    [166352, -4233, 284439, -458072],
                    [359859, -13284, 553195, -600812],
                ],
            ],
            [[1050, 2204], [1057, 2021], [3355, 0]],
            61,
            15306690596846010456,
        ),
        (
            [[24066, 12703], [1389, 24120]],
            [
                [
                    [521480, -16083, 599575, -701704],
                    [262361, -4572, 349664, -506283],
                ],
                [
                    [190946, -5021, 289671, -441371],
                    [386792, -9704, 562660, -632182],
                ],
            ],
            [[1035, 2289], [1019, 2174], [3361, 1]],
            64,
            16386491623638930350,
        ),
        (
            [[19725, -71], [7421, 21844]],
            [
                [
                    [495304, -18221, 647266, -798393],
                    [244039, -6414, 352162, -519262],
                ],
                [
                    [190259, -3771, 286192, -450826],
                    [432796, -8921, 598445, -691424],
                ],
            ],
            [[1013, 2298], [1059, 2236], [3362, 0]],
            62,
            12850733033310318795,
        ),
        (
            [[3842, 6401], [426, 27727]],
            [
                [
                    [622162, -18440, 688789, -836063],
                    [294188, -9708, 357745, -530848],
                ],
                [
                    [174723, -4774, 264752, -437287],
                    [378016, -13856, 567930, -647927],
                ],
            ],
            [[1100, 2435], [1068, 2233], [3358, 0]],
            64,
            3760025102760018832,
        ),
        (
            [[19515, 8611], [9162, 20540]],
            [
                [
                    [506873, -18255, 605974, -742982],
                    [237777, -8109, 349257, -498217],
                ],
                [
                    [230769, -5116, 304794, -485645],
                    [499693, -17154, 665075, -764366],
                ],
            ],
            [[1045, 2347], [1070, 2270], [3350, 0]],
            63,
            8703868435888851415,
        ),
        (
            [[30795, 9068], [5654, 25310]],
            [
                [
                    [573699, -19014, 655442, -787007],
                    [272231, -7132, 342022, -523971],
                ],
                [
                    [210594, -5695, 316793, -478445],
                    [503300, -18395, 656501, -739361],
                ],
            ],
            [[1131, 2512], [1096, 2378], [3339, 0]],
            64,
            16031717036734567714,
        ),
        (
            [[25191, 14663], [8597, 22942]],
            [
                [
                    [518640, -15507, 651129, -783211],
                    [244415, -5023, 348131, -473804],
                ],
                [
                    [223637, -7328, 314321, -507042],
                    [584223, -20030, 709579, -837911],
                ],
            ],
            [[1100, 2343], [1035, 2541], [3373, 0]],
            62,
            445292498348548180,
        ),
        (
            [[23484, 5244], [4120, 28504]],
            [
                [
                    [482840, -23319, 711719, -807377],
                    [208318, -6193, 351867, -495218],
                ],
                [
                    [259210, -5777, 305839, -521928],
                    [661614, -13736, 692023, -820434],
                ],
            ],
            [[1047, 2453], [1023, 2649], [3330, 0]],
            63,
            9025449375175948578,
        ),
        (
            [[20308, 9249], [646, 22533]],
            [
                [
                    [535164, -18041, 705090, -920176],
                    [240536, -7329, 351579, -542669],
                ],
                [
                    [228470, -6968, 340513, -486160],
                    [576700, -20480, 702318, -812048],
                ],
            ],
            [[1171, 2468], [1090, 2566], [3373, 0]],
            63,
            15851379683308712809,
        ),
        (
            [[25788, -1050], [2665, 23794]],
            [
                [
                    [523085, -14261, 674759, -830106],
                    [242991, -5255, 348217, -513494],
                ],
                [
                    [245838, -4992, 321918, -514582],
                    [661311, -19056, 784966, -973490],
                ],
            ],
            [[1105, 2567], [1105, 2684], [3356, 0]],
            64,
            3366369379271963784,
        ),
        (
            [[-100, 2498], [12177, -311]],
            [
                [
                    [480827, -16940, 713777, -897007],
                    [206398, -5611, 343741, -533616],
                ],
                [
                    [267845, -4600, 328697, -534075],
                    [690647, -12126, 749291, -934390],
                ],
            ],
            [[1149, 2548], [1099, 2592], [3375, 0]],
            55,
            12464900087061242186,
        ),
        (
            [[488, 9090], [5651, -1759]],
            [
                [
                    [557676, -16460, 697064, -842133],
                    [254352, -6355, 352382, -551408],
                ],
                [
                    [237507, -6474, 318614, -469737],
                    [540368, -20388, 651649, -776829],
                ],
            ],
            [[1097, 2526], [1080, 2437], [3360, 0]],
            59,
            15394130423835981761,
        ),
        (
            [[-139, -952], [-733, 12100]],
            [
                [
                    [578079, -20419, 596118, -782962],
                    [274402, -6957, 357442, -543979],
                ],
                [
                    [183526, -5829, 297972, -455500],
                    [452238, -19104, 569671, -668773],
                ],
            ],
            [[1020, 2373], [1025, 2369], [3348, 0]],
            61,
            11365930261298796674,
        ),
        (
            [[296, 8970], [15454, -2726]],
            [
                [
                    [433457, -15506, 519827, -685853],
                    [240417, -8281, 368423, -535841],
                ],
                [
                    [223469, -6854, 329325, -490721],
                    [463682, -12111, 586552, -773533],
                ],
            ],
            [[1128, 2229], [1130, 2257], [3376, 0]],
            58,
            15845973744500780308,
        ),
        (
            [[-173, -8325], [-4335, -556]],
            [
                [
                    [443973, -10791, 501562, -697366],
                    [253935, -8235, 352778, -585066],
                ],
                [
                    [185302, -4311, 310602, -464235],
                    [409382, -11395, 488709, -581105],
                ],
            ],
            [[1057, 2065], [1126, 2176], [3365, 0]],
            54,
            14536737154175735619,
        ),
        (
            [[8666, 8937], [5708, -145]],
            [
                [
                    [386902, -13549, 459740, -666264],
                    [244099, -7046, 364822, -541245],
                ],
                [
                    [180036, -4483, 317552, -499299],
                    [353617, -10452, 455621, -629217],
                ],
            ],
            [[1141, 2002], [1143, 1990], [3375, 1]],
            46,
            7046580087927964586,
        ),
        (
            [[-32, 3382], [-299, -1596]],
            [
                [
                    [375214, -10185, 426355, -665065],
                    [266391, -6260, 344463, -509448],
                ],
                [
                    [198250, -5586, 256132, -454401],
                    [308200, -10394, 400523, -522522],
                ],
            ],
            [[1179, 2011], [1070, 1977], [3356, 0]],
            53,
            17696456235637113522,
        ),
        (
            [[-1575, -1160], [-2927, -2298]],
            [
                [
                    [275173, -12095, 373401, -550169],
                    [190912, -5952, 328559, -456324],
                ],
                [
                    [212054, -4988, 328678, -536129],
                    [348171, -9256, 432812, -576407],
                ],
            ],
            [[1051, 1879], [1038, 1919], [3352, 0]],
            41,
            11586575601300586107,
        ),
        (
            [[448, 4313], [9472, 1328]],
            [
                [
                    [306965, -7651, 404610, -591250],
                    [254213, -6739, 347636, -509108],
                ],
                [
                    [196340, -4689, 317586, -513947],
                    [269254, -6980, 386147, -539208],
                ],
            ],
            [[1074, 1857], [1093, 1880], [3362, 0]],
            45,
            13825897346865697716,
        ),
        (
            [[1474, -3879], [6558, -5487]],
            [
                [
                    [241371, -6050, 341546, -501630],
                    [198915, -4645, 290586, -485709],
                ],
                [
                    [249394, -6962, 339178, -498696],
                    [333974, -8723, 391171, -563571],
                ],
            ],
            [[1045, 1845], [1086, 1893], [3345, 0]],
            45,
            11051856880607612332,
        ),
        (
            [[-1059, -4322], [2599, 4449]],
            [
                [
                    [250524, -7226, 352806, -534099],
                    [237404, -7461, 340707, -521639],
                ],
                [
                    [231403, -5462, 322797, -489998],
                    [291760, -5642, 369699, -554416],
                ],
            ],
            [[1075, 1808], [1057, 1869], [3352, 0]],
            54,
            16531173401386837244,
        ),
        (
            [[-3540, 9219], [15729, -149]],
            [
                [
                    [267583, -7554, 364597, -563420],
                    [234922, -7189, 371217, -551007],
                ],
                [
                    [214573, -6212, 317548, -493779],
                    [237523, -5661, 369557, -538782],
                ],
            ],
            [[1066, 1876], [1096, 1805], [3359, 0]],
            43,
            563054014665051629,
        ),
        (
            [[-2286, 7087], [2702, 9112]],
            [
                [
                    [288725, -11995, 360797, -574887],
                    [237426, -8088, 363092, -482921],
                ],
                [
                    [206520, -7624, 328501, -488958],
                    [247469, -7681, 326266, -525652],
                ],
            ],
            [[1035, 1888], [1021, 1814], [3339, 0]],
            48,
            5291523704257026436,
        ),
        (
            [[5623, 7636], [5828, 4654]],
            [
                [
                    [263260, -6203, 359270, -585870],
                    [239883, -5440, 332336, -552902],
                ],
                [
                    [186932, -4785, 284040, -440520],
                    [227228, -8667, 326639, -467026],
                ],
            ],
            [[1001, 1790], [1093, 1802], [3357, 0]],
            46,
            17902478824682653344,
        ),
        (
            [[-4480, -5098], [-3027, -4180]],
            [
                [
                    [237565, -9425, 361326, -565575],
                    [247096, -6892, 331849, -544608],
                ],
                [
                    [200018, -7160, 316673, -498870],
                    [209527, -4451, 329090, -543512],
                ],
            ],
            [[1108, 1755], [1112, 1700], [3359, 0]],
            35,
            10647772592253448793,
        ),
        (
            [[-1721, 8300], [-10090, -3415]],
            [
                [
                    [217664, -5465, 348150, -551930],
                    [231018, -4330, 340471, -532800],
                ],
                [
                    [242439, -5485, 324973, -544591],
                    [252013, -6570, 340867, -519827],
                ],
            ],
            [[1208, 1867], [1122, 1792], [3373, 0]],
            40,
            8979020805552851594,
        ),
        (
            [[-1572, 5928], [-6729, 3490]],
            [
                [
                    [212094, -9356, 341314, -535148],
                    [250333, -8452, 364231, -512936],
                ],
                [
                    [228388, -6040, 334000, -529012],
                    [260203, -9463, 345293, -537951],
                ],
            ],
            [[1070, 1795], [1026, 1781], [3363, 0]],
            38,
            6142528565066678341,
        ),
        (
            [[2034, 3404], [-5819, 4110]],
            [
                [
                    [225745, -5547, 349829, -545387],
                    [227616, -3619, 396193, -502276],
                ],
                [
                    [246293, -8208, 314973, -522739],
                    [251861, -8087, 347346, -535530],
                ],
            ],
            [[1100, 1864], [1019, 1839], [3352, 0]],
            50,
            4245520504096552313,
        ),
        (
            [[-4061, 1728], [-3040, 11517]],
            [
                [
                    [222428, -6250, 321953, -532565],
                    [267963, -6883, 374501, -554859],
                ],
                [
                    [222590, -4786, 319386, -511653],
                    [222763, -5375, 331430, -488846],
                ],
            ],
            [[1128, 1895], [1131, 1816], [3361, 0]],
            39,
            18130493459418946854,
        ),
        (
            [[-1028, 9248], [1033, -2667]],
            [
                [
                    [223132, -7997, 333809, -533355],
                    [248998, -6752, 370891, -550417],
                ],
                [
                    [238750, -6996, 318546, -494028],
                    [215824, -4296, 322473, -511750],
                ],
            ],
            [[1037, 1789], [1065, 1813], [3355, 0]],
            45,
            5371807558896898256,
        ),
        (
            [[7513, -340], [1871, 552]],
            [
                [
                    [194518, -7944, 328747, -499668],
                    [246484, -7644, 419940, -498779],
                ],
                [
                    [283686, -9841, 354359, -552232],
                    [247249, -7685, 338521, -531527],
                ],
            ],
            [[1123, 1805], [1045, 1787], [3359, 1]],
            51,
            15785892076274772074,
        ),
        (
            [[2860, 8080], [8089, 9870]],
            [
                [
                    [166020, -3606, 309842, -480804],
                    [206514, -4530, 346337, -485935],
                ],
                [
                    [336944, -7731, 357207, -593803],
                    [276379, -8440, 354372, -560467],
                ],
            ],
            [[1068, 1803], [1054, 1890], [3367, 0]],
            48,
            17718907901063884648,
        ),
        (
            [[5385, 2229], [2856, 1070]],
            [
                [
                    [242750, -10372, 360805, -560176],
                    [323782, -10625, 481973, -633836],
                ],
                [
                    [227681, -8977, 348425, -535645],
                    [222321, -5963, 310454, -500255],
                ],
            ],
            [[1107, 1874], [1112, 1962], [3354, 0]],
            47,
            4213106075677133582,
        ),
        (
            [[-3707, 3475], [3129, -1153]],
            [
                [
                    [223381, -8405, 358423, -590172],
                    [316075, -11953, 481177, -626916],
                ],
                [
                    [266883, -6771, 342999, -540304],
                    [227106, -7300, 307633, -501807],
                ],
            ],
            [[1083, 1914], [1068, 1967], [3357, 0]],
            46,
            4103549187115971761,
        ),
        (
            [[3271, 2420], [-405, -2518]],
            [
                [
                    [228786, -7503, 335662, -528677],
                    [335767, -11269, 484318, -660145],
                ],
                [
                    [294951, -7595, 347916, -553164],
                    [198204, -6747, 284171, -502933],
                ],
            ],
            [[1160, 1912], [1172, 1897], [3350, 2]],
            46,
            10064208510337861134,
        ),
        (
            [[6403, -1929], [-611, -6336]],
            [
                [
                    [282695, -9594, 371031, -610122],
                    [375848, -13100, 502645, -687539],
                ],
                [
                    [225729, -6143, 352511, -520344],
                    [168206, -4897, 309755, -466029],
                ],
            ],
            [[1104, 1887], [1072, 1929], [3360, 0]],
            51,
            9108459636556892364,
        ),
        (
            [[-13580, 6798], [1211, 6817]],
            [
                [
                    [257364, -9964, 357964, -597946],
                    [398327, -12416, 527584, -670822],
                ],
                [
                    [255559, -8240, 398456, -529397],
                    [188137, -6624, 325352, -517417],
                ],
            ],
            [[1153, 1900], [1120, 2064], [3353, 0]],
            52,
            10549979710410162366,
        ),
        (
            [[825, 1307], [3959, 1678]],
            [
                [
                    [229457, -9513, 349899, -541665],
                    [334403, -12263, 502159, -672161],
                ],
                [
                    [341207, -11762, 413833, -578449],
                    [245359, -7931, 339615, -514725],
                ],
            ],
            [[1131, 2053], [1091, 2047], [3357, 1]],
            58,
            17622266452111278878,
        ),
        (
            [[-4443, 5235], [3269, 9783]],
            [
                [
                    [254494, -9384, 380076, -572180],
                    [484559, -17345, 629844, -784666],
                ],
                [
                    [286710, -7814, 382333, -536980],
                    [183172, -4222, 296362, -468417],
                ],
            ],
            [[1138, 1962], [1115, 2058], [3375, 0]],
            56,
            11629781341552848043,
        ),
        (
            [[-2460, 7478], [5260, 4871]],
            [
                [
                    [126698, -3506, 302319, -482833],
                    [275735, -9834, 518294, -607503],
                ],
                [
                    [463314, -15437, 485447, -741608],
                    [314525, -7360, 381577, -601563],
                ],
            ],
            [[1107, 2068], [1077, 2050], [3368, 0]],
            54,
            12915035038097382085,
        ),
        (
            [[3027, 10498], [4839, 3091]],
            [
                [
                    [236354, -8422, 342040, -583181],
                    [487797, -16972, 685624, -776594],
                ],
                [
                    [317631, -7027, 417955, -608573],
                    [207579, -5142, 341531, -520020],
                ],
            ],
            [[1082, 2041], [1092, 2252], [3355, 0]],
            60,
            8312266046644355459,
        ),
        (
            [[-13410, 14119], [9357, 17881]],
            [
                [
                    [227218, -6732, 311692, -518700],
                    [451793, -7678, 628558, -804204],
                ],
                [
                    [327788, -13510, 469257, -606335],
                    [258921, -8216, 322076, -529042],
                ],
            ],
            [[1093, 2138], [1163, 2297], [3356, 0]],
            55,
            3734859070189966964,
        ),
        (
            [[-2744, 11739], [9387, 917]],
            [
                [
                    [176393, -6779, 350470, -527818],
                    [397452, -16541, 654626, -734429],
                ],
                [
                    [464272, -12203, 509000, -654605],
                    [274760, -10078, 377879, -571370],
                ],
            ],
            [[1088, 2127], [1047, 2213], [3373, 2]],
            61,
            7734697010164902886,
        ),
        (
            [[6384, 19746], [10638, 794]],
            [
                [
                    [189988, -6281, 343117, -539290],
                    [417847, -14149, 674413, -737542],
                ],
                [
                    [461099, -13508, 515520, -720997],
                    [272149, -7337, 361688, -563849],
                ],
            ],
            [[1123, 2180], [1130, 2284], [3348, 0]],
            62,
            9809363651422524005,
        ),
        (
            [[-2073, 20572], [6876, 1750]],
            [
                [
                    [238548, -8007, 373753, -615605],
                    [584025, -22502, 789246, -961885],
                ],
                [
                    [367240, -13004, 494630, -610830],
                    [207572, -5719, 320780, -496859],
                ],
            ],
            [[1142, 2246], [1115, 2557], [3336, 1]],
            64,
            12079910359585560404,
        ),
        (
            [[-4896, 13112], [179, 3245]],
            [
                [
                    [253685, -12491, 365458, -607397],
                    [601049, -27875, 821853, -991769],
                ],
                [
                    [355165, -17780, 507073, -677813],
                    [194266, -5357, 299150, -471207],
                ],
            ],
            [[1101, 2178], [1115, 2493], [3365, 0]],
            57,
            867151891560317947,
        ),
        (
            [[-2692, 23024], [8745, 5270]],
            [
                [
                    [206483, -6911, 345979, -537799],
                    [564742, -31127, 797858, -922937],
                ],
                [
                    [423220, -17108, 541622, -738630],
                    [242969, -6928, 360303, -543551],
                ],
            ],
            [[1096, 2182], [1161, 2547], [3363, 0]],
            62,
            12701720896290944007,
        ),
        (
            [[2152, 31064], [21100, 6724]],
            [
                [
                    [254991, -11393, 382407, -613952],
                    [642994, -33365, 868930, -988398],
                ],
                [
                    [400024, -8985, 536233, -688805],
                    [215949, -5431, 311129, -454439],
                ],
            ],
            [[1180, 2333], [1083, 2591], [3363, 0]],
            62,
            2829291241514602874,
        ),
        (
            [[944, 33373], [10112, 3268]],
            [
                [
                    [255783, -6958, 351206, -614658],
                    [629446, -25158, 883136, -925251],
                ],
                [
                    [470384, -17919, 570177, -695418],
                    [221540, -6255, 332522, -529985],
                ],
            ],
            [[1147, 2451], [1145, 2640], [3344, 0]],
            64,
            14281426573331533624,
        ),
        (
            [[-7922, 39224], [19075, 15872]],
            [
                [
                    [248985, -9506, 343834, -594434],
                    [665546, -26340, 857647, -1045871],
                ],
                [
                    [465541, -13285, 620449, -766104],
                    [198365, -5586, 351877, -507158],
                ],
            ],
            [[1223, 2409], [1121, 2666], [3354, 0]],
            62,
            10729255112711598469,
        ),
        (
            [[-7325, 27361], [21906, 13495]],
            [
                [
                    [220383, -8848, 344934, -612716],
                    [589888, -20622, 842962, -1027515],
                ],
                [
                    [546715, -13396, 652822, -816647],
                    [250486, -5595, 370520, -577994],
                ],
            ],
            [[1137, 2489], [1151, 2669], [3364, 0]],
            59,
            14440903513537442352,
        ),
        (
            [[-8216, 26046], [-2892, -10135]],
            [
                [
                    [271123, -10001, 389641, -665809],
                    [730687, -24731, 948278, -1174356],
                ],
                [
                    [436556, -20303, 663434, -891809],
                    [202275, -8409, 346704, -533144],
                ],
            ],
            [[1230, 2493], [1176, 2874], [3365, 0]],
            56,
            14783452140352295642,
        ),
        (
            [[2869, 32434], [21063, 1239]],
            [
                [
                    [181939, -8157, 349441, -570887],
                    [559236, -25700, 854926, -923408],
                ],
                [
                    [638346, -22965, 755417, -926515],
                    [257332, -8225, 385550, -609622],
                ],
            ],
            [[1214, 2624], [1143, 2607], [3369, 1]],
            63,
            15552853780940702699,
        ),
        (
            [[5177, 48616], [16725, -2802]],
            [
                [
                    [234089, -9933, 343754, -592939],
                    [713403, -25139, 899907, -1035615],
                ],
                [
                    [477906, -20069, 619953, -736466],
                    [218935, -7315, 325879, -516131],
                ],
            ],
            [[1097, 2412], [1086, 2873], [3354, 0]],
            63,
            10212386097210300804,
        ),
        (
            [[502, 37184], [24223, -2250]],
            [
                [
                    [271678, -7617, 385201, -619055],
                    [747827, -32123, 948023, -1079886],
                ],
                [
                    [527565, -15886, 680238, -785328],
                    [227785, -7555, 332241, -538289],
                ],
            ],
            [[1156, 2711], [1144, 2996], [3348, 0]],
            63,
            13861403073308645080,
        ),
        (
            [[4084, 33432], [37907, 9779]],
            [
                [
                    [232123, -7365, 347979, -586356],
                    [696601, -23776, 936807, -1057845],
                ],
                [
                    [675924, -17730, 719551, -864083],
                    [278408, -6329, 347195, -587203],
                ],
            ],
            [[1163, 2909], [1227, 3050], [3370, 0]],
            64,
            12457235353367831476,
        ),
        (
            [[-3890, 31315], [25090, -1817]],
            [
                [
                    [217382, -9120, 389717, -600355],
                    [652345, -27712, 964071, -1095219],
                ],
                [
                    [662331, -22421, 804481, -936140],
                    [262285, -6466, 404562, -630682],
                ],
            ],
            [[1147, 2793], [1142, 2964], [3362, 0]],
            62,
            2568867730887177096,
        ),
        (
            [[4769, 36880], [27097, 11804]],
            [
                [
                    [231319, -8951, 358700, -595866],
                    [719238, -37701, 988516, -1127850],
                ],
                [
                    [631345, -14399, 811321, -1005088],
                    [248310, -4946, 399960, -584146],
                ],
            ],
            [[1143, 2878], [1178, 3016], [3362, 0]],
            64,
            5231318586447330160,
        ),
    ],
    &[
        (
            [[3196, 3662], [1161, 9427]],
            [
                [
                    [215147, -5352, 320129, -511963],
                    [241553, -5694, 347852, -499094],
                ],
                [
                    [227837, -5389, 284788, -464864],
                    [206203, -4676, 320225, -441099],
                ],
            ],
            [[1100, 1639], [1010, 1644], [3350, 0]],
            53,
            17445244923683074877,
        ),
        (
            [[-808, 6627], [-97, -5748]],
            [
                [
                    [178540, -4975, 307992, -487514],
                    [251005, -6421, 341726, -455885],
                ],
                [
                    [257193, -5634, 312775, -468296],
                    [220980, -4327, 306954, -483324],
                ],
            ],
            [[988, 1578], [1011, 1624], [3354, 0]],
            45,
            10345339912360189611,
        ),
        (
            [[-11157, 5462], [9376, 7232]],
            [
                [
                    [192697, -5517, 303428, -465169],
                    [249684, -5330, 370481, -487508],
                ],
                [
                    [231360, -5532, 296845, -448591],
                    [234520, -7043, 323142, -476969],
                ],
            ],
            [[969, 1608], [935, 1630], [3353, 1]],
            44,
            4727222272047810994,
        ),
        (
            [[-6283, 4805], [3076, 8618]],
            [
                [
                    [158883, -6749, 312664, -470674],
                    [239470, -5355, 358610, -513278],
                ],
                [
                    [304225, -7027, 342118, -509543],
                    [255144, -6033, 315906, -518938],
                ],
            ],
            [[1053, 1701], [1073, 1697], [3369, 1]],
            46,
            11057355399296864964,
        ),
        (
            [[1551, 1528], [10533, 6690]],
            [
                [
                    [190334, -6509, 308503, -466166],
                    [292791, -9559, 388591, -481258],
                ],
                [
                    [281380, -7011, 344165, -490549],
                    [256641, -5605, 298127, -450868],
                ],
            ],
            [[1062, 1654], [953, 1719], [3352, 0]],
            54,
            2418086231215259610,
        ),
        (
            [[2078, -307], [-1420, -1383]],
            [
                [
                    [237314, -5039, 319846, -476812],
                    [334764, -8708, 450805, -585526],
                ],
                [
                    [250434, -2908, 302207, -419611],
                    [180431, -2206, 284272, -423142],
                ],
            ],
            [[941, 1740], [999, 1766], [3363, 0]],
            55,
            13007266349364213603,
        ),
        (
            [[10869, 840], [-1352, 582]],
            [
                [
                    [174849, -6242, 312252, -460781],
                    [329527, -5580, 405520, -569472],
                ],
                [
                    [318206, -5363, 372873, -537704],
                    [236600, -4347, 326451, -493339],
                ],
            ],
            [[1031, 1646], [992, 1757], [3345, 1]],
            53,
            4085142636132419428,
        ),
        (
            [[-5392, 3010], [5791, 6953]],
            [
                [
                    [195618, -3501, 301693, -481513],
                    [333725, -9162, 474737, -599361],
                ],
                [
                    [344250, -9694, 397734, -534567],
                    [227778, -5443, 321767, -482302],
                ],
            ],
            [[1060, 1762], [1032, 1833], [3351, 0]],
            53,
            15953424154180786583,
        ),
        (
            [[3631, 7742], [-2340, 6147]],
            [
                [
                    [241299, -4391, 315340, -462938],
                    [419395, -10607, 557507, -631082],
                ],
                [
                    [315207, -5889, 412339, -552203],
                    [203985, -3963, 303154, -441380],
                ],
            ],
            [[1037, 1826], [1035, 1897], [3351, 0]],
            61,
            13235192802408156256,
        ),
        (
            [[-37, 15617], [1852, 6448]],
            [
                [
                    [171898, -2360, 272008, -462652],
                    [341312, -9013, 528391, -604554],
                ],
                [
                    [363627, -9799, 452251, -618471],
                    [248632, -6004, 326561, -500237],
                ],
            ],
            [[1013, 1747], [1025, 1886], [3363, 0]],
            57,
            7346871031600613629,
        ),
        (
            [[2701, 6189], [11577, -3343]],
            [
                [
                    [206228, -4838, 331108, -506252],
                    [448537, -9784, 565571, -671366],
                ],
                [
                    [322651, -8409, 376212, -516829],
                    [217525, -3292, 290755, -450003],
                ],
            ],
            [[954, 1818], [968, 2084], [3350, 0]],
            60,
            16157389430393035086,
        ),
        (
            [[-3615, 123], [14066, 5458]],
            [
                [
                    [186379, -6486, 303912, -497125],
                    [437261, -17668, 631038, -738595],
                ],
                [
                    [345877, -12243, 475973, -622353],
                    [213361, -4660, 329650, -484408],
                ],
            ],
            [[1063, 1881], [1028, 2047], [3361, 1]],
            60,
            4222170552893033217,
        ),
        (
            [[-6518, 19361], [12532, 15466]],
            [
                [
                    [183512, -6077, 318976, -506220],
                    [493861, -13565, 643341, -722802],
                ],
                [
                    [353031, -9949, 460601, -611701],
                    [220298, -4704, 291344, -469441],
                ],
            ],
            [[1053, 1924], [1011, 2144], [3364, 0]],
            64,
            10647001099694445971,
        ),
        (
            [[9294, 20546], [7723, 4973]],
            [
                [
                    [225229, -6387, 305405, -486922],
                    [497754, -20107, 700295, -819276],
                ],
                [
                    [342022, -9356, 453765, -591965],
                    [217128, -7013, 292627, -495324],
                ],
            ],
            [[1045, 1910], [1047, 2216], [3363, 0]],
            62,
            3676244975126331069,
        ),
        (
            [[13244, 25725], [14545, -2047]],
            [
                [
                    [244564, -8121, 337742, -509985],
                    [623399, -18623, 761423, -877173],
                ],
                [
                    [317036, -10947, 460658, -600494],
                    [186143, -4862, 304125, -450399],
                ],
            ],
            [[1028, 1922], [1072, 2279], [3355, 0]],
            60,
            885722004730213237,
        ),
        (
            [[5755, 36041], [8074, 6914]],
            [
                [
                    [215870, -5820, 301775, -500678],
                    [608602, -21931, 775146, -851698],
                ],
                [
                    [345893, -9977, 468238, -552115],
                    [196753, -4279, 302634, -479004],
                ],
            ],
            [[1014, 2006], [1035, 2385], [3361, 1]],
            63,
            14191812518756168786,
        ),
        (
            [[6705, 24145], [21090, -923]],
            [
                [
                    [210233, -5602, 334782, -490943],
                    [607080, -18635, 822335, -941646],
                ],
                [
                    [369872, -13221, 495553, -626271],
                    [199902, -3567, 303427, -494233],
                ],
            ],
            [[999, 2024], [1045, 2384], [3362, 0]],
            60,
            4830795528007511430,
        ),
        (
            [[-12538, 33001], [15420, 10880]],
            [
                [
                    [271107, -5608, 333069, -529830],
                    [707401, -26981, 824091, -956491],
                ],
                [
                    [317387, -10931, 485680, -611026],
                    [177627, -4502, 284999, -460136],
                ],
            ],
            [[1090, 2077], [1084, 2503], [3360, 0]],
            61,
            6270696921068538807,
        ),
        (
            [[-338, 37441], [14127, 10100]],
            [
                [
                    [213798, -6341, 282489, -473825],
                    [580711, -26661, 811099, -877201],
                ],
                [
                    [474303, -14617, 551635, -654725],
                    [230101, -4571, 331764, -522279],
                ],
            ],
            [[1019, 2171], [1089, 2347], [3352, 0]],
            64,
            10097220726170632800,
        ),
        (
            [[10484, 46182], [10896, 3743]],
            [
                [
                    [244329, -7024, 334474, -533046],
                    [636229, -24562, 809372, -873483],
                ],
                [
                    [431605, -13136, 590355, -733608],
                    [216341, -5503, 325403, -515290],
                ],
            ],
            [[1123, 2161], [1115, 2521], [3339, 0]],
            61,
            8349769073469241018,
        ),
        (
            [[462, 28254], [21661, 6261]],
            [
                [
                    [198760, -4460, 294682, -474714],
                    [565206, -16667, 748489, -842226],
                ],
                [
                    [504411, -18810, 619330, -711793],
                    [250635, -4901, 330876, -518138],
                ],
            ],
            [[1071, 2205], [1050, 2506], [3372, 0]],
            63,
            12898507375735323847,
        ),
        (
            [[8867, 37791], [16912, 9354]],
            [
                [
                    [193635, -5616, 301713, -465574],
                    [543551, -17851, 770003, -881353],
                ],
                [
                    [553469, -19096, 623112, -784814],
                    [268861, -7388, 348177, -549601],
                ],
            ],
            [[1021, 2309], [1049, 2558], [3335, 0]],
            64,
            10869756788429281223,
        ),
        (
            [[-6802, 54071], [10558, 1973]],
            [
                [
                    [196089, -5566, 323844, -533996],
                    [618830, -26932, 842312, -940716],
                ],
                [
                    [535870, -17938, 662794, -777420],
                    [218860, -4858, 334010, -520002],
                ],
            ],
            [[1141, 2336], [1073, 2630], [3376, 0]],
            63,
            2776020953257895773,
        ),
        (
            [[-507, 30277], [22334, 12249]],
            [
                [
                    [204759, -4540, 306011, -518369],
                    [591413, -20521, 790915, -972033],
                ],
                [
                    [596845, -14822, 700258, -862450],
                    [261280, -5796, 375909, -579300],
                ],
            ],
            [[1124, 2568], [1140, 2543], [3362, 0]],
            61,
            5480073248191638208,
        ),
        (
            [[-3283, 173], [-250, 3317]],
            [
                [
                    [189605, -3815, 338268, -517561],
                    [540612, -23669, 797782, -970914],
                ],
                [
                    [639961, -19960, 700634, -838492],
                    [283333, -5804, 372040, -555189],
                ],
            ],
            [[1146, 2463], [1127, 2546], [3371, 0]],
            55,
            14090091594452482689,
        ),
        (
            [[8540, 95], [5339, 7688]],
            [
                [
                    [216196, -5261, 338066, -559880],
                    [618617, -21635, 778426, -933483],
                ],
                [
                    [501541, -19540, 587823, -703321],
                    [235655, -6869, 335108, -515606],
                ],
            ],
            [[1112, 2366], [1098, 2558], [3361, 0]],
            57,
            532684575288962015,
        ),
        (
            [[8960, 330], [5612, -9058]],
            [
                [
                    [256467, -8105, 370876, -545225],
                    [620171, -26935, 761871, -892029],
                ],
                [
                    [404925, -14612, 542266, -632769],
                    [194565, -5897, 310394, -483393],
                ],
            ],
            [[1023, 2195], [1049, 2449], [3344, 0]],
            59,
            3498976442661500427,
        ),
        (
            [[5204, -242], [3092, 736]],
            [
                [
                    [209032, -6194, 323760, -504334],
                    [487437, -25042, 627952, -755202],
                ],
                [
                    [429064, -13505, 528080, -654614],
                    [220872, -5732, 334523, -541402],
                ],
            ],
            [[1130, 2159], [1108, 2282], [3374, 0]],
            55,
            8131684761986276135,
        ),
        (
            [[-724, -203], [-2386, -541]],
            [
                [
                    [222121, -6263, 337816, -541492],
                    [466021, -18412, 539310, -802870],
                ],
                [
                    [350756, -10852, 492634, -605166],
                    [214959, -4124, 318736, -480570],
                ],
            ],
            [[1059, 2023], [1131, 2169], [3366, 0]],
            55,
            17429248656933741873,
        ),
        (
            [[344, 16769], [1098, -4060]],
            [
                [
                    [222153, -7473, 353215, -557381],
                    [408576, -12726, 534732, -716125],
                ],
                [
                    [314151, -5861, 435082, -659944],
                    [206039, -5562, 323743, -545496],
                ],
            ],
            [[1151, 1979], [1172, 2020], [3376, 0]],
            41,
            10260873325347450028,
        ),
        (
            [[-104, 182], [535, -6957]],
            [
                [
                    [242222, -8476, 332112, -559576],
                    [394241, -12905, 478922, -663155],
                ],
                [
                    [313199, -6592, 373000, -555662],
                    [200295, -6299, 291044, -445617],
                ],
            ],
            [[1189, 1955], [1088, 1988], [3359, 0]],
            49,
            13687281576797130575,
        ),
        (
            [[-3443, 1762], [-1556, 3486]],
            [
                [
                    [202045, -8841, 307760, -474904],
                    [292986, -11609, 430750, -576352],
                ],
                [
                    [323846, -8675, 424077, -656698],
                    [258007, -9583, 364961, -525338],
                ],
            ],
            [[1058, 1906], [1052, 1952], [3350, 0]],
            54,
            6657433605221463893,
        ),
        (
            [[-3455, -250], [5127, 11944]],
            [
                [
                    [236222, -5628, 336883, -523448],
                    [347719, -11886, 431161, -618971],
                ],
                [
                    [305405, -6909, 395159, -619560],
                    [201836, -4464, 328234, -502835],
                ],
            ],
            [[1087, 1897], [1118, 1937], [3361, 0]],
            45,
            11487888512998809582,
        ),
        (
            [[6203, -2046], [4569, -6553]],
            [
                [
                    [189572, -4591, 303141, -465494],
                    [266035, -9276, 364347, -599905],
                ],
                [
                    [320390, -9325, 423764, -597539],
                    [268029, -6759, 371981, -534768],
                ],
            ],
            [[1067, 1866], [1095, 1943], [3344, 0]],
            46,
            7664473325022277103,
        ),
        (
            [[-1425, -2491], [348, 7344]],
            [
                [
                    [201868, -5763, 328869, -472556],
                    [286010, -10600, 395006, -607601],
                ],
                [
                    [304668, -8864, 391812, -558742],
                    [261089, -5052, 347134, -551878],
                ],
            ],
            [[1063, 1866], [1041, 1914], [3353, 0]],
            49,
            254990623148833692,
        ),
        (
            [[1662, 6081], [4284, 8900]],
            [
                [
                    [215271, -5589, 341642, -525180],
                    [285201, -8820, 427282, -640644],
                ],
                [
                    [267633, -7422, 365021, -556701],
                    [212671, -4107, 344995, -531381],
                ],
            ],
            [[1078, 1900], [1116, 1874], [3361, 0]],
            49,
            14562747466861282663,
        ),
        (
            [[-2770, 1254], [167, 8528]],
            [
                [
                    [250526, -7652, 330642, -526291],
                    [272069, -10518, 400648, -577125],
                ],
                [
                    [260074, -8815, 375963, -554343],
                    [241622, -6739, 324420, -524508],
                ],
            ],
            [[1038, 1921], [1037, 1839], [3344, 0]],
            46,
            13723601920239817227,
        ),
        (
            [[4227, 2951], [3393, 5897]],
            [
                [
                    [227852, -5904, 341285, -526170],
                    [280866, -9964, 363176, -639634],
                ],
                [
                    [233950, -6207, 328685, -476749],
                    [235830, -6025, 325663, -486273],
                ],
            ],
            [[1005, 1824], [1105, 1870], [3361, 0]],
            49,
            3560448186341434422,
        ),
        (
            [[-583, -6857], [-683, -3972]],
            [
                [
                    [233008, -9165, 360708, -530137],
                    [274215, -11608, 360438, -619331],
                ],
                [
                    [236770, -7733, 353410, -543041],
                    [226503, -5062, 354030, -547848],
                ],
            ],
            [[1098, 1817], [1123, 1784], [3359, 0]],
            39,
            9641780189758166256,
        ),
        (
            [[1318, -1316], [-6845, 2636]],
            [
                [
                    [218138, -6790, 343478, -544101],
                    [242180, -5485, 366875, -623479],
                ],
                [
                    [286834, -6990, 347131, -584965],
                    [306190, -8011, 371561, -531316],
                ],
            ],
            [[1197, 1940], [1137, 1895], [3372, 0]],
            42,
            7032266186811380847,
        ),
        (
            [[2758, 4246], [-6355, 2449]],
            [
                [
                    [205595, -5959, 352018, -528516],
                    [263606, -10670, 372203, -576666],
                ],
                [
                    [250706, -8303, 355659, -578596],
                    [288607, -7940, 397059, -565360],
                ],
            ],
            [[1076, 1816], [1037, 1885], [3362, 0]],
            44,
            7808431990012839459,
        ),
        (
            [[10051, 683], [-3207, 8984]],
            [
                [
                    [229914, -4901, 353433, -518331],
                    [220972, -8285, 385900, -559694],
                ],
                [
                    [267504, -8580, 344749, -565176],
                    [298266, -10794, 405588, -584555],
                ],
            ],
            [[1103, 1920], [1037, 1917], [3353, 0]],
            54,
            4029410580991980905,
        ),
        (
            [[-4016, -2398], [-2615, 19889]],
            [
                [
                    [234203, -5950, 349548, -523146],
                    [229386, -10782, 350331, -584969],
                ],
                [
                    [239175, -5661, 336156, -535351],
                    [285685, -7759, 404445, -536527],
                ],
            ],
            [[1141, 1871], [1143, 1853], [3360, 0]],
            40,
            8377848657533015244,
        ),
        (
            [[3221, 4518], [670, 4786]],
            [
                [
                    [249424, -6367, 377384, -564475],
                    [238378, -7919, 350891, -593754],
                ],
                [
                    [263213, -7496, 325856, -545805],
                    [297876, -6260, 406354, -575701],
                ],
            ],
            [[1053, 1903], [1072, 1938], [3352, 0]],
            44,
            2200829881109505252,
        ),
        (
            [[10934, -392], [708, 10864]],
            [
                [
                    [223767, -8565, 367761, -527321],
                    [215366, -8198, 382175, -548871],
                ],
                [
                    [283037, -7974, 380059, -611247],
                    [358517, -9439, 452814, -612261],
                ],
            ],
            [[1133, 1873], [1065, 1922], [3363, 1]],
            58,
            819213304379559813,
        ),
        (
            [[7729, 2565], [11308, 15582]],
            [
                [
                    [185417, -4248, 351027, -478767],
                    [174986, -5074, 312773, -541249],
                ],
                [
                    [320808, -10040, 372183, -633286],
                    [392069, -11276, 470545, -659648],
                ],
            ],
            [[1097, 1893], [1089, 1986], [3368, 0]],
            52,
            5705993033259391791,
        ),
        (
            [[11818, -10125], [17878, 4863]],
            [
                [
                    [279897, -11065, 427991, -563477],
                    [258208, -11681, 396596, -630890],
                ],
                [
                    [234057, -6280, 342427, -553791],
                    [326368, -8649, 437453, -601427],
                ],
            ],
            [[1117, 1971], [1133, 2102], [3352, 0]],
            50,
            5979859174503544683,
        ),
        (
            [[-366, -5920], [-4432, 6582]],
            [
                [
                    [302739, -8923, 445884, -604423],
                    [254446, -8324, 393574, -632397],
                ],
                [
                    [254792, -8471, 339165, -584730],
                    [343617, -10996, 470432, -595681],
                ],
            ],
            [[1098, 2048], [1089, 2109], [3358, 0]],
            52,
            3650329557200847853,
        ),
        (
            [[1353, -9257], [3458, 4879]],
            [
                [
                    [296943, -9795, 448485, -607835],
                    [247487, -8513, 362805, -642621],
                ],
                [
                    [252493, -8197, 315500, -567063],
                    [347486, -11195, 459853, -624863],
                ],
            ],
            [[1187, 2034], [1182, 2028], [3351, 1]],
            42,
            3351193806181040975,
        ),
        (
            [[9415, -4121], [-1398, 9161]],
            [
                [
                    [371851, -10185, 473487, -657783],
                    [260027, -12114, 392561, -658320],
                ],
                [
                    [199995, -7130, 319380, -492456],
                    [300338, -14795, 491707, -615928],
                ],
            ],
            [[1113, 2043], [1079, 2027], [3361, 0]],
            55,
            16773913303041016309,
        ),
        (
            [[-2419, 3874], [-2248, 11168]],
            [
                [
                    [341010, -11676, 478387, -722778],
                    [270289, -12177, 380504, -611531],
                ],
                [
                    [207753, -5036, 349312, -521545],
                    [326862, -10183, 516075, -631079],
                ],
            ],
            [[1138, 2011], [1084, 2118], [3355, 0]],
            53,
            10928565271568110595,
        ),
        (
            [[708, -12867], [4206, 16983]],
            [
                [
                    [311782, -10585, 492587, -628625],
                    [221186, -11977, 349560, -580521],
                ],
                [
                    [278370, -13725, 357392, -557695],
                    [447233, -12471, 587926, -667549],
                ],
            ],
            [[1145, 2153], [1098, 2218], [3358, 0]],
            56,
            4294734409782868258,
        ),
        (
            [[8795, 4540], [926, 11768]],
            [
                [
                    [452790, -15491, 550472, -681166],
                    [301743, -13688, 402398, -688912],
                ],
                [
                    [216753, -8059, 321472, -525619],
                    [369106, -9800, 569778, -673468],
                ],
            ],
            [[1161, 2229], [1118, 2244], [3382, 1]],
            57,
            1481604756747737972,
        ),
        (
            [[2126, -6644], [11049, 11342]],
            [
                [
                    [205387, -8184, 438684, -576099],
                    [145931, -8191, 322780, -502435],
                ],
                [
                    [340231, -11186, 400784, -682531],
                    [697117, -17929, 705272, -874109],
                ],
            ],
            [[1144, 2104], [1076, 2428], [3370, 0]],
            57,
            3553443984160296276,
        ),
        (
            [[12398, -2666], [10441, 11067]],
            [
                [
                    [409311, -12276, 540036, -730956],
                    [258000, -10447, 407894, -635504],
                ],
                [
                    [230253, -6000, 346212, -570103],
                    [433215, -11564, 654136, -770530],
                ],
            ],
            [[1108, 2260], [1087, 2374], [3353, 0]],
            55,
            17698034467369901547,
        ),
        (
            [[-2716, -6640], [426, 33933]],
            [
                [
                    [425267, -12521, 525206, -702629],
                    [233633, -6756, 354026, -629391],
                ],
                [
                    [243015, -7860, 367297, -560683],
                    [562118, -19901, 685898, -746902],
                ],
            ],
            [[1087, 2377], [1134, 2525], [3356, 0]],
            57,
            9733370967508277749,
        ),
        (
            [[6721, 5211], [11765, 20538]],
            [
                [
                    [337410, -9821, 568494, -686340],
                    [199155, -8092, 358817, -592378],
                ],
                [
                    [314824, -8277, 389518, -591655],
                    [628394, -21117, 747338, -935005],
                ],
            ],
            [[1086, 2324], [1043, 2595], [3375, 2]],
            57,
            16698851369084867222,
        ),
        (
            [[15167, -3037], [7961, 19667]],
            [
                [
                    [367910, -14299, 562076, -685667],
                    [194451, -7306, 342083, -553365],
                ],
                [
                    [329196, -8619, 370217, -601942],
                    [642736, -19789, 734396, -923051],
                ],
            ],
            [[1131, 2347], [1135, 2589], [3348, 0]],
            59,
            14969007229881004265,
        ),
        (
            [[21724, -2679], [2440, 10336]],
            [
                [
                    [512737, -16688, 684950, -816015],
                    [253723, -9533, 413752, -688674],
                ],
                [
                    [247652, -8017, 356228, -525588],
                    [506913, -19530, 688955, -790324],
                ],
            ],
            [[1160, 2576], [1147, 2601], [3337, 0]],
            61,
            15935355922911938237,
        ),
        (
            [[21503, -6459], [3252, 29242]],
            [
                [
                    [565128, -23404, 710942, -890282],
                    [274955, -13077, 406369, -702231],
                ],
                [
                    [225593, -8122, 350627, -532357],
                    [487021, -15750, 681632, -809448],
                ],
            ],
            [[1102, 2584], [1152, 2565], [3366, 0]],
            61,
            6481199568775575145,
        ),
        (
            [[12133, 7965], [7134, 21175]],
            [
                [
                    [473714, -22832, 669360, -879318],
                    [243619, -10355, 376670, -645874],
                ],
                [
                    [269479, -9024, 379103, -608435],
                    [590392, -19963, 729021, -890329],
                ],
            ],
            [[1122, 2461], [1177, 2705], [3363, 0]],
            60,
            192826684256008709,
        ),
        (
            [[15092, 1578], [5489, 17955]],
            [
                [
                    [552330, -21747, 783829, -942065],
                    [266310, -10927, 421674, -682816],
                ],
                [
                    [242067, -6986, 349258, -564045],
                    [536671, -20179, 690431, -865603],
                ],
            ],
            [[1176, 2717], [1104, 2647], [3364, 0]],
            61,
            3522502750659960390,
        ),
        (
            [[17693, 1953], [6802, 25809]],
            [
                [
                    [545200, -19542, 735768, -932432],
                    [263997, -7101, 399535, -630993],
                ],
                [
                    [277867, -6532, 362765, -557869],
                    [587173, -24307, 719854, -868499],
                ],
            ],
            [[1181, 2766], [1120, 2800], [3348, 0]],
            64,
            9381661528450571988,
        ),
        (
            [[15653, -3183], [20552, 32843]],
            [
                [
                    [605503, -21088, 723441, -1006421],
                    [311374, -10603, 389188, -652569],
                ],
                [
                    [275901, -7359, 375859, -573893],
                    [564079, -20173, 808401, -978241],
                ],
            ],
            [[1236, 2850], [1155, 2843], [3355, 0]],
            58,
            4808217633792031158,
        ),
        (
            [[39144, -2550], [9303, 33013]],
            [
                [
                    [535871, -27437, 704904, -909372],
                    [254765, -7740, 393257, -646103],
                ],
                [
                    [290002, -7691, 401843, -599013],
                    [697584, -21334, 846567, -1015518],
                ],
            ],
            [[1149, 2755], [1162, 2864], [3363, 0]],
            60,
            4389601460176427758,
        ),
        (
            [[33194, 999], [224, -26001]],
            [
                [
                    [656193, -21308, 809593, -960876],
                    [289896, -11914, 433870, -725199],
                ],
                [
                    [208552, -6496, 394050, -586316],
                    [562458, -27209, 796676, -1066785],
                ],
            ],
            [[1231, 2825], [1211, 2860], [3363, 0]],
            63,
            4928738719872583956,
        ),
        (
            [[30281, -8667], [10699, 49115]],
            [
                [
                    [454694, -24473, 753969, -936200],
                    [215567, -7300, 373485, -595595],
                ],
                [
                    [314385, -9330, 427375, -692220],
                    [791697, -30737, 910411, -1140344],
                ],
            ],
            [[1210, 2762], [1180, 3001], [3367, 0]],
            60,
            14542707850618946694,
        ),
        (
            [[39250, 8964], [443, 7620]],
            [
                [
                    [611400, -23963, 764767, -952600],
                    [280445, -9843, 405397, -669251],
                ],
                [
                    [240745, -5737, 361489, -546085],
                    [622639, -25095, 805368, -940806],
                ],
            ],
            [[1135, 2824], [1095, 2988], [3357, 0]],
            64,
            17875808517730254282,
        ),
        (
            [[38879, -3378], [7047, 26379]],
            [
                [
                    [651462, -20744, 773700, -927685],
                    [291714, -11622, 402938, -691927],
                ],
                [
                    [264734, -9541, 387986, -595414],
                    [663231, -25196, 884268, -1025094],
                ],
            ],
            [[1173, 2954], [1152, 3071], [3348, 0]],
            61,
            6895259397303921910,
        ),
        (
            [[34377, 15938], [13048, 42484]],
            [
                [
                    [582035, -19669, 788236, -925253],
                    [276236, -8603, 415493, -629428],
                ],
                [
                    [317414, -5623, 400208, -612460],
                    [801787, -25527, 914958, -1223890],
                ],
            ],
            [[1188, 3004], [1267, 3381], [3366, 0]],
            63,
            3454938303570807782,
        ),
        (
            [[-7811, -13041], [21926, 38911]],
            [
                [
                    [548843, -37979, 847784, -1043411],
                    [264440, -10732, 398687, -676052],
                ],
                [
                    [331829, -9693, 433783, -624159],
                    [872862, -29321, 1008401, -1165574],
                ],
            ],
            [[1170, 2928], [1153, 3327], [3359, 0]],
            62,
            4265736412414572539,
        ),
        (
            [[33941, 11898], [6142, 38371]],
            [
                [
                    [602304, -25288, 851929, -962852],
                    [260566, -10932, 412610, -647396],
                ],
                [
                    [278611, -7148, 413755, -626633],
                    [778542, -21490, 987948, -1206317],
                ],
            ],
            [[1141, 3025], [1178, 3296], [3362, 0]],
            63,
            16476154256335942503,
        ),
    ],
];
const PUNISHED_EARNED_1024: [&[EarnedBlock]; 2] = [
    &[
        (
            [[17, 14, 3], [11, 16, 3]],
            33,
            [[8889, -51315], [-24839, 55684]],
            -24175,
            41361,
        ),
        (
            [[18, 12, 1], [14, 18, 1]],
            36,
            [[23360, -29480], [-49186, 42237]],
            43043,
            108579,
        ),
        (
            [[11, 17, 3], [12, 19, 2]],
            30,
            [[45889, -47982], [-35529, 63825]],
            31280,
            -34256,
        ),
        (
            [[10, 15, 3], [9, 25, 2]],
            35,
            [[5881, -11039], [-13262, 88808]],
            45655,
            111191,
        ),
        (
            [[16, 13, 1], [9, 23, 2]],
            39,
            [[49607, -59856], [-41630, 93360]],
            28323,
            93859,
        ),
        (
            [[27, 7, 2], [7, 20, 1]],
            47,
            [[125390, -14673], [-14881, 71117]],
            40536,
            106072,
        ),
        (
            [[18, 10, 2], [6, 25, 3]],
            43,
            [[98637, -20167], [-20389, 117929]],
            -18777,
            46759,
        ),
        (
            [[25, 4, 3], [4, 28, 0]],
            53,
            [[115911, -3644], [-7338, 100708]],
            -2469,
            -68005,
        ),
        (
            [[31, 2, 1], [5, 23, 2]],
            54,
            [[169977, -13981], [-10490, 122549]],
            46617,
            112153,
        ),
        (
            [[26, 2, 0], [1, 35, 0]],
            61,
            [[125458, -17580], [-261, 153484]],
            46691,
            112227,
        ),
        (
            [[33, 0, 0], [1, 30, 0]],
            63,
            [[135123, 0], [-105, 119799]],
            46691,
            112227,
        ),
        (
            [[32, 0, 0], [0, 32, 0]],
            64,
            [[118841, 0], [0, 96209]],
            46691,
            112227,
        ),
        (
            [[32, 0, 0], [1, 31, 0]],
            63,
            [[123090, 0], [-3334, 116374]],
            46691,
            112227,
        ),
        (
            [[32, 1, 0], [0, 30, 1]],
            62,
            [[149876, -237], [0, 83354]],
            46691,
            112227,
        ),
        (
            [[37, 0, 0], [1, 26, 0]],
            63,
            [[147357, 0], [-950, 83899]],
            46691,
            112227,
        ),
        (
            [[34, 0, 1], [1, 28, 0]],
            62,
            [[150744, 0], [-344, 113788]],
            46691,
            112227,
        ),
        (
            [[32, 0, 1], [0, 31, 0]],
            63,
            [[151409, 0], [0, 127297]],
            46019,
            111555,
        ),
        (
            [[38, 0, 0], [0, 26, 0]],
            64,
            [[167532, 0], [0, 98524]],
            46691,
            112227,
        ),
        (
            [[31, 0, 0], [0, 33, 0]],
            64,
            [[119145, 0], [0, 105073]],
            46691,
            112227,
        ),
        (
            [[34, 0, 0], [0, 30, 0]],
            64,
            [[112237, 0], [0, 161931]],
            46691,
            112227,
        ),
        (
            [[30, 0, 0], [0, 34, 0]],
            64,
            [[73583, 0], [0, 147076]],
            46691,
            112227,
        ),
        (
            [[29, 0, 0], [0, 35, 0]],
            64,
            [[71119, 0], [0, 164664]],
            46691,
            112227,
        ),
        (
            [[32, 0, 0], [0, 32, 0]],
            64,
            [[27793, 0], [0, 119751]],
            46691,
            112227,
        ),
        (
            [[29, 0, 0], [0, 35, 0]],
            64,
            [[55058, 0], [0, 91182]],
            46691,
            112227,
        ),
        (
            [[29, 0, 0], [0, 35, 0]],
            0,
            [[-263122, 0], [0, -465803]],
            -46691,
            -112227,
        ),
        (
            [[33, 0, 0], [0, 31, 0]],
            0,
            [[-341665, 0], [0, -303945]],
            -46691,
            -112227,
        ),
        (
            [[36, 0, 0], [0, 28, 0]],
            0,
            [[-325185, 0], [0, -262436]],
            -46691,
            -112227,
        ),
        (
            [[29, 2, 1], [0, 32, 0]],
            2,
            [[-202785, 3799], [0, -216208]],
            -46691,
            -112227,
        ),
        (
            [[30, 3, 1], [2, 28, 0]],
            5,
            [[-195429, 957], [-38, -247585]],
            -46691,
            18845,
        ),
        (
            [[31, 0, 3], [3, 27, 0]],
            3,
            [[-129117, -206], [3161, -150171]],
            -40630,
            -106166,
        ),
        (
            [[28, 3, 4], [6, 22, 1]],
            9,
            [[-122045, 27953], [2482, -142430]],
            -45711,
            -111247,
        ),
        (
            [[19, 5, 5], [3, 32, 0]],
            8,
            [[-101898, 6301], [7700, -196141]],
            10937,
            -54599,
        ),
        (
            [[23, 8, 1], [6, 23, 3]],
            14,
            [[-100647, 15662], [2657, -108741]],
            -46019,
            -111555,
        ),
        (
            [[21, 6, 1], [9, 26, 1]],
            15,
            [[-61684, 5871], [14160, -138695]],
            -28982,
            -94518,
        ),
        (
            [[18, 10, 1], [11, 23, 1]],
            21,
            [[-48469, 27110], [9209, -109399]],
            2563,
            68099,
        ),
        (
            [[20, 4, 8], [10, 20, 2]],
            14,
            [[-52705, 6081], [1524, -66047]],
            -44671,
            20865,
        ),
        (
            [[22, 7, 3], [10, 21, 1]],
            17,
            [[-67168, 23807], [10897, -49286]],
            8533,
            -57003,
        ),
        (
            [[13, 17, 4], [10, 18, 2]],
            27,
            [[-24796, 13804], [12268, -54742]],
            -40528,
            25008,
        ),
        (
            [[13, 18, 1], [14, 15, 3]],
            32,
            [[-21278, 29132], [8510, -13712]],
            24613,
            90149,
        ),
        (
            [[17, 12, 0], [11, 21, 3]],
            23,
            [[-27345, 16701], [18393, -65645]],
            2859,
            68395,
        ),
        (
            [[12, 15, 3], [12, 19, 3]],
            27,
            [[-26450, 42230], [29449, -41744]],
            -40338,
            -105874,
        ),
        (
            [[15, 13, 2], [18, 16, 0]],
            31,
            [[-34291, 47444], [12170, -50072]],
            23407,
            88943,
        ),
        (
            [[11, 20, 1], [13, 15, 4]],
            33,
            [[-35206, 72252], [28533, -30016]],
            27387,
            92923,
        ),
        (
            [[9, 19, 3], [13, 16, 4]],
            32,
            [[-18274, 26380], [19674, -23794]],
            -18324,
            -83860,
        ),
        (
            [[9, 15, 3], [24, 12, 1]],
            39,
            [[-6454, 90581], [63803, -28889]],
            33341,
            98877,
        ),
        (
            [[2, 22, 0], [25, 11, 4]],
            47,
            [[2339, 49186], [83310, -9099]],
            46171,
            111707,
        ),
        (
            [[11, 22, 1], [15, 12, 3]],
            37,
            [[-10338, 126695], [23202, -16008]],
            13321,
            78857,
        ),
        (
            [[5, 24, 4], [20, 10, 1]],
            44,
            [[1792, 115235], [35570, -11424]],
            38383,
            -27153,
        ),
        (
            [[3, 23, 5], [26, 3, 4]],
            49,
            [[-11402, 85694], [68296, -4528]],
            45495,
            111031,
        ),
        (
            [[3, 32, 2], [22, 4, 1]],
            54,
            [[-3183, 135930], [37503, -1424]],
            -2461,
            63075,
        ),
        (
            [[3, 32, 0], [22, 6, 1]],
            54,
            [[-5717, 168088], [76236, -9007]],
            46431,
            111967,
        ),
        (
            [[2, 27, 1], [25, 8, 1]],
            52,
            [[139, 109488], [101041, -26581]],
            46617,
            112153,
        ),
        (
            [[1, 35, 1], [25, 2, 0]],
            60,
            [[-2053, 228194], [70658, -4023]],
            46691,
            112227,
        ),
        (
            [[1, 19, 0], [34, 8, 2]],
            53,
            [[-717, 107718], [118443, -30826]],
            46617,
            112153,
        ),
        (
            [[0, 34, 0], [26, 1, 3]],
            60,
            [[0, 215573], [55379, -711]],
            -2461,
            63075,
        ),
        (
            [[0, 30, 0], [27, 5, 2]],
            57,
            [[0, 150220], [73689, -21604]],
            46691,
            -18845,
        ),
        (
            [[0, 25, 0], [39, 0, 0]],
            64,
            [[0, 113951], [170334, -567]],
            46691,
            112227,
        ),
        (
            [[0, 26, 0], [36, 2, 0]],
            62,
            [[0, 132626], [121982, -4494]],
            46691,
            112227,
        ),
        (
            [[0, 35, 0], [28, 0, 1]],
            63,
            [[0, 191397], [98345, 0]],
            46683,
            112219,
        ),
        (
            [[0, 35, 0], [29, 0, 0]],
            64,
            [[0, 150627], [70818, 0]],
            46691,
            112227,
        ),
        (
            [[0, 31, 0], [33, 0, 0]],
            64,
            [[0, 122332], [101867, 0]],
            46691,
            112227,
        ),
        (
            [[0, 35, 0], [28, 1, 0]],
            63,
            [[0, 113017], [90182, -3665]],
            40630,
            106166,
        ),
        (
            [[0, 32, 0], [32, 0, 0]],
            64,
            [[0, 119785], [135785, 0]],
            46691,
            112227,
        ),
        (
            [[0, 34, 0], [30, 0, 0]],
            64,
            [[0, 68248], [166910, 0]],
            46691,
            112227,
        ),
        (
            [[0, 30, 0], [34, 0, 0]],
            64,
            [[0, 129526], [186741, 0]],
            46691,
            112227,
        ),
        (
            [[0, 36, 0], [28, 0, 0]],
            64,
            [[0, 131680], [69434, 0]],
            46691,
            112227,
        ),
        (
            [[0, 27, 0], [37, 0, 0]],
            64,
            [[0, 104820], [146149, 0]],
            46691,
            112227,
        ),
        (
            [[0, 34, 0], [30, 0, 0]],
            64,
            [[0, 149541], [98869, 0]],
            46691,
            112227,
        ),
        (
            [[0, 34, 0], [30, 0, 0]],
            64,
            [[0, 148425], [120932, 0]],
            46691,
            112227,
        ),
        (
            [[0, 30, 0], [34, 0, 0]],
            64,
            [[0, 169768], [165631, 0]],
            46691,
            112227,
        ),
        (
            [[0, 29, 0], [35, 0, 0]],
            64,
            [[0, 69371], [168873, 0]],
            46691,
            112227,
        ),
        (
            [[0, 31, 0], [33, 0, 0]],
            64,
            [[0, 90280], [127757, 0]],
            46691,
            112227,
        ),
    ],
    &[
        (
            [[15, 15, 4], [13, 14, 3]],
            28,
            [[-6501, 46844], [28063, -38166]],
            24965,
            -40571,
        ),
        (
            [[8, 18, 5], [16, 15, 2]],
            34,
            [[-11107, 75391], [59470, -36053]],
            -44205,
            -109741,
        ),
        (
            [[8, 20, 3], [17, 11, 5]],
            37,
            [[-31615, 74633], [65796, -10996]],
            -31276,
            34260,
        ),
        (
            [[4, 22, 2], [21, 13, 2]],
            43,
            [[-2755, 60542], [69294, -40563]],
            -30652,
            34884,
        ),
        (
            [[5, 24, 1], [21, 10, 3]],
            45,
            [[-6154, 155550], [101602, -25483]],
            -16428,
            -81964,
        ),
        (
            [[6, 29, 1], [19, 7, 2]],
            48,
            [[-31292, 137248], [106597, -6579]],
            44661,
            110197,
        ),
        (
            [[2, 26, 2], [31, 2, 1]],
            57,
            [[-11380, 103174], [110475, -6912]],
            43935,
            109471,
        ),
        (
            [[4, 28, 0], [26, 6, 0]],
            54,
            [[-2980, 144272], [128774, -5947]],
            29216,
            94752,
        ),
        (
            [[3, 30, 1], [25, 4, 1]],
            55,
            [[-1278, 239533], [126961, -4507]],
            29216,
            94752,
        ),
        (
            [[1, 27, 0], [30, 6, 0]],
            57,
            [[2032, 162575], [130432, -2926]],
            46691,
            112227,
        ),
        (
            [[2, 30, 1], [29, 1, 1]],
            59,
            [[-9840, 156669], [87962, -1425]],
            46691,
            112227,
        ),
        (
            [[0, 32, 0], [28, 2, 2]],
            60,
            [[0, 196876], [57839, -3135]],
            46667,
            112203,
        ),
        (
            [[0, 32, 0], [31, 1, 0]],
            63,
            [[0, 161550], [84539, -3331]],
            46467,
            112003,
        ),
        (
            [[0, 33, 0], [30, 1, 0]],
            63,
            [[0, 122816], [56205, -1276]],
            46691,
            112227,
        ),
        (
            [[0, 37, 0], [25, 0, 2]],
            62,
            [[0, 176489], [62124, 0]],
            46691,
            112227,
        ),
        (
            [[0, 35, 0], [29, 0, 0]],
            64,
            [[0, 162505], [92841, 0]],
            46691,
            112227,
        ),
        (
            [[0, 33, 0], [31, 0, 0]],
            64,
            [[0, 149412], [78639, 0]],
            46691,
            112227,
        ),
        (
            [[0, 38, 0], [25, 1, 0]],
            63,
            [[0, 133809], [80432, -993]],
            44671,
            110207,
        ),
        (
            [[0, 31, 0], [33, 0, 0]],
            64,
            [[0, 97065], [146049, 0]],
            46691,
            112227,
        ),
        (
            [[0, 34, 0], [30, 0, 0]],
            64,
            [[0, 115701], [136374, 0]],
            46691,
            112227,
        ),
        (
            [[0, 30, 0], [34, 0, 0]],
            64,
            [[0, 92652], [168392, 0]],
            46691,
            112227,
        ),
        (
            [[0, 29, 0], [35, 0, 0]],
            64,
            [[0, 48232], [143038, 0]],
            46691,
            112227,
        ),
        (
            [[0, 32, 0], [32, 0, 0]],
            64,
            [[0, 82606], [187898, 0]],
            46691,
            112227,
        ),
        (
            [[0, 29, 0], [35, 0, 0]],
            64,
            [[0, 56046], [129166, 0]],
            46691,
            112227,
        ),
        (
            [[0, 29, 0], [35, 0, 0]],
            0,
            [[0, -308161], [-435842, 0]],
            -46691,
            -112227,
        ),
        (
            [[0, 33, 0], [31, 0, 0]],
            0,
            [[0, -355384], [-269875, 0]],
            -46691,
            -112227,
        ),
        (
            [[0, 36, 0], [28, 0, 0]],
            0,
            [[0, -409193], [-223685, 0]],
            -46691,
            -112227,
        ),
        (
            [[1, 31, 0], [28, 4, 0]],
            5,
            [[-395, -291852], [-238258, 1628]],
            -44619,
            -110155,
        ),
        (
            [[2, 32, 0], [30, 0, 0]],
            2,
            [[147, -183286], [-191857, 0]],
            2461,
            -63075,
        ),
        (
            [[0, 32, 2], [27, 1, 2]],
            1,
            [[0, -164611], [-59579, 13]],
            2461,
            -63075,
        ),
        (
            [[3, 32, 0], [24, 1, 4]],
            4,
            [[738, -163482], [-98663, 1681]],
            -44671,
            -110207,
        ),
        (
            [[4, 25, 0], [23, 10, 2]],
            14,
            [[13280, -100092], [-61388, 25948]],
            -27186,
            -92722,
        ),
        (
            [[8, 23, 1], [23, 6, 3]],
            14,
            [[16423, -116082], [-68298, 7815]],
            -40539,
            24997,
        ),
        (
            [[4, 21, 3], [21, 11, 4]],
            15,
            [[3437, -28956], [-87511, 49973]],
            -40576,
            -106112,
        ),
        (
            [[3, 24, 2], [25, 6, 4]],
            9,
            [[7287, -78883], [-111053, 19158]],
            -44401,
            -109937,
        ),
        (
            [[7, 25, 0], [24, 7, 1]],
            14,
            [[21193, -33377], [-45683, 17012]],
            -24337,
            -89873,
        ),
        (
            [[9, 20, 3], [17, 14, 1]],
            23,
            [[22415, -51281], [-67448, 28056]],
            -46575,
            18961,
        ),
        (
            [[10, 23, 1], [18, 10, 2]],
            20,
            [[6503, -11762], [-59871, 23489]],
            15957,
            -49579,
        ),
        (
            [[12, 16, 4], [19, 9, 4]],
            21,
            [[7868, -28323], [-39738, 19311]],
            -24697,
            -90233,
        ),
        (
            [[11, 16, 2], [14, 17, 4]],
            28,
            [[7185, -12213], [-28315, 89979]],
            -40583,
            -106119,
        ),
        (
            [[9, 17, 4], [11, 19, 4]],
            28,
            [[16193, -35780], [-27801, 69704]],
            39697,
            -25839,
        ),
        (
            [[13, 15, 2], [17, 13, 4]],
            26,
            [[39359, -15568], [-20187, 58478]],
            -35773,
            -101309,
        ),
        (
            [[14, 15, 3], [17, 13, 2]],
            27,
            [[48568, -25819], [-38806, 69110]],
            -44931,
            -110467,
        ),
        (
            [[15, 11, 5], [14, 18, 1]],
            33,
            [[34765, -21842], [-6306, 89260]],
            20651,
            -44885,
        ),
        (
            [[11, 13, 3], [11, 20, 6]],
            31,
            [[34700, -40104], [-18252, 96706]],
            -32723,
            -98259,
        ),
        (
            [[10, 9, 5], [16, 24, 0]],
            34,
            [[16754, -1103], [-20086, 90391]],
            -30728,
            34808,
        ),
        (
            [[19, 10, 5], [6, 22, 2]],
            41,
            [[62411, -20229], [6015, 100021]],
            46005,
            -19531,
        ),
        (
            [[24, 6, 3], [5, 25, 1]],
            49,
            [[113820, -15696], [-14975, 93698]],
            46617,
            112153,
        ),
        (
            [[21, 6, 4], [6, 26, 1]],
            47,
            [[72143, 885], [-6526, 100836]],
            -2537,
            62999,
        ),
        (
            [[25, 9, 3], [4, 23, 0]],
            48,
            [[86869, -3538], [-12943, 51960]],
            46463,
            -19073,
        ),
        (
            [[27, 6, 2], [2, 25, 2]],
            52,
            [[57776, -9291], [-1938, 85317]],
            45929,
            111465,
        ),
        (
            [[25, 4, 1], [2, 31, 1]],
            56,
            [[83419, -11249], [-2322, 217311]],
            46667,
            112203,
        ),
        (
            [[32, 3, 2], [0, 26, 1]],
            58,
            [[198346, 1140], [0, 132910]],
            29191,
            94727,
        ),
        (
            [[19, 0, 1], [0, 44, 0]],
            63,
            [[18237, 0], [0, 303711]],
            46691,
            112227,
        ),
        (
            [[33, 0, 1], [0, 30, 0]],
            63,
            [[140201, 0], [0, 140195]],
            44671,
            110207,
        ),
        (
            [[27, 3, 0], [0, 34, 0]],
            61,
            [[126066, 212], [0, 177786]],
            46467,
            112003,
        ),
        (
            [[25, 0, 0], [0, 39, 0]],
            64,
            [[104680, 0], [0, 118670]],
            46691,
            112227,
        ),
        (
            [[25, 1, 0], [0, 38, 0]],
            63,
            [[97490, 398], [0, 113751]],
            46691,
            112227,
        ),
        (
            [[34, 0, 1], [0, 29, 0]],
            63,
            [[130028, 0], [0, 134982]],
            46467,
            112003,
        ),
        (
            [[35, 0, 0], [0, 29, 0]],
            64,
            [[122174, 0], [0, 72772]],
            46691,
            112227,
        ),
        (
            [[31, 0, 0], [0, 33, 0]],
            64,
            [[80872, 0], [0, 117938]],
            46691,
            112227,
        ),
        (
            [[35, 0, 0], [0, 29, 0]],
            64,
            [[142532, 0], [0, 120699]],
            46691,
            112227,
        ),
        (
            [[32, 0, 0], [0, 32, 0]],
            64,
            [[107533, 0], [0, 92695]],
            46691,
            112227,
        ),
        (
            [[34, 0, 0], [0, 30, 0]],
            64,
            [[73935, 0], [0, 119623]],
            46691,
            112227,
        ),
        (
            [[30, 0, 0], [0, 34, 0]],
            64,
            [[87378, 0], [0, 135800]],
            46691,
            112227,
        ),
        (
            [[36, 0, 0], [0, 28, 0]],
            64,
            [[99157, 0], [0, 50773]],
            46691,
            112227,
        ),
        (
            [[27, 0, 0], [0, 37, 0]],
            64,
            [[21325, 0], [0, 107053]],
            46691,
            112227,
        ),
        (
            [[34, 0, 0], [0, 30, 0]],
            64,
            [[45640, 0], [0, 106384]],
            46691,
            112227,
        ),
        (
            [[34, 0, 0], [0, 30, 0]],
            64,
            [[103804, 0], [0, 103316]],
            46691,
            112227,
        ),
        (
            [[30, 0, 0], [0, 34, 0]],
            64,
            [[43417, 0], [0, 117531]],
            46691,
            112227,
        ),
        (
            [[29, 0, 0], [0, 35, 0]],
            64,
            [[60491, 0], [0, 140019]],
            46691,
            112227,
        ),
        (
            [[31, 0, 0], [0, 33, 0]],
            64,
            [[98564, 0], [0, 81935]],
            46691,
            112227,
        ),
    ],
];
const PUNISHED_READ_1024: [u64; 2] = [0xb299ee60cde2ecbd, 0xfafd2eaa5834f3fb];
const PUNISHED_CENSUS_1024: [&[(u32, u64)]; 2] = [
    &[
        (0, 5),
        (1, 9829),
        (2, 74633),
        (3, 114416),
        (4, 33538),
        (5, 1449),
        (6, 209),
        (7, 73),
        (8, 22),
        (9, 15),
        (10, 6),
        (11, 5),
        (12, 1),
        (68, 1),
    ],
    &[
        (0, 5),
        (1, 9833),
        (2, 74508),
        (3, 114565),
        (4, 33409),
        (5, 1507),
        (6, 237),
        (7, 72),
        (8, 35),
        (9, 14),
        (10, 2),
        (11, 4),
        (12, 1),
        (14, 1),
        (71, 1),
    ],
];
const PUNISHED_MOVES_1024: [&[MovesBlock]; 2] = [
    &[
        [
            ([4762, 5040, 15508], [353551, -288978]),
            ([4641, 4573, 10848], [257653, -333807]),
        ],
        [
            ([5303, 5406, 17837], [356220, -290623]),
            ([4881, 5230, 10733], [206787, -285453]),
        ],
        [
            ([5905, 5546, 13220], [401929, -292215]),
            ([5182, 5254, 12036], [254549, -338060]),
        ],
        [
            ([6089, 5948, 15163], [425862, -331173]),
            ([3924, 3714, 12440], [216620, -240921]),
        ],
        [
            ([6045, 5840, 19088], [535511, -392544]),
            ([4671, 5044, 7945], [226220, -327706]),
        ],
        [
            ([7228, 7078, 22799], [683543, -487036]),
            ([2948, 3134, 5146], [115053, -144607]),
        ],
        [
            ([7030, 6801, 20344], [649309, -432743]),
            ([3189, 3283, 6376], [115341, -155897]),
        ],
        [
            ([7188, 6903, 28745], [768801, -552182]),
            ([1605, 1731, 2274], [52463, -63445]),
        ],
        [
            ([7255, 6529, 28073], [793002, -500476]),
            ([1663, 1812, 2933], [70760, -95231]),
        ],
        [
            ([7477, 7135, 33819], [926591, -647649]),
            ([645, 874, 891], [14322, -32163]),
        ],
        [
            ([7229, 6994, 35656], [921017, -666095]),
            ([38, 48, 712], [428, -533]),
        ],
        [
            ([6945, 6851, 36858], [876062, -661012]),
            ([0, 0, 0], [0, 0]),
        ],
        [
            ([6991, 6729, 36159], [870892, -631428]),
            ([352, 342, 104], [6156, -9490]),
        ],
        [
            ([7208, 7022, 34874], [895549, -662319]),
            ([94, 93, 619], [1889, -2126]),
        ],
        [
            ([7329, 6925, 35455], [906198, -674942]),
            ([357, 318, 123], [9081, -10031]),
        ],
        [
            ([7346, 6964, 34692], [908853, -644321]),
            ([63, 72, 663], [998, -1342]),
        ],
        [
            ([7493, 7072, 35280], [1020063, -741357]),
            ([0, 0, 0], [0, 0]),
        ],
        [
            ([7540, 7406, 35538], [993992, -727936]),
            ([0, 0, 0], [0, 0]),
        ],
        [
            ([7370, 7415, 35937], [1003104, -778886]),
            ([0, 0, 0], [0, 0]),
        ],
        [
            ([7452, 7163, 36039], [1016737, -742569]),
            ([0, 0, 0], [0, 0]),
        ],
        [
            ([7575, 7118, 36029], [977594, -756935]),
            ([0, 0, 0], [0, 0]),
        ],
        [
            ([7362, 6865, 36597], [950565, -714782]),
            ([0, 0, 0], [0, 0]),
        ],
        [
            ([7572, 7265, 35851], [987022, -839478]),
            ([0, 0, 0], [0, 0]),
        ],
        [
            ([7190, 6986, 36580], [954141, -807901]),
            ([0, 0, 0], [0, 0]),
        ],
        [
            ([38, 28, 743], [3616, -1941]),
            ([7484, 9340, 33191], [877271, -1607871]),
        ],
        [
            ([0, 0, 0], [0, 0]),
            ([7168, 8625, 34861], [773584, -1419194]),
        ],
        [
            ([0, 0, 0], [0, 0]),
            ([6927, 8455, 35170], [715654, -1303275]),
        ],
        [
            ([513, 448, 651], [12345, -8546]),
            ([7272, 8113, 32978], [748471, -1167464]),
        ],
        [
            ([349, 333, 2526], [5892, -4973]),
            ([6943, 7935, 31799], [661746, -1104760]),
        ],
        [
            ([1099, 1163, 938], [28739, -25784]),
            ([6949, 7463, 31456], [681190, -960478]),
        ],
        [
            ([2425, 2258, 2523], [112255, -81820]),
            ([6490, 7171, 25062], [570580, -835055]),
        ],
        [
            ([1550, 1497, 3377], [53946, -39945]),
            ([6999, 7740, 26649], [601784, -899823]),
        ],
        [
            ([2808, 2788, 5640], [100072, -81753]),
            ([6874, 7596, 21187], [522545, -731933]),
        ],
        [
            ([2770, 2877, 6371], [128376, -108345]),
            ([6553, 7227, 23529], [493229, -693608]),
        ],
        [
            ([4184, 3773, 8083], [177959, -141640]),
            ([6134, 6876, 20322], [466072, -623940]),
        ],
        [
            ([2728, 2812, 5664], [110041, -102436]),
            ([5668, 6072, 19940], [465273, -584025]),
        ],
        [
            ([2686, 2506, 9228], [144825, -110121]),
            ([6351, 6371, 20508], [503128, -619582]),
        ],
        [
            ([4640, 4754, 11482], [264285, -238213]),
            ([5436, 6092, 13918], [318031, -397569]),
        ],
        [
            ([5114, 5193, 15381], [313931, -276289]),
            ([4995, 5057, 12158], [293821, -328811]),
        ],
        [
            ([3859, 3961, 10630], [215855, -180761]),
            ([6645, 6823, 16696], [450744, -543734]),
        ],
        [
            ([6107, 5724, 10633], [361880, -290201]),
            ([5214, 5400, 13282], [311694, -379888]),
        ],
        [
            ([5783, 5407, 12846], [315797, -256183]),
            ([5513, 5793, 14038], [347708, -432071]),
        ],
        [
            ([6138, 5889, 14475], [414778, -313993]),
            ([4883, 5149, 10628], [246204, -311426]),
        ],
        [
            ([5015, 5142, 16329], [349401, -303347]),
            ([4601, 4921, 10397], [252359, -294427]),
        ],
        [
            ([6785, 6451, 17200], [509840, -355456]),
            ([4366, 4596, 7721], [195855, -231198]),
        ],
        [
            ([6891, 7020, 23779], [627377, -494881]),
            ([2897, 3057, 4495], [109613, -116373]),
        ],
        [
            ([6520, 6417, 16765], [569540, -419643]),
            ([3324, 3769, 11140], [167129, -193475]),
        ],
        [
            ([7381, 7404, 21317], [687554, -536749]),
            ([2606, 2693, 5891], [101070, -110702]),
        ],
        [
            ([7005, 7033, 24442], [717798, -563808]),
            ([1486, 1717, 2324], [55480, -71410]),
        ],
        [
            ([7052, 6913, 29391], [738975, -565542]),
            ([1260, 1458, 2843], [36248, -40855]),
        ],
        [
            ([7806, 7518, 28024], [856512, -612188]),
            ([2219, 2410, 2550], [68939, -83663]),
        ],
        [
            ([7582, 6961, 27169], [804232, -593703]),
            ([2006, 2368, 3648], [85033, -111475]),
        ],
        [
            ([7763, 7554, 32843], [983364, -684512]),
            ([753, 786, 854], [16719, -22795]),
        ],
        [
            ([7479, 7307, 27660], [892382, -666221]),
            ([2361, 2959, 1927], [71311, -102854]),
        ],
        [
            ([7833, 7489, 32822], [953864, -682912]),
            ([63, 74, 672], [501, -1212]),
        ],
        [
            ([7582, 7019, 31931], [870479, -646570]),
            ([994, 1298, 944], [37111, -58715]),
        ],
        [
            ([7637, 7325, 35512], [971930, -687645]),
            ([68, 88, 653], [912, -1479]),
        ],
        [
            ([7601, 7395, 34680], [907604, -652996]),
            ([437, 459, 722], [6911, -11405]),
        ],
        [
            ([8064, 7679, 34811], [1055517, -765775]),
            ([0, 0, 0], [0, 0]),
        ],
        [
            ([7351, 7170, 36839], [1003422, -781977]),
            ([0, 0, 0], [0, 0]),
        ],
        [
            ([8330, 8127, 34863], [1098502, -874303]),
            ([0, 0, 0], [0, 0]),
        ],
        [
            ([7943, 7665, 34938], [1025485, -822286]),
            ([352, 339, 118], [7063, -10728]),
        ],
        [
            ([7632, 7074, 36622], [990726, -735156]),
            ([0, 0, 0], [0, 0]),
        ],
        [
            ([7782, 6987, 36583], [1027025, -791867]),
            ([0, 0, 0], [0, 0]),
        ],
        [
            ([8518, 7970, 34824], [1202238, -885971]),
            ([0, 0, 0], [0, 0]),
        ],
        [
            ([8480, 7952, 34928], [1148383, -947269]),
            ([0, 0, 0], [0, 0]),
        ],
        [
            ([7885, 7351, 36044], [1059665, -808696]),
            ([0, 0, 0], [0, 0]),
        ],
        [
            ([7291, 6691, 37362], [974419, -726009]),
            ([0, 0, 0], [0, 0]),
        ],
        [
            ([8077, 7236, 36039], [1092076, -822719]),
            ([0, 0, 0], [0, 0]),
        ],
        [
            ([8233, 7706, 35373], [1203261, -867862]),
            ([0, 0, 0], [0, 0]),
        ],
        [
            ([8194, 7658, 35444], [1137669, -899425]),
            ([0, 0, 0], [0, 0]),
        ],
        [
            ([7809, 7378, 36133], [1112261, -894224]),
            ([0, 0, 0], [0, 0]),
        ],
    ],
    &[
        [
            ([4972, 5107, 12385], [345060, -270153]),
            ([4187, 3940, 14015], [260738, -305405]),
        ],
        [
            ([6190, 5718, 15368], [432803, -297942]),
            ([4344, 4322, 10478], [194468, -241628]),
        ],
        [
            ([5830, 5758, 17292], [456578, -316149]),
            ([4123, 4246, 6730], [212312, -254923]),
        ],
        [
            ([5590, 5543, 23357], [506577, -376741]),
            ([3320, 3427, 6870], [146851, -190169]),
        ],
        [
            ([7485, 7151, 22272], [733676, -476524]),
            ([2549, 2528, 6079], [124608, -156245]),
        ],
        [
            ([8001, 7512, 22225], [749481, -505636]),
            ([2656, 2600, 5866], [104060, -141931]),
        ],
        [
            ([7145, 7049, 31500], [811934, -598285]),
            ([778, 883, 1507], [34676, -52968]),
        ],
        [
            ([7620, 7008, 28680], [856017, -582971]),
            ([2197, 2226, 3531], [64173, -73100]),
        ],
        [
            ([7748, 6880, 29502], [944074, -577580]),
            ([814, 846, 3901], [13544, -19329]),
        ],
        [
            ([7771, 7525, 30414], [922342, -629335]),
            ([801, 930, 3898], [23256, -24150]),
        ],
        [
            ([6862, 6553, 33899], [813321, -568690]),
            ([965, 1052, 342], [23748, -35013]),
        ],
        [
            ([7324, 6837, 33983], [870611, -615896]),
            ([414, 416, 788], [8455, -11590]),
        ],
        [
            ([7243, 7006, 36281], [915046, -668957]),
            ([376, 369, 64], [7951, -11282]),
        ],
        [
            ([7180, 7264, 36086], [870882, -691861]),
            ([322, 352, 135], [6688, -7964]),
        ],
        [
            ([7553, 7068, 35151], [955885, -717272]),
            ([0, 0, 0], [0, 0]),
        ],
        [
            ([7198, 7035, 37119], [955186, -699840]),
            ([0, 0, 0], [0, 0]),
        ],
        [
            ([7557, 7160, 36627], [986699, -758648]),
            ([0, 0, 0], [0, 0]),
        ],
        [
            ([7599, 7546, 35433], [974702, -760461]),
            ([388, 303, 118], [8246, -9239]),
        ],
        [
            ([7682, 7252, 36386], [991229, -748115]),
            ([0, 0, 0], [0, 0]),
        ],
        [
            ([7701, 7220, 36415], [1012654, -760579]),
            ([0, 0, 0], [0, 0]),
        ],
        [
            ([7805, 7389, 36126], [1020785, -759741]),
            ([0, 0, 0], [0, 0]),
        ],
        [
            ([7274, 7113, 36909], [967638, -776368]),
            ([0, 0, 0], [0, 0]),
        ],
        [
            ([7663, 7220, 36445], [1049142, -778638]),
            ([0, 0, 0], [0, 0]),
        ],
        [
            ([7387, 7005, 36920], [972413, -787201]),
            ([0, 0, 0], [0, 0]),
        ],
        [
            ([39, 35, 724], [5862, -1454]),
            ([7599, 9221, 33678], [854218, -1602629]),
        ],
        [
            ([0, 0, 0], [0, 0]),
            ([7265, 8919, 35152], [794897, -1420156]),
        ],
        [
            ([0, 0, 0], [0, 0]),
            ([7023, 8688, 35649], [787869, -1420747]),
        ],
        [
            ([1202, 1311, 1498], [35907, -34674]),
            ([7055, 8306, 31969], [663020, -1193130]),
        ],
        [
            ([382, 386, 782], [9111, -8964]),
            ([7246, 8282, 34204], [718284, -1093427]),
        ],
        [
            ([83, 75, 651], [1038, -1025]),
            ([7352, 7402, 32584], [739547, -963737]),
        ],
        [
            ([800, 854, 1480], [19352, -16933]),
            ([7103, 7524, 30317], [661423, -923568]),
        ],
        [
            ([3478, 3175, 4537], [144044, -104816]),
            ([7235, 7252, 24017], [591337, -752817]),
        ],
        [
            ([3153, 2673, 4453], [96155, -71917]),
            ([6659, 7146, 23893], [597117, -781497]),
        ],
        [
            ([3997, 3541, 5236], [183756, -130346]),
            ([6466, 6629, 19783], [494216, -610683]),
        ],
        [
            ([2245, 1995, 2939], [71915, -45470]),
            ([6365, 7122, 25815], [550340, -740276]),
        ],
        [
            ([3876, 3715, 3497], [176745, -138540]),
            ([6437, 6665, 26200], [501513, -580573]),
        ],
        [
            ([4630, 4654, 8208], [282785, -232314]),
            ([5256, 5812, 19416], [394240, -512969]),
        ],
        [
            ([4900, 4777, 6972], [186708, -156716]),
            ([5558, 5752, 20786], [412763, -484396]),
        ],
        [
            ([3460, 3632, 9489], [194780, -167601]),
            ([5378, 5524, 17164], [396028, -464089]),
        ],
        [
            ([6000, 5484, 10794], [384446, -287282]),
            ([5138, 5176, 13754], [374165, -414693]),
        ],
        [
            ([5751, 5715, 10880], [378666, -292769]),
            ([4357, 4696, 14225], [309702, -373283]),
        ],
        [
            ([5574, 5519, 9499], [363196, -265359]),
            ([4769, 5019, 15062], [345820, -381575]),
        ],
        [
            ([5137, 4933, 11297], [386679, -269001]),
            ([4795, 4868, 16001], [294364, -358989]),
        ],
        [
            ([6189, 5951, 14047], [453859, -329834]),
            ([4120, 4315, 12401], [272137, -300285]),
        ],
        [
            ([6449, 6066, 12190], [456695, -325289]),
            ([4650, 5150, 8650], [256626, -314982]),
        ],
        [
            ([5998, 5706, 14653], [415409, -308264]),
            ([4594, 4651, 11583], [316060, -337249]),
        ],
        [
            ([7221, 6734, 19377], [603546, -441114]),
            ([3572, 3733, 5543], [164141, -178355]),
        ],
        [
            ([7052, 7311, 23687], [731146, -523628]),
            ([2655, 2772, 3399], [152778, -183449]),
        ],
        [
            ([7278, 7203, 22828], [725592, -552613]),
            ([2455, 2353, 4816], [109934, -115575]),
        ],
        [
            ([6566, 6560, 25631], [633295, -494466]),
            ([2289, 2429, 4930], [99762, -116243]),
        ],
        [
            ([7694, 7668, 24979], [799976, -656883]),
            ([1430, 1733, 4067], [45697, -56926]),
        ],
        [
            ([8144, 7479, 28831], [949543, -648813]),
            ([1309, 1768, 1743], [40691, -54262]),
        ],
        [
            ([8017, 7596, 30221], [1004858, -673602]),
            ([527, 527, 1364], [14637, -13497]),
        ],
        [
            ([8214, 7329, 34778], [1071028, -749080]),
            ([0, 0, 0], [0, 0]),
        ],
        [
            ([7946, 7336, 34597], [1046328, -765932]),
            ([0, 0, 0], [0, 0]),
        ],
        [
            ([7216, 6835, 34346], [944531, -640679]),
            ([529, 488, 1401], [13382, -13170]),
        ],
        [
            ([7524, 7316, 36086], [997008, -773658]),
            ([0, 0, 0], [0, 0]),
        ],
        [
            ([7564, 7272, 35315], [980565, -769324]),
            ([76, 69, 661], [1886, -1488]),
        ],
        [
            ([7763, 7526, 34522], [986120, -721110]),
            ([0, 0, 0], [0, 0]),
        ],
        [
            ([7332, 7019, 36201], [950950, -756004]),
            ([0, 0, 0], [0, 0]),
        ],
        [
            ([8229, 7687, 34806], [1061339, -862529]),
            ([0, 0, 0], [0, 0]),
        ],
        [
            ([8157, 7514, 34949], [1103679, -840448]),
            ([0, 0, 0], [0, 0]),
        ],
        [
            ([7482, 7320, 35886], [1069354, -869126]),
            ([0, 0, 0], [0, 0]),
        ],
        [
            ([7598, 7007, 35981], [1021108, -827550]),
            ([0, 0, 0], [0, 0]),
        ],
        [
            ([8181, 7725, 34850], [1094817, -871639]),
            ([0, 0, 0], [0, 0]),
        ],
        [
            ([7844, 7539, 35169], [1028216, -878286]),
            ([0, 0, 0], [0, 0]),
        ],
        [
            ([7814, 7572, 35506], [1060142, -931764]),
            ([0, 0, 0], [0, 0]),
        ],
        [
            ([7098, 6903, 36619], [898134, -746110]),
            ([0, 0, 0], [0, 0]),
        ],
        [
            ([7586, 7186, 35814], [1015279, -808159]),
            ([0, 0, 0], [0, 0]),
        ],
        [
            ([7898, 7333, 35525], [1084466, -923518]),
            ([0, 0, 0], [0, 0]),
        ],
        [
            ([8193, 7580, 35051], [1127668, -927158]),
            ([0, 0, 0], [0, 0]),
        ],
        [
            ([7609, 7146, 35967], [1042856, -862357]),
            ([0, 0, 0], [0, 0]),
        ],
    ],
];
/// The one signed image both arms decode, its CRC-64.
const PUNISHED_IMAGE_CRC_1024: u64 = 0x3771636d385191ac;
/// The four couplings at the end of the 1 537th trial, per arm, as read.
const PUNISHED_CARRY_1024: [[[i64; 2]; 2]; 2] = [
    [[8621458, 6428657], [6361667, 9355767]],
    [[6136682, 9650801], [9027575, 6627178]],
];
/// Clause 1's and clause 2's counts per arm, `[before the flip, the run's last 128]`, against
/// `REWARDED_MIN`.
const CORRECT_PUNISHED_1024: [[u32; 2]; 2] = [[128, 128], [128, 128]];
/// The assertion's reach per arm, as read.
const REACH_PUNISHED_1024: [Reach; 2] = [
    Reach {
        excitatory: (3188, 0),
        inhibitory: (0, 6508),
    },
    Reach {
        excitatory: (3188, 0),
        inhibitory: (0, 6498),
    },
];
/// H-17's measured need, read after the flip, per arm.
const NEED_PUNISHED_1024: [Need; 2] = [
    Need {
        selected_new: 1939,
        selected_old: 1022,
        ties: 111,
        rewards: 1939,
        crossed: Some(2944),
    },
    Need {
        selected_new: 1909,
        selected_old: 1044,
        ties: 119,
        rewards: 1909,
        crossed: Some(3008),
    },
];
/// The first trial after the flip that earned a reward, per arm, as read.
const FIRST_REWARD_PUNISHED_1024: [Option<usize>; 2] = [Some(1762), Some(1747)];
/// Per arm, per stimulus, the first trial after the flip that selected the new answer.
const FIRST_NEW_1024: [[Option<usize>; 2]; 2] =
    [[Some(1762), Some(1815)], [Some(1747), Some(1760)]];
/// Per arm, per stimulus, the block in which the selection crossed to the new answer.
const CROSSED_BLOCK_1024: [[Option<usize>; 2]; 2] = [[Some(37), Some(41)], [Some(43), Some(39)]];
/// ADR-0093's predicted readings as read, per arm, per stimulus: (a), (b) and (c).
const OLD_FALLS_1024: [[bool; 2]; 2] = [[true, true], [true, true]];
const FALL_SLOWS_1024: [[bool; 2]; 2] = [[true, true], [true, true]];
const WRONG_BELOW_1024: [[bool; 2]; 2] = [[true, true], [true, true]];
/// Per arm, the moves after a punishment and after a reward, `[before the flip, after it]`.
const PUNISHED_MOVES_TOTAL_1024: [[Moves; 2]; 2] = [
    [
        ([34253, 35522, 73843], [1548320, -2040812]),
        ([150291, 166290, 494271], [11260983, -16407273]),
    ],
    [
        ([28234, 28440, 64322], [1233774, -1534936]),
        ([147262, 159991, 530227], [11968594, -16758715]),
    ],
];
const REWARDED_MOVES_TOTAL_1024: [[Moves; 2]; 2] = [
    [
        ([166884, 161001, 716141], [19176108, -14265580]),
        ([272312, 261003, 1021858], [29336093, -22352656]),
    ],
    [
        ([172380, 164919, 736775], [20114735, -14723583]),
        ([272563, 259893, 981348], [29278652, -22704118]),
    ],
];
/// The new answer's pairs' rise from the flip to the run's end, `[A, B]`, per arm.
const NEW_RISE_PUNISHED_1024: [[i64; 2]; 2] = [[3953274, 3028488], [2674728, 3895398]];
/// The blocks, of seventy-two, in which the stimulus fired once, per arm.
const ONCE_BLOCKS_PUNISHED_1024: [u32; 2] = [72, 72];
/// Whether the inhibitory sum fell in every block of the run, per arm.
const FALLS_PUNISHED_1024: [bool; 2] = [false, false];
/// ADR-0080's derivation as read over each arm's whole run, clause by clause.
const DERIVATION_PUNISHED_1024: [[bool; 3]; 2] = [[true, true, false], [true, true, false]];
/// The arena's sums by polarity after each arm's run, `(inhibitory, excitatory)`.
const SUMS_AFTER_PUNISHED_1024: [(i64, i64); 2] = [(18207853, 224498537), (19024488, 225117757)];

/// The verdict, by the rule committed first, over the pinned tables: clause 1 held in both
/// arms, 128 of 128 before the flip, and clause 2 in both, 128 of the last 128 at the run's
/// end. H-18 is yes; ADR-0093 wrote no prediction for it.
const PUNISHED_1024: Punished = Punished {
    learned: [true, true],
    revised: [true, true],
    yes: true,
};

// =================================================================================== H-19

// ------------------------------------------ written before the run (ADR-0106, ADR-0107)

/// H-19's run (ADR-0106): H-18's, 4 608 trials in seventy-two blocks, the flip where H-18's
/// was, before the trial of index 1 536. Neither moves after a rewarded run.
const CRITIC_TRIALS: usize = PUNISHED_TRIALS;
/// The blocks of an H-19 run.
const CRITIC_BLOCKS: usize = PUNISHED_BLOCKS;
const _: () = assert!(CRITIC_TRIALS == 4_608 && CRITIC_BLOCKS == 72 && FLIP == 1_536);

/// The arms of H-19 (ADR-0106): H-18's two, in their order, each its own weekly test — the
/// assignment first and the mirrored first — from H-18's one signed image with the critic set,
/// the flip `Task::mirrored` negated and nothing else, the critic's expectations carried across
/// it.
const CRITIC_ARMS: [Reversal; 2] = PUNISHED_ARMS;

/// The critic's shift (ADR-0106): an expectation moves by a thirty-second of the error, so it
/// forgets over about thirty-two presentations of its stimulus, one block of trials. It does
/// not move after a rewarded run.
const CRITIC_SHIFT: u32 = 5;

/// The critic at the run's start (ADR-0106): both expectations zero.
const CRITIC_AT_START: Critic = Critic::new(CRITIC_SHIFT);

/// ADR-0106 writes no prediction for the verdict.
const CRITIC_PREDICTED: Option<bool> = None;

/// Clause 3's span (ADR-0106): the last four blocks of each mapping, 256 trials — trials 1 281
/// to 1 536 under the first mapping, 4 353 to 4 608 under the second — over which H-18's
/// answer pairs rose by 3.7 to 9.5 per cent of their image couplings.
const SETTLE_BLOCKS: usize = 4;
/// Clause 3's bound (ADR-0106): a move of less than one per cent of the pair's image coupling,
/// read in integers as `|Δ| × SETTLE_PER_CENT < image`.
const SETTLE_PER_CENT: i64 = 100;
const _: () = assert!(SETTLE_BLOCKS * BLOCK == 256 && FLIP_BLOCK >= SETTLE_BLOCKS);
const _: () = assert!(CRITIC_BLOCKS - FLIP_BLOCK >= SETTLE_BLOCKS);

/// ADR-0106's predicted reading (1), a Hypothesis written before the run and never asserted:
/// for both stimuli in both arms, the first mapping's answer pair at the flip stands below
/// H-18's same pair in the same arm at the flip (`below_punished`).
const BELOW_PUNISHED_PREDICTED: bool = true;

/// ADR-0106's predicted reading (2), a Hypothesis written before the run and never asserted:
/// for both stimuli in both arms, the stimulus's expectation is below zero after a trial within
/// `FALLS_WITHIN` trials of the flip, trials 1 537 to 1 600 (`falls_within`). ADR-0106's rule,
/// from an expectation of 1.0 punished at every presentation, says after 22 presentations.
const EXPECTATION_FALLS_PREDICTED: bool = true;
/// The trials from the flip the predicted reading (2) reads.
const FALLS_WITHIN: usize = BLOCK;
const _: () = assert!(FALLS_WITHIN == 64);

/// The signal at or below which a trial's delivery is counted as a strong punishment of the old
/// answer's pair (ADR-0106's reading): −0.5, the modulation the pair starts consolidating under
/// in the next trial, the baseline being zero.
const STRONG_PUNISHMENT_Q16: i32 = -(ONE / 2);

// ------------------------------------------------------------ the criterion (ADR-0106)

/// Clause 3's moves (ADR-0106), per mapping `[first, second]` and per stimulus `[A, B]`: the
/// stimulus's answer pair under that mapping, its coupling at the end of the mapping's last
/// block less its coupling `SETTLE_BLOCKS` blocks before — over trials 1 281 to 1 536 under
/// the first mapping and 4 353 to 4 608 under the second; none for a run of any other length.
fn settle_moves(blocks: &[Block], first: bool) -> Option<[[i64; 2]; 2]> {
    if blocks.len() != CRITIC_BLOCKS {
        return None;
    }
    let ends = [FLIP_BLOCK - 1, CRITIC_BLOCKS - 1];
    let mappings = [first, !first];
    Some([0usize, 1].map(|m| {
        [0usize, 1].map(|s| {
            let answer = answer_of(s as u8, mappings[m]);
            let at = |j: usize| blocks.get(j).map_or(0, |b| b.10[s][answer]);
            at(ends[m]).saturating_sub(at(ends[m].saturating_sub(SETTLE_BLOCKS)))
        })
    }))
}

/// Clause 3 as a rule (ADR-0106): each of the four moves less than one per cent of its pair's
/// image coupling, `|Δ| × 100 < image`, whichever way it moved; false for a run of any other
/// length.
fn settles(blocks: &[Block], first: bool) -> bool {
    let Some(moves) = settle_moves(blocks, first) else {
        return false;
    };
    moves.iter().zip([first, !first]).all(|(pairs, mapping)| {
        pairs.iter().enumerate().all(|(s, &delta)| {
            let image = IMAGE_COUPLINGS_1024[s][answer_of(s as u8, mapping)];
            delta.saturating_abs().saturating_mul(SETTLE_PER_CENT) < image
        })
    })
}

/// H-19's criterion (ADR-0106), clause by clause per arm `[assignment first, mirrored first]`:
/// (1) it learned — `correct_before`, trials 1 409 to 1 536 under the first mapping, at least
/// `REWARDED_MIN`; (2) it revised — `revised_count`, trials 4 481 to 4 608 under the second, at
/// least `REWARDED_MIN`; (3) it settled — `settles`; a tie not correct, the task's count. `yes`
/// is all six; a no names the clause that failed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Predicted {
    learned: [bool; 2],
    revised: [bool; 2],
    settled: [bool; 2],
    yes: bool,
}

fn predicted(arms: [&[Block]; 2]) -> Predicted {
    let learned = arms.map(|blocks| correct_before(blocks) >= REWARDED_MIN);
    let revised = arms.map(|blocks| revised_count(blocks) >= REWARDED_MIN);
    let settled = [0usize, 1].map(|k| settles(arms[k], first_mapping(CRITIC_ARMS[k])));
    Predicted {
        learned,
        revised,
        settled,
        yes: learned.iter().chain(&revised).chain(&settled).all(|&c| c),
    }
}

// --------------------------------------------------- the readings' shape (ADR-0106)

/// ADR-0106's predicted reading (1) as a rule: per stimulus, its answer pair under the first
/// mapping at the end of the 1 536th trial below the same pair at the same trial of `punished`,
/// H-18's arm of the same first mapping; false for a run that does not reach the flip.
fn below_punished(blocks: &[Block], punished: &[Block], first: bool) -> [bool; 2] {
    let (Some(here), Some(there)) = (blocks.get(FLIP_BLOCK - 1), punished.get(FLIP_BLOCK - 1))
    else {
        return [false; 2];
    };
    [0usize, 1].map(|s| {
        let answer = answer_of(s as u8, first);
        here.10[s][answer] < there.10[s][answer]
    })
}

/// Per stimulus, the first trial from the flip on, by index, after which its expectation was
/// below zero; none when none was.
fn below_zero(expected: &[[i32; 2]]) -> [Option<usize>; 2] {
    [0usize, 1].map(|s| {
        expected
            .iter()
            .enumerate()
            .skip(FLIP)
            .find(|(_, e)| e[s] < 0)
            .map(|(t, _)| t)
    })
}

/// ADR-0106's predicted reading (2) as a rule: per stimulus, `below_zero` within
/// `FALLS_WITHIN` trials of the flip, before the trial of index `FLIP + FALLS_WITHIN`.
fn falls_within(expected: &[[i32; 2]]) -> [bool; 2] {
    below_zero(expected).map(|t| t.is_some_and(|t| t < FLIP + FALLS_WITHIN))
}

/// Per stimulus, its presentations from the flip up to and including the trial `below_zero`
/// names — ADR-0106's arithmetic says 22 from an expectation of 1.0 punished at every one; none
/// when its expectation never fell below zero. A reading.
fn presentations_to_below(read: &[EarnedTrial], expected: &[[i32; 2]]) -> [Option<u32>; 2] {
    let at = below_zero(expected);
    [0usize, 1].map(|s| {
        at[s].map(|t| {
            read.get(FLIP..=t)
                .unwrap_or(&[])
                .iter()
                .filter(|r| usize::from(r.0) == s)
                .count() as u32
        })
    })
}

/// The punishment's course after the flip (ADR-0106's reading), per block from the flip and
/// per stimulus `[A, B]`: the trials that selected the stimulus's old answer — its answer under
/// the first mapping, an error under the second — and left a signal at or below
/// `STRONG_PUNISHMENT_Q16`, the modulation under which the old answer's pair starts
/// consolidating in the next trial, the baseline being zero. The blocks before the flip are not
/// read.
fn strong_punishments(read: &[EarnedTrial], first: bool) -> Vec<[u32; 2]> {
    read.get(FLIP..)
        .unwrap_or(&[])
        .chunks(BLOCK)
        .map(|block| {
            let mut out = [0u32; 2];
            for t in block {
                let old = answer_of(t.0, first) as u8;
                if t.2 == Some(old) && t.7 <= STRONG_PUNISHMENT_Q16 {
                    let into = &mut out[usize::from(t.0)];
                    *into = into.saturating_add(1);
                }
            }
            out
        })
        .collect()
}

/// `strong_punishments` summed over the blocks, per stimulus.
fn strong_total(blocks: &[[u32; 2]]) -> [u32; 2] {
    blocks.iter().fold([0u32; 2], |sum, b| {
        [sum[0].saturating_add(b[0]), sum[1].saturating_add(b[1])]
    })
}

/// ADR-0107's arithmetic of the punishment's fading with the dopamine signal's carry-over
/// (F-53), from the rules and not a reading: both expectations at 1.0 and the signal at rest,
/// every trial a punished presentation of the stimuli in turn — the course ADR-0106's
/// $delta_n = -2(31/32)^n$ describes — the error each trial delivers (`critic_step`) and the
/// signal it leaves, the error plus what the deliveries before it left, decayed over a trial
/// by the executor's rule (`signal_course`).
fn carried_punishment(trials: usize) -> Vec<(i32, i32)> {
    let mut expected = [ONE; 2];
    let mut signal = 0i32;
    (0..trials)
        .map(|t| {
            let s = t & 1;
            let (error, after) = critic_step(expected[s], ONE.saturating_neg(), CRITIC_SHIFT);
            expected[s] = after;
            signal = signal_end(&signal_course(signal)).saturating_add(error);
            (error, signal)
        })
        .collect()
}

/// Each stimulus's expectation at every block's end, `[A, B]`, from the trials' expectations.
fn expected_blocks(expected: &[[i32; 2]]) -> Vec<[i32; 2]> {
    expected
        .chunks(BLOCK)
        .filter_map(|block| block.last().copied())
        .collect()
}

/// The four couplings after every block as fractions of the image's, in parts per ten
/// thousand, `[stimulus][readout]`: the course the dump reads beside H-18's.
fn couplings_course(blocks: &[Block]) -> Vec<[[i64; 2]; 2]> {
    blocks
        .iter()
        .map(|b| {
            [0usize, 1]
                .map(|s| [0usize, 1].map(|r| per_myriad(b.10[s][r], IMAGE_COUPLINGS_1024[s][r])))
        })
        .collect()
}

// ---------------------------------------------------------------- the run (brief 046)

/// An arm's run from the signed engine (brief 046): `earned_run_predicted` under the answer's
/// feedback at the gate's zero with the signed gate set, the arm's first mapping, the flip
/// before the trial of index `flip`, the critic given, and `after` reading the executor at
/// every trial's end — the oracle, fed the reward each trial delivered, held to the record at
/// every trial; the task's error and expectations held to the harness's critic; every trial's
/// contract asserted under the mapping in force at it. With no critic it is `punished_run`'s
/// run.
fn critic_run(
    exec: &mut Engine,
    arm: Reversal,
    trials: usize,
    flip: usize,
    critic: Option<Critic>,
    after: &mut dyn FnMut(&Engine, usize),
) -> (EarnedRun, Vec<Moves>, Vec<[i32; 2]>) {
    earned_run_predicted(
        exec,
        Feedback::Answer,
        first_mapping(arm),
        1024,
        trials,
        GATE_BASELINE_Q16,
        Some(flip),
        true,
        critic,
        after,
    )
}

/// One arm of H-19 at 1 024 units (brief 046): the settled engine held to ADR-0077 step by step
/// and its images, the signed image H-18's by its CRC; the calibration before any rewarded run
/// (H-19's stopping rule, step 2) — a frozen block from the zero image, the inhibitory baseline
/// and the signed gate unset, held to ADR-0077's frozen run, and H-18's arm's first block from
/// the signed image with the critic unset, held to H-18's tables; then the arm's 4 608 trials
/// from the signed image with the critic set, the flip between the 1 536th and the 1 537th, the
/// couplings read at the end of the 1 537th; everything dumped and the clauses, the assertion
/// and the readings computed before anything is held; then the assertion, and the pinned
/// tables.
fn critic_arm(arm: Reversal) {
    let k = CRITIC_ARMS
        .iter()
        .position(|&a| a == arm)
        .expect("an arm of H-19");
    let name = format!("critic1024 {arm:?}");
    let (zero, signed) = signed_images(&name);
    let image_crc = crc64(&signed);
    assert_eq!(
        image_crc, PUNISHED_IMAGE_CRC_1024,
        "{name}: H-18's one image"
    );
    {
        let mut frozen = frozen_from(&zero, 1024);
        assert_eq!(
            (frozen.inhibitory_baseline_q16(), frozen.signed_gate()),
            (None, false),
            "{name}: the calibration's image leaves the inhibitory baseline and the signed gate unset"
        );
        let calibration = taught_run(&mut frozen, Arm::Withheld, 1024, BLOCK);
        calibration_holds(&format!("{name} calibration"), &calibration);
    }
    {
        let mut exec = signed_from(&signed, 1024);
        let (run, moves) = punished_run(&mut exec, arm, BLOCK, FLIP, &mut |_, _| {});
        let (blocks, _, trials, read, _) = &run;
        let compositions: Vec<Composition> = trials.chunks(BLOCK).map(composition).collect();
        assert_eq!(
            blocks.as_slice(),
            &PUNISHED_BLOCKS_1024[k][..1],
            "{name}: H-18's first block, the critic unset"
        );
        assert_eq!(
            compositions.as_slice(),
            &PUNISHED_COMPOSITIONS_1024[k][..1],
            "{name}: and its composition"
        );
        assert_eq!(
            earned_blocks(read).as_slice(),
            &PUNISHED_EARNED_1024[k][..1],
            "{name}: and its earned block"
        );
        assert_eq!(
            moves_blocks(read, &moves).as_slice(),
            &PUNISHED_MOVES_1024[k][..1],
            "{name}: and its moves"
        );
        eprintln!(
            "DUMP {name} calibration holds: ADR-0077's settled candidate, H-18's image (crc {image_crc:#018x}) and H-18's first block reproduced"
        );
    }
    let sets = geometry(1024, ROTATION_1024);
    let image_sums = QUIET_1024[SETTLED].1;
    let mut exec = signed_from(&signed, 1024);
    assert_eq!(
        weights_by_polarity(&exec),
        image_sums,
        "{name}: the image's sums"
    );
    assert_eq!(
        pair_couplings(&exec, &sets),
        IMAGE_COUPLINGS_1024,
        "{name}: the same image"
    );
    let image = weights_of(&exec);
    let first = first_mapping(arm);
    let mut at_carry: Option<[[i64; 2]; 2]> = None;
    let (run, moves, expected) = critic_run(
        &mut exec,
        arm,
        CRITIC_TRIALS,
        FLIP,
        Some(CRITIC_AT_START),
        &mut |exec, trial| {
            if trial == FLIP {
                at_carry = Some(pair_couplings(exec, &sets));
            }
        },
    );
    let (blocks, trace, trials, read, volley_ticks) = &run;
    let at_carry = at_carry.expect("the run reached the trial after the flip");
    let earned = earned_blocks(read);
    let compositions: Vec<Composition> = trials.chunks(BLOCK).map(composition).collect();
    let moved = moves_blocks(read, &moves);
    let expected_by_block = expected_blocks(&expected);
    let strong = strong_punishments(read, first);
    assert_eq!(blocks.len(), CRITIC_BLOCKS, "{name}: seventy-two blocks");
    assert_eq!((read.len(), expected.len()), (CRITIC_TRIALS, CRITIC_TRIALS));
    assert_eq!(
        (moved.len(), expected_by_block.len(), strong.len()),
        (CRITIC_BLOCKS, CRITIC_BLOCKS, CRITIC_BLOCKS - FLIP_BLOCK)
    );
    // Everything dumped, and the clauses, the assertion and the readings computed, before
    // anything is held.
    dump_earned(&name, &run, &earned);
    eprintln!("DUMP {name} PIN blocks {blocks:?}");
    eprintln!("DUMP {name} PIN trace {trace:#018x}");
    eprintln!("DUMP {name} PIN compositions {compositions:?}");
    eprintln!("DUMP {name} PIN earned {earned:?}");
    eprintln!("DUMP {name} PIN read {:#018x}", earned_hash(read));
    eprintln!("DUMP {name} PIN census {:?}", census_of(volley_ticks));
    eprintln!("DUMP {name} PIN moves {moved:?}");
    eprintln!("DUMP {name} PIN expected {expected_by_block:?}");
    eprintln!("DUMP {name} PIN strong {strong:?}");
    eprintln!("DUMP {name} PIN carry {at_carry:?}");
    let reach = reach_by_polarity(&exec, &image, 1024, &ALL_PAIRS);
    let correct = [correct_before(blocks), revised_count(blocks)];
    let settle = settle_moves(blocks, first);
    let settled = settles(blocks, first);
    let need_read = need(blocks, &earned);
    let first_rewarded = first_reward(read);
    let first_new_read = first_new(read, first);
    let crossed = crossed_block(&earned, first);
    let below = below_punished(blocks, PUNISHED_BLOCKS_1024[k], first);
    let below_zero_read = below_zero(&expected);
    let within = falls_within(&expected);
    let to_below = presentations_to_below(read, &expected);
    let strong_sum = strong_total(&strong);
    let old_falls_read = old_falls(blocks, &earned, first);
    let fall_slows_read = fall_slows(blocks, &earned, first);
    let wrong_below_read = wrong_below(blocks, first);
    let punished_moves = moves_totals(&moved, 1);
    let rewarded_moves = moves_totals(&moved, 0);
    let new_rise_read = new_rise(blocks, first);
    let once = once_blocks(blocks, &compositions);
    let falls = falls_every_block(image_sums.0, blocks);
    let derivation_read = derivation(read);
    let sums_after = weights_by_polarity(&exec);
    eprintln!(
        "DUMP {name} PIN readings correct {correct:?} settle {settle:?} settled {settled} reach {reach:?} need {need_read:?} first reward {first_rewarded:?} first new {first_new_read:?} crossed {crossed:?} below punished {below:?} (predicted {BELOW_PUNISHED_PREDICTED}) below zero {below_zero_read:?} within {within:?} (predicted {EXPECTATION_FALLS_PREDICTED}) to below {to_below:?} strong total {strong_sum:?} old falls {old_falls_read:?} slows {fall_slows_read:?} wrong below {wrong_below_read:?} punished moves {punished_moves:?} rewarded moves {rewarded_moves:?} new rise {new_rise_read:?} once {once} falls {falls} derivation {derivation_read:?} sums after {sums_after:?} image crc {image_crc:#018x}"
    );
    eprintln!(
        "DUMP {name} couplings course {:?} beside H-18's {:?}",
        couplings_course(blocks),
        couplings_course(PUNISHED_BLOCKS_1024[k])
    );
    eprintln!(
        "DUMP {name} old course {:?} gaps {:?} course {:?} image {image_sums:?} last splits {:?}",
        old_course(blocks, first),
        gaps(blocks, first),
        course(image_sums.0, blocks),
        last_splits(read)
    );
    // The assertion (ADR-0106), after the dump and beside the verdict: H-18's rule.
    assert!(
        punished_held(&reach),
        "{name}: ADR-0106's assertion — no excitatory synapse outside the four stimulus–readout pairs moved: {reach:?}"
    );
    // The pinned tables, and the readings as the constants state.
    pinned(
        &format!("{name} sight"),
        blocks,
        *trace,
        CRITIC_BLOCKS_1024[k],
        CRITIC_TRACES_1024[k],
    );
    assert_eq!(
        compositions.as_slice(),
        CRITIC_COMPOSITIONS_1024[k],
        "{name}: the composition per block"
    );
    assert_eq!(
        earned.as_slice(),
        CRITIC_EARNED_1024[k],
        "{name}: the earned blocks"
    );
    assert_eq!(
        earned_hash(read),
        CRITIC_READ_1024[k],
        "{name}: the readings"
    );
    assert_eq!(
        census_of(volley_ticks),
        CRITIC_CENSUS_1024[k].to_vec(),
        "{name}: the volley's ticks"
    );
    assert_eq!(
        moved.as_slice(),
        CRITIC_MOVES_1024[k],
        "{name}: the moves per block"
    );
    assert_eq!(
        expected_by_block.as_slice(),
        CRITIC_EXPECTED_1024[k],
        "{name}: the expectations per block"
    );
    assert_eq!(
        strong.as_slice(),
        CRITIC_STRONG_1024[k],
        "{name}: the strong punishments per block"
    );
    assert_eq!(at_carry, CRITIC_CARRY_1024[k]);
    assert_eq!(correct, CORRECT_CRITIC_1024[k]);
    assert_eq!(settle, SETTLE_MOVES_1024[k]);
    assert_eq!(settled, SETTLED_1024[k]);
    assert_eq!(reach, REACH_CRITIC_1024[k]);
    assert_eq!(need_read, NEED_CRITIC_1024[k]);
    assert_eq!(first_rewarded, FIRST_REWARD_CRITIC_1024[k]);
    assert_eq!(first_new_read, FIRST_NEW_CRITIC_1024[k]);
    assert_eq!(crossed, CROSSED_CRITIC_1024[k]);
    assert_eq!(below, BELOW_PUNISHED_1024[k]);
    assert_eq!(below_zero_read, BELOW_ZERO_1024[k]);
    assert_eq!(within, FALLS_WITHIN_1024[k]);
    assert_eq!(to_below, TO_BELOW_1024[k]);
    assert_eq!(strong_sum, STRONG_TOTAL_1024[k]);
    assert_eq!(old_falls_read, OLD_FALLS_CRITIC_1024[k]);
    assert_eq!(fall_slows_read, FALL_SLOWS_CRITIC_1024[k]);
    assert_eq!(wrong_below_read, WRONG_BELOW_CRITIC_1024[k]);
    assert_eq!(punished_moves, PUNISHED_MOVES_CRITIC_1024[k]);
    assert_eq!(rewarded_moves, REWARDED_MOVES_CRITIC_1024[k]);
    assert_eq!(new_rise_read, NEW_RISE_CRITIC_1024[k]);
    assert_eq!(once, ONCE_BLOCKS_CRITIC_1024[k]);
    assert_eq!(falls, FALLS_CRITIC_1024[k]);
    assert_eq!(derivation_read, DERIVATION_CRITIC_1024[k]);
    assert_eq!(
        (sums_after, blocks.last().map(|b| (b.7, b.8))),
        (SUMS_AFTER_CRITIC_1024[k], Some(SUMS_AFTER_CRITIC_1024[k])),
        "{name}: the sums after the run are the last block's"
    );
}

/// H-19's arm that starts from the assignment (brief 046): A onto readout 0 and B onto readout
/// 1 for 1 536 trials, then the mirrored mapping for 3 072, the signed gate and the critic set.
#[test]
#[ignore]
fn the_critic_from_the_assignment_at_1024_units_exhaustive() {
    critic_arm(Reversal::AssignmentFirst);
}

/// H-19's arm that starts from the mirrored assignment (brief 046): A onto readout 1 and B onto
/// readout 0 for 1 536 trials, then the assignment for 3 072, the signed gate and the critic
/// set.
#[test]
#[ignore]
fn the_critic_from_the_mirrored_assignment_at_1024_units_exhaustive() {
    critic_arm(Reversal::MirroredFirst);
}

/// The gate's test (ADR-0061's class; brief 046): the arms and the constants as ADR-0106 and
/// ADR-0107 fixed them; the criterion's three clauses at their edges over blocks written by
/// hand, clause 3 at one per cent and one LSB either side, and clause 3 over H-18's pinned
/// tables; the readings' rules over blocks, trials and expectations written by hand; the
/// harness's critic against the task's over the lattice; and a few trials over a flip on the
/// instrument's network at 1 024 units with the inhibitory baseline and the signed gate set,
/// the critic set beside the same trials with it unset — the oracle held at every trial inside
/// `earned_run_predicted` in both, the task's error and expectations held to the harness's
/// critic — where the reward delivered is the error against the expectation, each expectation
/// moves by the error shifted by five, the two runs are one run up to the first trial whose
/// expectation was not zero, and no excitatory synapse outside the pairs the deliveries
/// addressed moves. No whole run, and nothing else added to the gate.
#[test]
fn a_few_trials_under_the_critic_at_1024_units_and_the_rules_of_the_critic() {
    // The arms and the constants.
    assert_eq!(
        CRITIC_ARMS,
        [Reversal::AssignmentFirst, Reversal::MirroredFirst]
    );
    assert_eq!(
        (CRITIC_TRIALS, CRITIC_BLOCKS, FLIP, FLIP_BLOCK),
        (4_608, 72, 1_536, 24)
    );
    assert_eq!(CRITIC_SHIFT, 5, "ADR-0106's shift");
    assert_eq!(
        CRITIC_AT_START,
        Critic {
            expected_q16: [0; 2],
            shift: 5
        },
        "both expectations zero at the start"
    );
    assert_eq!(CRITIC_PREDICTED, None, "ADR-0106 predicts no verdict");
    assert_eq!(
        [BELOW_PUNISHED_PREDICTED, EXPECTATION_FALLS_PREDICTED],
        [true; 2],
        "ADR-0106's two predicted readings"
    );
    assert_eq!((FALLS_WITHIN, STRONG_PUNISHMENT_Q16), (64, -0x8000));
    assert_eq!((SETTLE_BLOCKS, SETTLE_PER_CENT), (4, 100));
    // Every constant of H-19 restated unchanged: ADR-0065's window, trial, seed and gain,
    // ADR-0066's mark and window of the criterion, ADR-0076's stimulus and cancel, ADR-0077's
    // settled candidate, ADR-0080's reward, ADR-0085's two baselines, ADR-0089's flip,
    // ADR-0093's run and arms, ADR-0094's flag and H-18's one image.
    assert_eq!(
        (WINDOW.from, WINDOW.ticks, TRIAL_TICKS, SEED, GAIN_1024),
        (100, 500, 1 << 14, 27, 0x0001_C000)
    );
    assert_eq!((REWARDED_MIN, LAST_BLOCKS, BLOCK), (80, 2, 64));
    assert_eq!(SHAPE_F46, (2, 0x0001_4000));
    assert_eq!(CANCEL_PICKED_1024, Some(CANCEL_AT_THE_EXTREME));
    assert_eq!((SETTLED, BACKGROUNDS[SETTLED]), (0, 0));
    assert_eq!((GATE_BASELINE_Q16, INHIBITORY_BASELINE_Q16), (0, 0x8000));
    assert_eq!(REWARD_Q16, ONE);
    assert_eq!((FLIP, LAST_BEFORE_FLIP), (INHIBITION_TRIALS, 1_535));
    assert_eq!((PUNISHED_TRIALS, PUNISHED_ARMS), (4_608, REVERSAL_ARMS));
    assert_eq!(SIGNED_GATE_BYTE, 25);
    assert_eq!(
        PUNISHED_IMAGE_CRC_1024, 0x3771_636d_3851_91ac,
        "H-18's image"
    );
    // The criterion's clauses 1 and 2 at their edges over blocks written by hand, as H-18's.
    let blocks_of = |correct: &[u32]| -> Vec<Block> {
        correct
            .iter()
            .map(|&c| {
                (
                    c,
                    0,
                    [[0; 2]; 2],
                    [0; 2],
                    [0; 2],
                    [0; 2],
                    0,
                    0,
                    0,
                    0,
                    IMAGE_COUPLINGS_1024,
                    (BLOCK as u32).saturating_sub(c),
                )
            })
            .collect()
    };
    let run_of = |before: [u32; 2], after: [u32; 2]| -> Vec<Block> {
        let mut correct = vec![BLOCK as u32; CRITIC_BLOCKS];
        correct[FLIP_BLOCK - 2] = before[0];
        correct[FLIP_BLOCK - 1] = before[1];
        correct[CRITIC_BLOCKS - 2] = after[0];
        correct[CRITIC_BLOCKS - 1] = after[1];
        blocks_of(&correct)
    };
    let edge = run_of([40, 40], [40, 40]);
    assert_eq!((correct_before(&edge), revised_count(&edge)), (80, 80));
    assert_eq!(
        predicted([&edge, &edge]),
        Predicted {
            learned: [true; 2],
            revised: [true; 2],
            settled: [true; 2],
            yes: true
        },
        "80 and 80 in both arms, and no pair moved: yes"
    );
    let short_before = run_of([39, 40], [64, 64]);
    assert_eq!(
        predicted([&edge, &short_before]),
        Predicted {
            learned: [true, false],
            revised: [true; 2],
            settled: [true; 2],
            yes: false
        },
        "79 before the flip in one arm: clause 1 fails there"
    );
    let short_after = run_of([64, 64], [40, 39]);
    assert_eq!(
        predicted([&short_after, &edge]),
        Predicted {
            learned: [true; 2],
            revised: [false, true],
            settled: [true; 2],
            yes: false
        },
        "79 at the end in one arm: clause 2 fails there"
    );
    // Clause 3 at its edges: the four answer pairs, two per mapping, each moved over its span
    // by as much as stays below one per cent of its image coupling, then by one LSB more, up
    // and down, under both first mappings. Only the blocks at a span's two ends are read.
    let settle_run = |first: bool, moves: [[i64; 2]; 2]| -> Vec<Block> {
        let mut blocks = blocks_of(&[BLOCK as u32; CRITIC_BLOCKS]);
        for (m, (end, mapping)) in [(FLIP_BLOCK - 1, first), (CRITIC_BLOCKS - 1, !first)]
            .into_iter()
            .enumerate()
        {
            for s in 0..2usize {
                let answer = answer_of(s as u8, mapping);
                blocks[end].10[s][answer] =
                    IMAGE_COUPLINGS_1024[s][answer].saturating_add(moves[m][s]);
            }
        }
        blocks
    };
    assert_eq!(
        IMAGE_COUPLINGS_1024[0][0]
            .saturating_sub(1)
            .saturating_div(SETTLE_PER_CENT),
        62_495,
        "A→R0's image coupling, 6 249 552: 62 495 is below one per cent of it and 62 496 is not"
    );
    for first in [false, true] {
        let largest = [first, !first].map(|mapping| {
            [0usize, 1].map(|s| {
                IMAGE_COUPLINGS_1024[s][answer_of(s as u8, mapping)]
                    .saturating_sub(1)
                    .saturating_div(SETTLE_PER_CENT)
            })
        });
        let down = largest.map(|m| m.map(|d| d.saturating_neg()));
        assert_eq!(
            settle_moves(&settle_run(first, largest), first),
            Some(largest)
        );
        assert!(
            settles(&settle_run(first, largest), first),
            "{first}: each pair just below one per cent, up"
        );
        assert!(
            settles(&settle_run(first, down), first),
            "{first}: and down"
        );
        assert!(settles(&settle_run(first, [[0; 2]; 2]), first));
        for m in 0..2usize {
            for s in 0..2usize {
                let mut over = largest;
                over[m][s] = over[m][s].saturating_add(1);
                assert!(
                    !settles(&settle_run(first, over), first),
                    "{first} {m} {s}: one LSB past, up"
                );
                let mut under = down;
                under[m][s] = under[m][s].saturating_sub(1);
                assert!(
                    !settles(&settle_run(first, under), first),
                    "{first} {m} {s}: one LSB past, down"
                );
            }
        }
        let mut wrong = settle_run(first, largest);
        for (end, mapping) in [(FLIP_BLOCK - 1, first), (CRITIC_BLOCKS - 1, !first)] {
            for s in 0..2usize {
                let other = answer_of(s as u8, !mapping);
                wrong[end].10[s][other] = wrong[end].10[s][other].saturating_add(1_000_000);
            }
        }
        assert!(
            settles(&wrong, first),
            "{first}: only the answer pairs are read"
        );
        let mut before_span = settle_run(first, largest);
        before_span[FLIP_BLOCK - 1 - SETTLE_BLOCKS - 1].10 = [[0; 2]; 2];
        before_span[CRITIC_BLOCKS - 1 - SETTLE_BLOCKS - 1].10 = [[0; 2]; 2];
        assert!(
            settles(&before_span, first),
            "{first}: nothing before a span's first block is read"
        );
    }
    let over = settle_run(false, [[62_496, 0], [0, 0]]);
    assert_eq!(
        predicted([&over, &settle_run(true, [[0; 2]; 2])]),
        Predicted {
            learned: [true; 2],
            revised: [true; 2],
            settled: [false, true],
            yes: false
        },
        "A→R0 past one per cent in the first arm's first mapping: clause 3 fails there"
    );
    let all = settle_run(false, [[0; 2]; 2]);
    assert_eq!(settle_moves(&all[..CRITIC_BLOCKS - 1], false), None);
    assert!(
        !settles(&all[..CRITIC_BLOCKS - 1], false),
        "a run short of 4 608 does not settle"
    );
    let mut longer = all.clone();
    longer.push(all[0]);
    assert!(!settles(&longer, false), "nor one past it");
    // Clause 3 over H-18's pinned tables: its answer pairs rose over both spans in both arms,
    // by 3.7 to 9.5 per cent of the image's couplings, and it does not settle.
    let h18: Vec<[[i64; 2]; 2]> = (0..2usize)
        .map(|k| {
            let first = first_mapping(PUNISHED_ARMS[k]);
            let moves = settle_moves(PUNISHED_BLOCKS_1024[k], first).expect("seventy-two blocks");
            [first, !first]
                .into_iter()
                .zip(moves)
                .map(|(mapping, pairs)| {
                    [0usize, 1].map(|s| {
                        per_myriad(
                            pairs[s],
                            IMAGE_COUPLINGS_1024[s][answer_of(s as u8, mapping)],
                        )
                    })
                })
                .collect::<Vec<[i64; 2]>>()
                .try_into()
                .expect("two mappings")
        })
        .collect();
    eprintln!("DUMP critic1024 H-18's clause-3 moves, per myriad of the image's: {h18:?}");
    assert_eq!(
        h18, H18_SETTLE_PER_MYRIAD,
        "H-18's moves, as ADR-0106 read them"
    );
    for (k, &arm) in PUNISHED_ARMS.iter().enumerate() {
        assert!(
            !settles(PUNISHED_BLOCKS_1024[k], first_mapping(arm)),
            "{arm:?}: H-18 does not settle"
        );
    }
    assert_eq!(
        predicted([PUNISHED_BLOCKS_1024[0], PUNISHED_BLOCKS_1024[1]]),
        Predicted {
            learned: [true; 2],
            revised: [true; 2],
            settled: [false; 2],
            yes: false
        },
        "H-18's tables read under H-19's rule: learned and revised, and never settled"
    );
    // The readings' rules over blocks, trials and expectations written by hand, the
    // assignment first: A's old answer is readout 0 and its new one readout 1.
    let mut hand = blocks_of(&[BLOCK as u32; CRITIC_BLOCKS]);
    let mut there = hand.clone();
    hand[FLIP_BLOCK - 1].10[0][0] = IMAGE_COUPLINGS_1024[0][0].saturating_add(10);
    there[FLIP_BLOCK - 1].10[0][0] = IMAGE_COUPLINGS_1024[0][0].saturating_add(11);
    hand[FLIP_BLOCK - 1].10[1][1] = IMAGE_COUPLINGS_1024[1][1].saturating_add(11);
    there[FLIP_BLOCK - 1].10[1][1] = IMAGE_COUPLINGS_1024[1][1].saturating_add(11);
    assert_eq!(
        below_punished(&hand, &there, false),
        [true, false],
        "A's pair below H-18's by one; B's level with it, which is not below"
    );
    assert_eq!(
        below_punished(&hand, &there, true),
        [false; 2],
        "the other mapping's pairs are level"
    );
    assert_eq!(
        below_punished(&hand[..FLIP_BLOCK - 1], &there, false),
        [false; 2]
    );
    let mut expected_hand = vec![[ONE, ONE]; CRITIC_TRIALS];
    assert_eq!(below_zero(&expected_hand), [None; 2]);
    assert_eq!(falls_within(&expected_hand), [false; 2]);
    expected_hand[FLIP - 1] = [-1, -1];
    assert_eq!(
        below_zero(&expected_hand),
        [None; 2],
        "below zero before the flip is not read"
    );
    expected_hand[FLIP + FALLS_WITHIN - 1][0] = -1;
    expected_hand[FLIP + FALLS_WITHIN][1] = -1;
    assert_eq!(
        below_zero(&expected_hand),
        [Some(FLIP + FALLS_WITHIN - 1), Some(FLIP + FALLS_WITHIN)]
    );
    assert_eq!(
        falls_within(&expected_hand),
        [true, false],
        "the 1 600th trial is within 64 of the flip and the 1 601st is not"
    );
    expected_hand[FLIP + 3][1] = 0;
    assert_eq!(
        below_zero(&expected_hand)[1],
        Some(FLIP + FALLS_WITHIN),
        "zero is not below zero"
    );
    let trial = |stimulus: u8, selection: Option<u8>, signal: i32| -> EarnedTrial {
        (
            stimulus,
            [0; 2],
            selection,
            false,
            -ONE,
            [[0; 2]; 2],
            0,
            signal,
        )
    };
    let mut read_hand = vec![trial(0, Some(1), ONE); CRITIC_TRIALS];
    for t in read_hand.iter_mut().skip(FLIP).step_by(2) {
        t.0 = 1;
        t.2 = Some(0);
    }
    assert_eq!(
        presentations_to_below(&read_hand, &expected_hand),
        [Some(32), Some(33)],
        "A at the odd trials after the flip, B at the even: 32 of A up to the 1 600th, 33 of B up to the 1 601st"
    );
    read_hand[FLIP - 1] = trial(0, Some(0), -ONE);
    read_hand[FLIP] = trial(0, Some(0), STRONG_PUNISHMENT_Q16);
    read_hand[FLIP + 1] = trial(0, Some(0), STRONG_PUNISHMENT_Q16.saturating_add(1));
    read_hand[FLIP + 2] = trial(1, Some(1), -ONE);
    read_hand[FLIP + 3] = trial(1, Some(0), -ONE);
    read_hand[FLIP + BLOCK] = trial(0, Some(0), i32::MIN);
    read_hand[CRITIC_TRIALS - 1] = trial(1, Some(1), -ONE);
    let strong = strong_punishments(&read_hand, false);
    assert_eq!(strong.len(), CRITIC_BLOCKS - FLIP_BLOCK);
    assert_eq!(
        strong[0],
        [1, 1],
        "the 1 537th's old answer at −0.5 counted, the 1 538th's one LSB above not, B's old answer at −1.0 counted and its new answer not, the 1 536th not read"
    );
    assert_eq!(strong[1], [1, 0]);
    assert_eq!(strong[CRITIC_BLOCKS - FLIP_BLOCK - 1], [0, 1]);
    assert_eq!(strong_total(&strong), [2, 2]);
    assert_eq!(
        strong_punishments(&read_hand, true)[0],
        [0, 1],
        "under the other first mapping the old answers are the other readouts: B's readout 0 at −1.0, the 1 540th"
    );
    assert_eq!(
        strong_punishments(&read_hand[..FLIP], false),
        Vec::<[u32; 2]>::new()
    );
    let mut ramp: Vec<[i32; 2]> = Vec::new();
    for t in 0..(3 * BLOCK) {
        ramp.push([t as i32, (t as i32).saturating_neg()]);
    }
    assert_eq!(
        expected_blocks(&ramp),
        vec![[63, -63], [127, -127], [191, -191]],
        "each block's last trial"
    );
    assert_eq!(
        couplings_course(&hand[..1]),
        vec![[[10_000; 2]; 2]],
        "the image's couplings are ten thousand parts of themselves"
    );
    // ADR-0107's arithmetic of the fading with the carry-over (F-53): the error alone is at or
    // below −1.0 for 22 presentations of each stimulus, as ADR-0106 wrote; the signal a delivery
    // leaves, the carry-over counted, for 42 of each and at or below −0.5 for 133 trials; the
    // full punishment summed about 68 presentations' worth of each stimulus where the error
    // alone is about 54. A signal of −2.0 ends the next trial at −0.820.
    let carried = carried_punishment(400);
    let count = |pick: &dyn Fn(&(i32, i32)) -> bool| carried.iter().filter(|c| pick(c)).count();
    let worth = |of: &dyn Fn(&(i32, i32)) -> i32| {
        carried
            .iter()
            .fold(0i64, |sum, c| {
                sum.saturating_add(i64::from(of(c).clamp(-ONE, 0)))
            })
            .saturating_neg()
            .saturating_div(i64::from(ONE).saturating_mul(2))
    };
    let summary = (
        count(&|c| c.0 <= -ONE),
        count(&|c| c.1 <= -ONE),
        count(&|c| c.1 <= STRONG_PUNISHMENT_Q16),
        worth(&|c| c.0),
        worth(&|c| c.1),
        signal_end(&signal_course(-2 * ONE)),
    );
    eprintln!("DUMP critic1024 the carried punishment {summary:?}");
    assert_eq!(
        summary, CARRIED_PUNISHMENT,
        "(trials the error alone is at or below −1.0, the signal is, the signal at or below −0.5, presentations' worth of each stimulus by the error alone and by the signal, the end of a trial from −2.0)"
    );
    // The harness's critic, `div_euclid` in `i64`, against the task's, a shift saturating in
    // `i32`, over the lattice: one rule, written twice.
    for &expected in I32_LATTICE.iter() {
        for &reward in I32_LATTICE.iter() {
            for shift in [0, 1, 5, 30, 31, 32, u32::MAX] {
                let mut c = Critic {
                    expected_q16: [expected, 0],
                    shift,
                };
                let (error, before, after) = c.predict(0, reward);
                assert_eq!(
                    (error, after),
                    critic_step(expected, reward, shift),
                    "{expected} {reward} {shift}"
                );
                assert_eq!(before, expected);
            }
        }
    }
    // A few trials over a flip on the instrument's network at 1 024 units, the inhibitory
    // baseline and the signed gate set, the assignment first, flipped before the trial of index
    // `GATE_FLIP`: the critic set, and beside it the same network with the critic unset. The
    // oracle is held at every trial inside `earned_run_predicted` in both, and the task's error
    // and expectations to the harness's critic in the first.
    const GATE_FLIP: usize = GATE_TRIALS / 2;
    let p = prior(1024);
    let network = Config {
        inhibitory_baseline_q16: Some(INHIBITORY_BASELINE_Q16),
        signed_gate: true,
        ..config(1024, 2, GATE_BASELINE_Q16)
    };
    let mut critic_exec = at_gain(&p, network.clone(), GAIN_1024);
    let before = weights_of(&critic_exec);
    let (run, _, expected) = critic_run(
        &mut critic_exec,
        Reversal::AssignmentFirst,
        GATE_TRIALS,
        GATE_FLIP,
        Some(CRITIC_AT_START),
        &mut |_, _| {},
    );
    let (blocks, trace, _, read, _) = &run;
    assert!(blocks.is_empty(), "a few trials are no whole block");
    assert_eq!((read.len(), expected.len()), (GATE_TRIALS, GATE_TRIALS));
    let mut plain_exec = at_gain(&p, network, GAIN_1024);
    let (plain_run, _) = punished_run(
        &mut plain_exec,
        Reversal::AssignmentFirst,
        GATE_TRIALS,
        GATE_FLIP,
        &mut |_, _| {},
    );
    let plain = &plain_run.3;
    eprintln!(
        "DUMP critic1024 a few trials trace {trace:#018x} read {read:?} expected {expected:?} plain {plain:?}"
    );
    // By hand: the outcome's reward, ±1.0 by whether the trial was correct under the mapping in
    // force (a tie not); the reward delivered the error against the expectation before; the
    // expectation moved by the error shifted by five, the other stimulus's unmoved.
    let mut held = [0i32; 2];
    let mut first_moved: Option<usize> = None;
    for (t, r) in read.iter().enumerate() {
        let s = usize::from(r.0);
        let in_force = t >= GATE_FLIP;
        assert_eq!(
            r.3,
            r.2 == Some(answer_of(r.0, in_force) as u8),
            "trial {t}: correct under the mapping in force"
        );
        let outcome = if r.3 { ONE } else { -ONE };
        let error = outcome.saturating_sub(held[s]);
        assert_eq!(r.4, error, "trial {t}: the reward delivered is the error");
        if first_moved.is_none() && held[s] != 0 {
            first_moved = Some(t);
        }
        held[s] = held[s].saturating_add(error >> CRITIC_SHIFT);
        assert_eq!(
            expected[t], held,
            "trial {t}: the expectation moved by the error shifted by five"
        );
    }
    let diverged = first_moved.expect("a stimulus presented twice in eight trials");
    assert_eq!(
        read[..diverged],
        plain[..diverged],
        "up to the first trial whose expectation was not zero the two runs are one run"
    );
    let (c, u) = (&read[diverged], &plain[diverged]);
    assert_eq!(
        (c.0, c.1, c.2, c.3),
        (u.0, u.1, u.2, u.3),
        "the trial there is the same trial"
    );
    assert_ne!(
        c.4, u.4,
        "and its reward is not: the critic's error, not the outcome's"
    );
    // No excitatory synapse outside the pairs the deliveries addressed moved: a delivery
    // addresses the selected pair whatever the error, the signal it consolidates under being
    // the error plus what the last delivery left.
    let addressed: Vec<(usize, usize)> = read
        .iter()
        .take(GATE_TRIALS - 1)
        .filter_map(|t| t.2.map(|r| (usize::from(t.0), usize::from(r))))
        .collect();
    let reach = reach_by_polarity(&critic_exec, &before, 1024, &addressed);
    assert_eq!(
        reach.excitatory.1, 0,
        "no excitatory synapse outside the addressed pairs moved: {reach:?}"
    );
    assert!(reach.excitatory.0 > 0, "the addressed pairs moved");
}

/// ADR-0107's arithmetic of the fading with the carry-over (F-53), by `carried_punishment` over
/// 400 trials: the trials whose error alone is at or below −1.0 (44, ADR-0106's 22
/// presentations of each stimulus), whose signal is (84, 42 of each) and whose signal is at or
/// below −0.5; the full punishment summed in presentations of each stimulus by the error alone
/// (53, truncated: ADR-0106's about 54) and by the signal (68); and a signal of −2.0 at the end
/// of a trial, −0.820.
const CARRIED_PUNISHMENT: (usize, usize, usize, i64, i64, i32) = (44, 84, 133, 53, 68, -53_714);

/// H-18's answer pairs over clause 3's spans, as fractions of their image couplings in parts
/// per ten thousand, truncated, per arm `[assignment first, mirrored first]`, per mapping
/// `[first, second]`, per stimulus `[A, B]`: read from `PUNISHED_BLOCKS_1024` by
/// `settle_moves` before any run of H-19 — 3.64 to 9.55 per cent. ADR-0106 wrote 3.7, 7.6 and
/// 8.8 where the table reads 3.64, 7.67 and 8.86, and so a range from 3.7 (F-52).
const H18_SETTLE_PER_MYRIAD: [[[i64; 2]; 2]; 2] =
    [[[364, 766], [713, 885]], [[417, 954], [490, 649]]];

// ----------------------------------------------------------- the measurement (brief 046)

/// The two arms at 1 024 units, in `CRITIC_ARMS`'s order, each pinned from one run: the
/// sight's blocks and the trace, the composition, the earned blocks, the moves, each
/// stimulus's expectation and the strong punishments per block, the hash of the readings and
/// the volley's census. Empty until the run: the constants above are committed before the
/// first rewarded run, and the tables after it.
const CRITIC_BLOCKS_1024: [&[Block]; 2] = [&[], &[]];
const CRITIC_TRACES_1024: [u64; 2] = [0; 2];
const CRITIC_COMPOSITIONS_1024: [&[Composition]; 2] = [&[], &[]];
const CRITIC_EARNED_1024: [&[EarnedBlock]; 2] = [&[], &[]];
const CRITIC_READ_1024: [u64; 2] = [0; 2];
const CRITIC_CENSUS_1024: [&[(u32, u64)]; 2] = [&[], &[]];
const CRITIC_MOVES_1024: [&[MovesBlock]; 2] = [&[], &[]];
/// Each stimulus's expectation at every block's end, `[A, B]`, per arm.
const CRITIC_EXPECTED_1024: [&[[i32; 2]]; 2] = [&[], &[]];
/// The strong punishments of the old answer per block from the flip, `[A, B]`, per arm.
const CRITIC_STRONG_1024: [&[[u32; 2]]; 2] = [&[], &[]];
/// The four couplings at the end of the 1 537th trial, per arm, as read.
const CRITIC_CARRY_1024: [[[i64; 2]; 2]; 2] = [[[0; 2]; 2]; 2];
/// Clause 1's and clause 2's counts per arm, `[before the flip, the run's last 128]`, against
/// `REWARDED_MIN`.
const CORRECT_CRITIC_1024: [[u32; 2]; 2] = [[0; 2]; 2];
/// Clause 3's moves per arm, `[first mapping, second][A, B]`, and whether each arm settled.
const SETTLE_MOVES_1024: [Option<[[i64; 2]; 2]>; 2] = [None; 2];
const SETTLED_1024: [bool; 2] = [false; 2];
/// The assertion's reach per arm, as read.
const REACH_CRITIC_1024: [Reach; 2] = [Reach {
    excitatory: (0, 0),
    inhibitory: (0, 0),
}; 2];
/// The need after the flip, per arm, H-17's reading.
const NEED_CRITIC_1024: [Need; 2] = [Need {
    selected_new: 0,
    selected_old: 0,
    ties: 0,
    rewards: 0,
    crossed: None,
}; 2];
/// The first trial after the flip that earned a reward, per arm.
const FIRST_REWARD_CRITIC_1024: [Option<usize>; 2] = [None; 2];
/// Per arm, per stimulus, the first trial after the flip that selected the new answer, and
/// the block in which the selection crossed to it.
const FIRST_NEW_CRITIC_1024: [[Option<usize>; 2]; 2] = [[None; 2]; 2];
const CROSSED_CRITIC_1024: [[Option<usize>; 2]; 2] = [[None; 2]; 2];
/// ADR-0106's predicted readings as read, per arm, per stimulus: (1) below H-18's at the
/// flip; (2) the expectation below zero within 64 trials of the flip, with the trial and the
/// presentations it took.
const BELOW_PUNISHED_1024: [[bool; 2]; 2] = [[false; 2]; 2];
const BELOW_ZERO_1024: [[Option<usize>; 2]; 2] = [[None; 2]; 2];
const FALLS_WITHIN_1024: [[bool; 2]; 2] = [[false; 2]; 2];
const TO_BELOW_1024: [[Option<u32>; 2]; 2] = [[None; 2]; 2];
/// The strong punishments of the old answer after the flip, summed, `[A, B]`, per arm.
const STRONG_TOTAL_1024: [[u32; 2]; 2] = [[0; 2]; 2];
/// ADR-0093's readings (a), (b) and (c), read again under the critic, per arm, per stimulus.
const OLD_FALLS_CRITIC_1024: [[bool; 2]; 2] = [[false; 2]; 2];
const FALL_SLOWS_CRITIC_1024: [[bool; 2]; 2] = [[false; 2]; 2];
const WRONG_BELOW_CRITIC_1024: [[bool; 2]; 2] = [[false; 2]; 2];
/// Per arm, the moves after a negative delivery and after a positive one, `[before the flip,
/// after it]`.
const PUNISHED_MOVES_CRITIC_1024: [[Moves; 2]; 2] = [[([0; 3], [0; 2]); 2]; 2];
const REWARDED_MOVES_CRITIC_1024: [[Moves; 2]; 2] = [[([0; 3], [0; 2]); 2]; 2];
/// The new answer's pairs' rise from the flip to the run's end, `[A, B]`, per arm.
const NEW_RISE_CRITIC_1024: [[i64; 2]; 2] = [[0; 2]; 2];
/// The blocks, of seventy-two, in which the stimulus fired once, per arm.
const ONCE_BLOCKS_CRITIC_1024: [u32; 2] = [0; 2];
/// Whether the inhibitory sum fell in every block of the run, per arm.
const FALLS_CRITIC_1024: [bool; 2] = [false; 2];
/// ADR-0080's derivation as read over each arm's whole run, clause by clause.
const DERIVATION_CRITIC_1024: [[bool; 3]; 2] = [[false; 3]; 2];
/// The arena's sums by polarity after each arm's run, `(inhibitory, excitatory)`.
const SUMS_AFTER_CRITIC_1024: [(i64, i64); 2] = [(0, 0); 2];
