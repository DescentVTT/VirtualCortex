---
status: accepted
date: 2026-09-20
amends: ADR-0060
decision-makers: VirtualCortex maintainers
---

# ADR-0061: The learning runs leave the gate — brief 027's five full runs at 256 units become weekly `exhaustive` tests and the gate keeps the rewarded run's first block, because a run in the gate is paid again by every runtime mutant of the weekly sweep

## Context and Problem Statement

[ADR-0058](0058-the-weekly-sweep-and-its-timeouts.md) sweeps `runtime/**` in six shards, each timing the runtime's own test suite on its runner and bounding every mutant at three times that plus thirty seconds; every runtime mutant runs that suite until a test fails. [ADR-0060](0060-the-reward-path-measured.md), within what brief 027 allowed, added five runs of $2^{21}$ ticks at 256 units to the pull request's gate ("the gate's runtime tests gain five runs of $2^{21}$ ticks at 256 units"). A run in the gate is therefore paid once per pull request and again, in the debug profile, by each of the runtime's mutants every week.

The weekly run dispatched on `main` after brief 027 merged measured the price. Run [35444829805](https://github.com/DescentVTT/VirtualCortex/actions/runs/35444829805), `main` at `07c6774`, against run `35284722491` before it:

| | Before brief 027 | After brief 027 |
| :--- | ---: | ---: |
| The runtime suite on the hosted runners | 216 to 464 s | 709 to 854 s |
| The six runtime shards | 1 h 10 m to 1 h 58 m | 3 h 27 m to 4 h 19 m |
| The slowest shard's distance from the job's 330-minute limit | 2 h 32 m | 71 m |
| Runtime mutants | 979 | 1 060 (81 in `task.rs`) |
| Survivors over the whole tree | 21 | 21, the same list |
| Timeouts over the whole tree | 35 | 39 |

Every shard crossed ADR-0058's three-hour mark, and with runners varying by two to one on identical work (ADR-0058), 71 minutes is one slow runner from a cancelled sweep. On a developer machine (not admissible, a ratio only) the runtime suite was 140 s before brief 027 and 391 s after it, and the slowest tests are the five learning runs beside the two waking days that were there before. Brief 029 is about to add more learning runs.

The four new timeouts are `SpinBarrier::wait` (`barrier.rs:41`), `trace` in `branching.rs:100`, the `clauses` field of a `Config` expression in `Image::decode` (`image.rs:535`) and `Induction::search` (`store.rs:259`); brief 028 re-derives its lists from the latest weekly run and classifies whatever it finds there.

## Decision Drivers

- The gate is for rules and the tests that constrain them; a full run of an experiment is a measurement whose numbers are pinned, and the repository's place for a heavy pinned run is the weekly job ("a heavy exit test runs in the weekly job as an ignored test whose name contains `exhaustive`").
- The five runs cost several minutes in the debug profile and 12.35 s together in the release profile (developer machine): the weekly job, which runs in release, pays almost nothing for them; the gate and the sweep, in debug, pay the most.
- The loop must still run end to end on every pull request, so that the in-diff mutation gate on `task.rs` has a trial to fail.
- Every number ADR-0060 pinned must still run, unchanged.

## Considered Options

1. **Reshard `runtime/**` by ADR-0058's rule** after a second run over three hours. It answers the symptom with more jobs, and every later learning run in the gate multiplies again by every runtime mutant.
2. **The five full runs become weekly `exhaustive` tests, and the gate keeps the rewarded run's first block.**
3. Option 2 and the criterion's test with them. The criterion's test reads the pinned tables and runs nothing; moving it would take the verdict out of the gate for no saving.
4. Leave `tests/learning.rs` out of the mutants' test run. A mutant of `task.rs` would then meet no trial at all; refused.

## Decision Outcome

Option 2, in `runtime/cortex-runtime/tests/learning.rs`.

- **The five full runs** at 256 units gain `#[ignore]` and the suffix `_exhaustive`, so that the weekly job's `--ignored exhaustive` runs them: `the_rewarded_run_at_256_units_on_four_workers_exhaustive`, `…_on_one_worker_is_the_same_run_exhaustive`, `the_shuffled_reward_at_256_units_exhaustive`, `the_fixed_modulation_at_256_units_exhaustive`, `the_mirrored_assignment_at_256_units_exhaustive`. Their tables and their trace are unchanged; all five pass in the release profile.
- **The gate's run**, `the_first_block_of_the_rewarded_run_at_256_units`: sixty-four trials, held to the first row of the rewarded run's pinned table (`REWARDED_256[..1]`, the block of 35 correct trials). A trial does not read how many follow it, so the first block of a short run is the first block of the long one; the test passes, at an eighth of one full run's ticks (9 s in debug on a developer machine). No new number is pinned: the row is ADR-0060's.
- **The criterion's test** stays in the gate, reading the pinned tables.
- **What it measured.** On a developer machine the runtime suite takes 170 s warm after the move, against 391 s before it and 140 s before brief 027. The figures that decide are the runners': RUNNERS_AFTER_MOVE.
- **ADR-0058's rule is not applied**: its second run over three hours is the one this change removes the cause of. If the runners' shards stay over three hours, the rule applies as written.

### Consequences

- Good: the sweep's runtime shards return toward their cost before brief 027, and brief 029's full runs, which its brief places in the weekly job, do not multiply by the runtime's mutants.
- Good: every number ADR-0060 pinned still runs every week, at both sizes, in the profile the engine ships in.
- Bad: a pull request no longer runs the learning runs past their first block. A change that moves block two onward and leaves block one alone is caught by the weekly job, not by the gate; the weekly job blocks nothing, so it would be the next round's to find.
- Neutral: `task.rs`'s mutants now meet the module's own tests and the first block in the sweep; whether each one the full runs caught is still caught is read from the dispatched sweep below.
