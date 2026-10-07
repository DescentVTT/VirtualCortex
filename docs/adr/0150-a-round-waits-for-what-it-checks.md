---
status: accepted
date: 2026-10-07
depends-on: ADR-0075
decision-makers: VirtualCortex maintainers
---

# ADR-0150: A round waits for what it checks — a measuring round took seven hours and twenty minutes from its brief to its merge, seventy minutes of it the runs it was for; four waits that check nothing more are removed and no check is: the merge waits for the dispatch's whole-domain shards and not for its sweep, an ADR cites its round's commits as the pull request holds them so that no second pull request rewrites them, the arms run once on the developer machine and are reproduced by the dispatch, and the whole-domain tests are dealt to twelve shards; `CLAUDE.md`'s account of the tree returns to a switchboard's length

## Context and Problem Statement

**What a round costs**, read from brief 060's round ([ADR-0145](0145-the-gates-output-measured.md)), from the brief to the merge, 7 h 20 m:

| Stage | Wall time | What it is |
| :--- | ---: | :--- |
| Reading, the build, the protocol | about 60 m | the round's own work |
| The calibration | 83 m | every whole-domain test of the tree, on the developer machine, before any rewarded run |
| The arms | about 70 m | run once to write the tables, 26 m, and once more to reproduce them, 25 m |
| The dispatch | 213 m | the whole-domain shards ended after 68 m; the sweep's last shard after 213 |
| The evidence, the gate, the merge | about 30 m | |
| After the merge | 20 to 30 m each | a pull request that rewrites the ADR's commit hashes, then the next decision's |

The runs the round exists for are the 26 minutes of the arms' first pass. The maintainers asked on 2026-10-07 what of the rest could go with nothing lost, and decided the four below. A fifth was weighed and not taken: moving the calibration behind the rewarded run.

**What each wait checks:**
- **The sweep, 145 minutes past the shards.** [ADR-0075](0075-the-dispatch-scope-follows-the-diff.md) sends a round that changed `src/` to the whole-tree sweep, because a change to source *"may reroute what the existing tests reach elsewhere"*. [ADR-0030](0030-verification-governance.md) says of the scheduled sweep that *"it blocks nothing … and its survivors are the next round's list"*, and ADR-0075 priced a round's sweep the same way. The lines the round changed are already mutated by the pull request's own gate. So the sweep's result is wanted, and nothing about the merge depends on it. The briefs nevertheless asked for a dispatch *"green in every job"* before the merge, and the merge waited.
- **The second pull request.** `main` takes a round by a rebase, which gives its commits new hashes. Each round's ADR cites its commits, so each round opened a second pull request to cite them *"as `main` holds them"*, and set its ADRs to `accepted` there. The commits the ADR first cited do not go away: GitHub keeps a merged pull request's commits under `refs/pull/<n>/head`.
- **The arms' second pass.** It shows that the pinned tables reproduce. The dispatch then shows it again, on other hardware and another operating system, which is the stronger evidence.
- **Six shards.** The whole-domain tests' shards read 41 to 68 minutes in that dispatch, at two tests a time on a four-core runner. The repository is public, and the hosted runners are not metered.

**`CLAUDE.md`.** Its section *"What this repository is"* opens: *"This file is the switchboard, not the rulebook."* The section's account of what exists has grown by a clause or two every round into one sentence of several thousand words. Every session reads it whole at its start, every round edits it, and two pull requests open at once conflict in it. The whitepaper's §1.6 is the table of what is built, §11.1 holds every hypothesis with its verdict, and the changelog holds every round.

## Decision Drivers

- **The maintainers' question** (2026-10-07): the same effect, much sooner.
- **No check is removed.** Every job that ran still runs, on the same commit, and every result is still read and recorded.
- **A structural boundary beats a reviewed one** (principle 5): what the merge waits for is what the merge's watcher reads, not a sentence in a brief.
- **The record stays true**: a commit an ADR cites must stay reachable, and a result not yet known when a round merges must be said to be unknown.
- Latest ≠ Newest: no tool, no dependency; one number in the workflow.

## Considered Options

1. **The sweep**: (a) dispatched as ADR-0075 says, and not waited for; (b) waited for, as the briefs asked; (c) not dispatched by a round at all.
2. **The commits an ADR cites**: (a) the pull request's, with its number, never rewritten; (b) `main`'s, by a second pull request; (c) `main` takes a round by a merge commit, so that the hashes stay.
3. **The arms' reproduction**: (a) the dispatch's; (b) a second pass on the developer machine, then the dispatch.
4. **The shards**: (a) twelve; (b) six; (c) one a test.
5. **The calibration**: (a) every whole-domain test before any rewarded run, as now; (b) the arm's own calibration before, and the tree's by the dispatch after.
6. **`CLAUDE.md`'s account of the tree**: (a) what the tree is, where it stands in a paragraph, and where to read the rest; (b) the running account.

## Decision Outcome

**Options 1(a), 2(a), 3(a), 4(a), 5(a) and 6(a).**

### The merge waits for the shards (option 1(a))

