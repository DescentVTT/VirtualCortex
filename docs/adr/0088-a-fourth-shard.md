---
status: accepted
date: 2026-09-23
amends: ADR-0073
decision-makers: VirtualCortex maintainers
---

# ADR-0088: A fourth shard — the three shards read 48, 23 and 77 per cent of their bound on a runner two and a half times slower than the one before it, and every measuring round adds a twenty-minute test, so the matrix takes a fourth shard and the script its `n`; the floor is now the longest single test, which is said rather than hidden

## Context and Problem Statement

[ADR-0084](0084-the-shards-take-tests.md) made the weekly `exhaustive` shards take tests rather than binaries, and its dispatch read **36 m 52 s, 30 m 40 s and 43 m 40 s** — the heaviest shard's tests 2 585 s, 36 per cent of the 7 200 the job allows. Two rounds later, [ADR-0087](0087-inhibition-off-the-reward-gate-measured.md)'s dispatch read **58 m 10 s, 28 m 04 s and 93 m 37 s**, the heaviest shard's tests 5 570 s, **77 per cent**:

| Shard | `cortex_core` | `everywhere` | `inhibition` | `instrument` | `learning` | `reference` | The shard's tests |
| ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| 0 | — | — | 1 in 1 238 | 9 in 1 647 | 4 in 237 | 2 in 324 | 3 446 s, 48 % |
| 1 | 1 in 2 | — | — | 10 in 1 117 | 3 in 103 | 3 in 401 | 1 623 s, 23 % |
| 2 | — | 1 in 2 294 | — | 9 in 1 901 | 4 in 674 | 2 in 701 | **5 570 s, 77 %** |

The round robin did what ADR-0084 built it to do: H-14's, H-15's and H-16's twenty-minute tests fell in three different shards, and `instrument`'s twenty-eight are nine or ten to a shard. What changed is the runner. ADR-0087 read it as "about 2.5 times ADR-0084's runner, past the 1.66 spread ADR-0072 measured, and its shard at 77 per cent of the bound with the same tests that read 36 there", and left the next move as a reading: **"a reading for the next round that adds a heavy test and not a decision here."**

This is that decision, and it is due before the next measuring round, not after it: every round since ADR-0077 has added a test of five to twenty minutes, and at 77 per cent one more twenty-minute test on a runner of that speed passes the bound.

## Decision Drivers

- The margin against the slowest runner seen, not the trend: the spread is now measured at 2.5 times, not the 1.66 ADR-0072 read.
- No list of tests or of times written anywhere (F-44, F-45): whatever the split is, it is read from `--list` at run time.
- The script already takes `n`; ADR-0073 wrote it that way "for the day that changes".
- A shard is a job: four jobs where three were, each building the workspace in the release profile behind one cache key. The sweep already runs seven.
- Latest ≠ Newest (§2.1): no new tool.

## Considered Options

