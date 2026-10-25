---
status: exploratory
date: 2026-10-25
---

# 002: What the pinned tables cost

**Branch**: `explore/002-what-the-pinned-tables-cost`
**Commit**: `5ee329b` (main's tests, the baseline), `0807bb8` (the tables in the source and in a data file), `532d2b6` (the data file read at run time), `4cac2e8` (the data file in the binary by `include_str!`); the build-cost job as run at `f447345`, the move script and the readers at `7983e5b`
**Runs**: [40192831652](https://github.com/DescentVTT/VirtualCortex/actions/runs/40192831652) (the three trees built in turn on each runner), [40192286352](https://github.com/DescentVTT/VirtualCortex/actions/runs/40192286352) and [40191314022](https://github.com/DescentVTT/VirtualCortex/actions/runs/40191314022) (earlier build-cost runs, a runner a tree, the second with incremental compilation on), [40192289744](https://github.com/DescentVTT/VirtualCortex/actions/runs/40192289744) on `0807bb8` and [40192292643](https://github.com/DescentVTT/VirtualCortex/actions/runs/40192292643) on `532d2b6` (`scope=arms`); read, not made: the sweeps [40097581635](https://github.com/DescentVTT/VirtualCortex/actions/runs/40097581635) and [40171928796](https://github.com/DescentVTT/VirtualCortex/actions/runs/40171928796), the calibration [40164773490](https://github.com/DescentVTT/VirtualCortex/actions/runs/40164773490), the arms [40170518252](https://github.com/DescentVTT/VirtualCortex/actions/runs/40170518252), the gates [40183165634](https://github.com/DescentVTT/VirtualCortex/actions/runs/40183165634) and [40178742220](https://github.com/DescentVTT/VirtualCortex/actions/runs/40178742220)

The rounds pin their tables as `const` items in the test sources. On `main` at 8d7f36d, `tests/inhibition.rs` is
424 145 lines and 13.5 MB with 1 078 `const` and `static` items, and `tests/assembly.rs` is 70 412 lines and 2.7 MB.
[F-76](../WHITEPAPER.md#11-risks-and-technical-debt) and F-78 name these tables as what grows the test crates' build,
and [ADR-0211](../adr/0211-the-round-shortened.md) left where they live to an exploration. This note asks how much
the tables cost the test crates' build, the pull request's gate and the mutation sweep, and how much another
placement would save. It writes no prediction and decides nothing. **Every figure here is exploratory**: from one or
two runs on GitHub's hosted runners, four cores each, whose speed differed by a quarter between two runners of one
image.

**In brief**
- **The tables cost the builds little.** In release, building them as `const` items or reading them from a file made
  no difference beyond the runners' spread: 67.2 s against 69.4 s and 67.5 s for the whole workspace's test binaries.
  In debug they cost about 3 s of a cold `cargo check` of 15.6 s, and 2 to 3 s of a mutant's build of 15 to 22 s.
- **What `inhibition` costs to build is its code.** Without 10.4 MB of tables it is still the longest unit of every
  build, about 39 s in release.
- **The sweep's time is in its tests.** Building was 12.2 % of the runtime's 89.3 machine-hours in brief 079's sweep.
  `criticality` passing before the binary that caught a mutant took 40.6 h of them.
- **The gate's builds are about a minute and a half of each job.** Most of the release step's 493 s on #234's run was
  the tests running, not the 46 s of build.

## What ran

**The build-cost job** (`scripts/build-cost.sh`, on the branch alone) builds every test binary of the workspace from
`cargo clean`, then changes one line under `runtime/cortex-runtime/src/` as cargo-mutants writes a mutant and builds
again with the sweep's own command, `cargo test --no-run -p cortex-runtime`. It runs with `CARGO_INCREMENTAL=0`, as
rust-cache sets it for every other job of the workflow; no build in the sweep's logs carries `-C incremental`. In debug
it also times a cold `cargo check --workspace --all-targets`. Cargo's `--timings` report gives each unit's seconds.
The readers in `scripts/explore-002/` make every table below from the runs' artifacts.

The first two runs built each tree on a runner of its own, and the runners differed more than the trees did:
`tests/instrument.rs`, which no tree changes, compiled in 11.6 to 12.5 s in release on one runner and in 8.6 s on
another. The run this note reads, 40192831652, checks out all three trees in turn on one runner, in two rounds, the
second in reverse order and with a second mutant. It does this on two runners per profile, so each figure is the mean
of four builds.

**The placements.** Of the items in `tests/inhibition.rs`, 204 are plain literals of at least a kilobyte (integers,
`true`, `None`, tuples, arrays, `&[..]`), 10 428 412 bytes. In `tests/assembly.rs` there are 251 such literals,
2 096 838 bytes. A script, `scripts/explore-002/move-tables.mjs` on the branch, moved them, character for character,
into `tests/data/inhibition.tables` and `tests/data/assembly.tables`. `tests/instrument/data_file.rs`, 257 lines,
parses each into its `const`'s type the first time a test reads it, through a `static LazyLock`. Sixteen items of
`tests/assembly.rs` that name moved tables in their values became lazy too. Uses that are not an index or a method
call read `(*NAME)`. No table was read in a const context, so none had to stay. Afterwards `tests/inhibition.rs` is
76 743 lines and 3.1 MB, and `tests/assembly.rs` 16 356 lines and 0.6 MB.
- `532d2b6` reads each data file from the disk at run time.
- `4cac2e8` compiles it into the binary with `include_str!` and parses it at run time.

## 1. What each test binary costs to build, on main's tree

Seconds per unit on `5ee329b`, the mean of four builds in run 40192831652. *Cold* is from `cargo clean`; *a mutant* is
the sweep's command after one changed line under `src/`. The units overlap on four cores, so they do not sum to the
build's wall clock, the last row.

| Unit | Check, cold | Debug, cold | Debug, a mutant | Release, cold | Release, a mutant |
| :--- | ---: | ---: | ---: | ---: | ---: |
| `inhibition` | 9.1 | 17.8 | 13.8 | 38.8 | 35.5 |
| the runtime's unit tests | 2.4 | 6.8 | 7.3 | 18.0 | 22.5 |
| `assembly` | 2.5 | 6.4 | 5.8 | 16.7 | 15.9 |
| `instrument` | 0.9 | 3.4 | 3.0 | 9.0 | 9.2 |
| `image` | 0.4 | 2.3 | 2.3 | 7.7 | 7.2 |
| `everywhere` | 0.6 | 2.4 | 2.3 | 5.8 | 5.8 |
| `reference` | 0.2 | 1.6 | 1.6 | 5.4 | 5.3 |
| `criticality` | 0.2 | 1.4 | 1.4 | 4.5 | 4.6 |
| the runtime's other 12 test binaries and its library, together | 2.4 | 13.0 | 12.2 | 31.5 | 30.5 |
| the 33 other workspace crates, together | 8.5 | 12.6 | | 20.5 | |
| criterion's dependencies, together | 25.0 | 25.0 | | 59.5 | |
| **the build's wall clock** | **15.6** | **25.8** | **18.5** | **67.2** | **50.4** |

The four builds of a column spread by up to a third: release cold read 57 to 78 s. With incremental compilation on,
in run 40191314022, the debug rebuild of every test binary after a mutant took 4 to 6 s, against 15 to 22 s for the
runtime's alone with it off.

The release `inhibition` binary is 11.4 MB, where the runtime's other test binaries are 2 to 3.5 MB; in debug it is
55 MB.

## 2. The sweep: building is an eighth, the tests the rest

Each runtime mutant's `outcomes.json` gives its two phases: the build, `cargo test --no-run -p cortex-runtime`, and the
test, `cargo test -p cortex-runtime`. Each runner runs two cargo-mutants jobs at once, so a build shares the runner
with the other job's tests.

| Sweep | Mutants read | Build, s | Test, s | Build's share |
| :--- | ---: | ---: | ---: | ---: |
| 40097581635, brief 079, 12 shards | 1 457 | 39 107 | 282 428 | 12.2 % |
| 40171928796, brief 080, the 6 of 16 shards that had ended | 570 | 14 405 | 83 590 | 14.7 % |

A viable mutant's build took 28 s at the median, 37 s at the 90th percentile and 52 s at most in brief 079's sweep.
A shard's first build was no longer than its others: without incremental compilation, every mutant compiles the
runtime and its twenty test binaries again in full.

The test phase is where the time is, and it is concentrated. Of brief 079's 89.3 runtime machine-hours:
- **164 caught mutants whose test phase ran past a minute took 67.9 %.** The binary that failed was `image` for 69 of
  them, `no_alloc` for 34, `criticality` for 24, `reference` for 20 and `store` for 8.
- **Before that binary, the binaries cargo runs earlier by name passed.** `criticality` passing took 40.6 h of them,
  `inhibition` 5.8 h and `instrument` 2.5 h.
- **The 17 timeouts took 14.9 %.**
- **All 1 457 builds took 12.2 %.**

## 3. The gate: the builds are about a minute and a half of each job

From the steps' logs on pull request #234's run, 40183165634, whose diff changed the workflow and so ran every Rust
job:

| Job | What it builds, and how long | Its test steps |
| :--- | :--- | :--- |
| Rust | check 10.0 s, debug test binaries 19.2 s, release 46.1 s, clippy 13.7 s, rustdoc 2.2 s, benchmarks 1.7 s | Test 574 s; Test in the release profile 493 s; each includes its build |
| MSRV | check 12.1 s, test binaries 27.4 s | 1 356 s |
| AArch64 | test binaries 24.5 s | 917 s |

In the release step the binaries ran for 447 s: `criticality` 245 s, `reference` 96 s, `sleep` 69 s, `inhibition`
19 s. In the debug step: `criticality` 215 s, `reference` 127 s, `inhibition` 127 s, `instrument` 51 s. F-76 named the
tables' compile time as what grew the release step to 516 s. On this run the step's build was 46 s of 493.

Elsewhere, from the same kind of log:
- **The mutation gate** of #232, run 40178742220: its unmutated baseline was 20 s of build and 941 s of tests; its 61
  mutants took 39 minutes.
- **The weekly shards** of brief 080's calibration, run 40164773490: each of the 18 built the release test binaries in
  46 to 73 s, about 18 runner-minutes a dispatch, in jobs of 40 to 83 minutes.
- **The arms** of brief 080, run 40170518252: each of the twelve AArch64 jobs built them in 46 to 67 s.

## 4. Two placements

The same job and the same four builds per figure, the three trees in turn on each runner:

| Build | main's tree | data file, read at run time | `include_str!` |
| :--- | ---: | ---: | ---: |
| check, cold | 15.6 | 12.2 | 12.7 |
| debug, cold | 25.8 | 24.7 | 25.1 |
| debug, a mutant (the sweep's build) | 18.5 | 15.6 | 16.6 |
| release, cold (the gate's, a shard's, an arm's) | 67.2 | 69.4 | 67.5 |
| release, a mutant | 50.4 | 49.0 | 50.9 |
| `inhibition` alone, release, cold | 38.8 | 39.6 | 43.7 |
| `inhibition`'s release binary, MB | 11.4 | 6.0 | 16.4 |

Only `cargo check` moved by more than the builds' spread: about 3 s, all of it in `inhibition` and `assembly`
(9.1 + 2.5 s on main's tree, 6.6 + 1.9 s with the data file). A mutant's debug build read 2 to 3 s shorter, within a
spread of 14.9 to 22.0 s on main's tree. Release did not move.

Reading the tables costs a test binary, once per process: 0.35 to 0.38 s for `inhibition` in debug, 26 to 37 ms in
release, and 0.09 to 0.10 s and 6 to 9 ms for `assembly`, on x86-64. On AArch64 in release, read once, they took
40 ms and 10 ms.

**Priced against the sweep and the gate**, at the most these readings allow:
- **The sweep:** 2 to 3 s on each of brief 079's 1 457 builds is 0.8 to 1.2 h of its 89.3 runtime machine-hours,
  before the 0.35 s each mutant that reaches `inhibition` would then spend reading.
- **The gate:** about 3 s on the Rust job's check and on the MSRV's, of jobs of 15 to 25 minutes. Clippy runs the same
  front end and was not measured.

## 5. The tables read back

No pin's value changed: the data files are the source's text, character for character.
- **`0807bb8` holds both the consts and the data files.**
  `the_moved_tables_read_as_written_in_{inhibition,assembly}_explore` held every one of the 204 and 251 tables read
  from the file to its `const`: on AArch64 at `scope=arms` (run 40192289744), and in debug and release on x86-64 in
  run 40192286352.
- **On `532d2b6`, the data file read at run time, two weekly tests ran again under `_explore` names**, at `scope=arms`
  (run 40192292643):
  - `the_kick_and_the_background_pins_from_the_data_file_explore` is `the_kick_and_the_background_at_1024_units_exhaustive`,
    its pins `BACKGROUND_ROWS_1024` and `CONTROL_ROWS_1024` read from the file. It passed in 98 s.
  - `a_reward_one_trial_late_on_seed_22_from_the_mirrored_assignment_pins_from_the_data_file_explore` is H-43's arm
    on seed 22 from the mirrored assignment, one trial late. Its pins include `KEPT_ARM_BLOCKS`, the largest table
    moved, 618 278 bytes of source. It passed in 1 058 s.
- **`inhibition`'s and `assembly`'s gate tests** passed on `532d2b6`. They ran on the developer machine as the
  pre-push check, and their times are not read here.

## What it suggests

- **The tables are not what the builds pay for.** F-76 and F-78 name them as the cause of the release step's growth
  and of the sweep's length. On these runs the tables cost no measurable release time, a few seconds of debug, and
  about 1 % of the sweep. Whether those findings should say so is the maintainers' to decide, by an ADR.
- **Moving them would buy about 3 s of `cargo check`.** It would cost every moved `const` its type-checked literal, a
  `LazyLock` behind every read, a parser of 257 lines, and a third of a second of reading in each debug process of
  `inhibition`.
  These runs suggest no placement worth a decision ADR.
- **The sweep's time is in its tests, and it is concentrated.** About 83 % of the runtime's machine time went to 181
  mutants: 164 caught slowly, 17 timed out. A question for a later exploration: how much of the 40.6 h of `criticality`
  passing a mutant that a later binary catches would go if the binaries that catch most ran first, or if each test ran
  in a process of its own (brief 065's nextest).
- **Incremental compilation in the sweep's jobs** read 4 to 6 s for a mutant's rebuild where 15 to 22 s is paid now.
  That is at most a few hours of a sweep. It was read once, on a runner alone, and is a question, not a reading.
- **What `inhibition` itself costs to build** is its 77 000 lines of test code, about 39 s in release and always the
  build's longest unit. Splitting that binary would shorten the release build's critical path, and only that path:
  another question, not asked here.
