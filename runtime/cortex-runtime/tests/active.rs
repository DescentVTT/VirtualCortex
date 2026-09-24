//! Brief 042 (ADR-0097) measures the active set and changes no rule. The executor serves a unit
//! every tick from the one a message wakes it until it is at rest (ADR-0023), so what a tick
//! costs turns on how many units are not at rest, which the tree had never counted. The engine
//! now counts the turns it serves (`Executor::turns`, kept where `delivered` is), and this
//! binary reads three things through it.
//!
//! The ticks one message keeps a unit awake: an oracle of the membrane, written from
//! `membrane.rs`'s rule before the engine ran — the three compartments' leak and coupling below
//! the threshold, beside the basal leak alone, which is where the brief's estimate came from —
//! and the engine held to it on one armed unit with no synapse, one message of 0.125 at the
//! gain 1.0 and at the instrument's 1.75, the unit served on every tick until it rests and not
//! after.
//!
//! The prediction, written before the runs: the fraction of ticks a unit is not at rest is at
//! least `1 − e^{−rD}` when messages reach it at `r` per tick and one keeps it awake `D` ticks
//! (it is at rest only if none arrived in the last `D`), the exponential computed in integers
//! from the oracle's `D` at 1.75 — the drive's floor, which the network's own messages move.
//!
//! The active set on the reference network: ADR-0044's prior at seed 22 under the instrument's
//! configuration — the gain 1.75, the controller off, the modulation baseline zero (the
//! calibration's, so no weight moves), two workers — at 1 024 units under ADR-0044's drive and
//! under two drives sixteen and 256 times sparser, and at 4 096 units under ADR-0044's drive;
//! each at 1 024 units beside a control, the same units armed and wired to nothing under the
//! same drive, which is the drive's active set alone. Every run is read tick by tick — the
//! turns served, held every tick to the units whose gate the tick before left scheduled, the
//! messages delivered, those the drive sent, the spikes — over the first 512 ticks in rows of
//! 64, the rest of a lead-in window of $2^{17}$ ticks, and four windows after it, and pinned
//! row by row. The runs are weekly `exhaustive` tests; the gate runs the oracle, the
//! exponential and the prediction as committed, the counter at its edges and the ticks to rest
//! at both gains on the engine, and the first 512 ticks of the first run, held to the first
//! rows of its table.

#![deny(clippy::arithmetic_side_effects)]

// The harness — the prior, the drive, the instrument's configuration and the network at a gain
// — is `tests/instrument.rs`'s module, compiled into this binary as well. What this binary does
// not call is that binary's, so the module's unused items and imports are allowed here and
// nowhere else in this file.
#[allow(dead_code, unused_imports)]
#[path = "instrument/harness.rs"]
mod harness;
use harness::*;

use cortex_core::{
    APICAL_LEAK_SHIFT, BASAL_LEAK_SHIFT, COUPLING_SHIFT, GateState, SOMA_LEAK_SHIFT,
};
use cortex_homeostasis::GAIN_ONE_Q16;
use std::time::Instant;

// ------------------------------------------- written before the engine ran (brief 042)

/// The message the drive of ADR-0044 sends, 0.125, which the executor scales by the tick's
/// gain on arrival (F-47).
const MESSAGE_Q16: i32 = 0x2000;

/// The two gains the ticks to rest are read at: 1.0, where the message arrives as sent, and
/// the instrument's 1.75, at which every run below is made.
const REST_GAINS: [u32; 2] = [GAIN_ONE_Q16, GAIN_1024];

/// The oracle's counts at the two gains, computed from the rule before the engine ran: the
/// turns one message keeps an armed unit at rest awake (the three compartments), the ticks the
/// basal compartment alone takes to leak to zero from the message, and the soma's peak. The
/// brief's estimate, about 1 900 ticks from 0.125, took the one-LSB tail as starting at 512;
/// `|v| >> 9` is one from 1 023 down, so the tail is 1 023 ticks and the count 2 209.
const TICKS_TO_REST: [u32; 2] = [2_210, 2_503];
const BASAL_ALONE: [u32; 2] = [2_209, 2_502];
const SOMA_PEAK_Q16: [i32; 2] = [3_842, 6_715];

/// A bound on the turns the oracle and the engine are stepped over, above either count.
const REST_BOUND: u32 = 4_096;

