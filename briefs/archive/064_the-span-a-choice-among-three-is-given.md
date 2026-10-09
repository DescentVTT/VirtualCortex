---
status: archived
date: 2026-10-09
---

> **Executed 2026-10-09 (UTC) in pull request #193.** Writes ADR-0161 (the span a choice among three is given,
> measured); opens no finding. The schedule became a parameter of a run in the harness: `Schedule`, a run's three
> flips and its trials, given to H-29's and H-30's twenty readers and to `answered_run` where they took H-20's as
> constants, H-20's held to those constants at compile time. ADR-0157's schedule is `SIZED_SCHEDULE`, 168 blocks. No
> file under `src/` changed and no pinned number moved. H-31's clauses and constants were committed before any trial
> past H-29's second flip; the tree's 91 whole-domain tests then passed; and each arm held, inside its own run at
> the end of its 56th block, its blocks to H-29's pinned first 56, every trial's reading to the hash of H-29's first
> 3 584 and its trials by brief 063's reader to H-29's.
>
> **H-31 is no, on clauses 1 and 2 in both arms and on clause 4 in one, where a yes was predicted, and its stopping
> rule reached step 5.** The first mapping and the first and the second reversals' mappings were learned in both
> arms, 118, 120 and 85, and 119, 117 and 118 of each mapping's last 128, the mapping H-29 failed among them. The
> third reversal's was not: 96 with one stimulus at exactly half of its presentations, and 72 with one at 8 of 62,
> where H-29 had learned it. An answer's coupling passed 1.30 of its image's in both arms, 1.318 and 1.320; the
> excitatory sum outside the six couplings stayed within one part in ten thousand of the image's; one stimulus's
> mean value in one mapping stood 0.31 of the reward from 2p − 1. The span bought the first reversal, 120 and 117
> where 32 blocks had given 87 and 85, and the reversals after it were slower than H-29's, 46 and 37 blocks to 40 of
> 64 where H-29's took 31 and 23. Read from the tables and not measured: across the twenty distinct flips and
> stimuli of the two rounds the old answer led longer the higher its coupling stood at the flip. The next decision,
> the ADR on an exploration that H-29's and H-30's stopping rules name, with three rounds' readings, is named and
> not taken.
>
> Every deliverable below is done and its box ticked. Three things differ from the brief's letter, each by its
> empowerment clause and recorded in ADR-0161: the calibration against H-29's first 3 584 trials is held inside
> each arm's own run rather than before it; it is held to the hash of those trials' readings as well, a pin H-29's
> own arms now hold, beside the blocks and the reader's trials the brief names; and an arm's pinned blocks are the
> 112 from H-29's second flip, the first 56 being H-29's own table. Relative links gained one `../` so that they
> resolve from `archive/`; no word, claim or figure changed.
>
> *The body below describes the tree before execution and is not maintained.*

# Brief 064: The span a choice among three is given — ADR-0157's schedule in the harness, a mapping after a flip 48 blocks where it was 32, the configuration H-29's and nothing under `src/` changed; then H-31 run once

## Mission

**This brief builds nothing in the engine and runs one hypothesis once.** [ADR-0153](../../docs/adr/0153-three-answers-measured.md)
read that H-25's configuration learns a first choice among three and revises it slowly, 21 to 31 of a mapping's 32
blocks. [ADR-0156](../../docs/adr/0156-a-punishment-the-value-does-not-soften-measured.md) read that the one change
aimed at the revision's speed gains the first reversal and loses the two after it.
[ADR-0157](../../docs/adr/0157-the-span-a-choice-among-three-is-given.md) keeps the configuration and sizes the schedule
for three answers: a mapping after a flip is 48 blocks.

ADR-0157 wrote **H-31** before any run: H-29's four clauses on the longer schedule.

When the round is done, the tree holds:
- the harness's schedule as a parameter of a run, with every run on H-25's schedule the run it was;
- H-31's two arms run once, as weekly `exhaustive` tests, each H-29's arm trial for trial for its first 3 584 trials;
- H-31's verdict and **the step of its stopping rule reached**, with the next decision it names and does not take.

---

## Standing directives

- **Latest ≠ Newest** (whitepaper §2.1, `CLAUDE.md` principle 4). Nothing is adopted: no mechanism, no parameter, no
  dependency, no version bump of a tool. **In particular**:
  - no exploration and no noise in the selection;
  - the whole punishment of ADR-0155 stays unset;
  - no change to the critic, its window, the address, the modulator, the signed gate, STDP, the membrane, the
    inhibitory rule or the selection.
- **No file under `src/` and no file of a state crate changes.** The schedule is the harness's, in `tests/`.
- **One change from H-29**: the span of a mapping after a flip. The image, the readouts, the deal, the seed, the first
  mapping and the arms are H-29's.
- **Every run on H-25's schedule is the run it was**, bit for bit. Every pinned number holds, and the determinism pin
  does not move.
