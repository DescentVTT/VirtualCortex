---
status: accepted
date: 2026-09-21
amends: ADR-0071
decision-makers: VirtualCortex maintainers
---

# ADR-0073: The whole-domain tests run in three shards — the first per-binary table showed one job at 1 h 43 m of its 120-minute bound with half of it in one binary, so the shards are taken from the binaries `--list` names at run time and each checks what it ran against what the list counted; the floor is now the largest single binary, and that is said rather than hidden

## Context and Problem Statement

[ADR-0071](0071-the-exhaustive-job-times-its-own-binaries.md) made the weekly `exhaustive` job report the seconds each test binary took, and left what to do with them to "the round that has the numbers". The first table, from the dispatch of brief 032 (run [35548410926](https://github.com/DescentVTT/VirtualCortex/actions/runs/35548410926)):

| Binary | `exhaustive` tests | Seconds |
| :--- | ---: | ---: |
| `instrument` | 22 | 3 006.92 |
| `reference` | 7 | 2 080.97 |
| `learning` | 11 | 1 084.28 |
| `cortex_core` | 1 | 1.83 |
| **the tests** | **41** | **6 174.00** |

The job read **1 h 43 m 49 s of its 120-minute bound**, the nearest a run has come. [ADR-0072](0072-what-the-trace-is-made-of.md) read the table it had and said what it meant: its own two tests added about a minute, and "the unchanged binaries 1.35 to 1.67 times slower than in the run before" — so most of the rise over the 69 minutes of the two runs before it was the runner, not content. It then handed the question on: "the job's shape, which ADR-0071 left to the round that has the numbers, is now the first thing a measuring round must decide, and this round did not, the workflow being outside its brief."

That is the decision to take, and the table takes it. Whether the rise was runner or content is now beside the point: **a slow runner alone consumes 87 per cent of the bound**, the next measuring round adds to `instrument`, and `instrument` is already half the job. A single job has no margin for both.

## Decision Drivers

- The margin, not the trend, is what fails a run. A job at 104 minutes of 120 fails when a runner is slow, whatever made it 104.
- The sweep's own answer to the same problem is a job per area with a completeness check ([ADR-0058](0058-the-weekly-sweep-and-its-timeouts.md)); the shape is proven here.
- A list of binaries written into the workflow is the drift of F-44 and F-45 again. The split must read the tree.
- A shard that silently runs less than its slice is worse than a slow job.

## Considered Options

1. **Raise the bound.** It removes the signal that would say something is wrong, and ADR-0071 rejected it once already.
2. **Retire a superseded round's runs.** A pinned number that stops being run stops being true; rejected in ADR-0071 for the same reason and not revisited.
3. **A static matrix of the four binaries that have `exhaustive` tests today.** It works until a fifth file gains one and nobody adds it — which is exactly how `spec:decisions` came to run nowhere (F-44) and how one duration came to answer for thirty-seven tests (F-45).
4. **Shards taken from the binaries `--list` names at run time**, each checking its slice.

## Decision Outcome

**Option 4**, in `scripts/exhaustive-shard.sh <k> <n>`, run by a three-way matrix:

- `cargo test --workspace --release --locked -- --ignored exhaustive --list` names every test binary and how many of these tests it holds. It builds nothing it would not build anyway and runs no test. The binaries that hold none are dropped, and the rest are sorted, so the order is the tree's and not the workflow's.
- Shard `k` of `n` takes every n-th line. The union is the whole list by construction; **no list of binaries is written down anywhere.**
- Each binary is run directly (`<binary> --ignored exhaustive`) and timed, and **what it reports having run is checked against what `--list` counted for it**. A mismatch fails the job, as ADR-0058's completeness check fails a partial sweep.
- The per-binary seconds ADR-0071 read out of cargo's lines are now written by the script itself, into `exhaustive-times-<k>.txt` and the job summary, and kept ninety days. The `tee` and the awk pairing of ADR-0071 are superseded by it: that decision's value was the number, and the number now has a shorter path.

Three shards, because four binaries hold these tests and the round robin over the sorted list puts `learning` (18 m) in one, `cortex_core` and `reference` (35 m) in another and `instrument` (50 m) in the third. The bound stays 120 minutes **per shard**, so the margin against the slowest is 2.4 times where it was 1.15.

### Consequences

- Good: a slow runner no longer threatens the job. The worst shard at the measured times is 50 minutes, and the 1.35 to 1.67 times ADR-0072 measured would leave it at 84.
- Good: a file that gains an `exhaustive` test is swept into a shard with no edit here, and a shard that fails to run what the tree holds says so.
- Good: the table survives per shard, so the series ADR-0071 exists to build continues.
- Neutral: three jobs where there was one, each building the workspace in the release profile. They share a cache key, and the sweep already runs seven.
- **Bad: the floor is now the largest single binary.** Sharding cannot split `instrument.rs`, which is 22 of the 41 tests and half the seconds, and briefs 029, 030 and 032 all fed it. If it approaches 120 minutes alone the next move is not another shard: it is splitting that file, or moving what a superseded round pinned out of the weekly job and accepting what ADR-0071 said about that. This decision buys time and names its own limit.
- Bad: the shard script is the fourth parser this workflow has gained in three days. It is tested below, and the two before it were each wrong the first time.

## Alternatives considered and why rejected

- **Options 1 to 3 above**, on the reasoning given.
- **Sharding by test name rather than by binary**, which would split `instrument.rs`. libtest has no sharding, so it would mean naming tests in the workflow or running one process per test; the first is the drift again and the second pays a process and a fixture per test.
- **`cargo nextest`, which shards natively.** A new tool on the verification path against Latest ≠ Newest, for a split four lines of shell already make.
- **Deriving the shard count from the times.** It would make the workflow depend on last week's runner; three is the count of binaries that cost anything, and the script takes `n` as an argument for the day that changes.

## Confirmation

- `scripts/exhaustive-shard.sh` and the `weekly` matrix in `.github/workflows/ci.yml`.
- The enumeration run against this tree: **41 tests in 4 binaries**, which is the count the CI table reported, and the partition at three shards placing `learning`, then `cortex_core` with `reference`, then `instrument`.
- The script run end to end at `1 4`, a shard holding only `cortex-core`'s one test: it enumerated 41 in 4, took its slice, ran it in 2 s, checked 1 against the list's 1 and wrote its row.
- The summary step run against a shard's times file and against an empty one, which reports and exits 0.
- Whitepaper §11's F-45, whose deferred decision this is, and Appendix B's line for the whole-domain tests.
