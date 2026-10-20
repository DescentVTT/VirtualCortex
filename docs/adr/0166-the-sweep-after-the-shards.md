---
status: accepted
date: 2026-10-11
depends-on: ADR-0158
title: The sweep after the shards
kind: process
summary: The sweep's jobs now wait for the whole-domain shards, which keeps the sweep's twelve shards and gives the merge's path the first places. With twelve, a dispatch at `scope=both` had become twenty-five jobs for twenty places.
decision-makers: VirtualCortex maintainers
---

# ADR-0166: The sweep after the shards — ADR-0158 dealt the runtime's sweep to twelve shards, a dispatch at `scope=both` became twenty-five jobs for twenty places, and on its first run the whole-domain shards a merge waits for ended an hour later than before; the sweep's jobs now wait for the shards, which keeps the twelve and gives the merge's path the first places

## Context and Problem Statement

[ADR-0158](0158-the-sweep-off-the-next-decisions-path.md) dealt the runtime's sweep to twelve shards and wrote of its cost: *"if a round reads its gate held past its shards, the count is revisited."*

**The first run** was brief 066's dispatch, run [38111246283](https://github.com/DescentVTT/VirtualCortex/actions/runs/38111246283), dispatched at 04:20:04Z on 2026-10-11 with twelve whole-domain shards and thirteen jobs of the sweep:
- its jobs started one after another over more than an hour, from 04:21Z to 05:27Z, the shards and the sweep's mixed;
- the last whole-domain shard started at 05:23Z and ended at 06:16Z, **1 h 56 m after the dispatch**;
- the two dispatches before it, of nineteen jobs each, ended their shards 52 and 55 minutes after.

What was held was not the pull request's gate but the shards themselves, which is the worse case: a round's merge waits for them ([ADR-0150](0150-a-round-waits-for-what-it-checks.md)).

**What is not known**: how much of the hour was the twenty places and how much the platform being slow to start runners that day. Six jobs were running sixteen minutes after the dispatch, which is fewer than twenty places would explain. Both point the same way: the jobs a merge waits for should not queue behind jobs it does not wait for.

## Decision Drivers

- **A measured cost** on the path ADR-0150 and ADR-0158 exist to shorten.
- **A structural boundary beats a reviewed one** (principle 5): the order is the workflow's, not luck's.
- **No check is removed**, and the twelve shards the maintainers chose stay.
- Latest ≠ Newest: `needs` and a status function are the platform's own keys.

## Considered Options

1. (a) the sweep's jobs wait for the whole-domain shards; (b) the runtime's sweep back to six shards, or to seven; (c) a cap on how many of the sweep's jobs run at once; (d) nothing, on one reading.

## Decision Outcome

**Option 1(a).**

- **`mutants-weekly` needs `weekly`.** Where both run, the sweep's thirteen jobs start when the last whole-domain shard has ended.
- **It runs whatever the shards did**: its condition holds `!cancelled()`, so it runs where the shards were skipped, as at `scope=mutants`, and where one of them failed. A sweep still hides nothing behind another job's failure.
- **During a round's dispatch** the places are then the twelve shards' and the pull request's five, seventeen of twenty, with nothing waiting.
- **Expected**: the sweep ends about an hour later than it would have, at about the time the shards take, which is on no merge's path and on no decision's (ADR-0158). That is an expectation, not a reading.

### What does not change

- What the sweep mutates, its twelve shards, its bound and its completeness check.
- The schedule: on Mondays the shards run and then the sweep.
- A dispatch at `scope=mutants`, which runs the sweep at once.

## Consequences

- Good: the shards a merge waits for take the first places, by the workflow's order.
- Good: the twelve shards stay, and the pull request's gate no longer queues behind the sweep.
- Bad: the sweep's outcome is known about an hour later. Nothing waits for it.
- Bad: one reading, on a day the platform was slow. The order is right on any day; how much it buys is not measured.
- Neutral: the first run of this order is the next dispatch at `scope=both`, or Monday's schedule.

## Alternatives considered and why rejected

- **Option 1(b), fewer shards**: seven fit the twenty places beside the shards and not beside the pull request's five; six is where ADR-0158 started. Either gives up the sweep's wall time to fix an order.
- **Option 1(c), a cap on the sweep's jobs at once**: the shards would still share the places with whatever the cap lets through.
- **Option 1(d), nothing**: the next dispatch at `scope=both` pays the hour again if the cause is the places.

## Confirmation

- `.github/workflows/ci.yml`: `needs: [weekly]` on `mutants-weekly`, and `!cancelled()` in its condition.
- Whitepaper §9's row and a directive that holds the workflow to the order.
