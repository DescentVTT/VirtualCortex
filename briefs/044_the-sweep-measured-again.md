---
status: proposed
date: 2026-09-26
---

# Brief 044: The sweep measured again — ADR-0100's sweep without the gate, re-applied as built and timed on ADR-0097's four runs with no census in the timed ticks, against ADR-0099's bounds unchanged; kept or not by that reading alone

## Mission

**This brief makes the engine faster, or reads that it does not, and changes no rule.**
[ADR-0100](../docs/adr/0100-the-sweep-without-the-gate.md) built the sweep without the gate: each worker owns a fixed
range of the units and serves it in unit order from a schedule bitmap, with no deque, no stealing and no
compare-and-swap in the turn. It held every reading of behaviour bit for bit. It did not keep the sweep, because
[ADR-0099](../docs/adr/0099-the-engines-speed.md)'s workload read 1.178 of the base's wall time per tick at
[ADR-0097](../docs/adr/0097-the-active-set-measured.md)'s run (c), against a bound of 1.10. That workload's harness reads
every unit's gate byte on worker 0 between the timed ticks (F-50).

[ADR-0101](../docs/adr/0101-the-sweep-measured-again.md) takes ADR-0100's second option: the same sweep, timed again on
a workload whose timed ticks carry no census, against the same bounds. ADR-0101 says openly that this workload was chosen
after the first reading failed.

When the round is done, the tree holds either the sweep, landed as ADR-0100 designed it, or the reading that it is still
not worth keeping. In both cases it holds:
- the census-free timing path, as an instrument, in `tests/active.rs`;
- the criterion's readings, with ADR-0099's census workload read again beside them;
- a new ADR that records the verdict.

**Every reading of behaviour holds bit for bit.**

---

## Standing directives

- **Latest ≠ Newest** (whitepaper §2.1, `CLAUDE.md` principle 4). This round adopts nothing new:
  - no SIMD, no working layout of the integrated fields, no lookahead, no longer tick, no change of the rest condition;
  - no balance for the sparse case, and no lock-free structure the tree does not have;
  - no dependency, no nightly feature, no `std::simd`, no version bump of a tool.
- **The sweep is ADR-0100's, as built** (`ff6a297` on `main`), re-applied by reverting its revert (`5aa57c5`). It may
  change only where `main` has moved under it, or where the mutation gate on its diff requires a change that moves
  neither behaviour nor a turn's work (a range pattern for a guard, say). Every such change is listed in the round's ADR
  and **committed before the first timed run**. Tuning the sweep is a new lever, not this round.
- **The workload is ADR-0101's.**
  - ADR-0097's four configurations unchanged: (a) 1 024 units under ADR-0044's drive, (b) sixteen times sparser, (c) 256
    times sparser, (d) 4 096 units; the prior at seed 22, the gain 1.75, two workers, the lead-in and four windows.
  - **No census in the timed ticks**: no test-side read of any unit's record between two timed ticks.
  - Every row still held to ADR-0097's pinned tables, from the counts the engine keeps.
  - ADR-0097's four weekly tests keep their census as tests of behaviour.
- **The bounds are ADR-0099's, unchanged.** The change is kept at a median wall time per tick of at most **0.80** of the
  base's at (a) and (d) and at most **1.10** at (b) and (c). Nothing about them moves after a timed run.
- **The protocol is ADR-0099's and ADR-0100's.**
  - Builds: each build is exported with `git archive` and built in a target directory of its own. The base is `main` at
    the round's start plus the census-free timing path, with no sweep. The change is the base plus the sweep.
  - Pairs: five, alternating, the base first in pairs 1, 3 and 5. The four runs go in order within each build's turn,
    and each run's figure is the median of its five readings. Every run's whole output is kept.
  - Idle machine: the session starts only after the processor total has stayed below 15 per cent for thirty seconds with
    none of the round's processes running, and the total is sampled every five seconds through it. **A session that
    another process tree disturbs is discarded unread**, as ADR-0100's first was.
- **No reading of behaviour moves** (ADR-0099):
  - every potential, window, stamp, threshold, short-term factor, weight and trace;
  - the spike train and count;
  - the messages delivered and the turns served, summed over the workers;
  - every pinned number of every round;
  - the AArch64 job;
  - the differential test at one, two and four workers.

  A pin that also holds the scheduler's own bytes is restated only under ADR-0099's masked check. ADR-0100 read none
  moved: the determinism pin with those bytes masked is the pin. Any other pin that moves stops the round: it is a
  finding, the change is not merged, and there is no re-pin and no second attempt.
