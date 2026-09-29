---
status: proposed
date: 2026-09-29
---

# Brief 055: A critic of the engine's own — ADR-0130's critic built, a value weight on every unit read from the engine's own spikes since the last reward, unset bit for bit; then H-22 run once on H-21's configuration with the task's critic replaced by the engine's

## Mission

**This brief builds one mechanism and runs one hypothesis once.** H-19, H-20 and H-21 learned with a critic that lives in
the task. It holds one expected reward per stimulus, indexed by the stimulus the task says it drew
([ADR-0107](../docs/adr/0107-the-critic-built.md)).

[ADR-0130](../docs/adr/0130-a-critic-of-the-engines-own.md) moves the critic into the engine:
- every unit carries a value weight;
- the value is the weights times each unit's spikes since the previous reward, read from the engine's own train;
- at a reward the engine forms the error, delivers it to the modulator, and moves the weights by the delta rule.

ADR-0130 wrote **H-22** before any run.

When the round is done, the tree holds:
- the critic in the rule, the executor and the image, format 19, with its own ADR and tests, and every pinned number
  standing when it is unset;
- H-22's two arms run once on H-21's configuration with the engine's critic, as weekly `exhaustive` tests;
- H-22's verdict and **the step of its stopping rule reached**, with the next decision it names and does not take.

---

## Standing directives

- **Latest ≠ Newest** (whitepaper §2.1, `CLAUDE.md` principle 4). The one mechanism is ADR-0130's: a linear value
  function over the units' spikes and the delta rule (Sutton and Barto 2018; Potjans, Morrison and Diesmann 2009), in
  Q16.16 integers. **Nothing else is adopted**:
  - no critic population;
  - no temporal-difference bootstrap across trials;
  - no change to the modulator's rule, STDP, the membrane, the task's selection or the inhibitory rule;
  - no dependency, and no version bump of a tool.
- **The engine's own inputs only.** The critic reads the executor's train and the rewards. It takes no stimulus index,
  set, geometry or window from the task.
- **Unset, bit for bit.**
  - `Executor::reward` is what it is today when the critic is unset.
  - Every pinned number of the tree holds, and the determinism pin does not move.
  - A pin of a whole image moves with the format number and nothing else, restated under a masked check
    ([ADR-0095](../docs/adr/0095-an-image-pin-moves-with-its-format.md)).
- **Axiom A3.** The weights are written between ticks, where no worker holds a record; the build shows it.
- **H-22's constants, clauses and the critic's shift and scale are written before the first rewarded run** and do not
  move after it. The shift and the scale are placed by an arithmetic written first (below).
- **The engine is read before a description of it is trusted**, this brief's and ADR-0130's included.
- No `f32`/`f64`, oracles included; every operation on a state field saturates or wraps by name
  ([ADR-0029](../docs/adr/0029-structural-enforcement.md)). Every loop ends by construction
  ([ADR-0062](../docs/adr/0062-the-first-complete-sweeps-list.md)).
- A heavy run is an `#[ignore]`d test whose name contains `exhaustive`. For the measurement, the runtime's gate grows by
  at most one test ([ADR-0061](../docs/adr/0061-the-learning-runs-leave-the-gate.md)); the build's own tests are beside
  it. The mutation gate on the changed lines must pass ([ADR-0030](../docs/adr/0030-verification-governance.md)).
- **No shard of the weekly job passes 60 per cent of its bound** under the regenerated deal.
- No pinned number of an earlier round moves, but for the whole-image pins restated above.
- Conventional Commits with a real body; never commit on `main`; the required checks keep their names.

## Context

Re-derived on 2026-09-29 against `main` after ADR-0129 and ADR-0130 merged. Line numbers move; the symbols and the
quoted sentences are what to re-derive.

1. **The task's critic** (`runtime/cortex-runtime/src/task.rs`): `Critic { expected_q16: [i32; 2], shift }`,
   `Critic::predict`, `Task::critic`, `Outcome::expected_q16`, `TaskError::ExpectationBeyondReward`. The module's own
   note: *"The critic is the task's state ... no record holds it and no image carries it."*
2. **The reward** (`runtime/cortex-runtime/src/executor.rs`): `Executor::reward(reward_prediction_error_q16)`, between
   ticks, into `self.modulator.reward`; `Executor::train`, the bounded ring of `(tick, unit)`.
3. **The record** (`crates/cortex-core/src/dynamics/neuron.rs`, `serial.rs`): `_reserved: u16` at `[52..54)`, "MUST be
   zero", read by `is_at_rest_image`. The format is 18 (ADR-0123).
4. **The image**: the modulator section's reserved bytes after ADR-0123's slow current.
5. **H-21** ([ADR-0129](../docs/adr/0129-a-target-the-network-fires-at-measured.md), `tests/inhibition.rs`):
   - H-20's configuration and arms with the target period 58 201;
   - the task's critic of shift 5;
   - its tables and readings, including the task critic's expectations per trial.
6. **The stimulus's volley**: each of a stimulus set's 51 units fires once per trial under ADR-0076's shape and cancel.
   The rest fire at about 1.7 Hz, a few hundred spikes in a trial of $2^{14}$ ticks.
7. **The weekly job**: six shards. A round that changes `src/` dispatches `scope=both`
   ([ADR-0075](../docs/adr/0075-the-dispatch-scope-follows-the-diff.md)). The cost table is regenerated from its own
   dispatch.

