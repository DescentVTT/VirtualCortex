---
status: accepted
date: 2026-10-25
amends: [ADR-0075, ADR-0184, ADR-0191]
title: The round shortened
kind: process
summary: A round dispatches its calibration with its arms, and its reproduction runs only the round's own tests. The whole-tree sweep runs on the weekly schedule alone, and a pull request of documents alone skips the four Rust jobs. Brief 080's sweep had not ended; F-80.
decision-makers: VirtualCortex maintainers
---

# ADR-0211: The round shortened

**In brief**
- **Decision.** A round dispatches its calibration on the same commit and at the same time as its arms, and reads the calibration first. Its reproduction runs only the round's own tests. No round dispatches the whole-tree sweep; the schedule runs it every Monday. A pull request whose every file is a document skips the four jobs that build Rust.
- **Why.** Brief 080's dispatches took 3 h 35 min from its calibration to its reproduction's last shard, and 166 of the reproduction's 184 tests were the calibration's again. Its sweep then held sixteen of the account's twenty places for hours.
- **Left open.** Where the rounds' pinned tables live (F-76, F-78), measured by an exploration first. Then whether a calibration need run only what a change can reach.

## Context and Problem Statement

[ADR-0150](0150-a-round-waits-for-what-it-checks.md) made a round wait for what it checks, and [ADR-0184](0184-the-runs-off-the-developer-machine.md) moved its runs to dispatches.
Brief 080's round made four on 2026-10-25 ([H-43's report](../research/H-43.md)):

