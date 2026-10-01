---
status: proposed
date: 2026-10-01
---

# Brief 059: The readouts' own competition — H-26 run once on H-25's runs, reproduced bit for bit, with an open-loop shadow of the released target side beside them; nothing of the engine changed

## Mission

**This brief measures one gap, changes nothing of the engine, and runs no new configuration.**
H-25 ([ADR-0140](../docs/adr/0140-the-address-drawn-measured.md)) learned with the reward's address drawn by the
engine: its sources come from the critic's window, and its targets are the readout the selection chose, an efference
copy. The second step [ADR-0138](../docs/adr/0138-the-address-drawn.md) planned is to replace that target side with
a neural form, so that the targets could be released to every unit.

[ADR-0141](../docs/adr/0141-the-readouts-own-competition.md) begins that step by measuring. On H-25's own runs,
reproduced bit for bit, an open-loop shadow replays the stimulus–readout synapses as if the targets were every unit.
That reads what the release would cost against the learning signal the run delivered. **H-26** was written before
any run:
- its yes names the release as the next round;
- its no names the lateral competition `cortex-basal-ganglia` specifies, with the measured cost as its need.

When the round is done, the tree holds:
- the shadow beside H-25's two arms, their pins untouched, with the cost, the signal and the readings pinned per block;
- H-26's verdict and **the step of its stopping rule reached**, with the next decision it names and does not take.

---

## Standing directives

- **Latest ≠ Newest** (whitepaper §2.1, `CLAUDE.md` principle 4). The round measures with the composer's own rule and
  one address changed. **Nothing is adopted**:
  - no rule, parameter or field of the engine changes, and no file under `src/`;
  - no competition, no feedback and no released run, which are the next round's if this one names them;
  - no dependency, and no version bump of a tool.
- **H-25's runs are reproduced bit for bit.** The shadow reads the run and never writes to it. Every pinned number of
  H-25 and of the tree holds, and the determinism pin does not move.
- **The shadow is the composer's rule with one change**: the targets every unit in place of the selected readout's.
  It keeps its own traces and weights for the stimulus–readout synapses, starts from the image's, and replays them
  from the run's own train and signal.
- **H-26's rule is written before the run** and does not move after it. There is no second attempt.
- **The engine is read before a description of it is trusted**, this brief's and ADR-0141's included.
- No `f32`/`f64`, oracles included; every operation on a state field saturates or wraps by name
  ([ADR-0029](../docs/adr/0029-structural-enforcement.md)). Every loop ends by construction
  ([ADR-0062](../docs/adr/0062-the-first-complete-sweeps-list.md)).
- A heavy run is an `#[ignore]`d test whose name contains `exhaustive`. The runtime's gate grows by at most one test
  ([ADR-0061](../docs/adr/0061-the-learning-runs-leave-the-gate.md)).
- **No shard of the weekly job passes 60 per cent of its bound** under the regenerated deal.
- **No pinned number of an earlier round moves.**
- Conventional Commits with a real body; never commit on `main`; the required checks keep their names.

## Context

Re-derived on 2026-10-01 against `main` after ADR-0140 and ADR-0141 merged. Line numbers move; the symbols and the
quoted sentences are what to re-derive.

1. **The selection** (`runtime/cortex-runtime/src/task.rs`, `Readout::select`): each channel's direct drive is its own
   count in the readout window and its indirect drive the other's; `compute_gating`
   (`crates/cortex-basal-ganglia/src/lib.rs`) selects where the net output falls below zero. Whitepaper §5.2.5:
   *"Lateral competition and dopamine modulation: Specified (§8.8)"*.
2. **The drawn address** ([ADR-0139](../docs/adr/0139-the-address-drawn-built.md)): `Executor::address_drawn` and
   `Delivery::Drawn`. Its sources are the units with a nonzero critic count, and its targets the selected readout's
   units, none at a tie.
3. **The composer** (`runtime/cortex-runtime/tests/instrument/harness.rs`): it has replayed the 3 188 stimulus–readout
   synapses from the train since brief 036, held to the record at every trial. Under the drawn delivery
   (`Composer::drawn`) it consolidates a pair synapse under the signal where its source was drawn and its readout
   selected.
4. **H-25** ([ADR-0140](../docs/adr/0140-the-address-drawn-measured.md), `tests/inhibition.rs`):
   - `drawn_arm`, its pinned tables, and its verdict `DRAWN_1024`;
   - `DRAWN_WENT_1024`, where the consolidation went: the answer's pairs, the other pairs and outside, per mapping;
   - the two weekly tests `the_address_drawn_from_the_assignment_at_1024_units_exhaustive` and
     `the_address_drawn_from_the_mirrored_assignment_at_1024_units_exhaustive`.
