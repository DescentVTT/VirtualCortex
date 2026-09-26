---
status: archived
date: 2026-09-26
---

> **Executed 2026-09-26 in pull request #131.** Writes ADR-0104 (the membrane's rule in lanes, built and not kept);
> opens and resolves finding F-51. The design was written in ADR-0104 before the code (`e2879c7`), chosen on development
> readings it records: ADR-0103's first option, a chunk of eight scheduled units in two vectors of four `i32` lanes,
> every step at one width (`integrate`'s two `i64` steps rewritten in `i32` with the same result, the inputs read in
> every lane and masked), the spike's path in lanes, one reference to a record at a time. The lane form (`b50ed8c`) was
> held to `integrate` on every lane by three property tests over the lattice, its use in phase 1 (`dc9c5ad`) passed every
> test of the tree, the compiler vectorized the rule at width 4, and the mutation gate caught 63 of 63 mutants on its
> lines in CI before any run was timed. Timed by ADR-0099's protocol on ADR-0101's workload on an idle developer
> machine (not admissible), in a session the disturbance rule found undisturbed, five alternating pairs, medians,
> change/base:
>
> - (a) 1.209 and (d) 1.122, against their bound of 0.80;
> - (b) 1.132 and (c) 1.211, against their bound of 1.10.
>
> **Not kept**, as ADR-0104 predicted before the run: moving a chunk's fields through the lanes cost more than the vector
> rule saved, against a scalar rule whose branches the reference network makes predictable. The code is reverted
> (`413590f`) and stays in the history. Beside the criterion, one worker read 1.174 and 1.364 at (a) and (c). F-51: the
> bench's `neuron/integrate` reads one unit's chain, which ADR-0100 and ADR-0102 set beside the sweep's turn as "the
> integration is the turn"; the new case `neuron/integrate_x1024` (`2ef7ae0`) reads the rule over independent units at
> 5.67 ns a unit against a one-worker turn of 8.49 ns, about two thirds. The weekly dispatch at `scope=exhaustive` (run
> 36225957998) was green: all 63 whole-domain tests passed, and the cost table is regenerated from it. Image format 16; no rule of the
> engine changed; whitepaper 4.56.0. Every deliverable is dispositioned below. Relative links gained one `../` so that
> they resolve from `archive/`; no other word, claim or figure changed.
> *The body below describes the tree before execution and is not maintained.*

# Brief 045: The working layout — the membrane's rule in lanes, for the compiler's vectorizer, beside the scalar rule and held to it on every lane; used by the sweep's turn phase within a phase, the record staying the only home of a unit's state; timed on ADR-0101's workload against ADR-0099's bounds

## Mission

**This brief makes the engine faster, or reads that it does not, and changes no rule.**
[ADR-0102](../../docs/adr/0102-the-sweep-timed-with-no-census.md) kept the sweep without the gate. On one worker the
sweep's turn at [ADR-0097](../../docs/adr/0097-the-active-set-measured.md)'s run (a) is about 11.4 ns, and the bench's
`neuron/integrate` reads 10.96 ns (a developer machine's, not admissible): **the integration is the turn.**
[ADR-0103](../../docs/adr/0103-the-working-layout.md) takes [ADR-0099](../../docs/adr/0099-the-engines-speed.md)'s second
lever under one constraint and one technique:
- the constraint: the 64-byte record stays the only home of a unit's state between turn phases, so quality goal 2 holds
  unchanged;
- the technique: the rule is written in lanes in safe Rust for the compiler's vectorizer, as
  [ADR-0039](../../docs/adr/0039-hypervector-body.md)'s hypervector is.

When the round is done, the tree holds either the lanes, kept, or the reading that they are not worth keeping. With the
lanes:
- `cortex-core` holds a lane form of the membrane's rule beside `integrate`, held to it on every lane by a property
  test over the lattice;
- the executor's phase 1 integrates a chunk of a worker's scheduled units in lanes, or `integrate`'s arithmetic runs in
  lanes within one record, whichever the round's design chose on its readings.

In both cases the tree holds the gain measured against ADR-0099's bounds on ADR-0101's workload, written before the
first timed run, and a per-turn breakdown after the change. **Every reading of behaviour holds bit for bit.**

---

## Standing directives

- **Latest ≠ Newest** (whitepaper §2.1, `CLAUDE.md` principle 4). The vector code is **safe Rust over fixed-size arrays
  for the compiler's vectorizer**, ADR-0039's footing. The round adopts none of the following:
  - no `core::arch` intrinsic (they are `unsafe`, and `cortex-core` forbids `unsafe`);
  - no `core::simd` (nightly);
  - no vector crate, and no dependency of any kind;
  - no `target-cpu` or `rustflags` (the baseline is SSE2 on x86-64 and NEON on AArch64, four `i32` lanes);
  - no version bump of a tool.
- **`integrate` does not change.** The scalar rule is the specification. The lane form is held to it on every lane, for
  every unit and input, including:
  - the refractory window and a plateau that ends;
  - the burst's coupling;
  - the threshold above its base and at or below zero;
  - a spike;
  - both saturating ends.
- **Quality goal 2 holds.** A lane array is scratch within phase 1 ([ADR-0001](../../docs/adr/0001-64-byte-pod-records.md)).
  No persistent working copy, and no state kept outside the record across a barrier.
- **The measure is ADR-0101's and ADR-0102's, with ADR-0099's bounds unchanged.**
  - Workload: ADR-0097's four configurations with no census in the timed ticks (`tests/active.rs`, the six
    `…timed_with_no_census…` tests).
  - Builds: the base is `main` at the round's start, the sweep kept; the change is the base with the lanes. Each build
    is exported with `git archive` and built in a target directory of its own.
  - Pairs: five alternating pairs, the base first in pairs 1, 3 and 5.
  - Idle machine: ADR-0102's idle check and its disturbance rule, judged from the load log before any reading is looked
    at. A disturbed session is discarded unread.
  - Criterion: kept at a median wall time per tick of at most **0.80** of the base's at (a) and (d) and at most
    **1.10** at (b) and (c).
  - Nothing about it moves after a timed run, and **the code timed is the code the mutation gate read**, committed
    before the first timed run.
