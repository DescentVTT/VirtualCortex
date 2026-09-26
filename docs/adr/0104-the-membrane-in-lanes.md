---
status: accepted
date: 2026-09-26
depends-on: ADR-0103
decision-makers: VirtualCortex maintainers
---

# ADR-0104: The membrane's rule in lanes, built and not kept — ADR-0103's first option, a chunk of eight scheduled units in two vectors of four `i32` lanes with every step at one width and every lane held to `integrate` over the lattice, was vectorized by the compiler at the baseline and read 1.209, 1.132, 1.211 and 1.122 of the base's wall time per tick at ADR-0097's runs (a) to (d), against ADR-0099's bounds of 0.80 and 1.10, as predicted before the run; moving the fields through the lanes cost more than the vector rule saved against a scalar rule whose branches the reference network makes predictable; the code stays in the history; finding F-51, the integration about two thirds of a turn and not all of it, resolved by a bench case of the rule's throughput; the next decision named and not taken

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

**Option 1 — a chunk of eight units, one array per field, `cortex-core`'s `MembraneLanes` — was built and measured, and is not kept**: no bound of ADR-0099 is met (1.209 at (a) against 0.80). The code is `b50ed8c` and `dc9c5ad` and is reverted in the same pull request (`413590f`); what the pull request keeps is this record, the readings in `docs/benchmarks/results/2026-09-26-dancr-win11-brief-045.md` and a bench case of the rule's throughput.

### The design built (`b50ed8c`, `dc9c5ad`)

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

### The acceptance and the measure, fixed before the first timed run

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

- **Where run (a) sits.** A snapshot of the reference network's 1 024 units after run (a)'s lead-in and first window (the first ten rows of its layout, one worker): every unit in the working range, soma about 0.3 to 0.5 and basal about 0.9 of the threshold, apical zero, threshold at its base, no window running. Every branch of `integrate` goes the same way on every unit: on this workload the scalar rule's branches are predictable.
- **On that snapshot** (a bench in the session's scratch directory, 1 024 units, 64 ticks from the snapshot, 2 000 rounds; nanoseconds a unit): `integrate` over the independent units 4.8 to 5.1; `MembraneLanes::integrate` alone 2.7 to 2.8; the lanes with `load` and `store` 5.7 to 6.1 at a chunk of eight, 7.7 at four and 6.2 at sixteen; the second layout (each unit's sixteen bytes of potentials copied whole into a row, the compiler transposing: fully unrolled, vectorized) 6.0, and 6.4 inlined into the chunk's loop.
- **In the executor**, this ADR's design at run (a) on one worker, two readings each: 14 573 and 13 586 ns a tick against the base's 11 799 and 11 576, every row of ADR-0097's table held.
- **The ceiling.** The vector rule saves at most about 2.2 ns a unit on this snapshot (4.9 against 2.75), against a turn of about 11.2 ns on one worker: even if loading and storing a chunk cost nothing, the turn would be about 0.8 of the base's at (a). Moving eight units' fields into the lanes and back costs about 3 ns a unit, more than the vector rule saves, because the scalar rule's branches are predictable here and SSE2's four lanes take about 58 instructions a unit for the rule.

**Prediction, written before the first timed run:** the change is not kept. The ratio at (a) and at (d) is above 0.80, and likely above 1.0. The criterion is read anyway: it is ADR-0099's verdict, and the development readings are not.

### The mutation gate, before the first timed run

