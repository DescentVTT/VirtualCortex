//! Brief 018's exit test (ADR-0036; whitepaper §8.8): criticality control as the executor
//! composes it. The executor tallies every spike into the homeostasis record's open bin, closes
//! the bin on its cadence and regulates the gain on the window's, exactly as the record's rule
//! says when fed the same counts (an oracle record): a window whose activity grows across its
//! bins lowers the gain, one whose activity decays raises it, one whose activity is a straight
//! line leaves it, and a window at or above the saturation ceiling lowers it whatever its
//! slope. A silent engine's gain climbs to the ceiling and stops there exactly; with a step
//! of zero nothing moves; a sub-critical network's gain rises until spikes beget spikes; the
//! loop is bit-identical on one and four workers; the state round-trips through the image
//! mid-window and a loaded engine continues alike.
//!
//! What this file does not show: the branching ratio settling at 1 on a network. The
//! estimator reads the lag-one slope of population activity across bins, which is the
//! branching ratio when the activity is a stationary branching process observed at its
//! generation time (Wilting and Priesemann 2018); a 64-unit network with depleting synapses
//! produces avalanches that finish inside one bin, which the slope reads as sub-critical. The
//! regime is the reference population's (whitepaper hypothesis H-8).

#![deny(clippy::arithmetic_side_effects)]

use cortex_core::{STP_MAX, STP_U, THRESHOLD_BASE, spike_message, synaptic_efficacy_q16};
use cortex_homeostasis::{
    ACTIVITY_BIN_SHIFT, ACTIVITY_WINDOW_BINS, ACTIVITY_WINDOW_SHIFT, CONTROL_STEP_MAX_Q0_16,
    GAIN_MAX_Q16, GAIN_ONE_Q16, HomeostaticDrivePool, SIGMA_MAX_Q16,
};
use cortex_runtime::{Config, ConfigError, Executor, Image, Inject};

const UNITS: usize = 64;
const BIN: u64 = 1 << ACTIVITY_BIN_SHIFT;
const WINDOW: u64 = 1 << (ACTIVITY_BIN_SHIFT.wrapping_add(ACTIVITY_WINDOW_SHIFT));
/// The control step of every closed-loop run here: an eighth.
const STEP: u16 = 0x2000;

fn config(workers: usize, blocks: usize, step: u16) -> Config {
    Config {
        workers,
        units: UNITS,
        blocks,
        nodes_per_worker: 1 << 14,
        injector_capacity: 4096,
        trace_capacity: 1 << 20,
        control_step_q0_16: step,
        ..Config::default()
    }
}

/// Every unit armed to fire.
fn arm(exec: &mut Executor<64>) {
    for unit in exec.units_mut() {
        unit.v_thresh = THRESHOLD_BASE;
        unit.stp_u_rel = STP_U;
        unit.stp_r_ves = STP_MAX;
    }
}

/// Three messages that make an armed unit at rest fire on the tick after the next.
fn kick(inject: &Inject, unit: u32) {
    let strong = synaptic_efficacy_q16(i16::MAX, STP_MAX, STP_MAX);
    for _ in 0..3 {
        inject.inject(unit, spike_message(strong, false)).unwrap();
    }
}

/// A pseudo-random recurrent network: every unit two blocks, eight synapses to two random
/// targets (four each, at one delay per target near the wheel's horizon, so that a cascade
/// advances about one generation per bin), weights near the rail. At a gain of 1.0 a spike
/// begets almost none: four coincident releases at rest fall just short of the threshold;
/// above about 1.25 they do not.
fn wire_recurrent(exec: &mut Executor<64>) {
    let mut x = 0x9E37_79B9u32;
    let mut next = || {
        x = x.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
        x
    };
    {
        let blocks = exec.blocks_mut();
        for i in 0..UNITS {
            for t in 0..2usize {
                let mut target = (next() >> 8)
                    .checked_rem(UNITS as u32)
                    .expect("UNITS is not zero");
                if target as usize == i {
                    target = target
                        .wrapping_add(1)
                        .checked_rem(UNITS as u32)
                        .unwrap_or(0);
                }
                let delay = ((next() >> 8) % 559).wrapping_add(2_000) as u16;
                let block = if t == 0 { i } else { UNITS.wrapping_add(i) };
                for slot in 0..4 {
                    let weight = ((next() >> 8) % 4_000).wrapping_add(28_000) as i16;
                    assert!(blocks[block].set_synapse(slot, target, weight, delay, false));
                }
            }
            assert!(blocks[i].link(UNITS.wrapping_add(i) as u32));
        }
    }
    arm(exec);
    for (i, unit) in exec.units_mut().iter_mut().enumerate() {
        assert!(unit.set_first_block(i as u32));
    }
}

