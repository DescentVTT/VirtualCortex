//! Milestone M2's exit test: 10⁶ events from four producer threads into 64 units, delivered
//! exactly once with no deadlock, on one, two and four workers (whitepaper Appendix C; brief
//! 012; ADR-0023). Producers push through the injector while the coordinator ticks; the
//! workers drain the mailboxes; the report of every worker is joined and checked. The units'
//! threshold is zero, so they never fire and every event is a plain delivery.

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
        deque_capacity: 0,
        injector_capacity: 1 << 16,
        trace_capacity: total as usize,
    })
    .expect("a valid configuration");
    let inject = exec.injector();
    let producers: Vec<_> = (0..PRODUCERS)
        .map(|p| {
            let inject = inject.clone();
            thread::spawn(move || {
                for i in 0..PER_PRODUCER {
                    let payload = p * PER_PRODUCER + i;
                    let unit = (payload.wrapping_mul(2_654_435_761) >> 8) % UNITS as u32;
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
        ticks += 1;
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
    assert_eq!((all[0], all[total as usize - 1]), (0, total - 1));
    assert!(
        reports.iter().all(|r| r.spikes.is_empty()),
        "an unconfigured unit never fires"
    );
    if workers > 1 {
        assert!(
            reports.iter().filter(|r| r.delivered_count > 0).count() > 1,
            "the work was shared: {:?}",
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
