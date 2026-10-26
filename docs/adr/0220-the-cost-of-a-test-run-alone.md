---
status: accepted
date: 2026-10-26
amends: [ADR-0092, ADR-0211]
title: The cost of a test run alone, and a bound of 240 minutes
kind: process
summary: A shard whose dispatch named `only` ran its tests alone and is not read into the cost table, which keeps its lines for what a run did not measure. A test with no line is costed at 2 400 s. The weekly shard's bound is 240 minutes, by ADR-0179's rule. F-82 is resolved.
decision-makers: VirtualCortex maintainers
---

# ADR-0220: The cost of a test run alone, and a bound of 240 minutes

**In brief**
- **Decision.** A shard whose dispatch named `only` is not read into the cost table, and `from --keep` keeps the table's line for every test a run did not measure. A test with no line is costed at 2 400 s, where it was 900. The weekly shard's bound, and an arm's, is 240 minutes, by ADR-0179's rule.
- **Why.** Brief 081's reproduction ran H-44's arms alone and read them at 634 to 1 257 s; in a full deal they took 2 100 to 2 400. Costed so, four shared one shard of brief 082's calibration, which read 128.1 minutes (F-82). The weekly job's 244 578 s now average 113 minutes a shard, past ADR-0179's 108.
- **Left open.** The weekly job grows by about 12 minutes a shard with each round of eighteen arms. What it holds is named and not taken.

## Context and Problem Statement

[ADR-0211](0211-the-round-shortened.md) made a round's reproduction run its own tests alone, `-f only`, and regenerated the cost table from the calibration's and the reproduction's artifacts together.
A shard runs half as many tests at once as its runner has cores (`scripts/exhaustive-shard.sh`), two on the hosted runners. A reproduction with `only` deals a round's tests one or none a shard, so each runs alone.

| H-44's twelve arms | Seconds |
| :--- | ---: |
| Alone, in brief 081's reproduction, 40208314220 | 634 to 1 257 |
| Two at a time, in brief 082's calibration, 40226374616 | 2 100 to 2 400 |

- Dealt from the table those alone-timings wrote, brief 082's calibration put four of the arms in shard 10, which read 128.1 minutes, 71 % of its 180-minute bound. Three more read 108.3 to 111.7 (F-82, [ADR-0219](0219-a-delay-the-engine-reads-measured.md)).
- Brief 082's reproduction, 40237389553, dealt from the calibration's table with H-45's twenty tests at the default of 900 s, read ten shards past 108 minutes, the longest 127.1.
- The table now holds 219 tests and 244 578 s. Over eighteen shards, two at a time, that is 113 minutes a shard on average. No deal brings the slowest under 108.
- [ADR-0179](0179-the-bounds-rule-restated.md)'s rule: the least multiple of thirty minutes of which the slowest shard's job read is at most 60 per cent. For 128.1 minutes that is 240; 210's 60 per cent is 126.
- An arm's bound is the weekly shard's ([ADR-0184](0184-the-runs-off-the-developer-machine.md)).

## Decision Drivers

- **The table decides the balance and never the coverage** ([ADR-0092](0092-the-shards-dealt-by-cost.md)): a wrong cost costs minutes, not tests.
- **A test run alone is not a test in a deal**, and its seconds do not say what it costs in one.
- **ADR-0179's rule moves the bound** when no deal holds the line.
- **Latest ≠ Newest**: no dependency; the scripts and the workflow change.

## Considered Options

1. **A shard that ran its tests alone**: (a) not read, the table's line kept; (b) read, doubled; (c) read as it is.
2. **The default cost**: (a) 2 400 s, above every arm the last rounds measured; (b) 900 s.
3. **The bound**: (a) 240 minutes, by ADR-0179's rule; (b) 180, with more shards; (c) 180, with tests out of the weekly job.

## Decision Outcome

**Options 1(a), 2(a) and 3(a)**, built in this pull request.

### The table

- **`from` skips a shard that ran its tests alone**: an `exhaustive-tests-*.tsv` with a non-empty `only.txt` beside it, as a dispatch naming `only` writes it. It prints each one skipped.
- **`--keep <costs.tsv>` keeps that table's line** for every test the artifacts did not measure. Without it, a run with nothing read fails.
- **A test with no line is costed at 2 400 s.** H-44's and H-45's arms took 1 128 to 2 385 s in a full deal.
- The committed table's header says so; its rows do not change.
- `briefs/README.md`'s rule 4 regenerates the table with `--keep`. A round's own tests keep the default until a full deal measures them: a calibration, a reproduction of every test, or the schedule's run.
- Three new cases in `spec:costs:test`.

### The bound

- **The weekly shard's `timeout-minutes` is 240**, and so is an arm's.
- ADR-0179's line is now 144 minutes. The rule is unchanged.

### F-82, and the sweep

- **F-82**, resolved by this record.
- **The schedule's run of 2026-10-26** at 03:00Z had not begun when this record was written. The first ADR merged after its sweep ends records it ([ADR-0158](0158-the-sweep-off-the-next-decisions-path.md)).

## Consequences

- Good: a round's own tests are dealt as heavy ones until a full deal measures them, so they spread over the shards.
- Good: at 113 minutes a shard on average, a calibration reads 47 % of its bound.
- Bad: a whole calibration, or a reproduction of every test, takes about two hours.
- Bad: each round of eighteen arms adds about 27 000 s, 12.5 minutes a shard; at that rate 144 minutes is reached in about two rounds.
- Bad: a test costed at the default can leave its shard light. It costs balance, not coverage.
- Neutral: no code, test or pin of the engine changes.

## Amended

- **ADR-0092**: the default cost is 2 400 s, and the table is regenerated with the lines of what a run did not measure kept.
- **ADR-0211**: a reproduction with `only` is not read into the table.
- Each is accepted and is not edited.

## Alternatives considered and why rejected

- **Option 1(b), doubled**: the ratio read 1.9 to 3.3, not one number.
- **Option 1(c), as it is**: F-82.
- **Option 2(b), 900 s**: it under-costed every arm of the last three rounds.
- **Option 3(b), more shards**: past eighteen, the shards share the account's twenty places with the arms and the gate, and wait for them.
- **Option 3(c), tests out of the weekly job**: which tests it holds is a decision of its own, named in "Left open".

## Confirmation

- `npm run spec` runs the new cases of `spec:costs:test`, and the directives on the bound, the default and the skip.
- `from` on brief 081's reproduction, 40208314220, skipped its eighteen shards. With `--keep` it returned the table's rows unchanged.
- The next calibration reads its shards against 240 minutes.
