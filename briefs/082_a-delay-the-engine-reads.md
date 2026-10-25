---
status: proposed
date: 2026-10-25
affectedFiles: [runtime/cortex-runtime/src/**, runtime/cortex-runtime/tests/**, docs/**, briefs/**, CHANGELOG.md, CLAUDE.md, README.md, scripts/exhaustive-costs.tsv]
protectedFiles: [crates/**, benches/**, .github/workflows/**, scripts/arm-run.sh, scripts/exhaustive-shard.sh, scripts/exhaustive-costs.mjs, scripts/check-additive.mjs, scripts/check-archive.mjs, scripts/check-scope.mjs, scripts/check-explorations.mjs, scripts/check-doc-shape.mjs, scripts/adr-index.mjs, .spec-brief.json, package.json, package-lock.json, rust-toolchain.toml, Cargo.toml, Cargo.lock]
---

# Brief 082: A delay the engine reads from its kept addresses

## Mission

**This brief builds one thing in the engine and runs one hypothesis once.**
[ADR-0216](../docs/adr/0216-a-delay-the-engine-reads.md) takes a delay the engine is not told:
- with `Config::reads_its_delay` set, the engine carries a value critic per kept age, each learning from its own age's counts;
- it consolidates at age zero until another age's error leads for 64 rewards running by a thirty-second of the reward, then moves to that age once and stays.

ADR-0216 wrote **H-45** before any run, in [H-45's report](../docs/research/H-45.md), and **predicts yes** with three readings.

When the round is done, the tree holds:
- the rule in the engine, unset bit for bit, and the task's untold delay;
- on each of seeds 22, 23 and 24 at 1 024 units, H-25's two arms at a delay of one trial and of two given without its age, and on time given without its age, under the default's network, as weekly `exhaustive` tests: eighteen arms;
- H-45's verdict per network, arm, delay and clause, and the step of its stopping rule reached, with the next decision it names and does not take.

Its readings go to H-45's report, its measured ADR is short, and it rewrites `docs/STATUS.md` ([briefs/README.md](README.md#a-rounds-documents)).

The scope is the front matter's ([ADR-0191](../docs/adr/0191-a-rounds-scope-eighteen-shards-and-the-reproduction-first.md)): `spec-brief archive --strict` refuses the round outside it.

**Do not run this round beside brief 065's**: both dispatch the weekly workflow.

---

## Standing directives

- **Latest ≠ Newest** (whitepaper §2.1, `CLAUDE.md` principle 4). The round depends on nothing new: no crate, no tool and no version bump. The engine's own critic is kept once per age.
- **The rule is ADR-0216's.**
  - `Config::reads_its_delay`, unset by default, requires `kept_addresses` and the critic.
  - `kept_addresses` + 1 value critics, one per age, each starting from the critic's weights when the rule starts.
  - At a reward given without an age, each age's critic reads its value from the counts of the trial that age closes before the newest, whether its entry is spent or not, records its error's magnitude, then moves its weights by the error times those counts.
  - The age in use is zero at the start: its address written, its error to the modulator, its entry spent, its critic the engine's. A reward whose age in use has no live entry reaches no address and no modulator.
  - An age's score is the sum of its error magnitudes over its last 64 rewards; scores are compared once every age has 64.
  - While at age zero, when one other age has the strictly lowest score at 64 rewards running, each time below every other by at least 64 × a thirty-second of the reward, the engine moves to it, and never moves again.
  - Unset, every run is the run it was, bit for bit. A reward that names its age is H-43's, set or not. Nothing of it is in the image, and the format stays 25.
- **The task's untold delay is ADR-0216's**: trial t's sign at the end of trial t + k, given without its age; none at the first k trials' ends; the last k signs not given.
- **The runs are on seeds 22, 23 and 24 at 1 024 units, from the default's network** (H-42's derivations, pinned), `kept_addresses` 2, the rule set, H-25's arms on H-20's schedule, at k = 1, 2 and the control at 0. No other network, delay, lead or arm is run.
- **H-45's clauses, its constants and its three predicted readings are committed before any arm runs**, and do not move after it. There is no second attempt.
- **The runs are dispatches, as `briefs/README.md`'s "Evidencing a round" says** ([ADR-0211](../docs/adr/0211-the-round-shortened.md), [ADR-0215](../docs/adr/0215-the-calibration-a-change-needs.md)).
  - The round runs `npm run round:additive -- origin/main <the protocol's commit>` and quotes it in the protocol ADR. A change under `src/` does not only add, so the calibration is the whole tree's, excluding the round's own tests, dispatched with the arms.
  - The calibration is read first. No arm's log is read until every calibration shard is green and the calibration check of H-45's stopping rule, step 2, holds; one that fails stops the round, and its arms are void.
  - The arms run at `scope=arms`, once.
  - The reproduction is dispatched as soon as the tables are pinned, at `scope=exhaustive` with only the round's own tests, when nothing the calibration ran has changed since its commit; otherwise with every test.
  - No round dispatches the sweep. No whole-domain test runs on the developer machine.
- **The branch is named `claude/brief-082-…`**, never rebased or force-pushed once a dispatch has run on it. Every commit is signed.
- **The prediction is ADR-0216's**: yes. The round reports it against what was read, and each predicted reading as held or missed.
- **The engine and the harness are read before a description of them is trusted**, this brief's and ADR-0216's included. So is exploration 003's branch, `explore/003-a-delay-the-engine-is-not-told`, whose host-side critics and switch ran this rule unlatched.
- **No capacity of a run's configuration is reached silently.**
- No `f32`/`f64`, oracles included; every operation on a state field saturates or wraps by name ([ADR-0029](../docs/adr/0029-structural-enforcement.md)). Every loop ends by construction ([ADR-0062](../docs/adr/0062-the-first-complete-sweeps-list.md)). No `unsafe`.
- A heavy run is an `#[ignore]`d test whose name contains `exhaustive`. The runtime's gate grows by at most one test ([ADR-0061](../docs/adr/0061-the-learning-runs-leave-the-gate.md)). A new rule carries a test over the lattice of `testkit/prop.rs`, and the mutation gate catches every mutant in the lines the change touches ([ADR-0030](../docs/adr/0030-verification-governance.md)).
- **The documents are ADR-0197's and ADR-0201's**: new ADRs in their shape, the tables written by `node scripts/adr-index.mjs`, new text within `npm run spec:shape`'s limits and its math GitHub's.
- **The pull request leaves draft with the commit that asks for the merge**, which sets the round's ADRs to `accepted`.
- **No shard of the weekly job passes 60 per cent of its bound**, 180 minutes, at eighteen shards ([ADR-0179](../docs/adr/0179-the-bounds-rule-restated.md), ADR-0191), read from the calibration's shards. One that does is reported with its job's minutes, and the bound moves by ADR-0179's rule.
- No pinned number of an earlier round moves.
- **The maintainer is notified** as `briefs/README.md` says: when a question only they can answer stops the round, and when the pull request is ready for review.
- Conventional Commits with a real body; never commit on `main`; the required checks keep their names.

## Context

Re-derived on 2026-10-25 against `main` at `b4442fd`. Line numbers move; the symbols and the quoted sentences are what to re-derive.

1. **The kept addresses** (`runtime/cortex-runtime/src/executor.rs`, [ADR-0208](../docs/adr/0208-a-reward-past-the-trial-its-address-kept-built.md)): `Config::kept_addresses`, `KEPT_ADDRESSES_MAX`, `Executor::keep_address`, `Executor::kept_at(age)`, the counts and the targets of an entry, `Executor::reward_kept(reward_q16, age)`, `kept_let_go`.
2. **The critic** (`crates/cortex-neuromod`, protected): `ValueCritic`, its weights an `i16` a unit, `value_q16` over `(weight, count)` pairs; the executor's `features()` and its delta rule at `reward`.
3. **The task** (`runtime/cortex-runtime/src/task.rs`): `Delay`, `Delay::ON_TIME`, `Delay::of(trials)`, and how `Task::trial` keeps the address and gives the sign under a delay.
4. **The default's network**: `UNDER_THE_PROBE` and `PROBE_RULES` in `tests/inhibition.rs`, H-42's derivations pinned. H-42's arms ([ADR-0204](../docs/adr/0204-a-step-the-engine-makes-from-its-own-loop-measured.md)) and H-43's arms at one and two trials' delay ([ADR-0210](../docs/adr/0210-a-reward-past-the-trial-its-address-kept-measured.md)), their blocks pinned.
5. **Exploration 003** ([its note](../docs/explorations/003-a-delay-the-engine-is-not-told.md)): `untold_arm` on its branch ran the per-age critics on the host by the harness's `value_step`, and a switch at a thirty-second, unlatched. Every first move went to the true age; two second moves went wrong. Its figures are exploratory.
6. **The process** ([ADR-0211](../docs/adr/0211-the-round-shortened.md), [ADR-0215](../docs/adr/0215-the-calibration-a-change-needs.md), ADR-0191, ADR-0197, ADR-0201, `briefs/README.md`): eighteen weekly shards; the check of a change that only adds; the calibration with the arms; the reproduction with `-f only`; the cost table from the calibration's and the reproduction's artifacts together; `spec-brief archive <NNN> --strict` measured from `origin/main`.
7. **The sweep**: the schedule's run every Monday at 03:00Z sweeps the whole tree and holds most of the account's twenty places for hours; a dispatch made then waits. The first ADR that merges after a scheduled sweep ends records it ([ADR-0158](../docs/adr/0158-the-sweep-off-the-next-decisions-path.md)); that of 2026-10-26 had not run when this brief was written.

## Deliverables

- [ ] **The rule and the task's untold delay, built, in a new ADR at the next free number (`ls docs/adr`), kind `built`.**
  - ADR-0216's `Config::reads_its_delay`, the per-age critics, the score, the latched move and the reward given without an age, with their refusals.
  - The task's untold delay.
  - Unset, every run the run it was, bit for bit; a reward that names its age H-43's, set or not.
  - Its tests:
    - by hand at the edges, the refusals among them;
    - over the lattice against an oracle of the per-age critics, the scores and the move;
    - the move made once and never again, and no move before every age has 64 rewards;
    - on time with no move, the run without the rule.
- [ ] **H-45's protocol, in a new ADR at the next free number, kind `protocol`, before any arm runs.**
  - The networks, the delays, the arms and the controls.
  - H-45's clauses and constants, restated from ADR-0216 and pinned in the tests.
  - The three predicted readings as rules over the tables.
  - The calibration's check of H-45's stopping rule, step 2, as a test.
  - The output of `npm run round:additive` on the protocol's commit.
- [ ] **The calibration and the arms, dispatched together on the protocol's commit**: a dispatch at `scope=exhaustive` excluding this round's own tests, every pinned number held, and the eighteen arms at `scope=arms`, each a weekly `exhaustive` test run once. The calibration is read first.
- [ ] **The arms' tables pinned per block**, once the calibration is green: each arm's move, its trial and its age, and the scores at it.
- [ ] **The reproduction**, dispatched as soon as the tables are pinned: `scope=exhaustive` with `-f only` naming the round's own tests, when the diff from the calibration's commit changes no file under `src/` and no test or helper the calibration ran. Its shards are green and reproduce every pinned number, and the cost table is regenerated from the calibration's and the reproduction's artifacts.
- [ ] **H-45's report**, `docs/research/H-45.md`: its `verdict:` set, the readings' tables, each arm beside H-43's told arm and exploration 003's switch arm at the same delay, and the dispatches that are their evidence.
- [ ] **The measured ADR**, kind `measured`, short:
  - the verdict per network, arm, delay and clause, in one table;
  - each of the three predicted readings, held or missed, in a line;
  - the step of the stopping rule reached, and the next decision it names, not taken.
- [ ] **The gate.** At most one runtime test: H-45's clauses and its predicted readings' rules at their edges, over tables written by hand.
- [ ] **The documents, in the same pull request.**
  - `docs/STATUS.md` rewritten whole.
  - Whitepaper: where the reward's path, the critic and the task are described, the rule and the untold delay; and the tables written by `node scripts/adr-index.mjs`.
  - `CHANGELOG.md`, one entry of at most 400 characters a change.
  - `README.md` and `docs/zh-TW/README.md` as the result requires.
  - The whitepaper's version in both declarations, with its date.
- [ ] **The brief archived** with `spec-brief archive 082 --strict`, every deliverable dispositioned. The round's ADRs are set to `accepted` by the commit that asks for the merge, and the pull request is marked ready for review with it.

## Not empowered

- **The round's scope** is the front matter's. It writes only inside `affectedFiles`, and never into `protectedFiles`. A file it must change outside the scope is handled as `briefs/README.md` says.
  - The critic's rule, the trace and `crates/` do not change; `crates/` is protected.
  - The default, the read, the probe and the kept addresses' rule for a reward that names its age do not change.
- **The window of 64, the run of 64, the lead of a thirty-second and the latch are ADR-0216's**, and so are `kept_addresses` 2, the delays 0, 1 and 2, the arms and the networks. None is fitted to the result, and none other is run.
- **No record, section or format changes.** The rule is not in the image.
- H-45's clauses, its constants and its predicted readings do not move after such a run. There is no second attempt, and a run is not repeated for a check of the harness.
- No arm's log is read before the calibration is green.
- No whole-domain test runs on the developer machine.
- No pinned number of an earlier round moves.
- No float anywhere, no dependency, no `unsafe`, and no rebase of the branch once a dispatch has run on it.

## Architectural empowerment

The executing session may replace any instruction above with a better decision, recorded in its ADR. That covers:
- the names of the field, the call and their errors, and how the critics, their windows and the counts of spent entries are held in the executor;
- how the task carries its untold delay, and how a reward given without an age is called from it;
- how a score and a lead are computed in fixed point, so long as each is ADR-0216's quantity exactly;
- how the arms, the controls and the readings are dealt into tests and dispatches, and what of exploration 003's tests is taken;
- how each reading is computed and pinned;
- a file the round must change and its scope does not hold, within this brief's mission, widened as `briefs/README.md` says.

It may not:
- write into a protected file, or shrink `protectedFiles`;
- change a delay, `kept_addresses`, the window, the run, the lead, the latch, a network, the task's feedback or the schedule;
- put the rule in the image;
- move a clause, a constant or a predicted reading after such a run;
- run a whole-domain test on the developer machine;
- reach the standing directives, the whitepaper's invariants or the constraints in `CLAUDE.md`.

If the calibration fails, the round stops there as H-45's stopping rule says: the finding is written, and no arm's log is read into a table.

## Verification

Before each push, on the developer machine:

```bash
cargo check --workspace --all-targets --locked
cargo test --workspace --locked
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --locked -- -D warnings
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --locked
npm ci
npm run spec
```

The pull request's gate runs those and the rest: the release tests, the benchmarks once, the MSRV, AArch64 and the mutation gate on the changed lines. Every check passes on the last commit.

By dispatch, never on the developer machine:

```bash
npm run round:additive -- origin/main <the protocol's commit>
gh workflow run ci.yml --ref <this round's branch> -f scope=exhaustive -f exclude='<this round's own tests>'
gh workflow run ci.yml --ref <this round's branch> -f scope=arms -f tests='<the eighteen arms>'
gh workflow run ci.yml --ref <this round's branch> -f scope=exhaustive -f only='<this round's own tests>'
node scripts/exhaustive-costs.mjs from <both runs' downloaded artifacts> --run '<calibration> and <reproduction>' > scripts/exhaustive-costs.tsv
node scripts/adr-index.mjs
npx --no-install @descent-vtt/spec-brief archive 082 --strict --pr <n> --summary "<one paragraph>"
```

- The calibration's shards are green with the round's tests excluded, and its check holds.
- Each arm's job exits 0 with one test passed and its log kept.
- The reproduction's shards are green and reproduce every pinned number.
- In the history, H-45's constants and its predicted readings precede every dispatch that runs an arm, and every commit a dispatch ran on is an ancestor of the pull request's head.
- The archive passes `--strict`: nothing outside `affectedFiles` and nothing in `protectedFiles` changed.

## Report

The closing message states:
- the rule and the untold delay as built, and that every run with the rule unset, and every reward that names its age, is the run it was;
- the protocol, when it was committed, and what `round:additive` read;
- H-45's verdict per network, arm, delay and clause, and the prediction and each predicted reading against what was read;
- each arm's move, its trial and its age, beside H-43's told arm and exploration 003's switch arm;
- the step of the stopping rule reached and the next decision it names, not taken;
- each dispatch with its run id and commit: the calibration, the arms and the reproduction, and whether the reproduction named `only`;
- any widening of `affectedFiles`, and why;
- that no whole-domain test ran on the developer machine, that each run ran once, and that no pinned number moved;
- each shard's job time against its bound and the regenerated cost table;
- that the pull request is ready for review;
- what was not done and why.
