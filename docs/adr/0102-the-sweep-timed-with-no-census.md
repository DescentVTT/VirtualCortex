---
status: accepted
date: 2026-09-26
depends-on: ADR-0101
amends: [ADR-0023, ADR-0017]
decision-makers: VirtualCortex maintainers
---

# ADR-0102: The sweep timed with no census, and kept — ADR-0100's sweep without the gate, re-applied as built by reverting its revert and timed on ADR-0101's workload, read 0.517, 0.527, 0.745 and 0.643 of the base's wall time per tick at ADR-0097's runs (a) to (d), inside every bound of ADR-0099, with every reading of behaviour held bit for bit; the census workload read again beside it at 1.156 at (c); the instrument, the re-application, the pins, the mutation gate and the protocol written before the first timed run; axiom A3 enforced by ownership, and ADR-0017's and ADR-0023's decisions amended

## Context and Problem Statement

[ADR-0100](0100-the-sweep-without-the-gate.md) built the sweep without the gate: each worker owns a fixed, contiguous range of the unit arena and serves in unit order the units of it a schedule bitmap names, with no deque, no stealing and no compare-and-swap in the turn. It held every reading of behaviour bit for bit and was not kept, because [ADR-0099](0099-the-engines-speed.md)'s workload read 1.178 of the base's wall time per tick at [ADR-0097](0097-the-active-set-measured.md)'s run (c) against a bound of 1.10. That workload's harness reads every unit's gate byte on worker 0 before each timed tick (F-50). [ADR-0101](0101-the-sweep-measured-again.md) took ADR-0100's second named option: the same sweep, timed on ADR-0097's four configurations with no census in the timed ticks, against the same bounds, and it says that this workload was chosen after the first reading failed. Brief 044 is the round that applies it.

This record says how the census-free workload is built, how the sweep is re-applied and what in it changed, which pins it is held to, what the mutation gate read on its lines before any run was timed, and how the session is run and judged. All of that was committed before the first timed run (`b839388` and `86c2b0b` on the round's branch). The readings and the verdict follow.

## Decision Drivers

- **ADR-0101's workload rule**: no test-side read of any unit's record between two timed ticks; every row a timed run produces held to ADR-0097's pinned tables, from the counts the engine keeps; ADR-0097's four weekly tests keep their census as tests of behaviour.
- **ADR-0099's acceptance, unchanged**: every reading of behaviour bit for bit; a pin that also holds the scheduler's own bytes restated only under the masked check; any other pin that moves stops the round.
- **ADR-0099's bounds, unchanged**: kept at a median of at most 0.80 of the base's wall time per tick at (a) and (d), at most 1.10 at (b) and (c).
- **The sweep is ADR-0100's as built** (`ff6a297` on `main`). It may change only where `main` has moved under it, or where the mutation gate on its diff requires a change that moves neither behaviour nor a turn's work, and every such change is committed before the first timed run.
- **A disturbed session is discarded unread**, as ADR-0100's first was. What counts as disturbed must be written before the session, not judged from its readings.

## Considered Options

For the census-free path (the brief leaves its form to the round):

1. **A parameter of `read`**: `Census::Held`, ADR-0097's census as it was, and `Census::Off`, which reads no unit's record between ticks; the timed runs in tests of their own that call `read` with `Off` and hold their rows to the same tables.
2. A timing-only copy of `read` without the census: two loops that must be kept alike by review.
3. An environment variable that switches the census off in ADR-0097's own tests: the tests of behaviour would then depend on the environment, and the census could be switched off where it is the assertion.

For the one-worker diagnostic: tests of their own on the committed tree, or copies of the two trees edited for the session as ADR-0100 did.

For the controls: in the timed session, where ADR-0100's tests made them, or out.

## Decision Outcome

**Option 1; the one-worker diagnostic as tests of its own; the controls in the session, read beside the criterion and not a criterion; one ADR. The criterion read every bound met, and the sweep is kept.**

### The instrument (`ad32761` on the round's branch)

- `tests/active.rs`'s `read` takes a `Census`. Under `Census::Held` it is ADR-0097's reading, unchanged: before each tick every unit's gate byte, and the tick's turns held to the units it found scheduled. Under `Census::Off`, between two timed ticks it reads the clock, calls the drive's injection (the engine's input, part of the workload in both), and reads the turns and messages each worker counted (`Executor::turns`, `Executor::delivered`, one atomic a worker each); the spike train is read at a row's end. No unit's record is read.
- `timed(k, workers)` runs ADR-0097's run `k` and, at 1 024 units, its control with `Census::Off`, dumps each and holds it to `NETWORK_ROWS` and `CONTROL_ROWS`. Six weekly `exhaustive` tests: (a) to (d) on ADR-0097's two workers (the criterion), and (a) and (c) on one worker (the diagnostic). The rows are sums the engine keeps over its workers, so one table holds on any worker count; all six passed on the base.
- ADR-0097's four tests keep their census and their tables. The gate's test also reads the first 512 ticks of (a) with `Census::Off` on one and on two workers and holds them to the table's first rows, so the path is in the gate with no test added to it.

