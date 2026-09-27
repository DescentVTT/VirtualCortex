---
status: accepted
date: 2026-09-27
depends-on: ADR-0092
decision-makers: VirtualCortex maintainers
---

# ADR-0118: A fifth shard — the weekly job's whole-domain tests dealt to five shards where ADR-0088 dealt them to four, by ADR-0092's deal unchanged, because each round's new tests are costed at the table's default until their own dispatch measures them, and ADR-0117's first dispatch put one shard at 59 per cent of its bound; one matrix entry and one argument, the script unchanged

## Context and Problem Statement

[ADR-0088](0088-a-fourth-shard.md) dealt the whole-domain (`exhaustive`) tests to four shards of the weekly job, each bounded at 120 minutes. [ADR-0092](0092-the-shards-dealt-by-cost.md) deals them by cost: `scripts/exhaustive-costs.mjs plan` gives each test, longest first, to the least-loaded shard, costed by the seconds `scripts/exhaustive-costs.tsv` records for it. A test the table does not know is costed at a default of 900 s, and `scripts/exhaustive-shard.sh` runs half as many tests at once as the runner has cores.

The learning line's rounds each add whole-domain tests, and the briefs since 049 hold every shard under 60 per cent of its bound. The last round came within a point of that ([ADR-0117](0117-the-region-measured.md), dispatch 36313742568):

| Shard | Job | Tests' wall time | Its heaviest test (s) |
| ---: | ---: | :--- | :--- |
| 0 | 50 m 30 s | 2 983 s, 41 % | H-20 from the mirrored, 1 275 |
| 1 | 71 m 04 s | 4 216 s, 59 % | condition (c), 1 890 |
| 2 | 41 m 18 s | 2 440 s, 34 % | H-18 from the mirrored, 907 |
| 3 | 56 m 33 s | 3 340 s, 46 % | plasticity everywhere, 1 122 |

The cause is not the total. The table did not know the round's nine new tests and costed each at 900 s. On the runners they took 259 to 795 s, and condition (c)'s 1 890 s, which the deal had put in one shard with H-20's arm from the assignment. The table regenerated from that dispatch (88 tests, 25 743 s) replays at 6 435 to 6 436 s summed a shard, about 45 per cent of the bound at that run's ratio of summed seconds to wall time, 1.97 to 2.00.

So the four shards hold the tree as it is, and they hold a round's tests only once a dispatch has measured them. Each round's first dispatch deals its new tests blind, and the rounds are growing. ADR-0116 estimated brief 050 at about a third of the table's cost, and the next round of the learning line, a context with an inhibition of its own, is of that kind.

## Decision Drivers

- The briefs' 60 per cent, and a weekly job that does not time out on `main` for a reason no diff explains (ADR-0088).
- ADR-0092's deal, which balances what the table knows and cannot balance what it does not.
- One lever at a time: the shard count is the cheapest, and it moves nothing a test reads.

## Considered Options

1. **Five shards.**
2. **A better default cost** for a test the table does not know.
3. **Four shards, and each round's brief to hold its first dispatch.**
4. **Raise the bound.**

## Decision Outcome

**Option 1.** In `.github/workflows/ci.yml` the `weekly` matrix takes `shard: [0, 1, 2, 3, 4]`, the step runs `exhaustive-shard.sh ${{ matrix.shard }} 5`, and the job's name and the summary's heading read "of 5". `scripts/exhaustive-shard.sh` and `scripts/exhaustive-costs.mjs` take `n` as they are, and are unchanged.

The present table, dealt by `plan`'s rule, gives each shard 5 148 to 5 149 s summed. At ADR-0117's ratio that is about 2 600 s of tests' wall time, **36 per cent of the bound, where four read 45**. A round the size of ADR-0116's estimate for brief 050, about 7 000 s more, would plan about 46 per cent at five shards and about 58 at four, before the blindness of its first dispatch.

**The floor**, as ADR-0088 said it: a shard cannot be smaller than its longest test. That is condition (c)'s 1 890 s, 26 per cent of the bound by itself, under a fifth shard's 36, so a fifth shard still buys margin; a sixth would buy less.

### Consequences

- Good: the heaviest shard returns to about a third of its bound, and a round's first dispatch has about twenty points of margin for the tests the table does not yet know.
- Good: one matrix entry and one argument; the deal, the script and the completeness check are unchanged.
- Neutral: a fifth job per weekly run, sharing the `exhaustive` cache key.
- Bad: the job's name changes from "…/4" to "…/5". The repository's ruleset requires three checks by name — `Rust (check, test, fmt, clippy)`, `Rust (minimum supported version)` and `Documentation (spec-guard, spec-graph)` (read on 2026-09-27) — and the weekly job is skipped on pull requests, so no required check is renamed.
- Bad: a round's first dispatch still costs its new tests at 900 s. Five shards give it room; they do not make it see.

## Alternatives considered and why rejected

- **Option 2, a better default**: any fixed default is wrong for some test, as 900 s was wrong both ways for brief 050's nine (259 to 1 890 s). A default read from the test's own code would be a new mechanism in the deal, for a problem margin solves.
- **Option 3, the briefs to hold it**: a brief cannot know its tests' costs before its dispatch measures them, which is the problem.
- **Option 4, raise the bound**: the bound is the signal. ADR-0071, ADR-0073 and ADR-0088 rejected raising it.

## Confirmation

`.github/workflows/ci.yml`'s `weekly` job. The next dispatch at `scope=exhaustive` or `both` reads five shards, and its evidence says what each took. Whitepaper 4.68.0 carries §9's row and the shard count in its commands section.
