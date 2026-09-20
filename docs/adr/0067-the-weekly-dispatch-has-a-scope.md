---
status: accepted
date: 2026-09-20
amends: ADR-0058
decision-makers: VirtualCortex maintainers
---

# ADR-0067: The weekly dispatch has a scope — a round runs the exhaustive tests, where its own measurement lives, and runs the whole-tree sweep beside them only when it added or removed a test, moved one between the gate and the weekly job, or changed the sweep itself

## Context and Problem Statement

Two different things run on the weekly trigger, and `gh workflow run ci.yml` has no way to ask for one of them. `weekly` runs the `#[ignore]`d `exhaustive` tests; `mutants-weekly` runs the whole tree under mutation in seven jobs ([ADR-0058](0058-the-weekly-sweep-and-its-timeouts.md)). Both carried `if: github.event_name == 'schedule' || github.event_name == 'workflow_dispatch'`, so a dispatch started all eight.

Since [ADR-0061](0061-the-learning-runs-leave-the-gate.md) moved the learning runs out of the pull request's gate, **a round's own measurement lives in the `exhaustive` tests**, so a round has to dispatch. Brief 029 is the case: its criterion runs are weekly tests and its result did not exist anywhere else. It therefore waited for the sweep too. Its dispatch, run [35502753399](https://github.com/DescentVTT/VirtualCortex/actions/runs/35502753399), priced the difference:

| Job | Time |
| :--- | ---: |
| `weekly` (exhaustive tests) | **40 m** (2 425 s, against a 120-minute bound) |
| `crates` | 19 m |
| `runtime-2`, `runtime-3`, `runtime-1` | 1 h 19 m, 1 h 37 m, 1 h 49 m |
| `runtime-0`, `runtime-4`, `runtime-5` | 2 h 00 m, 2 h 05 m, **2 h 06 m** |

The seven sweep jobs run in parallel, so the wall clock they add is the slowest of them: a round waits about two hours for a result that arrives in forty minutes.

The sweep is worth that wait when it can find something the pull request's gate cannot. The gate is `cargo mutants --in-diff`: it makes mutants only in the lines the change touches. What it structurally cannot see is a mutant in **unchanged** code that the change has left uncaught — and that is a real defect, not a hypothetical. [ADR-0061](0061-the-learning-runs-leave-the-gate.md) moved five tests from the gate to the weekly job and the sweep dispatched on that branch exposed `task.rs:301:26: replace ^ with | in Task::coin_at`, which only those five runs had been catching; the gate was green, because `coin_at` was not in the diff.

But that class has a shape. Coverage of unchanged code shifts when a round **adds or removes a test, moves one between the gate and the weekly job, or changes the sweep's own configuration**. A round that adds code and its tests is covered by the gate; a round that only touches documents or a workflow moves no coverage at all. For those, the seven jobs are two hours that find what Monday's scheduled run would have found anyway — and [ADR-0030](0030-verification-governance.md) already says what happens to that: "**The weekly job** runs the exhaustive tests and the whole-tree mutation run **on a schedule**; it blocks nothing, since there is no pull request to block, and its survivors are the next round's list."

Nothing in ADR-0030 asks a round to dispatch the sweep. Whitepaper Appendix B's V-6 row says "the whole tree is run weekly **and when a round ends**", and briefs 028 and 029 each made a branch dispatch a deliverable. That is where the practice came from, and it was applied to every round rather than to the rounds it was for.

## Decision Drivers

- A round's measurement lives in the `exhaustive` tests; that dispatch is not optional and must stay cheap to ask for.
- The sweep's value is bounded to a nameable class of change. Paying it outside that class buys a duplicate of Monday.
- ADR-0030 already prices a survivor found late: it blocks nothing and becomes the next round's list. The decision is only whether it is found on a branch or on `main` a few days later.
- A rule a round can apply without judgement. "Did you add, remove or move a test, or change the sweep?" is answerable from the diff.

## Considered Options

1. **Keep dispatching both every round.** Two hours per round, most of it re-proving Monday.
2. **A `scope` input on the dispatch, and a rule for when each is asked for.**
3. **Decide by the diff's paths in the workflow** — dispatch the sweep automatically when `crates/**` or `runtime/**` changed. It reads the wrong thing: a round can add a hundred lines with their tests and shift no coverage, and can move one `#[ignore]` and shift a lot.
4. **Drop the per-round sweep entirely and rely on the schedule.** It gives up the one case the gate cannot see, in the rounds most likely to cause it.

## Decision Outcome

**Option 2.** `workflow_dispatch` gains an input:

```yaml
workflow_dispatch:
  inputs:
    scope:
      description: Which weekly jobs to run
      type: choice
      default: both
      options: [both, exhaustive, mutants]
```

and the two jobs read it:

- `weekly`: `if: github.event_name == 'schedule' || inputs.scope == 'both' || inputs.scope == 'exhaustive'`
- `mutants-weekly`: `if: github.event_name == 'schedule' || inputs.scope == 'both' || inputs.scope == 'mutants'`

On a schedule the first clause carries both, whatever `inputs.scope` is, so **Monday is unchanged and always runs both**. On a push or a pull request the `inputs` context is empty and every clause is false, which is the behaviour those events had before.

**The rule a round follows**, which replaces "and when a round ends":

- **Always** `gh workflow run ci.yml --ref <branch> -f scope=exhaustive` — the round's own pinned numbers, and the exhaustive job's time against its 120-minute bound.
- **Also `scope=mutants`** (or `scope=both` in one dispatch) when the round **added or removed a test, moved a test between the pull request's gate and the weekly job, or changed `.cargo/mutants.toml` or the sweep's jobs, bounds, shards or completeness check**. This is ADR-0061's situation exactly, and it is the situation in which the gate's `--in-diff` scope is known to be blind.
- Otherwise the sweep stays on its schedule, and its survivors are the next round's list, as ADR-0030 says they are.

A brief's evidence deliverable names the scope it asks for and why, so the choice is in the frozen input rather than in the executing session's judgement.

### Consequences

- Good: a round that shifts no coverage waits about forty minutes instead of about two hours, and the seven sweep jobs are not spent re-proving what the schedule proves.
- Good: the rounds that historically found something — ADR-0061's, which moved five tests, and brief 028's, which rewrote ten loops and removed exclusions — are exactly the ones the rule still sends to the sweep.
- Good: the default is `both`, so a dispatch that names no scope behaves as every dispatch did before this decision.
- Neutral: `scope=mutants` alone exists for the case where a round needs the sweep and has no new pinned number to check; no round has needed it yet.
- Bad: a round inside the rule's class that judges itself outside it ships a survivor to `main`, found on Monday instead of on the branch. ADR-0030 already treats that survivor as the next round's list and blocks nothing on it, so the cost is the days between, not a defect in what the engine does.
- Bad: one more thing for a round to get right. The rule is in this ADR, in `.github/workflows/ci.yml`'s comment above the input, and in Appendix B's V-6 row.

## Alternatives considered and why rejected

- **Two workflows instead of one input.** The jobs share the schedule, the concurrency group and the repository's check names; splitting the file to avoid one input trades a small condition for a duplicated trigger block.
- **Running the sweep on `main` after every merge.** It is the schedule with a worse cadence: the same seven jobs, now serialised behind every merge, and still after the fact.
- **Shrinking the sweep instead (more shards, `--jobs 4`).** ADR-0058 sized the shards against the 330-minute job limit and F-42 is the cost of running two slots at all; more parallelism is a different decision, on its own evidence, and does not change that most rounds need none of it.
- **Making the choice automatic from the diff.** Option 3 above: the signal that matters is which tests exist and where they run, which a path filter does not read.

## Confirmation

- `.github/workflows/ci.yml`: the `scope` input, the two `if` expressions, and the comment above the input that states the rule.
- Two dispatches on this change's branch, which are the evidence that the conditions select: one at `scope=mutants` in which `Weekly (exhaustive tests)` is skipped, and one at `scope=exhaustive` in which the seven `Weekly mutation sweep` jobs are skipped and the exhaustive job is green; their run ids in the changelog entry.
- Whitepaper Appendix B's V-6 row and §9; [ADR-0030](0030-verification-governance.md)'s weekly bullet is the sentence this decision applies rather than changes.
