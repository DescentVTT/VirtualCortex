//! A bounded work-stealing deque of unit indices: Chase and Lev's algorithm (2005) with the
//! fences of Lê, Pop, Cox and Nardelli (2013) for the C11 memory model, without the growable
//! array. Every slot is an atomic and the indices are atomics, so it needs no `unsafe`; it
//! allocates once, at construction (ADR-0023). One owner pushes and pops at the bottom through
//! [`Local`]; any number of thieves take from the top through [`Stealer`]. The executor sizes
//! it so that it cannot fill: a unit is queued at most once at a time, by the gate of axiom A3,
//! so a capacity of one slot per unit suffices.

use std::sync::Arc;
use std::sync::atomic::{AtomicIsize, AtomicU32, Ordering, fence};

struct Inner {
    top: AtomicIsize,
    bottom: AtomicIsize,
    slots: Box<[AtomicU32]>,
    mask: usize,
}

/// The owner's end of a deque. Not `Clone`: one owner.
pub struct Local(Arc<Inner>);

/// A thief's end of a deque; any number may exist.
#[derive(Clone)]
pub struct Stealer(Arc<Inner>);

/// The deque is full: the capacity model was violated (whitepaper §8.9).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Full;

/// The outcome of one steal attempt.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Steal {
    /// Nothing to steal.
    Empty,
    /// A race with the owner or another thief was lost; try again.
    Retry,
    /// A unit index.
    Success(u32),
}

/// A deque holding at least `capacity` indices (rounded up to a power of two, at least 2).
pub fn new(capacity: usize) -> (Local, Stealer) {
    let size = capacity.max(2).next_power_of_two();
    let inner = Arc::new(Inner {
        top: AtomicIsize::new(0),
        bottom: AtomicIsize::new(0),
        slots: (0..size).map(|_| AtomicU32::new(0)).collect(),
        mask: size - 1,
    });
    (Local(Arc::clone(&inner)), Stealer(inner))
}

impl Local {
    /// Number of slots.
    pub fn capacity(&self) -> usize {
        self.0.slots.len()
    }

    /// Pushes at the bottom. Refused, with nothing changed, when every slot is taken.
    pub fn push(&self, unit: u32) -> Result<(), Full> {
        let b = self.0.bottom.load(Ordering::Relaxed);
        let t = self.0.top.load(Ordering::Acquire);
        if b.wrapping_sub(t) >= self.0.slots.len() as isize {
            return Err(Full);
        }
        self.0.slots[b as usize & self.0.mask].store(unit, Ordering::Relaxed);
        fence(Ordering::Release);
        self.0.bottom.store(b.wrapping_add(1), Ordering::Relaxed);
        Ok(())
    }

    /// Pops at the bottom (the most recently pushed), or `None` when empty.
    pub fn pop(&self) -> Option<u32> {
        let b = self.0.bottom.load(Ordering::Relaxed).wrapping_sub(1);
        self.0.bottom.store(b, Ordering::Relaxed);
        fence(Ordering::SeqCst);
        let t = self.0.top.load(Ordering::Relaxed);
        if t <= b {
            let unit = self.0.slots[b as usize & self.0.mask].load(Ordering::Relaxed);
            if t == b {
                // The last element: race the thieves for it.
                let won = self
                    .0
                    .top
                    .compare_exchange(t, t + 1, Ordering::SeqCst, Ordering::Relaxed)
                    .is_ok();
                self.0.bottom.store(b + 1, Ordering::Relaxed);
                return won.then_some(unit);
            }
            Some(unit)
        } else {
            self.0.bottom.store(b + 1, Ordering::Relaxed);
            None
        }
    }

    /// True when no index is queued (a snapshot; thieves may be at work).
    pub fn is_empty(&self) -> bool {
        let b = self.0.bottom.load(Ordering::Relaxed);
        let t = self.0.top.load(Ordering::Relaxed);
        b <= t
    }
}

impl Stealer {
    /// Takes from the top (the least recently pushed).
    pub fn steal(&self) -> Steal {
        let t = self.0.top.load(Ordering::Acquire);
        fence(Ordering::SeqCst);
        let b = self.0.bottom.load(Ordering::Acquire);
        if t < b {
            let unit = self.0.slots[t as usize & self.0.mask].load(Ordering::Relaxed);
            if self
                .0
                .top
                .compare_exchange(t, t + 1, Ordering::SeqCst, Ordering::Relaxed)
                .is_err()
            {
                return Steal::Retry;
            }
            Steal::Success(unit)
        } else {
            Steal::Empty
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn the_owner_pops_in_lifo_order_and_a_thief_steals_in_fifo_order() {
        let (local, stealer) = new(4);
        assert_eq!(local.capacity(), 4);
        assert!(local.is_empty());
        assert_eq!(local.pop(), None);
        assert_eq!(stealer.steal(), Steal::Empty);
        for i in 1..=4 {
            assert_eq!(local.push(i), Ok(()));
        }
        assert_eq!(local.push(5), Err(Full));
        assert_eq!(stealer.steal(), Steal::Success(1));
        assert_eq!(local.pop(), Some(4));
        assert_eq!(local.push(5), Ok(()));
        assert_eq!(local.pop(), Some(5));
        assert_eq!(stealer.steal(), Steal::Success(2));
        assert_eq!(local.pop(), Some(3));
        assert_eq!(local.pop(), None);
        assert_eq!(stealer.steal(), Steal::Empty);
        assert!(local.is_empty());
    }

    #[test]
    fn capacity_rounds_up_to_a_power_of_two_and_wraps_around_the_ring() {
        let (local, _s) = new(5);
        assert_eq!(local.capacity(), 8);
        for round in 0..100u32 {
            for i in 0..8 {
                assert_eq!(local.push(round * 8 + i), Ok(()));
            }
            assert_eq!(local.push(0), Err(Full));
            for i in (0..8).rev() {
                assert_eq!(local.pop(), Some(round * 8 + i));
            }
        }
    }

    #[test]
    fn every_pushed_index_is_taken_exactly_once_under_thieves() {
        const N: u32 = 200_000;
        let (local, stealer) = new(N as usize);
        let thieves: Vec<_> = (0..3)
            .map(|_| {
                let s = stealer.clone();
                thread::spawn(move || {
                    let mut taken = Vec::new();
                    let mut empties = 0;
                    while empties < 20_000 {
                        match s.steal() {
                            Steal::Success(x) => {
                                taken.push(x);
                                empties = 0;
                            }
                            Steal::Retry => {}
                            Steal::Empty => {
                                empties += 1;
                                thread::yield_now();
                            }
                        }
                    }
                    taken
                })
            })
            .collect();
        let mut mine = Vec::new();
        for i in 0..N {
            local.push(i).unwrap();
            if i % 3 == 0 {
                if let Some(x) = local.pop() {
                    mine.push(x);
                }
            }
        }
        while let Some(x) = local.pop() {
            mine.push(x);
        }
        let mut all = mine;
        for t in thieves {
            all.extend(t.join().unwrap());
        }
        all.sort_unstable();
        assert_eq!(all.len(), N as usize, "every index taken");
        all.dedup();
        assert_eq!(all.len(), N as usize, "none taken twice");
        assert_eq!(all[0], 0);
        assert_eq!(all[N as usize - 1], N - 1);
    }
}
