---
status: archived
date: 2026-09-27
---

> **Executed 2026-09-28 in pull request #150.** Writes ADR-0120 (the context's own inhibition measured). No rule of the
> engine changed and no pinned number moved. The inhibition's draw, the grid, the rules and the arithmetic were committed
> and pushed before any run (`f696974`), with the pull request opened as a draft. ADR-0117's cell was reproduced bit for
> bit under each of its four conditions, and every background pinned (`7e55921`), before any cell of this round ran. The
> kick fired every member once under every condition and inhibitory weight, so it was not derived again; under (b) it is
> the core control's, since an unwired assembly draws no delay and the inhibition's draw is its own seed's (ADR-0120).
>
> **No cell of the core grid is usable, so no cell is robust and there is no cell for the second round.** At a quarter of
> the range the assembly holds in at most 2 rounds of 8 in the core with the inhibition, and from three eighths up every
> cell ignites without a kick, under every condition. The inhibition lowers the members' background by 10 to 26 per cent
> and moves the assembly's threshold rather than widening the gap; its sources fire 1.4 to 2.0 times faster while the
> assembly bursts. **The next decision**, among a feedback inhibition, the context's isolation, the class's constants and
> the inhibitory drain, is named and not taken.
>
> The weekly dispatched at `scope=exhaustive` (run 36346275507) was green in every job: all ninety-eight whole-domain
> tests passed, this round's ten reproducing their tables. One shard ran at 73 per cent of its bound, having drawn
> condition (c)'s cells at the default cost; the cost table regenerated from the run plans every shard at 56 to 57 per
> cent. Whitepaper 4.70.0.
> Every deliverable is dispositioned below. Relative links gained one `../` so that they resolve from `archive/`; no
> other word, claim or figure changed.
> *The body below describes the tree before execution and is not maintained.*

# Brief 051: An inhibition of the context's own — ADR-0117's assembly of 64 marked members given sixteen added inhibitory synapses a member from the network's inhibitory units, its recurrent weight raised against them, measured under ADR-0117's four conditions for a cell that is robust; measured, nothing changed

## Mission

**This brief measures and changes no rule.** ADR-0117 read no robust cell
([ADR-0117](../../docs/adr/0117-the-region-measured.md)). The usable region under set (ii) is one weight wide, and both
the drained network and the task's stimulus pushed the usable cells over the igniting edge.

[ADR-0119](../../docs/adr/0119-an-inhibition-of-the-contexts-own.md) took the context's own inhibition. Every member
receives sixteen added inhibitory synapses from the network's inhibitory units, drawn over the ring, so that the
inhibition it meets rises when the network does. The recurrent weight is raised against it.

When the round is done, the tree holds:
- the added inhibition as the test's own wiring;
- a grid of recurrent and inhibitory weights under ADR-0117's four conditions, measured by ADR-0117's rules;
- an ADR that records the robust cells and the cell for the second round, or what failed where, and names the next
  decision **without taking it**.

---

## Standing directives

- **Latest ≠ Newest** (whitepaper §2.1, `CLAUDE.md` principle 4). This round adopts nothing:
  - no rule of the engine changes: not the membrane, the short-term plasticity or its class, STDP, the modulator, the
    executor, the drive, the task or the critic;
  - no dependency, and no version bump of a tool.

  The added inhibition is the test's own wiring, as the assembly's recurrent synapses are.
- **Dale's principle holds.** Every added synapse leaves an inhibitory unit of the prior and carries a weight at or
  below zero ([ADR-0049](../../docs/adr/0049-dale-principle-in-plasticity.md)).
- **Every weight frozen in every measured run**, as ADR-0117 froze them. The excitatory baseline is zero; the inhibitory
  baseline, the signed gate and the dopamine signal are at rest or unset (F-56); no reward. Every weight at the end of
  each run equals its value at the start, the added ones included.
- **ADR-0117's substrate, geometry, class, protocol, spans, release, stretch, measures, thresholds, conditions and rules
  stand.** Its cell of 64 units at 0.25 with no added inhibition is reproduced bit for bit under each condition, against
  ADR-0117's pins, before any other cell of that condition runs.
- **The grid, the sources' draw, the rules and the arithmetic are written before any run**, and none moves after one.
  The grid may be widened before the first run, not narrowed.
- **The engine is read before a description of it is trusted**, this brief's and ADR-0119's included. The kick is read on
  the engine under every condition and inhibitory weight before those cells run, by ADR-0112's measure.
- No `f32`/`f64`, oracles included; every operation on a state field saturates or wraps by name
  ([ADR-0029](../../docs/adr/0029-structural-enforcement.md)). Every loop ends by construction
  ([ADR-0062](../../docs/adr/0062-the-first-complete-sweeps-list.md)).
- A heavy run is an `#[ignore]`d test whose name contains `exhaustive`; the runtime's gate grows by at most one test
  ([ADR-0061](../../docs/adr/0061-the-learning-runs-leave-the-gate.md)). The mutation gate on the changed lines must pass
  ([ADR-0030](../../docs/adr/0030-verification-governance.md)).
