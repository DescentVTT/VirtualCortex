---
status: proposed
date: 2026-09-30
---

# Brief 058: The address drawn — ADR-0138's drawing built, the reward's sources the units the critic's window counted and its targets the channel the engine's selection chose; then H-25 run once on H-23's configuration with the engine's own address

## Mission

**This brief builds one call and runs one hypothesis once.** Since H-14 the task has addressed the reward: the synapses
from the stimulus it presented onto the readout the engine selected. H-24
([ADR-0137](../docs/adr/0137-the-reward-unaddressed-measured.md)) read that without the address the learning does not
follow: a global reward reached the other pairs and the rest of the network as much as the answer's.

[ADR-0138](../docs/adr/0138-the-address-drawn.md) has the engine draw the address itself:
- the sources are the units whose critic window count is not zero, the state the engine was given;
- the targets are the output channel its own selection chose, an efference copy;
- so the host no longer says which stimulus it drew.

This is the first of two steps the maintainers planned. The second, a neural form of the target side, is measured
against this round's run.

ADR-0138 wrote **H-25** before any run.

When the round is done, the tree holds:
- the drawing in the executor and the task, with its own ADR and tests, and every run under the other deliveries the
  run it was;
- H-25's two arms run once on H-23's configuration with the drawn address, as weekly `exhaustive` tests;
- H-25's verdict and **the step of its stopping rule reached**, with the next decision it names and does not take.

---

## Standing directives

- **Latest ≠ Newest** (whitepaper §2.1, `CLAUDE.md` principle 4). The one mechanism is ADR-0138's: the address's sources
  written by the executor from the critic's window counts it already keeps. **Nothing else is adopted**:
  - no change to the critic, its window, the modulator, STDP, the membrane, the selection or the inhibitory rule;
  - no competition between the readouts and no feedback tag, which are the second step;
  - no dependency, and no version bump of a tool.
- **The engine's own inputs for the per-trial part.** The sources come from the executor's counts, never from the
  task's stimulus index or sets. The targets are the readout set the selection chose, the body's interface, as the
  addressed delivery's are.
- **Unset, bit for bit.** Every run under `Delivery::Global` and `Delivery::Addressed` is the run it was. Every pinned
  number of the tree holds, and the determinism pin does not move. Nothing of the image or the records changes.
- **H-25's clauses and constants are written before the first rewarded run** and do not move after it. There is no
  second attempt.
- **The engine is read before a description of it is trusted**, this brief's and ADR-0138's included.
- No `f32`/`f64`, oracles included; every operation on a state field saturates or wraps by name
  ([ADR-0029](../docs/adr/0029-structural-enforcement.md)). Every loop ends by construction
  ([ADR-0062](../docs/adr/0062-the-first-complete-sweeps-list.md)).
- A heavy run is an `#[ignore]`d test whose name contains `exhaustive`. For the measurement, the runtime's gate grows by
  at most one test ([ADR-0061](../docs/adr/0061-the-learning-runs-leave-the-gate.md)); the build's own tests are beside
  it. The mutation gate on the changed lines must pass ([ADR-0030](../docs/adr/0030-verification-governance.md)).
- **No shard of the weekly job passes 60 per cent of its bound** under the regenerated deal.
- No pinned number of an earlier round moves.
- Conventional Commits with a real body; never commit on `main`; the required checks keep their names.

## Context

Re-derived on 2026-09-30 against `main` after ADR-0137 and ADR-0138 merged. Line numbers move; the symbols and the
quoted sentences are what to re-derive.

1. **The address** (`runtime/cortex-runtime/src/executor.rs`): `Executor::address(sources, targets)` and
   `Executor::address_all`, the two flag sides in the executor's shared state, read by the fan-out
   (`Modulations::for_synapse`). An excitatory synapse whose source and target are both addressed consolidates under
   the signal; any other under the baseline alone.
2. **The delivery** (`runtime/cortex-runtime/src/task.rs`): `Delivery::{Global, Addressed}` and the match in
   `Task::trial`, written between the trial's last tick and its reward. `Delivery::Addressed`'s own documentation
   explains why the presynaptic side is narrowed: *"the reward is consolidated at the next presynaptic spike, which is
   the next presentation of a stimulus: without the narrowing the reward of one trial would reach the other stimulus's
   synapses at half the trials (ADR-0068)."*
3. **The counts** ([ADR-0131](../docs/adr/0131-the-critic-built.md), [ADR-0134](../docs/adr/0134-the-critics-window-built.md)):
   `Executor::features`, each unit's spikes within the critic's window since the previous reward, zeroed at every
   reward the critic takes. At a trial's end, before its reward, they are that trial's window.
4. **H-23** ([ADR-0135](../docs/adr/0135-the-critics-window-measured.md)): the window admitted about 52.4 spikes a trial,
   50.8 of them the presented stimulus's volley (`WINDOWED_ADMITTED_1024`); its arms, image and tables in
   `tests/inhibition.rs`.
5. **H-24** ([ADR-0137](../docs/adr/0137-the-reward-unaddressed-measured.md)):
   - `earned_run_delivered`, the harness's run under a delivery;
   - the network's oracle over all 26 240 excitatory synapses;
   - the cells, `Went`, and clause 3's band, `within_band`;
   - its readings of where the global reward's consolidation went.