/// Runs the engine to the end of its current window with a kick to one unit every
/// `drive_every` ticks, units in turn; returns `(kicks, spikes)` of the window, the spikes
/// read from the record on the window's last tick, before it regulates.
fn drive_window(exec: &mut Executor<64>, drive_every: u64, next_unit: &mut u32) -> (u64, u64) {
    let inject = exec.injector();
    let mut kicks = 0u64;
    loop {
        let tick = exec.ticks();
        if tick.checked_rem(drive_every) == Some(0) {
            kick(&inject, *next_unit);
            *next_unit = next_unit
                .wrapping_add(1)
                .checked_rem(UNITS as u32)
                .unwrap_or(0);
            kicks = kicks.wrapping_add(1);
        }
        if tick.wrapping_add(1) % WINDOW == 0 {
            let h = exec.homeostasis();
            let spikes = h
                .sum_prev
                .wrapping_add(h.last_activity as u64)
                .wrapping_add(h.bin_activity as u64);
            exec.tick();
            return (kicks, spikes);
        }
        exec.tick();
    }
}

/// One window of designed activity on a network without synapses: bin `b` gets `counts[b]`
/// kicks to distinct units early in the bin, so that exactly that many units fire in it (an
/// isolated kick is one spike); the oracle record is fed the same counts. Both regulate at
/// the window's end, by the estimate under the saturation rule.
fn designed_window(
    exec: &mut Executor<64>,
    oracle: &mut HomeostaticDrivePool,
    counts: &[u32; ACTIVITY_WINDOW_BINS as usize],
) {
    designed_window_under(exec, oracle, counts, None);
}

/// `designed_window` with the rule the oracle regulates by named (ADR-0180): the count over
/// `target` where one is given, the rate's rule; the estimate under the saturation rule where
/// none is, as `designed_window` calls it.
fn designed_window_under(
    exec: &mut Executor<64>,
    oracle: &mut HomeostaticDrivePool,
    counts: &[u32; ACTIVITY_WINDOW_BINS as usize],
    target: Option<u32>,
) {
    let inject = exec.injector();
    assert_eq!(exec.ticks() % WINDOW, 0, "at a window boundary");
    for &count in counts {
        let start = exec.ticks();
        assert!(
            count as usize <= UNITS,
            "one kick per unit per bin: a second within the bin lands on a unit still charged and begets a spike of its own"
        );
        for j in 0..count {
            // Kick `j` at tick `start + 10 + 30 j`: the last one 2 000 ticks before the bin
            // ends, so that every spike lands in this bin.
            while exec.ticks()
                < start
                    .wrapping_add(10)
                    .wrapping_add(30u64.wrapping_mul(j as u64))
            {
                exec.tick();
            }
            kick(&inject, j);
        }
        while exec.ticks() < start.wrapping_add(BIN).wrapping_sub(1) {
            exec.tick();
        }
        assert_eq!(
            exec.homeostasis().bin_activity,
            count,
            "the open bin holds exactly the units that were kicked (bin {})",
            oracle.window_bins
        );
        exec.tick();
        oracle.count_activity(count);
        assert!(oracle.close_bin().is_some());
    }
    match target {
        Some(target) => oracle.regulate_rate(target),
        None => oracle.regulate(UNITS as u32),
    };
    // The window's cadence also steps the sleep stage (ADR-0037): with the shift at 0 only
    // the circadian phase moves.
    oracle.step_sleep();
}

