---
status: accepted
date: 2026-09-24
amends: [ADR-0073, ADR-0084]
decision-makers: VirtualCortex maintainers
---

# ADR-0092: The shards dealt by cost — the round robin reads names, not costs, and put two of the longest tests in one shard at 66 per cent of its bound, so the tests go longest first to the least-loaded shard, costed by a table the weekly run itself measures: each test in a process of its own, its seconds its own; the table decides the balance and never the coverage, and a line that names no test fails `npm run spec`

## Context and Problem Statement

[ADR-0084](0084-the-shards-take-tests.md) made the weekly `exhaustive` shards take tests rather than binaries and named what it left: *"which tests share a shard is decided by the alphabetical order of their names, not by their cost; two of the longest tests can land together … the answer then is option 5 [balance by measured times] or a fourth shard, not a list."* [ADR-0088](0088-a-fourth-shard.md) took the fourth shard and named its floor, the longest single test. Two dispatches later the round robin did what ADR-0084 said it could:

| Dispatch | Shard 0 | Shard 1 | Shard 2 | Shard 3 | What shares the heaviest |
| :--- | ---: | ---: | ---: | ---: | :--- |
| [ADR-0088](0088-a-fourth-shard.md) | 18 % | 27 % | 26 % | **46 %** | H-16's test and `instrument`'s heaviest |
| [ADR-0091](0091-the-assignment-reversed-measured.md) | 22 % | **66 %** | 43 % | 34 % | H-17's mirrored arm (1 500 s) and seven of `instrument`'s in 2 793 s |

The shards' totals differ by a factor of three on one runner, from nothing but the order of the tests' names; on a runner as slow as ADR-0087's the 66 per cent reads near 90. Every measuring round since ADR-0077 has added a test of five to twenty minutes, so each round reshuffles the names and the next cluster is a matter of time. A fifth shard lowers the odds of a cluster without removing it, and each shard is a job that builds the workspace.

Two facts of the tree bound what a cost-aware deal can be:

