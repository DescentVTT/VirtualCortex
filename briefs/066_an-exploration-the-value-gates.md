---
status: proposed
date: 2026-10-09
---

# Brief 066: An exploration the value gates — ADR-0162's rule built in the task, a selection drawn among the readout's channels with a probability equal to the part of the engine's value below zero, unset every run the run it was; then H-32 run once on H-29's readouts, schedule and arms

## Mission

**This brief builds one option of the task and runs one hypothesis once.** H-29, H-30 and H-31 each read no on a
choice among three answers, and each read the same thing: after a flip the gate goes on selecting the old answer,
and the new one is selected only once the old one's pairs have been punished down to it.
[ADR-0162](../docs/adr/0162-an-exploration-the-value-gates.md) takes the decision three stopping rules name: with the
task's exploration set, a trial's selection is drawn among the readout's channels with a probability equal to the part
of the engine's own value that lies below zero. Nothing else changes.

ADR-0162 wrote **H-32** before any run: H-29's four clauses, with no prediction for the verdict.

When the round is done, the tree holds:
- the exploration in the task, with its own ADR and tests, and every run with it unset the run it was;
- H-32's two arms run once on H-29's readouts and schedule, as weekly `exhaustive` tests;
- H-32's verdict and **the step of its stopping rule reached**, with the next decision it names and does not take.

**Do not run this round beside brief 065's**: both dispatch the weekly workflow, and the account runs twenty jobs at
once.

---

## Standing directives

- **Latest ≠ Newest** (whitepaper §2.1, `CLAUDE.md` principle 4). The one mechanism is ADR-0162's: a selection that is
  sometimes drawn, in the task. **Nothing else is adopted**:
  - no change to `cortex-basal-ganglia`'s gate, the critic, its window, the address, the modulator, the signed gate,
    STDP, the membrane or the inhibitory rule;
  - no temperature, no constant share, no noise on the counts, no drive into the readouts;
  - the whole punishment of ADR-0155 stays unset, and the schedule is H-25's, not ADR-0157's;
  - no dependency, and no version bump of a tool.
- **One change from H-29**: the selection. The image, the readouts, the deal, the schedule, the seed and the arms are
  H-29's.
- **Unset, bit for bit.** With the exploration unset every run is the run it was. Every pinned number holds, and the
  determinism pin does not move. No record, nothing of the image and no format changes.
- **Set, nothing changes where the value is at or above zero.**
- **The probability is the rule's**, the part of the value below zero over the reward's magnitude. It is not fitted.
- **The coin and the draw are bits of the trial's own draw** that the stimulus, the shuffled control and the
  misleading coin do not read. The bits are committed before the first rewarded run.
- **H-32's clauses and constants are written before the first rewarded run** and do not move after it. There is no
  second attempt.
- **Literature is a prior, not a verdict.** ADR-0162 cites the account and names where the engine's premise differs,
  and predicts no verdict. The measurement decides.
- **The engine is read before a description of it is trusted**, this brief's and ADR-0162's included.
- No `f32`/`f64`, oracles included; every operation on a state field saturates or wraps by name
  ([ADR-0029](../docs/adr/0029-structural-enforcement.md)). Every loop ends by construction
  ([ADR-0062](../docs/adr/0062-the-first-complete-sweeps-list.md)).
- A heavy run is an `#[ignore]`d test whose name contains `exhaustive`. For the measurement, the runtime's gate grows by
  at most one test ([ADR-0061](../docs/adr/0061-the-learning-runs-leave-the-gate.md)); the build's own tests are beside
  it. The mutation gate on the changed lines must pass ([ADR-0030](../docs/adr/0030-verification-governance.md)).
- **The round is evidenced as `briefs/README.md`'s "Evidencing a round" says**
  ([ADR-0150](../docs/adr/0150-a-round-waits-for-what-it-checks.md),
  [ADR-0158](../docs/adr/0158-the-sweep-off-the-next-decisions-path.md)): the arms run once, the merge waits for the
  dispatch's whole-domain shards and not for its sweep, the ADRs cite the pull request's commits, the commit that asks
  for the merge sets them to `accepted`, and a sweep that has ended and that no ADR has recorded is recorded by this
  round's.
- **No shard of the weekly job passes 60 per cent of its bound** under the regenerated deal.
- No pinned number of an earlier round moves.
- Conventional Commits with a real body; never commit on `main`; the required checks keep their names.

## Context

Re-derived on 2026-10-09 against `main` after ADR-0161 and ADR-0162 merged. Line numbers move; the symbols and the
quoted sentences are what to re-derive.

