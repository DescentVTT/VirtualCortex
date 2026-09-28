---
status: accepted
date: 2026-09-28
depends-on: ADR-0118
decision-makers: VirtualCortex maintainers
---

# ADR-0121: A sixth shard — the weekly job's whole-domain tests dealt to six shards where ADR-0118 dealt them to five, because the table ADR-0120's dispatch regenerated, read on runners about a fifth slower, plans every shard at 57 per cent of its bound at five; one matrix entry and one argument, the script unchanged; the last shard this lever buys, since the longest test alone is 37 per cent

## Context and Problem Statement

[ADR-0118](0118-a-fifth-shard.md) dealt the whole-domain tests to five shards, so that a round's first dispatch has room for the tests the cost table does not yet know. At that time the table was 88 tests and 25 743 s, and it planned about 36 per cent a shard.

[ADR-0120](0120-the-contexts-own-inhibition-measured.md)'s dispatch (run 36346275507) read two things.
- **The runners were slower.** The 88 earlier tests took 30 743 s against the table's 25 743, 19 per cent more.
- **One shard reached 73 per cent of its bound.** It drew that round's condition (c) cells, costed at 900 s and taking 2 680, beside H-20's arm from the mirrored assignment, which took 1 896 s against the table's 1 275.

The table regenerated from that dispatch is 98 tests and 40 263 s. Replayed through [ADR-0092](0092-the-shards-dealt-by-cost.md)'s deal it plans each of five shards at 8 052 to 8 053 s summed, about 4 090 s of tests' wall time at the run's ratio of 1.97 to 2.00: **57 per cent of the bound before any new test**. A round that adds about 2 000 s on runners as slow passes the briefs' 60 per cent, and the next round of the learning line builds a mechanism and measures it.

## Decision Drivers

- The briefs' 60 per cent, and a weekly job that does not time out on `main` for a reason no diff explains (ADR-0088).
- ADR-0118's reasoning, unchanged: a round's first dispatch deals its new tests blind, and the shard count is the cheapest lever that moves nothing a test reads.
- The floor: a shard cannot be smaller than its longest test.

## Considered Options

1. **Six shards.**
2. **Fewer copies of H-20's arm.** Three whole-domain tests besides H-20's own two run its arm to reach the image it leaves, each taking 1 678 to 2 680 s.
3. **Five shards, and the next brief to hold its cost.**

## Decision Outcome

**Option 1.** In `.github/workflows/ci.yml` the `weekly` matrix takes `shard: [0, 1, 2, 3, 4, 5]`, the step runs `exhaustive-shard.sh ${{ matrix.shard }} 6`, and the job's name and the summary's heading read "of 6". `scripts/exhaustive-shard.sh` and `scripts/exhaustive-costs.mjs` take `n` as they are, and are unchanged.

The present table, dealt by `plan`'s rule, gives each shard 6 710 to 6 711 s summed: about 3 400 s of tests' wall time, **47 per cent of the bound, where five read 57**.

**The floor** is ADR-0120's condition (c) cells at 2 680 s, 37 per cent of the bound by itself. A seventh shard would plan about 41 per cent, four points above that floor. **This is the last shard the lever buys**: the next lever is option 2, or splitting the longest tests.

### Consequences

- Good: about ten points of margin back, for the next round's first dispatch.
- Good: one matrix entry and one argument; the deal, the script and the completeness check are unchanged.
- Neutral: a sixth job per weekly run, sharing the `exhaustive` cache key.
- Bad: the job's name changes from "…/5" to "…/6". The ruleset requires the Rust, minimum-version and documentation checks by name (read on 2026-09-27, ADR-0118), and the weekly job is skipped on pull requests, so no required check is renamed.
- Bad: the floor is now within ten points of the plan. A round that adds a test longer than about 2 700 s on these runners sets a new floor; that is what option 2 is for.

## Alternatives considered and why rejected

- **Option 2, fewer copies of H-20's arm**: a test that reaches the drained image rebuilds it, since the whole-domain tests share no state. Carrying the image between tests would be a fixture in the tree or a new mechanism in the job. It is named as the next lever, not taken now.
- **Option 3, the brief to hold it**: a brief cannot know its tests' costs before its dispatch measures them (ADR-0118).

## Confirmation

`.github/workflows/ci.yml`'s `weekly` job. The dispatch below reads six shards, and the next round's evidence says what each took. Whitepaper 4.71.0 carries §9's row and the shard count in its commands section.
