---
status: proposed
date: 2026-09-27
---

# Brief 050: The region before the readout — on the geometry F-54's decision gives the task and the context, the usable region of ADR-0115's marked assemblies measured for its width and for whether it survives another seed, a drained network and the task's own stimulus; measured, nothing changed

## Mission

**This brief measures and changes no rule.** ADR-0115 read one usable cell: 64 members marked under set (ii) at a
quarter of Q1.15's range ([ADR-0115](../docs/adr/0115-the-assemblies-marked.md)). It sits at the grid's lightest weight
and largest size, on one draw of the delays, on the settled network, with no task running.

[ADR-0116](../docs/adr/0116-the-region-before-the-readout.md) deferred the gated readout by one round to ask four
things first:
- **where the context sits** beside the task: F-54 decided, the readouts give up places 5 and 16;
- **how wide the usable region is**;
- **whether it survives** another seed, a network drained as a learning run leaves it, and the task's own stimulus;
- **which cell** the second round builds on.

When the round is done, the tree holds:
- the geometry beside the task's, the readouts at eight places each;
- the core grid and the three conditions, measured by ADR-0115's rules;
- an ADR that records the region, the robust cells and the cell for the second round, or what failed where, and names
  the next decision **without taking it**.

---

## Standing directives

- **Latest ≠ Newest** (whitepaper §2.1, `CLAUDE.md` principle 4). This round adopts nothing:
  - no rule of the engine changes: not the membrane, the short-term plasticity or its class, STDP, the modulator, the
    executor, the drive, the task or the critic;
  - no dependency, and no version bump of a tool.

  The geometry, the assemblies, the marks and the conditions are the test's own, as ADR-0115's were.
- **The geometry is added, not changed.** `geometry` stays as it is for every earlier test; the new one is beside it.
- **Every weight frozen in every measured run.** The excitatory baseline is zero, the inhibitory baseline and the
  signed gate unset, and no reward. Every weight at the end of each run equals its value at the start. Condition (c)'s
  learning run produces an image and is not a measured run. It runs under H-20's configuration, whose inhibitory
  baseline would go on consolidating every inhibitory synapse, so its image is written with the inhibitory baseline and
  the signed gate unset before any cell runs on it.
- **ADR-0115's protocol, spans, release, stretch, measures and thresholds**, unchanged. Its two cells in the core grid
  (64 units at 0.25 and at 0.375) reproduce ADR-0115's tables bit for bit, or no other cell runs.
- **The grid, the subset, the conditions and the rules for "robust" and for the second round's cell are written before
  any run**, and none moves after one. The grid may be widened before the first run, not narrowed.
- **The engine is read before a description of it is trusted**, this brief's and ADR-0116's included. The kick is read
  on the engine under every condition before its cells run, by ADR-0112's measure.
- No `f32`/`f64`, oracles included; every operation on a state field saturates or wraps by name
  ([ADR-0029](../docs/adr/0029-structural-enforcement.md)). Every loop ends by construction
  ([ADR-0062](../docs/adr/0062-the-first-complete-sweeps-list.md)).
- A heavy run is an `#[ignore]`d test whose name contains `exhaustive`; the runtime's gate grows by at most one test
  ([ADR-0061](../docs/adr/0061-the-learning-runs-leave-the-gate.md)). The mutation gate on the changed lines must pass
  ([ADR-0030](../docs/adr/0030-verification-governance.md)).
- **No shard of the weekly job passes 60 per cent of its bound** under the regenerated deal
  ([ADR-0092](../docs/adr/0092-the-shards-dealt-by-cost.md)). ADR-0116 estimates the round at about a third of the
  present cost.
- No pinned number of an earlier round moves.
- Conventional Commits with a real body; never commit on `main`; the required checks keep their names.

## Context

Re-derived on 2026-09-27 against `main` at `8a7726a`, after ADR-0115 merged. Line numbers move; the symbols and the
quoted sentences are what to re-derive.

1. **The task's geometry** (`runtime/cortex-runtime/tests/instrument/harness.rs`).
   - `geometry(units, rotation)` returns the four sets over whole periods of `PERIOD` (20) from `rotation`
     (`ROTATION_1024` is 0).
   - Stimulus A is at `A_OFFSET` (0) and B at `B_OFFSET` (11).
   - Readout 0 is `R0_MASK` (`0xAA2AA`: places 1, 3, 5, 7, 9, 13, 15, 17, 19) and readout 1 `R1_MASK` (`0x55554`:
     2, 4, …, 18).
   - `PRIOR_WINDOW` is 8.
