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

// The harness — the network, the task, the readings, the oracle, every rule and every pinned
// table of ADR-0065 to ADR-0081 — is a module shared with `tests/everywhere.rs` (brief 038,
// ADR-0083): one file, compiled into each binary that declares it, so that the runs at the
// baseline of 0.5 leave this binary alone in its shard (ADR-0073, ADR-0082). The tests below
// are this binary's own.
#[path = "instrument/harness.rs"]
mod harness;
use harness::*;

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
    // Over the pinned tables: the verdict as written, by the rules; the prediction as read;
    // the correct selections, the crossings, the splits, the lock-in, the reach and the
    // derivation as the constants state; each arm's twenty-four blocks with the same trials
    // presenting A; in a rewarded arm the wrong pairs' couplings the image's in every block
    // and nothing consolidated on them; each block's earned readings consistent with its
    // sight's block — the correct trials the answer's splits, the ties the splits' ties, the
    // signal after the block's last reward the earned block's.
    let verdict = reinforced([REINFORCED_BLOCKS_1024[0], REINFORCED_BLOCKS_1024[1]]);
    assert_eq!(verdict, REINFORCED_1024, "the verdict as written");
    assert_eq!(verdict.yes, REINFORCED_PREDICTED, "the prediction as read");
    assert_eq!(
        [
            last_correct(REINFORCED_BLOCKS_1024[0]),
            last_correct(REINFORCED_BLOCKS_1024[1]),
        ],
        CORRECT_LAST_1024
    );
    assert_eq!(
        [
            crossing(REINFORCED_BLOCKS_1024[0]),
            crossing(REINFORCED_BLOCKS_1024[1]),
            crossing(REINFORCED_BLOCKS_1024[2]),
        ],
        CROSSED_1024
    );
    assert_eq!(
        locked_in(
            SPLITS_LAST_1024[2],
            [SPLITS_LAST_1024[0], SPLITS_LAST_1024[1]]
        ),
        LOCKED_1024
    );
    let assignment = REINFORCED_BLOCKS_1024[0];
    for (k, &arm) in EARNED_ARMS.iter().enumerate() {
        let blocks = REINFORCED_BLOCKS_1024[k];
        let earned = REINFORCED_EARNED_1024[k];
        let compositions = REINFORCED_COMPOSITIONS_1024[k];
        assert_eq!(
            blocks.len(),
            REINFORCED_TRIALS / BLOCK,
            "{arm:?}: twenty-four blocks"
        );
        assert_eq!(earned.len(), REINFORCED_TRIALS / BLOCK);
        assert_eq!(compositions.len(), REINFORCED_TRIALS / BLOCK);
        assert_ne!(REINFORCED_TRACES_1024[k], 0);
        assert_ne!(REINFORCED_READ_1024[k], 0);
        assert!(!REINFORCED_CENSUS_1024[k].is_empty());
        assert_eq!(REINFORCED_REACH_1024[k].1, 0);
        assert!(REINFORCED_REACH_1024[k].0 > 0);
        assert_eq!(DERIVATION_1024[k], [true; 3]);
        let mirrored = earned_mirrors(arm);
        let pairs = assigned_pairs(mirrored);
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
            last_two, SPLITS_LAST_1024[k],
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
            assert!(earned[j].3 <= SIGNAL_END_FIXED_Q16);
            assert!(compositions[j].3 <= BLOCK as u32);
            if answers(arm) {
                let correct =
                    earned[j].0[0][answer_of(0, mirrored)] + earned[j].0[1][answer_of(1, mirrored)];
                assert_eq!(block.0, correct, "{arm:?} block {j}: the correct trials");
                assert_eq!(
                    earned[j].1, correct,
                    "{arm:?} block {j}: one positive reward per correct trial"
                );
                for &(s, r) in &ALL_PAIRS {
                    if !pairs.contains(&(s, r)) {
                        assert_eq!(
                            block.10[s][r], IMAGE_COUPLINGS_1024[s][r],
                            "{arm:?} block {j}: the wrong pair {s}→{r} is the image's"
                        );
                        assert_eq!(earned[j].2[s][r], 0);
                    }
                }
            } else {
                assert_eq!(
                    block.0,
                    earned[j].0[0][0] + earned[j].0[1][1],
                    "{arm:?} block {j}: correct reads the assignment's answers"
                );
            }
        }
    }
}
