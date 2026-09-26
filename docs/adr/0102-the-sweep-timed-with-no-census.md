---
status: proposed
date: 2026-09-26
depends-on: ADR-0101
decision-makers: VirtualCortex maintainers
---

# ADR-0102: The sweep timed with no census — ADR-0100's sweep without the gate, re-applied as built by reverting its revert, timed on ADR-0101's workload against ADR-0099's bounds unchanged; the instrument, the re-application, the pins, the mutation gate and the protocol written before the first timed run

## Context and Problem Statement

[ADR-0100](0100-the-sweep-without-the-gate.md) built the sweep without the gate: each worker owns a fixed, contiguous range of the unit arena and serves in unit order the units of it a schedule bitmap names, with no deque, no stealing and no compare-and-swap in the turn. It held every reading of behaviour bit for bit and was not kept, because [ADR-0099](0099-the-engines-speed.md)'s workload read 1.178 of the base's wall time per tick at [ADR-0097](0097-the-active-set-measured.md)'s run (c) against a bound of 1.10. That workload's harness reads every unit's gate byte on worker 0 before each timed tick (F-50). [ADR-0101](0101-the-sweep-measured-again.md) took ADR-0100's second named option: the same sweep, timed on ADR-0097's four configurations with no census in the timed ticks, against the same bounds, and it says that this workload was chosen after the first reading failed. Brief 044 is the round that applies it.

This record says how the census-free workload is built, how the sweep is re-applied and what in it changed, which pins it is held to, what the mutation gate read on its lines before any run was timed, and how the session is run and judged — all before the first timed run — and then the readings and the verdict.

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

**Option 1; the one-worker diagnostic as tests of its own; the controls in the session, read beside the criterion and not a criterion; one ADR.**

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

To be completed by the round after the session.

### The verdict

To be completed by the round.

### Consequences

To be completed by the round.

## Alternatives considered and why rejected

- **A timing-only copy of `read`**: two loops kept alike by review, where one loop with a parameter is held by the gate's test on both paths.
- **An environment variable**: it would let the census, the assertion of ADR-0097's tests, be switched off where it is a test of behaviour.
- **The one-worker diagnostic on edited copies of the trees**, as ADR-0100 ran it: a reading nobody can make again from the tree. As tests, it is a committed command, and the weekly job holds its rows.
- **The controls out of the session**: the census workload beside the criterion would then not be the workload ADR-0100 read, and the controls are the drive's active set alone, a reading of the sweep's cost with no network in it.

## Confirmation

To be completed by the round.
