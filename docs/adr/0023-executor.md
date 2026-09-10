---
status: accepted
date: 2026-09-10
decision-makers: VirtualCortex maintainers
depends-on: ADR-0017
---

# ADR-0023: The executor — a runtime crate, in-house work-stealing deques, three barrier-separated phases per tick, and the one `unsafe` in the workspace

## Context and Problem Statement

Thirty-two state crates existed with no loop to run them: whitepaper R-1 steps 2 to 6 were record methods, R-2 a wheel per worker, and "the deque is the executor's (Specified)". Brief 012 asked for a runtime crate outside `crates/` with a fixed pool of workers, a work-stealing deque of unit indices per worker, batch draining in a deterministic order (§8.3), the wheel drained into mailboxes every fine tick, core pinning, milestone M2's exit test (10⁶ events delivered under contention with no loss and no deadlock), a differential test on one and four workers, and no allocation after start-up (TC-5). It asked the ADR to decide the deque (in-house without `unsafe`, or `crossbeam-deque` on [ADR-0005](0005-crate-per-subsystem.md)'s allow-list), the batch order, core pinning, how wheel tokens become mailbox pushes, and what happens to a unit whose turn cannot be claimed.

One fact the brief did not anticipate shaped the design. A `DendriticSuperNeuron` holds atomics (the gate, the mailbox head) that any worker touches through `&self`, and plain fields that the turn holder writes through `&mut self` (`integrate`, `step_stp`). In Rust a `&mut T` is exclusive over every byte of `T`: a pusher's `&T` alive while the turn holder's `&mut T` is alive is undefined behaviour whichever fields each touches, however the gate orders them. The same holds for a `SynapseBlock` written by its unit's fan-out and read by the worker whose wheel delivers a token naming it. No executor can share these records between threads in safe Rust; one that shares them in `unsafe` Rust must keep the two kinds of reference from ever overlapping in time, not only in the fields they touch.

## Decision Drivers

- TC-5: after initialisation the loop allocates nothing, blocks on nothing, makes no system call.
- TC-9: `unsafe` only under an ADR that names the invariant and the test.
- §8.3: bit-identical arenas on one and on many workers for the same input trace; a batch order that does not depend on arrival.
- §8.5: A3, the turn invariant; nodes from per-worker pools; single writer per record.
- "Latest ≠ Newest" and ADR-0005: a dependency needs a stability record; `crossbeam-deque` is sub-1.0 and the bounded deque this engine needs is a hundred lines of atomics.
- §8.9: inside the loop a violated invariant aborts.

## Considered Options

1. Workers hold `&mut` to units and `&` to others concurrently, with the gate as the only order; `crossbeam-deque`; mutex-and-condvar synchronisation.
2. **A tick of three phases separated by a spin barrier, so that every `&mut` a worker creates is dropped before any other worker can create a `&` to the same record; an in-house bounded Chase–Lev deque of atomics; an in-house bounded MPMC ring as the only entry for the outside; per-worker node pools with lock-free free lists; every buffer sized at construction; `unsafe` confined to one arena type.**
3. Make every field of the two records atomic, so the executor is safe Rust with relaxed loads and stores everywhere.

## Decision Outcome

Option 2. The crate is `runtime/cortex-runtime`, `publish = false`, `std`, depending on `cortex-core` and nothing else; `benches/cortex-bench` depends on it for its benchmark. It is outside `crates/`, so the `spec-guard` directives that hold TC-4, TC-5 and TC-9 over the state crates do not reach it; this decision and its tests hold it.