#[test]
fn the_executor_tallies_every_spike_and_regulates_exactly_as_the_record_s_rule_says() {
    let mut exec = Executor::<64>::new(config(2, 0, STEP)).unwrap();
    arm(&mut exec);
    let mut oracle = HomeostaticDrivePool {
        control_step_q0_16: STEP,
        ..HomeostaticDrivePool::new()
    };
    assert_eq!(*exec.homeostasis(), oracle, "at rest, with the step");
    let bins = ACTIVITY_WINDOW_BINS as usize;

    // Growth by a tenth per bin: the slope is above 1, the gain falls.
    let mut growth = [0u32; ACTIVITY_WINDOW_BINS as usize];
    let mut x = 3_000u64;
    for c in growth.iter_mut() {
        *c = (x / 1_000) as u32;
        x = x.wrapping_mul(11) / 10;
    }
    designed_window(&mut exec, &mut oracle, &growth);
    assert_eq!(*exec.homeostasis(), oracle, "the growth window");
    let after_growth = exec.homeostasis().synaptic_gain_q16;
    assert!(
        exec.homeostasis().branching_ratio_q16 > GAIN_ONE_Q16 && after_growth < GAIN_ONE_Q16,
        "growing activity reads as supercritical: sigma {:#x}, gain {after_growth:#x}",
        exec.homeostasis().branching_ratio_q16
    );

    // Decay by a tenth per bin: the slope is below 1, the gain rises.
    let mut decay = [0u32; ACTIVITY_WINDOW_BINS as usize];
    let mut x = 40_000u64;
    for c in decay.iter_mut() {
        *c = (x / 1_000) as u32;
        x = x.wrapping_mul(9) / 10;
    }
    designed_window(&mut exec, &mut oracle, &decay);
    assert_eq!(*exec.homeostasis(), oracle, "the decay window");
    let after_decay = exec.homeostasis().synaptic_gain_q16;
    assert!(
        exec.homeostasis().branching_ratio_q16 < GAIN_ONE_Q16 && after_decay > after_growth,
        "decaying activity reads as sub-critical: sigma {:#x}",
        exec.homeostasis().branching_ratio_q16
    );

    // A straight line: the slope is exactly 1, the gain stays.
    let mut line = [0u32; ACTIVITY_WINDOW_BINS as usize];
    for (b, c) in line.iter_mut().enumerate() {
        *c = b as u32 + 1;
    }
    designed_window(&mut exec, &mut oracle, &line);
    assert_eq!(*exec.homeostasis(), oracle, "the linear window");
    assert_eq!(
        exec.homeostasis().branching_ratio_q16,
        GAIN_ONE_Q16,
        "a slope of exactly 1"
    );
    assert_eq!(
        exec.homeostasis().synaptic_gain_q16,
        after_decay,
        "critical: no move"
    );

    // Saturation: a unit per bin on average, a slope of zero; read as supercritical.
    let saturated = [UNITS as u32; ACTIVITY_WINDOW_BINS as usize];
    designed_window(&mut exec, &mut oracle, &saturated);
    assert_eq!(*exec.homeostasis(), oracle, "the saturated window");
    assert_eq!(exec.homeostasis().branching_ratio_q16, SIGMA_MAX_Q16);
    assert_eq!(
        exec.homeostasis().synaptic_gain_q16,
        ((after_decay as u64 * 7 + 4) / 8) as u32,
        "an eighth off, rounded to nearest"
    );
    // One below the ceiling on average: the slope regulates, not the ceiling. The two dips
    // at the ends make the series read as a slope just below zero: no descendants, an eighth
    // up.
    let mut below = saturated;
    below[0] = UNITS as u32 - 1;
    below[bins - 1] = UNITS as u32 - 1;
    let before = exec.homeostasis().synaptic_gain_q16;
    designed_window(&mut exec, &mut oracle, &below);
    assert_eq!(
        *exec.homeostasis(),
        oracle,
        "the window just below the ceiling"
    );
    assert_eq!(exec.homeostasis().branching_ratio_q16, 0);
    assert_eq!(
        exec.homeostasis().synaptic_gain_q16,
        ((before as u64 * 9 + 4) / 8) as u32
    );
    assert_eq!(exec.ticks(), 5 * WINDOW);
}