- **The span is ADR-0157's, 48 blocks**, and the first mapping's is 24. Neither is fitted to what the run reads.
- **H-31's clauses and constants are written before any trial past H-29's second flip is run** and do not move after
  it. There is no second attempt.
- **Literature is a prior, not a verdict.** ADR-0157 cites the account that predicts a yes and names where the
  engine's premise differs. The measurement decides.
- **The engine is read before a description of it is trusted**, this brief's and ADR-0157's included.
- No `f32`/`f64`, oracles included; every operation on a state field saturates or wraps by name
  ([ADR-0029](../../docs/adr/0029-structural-enforcement.md)). Every loop ends by construction
  ([ADR-0062](../../docs/adr/0062-the-first-complete-sweeps-list.md)).
- A heavy run is an `#[ignore]`d test whose name contains `exhaustive`. The runtime's gate grows by at most one test
  ([ADR-0061](../../docs/adr/0061-the-learning-runs-leave-the-gate.md)). The mutation gate on the changed lines must pass
  ([ADR-0030](../../docs/adr/0030-verification-governance.md)); with no line under `src/` changed it has nothing to
  mutate, and says so.
- **The round is evidenced as `briefs/README.md`'s "Evidencing a round" says**
  ([ADR-0150](../../docs/adr/0150-a-round-waits-for-what-it-checks.md)): the arms run once, the dispatch's scope follows
  the diff, the merge waits for the whole-domain shards, the ADR cites the pull request's commits, and the commit
  that asks for the merge sets it to `accepted`.
- **No shard of the weekly job passes 60 per cent of its bound** under the regenerated deal.
- No pinned number of an earlier round moves.
- Conventional Commits with a real body; never commit on `main`; the required checks keep their names.

## Context

Re-derived on 2026-10-09 against `main` after ADR-0155, ADR-0156 and ADR-0157 merged. Line numbers move; the symbols
and the quoted sentences are what to re-derive.

1. **The schedule** (`runtime/cortex-runtime/tests/inhibition.rs`): `SCHEDULE_FLIPS`, the flips before the trials of
   index 1 536, 3 584 and 5 632, and `SCHEDULE_TRIALS`, 7 680. H-29's readers take them as constants: `answers_at`
   counts the flips at or before a trial, and `SPANS` and `MAPPINGS` give each mapping's trials and blocks.
2. **H-29's arm** (`answered_arm`, `answered_run`, `ANSWERED_ARMS`): the calibration (`answered_images`,
   `drawn_first_block`, `deal_calibration`, `answered_frozen`), then 7 680 trials with the critic's oracle and the
   network's held at every trial. Its pins are `ANSWERED_*_1024`, with its trials by brief 063's reader in
   `ANSWERED_TRIALS_1024`.
3. **H-29's rules**: `learned_by_each`, `answered_over`, `answered_left`, `holds_expected`, the verdict `answered` and
   its step `answered_step`; the readings `answered_crossings`, `by_role`, `each_crossed`, `course_by_role`,
   `went_by_mapping` and `low_high_last`.
4. **The task's flip** (`runtime/cortex-runtime/src/task.rs`, `Task::flip`) and the harness's `run_on_scheduled`
   (`tests/instrument/harness.rs`), which takes the flips as a slice.
5. **The whole punishment** ([ADR-0155](../../docs/adr/0155-a-punishment-the-value-does-not-soften-built.md)):
   `Config::whole_punishment`, a flag of the image at format 21. H-29's image carries it unset.
6. **H-29's readings** (ADR-0153): the first mapping passed 40 of 64 in its fifth and its seventh block; the reversals
   in 31, 31 and 21 blocks from the assignment and 30, 23 and 23 from the mirrored assignment; the first reversal's
   mapping read 44 and 43 correct of 64 in its last two blocks from the assignment and 42 and 43 from the mirrored
   assignment; the six couplings' sum stood at 1.008 to 1.053 of the image's at each mapping's end; the inhibitory
   sum ended at 0.843 and 0.842.
7. **The weekly job**: twelve shards. The cost table holds 91 tests and 50 038 s; H-29's two arms are 1 446 and
   1 729 s of it and H-30's 1 798 and 1 729 s. The bound of a shard is 120 minutes.
8. **The dispatch's scope** ([ADR-0075](../../docs/adr/0075-the-dispatch-scope-follows-the-diff.md)): `scope=exhaustive`
   for a round that changes no file under `src/`.

## Deliverables

- [x] **The schedule as a parameter of a run, in the tests.**
  - A run's flips and its length are given to the readers that took `SCHEDULE_FLIPS` and `SCHEDULE_TRIALS` as
    constants, or readers for the longer schedule are written beside them. Either way H-29's and H-30's arms read
    what they read, table for table.
  - ADR-0157's schedule: the flips before the trials of index 1 536, 4 608 and 7 680; 168 blocks, 10 752 trials.
