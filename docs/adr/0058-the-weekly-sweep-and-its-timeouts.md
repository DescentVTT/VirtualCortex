---
status: accepted
date: 2026-09-18
depends-on: ADR-0030
decision-makers: VirtualCortex maintainers
---

# ADR-0058: The weekly sweep's cost is its timeouts, not its mutants — a mutation run per area with a timeout matched to that area's own tests, a completeness check that refuses a partial sweep, and the runtime's area sharded six ways by the rule that measured it

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
- **A timeout the runner measures, not one this file writes down.** Each job first runs the area's own tests as a timed step (`--workspace --exclude cortex-runtime --exclude cortex-bench` for `crates`, `-p cortex-runtime` for `runtime`) and sets the per-mutant bound to three times that elapsed time plus thirty seconds. Three, because the tool's own default is five times a baseline and a bound below the area's suite would record a slow mutant as a timeout, which V-6 counts as caught: a survivor hidden by the clock is the one failure mode worse than a slow sweep.

  A first attempt wrote the bounds down — 60 seconds for `crates`, 420 for `runtime`, from suites measured at 6 and 140 seconds on a developer machine — and the dispatched run that measured it ([35249647298](https://github.com/DescentVTT/VirtualCortex/actions/runs/35249647298)) refused them: `TIMEOUT Unmutated baseline in 5s build + 420s test`, exit code 4. Two things that file did not know: `--timeout` bounds **every** cargo command, the tool's own baseline included, so an absolute bound at the scale of the mutants kills the baseline it is measured against; and the runtime's suite, 140 seconds here, had not finished in 420 on a hosted runner. A number measured on a developer machine is not admissible for the engine ([ADR-0010](0010-measured-or-target.md)) and, this run says, not for CI either.

- **The baseline, kept as a step rather than as the tool's.** The sweep runs with `--baseline=skip`, because the timed step above has just run the area's unmutated tests under the job's `-e` shell: if they fail, the job fails there, which is the guarantee the tool's baseline gives. Nothing is given up and the bound no longer has to cover a run it is derived from.
- **A completeness check, per area.** After the run, the job counts the outcomes in `mutants.out` and compares the sum of caught, missed, timeout and unviable with `cargo mutants --list --file "<area>/**" | wc -l`, the same filter under the same configuration. Unequal, or a run that ended without a terminal exit code, fails the job: a partial sweep is a failed job, never a quiet artifact. Survivors still block nothing, as ADR-0030 has it; the artifact keeps its ninety days and its name now carries the area.
- **The rule for sharding, and what it decided.** An area that overruns its job's own limit is sharded at once, because it produced no sweep at all; an area that finishes but takes over three hours is sharded after two consecutive weekly runs. `n = ceil(measured / 90 minutes)`, with `--sharding round-robin` rather than the tool's default `slice`, because the expensive mutants of this tree cluster by file (verified with the pinned tool: `--shard 0/8` is 425 mutants from `cortex-affect` to `cortex-connectome`, `--shard 1/8` another 425 beginning inside `cortex-core`). Each shard checks its own slice with the same filter and the same shard arguments, which is what the tool requires of every shard of one sweep, and an area is complete when each of its shards is; `fail-fast: false` keeps one shard's failure from hiding what the others found. There is no merge job, which is where this differs from the sibling repositories: their gate is a score over the whole sweep and has to be merged before it can be applied, and here there is no score, since ADR-0030's weekly level blocks nothing.

  The first run under this shape ([35252449360](https://github.com/DescentVTT/VirtualCortex/actions/runs/35252449360)) decided it. **`crates`: 2 417 of 2 417 mutants in 24 minutes** (2 311 caught, 13 missed, 77 unviable, 16 timeouts; job 27 m 16 s), with the area-scoped baseline at 2 s build and 3 s test against the workspace's 434, so this area is one job and needs no shard. **`runtime`: 590 of 979 in 5 h 23 m**, then cancelled by the job's 330-minute limit (544 caught, 8 missed, 4 timeouts, 34 unviable). Its suite measures **359 seconds** on a hosted runner, so its bound was 1 107; the four timeouts are 23 per cent of that time and the rest is real test time at a mean of about 65 seconds per mutant, because a mutant of the runtime runs the runtime's own suite until a test fails. **Sharding is this area's lever and a tighter bound is not** -- the opposite of the crates area, and the reason the diagnosis had to come before the remedy. At 979 over 590 the area is about nine hours, so `n = 6`; the six shards are 164 + 163 x 5 = 979, verified against `cargo mutants --list` under the same arguments.
- **What is measured and what is not.** Splitting by area was half the answer on its own: the tool scopes its baseline to the packages that hold the area's mutants, so `crates` fell from a 434-second baseline to three seconds and from an implied 2 172-second bound to 39. What is **not** measured is whether the six runtime shards finish inside their job; the run that follows this change measures it, and if a shard overruns, the rule above applies to `n` again. What the whole sweep now costs is not predicted here: the run that follows this change measures it, and the number enters Appendix B and this ADR's consequences when it exists. No figure in this ADR is a timing claim about the engine; every one is the cost of running its tests ([ADR-0010](0010-measured-or-target.md) governs the engine's figures, not CI's wall clock).
- **What the first complete sweep found.** Thirteen survivors under `crates/**` and eight under `runtime/**` in the 60 per cent of that area which ran. They are not this ADR's subject and they block nothing: ADR-0030 makes them the next round's list, and whitepaper §11.1 carries them as one.

### Consequences

- Good: the weekly level can complete, and when it cannot, it fails loudly instead of publishing a partial list.
- Good: the diagnosis is recorded with its numbers, so the next person to find the sweep slow starts from the distribution rather than from the mean.
- Good: no change to the in-diff gate, which took 1 m 29 s on the pull request that preceded this one, and no change to what the tool tests or to any rule of the engine.
- Bad: two jobs where there was one, and a completeness check that must be kept in step with the areas if a third area is ever added; the check fails closed, so a forgotten area shows up as a mismatch rather than as a gap.
- Good: the bound is derived on the runner that will use it, so it does not age and does not have to be re-measured when the tests or the runners change.
- Bad: the timed step pays the area's suite once per job before the sweep, and the factor of three is a judgement, not a measurement; an area whose timeouts are not hangs has outgrown it, which the artifact shows because a hang and a slow pass are both listed in `timeout.txt` with their mutants.

## Alternatives considered and why rejected

- **`--baseline=skip` with nothing in its place.** Refused, and this is why the timed step exists: skipping the baseline without running the unmutated tests would let a broken suite report every mutant as caught. The step is the baseline, outside the bound.
- **`--timeout-multiplier` instead of an absolute timeout.** The multiplier is relative to the workspace baseline, which is the very mismatch this ADR removes.
- **Excluding the hanging mutants by name in `.cargo/mutants.toml`.** They are caught, and an exclusion would hide a real defect if one of those functions later stops terminating for another reason. The cost was the timeout, not the mutants.
- **A percentage threshold on the weekly sweep.** Refused by ADR-0030 already, for the reason it gave: a threshold either fails on an equivalent mutant or is set low enough to pass anything.
