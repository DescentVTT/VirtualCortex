---
status: archived
date: 2026-09-28
---

> **Executed 2026-09-28 in pull request #154.** Writes ADR-0123 (the slow current built) and ADR-0124 (the slow current
> measured), and opens and resolves F-57 (the slow potential a staircase). The current was built beside `integrate`, image
> format 18, unset bit for bit, the four whole-image pins re-pinned with the format under a masked check; the mutation
> gate's one survivor on the changed lines was caught by a test of the firing edge. The measurement's protocol, the
> arithmetic by the slow rule itself and the three shifts, placed by a rule written before the arithmetic was read, were
> committed and pushed before any run, with the pull request opened as a draft. ADR-0117's cell was reproduced bit for bit
> first.
>
> **No cell ran.** Fed by the prior's synapses alone, with no assembly wired, the slow current raised the members' rate from
> 1.6 Hz to 31, 11 and 4.1 Hz under arm (A) and 54, 16 and 4.3 Hz under arm (B) at shifts 0, 1 and 2: its gate at the
> drive's mean standing opens on every upward excursion and shrinks the distance a fluctuation must cover to the threshold
> by the ratio of the slow potential to its critical level. The kick's volley was full at every arm and shift, and the
> second clause of ADR-0112's measure failed at every one, at shifts 0 and 1 by the members' background alone. By the
> protocol no cell runs until the kick passes; the maintainers chose to stop the round there rather than move the measure
> after a run. **The next decision** — the gate's voltages, the input's shift beyond the grid, the slow time constant, or
> the line paused — is named and not taken.
>
> The weekly dispatched at `scope=both` (run 36418523313) was green in every job: all one hundred whole-domain tests passed, the ninety-eight before this round reproducing their pinned numbers with no unit marked, the four whole-image pins at format 18 among them; no shard's tests passed 54 per cent of its bound; the mutation sweep caught 3 544 mutants and missed none, 87 of them in the slow current's code. The cost table is regenerated from it (one hundred lines, 42 699 s), and replayed through the deal it plans each of the six shards at about 51 per cent of its bound.
>
> The body below describes the tree before execution and is not maintained. Its relative links gained one `../` when it
> moved to `archive/`; no word, claim or figure changed.

# Brief 052: A slow, voltage-gated current — the current ADR-0122 decided, built beside `integrate` and unset bit for bit; then ADR-0117's assembly measured with its members marked, alone and with the facilitating class, under ADR-0117's conditions and rules

## Mission

**This brief builds one mechanism and measures with it.** Four rounds of the rule held by the network read one
bottleneck ([ADR-0120](../../docs/adr/0120-the-contexts-own-inhibition-measured.md)): a held assembly and a quiet one
differ by the release their synapses make per spike, at most about 1.4 to 1.6 times, and every change tried acted on
both states alike.

[ADR-0122](../../docs/adr/0122-a-slow-voltage-gated-current.md) took ADR-0111's other candidate: **a slow, voltage-gated
current**. A marked unit carries a slow potential that integrates its excitatory synaptic input over tens of
milliseconds. That potential reaches the soma only as far as the soma is depolarised, so its contrast between the two
states follows the members' rate ratio, and the quiet state's background input barely reaches the soma.

When the round is done, the tree holds:
- the current in the rule, the executor and the image, format 18, with its own ADR and its tests, and every pinned
  number of the tree standing when no unit is marked;
- ADR-0117's assembly measured with its members marked, in two arms, by ADR-0117's rules;
- an ADR that records the robust cells, or what failed where, and names the next decision **without taking it**.

---

## Standing directives

- **Latest ≠ Newest** (whitepaper §2.1, `CLAUDE.md` principle 4). The one mechanism is ADR-0122's: a slow, voltage-gated
  current as the NMDA receptor's account of persistent activity gives it (Jahr and Stevens 1990; Lisman, Fellous and
  Wang 1998; Wang 1999), in Q16.16 integers. **Nothing else is adopted**:
  - no conductance-based membrane, no reversal potentials, no second gate;
  - no change to `integrate`, the short-term plasticity or its class, STDP, the modulator, the executor's schedule, the
    drive, the task or the critic;
  - no dependency, and no version bump of a tool.