| Run | Scope | Dispatched | Ended | What it ran |
| :--- | :--- | :--- | :--- | :--- |
| [40164773490](https://github.com/DescentVTT/VirtualCortex/actions/runs/40164773490) | `exhaustive` | 05:14Z | 06:37Z | the calibration, 166 tests in 18 shards, 40.2 to 82.8 minutes a shard |
| [40164780397](https://github.com/DescentVTT/VirtualCortex/actions/runs/40164780397) | `arms` | 05:14Z | 06:47Z | the six twins, 1 061 to 1 117 s each |
| [40170518252](https://github.com/DescentVTT/VirtualCortex/actions/runs/40170518252) | `arms` | 06:50Z | 07:10Z | the twelve arms at a delay |
| [40171928796](https://github.com/DescentVTT/VirtualCortex/actions/runs/40171928796) | `both` | 07:13Z | 08:49Z, its last shard | the reproduction, 184 tests, 54.0 to 95.3 minutes a shard; its sweep began at 08:48Z |

What each step cost, and what it checked:
- **The calibration before the arms.** The arms waited 83 minutes for the calibration's last shard. The twins ran beside it, each in under 19 minutes, and their run still took 93, waiting for places: the account runs at most twenty jobs at once.
- **The reproduction ran every test again.** 166 of its 184 tests were the calibration's, at the calibration's source: [ADR-0191](0191-a-rounds-scope-eighteen-shards-and-the-reproduction-first.md) lets no file under `src/` change after the calibration unless it is calibrated again. Only the round's own 18 were new to x86-64.
- **The sweep ran on every round whose diff changed a file under `src/`** ([ADR-0075](0075-the-dispatch-scope-follows-the-diff.md)). No round waits for it, but its seventeen jobs hold the places: brief 079's held them from 16:58Z to 22:30Z on 2026-10-24 ([ADR-0207](0207-a-reward-past-the-trial-its-address-kept.md)).
- **A pull request of documents alone runs the Rust jobs.** Pull request #233 changed two documents. It was opened at 05:42Z, behind the calibration's shards; its MSRV job ran 25 minutes and its Rust job 17, the last ending at 06:47Z, for a diff no Rust build reads.

## Decision Drivers

- **The maintainers' choices** (2026-10-25): the sweep weekly only; the calibration with the arms; the reproduction of the round's own tests; documents alone skip the Rust jobs; no round waits for the sweep.
- **What each run checks is kept.** Every whole-domain test runs once on x86-64 at the round's source, in the calibration or the reproduction; the arms run on AArch64; the whole tree's mutants are swept every week.
- **A survivor found by the schedule costs days**: it blocks nothing and is the next round's list ([ADR-0030](0030-verification-governance.md)), as ADR-0075 priced a sweep wrongly skipped.
- **Latest ≠ Newest**: the change is to this repository's workflow and script. The job that reads a pull request's files is `git diff`, not an action from the marketplace.

## Considered Options

1. **The sweep**: (a) on the schedule alone; (b) as ADR-0075, on every round that changed `src/`.
2. **The calibration**: (a) with the arms, read before them; (b) before them, as ADR-0184.
3. **The reproduction**: (a) the round's own tests, when nothing the calibration ran has changed; (b) every test.
4. **A pull request of documents alone**: (a) the Rust jobs skipped by their conditions; (b) skipped step by step inside each job; (c) run.

## Decision Outcome

**Options 1(a), 2(a), 3(a) and 4(a)**, built in this pull request.

### The sweep on the schedule

- **A round dispatches `exhaustive` and `arms`**, never `mutants` or `both`. The scheduled run, Monday at 03:00Z, sweeps the whole tree as before.
- **The dispatch's default scope is `exhaustive`**, where it was `both`, so a dispatch that names none runs no sweep.
- **The pull request's mutation gate** still meets every line the round changes (ADR-0030).
- **`mutants` and `both` stay for the maintainers**, by hand, for a change to `.cargo/mutants.toml` or to the sweep's job that should be read before Monday.

### The calibration with the arms

- **The calibration** (`scope=exhaustive -f exclude=…`) **and the arms, twins and lead-ins** (`scope=arms`) are dispatched together, on the protocol's commit, which ADR-0184 already puts both on.
- **The calibration is read first.** A twin, an arm a stopping rule holds bit for bit to a pinned one, is read with it. The arms' logs are read, and their tables written, only once every calibration shard and every twin is green.
- **A calibration that fails stops the round as a finding**, as before, and its arms are void: no table is made from them.
- Nothing else in a round's order moves: the protocol committed before any rewarded run, and no constant moved after one.

### The reproduction of the round's own tests

- **A new input, `only`**: `gh workflow run ci.yml --ref <branch> -f scope=exhaustive -f only='<test> ...'`. The shards deal those tests alone (`scripts/exhaustive-shard.sh`).
  - A name the listing does not hold fails every shard before any test runs, as `exclude` does.
  - `only` with `exclude` fails.
  - A shard dealt no test passes, having run nothing.
- **When a round names `only`**: from the calibration's commit to the reproduction's, no file under `src/` changed, and no test or helper the calibration ran; the diff is the round's own tests and documents. Otherwise its reproduction runs every test, as before.
- **That is read, not checked.** The round reads it from `git diff <calibration>..<reproduction>`, and the session that merges reads it again. Nothing holds it mechanically.
- **A dispatch naming `only` is a concurrency group of its own**, as one at `arms` is, so that a reproduction dispatched while the calibration still runs does not cancel it.
- **The cost table** is regenerated from the calibration's artifacts and the reproduction's together, each test in one of the two: both downloaded under one directory, then `node scripts/exhaustive-costs.mjs from <dir> --run '<calibration> and <reproduction>'`.
- **The schedule runs every test**, with `only` and `exclude` empty.

### Documents alone skip the Rust jobs

- **A new job, `changes`**, on a pull request: when every changed file is under `docs/` or `briefs/`, or is a Markdown file at the repository's root, the `rust`, `msrv`, `arm64` and `mutants` jobs are skipped by their conditions.
- **The required checks still pass.** A job skipped by its condition reports success, and GitHub counts a skipped required check as passed. The ruleset requires `rust`, `msrv` and `specs`, and `specs` always runs.
- **Any other file runs them all**: this workflow, the scripts and the manifests among them. So do a failed `changes` job, a push and a dispatch.

### No round waits for the sweep

- Stated again, since the sweep now comes on the schedule: no round, merge or ADR waits for it. The first ADR merged after a sweep ends records it ([ADR-0158](0158-the-sweep-off-the-next-decisions-path.md)).

### Brief 080's sweep, and F-80

- **Run 40171928796's sweep had not ended** when this record was written: its crates job green at 09:17Z, its sixteen runtime jobs running since 08:48Z. The first ADR merged after it ends records it.
- **F-80**: the workflow's header said the runtime's sweep ran in twelve shards after ADR-0207 made it sixteen. Resolved by this record: the header says sixteen.

### What it saves (Target)

Against brief 080's runs, to be read again from the next round's:
- **The calibration with the arms**: the arms no longer wait for its last shard, but share its places, so about half an hour at most.
- **The reproduction**: a shard builds and runs about one test, where brief 080's took 54.0 to 95.3 minutes; about an hour.
- **Documents alone**: the Rust jobs' 17 to 25 minutes on each such pull request, and their wait for places.
- **The sweep**: two to five and a half hours of sixteen places, now taken once a week rather than by every round with source.

## Consequences

- Good: a round's dispatches end about two hours after its calibration starts, where brief 080's took 3 h 35 min.
- Good: between rounds the places are free for the gate and the next calibration.
- Bad: a survivor that a round's source leaves outside its changed lines is found on the Monday after, not by the round's own sweep.
- Bad: arms run beside a calibration that may fail, and their machine time is then spent for nothing.
- Bad: whether a reproduction may name `only` is read from a diff by people, not by a check.
- Bad: a shard dealt no test still builds and lists the tests, a few minutes of a runner.
- Neutral: no code, test or pin of the engine changes.

## Amended

- **ADR-0075**: a round no longer dispatches `both` when its diff changes `src/`, deletes a test or changes the sweep's configuration. The schedule sweeps.
- **ADR-0184**: the calibration is no longer made *"before any rewarded run"*. It is made with them, and read before them.
- **ADR-0191**: the dispatch after the tables are pinned reproduces them by running the round's own tests, under the condition above.
- Each is accepted and is not edited.

## Alternatives considered and why rejected

- **Option 1(b), a sweep per round**: every round with source paid two to five hours of places for what Monday's run also finds.
- **Option 2(b), the calibration first**: the arms waited 83 minutes for a check that stops the round either way.
- **Option 3(b), every test again**: 166 tests ran again at a source the calibration had already run.
- **Option 4(b), skipped step by step**: the jobs would still take places and start runners, which is the wait this removes.
- **Option 4(c)**: today's cost, for a build no document is read by.

## Confirmation

- `scripts/exhaustive-shard.sh` was run with a fake `cargo` and test binaries: two of four tests named in `only` over four shards ran one test each in two shards and none in two. A misspelt name, and `only` with `exclude`, each failed before any test ran, and `exclude` and no filter ran as before.
- This pull request changes the workflow, so its gate runs the Rust jobs. The first pull request of documents alone after it shows them skipped and its merge allowed.
- The next round's reproduction is the first dispatch with `only`, and its record reads the time it took.
- `npm run spec` holds the directives that name the input, the job and the default, and the links and tables.