/// `membrane.rs`'s leak, written again from its rule: `v` moves toward zero by `|v| >> shift`,
/// by at least one LSB and never past zero.
fn leak(v: i32, shift: u32) -> i32 {
    let v = i64::from(v);
    let magnitude = v.saturating_abs();
    let step = (magnitude >> shift).max(1).min(magnitude);
    v.saturating_sub(v.signum().saturating_mul(step)) as i32
}

/// The ticks `v` takes to reach zero under the leak alone at `shift`.
fn leak_ticks(v: i32, shift: u32) -> u32 {
    let mut v = v;
    for tick in 0..REST_BOUND {
        if v == 0 {
            return tick;
        }
        v = leak(v, shift);
    }
    panic!("the leak did not reach zero within the bound");
}

/// The executor's scaling of a turn's summed input by the gain (ADR-0036), written again:
/// `sum × gain` in Q16.16, rounded to nearest.
fn scaled(sum: i32, gain_q16: u32) -> i32 {
    (i64::from(sum)
        .saturating_mul(i64::from(gain_q16))
        .saturating_add(0x8000)
        >> 16) as i32
}

/// The turns one basal message of `basal_q16` keeps an armed unit at rest awake, and the
/// soma's peak, from `membrane.rs`'s rule and not from `integrate`: per tick the basal and
/// apical compartments leak and take their input, the soma leaks and takes a sixteenth of its
/// difference to each. The unit is served from the tick that integrates the message through
/// the tick after which every potential is zero — ADR-0023's rest, since below the threshold
/// (asserted) no window starts and the threshold stays at its base.
fn awake(basal_q16: i32) -> (u32, i32) {
    let (mut basal, mut apical, mut soma, mut peak) = (0i32, 0i32, 0i32, 0i32);
    let mut input = basal_q16;
    for turn in 1..=REST_BOUND {
        basal = leak(basal, BASAL_LEAK_SHIFT).saturating_add(input);
        apical = leak(apical, APICAL_LEAK_SHIFT);
        input = 0;
        let pull = |to: i32| i64::from(to).saturating_sub(i64::from(soma)) >> COUPLING_SHIFT;
        let moved = pull(basal).saturating_add(pull(apical));
        soma = i64::from(leak(soma, SOMA_LEAK_SHIFT))
            .saturating_add(moved)
            .clamp(i64::from(i32::MIN), i64::from(i32::MAX)) as i32;
        peak = peak.max(soma);
        assert!(
            soma < THRESHOLD_BASE,
            "the message stays below the threshold"
        );
        if basal == 0 && apical == 0 && soma == 0 {
            return (turn, peak);
        }
    }
    panic!("the unit did not rest within the bound");
}

// ------------------------------------------------ written before the runs (brief 042)

/// The runs, in the order the tests make them: `(units, every, messages)` of the drive, the
/// efficacy, the seed and the draw ADR-0044's (`drive`). (a) 1 024 units, ADR-0044's drive, a
/// message per 128 units every tick; (b) one message every second tick, sixteen times fewer;
/// (c) one every thirty-second, 256 times fewer; (d) 4 096 units, ADR-0044's drive.
const RUNS: [(u32, u32, u32); 4] = [(1024, 1, 8), (1024, 2, 1), (1024, 32, 1), (4096, 1, 32)];

/// The drive of run `k`.
fn run_drive(k: usize) -> Drive {
    let (units, every, messages) = RUNS[k];
    Drive {
        every,
        messages,
        ..drive(units)
    }
}

/// The exponential's scale: $2^{62}$.
const EXP_ONE: u128 = 1 << 62;

/// The terms of the series summed, a bound above what any exponent below reaches (the terms of
/// $e^{20}$ fall below one part in $2^{62}$ before the hundredth).
const EXP_TERMS: u128 = 128;

/// $e^{num/den}$ at the scale $2^{62}$, by its series, every term positive and rounded down.
fn exp_q62(num: u64, den: u64) -> u128 {
    let (num, den) = (u128::from(num), u128::from(den));
    let (mut term, mut sum) = (EXP_ONE, EXP_ONE);
    for k in 1..=EXP_TERMS {
        term = term
            .checked_mul(num)
            .expect("an exponent within the series' range")
            .checked_div(den.saturating_mul(k))
            .expect("a denominator above zero");
        sum = sum.saturating_add(term);
    }
    sum
}