1. **libtest on the stable toolchain reports no test's seconds.** `--report-time` is unstable, and a binary run with several `--exact` filters reports one wall clock for all of them. The only per-test time available is a test run alone.
2. **No whole-domain test shares a fixture with another.** Every one builds its own network or image in its own body; no `OnceLock`, `LazyLock` or shared static holds anything between the tests of a binary (`tests/no_alloc.rs`'s serialising mutex is in a binary with no `exhaustive` test). So a test run in a process of its own pays a process start and nothing else — the cost ADR-0073 put on "one process per test" was the fixture, and there is none.

## Decision Drivers

- The margin against the slowest runner seen (ADR-0073, ADR-0088), which a cluster of two long tests spends.
- **No list decides coverage** (F-44, F-45): whatever the deal reads, the tests that run are the tests `--list` names.
- A test's cost must be measured, not guessed, and measured where the deal will use it — on the hosted runner.
- A parser this workflow depends on is tested (ADR-0073: "the two before it were each wrong the first time").
- Latest ≠ Newest (§2.1): no new tool — no `cargo nextest`, no nightly flag.

## Considered Options

1. **Deal by cost from a measured table** — the tests longest first to the least-loaded shard; the table made from the weekly run's own per-test seconds, each test in a process of its own.
2. **A fifth or sixth shard**, round robin as before.
3. **`cargo nextest`**, which times and shards tests natively.
4. **Mark the heavy tests by name** (a `_heavy_` infix) and deal those first.
5. **Retire superseded runs** from the weekly job.

## Decision Outcome

**Option 1**, in three pieces:

- **`scripts/exhaustive-costs.mjs`** (node, no dependency; tested by `scripts/exhaustive-costs.test.mjs`, `npm run spec:costs:test`):
  - `plan <list.log> <costs.tsv> <k> <n>` reads every test `--list` names under its binary (a `Doc-tests` section dropped, a Windows listing read the same), costs each by the table — a test the table does not know at **900 s**, as a heavy one, because every round that has added a whole-domain test since ADR-0077 added one of five to twenty minutes — and deals them **longest first to the least-loaded shard**, the lowest index on a tie, equal costs in byte order of binary and name, so every shard computes the same deal from the same listing and table; it prints shard k's tests and every shard's planned load, and fails when the listing names no test (F-48).
  - `check [costs.tsv]` fails on a malformed or repeated line and on a line whose test's function — the last `::` segment of its name — appears as `fn <name>(` in no `.rs` file under `runtime/` or `crates/`: a renamed or removed test leaves a line `npm run spec` names (`npm run spec:costs`).
  - `from <dir> [--run <id>]` writes a table from a weekly run's per-test artifacts, the passing tests only, headed with the run it came from.
- **`scripts/exhaustive-costs.tsv`**: `<stem>\t<test>\t<seconds>` a line, advisory. **Which tests run is always what `--list` names**, so a stale or missing line costs a shard its balance and never a test its run; a stale line fails `npm run spec`; a missing one is costed as heavy.
- **`scripts/exhaustive-shard.sh`**: the listing (failing on a failed or empty one, F-48), the plan, then **each test of the shard in a process of its own** — `<binary> --ignored --exact <test>`, **half as many at once as the runner has cores** (`xargs -P`, a line an argument so no path is read for quotes or backslashes) — so each test's seconds are its own. Half, because the heavy tests each run an executor of two or four worker threads: at one process a core the first dispatch below read them about three times slower apiece and the shards' test time a fifth higher than ADR-0091's model, which ran each binary's share in one process and the binaries one after another. The fixture costs nothing; the cores are what a process per test must not oversubscribe. Each test must exit 0 **and report one test passed**: a name `--exact` did not match runs nothing and exits 0, and is failed here as a test that did not run, beside a failure and a test that never ran. It writes `exhaustive-tests-<k>.tsv` (each test's seconds, exit and count), `exhaustive-times-<k>.txt` (per binary, its tests in the shard and their seconds summed, ADR-0071's table) and `exhaustive-wall-<k>.txt`; the job's summary is the per-test table longest first, and the artifact carries all of them with the plan and the listing.

The first table is the weekly run's own: this change is dispatched once with the table empty — every test at 900 s, which deals in name order and so reproduces the round robin's clustering to read — its per-test seconds are written into the table by `from`, and it is dispatched again to read the deal. **A measuring round that adds or changes a whole-domain test regenerates the table from its own dispatch** in its evidence commit; brief 041 and every measuring brief after it name that in their evidence deliverable.

- **Why not more shards** (option 2): it lowers the odds of a cluster and leaves them to the names; each shard is a build.
- **Why not `cargo nextest`** (option 3): a new tool on the verification path, where a process per test and a table make the same deal.
- **Why not a naming convention** (option 4): the heavy tests' names are cited by the whitepaper's `@assert-count` directives and by the ADRs that pinned them; a rename ripples through documents for a class the table measures anyway.
- **Why not retire runs** (option 5): rejected by ADR-0071 and ADR-0073, and not revisited.

### Consequences

- Good: the deal reads cost, measured on the runner that pays it; two long tests share a shard only when there are more long tests than shards.
- Good: every weekly run reports every test's seconds, which ADR-0071's per-binary table could not, so the next round sees which test grew and by how much.
- Good: a name `--exact` does not match now fails the shard, where a binary run with several filters would have reported fewer tests and been caught only by its count.
- Neutral: ADR-0071's per-binary seconds are now the sum of the binary's tests' own seconds, not one process's wall clock; the series changes meaning from this dispatch, as it did at ADR-0084.
- Bad: the table is a list in the tree, and it can go stale. Its staleness costs balance only, `npm run spec` names a line that outlived its test, and a round that adds a test is told to regenerate it; a test that grows without a round still reads its old cost until the next regeneration.
- Bad: the floor stays the longest single test (ADR-0088); no deal splits a test.

## Alternatives considered and why rejected

- **Options 2 to 5**, on the reasoning above.
- **Downloading last week's artifacts in the job** to cost the deal: it would make each run depend on the retention of another run's artifacts and on a token with `actions: read`, where a committed table and a round that regenerates it make the same deal without either.

## Confirmation

- `scripts/exhaustive-costs.mjs`, `scripts/exhaustive-costs.test.mjs` (ten cases: the stem on either separator; the listing with a doc-test section and a Windows listing; the table's refusals; the longest-first deal parting what round robin stacks; every test dealt once whatever the listing's order; the empty table cycling; an unknown test costed as heavy; empty shards and a bad count; a stale line naming its place; a table written from a run and read back), `scripts/exhaustive-costs.tsv`, `scripts/exhaustive-shard.sh`, the `weekly` job in `.github/workflows/ci.yml`, and `spec:costs` and `spec:costs:test` in `npm run spec`.
- **The script run on this tree** on a developer machine (Windows, Git Bash) at `0 60`: the first attempt's release build failed to link (`LNK1104`) and the shard exited 1 on F-48's guard; the second listed **51 tests**, dealt shard 0 `cortex-core`'s one, ran it in a process of its own — exit 0, one test passed, 3 s — and wrote the three files. The completeness check, run on a results file written by hand, failed a test that ran nothing, a test that failed and a test that never ran, and passed the one that passed.
- The whitepaper's F-45 row and Appendix B's line for the whole-domain tests; `CLAUDE.md`'s and `CONTRIBUTING.md`'s lists of what `npm run spec` runs.
- **The first dispatch on this change's branch**, the table empty and one process a core, at `scope=exhaustive` (the diff is the exhaustive job's script, its planner, its table and documents, [ADR-0075](0075-the-dispatch-scope-follows-the-diff.md)'s last clause) — run [35889573579](https://github.com/DescentVTT/VirtualCortex/actions/runs/35889573579) at `2fb8249`: four shards green, **all 51 tests exited 0 with one test passed**, the shards' tests **5 618, 3 091, 3 598 and 1 772 s** of wall clock (78, 43, 50 and 25 per cent of the bound). The empty table deals in name order, which is ADR-0084's round robin rotated by one shard, so this is the clustering read with every test's own seconds: the longest were H-14's `the_reinforced_form_at_1024_units_exhaustive` at 5 231 s, H-17's mirrored arm at 5 021 s and `the_recalibrated_rewarded_run_at_256_units_on_four_workers_exhaustive` at 4 655 s. Against ADR-0091's dispatch the same tests read longer — H-17's mirrored arm 5 021 s here against 1 500 s there as a share alone — and the shards' test time summed to 14 079 s against 11 806: four heavy tests at once, each with two to four worker threads, on four cores. That reading changed the decision before merge: the processes now run half a core count at a time. The table made from this run (`from`, 51 lines) deals the four shards **11 644, 11 650, 11 642 and 11 645 s** of those seconds.
- **The second dispatch**, the table from the first and half a core count at a time — run [35901537202](https://github.com/DescentVTT/VirtualCortex/actions/runs/35901537202) at `79f7a9b`: four shards green, **all 51 tests exited 0 with one test passed**, the jobs **39 m 32 s, 44 m 03 s, 41 m 06 s and 40 m 38 s**, the shards' tests **2 326, 2 590, 2 414 and 2 402 s** of wall clock — **32, 36, 34 and 33 per cent** of the bound, the heaviest 1.11 times the lightest:

  | Dispatch | Shard 0 | Shard 1 | Shard 2 | Shard 3 | Heaviest ÷ lightest | The shards' test time |
  | :--- | ---: | ---: | ---: | ---: | ---: | ---: |
  | ADR-0091's, round robin, a binary's share a process | 22 % | 66 % | 43 % | 34 % | 3.0 | 11 806 s |
  | the first here, name order, one process a core | 78 % | 43 % | 50 % | 25 % | 3.2 | 14 079 s |
  | **the second here, by cost, half a core count** | **32 %** | **36 %** | **34 %** | **33 %** | **1.11** | **9 732 s** |

  The tests read their own seconds again once the cores are not oversubscribed — H-17's mirrored arm 1 457 s against 1 500 as ADR-0091's share alone, the tests' seconds summing to 18 207 against 46 581 in the first — and the shards' test time is 18 per cent below ADR-0091's model. The longest test is now H-15's `plasticity_everywhere_at_1024_units_exhaustive` at 2 095 s, **29 per cent of a shard's bound alone**, the floor ADR-0088 named. **The table is regenerated from this run** (`from`, 51 lines, `npm run spec:costs` passing), so the table the weekly reads was measured under the execution it deals for; it deals **4 551, 4 549, 4 554 and 4 553 s** and puts the four longest tests — H-15's, H-16's, H-14's and H-17's mirrored arm — one to a shard. On a runner a third slower, ADR-0087's, the heaviest shard of this run would read about 48 per cent.
