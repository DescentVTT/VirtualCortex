//! Per-worker pools of mailbox nodes (whitepaper §8.5: nodes come from a per-worker pool,
//! never from the allocator). One arena of `MailboxNode`s, one free list per worker, threaded
//! through the nodes' own `next` fields with the index + 1 encoding of ADR-0017: a node is in
//! a mailbox, in the hands of the worker that drained it, or in its owner's free list, never
//! two at once. The owner pops; any worker pushes a node back to the owner's list when it has
//! drained it. Only the owner pops, so the compare-exchange on the head cannot see the ABA
//! pattern (the same argument as ADR-0017's mailbox).

use cortex_core::{MAILBOX_NIL, MailboxNode};
use std::sync::atomic::{AtomicU32, Ordering};

pub struct Pools {
    nodes: Box<[MailboxNode]>,
    heads: Box<[AtomicU32]>,
    per_worker: usize,
}

impl Pools {
    /// `workers` pools of `per_worker` nodes each, every node free.
    pub fn new(workers: usize, per_worker: usize) -> Self {
        let total = workers * per_worker;
        let nodes: Box<[MailboxNode]> = (0..total).map(|_| MailboxNode::new()).collect();
        let heads: Box<[AtomicU32]> = (0..workers)
            .map(|w| {
                let first = w * per_worker;
                for i in first..first + per_worker {
                    let next = if i + 1 < first + per_worker {
                        i as u32 + 2
                    } else {
                        MAILBOX_NIL
                    };
                    nodes[i].next.store(next, Ordering::Relaxed);
                }
                AtomicU32::new(if per_worker > 0 { first as u32 + 1 } else { 0 })
            })
            .collect();
        Self {
            nodes,
            heads,
            per_worker,
        }
    }

    /// The node arena every mailbox indexes.
    pub fn nodes(&self) -> &[MailboxNode] {
        &self.nodes
    }

    /// Nodes per pool.
    pub fn per_worker(&self) -> usize {
        self.per_worker
    }

    /// The owner of a node.
    pub fn owner(&self, node: u32) -> usize {
        node as usize / self.per_worker
    }

    /// Owner only: takes a free node of pool `worker`, or `None` when the pool is exhausted.
    pub fn alloc(&self, worker: usize) -> Option<u32> {
        let head = &self.heads[worker];
        let mut encoded = head.load(Ordering::Acquire);
        loop {
            if encoded == 0 {
                return None;
            }
            let node = encoded - 1;
            let next = self.nodes[node as usize].next.load(Ordering::Relaxed);
            match head.compare_exchange_weak(encoded, next, Ordering::AcqRel, Ordering::Acquire) {
                Ok(_) => return Some(node),
                Err(current) => encoded = current,
            }
        }
    }

    /// Any worker: returns a drained node to its owner's pool.
    pub fn free(&self, node: u32) {
        let head = &self.heads[self.owner(node)];
        let encoded = node + 1;
        let mut current = head.load(Ordering::Relaxed);
        loop {
            self.nodes[node as usize]
                .next
                .store(current, Ordering::Relaxed);
            match head.compare_exchange_weak(current, encoded, Ordering::Release, Ordering::Relaxed)
            {
                Ok(_) => return,
                Err(now) => current = now,
            }
        }
    }

    /// Free nodes in pool `worker` (a walk; for tests and reports).
    pub fn free_count(&self, worker: usize) -> usize {
        let mut n = 0;
        let mut encoded = self.heads[worker].load(Ordering::Acquire);
        while encoded != 0 && n <= self.per_worker {
            n += 1;
            encoded = self.nodes[encoded as usize - 1]
                .next
                .load(Ordering::Relaxed);
        }
        n
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn each_pool_hands_out_its_own_nodes_once_and_takes_them_back() {
        let pools = Pools::new(2, 3);
        assert_eq!(pools.nodes().len(), 6);
        assert_eq!(pools.per_worker(), 3);
        assert_eq!((pools.free_count(0), pools.free_count(1)), (3, 3));
        let mut got = [pools.alloc(0), pools.alloc(0), pools.alloc(0)];
        assert_eq!(pools.alloc(0), None, "exhausted");
        got.sort();
        assert_eq!(got, [Some(0), Some(1), Some(2)]);
        assert_eq!(pools.owner(4), 1);
        assert_eq!(pools.alloc(1), Some(3));
        pools.free(1);
        pools.free(3);
        assert_eq!(pools.alloc(0), Some(1));
        assert_eq!(pools.alloc(1), Some(3));
        assert_eq!(pools.free_count(0), 0);
        assert_eq!(pools.free_count(1), 2);
    }

    #[test]
    fn nodes_freed_by_other_workers_come_back_to_their_owner_without_loss() {
        const NODES: usize = 64;
        let pools = Arc::new(Pools::new(1, NODES));
        let mut taken: Vec<u32> = (0..NODES).map(|_| pools.alloc(0).unwrap()).collect();
        assert_eq!(pools.alloc(0), None);
        taken.sort_unstable();
        assert_eq!(taken, (0..NODES as u32).collect::<Vec<_>>());
        // Three other workers free sixteen nodes each, slowly; the owner frees its own sixteen
        // and keeps taking and returning whatever has come back.
        let returners: Vec<_> = (0..3)
            .map(|r| {
                let pools = Arc::clone(&pools);
                let mine: Vec<u32> = taken[r * 16..(r + 1) * 16].to_vec();
                thread::spawn(move || {
                    for node in mine {
                        pools.free(node);
                        thread::yield_now();
                    }
                })
            })
            .collect();
        for &node in &taken[48..] {
            pools.free(node);
        }
        for _ in 0..20_000 {
            if let Some(node) = pools.alloc(0) {
                pools.free(node);
            }
        }
        for r in returners {
            r.join().unwrap();
        }
        assert_eq!(pools.free_count(0), NODES, "every node is back");
        let mut seen = std::collections::HashSet::new();
        let mut encoded = pools.heads[0].load(Ordering::Acquire);
        while encoded != 0 {
            assert!(seen.insert(encoded), "no node twice, no cycle");
            encoded = pools.nodes[encoded as usize - 1]
                .next
                .load(Ordering::Relaxed);
        }
        assert_eq!(seen.len(), NODES);
    }
}
