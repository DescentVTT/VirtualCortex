//! A sense-reversing spin barrier: every worker waits for every other at the end of each
//! phase of a tick (ADR-0023). Spinning, not a mutex and condition variable, because the tick
//! loop must not block or make system calls (whitepaper TC-5); after a bounded number of
//! spins the waiter yields, which is the one system call the loop makes until the workers are
//! pinned to isolated cores (§7.3, Specified) and never share a core.

use std::hint::spin_loop;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;

/// Spins before the first yield.
const SPINS_BEFORE_YIELD: u32 = 1 << 10;

pub struct SpinBarrier {
    parties: usize,
    count: AtomicUsize,
    generation: AtomicUsize,
}

impl SpinBarrier {
    /// A barrier for `parties` participants (at least one).
    pub fn new(parties: usize) -> Self {
        Self {
            parties: parties.max(1),
            count: AtomicUsize::new(0),
            generation: AtomicUsize::new(0),
        }
    }

    /// Blocks until every participant has called `wait` for this generation. Returns `true`
    /// on the participant that arrived last.
    pub fn wait(&self) -> bool {
        let generation = self.generation.load(Ordering::Acquire);
        // The count before this arrival is below `parties`, so the increment cannot wrap.
        if self.count.fetch_add(1, Ordering::AcqRel).wrapping_add(1) == self.parties {
            self.count.store(0, Ordering::Relaxed);
            self.generation.fetch_add(1, Ordering::Release);
            return true;
        }
        let mut spins = 0u32;
        while self.generation.load(Ordering::Acquire) == generation {
            if spins < SPINS_BEFORE_YIELD {
                spins = spins.wrapping_add(1);
                spin_loop();
            } else {
                thread::yield_now();
            }
        }
        false
    }

    /// The number of participants.
    pub fn parties(&self) -> usize {
        self.parties
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::sync::atomic::AtomicU64;

    #[test]
    fn the_party_count_is_at_least_one_and_reported() {
        assert_eq!(SpinBarrier::new(0).parties(), 1);
        assert_eq!(SpinBarrier::new(4).parties(), 4);
    }

    #[test]
    fn a_barrier_of_one_never_waits() {
        let b = SpinBarrier::new(1);
        assert_eq!(b.parties(), 1);
        for _ in 0..1000 {
            assert!(b.wait());
        }
        assert_eq!(SpinBarrier::new(0).parties(), 1);
    }

    #[test]
    fn no_participant_passes_a_generation_before_every_other_has_arrived() {
        const PARTIES: usize = 4;
        const ROUNDS: u64 = 2000;
        let barrier = Arc::new(SpinBarrier::new(PARTIES));
        let arrivals = Arc::new(AtomicU64::new(0));
        let handles: Vec<_> = (0..PARTIES)
            .map(|_| {
                let barrier = Arc::clone(&barrier);
                let arrivals = Arc::clone(&arrivals);
                thread::spawn(move || {
                    let mut last = 0;
                    for round in 1..=ROUNDS {
                        arrivals.fetch_add(1, Ordering::SeqCst);
                        last += barrier.wait() as u64;
                        // After the wait, every participant of this round has arrived.
                        assert!(arrivals.load(Ordering::SeqCst) >= round * PARTIES as u64);
                    }
                    last
                })
            })
            .collect();
        let lasts: u64 = handles.into_iter().map(|h| h.join().unwrap()).sum();
        assert_eq!(lasts, ROUNDS, "exactly one participant is last per round");
        assert_eq!(arrivals.load(Ordering::SeqCst), ROUNDS * PARTIES as u64);
    }
}
