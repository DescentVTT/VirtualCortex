---
status: accepted
date: 2026-10-09
depends-on: ADR-0150
decision-makers: VirtualCortex maintainers
---

# ADR-0158: The sweep off the next decision's path — ADR-0150 took the sweep off a round's merge and left it on the next decision's, which waited two to three hours for it twice; its outcome is now written down by the first ADR that merges after it ends, and no ADR waits; the runtime's sweep is dealt to twelve shards where it was six; where the sweep's time goes measured mutant by mutant, 88 mutants of 1 320 taking 64 per cent of it, and running its tests under nextest named as the next step and not decided

## Context and Problem Statement

[ADR-0150](0150-a-round-waits-for-what-it-checks.md) had a round's merge wait for the whole-domain shards and not for the sweep, and wrote: *"The sweep's outcome is read when it ends and written down by the next decision, in the Context of the ADR that follows the round."*

**What that rule cost**, twice:
- [ADR-0154](0154-a-punishment-the-value-does-not-soften.md) was written an hour after brief 062's round merged and waited for the sweep of run 37810004245, which ended two hours after that merge.
- [ADR-0157](0157-the-span-a-choice-among-three-is-given.md) was written within the hour after brief 063's round merged, and the sweep of run 37872100407 had about ninety minutes left.

The wait moved from the round's merge to the next decision's. No brief could be handed on while it lasted. The maintainers asked on 2026-10-09 whether the sweep itself could be made faster, and chose the three things below.

**Where the sweep's time goes**, read mutant by mutant from the artifacts of run 37810004245 (`outcomes.json` of each job):

| Area | Mutants | Machine time | Wall time |
| :--- | ---: | ---: | :--- |
| the state crates | 2 485 | 56 min | 30 min, one job |
| the runtime | 1 320 | 30.2 h | 1 h 52 m to 3 h 11 m, six jobs of two at a time |

The runtime's 30.2 hours, by what a mutant's time was spent on:

| | Mutants | Machine time | Share |
| :--- | ---: | ---: | ---: |
| building | all | 404 min | 22 % |
| tests, a mutant caught within 30 s | 1 056 | 119 min | 7 % |
| tests, caught within 30 s to 4 min | 103 | 126 min | 7 % |
| tests, caught after more than 4 min | 72 | 768 min | 42 % |
| tests, cut by the bound | 16 | 393 min | 22 % |

- **Of the 72, 67** are first caught by one of the four test binaries `cargo test` runs last, in the order of their names: `no_alloc` 35, `reference` 20, `store` 8 and `sleep` 4. Every binary before them runs first. The other five are first caught by `modulation` and `image`.
- **The 16** are the hangs [ADR-0062](0062-the-first-complete-sweeps-list.md) reads as inherent, in the injector, the barrier and the workers. Each waits out its bound, 18 to 29 minutes.
- **So 88 mutants of 1 320 take 64 per cent of the area's time.**

## Decision Drivers

- **The maintainers' question** (2026-10-09), and their choice of the three below.
- **No check is removed.** The sweep runs as it ran, on the same commit, and its outcome is still read and written into an ADR.
- **The record stays true**: an ADR written before a sweep ends says so.
- **A measured need** for the shards and for the next step: the table above.
- Latest ≠ Newest: this ADR adopts nothing. The tool the next step names is decided by its own ADR, under that test.

## Considered Options

1. **Who writes down a sweep's outcome**: (a) the first ADR that merges after the sweep has ended; (b) the next decision's ADR, which waits for it; (c) the round's own ADR, by a second pull request.
2. **The runtime's shards**: (a) twelve; (b) six; (c) seven, the most that fit the account's twenty jobs beside the twelve whole-domain shards and the state crates' job.
3. **The 88 mutants**: (a) named here, and decided by an ADR of its own with a round that measures it; (b) decided here.

## Decision Outcome

**Options 1(a), 2(a) and 3(a).**

### Who writes down a sweep's outcome (option 1(a))