### The sweep re-applied (`a46cf33` on the round's branch)

`git revert 5aa57c5`. Under `runtime/`, `crates/` and `.cargo/`, `main` had not moved since `5aa57c5`, so the revert restores `ff6a297`'s tree there byte for byte, except `tests/active.rs`, where the instrument's commit had changed the lines the revert touched. The one conflict is resolved by keeping both: ADR-0100's `shares` dump of the turns each worker served stays in ADR-0097's four census tests, where ADR-0100 put it, after each run's timed ticks. The whitepaper's directive on `.cargo/mutants.toml` returns to ADR-0100's absence of the steal's exclusions. **No line of the sweep's code is changed.**

The mutation gate required none (below), so the code the session times is `ff6a297`'s.

### The pins

As ADR-0100 listed them, read on `a46cf33` before any timed run:

- `PINNED_ARENA_HASH` `0x6c27858ece2dd412` and `PINNED_SPIKE_COUNT` 95 (`tests/differential.rs`), with the scheduler's bytes masked and not: the test asserts both. No pin is restated.
- The differential test on one, two and four workers; the equalities across runs of `tests/criticality.rs`, `tests/image.rs` and `tests/reference.rs`; the trial forks' hashes of `tests/amendment.rs`: every test of the workspace in debug and release, 635 passed, and under the MSRV.
- ADR-0097's four tables, with the census on every tick, and the six timed tests' rows without it, on two workers and on one: the binary's ten ignored tests, run once in release. Their wall times were not read; they are not a session.
- The whole-image pins (`REVERSAL_IMAGE_CRC_1024`, `REVERSAL_IMAGE_CRC_FORMAT_15_1024`, `PUNISHED_IMAGE_CRC_1024`) and every other weekly pin are the weekly dispatch's to read.

### The mutation gate, read before the first timed run

The pull request's gate, run `36208294392` at `b839388` (the sweep's code as `a46cf33` holds it), on the lines the pull request changes: **50 mutants, 49 caught, 1 unviable, none missed, no timeout**, in 15 minutes after a baseline of 441 s. The unviable one is `replace >= with < in Shared::owner`: `unit as usize < self.units.len()` parses the `<` as the start of generic arguments, so it does not compile (the same on this machine, `cargo mutants --check`). The bound it would invert is held by the partition test's `exec.owner(units) == None`, and its other mutants (`Shared::owner` replaced by `None`, `Some(0)`, `Some(1)`) are caught. With no survivor there is nothing to meet, and the sweep's code is not changed.

### The measure, fixed before the first timed run

- *Builds.* The base is the instrument's commit (`ad32761` on the branch): `main` at the round's start (`5a1e3c9`) with the census-free path, and no sweep. The change is the base with the sweep re-applied, at the commit whose code the mutation gate read. Each is exported with `git archive` into a directory of its own and built with `cargo test -p cortex-runtime --release --locked --test active --no-run` into a target directory of its own.
- *Runs.* Each test alone: `<binary> --ignored --exact <test> --nocapture --test-threads=1`. The reading is the `ns_per_tick` of the run's network line; the control's line is read beside it.
- *Three parts, in this order, in one session:*
  1. **The criterion**: the four tests `timed(k, 2)`, (a) to (d).
  2. **Beside it, ADR-0099's census workload**: ADR-0097's four tests, as ADR-0100's session ran them.
  3. **Beside it, one worker**: the two tests `timed(0, 1)` and `timed(2, 1)`.

  In each part, five pairs alternate the builds, the base first in pairs 1, 3 and 5; within each build's turn the part's tests run in order. Each run's figure is the median of its five readings. Every run's whole output is kept.
