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
/// the 1 537th trial, and the whole run's volley census. Empty until the run: the constants
/// above are committed before the first rewarded run, and the tables after it.
const REVERSAL_BLOCKS_1024: [&[Block]; 2] = [&[], &[]];
const REVERSAL_TRACES_1024: [u64; 2] = [0; 2];
const REVERSAL_COMPOSITIONS_1024: [&[Composition]; 2] = [&[], &[]];
const REVERSAL_EARNED_1024: [&[EarnedBlock]; 2] = [&[], &[]];
const REVERSAL_READ_1024: [u64; 2] = [0; 2];
const REVERSAL_CENSUS_1024: [&[(u32, u64)]; 2] = [&[], &[]];
/// The one inhibited image both arms decode, its CRC-64: the same bytes in both tests.
const REVERSAL_IMAGE_CRC_1024: u64 = 0;
/// The four couplings at the ends of the 1 536th and the 1 537th trials, `[stimulus][readout]`.
type Carry = ([[i64; 2]; 2], [[i64; 2]; 2]);
/// The couplings around the flip per arm, as read.
const REVERSAL_SNAPSHOTS_1024: [Carry; 2] = [([[0; 2]; 2], [[0; 2]; 2]); 2];
/// Clause 1's and clause 2's counts per arm, `[before the flip, the run's last 128]`,
/// against `REWARDED_MIN`.
const CORRECT_REVERSAL_1024: [[u32; 2]; 2] = [[0; 2]; 2];
/// ADR-0090's assertion's reach per arm, as read.
const HELD_1024: [Held; 2] = [Held {
    after_carry: 0,
    uncarried: 0,
    outside: 0,
    carried: 0,
}; 2];
/// The measured need per arm, as read.
const NEED_1024: [Need; 2] = [Need {
    selected_new: 0,
    selected_old: 0,
    ties: 0,
    rewards: 0,
    crossed: None,
}; 2];
/// The first trial after the flip that earned a reward, per arm, as read.
const FIRST_REWARD_1024: [Option<usize>; 2] = [None; 2];
/// The pair the 1 536th trial carried into the 1 537th, per arm, as read.
const CARRIED_1024: [Option<(usize, usize)>; 2] = [None; 2];
/// The new answer's pairs' rise from the flip to the run's end, `[A, B]`, per arm, as read.
const NEW_RISE_1024: [[i64; 2]; 2] = [[0; 2]; 2];
/// The blocks, of forty-eight, in which the stimulus fired once, per arm, as read.
const ONCE_BLOCKS_REVERSAL_1024: [u32; 2] = [0; 2];
/// Whether the inhibitory sum fell in every block of the run, per arm, as read.
const FALLS_REVERSAL_1024: [bool; 2] = [false; 2];
/// ADR-0080's derivation as read over each arm's whole run, clause by clause.
const DERIVATION_REVERSAL_1024: [[bool; 3]; 2] = [[false; 3]; 2];
/// The arena's sums by polarity after each arm's run, `(inhibitory, excitatory)`.
const SUMS_AFTER_REVERSAL_1024: [(i64, i64); 2] = [(0, 0); 2];

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
}
