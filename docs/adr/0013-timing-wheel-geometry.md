---
status: accepted
date: 2026-09-10
decision-makers: VirtualCortex maintainers
amends: ADR-0004
---

# ADR-0013: Timing wheel geometry — 256 × 10 µs fine, 256 × 100 µs coarse, fixed-capacity token lists

## Context and Problem Statement

[ADR-0004](0004-two-tier-timing-wheel.md) chose a two-tier timing wheel and left two points open (whitepaper finding F-11): the rings were 200 and 80 slots long, so slot selection was a multiply-shift rather than a mask; and each slot was a 64-bit lane mask, which cannot carry the thousands of deliveries a 10 µs slot must hold at scale, while the capacity plan (whitepaper Appendix A) budgeted 8 MB per worker for slots of `SynapseBlock` offsets. Nothing drained the wheel. What are the ring lengths, what does a slot hold, and how does the coarse ring reach the fine ring?

## Decision Drivers

- O(1) schedule and advance, no allocation, no comparison (ADR-0003, ADR-0004).
- Deterministic delivery order (whitepaper §8.3): two workers replaying the same schedule must produce identical slots in identical order. Global scheduling order is not required, only a fixed order.
- Biological axonal delays run from about 1 ms to 20–30 ms; a horizon of 8 ms was too short.
- Memory per worker must stay a few megabytes so 64 wheels fit the capacity plan.
- A slot must hold many events, not 64 lanes.

## Considered Options

1. Rings of 256 fine and 64 coarse slots (2.56 ms / 6.4 ms), lane masks kept.
2. **Rings of 256 fine (10 µs) and 256 coarse (100 µs) slots; each slot a fixed-capacity list of opaque 28-bit tokens; coarse entries carry their fine residual in the top four bits; the coarse slot whose window begins at the current tick is cascaded into the fine ring before that tick's slot is returned.**
3. A single 2 560-slot fine ring (25.6 ms) with no cascade.
4. Per-slot linked lists of blocks (dynamic).

## Decision Outcome

Option 2, as `cortex_core::FlatTimingWheel<const CAP: usize>` with `WorkerWheel = FlatTimingWheel<2048>`.

- **Geometry.** Fine 256 × 10 µs = 2.56 ms; coarse 256 × 100 µs = 25.6 ms; ten fine ticks per coarse slot. Both lengths are powers of two: `slot = due & 255`, `window = (due / 10) & 255`. The horizon is 2 560 fine ticks; `schedule` returns `BeyondHorizon` at or above it, `ZeroDelay` for 0 (R-1 delivers zero-delay spikes through the mailbox), `TokenTooLarge` above 2²⁸ − 1, and `SlotFull` when a slot holds `CAP` tokens. No error mutates the wheel.
- **Slot representation.** A slot is `[u32; CAP]` with a `u16` length. A token is opaque to the wheel: a `SynapseBlock` offset (the arena of 128 M blocks needs 27 bits) or a unit index. In the coarse ring the token's top four bits hold the residual `due % 10`, which is why tokens are 28 bits.
- **Cascade.** `advance` clears the slot consumed at the previous tick, increments the tick, and, when the tick is a multiple of ten, moves every entry of coarse window `(tick / 10) & 255` into fine slot `(tick + residual) & 255`, clearing the window. Those fine slots are the current one and the next nine, never the consumed one, so a cascaded token is neither lost nor late. It then returns the due slot as a slice. The order is deterministic but not global scheduling order: tokens scheduled directly into the fine slot before the cascade come first, then the cascaded tokens, then any fine-scheduled after the window began, each group in scheduling order; a unit test pins this. A coarse entry is accepted at schedule time; if the fine slot is full at cascade time the token is dropped rather than the tick loop aborted, and the runtime sizes `CAP` so that this cannot happen.
- **Memory.** `WorkerWheel` is 4 195 336 bytes: two rings of 256 × 2 048 × 4 bytes plus two length arrays and the tick. Sixty-four workers: 268.5 MB (Appendix A row 19). The wheel is per-worker runtime state, not part of the `.cortex` image, so `FORMAT_VERSION` is unaffected.
- **What a lane was.** The 64-bit mask is gone; there are no lanes. Delivery is a list of tokens.

### Consequences

- Good: O(1) schedule with a mask; O(1) advance plus the length of one cascaded window every ten ticks; deterministic order; 3.2× the previous horizon.
- Good: `CAP` is a const generic, so tests run on tiny wheels and the production alias is fixed in one place.
- Bad: a 4 MB value per worker must live in an arena, never on a stack; the runtime allocates it once (ADR-0003).
- Bad: fixed capacity means a hot slot can fill; the failure is explicit (`SlotFull`) at schedule time and silent (dropped) only at cascade time, which is documented and is a sizing fault, not a logic path.
- Bad: tokens are 28 bits; an arena larger than 268 M entries would need a wider token and a narrower residual.
- The `CortexFileHeader` question (should an image record tick duration?) is not answered here: geometry and tick sizes are `cortex-core` constants; a self-describing image needs the tick duration, and that belongs to the loader milestone.

## Alternatives considered and why rejected

- **Option 1**: keeps lane masks, which cannot represent a slot's real load; 6.4 ms is below biological delays.
- **Option 3**: 2 560 slots × 8 KB = 20 MB per worker, five times the two-tier cost, to remove a cascade that costs one window copy every ten ticks.
- **Option 4**: dynamic lists allocate on the hot path (ADR-0003) and chase pointers (ADR-0001).

## Confirmation

Eight unit tests in `cortex-core`: delivery on exactly the requested advance across both rings and both wrap boundaries; exactness from eight starting ticks including the coarse wrap; no early delivery; rejections leave the wheel unchanged; a full slot is reported and the rest still arrives; the delivery order across both rings; two wheels fed a pseudo-random sequence of 4 000 schedules agree at every tick; the constants. `size_of::<WorkerWheel>()` is asserted at compile time. `npx spec-guard` asserts `WorkerWheel` and `ScheduleError` exist.