- **No reading of behaviour moves** (ADR-0099):
  - every potential, window, stamp, threshold, short-term factor, weight and trace;
  - the spike train and count;
  - the messages delivered and the turns served, summed over the workers;
  - every pinned number of every round;
  - the AArch64 job;
  - the differential test at one, two and four workers.

  A pin that also holds the scheduler's own bytes is restated only under ADR-0099's masked check. Any other pin that
  moves stops the round: it is a finding, the change is not merged, and there is no re-pin and no second attempt.
- No record field and no image format moves (16). `unsafe` stays in the runtime under ADR-0100's invariant, restated if
  a chunk holds several units' references at once ([ADR-0023](../../docs/adr/0023-executor.md)). No float anywhere.
  Saturating or named-wrapping arithmetic on state ([ADR-0029](../../docs/adr/0029-structural-enforcement.md)). Every loop
  ends by construction ([ADR-0062](../../docs/adr/0062-the-first-complete-sweeps-list.md)). Nothing allocates after
  start-up.
- A heavy run is an `#[ignore]`d `exhaustive` test; the runtime's gate grows by at most one test
  ([ADR-0061](../../docs/adr/0061-the-learning-runs-leave-the-gate.md)). A new rule carries a test over the lattice of
  `testkit/prop.rs`, and the mutation gate on the changed lines must pass
  ([ADR-0030](../../docs/adr/0030-verification-governance.md)).
- **The engine is read before a description of it is trusted**, this brief's included.
- Conventional Commits with a real body; never commit on `main`; the required checks keep their names.

## Context

Re-derived on 2026-09-26 against `main` at `d90fa8a`, after ADR-0102 merged. Line numbers move; the symbols and the
quoted sentences are what to re-derive.

1. **The rule** (`crates/cortex-core/src/dynamics/membrane.rs`, `DendriticSuperNeuron::integrate`). For one tick it:
   - counts a running plateau down and clears the burst flag at zero;
   - decays a threshold above `THRESHOLD_BASE` toward it by `excess >> THRESHOLD_DECAY_SHIFT`, at least one;
   - counts the refractory window down and drops the inputs inside it;
   - leaks the basal and apical compartments by `leak(v, shift)`, which moves toward zero by `|v| >> shift`, at least
     one LSB and never past zero, and adds the inputs with `add`, which widens to `i64` and clamps;
   - couples the soma by `(v_basal − v_soma) >> COUPLING_SHIFT` and `(v_apical − v_soma) >> apical_shift`, where the
     apical shift is `PLATEAU_COUPLING_SHIFT` while bursting, and leaks it by `SOMA_LEAK_SHIFT`;
   - on a spike, when not refractory, the threshold is above zero and the soma is at or above it: stamps
     `last_soma_spike_tick`, resets the soma to `V_RESET`, steps the threshold up by `THRESHOLD_STEP`, and starts a
     plateau when `v_apical ≥ BAC_APICAL_THRESHOLD`, with the burst's refractory window, or else the ordinary one.

   The four potentials are contiguous at `[24..40)` of the 64-byte record, and the countdowns at `[40..44)`.
   `membrane.rs`'s tests already `include!` `testkit/prop.rs`.