/// $e^{-num/den}$ in parts per million, rounded to nearest: the reciprocal of the series.
fn exp_neg_ppm(num: u64, den: u64) -> u64 {
    let sum = exp_q62(num, den);
    let scaled = 1_000_000u128.saturating_mul(EXP_ONE).saturating_mul(2);
    scaled
        .saturating_add(sum)
        .checked_div(sum.saturating_mul(2))
        .expect("a series of at least one") as u64
}

/// The prediction for run `k`: $1 - e^{-rD}$ in parts per million, with `r` the drive's
/// messages per unit per tick and `D` the oracle's turns at 1.75.
fn predicted_ppm(k: usize) -> u64 {
    let (units, every, messages) = RUNS[k];
    let d = u64::from(TICKS_TO_REST[1]);
    let num = d.saturating_mul(u64::from(messages));
    let den = u64::from(every).saturating_mul(u64::from(units));
    1_000_000u64.saturating_sub(exp_neg_ppm(num, den))
}

/// The predictions, as computed before the runs: (a) and (d) all but three parts in a
/// thousand million, 100 per cent (rD = 19.55); (b) 70.54 per cent (rD = 1.222); (c) 7.35 per
/// cent (rD = 0.0764). The whitepaper's 1–2 per cent needs rD of 0.010 to 0.020, a message per
/// unit every 124 000 to 248 000 ticks: 0.4 to 0.8 a second.
const PREDICTED_PPM: [u64; 4] = [1_000_000, 705_409, 73_541, 1_000_000];

// --------------------------------------------------------------------------- the reading

/// One row of a run: over its ticks, the turns served, the messages delivered, those of them
/// the drive sent, the spikes, the fewest and the most turns in one tick, and the ticks at
/// which every unit was served.
type Row = (u64, u64, u64, u64, u64, u64, u64);

/// The rows a run is read in: the first 512 ticks in eight rows of 64, the rest of the lead-in
/// window, and four windows of $2^{17}$ ticks.
const PREFIX_ROWS: usize = 8;
const PREFIX_TICKS: u64 = 64;
const MEASURED_WINDOWS: usize = 4;

fn layout() -> Vec<u64> {
    let prefix = PREFIX_TICKS.saturating_mul(PREFIX_ROWS as u64);
    let mut rows = vec![PREFIX_TICKS; PREFIX_ROWS];
    rows.push(WINDOW_TICKS.saturating_sub(prefix));
    rows.extend([WINDOW_TICKS; MEASURED_WINDOWS]);
    rows
}

/// The units whose gate is scheduled between ticks: the units the next tick serves.
fn scheduled(exec: &Engine) -> u64 {
    exec.units()
        .iter()
        .filter(|u| u.gate() == Some(GateState::Scheduled))
        .count() as u64
}

/// ADR-0023's rest, written again from its rule: every potential zero, no window running, the
/// threshold at or below its base.
fn rests(u: &DendriticSuperNeuron) -> bool {
    u.v_soma == 0
        && u.v_basal == 0
        && u.v_apical == 0
        && u.refractory_ticks == 0
        && u.bac_plateau_ticks == 0
        && u.v_thresh <= THRESHOLD_BASE
}

/// The spikes so far, from the executor's own train (ADR-0050).
fn spikes(exec: &mut Engine) -> u64 {
    (exec.train().len() as u64).saturating_add(exec.train_overwritten())
}

/// Runs `exec` under `drive` for the rows `lengths` names, back to back from its clock, and
/// reads each; every tick's turns are held to the units the tick before left scheduled. Also
/// returns the wall time the ticks took, in nanoseconds: a developer machine's, never pinned.
fn read(exec: &mut Engine, drive: &Drive, lengths: &[u64]) -> (Vec<Row>, u128) {
    let units = exec.units().len() as u64;
    let inject = exec.injector();
    let mut rows = Vec::new();
    let mut nanos = 0u128;
    for &length in lengths {
        let (turns, delivered, spiked) = (exec.turns(), exec.delivered(), spikes(exec));
        let (mut sent, mut fewest, mut most, mut full) = (0u64, u64::MAX, 0u64, 0u64);
        for _ in 0..length {
            let tick = exec.ticks();
            // A message injected before tick t is drained in its phase 3 and integrated at
            // t + 1: this tick's turns drain what the drive sent before the tick before.
            if tick.checked_sub(1).is_some_and(|t| drive.is_due(t)) {
                sent = sent.saturating_add(u64::from(drive.messages));
            }
            drive
                .step(&inject, tick)
                .expect("the ring holds a tick's drive");
            let due = scheduled(exec);
            let before = exec.turns();
            let start = Instant::now();
            exec.tick();
            nanos = nanos.saturating_add(start.elapsed().as_nanos());
            let served = exec.turns().saturating_sub(before);
            assert_eq!(
                served, due,
                "the turns are the units the tick before scheduled"
            );
            fewest = fewest.min(served);
            most = most.max(served);
            full = full.saturating_add(u64::from(served == units));
        }
        rows.push((
            exec.turns().saturating_sub(turns),
            exec.delivered().saturating_sub(delivered),
            sent,
            spikes(exec).saturating_sub(spiked),
            fewest,
            most,
            full,
        ));
    }
    (rows, nanos)
}

