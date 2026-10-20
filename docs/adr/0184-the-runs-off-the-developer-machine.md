---
status: accepted
date: 2026-10-17
amends: ADR-0150
title: The runs off the developer machine
kind: process
summary: A round's runs now go by dispatch, the calibration at `scope=exhaustive` with the round's own tests excluded and the arms and any trial at a new `scope=arms` on AArch64. Before a push the developer machine runs the fast checks and the pull request's gate runs the rest.
decision-makers: VirtualCortex maintainers
---

# ADR-0184: The runs off the developer machine — a round ran its calibration, three and a quarter to nearly five hours of every whole-domain test eight at a time, and its arms and trials, half an hour to an hour more, on the developer machine, a laptop its maintainer works at, and the weekly shards then ran the same tests again to reproduce them; a round's runs now go by dispatch: the calibration at `scope=exhaustive` with the round's own tests excluded by name, the arms and any trial at a new `scope=arms`, one job a test on AArch64 with its output kept, reproduced on x86-64 by the shards as before; before a push the developer machine runs the fast checks and the pull request's gate runs the rest

## Context and Problem Statement

**What a round runs on the developer machine**, read from the ADRs of the last six measuring rounds:

| Round | The calibration, eight at a time | The arms, and trials of the harness |
| :--- | :--- | :--- |
| Brief 068 ([ADR-0169](0169-twice-the-size-measured.md)) | | four arms at 2 048 units, 63 m |
| Brief 069 ([ADR-0172](0172-a-reward-right-three-times-in-four-measured.md)) | 108 tests, 3 h 23 m | the arms, 25 m |
| Brief 070 ([ADR-0174](0174-three-in-four-on-the-other-networks-measured.md)) | 110 tests, 4 h 24 m | seven pinned arms through a new body, 43 m; the arms, 31 m |
| Brief 071 ([ADR-0177](0177-the-controllers-step-measured.md)) | 115 tests, 3 h 19 m | no arm |
| Brief 072 ([ADR-0181](0181-a-rate-the-controller-holds-measured.md)) | 120 tests, 3 h 41 m | the arms, 29 m, and again, 26 m, for a check that had missed them |
| Brief 073 ([ADR-0183](0183-a-sixteenth-on-three-networks-measured.md)) | 128 tests, 4 h 47 m | two derivations and six arms after it |

Before every push the round also ran `CLAUDE.md`'s block: the workspace built and tested in debug, in release and on the MSRV, linted, documented, its benchmarks executed once, and the mutation gate on the lines it changed, each a build of the workspace.

**The developer machine** is the maintainer's own: 8 cores and 16 threads, 31 GB, Windows 11. On 2026-10-17 the maintainers reported that a round's runs leave it hard to use, and asked for the cause to be removed rather than the runs made gentler.

**Where else the same runs already happen.**
- **The calibration** checks that every whole-domain test of the tree before the round passes from the round's build. The weekly job runs those tests, and the round's with them, in twelve shards; the slowest shard's job read 65.9 to 91.0 minutes over the last six dispatches ([ADR-0179](0179-the-bounds-rule-restated.md), brief 072's).
- **The arms** run once, and [ADR-0150](0150-a-round-waits-for-what-it-checks.md) made the dispatch's shards their reproduction: *"on other hardware and another operating system, which is the stronger evidence"*.
- **What a run prints does not depend on where it runs.** The whitepaper's first quality goal is that the same seed and inputs produce bit-identical state on x86-64 and AArch64 (§1.3), met by integer-only dynamics and a fixed tick ordering (§4.2). V-7 holds the seeded network's hash on both (Appendix B). Every table a round has pinned since ADR-0150 was written from a run on Windows and reproduced by the shards on Linux. A run's place changes its seconds.
- **A run's seconds are not a result.** A table of a learning run is not a performance figure, and no second of these runs is admissible on either machine ([ADR-0010](0010-measured-or-target.md)).
- **The repository is public**: its hosted runners are not charged by the minute, and the account runs twenty jobs at once.

**What the pull request's gate already runs.** Every line of `CLAUDE.md`'s block but the whole-domain line: `rust` (the check, the tests in debug and in release, the format, the lints, rustdoc, the benchmarks once), `msrv`, `arm64`, `mutants` on the changed lines and `specs`. The block was run before a push so that the push would not be red. The local run of it enforces nothing that the gate does not.