5. **H-24** ([ADR-0137](../docs/adr/0137-the-reward-unaddressed-measured.md)): under a global delivery the other pairs
   moved as much weight as the answer's. Its sources were global too, so it does not separate the target side's cost.
6. **The weekly job**: six shards. This round changes no file under `src/`, so it dispatches `scope=exhaustive`
   ([ADR-0075](../docs/adr/0075-the-dispatch-scope-follows-the-diff.md)). After ADR-0140 the cost table plans about 46
   per cent of the bound.

## Deliverables

- [ ] **H-26's protocol, in a new ADR at the next free number (`ls docs/adr`), before the run.**
  - The shadow: the composer's rule with the targets every unit, its own traces and weights from the image's, replayed
    from the run's train and signal, never written back.
  - H-26's rule, restated from ADR-0141 and pinned in the tests:
    - **the cost of a mapping** is the shadow's consolidation on the pairs of the readout not selected, signed against
      the answer (a rise onto the other readout and a fall onto the answer both count), summed over the mapping;
    - **the signal** is the answer pairs' net less the other pairs' net under the run, summed over the mapping;
    - **H-26 holds** when the cost is at most half the signal in every mapping of both arms.
  - The readings, no clause, per block and mapping:
    - the selected readout's spikes and the other's, in the readout window and over the trial;
    - the eligibility at each reward onto each readout, from the drawn sources;
    - the cost and the signal per block, the reversal blocks apart from the learned ones;
    - the count margin the gating decided by.
- [ ] **The calibration**:
  - every pinned number holds;
  - H-25's arms reproduce their pinned tables bit for bit with the shadow beside them;
  - the shadow, run under the drawn address in place of the released one, reproduces the composer's moves.
- [ ] **The runs**: H-25's two arms with the shadow, as weekly `exhaustive` tests — riding on H-25's two, their pins
  untouched, or as two of their own — the new readings pinned per block.
- [ ] **The ADR's reading.**
  - H-26's verdict per mapping and arm.
  - The prediction (no) against it.
  - The readings.
  - **The step of the stopping rule reached, and the next decision it names, not taken.**
- [ ] **The gate.** At most one runtime test: H-26's rule at its edges over tables written by hand, and the shadow's
  rule against the composer's on a run written by hand.
- [ ] **The evidence.** A weekly dispatched on this round's branch at `scope=exhaustive`, green in every job and
  reproducing every pinned number. The cost table is regenerated from that run's artifacts.
- [ ] **The documents, in the same pull request.**
  - Whitepaper §11.1's H-26 with its verdict and step, and §9.
  - The ADR index, `CHANGELOG.md`, `CLAUDE.md`, and `docs/zh-TW`'s reader's guide as the result requires.
  - The whitepaper's version in both declarations, with its date
    ([ADR-0064](../docs/adr/0064-the-documentation-gate-and-the-version.md)).
- [ ] **The brief archived** as `briefs/README.md` says, every deliverable dispositioned.

## Not empowered

- **No rule, parameter or field of the engine changes.** No file under `src/` changes.
- The shadow never writes to the run, and H-25's pins never move.
- No released run, no competition and no feedback tag. They are the next round's, if this one names them.
- H-26's rule does not move after the run. There is no second attempt.
- No pinned number of an earlier round moves.
- No change to `.github/workflows/ci.yml`, `scripts/exhaustive-shard.sh` or `scripts/exhaustive-costs.mjs`; the cost
  table changes only by regeneration from this round's dispatch.
- No float anywhere, and no dependency.

## Architectural empowerment

The executing session may replace any instruction above with a better decision, recorded in its ADR. That covers:
- how the shadow is built on the composer, and how it is held to the composer under the drawn address;
- whether the shadow rides on H-25's two weekly tests or runs in two of its own;
- how each reading is computed and pinned.

It may not:
- change a rule of the engine;
- let the shadow write to the run;
- move H-26's rule after the run;
- reach the standing directives, the whitepaper's invariants or the constraints in `CLAUDE.md`.

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
gh workflow run ci.yml --ref <this round's branch> -f scope=exhaustive
node scripts/exhaustive-costs.mjs from <the dispatch's downloaded artifacts> --run <its id> > scripts/exhaustive-costs.tsv
```

Every command exits 0. The dispatched run is green and reproduces every pinned number. In the history, H-26's rule
precedes the run.

## Report

The closing message states:
- the shadow as built, and how it was held to the composer;
- the calibration, and that H-25's runs reproduced bit for bit;
- H-26's verdict per mapping and arm, the prediction against it, and the readings;
- the step of the stopping rule reached and the next decision it names, not taken;
- that no pinned number moved and no file under `src/` changed;
- the scope ADR-0075 gave the diff, the shards' times and the regenerated cost table;
- what was not done and why.
