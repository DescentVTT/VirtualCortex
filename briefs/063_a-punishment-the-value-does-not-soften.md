---
status: proposed
date: 2026-10-09
---

# Brief 063: A punishment the value does not soften — ADR-0154's rule built, a reward below zero delivered whole where the critic's value is below zero, a parameter of the image, unset every run the run it was; then H-30 run once on H-29's readouts, schedule and arms

## Mission

**This brief builds one rule of the critic and runs one hypothesis once.** [ADR-0153](../docs/adr/0153-three-answers-measured.md)
read H-29 as no: among three answers a reversal took 21 to 31 of a mapping's 32 blocks. It also read that after a
flip the critic's value falls below zero, so a punishment of the old answer delivers a third to an eighth of the
reward. [ADR-0154](../docs/adr/0154-a-punishment-the-value-does-not-soften.md) takes that reading: with a parameter of
the image set, a reward below zero is delivered whole where the value is below zero. Nothing else changes.

ADR-0154 wrote **H-30** before any run: H-29's four clauses, and each reversal within H-25's 23 blocks.

When the round is done, the tree holds:
- the rule in `cortex-neuromod` and the executor, the parameter in the image, with their own ADR and tests, and every
  run with the parameter unset the run it was;
- H-30's two arms run once on H-29's readouts, as weekly `exhaustive` tests;
- H-30's verdict and **the step of its stopping rule reached**, with the next decision it names and does not take.

---

## Standing directives

- **Latest ≠ Newest** (whitepaper §2.1, `CLAUDE.md` principle 4). The one mechanism is ADR-0154's: what the modulator
  receives at a reward below zero under a value below zero. **Nothing else is adopted**:
  - no change to the critic's value, its step, its window, the address, the modulator's decay, the signed gate, STDP,
    the membrane, the inhibitory rule or the selection;
  - no noise in the selection, no second form of the rule, no longer schedule;
  - no dependency, and no version bump of a tool.
- **One change from H-29.** The readouts, the deal, the image, the schedule, the seed and the arms are H-29's.
- **Unset, bit for bit.** With the parameter unset every run is the run it was. Every pinned number holds, and the
  determinism pin does not move. The whole-image pins a format moves are re-pinned with it, and nothing else is.
- **Set, nothing changes where the value is at or above zero**, and the critic's weights move by the reward less the
  value in every case.
- **H-30's clauses and constants are written before the first rewarded run** and do not move after it. There is no
  second attempt.
- **Literature is a prior, not a verdict.** ADR-0154 cites the account that predicts a yes and names where the
  engine's premise differs. The measurement decides.
- **The engine is read before a description of it is trusted**, this brief's and ADR-0154's included.
- A change to a record's bytes, reserved ones included, bumps `CortexFileHeader::version`, updates the layout in the
  whitepaper and gets a changelog entry.
- No `f32`/`f64`, oracles included; every operation on a state field saturates or wraps by name
  ([ADR-0029](../docs/adr/0029-structural-enforcement.md)). Every loop ends by construction
  ([ADR-0062](../docs/adr/0062-the-first-complete-sweeps-list.md)).
- A heavy run is an `#[ignore]`d test whose name contains `exhaustive`. For the measurement, the runtime's gate grows by
  at most one test ([ADR-0061](../docs/adr/0061-the-learning-runs-leave-the-gate.md)); the build's own tests are beside
  it. The mutation gate on the changed lines must pass ([ADR-0030](../docs/adr/0030-verification-governance.md)).
- **The round is evidenced as `briefs/README.md`'s "Evidencing a round" says**
  ([ADR-0150](../docs/adr/0150-a-round-waits-for-what-it-checks.md)): the arms run once, the merge waits for the
  dispatch's whole-domain shards and not for its sweep, the ADRs cite the pull request's commits, and the commit that
  asks for the merge sets them to `accepted`.
- **No shard of the weekly job passes 60 per cent of its bound** under the regenerated deal.
- No pinned number of an earlier round moves.
- Conventional Commits with a real body; never commit on `main`; the required checks keep their names.

## Context

Re-derived on 2026-10-09 against `main` after ADR-0152, ADR-0153 and ADR-0154 merged. Line numbers move; the symbols
and the quoted sentences are what to re-derive.

1. **The reward path** (`runtime/cortex-runtime/src/executor.rs`, `Executor::reward`): with the critic set the
   argument is the reward. The value is `critic.value_q16` over each unit's `value_weight` and its counted spikes;
   `ValueCritic::error_q16(reward_q16, value_q16)` is the error; every unit's weight moves by `critic.step` of the
   error and its count; `self.prediction` records the value and the error; and `self.modulator.reward(error_q16)` is
   what the modulator receives, *"the error the reward less the value is what the modulator receives"*.
