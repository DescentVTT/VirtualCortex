//! Exploration 008 (`explore/008-a-unit-served-on-arrival`, on its branch alone, ADR-0193): a
//! unit served on arrival, held to the per-tick service bit for bit.
//!
//! Under `OnArrival::Integrate` or `OnArrival::Quiet` a unit that is not at rest and cannot fire
//! with no input (`DendriticSuperNeuron::quiet_cannot_fire`) leaves the schedule at the end of its
//! turn, asleep, and is brought up to date before its next turn or before the arena is read. These
//! tests run the reference network of ADR-0097 under each mode beside the same network under the
//! per-tick service, from one image, and hold every unit's 64 bytes and every block's, the spike
//! train and the messages delivered, the one to the other: every tick over a short run, and at
//! ticks far apart over a longer one, under ADR-0044's drive and one 256 times sparser. One armed
//! unit given one message of 0.125 at the gain 1.75 is served, in turns and catch-up steps
//! together, the 2 503 turns ADR-0097's oracle gives.

#![deny(clippy::arithmetic_side_effects)]

#[allow(dead_code, unused_imports)]
#[path = "instrument/harness.rs"]
mod harness;
use harness::*;

use menteion_runtime::{ArrivalCounts, OnArrival};

/// The bytes of a unit the two services are held to: its 64 bytes, atomics as plain values, the
/// mailbox head but as whether the mailbox holds a message. The head is a node index from a
/// worker's pool, whose order the two workers' frees in one phase set (ADR-0099's scheduler
/// bytes): two runs of one engine on two workers need not share it.
fn unit_bytes(u: &DendriticSuperNeuron) -> [u8; 64] {
    let mut bytes = u.encode();
    bytes[8..16].copy_from_slice(&u64::from(!u.mailbox_is_empty()).to_le_bytes());
    bytes
}

