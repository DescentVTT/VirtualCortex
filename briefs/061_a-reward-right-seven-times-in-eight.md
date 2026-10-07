---
status: proposed
date: 2026-10-07
---

# Brief 061: A reward right seven times in eight — ADR-0147's feedback built, the reward's sign the outcome's in seven trials of eight and its opposite in one; then H-28 run once on H-25's configuration and schedule

## Mission

**This brief builds one feedback of the task and runs one hypothesis once.** From H-13 to H-27 every reward the
engine received was true. [ADR-0146](../docs/adr/0146-the-tag-the-address-already-is.md) named the learning
configuration as H-25's. [ADR-0147](../docs/adr/0147-a-reward-right-seven-times-in-eight.md) asks it the first task it
has not been asked: a reward that is right seven times in eight, on H-25's schedule of three reversals.

ADR-0147 wrote **H-28** before any run:
- every mapping learned;
- every coupling bounded;
- the network outside the pairs held;
- the critic holding the **expected** reward, three quarters of what a true reward would give.

When the round is done, the tree holds:
- the feedback in the task, with its own ADR and tests, and every run under the three feedbacks before it the run it
  was;
- H-28's two arms run once on H-25's configuration, as weekly `exhaustive` tests;
- H-28's verdict and **the step of its stopping rule reached**, with the next decision it names and does not take.

---

## Standing directives

- **Latest ≠ Newest** (whitepaper §2.1, `CLAUDE.md` principle 4). The one mechanism is ADR-0147's: a feedback that
  flips the reward's sign by a coin of the task's own draw. **Nothing else is adopted**:
  - no change to the critic, its window, the address, the modulator, the signed gate, STDP, the membrane or the
    inhibitory rule;
  - no second reliability and no longer schedule;
  - no dependency, and no version bump of a tool.
- **One change from H-25**: the reward's truth. The image, the delivery, the schedule and the arms are H-25's.
- **Unset, bit for bit.** Under `Feedback::Answer`, `Shuffled` and `Withheld` every run is the run it was. Every
  pinned number holds, and the determinism pin does not move. Nothing of the image or the records changes.
- **The coin is the task's own draw**: three bits of the trial's `mix64` that neither the stimulus's bit nor the
  shuffled control's reads, misleading when all three are zero. The bits are committed before the first rewarded run.
- **"Correct" is the selection, not the reward**: a trial is correct when the selection equals the stimulus's answer
  under the mapping in force, whatever reward it then received.
- **H-28's clauses and constants are written before the first rewarded run** and do not move after it. There is no
  second attempt.
- **The engine is read before a description of it is trusted**, this brief's and ADR-0147's included.
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

Re-derived on 2026-10-07 against `main` after ADR-0145, ADR-0146 and ADR-0147 merged. Line numbers move; the symbols
and the quoted sentences are what to re-derive.

1. **The feedback** (`runtime/cortex-runtime/src/task.rs`): `Feedback::{Answer, Shuffled, Withheld}`. In `Task::trial`
   the outcome's sign is `correct` under `Answer` and `coin_at(trial)` under `Shuffled`; the reward is the task's
   magnitude with that sign.
2. **The draw**: `stimulus_at(trial)` is bit 0 of `mix64(seed ^ trial)`, and `coin_at(trial)` is bit 32 of the same
   draw, *"a coin the stimulus's bit does not read"*.
3. **The outcome**: `Outcome::{correct, reward_q16, signal_q16, value_q16}`. Under the engine's critic `reward_q16` is
   the error the modulator received.
4. **H-25** ([ADR-0140](../docs/adr/0140-the-address-drawn-measured.md), `tests/inhibition.rs`):
   - `drawn_arm`, with both oracles held at every trial;
   - its pinned tables and its verdict `DRAWN_1024`;
   - the reversals at 19, 21, 16 and 20, 15, 14 blocks;
   - each stimulus's mean value within 0.12 of the reward of $2p - 1$.
5. **The harness** (`tests/instrument/harness.rs`): the composer and the network's oracle replay the consolidation from
   the train and the signal the reward left, and the harness's critic replays the engine's. Its tallies and its
   "earned" tables were written where a correct trial was a rewarded one.
6. **The signed gate** ([ADR-0094](../docs/adr/0094-the-signed-gate-built.md)): an addressed synapse consolidates under
   the dopamine signal clamped to $[-1, 1]$.
7. **The weekly job**: six shards. A round that changes `src/` dispatches `scope=both`
   ([ADR-0075](../docs/adr/0075-the-dispatch-scope-follows-the-diff.md)). After ADR-0145 the cost table plans about 47
   per cent of the bound.

## Deliverables

