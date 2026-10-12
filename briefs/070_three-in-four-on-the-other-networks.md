---
status: proposed
date: 2026-10-12
---

# Brief 070: Three in four on the other networks — ADR-0173's question: the reward right three times in four asked of the two other networks of the size, seeds 23 and 24, each under the configuration its rules derived, H-25's two arms on each; nothing under `src/` changed; H-36 run once

## Mission

**This brief builds nothing in the engine and runs one hypothesis once.** H-35 read no under a reward right three
times in four, on seed 22 at 1 024 units: seven mappings of eight learned, the eighth five trials short, and the rule
that splits the no two trials from its other branch
([ADR-0172](../docs/adr/0172-a-reward-right-three-times-in-four-measured.md)).
[ADR-0173](../docs/adr/0173-three-in-four-on-the-other-networks.md) reads the need before a rule of the engine is
changed for it: the same share on the two other networks of the size, seeds 23 and 24, which H-33 read under the true
reward.

ADR-0173 wrote **H-36** before any run: on each network, H-35's four clauses in both arms. It predicts no verdict; it
writes what H-35's own ratios forecast and four predicted readings.

When the round is done, the tree holds:
- on each of the two networks, H-25's two arms under `Feedback::ThreeInFour` as weekly `exhaustive` tests, each
  beside its own network's arm under the true reward;
- H-36's verdict per network, arm and clause, and **the step of its stopping rule reached**, with the next decision it
  names and does not take;
- the readings over the three networks.

**Do not run this round beside brief 065's**: both dispatch the weekly workflow.

---

## Standing directives

- **Latest ≠ Newest** (whitepaper §2.1, `CLAUDE.md` principle 4). Nothing is adopted: no mechanism, no parameter, no
  dependency, no version bump of a tool. **In particular**:
  - no change to the gate's clamp, a single error's weight, the critic's step or the schedule: ADR-0173 reads the need
    for them and takes none;
  - the whole punishment of ADR-0155 and the exploration of ADR-0163 stay unset;
  - two readouts and two stimuli, H-25's task.
- **No file under `src/` and no file of a state crate changes.**
- **One change from H-35**: the network. The share, the coin, the schedule, the arms, the task's seed and the drive's
  are H-35's. Seen from H-33, one change too: the reward's reliability.
- **The networks are seeds 23 and 24 at 1 024 units**, each under the configuration ADR-0167 pinned for it. No other
  network, size or share is run, before or after.
- **The same trials mislead on every network.** The task's seed is 27, so the coin's count is H-35's: 1 913 in all,
  and 363, 514, 515 and 521 by mapping.
- **"Correct" is the selection, not the reward.**
- **H-36's clauses, its constants and its four predicted readings are written before the first rewarded run at three
  in four on either network** and do not move after it. There is no second attempt.
- **A prediction of the verdict is not owed.** ADR-0173 writes none; this round does not add one. Each predicted
  reading is reported against what was read, a miss as a miss.
- **The engine and the harness are read before a description of them is trusted**, this brief's and ADR-0173's
  included.
- No `f32`/`f64`, oracles included; every operation on a state field saturates or wraps by name
  ([ADR-0029](../docs/adr/0029-structural-enforcement.md)). Every loop ends by construction
  ([ADR-0062](../docs/adr/0062-the-first-complete-sweeps-list.md)).
- A heavy run is an `#[ignore]`d test whose name contains `exhaustive`. The runtime's gate grows by at most one test
  ([ADR-0061](../docs/adr/0061-the-learning-runs-leave-the-gate.md)). The mutation gate on the changed lines must pass
  ([ADR-0030](../docs/adr/0030-verification-governance.md)); with no line under `src/` changed it has nothing to
  mutate, and says so.
