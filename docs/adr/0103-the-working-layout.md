---
status: accepted
date: 2026-09-26
depends-on: ADR-0102
decision-makers: VirtualCortex maintainers
---

# ADR-0103: The working layout — ADR-0099's second lever: phase 1 may gather a chunk of a worker's scheduled units' integrated fields into lanes, integrate them together and write them back within the turn phase, the 64-byte record staying the only home of a unit's state across ticks, so quality goal 2 holds unchanged; the lanes in safe Rust for the compiler's vectorizer, as ADR-0039's hypervector does, with no intrinsic, no target feature, no dependency; every lane bit for bit the scalar rule, held by a property test over the lattice; the gain on ADR-0101's workload against ADR-0099's bounds; brief 045

## Context and Problem Statement

[ADR-0102](0102-the-sweep-timed-with-no-census.md) kept the sweep without the gate. Each worker owns a fixed range of the unit arena and serves, in unit order, the units a schedule bitmap names, with no deque, no stealing and no compare-and-swap in the turn.

On one worker, the sweep's turn at [ADR-0097](0097-the-active-set-measured.md)'s run (a) is about 11.4 ns, and the bench's `neuron/integrate` reads 10.96 ns on the same machine (a developer machine's, not admissible). **On the sweep the integration is the turn.** [ADR-0099](0099-the-engines-speed.md) ordered the working layout for the vector units next. It left to this ADR the question ADR-0099 and [ADR-0098](0098-the-integration-model.md) named: quality goal 2 says the state per neural unit MUST be exactly one cache line, and [ADR-0001](0001-64-byte-pod-records.md) kept a struct of arrays "available inside a subsystem where SIMD gathers need it; the 64-byte record is the interchange unit, not a ban on SoA scratch buffers".

The tree, read on 2026-09-26:

- `DendriticSuperNeuron::integrate` (`crates/cortex-core/src/dynamics/membrane.rs`) reads and writes the four potentials `v_soma`, `v_basal`, `v_apical` and `v_thresh`, contiguous at `[24..40)`. It also reads and writes the two countdowns `bac_plateau_ticks` and `refractory_ticks` at `[40..44)`, the burst flag in `flags`, and, on a spike, `last_soma_spike_tick`.
  - Each step is saturating integer arithmetic: a leak by a right shift with a one-LSB floor, never past zero; a coupling by a right shift of a difference in `i64`; a threshold decay toward its base.
  - Branches decide the refractory window, the burst's coupling and the spike.
- The turn around it (`Worker::turn`, `runtime/cortex-runtime/src/executor.rs`) does the following for every unit:
  - drains the mailbox if it holds mail;
  - sorts the batch, sums it and scales it by the gain;
  - integrates;
  - on a spike, steps the short-term factors, counts a descendant, and records the spike in the spiked list, the trace and the train;
  - keeps the unit on the schedule if it is not at rest, and records the gate byte.
  At run (a) about 8.6 messages reach 1 024 units a tick, so nearly every turn has no mail. The units that fire in a tick are about 0.0018 per cent.
- **The precedent for vector code.** The hypervector body's rules ([ADR-0039](0039-hypervector-body.md), whitepaper §5.2) are "a loop over the words with named operations and no intrinsic". The reasons given are that every `core::arch` intrinsic is `unsafe`, that `core::simd` is nightly, and that "the compiler vectorises the loops for the target it builds for and the result is the same integer everywhere, which is what T-1 holds". The workspace sets no `target-cpu` and no `rustflags`, so the build's baseline is SSE2 on x86-64 and NEON on AArch64: four `i32` lanes on both.

## Decision Drivers

- **Every reading of behaviour bit for bit** (ADR-0099): a lane must compute exactly what the scalar `integrate` computes for the same unit and inputs, on both architectures.
- **Quality goal 2 and ADR-0001.** The record is the state, the image's layout, and what the clock sweep, the image writer, STDP's read of a target's spike stamp and every test read between ticks.
- **Latest ≠ Newest** (whitepaper §2.1): no nightly `core::simd`, no sub-1.0 vector crate, no dependency. The compiler's loop and SLP vectorizers are mature, and ADR-0039 already relies on them.
- **`unsafe` stays where ADR-0023 allows it**, and `cortex-core` forbids it.
- **ADR-0078.** The need is measured: the integration is the turn. The gain is read, not argued.

## Considered Options

For where the lanes live:

1. **A transient chunk.** Within phase 1, a worker takes a chunk of its scheduled units (from a word's set bits, in unit order), gathers their integrated fields and inputs into lane arrays, integrates the lanes, and writes the fields back to the records before the chunk's spikes are recorded. Nothing persists outside the phase.
2. **A persistent working copy.** The integrated fields live in a struct of arrays per worker range across ticks, and the records are brought up to date on demand.
3. **Lanes within a record.** The four contiguous potentials of one unit are one 128-bit vector, and the leaks and the threshold's decay run in its lanes.

For how the vector code is written: safe Rust over fixed-size arrays for the compiler to vectorize; or `core::arch` intrinsics per architecture; or `core::simd`.

## Decision Outcome

**Option 1 or option 3, the round's choice on its own readings. Option 2 is not admissible in this line. The code is safe Rust for the compiler's vectorizer.**

- **Quality goal 2 holds, unchanged.** The 64-byte record stays the only home of a unit's state between turn phases. A lane array is scratch within phase 1, as ADR-0001 allows. Option 2 would give a unit's state a second home across ticks. The image, the clock sweep, STDP's read of a target's stamp, the differential test's arena and every reader of `units()` would then each need an argument for which home is current. That is an ADR of its own, which amends quality goal 2 and ADR-0001, and is not this line.
- **The rule, in lanes, in `cortex-core`.** A lane form of the membrane's rule lives beside `integrate` as a pure function over fixed-size arrays: no `unsafe`, no allocation, no dependency, `#![no_std]` as the crate is. **`integrate` itself does not change.** The scalar rule stays the specification, and the lane form is proven against it:
  - a property test over the lattice of `testkit/prop.rs`: every lane of the lane form equals `integrate` on the same unit and inputs, including the refractory window, a plateau, the threshold above its base, a spike and the saturating ends;
  - the mutation gate on its lines.
  The lane width, the chunk's length and whether the spike's rare path runs in lanes or falls back to the scalar rule are the round's.
- **The executor** (option 1) gathers and scatters a chunk within `phase_turns`. It keeps the sweep's unit order for the spiked list, the fan-out and the nodes a later message takes. It keeps the mail path, the gain's scaling, the short-term step on a spike, the descendant count, the trace, the train and the schedule's record as they are, for each unit. Phase 1's `unsafe` keeps ADR-0100's invariant, restated if a chunk holds more than one unit's reference at once. Option 3 changes only how `integrate`'s arithmetic is laid out, inside `cortex-core`.
- **No intrinsic, no target feature, no dependency.** `core::arch` intrinsics are `unsafe` and per architecture; `core::simd` is nightly; neither is taken. No `target-cpu` or `rustflags` is set: moving the build's target features is a decision about every binary the tree builds, not this lever's. If the compiler does not vectorize the lanes at the baseline, the criterion reads that, and a further step is an ADR's.
- **The acceptance**, ADR-0099's, unchanged: every reading of behaviour bit for bit; a pin that also holds the scheduler's own bytes restated only under the masked check; any other moved pin stops the round. No record field and no image format moves.
- **The measure**, ADR-0101's workload and ADR-0099's bounds, unchanged:
  - *Workload:* ADR-0097's four configurations with no census in the timed ticks (`tests/active.rs`, `Census::Off`).
  - *Builds and pairs:* the base is `main` at the round's start (the sweep kept) and the change is the base with the lanes. Five alternating pairs, the idle check and the disturbance rule of ADR-0102, each run's median.
  - *Criterion:* kept if at most **0.80** of the base's wall time per tick at (a) and (d), and at most **1.10** at (b) and (c).
  - *Read beside, not a criterion:* one worker at (a) and (c), and the per-turn breakdown after the change.
  The code the session times is the code the mutation gate read, committed before the first timed run.
- **After this lever.** ADR-0099 put the lookahead third. At the sizes the tree runs, the barriers are about 4 per cent of a tick and the arenas fit the caches, so the lookahead would read little there. The round's ADR names the next decision and does not take it: the lookahead, or the speed line closed and the learning line resumed from H-18's named next decision (ADR-0098).

### Consequences

- Good: quality goal 2 and ADR-0001 stand as written; the lanes are the scratch ADR-0001 kept room for.
- Good: the scalar rule stays the specification. The lane form is held to it on every lane over the lattice, so a vectorized result is the scalar result on both architectures, which T-1 needs.
- Good: no `unsafe`, intrinsic, target feature or dependency is added, which is ADR-0039's footing.
- Bad: the gain depends on what the compiler makes of the lanes at the baseline, four `i32` lanes with SSE2's integer operations on x86-64. The saturating steps widen to `i64`, which SSE2 serves poorly. The criterion may read no gain worth keeping.
- Bad: two forms of one rule must be kept in step. Each is held to the other only by the property test and the pins.
- Neutral: a persistent working copy, and a move of the build's target features, are named here and left to ADRs of their own.

## Alternatives considered and why rejected

- **Option 2, a persistent working copy**: it gives a unit's state two homes across ticks, which quality goal 2 and ADR-0001 do not allow without an ADR that re-argues every reader of a record.
- **`core::arch` intrinsics**: they are `unsafe` in a crate that forbids it, need a code path per architecture held bit for bit to the other, and would move the vector code into the runtime.
- **`core::simd`**: it is nightly, which Latest ≠ Newest rejects.
- **A vector crate** (`wide`, `pulp` and the like): each is a dependency, and state crates declare none. They are sub-1.0 without a stability policy.
- **Setting `target-cpu=native` or AVX2**: that makes the binary the machine's, which is a decision about every build and CI job. It is not this lever's.

## Confirmation

`briefs/045_the-working-layout.md` is the round that builds and times the lanes under this decision. Whitepaper 4.55.0 carries §11.1's question on the engine's speed with this decision, and §9's row. `npm run spec` holds the links, the rows, the brief's sections and the version.
