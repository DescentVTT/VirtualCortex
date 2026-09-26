---
status: archived
date: 2026-09-25
---

> **Executed 2026-09-26 in pull request #124.** Writes ADR-0100 (the sweep without the gate, built and not kept) and
> opens finding F-50. The design was written into the ADR before the code: each worker owns a fixed, contiguous range
> of the unit arena and serves in unit order the units a schedule bitmap names; the owner keeps the bit of a unit its
> turn leaves awake, and a push or an activation that finds a unit idle records its gate byte scheduled and sets its
> bit; the deque, the stealing and the compare-and-swap leave the turn. Built (`3887e83`), it held every reading of
> behaviour bit for bit — the determinism pin and its spike count masked and unmasked, ADR-0097's four tables, the
> differential test on one, two and four workers, every test of the workspace — so no pin was restated. Timed by
> ADR-0099's protocol on an idle developer machine (not admissible), five alternating pairs, medians, change/base:
>
> - (a) 0.406, (b) 0.369 and (d) 0.488, within their bounds of 0.80, 1.10 and 0.80;
> - (c) **1.178**, above its bound of 1.10.
>
> **Not kept**: the code is reverted in the same pull request and stays in the history; the pull request carries the
> readings, the ADR and the tests that hold behaviour (the pin's masked hash, the differential test on two workers).
> Two diagnostics beside the criterion place the regression in the workload, not the sweep: 0.657 at (c) on one
> worker, 0.748 on two with the harness's per-tick census of every unit taken out; F-50 records that the measure reads
> the instrument beside the engine. A first timing session was discarded unread (its parser missed the network lines,
> and another repository's mutation run held the machine at 100 per cent); how the idle machine is checked was written
> into the protocol before the session that is read. The next decision is named in ADR-0100 and not taken. Image
> format 16; no rule of the engine changed. Every deliverable is dispositioned below. Relative links gained one `../`
> so that they resolve from `archive/`; no other word, claim or figure changed.
> *The body below describes the tree before execution and is not maintained.*

# Brief 043: The sweep without the gate — each worker serves the units it owns, in order, with no deque and no compare-and-swap in the turn; every reading of behaviour held bit for bit, and the gain read against ADR-0099's criterion before it is kept

## Mission

**This brief makes the engine faster and changes no rule.** Under [ADR-0044](../../docs/adr/0044-reference-network.md)'s
drive the executor serves 99.99 per cent of the reference network's units on every tick
([ADR-0097](../../docs/adr/0097-the-active-set-measured.md)). It still finds each one through a work-stealing deque and
takes each turn through a compare-and-swap gate. On a developer machine the gate's schedule, begin and end cost 9.79 ns
of a turn of about 25 ns (not admissible). [ADR-0098](../../docs/adr/0098-the-integration-model.md) kept per-tick service and
named the levers on the engine's speed. [ADR-0099](../../docs/adr/0099-the-engines-speed.md) took the three that change no
rule, in order, and this is the first: **a sweep without the gate**.

When the round is done, the tree holds either the sweep or the readings that show it is not worth keeping. With the
sweep, each worker owns a set of units and serves, every tick, those of its own that are awake or have mail, in a fixed
order, with no deque and no compare-and-swap on the turn's path. Axiom A3's single-writer invariant is enforced by that
ownership, restated with its test, and the `unsafe` of phase 1 carries its invariant restated. In both outcomes the
tree holds the gain measured under ADR-0099's protocol against its criterion, written before the first timed run: at
most 0.80 of the base's wall time per tick at ADR-0097's runs (a) and (d), at most 1.10 at (b) and (c). It also holds a
per-turn breakdown after the change, which the next lever's ADR needs. **Every reading of behaviour holds bit for bit.**
A pin that also holds the scheduler's own bytes is restated only under ADR-0099's masked check.

---

## Standing directives

- **Latest ≠ Newest** (whitepaper §2.1, `CLAUDE.md` principle 4). This round adopts no new technique beyond the
  ownership it builds: no SIMD, no working layout of the integrated fields, no lookahead, no longer tick, no change of
  the rest condition. Those are later levers or rule changes (ADR-0098). No dependency, no nightly feature, no
  `std::simd`, no version bump of a tool. A lock-free structure the tree does not have is admissible only with a
  published algorithm and its memory-model argument, as `deque.rs` cites Chase and Lev (2005) and Lê et al. (2013);
  the round should prefer needing none.
- **No reading of behaviour moves** (ADR-0099). Every potential, window, stamp, threshold, short-term factor, weight and
  trace; the spike train and the spike count; the messages delivered and the turns served, summed over the workers;
  every pinned number of every round in the gate and in the weekly whole-domain tests; the AArch64 job; the
  differential test at one worker and four. **A pin that also holds the scheduler's own bytes** (the gate byte, a
  mailbox head's node index) **may be restated only if**, before it is restated, the round computes it with those bytes
  masked on the base and on the change, finds the two equal, and records both values beside the old and the new pin,
  with the spike count or any behaviour reading beside it unmoved. Any other pin that moves stops the round: it is a
  finding, the change is not merged, and there is no re-pin and no second attempt.
- **The gain is read, not argued** (ADR-0099, [ADR-0010](../../docs/adr/0010-measured-or-target.md)). Use ADR-0097's four
  runs, and build the base (main at the round's start) and the change in release in two target directories. Make five
  alternating pairs in one session on one otherwise idle machine, and take each run's median. The change is kept at
  **≤ 0.80 × base at (a) and (d) and ≤ 1.10 × base at (b) and (c)**, the wall time per tick. The figures are a
  developer machine's, recorded in `docs/benchmarks/results/` as `admissible: no`, and never written into §10.2. The
  constants do not move after a timed run.
- No record field and no image format moves (the format stays 16). `unsafe` stays in the runtime, under an invariant
  restated with its test ([ADR-0023](../../docs/adr/0023-executor.md)); `unsafe_code` stays forbidden in every state crate.
- No `f32`/`f64` anywhere; every operation on a state field saturates or wraps by name
  ([ADR-0029](../../docs/adr/0029-structural-enforcement.md)). Every loop ends by construction
  ([ADR-0062](../../docs/adr/0062-the-first-complete-sweeps-list.md)); a wait on another thread is the runtime's protocol
  and lives there only. Nothing allocates after start-up (`tests/no_alloc.rs`).
- A heavy run is an `#[ignore]`d test whose name contains `exhaustive`; the runtime's gate grows by at most one test
  ([ADR-0061](../../docs/adr/0061-the-learning-runs-leave-the-gate.md)). The mutation gate on the changed lines must pass
  ([ADR-0030](../../docs/adr/0030-verification-governance.md)).
- **The engine is read before a description of it is trusted**, this brief's included.
- Conventional Commits with a real body; never commit on `main`; the required checks keep their names.

## Context

Re-derived on 2026-09-25 against `main` at `3807d8c`, after ADR-0098 merged. Line numbers move; the symbols and the
quoted sentences are what to re-derive.

1. **The turn** (`runtime/cortex-runtime/src/executor.rs`, `Worker::phase_turns`, `steal`, `turn`). Phase 1 pops a
   unit from the worker's local deque, steals from the others' in a ring when it is empty, and per unit:
   - `begin_turn`, then drain the mailbox (each node freed to its pool), sort the batch "in message order, never in
     arrival order" (§8.3), sum it basal and apical, and scale it by the tick's gain;
   - `integrate`, and on a spike `step_stp`, the descendant count, the worker's spiked list, its spike trace and a slot
     in `Shared::fired`;
   - `end_turn`, and "if the unit is not at rest `try_schedule` it" onto the worker's `next_tick` list, which phase 3
     pushes onto the local deque. `turns` counts each turn (ADR-0097).
2. **Phases 2 and 3** (`phase_fan_out`, `phase_deliveries`, `deliver`). Phase 2 walks each spiked unit's chain: STDP
   against each target's last spike, consolidation, release, and a token into **this worker's** wheel, or a direct
   `deliver` for a delay of zero. Phase 3 advances the wheel and delivers what is due. `deliver` takes a node from
   this worker's pool, `mailbox_push`es it into the target's mailbox (any worker may push into any unit's), and "if
   its gate was idle" `try_schedule`s the target onto this worker's deque. Worker 0 also drains the injector
   (`ACTIVATE` wakes a unit without a message) and delivers a ripple's replay.
3. **The gate** (`crates/cortex-core/src/dynamics/neuron.rs`: `gate`, `try_schedule`, `begin_turn`, `end_turn`;
   [ADR-0017](../../docs/adr/0017-mailbox-and-gate-protocol.md)). `gate_state` at `[56]` is idle 0, scheduled 1,
   running 2. Axiom A3 (whitepaper §4): "At most one worker touches a record in any tick, enforced by a
   compare-and-swap gate." The deque (`runtime/cortex-runtime/src/deque.rs`) is sized "so that it cannot fill: a unit
   is queued at most once at a time, by the gate of axiom A3".
4. **Phase 1's `unsafe`** (`turn`): "SAFETY (phase 1): this worker took `unit` from a deque, where it was put by the
   one `try_schedule` that moved its gate to scheduled, so no other worker holds it".
5. **The pools** (`runtime/cortex-runtime/src/pool.rs`): "one free list per worker … The owner pops; any worker pushes
   a node back to the owner's list when it has drained it." Which pool a mailbox's nodes come from depends on which
   worker delivered them.
6. **The pins that may hold the scheduler's bytes.** `tests/differential.rs`: `PINNED_ARENA_HASH` is a CRC over every
   unit's 64 image bytes, "atomics as plain values", with the blocks and the spike train; `PINNED_SPIKE_COUNT` beside
   it is 95. Its comment records three earlier moves, one of them (ADR-0054) "for the unit's bytes … the dynamics did
   not change and the spike count did not move". The same file's `snapshot` compares `mailbox_head` and `gate` across
   one worker and four. **Any pin over an image's bytes** — for example the whole-image CRC ADR-0095 amended — may hold
   the gate byte and the mailbox heads too. Find every such pin before the change.
7. **Per-worker shares.** `tests/contention.rs` reports the shares "not asserted" ("Whether the work was shared
   depends on timing"); `tests/accounting.rs` reads `reports[0]` on one worker.
8. **The costs** (`docs/benchmarks/results/2026-09-25-dancr-win11.md`, not admissible): `gate/schedule_begin_end`
   9.79 ns, `neuron/integrate` 11.03 ns, `mailbox/push_drain_x16` 4.70 ns a message, `executor/idle_tick/2` 468 to
   543 ns. The runs: 24 to 25 ns of one worker per turn, and up to 38 ns in an earlier session of the same tests.
9. **The workload** (`runtime/cortex-runtime/tests/active.rs`). The four `exhaustive` runs print `DUMP … ns_per_tick …
   worker_ns_per_turn` on stderr: (a) 1 024 units under ADR-0044's drive, (b) sixteen times sparser, (c) 256 times
   sparser, (d) 4 096 units. At (c) the executor serves about 7.4 per cent of the units a tick, which is where a sweep
   that visits every unit it owns could be slower than the deque.
10. **The weekly job** ([ADR-0092](../../docs/adr/0092-the-shards-dealt-by-cost.md),
    [ADR-0075](../../docs/adr/0075-the-dispatch-scope-follows-the-diff.md)): a change under `src/` dispatches
    `scope=both`, and the round regenerates `scripts/exhaustive-costs.tsv` from its own dispatch.

## Deliverables

- [x] **The design, in a new ADR at the next free number (`ls docs/adr`), before the code.** Which units each worker
  owns; how a worker finds, each tick, the units of its own that are awake, have mail or were activated; what becomes of
  the deque and of stealing, including whether the sparse case keeps a list rather than a sweep; what the gate still
  does, if anything; how a message delivered by another worker reaches its owner's turn; how the order in which a worker
  serves its units, and so the order of its spiked list, fan-out and node allocation, is fixed; **axiom A3 restated**
  with the enforcement it now has and the test that holds it; and **phase 1's `unsafe` restated** with its invariant and
  its test. The round weighs load balance at (c) and at the learning line's configurations (two and four workers) and
  says what it chose.
  **Done** (ADR-0100, `0f7fee9`, before the code): contiguous ranges of ⌈N/W⌉ units fixed at construction; a schedule bitmap in each owner's region of words, padded to a cache line; a unit marked by its owner when its turn leaves it awake and by a push or an activation that finds its gate byte idle (a store and a fetch-or, idempotent, so no claim); the gate byte kept as the schedule's record; the deque and stealing removed, the bitmap the sparse case's list; A3 and phase 1's `unsafe` restated with their tests; the balance weighed at (c) and at two and four workers. Since the change is not kept, A3's enforcement and the `unsafe`'s invariant stand as ADR-0017 and ADR-0023 wrote them.
- [x] **The pins that hold the scheduler's bytes, found before the change.** Every pin over a unit's or an image's
  bytes, listed with the bytes it covers. For each one the change moves: its value with the gate byte and the mailbox
  heads masked, on the base and on the change, shown equal, both values recorded beside the old and new pin, and the
  spike count or behaviour reading beside it unmoved. If none moves, say so.
  **Done** (`014db40`, before the change): every pin over a unit's or an image's bytes is listed in ADR-0100 with the bytes it covers. Only `PINNED_ARENA_HASH` holds the scheduler's bytes, and with every mailbox head and gate byte masked it is the pin itself, `0x6c27858ece2dd412`: at its end every gate is idle and every mailbox empty. It did not move on the change, so no pin was restated; the test now asserts that the masked hash equals the pin.
- [x] **The code and its tests.** The ownership and the sweep in the executor. Tests that each unit is served by
  exactly one worker; that a unit woken by another worker's message is served by its owner at the next tick; that a
  unit at rest with no mail is not served (ADR-0097's counter holds the served set to the scheduled set, and its tables
  pin the turns); and that the differential test holds at one, two and four workers. `tests/no_alloc.rs` passes. The
  mutation gate on the diff reports no survivor.
  **Rejected:** by ADR-0099's criterion, not by review. Built at `3887e83` with `a_unit_is_served_by_its_owner_alone_and_a_woken_unit_at_the_next_tick`, `the_ranges_partition_the_arena_and_the_places_partition_the_schedule` and `set_gate`'s test; ADR-0097's tables and census, the differential test on one, two and four workers and `tests/no_alloc.rs` passed on it. Reverted in `5aa57c5` (`4b2f694` on the round's branch before the rebase that merged it, the same tree) because (c) read 1.178 against 1.10. The mutation gate was not run on code that is not merged; this pull request's diff has no source line to mutate.
- [x] **The gain, by ADR-0099's protocol.** The four runs, base and change, five alternating pairs, with each run's
  median and the ratio change/base, recorded in `docs/benchmarks/results/` with the machine, `admissible: no`. **Kept**
  when (a) and (d) are ≤ 0.80 and (b) and (c) ≤ 1.10. **If not kept**, the pull request carries the readings, the ADR
  and any test that holds behaviour, not the change, and the ADR names the next decision.
  **Done** (ADR-0100, `docs/benchmarks/results/2026-09-26-dancr-win11.md`): (a) 0.406, (b) 0.369, (c) 1.178, (d) 0.488 — **not kept**, so the pull request carries the readings, the ADR and the tests that hold behaviour, and ADR-0100 names the next decision. The controls, the per-worker turns, a discarded first session and two diagnostics are recorded beside the criterion.
- [x] **The per-turn breakdown after the change**, for the next lever's ADR. The runs' wall time per turn on the sweep,
  beside the bench's `neuron/integrate` and `mailbox/push_drain_x16`, as a developer machine's figures: what share of a
  turn is now the integration. A bench case for the sweep's turn if the round finds one needed.
  **Done** (ADR-0100, the results file): on one worker the sweep's tick at (a) is 10 981 ns for 1 024 turns, about 10.7 ns a turn against the base's 20.8, where the bench's `neuron/integrate` reads 13.45 ns that day (11.03 the day before) and `mailbox/push_drain_x16` 7.6 ns a message: the integration is the turn. On two workers the runs read 13, 11 and 18 ns a worker-turn at (a), (b) and (d). No bench case was needed: the one-worker run is the sweep's turn in isolation. The base's bench of that session was overtaken by another process tree's load and is not read.
- [x] **The evidence.** A weekly dispatched on this round's branch at the scope ADR-0075 gives the diff (the clause
  stated in the ADR), green in every job, every pinned number reproduced or restated under the masked check, and the
  sweep's survivors dispositioned. The shards' times are read beside the base's as a secondary reading. The cost table
  is regenerated from that run.
  **Done** (ADR-0100): the weekly dispatched on this round's branch, run `36188866157` at the branch's `df38fa8` (`a7db817` on `main`, the same tree) with `scope=exhaustive` (ADR-0075: against `main` no file under `src/` changes, the change and its revert netting to nothing; no test is deleted or taken out of the swept suite; `.cargo/mutants.toml` is as it was), green in every job it runs: the four whole-domain shards took 44m42s, 50m04s, 1h19m35s and 58m44s, their tests' wall 2 631, 2 957, 4 722 and 3 471 s, 37, 41, 66 and 48 per cent of the 7 200-second bound, every pinned number of those tests reproduced. Beside them, as the secondary reading, ADR-0097's run `36050252444` on the same code (ADR-0098 and ADR-0099 changed documents only): 95m38s, 50m03s, 44m37s and 77m11s. The same tests moved by 0.55 to 1.99 times between the two runs on identical code, the runners' (F-45's kind): the longest, H-14's run, 2 475 s then and 3 097 now, and H-18's assignment arm 4 121 then and 2 284 now. The cost table is regenerated from this run, 57 lines, 26 945 seconds against the previous table's 29 300. No sweep was dispatched, so there is no survivor to disposition; Monday's schedule sweeps `main` as it always does.
- [x] **The documents, in the same pull request.** Whitepaper §4's A3 row and every sentence that says the gate
  enforces it; §6.1's executor paragraph and its assertions; §8.5; §11.1's question on the engine's speed, with the
  first lever's outcome; §9. The ADR index; `CHANGELOG.md`; `CLAUDE.md`; `docs/zh-TW`'s reader's guide as the result
  requires. The whitepaper's version in both declarations with its date
  ([ADR-0064](../../docs/adr/0064-the-documentation-gate-and-the-version.md)).
  **Done**, as the result required: the change is not kept, so §4's A3 row, §6.1 and §8.5 stand as they were, and so do ADR-0023 and ADR-0017. §11.1 records the first lever's outcome, §9 the ADR's row, §11 finding F-50, and the executive summary and §8.3 the differential test on two workers, with assertions; the ADR index, `CHANGELOG.md`, `CLAUDE.md` and the reader's guide; whitepaper 4.52.0 in both declarations, dated 2026-09-26.
- [x] **The brief archived** as `briefs/README.md` says, every deliverable dispositioned.
  **Done**: this file.

## Not empowered

- **No rule of the dynamics changes**: not the membrane, the rest condition, the order of a batch, STDP, the
  consolidation, the modulation, the gain, the drive, the task. No reading of behaviour moves.
- No pin is restated except under ADR-0099's masked check. The determinism pin's spike count does not move.
- No SIMD, no working layout of the integrated fields, no lookahead, no change of the barrier's count per tick, no
  longer tick. Those are later levers (ADR-0099) or rule changes (ADR-0098).
- No record field, no image format, no new crate, no dependency, no `unsafe` outside the runtime, no float.
- The criterion's constants and the protocol do not move after the first timed run, and no second attempt is made at
  this lever in this round.
- No change to `.github/workflows/ci.yml`, `scripts/exhaustive-shard.sh` or `scripts/exhaustive-costs.mjs`; the cost
  table changes only by regeneration from this round's dispatch.
- The learning line (H-18 and after) is not touched.

## Architectural empowerment

The executing session may replace any instruction above with a better decision, recorded in its ADR. That covers:
- how units are partitioned among workers, and whether ownership is fixed at construction or at each window;
- how a worker finds its awake units (a sweep, a list, a bitmap, or a hybrid chosen by the fraction served);
- whether the deque and stealing survive for the sparse case;
- what the gate still records;
- whether a bench case is added;
- whether the round writes one ADR or two;
- which further runs it reads beside the criterion's four.

It may not reach the acceptance or the criterion of ADR-0099, the standing directives, the whitepaper's invariants
beyond A3's enforcement (which ADR-0099 opens to this round), or the constraints in `CLAUDE.md`.

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
cargo bench -p cortex-bench --bench hot_path --locked
cargo +1.85 check --workspace --all-targets --locked
cargo +1.85 test --workspace --locked
npm ci
npm run spec
git diff main...HEAD > target/pr.diff && cargo mutants --workspace --in-diff target/pr.diff
gh workflow run ci.yml --ref <this round's branch> -f scope=<what ADR-0075 gives this diff>
node scripts/exhaustive-costs.mjs from <the dispatch's downloaded artifacts> --run <its id> > scripts/exhaustive-costs.tsv
```

Every command exits 0; the in-diff gate reports no survivor in CI; the dispatched run is green; every reading of
behaviour is unchanged; the image format is 16. The timed runs follow ADR-0099's protocol: the base and the change in
two target directories, five alternating pairs, and the medians.

## Report

The closing message states:
- the design, and how A3 and phase 1's `unsafe` are now held;
- every pin over a unit's or an image's bytes, and for each one that moved, its masked values on the base and the
  change;
- the four runs' medians and ratios against the criterion, and whether the change was kept;
- the per-turn breakdown after the change;
- the scope ADR-0075 gave this diff, the shards' times beside the base's, and the regenerated cost table;
- what the mutation gate and the sweep found;
- what was not done and why;
- what the next lever's ADR (the working layout) should weigh.