- **Unset, bit for bit.**
  - `integrate` is not touched. The slow rule is written beside it, and with the slow potential at zero and no slow
    input it is `integrate` bit for bit over the lattice of `testkit/prop.rs`.
  - With no unit marked, every pinned number of the tree holds and the determinism pin does not move.
  - A pin of a whole image moves with the format number and nothing else, restated under a masked check
    ([ADR-0095](../../docs/adr/0095-an-image-pin-moves-with-its-format.md)).
- **An unmarked unit's turn costs nothing new**: no word, no allocation, no syscall, and no work but a test of its mark.
- **Every weight frozen in every measured run**, as ADR-0117 froze them (F-56 included). Every weight at the end of
  each run equals its value at the start.
- **ADR-0117's substrate, geometry, protocol, spans, release, stretch, measures, thresholds, conditions and rules
  stand.** The mark and the slow constants are the difference.
- **The grid, the slow constants, the arithmetic and the rule that picks the cells run under the conditions are written
  before any measurement run**, and none moves after one. The grid may be widened before the first run, not narrowed.
- **The arithmetic is computed first, with the slow rule itself**, and it places the three input shifts before any run.
- **The engine is read before a description of it is trusted**, this brief's and ADR-0122's included. The kick is read on
  the engine under each arm and condition before those cells run, by ADR-0112's measure.
- No `f32`/`f64`, oracles included; every operation on a state field saturates or wraps by name
  ([ADR-0029](../../docs/adr/0029-structural-enforcement.md)). Every loop ends by construction
  ([ADR-0062](../../docs/adr/0062-the-first-complete-sweeps-list.md)).
- A heavy run is an `#[ignore]`d test whose name contains `exhaustive`. For the measurement, the runtime's gate grows by
  at most one test ([ADR-0061](../../docs/adr/0061-the-learning-runs-leave-the-gate.md)); the build's own tests are beside
  it. The mutation gate on the changed lines must pass ([ADR-0030](../../docs/adr/0030-verification-governance.md)).
- **No shard of the weekly job passes 60 per cent of its bound** under the regenerated deal, six shards
  ([ADR-0121](../../docs/adr/0121-a-sixth-shard.md)). A test the cost table does not know is costed at 900 s, so the round
  deals its runs into tests of about that size or says why not.
- No pinned number of an earlier round moves, but for the whole-image pins restated above.
- Conventional Commits with a real body; never commit on `main`; the required checks keep their names.

## Context

Re-derived on 2026-09-28 against `main` after ADR-0120, ADR-0121 and ADR-0122 merged. Line numbers move; the symbols
and the quoted sentences are what to re-derive.

1. **The membrane** (`crates/cortex-core/src/dynamics/membrane.rs`, [ADR-0018](../../docs/adr/0018-membrane-integration.md)).
   - `integrate(basal_q16, apical_q16, now_tick)`: the compartments leak (`BASAL_LEAK_SHIFT` 9, `APICAL_LEAK_SHIFT`
     10) and take their inputs, which are dropped in the refractory window.
   - The soma leaks (`SOMA_LEAK_SHIFT` 11) and takes each compartment's difference `>> COUPLING_SHIFT` (4), the
     apical's `>> PLATEAU_COUPLING_SHIFT` in a plateau.
   - It fires at `v_thresh`, resets to `V_RESET` (−0.25), steps the threshold by `THRESHOLD_STEP` and starts
     `REFRACTORY_TICKS` (200).
   - `flags` bits 0 to 2: `FLAG_BURST_MODE`, `FLAG_INHIBITORY`, `FLAG_FACILITATING`; `integrate` touches only bit 0.
2. **The record** (`crates/cortex-core/src/dynamics/neuron.rs`, `serial.rs`): `_reserved_20` at `[20..24)`, "MUST be
   zero", read by `is_at_rest_image`. The format is `CortexFileHeader::FORMAT_VERSION`, 17.
3. **The executor's turn** (`runtime/cortex-runtime/src/executor.rs`).
   - The turn sums the sorted batch into `basal` and `apical` by `message_is_apical`, and marks `synaptic` by
     `message_is_synaptic`.
   - It scales both by the gain (`scaled`) and calls `u.integrate(basal, apical, now)`.
   - It calls `step_stp` or `step_stp_class` on a spike.
   - `Executor::new` holds the image's class (ADR-0114).
