---
status: accepted
date: 2026-09-10
decision-makers: VirtualCortex maintainers
---

# ADR-0006: Virtual-actor turn invariant enforced by an atomic gate

## Context and Problem Statement

Tens of millions of neural units share a few dozen worker threads. Two workers must never update the same unit concurrently, and the mechanism that guarantees this must cost nothing on the common path and never block.

## Decision Drivers

- No mutex on a 64-byte record: a lock word alone would consume space and a contended lock would stall an isolated core.
- Units are passive data (axiom A2); behaviour is in the worker.
- Deterministic scheduling requires that "who runs this unit" is decided by a single atomic operation.

## Considered Options

1. A mutex or spinlock per unit.
2. Sharding units to workers statically by id, with cross-shard messages.
3. **A one-byte atomic gate per unit (`gate_state`: idle / scheduled / running). A pusher does `compare_exchange(idle -> scheduled)` and, on success, enqueues the unit; a worker sets `running` on claim and `idle` on release. Inputs accumulate in a lock-free MPSC mailbox whose head pointer is paired with an ABA tag.**

## Decision Outcome

Option 3. The fields `gate_state: AtomicU8`, `mailbox_head_ptr: AtomicU64` and `mailbox_tag: u64` in `DendriticSuperNeuron` exist for this purpose. A worker that claims a unit drains its entire mailbox in one pass (batch draining), then integrates once.

### Consequences

- Good: no locks, no deadlock, single-writer semantics for every plain field.
- Good: the unit is scheduled at most once no matter how many spikes arrive between turns.
- Bad: a unit is not migratable mid-turn; a very hot unit serialises on one worker. Biological sparsity makes this rare; a hot-spot diagnostic belongs in telemetry.
- Bad: the mailbox node pool must be sized at start-up (ADR-0003).

## Confirmation

The gate protocol is Specified. Milestone M1's exit test is a push, gate, callback unit test; milestone M2's is a 10⁶-event contention test with no loss and no deadlock.