- **The round is evidenced as `briefs/README.md`'s "Evidencing a round" says**
  ([ADR-0150](../docs/adr/0150-a-round-waits-for-what-it-checks.md),
  [ADR-0158](../docs/adr/0158-the-sweep-off-the-next-decisions-path.md)): the arms run once, the dispatch's scope
  follows the diff, the merge waits for the whole-domain shards, the ADR cites the pull request's commits, the commit
  that asks for the merge sets it to `accepted`, and **a sweep that has ended and that no ADR has recorded is recorded
  by this round's**. One is named in the Context.
- **The pull request leaves draft with the commit that asks for the merge.**
- **The weekly job's cost is reported.** The regenerated deal is planned against 60 per cent of a shard's bound; a
  shard that holds one of H-34's arms, half of the bound by itself, is reported with the arm's seconds. Where the
  regenerated plan itself passes 60, the round says so and goes on: ADR-0173 names the workflow's decision as due
  before the round after this one.
- No pinned number of an earlier round moves.
- Conventional Commits with a real body; never commit on `main`; the required checks keep their names.

## Context

Re-derived on 2026-10-12 against `main` at `58659b5`, after ADR-0171 and ADR-0172 merged. Line numbers move; the
symbols and the quoted sentences are what to re-derive.

1. **The feedback** (`runtime/cortex-runtime/src/task.rs`): `Feedback::ThreeInFour`, its coin
   `Task::misleading_in_four_at`, bits 48 and 49 of the trial's draw both zero (`MISLEADING_IN_FOUR_BITS`).
2. **A network is a `Site`** (`runtime/cortex-runtime/tests/inhibition.rs`, ADR-0169): its size, its prior's seed, the
   name its dumps carry and its pinned tables. `seeded_site(n)` is seeds 23 and 24 at 1 024 units, with ADR-0167's
   pins; `site_derivation` and `site_arm(site, arm)` run a network's derivation and an arm of H-25 on it under the
   true reward, the derivation held to its pins before the first trial.
3. **A share is a `Share`** (ADR-0172): the task's feedback, the coin as the harness writes it and as the task draws
   it, the reliability's part in an expected reward, the bounds the coin's count is held to and its count by mapping,
   the pins. `seven_in_eight()` is H-28's and `three_in_four()` H-35's. `misled_arm_at(share, arm)` is the arm on
   seed 22, from H-23's image by its CRC; `misled_tables(share, verdict, signed)` the gate's checks over an arm's
   pinned tables.
4. **The two do not yet meet**: `site_arm` runs under the true reward, and `misled_arm_at` on seed 22's image.
5. **H-35's rules** (ADR-0172): `holds_expected_reward_at`, `misled_at`, `beside_expected_at`, `nearer_expected_at`,
   `crossings`, `beside_accounts`, `in_four_predicted`, `in_four_step`, `taken_back`, `IN_FOUR_BOUNDS`,
   `IN_FOUR_BY_MAPPING_1024` (363, 514, 515, 521), the verdict `IN_FOUR_1024` and its step `IN_FOUR_STEP_1024`.
6. **What H-35 read on seed 22**: each mapping's last 128 correct 115, 80, 110, 75 and 117, 94, 110, 97; the reversals
   at 26, 20 and none in its 32, and 24, 19 and 24 blocks, a mean ratio to H-25's of 1.42; the learning signal 0.46 to
   0.73 of H-25's; a true reward leaving the signal above the gate's ceiling in 47 per cent of them; misleading
   punishments taking back 42 and 44 per cent of what true rewards gave, and misleading rewards putting back 41 and
   43 per cent of what true punishments took.
7. **What H-33 read on the two networks under the true reward** (ADR-0167): each mapping's last 128 correct 126, 124,
   124, 123 and 122, 115, 118, 126 on seed 23, and 117, 123, 120, 115 and 124, 120, 121, 123 on seed 24; the
   reversals at 15, 13, 13 and 16, 14, 12 blocks on seed 23, and 21, 12, 15 and 17, 16, 14 on seed 24.