4. **The image** (`runtime/cortex-runtime/src/image.rs`): the modulator section, ADR-0114's class at `[32..36)`, and
   `[36..64)` reserved and refused unless zero.
5. **ADR-0117's substrate and ADR-0120's refactors** (`runtime/cortex-runtime/tests/assembly.rs`): `context_geometry`,
   `members`, `marked`, `grown`, `wire_among`, the kick, `layout`, `span_protocol_counting`, `run_050`, the conditions,
   `robust_cells`/`robust_051`, `usable_neighbours`, `round_two_051`, `failures_051`, and ADR-0117's cell
   (`CELL_050`).
6. **The drive's standing** (ADR-0112's arithmetic): the drive's mean input holds a unit at 0.875 basal and 0.436 soma,
   0.564 below the base threshold.
7. **The weekly job**: six shards since ADR-0121. A round that changes `src/` dispatches `scope=both`
   ([ADR-0075](../../docs/adr/0075-the-dispatch-scope-follows-the-diff.md)). The cost table is regenerated from its own
   dispatch.

## Deliverables

- [x] **The current built, in a new ADR at the next free number (`ls docs/adr`).**
  - *The mark*: a bit of `flags`, named, that `integrate` and the slow rule leave as they find it.
  - *The state*: the slow potential in `[20..24)`, the field renamed, `is_at_rest_image` and the serial form updated;
    format 18; whitepaper §5.2's table.
  - *The constants*: $k_s$, $g$, $V_{lo}$, $V_{hi}$, a parameter of the image in the modulator section's reserved bytes.
    The loader refuses a value out of range, $V_{lo} \ge V_{hi}$, a marked unit while the constants are unset, and the
    reserved bytes as today.
  - *The rule*: beside `integrate`, as ADR-0122 specifies: the slow potential's leak and input (taken in the refractory
    window), the gate read on the soma before the update, and the gated current added to the soma's update.
  - *The executor*: for a marked unit only, the positive efficacies of the synaptic basal messages summed and scaled by
    the gain, and the slow rule called; an unmarked unit's turn is what it was.
  - *The tests*:
    - `cortex-core`: over the lattice, the slow rule with $s = 0$ and no slow input is `integrate` bit for bit; at any
      state it is an `i64` oracle of ADR-0122's rule; at the edges, the gate at $V_{lo}$, $V_{hi}$, below and above,
      the leak's shifts at their bounds, a saturating input, and the refractory window;
    - the executor: in one run a marked unit takes only its synaptic excitatory basal input into its slow potential,
      and an unmarked unit is `integrate`'s;
    - the image: the constants written and read set and unset, each refusal, and a version-17 header refused;
    - `crates/cortex-connectome`: the version's assertions at 18;
    - the whole-image pins restated under a masked check.

  ADR-0123, `2038c6f`: `FLAG_SLOW` (bit 3), `v_slow` at `[20..24)`, `SlowCurrent` at the modulator section's `[36..48)` with `[39]` reserved, `integrate_slow` beside the untouched `integrate`, format 18. The mutation gate on the changed lines found one survivor, the firing comparison of `integrate_slow`, caught by `e7eeb61`.
- [x] **The measurement's protocol, in an ADR, before any measurement run.** At least:
  - *The arms*: (A) the members marked for the slow current under ADR-0019's short-term plasticity; (B) marked for the
    slow current and facilitating under ADR-0114's set (ii).
  - *The slow constants*: $\tau_s = 2^{13}$ ticks; $V_{lo}$ at the soma's standing under the drive's mean input and
    $V_{hi}$ at the threshold's base; three input shifts $g$ placed by the arithmetic.
  - *The grid*: recurrent weights 0.125, 0.25, 0.375 and 0.5 × the three shifts × the two arms, twenty-four cells, at
    64 members; each arm's and shift's background and control.
  - *The conditions*: every cell usable in the core run under ADR-0117's (b), (c) and (d); if more than eight are, the
    eight with the most usable neighbours in the core grid (one weight step, one shift step), ties by ADR-0117's order.
  - *The rules*, ADR-0117's: **robust** is usable under all four; **the cell for the second round** is the robust one
    with the most usable neighbours; **if none is robust**, what failed where, by `failed`.
  - *Readings, no clause*: the members' slow potential and the gate's opening per window in the held and the unkicked
    spans; the bursts; under (d), each readout's count per trial with the context held and quiet.

  ADR-0124, `d0cb546`, pushed with the pull request opened as a draft before any run.