1. **A fourth shard**: the matrix `[0, 1, 2, 3]` and the script's `n` at 4.
2. **Balance the shards by their measured times** (ADR-0084's fifth option): deal the tests longest-first from a table of last week's seconds.
3. **Raise the bound** past 120 minutes per shard.
4. **Split the longest tests** — H-15's and H-16's arms into two tests each.
5. **Nothing yet**: wait for a shard to fail.

## Decision Outcome

**Option 1.** In `.github/workflows/ci.yml`: the `weekly` matrix takes `shard: [0, 1, 2, 3]`, the step runs `exhaustive-shard.sh ${{ matrix.shard }} 4`, the job's name and the summary's heading read "of 4". The script is unchanged — ADR-0084's enumeration, round robin and completeness check take `n` as they are.

By the tree's present listing — 49 tests in six binaries, `cortex_core` 1, `everywhere` 1, `inhibition` 1, `instrument` 28, `learning` 11, `reference` 7 — the round robin at four deals `instrument` seven to a shard and puts the three twenty-minute tests in three different shards, as it did at three. With ADR-0087's seconds, on that runner, the shards come to about 1 850, 1 850, 3 940 and 3 000 s: **the heaviest about 55 per cent where it read 77**, and about 22 per cent on ADR-0084's runner. The dispatch below is the reading.

- **Why not balance by measured times** (option 2). It would need last week's seconds in the tree or in the job, which is the list F-44 and F-45 are about, and it would make this week's split depend on a runner that may not come back. It stays the answer if a fifth shard is ever not enough.
- **Why not raise the bound** (option 3). The bound is the signal; ADR-0071 and ADR-0073 rejected raising it twice.
- **Why not split the long tests** (option 4). A test that builds the settled image and runs three arms from it is one measurement; splitting it would build the image three times or pass state between tests. The cost is real and the shard count is the cheaper lever.
- **Why not wait** (option 5). The failure mode is a weekly job that times out on `main` for a reason no diff explains, and the next round adds a test.

**The floor this decision leaves**, said rather than hidden: a shard cannot be smaller than the longest single test it holds. That is now H-15's arms at 2 294 s on ADR-0087's runner — 32 per cent of the bound alone — with H-16's at 1 238 s behind it. A fourth shard buys the margin back to about 55 per cent on that runner; a fifth would buy little more while the longest test stands where it is, and the next lever after that is option 2 or option 4, not another shard.

### Consequences

- Good: the heaviest shard returns to about half its bound on the slowest runner measured, and to about a fifth on the fastest.
- Good: one line of the matrix and one argument; the script, its parser and its completeness check are ADR-0084's, unchanged.
- Neutral: a fourth job per weekly run, sharing the `exhaustive` cache key.
- Bad: the floor is the longest single test and this decision does not move it; a round that adds a test longer than about forty minutes on a slow runner meets it again.
- Bad: the job's name changes from "…/3" to "…/4". `main` is not a protected branch and the weekly job is skipped on pull requests, so no required check is renamed.

## Alternatives considered and why rejected

- **Options 2 to 5**, on the reasoning above.

## Confirmation

- `.github/workflows/ci.yml`: the matrix, the script's argument, the job's name and the summary's heading.
- Whitepaper Appendix B's line for the whole-domain tests and `CLAUDE.md`'s command comment, which read "three shards".
- **The sharded job dispatched on this change's branch** at `scope=exhaustive` — the diff is the exhaustive job's matrix and documents, [ADR-0075](0075-the-dispatch-scope-follows-the-diff.md)'s last clause — run [35800692998](https://github.com/DescentVTT/VirtualCortex/actions/runs/35800692998): **four shards, all green**, every completeness check holding, the union 49 tests, **22 m 07 s, 32 m 47 s, 31 m 57 s and 56 m 06 s** of their 120-minute bound where three read 58, 28 and 94 minutes. Each binary's share in seconds:

  | Shard | `cortex_core` | `everywhere` | `inhibition` | `instrument` | `learning` | `reference` | The shard's tests |
  | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
  | 0 | — | — | — | 7 in 1 029 | 3 in 13 | 2 in 249 | **1 291 s**, 18 % |
  | 1 | 1 in 2 | — | — | 7 in 735 | 3 in 477 | 2 in 708 | **1 922 s**, 27 % |
  | 2 | — | 1 in 1 169 | — | 7 in 478 | 3 in 223 | 1 in 3 | **1 873 s**, 26 % |
  | 3 | — | — | 1 in 1 205 | 7 in 1 775 | 2 in 238 | 2 in 101 | **3 319 s**, 46 % |

  The round robin deals `instrument` seven to a shard as the count says it must, and puts H-15's test, H-16's test and `instrument`'s heaviest in three shards — but not H-16's and `instrument`'s heaviest, which share shard 3 and make it the heaviest at 46 per cent.

  **A binary's seconds are not a runner's speed alone**: the same tests contend for the runner's cores with whatever shares their shard. H-15's test read **1 169 s here against 2 294 s in ADR-0087's dispatch**, where it ran beside nine of `instrument`'s, and `reference`'s seven read 1 061 s against 1 426; `inhibition`'s read 1 205 against 1 238 and `learning`'s eleven 951 against 1 014, which are the same within a tenth. So part of the fall from 77 to 46 per cent is the fourth shard and part is the contention it removed, and the two cannot be separated from these tables. What the bound sees is the shard's total, and no shard is above half of it.

  At ADR-0087's runner's speed — taking `reference` and `instrument`, the two least affected by what shares their shard, as about a third slower there — the heaviest shard would read about 60 per cent where three shards read 77. The floor stands where this decision put it: the longest single test.