- [x] **H-31's protocol, in a new ADR at the next free number (`ls docs/adr`), before any trial past H-29's second
  flip.**
  - The arms: H-29's two, from H-29's image, with H-29's readouts and deal and the whole punishment unset. The image
    is asserted H-29's by its CRC.
  - H-31's clauses and constants, restated from ADR-0157 and pinned in the tests: H-29's four, by H-29's rules, over
    each mapping's last 128 trials.
  - The predicted readings, restated as rules and never asserted:
    - the mirrored arm's second mapping learned by each stimulus;
    - every reversal passing 40 of 64 within 35 blocks of its flip;
    - the six couplings' sum at or above the image's at every mapping's end;
    - the inhibitory sum ending between 0.75 and 0.80 of the image's.
  - The readings, no clause:
    - the block each mapping passes 40 of 64 in, and each stimulus's, beside H-29's;
    - each mapping's correct trials by block from its 32nd to its 48th;
    - where the wrong selections went, the ties and the margin, by mapping;
    - the six couplings' courses and their sum, and the value's troughs;
    - the inhibitory sum's course, beside H-21's bar of a half.
- [x] **The calibration**, before any trial past H-29's second flip:
  - every pinned number of the tree holds;
  - each arm is H-29's arm trial for trial for its first 3 584 trials, held to `ANSWERED_TRIALS_1024` and to H-29's
    pinned blocks.
- [x] **The runs**: H-31's two arms, each a weekly `exhaustive` test, run once, their tables pinned per block.
- [x] **The ADR's reading.**
  - H-31's verdict per clause and arm.
  - The prediction (yes) and each predicted reading against what was read.
  - What the added span bought: each mapping at its 32nd block beside its 48th.
  - **The step of the stopping rule reached, and the next decision it names, not taken.**
- [x] **The gate.** At most one runtime test: the schedule's hand rule, H-31's clauses at their edges and the
  readings' rules over tables written by hand.
- [x] **The evidence**, as "Evidencing a round" says. A weekly dispatched on this round's branch at
  `scope=exhaustive`. Its twelve whole-domain shards are green and reproduce every pinned number. The cost table is
  regenerated from their artifacts. The ADR names the dispatch.
- [x] **The documents, in the same pull request.**
  - Whitepaper §11.1's H-31 with its verdict and step, and §9.
  - The ADR index and `CHANGELOG.md`.
  - `CLAUDE.md` only where the standing changes (ADR-0150); `README.md` and `docs/zh-TW`'s reader's guide as the
    result requires.
  - The whitepaper's version in both declarations, with its date
    ([ADR-0064](../../docs/adr/0064-the-documentation-gate-and-the-version.md)).
- [x] **The brief archived** as `briefs/README.md` says, every deliverable dispositioned, and the round's ADR set to
  `accepted` by the commit that asks for the merge.

## Not empowered

- **No file under `src/` and no file of a state crate changes.** No rule, no record, no format.
- No exploration, no noise, no change to the selection, and the whole punishment stays unset.
- The span is 48 blocks and the first mapping's 24. The reversals are three. None is fitted to the result.
- The readouts, the deal, the image, the seed and the arms are H-29's.
- H-31's clauses and constants do not move after a trial past H-29's second flip. There is no second attempt.
- No pinned number of an earlier round moves.
- No change to `.github/workflows/ci.yml`, `scripts/exhaustive-shard.sh` or `scripts/exhaustive-costs.mjs`; the cost
  table changes only by regeneration from this round's dispatch.
- No float anywhere, no dependency, and no `unsafe`.

## Architectural empowerment

The executing session may replace any instruction above with a better decision, recorded in its ADR. That covers:
- how the schedule becomes a parameter of H-29's readers, or which readers are written beside them;
- how the calibration's comparison with H-29's first 3 584 trials is made and pinned;
- how each reading is computed and pinned, and how the arms are dealt into tests;
- whether an arm is one weekly test or is split so that no single test outruns what a shard's deal can balance, as
  long as the trials run are one run of 10 752.

It may not:
- change a file under `src/` or of a state crate;
- change the span, the number of reversals, the configuration or the arms;
- move a clause or a constant after a trial past H-29's second flip;
- reach the standing directives, the whitepaper's invariants or the constraints in `CLAUDE.md`.

If the calibration fails, the round stops there as H-31's stopping rule says: the finding is written, and no trial
past H-29's second flip is run.

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

Every command exits 0. The dispatched run's twelve whole-domain shards are green and reproduce every pinned number.
In the history, the schedule and H-31's constants precede any trial past H-29's second flip.

## Report

The closing message states:
- the schedule as built in the harness, and that every run on H-25's schedule read what it read;
- the protocol, and when it was committed;
- the calibration: the pins, and each arm against H-29's first 3 584 trials;
- H-31's verdict per clause and arm, the prediction and each predicted reading against what was read, and the
  readings;
- the step of the stopping rule reached and the next decision it names, not taken;
- that no pinned number moved and no file under `src/` changed;
- the scope ADR-0075 gave the diff, the shards' times and the regenerated cost table;
- what was not done and why.
