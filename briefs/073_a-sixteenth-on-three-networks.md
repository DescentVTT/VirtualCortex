---
status: proposed
date: 2026-10-13
---

# Brief 073: A sixteenth, on three networks — ADR-0182's question: under the rate's rule at a step of a sixteenth, on seeds 22, 23 and 24 at 1 024 units, each with the target its own derivation returns with the gain held, H-25's two arms on each; nothing under `src/` changed; H-39 run once

## Mission

**This brief builds nothing in the engine and runs one hypothesis once.** H-38 read that the engine learns and
revises with its controller holding the settled network's rate
([ADR-0181](../docs/adr/0181-a-rate-the-controller-holds-measured.md)). Its protocol took the largest step that
holds, an eighth, which is the one step at its loop's edge; a sixteenth's lead-in is quiet and no arm has run under
it; and the yes is seed 22's. [ADR-0182](../docs/adr/0182-a-sixteenth-on-three-networks.md) asks both at once: a
sixteenth, on the three networks of the size.

ADR-0182 wrote **H-39** before any run and **predicts yes**, with what would refute it and four predicted readings.

When the round is done, the tree holds:
- on seeds 23 and 24, the derivation under the rate's rule at a sixteenth as a weekly `exhaustive` test, beside seed
  22's, which brief 072 pinned;
- on each of the three networks, H-25's two arms under it as weekly `exhaustive` tests;
- H-39's verdict per network, arm and clause, and **the step of its stopping rule reached**, with the next decision it
  names and does not take.

**Do not run this round beside brief 065's**: both dispatch the weekly workflow.

---

## Standing directives

- **Latest ≠ Newest** (whitepaper §2.1, `CLAUDE.md` principle 4). Nothing is adopted: no mechanism, no parameter, no
  dependency, no version bump of a tool. **In particular**:
  - the rate's rule is ADR-0180's, run as built; no other form of it, no second target and no other step;
  - no change to the estimate's rule, the inhibitory rule, the critic, the address, the signed gate or the schedule;
  - the whole punishment of ADR-0155 and the exploration of ADR-0163 stay unset, and the feedback is the true one.
- **No file under `src/` and no file of a state crate changes.**
- **The networks are seeds 22, 23 and 24 at 1 024 units.** No other network and no other size is run.
- **The step is a sixteenth**, `0x1000` in Q0.16, on every network.
- **A network's target is the period its own derivation returns with the gain held**: 58 201 ticks on seed 22, 59 510
  on seed 23 and 59 179 on seed 24. None is carried from another network.
- **The rule is on from the lead-in's first window**, and the step and the target are carried in the image through
  the frozen block and every trial of an arm.
- **Seed 22's derivation at a sixteenth is the one brief 072 pinned.** Every pinned number of the tree holds, H-38's
  six steps and two arms among them. The determinism pin does not move.
- **A rule that gives no value on a network, or a frozen block that fails a measure there, is a reading of H-39** (its
  clause 0). It stops that network, not the round, and it is not repaired.
- **H-39's clauses, its constants and its four predicted readings are written before any lead-in is run under the
  rule on seed 23 or seed 24 and before any arm is run at a sixteenth**, and do not move after it. There is no second
  attempt.
- **An arm runs once.** A check added to an arm is tried first on an arm the tree already pins and on a network whose
  first ticks hold a spike, so that a failure of the check is found before the run it would spend.
- **The prediction is ADR-0182's**: yes. The round reports it against what was read, and each predicted reading as
  held or missed.
- **The engine and the harness are read before a description of them is trusted**, this brief's and ADR-0182's
  included.
- **No capacity of a run's configuration is reached silently.**
- No `f32`/`f64`, oracles included; every operation on a state field saturates or wraps by name
  ([ADR-0029](../docs/adr/0029-structural-enforcement.md)). Every loop ends by construction
  ([ADR-0062](../docs/adr/0062-the-first-complete-sweeps-list.md)).
