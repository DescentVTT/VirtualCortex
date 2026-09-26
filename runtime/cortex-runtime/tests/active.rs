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
//!
//! Brief 044 (ADR-0101) times the same four runs with no census: a lever's gain on the
//! engine's speed is read on a workload whose instrument reads no record the engine does not
//! (F-50). The census above reads every unit's gate byte on the coordinator's thread, worker 0,
//! before each tick, which leaves every unit's line in that worker's cache before the tick is
//! timed. The timed runs read between two timed ticks only what the engine keeps — the clock,
//! the turns and the messages each worker counted — and are held to the same tables; the
//! census stays in the four runs above as the test of behaviour it is. Two of them are timed on
//! one worker too, (a) and (c), a turn with no barrier and no balance in it.

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

/// What a run reads of the units between its ticks.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Census {
    /// ADR-0097's test of behaviour: before every tick, every unit's gate byte, and the tick's
    /// turns held to the units it found scheduled.
    Held,
    /// ADR-0101's timed workload: no unit's record between two ticks, only the engine's own
    /// counts.
    Off,
}

/// Runs `exec` under `drive` for the rows `lengths` names, back to back from its clock, and
/// reads each; under `Census::Held` every tick's turns are held to the units the tick before
/// left scheduled. Also returns the wall time the ticks took, in nanoseconds: a developer
/// machine's, never pinned.
fn read(exec: &mut Engine, drive: &Drive, lengths: &[u64], census: Census) -> (Vec<Row>, u128) {
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
            let due = (census == Census::Held).then(|| scheduled(exec));
            let before = exec.turns();
            let start = Instant::now();
            exec.tick();
            nanos = nanos.saturating_add(start.elapsed().as_nanos());
            let served = exec.turns().saturating_sub(before);
            if let Some(due) = due {
                assert_eq!(
                    served, due,
                    "the turns are the units the tick before scheduled"
                );
            }
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

/// The workers every run is made on, ADR-0097's; ADR-0101's diagnostic times two runs on one.
const WORKERS: usize = 2;

/// The reference network at `units` on `workers`: ADR-0044's prior synthesized at the
/// instrument's gain, the controller off and the modulation baseline zero.
fn network(units: u32, workers: usize) -> Engine {
    let exec = at_gain(&prior(units), config(units, workers, 0), GAIN_1024);
    assert_eq!(exec.modulation_baseline_q16(), 0, "no weight moves");
    exec
}

/// The control at `units` on `workers`: the same executor and gain, every unit armed at its
/// base threshold and wired to nothing, so that what serves a unit is the drive alone.
fn unwired(units: u32, workers: usize) -> Engine {
    let config = config(units, workers, 0);
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

/// Dumps a run on `workers`: its rows, its four windows beside the prediction, and its wall
/// time per tick and per turn on one worker (the workers' time over the turns), a developer
/// machine's.
fn dump(name: &str, k: usize, rows: &[Row], nanos: u128, workers: usize) {
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
         ns_per_tick {} worker_ns_per_turn {} (a developer machine's)",
        PREDICTED_PPM[k],
        nanos.checked_div(u128::from(ticks)).unwrap_or(0),
        nanos
            .saturating_mul(workers as u128)
            .checked_div(u128::from(turns))
            .unwrap_or(0),
    );
}

// ---------------------------------------------------------------------- the pinned tables

/// The four runs' rows, in `RUNS`'s order, each pinned from one run and reproduced by a
/// second; committed empty before the runs. Each row is `(turns, delivered, sent by the drive,
/// spikes, fewest turns in a tick, most, ticks with every unit served)`; the four windows are
/// the last four rows. At 1 024 units under ADR-0044's drive 99.99 per cent of the units are
/// served in every window (every unit on 91 to 94 per cent of the ticks), the population
/// firing 1.73 to 1.79 Hz a unit and the synapses delivering 55 to 57 messages a unit a second
/// beside the drive's 781; sixteen times sparser, 71.9 to 72.1 per cent against the floor's
/// 70.54, and no spike; 256 times sparser, 7.35 to 7.41 per cent against 7.35 (one window
/// 7.345, below it by a thousandth of its value), and no spike; at 4 096 units, 99.99 per cent,
/// every unit on 77 to 83 per cent of the ticks.
const NETWORK_ROWS: [&[Row]; 4] = [
    &[
        (13_612, 504, 504, 0, 0, 391, 0),
        (33_517, 512, 512, 0, 395, 631, 0),
        (45_472, 512, 512, 0, 635, 782, 0),
        (53_368, 512, 512, 0, 784, 879, 0),
        (58_553, 512, 512, 0, 881, 944, 0),
        (61_576, 512, 512, 0, 944, 977, 0),
        (63_385, 512, 512, 0, 978, 995, 0),
        (63_982, 512, 512, 0, 995, 1004, 0),
        (133_683_066, 1_123_070, 1_044_480, 2464, 1004, 1024, 122_674),
        (134_207_526, 1_124_137, 1_048_576, 2359, 1022, 1024, 121_388),
        (134_210_016, 1_122_505, 1_048_576, 2317, 1021, 1024, 123_714),
        (134_207_114, 1_124_129, 1_048_576, 2356, 1022, 1024, 120_642),
        (134_206_286, 1_125_495, 1_048_576, 2403, 1022, 1024, 119_832),
    ],
    &[
        (1024, 32, 32, 0, 0, 32, 0),
        (3023, 32, 32, 0, 32, 63, 0),
        (4865, 32, 32, 0, 63, 90, 0),
        (6765, 32, 32, 0, 90, 121, 0),
        (8658, 32, 32, 0, 121, 147, 0),
        (10_332, 32, 32, 0, 147, 175, 0),
        (11_996, 32, 32, 0, 175, 201, 0),
        (13_668, 32, 32, 0, 201, 227, 0),
        (95_879_032, 65_280, 65_280, 0, 227, 771, 0),
        (96_634_999, 65_536, 65_536, 0, 704, 767, 0),
        (96_540_672, 65_536, 65_536, 0, 697, 767, 0),
        (96_799_753, 65_536, 65_536, 0, 701, 775, 0),
        (96_517_112, 65_536, 65_536, 0, 709, 770, 0),
    ],
    &[
        (94, 2, 2, 0, 0, 2, 0),
        (222, 2, 2, 0, 2, 4, 0),
        (350, 2, 2, 0, 4, 6, 0),
        (478, 2, 2, 0, 6, 8, 0),
        (606, 2, 2, 0, 8, 10, 0),
        (734, 2, 2, 0, 10, 12, 0),
        (862, 2, 2, 0, 12, 14, 0),
        (990, 2, 2, 0, 14, 16, 0),
        (9_828_836, 4080, 4080, 0, 16, 81, 0),
        (9_930_259, 4096, 4096, 0, 71, 81, 0),
        (9_901_563, 4096, 4096, 0, 69, 80, 0),
        (9_858_485, 4096, 4096, 0, 70, 81, 0),
        (9_940_766, 4096, 4096, 0, 70, 81, 0),
    ],
    &[
        (54_835, 2016, 2016, 0, 0, 1593, 0),
        (136_558, 2048, 2048, 0, 1609, 2574, 0),
        (185_362, 2048, 2048, 0, 2586, 3178, 0),
        (216_471, 2048, 2048, 0, 3189, 3546, 0),
        (234_464, 2048, 2048, 0, 3554, 3757, 0),
        (245_747, 2048, 2048, 0, 3763, 3896, 0),
        (252_493, 2048, 2048, 0, 3899, 3983, 0),
        (256_665, 2048, 2048, 1, 3985, 4031, 0),
        (
            534_735_614,
            4_500_501,
            4_177_920,
            10_123,
            4033,
            4096,
            102_024,
        ),
        (536_846_299, 4_497_765, 4_194_304, 9483, 4093, 4096, 108_557),
        (536_836_260, 4_506_084, 4_194_304, 9751, 4092, 4096, 101_087),
        (536_843_551, 4_499_723, 4_194_304, 9537, 4093, 4096, 105_862),
        (536_842_407, 4_497_635, 4_194_304, 9495, 4093, 4096, 106_379),
    ],
];

/// The controls at 1 024 units, (a) to (c): the same units armed and wired to nothing. Under the
/// two sparser drives the network never fires, so its rows are its control's bit for bit; under
/// ADR-0044's drive the control serves every unit on every tick and fires 1.14 to 1.24 Hz a
/// unit unwired, and the network's own messages bring a unit to rest now and then (its fewest turns
/// in a tick 1 021 or 1 022).
const CONTROL_ROWS: [&[Row]; 3] = [
    &[
        (13_612, 504, 504, 0, 0, 391, 0),
        (33_517, 512, 512, 0, 395, 631, 0),
        (45_472, 512, 512, 0, 635, 782, 0),
        (53_368, 512, 512, 0, 784, 879, 0),
        (58_553, 512, 512, 0, 881, 944, 0),
        (61_576, 512, 512, 0, 944, 977, 0),
        (63_385, 512, 512, 0, 978, 995, 0),
        (63_982, 512, 512, 0, 995, 1004, 0),
        (133_690_894, 1_044_480, 1_044_480, 1555, 1004, 1024, 130_199),
        (134_217_728, 1_048_576, 1_048_576, 1533, 1024, 1024, 131_072),
        (134_217_728, 1_048_576, 1_048_576, 1598, 1024, 1024, 131_072),
        (134_217_728, 1_048_576, 1_048_576, 1634, 1024, 1024, 131_072),
        (134_217_728, 1_048_576, 1_048_576, 1664, 1024, 1024, 131_072),
    ],
    &[
        (1024, 32, 32, 0, 0, 32, 0),
        (3023, 32, 32, 0, 32, 63, 0),
        (4865, 32, 32, 0, 63, 90, 0),
        (6765, 32, 32, 0, 90, 121, 0),
        (8658, 32, 32, 0, 121, 147, 0),
        (10_332, 32, 32, 0, 147, 175, 0),
        (11_996, 32, 32, 0, 175, 201, 0),
        (13_668, 32, 32, 0, 201, 227, 0),
        (95_879_032, 65_280, 65_280, 0, 227, 771, 0),
        (96_634_999, 65_536, 65_536, 0, 704, 767, 0),
        (96_540_672, 65_536, 65_536, 0, 697, 767, 0),
        (96_799_753, 65_536, 65_536, 0, 701, 775, 0),
        (96_517_112, 65_536, 65_536, 0, 709, 770, 0),
    ],
    &[
        (94, 2, 2, 0, 0, 2, 0),
        (222, 2, 2, 0, 2, 4, 0),
        (350, 2, 2, 0, 4, 6, 0),
        (478, 2, 2, 0, 6, 8, 0),
        (606, 2, 2, 0, 8, 10, 0),
        (734, 2, 2, 0, 10, 12, 0),
        (862, 2, 2, 0, 12, 14, 0),
        (990, 2, 2, 0, 14, 16, 0),
        (9_828_836, 4080, 4080, 0, 16, 81, 0),
        (9_930_259, 4096, 4096, 0, 71, 81, 0),
        (9_901_563, 4096, 4096, 0, 69, 80, 0),
        (9_858_485, 4096, 4096, 0, 70, 81, 0),
        (9_940_766, 4096, 4096, 0, 70, 81, 0),
    ],
];

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

    // The first 512 ticks of run (a), held to the first rows of its table: under the census on
    // ADR-0097's two workers, and without it on each worker count a timed run is made on.
    let prefix = [PREFIX_TICKS; PREFIX_ROWS];
    let mut exec = network(1024, WORKERS);
    let (rows, _) = read(&mut exec, &run_drive(0), &prefix, Census::Held);
    assert_eq!(rows, NETWORK_ROWS[0][..PREFIX_ROWS]);
    for workers in [1, WORKERS] {
        let mut exec = network(1024, workers);
        let (rows, _) = read(&mut exec, &run_drive(0), &prefix, Census::Off);
        assert_eq!(rows, NETWORK_ROWS[0][..PREFIX_ROWS], "{workers} workers");
    }
}

// ------------------------------------------------------------------- the runs (weekly)

/// Run `k` on the reference network, and at 1 024 units its control, each dumped before either
/// is held to its table.
fn active_set(k: usize) {
    let (units, _, _) = RUNS[k];
    let drive = run_drive(k);
    let mut exec = network(units, WORKERS);
    let (rows, nanos) = read(&mut exec, &drive, &layout(), Census::Held);
    dump(&format!("network {k}"), k, &rows, nanos, WORKERS);
    let control = CONTROL_ROWS.get(k).map(|&table| {
        let mut exec = unwired(units, WORKERS);
        let (rows, nanos) = read(&mut exec, &drive, &layout(), Census::Held);
        dump(&format!("control {k}"), k, &rows, nanos, WORKERS);
        (rows, table)
    });
    assert_eq!(rows, NETWORK_ROWS[k], "network {k}");
    if let Some((rows, table)) = control {
        assert_eq!(rows, table, "control {k}");
    }
}

/// Run `k` as ADR-0101 times it, on `workers`: the reference network and, at 1 024 units, its
/// control, with no census between the ticks, each dumped before either is held to ADR-0097's
/// table. The rows are the engine's own counts, so a table holds on any worker count.
fn timed(k: usize, workers: usize) {
    let (units, _, _) = RUNS[k];
    let drive = run_drive(k);
    let mut exec = network(units, workers);
    let (rows, nanos) = read(&mut exec, &drive, &layout(), Census::Off);
    let label = format!("{k}, no census, {workers} workers");
    dump(&format!("network {label}"), k, &rows, nanos, workers);
    let control = CONTROL_ROWS.get(k).map(|&table| {
        let mut exec = unwired(units, workers);
        let (rows, nanos) = read(&mut exec, &drive, &layout(), Census::Off);
        dump(&format!("control {label}"), k, &rows, nanos, workers);
        (rows, table)
    });
    assert_eq!(rows, NETWORK_ROWS[k], "network {label}");
    if let Some((rows, table)) = control {
        assert_eq!(rows, table, "control {label}");
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

// ------------------------------------------ the runs as ADR-0101 times them (weekly, brief 044)

#[test]
#[ignore = "whole-domain: run (a) and its control with no census, timed; weekly"]
fn the_reference_drive_at_1024_units_timed_with_no_census_exhaustive() {
    timed(0, WORKERS);
}

#[test]
#[ignore = "whole-domain: run (b) and its control with no census, timed; weekly"]
fn a_drive_sixteen_times_sparser_at_1024_units_timed_with_no_census_exhaustive() {
    timed(1, WORKERS);
}

#[test]
#[ignore = "whole-domain: run (c) and its control with no census, timed; weekly"]
fn a_drive_256_times_sparser_at_1024_units_timed_with_no_census_exhaustive() {
    timed(2, WORKERS);
}

#[test]
#[ignore = "whole-domain: run (d) with no census, timed; weekly"]
fn the_reference_drive_at_4096_units_timed_with_no_census_exhaustive() {
    timed(3, WORKERS);
}

#[test]
#[ignore = "whole-domain: run (a) and its control with no census on one worker, timed; weekly"]
fn the_reference_drive_at_1024_units_timed_with_no_census_on_one_worker_exhaustive() {
    timed(0, 1);
}

#[test]
#[ignore = "whole-domain: run (c) and its control with no census on one worker, timed; weekly"]
fn a_drive_256_times_sparser_at_1024_units_timed_with_no_census_on_one_worker_exhaustive() {
    timed(2, 1);
}