2. **The critic's rule** (`crates/cortex-neuromod/src/lib.rs`): `ValueCritic { shift, scale }`, `value_q16`,
   `error_q16` (*"the reward less the value, saturating"*) and `step`. `NeuromodulatorState::reward` adds its argument
   to `dopamine_rpe`, saturating.
3. **The prediction** (`Prediction { value_q16, error_q16 }`): `error_q16` is documented as *"The reward less the
   value, saturating"*, and the task records it as the reward the modulator received (`Outcome::reward_q16`).
4. **The image** (`runtime/cortex-runtime/src/image.rs`): the modulator section holds the critic's flag at `[48]`
   (`CRITIC_FLAG`, `CRITIC_SET`), its shift and scale at `[49]` and `[50]`, and its window at `[52..54)`
   (`CRITIC_WINDOW`). `CortexFileHeader::FORMAT_VERSION` is 20. The loader refuses a window without a critic.
5. **The signed gate** (`crates/cortex-core/src/dynamics/synapse.rs`, `consolidate_signed`): below zero a weight moves
   against its trace by `round(|trace| × |m|)`, *"`m` clamped at −1.0"*.
6. **H-29** ([ADR-0153](../docs/adr/0153-three-answers-measured.md), `tests/inhibition.rs`): `answered_arm`,
   `answered_run` and `ANSWERED_ARMS`; the deal `DEAL_1024`; the image asserted by `WINDOWED_IMAGE_CRC_1024`; the tests
   `three_answers_from_the_{assignment,mirrored_assignment}_at_1024_units_exhaustive` and their `ANSWERED_*_1024`
   pins; the critic's oracle (`value_step` in `tests/instrument/harness.rs`) and the network's oracle held at every
   trial.
7. **H-29's readings**: the reversals passed 40 of 64 in 31, 31 and 21 blocks from the assignment and 30, 23 and 23
   from the mirrored assignment; the old answer led for 5 to 24 blocks; the value's block mean was below minus half
   the reward in 5 to 24 blocks of a mapping; the trials that selected the old answer delivered a mean error of −0.31
   to −0.67 of the reward.
8. **H-25's clause 3** (`REVERSAL_BLOCKS_MAX` 23, `reversals_within`, `CROSSING_MARK`): each reversal passes 40 of 64
   within 23 blocks of its flip, the crossing block counted.
9. **The weekly job**: twelve shards. The cost table holds 89 tests and 46 803 s; H-29's two arms are 915 and 1 041 s
   of it.

## Deliverables

- [ ] **The rule built, in a new ADR at the next free number (`ls docs/adr`).**
  - *The rule*, in `cortex-neuromod` beside `ValueCritic::error_q16`: what the modulator receives at a reward against
    a value. It is the reward less the value, unless the reward and the value are both below zero; then it is the
    reward.
  - *The executor*: with the parameter set, the modulator receives the rule's result. The critic's weights move by the
    reward less the value in every case. What a trial records as received is what the modulator received.
  - *The parameter*: a flag in the modulator section's reserved bytes, in the configuration and in the image; the
    format goes to 21. It is refused without the critic. An image of format 20 is refused, as every earlier format is.
  - *The tests*:
    - with the parameter unset, a reward is the reward it was, bit for bit, under the critic and without it;
    - over the lattice of rewards and values (`testkit/prop.rs`), the rule against a hand rule, at zero on either
      side, at the width and beyond it;
    - with the parameter set, a reward's four cases by sign of the reward and of the value: what the modulator
      receives, what the weights move by and what the prediction records;
    - the image: the flag written and read, a flag without a critic refused, the reserved bytes held to zero, and a
      format-20 image refused.
- [ ] **H-30's protocol, in an ADR, before the first rewarded run.**
  - The arms: H-29's two, from H-29's image with the parameter written into it and nothing else. The image before the
    parameter is asserted H-29's by its CRC.
  - H-30's clauses and constants, restated from ADR-0154 and pinned in the tests:
    - H-29's four, by H-29's rules;
    - each of an arm's three reversals passing 40 of 64 within 23 blocks of its flip, by H-25's rule.
  - The predicted readings, restated as rules and never asserted: the old answer leading for fewer blocks than in
    H-29 at each of the twelve flips and stimuli; every reversal faster than its own in H-29; the value below minus
    half the reward in fewer blocks; the old answer's coupling below the image's at the end of every mapping after a
    flip.
  - The readings, no clause:
    - per mapping, the punished trials whose value was below zero: how many, the error the critic took and what the
      modulator received;
    - the signal at each reward, and the trials it stood at or beyond the gate's bounds;
    - where the wrong selections went, the ties and the margin, beside H-29's;
    - the six couplings' courses, the third readout's pairs among them, beside H-29's;
    - where the consolidation went and the inhibitory sum's course, beside H-29's.