- **A sweep's outcome is read when it ends and written down by the first ADR that merges after it has ended**, a decision's or a round's, in that ADR's Context.
- **An ADR written before it ends does not wait.** It names the run, says the sweep had not ended, and says which of its jobs had.
- **A survivor is a finding**, and the next round's list, as [ADR-0030](0030-verification-governance.md) has it. Nothing here changes what a survivor is.
- **Who looks.** The session that writes an ADR reads whether the last dispatched sweep has ended and whether an ADR has recorded it. `briefs/README.md`'s *"Evidencing a round"* says so. No check in the gate holds this, since the gate cannot read a run: it is a described practice, not an enforced rule (principle 5).

### Twelve shards for the runtime (option 2(a))

- **The runtime's sweep is dealt to twelve shards**, round-robin as before, each with its own timed bound and its own completeness check ([ADR-0058](0058-the-weekly-sweep-and-its-timeouts.md)).
- **A dispatch at `scope=both` is then twenty-five jobs**: twelve whole-domain shards, the state crates' job and twelve of the runtime. The account runs twenty at once, so five wait until the first five jobs end, which was 33 and 35 minutes in the last two dispatches.
- **Expected**: a shard of about 110 mutants and 2.5 hours of machine time, so about 1 h to 1 h 40 m of wall time each, and the sweep ended within about two and a quarter hours of the dispatch where it took 3 h 11 m. That is an expectation from the table, not a reading.

### The 88 mutants (option 3(a))

Running the sweep's tests under `cargo-nextest` is the candidate: it runs every test of every binary at once and stops at the first failure, so a mutant first caught by a late binary is caught when that test fails, and it bounds each test by itself, so a hang is a failed test within its own bound. **It is not decided here.** It is a tool in CI, to be held to Latest ≠ Newest by its own ADR, and kept only if a round reads every mutant's verdict no weaker and the time saved.

### What does not change

- What the sweep mutates, the bound's rule, the completeness check and the timeouts' evidence.
- ADR-0075's rule for a dispatch's scope, and everything else of ADR-0150: what a round's merge waits for, the commits an ADR cites, and the arms run once.
- The scheduled sweep on Mondays.

## Consequences

- Good: a decision's ADR merges when its own gate is green. Between two rounds that changed source that is two to three hours sooner.
- Good: the sweep ends sooner by about a third, with the same mutants and the same bound.
- Good: where the time goes is on record, with the 88 mutants named by what they wait for.
- Bad: a sweep's outcome may be written into a round's ADR by a session that did not run that sweep's round. It reads the same artifacts.
- Bad: twenty-five jobs against twenty. During a round's dispatch the pull request's own five jobs queue behind the five sweep jobs that wait. In the last two dispatches the slowest whole-domain shard ended at 52 and 55 minutes, which is later than the queue is expected to clear; if a round reads its gate held past its shards, the count is revisited.
- Bad: who looks is a practice and not a check.
- Neutral: twelve jobs build the runtime's tests and time its suite where six did. The runners are not metered.

## Alternatives considered and why rejected

- **Option 1(b), the next decision waits**: two to three hours in which no brief can be handed on, for a result that blocks nothing.
- **Option 1(c), a second pull request for the round's ADR**: ADR-0150 removed it, and it would return for one paragraph.
- **Option 2(b), six shards**: 3 h 11 m for the slowest.
- **Option 2(c), seven shards**: no job waits, and the slowest shard is a seventh shorter. Twelve halves it and has five jobs wait about half an hour.
- **Option 3(b), nextest decided here**: it changes what runs the tests under mutation. Whether every verdict holds is a measurement, and a tool is an ADR.
- **A shorter bound for the hangs alone**: the bound is three times the area's own suite so that no slow pass is recorded as a timeout ([ADR-0058](0058-the-weekly-sweep-and-its-timeouts.md)). A second, shorter bound for a list of mutants is a list to keep true by hand.
- **A sweep of the changed files only**: ADR-0075 sends a round to the whole tree because a change to source may reroute what the existing tests reach elsewhere.

## Confirmation

- `.github/workflows/ci.yml`: the `mutants-weekly` job's matrix, `runtime-0` to `runtime-11`, each `--shard k/12`.
- `briefs/README.md`: rule 7 of *"Evidencing a round"*.
- Whitepaper §9's row and the directive that holds the workflow to twelve.