- A heavy run is an `#[ignore]`d test whose name contains `exhaustive`. The runtime's gate grows by at most one test
  ([ADR-0061](../docs/adr/0061-the-learning-runs-leave-the-gate.md)). The mutation gate on the changed lines must pass
  ([ADR-0030](../docs/adr/0030-verification-governance.md)); with no line under `src/` changed it has nothing to
  mutate, and says so.
- **The round is evidenced as `briefs/README.md`'s "Evidencing a round" says**
  ([ADR-0150](../docs/adr/0150-a-round-waits-for-what-it-checks.md),
  [ADR-0158](../docs/adr/0158-the-sweep-off-the-next-decisions-path.md)): the runs run once, the dispatch's scope
  follows the diff, the merge waits for the whole-domain shards, the ADR cites the pull request's commits, the commit
  that asks for the merge sets it to `accepted`, and **a sweep that has ended and that no ADR has recorded is recorded
  by this round's**. One is named in the Context.
- **The pull request leaves draft with the commit that asks for the merge.**
- **No shard of the weekly job passes 60 per cent of its bound**, 180 minutes, under the regenerated deal
  ([ADR-0179](../docs/adr/0179-the-bounds-rule-restated.md)); one that does is reported with its job's minutes.
- No pinned number of an earlier round moves.
- Conventional Commits with a real body; never commit on `main`; the required checks keep their names.

## Context

Re-derived on 2026-10-13 against `main` at `92f52c0`, after ADR-0180 and ADR-0181 merged. Line numbers move; the
symbols and the quoted sentences are what to re-derive.

1. **The rule** (`crates/cortex-homeostasis/src/lib.rs`, `HomeostaticDrivePool::regulate_rate(target_per_window)`,
   ADR-0180): the gain moved by the window's count over its target in the estimate's place.
   `Config::rate_target_period_ticks`, zero unset; `Executor::rate_target_per_window`; the count is the units times
   131 072 ticks over the period, to nearest: 2 306, 2 255 and 2 268 at 1 024 units for 58 201, 59 510 and 59 179
   ticks. The image carries the period in its modulator section; the format is 22.
2. **The harness under the rule** (`runtime/cortex-runtime/tests/inhibition.rs`, ADR-0181): `Settling` carries the
   step, the target and whether the settled network's rule reads the gain's mean; `derive_under(units, seed,
   settling, name)` and `candidate_under`; the oracle `rate_regulated`; a lead-in's reading `RateRead` by `rate_read`,
   and `steepness`; `rate_step`, a step's weekly test; the arms' ground `Ground::Ruled` with `RuledSite` and
   `ruled_made`; `ruled_stand`, the tables an arm is read beside; `GainWatch`, `GainRead` and `gain_blocks`, the
   controller read beside every arm, counting from the load.
3. **What is pinned on seed 22**: the six steps, `the_rate_s_rule_at_a_step_of_…_at_1024_units_exhaustive`, their
   tables `RATE_*`; the two arms under an eighth,
   `the_address_drawn_under_the_rate_s_rule_from_the_{assignment,mirrored}_at_1024_units_exhaustive`. At a sixteenth
   the lead-in settles at its 96th window, its gain within 1.7419 and 1.7599 and its count within 2 067 and 2 507, the
   image carries a gain of 1.7505, the stimulus is seen in 62 trials of 64 and the inhibitory target period is 58 188.
4. **What the arms under an eighth read** (ADR-0181): each mapping's last 128 correct 122, 123, 118, 124 and 118, 118,
   124, 119; the reversals in 18, 19, 16 and 15, 15, 15 blocks; the highest coupling 1.191 and 1.220; 2 306.1 and
   2 306.0 spikes a window, a window holding 1 880 to 2 882; the gain's mean 1.7215 and 1.7210 and its band 1.6985 to
   1.7407; consecutive windows on opposite sides of the target in 750 and 758 pairs of 959; the inhibitory sum ending
   at 0.782 and 0.783 of its image's.
5. **The steepness read on seed 22**: fifteen per cent of rate for one per cent of gain, so a deviation of the gain
   stands at $-0.88$ of itself a window later at an eighth and $+0.06$ at a sixteenth.