## Deliverables

- [ ] **The critic built, in a new ADR at the next free number (`ls docs/adr`).**
  - *The weights*: the value weight in `[52..54)`, the field renamed; `is_at_rest_image` and the serial form updated;
    format 19; whitepaper §5.2's table.
  - *The constants*: set flag, shift and scale, a parameter of the image, each refused outside what the rule resolves.
  - *The rule*: the value, the error and the step as a pure function over counts and weights, in a state crate beside
    the modulator, with its widths written first.
  - *The executor*: with the critic set, `reward` reads each unit's spikes since the previous reward from the train,
    forms the error, delivers it, and writes the weights between ticks. Unset, it is today's `reward`.
  - *The task*: refuses its own critic with the engine's set.
  - *The tests*:
    - the rule at its edges and over the lattice: saturation, a zero feature moves nothing, the step's sign and floor;
    - with the critic unset, `reward` is today's, bit for bit;
    - the executor: the features are exactly the train's spikes since the previous reward, and the weights move only
      between ticks;
    - the image: the constants written and read set and unset, each refusal, and a version-18 header refused;
    - `crates/cortex-connectome`: the version's assertions at 19;
    - the whole-image pins restated under a masked check.
- [ ] **H-22's protocol, in an ADR, before the first rewarded run.**
  - The arithmetic that places the shift and the scale: what one trial's stimulus volley alone (51 spikes) moves the
    value by, against the task critic's step of a thirty-second of the error; and what the background's spikes, about
    1.7 Hz a unit, add as noise to the value in a trial.
  - H-22's clauses and constants, restated from ADR-0130 and pinned in the tests.
  - The readings, no clause:
    - the engine's value per trial beside H-21's task critic's expectation for the same stimulus;
    - the weights by class of unit at every block's end;
    - the reversal speeds beside H-20's and H-21's;
    - the inhibitory sum's course.
- [ ] **The calibration**, before any rewarded run: every pinned number holds, and H-21's arms reproduce, with the
  critic unset.
- [ ] **The runs**: H-22's two arms, each a weekly `exhaustive` test, their tables pinned per block as H-21's.
- [ ] **The ADR's reading.**
  - H-22's verdict per clause and arm.
  - The engine's value beside the task critic's.
  - The readings.
  - **The step of the stopping rule reached, and the next decision it names, not taken.**
- [ ] **The gate.** The build's tests, and at most one runtime test for the measurement: the rule's arithmetic over a
  table written by hand, and H-22's clauses at their edges.
- [ ] **The evidence.** A weekly dispatched on this round's branch at `scope=both`. It must be green in every job, with
  no survivor of the sweep on the critic and every pinned number reproduced. The cost table is regenerated from that
  run's artifacts.
- [ ] **The documents, in the same pull request.**
  - Whitepaper §5.2's record table and the image's bytes, the format's version row, §8.8's modulator row, §11.1's H-22
    with its verdict and step, and §9.
  - The ADR index, `CHANGELOG.md`, `CLAUDE.md`, and `docs/zh-TW`'s reader's guide as the result requires.
  - The whitepaper's version in both declarations, with its date
    ([ADR-0064](../docs/adr/0064-the-documentation-gate-and-the-version.md)).
- [ ] **The brief archived** as `briefs/README.md` says, every deliverable dispositioned.

## Not empowered

- **No rule of the engine changes but the critic.** The modulator's rule, STDP, the membrane, the selection, the
  inhibitory rule and the task's reward are untouched.
- The critic reads nothing but the engine's train and its rewards.
- No critic population, no temporal-difference bootstrap, no second attempt.
- H-22's clauses, constants, shift and scale do not move after a rewarded run.
- No pinned number of an earlier round moves, but for the whole-image pins restated under a masked check.
- No change to `.github/workflows/ci.yml`, `scripts/exhaustive-shard.sh` or `scripts/exhaustive-costs.mjs`; the cost
  table changes only by regeneration from this round's dispatch.
- No float anywhere, no dependency, and no `unsafe` beyond ADR-0023's invariant.

## Architectural empowerment

The executing session may replace any instruction above with a better decision, recorded in its ADR. That covers:
- the field's name, the weight's width and scale, where the constants sit in the image, and how the loader refuses;
- which state crate holds the rule, and how the executor composes it;
- how the features are counted from the train, within "each unit's spikes since the previous reward";
- the shift and the scale, by the arithmetic written first;
- where the tests live, and how the two arms are dealt into tests;
- whether the round writes one ADR or two (the build and the measurement).

It may not give the critic an input the engine does not have, move a clause or a constant after a rewarded run, change a
rule of the engine beyond the critic, or reach the standing directives, the whitepaper's invariants or the constraints in
`CLAUDE.md`.

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

Every command exits 0. The dispatched run is green and reproduces every pinned number. The critic, its tests, the
arithmetic and H-22's constants precede the first rewarded run in the history.

## Report

The closing message states:
- the critic as built: the field, the image's bytes, the rule, the refusals, and that unset nothing moved;
- the arithmetic, the shift and the scale and why, and when they were committed;
- the calibration;
- H-22's verdict per clause and arm, the engine's value beside the task critic's, and the readings;
- the step of the stopping rule reached and the next decision it names, not taken;
- that no pinned number moved;
- the scope ADR-0075 gave the diff, the shards' times and the regenerated cost table;
- what was not done and why.