#[test]
fn a_silent_engine_s_gain_climbs_to_the_ceiling_and_stops_there_exactly() {
    let mut exec = Executor::<64>::new(config(1, 0, CONTROL_STEP_MAX_Q0_16)).unwrap();
    let mut gains = Vec::new();
    for _ in 0..6 {
        exec.run(WINDOW);
        gains.push(exec.homeostasis().synaptic_gain_q16);
    }
    assert_eq!(
        gains,
        vec![
            0x0001_8000,
            0x0002_4000,
            0x0003_6000,
            GAIN_MAX_Q16,
            GAIN_MAX_Q16,
            GAIN_MAX_Q16
        ],
        "half more per window, then the ceiling"
    );
    assert_eq!(
        exec.homeostasis().branching_ratio_q16,
        0,
        "silence is no descendants"
    );
    // Sixteen windows short of the boundary the gain has not moved yet: the cadence is a mask
    // on the tick, not a count of calls.
    let mut late = Executor::<64>::new(config(1, 0, CONTROL_STEP_MAX_Q0_16)).unwrap();
    late.run(WINDOW - 1);
    assert_eq!(late.homeostasis().synaptic_gain_q16, GAIN_ONE_Q16);
    assert_eq!(late.homeostasis().window_bins, ACTIVITY_WINDOW_BINS - 1);
    late.tick();
    assert_eq!(late.homeostasis().synaptic_gain_q16, 0x0001_8000);
    assert_eq!(late.homeostasis().window_bins, 0);
}

#[test]
fn a_sub_critical_network_s_gain_rises_until_spikes_beget_spikes_and_a_step_of_zero_moves_nothing()
{
    let mut exec = Executor::<64>::new(config(1, 2 * UNITS, STEP)).unwrap();
    wire_recurrent(&mut exec);
    let mut unit = 0u32;
    let (kicks, spikes) = drive_window(&mut exec, 6_000, &mut unit);
    assert!(
        spikes >= kicks && spikes <= kicks.wrapping_add(2),
        "at a gain of 1.0 a spike begets almost none: {spikes} spikes for {kicks} kicks"
    );
    assert_eq!(exec.homeostasis().branching_ratio_q16, 0);
    assert_eq!(
        exec.homeostasis().synaptic_gain_q16,
        0x0001_2000,
        "an eighth up"
    );
    let mut rows = Vec::new();
    for _ in 0..7 {
        let (kicks, spikes) = drive_window(&mut exec, 6_000, &mut unit);
        let h = exec.homeostasis();
        rows.push((kicks, spikes, h.synaptic_gain_q16, h.branching_ratio_q16));
    }
    let (kicks, spikes, gain, sigma) = rows[rows.len() - 1];
    assert!(
        spikes > 4 * kicks,
        "eight windows in, every kick begets several spikes: {spikes} for {kicks}"
    );
    assert!(
        gain > 0x0001_8000 && gain < 0x0003_0000,
        "the gain rose while the slope read sub-critical and slowed as it rose: {gain:#x}"
    );
    assert!(sigma > 0x4000, "descendants across bins: {sigma:#x}");
    assert!(
        rows.iter()
            .all(|&(_, s, _, _)| s < (UNITS as u64) * (ACTIVITY_WINDOW_BINS as u64) / 4),
        "far below saturation: the synapses deplete"
    );

    // The same network with a step of zero: the gain stays at 1.0, the estimator still runs.
    let mut frozen = Executor::<64>::new(config(1, 2 * UNITS, 0)).unwrap();
    wire_recurrent(&mut frozen);
    let mut unit = 0u32;
    for _ in 0..3 {
        let (kicks, spikes) = drive_window(&mut frozen, 6_000, &mut unit);
        assert!(spikes >= kicks && spikes <= kicks.wrapping_add(2));
        assert_eq!(frozen.homeostasis().synaptic_gain_q16, GAIN_ONE_Q16);
    }
    assert_eq!(
        frozen.homeostasis().branching_ratio_q16,
        0,
        "measured, not acted on"
    );
}

