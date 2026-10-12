---
status: proposed
date: 2026-10-12
---

# Brief 069: A reward right three times in four — ADR-0170's feedback built, the reward's sign the outcome's in three trials of four and its opposite in one, by two of the misleading coin's three bits; then H-35 run once on H-25's configuration, network and schedule

## Mission

**This brief builds one feedback of the task and runs one hypothesis once.** H-28 read the learning configuration
under a reward right seven times in eight, and it learned and revised
([ADR-0149](../docs/adr/0149-a-reward-right-seven-times-in-eight-measured.md)).
[ADR-0170](../docs/adr/0170-a-reward-right-three-times-in-four.md) asks it of a less reliable reward: right three
times in four, on H-25's configuration, network, schedule and arms.

ADR-0170 wrote **H-35** before any run:
- every mapping learned;
- every coupling bounded;
- the network outside the pairs held;
- the critic holding the **expected** reward, a half of what a true reward would give.

It predicts no verdict: the naive scaling says the schedule does not hold the revision, and H-28's reading carried in
proportion says it does, at its edge. It predicts three readings, and it splits a no on the learning beforehand
between slower than the schedule and not revised.

When the round is done, the tree holds:
- the feedback in the task, with its own ADR and tests, and every run under the four feedbacks before it the run it
  was;
- H-35's two arms run once on H-25's configuration, as weekly `exhaustive` tests;
- H-35's verdict and **the step of its stopping rule reached**, the branch of step 5 among them where it applies, with
  the next decision it names and does not take.

**Do not run this round beside brief 065's**: both dispatch the weekly workflow.

---

## Standing directives

- **Latest ≠ Newest** (whitepaper §2.1, `CLAUDE.md` principle 4). The one mechanism is ADR-0170's: a feedback that
  flips the reward's sign by a coin of the task's own draw, at one trial in four. **Nothing else is adopted**:
  - no change to the critic, its window, the address, the modulator, the signed gate, STDP, the membrane or the
    inhibitory rule;
  - no second reliability and no longer schedule;
  - the whole punishment of ADR-0155 and the exploration of ADR-0163 stay unset;
  - no dependency, and no version bump of a tool.
- **One change from H-28**: the coin's share. The image, the delivery, the schedule, the arms and the network, seed
  22 at 1 024 units, are H-25's.
- **Unset, bit for bit.** Under `Feedback::Answer`, `Shuffled`, `Withheld` and `SevenInEight` every run is the run it
  was. Every pinned number holds, H-28's two arms among them, and the determinism pin does not move. Nothing of the
  image or the records changes.
- **The coin is bits 48 and 49 of the trial's draw**, two of the three `MISLEADING_BITS`: misleading when both are
  zero, whatever bit 50 is. Every trial misleading at seven in eight is misleading here. The bits are ADR-0170's and
  are committed before the first rewarded run.
- **"Correct" is the selection, not the reward**: a trial is correct when the selection equals the stimulus's answer
  under the mapping in force, whatever reward it then received.
- **H-35's clauses and constants, its three predicted readings and the rule that splits step 5 are written before
  the first rewarded run** and do not move after it. There is no second attempt.
- **A prediction of the verdict is not owed.** ADR-0170 writes none; this round does not add one. Each of the three
  predicted readings is reported against what was read, a miss as a miss.
- **The engine and the harness are read before a description of them is trusted**, this brief's and ADR-0170's
  included.
- No `f32`/`f64`, oracles included; every operation on a state field saturates or wraps by name
  ([ADR-0029](../docs/adr/0029-structural-enforcement.md)). Every loop ends by construction
  ([ADR-0062](../docs/adr/0062-the-first-complete-sweeps-list.md)).
- A heavy run is an `#[ignore]`d test whose name contains `exhaustive`. For the measurement, the runtime's gate grows
  by at most one test ([ADR-0061](../docs/adr/0061-the-learning-runs-leave-the-gate.md)); the build's own tests are
  beside it. The mutation gate on the changed lines must pass
  ([ADR-0030](../docs/adr/0030-verification-governance.md)).