- [x] **The arithmetic, before any measurement run, with the slow rule itself**, at the backgrounds' rates:
  - the slow potential's steady level under the background's synaptic input, and at a sustained 5, 10 and 20 Hz of
    the assembly, at each weight, shift and arm;
  - the gate's opening at the drive's mean standing and near the threshold;
  - the current each delivers to the soma beside the gap of 0.564;
  - the contrast between the held and the quiet state's slow current, beside the per-spike ratio of 1.4 to 1.6.

  The three shifts are chosen from it, committed with the reason, before any run.

  Pinned at `d0cb546` (`SLOW_CRITICAL_052`, `GATE_052`, `PER_SPIKE_052`, `ARITHMETIC_052`): the critical slow potential 1.138 of the threshold; the shifts 0, 1 and 2 by a rule written before the arithmetic was read. The slow potential reads as a staircase (F-57).
- [x] **ADR-0117's cell reproduced** with nothing marked for the slow current, bit for bit against ADR-0117's pins, before
  any cell of this round.

  First in arm (A)'s core test, held to ADR-0117's pins, pinned at `3428e1e`.
- [x] **The kick read on the engine** under each arm and condition, before those cells. It fires every member once, or
  it is derived again before any cell.

  **Rejected:** read under both arms at every shift in the core: the volley full, the second clause of ADR-0112's measure failed at every one (ADR-0124). Not derived again: at shifts 0 and 1 the members' own background alone passes the clause's mark, so no kick could pass there, and the maintainers chose on 2026-09-28 to stop the round rather than move the measure after a run. Not read under the conditions, which ran no cell.
- [x] **The runs.** Every cell, background and control. The weights shown unchanged at each run's end. The tables pinned
  as ADR-0117's: stretches in full, windows by hash.

  **Rejected:** the core's twelve backgrounds and controls and ADR-0117's cell ran, their weights unchanged at each run's end, pinned as ADR-0117's with the slow readings beside them; no cell ran, by the kick's failure and the maintainers' choice (ADR-0124).
- [x] **The ADR's reading.**
  - The core grid's table per arm, and its usable region.
  - Each condition's table for the cells run under it, and the robust cells.
  - The cell for the second round, or what failed under which condition.
  - The arithmetic beside what the runs read.
  - **The next decision named and not taken**:
    - if a cell is robust, ADR-0111's second round, a readout gated by the context, with ADR-0117's reading that the
      prior carries a held context to both readouts alike as its need;
    - if none is, what the readings name: the gate's voltages, the slow time constant, or the line paused.

  ADR-0124: the backgrounds' and the kick's tables, the slow potential and the gate over the unkicked spans, the arithmetic beside them, and the next decision named and not taken. The core grid's and the conditions' tables, the robust cells and the second round's cell are not made, since no cell ran; what failed is the kick's measure, at every arm and shift.
- [x] **The gate.** The build's tests, and at most one runtime test for the measurement: the rules at their edges over
  tables written by hand, and one marked assembly with the slow current kicked for a few hundred ticks.

  The build's tests in `cortex-core`, the executor and `tests/image.rs`, and one runtime test for the measurement (`a_slow_current_the_arms_the_grid_the_arithmetic_the_rules_and_a_marked_assembly_kicked_with_it`).
- [x] **The evidence.** A weekly dispatched on this round's branch at `scope=both`, since `src/` changes
  ([ADR-0075](../../docs/adr/0075-the-dispatch-scope-follows-the-diff.md)). It must be green in every job, with no survivor
  of the sweep on the current and every pinned number reproduced. The cost table is regenerated from that run's
  artifacts.

  Run 36418523313 at `880bcf5`, `scope=both` since `src/` changes (ADR-0075): green in every job; the whole-domain tests' six shards at 38 to 54 per cent of their bound, this round's two tests 796 and 734 s; the sweep 3 544 caught and none missed. The cost table regenerated from its artifacts: one hundred lines, 42 699 s, each shard planned at 7 116 to 7 118 s summed, about 51 per cent of the bound (ADR-0124).