8. **The weekly job**: twelve shards of 120 minutes, each running its tests two at a time. The cost table holds 110
   tests and 83 129 s; H-33's four arms read 1 871 to 1 892 s in the last run and H-35's two 1 807 and 1 827.
9. **The dispatch's scope** ([ADR-0075](../docs/adr/0075-the-dispatch-scope-follows-the-diff.md)): `scope=exhaustive`
   for a round that changes no file under `src/`.
10. **A sweep to look for.** The sweep of brief 069's dispatch, run 38228081172, is recorded in ADR-0173: 3 857
    mutants, 3 680 caught, none missed. The schedule's weekly on `main`, run 38234325824, had not ended its sweep
    when ADR-0173 was written. If it has ended when this round's ADR is written, it is recorded there: the
    artifacts `mutants-out-crates` and `mutants-out-runtime-0` to `-11`, each `caught.txt`, `missed.txt`,
    `timeout.txt` and `unviable.txt` counted by its lines, the timeouts read against ADR-0062's list with
    `timeout-evidence.txt` beside them, and a survivor, if there is one, a finding.

## Deliverables

- [ ] **An arm of a site at a share, in the tests.**
  - H-25's arm on a network that is not seed 22's, under a feedback that is not the true reward's, with the oracles
    and the readings H-35's arm holds.
  - Through it, H-33's arms on the two networks under the true reward and H-35's arms on seed 22 read what the tree
    pins.
- [ ] **H-36's protocol, in a new ADR at the next free number (`ls docs/adr`), before the first rewarded run at three
  in four on either network.**
  - The networks, seeds 23 and 24 at 1 024 units, and the arms, H-25's two on each.
  - The coin's count, held to H-35's in all and by mapping.
  - H-36's clauses and constants, restated from ADR-0173 and pinned in the tests: H-35's four on each network, and
    H-35's split read per network.
  - The four predicted readings as rules over the tables:
    - the first mapping learned in all four arms;
    - on each network the mean of its six reversals' ratios to its own under the true reward above 1.13, a reversal
      that does not cross counted at 32 blocks;
    - the value nearer a half of $2p - 1$ than $2p - 1$ for each stimulus in each mapping clause 1 holds for and
      whose accuracy over those trials is at least three quarters;
    - in each arm, misleading punishments taking back between a third and a half of what true rewards give the
      answer's pairs, and misleading rewards putting back between a third and a half of what true punishments take.
  - The readings, no clause, as ADR-0173 lists them: per network beside its own arms under the true reward and
    beside H-35's, and over the three networks.
- [ ] **The calibration**, before any such run: every pinned number holds, H-33's arms on the two networks and H-35's
  on seed 22 among them; on each network the derivation is the one pinned and the first block under the true
  feedback is H-33's; the coin's count is H-35's.
- [ ] **The runs**: on each network H-25's two arms, each a weekly `exhaustive` test, run once, their tables pinned
  per block.
- [ ] **The ADR's reading.**
  - H-36's verdict per network, arm and clause, and the split on a network where clause 1 or 2 fails.
  - Each of the four predicted readings against what was read.
  - The reversal speeds beside each network's own under the true reward and beside ADR-0173's forecast.
  - The readings by kind on each network beside H-35's: what a misleading punishment and a misleading reward moved,
    a true reward's error, and how often the clamp acted.
  - Over the three networks: the mappings learned of twenty-four, the reversals that crossed of eighteen, and the
    mean ratio.
  - **The step of the stopping rule reached, and the next decision it names, not taken.**
- [ ] **The gate.** At most one runtime test: H-36's clauses and its predicted readings' rules at their edges, over
  tables written by hand.
- [ ] **The evidence**, as "Evidencing a round" says. A weekly dispatched on this round's branch at
  `scope=exhaustive`. Its twelve whole-domain shards are green and reproduce every pinned number. The cost table is
  regenerated from their artifacts. The ADR names the dispatch, each shard's wall time against its bound, and what
  the regenerated deal plans.