## Decision Drivers

- **The developer machine is its maintainer's to work at**, and a round's calibration alone took it for three and a quarter to nearly five hours.
- **Nothing a run reads depends on where it runs** (V-7, and every reproduction since ADR-0150).
- **A run the host records is better evidence than a run on the developer machine.** Its commit, its log and its time are kept and anyone can read them, and "the protocol committed before any run" can be read from the run's own commit.
- **Two machines stay two**: the reproduction is on other hardware than the run (ADR-0150).
- **A check run only on the developer machine is enforced nowhere** (F-44); a check run only in the gate is enforced, at the price of a push that can be red.

## Considered Options

1. Where a round's runs go:
   - (a) a dispatch: the arms at a new scope on AArch64, the calibration at `scope=exhaustive` with the round's own tests excluded;
   - (b) the same, with the arms on x86-64;
   - (c) the developer machine as now, at a lower priority and on fewer threads;
   - (d) a self-hosted runner.
2. What the developer machine runs before a push:
   - (a) the fast checks: the check, the debug tests, the format, the lints, rustdoc, the documentation gate; the rest the gate's;
   - (b) the whole block, as now.

## Decision Outcome

**Options 1(a) and 2(a).**

### A round's runs

- **No whole-domain test runs on the developer machine.** A round's calibration, its arms, its lead-ins and any trial of its harness are dispatches on its branch.
- **The calibration** is `gh workflow run ci.yml --ref <branch> -f scope=exhaustive -f exclude='<test> ...'`. It is made on the commit the round's runs will be made from, before any rewarded run, with the round's own tests named in `exclude`. The shards take those out of the listing before the deal, and a name the listing does not hold fails every shard before any test runs, since a name misspelt would leave its test in the calibration.
- **The arms, and any test a round runs before them**, are `gh workflow run ci.yml --ref <branch> -f scope=arms -f tests='<test> ...'`, the names as `--list` prints them.
  - Each test runs in a job of its own on `ubuntu-24.04-arm`, by `scripts/arm-run.sh`.
  - The test is found from `--list` at run time and refused if no binary or two binaries list it. It runs with its output not captured, and must exit 0 with one test passed.
  - The job keeps the log, the seconds and the listing as the artifact `arm-<n>`, for 90 days.
- **The round's tables are written from those logs.** Its ADR names the run, the commit the run was made on, and the job each table came from.
- **The arms run once.** The run's commit holds the protocol, and the reviewer reads that from the run. A harness defect found after the run means a second run, as it did before; the ADR says so.
- **The reproduction** is the dispatch at `scope=exhaustive` after the tables are pinned: on x86-64, nothing excluded, as ADR-0150 has it. The cost table is regenerated from it, and the merge waits for it.
- **The arms job's bound is the weekly shard's**, 180 minutes, since they are the same tests, and ADR-0179's rule reads its minutes as it reads a shard's.
- **The scope is part of the concurrency group**, so arms dispatched on a branch do not cancel the calibration dispatched on it before them. A dispatch at `scope=arms` is a group of its own, so no later dispatch cancels a run made once.
- Everything else in a round's order is unchanged: the protocol committed first, the calibration before any rewarded run, and constants that do not move after a rewarded run.

### What the developer machine runs before a push

| Where | What |
| :--- | :--- |
| The developer machine, before a push | `cargo check --workspace --all-targets --locked`, `cargo test --workspace --locked`, `cargo fmt --all -- --check`, `cargo clippy --workspace --all-targets --locked -- -D warnings`, `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --locked`, `npm ci`, `npm run spec` |
| The pull request's gate alone | the tests in release, the benchmarks once, the MSRV's check and tests, the tests on AArch64, the mutation gate on the changed lines |
| A dispatch alone | the whole-domain tests: `scope=exhaustive` for all of them in twelve shards, `scope=arms` for those a round names |

`CLAUDE.md`'s block is split into those three, and `CONTRIBUTING.md` and Appendix B follow it. Every line of the first two rows still runs in the gate, so a line that only the gate runs is enforced as it was.

### What this amends

