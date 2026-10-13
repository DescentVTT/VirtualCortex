---
status: accepted
date: 2026-10-13
depends-on: ADR-0168
decision-makers: VirtualCortex maintainers
---

# ADR-0176: A shard's bound by its longest test — the decision ADR-0168 named: an arm at 2 048 units is half of a weekly shard's 120 minutes by itself, which no deal moves, and twelve shards no longer hold a round's new arms under the briefs' 60 per cent; the bound becomes 180 minutes, the least multiple of thirty that holds the longest test two and a half times over, the slowest runner read; the twelve shards, the deal and the 60 per cent stay, and a round waits no longer

## Context and Problem Statement

[ADR-0168](0168-twice-the-size.md) took the learning configuration to 2 048 units and named what it left the workflow: *"A shard's bound against its longest test is the workflow's next decision."* [ADR-0173](0173-three-in-four-on-the-other-networks.md) said when it falls due: before the round after brief 070's. **The maintainers took it on 2026-10-13: the bound of a weekly shard becomes 180 minutes.**

**The longest test.** An arm at 2 048 units read 2 151 to 3 730 s over the three dispatches that ran it. Against a bound of 120 minutes, 7 200 s, the longest reading is 52 per cent by itself. The whole-domain tests are dealt longest first to the least-loaded shard and run two at a time ([ADR-0092](0092-the-shards-dealt-by-cost.md)), so a shard that holds such an arm cannot read under it, however the rest is dealt.

**The table.** It holds 114 tests. A hosted runner's seconds are a range: the 110 tests before brief 070's round summed to 83 129 s in one run and 75 150 in the next.

| The table, by run | Summed | A shard of twelve | Its wall time, at 1.66 to 1.99 of the summed seconds | Of 120 minutes | Of 180 |
| :--- | ---: | ---: | :--- | :--- | :--- |
| run 38307712843, a fast one | 81 785 s | 6 815 s | 3 425 to 4 105 s | 48 to 57 % | 32 to 38 % |
| the same tests at the seconds of the run before | 89 760 s | 7 480 s | 3 759 to 4 506 s | 52 to 63 % | 35 to 42 % |

In the last three dispatches one shard, one and two read past 60 per cent, at 61 to 75, each time by an arm at 2 048 units or by new arms the deal had costed at its default of 900 s.

**What the bound is for.** A shard that hangs is killed at its bound, and a shard on a slow runner must end inside it. [ADR-0088](0088-a-fourth-shard.md) read a runner two and a half times slower than the one before it. A merge waits for the shards' wall time ([ADR-0150](0150-a-round-waits-for-what-it-checks.md)), which the bound does not change.

## Decision Drivers

- **The decision ADR-0168 named**, due now by ADR-0173.
- **The floor is one test**, so more shards do not lower it.
- **A structural boundary beats a reviewed one** (principle 5): the bound is the workflow's, and a brief's directive reads against it.
- **No check is removed** and no round waits longer.
- Latest ≠ Newest: `timeout-minutes` is the platform's own key.

## Considered Options

1. (a) the bound at 180 minutes, by a rule of the longest test; (b) fourteen shards; (c) the arms of closed questions taken out of the weekly job; (d) nothing.

## Decision Outcome

**Option 1(a).**

- **The rule**: a shard's bound is the least multiple of thirty minutes that holds the longest whole-domain test two and a half times over, the slowest runner ADR-0088 read. The longest is 3 730 s; two and a half times is 9 325 s, 155 minutes; **the bound is 180 minutes**.
- **What it reads at**: the longest test is 35 per cent of the bound by itself, and the deal plans a shard at 32 to 42.
- **The briefs' 60 per cent stays**, read against the new bound: 6 480 s of wall time.
- **When it moves again**: when a test is added that the rule does not hold, 4 320 s and more at its slowest reading.

### What does not change

- The twelve shards, the deal by cost and the default a new test is costed at.
- What a merge waits for, and how long: the shards' wall time is what it was, 66 to 91 minutes for the slowest shard over the last four dispatches.
- The sweep's jobs and their bound of 330 minutes.

## Consequences

- Good: a round's new arms no longer stand between a shard and the briefs' 60 per cent, by the workflow's own number.
- Good: the bound is a rule of the longest test, so the next move is known beforehand.
- Good: no test is taken out of the weekly job and no round waits longer.
- Bad: a shard that hangs is killed an hour later than it was.
- Bad: the weekly job still grows by every round's arms. The table is about 82 000 to 90 000 s and plans 32 to 42 per cent; at this bound twenty to forty more arms of 1 800 s fit under 60, by the ratios read. **Which arms of closed questions still earn their place every week is not decided here.**
- Neutral: the first run under it is the next dispatch.

## Alternatives considered and why rejected

- **Option 1(b), fourteen shards**: fourteen and the pull request's five are nineteen of twenty places. A shard would plan at 45 to 50 per cent, and one that holds an arm at 2 048 units still at 52 and more. It buys about six arms.
- **Option 1(c), closed questions' arms out of the weekly job**: the largest saving: forty-six arms run weekly, 71 424 of the table's 81 785 s, and most of questions that are closed. It needs a rule for which stay and where the others run, which is a decision of its own.
- **Option 1(d), nothing**: the plan holds 60 on a fast run's table and not on the run before's, and the next round adds tests.

## Confirmation

- `.github/workflows/ci.yml`: `timeout-minutes: 180` on `weekly`, with the rule in its comment.
- Whitepaper §9's row and a directive that holds the workflow to the bound.