- *The idle machine.* The session starts only after six consecutive five-second samples of the processor total are below 15 per cent with none of the round's processes running. Through the session a sampler records, every five seconds, the processor total and, from each process's processor time, the logical processors held by the round's two test binaries and by every other process, with the three heaviest others named.
- *Disturbed.* A part is disturbed if, in three consecutive samples within it (fifteen seconds), the processes outside the round's two binaries held more than four logical processors, a quarter of the machine. A disturbed part is discarded unread and says why. If the criterion's part is disturbed, the whole session is discarded unread and a new one is started after the idle check; the parts beside it, disturbed, are run again alone after the idle check. The sampler's log is judged before any reading of the part is looked at.
- *The criterion*, ADR-0099's: the change is **kept** if the ratio of its median to the base's is at most **0.80** at (a) and at (d) and at most **1.10** at (b) and at (c) in part 1. Otherwise it is not kept. Parts 2 and 3 decide nothing.
- The figures are a developer machine's, recorded in `docs/benchmarks/results/` as `admissible: no`, and never written into §10.2. None of this moves after a timed run.

### The readings

The session ran from 2026-09-26T01:44:30Z to 02:09:32Z on the developer machine of ADR-0097's and ADR-0100's readings (AMD Ryzen 7 7735HS, eight cores, sixteen threads, Windows 11, unpinned; not admissible). The idle check passed at 01:44:17Z after six samples at 8.1 to 14.4 per cent. Through the three parts, 302 samples: the total averaged 17 to 24 per cent a part, two samples were at or above 40 per cent (62.5 and 40.2); processes outside the round's binaries held 1.20 to 1.22 logical processors on average and 1.58 at most. The heaviest of them, in every sample, was a test worker of another repository's session, at about one. **No part was disturbed**, and the rule was applied to the log before any reading was looked at. Every run of both builds passed its pinned table. All figures are in `docs/benchmarks/results/2026-09-26-dancr-win11-brief-044.md`, a developer machine's, not admissible.

