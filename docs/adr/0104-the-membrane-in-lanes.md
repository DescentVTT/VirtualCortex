---
status: proposed
date: 2026-09-26
depends-on: ADR-0103
decision-makers: VirtualCortex maintainers
---

# ADR-0104: The membrane's rule in lanes — ADR-0103's first option: phase 1 integrates a chunk of eight of a worker's scheduled units in two vectors of four `i32` lanes, each unit's inputs taken and its fields loaded into a lane in unit order, the chunk integrated together, and each unit's fields stored back and its spike recorded in unit order again; every step of the rule written at one width, `integrate`'s two `i64` steps rewritten in `i32` with the same result and the inputs read in every lane and masked, so that the compiler's loop vectorizer takes it at the baseline; the spike's path in lanes; one reference to a record at a time, so phase 1's `unsafe` stands as ADR-0100 wrote it; development readings that predict no gain at (a), written before the first timed run; brief 045

## Context and Problem Statement

[ADR-0103](0103-the-working-layout.md) took [ADR-0099](0099-the-engines-speed.md)'s second lever and left four things to the round: the lanes across a chunk of units (its option 1) or within one record (option 3), the lane width and the chunk's length, whether the spike's rare path runs in lanes, and how the saturating steps are written for the vectorizer at the baseline. It fixed the rest: the membrane's rule in lanes beside `integrate`, which does not change; the lanes scratch within phase 1, so that the 64-byte record stays the only home of a unit's state (quality goal 2, [ADR-0001](0001-64-byte-pod-records.md)); safe Rust for the compiler's vectorizer, with no intrinsic, no target feature and no dependency; every lane held to `integrate` by a property test over the lattice; the gain on [ADR-0101](0101-the-sweep-measured-again.md)'s workload against ADR-0099's bounds, the mutation gate read before the first timed run.

The tree, read on 2026-09-26 at `f3083d7`:

- **The rule** (`DendriticSuperNeuron::integrate`, `crates/cortex-core/src/dynamics/membrane.rs`) reads and writes the four potentials at `[24..40)`, the two countdowns at `[40..44)` and the burst bit of `flags`, and on a spike the stamp at `[44..48)`. It has three branches of state (a running plateau, a threshold above its base, the refractory window), two of sign in each `leak`, one of the burst's coupling, and the spike's. Two of its steps are taken in `i64`: `add`, which widens an `i32` input and clamps, and the coupling, which shifts a difference taken in `i64`. Compiled for the baseline it is 168 instructions with 17 branches.
- **The turn** (`Worker::turn`, `runtime/cortex-runtime/src/executor.rs`, since [ADR-0102](0102-the-sweep-timed-with-no-census.md)) serves a word of the worker's region at a time, its set bits from the lowest. Nothing in a turn reads another unit (ADR-0100): a turn reads its own record, its own mailbox, the tick's gain and the clock.
- **The baseline.** The workspace sets no `target-cpu` and no `rustflags`: SSE2 on x86-64 and NEON on AArch64, four `i32` lanes. SSE2 has no variable shift per lane (`psrad` and `psrld` shift every lane by one count), no 32-bit `min`, `max` or `abs` (SSE4.1 and SSSE3), no 32-bit saturating add, no arithmetic right shift of a 64-bit lane (AVX-512) and no masked load (AVX).

## Decision Drivers

- **Every lane is `integrate`** for every unit and input, on both architectures (ADR-0099, T-1).
- **Quality goal 2 and ADR-0001**: the lanes are scratch within phase 1; no state outside the record across a barrier.
- **The vectorizer at the baseline**: a step the baseline has no vector form for makes the vectorizer's cost model decline the loop, or scalarises it.
- **Phase 1's `unsafe`** keeps ADR-0100's invariant, or restates it.
- **ADR-0078**: the gain is read, not argued, and the reading that chose the design is written down before the criterion's run.

## Considered Options