- **Phases.** A tick $t$ is, on every worker, (1) *turns*: pop units from the local deque, steal when it is empty; per unit `begin_turn`, drain the mailbox into a bounded buffer, `sort_unstable` it (§8.3), sum the basal and apical efficacies saturating, `integrate(basal, apical, t)`, `step_stp` if it fired, `end_turn`, and if the unit is not at rest `try_schedule` it onto the worker's next-tick list; (2) *fan-out*: for each unit that fired, walk its chain, per block `step_stdp_all(t, the four targets' last spikes)` and `release_all(u, r)`, then each synapse with delay 0 into the target's mailbox and each other into this worker's wheel as a `synapse_token`; (3) *deliveries*: `advance` the wheel to $t + 1$, push each due token's stored release as a `spike_message` into its target's mailbox, on worker 0 drain at most one ring's worth of the injector, and move the next-tick list onto the deque. A push does `try_schedule` and, when it wins, queues the unit on the pushing worker's deque. Four barrier waits per tick: before phase 1 (the coordinator publishes $t$), between the phases, and after phase 3, so that between ticks no worker holds any reference and the caller may read or wire the arenas.
- **References.** In phase 1 a worker holds `&mut` to the unit whose turn it holds and to nothing else; no push happens. In phase 2 it holds `&mut` to the blocks of the units it ran that fired, which no other worker walks (a unit fires at most once per tick and lands in one worker's list), and `&` to units, which nobody holds mutably. In phase 3 every reference is `&`. The arena type (`arena.rs`) is the only `unsafe`: `UnsafeCell` cells with `get` and `get_mut` as `unsafe fn`s whose contract is the sentence above; every call site names its phase. The invariant is checked by the differential test (a race would break bit-identity) and the contention test, and by the compiler's `invalid_reference_casting` and `mut_from_ref` lints being satisfied through `UnsafeCell::raw_get`.
- **Deque.** A bounded Chase–Lev deque over `[AtomicU32]` with `AtomicIsize` top and bottom and the fences of Lê, Pop, Cox and Nardelli (2013); owner pushes and pops at the bottom (`Local`), thieves steal at the top (`Stealer`, `Retry` on a lost race). Capacity one slot per unit by default, which cannot fill: a unit is queued at most once at a time, by its gate. A full deque or an unclaimable turn is an invariant violation and aborts (§8.9): a unit reaches a deque only by winning the idle-to-scheduled transition, so `begin_turn` cannot fail.
- **Timing.** A message pushed in phase 2 or 3 of tick $t$ is integrated in phase 1 of $t + 1$; a synapse of delay $d \ge 1$ scheduled in phase 2 of $t$ is due at $t + d$ and delivered in phase 3 of $t + d - 1$, so it too is integrated at $t + d$. A zero-delay synapse therefore arrives with a one-tick one; the difference is the path (mailbox now, not the wheel). The single-threaded loop of `crates/cortex-core/tests/oscillator.rs` integrates a zero-delay message in the same tick when the target's turn comes later; the executor never does, so that the result is the same on every worker count.
- **Active set.** A unit integrates every tick from the one it is woken until it is at rest: every potential zero, no refractory or plateau window, the threshold at or below its base. It then leaves the deque until a message wakes it. A unit that is at rest costs nothing; one that is not is served every tick, so no unit starves.
- **Outside.** The injector is a bounded MPMC ring (Vyukov's sequence-numbered queue) of `(unit, payload)`; producers push from any thread, worker 0 drains at most one ring's worth per tick. `Inject::inject(unit, message)` and `Inject::activate(unit)` are the only way in; a unit index outside the arena or the reserved payload is refused before the ring. `Executor::units_mut` and `blocks_mut` wire a network between ticks.
- **Pools.** One arena of `MailboxNode`s, one free list per worker threaded through the nodes' `next` fields (index + 1); the owner pops, any worker pushes back a node it drained. Only the owner pops, so the compare-exchange cannot see the ABA pattern. A tick takes at most one wheel slot's tokens, the zero-delay synapses of the units that fired and, on worker 0, one ring's worth; a pool that runs out aborts (§8.9) rather than losing a message.
- **Barrier.** A sense-reversing spin barrier of atomics; after 1 024 spins a waiter yields, the one system call in the loop until the workers are pinned. Worker 0 is the thread that calls `tick`; with one worker no thread is spawned and no barrier waits.
- **Core pinning.** Specified. It needs `sched_setaffinity` (Linux) or `SetThreadAffinityMask` (Windows), which is `libc` or a sub-1.0 crate; the ADR that admits one names it. Until then the workers are unpinned `std` threads and the barrier's yield is what keeps an oversubscribed machine from stalling.
- **Allocation.** Every arena, pool, deque, ring, wheel and buffer is allocated in `Executor::new`; the wheels on a thread whose stack holds one. The buffers are bounded by the theoretical maximum (a batch by the node count, the fired list and the next-tick list by the unit count, the due list by the slot capacity) and a push past a bound aborts rather than growing. A counting global allocator in `tests/no_alloc.rs` counts zero allocations over 20 000 ticks of spikes, fan-out, cascades and injections.

### Consequences

- Good: the M2 exit test passes on one, two and four workers: 10⁶ events from four producers into 64 units, delivered exactly once, no deadlock; and the first differential test toward T-1 passes: a 128-unit random network with STDP and the oscillator ring leave bit-identical arenas and spike trains on one and four workers.
- Good: no dependency entered; TC-2's sentence still holds.
- Good: the phase discipline makes the lost-wakeup rule of [ADR-0017](0017-mailbox-and-gate-protocol.md) unreachable (no push overlaps a turn); it stays as defence in depth and `end_turn`'s answer is honoured if it ever comes.
- Bad: four barrier waits per 10 µs tick. On 64 workers that is the dominant cost of an idle tick and a T-3 measurement subject; pipelining the phases across ticks is the next executor round, not this one.
- Bad: the deque's one-slot-per-unit sizing is $W \times N \times 4$ bytes; at 64 workers and the Appendix A unit count it is a global overflow ring's job, which this crate does not yet have.
- Bad: a worker that panics inside a phase hangs the others at the barrier; the loop aborts on invariant violations instead, as §8.9 says, and a test that hangs is a test that found a bug.
- Bad: a batch buffer of one slot per node per worker.

## Alternatives considered and why rejected

- **Option 1** is undefined behaviour in Rust and a stack of futex system calls in the loop.
- **Option 3** changes the field types of the two hot records and every method on them for a property the phases give without touching a state crate; and it does not remove the barrier, which determinism needs anyway (a worker that is a tick ahead delivers into the past).
- **`crossbeam-deque`** is sub-1.0; the bounded deque needs no growth, no epochs and no `unsafe`.
- **A mutex per unit** serialises pushers with the turn holder and makes the lock-free mailbox pointless.
- **Same-tick delivery of zero-delay synapses** (as the oscillator harness does) depends on the order in which units are processed, which is the worker count's, and would break bit-identity.

## Confirmation

Twelve unit tests (`arena`, `deque` with three thieves over 2 × 10⁵ indices, `barrier` with four parties over 2 000 generations, `injector` with four producers, `pool` with three returners, `executor` configuration and the active-set rule). Integration tests: `tests/contention.rs` (M2: 10⁶ events exactly once on 1, 2 and 4 workers; a unit that keeps receiving is served every tick), `tests/differential.rs` (the ring and a random network with STDP bit-identical on 1 and 4 workers; a delayed synapse arrives `delay` ticks after the spike and a zero-delay one on the next tick), `tests/no_alloc.rs` (zero allocations after `new`). `npx spec-guard` asserts `runtime/cortex-runtime` is a workspace member, that every `unsafe fn` of the crate is in `arena.rs` and every `unsafe` block carries a `SAFETY` comment, and that `crates/` still has no `unsafe`.