- [x] **The documents, in the same pull request.**
  - Whitepaper §5.2's record table and the image's bytes, the format's version row, §8.8's membrane row, §11.1's
    question on a rule held by the network with this round's reading, and §9.
  - The ADR index, `CHANGELOG.md`, `CLAUDE.md`, and `docs/zh-TW`'s reader's guide as the result requires.
  - The whitepaper's version in both declarations, with its date
    ([ADR-0064](../../docs/adr/0064-the-documentation-gate-and-the-version.md)).
- [x] **The brief archived** as `briefs/README.md` says, every deliverable dispositioned.

## Not empowered

- **No rule of the engine changes but the slow current.** `integrate`, the short-term plasticity and its class, STDP,
  the modulator, the executor's schedule, the drive, the task and the critic are untouched.
- No weight moves during a measurement run. The assembly, the marks and the conditions are the test's own.
- No readout gated by the context, no switch on errors, no reward. Those are ADR-0111's later rounds.
- The grid, the slow constants, the arithmetic and the conditions' rule do not move after a run, and the grid is not
  narrowed.
- No pinned number of an earlier round moves, but for the whole-image pins restated under a masked check.
- No change to `.github/workflows/ci.yml`, `scripts/exhaustive-shard.sh` or `scripts/exhaustive-costs.mjs`; the cost
  table changes only by regeneration from this round's dispatch.
- No float anywhere, no dependency, and no `unsafe` beyond ADR-0023's invariant.

## Architectural empowerment

The executing session may replace any instruction above with a better decision, recorded in its ADR. That covers:
- which bit marks a unit, the field's name, where the constants sit in the image, and how the loader refuses;
- the slow rule's name and signature, and how the executor selects it;
- $V_{lo}$ within the band the arithmetic reads, and $\tau_s$ at $2^{12}$ or $2^{14}$ if the arithmetic reads $2^{13}$ as
  outside the current's purpose, each with its reason before any run;
- widening the grid: a weight between two of the four, a fourth shift, a size of 96;
- how the runs are dealt into tests for the budget, and whether ADR-0117's pinned runs are reused where a run would be
  theirs bit for bit;
- where the tests live;
- whether the round writes one ADR or two (the build and the measurement).

It may not narrow the grid, move a threshold or a rule after a run, change a rule of the engine beyond the slow
current, or reach the standing directives, the whitepaper's invariants or the constraints in `CLAUDE.md`.

## Verification

```bash
cargo check --workspace --all-targets --locked
cargo test --workspace --locked
cargo test --workspace --release --locked
cargo test --workspace --release --locked -- --ignored exhaustive --list
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --locked -- -D warnings
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --locked
cargo bench -p cortex-bench --bench hot_path --locked -- --test
cargo +1.85 check --workspace --all-targets --locked
cargo +1.85 test --workspace --locked
npm ci
npm run spec
git diff main...HEAD > target/pr.diff && cargo mutants --workspace --in-diff target/pr.diff
gh workflow run ci.yml --ref <this round's branch> -f scope=both
node scripts/exhaustive-costs.mjs from <the dispatch's downloaded artifacts> --run <its id> > scripts/exhaustive-costs.tsv
```

Every command exits 0. The dispatched run is green and reproduces every pinned number. The current, its tests and the
measurement's protocol precede the first measurement run in the history.

## Report

The closing message states:
- the current as built: the bit, the field, the image's bytes, the rule, the refusals, and that unset nothing moved;
- the arithmetic, the three shifts and why, and when they were committed;
- ADR-0117's cell reproduced;
- the kick read on the engine under each arm and condition;
- the core grid's table per arm, each condition's, the robust cells, and the cell for the second round or what failed
  where;
- the readings: the slow potential and the gate in the held and unkicked spans, the bursts, and the readouts under (d);
- that no weight moved in a measurement run and no pinned number moved;
- the scope ADR-0075 gave the diff, the shards' times and the regenerated cost table;
- what was not done and why;
- the next decision named, not taken.
