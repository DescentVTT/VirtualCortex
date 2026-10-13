---
status: accepted
date: 2026-10-13
depends-on: ADR-0176
decision-makers: VirtualCortex maintainers
---

# ADR-0179: The bound's rule restated — F-68: ADR-0176 set a weekly shard's bound by the longest test's slowest reading two and a half times over, and the next dispatch read that test slower, so the rule as written asked for 210 minutes; the rule counted a slow runner twice and moved with every reading; the bound stays 180 minutes and its rule is the one the briefs already hold a round to: no shard of a dispatch past 60 per cent of it, by the job's own minutes

## Context and Problem Statement

[ADR-0176](0176-a-shards-bound-by-its-longest-test.md) set a weekly shard's bound at 180 minutes by a rule: *"the least multiple of thirty minutes that holds the longest whole-domain test two and a half times over, the slowest runner ADR-0088 read."* It took the longest test as 3 730 s, the slowest of three dispatches' readings of an arm at 2 048 units.

**F-68** ([ADR-0177](0177-the-controllers-step-measured.md)): the next dispatch, run 38352602613, read the same arm at 4 412 s. Two and a half times that is 184 minutes, so by the rule as written the bound would be 210. No test was added; an arm the tree has held since brief 068 read slower on one run's runner.

**What was wrong with the rule.**
- **It counted a slow runner twice.** The slowest reading of a test is already a slow runner's: over four dispatches that arm read 2 151 to 4 412 s, a factor of two. The rule then multiplied it by the factor ADR-0088 read between runners.
- **It moved with every reading.** A rule of one test's slowest second changes whenever a runner is slower than any before it, with no test added and no shard near its bound.
- **It bounded the wrong thing.** The bound is a job's. What a job takes is its shard's tests two at a time and its build, not its longest test alone.

**What the jobs read.** The slowest shard of each of the last five dispatches took 65.9, 90.9, 79.8, 82.1 and 86.5 minutes, the job's own time. In the dispatch that opened F-68 the shard that held the arm of 4 412 s was that slowest, at 86.5 minutes: 48 per cent of 180.

## Decision Drivers

- **F-68**, a finding against ADR-0176's own rule, a day after it was written.
- **A rule is not kept that the next reading breaks** without anything having changed.
- **One number for one thing**: the briefs already hold a round to 60 per cent of a shard's bound.
- **A structural boundary beats a reviewed one** (principle 5): the bound is the workflow's; the rule says when it moves.

## Considered Options

1. (a) the bound stays 180 minutes, by the briefs' 60 per cent of the slowest job read; (b) 210 minutes, by the rule as written; (c) the rule as written with a typical reading in the slowest's place.

## Decision Outcome

**Option 1(a).**

- **The rule**: a weekly shard's bound is the least multiple of thirty minutes of which the slowest shard's job, over the dispatches read, is at most 60 per cent. The slowest read is 90.9 minutes; that asks for 152; **the bound stays 180 minutes.**
- **When it moves**: when a shard of a dispatch reads past 60 per cent of it, 108 minutes of the job's time, under a deal made from a table that knew its tests. The round that reads it says so, and the bound or the deal is then the workflow's decision.
- **A shard past 60 per cent in a round's first dispatch**, by tests the deal costed at its default, is reported as before and is no reason by itself.
- **ADR-0176's rule is withdrawn**; its bound, its twelve shards and everything else it decided stand. ADR-0176 is accepted and is not edited.

## Consequences

- Good: the bound's rule and the briefs' directive are one statement, read from what the bound bounds.
- Good: F-68 is resolved without the bound moving, and a slow runner's reading of one test no longer moves it.
- Bad: the rule is read after the fact. A test long enough to break a shard is found by the dispatch that runs it, as now.
- Bad: two ADRs for one number, the second correcting the first within a day.
- Neutral: 180 minutes has run once, with its slowest shard at 48 per cent.

## Alternatives considered and why rejected

- **Option 1(b), 210 minutes**: the rule would ask for 240 at the next slower reading, with the shards where they are.
- **Option 1(c), a typical reading**: it still bounds a test where the bound is a job's, and "typical" is a choice among four readings.

## Confirmation

- `.github/workflows/ci.yml`: `timeout-minutes: 180` on `weekly`, with this rule in its comment.
- Whitepaper §11's F-68 resolved, §9's row, and the directive's reason.
