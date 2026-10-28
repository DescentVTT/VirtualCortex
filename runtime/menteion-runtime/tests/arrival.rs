//! Brief 084 (ADR-0224, ADR-0225): a unit served on arrival, held to per-tick service bit for
//! bit. A unit that is not at rest and cannot fire with no input
//! (`DendriticSuperNeuron::quiet_cannot_fire`) leaves the schedule at the end of its turn,
//! asleep, and is brought up to date (`quiet_catch_up`) before its next turn and before the
//! arena is read; a write of the arena puts it back on the schedule first.
//!
//! The differential test runs the reference network of ADR-0097 at 256 units beside the same
//! network served per tick, the tests' reference (`Executor::per_tick`), from one image, and
//! holds every unit's 64 bytes and every block's, the spikes and the messages delivered, the one
//! to the other: after every tick for 2 048 ticks, and every 4 999 ticks over 32 768, so that
//! gaps longer than a tick are caught up, under ADR-0044's drive and under ADR-0097's drive 256
//! times sparser, where units reach rest asleep. At each run's end its turns and the steps it
//! caught up are the reference's turns. The tests after it hold, each on one unit beside its
//! per-tick twin, what a read, a write, the sweep and the slow current do to an asleep unit.

#![deny(clippy::arithmetic_side_effects)]

#[allow(dead_code, unused_imports)]
#[path = "instrument/harness.rs"]
mod harness;
use harness::*;

use menteion_core::{FLAG_SLOW, GateState, SlowCurrent};
use std::path::PathBuf;

/// The bytes of a unit the two services are held to: its 64 bytes, atomics as plain values, the
/// mailbox head but as whether the mailbox holds a message. The head is a node index from a
/// worker's pool, whose order the two workers' frees in one phase set (ADR-0099's scheduler
/// bytes): two runs of one engine on two workers need not share it (exploration 008).
fn unit_bytes(u: &DendriticSuperNeuron) -> [u8; 64] {
    let mut bytes = u.encode();
    bytes[8..16].copy_from_slice(&u64::from(!u.mailbox_is_empty()).to_le_bytes());
    bytes
}

/// True when the two engines' arenas hold the same bytes, every unit's as `unit_bytes` reads it
/// and every block's; the first unit and block that differ are written out otherwise.
fn same(per_tick: &Engine, arrival: &Engine) -> bool {
    let units = per_tick.units().iter().zip(arrival.units());
    let unit = units
        .enumerate()
        .find(|(_, (a, b))| unit_bytes(a) != unit_bytes(b));
    if let Some((i, (a, b))) = unit {
        eprintln!("unit {i}: per tick {a:?}");
        eprintln!("unit {i}: on arrival {b:?}");
    }
    // A block holds no atomic, so its fields are its bytes.
    let blocks = per_tick.blocks() == arrival.blocks();
    if !blocks {
        let pairs = per_tick.blocks().iter().zip(arrival.blocks());
        if let Some((i, (a, b))) = pairs.enumerate().find(|(_, (a, b))| a != b) {
            eprintln!("block {i}: per tick {a:?}");
            eprintln!("block {i}: on arrival {b:?}");
        }
    }
    unit.is_none() && blocks
}

/// The spikes so far, from the executor's own train (ADR-0050).
fn spikes(exec: &mut Engine) -> u64 {
    (exec.train().len() as u64).saturating_add(exec.train_overwritten())
}

/// The reference network at `units` on two workers, ADR-0097's: ADR-0044's prior synthesized at
/// the instrument's gain, the controller off and the modulation baseline zero.
fn network(units: u32) -> Engine {
    at_gain(&prior(units), config(units, 2, 0), GAIN_1024)
}

/// ADR-0097's drive 256 times sparser, its run (c): one message every 32nd tick.
fn sparse(units: u32) -> Drive {
    Drive {
        every: 32,
        messages: 1,
        ..drive(units)
    }
}

/// What a run counted: the turns on arrival served and the steps it caught up, and the turns
/// per-tick service served.
#[derive(Debug)]
struct Counted {
    served: u64,
    caught_up: u64,
    per_tick: u64,
}