#[test]
fn the_loop_is_bit_identical_on_one_and_four_workers() {
    let outcome = |workers: usize| {
        let mut exec = Executor::<64>::new(config(workers, 2 * UNITS, STEP)).unwrap();
        wire_recurrent(&mut exec);
        let mut unit = 0u32;
        for _ in 0..4 {
            drive_window(&mut exec, 6_000, &mut unit);
        }
        let pool = *exec.homeostasis();
        let units: Vec<[u8; 64]> = exec.units().iter().map(|u| u.encode()).collect();
        let blocks = exec.blocks().to_vec();
        let reports = exec.shutdown();
        let mut spikes: Vec<(u32, u32)> = reports
            .iter()
            .flat_map(|r| r.spikes.iter().map(|&(u, t)| (t, u)))
            .collect();
        spikes.sort_unstable();
        (pool, units, blocks, spikes)
    };
    let one = outcome(1);
    let four = outcome(4);
    assert!(one.0.synaptic_gain_q16 != GAIN_ONE_Q16, "the loop acted");
    assert_eq!(one.0, four.0, "the homeostasis record: the tally is a sum");
    assert_eq!(one.1, four.1, "the unit arenas");
    assert_eq!(one.2, four.2, "the synapse arenas");
    assert_eq!(one.3, four.3, "the spike trains");
    assert!(one.3.len() > 100);
}

#[test]
fn the_homeostasis_state_round_trips_mid_window_and_a_loaded_engine_continues_alike() {
    let mut exec = Executor::<64>::new(config(2, 2 * UNITS, STEP)).unwrap();
    wire_recurrent(&mut exec);
    let mut unit = 0u32;
    for _ in 0..2 {
        drive_window(&mut exec, 6_000, &mut unit);
    }
    // Into the third window, then quiet until every token has landed and every mailbox is
    // drained: the state an image is written from.
    let inject = exec.injector();
    for _ in 0..5 {
        exec.run(3_000);
        kick(&inject, unit);
        unit = unit.wrapping_add(1);
    }
    let mut waited = 0u64;
    while !exec.is_quiescent() {
        exec.tick();
        waited = waited.wrapping_add(1);
        assert!(waited < 40_000, "the network falls quiet");
    }
    let written = *exec.homeostasis();
    assert!(
        written.window_bins > 0 && written.window_bins < ACTIVITY_WINDOW_BINS,
        "mid-window: {} bins",
        written.window_bins
    );
    assert!(
        written.sum_prev > 0 && written.synaptic_gain_q16 != GAIN_ONE_Q16,
        "a window under way, a gain that moved"
    );
    let image = Image::encode(&exec).unwrap();
    let mut loaded = Image::decode::<64>(&image, config(2, 0, 0)).unwrap();
    assert_eq!(*loaded.homeostasis(), written, "read back whole");
    assert_eq!(
        loaded.homeostasis().control_step_q0_16,
        STEP,
        "the image's step outranks the configuration's zero"
    );
    assert_eq!(loaded.ticks(), exec.ticks());
    // Both finish the window with the same kicks, then run another: the same regulation.
    let mut unit_copy = unit;
    for _ in 0..2 {
        drive_window(&mut exec, 6_000, &mut unit);
        drive_window(&mut loaded, 6_000, &mut unit_copy);
        assert_eq!(loaded.homeostasis(), exec.homeostasis());
    }
    assert_ne!(
        exec.homeostasis().synaptic_gain_q16,
        written.synaptic_gain_q16,
        "the loop went on"
    );
    assert_eq!(
        loaded
            .units()
            .iter()
            .map(|u| u.encode())
            .collect::<Vec<_>>(),
        exec.units().iter().map(|u| u.encode()).collect::<Vec<_>>()
    );
}