/// The reference network at `units`: ADR-0044's prior synthesized at the instrument's gain,
/// the controller off and the modulation baseline zero.
fn network(units: u32) -> Engine {
    let exec = at_gain(&prior(units), config(units, 2, 0), GAIN_1024);
    assert_eq!(exec.modulation_baseline_q16(), 0, "no weight moves");
    exec
}

/// The control at `units`: the same executor and gain, every unit armed at its base threshold
/// and wired to nothing, so that what serves a unit is the drive alone.
fn unwired(units: u32) -> Engine {
    let config = config(units, 2, 0);
    let mut exec = Engine::new(config.clone()).unwrap();
    for u in exec.units_mut() {
        u.v_thresh = THRESHOLD_BASE;
    }
    reload_with(&exec, config, |h| h.synaptic_gain_q16 = GAIN_1024)
}

/// One armed unit at rest with no synapse, at `gain`.
fn one_unit(gain: u32) -> Engine {
    let config = Config {
        units: 1,
        nodes_per_worker: 16,
        injector_capacity: 16,
        modulation_baseline_q16: 0,
        ..Config::default()
    };
    let mut exec = Engine::new(config.clone()).unwrap();
    exec.units_mut()[0].v_thresh = THRESHOLD_BASE;
    reload_with(&exec, config, |h| h.synaptic_gain_q16 = gain)
}

/// A row's turns as parts per million of every unit on every tick.
fn ppm(row: &Row, units: u32, ticks: u64) -> u64 {
    row.0
        .saturating_mul(1_000_000)
        .checked_div(u64::from(units).saturating_mul(ticks))
        .unwrap_or(0)
}

/// Dumps a run, its four windows beside the prediction and its wall time, then holds it to its
/// table.
fn hold(name: &str, k: usize, rows: &[Row], nanos: u128, table: &[Row]) {
    let units = RUNS[k].0;
    let windows: Vec<u64> = rows
        .iter()
        .skip(PREFIX_ROWS.saturating_add(1))
        .map(|row| ppm(row, units, WINDOW_TICKS))
        .collect();
    let ticks = layout().iter().fold(0u64, |sum, &t| sum.saturating_add(t));
    let turns = rows.iter().fold(0u64, |sum, row| sum.saturating_add(row.0));
    eprintln!(
        "DUMP {name} {rows:?} windows_ppm {windows:?} predicted_ppm {} \
         ns_per_tick {} ns_per_turn {} (a developer machine's)",
        PREDICTED_PPM[k],
        nanos.checked_div(u128::from(ticks)).unwrap_or(0),
        nanos.checked_div(u128::from(turns)).unwrap_or(0),
    );
    assert_eq!(rows, table, "{name}");
}

// ---------------------------------------------------------------------- the pinned tables

/// The four runs' rows, in `RUNS`'s order, each pinned from one run; committed before the
/// runs empty, and after them filled.
const NETWORK_ROWS: [&[Row]; 4] = [&[], &[], &[], &[]];

/// The controls at 1 024 units, (a) to (c).
const CONTROL_ROWS: [&[Row]; 3] = [&[], &[], &[]];

// ------------------------------------------------------------------------------ the gate

