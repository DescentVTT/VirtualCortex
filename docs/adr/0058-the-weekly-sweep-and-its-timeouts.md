---
status: accepted
date: 2026-09-18
depends-on: ADR-0030
decision-makers: VirtualCortex maintainers
---

# ADR-0058: The weekly sweep's cost is its timeouts, not its mutants — a mutation run per area with a timeout matched to that area's own tests, a completeness check that refuses a partial sweep, and the rule that decides whether the runtime's area is sharded

## Context and Problem Statement

[ADR-0030](0030-verification-governance.md) gives mutation testing two levels: the blocking gate on the lines a pull request changes (`--in-diff`), and a weekly whole-tree run that "blocks nothing, since there is no pull request to block, and its survivors are the next round's list". Whitepaper Appendix B's V-6 row says the same, and ADR-0030's rejected option priced a whole-tree run in CI at "66 minutes per push".

The weekly level has never produced a complete sweep (F-38). The only scheduled run on record, `34822921422` of 2026-09-14, ran for 5 h 00 m 57 s and its mutation step was cancelled by the job's own `timeout-minutes: 300`; the step that uploads `mutants.out` runs `if: always()`, so the run published a `missed.txt` of eight survivors drawn from 36 per cent of the tree, with no mark saying so. A reader taking that file as the week's answer would be reading a partial sweep as a result, which is the class of claim principle 3 and [ADR-0010](0010-measured-or-target.md) exist to prevent.

The artifact of that run measures why, and the measurement is not what the shape of the problem suggests. Of 3 396 mutants the tool generates over this tree, 1 233 were tested before the cut, in 31 597 seconds of work across two jobs:

| Outcome | Mutants | Seconds | Share of the time |
| :--- | ---: | ---: | ---: |
| Caught | 1 153 | 1 134 | 3.6 % |
| Missed | 8 | 10 | 0.03 % |
| Unviable | 57 | 31 | 0.1 % |
| **Timeout** | **14** | **30 420** | **96.3 %** |

The median mutant costs **zero seconds**; the mean is 25.6 only because fourteen mutants cost 2 172 to 2 173 seconds each. That number is the tool's automatic test timeout, five times the baseline test run, and the baseline is the **workspace's** test suite (build 7.7 s, test 434 s) while each mutant runs only **its own package's** tests. Measured on a developer machine at `07569a3`, `cargo test -p cortex-core --locked` is 1 second and `cargo test -p cortex-runtime --locked` is 140 seconds: for a state crate the automatic timeout is roughly two thousand times its own suite, so a mutant that hangs a one-second test suite is paid for at thirty-six minutes.

The two areas of the tree also differ in kind, which the same listing shows: `crates/**` holds 2 417 mutants whose suites are `#![no_std]` unit tests of about a second, and `runtime/**` holds 979 whose suite is 140 seconds because it runs the reference network's days at 256 units ([ADR-0053](0053-the-waking-day-and-the-target-period.md)). One timeout cannot be right for both.

## Decision Drivers

- ADR-0030's weekly level must produce a sweep that finished, or say that it did not. A survivor list from a cancelled run is worse than no list.
- GitHub-hosted jobs are capped at six hours and the cap cannot be raised, so a sweep that needs twelve is not a timeout to raise but a run to make cheaper or to split.
- "Latest ≠ Newest" and principle 2: the tool's own `--timeout` and `--file` have been in `cargo-mutants` for years and are what the tool's documentation recommends for CI; the numbers below were re-derived from the run's artifact and from the pinned tool (27.1.0) on this tree, not from the shape of the problem.
- A hang is a detection: Appendix B's V-6 already says "a mutant that hangs a test is counted as caught", so cutting a hang short costs no information, only the wall clock it was burning.
- Principle 5: a structural boundary beats a reviewed one. "The sweep was complete" should be an arithmetic identity a job checks, not a reader's assumption.
- The sibling repositories answered the same problem by sharding (`spec-guard`, four shards against a 45-minute limit at 42 m 39 s; `spec-graph`, four shards at 166 to 171 minutes), and their merge job "refuses anything that is not exactly one sweep". The second half of that answer applies here; the first half is premature until the timeout is fixed, because 96 per cent of this tree's sweep is fourteen mutants and no amount of sharding makes them cheaper.

## Considered Options