#[test]
fn the_control_step_is_refused_above_its_bound_and_the_default_is_zero() {
    assert!(matches!(
        Executor::<64>::new(config(1, 0, CONTROL_STEP_MAX_Q0_16 + 1)),
        Err(ConfigError::ControlStepOutOfRange)
    ));
    assert!(Executor::<64>::new(config(1, 0, CONTROL_STEP_MAX_Q0_16)).is_ok());
    let exec = Executor::<64>::new(Config {
        units: 1,
        ..Config::default()
    })
    .unwrap();
    assert_eq!(exec.homeostasis(), &HomeostaticDrivePool::new());
    assert_eq!(Config::default().control_step_q0_16, 0);
}

/// The period whose count for `UNITS` units is `RATE_TARGET` spikes a window: 64 units times
/// 131 072 ticks over 26 214 are 320.005.
const RATE_PERIOD: u32 = 26_214;
/// Ten spikes a bin.
const RATE_TARGET: u32 = 320;

fn rate_config(workers: usize, blocks: usize, step: u16) -> Config {
    Config {
        rate_target_period_ticks: RATE_PERIOD,
        ..config(workers, blocks, step)
    }
}

/// The rate's rule as the executor composes it (ADR-0180): with a target set, the executor
/// regulates each window by its count over the target's, exactly as the record's rule says when
/// fed the same counts, and the saturation rule and the estimate move nothing. Beside the
/// engine, a record fed the same counts and regulated by the estimate under the saturation rule,
/// which is what the executor does with no target
/// (`the_executor_tallies_every_spike_and_regulates_exactly_as_the_record_s_rule_says`): a
/// decaying window above its target lowers the gain where the estimate's rule raises it, a
/// growing one below it raises the gain where the estimate's rule lowers it, and a window at
/// one spike a unit a bin leaves the estimate it had. Each gain is the hand rule's. The windows
/// are ordered so that the engine's gain is at most 1.0 in each: above it a kick fires its unit
/// twice, and a designed window would not hold its counts.
#[test]
fn under_a_rate_s_target_the_executor_regulates_by_the_window_s_count_exactly_as_the_rule_says() {
    let mut exec = Executor::<64>::new(rate_config(2, 0, STEP)).unwrap();
    arm(&mut exec);
    assert_eq!(
        (
            exec.rate_target_period_ticks(),
            exec.rate_target_per_window()
        ),
        (RATE_PERIOD, Some(RATE_TARGET))
    );
    let mut oracle = HomeostaticDrivePool {
        control_step_q0_16: STEP,
        ..HomeostaticDrivePool::new()
    };
    assert_eq!(*exec.homeostasis(), oracle, "at rest, with the step");
    let mut by_estimate = oracle;
    // One window on the engine and on the two records; returns the gain under each rule.
    let mut window = |counts: &[u32; ACTIVITY_WINDOW_BINS as usize], what: &str| {
        designed_window_under(&mut exec, &mut oracle, counts, Some(RATE_TARGET));
        assert_eq!(*exec.homeostasis(), oracle, "{what}: the rate's rule");
        for &count in counts {
            by_estimate.count_activity(count);
            assert!(by_estimate.close_bin().is_some());
        }
        by_estimate.regulate(UNITS as u32);
        by_estimate.step_sleep();
        (
            exec.homeostasis().synaptic_gain_q16,
            exec.homeostasis().branching_ratio_q16,
            by_estimate.synaptic_gain_q16,
            by_estimate.branching_ratio_q16,
        )
    };

    // At its target, ten spikes in every bin: the gain stays. A constant window has no slope,
    // so the estimate's rule moves nothing either.
    let at_target = [10u32; ACTIVITY_WINDOW_BINS as usize];
    assert_eq!(at_target.iter().sum::<u32>(), RATE_TARGET);
    assert_eq!(
        window(&at_target, "at the target"),
        (GAIN_ONE_Q16, 0, GAIN_ONE_Q16, 0)
    );

    // Decay by a tenth a bin from forty spikes, 373 spikes: the slope reads below one and the
    // count above its target. The rate's rule lowers the gain, by an eighth of 373 over 320
    // less one; the estimate's raises it.
    let mut decay = [0u32; ACTIVITY_WINDOW_BINS as usize];
    let mut x = 40_000u64;
    for c in decay.iter_mut() {
        *c = (x / 1_000) as u32;
        x = x.wrapping_mul(9) / 10;
    }
    assert_eq!(decay.iter().sum::<u32>(), 373);
    let (gain, slope, other, other_slope) = window(&decay, "decay above the target");
    assert_eq!(gain, 64_179, "1 - (76 390 - 65 536) / 8, to nearest");
    assert!(slope < GAIN_ONE_Q16 && slope == other_slope, "{slope:#x}");
    assert!(other > GAIN_ONE_Q16, "the estimate's rule raises it");

    // One spike a unit a bin, the saturation rule's count, 6.4 times the target: both rules
    // take an eighth off. The estimate's rule stores its ceiling; the rate's rule does not
    // consult the saturation rule, and a constant window has no slope, so the record keeps the
    // estimate the decay left.
    let saturated = [UNITS as u32; ACTIVITY_WINDOW_BINS as usize];
    let (gain, kept, after, ceiling) = window(&saturated, "at the saturation rule's count");
    assert_eq!(gain, 56_157, "seven eighths of 64 179, to nearest");
    assert_eq!(after, ((other as u64 * 7 + 4) / 8) as u32);
    assert_eq!((kept, ceiling), (slope, SIGMA_MAX_Q16));

    // Growth by a tenth a bin from one spike, 240 spikes, three quarters of the target: the
    // slope reads above one and the count below its target. The rate's rule raises the gain, by
    // a quarter of an eighth; the estimate's lowers it.
    let mut growth = [0u32; ACTIVITY_WINDOW_BINS as usize];
    let mut x = 1_265u64;
    for c in growth.iter_mut() {
        *c = (x / 1_000) as u32;
        x = x.wrapping_mul(11) / 10;
    }
    assert_eq!(growth.iter().sum::<u32>(), 240);
    let before = after;
    let (gain, slope, other, other_slope) = window(&growth, "growth below the target");
    assert_eq!(gain, 57_912, "56 157 and a thirty-second of it, to nearest");
    assert!(slope > GAIN_ONE_Q16 && slope == other_slope, "{slope:#x}");
    assert!(other < before, "the estimate's rule lowers it");

    // No spike: an eighth up under either rule, and the estimate reads no descendants.
    let silent = [0u32; ACTIVITY_WINDOW_BINS as usize];
    let (gain, slope, after, other_slope) = window(&silent, "silent");
    assert_eq!(gain, 65_151, "nine eighths of 57 912");
    assert_eq!(after, ((other as u64 * 9 + 4) / 8) as u32);
    assert_eq!((slope, other_slope), (0, 0));
    assert_eq!(exec.ticks(), 5 * WINDOW);
}