- [ ] **The sweep recorded**: the one named in the Context, in this round's ADR if it has ended; said so if it has
  not.
- [ ] **The documents, in the same pull request.**
  - Whitepaper §11.1's H-36 with its verdict and step, and §9.
  - The ADR index and `CHANGELOG.md`.
  - `CLAUDE.md` only where the standing changes (ADR-0150); `README.md` and `docs/zh-TW`'s reader's guide as the
    result requires.
  - The whitepaper's version in both declarations, with its date
    ([ADR-0064](../docs/adr/0064-the-documentation-gate-and-the-version.md)).
- [ ] **The brief archived** as `briefs/README.md` says, every deliverable dispositioned, the round's ADR set to
  `accepted` by the commit that asks for the merge, and the pull request marked ready for review with it.

## Not empowered

- **No file under `src/` and no file of a state crate changes.** No rule, no record, no format.
- No change to the gate's clamp, a single error's weight, the critic's step or the schedule, in the engine or in a
  test's configuration of it.
- The networks are seeds 23 and 24 at 1 024 units, the share is three in four and the task's seed is 27. None is
  fitted to the result, and none other is run.
- No constant of a network's derivation is moved or carried from another network.
- H-36's clauses, its constants and its predicted readings do not move after a rewarded run. There is no second
  attempt.
- No pinned number of an earlier round moves.
- No change to `.github/workflows/ci.yml`, `scripts/exhaustive-shard.sh` or `scripts/exhaustive-costs.mjs`; the cost
  table changes only by regeneration from this round's dispatch. A shard's bound and the shards' count are not this
  round's.
- No float anywhere, no dependency, and no `unsafe`.

## Architectural empowerment

The executing session may replace any instruction above with a better decision, recorded in its ADR. That covers:
- how a site's arm takes a share, or a share's arm a site, with every call of the tree the call it was;
- how the first block under the true feedback is held to H-33's on each network;
- how an arm's readings stand beside its own network's under the true reward, where H-33 pinned the table, and how
  they are read where it did not;
- how the four arms are dealt into tests, and whether the local runs go one after another or side by side;
- how each reading is computed and pinned, the ones over three networks among them.

It may not:
- change a file under `src/` or of a state crate;
- change the share, the coin, the schedule, the task's seed or a network;
- move a clause, a constant or a predicted reading after a rewarded run;
- reach the standing directives, the whitepaper's invariants or the constraints in `CLAUDE.md`.

If the calibration fails, the round stops there as H-36's stopping rule says: the finding is written, and there is no
rewarded run at three in four on either network.

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
gh workflow run ci.yml --ref <this round's branch> -f scope=exhaustive
node scripts/exhaustive-costs.mjs from <the dispatch's downloaded artifacts> --run <its id> > scripts/exhaustive-costs.tsv
```

Every command exits 0. The dispatched run's twelve whole-domain shards are green and reproduce every pinned number.
In the history, the networks, H-36's constants and its predicted readings precede the first rewarded run at three in
four on either network.

## Report

The closing message states:
- how a site's arm took the share, and that H-33's and H-35's arms read what the tree pins through it;
- the protocol, and when it was committed;
- H-36's verdict per network, arm and clause, and the split where clause 1 or 2 fails;
- each of the four predicted readings against what was read;
- the reversal speeds beside each network's own and beside ADR-0173's forecast, and the readings by kind beside
  H-35's;
- over the three networks, the mappings learned, the reversals that crossed and the mean ratio;
- the step of the stopping rule reached and the next decision it names, not taken;
- that no pinned number moved and no file under `src/` changed;
- the scope ADR-0075 gave the diff, each shard's time against its bound and what the regenerated deal plans;
- the schedule's sweep: recorded, or not yet ended;
- that the pull request is ready for review;
- what was not done and why.
