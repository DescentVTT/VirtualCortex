---
status: accepted
date: 2026-09-10
decision-makers: VirtualCortex maintainers
---

# ADR-0004: Two-tier timing wheel for axonal delay

## Context and Problem Statement

Every synapse carries a conduction delay of one to tens of milliseconds. A simulator must deliver each spike at the right future tick. The classical structure is a priority queue, which is O(log n) per operation, pointer-based, and hostile to the cache at hundreds of millions of events per second.

## Decision Drivers

- O(1) insert and expiry regardless of load.
- No dynamic memory, no comparison, no rebalancing.
- Deterministic delivery order within a tick (whitepaper §8.3).
- Delays are bounded and quantised, so a ring is a natural fit (axiom A4).

## Considered Options

1. Binary heap keyed by delivery tick.
2. Calendar queue with dynamic bucket resizing.
3. **Hashed timing wheel (Varghese and Lauck, 1987) with two tiers: a fine ring of 10 µs slots and a coarse ring of 100 µs slots that drains into the fine ring.**

## Decision Outcome

Option 3, implemented as `FlatTimingWheel` in `cortex-core` with `fine_ring: [u64; 200]` and `coarse_ring: [u64; 80]`. Horizons are 2 ms and 8 ms at the current slot widths; a delay beyond the coarse horizon is a load-time error.

### Consequences

- Good: every insert is one index computation and one OR; every tick drain is one load and one clear.
- Good: the per-worker wheel is 2 248 bytes and lives in L1.
- Bad: the ring length 200 is not a power of two, so slot selection is a multiply-shift rather than a mask; and each slot is a 64-bit lane mask rather than a list of `SynapseBlock` offsets (finding F-11). Both are open questions in whitepaper §11.1.
- Bad: delay resolution is the slot width; sub-slot jitter is quantised away by design.

## Confirmation

`npx spec-guard` asserts `FlatTimingWheel` exists in `cortex-core`. Milestone M3 adds a three-neuron delayed oscillator whose period must be exact to the tick.