/// The target's count is made of the period and the engine's units (ADR-0180): H-38's period,
/// 58 201 ticks, at the three sizes the tree runs networks at.
#[test]
fn the_rate_s_target_s_count_is_made_of_the_period_at_256_1024_and_2048_units() {
    for (units, count) in [(256usize, 577u32), (1024, 2_306), (2048, 4_612)] {
        let exec = Executor::<64>::new(Config {
            units,
            control_step_q0_16: STEP,
            rate_target_period_ticks: 58_201,
            ..Config::default()
        })
        .unwrap();
        assert_eq!(
            (
                exec.rate_target_period_ticks(),
                exec.rate_target_per_window()
            ),
            (58_201, Some(count)),
            "{units} units"
        );
    }
}

/// Under the rate's rule a silent engine's gain climbs as under the estimate's, a step up a
/// window to the ceiling; a step of zero moves nothing whatever the count; and the loop is
/// bit-identical on one and four workers.
#[test]
fn under_a_rate_s_target_a_silent_engine_climbs_a_step_of_zero_moves_nothing_and_workers_agree() {
    let mut exec = Executor::<64>::new(rate_config(1, 0, CONTROL_STEP_MAX_Q0_16)).unwrap();
    let mut gains = Vec::new();
    for _ in 0..5 {
        exec.run(WINDOW);
        gains.push(exec.homeostasis().synaptic_gain_q16);
    }
    assert_eq!(
        gains,
        vec![
            0x0001_8000,
            0x0002_4000,
            0x0003_6000,
            GAIN_MAX_Q16,
            GAIN_MAX_Q16
        ],
        "half more per window, then the ceiling"
    );
    // A step of zero: a driven network far from its target, and the gain stays at 1.0.
    let mut held = Executor::<64>::new(rate_config(1, 2 * UNITS, 0)).unwrap();
    wire_recurrent(&mut held);
    let mut unit = 0u32;
    for _ in 0..2 {
        let (kicks, spikes) = drive_window(&mut held, 6_000, &mut unit);
        assert!(spikes >= kicks && spikes < u64::from(RATE_TARGET) / 4);
        assert_eq!(held.homeostasis().synaptic_gain_q16, GAIN_ONE_Q16);
    }
    // One and four workers: the tally is a sum, whichever rule reads it.
    let outcome = |workers: usize| {
        let mut exec = Executor::<64>::new(rate_config(workers, 2 * UNITS, STEP)).unwrap();
        wire_recurrent(&mut exec);
        let mut unit = 0u32;
        let mut rows = Vec::new();
        for _ in 0..4 {
            let (_, spikes) = drive_window(&mut exec, 6_000, &mut unit);
            rows.push((spikes, exec.homeostasis().synaptic_gain_q16));
        }
        let pool = *exec.homeostasis();
        let units: Vec<[u8; 64]> = exec.units().iter().map(|u| u.encode()).collect();
        (rows, pool, units)
    };
    let one = outcome(1);
    assert_eq!(one, outcome(4));
    assert!(
        one.0.windows(2).all(|pair| pair[1].1 > pair[0].1) && one.0[0].1 > GAIN_ONE_Q16,
        "below its target in every window, the gain rises in every window: {:?}",
        one.0
    );
}