- [ ] **The calibration**, before any rewarded run of H-30's protocol:
  - with the parameter unset, every pinned number of the tree holds, H-29's two arms among them;
  - with the parameter set, each arm is H-29's arm trial for trial up to the first trial at which a reward below zero
    meets a value below zero, and differs from it there. The trial's index is pinned for each arm.
- [ ] **The runs**: H-30's two arms, each a weekly `exhaustive` test, run once, their tables pinned per block.
- [ ] **The ADR's reading.**
  - H-30's verdict per clause and arm.
  - The prediction (yes) and each predicted reading against what was read.
  - What the rule changed: the punishments it reached and what they then delivered, beside H-29's.
  - **The step of the stopping rule reached, and the next decision it names, not taken.**
- [ ] **The gate.** The build's tests, and at most one runtime test for the measurement: H-30's clauses at their edges
  and the readings' rules over tables written by hand.
- [ ] **The evidence**, as "Evidencing a round" says. A weekly dispatched on this round's branch at `scope=both`. Its
  twelve whole-domain shards are green and reproduce every pinned number. The cost table is regenerated from their
  artifacts. The ADR names the dispatch and says where the sweep stood.
- [ ] **The documents, in the same pull request.**
  - Whitepaper §5.2's and §8.7's layouts where the modulator section's bytes are listed, §6.5 (the loop as the runtime
    composes it), §11.1's H-30 with its verdict and step, and §9.
  - The ADR index and `CHANGELOG.md`.
  - `CLAUDE.md` only where the standing changes (ADR-0150): the image's format; `README.md` and `docs/zh-TW`'s
    reader's guide as the result requires.
  - The whitepaper's version in both declarations, with its date
    ([ADR-0064](../docs/adr/0064-the-documentation-gate-and-the-version.md)).
- [ ] **The brief archived** as `briefs/README.md` says, every deliverable dispositioned, and the round's ADRs set to
  `accepted` by the commit that asks for the merge.

## Not empowered

- **No other rule of the engine changes.** The critic's value and step, its window, the address, the modulator's
  decay, the signed gate, STDP, the membrane, the inhibitory rule and the selection are untouched.
- The rule's form is ADR-0154's: the reward whole where the reward and the value are both below zero. No floor at
  another level, and no change to what a reward at or above zero delivers.
- The critic's weights are not moved by what the modulator received.
- The readouts, the deal, the schedule, the seed and the arms are H-29's, and none is fitted to the result.
- H-30's clauses and constants do not move after a rewarded run. There is no second attempt.
- No two-answer arm is run under the parameter in this round.
- No pinned number of an earlier round moves, but for the whole-image pins the format moves.
- No change to `.github/workflows/ci.yml`, `scripts/exhaustive-shard.sh` or `scripts/exhaustive-costs.mjs`; the cost
  table changes only by regeneration from this round's dispatch.
- No float anywhere, no dependency, and no `unsafe` beyond ADR-0023's invariant.

## Architectural empowerment

The executing session may replace any instruction above with a better decision, recorded in its ADR. That covers:
- the rule's name and signature, and whether the prediction's record gains a field for what the modulator received;
- which reserved byte of the modulator section holds the flag, and the parameter's name in the configuration;
- how the harness's oracles replay a reward whose delivered error is not the critic's;
- how the calibration's comparison with H-29's arm is made and pinned;
- how each reading is computed and pinned, and how the arms are dealt into tests;
- whether the round writes one ADR or two (the build and the measurement).

It may not:
- change the rule's form, or any other rule of the engine or of a state crate;
- change the readouts, the deal, the schedule or the arms;
- move a clause or a constant after a rewarded run;
- reach the standing directives, the whitepaper's invariants or the constraints in `CLAUDE.md`.

If the calibration fails, the round stops there as H-30's stopping rule says: the build stays, the finding is written,
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
the merge does not wait for its sweep. In the history, the build, its tests, the parameter's place in the image and
H-30's constants precede the first rewarded run.

## Report

The closing message states:
- the rule as built: its form, where it lives, the flag's byte and the format, and that unset nothing moved;
- the protocol, and when it was committed;
- the calibration, with the trial at which each arm first leaves H-29's;
- H-30's verdict per clause and arm, the prediction and each predicted reading against what was read, and the
  readings;
- the step of the stopping rule reached and the next decision it names, not taken;
- that no pinned number moved but the whole-image pins;
- the scope ADR-0075 gave the diff, the shards' times, where the sweep stood at the merge, and the regenerated cost
  table;
- what was not done and why.