Run `36224029050` on the pull request, at the branch's `f57349d` (`dc9c5ad` on `main`, the same tree), the code the session times: 63 mutants in the lines the pull request changes (the lane form, its registration and the executor's `phase_turns`, `take_inputs` and `finish_turn`), **63 caught**, none unviable, no timeout, none missed, in 11 minutes. No survivor, so the code is not changed before the timing. On the developer machine the same gate over `lanes.rs` alone read 28 caught and none missed; its other 29 did not link (`LNK1104`, the machine's), which is why the reading is CI's.

### The readings

The session of 2026-09-26, 06:45:57Z to 06:55:11Z, on the developer machine (`docs/benchmarks/results/2026-09-26-dancr-win11-brief-045.md`, not admissible). The idle check passed on six samples at 5.1 to 8.4 per cent. The sampler's log was judged by ADR-0102's rule before any reading was looked at: other processes held at most 1.26 logical processors in part 1 and 0.69 in part 2, against the rule's four, so **neither part was disturbed**. Every run of both builds held ADR-0097's pinned rows, 110 runs in all.

Part 1, the criterion: two workers, five alternating pairs, the median of five readings a build, change/base:

| Run | Base (ns a tick) | Change | Change/base | Bound |
| :--- | ---: | ---: | ---: | :--- |
| (a) 1 024 units, ADR-0044's drive | 5 003 | 6 047 | **1.209** | 0.80, not met |
| (b) sixteen times sparser | 3 594 | 4 070 | **1.132** | 1.10, not met |
| (c) 256 times sparser | 722 | 874 | **1.211** | 1.10, not met |
| (d) 4 096 units, ADR-0044's drive | 26 628 | 29 877 | **1.122** | 0.80, not met |

The controls, beside them: 1.083, 1.149 and 1.244 at (a) to (c). Part 2, one worker, beside it and not a criterion: 1.174 at (a) (8 692 against 10 205 ns a tick) and 1.364 at (c) (711 against 970), the controls 1.141 and 1.370. The prediction, written first, held: slower at every run, above 1.0 at (a) and (d).

**The vectorizer** (`cargo rustc -p cortex-core --release --locked --lib -- --emit asm -C remark=loop-vectorize` at `b50ed8c`, x86-64; AArch64 not read): `MembraneLanes::integrate` is "vectorized loop (vectorization width: 4, interleaved count: 1)", two iterations of 231 instructions, every one SSE2 but the loop's counter and branch, against `integrate`'s 168 instructions and 17 branches. The compiler did what the design asked of it. It is the rest that did not pay.

### The per-turn breakdown, and F-51

From the same session and the bench four minutes after it (`2ef7ae0`, not disturbed; figures of one session only, since this machine's frequency is not fixed and the whole bench read 20 to 52 per cent below brief 044's that morning):

- **`neuron/integrate` reads one unit's chain.** The case steps one unit on its own state, so each call waits on the one before: 6.86 ns. The sweep integrates units that do not wait on one another. `neuron/integrate_x1024`, added by this round, integrates 1 024 armed units each on its own record, from where run (a) leaves them after its lead-in: **5.67 ns a unit**.
- **On one worker at (a) the base's turn is 8.49 ns.** The integration is about two thirds of it. The rest, about **2.8 ns**, is the turn's other steps: the mail check, the batch's sort, sum and scaling, the rest check and the gate byte, and the schedule's bit.
- **The lanes' turn was 9.97 ns**, 1.48 ns more. Taking a chunk's fields into the lanes and back, and serving the turn in two passes, cost more than the vector rule saved. On the snapshot the vector rule alone read 2.75 ns a unit against the scalar rule's 4.9, a ceiling of about 2 ns a turn even if moving the fields had cost nothing: short of the 0.80 bound at (a) by itself.
- **At (c)** on two workers, an idle tick's synchronisation (`executor/idle_tick/2`, 206 ns) is about two sevenths of the base's tick (722 ns). A message costs `mailbox/push_drain_x16` / 16 = 4.45 ns.

**Finding F-51** (whitepaper §11). The chain was read as the turn's integration. ADR-0100's and ADR-0102's breakdowns set the one-worker turn at (a) beside `neuron/integrate` from other sessions and read "on the sweep the integration is the turn". ADR-0103 and brief 045 took that as the working layout's measured need. Read in one session, the integration is about two thirds of the turn and the rest about a third. The lever was still aimed at the larger part, but the need was a third smaller than written, and a lever on the rest of the turn was never named. **Resolved here:** `neuron/integrate_x1024` reads the rule's throughput beside the chain, and whitepaper §11.1 says what a turn is made of. ADR-0100, ADR-0102 and ADR-0103 are records and are not edited.

### The verdict and what the pull request carries

- **Not kept.** No bound is met. Under ADR-0099 no constant moves and there is no further attempt at this lever in this form.
- **Reverted** in `413590f`: the lane form (`lanes.rs`, its registration and its property tests) and its use in phase 1. Against `f3083d7` the tree's code under `crates/` and `runtime/` is unchanged. The code stays in the history at `b50ed8c` and `dc9c5ad`: every lane held to `integrate` over the lattice, the loop vectorized at width 4, the gate 63 caught of 63, every pinned row held.
- **Kept:** this record, the readings, and the bench case `neuron/integrate_x1024` (`2ef7ae0`, `docs/benchmarks/README.md`), which reads the rule's throughput beside the chain for whatever lever comes next. Adding it is within the brief's empowerment ("whether a bench case … is added").
- **The mutation gate** on the pull request's final diff has no source line to mutate: the lanes and their revert net to nothing, and `benches/**` is excluded (`.cargo/mutants.toml`). The lanes' own lines were gated before the timing (above).
- **The weekly dispatch's scope** ([ADR-0075](0075-the-dispatch-scope-follows-the-diff.md)): `scope=exhaustive`. Against `main` the diff changes no file under a `src/` directory (the lanes and their revert net to nothing, and the bench case is in `benches/cortex-bench/benches/`), deletes no test and takes none out of the swept suite, and leaves `.cargo/mutants.toml` and the `mutants-weekly` job as they were.

### The next decision (named, not taken)

ADR-0103: after this lever, "the round's ADR names the next decision and does not take it". The readings above say what each candidate would meet:

1. **The lookahead**, ADR-0099's third lever. At (c) the synchronisation of a tick is about two sevenths of it on two workers. At (a) and (d) there is little to take, since the barriers are a few per cent of a tick.
2. **The rest of the turn** (F-51), a lever ADR-0099 did not name. About a third of a turn at (a) on one worker is not the integration: the mail check, the empty batch's sort, sum and scaling, the rest check and the gate byte. It changes no rule.
3. **A persistent working copy of the integrated fields**, which removes what cost the lanes here, the transposition every tick. It amends quality goal 2 and ADR-0001 (ADR-0103), and the snapshot bounds its gain: the vector rule saves about 2 ns of a turn of about 8.5 at (a).
4. **A wider baseline**: target features beyond SSE2 and NEON, eight lanes and a vector `min`, `max` and `abs`. It is a decision about every build and every CI job (ADR-0103), and alone it leaves the transposition as it is.
5. **The speed line closed** and the learning line resumed from H-18's named next decision ([ADR-0098](0098-the-integration-model.md)).

### Consequences

- Good: the round read what it was built to read, and the prediction written before the run held. The lever is spent in this form, and the reading says why: the baseline's four lanes take the rule, but a scalar rule whose branches the reference network makes predictable leaves them less to save than moving the fields costs.
- Good: the rewrite of `integrate`'s two `i64` steps in `i32` is proved, tested over the lattice and in the history at `b50ed8c`, for any later vector form of the rule.
- Good: F-51 corrects the need the speed line was steered by, and the bench now reads the rule's throughput beside the chain.
- Bad: the engine is not faster.
- Neutral: two forms of one rule are not kept in step, since the lanes are not in the tree.

## Alternatives considered and why rejected

- **Lanes within one record** (ADR-0103's option 3): above — the four lanes take four different shifts, and the soma's step waits on the other two potentials.
- **A chunk of four or sixteen**: read beside eight on the snapshot (7.7 and 6.2 ns a unit against 6.1).
- **Rows copied whole, transposed by the compiler**: vectorized, and no faster (6.0, and 6.4 inlined).
- **The inputs read only where the window is open**: the vectorizer declines the loop at the baseline.
- **The spike's path falling back to the scalar rule**: it needs the state before the tick and a branch per chunk.

## Confirmation

The pull request (#131) was merged by rebase. Its commits are cited as `main` holds them; on the round's branch the same trees were `f026707` (`e2879c7`), `2aca7d3` (`b50ed8c`), `f57349d` (`dc9c5ad`), `89d414a` (`9596276`), `49fabb0` (`413590f`), `8184d32` (`2ef7ae0`), `c665f30` (`ac46cb6`) and `a55dc1e` (`78171e9`), and each run names the branch's commit it ran on.

- `b50ed8c`: the lane form and its three property tests; `dc9c5ad`: its use in phase 1; every test of the tree passed on it, the differential test on one, two and four workers and `tests/no_alloc.rs` among them, and ADR-0097's rows held in every timed run.
- Run `36224029050`: the mutation gate on the lanes' lines, 63 caught of 63, before the first timed run.
- `413590f`: the revert; against `f3083d7` the code under `crates/` and `runtime/` is unchanged.
- `2ef7ae0`: `neuron/integrate_x1024`.
- `docs/benchmarks/results/2026-09-26-dancr-win11-brief-045.md`: every reading above, the session's load log judged, the scripts, the snapshot and the scratch bench.
- The evidence: the weekly dispatched on this round's branch at the branch's `c665f30` (`ac46cb6` on `main`, the same tree), run `36225957998`, with `scope=exhaustive` (the clause above), green in every job it runs. The four whole-domain shards took 31m40s, 29m00s, 22m25s and 15m47s, their tests' wall 1 843, 1 693, 1 303 and 912 s: 26, 24, 18 and 13 per cent of the 7 200-second bound. All 63 tests passed, so every pinned number of those tests is reproduced on the tree as it merges, whose engine is `f3083d7`'s. Beside ADR-0102's run `36209381108` on the same engine, the secondary reading: 27m20s, 41m37s, 34m59s and 30m14s. The 34 tests that took a minute or more there took 0.66 of their seconds here at the median (0.15 to 1.07), the runners' variance on identical code (F-45's kind). The cost table is regenerated from this run: 63 lines, 10 779 seconds, against 15 263. No sweep was dispatched, so there is no survivor to disposition. The lanes' code passed every job of the pull request's gate before its revert (run `36224692199` at the branch's `89d414a`, `9596276` on `main`: check, test, fmt and clippy; the minimum supported version; AArch64; the mutation gate), and the final head passed it again (run `36225950793`).
