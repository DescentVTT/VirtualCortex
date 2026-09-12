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
//! generation time (Wilting and Priesemann 2018); a 48-unit network with depleting synapses
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
/// the window's end.
fn designed_window(
    exec: &mut Executor<64>,
    oracle: &mut HomeostaticDrivePool,
    counts: &[u32; ACTIVITY_WINDOW_BINS as usize],
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
    oracle.regulate(UNITS as u32);
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