/// The rate's target round-trips through the image mid-window (ADR-0180): a loaded engine holds
/// the image's period, whatever its configuration says, and regulates the window it was written
/// in and the next as the engine that wrote it does.
#[test]
fn the_rate_s_target_round_trips_mid_window_and_a_loaded_engine_regulates_alike() {
    let mut exec = Executor::<64>::new(rate_config(2, 2 * UNITS, STEP)).unwrap();
    wire_recurrent(&mut exec);
    let mut unit = 0u32;
    for _ in 0..2 {
        drive_window(&mut exec, 6_000, &mut unit);
    }
    let inject = exec.injector();
    for _ in 0..5 {
        exec.run(3_000);
        kick(&inject, unit);
        unit = unit.wrapping_add(1);
    }
    let mut waited = 0u64;
    while !exec.is_quiescent() {
        exec.tick();
        waited = waited.wrapping_add(1);
        assert!(waited < 40_000, "the network falls quiet");
    }
    let written = *exec.homeostasis();
    assert!(written.window_bins > 0 && written.synaptic_gain_q16 != GAIN_ONE_Q16);
    let image = Image::encode(&exec).unwrap();
    let mut loaded = Image::decode::<64>(&image, config(2, 0, 0)).unwrap();
    assert_eq!(*loaded.homeostasis(), written, "read back whole");
    assert_eq!(
        (
            loaded.rate_target_period_ticks(),
            loaded.rate_target_per_window()
        ),
        (RATE_PERIOD, Some(RATE_TARGET)),
        "the image's target outranks the configuration's none"
    );
    // Both finish the window with the same kicks, then run another: the same regulation.
    let mut unit_copy = unit;
    for _ in 0..2 {
        drive_window(&mut exec, 6_000, &mut unit);
        drive_window(&mut loaded, 6_000, &mut unit_copy);
        assert_eq!(loaded.homeostasis(), exec.homeostasis());
    }
    assert_ne!(
        exec.homeostasis().synaptic_gain_q16,
        written.synaptic_gain_q16,
        "the loop went on"
    );
}