**Part 1, the criterion** (ADR-0101's workload, two workers), wall time per tick in nanoseconds, medians of five:

| Run | Base | Change | Change/base | Bound | |
| :--- | ---: | ---: | ---: | ---: | :--- |
| (a) 1 024 units, ADR-0044's drive | 10 939 | 5 651 | **0.517** | 0.80 | met |
| (b) sixteen times sparser | 7 037 | 3 707 | **0.527** | 1.10 | met |
| (c) 256 times sparser | 1 077 | 802 | **0.745** | 1.10 | met |
| (d) 4 096 units, ADR-0044's drive | 46 810 | 30 078 | **0.643** | 0.80 | met |

The controls beside them, not a criterion: 0.602, 0.565 and 0.716 at (a) to (c). The base's readings at (a) are bimodal: pairs 1 and 3 read 21 340 and 22 889 against 10 926 to 10 939, and the controls at (a) and one reading at (b) do the same. The change's are not. The median reads the lower mode, the conservative choice for the change; the mechanism is not measured.

**Part 2, ADR-0099's census workload beside it**, not a criterion: 0.465, 0.435, **1.156** and 0.524 at (a) to (d), controls 0.340, 0.520 and 1.109. ADR-0100's reading of that workload repeats: 1.156 at (c) against its 1.178. The census has a cost of its own, and it differs between the builds: at (a) it adds about 3 200 ns a tick to the base and 900 to the change; at (c) about 180 to the base and 650 to the change. The change's turns per worker are its partition's in every pair, the figures ADR-0100 recorded.

**Part 3, one worker beside it**, not a criterion: 0.521 at (a) and 0.649 at (c), controls 0.586 and 0.643, where ADR-0100's one-worker diagnostic, with the census, read 0.516 and 0.657.

**The per-turn breakdown** after the sweep, from part 3 and the bench on the change's code (after the same idle check; not admissible):

- On one worker the sweep's turn at (a) is 11.4 ns, and `neuron/integrate` is 10.96 ns: the integration is the turn. The base's is 21.9 ns, the difference about `gate/schedule_begin_end`'s 9.76 ns and the deque's push and pop.
- On two workers the runs' own figure is 11, 10 and 14 ns a turn at (a), (b) and (d), against the base's 21, 19 and 22.
- At (c) about 75 units are served a tick. `executor/idle_tick/2`, the barriers and coordinator of a tick with nothing to serve, is 337 ns of the change's 802: about two fifths of the tick is synchronisation.

### The verdict

**Kept.** Every bound is met in part 1, with room: 0.517 and 0.643 against 0.80, 0.527 and 0.745 against 1.10. No reading of behaviour moved and no pin was restated. Parts 2 and 3 decide nothing: they repeat ADR-0100's readings of its own workload and of one worker.

What lands is ADR-0100's design as built (`ff6a297`), unchanged:

- **Ownership.** Worker $w$ owns the units $[ws, \min((w+1)s, N))$, $s = \lceil N/W \rceil$, fixed in `Executor::new`; `Executor::owner` names a unit's owner.
- **The schedule.** One bit per unit, each owner's region padded to a cache line. Phase 1 serves the region's set bits in unit order and keeps the bit of a unit its turn leaves awake. A push or an activation that finds a unit idle stores its gate byte scheduled and sets its bit with a fetch-or. No compare-and-swap is left in the turn, and a unit with no mail takes no swap.
- **The gate byte** is the schedule's record, scheduled between ticks exactly when the next tick serves the unit, never running, written by `cortex-core`'s `set_gate`. `is_image_ready`, the clock sweep, the image writer, ADR-0097's census and the differential test's snapshot read it as before.
- **Removed:** `deque.rs` and its four tests, `Config::deque_capacity`, `Worker::steal`, and the two exclusions of `.cargo/mutants.toml` that named the steal.
- **Axiom A3**, whitepaper §4: "At most one worker writes a record's plain fields in any tick, enforced by ownership: each worker owns a fixed range of the unit arena and alone runs its units' turns; the barriers keep every other worker's access to the record — a push onto its mailbox, a mark of its gate byte, a read of its spike stamp — out of the phase in which the owner writes it." It is held by `a_unit_is_served_by_its_owner_alone_and_a_woken_unit_at_the_next_tick` (`tests/contention.rs`), by `the_ranges_partition_the_arena_and_the_places_partition_the_schedule` (`executor.rs`), and by the differential test on one, two and four workers.
- **Phase 1's `unsafe`** ([ADR-0023](0023-executor.md)'s one), as `executor.rs` states it: "`unit` is a set bit of this worker's region of the schedule, and so in this worker's range, which no other worker's range overlaps: the ranges partition the arena and are fixed in `Executor::new`. In phase 1 a worker references only its own range's units, one at a time, and no push happens; phases 2 and 3 have ended at the barrier." ADR-0023's invariant, that a `&mut` to a record never overlaps another reference to it, is unchanged; what upholds it in phase 1 is now the partition, not the claim.

**The decisions amended:**

- **[ADR-0023](0023-executor.md)**: "in-house work-stealing deques" is replaced by the ownership and the schedule above. The three barrier-separated phases, the one `unsafe` and its invariant, and the fixed pool stand.
- **[ADR-0017](0017-mailbox-and-gate-protocol.md)**: the mailbox is unchanged — the index stack drained whole, no ABA tag, the push's compare-exchange. The gate no longer claims a turn in the executor. `try_schedule`, `begin_turn` and `end_turn` stay in `cortex-core` with their tests and their bench case, for a scheduler without ownership. The executor records its schedule with `set_gate`, and the four sequentially consistent operations that close the lost-wakeup window are not on its path: no push overlaps a turn, as ADR-0023 already said, and the barriers order every phase's writes before the next phase's reads.

**The weekly dispatch's scope** ([ADR-0075](0075-the-dispatch-scope-follows-the-diff.md)): `scope=both`. The diff changes files under `src/` (`executor.rs`, `lib.rs`, `neuron.rs`, `deque.rs` deleted), deletes tests from the swept suite (the deque's four), and changes `.cargo/mutants.toml`. Any one of the three would give it.

**What the working layout's round should weigh** (ADR-0099's order: the working layout second, the lookahead third):

- On the sweep the integration is the whole turn, 11 ns of 11 on one worker at (a). A layout of the integrated fields for the vector units is aimed at all of what is left of a turn. Its gain is bounded by how many units' integrations one vector step can do at once. Its cost is quality goal 2: one unit, one cache line.
- At the sparsest drive, synchronisation is about two fifths of a tick on two workers. That is the lookahead's measured need, and it grows with the worker count (`executor/idle_tick/4` 1 234 ns).
- The census reading (1.156 at (c)) shows that under ownership a tick pays for any thread that pulled the owners' lines between ticks. The engine's own between-tick readers of every record — the clock sweep, the image writer and the instrument's `is_quiescent` — do not run between the ticks the learning line times. A round that adds one should read its cost.