- **No shard of the weekly job passes 60 per cent of its bound** under the regenerated deal, now five shards
  ([ADR-0118](../../docs/adr/0118-a-fifth-shard.md), [ADR-0092](../../docs/adr/0092-the-shards-dealt-by-cost.md)). A test the
  cost table does not know is costed at 900 s, so the round deals its runs into tests of about that size or says why not.
- No pinned number of an earlier round moves.
- Conventional Commits with a real body; never commit on `main`; the required checks keep their names.

## Context

Re-derived on 2026-09-27 against `main` after ADR-0117 and ADR-0118 merged. Line numbers move; the symbols and the
quoted sentences are what to re-derive.

1. **ADR-0117's substrate** (`runtime/cortex-runtime/tests/assembly.rs`):
   - the geometry (`R0_MASK_050`, `R1_MASK_050`, `context_geometry`) and 64 members at places 5 and 16 of the first 32
     periods;
   - set (ii) (`SET_050`), `marked`, `wire`, `wire_drawn`, `grown` and the kick;
   - `layout`, `span_protocol_under`, `run_050`, `backgrounds_controls_050`, `holding`, `failed` and `bursts_049`;
   - the conditions (`Condition`, `DELAY_SEED_050`, `drained_image`, `frozen_again`, `stimulus_task`, `STIMULUS_AT`);
   - the rules (`robust_cells`, `usable_neighbours`, `round_two_cell`, `failures_050`);
   - the pins `GRID_050` and `CONDITION_GRID_050`.
2. **The prior's inhibitory units**: every fifth unit from the fifth, places 4, 9, 14 and 19 of each period, carrying
   `FLAG_INHIBITORY`, their weights at the rail. Every one is a readout place of the task's geometry and of ADR-0116's.
3. **The wiring** (`crates/cortex-core/src/dynamics/synapse.rs`): `SynapseBlock::set_synapse` and `link` write a chain,
   and `Polarity::of_flags` gives a block its unit's sign. `grown` appends blocks to the image's synapse section.
4. **ADR-0117's rates.** The core grid's members fired at about 1.61 Hz and the rest at about 1.68. Under (c) the
   members fired 13 per cent above that and the rest about 13; under (d) the members 7 per cent and the rest 24.
5. **ADR-0119's grid**: recurrent weights 0.25, 0.375, 0.5 and 0.75; added inhibitory weights −0.25, −0.5 and −1.0 of
   Q1.15's range; sixteen sources a member, drawn over the ring.
6. **The weekly job**: five shards since ADR-0118; a round that changes only tests dispatches `scope=exhaustive`
   ([ADR-0075](../../docs/adr/0075-the-dispatch-scope-follows-the-diff.md)). The cost table is regenerated from its own
   dispatch.

## Deliverables

- [x] **The protocol, in a new ADR at the next free number (`ls docs/adr`), before any run.** At least:
  - *The inhibition*: for each of the 64 members, sixteen distinct inhibitory units of the prior drawn over the ring by a
    seed written here, each sending the member one basal synapse at the cell's inhibitory weight with a delay from the
    prior's local band; the added synapses chained after each source's own chain in appended blocks. The gate shows the
    wiring: sixteen sources a member, all inhibitory, all weights at or below zero, and the sources' own prior synapses
    untouched.
  - *The grid*: recurrent weights 0.25, 0.375, 0.5 and 0.75, and inhibitory weights −0.25, −0.5 and −1.0, twelve
    cells, at 64 members; beside them ADR-0117's cell, 64 units at 0.25 with no added inhibition.
  - *The conditions*: ADR-0117's core, (b), (c) and (d), each as ADR-0117 built it.
  - *The backgrounds and controls*: one of each per condition and inhibitory weight, the added inhibition wired and the
    recurrent synapses not.
  - *The rules*, ADR-0117's:
    - **robust**: usable under all four conditions;
    - **the cell for the second round**: of the robust cells, the one with the most usable neighbours in the core grid,
      one recurrent step and one inhibitory step either way; ties to the lighter recurrent weight, then the lighter
      inhibition;
    - **if none is robust**: what failed in each cell under each condition, by `failed`.
  - *Readings, no clause*:
    - the sources' rates and the members' under each condition and inhibitory weight;
    - the bursts of every cell;
    - under (d), each readout's count per trial with the context held and quiet.

  Committed at `f696974` before any run (ADR-0120). The draw is over the whole ring, a source within the prior's
  window of its member allowed and a source shared by several members allowed (option 1(a)): 203 of the 204
  inhibitory units are sources. The whole of the range is the prior's own inhibitory rail, −32 767 (option 2(b)). The
  grid was not widened.
- [x] **The arithmetic, before any run**, at the backgrounds' rates:
  - the members' mean input from the prior's excitatory and inhibitory synapses;
  - the members' mean input from the assembly at each recurrent weight, at the unkicked and at the primed product;
  - the members' mean input from the added inhibition at each inhibitory weight;
  - under (c) and (d), how much of the members' extra excitatory input the added inhibition takes back, from ADR-0117's
    rates.

  The gate's tables committed at `f696974`; the settled image's (`ARITHMETIC_051`) computed and pinned there too, before
  any run, and held by the core's test before its first run (ADR-0120).