1. **Lanes across a chunk of units** (ADR-0103's option 1): unit *k* of the chunk in lane *k* of every field, one vector operation doing one step for four units.
2. **Lanes within one record** (ADR-0103's option 3): the four contiguous potentials of one unit as one 128-bit vector.

For the chunk: four units (one vector), eight (two), sixteen (four). For the layout of the lanes: one array per field (structure of arrays), or each unit's sixteen bytes of potentials copied whole into a lane array of rows, the compiler transposing them.

## Decision Outcome

**Option 1: a chunk of eight units, one array per field, in `cortex-core`'s `MembraneLanes`.**

- **Why not within a record.** The four lanes would take four different steps: the three leaks by shifts of 11, 9 and 10 and the threshold's decay by 12 toward its base, where SSE2 shifts every lane by one count; and the soma's step reads the basal and apical potentials the same tick has just moved, so three of the four lanes feed the fourth in sequence. What is left to run in parallel is the leak of three potentials, against the shuffles and the per-lane shift emulation it would cost.
- **The lane form.** `MembraneLanes` (`crates/cortex-core/src/dynamics/lanes.rs`) holds `LANES = 8` lanes of each integrated field and input, every one widened to `i32` so that every step runs at one width: the four potentials, the plateau's and the refractory countdowns, the burst bit, the basal and apical inputs, and the spike. `load(lane, unit, basal, apical)` takes a unit's fields and inputs into a lane; `integrate()` runs one tick in every lane; `store(lane, unit, now)` writes a lane back, stamps the tick if it fired, and returns what `integrate` returns. `integrate` itself is not edited. No `unsafe`, no allocation, no dependency; `#![no_std]` as the crate is.
- **Every step at one width, with the same result.** Proved in the module's documentation and held by the property test:
  - `leak(v, s)` takes the magnitude as `u32` (so `i32::MIN` has one), its fraction `m >> s` (below $2^{31}$ for `s ≥ 1`), the step `max(m >> s, [m ≠ 0])`, which is `integrate`'s `max(1).min(|v|)` because the fraction never exceeds the magnitude, and applies it through the sign mask `v >> 31`. The move is at most `|v|`, so it is exact in `i32`.
  - `add(v, d)` for an `i32` input is `v.saturating_add(d)`: two `i32` values never overflow `i64`, and the clamp is the saturation.
  - The coupling `((x as i64) − (y as i64)) >> k` is `(x >> k) − (y >> k) + (((x & (2^k − 1)) − (y & (2^k − 1))) >> k)`: the low bits' difference lies in $(−2^k, 2^k)$, so its floor over $2^k$ is `−1` or `0`, and every term, and the two couplings' sum, is within `i32` for `k ≥ 2`.
  - Every branch is a select, and the burst's two apical couplings are both computed and one selected.
  - **The inputs are read in every lane and masked**, `input & −[window open]`. Read only where the window is open, they are a conditional load, which the baseline has no vector form for: written that way, the loop vectorizer's cost model declined the loop ("the cost-model indicates that vectorization is not beneficial") and the rule compiled to scalar code with `cmov`s. Masked, it reports "vectorized loop (vectorization width: 4, interleaved count: 1)": a loop of two iterations, each 231 instructions, all SSE2 but the loop's own counter and branch.
- **The spike's path runs in lanes**: the reset, the threshold's step, the plateau and the two windows are selects on the spike's mask. A fall-back to the scalar rule would need each lane's state before the tick and a branch per chunk; the selects are a few instructions a vector.
- **The executor.** Phase 1 serves a word's set bits in chunks of `LANES`, in unit order. For each unit of a chunk, `take_inputs` drains its mail, sorts, sums and scales the batch, stamps a synaptic input, and loads the unit into its lane, exactly as the turn's first half did. Then `MembraneLanes::integrate` runs once. Then, for each unit in the same order, `finish_turn` stores the lane back and, on a spike, steps the short-term factors, counts a descendant, and records the spike in the spiked list, the trace and the train's slot, then keeps the unit's bit if it is not at rest and records its gate byte, exactly as the turn's second half did. What moves is only when a chunk's later units' mail is drained against its earlier units' spike records, and neither reads the other: the pools take the chunk's nodes back in the same order, the spiked list, the traces and the train's slots are written in unit order, and the counts are sums. `Worker` holds one `MembraneLanes`, allocated with it.
- **Phase 1's `unsafe` stands as ADR-0100 wrote it.** Each of `take_inputs` and `finish_turn` takes the unit's `&mut` and drops it when it returns: a worker still references only its own range's units, one at a time. A chunk holds copies of its units' fields in the lanes, not references. The `SAFETY` comment says so.
- **The chunk's length.** Eight, two vectors. The development readings below read four and sixteen beside it; eight was the fastest, and a word of 64 units is eight chunks.
- **The property test** (`lanes.rs`'s `prop` module, over `testkit/prop.rs`), three tests, each counting and asserting that it reached every branch the brief names — a running window with an input, a plateau that ends, the burst's coupling, a threshold above its base, one at or below zero, a spike, a plateau starting, a potential at either saturating end:
  - every lattice pair of inputs dealt across the lanes over 64 rounds, on units anywhere in the record's domain (every potential edge-biased, both countdowns at the windows' lengths and their neighbours, `u16`'s ends or any value, every bit of `flags`);
  - a seeded walk of $2^{17}$ ticks of eight lanes, a million lane-ticks, from armed units under edge-biased inputs and a working-range drive, a lane restarting anywhere in the domain one tick in 64;
  - the two comparisons a walk meets at equality only by chance: a soma exactly at its threshold (fires) and one LSB short (does not), and an apical potential exactly at the plateau's threshold on a spike (starts one) and one LSB below (does not).
  Each lane is compared with `integrate` on a copy of the same record: every plain field of the record, and the return value.
- **The acceptance** is ADR-0099's, unchanged.
- **The measure**, fixed here before the first timed run, is ADR-0102's on ADR-0101's workload with ADR-0099's bounds:
  - *Builds:* the base is `main` at the round's start, `f3083d7`, the sweep kept; the change is the round's commit of the code, the code the mutation gate read in CI. Each is exported with `git archive` into a directory of its own and built with `cargo test -p cortex-runtime --release --locked --test active --no-run` into a target directory of its own.
  - *Part 1, the criterion:* the four `…timed_with_no_census_exhaustive` tests on two workers, (a) to (d), each run alone by its binary (`<binary> --ignored --exact <test> --nocapture --test-threads=1`), the reading the `ns_per_tick` of its `DUMP network` line. Five pairs, the base first in pairs 1, 3 and 5. Kept if the medians' ratio change/base is at most **0.80** at (a) and (d) and at most **1.10** at (b) and (c).
  - *Part 2, beside it and not a criterion:* the two `…on_one_worker_exhaustive` tests, (a) and (c), in five pairs the same way.
  - *The machine:* ADR-0102's idle check (six consecutive five-second samples of the processor total below 15 per cent) before part 1, a sampler of the load every five seconds through both parts, and ADR-0102's rule: a part is disturbed when processes outside the round's two binaries hold more than four logical processors in three consecutive samples, judged from the sampler's log before any reading is looked at. A disturbed part is discarded unread and run again in a new session; a part that has been read is never replaced.
  - Every run of both builds is held to ADR-0097's tables by the test itself. The figures are a developer machine's, recorded in `docs/benchmarks/results/` as `admissible: no`. None of this moves after a timed run, and there is no tuning of the lanes after it.
- **If not kept**, the pull request carries the readings, this ADR and the lane form's history, not the change: the code is reverted after the readings, stays in the history, and there is no further attempt at this lever in this form.

### Development readings, before this ADR (not the criterion)

The design was chosen on readings taken on the developer machine before this ADR was written, and they are recorded here because they predict the criterion's outcome. They are not admissible, and not a criterion; none is pinned.

- **Where run (a) sits.** A snapshot of the reference network's 1 024 units after run (a)'s lead-in (the first nine rows of its layout, one worker): every unit in the working range, soma about 0.3 to 0.5 and basal about 0.9 of the threshold, apical zero, threshold at its base, no window running. Every branch of `integrate` goes the same way on every unit: on this workload the scalar rule's branches are predictable.
- **On that snapshot** (a bench in the session's scratch directory, 1 024 units, 64 ticks from the snapshot, 2 000 rounds; nanoseconds a unit): `integrate` over the independent units 4.8 to 5.1; `MembraneLanes::integrate` alone 2.7 to 2.8; the lanes with `load` and `store` 5.7 to 6.1 at a chunk of eight, 7.7 at four and 6.2 at sixteen; the second layout (each unit's sixteen bytes of potentials copied whole into a row, the compiler transposing: fully unrolled, vectorized) 6.0, and 6.4 inlined into the chunk's loop.
- **In the executor**, this ADR's design at run (a) on one worker, two readings each: 14 573 and 13 586 ns a tick against the base's 11 799 and 11 576, every row of ADR-0097's table held.
- **The ceiling.** The vector rule saves at most about 2.2 ns a unit on this snapshot (4.9 against 2.75), against a turn of about 11.2 ns on one worker: even if loading and storing a chunk cost nothing, the turn would be about 0.8 of the base's at (a). Moving eight units' fields into the lanes and back costs about 3 ns a unit, more than the vector rule saves, because the scalar rule's branches are predictable here and SSE2's four lanes take about 58 instructions a unit for the rule.

**Prediction, written before the first timed run:** the change is not kept. The ratio at (a) and at (d) is above 0.80, and likely above 1.0. The criterion is read anyway: it is ADR-0099's verdict, and the development readings are not.

### Consequences

- Good: the lane form exists, is held to `integrate` on every lane over the lattice, and is vectorized at the baseline with no intrinsic, no target feature and no dependency; the i32 rewrite of the two `i64` steps is proved and tested, and is what any later vector form of the rule starts from.
- Good: the readings say where the lever meets the baseline: not in the rule's `i64` steps, which the rewrite removes, but in moving the fields through the lanes, against a scalar rule whose branches the reference network makes predictable.
- Bad: if the prediction holds, the lever in this form is spent, and what the next decision can weigh is a wider baseline (a decision about every build, left to an ADR of its own by ADR-0103), the lookahead, or closing the speed line.
- Neutral: two forms of one rule, held to each other by the property test, while the lanes are in the tree.

## Alternatives considered and why rejected

- **Lanes within one record** (ADR-0103's option 3): above — the four lanes take four different shifts, and the soma's step waits on the other two potentials.
- **A chunk of four or sixteen**: read beside eight on the snapshot (7.7 and 6.2 ns a unit against 6.1).
- **Rows copied whole, transposed by the compiler**: vectorized, and no faster (6.0, and 6.4 inlined).
- **The inputs read only where the window is open**: the vectorizer declines the loop at the baseline.
- **The spike's path falling back to the scalar rule**: it needs the state before the tick and a branch per chunk.

## Confirmation

`crates/cortex-core/src/dynamics/lanes.rs` holds the lane form and its property test; `runtime/cortex-runtime/src/executor.rs`'s `phase_turns`, `take_inputs` and `finish_turn` use it. The mutation gate's reading, the timed session's readings, the one-worker readings, the vectorizer's reading from the generated code, the per-turn breakdown after the change and the verdict are completed by the round.
