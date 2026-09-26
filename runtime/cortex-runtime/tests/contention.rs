//! Milestone M2's exit test: 10⁶ events from four producer threads into 64 units, delivered
//! exactly once with no deadlock, on one, two and four workers (whitepaper Appendix C; brief
//! 012; ADR-0023). Producers push through the injector while the coordinator ticks; the
//! workers drain the mailboxes; the report of every worker is joined and checked. The units'
//! threshold is zero, so they never fire and every event is a plain delivery. Since ADR-0100
//! each worker owns a range of the units and alone serves them, which the last test holds.

#![deny(clippy::arithmetic_side_effects)]

use cortex_core::{
    GateState, ISTDP_TARGET_PERIOD_TICKS, MODULATION_ONE_Q16, STP_MAX, STP_U, THRESHOLD_BASE,
    spike_message, synaptic_efficacy_q16,
};
use cortex_runtime::{Config, Executor, InjectError};
use std::thread;

const PRODUCERS: u32 = 4;
const PER_PRODUCER: u32 = 250_000;
const UNITS: usize = 64;

fn every_event_is_delivered_exactly_once_on(workers: usize) {
    let total = PRODUCERS * PER_PRODUCER;
    let mut exec = Executor::<64>::new(Config {
        workers,
        units: UNITS,
        blocks: 0,
        deltas: 0,
        nodes_per_worker: 1 << 17,
        injector_capacity: 1 << 16,
        trace_capacity: total as usize,
        amendments: 0,
        modulation_baseline_q16: MODULATION_ONE_Q16,
        control_step_q0_16: 0,
        sleep_shift: 0,
        episodes: 0,
        train_capacity: 0,
        terms: 0,
        clauses: 0,
        search_shift: 0,
        search_budget: 0,
        discovery_tag: 0,
        istdp_target_period_ticks: ISTDP_TARGET_PERIOD_TICKS,
        inhibitory_baseline_q16: None,
        signed_gate: false,
    })
    .expect("a valid configuration");
    assert_eq!(
        exec.workers(),
        workers,
        "the caller's thread and the spawned ones"
    );
    let inject = exec.injector();
    let producers: Vec<_> = (0..PRODUCERS)
        .map(|p| {
            let inject = inject.clone();
            thread::spawn(move || {
                for i in 0..PER_PRODUCER {
                    let payload = p.wrapping_mul(PER_PRODUCER).wrapping_add(i);
                    let unit = (payload.wrapping_mul(2_654_435_761) >> 8)
                        .checked_rem(UNITS as u32)
                        .expect("UNITS is not zero");
                    loop {
                        match inject.inject(unit, payload) {
                            Ok(()) => break,
                            Err(InjectError::Full) => thread::yield_now(),
                            Err(other) => panic!("{other:?}"),
                        }
                    }
                }
            })
        })
        .collect();
    let mut ticks = 0u64;
    while exec.delivered() < total as u64 {
        exec.tick();
        ticks = ticks.wrapping_add(1);
        assert!(
            ticks < 5_000_000,
            "no progress: {} delivered",
            exec.delivered()
        );
    }
    for producer in producers {
        producer.join().expect("a producer");
    }
    exec.tick();
    assert_eq!(
        exec.delivered(),
        total as u64,
        "nothing arrives after the last event"
    );
    let reports = exec.shutdown();
    assert_eq!(reports.len(), workers);
    assert_eq!(reports.iter().map(|r| r.dropped).sum::<u64>(), 0);
    let mut all: Vec<u32> = reports
        .iter()
        .flat_map(|r| r.delivered.iter().copied())
        .collect();
    assert_eq!(all.len(), total as usize, "every event delivered");
    all.sort_unstable();
    all.dedup();
    assert_eq!(all.len(), total as usize, "no event delivered twice");
    assert_eq!(
        (all[0], all[(total as usize).wrapping_sub(1)]),
        (0, total - 1)
    );
    assert!(
        reports.iter().all(|r| r.spikes.is_empty()),
        "an unconfigured unit never fires"
    );
    // Whether the work was shared depends on timing: in the release profile on a few cores,
    // worker 0 can drain every event before another worker wakes to steal (the release step's
    // first run, ADR-0030). Exactly-once delivery is the property; the shares are reported,
    // not asserted.
    if workers > 1 {
        eprintln!(
            "shares on {workers} workers: {:?}",
            reports
                .iter()
                .map(|r| r.delivered_count)
                .collect::<Vec<_>>()
        );
    }
}

#[test]
fn a_million_events_are_delivered_exactly_once_on_one_worker() {
    every_event_is_delivered_exactly_once_on(1);
}

#[test]
fn a_million_events_are_delivered_exactly_once_on_two_workers() {
    every_event_is_delivered_exactly_once_on(2);
}

#[test]
fn a_million_events_are_delivered_exactly_once_on_four_workers() {
    every_event_is_delivered_exactly_once_on(4);
}

#[test]
fn a_unit_that_keeps_receiving_is_served_on_every_tick_and_is_never_starved() {
    let mut exec = Executor::<64>::new(Config {
        workers: 2,
        units: 8,
        nodes_per_worker: 64,
        injector_capacity: 16,
        trace_capacity: 4096,
        amendments: 0,
        ..Config::default()
    })
    .unwrap();
    let inject = exec.injector();
    // Seven other units are kept busy every tick too.
    for tick in 0..1000u32 {
        for unit in 0..8u32 {
            inject.inject(unit, 1 + tick).unwrap();
        }
        let before = exec.delivered();
        exec.tick();
        if tick > 0 {
            assert_eq!(
                exec.delivered() - before,
                8,
                "the eight messages of the previous tick were all served at tick {tick}"
            );
        }
    }
    exec.tick();
    assert_eq!(exec.delivered(), 8000);
    let reports = exec.shutdown();
    let served: u64 = reports.iter().map(|r| r.delivered_count).sum();
    assert_eq!(served, 8000);
}