- **The round is evidenced as `briefs/README.md`'s "Evidencing a round" says**
  ([ADR-0150](../docs/adr/0150-a-round-waits-for-what-it-checks.md),
  [ADR-0158](../docs/adr/0158-the-sweep-off-the-next-decisions-path.md)): the arms run once, the dispatch's scope
  follows the diff, the merge waits for the whole-domain shards and not for the sweep, the ADR cites the pull request's
  commits, the commit that asks for the merge sets it to `accepted`, and a sweep that has ended and that no ADR has
  recorded is recorded by this round's.
- **The pull request leaves draft with the commit that asks for the merge.** A reviewer's gate waits while it is a
  draft.
- **No shard of the weekly job passes 60 per cent of its bound** under the regenerated deal, but a shard that holds
  one of H-34's arms, which is half of the bound by itself; such a shard is reported with the arm's seconds.
- No pinned number of an earlier round moves.
- Conventional Commits with a real body; never commit on `main`; the required checks keep their names.

## Context

Re-derived on 2026-10-12 against `main` at `85dc6b2`, after ADR-0169 merged. Line numbers move; the symbols and the
quoted sentences are what to re-derive.

1. **The feedback** (`runtime/cortex-runtime/src/task.rs`): `Feedback::{Answer, Shuffled, Withheld, SevenInEight}`.
   In `Task::trial` the reward's sign is `correct` under `Answer`, `coin_at(trial)` under `Shuffled` and
   `correct != self.misleading_at(trial)` under `SevenInEight`.
2. **The draw**: one `mix64(seed ^ trial)`. Bit 0 is the stimulus, bit 32 the shuffled control's coin, bits 16 to 31
   the exploration's coin and 33 to 47 its channel. `MISLEADING_BITS` is `0x0007_0000_0000_0000`, bits 48, 49 and 50,
   and `misleading_at` is *"the three `MISLEADING_BITS` of the same draw all zero, one trial in eight"*. Compile-time
   assertions hold the fields apart.
3. **The build's tests at seven in eight** (`task.rs`, ADR-0148): the coin against an oracle written apart from the
   tree over the first sixty-four trials at seeds 0 and 27; its share over 4 096 trials by stimulus and by the
   shuffled coin, 513 in all; the reward's sign the outcome's exactly where the coin is not misleading.
4. **H-28's harness** (`tests/inhibition.rs`): `MISLED_ARMS`, `MISLED_FEEDBACK`, `RELIABILITY` (3, 4),
   `MISLEADING_BOUNDS` (880, 1 040), `MISLEADING_BY_MAPPING_1024` (181, 262, 250, 256; 949 in all),
   `holds_expected_reward`, `reward_kind`, `beside_expected`, `nearer_expected`, `speeds`, `misled_arm`, `misled`,
   `misled_step`, `MisledRead`, the pinned tables `MISLED_*_1024` and the verdict `MISLED_1024`;
   `tests/instrument/harness.rs`: `misleads(seed, trial)`.
5. **What H-28 read** (ADR-0149): each mapping's last 128 correct 123, 113, 122, 119 and 124, 107, 122, 114; the
   highest coupling 1.181 and 1.187; the reversals at 19, 19, 20 and 24, 14, 21 blocks, where H-25's took 19, 21, 16
   and 20, 15, 14, a mean ratio of 1.13; a true reward's error 0.30 to 0.52 of the reward over a learned mapping's last
   128 trials; a misleading punishment taking 1.9 and 2.0 times what a true reward gave, the clamp passing on no more
   than −1 in 68 per cent of them; the learning signal 0.87 to 1.03 of H-25's.
6. **H-25's schedule** (`SCHEDULE`): 7 680 trials, the first mapping 24 blocks of 64 and each of the three after a
   flip 32.
7. **The signed gate** ([ADR-0094](../docs/adr/0094-the-signed-gate-built.md)): an addressed synapse consolidates
   under the dopamine signal clamped to $[-1, 1]$.
8. **The weekly job**: twelve shards of 120 minutes, each running its tests two at a time. The cost table holds 108
   tests and 79 490 s; H-28's two arms read 1 822 and 1 823 s; H-34's four arms at 2 048 units read 2 151 to 3 580 s.
