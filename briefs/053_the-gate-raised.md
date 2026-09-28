---
status: proposed
date: 2026-09-29
---

# Brief 053: The gate raised — ADR-0123's slow current with its lower gate voltage at 0.7, 0.8 and 0.9 of the threshold, its backgrounds and controls read first and the core grid run only where the kick passes, under ADR-0125's rule that pauses the line; measured, nothing of the engine changed

## Mission

**This brief measures and changes no rule.** [ADR-0124](../docs/adr/0124-the-slow-current-measured.md) read the slow
current's backgrounds 4 to 54 Hz where ADR-0117's fire at 1.6. The gate's lower voltage sat at the soma's mean, so it
opened on every upward excursion, the kick's second clause failed everywhere, and no cell ran.

[ADR-0125](../docs/adr/0125-the-gate-raised.md) takes the gate's voltages as the last try of the rule held by the
network: $V_{lo}$ at 0.7, 0.8 and 0.9 of the threshold's base, a parameter of the image. It also writes before any run
the rule that pauses the line.

When the round is done, the tree holds:
- the quiet soma's distribution on ADR-0117's background;
- the slow current's backgrounds and controls at every arm, shift and $V_{lo}$, and the core grid where the kick passes;
- an ADR that records the readings, applies ADR-0125's rule, and names the next decision **without taking it**. If the
  rule says the line pauses, that ADR says so.

---

## Standing directives

- **Latest ≠ Newest** (whitepaper §2.1, `CLAUDE.md` principle 4). This round adopts nothing:
  - no rule of the engine changes: $V_{lo}$ is a parameter of the image (ADR-0123);
  - no dependency, and no version bump of a tool.
- **Every weight frozen in every measured run**, as ADR-0117 and ADR-0124 froze them. Every weight at the end of each run
  equals its value at the start.
- **ADR-0124's substrate, arms, slow constants but $V_{lo}$, protocol, spans, release, stretch, measures, thresholds and
  kick stand.** ADR-0117's cell is reproduced with nothing marked for the slow current, bit for bit, before any other
  run.
- **ADR-0125's rule that pauses the line is applied as written.** The grid, the voltages, the rule that picks the cells
  and the arithmetic are written before any run, and none moves after one. The grid may be widened before the first
  run, not narrowed.
- **The kick is not re-derived to pass a background that fails it.** ADR-0124's measure is the kick's; where the
  background alone passes the mark, the round reads it as it is.
- **The engine is read before a description of it is trusted**, this brief's and ADR-0125's included.
- No `f32`/`f64`, oracles included; every operation on a state field saturates or wraps by name
  ([ADR-0029](../docs/adr/0029-structural-enforcement.md)). Every loop ends by construction
  ([ADR-0062](../docs/adr/0062-the-first-complete-sweeps-list.md)).
- A heavy run is an `#[ignore]`d test whose name contains `exhaustive`; the runtime's gate grows by at most one test
  ([ADR-0061](../docs/adr/0061-the-learning-runs-leave-the-gate.md)). The mutation gate on the changed lines must pass
  ([ADR-0030](../docs/adr/0030-verification-governance.md)).
- **No shard of the weekly job passes 60 per cent of its bound** under the regenerated deal
  ([ADR-0121](../docs/adr/0121-a-sixth-shard.md)). ADR-0125 read about 8 000 s left under it; a test the cost table does
  not know is costed at 900 s, so the round deals its runs into tests of about that size or says why not.
- No pinned number of an earlier round moves.
- Conventional Commits with a real body; never commit on `main`; the required checks keep their names.

## Context

Re-derived on 2026-09-29 against `main` at `0213dcb` and after ADR-0125 merged. Line numbers move; the symbols and the
quoted sentences are what to re-derive.

1. **The slow current** (`crates/cortex-core/src/dynamics/membrane.rs`, ADR-0123).
   - `SlowCurrent { leak_shift, input_shift, v_lo_q16, v_hi_q16 }`, valid when `0 < v_lo < v_hi <= THRESHOLD_BASE`.
   - `gate_q16`, `integrate_slow` and `FLAG_SLOW`.
   - The image's `[36..48)` of the modulator section.
2. **ADR-0124's substrate** (`runtime/cortex-runtime/tests/assembly.rs`):
   - `ARMS_052`, `current_052`, `SHIFTS_052`, `WEIGHTS_052` and `SIZE_052`;
   - the backgrounds' and controls' runs (`BG_RUNS_052`, `BG_READ_052`, `KICK_AFTER_052`) and the slow reading
     (`SlowRead`);
   - ADR-0117's cell and its pins;
   - the rules held at their edges (`condition_cells_052`, `neighbours_052`, `robust_052`, `round_two_052`,
     `failures_052`).
3. **The kick's measure** (ADR-0112): the volley within the tolerance, and at most a tenth of a spike a member a kick in
   the pair window after the span, a mark of 51 over eight kicks at 64 members. ADR-0124's backgrounds alone gave that
   window 42 to 562.
4. **ADR-0124's backgrounds**: 31.2, 11.1 and 4.1 Hz under (A) and 53.6, 15.7 and 4.3 Hz under (B) at shifts 0, 1 and 2,
   the gate open by 0.13 to 0.18 on average at $V_{lo}$ = 0.436.