/// Axiom A3 by ownership (ADR-0100). Ten units on four workers are ranges of three, three,
/// three and one; every message injected into a unit is drained by the unit's owner and by no
/// other worker. Then two units on two workers, one each: unit 0 fires with a zero-delay
/// synapse onto unit 1, which is at rest and not served until the message worker 0 pushed
/// reaches it; worker 1 serves it at the next tick, and drains the message.
#[test]
fn a_unit_is_served_by_its_owner_alone_and_a_woken_unit_at_the_next_tick() {
    const UNITS: u32 = 10;
    const PER_UNIT: u32 = 5;
    let mut exec = Executor::<64>::new(Config {
        workers: 4,
        units: UNITS as usize,
        nodes_per_worker: 256,
        injector_capacity: 256,
        trace_capacity: 1024,
        ..Config::default()
    })
    .unwrap();
    let owners: Vec<usize> = (0..UNITS).map(|u| exec.owner(u).unwrap()).collect();
    assert_eq!(
        owners,
        [0, 0, 0, 1, 1, 1, 2, 2, 2, 3],
        "contiguous ranges of three"
    );
    assert_eq!(exec.owner(UNITS), None, "no owner outside the arena");
    let inject = exec.injector();
    for round in 0..PER_UNIT {
        for unit in 0..UNITS {
            // The payload names the unit: an unconfigured unit never fires on it.
            let payload = unit.wrapping_mul(1000).wrapping_add(round);
            inject.inject(unit, payload).unwrap();
        }
    }
    exec.run(3);
    assert_eq!(exec.delivered(), u64::from(UNITS.wrapping_mul(PER_UNIT)));
    let turns = exec.turns();
    let reports = exec.shutdown();
    assert_eq!(reports.iter().map(|r| r.turns).sum::<u64>(), turns);
    for (worker, report) in reports.iter().enumerate() {
        let units = owners.iter().filter(|&&o| o == worker).count() as u64;
        assert_eq!(
            report.delivered_count,
            units.wrapping_mul(u64::from(PER_UNIT)),
            "worker {worker} drained its own units' messages"
        );
        for &payload in &report.delivered {
            let unit = payload.checked_div(1000).unwrap() as usize;
            assert_eq!(
                owners[unit], worker,
                "unit {unit}'s message on worker {worker}"
            );
        }
        // A message leaves its unit awake, so it may be served again the tick after.
        assert!(
            report.turns >= units,
            "worker {worker} served each unit it owns"
        );
    }

    let mut exec = Executor::<64>::new(Config {
        workers: 2,
        units: 2,
        blocks: 1,
        nodes_per_worker: 64,
        injector_capacity: 64,
        trace_capacity: 64,
        ..Config::default()
    })
    .unwrap();
    assert_eq!((exec.owner(0), exec.owner(1)), (Some(0), Some(1)));
    assert!(exec.blocks_mut()[0].set_synapse(0, 1, i16::MAX, 0, false));
    for unit in exec.units_mut() {
        unit.v_thresh = THRESHOLD_BASE;
        unit.stp_u_rel = STP_U;
        unit.stp_r_ves = STP_MAX;
    }
    assert!(exec.units_mut()[0].set_first_block(0));
    let strong = spike_message(synaptic_efficacy_q16(i16::MAX, STP_U, STP_MAX), false);
    for _ in 0..14 {
        exec.injector().inject(0, strong).unwrap();
    }
    let mut spike = None;
    for _ in 0..40u32 {
        let now = exec.ticks() as u32;
        let before = exec.turns();
        let unit1 = &exec.units()[1];
        let woken = unit1.gate() == Some(GateState::Scheduled);
        assert_eq!(
            woken,
            spike.is_some(),
            "tick {now}: marked by the message alone"
        );
        exec.tick();
        let served = exec.turns().wrapping_sub(before);
        let awake0 = u64::from(now >= 1);
        assert_eq!(
            served,
            awake0.wrapping_add(u64::from(woken)),
            "tick {now}: unit 1 is served only once the message woke it"
        );
        if spike.is_none() && exec.units()[0].last_soma_spike_tick == now && now > 0 {
            spike = Some(now);
            assert_eq!(
                exec.units()[1].v_basal,
                0,
                "not integrated in the spike's tick"
            );
        }
        if woken {
            assert_ne!(exec.units()[1].v_basal, 0, "integrated at the next tick");
            break;
        }
    }
    let spike = spike.expect("unit 0 fired");
    assert_eq!(exec.ticks(), u64::from(spike).wrapping_add(2));
    let reports = exec.shutdown();
    assert_eq!(
        reports[0].delivered.len(),
        14,
        "worker 0 drained unit 0's kicks"
    );
    assert_eq!(
        reports[1].delivered.len(),
        1,
        "worker 1 drained the synapse's message"
    );
    assert_eq!(
        reports[1].turns, 1,
        "worker 1 served unit 1 once, at the next tick"
    );
    assert_eq!(reports[0].spikes, vec![(0, spike)]);
}