/// Runs the network at `units` under `drive` for `ticks` ticks twice, built alike, served per
/// tick and on arrival, and holds the two to each other after every tick `read` admits: the
/// arena, the spikes so far and the messages delivered, and the arena again at the end.
fn held(units: u32, drive: &Drive, ticks: u64, read: impl Fn(u64) -> bool) -> Counted {
    let mut per_tick = network(units).per_tick();
    let mut arrival = network(units);
    assert!(same(&per_tick, &arrival), "the same network");
    let (inject_per_tick, inject_arrival) = (per_tick.injector(), arrival.injector());
    let mut readings = 0u32;
    for k in 0..ticks {
        let tick = per_tick.ticks();
        assert_eq!(tick, arrival.ticks());
        drive.step(&inject_per_tick, tick).expect("the drive runs");
        drive.step(&inject_arrival, tick).expect("the drive runs");
        per_tick.tick();
        arrival.tick();
        if read(k) {
            readings = readings.saturating_add(1);
            assert_eq!(
                (spikes(&mut arrival), arrival.delivered()),
                (spikes(&mut per_tick), per_tick.delivered()),
                "tick {tick}: the spikes and the messages"
            );
            assert!(same(&per_tick, &arrival), "tick {tick}: the arena");
        }
    }
    assert!(readings > 0);
    // Every asleep unit brought up to date by the read, so that its steps are counted.
    assert!(same(&per_tick, &arrival), "the arena at the end");
    assert_eq!(
        per_tick.caught_up(),
        0,
        "per-tick service catches nothing up"
    );
    Counted {
        served: arrival.turns(),
        caught_up: arrival.caught_up(),
        per_tick: per_tick.turns(),
    }
}

/// A run's counts: units slept and were caught up, the turns on arrival served were fewer than
/// per-tick service's, and with the steps caught up at a turn and at a read they are its turns.
fn counted(what: &str, c: &Counted) {
    eprintln!("COUNT {what}: {c:?}");
    assert!(
        c.caught_up > 0 && c.served < c.per_tick,
        "{what}: units slept: {c:?}"
    );
    assert_eq!(
        c.served.saturating_add(c.caught_up),
        c.per_tick,
        "{what}: the turns of per-tick service, served or caught up"
    );
}

#[test]
fn the_reference_network_served_on_arrival_is_per_tick_service_after_every_tick() {
    for (what, d) in [
        ("ADR-0044's drive", drive(256)),
        ("a drive 256 times sparser", sparse(256)),
    ] {
        let c = held(256, &d, 1 << 11, |_| true);
        counted(&format!("{what}, every tick"), &c);
    }
}

#[test]
fn the_reference_network_served_on_arrival_is_per_tick_service_at_ticks_far_apart() {
    for (what, d) in [
        ("ADR-0044's drive", drive(256)),
        ("a drive 256 times sparser", sparse(256)),
    ] {
        let c = held(256, &d, 1 << 15, |k| k % 4_999 == 4_998);
        counted(&format!("{what}, every 4 999 ticks"), &c);
    }
}

// ------------------------------------------------------------- one unit beside its twin

/// One armed unit at rest with no synapse, at the instrument's gain, and its twin served per
/// tick.
fn one_unit(config: Config) -> (Engine, Engine) {
    let mut exec = Engine::new(config.clone()).unwrap();
    exec.units_mut()[0].v_thresh = THRESHOLD_BASE;
    let arrival = reload_with(&exec, config.clone(), |h| h.synaptic_gain_q16 = GAIN_1024);
    let per_tick = reload_with(&exec, config, |h| h.synaptic_gain_q16 = GAIN_1024).per_tick();
    (arrival, per_tick)
}

fn one_unit_config() -> Config {
    Config {
        units: 1,
        nodes_per_worker: 16,
        injector_capacity: 16,
        modulation_baseline_q16: 0,
        ..Config::default()
    }
}

/// ADR-0097's message, 0.125, to both, and `ticks` ticks of each.
fn message_and_run(arrival: &mut Engine, per_tick: &mut Engine, ticks: u64) {
    for exec in [&mut *arrival, &mut *per_tick] {
        exec.injector()
            .inject(0, spike_message(0x2000, false))
            .unwrap();
        exec.run(ticks);
    }
}

/// The ticks one message keeps the unit awake under per-tick service (ADR-0097).
const TICKS_TO_REST: u64 = 2_503;

#[test]
fn one_message_keeps_a_unit_awake_its_turns_served_once_and_the_rest_caught_up_at_a_read() {
    let (mut arrival, mut per_tick) = one_unit(one_unit_config());
    message_and_run(&mut arrival, &mut per_tick, 4_096);
    assert_eq!(per_tick.turns(), TICKS_TO_REST, "ADR-0097's oracle");
    assert_eq!(
        (arrival.turns(), arrival.caught_up()),
        (1, 0),
        "served once, at the message, and asleep after it, nothing yet caught up"
    );
    assert!(same(&per_tick, &arrival), "the arena, read");
    assert_eq!(
        (arrival.turns(), arrival.caught_up()),
        (1, TICKS_TO_REST.wrapping_sub(1)),
        "the read caught up the rest"
    );
    assert_eq!(arrival.units()[0].gate(), Some(GateState::Idle), "at rest");
}

