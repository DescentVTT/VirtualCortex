---
status: exploratory
date: 2026-10-27
---

# 006: Where an arm spends its time

**Branch**: `explore/006-where-an-arm-spends-its-time`
**Commit**: `c25ed27` for every run but the x86-64 run under perf, which ran on `103544a`; the two differ in
`scripts/explore-006/run.sh` alone. Nothing under `src/` differs from `main`. The readers that made this note's tables
are at `cc15173`.
**Runs**: [40439977283](https://github.com/Menteion/Menteion/actions/runs/40439977283) (x86-64 under perf; the 16-block
test once on AArch64; AArch64 under perf, refused), [40440647670](https://github.com/Menteion/Menteion/actions/runs/40440647670)
(the profiles alternated on each architecture; AArch64 under perf) and
[40440650790](https://github.com/Menteion/Menteion/actions/runs/40440650790) (the whole arm once in each profile on each
architecture), all at `scope=arms`; read, not made: brief 082's arm,
[40226384795](https://github.com/Menteion/Menteion/actions/runs/40226384795), and the cost table of dispatch
[40371343923](https://github.com/Menteion/Menteion/actions/runs/40371343923)

In `scripts/exhaustive-costs.tsv`, of the 229 444 s the weekly tests cost, `inhibition`'s 157 tests take 219 435 s,
95.6 %. A round adds about eighteen arms, and with them about twelve minutes to each weekly shard. The tree has no
`[profile]`, so its tests build with no LTO across crates and with 16 codegen units. This note asks two things of one
heavy arm:

1. whether its time is spent in the engine or in the harness that drives and checks it;
2. how much a test profile with whole-program optimisation, `lto = "fat"` and `codegen-units = 1`, shortens it, and
   what its build costs more.

It writes no prediction and decides nothing. **Every figure here is exploratory**: from one or three runs of each
kind, on GitHub's hosted runners, four CPUs each. The x86-64 runners came with three processors, EPYC 7763, 9V74 and
9V45, each two cores of two threads, and the AArch64 runners with Neoverse-N2, four cores of one.

**In brief**
- **The time is the engine's, and most of it is phase 1.** On both architectures the test crate's own code held 6 of the
  test thread's samples, 0.02 % on x86-64 and 0.01 % on AArch64. Of the engine, each awake unit's turn every tick held
  84 % (x86-64) and 65 % (AArch64) of the test thread. STDP, iSTDP and the rest of the fan-out held 0.2 to 0.5 % of
  each thread, and the image's encoding and CRC nothing measurable.
- **The rest is waiting and worker 0's own work.** The spin barrier held 7 % and 18 % of the test thread, 14 % and 24 % of
  the other worker's. Phase 3's injector deliveries and the drive, which worker 0 does alone, held 7 % and 16 % of the
  test thread. On AArch64 the atomics are calls to out-of-line helpers, 12 % of all samples.
- **Fat LTO shortened the arm by 4 to 6 %.** Alternated with `release` on one runner, three rounds each, it read 0.941 of
  `release` on x86-64 and 0.959 on AArch64 over the whole arm. Building every test binary of the workspace took 83 to
  92 s longer, 2.3 times as long. Thin LTO read 0.980 and 0.993 and cost more to build than fat. `codegen-units = 1`
  alone read 0.976 and 0.993, within the runners' spread, for no longer a build.
- **Every pin held bit for bit** under all four profiles on both architectures.
- **The runner and the shard's pairing count for more than any profile.** In `release`, alone on its runner, the arm took
  672 s under perf on an EPYC 9V45 and 1 070 s on an EPYC 7763. The cost table reads 1 941 s from a weekly shard on
  x86-64, which runs two tests at once on two cores.

## What ran

**The arm.** H-45's control on seed 22 from the assignment,
`a_delay_read_on_time_untold_on_seed_22_from_the_assignment_at_1024_units_exhaustive`, 1 941 s in the cost table. It is
`untold_arm(0, Reversal::AssignmentFirst, 0)`: the network's derivation under the probe, run again and held to H-42's
pinned tables (`probe_made`), then 7 680 trials, 120 blocks of 64, from the image it wrote. On the branch,
`timed_untold_run` is `untold_run` copied whole, with the clock read after the derivation, after the engine is decoded
and at each block's end, and printed as `TIME` lines. Two `_explore` tests run it:
- `the_control_on_time_on_seed_22_from_the_assignment_timed_explore`, the whole control, held as `untold_arm` holds it:
  to H-42's arm bit for bit, its 120 blocks, its trace and every reward's value, and to `UNTOLD_CONTROLS`. The control's
  readings hold nothing and are not computed.
- `the_control_on_time_on_seed_22_from_the_assignment_over_16_blocks_timed_explore`, its derivation and its first 16
  blocks, 1 024 of the 7 680 trials. The derivation is held whole and the 16 blocks to the first 16 pinned. It is the
  workload the profiles were alternated on.

The test's thread is the executor's worker 0 and its coordinator, and one spawned worker runs beside it
(`config(units, 2, 0)`). Between phases each waits for the other in a spin barrier, so both threads are on a core the
whole run, and the test thread's samples are the run's wall clock.

**The profiles**, added to `Cargo.toml` on the branch, each inheriting `release`:

| Profile | `lto` | `codegen-units` |
| :--- | :--- | ---: |
| `release`, main's | not set: thin LTO within a crate only | 16 |
| `lto-fat` | `"fat"` | 1 |
| `lto-thin` | `"thin"` | 16 |
| `cgu1` | not set | 1 |

**The jobs.** The arms job runs a test named `<arch>/<mode>/<test>` through `scripts/explore-006/run.sh`, `x86-64` on
the weekly shards' runner and `aarch64` on its own. These jobs use no cache, so every build is cold.
- `perf`: the `inhibition` binary in `release` with line tables (`CARGO_PROFILE_RELEASE_DEBUG=line-tables-only`), and the
  whole control under `perf record` at 49 samples a second a thread.
- `profiles`: every test binary of the workspace built in each profile in turn, as a shard or an arm builds them. Then the
  16-block test ran `release`, `lto-fat`, `lto-thin`, `cgu1` in turn, three rounds, on the same runner, each run in a
  process of its own.
- `release`, `lto-fat`, `lto-thin`, `cgu1`: the `inhibition` binary alone built in that profile, then the whole control
  once.

## 1. Where the time goes

**The engine's.** The test crate's own code held 6 of the test thread's 32 924 samples on x86-64 and 6 of 55 412 on
AArch64. That is all of the harness's derivation checks, replays of the rule, dumps and assertions together: under
0.03 %. perf's report by source file gives inlined code to its own file, and no file under `tests/` reached 0.1 % in it
on either architecture.

**Phase 1, the turns.** Each part of the tick, as a share of each thread's samples (`read.mjs`, from `classify.mjs`'s
`classify-owners.tsv`):

| Part of the tick | x86-64, test | x86-64, worker | AArch64, test | AArch64, worker |
| :--- | ---: | ---: | ---: | ---: |
| Phase 1, the turns: each awake unit's mailbox drained, its inputs summed and scaled, `integrate` | 84.0 % | 84.7 % | 65.3 % | 75.5 % |
| The barrier, waiting | 7.3 % | 14.0 % | 18.1 % | 23.6 % |
| Phase 3, the injector's messages into mailboxes, and the drive | 7.2 % | 0.6 % | 15.6 % | 0.6 % |
| The coordinator, the task and the rest of the engine | 1.1 % | 0.4 % | 0.7 % | 0.1 % |
| Phase 2, the fan-out: STDP, iSTDP, consolidation, release | 0.45 % | 0.30 % | 0.28 % | 0.23 % |
| The image, encoded or decoded, and its CRC | 1 sample | 0 | 2 samples | 0 |
| The test crate | 0.02 % | 0 | 0.01 % | 0 |
| Samples | 32 924 | 32 914 | 55 412 | 55 363 |

The functions, both threads together, as perf's own report gives each sample to the function it was in, inlined code
included:

| Function | Where | x86-64 | AArch64 |
| :--- | :--- | ---: | ---: |
| `DendriticSuperNeuron::integrate` | `crates/menteion-core/src/dynamics/membrane.rs:174` | 45.4 % | 37.8 % |
| `Worker::phase_turns`, with `turn` inlined in it | `runtime/menteion-runtime/src/executor.rs:3663`, `:3696` | 36.6 % | 23.6 % |
| `SpinBarrier::wait` | `runtime/menteion-runtime/src/barrier.rs:32` | 10.5 % | 16.8 % |
| `Pools::free` and `Pools::alloc` | `runtime/menteion-runtime/src/pool.rs:87`, `:70` | 2.4 % | 4.8 % |
| `MailboxDrain::next` and `mailbox_push` | `crates/menteion-core/src/dynamics/neuron.rs:77`, `:226` | 1.7 % | 2.9 % |
| `Drive::step` and `Injector::pop` | `runtime/menteion-runtime/src/synthesis.rs:159`, `injector.rs:90` | 1.7 % | 1.3 % |
| The atomics' out-of-line helpers, `__aarch64_*` | compiler-builtins' `lse.S` | | 11.9 % |
| Everything else | | 1.7 % | 0.9 % |

- **The turn is the cost.** With 1 024 units, each of the two workers owns 512, and every unit not at rest takes a turn
  every tick. The fan-out runs only for a unit that fired, so STDP, iSTDP and plasticity under the reward are a few
  tenths of a percent.
- **The barrier measures the imbalance between the workers.** Worker 0 is also the test thread and the coordinator. In
  phase 3 it alone drains the injector into mailboxes (`executor.rs:3908`), and between ticks it steps the drive. The
  spawned worker waits for it at the next barrier: 14 % of its samples on x86-64, 24 % on AArch64.
- **AArch64 pays for its atomics through calls.** On `aarch64-unknown-linux-gnu` an atomic read-modify-write calls a
  helper that picks LSE or the older instructions at run time. The barrier calls `__aarch64_ldadd8_acq_rel`,
  `mailbox_push` calls `__aarch64_cas8_acq_rel`, `phase_turns` calls `__aarch64_swp8_acq_rel` and the pools call
  `__aarch64_cas4_*`. Those helpers held 11.9 % of the samples. On x86-64 the same operations are inlined.

**How it was read.** On x86-64 (EPYC 9V45) perf sampled `cpu-clock` with DWARF call chains. On the AArch64 runner perf
6.17 refused DWARF chains on every event offered (`PMU Hardware doesn't support sampling/overflow-interrupts`), and the
run sampled `cpu-clock:u` with frame-pointer chains. On neither did a chain reach its thread's root: the DWARF unwinder
stopped after the sampled function on almost every sample, and the binary keeps no frame pointers. So the split rests on
the function each sample was in. That is enough here: a sample in the engine's code is the engine's whoever called it,
and only 3 of the test thread's samples on x86-64 were in a library, the kernel or a frame with no crate. On AArch64 the
frame-pointer chain still named each atomic helper's caller, and every one is the engine's. No cycle counter was
sampled: x86-64's runner exposed none, and AArch64's was not tried. Under perf the arm took 672 s on x86-64 and 1 131 s
on AArch64; on another AArch64 runner it took 1 118 s without perf.

**The derivation and the schedule.** The clock in the test splits the arm. Under perf, x86-64 took 123 s for the
derivation and 549 s for the 120 blocks, 4.4 to 4.8 s a block, and AArch64 took 206 s and 926 s, 7.6 to 7.8 s a block.
In every whole run, in every profile, the slowest of blocks 2 to 120 was within 0.2 s of the fastest. So a block of the
first 16 costs what a later block does.

## 2. The profiles

**The build.** Every test binary of the workspace, cold, each profile in turn on one runner, and the `inhibition`
unit's own seconds from cargo's report:

| Profile | x86-64 (EPYC 9V74), workspace | its `inhibition` | AArch64, workspace | its `inhibition` |
| :--- | ---: | ---: | ---: | ---: |
| `release` | 68.8 s | 39.2 s | 62.3 s | 48.4 s |
| `lto-fat` | 160.4 s | 73.9 s | 145.2 s | 70.3 s |
| `lto-thin` | 182.5 s | 75.4 s | 163.3 s | 70.8 s |
| `cgu1` | 60.8 s | 43.8 s | 61.6 s | 47.9 s |

Built alone with its dependencies, in the whole-arm jobs, one runner each: `inhibition` took 38.6, 60.7, 42.5 and 43.7 s
on x86-64 (EPYC 7763) in the four profiles in that order, and 41.4, 66.2, 42.4 and 51.4 s on AArch64.

**The runs.** The 16-block workload, three rounds on one runner. The whole arm is the derivation and 120 blocks at the
mean block, from the means over the rounds:

| x86-64, EPYC 9V74 | The run, rounds 1 to 3 | Derivation | A block | The whole arm | Beside `release` |
| :--- | :--- | ---: | ---: | ---: | ---: |
| `release` | 253.7, 253.0, 252.8 s | 158.7 s | 5.90 s | 866.9 s | |
| `lto-fat` | 239.1, 241.3, 245.0 s | 153.5 s | 5.52 s | 815.6 s | 0.941 |
| `lto-thin` | 248.0, 245.2, 245.9 s | 153.6 s | 5.80 s | 849.5 s | 0.980 |
| `cgu1` | 246.2, 251.7, 255.3 s | 159.5 s | 5.73 s | 846.4 s | 0.976 |

| AArch64, Neoverse-N2 | The run, rounds 1 to 3 | Derivation | A block | The whole arm | Beside `release` |
| :--- | :--- | ---: | ---: | ---: | ---: |
| `release` | 308.3, 313.2, 316.5 s | 193.1 s | 7.48 s | 1 090.1 s | |
| `lto-fat` | 304.6, 305.1, 299.2 s | 188.8 s | 7.14 s | 1 045.1 s | 0.959 |
| `lto-thin` | 313.4, 311.8, 308.9 s | 192.8 s | 7.41 s | 1 082.3 s | 0.993 |
| `cgu1` | 309.4, 318.1, 315.8 s | 196.2 s | 7.39 s | 1 083.0 s | 0.993 |

- **Fat LTO read below `release` in every round.** Each of its runs, beside the same round's `release`, read 0.942, 0.954
  and 0.969 on x86-64 and 0.988, 0.974 and 0.945 on AArch64. Its block gained more than its derivation: 0.935 against
  0.967 on x86-64, 0.955 against 0.978 on AArch64.
- **Thin LTO read below `release` in every round on x86-64 only**: 0.978, 0.969 and 0.973 there, against 1.017, 0.996
  and 0.976 on AArch64. `release`'s own three runs spread by 0.4 % on x86-64 and by 2.7 % on AArch64, where they grew
  slower each round.
- **`codegen-units = 1` read within its own spread on both**: 0.970, 0.995 and 1.010 on x86-64, where its three runs
  spread by 3.7 %, and 1.004, 1.016 and 0.998 on AArch64.
- **The whole arm once in each profile**, one runner each, agrees in direction. On x86-64 (EPYC 7763) it took 1 069.8 s
  in `release`, 1 037.6 s in `lto-fat`, 1 037.8 s in `lto-thin` and 1 020.1 s in `cgu1`. On AArch64 it took 1 118.0,
  1 045.8, 1 077.4 and 1 052.4 s. These are four runners per architecture, not one.

**Priced against the weekly run**, on two assumptions this note did not test: that `inhibition`'s other tests gain what
this arm did, and that the shards would build in the new profile.
- **The tests:** 4 to 6 % of `inhibition`'s 219 435 s is about 2.4 to 3.7 machine-hours a dispatch.
- **The builds:** 83 to 92 s more on each of the eighteen shards, 25 to 28 machine-minutes, and as much on each arm's
  job.

## 3. Every pin held

The whole control ran once in each profile on each architecture, and in `release` with line tables under perf on both.
Every run held it as `untold_arm` holds the control, `UntoldControl { moved: None, h42: true }`: its 120 blocks, its
trace and every reward's value are H-42's arm's bit for bit, and its derivation is H-42's pinned derivation. Every run
of the 16-block workload held the derivation whole and its 16 blocks to the first 16 pinned. No profile moved a pin.

## 4. The harness's heaviest functions

The harness held under 0.03 % of the test thread, so this note names none of it as a cost. The question asked for them
only where the harness held most of the time. Its samples were in `lead_in_window`, `run_on_scheduled_over`, the arm's
closure for each trial, `derive_under`, `Composer::observe`, `advance` and `weights_by_polarity`, one or two each.

## 5. The runners

The same arm in `release`, alone on its runner:

| Runner | The arm | How it was read |
| :--- | ---: | :--- |
| x86-64, EPYC 9V45 | 672 s | once, under perf |
| x86-64, EPYC 9V74 | 867 s | the derivation and 120 mean blocks of the 16-block runs |
| x86-64, EPYC 7763 | 1 070 s | once |
| AArch64, Neoverse-N2 | 1 078 to 1 118 s | once here, and once in brief 082's run 40226384795 |

The cost table's 1 941 s came from a weekly shard on x86-64. `scripts/exhaustive-shard.sh` runs half as many tests at
once as the runner has CPUs, two on four, and each test runs two threads that spin rather than sleep. On x86-64's two
cores of two threads, that is four busy threads on two cores. Which processor that shard had is not in its log.

## What it suggests

- **To make the arms cheaper is to make the turn cheaper, or to turn fewer units.** The harness has nothing to give.
  Either is a change to the engine and goes by an ADR and a measured round.
- **The workers wait for each other 7 to 24 % of the time.** Worker 0 carries phase 3's injector deliveries and the drive
  alone. A question: what the injector's messages delivered by the worker that owns each target would do to the wait,
  and to the delivery order the trace pins.
- **On AArch64 the atomics are calls, 12 % of the samples.** A question for a later exploration: the arms built for
  AArch64 with `-C target-feature=+lse`, which Neoverse-N2 has. It would tie the binary to Armv8.1 and later, which
  Latest ≠ Newest and a decision would weigh.
- **Fat LTO bought 4 to 6 % of this arm for 83 to 92 s of each build.** Thin LTO bought less for more. `codegen-units = 1`
  alone bought at most 2.4 %, within the spread, at no cost to the build. Whether any of them earns a profile for the
  weekly tests and the arms is the maintainers' to decide, by an ADR. Nothing here bears on the gate's own builds.
- **Larger than any profile: which runner a test gets, and how many tests share it.** The same arm took 672 to 1 118 s
  alone across four processors. The cost table's 1 941 s against 1 070 s alone on an EPYC 7763 suggests that a shard's
  two tests at once nearly halve each test's speed on x86-64's two cores, but this note did not run that comparison on
  one runner. A question for a later exploration: on one x86-64 runner, the shard's two tests at once against one at a
  time, read as each test's seconds and as the shard's throughput. It bears on ADR-0179's rule and F-82's bound, which
  are read from the shards' minutes.