/// FNV-1a over every unit's bytes and every block's.
fn arena_hash(exec: &Engine) -> u64 {
    let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
    let units = exec.units().iter().map(unit_bytes);
    let blocks = exec.blocks().iter().map(|b| b.encode());
    for record in units.chain(blocks) {
        for byte in record {
            hash ^= u64::from(byte);
            hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
        }
    }
    hash
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

/// Runs the network at `units` under `drive` for `ticks` ticks, once under the per-tick service
/// and once under `mode`, two networks built alike, and holds the two to each other after every tick
/// that `read` admits: the arena, the spikes so far and the messages delivered. Returns the
/// counts of the run under `mode`, read after the arena's last reading, and the turns of the
/// per-tick run.
fn held(
    units: u32,
    drive: &Drive,
    ticks: u64,
    mode: OnArrival,
    read: impl Fn(u64) -> bool,
) -> (ArrivalCounts, u64) {
    let mut each = network(units);
    let mut arrival = network(units);
    each.set_on_arrival(OnArrival::Off);
    arrival.set_on_arrival(mode);
    assert!(arrival_hash_pair(&each, &arrival), "the same network");
    let (inject_each, inject_arrival) = (each.injector(), arrival.injector());
    let mut readings = 0u32;
    for k in 0..ticks {
        let tick = each.ticks();
        assert_eq!(tick, arrival.ticks());
        drive.step(&inject_each, tick).expect("the drive runs");
        drive.step(&inject_arrival, tick).expect("the drive runs");
        each.tick();
        arrival.tick();
        if read(k) {
            readings = readings.saturating_add(1);
            assert_eq!(
                (spikes(&mut arrival), arrival.delivered()),
                (spikes(&mut each), each.delivered()),
                "{mode:?}, tick {tick}: the spikes and the messages"
            );
            assert!(
                arrival_hash_pair(&each, &arrival),
                "{mode:?}, tick {tick}: the arena"
            );
        }
    }
    assert!(readings > 0);
    // Every asleep unit brought up to date, so that its catch-up is counted.
    assert!(
        arrival_hash_pair(&each, &arrival),
        "{mode:?}: the arena at the end"
    );
    (arrival.arrival_counts(), each.turns())
}

fn arrival_hash_pair(each: &Engine, arrival: &Engine) -> bool {
    let same = arena_hash(each) == arena_hash(arrival);
    if !same {
        // The first unit or block that differs, for the failure's message.
        let units = each.units().iter().zip(arrival.units());
        if let Some((i, (a, b))) = units
            .enumerate()
            .find(|(_, (a, b))| unit_bytes(a) != unit_bytes(b))
        {
            eprintln!("unit {i}: per tick {a:?}");
            eprintln!("unit {i}: on arrival {b:?}");
        }
        let blocks = each.blocks().iter().zip(arrival.blocks());
        if let Some((i, (a, b))) = blocks
            .enumerate()
            .find(|(_, (a, b))| a.encode() != b.encode())
        {
            eprintln!("block {i}: per tick {a:?}");
            eprintln!("block {i}: on arrival {b:?}");
        }
    }
    same
}

/// The counts of a run, written out.
fn dump(what: &str, c: &ArrivalCounts, per_tick_turns: u64) {
    eprintln!(
        "COUNT {what}: ticks {} served {} (per tick {}..{}) of the per-tick service's {per_tick_turns}; \
         slept {} woken {} with {} messages after {} ticks, caught up {} steps; \
         settles {} of {} units, {} steps; steps {:?}; served by 32nds {:?}",
        c.ticks,
        c.served,
        c.served_min,
        c.served_max,
        c.slept,
        c.woken,
        c.woken_messages,
        c.gap,
        c.caught_up,
        c.settles,
        c.settled_units,
        c.settled_steps,
        c.steps_hist,
        c.served_hist,
    );
}

/// Checks a run's counts: units slept and woke and were caught up, the turns served on arrival
/// were fewer than the per-tick service's, and the turns, the catch-up's steps at a turn and at a
/// read together are its turns, since every asleep unit was brought up to date at the end.
fn counted(c: &ArrivalCounts, per_tick_turns: u64) {
    assert!(c.slept > 0 && c.woken > 0, "{c:?}");
    assert!(c.caught_up.saturating_add(c.settled_steps) > 0, "{c:?}");
    assert!(c.served < per_tick_turns, "{c:?}");
    assert_eq!(
        c.served
            .saturating_add(c.caught_up)
            .saturating_add(c.settled_steps),
        per_tick_turns,
        "the turns of the per-tick service, served or caught up"
    );
}

#[test]
fn the_reference_network_served_on_arrival_is_the_per_tick_service_at_every_tick() {
    for mode in [OnArrival::Integrate, OnArrival::Quiet] {
        let (counts, turns) = held(256, &drive(256), 1 << 11, mode, |_| true);
        dump(
            &format!("{mode:?}, ADR-0044's drive, every tick"),
            &counts,
            turns,
        );
        counted(&counts, turns);
    }
}

#[test]
fn the_reference_network_served_on_arrival_is_the_per_tick_service_at_ticks_far_apart() {
    // ADR-0044's drive and one 256 times sparser, ADR-0097's (c), where units reach rest asleep.
    let sparse = Drive {
        every: 32,
        messages: 1,
        ..drive(256)
    };
    for mode in [OnArrival::Integrate, OnArrival::Quiet] {
        for (what, d) in [
            ("ADR-0044's drive", drive(256)),
            ("a drive 256 times sparser", sparse),
        ] {
            let (counts, turns) = held(256, &d, 1 << 15, mode, |k| k % 4_999 == 4_998);
            dump(
                &format!("{mode:?}, {what}, every 4 999 ticks"),
                &counts,
                turns,
            );
            counted(&counts, turns);
        }
    }
}

#[test]
fn one_message_keeps_a_unit_awake_the_oracle_s_turns_served_or_caught_up() {
    // ADR-0097's one armed unit with no synapse, one message of 0.125 at the gain 1.75: 2 503
    // turns under the per-tick service.
    for mode in [OnArrival::Integrate, OnArrival::Quiet] {
        let config = Config {
            units: 1,
            nodes_per_worker: 16,
            injector_capacity: 16,
            modulation_baseline_q16: 0,
            ..Config::default()
        };
        let mut exec = Engine::new(config.clone()).unwrap();
        exec.units_mut()[0].v_thresh = THRESHOLD_BASE;
        let mut exec = reload_with(&exec, config, |h| h.synaptic_gain_q16 = GAIN_1024);
        exec.set_on_arrival(mode);
        exec.injector()
            .inject(0, spike_message(0x2000, false))
            .unwrap();
        exec.run(4_096);
        let at_rest = exec.units()[0].v_basal == 0 && exec.units()[0].v_soma == 0;
        let c = exec.arrival_counts();
        dump(&format!("{mode:?}, one message"), &c, 2_503);
        assert!(at_rest, "{mode:?}: the unit rests");
        assert_eq!(
            (c.served, c.slept),
            (1, 1),
            "{mode:?}: served once, and asleep after"
        );
        assert_eq!(
            c.served.saturating_add(c.settled_steps),
            2_503,
            "{mode:?}: the oracle's turns"
        );
    }
}
