---
status: accepted
date: 2026-09-21
amends: ADR-0067
decision-makers: VirtualCortex maintainers
---

# ADR-0075: The dispatch's scope follows the diff — a round dispatches the whole-tree sweep only when its actual change can have left a mutant uncaught (source under `src/`, a test removed or moved out of the swept suite, the sweep's own configuration), decided by the executing round from the diff it has rather than written into the brief before the diff exists; adding a test never qualifies, because an added test can only catch more

## Context and Problem Statement

[ADR-0067](0067-the-weekly-dispatch-has-a-scope.md) split the weekly dispatch into `exhaustive` and `mutants` and gave a round a rule for asking for the sweep: when it "**added or removed a test**, moved one between the pull request's gate and the weekly job, or changed `.cargo/mutants.toml` or the sweep's jobs". It also put the choice in the brief: "A brief's evidence deliverable names the scope it asks for and why, so the choice is in the frozen input rather than in the executing session's judgement."

Both halves were wrong, and four rounds show how. Every measuring round since ADR-0067 added tests, so every one was sent to the sweep by its brief:

| Round | Files under `src/` changed | The sweep's slowest shard | The exhaustive job | What the sweep could find |
| :--- | ---: | ---: | ---: | :--- |
| Brief 030 ([ADR-0068](0068-the-reward-addressed.md), #77) | **3** | — | — | a mutant the new `executor.rs` and `task.rs` code left uncaught |
| Brief 031 ([ADR-0070](0070-where-256-units-settle.md), #80) | **0** | 111 m | 69 m | nothing |
| Brief 032 ([ADR-0072](0072-what-the-trace-is-made-of.md), #83) | **0** | 136 m | 103 m | nothing |
| Brief 033 ([ADR-0074](0074-a-stimulus-that-fires-once.md), #88) | **0** | cancelled | — | nothing |

Three of the four changed only `tests/` and documents. `cargo-mutants` mutates source, not tests, so a sweep dispatched on those branches could only re-run the Monday before it: rounds 031 and 032 each waited the sweep's extra half hour to three quarters for a result the schedule already held, and brief 033's round cancelled its `scope=both` dispatch nineteen minutes in and re-dispatched `scope=exhaustive` once the reason was pointed out ([ADR-0074](0074-a-stimulus-that-fires-once.md) records the deviation). Under [ADR-0073](0073-the-whole-domain-tests-sharded.md)'s shards the exhaustive tests take about 34 minutes, so the gap a needless sweep adds is now nearer a hundred.

**"Added" should never have been in the rule.** A surviving mutant is one no test catches. A test added to a suite can only catch more, so the set of survivors after a round that only adds tests is the set before it or a subset of it; the sweep cannot find a new one. And the tests a measuring round adds are `#[ignore]`d `exhaustive` runs, which are not in the suite the sweep runs at all. What *can* leave a mutant uncaught in code the pull request's `--in-diff` gate does not look at is the other half of the old rule: a test removed, a test moved out of the swept suite (ADR-0061's case, which found `Task::coin_at`), a change to the sweep's own configuration — and a change to source, which the gate mutates only where the diff touches it while the change may reroute what the existing tests reach elsewhere.

**And the brief was the wrong place to decide it.** A brief is written before the diff exists. Brief 033's author — this decision's author — wrote `scope=both` into its evidence deliverable on the strength of "this round adds tests", and the diff the round produced touched no source at all. The fact that decides the scope is in the diff, and only the executing round has the diff.

## Decision Drivers

- The sweep's cost is two hours of wall clock per round; its value is exactly the class of change that can leave a mutant uncaught outside the changed lines. The rule should name that class and nothing else.
- A rule the executing round can apply to its own diff mechanically beats one a brief's author applies to a guess.
- Monday's scheduled run always sweeps the whole tree, so the cost of a round wrongly skipping it is bounded by days, as [ADR-0030](0030-verification-governance.md) already prices a survivor ("it blocks nothing … and its survivors are the next round's list").

## Considered Options

1. **Keep ADR-0067's rule and its placement.** Three of the last four rounds paid for nothing.
2. **Drop "added" and keep the choice in the brief.** Right about the class, still guessing the diff.
3. **Drop "added" and move the choice to the executing round, decided from its diff.**
4. **Compute the scope in the workflow from the diff** (a `scope: auto`). The source and configuration clauses are one `git diff --name-only`; a test moved out of the swept suite is not reliably visible in a diff — a new `#[ignore]` on a new test and one on an old test look alike — so an automatic scope would either miss ADR-0061's case, which is the case the sweep exists for, or sweep every round that adds an `#[ignore]`d test, which is every measuring round.

## Decision Outcome

**Option 3.** The rule, applied by the executing round to the diff it has, at the time it dispatches:

- **`scope=exhaustive`, always** — the round's own pinned numbers live there.
- **`scope=both`** when the diff, against the branch it merges into, does any of:
  - changes a file under a `src/` directory;
  - deletes a test, or takes one out of the suite the sweep runs (gives an existing test `#[ignore]`, or moves it into a target the sweep does not run);
  - changes `.cargo/mutants.toml` or the `mutants-weekly` job.
- Otherwise — tests added, whether in the gate or `#[ignore]`d, and documents — **`scope=exhaustive` alone**, and Monday sweeps the tree as it always does.

The first and the last clauses are `git diff --name-only <base>...HEAD`: a path containing `/src/`, `.cargo/mutants.toml`, or `.github/workflows/ci.yml` with the sweep's job in its hunk. The middle one is a reading of the round's own test changes, which the round wrote and can state.

**A brief no longer names a scope.** Its evidence deliverable cites this rule and asks the round to state which clause applied, so the choice is recorded in the ADR that owns the round, with the diff that decided it. The input a brief freezes is the rule, not the round's guess at the rule's outcome.

This reverses the second half of ADR-0067, which its author wrote, on the evidence of the table above.

### Consequences

- Good: a measuring round that adds only `#[ignore]`d tests and documents — three of the last four — waits on the exhaustive shards, about 34 minutes, instead of the sweep's two hours.
- Good: the rounds the sweep exists for still get it. Brief 030's changed source and would be swept under this rule as it was under ADR-0067's; ADR-0061's moved tests would be swept by the middle clause.
- Good: the decision sits beside the diff that made it, in the ADR, where a reader can check it.
- Neutral: the middle clause is a statement a round makes about its own test changes, not a check. A round that moves a test out of the swept suite and says it did not would skip a sweep it needed; Monday's run would find the survivor within days, which is ADR-0030's price for a survivor and blocks nothing.
- Bad: one more thing a round must get right, and the rule is written in three places — this decision, the workflow's comment above the input, and Appendix B's V-6 row — which is three places to keep in step.

## Alternatives considered and why rejected

- **Options 1, 2 and 4**, on the reasoning above: 1 keeps paying for nothing, 2 keeps the guess, 4 cannot see the case the sweep is for.
- **Sweeping only the areas the diff touched** (`crates` or `runtime`). A change's effect on coverage is not confined to its area — the runtime composes the state crates — and ADR-0058's area jobs are the unit the completeness check is written for; splitting their dispatch by path is a second decision, with its own evidence, if the sweep is ever the bottleneck on rounds that do change source.
- **Dropping the per-round sweep altogether.** It gives up ADR-0061's case in the rounds most likely to cause it.

## Confirmation

- `.github/workflows/ci.yml`: the comment above `workflow_dispatch`, which states the rule.
- Whitepaper Appendix B's V-6 row and §9.
- The table above, from the four pull requests' file lists and the two dispatches' job times (runs `35528304550` and `35548410926`), and brief 033's cancelled `scope=both` run `35598939093` beside its `scope=exhaustive` run `35600828516`.
