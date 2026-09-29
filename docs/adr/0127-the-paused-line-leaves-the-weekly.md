---
status: accepted
date: 2026-09-29
depends-on: ADR-0126
decision-makers: VirtualCortex maintainers
---

# ADR-0127: The paused line leaves the weekly — the rule held by the network paused (ADR-0126), forty of its forty-four whole-domain tests leave the weekly job, renamed from `…_exhaustive` to `…_paused` and kept in the tree with every pin, runnable by name; four stay as the line's representatives, each holding one mechanism the line built; the weekly job's plan falls from about 61 to about 24 per cent of its bound, with nothing of the engine or of any test's reading changed

## Context and Problem Statement

[ADR-0126](0126-the-gate-raised-measured.md) applied ADR-0125's rule and **paused the rule held by the network**. The six rounds from brief 048 to brief 053 left the engine two mechanisms, unset bit for bit: ADR-0114's facilitating class and ADR-0123's slow current. They also left forty-four whole-domain tests in `runtime/cortex-runtime/tests/assembly.rs`.

Those tests dominate the weekly job. The cost table regenerated from ADR-0126's dispatch (run 36460574405) is 111 tests and 51 421 s, of which the `assembly` binary is 44 tests and 32 742 s, 64 per cent. Replayed through [ADR-0092](0092-the-shards-dealt-by-cost.md)'s deal at six shards ([ADR-0121](0121-a-sixth-shard.md)), it plans every shard at 59.5 to 63.3 per cent of its bound, past the briefs' 60, and ADR-0126 recorded that. Three of them rebuild H-20's arm to reach the drained image, at 2 348 to 2 816 s each.

**A paused line's tests are records, not guards.** Each pins a measurement that no later round extends: a grid, a condition, a background. The engine's behaviour they depend on is guarded elsewhere:
- the mechanisms by their unit and property tests in `cortex-core` and the runtime (ADR-0114, ADR-0123);
- the rest of the engine by the other whole-domain tests and the determinism pin.

Running all forty-four every week buys a reproduction of readings that nothing uses, at the cost of the margin every other round needs.

## Decision Drivers

- The briefs' 60 per cent, which the tree passes now.
- **Nothing of any reading changes.** A test that leaves the weekly keeps its pins and its code, and runs as it did when run by name.
- ADR-0061's precedent: runs whose cost outweighs what they guard where they run leave that place, named and runnable.
- **Every mechanism the line built keeps one whole-domain run** that exercises it in a measured protocol, so that a change breaking it in a way the unit tests miss is still caught weekly.
- No change to the deal, the scripts or the job: the weekly runs what `--list` names under `exhaustive` (ADR-0073, ADR-0084).

## Considered Options

1. **Rename the tests that leave**, `…_exhaustive` to `…_paused`, `#[ignore]`d as before, so that `--ignored exhaustive` no longer lists them and `--ignored paused` does.
2. **A Cargo feature** that gates the `assembly` binary: the gate's tests in it would leave the pull request's checks with the heavy ones.
3. **A skip list in the shard script**: a list in a script and a list in the tree drift (F-44, F-45).
4. **A seventh shard**: ADR-0121 named the sixth as the last the lever buys; the floor was ADR-0120's condition (c) cells at 2 680 s.
5. **Keep all forty-four.**

## Decision Outcome

**Option 1.**

- **Four stay**, one for the substrate and one for each mechanism or pair of them:
  - `the_kick_and_the_background_at_1024_units_exhaustive` (ADR-0112): the substrate — the grown image unwired, the kick and the background;
  - `the_gate_raised_the_quiet_soma_and_adr_0117s_cell_exhaustive` (ADR-0126): ADR-0117's cell, 64 members under the facilitating class's set (ii) at a quarter of the range, the one usable cell ADR-0115 found, reproduced bit for bit, and the quiet soma's distribution;
  - `the_gate_raised_to_0_9_the_slow_current_alone_the_core_grid_at_shift_2_exhaustive` (ADR-0126): the slow current in a wired assembly;
  - `the_gate_raised_to_0_9_with_the_facilitating_class_the_core_grid_at_shift_2_exhaustive` (ADR-0126): the slow current with the facilitating class.

  Together they cost about 1 700 s on the runners.
- **Forty leave**, renamed to `…_paused`:
  - the rest of ADR-0112's grid, ADR-0115's and ADR-0117's grids and conditions, and all of ADR-0120's;
  - ADR-0124's backgrounds, and ADR-0126's other backgrounds and cells.

  Their code, their pins and their `#[ignore]` stay. `cargo test --workspace --release --locked -- --ignored paused` runs them, and a round that reopens the line runs them first.
- **The cost table loses the forty lines**, since `npm run spec:costs` fails a line that names no whole-domain test in the tree. The remaining 71 lines are the regenerated table's own figures, 20 405 s. At six shards the deal plans about 3 400 s a shard summed, about **24 per cent** of the bound at ADR-0126's ratio. The floor is now H-20's arm from the assignment, 1 794 s alone, about 25 per cent.
- **The whitepaper's assertions follow the names**: its seven directives that name a test that leaves name it as `…_paused`, and the commands section says what a `_paused` test is.

### Consequences

- Good: the weekly job returns to about a quarter of its bound, the room the next rounds need.
- Good: nothing of any reading or rule changes, and every pinned number stands in the tree.
- Good: each mechanism the line built is still exercised weekly in a measured protocol.
- Bad: forty tests' readings are no longer reproduced weekly. A change to the engine that moves one of them, and none of the four, is caught only when the line is reopened or they are run by name.
- Neutral: the convention that a heavy run's name contains `exhaustive` (ADR-0061) gains a sibling: a paused line's heavy run ends in `_paused`.

## Alternatives considered and why rejected

- **Option 2, a feature**: the binary's gate tests are the pull request's checks of the line's rules and would leave with it.
- **Option 3, a skip list**: F-44 and F-45 are what a list beside the tree does.
- **Option 4, a seventh shard**: it would buy about eight points while the line's forty tests went on costing two thirds of the job.
- **Option 5, keep them**: the job passes the briefs' 60 per cent now, and the next round would have to shrink itself to fit around records.

## Confirmation

`scripts/exhaustive-costs.tsv` (71 lines; `npm run spec:costs`), `runtime/cortex-runtime/tests/assembly.rs` (four `…_exhaustive` and forty `…_paused` whole-domain tests), and whitepaper 4.76.0's directives and commands section. The next dispatch reads the job's shards.
