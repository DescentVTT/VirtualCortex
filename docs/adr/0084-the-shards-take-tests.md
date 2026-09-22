---
status: accepted
date: 2026-09-22
amends: ADR-0073
decision-makers: VirtualCortex maintainers
---

# ADR-0084: The shards take tests, not binaries — `instrument.rs` read 61 per cent of its shard's bound with nothing added to it, on a runner four to eight per cent slow, so a runner as slow as ADR-0072's would pass the bound; the floor ADR-0073 named is removed by spreading every binary's tests over the shards, each binary's share run in one process with its tests named as exact filters read from `--list` at run time; and a listing that fails or names nothing now fails the shard, where it passed (F-48)

## Context and Problem Statement

[ADR-0073](0073-the-whole-domain-tests-sharded.md) split the weekly `exhaustive` job into three shards taken from the binaries `--list` names at run time, and named its own limit: **"the floor is now the largest single binary. Sharding cannot split `instrument.rs` … If it approaches 120 minutes alone the next move is not another shard: it is splitting that file, or moving what a superseded round pinned out of the weekly job."**

The series since then, `instrument`'s shard against its 7 200 s:

| Dispatch | `instrument` | Share of the bound | What changed in it |
| :--- | ---: | ---: | :--- |
| [ADR-0077](0077-the-background-side.md) | 2 928 s | 41 % | two tests added |
| [ADR-0079](0079-the-rewards-direction-measured.md) | 3 030 s | 42 % | one test added |
| [ADR-0081](0081-the-reinforced-form-measured.md) | 4 047 s | 56 % | one test of twenty minutes added |
| [ADR-0083](0083-plasticity-everywhere-measured.md) | **4 426 s** | **61 %** | **nothing** — `reference` and `learning` read that runner four to eight per cent slower |

