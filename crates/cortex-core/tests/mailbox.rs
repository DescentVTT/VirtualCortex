//! Milestone M1's exit test: push → gate → callback, under contention. Four producer threads
//! push into one unit's mailbox through a shared node arena while one consumer thread claims
//! the turn, drains, and ends the turn under the lost-wakeup rule; every message is delivered
//! exactly once and none is left behind (ADR-0017).
//!
//! The worker deque is modelled by one flag, since there is one unit; node pools are modelled
//! by per-producer slices of the arena with a busy flag per node that the consumer clears. This
//! is an integration test so that the library stays free of thread spawning and heap types
//! (whitepaper TC-5).

use core::sync::atomic::{AtomicBool, Ordering};
use cortex_core::{DendriticSuperNeuron, GateState, MailboxNode};
use std::sync::{Arc, Mutex};
use std::thread;

const PRODUCERS: usize = 4;
const NODES_PER_PRODUCER: usize = 16;
const MESSAGES_PER_PRODUCER: u32 = 25_000;

#[test]
fn four_producers_one_consumer_deliver_every_message_exactly_once_with_no_lost_wakeup() {
    let unit = Arc::new(DendriticSuperNeuron::new(1));
    let nodes: Arc<[MailboxNode; PRODUCERS * NODES_PER_PRODUCER]> =
        Arc::new(core::array::from_fn(|_| MailboxNode::new()));
    let busy: Arc<[AtomicBool; PRODUCERS * NODES_PER_PRODUCER]> =
        Arc::new(core::array::from_fn(|_| AtomicBool::new(false)));
    let queued = Arc::new(AtomicBool::new(false));
    let producers_done = Arc::new(AtomicBool::new(false));
    let delivered = Arc::new(Mutex::new(Vec::with_capacity(
        PRODUCERS * MESSAGES_PER_PRODUCER as usize,
    )));

    let consumer = {
        let unit = Arc::clone(&unit);
        let nodes = Arc::clone(&nodes);
        let busy = Arc::clone(&busy);
        let queued = Arc::clone(&queued);
        let producers_done = Arc::clone(&producers_done);
        let delivered = Arc::clone(&delivered);
        thread::spawn(move || {
            let mut turns = 0u64;
            loop {
                if !queued.swap(false, Ordering::SeqCst) {
                    if producers_done.load(Ordering::SeqCst)
                        && unit.mailbox_is_empty()
                        && unit.gate() == Some(GateState::Idle)
                    {
                        break;
                    }
                    thread::yield_now();
                    continue;
                }
                assert!(unit.begin_turn(), "a queued unit is scheduled");
                turns += 1;
                {
                    let mut out = delivered.lock().unwrap();
                    for (node, payload) in unit.mailbox_drain(&nodes[..]) {
                        out.push(payload);
                        busy[node as usize].store(false, Ordering::Release);
                    }
                }
                if unit.end_turn() {
                    queued.store(true, Ordering::SeqCst);
                }
            }
            turns
        })
    };

    let producers: Vec<_> = (0..PRODUCERS)
        .map(|p| {
            let unit = Arc::clone(&unit);
            let nodes = Arc::clone(&nodes);
            let busy = Arc::clone(&busy);
            let queued = Arc::clone(&queued);
            thread::spawn(move || {
                let first = p * NODES_PER_PRODUCER;
                for seq in 0..MESSAGES_PER_PRODUCER {
                    // A free node from this producer's own pool; the consumer frees them.
                    let node = loop {
                        let free = (first..first + NODES_PER_PRODUCER).find(|&i| {
                            busy[i]
                                .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
                                .is_ok()
                        });
                        match free {
                            Some(i) => break i as u32,
                            None => thread::yield_now(),
                        }
                    };
                    let payload = ((p as u32) << 20) | seq;
                    assert!(unit.mailbox_push(&nodes[..], node, payload));
                    if unit.try_schedule() {
                        queued.store(true, Ordering::SeqCst);
                    }
                }
            })
        })
        .collect();

    for producer in producers {
        producer.join().expect("producer");
    }
    producers_done.store(true, Ordering::SeqCst);
    let turns = consumer.join().expect("consumer");

    let mut out = delivered.lock().unwrap().clone();
    let expected = PRODUCERS * MESSAGES_PER_PRODUCER as usize;
    assert_eq!(out.len(), expected, "every message delivered");
    out.sort_unstable();
    out.dedup();
    assert_eq!(out.len(), expected, "no message delivered twice");
    for p in 0..PRODUCERS as u32 {
        for seq in 0..MESSAGES_PER_PRODUCER {
            assert!(out.binary_search(&((p << 20) | seq)).is_ok());
        }
    }
    assert!(unit.mailbox_is_empty(), "nothing left behind");
    assert_eq!(unit.gate(), Some(GateState::Idle));
    assert!(
        busy.iter().all(|b| !b.load(Ordering::Acquire)),
        "every node returned"
    );
    assert!(
        turns >= 1 && turns as usize <= expected,
        "batch draining took {turns} turns"
    );
}