- [x] **ADR-0117's cell reproduced** under each condition, bit for bit, before the other cells of that condition.

  Under all four conditions, in the backgrounds' three tests, pinned at `7e55921` before any cell of this round ran.
- [x] **The kick read on the engine** under each condition and inhibitory weight, before those cells. It fires every
  member once, or it is derived again before any cell.

  It fired every member once under every condition and inhibitory weight. Under (b) the kick is the core control's
  (ADR-0120, option 3(b)).
- [x] **The runs.** Every cell, background and control. The weights shown unchanged at each run's end, the added ones
  included. The tables pinned as ADR-0117's: stretches in full, windows by hash.

  Seventy runs: eighteen backgrounds and controls, four reproductions of ADR-0117's cell and forty-eight cells; no
  weight moved in any, the added synapses' included.
- [x] **The ADR's reading.**
  - The core grid's table over the recurrent and inhibitory weights, and its usable region.
  - Each condition's table and the robust cells.
  - The cell for the second round, or what failed under which condition.
  - The arithmetic beside what the runs read.
  - **The next decision named and not taken**:
    - if a cell is robust, ADR-0111's second round, a readout gated by the context, with ADR-0117's reading that the
      prior carries a held context to both readouts alike as its need;
    - if none is, the feedback inhibition (ADR-0119's option 1(b)), the context's isolation, the class's constants, or
      the inhibitory drain, with this round's readings as the need.

  No cell is robust, and none of the core grid is usable, so the second branch applies and its four options are named
  (ADR-0120); the first does not apply.
- [x] **The gate.** At most one runtime test: the added wiring's shape, the rules at their edges over tables written by
  hand, and one marked assembly with its inhibition on the new geometry kicked for a few hundred ticks.
- [x] **The evidence.** A weekly dispatched on this round's branch at `scope=exhaustive`, since only tests change
  ([ADR-0075](../../docs/adr/0075-the-dispatch-scope-follows-the-diff.md)). It must be green in every job and reproduce
  every pinned number. The cost table is regenerated from that run's artifacts.
- [x] **The documents, in the same pull request.**
  - Whitepaper §11.1's question on a rule held by the network, with this round's reading, and §9.
  - The ADR index, `CHANGELOG.md`, `CLAUDE.md`, and `docs/zh-TW`'s reader's guide as the result requires.
  - The whitepaper's version in both declarations, with its date
    ([ADR-0064](../../docs/adr/0064-the-documentation-gate-and-the-version.md)).
- [x] **The brief archived** as `briefs/README.md` says, every deliverable dispositioned.

## Not empowered

- **No rule of the engine changes**, and no weight moves during a measured run.
- No feedback inhibition, no readout gated by the context, no switch on errors, no reward in a measured run.
- ADR-0117's substrate, conditions, protocol, measures, thresholds and rules do not change; nor does any earlier test's
  constant.
- The grid, the sources' draw, the rules and the arithmetic do not move after a run, and the grid is not narrowed.
- No pinned number of an earlier round moves.
- No change to `.github/workflows/ci.yml`, `scripts/exhaustive-shard.sh` or `scripts/exhaustive-costs.mjs`; the cost
  table changes only by regeneration from this round's dispatch.
- No float anywhere, no dependency, no `unsafe`.

## Architectural empowerment

The executing session may replace any instruction above with a better decision, recorded in its ADR. That covers:
- the sources' draw: the seed, whether a member's sixteen exclude the inhibitory units within the prior's window of it,
  and whether two members may share a source;
- the delays drawn;
- how the runs are dealt into tests for the budget, and whether ADR-0117's pinned backgrounds are reused where a run
  would be theirs bit for bit;
- widening the grid: a recurrent or inhibitory weight between two of the grid's, a size of 96, or eight or thirty-two
  sources a member as a reading;
- where the tests live;
- whether the round writes one ADR or two.

It may not narrow the grid, move a threshold or a rule after a run, change a rule of the engine, or reach the standing
directives, the whitepaper's invariants or the constraints in `CLAUDE.md`.

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
run in the history, and ADR-0117's cell is reproduced under each condition before that condition's other cells.

## Report

The closing message states:
- the inhibition's wiring, the grid, the conditions and the rules, and when they were committed;
- the arithmetic beside the readings;
- ADR-0117's cell reproduced under each condition;
- the kick read on the engine under each condition and inhibitory weight;
- the core grid's table, each condition's, the robust cells, and the cell for the second round or what failed where;
- the readings: the sources' and the members' rates, the bursts, and the readouts under (d);
- that no weight moved in a measured run and no pinned number moved;
- the scope ADR-0075 gave the diff, the shards' times and the regenerated cost table;
- what was not done and why;
- the next decision named, not taken.