ADR-0073 measured runners 1.35 to 1.67 times slower than one another in a single week (ADR-0072's reading). At 1.66 times ADR-0083's runner's speed `instrument` alone takes about 7 350 s, past the bound: the weekly job would time out on a slow runner **with no round having added anything**, and the scheduled run on `main` would fail for a reason no diff explains. [ADR-0082](0082-plasticity-everywhere.md) kept the next round's runs out of `instrument.rs` by requiring the new binary's name to sort before `instrument`, which steers the round robin and does nothing for `instrument` itself.

ADR-0073 considered and rejected **sharding by test name**: "libtest has no sharding, so it would mean naming tests in the workflow or running one process per test; the first is the drift again and the second pays a process and a fixture per test." Both costs follow from an assumption this decision does not need. The names can be read from `--list` at run time, as the binaries already are; and libtest accepts several filters in one invocation, so a binary's share of a shard runs in **one** process. Verified on the pinned toolchain (1.97.1): `cortex-neuromod`'s unit-test binary given two of its five test names with `--exact` reports "2 passed; … 3 filtered out".

Running the script to verify this change found a second defect. On a developer machine the release build failed to link (`LNK1104`), `--list` printed no test, and the script — reading an empty list as a shard of no binaries — **printed "this shard takes 0 of them" and exited 0.** The same holds on the runner: a `main` that does not build would read as a green weekly job. That is **F-48**, in the script ADR-0073 wrote.

## Decision Drivers

- The margin against a slow runner, not the trend: a job at 61 per cent fails on a runner 1.66 times slower, and ADR-0073 measured that spread.
- No list written down anywhere (F-44, F-45): the unit of a shard, whatever it is, is read from `--list` at run time.
- A binary's tests run side by side in one process today; splitting them must not pay a process per test.
- ADR-0071 and ADR-0073 rejected retiring a superseded round's runs: a pinned number that stops being run stops being true.
- A gate that passes when it ran nothing is worse than no gate (F-44).
- Latest ≠ Newest (§2.1): no new tool on the verification path.

## Considered Options

1. **Split `instrument.rs`**: move a superseded round's tests and their pinned tables into a binary of their own.
2. **Retire superseded pins** from the weekly job.
3. **Raise the bound**, or **add shards**.
4. **`cargo nextest`**, which shards by test natively.
5. **Balance the shards by measured times**, from last week's tables.
6. **Shard by test, the names read from `--list` at run time, each binary's share run in one process with its tests as exact filters.**

## Decision Outcome

**Option 6**, in `scripts/exhaustive-shard.sh <k> <n>`, the `weekly` matrix unchanged:

- `--list` is read as ADR-0073 read it, but for its tests: a `Running … (<path>)` line opens a binary, each `<name>: test` line under it is one of its tests, and a `Doc-tests` line opens a section no binary of the script runs, whose lines are dropped. Every test becomes a line `<path>\t<name>`, sorted in byte order (`LC_ALL=C`), so the order is the tree's whatever the runner's locale.
- Shard `k` of `n` takes every n-th **test**. A binary's tests are spread over the shards in the order of their names; the union is the whole list by construction.
- Each binary with a share in the shard runs **once**, `<binary> --ignored --exact <test> <test> …`, so libtest runs its share side by side as it ran the whole binary, and no test pays a process or a fixture of its own. Its standard input is closed, so no binary reads the loop's list.
- **What each binary reports having run is checked against the names the shard gave it**; a mismatch fails the job, as before.
- **A listing that fails, or names no test, fails the shard (F-48).** The tree holds `exhaustive` tests; a shard that finds none has not run, and says so.
- `exhaustive-times-<k>.txt` keeps its three columns — the binary, the tests of it the shard ran, the seconds they took — so a binary can now appear in every shard's table, with its share. The job summary's column is renamed to say so. The job's name, the three shards and the 120-minute bound per shard do not change, so no required check is renamed.
- **ADR-0082's rule on a new binary's name is superseded**: with tests as the unit, a binary's name no longer decides which binaries share a shard. The rounds that followed it are not changed; the next round may name its binary as it likes.

### Consequences

- Good: the floor ADR-0073 named is gone. `instrument.rs`'s twenty-eight tests go nine or ten to a shard, and the floor becomes the longest single test (about twenty minutes on the runner, ADR-0081) together with whatever shares its shard.
- Good: F-48 is closed in the same change that found it: a tree that does not build, or a listing that changes its format, now reads as a failed weekly job.
- Good: no list, no tool, no new job; the change is the script's enumeration and its loop.
- Neutral: ADR-0071's per-binary series changes meaning from this dispatch on: a binary's seconds are now a share's, per shard; a binary's whole is no longer one number. The series before this ADR stands as it was read.
- Bad: which tests share a shard is decided by the alphabetical order of their names, not by their cost; two of the longest tests can land together. The shards' tables will say so, and the answer then is option 5 or a fourth shard, not a list.
- Bad: the script is the parser this workflow has rewritten most; it is exercised below on the tree's own listing before it is trusted.

## Alternatives considered and why rejected

- **Split `instrument.rs`** (option 1): thousands of lines of pinned tables moved for a floor the next heavy test recreates; the harness module brief 038 made would make it possible, and it is not needed once the unit is a test.
- **Retire superseded pins** (option 2): rejected by ADR-0071 and ADR-0073, and not revisited.
- **Raise the bound, add shards** (option 3): the bound's signal is the point of it; and no number of shards splits a binary under ADR-0073's unit.
- **`cargo nextest`** (option 4): a new tool on the verification path, for a split the script makes in a loop.
- **Balance by measured times** (option 5): the shards would depend on last week's runner and on a list of times kept somewhere; a later decision may take it if the round robin's clustering is ever read to matter.

## Confirmation

- `scripts/exhaustive-shard.sh`, and the `weekly` job's comment and summary in `.github/workflows/ci.yml`.
- Whitepaper §11: F-48, resolved here; F-45's row and Appendix B's line for the whole-domain tests read "tests" where they read "binaries".
- The script run on the tree and the sharded job dispatched on this change's branch: in the evidence below.