- [ ] **The feedback built, in a new ADR at the next free number (`ls docs/adr`).**
  - *The rule*: a feedback beside the three, under which the reward's sign is the outcome's unless the trial's coin
    is misleading, whatever the outcome was (a tie included).
  - *The coin*: three bits of the trial's draw that the stimulus and the shuffled control do not read; misleading when
    all three are zero.
  - *The tests*:
    - under the three feedbacks before it, a trial is the trial it was, bit for bit;
    - under the new one, the reward's sign is the outcome's exactly where the coin is not misleading, for each outcome
      (correct, wrong and a tie);
    - the coin reads neither the stimulus's bit nor the shuffled control's;
    - over the lattice of seeds and trials, the coin's rule against a hand rule.
- [ ] **H-28's protocol, in an ADR, before the first rewarded run.**
  - The arms: H-25's two, from H-25's image, with the new feedback. The image is asserted H-25's by its CRC.
  - The coin's count over each arm's 7 680 trials, per mapping, pinned; it is held to 880 to 1 040 in all.
  - H-28's clauses and constants, restated from ADR-0147 and pinned in the tests:
    - at least 80 of each mapping's last 128 correct;
    - no coupling above 1.30;
    - the excitatory sum outside the four couplings within 0.75 and 1.25 of the image's at every block's end;
    - each stimulus's mean value over each mapping's last 128 trials within a quarter of the reward of
      $(2p - 1) \cdot 3/4$ of it.
  - The readings, no clause:
    - the reversal speeds beside H-25's, and beside four thirds of them;
    - per mapping, the trials by outcome and by reward received: true rewards, true punishments, misleading rewards and
      misleading punishments;
    - what a misleading punishment moved in the answer's pair, and a misleading reward in the pair it reached, beside
      the true ones;
    - the value beside $(2p - 1) \cdot 3/4$ and beside $(2p - 1)$, and its troughs after each flip;
    - where the consolidation went, the couplings' separation and the inhibitory sum's course, beside H-25's.
- [ ] **The calibration**, before any rewarded run: every pinned number holds; H-25's first block reproduces under the
  true feedback; the coin's count is within its bounds.
- [ ] **The runs**: H-28's two arms, each a weekly `exhaustive` test, their tables pinned per block.
- [ ] **The ADR's reading.**
  - H-28's verdict per clause and arm.
  - The prediction (yes) against the reading.
  - The reversal speeds against the naive scaling, and what the clamp's part was.
  - **The step of the stopping rule reached, and the next decision it names, not taken.**
- [ ] **The gate.** The build's tests, and at most one runtime test for the measurement: H-28's clauses at their edges
  and the readings' rules over tables written by hand.
- [ ] **The evidence.** A weekly dispatched on this round's branch at `scope=both`. It must be green in every job, with
  no survivor of the sweep on the new lines and every pinned number reproduced. The cost table is regenerated from that
  run's artifacts.
- [ ] **The documents, in the same pull request.**
  - Whitepaper §6.5 (the loop as the runtime composes it), §11.1's H-28 with its verdict and step, and §9.
  - The ADR index, `CHANGELOG.md`, `CLAUDE.md`, and `docs/zh-TW`'s reader's guide as the result requires.
  - The whitepaper's version in both declarations, with its date
    ([ADR-0064](../docs/adr/0064-the-documentation-gate-and-the-version.md)).
- [ ] **The brief archived** as `briefs/README.md` says, every deliverable dispositioned.

## Not empowered

- **No rule of the engine changes.** The critic, its window, the address, the modulator, the signed gate, STDP, the
  membrane and the inhibitory rule are untouched. Nothing of the image or the records changes.
- The reliability is seven in eight, the schedule is H-25's, and neither is fitted to the result.
- H-28's clauses and constants do not move after a rewarded run. There is no second attempt.
- No pinned number of an earlier round moves.
- No change to `.github/workflows/ci.yml`, `scripts/exhaustive-shard.sh` or `scripts/exhaustive-costs.mjs`; the cost
  table changes only by regeneration from this round's dispatch.
- No float anywhere, no dependency, and no `unsafe` beyond ADR-0023's invariant.

## Architectural empowerment

The executing session may replace any instruction above with a better decision, recorded in its ADR. That covers:
- the feedback's name and shape, and which three bits the coin reads;
- how the harness's oracles and tallies read a trial whose reward is not its outcome's;
- how the arms are built on H-25's harness, and how they are dealt into tests;
- how each reading is computed and pinned;
- whether the round writes one ADR or two (the build and the measurement).

It may not:
- change a rule of the engine;
- change the reliability or the schedule;
- move a clause or a constant after a rewarded run;
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

Every command exits 0. The dispatched run is green and reproduces every pinned number. In the history, the feedback,
its tests, the coin's bits and H-28's constants precede the first rewarded run.

## Report

The closing message states:
- the feedback as built: the rule, the coin's bits, and that under the other feedbacks nothing moved;
- the protocol, and when it was committed;
- the calibration, with the coin's count;
- H-28's verdict per clause and arm, the prediction against it, and the readings;
- the step of the stopping rule reached and the next decision it names, not taken;
- that no pinned number moved;
- the scope ADR-0075 gave the diff, the shards' times and the regenerated cost table;
- what was not done and why.