9. **The dispatch's scope** ([ADR-0075](../docs/adr/0075-the-dispatch-scope-follows-the-diff.md)): a round that
   changes a file under `src/` dispatches `scope=both`. The sweep's thirteen jobs start when the shards have ended
   ([ADR-0166](../docs/adr/0166-the-sweep-after-the-shards.md)).
10. **A sweep to look for**: the schedule's weekly run on `main` of 2026-10-12 had not started when ADR-0170 was
    written. If its sweep has ended when this round's ADR is written, that ADR records it: the artifacts
    `mutants-out-crates` and `mutants-out-runtime-0` to `-11`, each `caught.txt`, `missed.txt`, `timeout.txt` and
    `unviable.txt` counted by its lines.

## Deliverables

- [ ] **The feedback built, in a new ADR at the next free number (`ls docs/adr`).**
  - *The rule*: a feedback beside the four, under which the reward's sign is the outcome's unless the trial's coin is
    misleading, whatever the outcome was (a tie included).
  - *The coin*: bits 48 and 49 of the trial's draw, both zero; bit 50 not read.
  - *The tests*:
    - under the four feedbacks before it, a trial is the trial it was, bit for bit;
    - under the new one, the reward's sign is the outcome's exactly where the coin is not misleading, for each outcome
      (correct, wrong and a tie);
    - the coin reads neither the stimulus's bit, the shuffled control's nor the exploration's;
    - the coin's rule against an oracle written apart from the tree, and its share over 4 096 trials;
    - **nested**: over the lattice of seeds and trials, every trial misleading at seven in eight is misleading here,
      and the trials misleading here and not there are those whose bit 50 is set.
- [ ] **H-35's protocol, in an ADR, before the first rewarded run.**
  - The arms: H-25's two, from H-25's image, with the new feedback. The image is asserted H-25's by its CRC.
  - The coin's count over each arm's 7 680 trials, per mapping, pinned; it is held to 1 760 to 2 080 in all.
  - H-35's clauses and constants, restated from ADR-0170 and pinned in the tests:
    - at least 80 of each mapping's last 128 correct;
    - no coupling above 1.30;
    - the excitatory sum outside the four couplings within 0.75 and 1.25 of the image's at every block's end;
    - each stimulus's mean value over each mapping's last 128 trials within a quarter of the reward of
      $(2p - 1) \cdot 1/2$ of it.
  - The three predicted readings as rules over the tables: the first mapping learned in both arms; the mean of the six
    reversals' ratios to H-25's above 1.13, a reversal that does not cross counted at 32 blocks; the value nearer a
    half of $2p - 1$ than $2p - 1$ in each mapping clause 1 holds for.
  - **The rule that splits step 5**, as ADR-0170 writes it: (a) when clause 2 holds, the first mapping is learned in
    both arms and each of the six reversals reaches a block with 40 of 64 correct inside its 32; (b) otherwise.
  - The readings, no clause, as ADR-0170 lists them, each beside H-28's and H-25's.
- [ ] **The calibration**, before any rewarded run: every pinned number holds, H-28's two arms under `SevenInEight`
  among them; H-25's first block reproduces under the true feedback; the coin's count is within its bounds; every
  trial of the 7 680 misleading at seven in eight is misleading here.
- [ ] **The runs**: H-35's two arms, each a weekly `exhaustive` test, run once, their tables pinned per block.
- [ ] **The ADR's reading.**
  - H-35's verdict per clause and arm.
  - Each of the three predicted readings against what was read.
  - The reversal speeds beside H-25's, H-28's, twice H-25's and 1.39 of them: which account the run stands nearer.
  - What a misleading punishment moved beside a true reward, what a true reward's error was, how often the clamp
    acted, and the share of the true rewards' gain that misleading punishments took back, beside H-28's 27 and 28 per
    cent.
  - **The step of the stopping rule reached, with the branch of step 5 where it applies, and the next decision it
    names, not taken.**
- [ ] **The gate.** The build's tests, and at most one runtime test for the measurement: H-35's clauses at their
  edges, the three predicted readings' rules and step 5's split over tables written by hand.