2. **The new geometry** (ADR-0116): readout 0 `0xAA28A` (place 5 dropped), readout 1 `0x45554` (place 16 dropped),
   the stimulus places unchanged. At 1 024 units, 102 units at places 5 and 16 are in no set.
3. **The substrate** (`runtime/cortex-runtime/tests/assembly.rs`, ADR-0112 and ADR-0115):
   - `members`, `wire`, `grown` and `marked`;
   - the kick (`inject_before`, `note_kick_spikes`, `KICK_MESSAGE_Q16`, `KICK_RESET_Q16`);
   - `layout`, `span_protocol`, `class_run`, `background_049`, `holding`, `failed` and `bursts_049`;
   - `SETS`, `WEIGHTS_049`, `RELEASE_STRETCHES_049` (six stretches under set (ii)) and `delay_of` with `DELAY_SEED`
     (48).
4. **The drained network.** H-20's arms (`tests/inhibition.rs`, `run_on_scheduled` on the shared harness) run H-19's
   configuration through three reversals over 7 680 trials. The arm from the assignment cost 1 505 s on the hosted
   runner (the cost table). ADR-0110 read the inhibitory sum levelling at about 0.07 and 0.08 of the image's.
   `run_on_scheduled` takes the engine by `&mut`, so the engine after the arm is the test's to encode.
5. **The task's stimulus** as H-20's arms present it: F-46's shape (`SHAPE_F46`, two messages of 1.25) with ADR-0076's
   cancel (`CANCEL_PICKED_1024`, `CANCEL_AT_THE_EXTREME`), into A or B by the task's draw.
6. **The prior's reach.** Place 5 is five places from A's place 0; place 16 is five from B's place 11 and four from the
   next period's place 0; all are within `PRIOR_WINDOW`.
7. **ADR-0115's readings**: the usable cell's bursts mostly 10 000 to 33 000 ticks apart, its product 1.45 times the
   unkicked over the held stretches; 32 units at 0.25 holding in 5 rounds and igniting in 2; 64 units at 0.375
   igniting in every round.
8. **The weekly job** ([ADR-0092](../docs/adr/0092-the-shards-dealt-by-cost.md),
   [ADR-0075](../docs/adr/0075-the-dispatch-scope-follows-the-diff.md)): a round that changes only tests dispatches
   `scope=exhaustive`. The cost table is regenerated from its own dispatch.

## Deliverables

- [ ] **The protocol, in a new ADR at the next free number (`ls docs/adr`), before any run.** At least:
  - *The geometry*: beside `geometry`, readouts at `0xAA28A` and `0x45554`, shown disjoint from every member at every
    size of the grid.
  - *The core grid*: the settled image, seed 48, no task; set (ii); sizes 48, 64, 80 and 96 at ADR-0112's nested
    placement; weights 0.125, 0.1875, 0.25, 0.3125 and 0.375 of Q1.15's range. Each size has its own background and
    control.
  - *The subset*: sizes 64 and 96 at 0.1875, 0.25 and 0.3125.
  - *The three conditions on the subset*, each with its own backgrounds and controls:
    - **(b)** the assembly's delays drawn with seed 49;
    - **(c)** the image H-20's arm from the assignment leaves at its last trial, frozen, pinned by its CRC and its
      inhibitory sum beside ADR-0110's;
    - **(d)** on the settled image, the task's stimulus at every epoch's start, A or B by the task's draw, weights
      frozen, no reward.
  - *The rules*, committed:
    - **robust**: a cell usable in the core grid and under (b), (c) and (d);
    - **the cell for the second round**: of the robust cells, the one with the most usable neighbours in the core
      grid, one weight step and one size step either way; ties to the lighter weight, then the smaller size;
    - **if none is robust**, the failure under each condition, read by ADR-0115's `failed`.
  - *Readings, no clause*:
    - under (d), each readout's count per trial with the context held and with it quiet;
    - under (d), whether the eight-place readouts resolve the stimulus from the background by ADR-0065's measure.
- [ ] **ADR-0115's two cells reproduced**, 64 units at 0.25 and at 0.375, bit for bit against ADR-0115's pins, before
  any other cell.
- [ ] **The kick read on the engine under each condition**, by ADR-0112's measure, before that condition's cells. It
  fires every member once, or it is derived again before any cell.
- [ ] **The runs.** Every cell, background and control. The weights shown unchanged at each run's end. The tables pinned
  as ADR-0115's: stretches in full, windows by hash.