- **ADR-0150**: *"the arms run once on the developer machine"* is now once by a dispatch, and the calibration *"on the developer machine"* is now a dispatch. Everything else it decided stands. ADR-0150 is accepted and is not edited.
- **`briefs/README.md`'s "Evidencing a round"** says this, and it is the rule where a brief written before it asks for a run on the developer machine. Brief 073 was executing when this was written; it runs as written.

## Consequences

- Good: the developer machine runs no whole-domain test. The longest thing it runs is the debug test suite.
- Good: every run a round's tables come from is recorded by the host, with its commit, its log and its time.
- Good: the reproduction is a second architecture's, where it was a second system's on one architecture.
- Good: a round's arms run side by side in jobs of their own, not eight to a machine.
- Bad: a round waits for the host's queue. A round's calibration is twelve jobs and a sweep thirteen, against the account's twenty at once.
- Bad: a trial of the harness is a dispatch of minutes to an hour, where it was a local run.
- Bad: a push can be red where the local block would have caught the fault, at the cost of one round trip through the gate.
- Bad: the logs are kept for 90 days, and after that only the ADR's tables remain. A local run's logs were never kept at all.
- Neutral: the runs' seconds on a runner are not admissible, as the developer machine's were not.
- Neutral: the ruleset requires `rust`, `msrv` and `specs` by name. The mutation gate and AArch64 are not required checks; the merge waits for them by ADR-0150's rule, the gate *"green on its last commit"*. This ADR changes no ruleset.

## Alternatives considered and why rejected

- **Option 1(b), the arms on x86-64**: the run and its reproduction would be on one architecture and one system, two runners of one image.
- **Option 1(c), a lower priority**: the machine is still taken for the same hours. The maintainers asked for the cause.
- **Option 1(d), a self-hosted runner**: a machine to keep, and on a public repository a self-hosted runner runs the code of any pull request.
- **Option 2(b), the whole block**: the developer machine would build the workspace three more times and run the mutation gate before every push, to catch what the gate catches.

## Confirmation

- **`.github/workflows/ci.yml`**:
  - the scope `arms` and the inputs `tests` and `exclude`;
  - the jobs `arms-plan` and `arms`;
  - the scope in the concurrency group, and a group of its own for each dispatch at `scope=arms`;
  - the weekly shard's step given `EXCLUDE`, its summary and its artifact the excluded list.
- **`scripts/arm-run.sh`**, and **`scripts/exhaustive-shard.sh`**'s exclusion.
- **The exclusion, tried on the listing dispatch 38417864748 kept**, 128 tests:
  - with nothing excluded, the deal is the one it was;
  - with two names given, spaces around them, 126 are left, and the deal's planned sum falls by their 185 and 1 834 s;
  - a name the listing lacks fails, with that name.
- **The scope, tried by dispatch 39033819438 on `1378111`**, before this file was in the tree. Four tests were named: three the tree pins, H-38's step of an eighth and its two arms, and one no binary lists.
  - **The three the tree pins passed on AArch64**, each in its own job, exit 0 with one test passed. An arm holds every table it pins once it has dumped it. So each table H-38's round wrote from a run on Windows, which the shards had reproduced on x86-64, was reproduced bit for bit on AArch64.
  - The step of an eighth ran in 128 s, the arm from the assignment in 999 s and the arm from the mirrored in 1 041 s, with 27, 87 and 87 lines of dump in their artifacts. The jobs took 3, 17 and 18 minutes with the build. The shards read the same tests at 185, 1 834 and 2 269 s, two to a runner. None of these seconds is admissible.
  - The name no binary lists failed its job before anything ran (*"the listing names this test in 0 binaries, where one is required"*), so the run reads as failed, as it should.
- **The final form, tried by dispatches 39034670133 and 39034719490 on `e56e8b5`.** Three dispatches at `scope=arms` ran at once on one branch, and none was cancelled.
  - A paused run, `an_assembly_of_16_units_at_four_weights_paused` ([ADR-0127](0127-the-paused-line-leaves-the-weekly.md)), was found by `--ignored --list` and passed in 97 s, its dump kept.
  - The step of an eighth passed again, in 128 s.
- `CLAUDE.md`'s Commands, `CONTRIBUTING.md`, Appendix B's block, its V-7 row and its directives, `briefs/README.md`, and this decision's rows in §9 and the index.
