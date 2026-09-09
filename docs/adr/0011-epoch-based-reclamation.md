---
status: accepted
date: 2026-09-10
decision-makers: VirtualCortex maintainers
---

# ADR-0011: Epoch-based reclamation for structural plasticity

## Context and Problem Statement

Synapses are created and pruned continuously while the simulation runs (structural plasticity), and the immune scrubber compacts arenas during sleep. Both mutate `SynapseBlock` chains that workers are concurrently traversing. A stop-the-world pause is not acceptable for an engine that drives a physical body at 1 ms.

## Decision Drivers

- Readers (workers) must never block or take a lock (ADR-0003, ADR-0006).
- A retired block must not be reused while any worker may still hold its index.
- The mechanism must be mature: it will guard every synapse in the engine.

## Considered Options

1. Reference counting per block (a contended atomic on every traversal).
2. Hazard pointers (per-reader slots to scan before every free).
3. **Epoch-based reclamation (Fraser 2004): readers pin the global epoch on entry to a turn; writers retire blocks into the current epoch's queue; a block is reused only after every worker has advanced past that epoch.**

## Decision Outcome

Option 3. The mechanism is Specified; the implementation will use `crossbeam-epoch`, which has been in production since 2017 and is on the dependency allow-list of ADR-0005, unless a `no_std`-compatible in-house variant proves necessary for the runtime crate. Retirement and reuse are recorded per arena segment in `ImmuneScrubNode`.

### Consequences

- Good: the read path costs one non-contended atomic per turn, not per traversal.
- Good: compaction and pruning proceed while the tick loop runs; there is no pause to schedule.
- Bad: memory is reclaimed late, by up to a few epochs; the free pool must carry that slack.
- Bad: a stalled worker pins an epoch and stops reclamation; the watchdog must detect it.

## Confirmation

Milestone M5 adds a concurrent prune-while-traverse test under a race detector (`loom` for the state machine, `miri` for the unsafe core once it exists).