- [ ] **The evidence**, as "Evidencing a round" says. A weekly dispatched on this round's branch at `scope=both`. Its
  twelve whole-domain shards are green and reproduce every pinned number. The cost table is regenerated from their
  artifacts. The ADR names the dispatch and each shard's wall time against its bound. The sweep's outcome is not
  waited for.
- [ ] **The documents, in the same pull request.**
  - Whitepaper §6.5 where the loop's description names the feedbacks, §11.1's H-35 with its verdict and step, and §9.
  - The ADR index and `CHANGELOG.md`.
  - `CLAUDE.md` only where the standing changes (ADR-0150); `README.md` and `docs/zh-TW`'s reader's guide as the
    result requires.
  - The whitepaper's version in both declarations, with its date
    ([ADR-0064](../docs/adr/0064-the-documentation-gate-and-the-version.md)).
- [ ] **The brief archived** as `briefs/README.md` says, every deliverable dispositioned, the round's ADRs set to
  `accepted` by the commit that asks for the merge, and the pull request marked ready for review with it.

## Not empowered

- **No rule of the engine changes.** The critic, its window, the address, the modulator, the signed gate, STDP, the
  membrane and the inhibitory rule are untouched. Nothing of the image or the records changes.
- The reliability is three in four, the coin is bits 48 and 49, the schedule is H-25's and the network is seed 22 at
  1 024 units. None is fitted to the result.
- H-35's clauses, its constants, its predicted readings and step 5's split do not move after a rewarded run. There is
  no second attempt.
- `Feedback::SevenInEight` and `Task::misleading_at` return what they returned. No pinned number of an earlier round
  moves.
- No change to `.github/workflows/ci.yml`, `scripts/exhaustive-shard.sh` or `scripts/exhaustive-costs.mjs`; the cost
  table changes only by regeneration from this round's dispatch.
- No float anywhere, no dependency, and no `unsafe` beyond ADR-0023's invariant.

## Architectural empowerment

The executing session may replace any instruction above with a better decision, recorded in its ADR. That covers:
- the feedback's name and shape: a variant of its own beside `SevenInEight`, or one form that carries the share, with
  `SevenInEight` the run it was either way;
- how the harness's reliability, its coin and its bounds, written for seven in eight, are given the share without
  moving H-28's pins;
- how the arms are built on H-28's harness, and how they are dealt into tests;
- how each reading is computed and pinned, and how the trials that mislead here and not at seven in eight are read;
- whether the round writes one ADR or two (the build and the measurement).

It may not:
- change a rule of the engine;
- change the reliability, the coin's bits, the schedule or the network;
- move a clause, a constant, a predicted reading or step 5's split after a rewarded run;
- reach the standing directives, the whitepaper's invariants or the constraints in `CLAUDE.md`.

If the calibration fails, the round stops there as H-35's stopping rule says: the finding is written, and there is no
rewarded run.

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
git diff origin/main...HEAD > target/pr.diff && cargo mutants --workspace --in-diff target/pr.diff
gh workflow run ci.yml --ref <this round's branch> -f scope=both
node scripts/exhaustive-costs.mjs from <the dispatch's downloaded artifacts> --run <its id> > scripts/exhaustive-costs.tsv
```

Every command exits 0. The dispatched run's twelve whole-domain shards are green and reproduce every pinned number.
In the history, the feedback, its tests, the coin's bits, H-35's constants, its predicted readings and step 5's split
precede the first rewarded run.

## Report

The closing message states:
- the feedback as built, and that under the four before it every run is the run it was;
- the coin's count per mapping and in all, and that the misleading trials at seven in eight are among them;
- the protocol, and when it was committed;
- H-35's verdict per clause and arm;
- each of the three predicted readings against what was read;
- the reversal speeds beside the two accounts, and what a misleading punishment cost beside H-28's;
- the step of the stopping rule reached, the branch of step 5 where it applies, and the next decision it names, not
  taken;
- that no pinned number moved;
- the scope ADR-0075 gave the diff, each shard's time against its bound and the regenerated cost table, and the
  sweep's state when the round ended;
- that the pull request is ready for review;
- what was not done and why.