6. **The other networks** (ADR-0167, ADR-0169): a network is a `Site`, `seeded_site(n)` seeds 23 and 24 at 1 024
   units, with its derivation with the gain held pinned (`RULES_SEEDED`: target periods 59 510 and 59 179) and its two
   arms with the gain held pinned (H-33's: each mapping's last 128 correct 126, 124, 124, 123 and 122, 115, 118, 126;
   117, 123, 120, 115 and 124, 120, 121, 123; the inhibitory sum ending at 0.908 and 0.910, and 0.896 and 0.897). An
   arm is `arm_on(ground, share, arm)` (ADR-0174).
7. **ADR-0181 on the arms' ground**: *"A `Site` given a settling … would need a pinned derivation in a `Site`'s shape
   for a network whose derivation before the settled network is seed 22's own"*. On seeds 23 and 24 a derivation
   under the rule has no pin yet: its first run's tables are pinned and the dispatch reproduces them, as ADR-0167
   held a derivation on a new seed.
8. **ADR-0181 on its own harness**: the watch it added *"missed 500 ticks of every arm, and the arms ran twice for
   it. A check of the watch on a network whose first 500 ticks hold a spike would have found it before the run."*
9. **The weekly job**: twelve shards of 180 minutes, each running its tests two at a time. The cost table holds 128
   tests and 101 602 s; the two arms under the rule read 1 834 and 2 269 s, a step's derivation 185 to 279 s.
10. **The dispatch's scope** ([ADR-0075](../docs/adr/0075-the-dispatch-scope-follows-the-diff.md)): `scope=exhaustive`
    for a round that changes no file under `src/`.
11. **A sweep to look for.** The sweep of brief 072's dispatch, run 38417864748, had not ended when ADR-0182 was
    written. If it has ended when this round's ADR is written and no ADR has recorded it, it is recorded there: the
    artifacts `mutants-out-crates` and `mutants-out-runtime-0` to `-11`, each `caught.txt`, `missed.txt`,
    `timeout.txt` and `unviable.txt` counted by its lines, the timeouts read against ADR-0062's list with
    `timeout-evidence.txt` beside them, and a survivor, if there is one, a finding.

## Deliverables

- [ ] **An arm of a network under the rule, in the tests**: H-25's arm on seeds 23 and 24 from the image the
  derivation under the rule writes, with the oracles and the readings an arm under the rule holds on seed 22; and on
  seed 22 at a sixteenth. Through it every arm the tree pins reads what it pins.
- [ ] **H-39's protocol, in a new ADR at the next free number (`ls docs/adr`), before any lead-in is run under the
  rule on seed 23 or seed 24 and before any arm is run at a sixteenth.**
  - The networks, the targets and the step.
  - What holds a derivation under the rule on seeds 23 and 24, where seed 22's has pinned tables.
  - H-39's clauses and constants, restated from ADR-0182 and pinned in the tests.
  - The four predicted readings as rules over the tables: the step holds on seeds 23 and 24; through each arm the
    gain after every window within 1.6625 and 1.8375; in each arm consecutive windows on opposite sides of the target
    in fewer than 600 pairs of 959; in each arm the inhibitory sum ending below the same network's arm's with the
    gain held.
  - The readings, no clause, as ADR-0182 lists them.
  - How a check added to an arm was tried before the runs.
- [ ] **The calibration**, before any such run: every pinned number of the tree holds, H-38's six steps and two arms
  among them, and seed 22's derivation at a sixteenth is the one pinned.
- [ ] **The derivations on seeds 23 and 24**, each a weekly `exhaustive` test, run once: its lead-in pinned by
  window, what each rule returned or that it returned nothing, the frozen block's readings and the steepness.
- [ ] **The arms**: on each network whose clause 0 holds, H-25's two, each a weekly `exhaustive` test, run once, their
  tables pinned per block, the controller's reading among them.
- [ ] **The ADR's reading.**
  - H-39's verdict per network, arm and clause.
  - The prediction and each of the four predicted readings against what was read.
  - Seed 22 at a sixteenth beside seed 22 at an eighth.
  - Each network beside its own arms with the gain held, the inhibitory sum's course among them.
  - Over the three networks: the mappings learned of twenty-four, the reversals' blocks, the steepness.
  - **The step of the stopping rule reached, and the next decision it names, not taken.**