#[test]
fn the_turns_are_counted_one_message_keeps_a_unit_awake_as_the_oracle_says_and_the_prediction_is_as_written()
 {
    // The oracle, before the engine: its counts are the constants the prediction reads.
    for (k, &gain) in REST_GAINS.iter().enumerate() {
        let message = scaled(MESSAGE_Q16, gain);
        assert_eq!(leak_ticks(message, BASAL_LEAK_SHIFT), BASAL_ALONE[k]);
        assert_eq!(awake(message), (TICKS_TO_REST[k], SOMA_PEAK_Q16[k]));
    }
    assert_eq!(scaled(MESSAGE_Q16, GAIN_1024), 0x3800, "0.125 at 1.75");
    assert_eq!(leak(1, SOMA_LEAK_SHIFT), 0, "the last LSB goes");
    assert_eq!(leak(-3, BASAL_LEAK_SHIFT), -2);
    assert_eq!(leak(1024, BASAL_LEAK_SHIFT), 1022);

    // The exponential at values known to six places, and the predictions as committed.
    let known = [
        ((0, 1), 1_000_000),
        ((1, 1), 367_879),
        ((2, 1), 135_335),
        ((1, 2), 606_531),
        ((10, 1), 45),
        ((20, 1), 0),
    ];
    for ((num, den), expected) in known {
        assert_eq!(exp_neg_ppm(num, den), expected, "e^-({num}/{den})");
    }
    for (k, &expected) in PREDICTED_PPM.iter().enumerate() {
        assert_eq!(predicted_ppm(k), expected, "run {k}");
    }
    assert_eq!(run_drive(0), drive(1024), "(a) is ADR-0044's drive");
    assert_eq!(run_drive(3), drive(4096), "(d) is ADR-0044's drive");

    // The counter at its edges, on one armed unit with no synapse: a tick with no unit awake
    // serves none; one message serves the unit on every tick from the one that integrates it
    // until it rests, and not after; the engine's count is the oracle's.
    for (k, &gain) in REST_GAINS.iter().enumerate() {
        let mut exec = one_unit(gain);
        exec.run(3);
        assert_eq!(exec.turns(), 0, "a tick with no unit awake serves none");
        exec.injector()
            .inject(0, spike_message(MESSAGE_Q16, false))
            .unwrap();
        exec.tick();
        assert_eq!(exec.turns(), 0, "drained into the mailbox, not yet a turn");
        let mut rested = None;
        for tick in 1..=REST_BOUND {
            let due = scheduled(&exec);
            let before = exec.turns();
            exec.tick();
            let served = exec.turns().saturating_sub(before);
            assert_eq!(served, due, "gain {gain:#x}, tick {tick}");
            assert_eq!(
                served,
                u64::from(rested.is_none()),
                "gain {gain:#x}, tick {tick}"
            );
            if rested.is_none() && rests(&exec.units()[0]) {
                rested = Some(tick);
            }
        }
        assert_eq!(rested, Some(TICKS_TO_REST[k]), "gain {gain:#x}");
        assert_eq!(exec.turns(), u64::from(TICKS_TO_REST[k]));
        assert_eq!(exec.delivered(), 1);
    }
}

// ------------------------------------------------------------------- the runs (weekly)

/// Run `k` on the reference network, and at 1 024 units its control.
fn active_set(k: usize) {
    let (units, _, _) = RUNS[k];
    let drive = run_drive(k);
    let mut exec = network(units);
    let (rows, nanos) = read(&mut exec, &drive, &layout());
    hold(&format!("network {k}"), k, &rows, nanos, NETWORK_ROWS[k]);
    if let Some(table) = CONTROL_ROWS.get(k) {
        let mut exec = unwired(units);
        let (rows, nanos) = read(&mut exec, &drive, &layout());
        hold(&format!("control {k}"), k, &rows, nanos, table);
    }
}

#[test]
#[ignore = "whole-domain: a lead-in and four windows at 1 024 units, and its control; weekly"]
fn the_active_set_under_the_reference_drive_at_1024_units_exhaustive() {
    active_set(0);
}

#[test]
#[ignore = "whole-domain: a lead-in and four windows at 1 024 units, and its control; weekly"]
fn the_active_set_under_a_drive_sixteen_times_sparser_at_1024_units_exhaustive() {
    active_set(1);
}

#[test]
#[ignore = "whole-domain: a lead-in and four windows at 1 024 units, and its control; weekly"]
fn the_active_set_under_a_drive_256_times_sparser_at_1024_units_exhaustive() {
    active_set(2);
}

#[test]
#[ignore = "whole-domain: a lead-in and four windows at 4 096 units; weekly"]
fn the_active_set_under_the_reference_drive_at_4096_units_exhaustive() {
    active_set(3);
}