1. **The selection** (`runtime/cortex-runtime/src/task.rs`): in `Task::trial` the counts come from
   `Readout::count_window` and the selection from `Readout::select`, *"the channel selected, or none where the largest
   count is shared"*. Without a hold both are read at the trial's end. `correct` is the selection compared with
   `Task::answer(stimulus)`, and the delivery's address is written from the selection.
2. **The value, before the reward** (`runtime/cortex-runtime/src/executor.rs`): `Executor::critic()` gives the
   `ValueCritic`; `Executor::features()` gives *"Each unit's spikes since the previous reward (ADR-0131), the critic's
   features, in unit order"*; a unit's `value_weight` is read from `Executor::units()`. `Executor::reward` computes
   the value as `critic.value_q16` over those pairs. With the critic's window set (100 ticks on H-25's image) the
   features are whole long before a trial's end.
3. **The draw**: `stimulus_at` reads bit 0 of `mix64(seed ^ trial)`, `coin_at` bit 32, and `misleading_at` the three
   `MISLEADING_BITS`, 48 to 50, with a compile-time assertion that they are neither of the others.
4. **The task's options**: `Task::hold`, `Task::critic` and `Task::feedback` are each unset or one of several, and
   `Task::check` refuses what a run cannot use. `Delivery::Drawn` is refused on an engine without the critic or its
   window.
5. **The outcome** (`Outcome`): `stimulus`, `counts`, `selection`, `correct`, `reward_q16`, `signal_q16`, `value_q16`
   and `held`.
6. **H-29** ([ADR-0153](../docs/adr/0153-three-answers-measured.md), `tests/inhibition.rs`): `answered_arm`,
   `answered_run` on `SCHEDULE`, `ANSWERED_ARMS`; its pins `ANSWERED_*_1024` and `ANSWERED_TRIALS_1024`; the hand rule
   of the selection, `largest_alone`, and of the answers, `answers_at`; the critic's oracle and the network's held at
   every trial.
7. **H-29's readings**: the reversals passed 40 of 64 in 31, 31 and 21 blocks from the assignment and 30, 23 and 23
   from the mirrored assignment; the value's block mean fell to −0.58 to −0.95 of the reward after a flip; the old
   answer took 62 to 90 per cent of the wrong selections; the six couplings' sum stood at 1.008 to 1.053 of the
   image's at each mapping's end.
8. **The regime ADR-0162 derives**: no exploration until the value passes zero, about 25 punished presentations after
   a flip; then a probability of three fifths while the gate still selects the old answer, the value at −0.6 of the
   reward and the new answer selected in one presentation of five; and no exploration once the gate selects the new
   answer.
9. **The weekly job**: twelve shards. H-29's two arms are in the cost table.

## Deliverables

- [ ] **The exploration built, in a new ADR at the next free number (`ls docs/adr`).**
  - *The rule*, in the task: with the exploration set, the value is read from the engine before the reward; the
    probability is the part of the value below zero over the reward's magnitude; the coin and the draw are bits of the
    trial's own draw; a drawn selection is one of the readout's channels with equal chance.
  - *What a trial records*: whether its selection was drawn, beside what it records now.
  - *The refusals*: the exploration without the engine's critic, and a reward's magnitude of zero with it.
  - *The tests*:
    - with the exploration unset, a trial is the trial it was, bit for bit;
    - with it set and the value at or above zero, the selection is the gate's at every trial;
    - over the lattice of values and draws (`testkit/prop.rs`), the probability and the drawn channel against a hand
      rule: at a value of zero, one LSB below it, at minus the reward and beyond it; each channel drawn with the same
      chance to within the draw's width, among two, three and four channels;
    - the bits read are none of the stimulus's, the shuffled control's or the misleading coin's, held at compile time;
    - under each delivery, what is addressed at a drawn selection, where the gate selected another channel and where
      it selected none.
- [ ] **H-32's protocol, in an ADR, before the first rewarded run.**
  - The arms: H-29's two, from H-29's image, on H-25's schedule, with the exploration set and nothing else. The image
    is asserted H-29's by its CRC, with the whole punishment unset.
  - H-32's clauses and constants, restated from ADR-0162 and pinned in the tests: H-29's four, by H-29's rules.
  - The predicted readings, restated as rules and never asserted: the new answer selected in more presentations than
    in H-29 over the second to the eighth block after each flip; no block mean value below −0.75 of the reward after a
    flip; every reversal faster than its own in H-29; fewer than one trial in ten drawn over each learned mapping's
    last 128; the six couplings' sum at or above 0.99 of the image's at each mapping's end.
  - The regime, restated, with what the run is read against: the probability, the value and the selections by role
    while the old answer leads.
  - The readings, no clause:
    - per block, the trials drawn, the value at them, and the drawn selections by the readouts' roles and by outcome;
    - what a reward on a drawn selection consolidated in the pair it reached, beside a reward on a selection of the
      gate's;
    - the blocks the old answer leads, and its coupling at each flip and at each mapping's end, beside H-29's;
    - the six couplings' courses and their highest; the ties and the margin of the gate's own reading;
    - the inhibitory sum's course, beside H-29's.