- No record field and no image format moves (16). `unsafe` stays in the runtime, under its invariant restated with its
  test ([ADR-0023](../docs/adr/0023-executor.md)). No `f32`/`f64`; saturating or named-wrapping arithmetic on state
  ([ADR-0029](../docs/adr/0029-structural-enforcement.md)). Every loop ends by construction
  ([ADR-0062](../docs/adr/0062-the-first-complete-sweeps-list.md)). Nothing allocates after start-up.
- A heavy run is an `#[ignore]`d `exhaustive` test; the runtime's gate grows by at most one test
  ([ADR-0061](../docs/adr/0061-the-learning-runs-leave-the-gate.md)). The mutation gate on the changed lines must pass
  ([ADR-0030](../docs/adr/0030-verification-governance.md)).
- **The engine is read before a description of it is trusted**, this brief's included.
- Conventional Commits with a real body; never commit on `main`; the required checks keep their names.

## Context

Re-derived on 2026-09-26 against `main` after ADR-0100 merged (`042f5aa`, with PR #125's citations). Line numbers move;
the symbols and the quoted sentences are what to re-derive.

1. **The sweep, as built** (`ff6a297` on `main`; ADR-0100's design):
   - Ownership: contiguous ranges of $\lceil N/W \rceil$ units, fixed in `Executor::new`; `Executor::owner`.
   - The schedule: `Shared::scheduled`, one bit per unit, each owner's region padded to a cache line. The owner keeps
     the bit of a unit its turn leaves awake. A push or an activation that finds a unit idle stores its gate byte
     scheduled and sets the bit with a fetch-or.
   - The gate byte is kept as the schedule's record, never running. `cortex-core`'s `set_gate`, a relaxed store.
   - `deque.rs`, `Config::deque_capacity` and `Worker::steal` removed.
   - Axiom A3 and phase 1's `unsafe` restated in ADR-0100, with the tests
     `a_unit_is_served_by_its_owner_alone_and_a_woken_unit_at_the_next_tick`,
     `the_ranges_partition_the_arena_and_the_places_partition_the_schedule` and
     `set_gate_records_each_state_without_a_claim`.
   - Its revert is `5aa57c5`. The mutation gate was never run on its lines.
2. **The census** (`runtime/cortex-runtime/tests/active.rs`, `read`). Before each timed tick, `let due = scheduled(exec)`
   reads every unit's gate byte, and after it `assert_eq!(served, due, …)`. The timed call is `exec.tick()` between
   `Instant::now()` and `elapsed()`. The rows (turns, delivered, sent, spikes, fewest, most, full) come from the engine's
   counters, `Executor::turns` and `Executor::delivered`, and the train. `NETWORK_ROWS` and `CONTROL_ROWS` pin them.
3. **F-50** (whitepaper §11) and ADR-0100's readings (`docs/benchmarks/results/2026-09-26-dancr-win11.md`, not
   admissible):

   | Run | Ratio | Bound |
   | :--- | ---: | :--- |
   | (a) | 0.406 | met |
   | (b) | 0.369 | met |
   | (c) | 1.178 | not met |
   | (d) | 0.488 | met |

   Diagnostics, not a verdict: 0.657 at (c) on one worker, and 0.748 at (c) on two workers without the census. The
   per-turn breakdown: on one worker at (a) the sweep's turn is about 10.7 ns against the base's 20.8, the integration
   alone.
4. **The one harness step that reads every record between ticks** outside ADR-0097's census: the instrument's `quiet`
   (`tests/instrument/harness.rs`) calls `Executor::is_quiescent`, every unit's mailbox head, between the ticks of the
   quiet run before an image. It is not in the timed workload.
5. **ADR-0100's first session was discarded** because another repository's mutation run held every logical processor.
   Nothing on the machine is the round's to stop. If the machine is not idle, the round waits and says so.
6. **The weekly job** ([ADR-0092](../docs/adr/0092-the-shards-dealt-by-cost.md),
   [ADR-0075](../docs/adr/0075-the-dispatch-scope-follows-the-diff.md)): the scope follows the round's own diff, and the
   cost table is regenerated from its dispatch.

## Deliverables

- [ ] **The census-free timing path, in the base, before the sweep.** A way to run ADR-0097's four configurations whose
  timed ticks carry no census, every row still held to `NETWORK_ROWS` (and to `CONTROL_ROWS` where a control is run).
  ADR-0097's four weekly tests keep their census as they are. Committed on the round's branch before the sweep's commit;
  the base build is this commit.
- [ ] **The sweep re-applied**, by reverting `5aa57c5`, with only the changes the standing directives allow, each listed
  in the round's ADR. All of the tree's tests pass on it, the differential test at one, two and four workers included.
  **The mutation gate on its diff is read before the first timed run**, in CI on the round's pull request (the Windows
  linker's LNK1104 makes a local unviable mark unreliable). A survivor is met by a test, or by a change that moves neither
  behaviour nor a turn's work, committed before the timing.
- [ ] **The gain, by ADR-0101's workload and ADR-0099's protocol.**
  - The four runs, base and change, five alternating pairs, each run's median and the ratio change/base, the processor
    samples beside them, recorded in `docs/benchmarks/results/` with the machine, `admissible: no`.
  - **Kept** when (a) and (d) are ≤ 0.80 and (b) and (c) ≤ 1.10.
  - Beside it, and not a criterion: the same five pairs on ADR-0099's census workload, and one worker at (a) and (c).
- [ ] **A new ADR at the next free number (`ls docs/adr`)**: the instrument, the sweep's re-application and any listed
  change, the pins, the readings and the verdict.
  - **If kept**: the sweep lands as ADR-0100 designed it; ADR-0017's and ADR-0023's decisions amended where it says; and
    what the working layout's round should weigh.
  - **If not kept**: the pull request carries the readings, the instrument and the ADR, not the sweep; there is no
    further attempt at the sweep in this form; and the next decision is named among ADR-0100's options 1, 3, 4 and 5 and
    not taken.
- [ ] **The evidence.** A weekly dispatched on this round's branch at the scope ADR-0075 gives the diff, the clause stated
  in the ADR. It must be green in every job, every pinned number reproduced, and the sweep's survivors dispositioned
  (which is the whole-tree reading of the sweep's lines if it is kept). The shards' times are read beside ADR-0100's as a
  secondary reading, and the cost table is regenerated from that run.
- [ ] **The documents, in the same pull request.**
  - If kept: whitepaper §4's A3 row and every sentence that says the gate enforces it; §6.1's executor paragraph and its
    assertions; §8.5.
  - Always: §11.1's question on the engine's speed, with this round's outcome; §9; the ADR index; `CHANGELOG.md`;
    `CLAUDE.md`; `docs/zh-TW`'s reader's guide.
  - The whitepaper's version in both declarations, with its date
    ([ADR-0064](../docs/adr/0064-the-documentation-gate-and-the-version.md)).
- [ ] **The brief archived** as `briefs/README.md` says, every deliverable dispositioned.

## Not empowered

- **No rule of the dynamics changes**, and no reading of behaviour moves.
- **The sweep is not tuned.** No change to its code beyond `main`'s drift and the mutation gate's needs, none after the
  first timed run, and no second timed session to replace a first that was read.
- The bounds, the protocol, the workload's configurations and the order of runs do not move. A session that is
  disturbed is discarded unread and says why.
- ADR-0097's census tests and their tables are not changed or loosened.
- No SIMD, no working layout, no lookahead, no balance for the sparse case, no longer tick, no rest condition.
- No record field, no image format, no new crate, no dependency, no `unsafe` outside the runtime, no float.
- No change to `.github/workflows/ci.yml`, `scripts/exhaustive-shard.sh` or `scripts/exhaustive-costs.mjs`; the cost
  table changes only by regeneration from this round's dispatch.
- The learning line (H-18 and after) is not touched.

## Architectural empowerment

The executing session may replace any instruction above with a better decision, recorded in its ADR. That covers:
- how the census-free path is built (a parameter of `read`, a timing-only test, or another form), so long as its timed
  ticks carry no read of any record the engine does not make and its rows are held to the pinned tables;
- whether the control runs are part of the timed session;
- how the one-worker diagnostic is run;
- whether the round writes one ADR or two;
- how the evidence is laid out.

It may not reach ADR-0099's acceptance, ADR-0101's workload rule or bounds, the standing directives, the whitepaper's
invariants beyond A3's enforcement (which ADR-0099 opened to the sweep), or the constraints in `CLAUDE.md`.

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
- the census-free path and how it holds the rows;
- the sweep's re-application and every listed change;
- the mutation gate's reading before the timing;
- the pins;
- the four runs' medians and ratios against the bounds, the processor samples, and whether the sweep was kept;
- the census workload's and the one worker's readings beside them;
- the scope ADR-0075 gave this diff, the shards' times beside ADR-0100's, and the regenerated cost table;
- what the sweep found;
- what was not done and why;
- what the next lever's ADR should weigh.