2. **The turn** (`runtime/cortex-runtime/src/executor.rs`, `Worker::phase_turns`, `Worker::turn`, after ADR-0102).
   Each word of the worker's region is served in order of its set bits. Per unit:
   - drain the mailbox if it is not empty;
   - sort, sum and `scaled` by the gain;
   - `note_synaptic_input` if a synapse's message came;
   - `integrate`;
   - on a spike, `step_stp`, the descendant count, the spiked list, the spike trace and the train's slot;
   - `at_rest` decides whether the bit stays, and `set_gate` records it.

   At run (a) about 8.6 messages reach 1 024 units a tick, so nearly every turn has no mail. About 0.0018 per cent of
   units fire a tick.
3. **The precedent** (whitepaper §5.2, the hypervector body): "a loop over the words with named operations and no
   intrinsic (every `core::arch` intrinsic is `unsafe`, which the crate forbids; `core::simd` is nightly; the compiler
   vectorises the loops for the target it builds for and the result is the same integer everywhere, which is what T-1
   holds)". The workspace sets no `target-cpu` and no `rustflags` (`.cargo/` holds only `mutants.toml`).
4. **The costs** (`docs/benchmarks/results/2026-09-26-dancr-win11-brief-044.md`, not admissible):
   - on one worker the sweep's turn at (a) is about 11.4 ns, against `neuron/integrate`'s 10.96 ns;
   - on two workers one worker's turn is 11, 10, 21 and 14 ns at (a) to (d), barriers included;
   - the base's readings were bimodal at (a), and the median read the lower mode.
5. **The pins.** The determinism pin (`tests/differential.rs`) with the scheduler's bytes masked equals the pin
   (ADR-0100); ADR-0097's census tables and the six timed tests' rows; the image CRCs of `tests/inhibition.rs`; the
   learning line's tables.
6. **The machine.** ADR-0100's first session was disturbed by another repository's mutation run holding every logical
   processor. ADR-0102's session recorded the other processes' load through it. Nothing on the machine is the round's to
   stop: if it is not idle, the round waits and says so.
7. **The weekly job** ([ADR-0092](../../docs/adr/0092-the-shards-dealt-by-cost.md),
   [ADR-0075](../../docs/adr/0075-the-dispatch-scope-follows-the-diff.md)): the scope follows the round's diff, and the
   cost table is regenerated from its dispatch.

## Deliverables

