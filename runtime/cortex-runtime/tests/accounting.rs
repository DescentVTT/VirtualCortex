//! The executor's accounting (ADR-0030): the traces report what they dropped, a chain that
//! cycles ends within the arena's length, a unit activated more than once before a tick takes
//! one turn, and the accessors say what the executor holds.

#![deny(clippy::arithmetic_side_effects)]

use cortex_core::{STP_MAX, STP_U, THRESHOLD_BASE, spike_message, synaptic_efficacy_q16};
use cortex_runtime::{Config, Executor};

fn config(units: usize, blocks: usize, trace_capacity: usize) -> Config {
    Config {
        workers: 1,
        units,
        blocks,
        deltas: 0,
        nodes_per_worker: 1024,
        injector_capacity: 256,
        trace_capacity,
        ..Config::default()
    }
}

fn arm(exec: &mut Executor<64>) {
    for unit in exec.units_mut() {
        unit.v_thresh = THRESHOLD_BASE;
        unit.stp_u_rel = STP_U;
        unit.stp_r_ves = STP_MAX;
    }
}

/// One strong message: about 0.2 of the threshold.
fn strong() -> u32 {
    spike_message(synaptic_efficacy_q16(i16::MAX, STP_U, STP_MAX), false)
}

/// Fourteen strong messages in one tick: 2.8 of the threshold, which the soma, pulled to half
/// by the resting apical compartment, crosses about ten ticks later (the `kick` of the image
/// tests).
fn kick(exec: &Executor<64>, unit: u32) {
    for _ in 0..14 {
        exec.injector().inject(unit, strong()).unwrap();
    }
}

#[test]
fn a_trace_that_overflows_reports_every_entry_it_dropped() {
    let mut exec = Executor::<64>::new(config(8, 0, 2)).unwrap();
    arm(&mut exec);
    assert_eq!(exec.workers(), 1);
    for unit in 0..8 {
        kick(&exec, unit);
    }
    exec.run(80);
    assert_eq!(exec.delivered(), 112, "every message was drained");
    let reports = exec.shutdown();
    let report = &reports[0];
    assert_eq!(report.delivered_count, 112);
    assert_eq!(
        report.delivered.len(),
        2,
        "the message trace holds its capacity"
    );
    assert_eq!(report.spikes.len(), 2, "so does the spike trace");
    assert_eq!(
        report.dropped,
        (112 - 2) + (8 - 2),
        "110 messages and 6 spikes beyond the two traces' capacity"
    );
}

#[test]
fn an_executor_without_a_trace_drops_nothing() {
    let mut exec = Executor::<64>::new(config(4, 0, 0)).unwrap();
    arm(&mut exec);
    for unit in 0..4 {
        kick(&exec, unit);
    }
    exec.run(80);
    assert_eq!(exec.delivered(), 56);
    let reports = exec.shutdown();
    assert_eq!(reports[0].delivered.len(), 0);
    assert_eq!(reports[0].spikes.len(), 0);
    assert_eq!(
        reports[0].dropped, 0,
        "a trace of zero capacity counts nothing as dropped"
    );
}

#[test]
fn a_cyclic_chain_ends_within_the_arena_and_the_tokens_in_flight_are_counted() {
    let mut exec = Executor::<64>::new(config(4, 3, 64)).unwrap();
    arm(&mut exec);
    {
        let blocks = exec.blocks_mut();
        for (i, block) in blocks.iter_mut().enumerate() {
            assert!(block.set_synapse(0, 1 + i as u32 % 3, 20_000, 5, false));
        }
        assert!(blocks[0].link(1));
        assert!(blocks[1].link(2));
        assert!(blocks[2].link(0), "block 2 points back at block 0: a cycle");
    }
    assert!(exec.units_mut()[0].set_first_block(0));
    assert_eq!(exec.tokens_in_flight(), 0);
    kick(&exec, 0);
    let mut fired_at = None;
    for t in 0..200u64 {
        exec.tick();
        if exec.tokens_in_flight() > 0 {
            fired_at = Some(t);
            break;
        }
    }
    assert_eq!(fired_at, Some(10), "unit 0 fired on the tenth tick");
    assert_eq!(
        exec.tokens_in_flight(),
        3,
        "the walk visited each block once, cycle or not"
    );
    exec.run(10);
    assert_eq!(exec.tokens_in_flight(), 0, "delivered after the delay");
    assert_eq!(
        exec.delivered(),
        14 + 3,
        "the injections and the three deliveries"
    );
}

#[test]
fn a_unit_activated_three_times_before_a_tick_takes_its_turn_without_a_message() {
    let mut exec = Executor::<64>::new(config(2, 0, 64)).unwrap();
    arm(&mut exec);
    // Unit 1 sits above its threshold; only a turn can make it fire, and only an activation
    // gives it one without a message.
    exec.units_mut()[1].v_soma = 2 * THRESHOLD_BASE;
    exec.injector().activate(1).unwrap();
    exec.injector().activate(1).unwrap();
    exec.injector().activate(1).unwrap();
    exec.run(3);
    assert_eq!(exec.delivered(), 0, "an activation carries no message");
    assert_eq!(exec.tokens_in_flight(), 0);
    let reports = exec.shutdown();
    assert_eq!(
        reports[0].spikes,
        vec![(1, 1)],
        "one turn, on the tick after the drain, and the unit fired in it"
    );
    assert_eq!(reports[0].dropped, 0);
}