#[test]
fn a_unit_that_reached_rest_asleep_reads_as_per_tick_service_left_it_marked_or_not() {
    // Past rest and unmarked: its gate idle, as the per-tick turn that found it at rest left it.
    let (mut arrival, mut per_tick) = one_unit(one_unit_config());
    message_and_run(&mut arrival, &mut per_tick, 3_000);
    assert!(same(&per_tick, &arrival));
    assert_eq!(arrival.units()[0].gate(), Some(GateState::Idle));
    // Past rest and marked by a message not yet integrated: its gate scheduled, as the mark that
    // found it idle left it.
    let (mut arrival, mut per_tick) = one_unit(one_unit_config());
    message_and_run(&mut arrival, &mut per_tick, 3_000);
    // The message is drained into the mailbox at the next tick and integrated at the one after.
    message_and_run(&mut arrival, &mut per_tick, 1);
    assert!(same(&per_tick, &arrival));
    assert_eq!(arrival.units()[0].gate(), Some(GateState::Scheduled));
    assert!(!arrival.units()[0].mailbox_is_empty());
    arrival.run(1);
    per_tick.run(1);
    assert!(
        same(&per_tick, &arrival),
        "and its turn is per-tick service's"
    );
    assert_eq!(
        arrival.turns().saturating_add(arrival.caught_up()),
        per_tick.turns()
    );
}

#[test]
fn a_write_of_the_arena_puts_every_asleep_unit_back_on_the_schedule() {
    let (mut arrival, mut per_tick) = one_unit(one_unit_config());
    message_and_run(&mut arrival, &mut per_tick, 8);
    assert_eq!(arrival.turns(), 1, "asleep after its first turn");
    // A write with nothing written: the unit is brought up to date and served at the next tick,
    // where it falls asleep again.
    let _ = arrival.units_mut();
    assert_eq!(arrival.caught_up(), 6, "the six ticks after its turn");
    arrival.run(1);
    per_tick.run(1);
    assert_eq!(arrival.turns(), 2, "served again after the write");
    arrival.run(16);
    per_tick.run(16);
    assert!(same(&per_tick, &arrival));
    assert_eq!(
        (
            arrival.turns(),
            arrival.caught_up().saturating_add(arrival.turns())
        ),
        (2, per_tick.turns()),
        "asleep again after one turn"
    );
}

fn scratch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join("menteion-arrival-tests");
    std::fs::create_dir_all(&dir).expect("a scratch directory");
    dir.join(format!("{}-{name}", std::process::id()))
}

#[test]
fn the_sweep_reads_rest_as_per_tick_service_leaves_it() {
    let (mut arrival, mut per_tick) = one_unit(one_unit_config());
    arrival.attach_log(&scratch("arrival.wal")).expect("a log");
    per_tick
        .attach_log(&scratch("per-tick.wal"))
        .expect("a log");
    message_and_run(&mut arrival, &mut per_tick, 1_000);
    // Not yet at rest: neither sweep evicts it.
    assert_eq!(
        (arrival.sweep(0, 1).unwrap(), per_tick.sweep(0, 1).unwrap()),
        (0, 0)
    );
    arrival.run(2_000);
    per_tick.run(2_000);
    // At rest under per-tick service, and asleep until the sweep brings it up to date.
    assert_eq!(
        (arrival.sweep(0, 1).unwrap(), per_tick.sweep(0, 1).unwrap()),
        (1, 1)
    );
    assert!(arrival.is_evicted(0) && per_tick.is_evicted(0));
    assert!(same(&per_tick, &arrival));
}

#[test]
fn a_unit_under_the_slow_current_never_sleeps_and_one_unmarked_does() {
    // ADR-0122's constants.
    let current = SlowCurrent {
        leak_shift: 13,
        input_shift: 1,
        v_lo_q16: 28_561,
        v_hi_q16: THRESHOLD_BASE,
    };
    for (flags, sleeps) in [(FLAG_SLOW, false), (0, true)] {
        let config = Config {
            slow_current: Some(current),
            ..one_unit_config()
        };
        let mut exec = Engine::new(config.clone()).unwrap();
        exec.units_mut()[0].v_thresh = THRESHOLD_BASE;
        exec.units_mut()[0].flags = flags;
        let arrival = reload_with(&exec, config.clone(), |h| h.synaptic_gain_q16 = GAIN_1024);
        let per_tick = reload_with(&exec, config, |h| h.synaptic_gain_q16 = GAIN_1024).per_tick();
        assert_eq!(arrival.slow_current(), Some(current));
        let (mut arrival, mut per_tick) = (arrival, per_tick);
        message_and_run(&mut arrival, &mut per_tick, 64);
        assert_eq!(
            arrival.turns() < per_tick.turns(),
            sleeps,
            "flags {flags:#x}: {} turns on arrival, {} per tick",
            arrival.turns(),
            per_tick.turns()
        );
        assert!(same(&per_tick, &arrival));
        assert_eq!(
            arrival.turns().saturating_add(arrival.caught_up()),
            per_tick.turns()
        );
    }
}