- [x] **The design, in a new ADR at the next free number (`ls docs/adr`), before the code.** It states:
  - whether the lanes run across a chunk of units (ADR-0103's option 1) or within one record (option 3), and why;
  - the lane width and the chunk's length;
  - how a chunk is gathered and written back within phase 1, and in what order its units' spikes are then recorded
    (the sweep's unit order);
  - whether the spike's rare path runs in lanes or falls back to the scalar rule;
  - how the saturating steps are written so that the compiler can vectorize them at the baseline;
  - phase 1's `unsafe` restated if a chunk holds several units' references at once.
  **Done** (ADR-0104, `e2879c7`, before the code): option 1, a chunk of eight units in two vectors of four `i32` lanes; the gather and write-back one unit at a time within phase 1 in unit order, the spikes recorded in the sweep's order; the spike's path in lanes; the saturating steps and the coupling's `i64` shift rewritten in `i32` with the same result, the inputs read in every lane and masked for the vectorizer; phase 1's `unsafe` unchanged, since a chunk holds copies and one reference at a time. Option 3 rejected: four different shifts where SSE2 shifts every lane by one count, and the soma's step waits on the other two potentials. Chunks of four and sixteen were read beside eight. The development readings that chose the design are recorded in the ADR.
- [x] **The lane form in `cortex-core`**, beside `integrate` and unchanged from it in behaviour. It is held to
  `integrate` on every lane by a property test over `testkit/prop.rs`'s lattice and its seeded walk, which covers every
  branch the Standing directives list. It adds no `unsafe`, no allocation and no dependency.
  **Done** (`b50ed8c`), then **reverted** (`413590f`) as the verdict requires; it stays in the history. `MembraneLanes` in `crates/cortex-core/src/dynamics/lanes.rs`, `integrate` not edited; three property tests over `testkit/prop.rs` held every lane, every plain field and the return value to `integrate` (every lattice pair on units anywhere in the domain; a seeded walk of a million lane-ticks; the soma exactly at its threshold and the apical potential exactly at the plateau's), each asserting that it reached every branch the Standing directives list. No `unsafe`, no allocation, no dependency.
- [x] **The executor's use of it**, if the design needs one. All of the tree's tests pass on it, the differential test
  at one, two and four workers included. `tests/no_alloc.rs` passes. **The mutation gate on the diff is read in CI on
  the round's pull request before the first timed run.** A survivor is met by a test, or by a change that moves neither
  behaviour nor the work, committed before the timing.
  **Done** (`dc9c5ad`), then **reverted** (`413590f`). Every test of the tree passed on it in debug and release, the differential test on one, two and four workers and `tests/no_alloc.rs` among them. The mutation gate was read in CI on the pull request before the first timed run: run `36224029050`, 63 mutants, 63 caught, none unviable, no timeout (ADR-0104, `9596276`). No survivor, so no change before the timing.
- [x] **The gain, by ADR-0101's workload and ADR-0099's protocol.**
  - The four runs on two workers, base and change, five alternating pairs, each run's median and the ratio
    change/base, the load log beside them, recorded in `docs/benchmarks/results/` with the machine, `admissible: no`.
  - **Kept** when (a) and (d) are ≤ 0.80 and (b) and (c) ≤ 1.10.
  - Beside it, and not a criterion: one worker at (a) and (c).
  - A reading of whether the compiler vectorized the lanes, from the generated code (for example `--emit asm` on the
    release build of the test binary): a diagnostic, not a criterion.
  **Done** (ADR-0104, `docs/benchmarks/results/2026-09-26-dancr-win11-brief-045.md`, `admissible: no`): two workers, (a) 1.209, (b) 1.132, (c) 1.211, (d) 1.122 — **not kept**. One worker: 1.174 at (a), 1.364 at (c). The load log beside them, no part disturbed. The vectorizer's reading from `--emit asm` on `cortex-core`'s release build at `b50ed8c` (x86-64; AArch64 not read): "vectorized loop (vectorization width: 4, interleaved count: 1)", two iterations of 231 instructions, all SSE2 but the loop's own.
- [x] **The per-turn breakdown after the change**: the one-worker turn at (a) beside `neuron/integrate`, and what now
  bounds a turn. This is what the next decision weighs.
  **Done** (ADR-0104, the results file, one session): on one worker at (a) the base's turn is 8.49 ns and the lanes' 9.97; `neuron/integrate`, one unit's chain, 6.86 ns; `neuron/integrate_x1024`, added (`2ef7ae0`), the rule over 1 024 independent units, 5.67 ns a unit, about two thirds of a turn; the rest of a turn about 2.8 ns (the mail check, the batch's sort, sum and scaling, the rest check, the gate byte). Finding F-51 records the chain read as the turn's integration, and is resolved.
- [x] **The verdict in the round's ADR.**
  - If kept: the lane form and its use land, and ADR-0103's next decision is named and not taken (the lookahead, or
    the speed line closed and the learning line resumed).
  - If not kept: the pull request carries the readings and the ADR, not the change. There is no further attempt at
    this lever in this form, and the next decision is named.
  **Done** (ADR-0104): not kept; the pull request carries the readings and the ADR, not the change; no further attempt at this lever in this form. The next decision is named and not taken: the lookahead, the rest of the turn, a persistent working copy, a wider baseline, or the speed line closed and the learning line resumed.
- [x] **The evidence.** A weekly dispatched on this round's branch at the scope ADR-0075 gives the diff, the clause
  stated in the ADR. It must be green in every job, every pinned number reproduced, and the sweep's survivors
  dispositioned. The shards' times are read beside ADR-0102's as a secondary reading, and the cost table is regenerated
  from that run.
  **Done** (ADR-0104): run `36225957998` at the branch's `c665f30` (`ac46cb6` on `main`, the same tree), `scope=exhaustive` (ADR-0075: against `main` no file under a `src/` directory changes — the lanes and their revert net to nothing and the bench case is under `benches/` — no test is deleted or taken out of the swept suite, and `.cargo/mutants.toml` and the `mutants-weekly` job are unchanged). Green in every job: the shards took 31m40s, 29m00s, 22m25s and 15m47s against ADR-0102's 27m20s, 41m37s, 34m59s and 30m14s on the same engine (the runners'), all 63 tests passed and every pinned number of them is reproduced. No sweep ran, so there is no survivor. The cost table is regenerated from the run: 63 lines, 10 779 s.
- [x] **The documents, in the same pull request.**
  - Always: whitepaper §11.1's question on the engine's speed, with this round's outcome; §9; the ADR index;
    `CHANGELOG.md`; `CLAUDE.md`; `docs/zh-TW`'s reader's guide as the result requires.
  - If kept: §5.2's `cortex-core` API and the membrane's paragraph, and §6.1's turn.
  - The whitepaper's version in both declarations, with its date
    ([ADR-0064](../../docs/adr/0064-the-documentation-gate-and-the-version.md)).
  **Done**: whitepaper §11.1's question on the engine's speed with this round's outcome, §9's row, F-51 in §11 with its directives, version 4.56.0 in both declarations (2026-09-26); the ADR index; `CHANGELOG.md`; `CLAUDE.md`; `docs/zh-TW`'s reader's guide; `docs/benchmarks/README.md`'s table. §5.2 and §6.1 are unchanged, since the lanes are not kept.
- [x] **The brief archived** as `briefs/README.md` says, every deliverable dispositioned.
  **Done**: this file.

## Not empowered

- **No rule of the dynamics changes**: `integrate` is not edited, and no reading of behaviour moves.
- No persistent working copy, and no state outside the record across a barrier. Quality goal 2 and ADR-0001 are not
  amended.
- No `core::arch`, no `core::simd`, no vector crate, no dependency, no `target-cpu`, no `rustflags`, no `unsafe` in
  `cortex-core`, no float.
- No record field and no image format moves. No pin is restated except under ADR-0099's masked check.
- No lookahead, no change to the sweep's ownership or schedule beyond what gathering a chunk needs, no longer tick, no
  rest-condition change.
- The bounds, the protocol, the workload's configurations and the order of runs do not move after a timed run. There
  is no second timed session to replace one that was read, and no tuning of the lanes after the first timed run.
- No change to `.github/workflows/ci.yml`, `scripts/exhaustive-shard.sh` or `scripts/exhaustive-costs.mjs`; the cost
  table changes only by regeneration from this round's dispatch.
- The learning line (H-18 and after) is not touched.

## Architectural empowerment

The executing session may replace any instruction above with a better decision, recorded in its ADR. That covers:
- option 1 or option 3, the lane width and the chunk's length;
- whether the spike's path falls back to the scalar rule;
- how the gather and the write-back are done;
- where in `cortex-core` the lane form lives and what it is called;
- whether a bench case for the lane form is added;
- whether the round writes one ADR or two;
- how the evidence is laid out.

It may not reach ADR-0099's acceptance, ADR-0101's workload, ADR-0103's constraints (quality goal 2, no intrinsic, no
target feature, no dependency, `integrate` unchanged), the standing directives, the whitepaper's invariants, or the
constraints in `CLAUDE.md`.

## Verification

```bash
cargo check --workspace --all-targets --locked
cargo test --workspace --locked
cargo test --workspace --release --locked
cargo test --workspace --release --locked -- --ignored exhaustive --list
cargo test -p cortex-runtime --release --locked --test active -- --ignored exhaustive --nocapture
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --locked -- -D warnings
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --locked
cargo bench -p cortex-bench --bench hot_path --locked -- --test
cargo +1.85 check --workspace --all-targets --locked
cargo +1.85 test --workspace --locked
npm ci
npm run spec
git diff main...HEAD > target/pr.diff && cargo mutants --workspace --in-diff target/pr.diff
gh workflow run ci.yml --ref <this round's branch> -f scope=<what ADR-0075 gives this diff>
node scripts/exhaustive-costs.mjs from <the dispatch's downloaded artifacts> --run <its id> > scripts/exhaustive-costs.tsv
```

Every command exits 0. The in-diff gate reports no survivor in CI before the first timed run. The dispatched run is
green. Every reading of behaviour is unchanged, and the image format is 16. The timed runs follow ADR-0101's workload
and ADR-0099's protocol.

## Report

The closing message states:
- the design, and why it chose lanes across units or within a record;
- the lane form and its property test;
- the mutation gate's reading before the timing;
- the pins;
- the four runs' medians and ratios against the bounds, with the load log, and whether the lanes were kept;
- the one-worker readings, and whether the compiler vectorized;
- the per-turn breakdown after the change;
- the scope ADR-0075 gave this diff, the shards' times beside ADR-0102's, and the regenerated cost table;
- what the sweep found;
- what was not done and why;
- what the next decision should weigh.