6. **The weekly job**: six shards. A round that changes `src/` dispatches `scope=both`
   ([ADR-0075](../docs/adr/0075-the-dispatch-scope-follows-the-diff.md)). The cost table is regenerated from its own
   dispatch; after ADR-0137 it plans about 41 per cent of the bound.

## Deliverables

- [ ] **The drawing built, in a new ADR at the next free number (`ls docs/adr`).**
  - *The call*: between ticks, the executor sets the sources to the units whose critic count is not zero, and the
    targets to the units given. It is refused while the critic or its window is unset.
  - *The task*: a delivery beside the two that calls it with the selected readout's units, and with no targets at a
    tie.
  - *The tests*:
    - the call's sources are exactly the units with a nonzero count, at a trial's end;
    - it is refused without the critic and without the window;
    - the targets are as given, and none at a tie;
    - under `Delivery::Global` and `Delivery::Addressed` every run is the run it was, bit for bit.
- [ ] **H-25's protocol, in an ADR, before the first rewarded run.**
  - The arms: H-23's, from H-23's image, with the new delivery. The image is asserted H-23's by its CRC.
  - H-25's clauses and constants, restated from ADR-0138 and pinned in the tests:
    - at least 80 of each mapping's last 128;
    - no coupling above 1.30;
    - each reversal within 23 blocks;
    - the excitatory sum outside the four couplings within 0.75 and 1.25 of the image's at every block's end.
  - The readings, no clause:
    - the drawn sources per trial beside the host's: the presented stimulus's units in and out, and the other units
      in, by class;
    - the synapses outside the pairs that moved, by H-24's cells, and where the consolidation went, by H-24's measure;
    - the couplings' separation, the reversal speeds, the value beside $2p - 1$ and the troughs, beside H-23's;
    - the inhibitory sum's course.
- [ ] **The calibration**, before any rewarded run: every pinned number holds; in each arm, H-23's first block from
  H-23's image under the host's address reproduces H-23's tables.
- [ ] **The runs**: H-25's two arms, each a weekly `exhaustive` test, their tables pinned per block.
- [ ] **The ADR's reading.**
  - H-25's verdict per clause and arm.
  - The account's prediction (yes) against the reading.
  - The readings, with this run named as the reference the second step is measured against.
  - **The step of the stopping rule reached, and the next decision it names, not taken.**
- [ ] **The gate.** The build's tests, and at most one runtime test for the measurement: H-25's clauses at their edges
  and the readings' rules over tables written by hand.
- [ ] **The evidence.** A weekly dispatched on this round's branch at `scope=both`. It must be green in every job, with
  no survivor of the sweep on the drawing and every pinned number reproduced. The cost table is regenerated from that
  run's artifacts.
- [ ] **The documents, in the same pull request.**
  - Whitepaper §11.1's H-25 with its verdict and step, and §9.
  - The ADR index, `CHANGELOG.md`, `CLAUDE.md`, and `docs/zh-TW`'s reader's guide as the result requires.
  - The whitepaper's version in both declarations, with its date
    ([ADR-0064](../docs/adr/0064-the-documentation-gate-and-the-version.md)).
- [ ] **The brief archived** as `briefs/README.md` says, every deliverable dispositioned.

## Not empowered

- **No rule of the engine changes.** The critic, its window, the modulator, STDP, the membrane, the selection, the
  inhibitory rule and the task's reward are untouched. Nothing of the image or the records changes.
- The sources never come from the task's stimulus index or sets.
- No competition between the readouts and no feedback tag; they are the second step.
- H-25's clauses and constants do not move after a rewarded run. There is no second attempt.
- No pinned number of an earlier round moves.
- No change to `.github/workflows/ci.yml`, `scripts/exhaustive-shard.sh` or `scripts/exhaustive-costs.mjs`; the cost
  table changes only by regeneration from this round's dispatch.
- No float anywhere, no dependency, and no `unsafe` beyond ADR-0023's invariant.

## Architectural empowerment

The executing session may replace any instruction above with a better decision, recorded in its ADR. That covers:
- the call's name and signature, and how it is refused;
- the delivery's name, and how the task composes it;
- how the arms are built on H-23's and H-24's harness, and how they are dealt into tests;
- what the oracles replay, and how each reading is computed and pinned;
- whether the round writes one ADR or two (the build and the measurement).

It may not:
- draw the sources from anything but the executor's own counts;
- change a rule of the engine;
- move a clause or a constant after a rewarded run;
- change anything of H-23's configuration but the address's sources;
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
gh workflow run ci.yml --ref <this round's branch> -f scope=both
node scripts/exhaustive-costs.mjs from <the dispatch's downloaded artifacts> --run <its id> > scripts/exhaustive-costs.tsv
```

Every command exits 0. The dispatched run is green and reproduces every pinned number. In the history, the drawing, its
tests and H-25's constants precede the first rewarded run.

## Report

The closing message states:
- the drawing as built: the call, the delivery, the refusals, and that under the other deliveries nothing moved;
- the protocol, and when it was committed;
- the calibration;
- H-25's verdict per clause and arm, the account's prediction against it, and the readings;
- the step of the stopping rule reached and the next decision it names, not taken;
- that no pinned number moved;
- the scope ADR-0075 gave the diff, the shards' times and the regenerated cost table;
- what was not done and why.