5. **The weekly job**: six shards. A round that changes only tests dispatches `scope=exhaustive`
   ([ADR-0075](../docs/adr/0075-the-dispatch-scope-follows-the-diff.md)). The cost table is regenerated from its own
   dispatch.

## Deliverables

- [ ] **The protocol, in a new ADR at the next free number (`ls docs/adr`), before any run.** At least:
  - *The quiet soma's distribution*: on ADR-0117's background (the settled image, 64 members marked for nothing, the
    drive alone), the fraction of the members' ticks with the soma above each of 0.5, 0.6, 0.7, 0.8, 0.9 and 0.95 of
    the threshold's base, and the spikes, as a reading with no clause.
  - *The gate*: $V_{lo}$ at 0.7, 0.8 and 0.9 of the threshold's base, $V_{hi}$ at the base; $\tau_s = 2^{13}$; shifts
    0, 1 and 2; arms (A) and (B).
  - *The backgrounds and controls*: eighteen pairs, one per arm, shift and $V_{lo}$, the kick read on each control.
  - *The cells*: recurrent weights 0.125, 0.25, 0.375 and 0.5, only for the pairs whose kick passes. If more than four
    pass, the four whose backgrounds' members fire slowest run; ties to the higher $V_{lo}$, then the larger shift, then
    arm (A).
  - *The rules*: ADR-0117's holds, ignites, lets go, spills and usable, the core condition only.
  - *ADR-0125's rule that pauses the line*, restated, and which of its cases the readings reach.
- [ ] **The arithmetic, before any run, with the slow rule itself**:
  - the critical slow potential at each $V_{lo}$;
  - the gate's opening at the drive's mean standing and in the band between $V_{lo}$ and the threshold;
  - the held and the quiet states' slow potentials and their contrast at each arm, shift and weight, as ADR-0124's.
- [ ] **ADR-0117's cell reproduced**, bit for bit against its pins, before any other run.
- [ ] **The runs.** The distribution's run, the eighteen pairs, and the cells the rule picks. The weights shown unchanged
  at each run's end. The tables pinned as ADR-0124's.
- [ ] **The ADR's reading.**
  - The distribution beside the three voltages.
  - The backgrounds' rates, the gate's opening and the slow potential at each arm, shift and $V_{lo}$, beside
    ADR-0124's at 0.436.
  - The kick at each, and which pass.
  - The core grid's table for the cells that ran, and its usable region.
  - **ADR-0125's rule applied**: which case the readings reach, and the next decision it names, **not taken**. That is
    either the conditions' round on the usable cells, or an ADR pausing the line.
- [ ] **The gate.** At most one runtime test: the rule that picks the cells and ADR-0125's cases over tables written by
  hand, and a marked assembly at the highest $V_{lo}$ kicked for a few hundred ticks.
- [ ] **The evidence.** A weekly dispatched on this round's branch at `scope=exhaustive`, since only tests change. It
  must be green in every job and reproduce every pinned number. The cost table is regenerated from that run's
  artifacts.
- [ ] **The documents, in the same pull request.**
  - Whitepaper §11.1's question on a rule held by the network, with this round's reading and the rule's case, and §9.
  - The ADR index, `CHANGELOG.md`, `CLAUDE.md`, and `docs/zh-TW`'s reader's guide as the result requires.
  - The whitepaper's version in both declarations, with its date
    ([ADR-0064](../docs/adr/0064-the-documentation-gate-and-the-version.md)).
- [ ] **The brief archived** as `briefs/README.md` says, every deliverable dispositioned.

## Not empowered

- **No rule of the engine changes**, and no weight moves during a measured run.
- No conditions (b), (c) or (d) in this round; no readout gated by the context, no switch on errors, no reward.
- ADR-0125's rule that pauses the line is not amended by this round.
- The grid, the voltages, the rule that picks the cells and the arithmetic do not move after a run, and the grid is not
  narrowed.
- No pinned number of an earlier round moves.
- No change to `.github/workflows/ci.yml`, `scripts/exhaustive-shard.sh` or `scripts/exhaustive-costs.mjs`; the cost
  table changes only by regeneration from this round's dispatch.
- No float anywhere, no dependency, no `unsafe`.

## Architectural empowerment

The executing session may replace any instruction above with a better decision, recorded in its ADR. That covers:
- how the distribution is read (a histogram or the fractions above each level), and on which run;
- how the runs are dealt into tests for the budget, and whether ADR-0124's pinned runs are reused where a run would be
  theirs bit for bit;
- widening the grid before any run: a $V_{lo}$ between two of the three, or a fifth cell-running pair where the budget
  allows;
- where the tests live;
- whether the round writes one ADR or two.

It may not narrow the grid, move a threshold or a rule after a run, amend ADR-0125's rule, change a rule of the engine,
or reach the standing directives, the whitepaper's invariants or the constraints in `CLAUDE.md`.

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
run in the history.

## Report

The closing message states:
- the voltages, the grid, the rule that picks the cells and when they were committed;
- the quiet soma's distribution beside the voltages;
- the arithmetic beside the readings;
- ADR-0117's cell reproduced;
- the backgrounds, the kicks, which pass, and the core grid's table;
- which case of ADR-0125's rule the readings reach, and the next decision it names, not taken;
- that no weight moved in a measured run and no pinned number moved;
- the scope ADR-0075 gave the diff, the shards' times and the regenerated cost table;
- what was not done and why.
