//! Two `std` threads exchange frames through the ring protocol over storage that the test
//! provides: atomic slots whose payload is written and read with relaxed operations only, so the
//! ordering is supplied solely by the cursor protocol, as it is over a shared mapping (ADR-0015).
//!
//! This is an integration test rather than a unit test so that the library itself stays free of
//! thread spawning and heap types, which whitepaper TC-5 forbids in state crates.

use core::sync::atomic::{AtomicI32, AtomicU64, Ordering};
use cortex_embodiment::{EmbodimentRingBuffer, CAPACITY, DOF};
use std::sync::Arc;
use std::thread;

struct Slot {
    epoch: AtomicU64,
    torques: [AtomicI32; DOF],
}

fn torque_for(frame: u64, joint: usize) -> i32 {
    (frame as i32).wrapping_mul(31).wrapping_add(joint as i32)
}

#[test]
fn two_threads_exchange_frames_in_order_without_loss() {
    const FRAMES: u64 = 100_000;
    let ring = Arc::new(EmbodimentRingBuffer::new());
    let slots: Arc<[Slot; CAPACITY as usize]> = Arc::new(core::array::from_fn(|_| Slot {
        epoch: AtomicU64::new(0),
        torques: core::array::from_fn(|_| AtomicI32::new(0)),
    }));

    let producer = {
        let ring = Arc::clone(&ring);
        let slots = Arc::clone(&slots);
        thread::spawn(move || {
            for frame in 0..FRAMES {
                let slot = loop {
                    if let Some(s) = ring.producer_claim() {
                        break s;
                    }
                    thread::yield_now();
                };
                slots[slot].epoch.store(frame, Ordering::Relaxed);
                for (j, t) in slots[slot].torques.iter().enumerate() {
                    t.store(torque_for(frame, j), Ordering::Relaxed);
                }
                ring.producer_publish(frame, frame);
            }
        })
    };

    let consumer = {
        let ring = Arc::clone(&ring);
        let slots = Arc::clone(&slots);
        thread::spawn(move || {
            let mut last_heartbeat = 0;
            for frame in 0..FRAMES {
                let slot = loop {
                    if let Some(s) = ring.consumer_peek() {
                        break s;
                    }
                    thread::yield_now();
                };
                assert_eq!(slot as u64, frame % CAPACITY);
                assert_eq!(slots[slot].epoch.load(Ordering::Relaxed), frame, "in order");
                for (j, t) in slots[slot].torques.iter().enumerate() {
                    assert_eq!(t.load(Ordering::Relaxed), torque_for(frame, j));
                }
                let hb = ring.heartbeat_ms.load(Ordering::Acquire);
                assert!(hb >= last_heartbeat);
                last_heartbeat = hb;
                ring.consumer_release();
            }
        })
    };

    producer.join().expect("producer");
    consumer.join().expect("consumer");
    assert!(ring.is_empty());
    assert_eq!(ring.write_cursor.load(Ordering::Relaxed), FRAMES);
    assert_eq!(ring.epoch_id.load(Ordering::Relaxed), FRAMES - 1);
}