### Consequences

- Good: the engine is faster at every run the tree times, on this machine: 1.9 times at (a) and (b), 1.3 at (c), 1.6 at (d). A turn on one worker costs its integration and little else.
- Good: axiom A3 is held by a partition fixed at construction and tested over a grid, where a compare-and-swap claim took each turn at run time. Every reading of behaviour held with no pin restated.
- Good: the working layout and the lookahead have the ownership and the contiguous ranges they need.
- Bad: the verdict rests on a workload chosen after ADR-0100's reading failed. ADR-0101 says so. The old workload, read again, still fails at (c), and that reading is published beside this one.
- Bad: the ranges are fixed. A worker whose range is idle waits at the barrier, and nothing moves a clustered activity's work to it. ADR-0097's drives draw units uniformly, so the shares are even: 24.85 and 24.61 million turns at (c). A network whose activity clusters in one range has not been read.
- Neutral: the gate byte and ADR-0017's claim protocol stay in `cortex-core`; the executor writes the byte and does not claim with it.

## Alternatives considered and why rejected

- **A timing-only copy of `read`**: two loops kept alike by review, where one loop with a parameter is held by the gate's test on both paths.
- **An environment variable**: it would let the census, the assertion of ADR-0097's tests, be switched off where it is a test of behaviour.
- **The one-worker diagnostic on edited copies of the trees**, as ADR-0100 ran it: a reading nobody can make again from the tree. As tests, it is a committed command, and the weekly job holds its rows.
- **The controls out of the session**: the census workload beside the criterion would then not be the workload ADR-0100 read, and the controls are the drive's active set alone, a reading of the sweep's cost with no network in it.

## Confirmation

- `ad32761` (on the round's branch): the instrument, `Census` and the six timed tests; all ten ignored tests of `tests/active.rs` passed on it.
- `a46cf33`: the sweep re-applied, `git revert 5aa57c5`, with `a_unit_is_served_by_its_owner_alone_and_a_woken_unit_at_the_next_tick`, `the_ranges_partition_the_arena_and_the_places_partition_the_schedule` and `set_gate_records_each_state_without_a_claim`; every test of the workspace in debug, release and under the MSRV, the determinism pin masked and not, and ADR-0097's tables with and without the census held on it.
- `b839388` and `86c2b0b`: this record as it stood before the first timed run, the measure and the disturbance rule, and the mutation gate's reading (run `36208294392`).
- `docs/benchmarks/results/2026-09-26-dancr-win11-brief-044.md`: every reading above, the load through the session, the bench, and the scripts.
- The evidence: the weekly dispatched on the round's branch at `86c2b0b`, whose code is the kept code, run `36209381108` with `scope=both` (the clause above), green in every job it runs:
  - **The whole-domain tests**: the four shards took 27m20s, 41m37s, 34m59s and 30m14s, their tests' wall 1 592, 2 446, 2 049 and 1 759 s, 22, 34, 28 and 24 per cent of the 7 200-second bound. All 63 tests passed, the six timed tests among them, so every pinned number of those tests is reproduced on the sweep: among them the whole-image CRCs of `tests/inhibition.rs`, every table of the learning line, and ADR-0097's tables with and without the census.
  - **The mutation sweep of the whole tree**, which reads the sweep's lines with the rest: 3 577 mutants, 3 411 caught, **none missed**, 25 timeouts and 141 unviable (ADR-0097's sweep of `36050252444`, on the deque: 3 404, 0, 25 and 142). The 25 timeouts are the same mutants as that sweep's, by file and name, ADR-0062's list; none is in a line of the sweep. There is no survivor to disposition. The crates took 17m12s and the six runtime shards 1h46m50s to 2h33m59s.
  - **Beside ADR-0100's run `36188866157`**, the secondary reading: its shards took 44m42s, 50m04s, 1h19m35s and 58m44s. The tests that took a minute or more there took 0.59 of their seconds here at the median (41 tests, 0.27 to 1.18). ADR-0100 read the same tests move by 0.55 to 1.99 between two runs of identical code, so this is consistent with a faster engine and is not a measure of it.
  - **The cost table** is regenerated from this run: 63 lines, 15 263 seconds, against the previous table's 57 lines and 26 945.