- [ ] **The calibration**, before any rewarded run of H-32's protocol:
  - with the exploration unset, every pinned number of the tree holds, H-29's two arms among them;
  - with it set, each arm is H-29's arm trial for trial up to the first trial at which the value is below zero and the
    coin draws, and differs from it there. The trial's index is pinned for each arm.
- [ ] **The runs**: H-32's two arms, each a weekly `exhaustive` test, run once, their tables pinned per block.
- [ ] **The ADR's reading.**
  - H-32's verdict per clause and arm.
  - Each predicted reading against what was read, and the regime against what was read.
  - What the exploration changed: the new answer's selections and the punishments of the old answer, beside H-29's.
  - **The step of the stopping rule reached, and the next decision it names, not taken.**
- [ ] **The gate.** The build's tests, and at most one runtime test for the measurement: H-32's clauses at their edges
  and the readings' rules over tables written by hand.
- [ ] **The evidence**, as "Evidencing a round" says. A weekly dispatched on this round's branch at `scope=both`. Its
  twelve whole-domain shards are green and reproduce every pinned number. The cost table is regenerated from their
  artifacts. The ADR names the dispatch and says where the sweep stood.
- [ ] **The documents, in the same pull request.**
  - Whitepaper §6.5 (the loop as the runtime composes it), §11.1's H-32 with its verdict and step, and §9.
  - The ADR index and `CHANGELOG.md`.
  - `CLAUDE.md` only where the standing changes (ADR-0150); `README.md` and `docs/zh-TW`'s reader's guide as the
    result requires.
  - The whitepaper's version in both declarations, with its date
    ([ADR-0064](../docs/adr/0064-the-documentation-gate-and-the-version.md)).
- [ ] **The brief archived** as `briefs/README.md` says, every deliverable dispositioned, and the round's ADRs set to
  `accepted` by the commit that asks for the merge.

## Not empowered

- **No rule of the engine changes**, and nothing of the executor, a state crate, a record or the image.
- The probability's form is ADR-0162's. No constant is added to it, and none is fitted.
- A drawn selection is one of all the channels with equal chance, not of the channels the gate left.
- The readouts, the deal, the schedule, the seed and the arms are H-29's, and none is fitted to the result.
- No two-answer arm is run under the exploration in this round.
- H-32's clauses and constants do not move after a rewarded run. There is no second attempt.
- No pinned number of an earlier round moves.
- No change to `.github/workflows/ci.yml`, `scripts/exhaustive-shard.sh` or `scripts/exhaustive-costs.mjs`; the cost
  table changes only by regeneration from this round's dispatch.
- No float anywhere, no dependency, and no `unsafe`.

## Architectural empowerment

The executing session may replace any instruction above with a better decision, recorded in its ADR. That covers:
- the option's name and shape in the task, and how a trial records a drawn selection;
- which bits the coin and the draw read, and how a draw of a few bits is made even among three channels;
- how the probability is compared with the coin in integers, at the reward's magnitude and at the width;
- how the harness's hand rules and oracles read a selection that is sometimes drawn;
- how the calibration's comparison with H-29's arm is made and pinned;
- how each reading is computed and pinned, and how the arms are dealt into tests;
- whether the round writes one ADR or two (the build and the measurement).

It may not:
- change the rule's form, or any rule of the engine or of a state crate;
- change the readouts, the deal, the schedule or the arms;
- move a clause or a constant after a rewarded run;
- reach the standing directives, the whitepaper's invariants or the constraints in `CLAUDE.md`.

If the calibration fails, the round stops there as H-32's stopping rule says: the build stays, the finding is written,
and no arm of the protocol is run.

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

Every command exits 0. The dispatched run's twelve whole-domain shards are green and reproduce every pinned number;
the merge does not wait for its sweep. In the history, the build, its tests, the bits the exploration reads and
H-32's constants precede the first rewarded run.

## Report

The closing message states:
- the exploration as built: the rule, the bits, where it lives, and that unset nothing moved;
- the protocol, and when it was committed;
- the calibration, with the trial at which each arm first leaves H-29's;
- H-32's verdict per clause and arm, each predicted reading and the regime against what was read, and the readings;
- the step of the stopping rule reached and the next decision it names, not taken;
- that no pinned number moved;
- the scope ADR-0075 gave the diff, the shards' times, where the sweep stood at the merge, and the regenerated cost
  table;
- what was not done and why.