- **ADR-0075 stands.** A round still decides its dispatch's scope from its diff, and a round that changed `src/` still dispatches the sweep.
- **A round asks for its merge when** the dispatch's whole-domain shards are green and the pull request's gate is green on its last commit. Its ADR names the dispatch and says the sweep had not ended.
- **The sweep's outcome is read when it ends and written down by the next decision**, in the Context of the ADR that follows the round. A survivor is a finding there and the next round's list, as ADR-0030 has it.
- **The cost table** is regenerated from the shards' artifacts, which a run gives as soon as each shard has uploaded them.

### An ADR cites the pull request's commits (option 2(a))

- **A round's ADR cites its commits by the hashes its branch holds, with its pull request's number.** They stay reachable under `refs/pull/<n>/head`. On `main` the same commits are found by their subjects, which a rebase keeps.
- **The commit that asks for the merge sets the round's ADRs to `accepted`**, in the files and in the index. The merge is the acceptance, and no pull request follows it to say so.
- An ADR merged before this one is not rewritten.

### The arms run once (option 3(a))

- **A round runs its arms once** on the developer machine, writes its tables from that run's dumps, and commits them. **The dispatch's shards are their reproduction.**
- A table the dispatch does not reproduce stops the round as a finding: the run is not deterministic across the two machines, or the table was written wrongly, and either is to be known before a merge.
- Everything before the arms is as it was: the calibration, the protocol committed first, the constants that do not move after a rewarded run.

### Twelve shards (option 4(a))

- **The whole-domain tests are dealt to twelve shards**, by cost, as [ADR-0092](0092-the-shards-dealt-by-cost.md) deals them. At the table after ADR-0145, 85 tests and 39 759 s, a shard plans about 3 300 s summed and about half of that in wall time.
- **The floor is the longest test**, 1 997 s in that table, which no number of shards divides.
- **With the sweep's seven jobs a dispatch at `scope=both` is nineteen jobs**, inside the twenty jobs the account's plan runs at once.

### `CLAUDE.md` (option 6(a))

- *"What this repository is"* says what the tree is, where the work stands in one paragraph, and where each kind of question is answered: whitepaper §1.6 for what is built, §11.1 for every hypothesis and its verdict, §11 for the findings, the ADR index for every decision, the changelog for every round.
- **A round no longer adds its account there.** It updates the paragraph only where the standing changes: the configuration the engine learns in, the live brief, the format's number.
- The principles, the invariants, the commands and the workflow are untouched.

### What does not change

- No rule of the engine, no test, no pinned number, no hypothesis and no stopping rule.
- The pull request's gate, and what it requires before a merge.
- ADR-0075's rule for the dispatch's scope, ADR-0030's for a survivor, and the calibration before any rewarded run.

## Consequences

- Good: a round that changes source merges more than three hours sooner, and one that does not about an hour and a quarter, with every check still run on the same commit.
- Good: one pull request a round where there were two, and a decision's pull request no longer waits behind the second.
- Good: the dispatch is the arms' reproduction in name as well as in fact.
- Bad: a round merges with its sweep unread. A survivor is then found after the merge and not before, by hours. ADR-0030 already prices that.
- Bad: a table written wrongly is found by the dispatch, forty minutes on, where the second pass found it in twenty-five.
- Bad: twelve jobs build the tests where six did. The repository's runners are not metered; a private one would pay for it.
- Bad: an ADR's commit hashes are not `main`'s. A reader goes through the pull request's number or the commit's subject.
- Neutral: `CLAUDE.md`'s history of the rounds is not lost. It is the changelog's, the ADR index's and whitepaper §11.1's, where it already was.

## Alternatives considered and why rejected

- **Option 1(b), wait for the sweep**: 145 minutes for a result that blocks nothing.
- **Option 1(c), no sweep from a round**: ADR-0075's class is real. A change to source can leave a mutant uncaught outside the changed lines, and Monday's run would find it days later instead of hours.
- **Option 2(b), a second pull request**: twenty to thirty minutes a round, to change seven hashes that were not wrong.
- **Option 2(c), a merge commit**: the hashes would stay, and `main`'s history would stop being linear, which every `git log` of this tree has been since its first commit.
- **Option 3(b), a second local pass**: the same evidence as the dispatch, weaker, and 25 minutes sooner on a failure.
- **Option 4(b), six shards**: 68 minutes where about 40 serve. **Option 4(c), one a test**: 85 jobs, each building the tests, queued behind the plan's twenty.
- **Option 5(b), the calibration after the run**: it would save the most, 80 to 110 minutes and more each round. But a pinned number that moved would then be found after the round's result was read and not before. The maintainers kept the order.
- **Option 6(b), the running account**: it is the changelog written a second time, in the one file every session must read whole.

## Confirmation

- `.github/workflows/ci.yml`: the `weekly` job's matrix of twelve shards, and `scripts/exhaustive-shard.sh` called with twelve.
- `briefs/README.md`: *"Evidencing a round"*, which a round's session reads before its brief.
- `CONTRIBUTING.md`: how an ADR becomes `accepted`.
- `CLAUDE.md`: *"What this repository is"*.
- Whitepaper Appendix B's line on the whole-domain tests, §9's row and the version.