1. Raise `timeout-minutes` and keep one job. Refused by the platform: the sweep needs about twelve hours against a six-hour cap.
2. Shard the run as it is, eight ways, as the sibling repositories do. Parallelism without a diagnosis: the hangs cluster by file (`cortex-core`'s `stp_decay_factor_q16` and `Chain::next`, `cortex-arithmetic`'s search, `cortex-hippocampus`), and the tool's default sharding is `slice`, which assigns consecutive mutants to a shard, so the hangs would concentrate and the slowest shard could still exceed the cap.
3. **A run per area with a timeout matched to that area's own tests, a completeness check per area, and a stated rule for when the runtime's area is sharded.**
4. Drop the weekly level and keep only the in-diff gate. Refused: the gate only sees the lines a round changes, so a rule that was never covered is never revisited, which is the hole the weekly level exists to find.

## Decision Outcome

Option 3, in `.github/workflows/ci.yml`.

- **Two areas, two jobs, one matrix.** `crates` runs `cargo mutants --file "crates/**"`, `runtime` runs `cargo mutants --file "runtime/**"`; together they are the whole tree, which the completeness check asserts rather than assumes. The benchmark crate stays excluded by `.cargo/mutants.toml`.
- **A timeout matched to the area.** `crates` takes `--timeout 60`, sixty times the measured one-second suite of a state crate; `runtime` takes `--timeout 420`, three times its measured 140 seconds. Both are the tool's `-t/--timeout`, an absolute bound that does not inherit the workspace baseline. A legitimate suite that grows past its bound shows up as a timeout in an area whose median is zero, which is a reading, not a silent cost.
- **A completeness check, per area.** After the run, the job counts the outcomes in `mutants.out` and compares the sum of caught, missed, timeout and unviable with `cargo mutants --list --file "<area>/**" | wc -l`, the same filter under the same configuration. Unequal, or a run that ended without a terminal exit code, fails the job: a partial sweep is a failed job, never a quiet artifact. Survivors still block nothing, as ADR-0030 has it; the artifact keeps its ninety days and its name now carries the area.
- **The rule for sharding, written before the first capped run.** If an area's job exceeds three hours in two consecutive weekly runs, that area gains `--shard k/n --sharding round-robin` at `n = ceil(measured / 90 minutes)`, with the same completeness check summed across the shards. Round-robin, not the default `slice`, because the expensive mutants of this tree cluster by file: verified at `07569a3` with the pinned tool, `--shard 0/8` is 425 mutants running from `cortex-affect` to `cortex-connectome` and `--shard 1/8` another 425 beginning inside `cortex-core`.
- **What is expected and what is measured.** With the caps above, the fourteen hangs of the 2026-09-14 run would have cost 840 seconds instead of 30 420. What the whole sweep now costs is not predicted here: the run that follows this change measures it, and the number enters Appendix B and this ADR's consequences when it exists. No figure in this ADR is a timing claim about the engine; every one is the cost of running its tests ([ADR-0010](0010-measured-or-target.md) governs the engine's figures, not CI's wall clock).

### Consequences

- Good: the weekly level can complete, and when it cannot, it fails loudly instead of publishing a partial list.
- Good: the diagnosis is recorded with its numbers, so the next person to find the sweep slow starts from the distribution rather than from the mean.
- Good: no change to the in-diff gate, which took 1 m 29 s on the pull request that preceded this one, and no change to what the tool tests or to any rule of the engine.
- Bad: two jobs where there was one, and a completeness check that must be kept in step with the areas if a third area is ever added; the check fails closed, so a forgotten area shows up as a mismatch rather than as a gap.
- Bad: an absolute timeout is a number that ages. The rule above says what moves it: an area whose median is zero and whose timeouts are not hangs has outgrown its bound, and that is a measurement, not a guess.

## Alternatives considered and why rejected

- **`--baseline=skip` to save the baseline run.** The baseline is 434 seconds once per job, which is small beside the areas' runs and is what distinguishes "this mutant was caught" from "these tests were already failing". Kept.
- **`--timeout-multiplier` instead of an absolute timeout.** The multiplier is relative to the workspace baseline, which is the very mismatch this ADR removes.
- **Excluding the hanging mutants by name in `.cargo/mutants.toml`.** They are caught, and an exclusion would hide a real defect if one of those functions later stops terminating for another reason. The cost was the timeout, not the mutants.
- **A percentage threshold on the weekly sweep.** Refused by ADR-0030 already, for the reason it gave: a threshold either fails on an equivalent mutant or is set low enough to pass anything.