- [ ] **The ADR's reading.**
  - The core grid's table and its usable region.
  - Each condition's table and the robust cells.
  - The cell for the second round, or what failed under which condition.
  - The readings under (d).
  - **The next decision named and not taken**:
    - if a cell is robust, ADR-0111's second round, a readout gated by the context, on that cell and this geometry, with
      H-20's schedule on this geometry without a context as the third round's baseline;
    - if none is, the branch ADR-0116 names for what failed: under (c), the inhibitory drain or an inhibition of the
      context's own; under (d), the context's isolation from the task's stimulus; under (b), the class's constants.
- [ ] **The gate.** At most one runtime test: the geometry's disjointness at every size, the rules for robust and for
  the second round's cell over tables written by hand, and a marked assembly on the new geometry kicked for a few
  hundred ticks.
- [ ] **The evidence.** A weekly dispatched on this round's branch at `scope=exhaustive`, since only tests change
  ([ADR-0075](../docs/adr/0075-the-dispatch-scope-follows-the-diff.md)). It must be green in every job and reproduce
  every pinned number. The cost table is regenerated from that run's artifacts.
- [ ] **The documents, in the same pull request.**
  - Whitepaper §11.1's question on a rule held by the network, F-54's row, and §9.
  - The ADR index, `CHANGELOG.md`, `CLAUDE.md`, and `docs/zh-TW`'s reader's guide as the result requires.
  - The whitepaper's version in both declarations, with its date
    ([ADR-0064](../docs/adr/0064-the-documentation-gate-and-the-version.md)).
- [ ] **The brief archived** as `briefs/README.md` says, every deliverable dispositioned.

## Not empowered

- **No rule of the engine changes**, and no weight moves during a measured run.
- No readout gated by the context, no switch on errors, no reward in a measured run. Those are ADR-0111's later rounds.
- `geometry` and every earlier test's constants are not touched.
- The grid, the subset, the conditions and the rules do not move after a run, and the grid is not narrowed.
- No pinned number of an earlier round moves.
- No change to `.github/workflows/ci.yml`, `scripts/exhaustive-shard.sh` or `scripts/exhaustive-costs.mjs`; the cost
  table changes only by regeneration from this round's dispatch.
- No float anywhere, no dependency, no `unsafe`.

## Architectural empowerment

The executing session may replace any instruction above with a better decision, recorded in its ADR. That covers:
- where the geometry lives and how the harness names it;
- how condition (c)'s image is produced, including H-18's arm in place of H-20's if the shards' budget requires it,
  and ADR-0116's option 3(b), the inhibitory weights scaled, as a reading beside it;
- how condition (d)'s stimulus is timed against the kick and the release;
- widening the grid: a weight between two of the five, a size between two of the four up to 102, set (i) at the lighter
  weights as a reading, or the subset;
- where the tests live;
- whether the round writes one ADR or two.

It may not narrow the grid or the subset, move a threshold or a rule after a run, change a rule of the engine, or
reach the standing directives, the whitepaper's invariants or the constraints in `CLAUDE.md`.

## Verification

```bash
cargo check --workspace --all-targets --locked
cargo test --workspace --locked
cargo test --workspace --release --locked
cargo test --workspace --release --locked -- --ignored exhaustive --list
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --locked -- -D warnings
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --locked
cargo +1.85 check --workspace --all-targets --locked
cargo +1.85 test --workspace --locked
npm ci
npm run spec
git diff main...HEAD > target/pr.diff && cargo mutants --workspace --in-diff target/pr.diff
gh workflow run ci.yml --ref <this round's branch> -f scope=exhaustive
node scripts/exhaustive-costs.mjs from <the dispatch's downloaded artifacts> --run <its id> > scripts/exhaustive-costs.tsv
```

Every command exits 0. The dispatched run is green and reproduces every pinned number. The protocol precedes the first
run in the history, and ADR-0115's two cells are reproduced before any other cell runs.

## Report

The closing message states:
- the geometry, the grid, the subset, the conditions and the rules, and when they were committed;
- ADR-0115's two cells reproduced;
- the kick read on the engine under each condition;
- the core grid's table and its usable region;
- each condition's table, the robust cells, and the cell for the second round or what failed where;
- the readings under (d);
- that no weight moved in a measured run and no pinned number moved;
- the scope ADR-0075 gave the diff, the shards' times and the regenerated cost table;
- what was not done and why;
- the next decision named, not taken.