- [ ] **The gate.** At most one runtime test: H-39's clauses and its predicted readings' rules at their edges, over
  tables written by hand.
- [ ] **The evidence**, as "Evidencing a round" says. A weekly dispatched on this round's branch at
  `scope=exhaustive`. Its twelve whole-domain shards are green and reproduce every pinned number. The cost table is
  regenerated from their artifacts. The ADR names the dispatch and each shard's job time against its bound.
- [ ] **The sweep recorded**: the one named in the Context, in this round's ADR if it has ended and no ADR has
  recorded it; said so if it has not.
- [ ] **The documents, in the same pull request.**
  - Whitepaper §11.1's H-39 with its verdict and step, and §9.
  - The ADR index and `CHANGELOG.md`.
  - `CLAUDE.md` only where the standing changes (ADR-0150); `README.md` and `docs/zh-TW`'s reader's guide as the
    result requires.
  - The whitepaper's version in both declarations, with its date
    ([ADR-0064](../docs/adr/0064-the-documentation-gate-and-the-version.md)).
- [ ] **The brief archived** as `briefs/README.md` says, every deliverable dispositioned, the round's ADR set to
  `accepted` by the commit that asks for the merge, and the pull request marked ready for review with it.

## Not empowered

- **No file under `src/` and no file of a state crate changes.** No rule, no record, no format.
- The step is a sixteenth, the networks are the three, and a network's target is its own derivation's with the gain
  held. None is fitted to the result, and none other is run.
- No rule that derives a constant is changed so that a network's step holds.
- H-39's clauses, its constants and its predicted readings do not move after such a run. There is no second attempt,
  and an arm is not run a second time for a check of the harness.
- No pinned number of an earlier round moves.
- No change to `.github/workflows/ci.yml`, `scripts/exhaustive-shard.sh` or `scripts/exhaustive-costs.mjs`; the cost
  table changes only by regeneration from this round's dispatch.
- No float anywhere, no dependency, and no `unsafe`.

## Architectural empowerment

The executing session may replace any instruction above with a better decision, recorded in its ADR. That covers:
- how an arm's ground carries a network and a settling together, with every call of the tree the call it was;
- how a derivation under the rule on a network with no pin is held: what is asserted by rule, and what is pinned from
  the one run;
- what an arm on seed 23 or seed 24 is read beside, and how its first block is held;
- how the two derivations and the six arms are dealt into tests, and whether the local runs go one after another or
  side by side;
- how each reading is computed and pinned;
- how a check added to an arm is tried before the runs.

It may not:
- change a file under `src/` or of a state crate;
- change the step, a target, a network, the task or the schedule;
- move a clause, a constant or a predicted reading after such a run;
- reach the standing directives, the whitepaper's invariants or the constraints in `CLAUDE.md`.

If the calibration fails, the round stops there as H-39's stopping rule says: the finding is written, and no such run
is made.

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
In the history, the networks, the targets, the step, H-39's constants and its predicted readings precede any lead-in
under the rule on seed 23 or seed 24 and any arm at a sixteenth.

## Report

The closing message states:
- how an arm's ground took a network and a settling, and that every pinned arm reads what the tree pins;
- the protocol, and when it was committed;
- per network, what each rule returned under the rule at a sixteenth, the frozen block's measures and the steepness;
- H-39's verdict per network, arm and clause, and the prediction and each predicted reading against what was read;
- seed 22 at a sixteenth beside seed 22 at an eighth, and each network beside its own arms with the gain held;
- the step of the stopping rule reached and the next decision it names, not taken;
- that no pinned number moved, no file under `src/` changed, and that each arm ran once;
- the scope ADR-0075 gave the diff, each shard's job time against its bound and the regenerated cost table;
- the sweep of run 38417864748: recorded, already recorded, or not yet ended;
- that the pull request is ready for review;
- what was not done and why.
