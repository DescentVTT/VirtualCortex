//! The boundary between the engine and the outside: a bounded multi-producer, multi-consumer
//! ring of `(unit, payload)` pairs (Vyukov's sequence-numbered queue, 2011), all atomics, no
//! `unsafe`, allocated once (ADR-0023). Producer threads outside the tick loop push into it at
//! any time; worker 0 drains it in the delivery phase of every tick, so an injected message
//! reaches its unit's mailbox at the end of the tick in which it is drained and is integrated
//! on the next. Nothing outside the loop ever touches a record directly.

use std::sync::atomic::{AtomicU32, AtomicUsize, Ordering};

struct Slot {
    sequence: AtomicUsize,
    unit: AtomicU32,
    payload: AtomicU32,
}

pub struct Injector {
    slots: Box<[Slot]>,
    mask: usize,
    enqueue_pos: AtomicUsize,
    dequeue_pos: AtomicUsize,
}

/// The ring is full; the producer retries later.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Full;

impl Injector {
    /// A ring of `capacity` entries, a power of two of at least 2.
    pub fn new(capacity: usize) -> Self {
        let size = capacity.max(2).next_power_of_two();
        Self {
            slots: (0..size)
                .map(|i| Slot {
                    sequence: AtomicUsize::new(i),
                    unit: AtomicU32::new(0),
                    payload: AtomicU32::new(0),
                })
                .collect(),
            mask: size - 1,
            enqueue_pos: AtomicUsize::new(0),
            dequeue_pos: AtomicUsize::new(0),
        }
    }

    /// Number of entries.
    pub fn capacity(&self) -> usize {
        self.slots.len()
    }

    /// True when no pair is queued. Exact only while no producer is pushing, which is the
    /// case at a quiescent point between ticks (ADR-0028).
    pub fn is_empty(&self) -> bool {
        self.enqueue_pos.load(Ordering::Acquire) == self.dequeue_pos.load(Ordering::Acquire)
    }

    /// Any thread: queues a pair. Refused, with nothing changed, when the ring is full.
    pub fn push(&self, unit: u32, payload: u32) -> Result<(), Full> {
        let mut pos = self.enqueue_pos.load(Ordering::Relaxed);
        loop {
            let slot = &self.slots[pos & self.mask];
            let sequence = slot.sequence.load(Ordering::Acquire);
            let difference = sequence as isize - pos as isize;
            if difference == 0 {
                if self
                    .enqueue_pos
                    .compare_exchange_weak(pos, pos + 1, Ordering::Relaxed, Ordering::Relaxed)
                    .is_ok()
                {
                    slot.unit.store(unit, Ordering::Relaxed);
                    slot.payload.store(payload, Ordering::Relaxed);
                    slot.sequence.store(pos + 1, Ordering::Release);
                    return Ok(());
                }
                pos = self.enqueue_pos.load(Ordering::Relaxed);
            } else if difference < 0 {
                return Err(Full);
            } else {
                pos = self.enqueue_pos.load(Ordering::Relaxed);
            }
        }
    }

    /// Any thread: takes the oldest pair, or `None` when the ring is empty.
    pub fn pop(&self) -> Option<(u32, u32)> {
        let mut pos = self.dequeue_pos.load(Ordering::Relaxed);
        loop {
            let slot = &self.slots[pos & self.mask];
            let sequence = slot.sequence.load(Ordering::Acquire);
            let difference = sequence as isize - (pos + 1) as isize;
            if difference == 0 {
                if self
                    .dequeue_pos
                    .compare_exchange_weak(pos, pos + 1, Ordering::Relaxed, Ordering::Relaxed)
                    .is_ok()
                {
                    let pair = (
                        slot.unit.load(Ordering::Relaxed),
                        slot.payload.load(Ordering::Relaxed),
                    );
                    slot.sequence.store(pos + self.mask + 1, Ordering::Release);
                    return Some(pair);
                }
                pos = self.dequeue_pos.load(Ordering::Relaxed);
            } else if difference < 0 {
                return None;
            } else {
                pos = self.dequeue_pos.load(Ordering::Relaxed);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn the_ring_is_fifo_bounded_and_reusable() {
        let q = Injector::new(3);
        assert_eq!(q.capacity(), 4);
        assert!(q.is_empty());
        assert_eq!(q.pop(), None);
        for i in 0..4 {
            assert_eq!(q.push(i, 100 + i), Ok(()));
            assert!(!q.is_empty());
        }
        assert_eq!(q.push(9, 9), Err(Full));
        assert_eq!(q.pop(), Some((0, 100)));
        assert_eq!(q.push(4, 104), Ok(()));
        for i in 1..5 {
            assert_eq!(q.pop(), Some((i, 100 + i)));
        }
        assert_eq!(q.pop(), None);
        for round in 0..1000u32 {
            assert_eq!(q.push(round, round), Ok(()));
            assert_eq!(q.pop(), Some((round, round)));
        }
        assert!(
            q.is_empty(),
            "after every push was popped, the wrap included"
        );
    }

    #[test]
    fn four_producers_and_one_consumer_lose_and_duplicate_nothing() {
        const PRODUCERS: u32 = 4;
        const EACH: u32 = 50_000;
        let q = Arc::new(Injector::new(256));
        let producers: Vec<_> = (0..PRODUCERS)
            .map(|p| {
                let q = Arc::clone(&q);
                thread::spawn(move || {
                    for i in 0..EACH {
                        let payload = p * EACH + i;
                        while q.push(p, payload).is_err() {
                            thread::yield_now();
                        }
                    }
                })
            })
            .collect();
        let mut got = Vec::with_capacity((PRODUCERS * EACH) as usize);
        while got.len() < (PRODUCERS * EACH) as usize {
            match q.pop() {
                Some((_, payload)) => got.push(payload),
                None => thread::yield_now(),
            }
        }
        for p in producers {
            p.join().unwrap();
        }
        assert_eq!(q.pop(), None);
        got.sort_unstable();
        got.dedup();
        assert_eq!(got.len(), (PRODUCERS * EACH) as usize);
    }
}
