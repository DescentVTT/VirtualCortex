---
status: accepted
date: 2026-10-09
depends-on: ADR-0158
decision-makers: VirtualCortex maintainers
---

# ADR-0160: The sweep's tests under nextest, measured before anything is adopted — ADR-0158 read that 88 mutants of the runtime's 1 320 take 64 per cent of its sweep, 72 waiting for the test binaries `cargo test` runs last and 16 hanging to their bound; `cargo-nextest`, which the pinned `cargo-mutants` can run its tests under, is held to Latest ≠ Newest and measured on the runtime's area in two sweeps of one commit, kept only if no mutant's verdict is weaker and the area's machine time falls to three quarters or less; the rule written first; brief 065 runs it

## Context and Problem Statement

[ADR-0158](0158-the-sweep-off-the-next-decisions-path.md) read where the weekly sweep's time goes and named running its tests under `cargo-nextest` as the next step, to be decided by an ADR of its own. The maintainers chose on 2026-10-09 to measure it.

**The sweep of brief 063's dispatch**, written down here as ADR-0158 asks of the first ADR that merges after a sweep has ended: run [37872100407](https://github.com/DescentVTT/VirtualCortex/actions/runs/37872100407) ended at 06:03Z on 2026-10-09, two and three quarter hours after that round merged, green in all seven of the sweep's jobs.
- **3 818 mutants over the tree, 3 642 caught and none missed**, 154 unviable and 22 timeouts.
- **In the files [ADR-0155](0155-a-punishment-the-value-does-not-soften-built.md) changed**: `executor.rs` 416 caught, 19 unviable and 3 timeouts; `image.rs` 159 and 26; `task.rs` 177 and 10; `cortex-neuromod` 47 and 1; `cortex-connectome` 43 and 1. None missed.
- **The 22 timeouts** are in the places [ADR-0062](0062-the-first-complete-sweeps-list.md) reads as inherent: `cortex-core`'s three chain iterators, six; the injector, twelve; the barrier's wait, one; the workers' stop and run, three.
- **Its six shards over the runtime took 1 h 44 m to 4 h 08 m.**
- There is no survivor, so the round leaves no finding.

**What ADR-0158 read**, from run 37810004245: of the runtime's 30.2 hours of machine time, 72 mutants caught only after more than four minutes take 42 per cent, 67 of them first caught by `no_alloc`, `reference`, `store` or `sleep`, the test binaries `cargo test` runs last; and 16 hangs cut by the bound take 22 per cent.

**What was read of the tool**, on 2026-10-09:
- **The pinned `cargo-mutants`, 27.1.0, takes `--test-tool <cargo|nextest>`** (its own `--help`).
- **How it runs tests** (cargo-mutants' documentation, *nextest*): *"runs each test in a separate process, and it can run tests from multiple test targets in parallel"*. It fails fast by default, which that page calls *"highly preferable when testing mutants"*.
- **What that page warns of**: nextest *"currently allows straggling tests to run to completion"*, and *"Some trees, including cargo-mutants itself, are slower under nextest."* It recommends trying both.
- **What it does not run**: doctests. A behaviour only a doctest catches shows as missed. `runtime/cortex-runtime/src` holds no doctest today.
- **A bound for each test** (nextest's documentation, *slow tests*): `slow-timeout = { period, terminate-after }` terminates a test after that many periods, and a terminated test is a failure.
- **Its stability policy** (nextest's documentation, *stability*): the series is 0.9.x; the command line, the machine-readable output and the configuration format are its public API, append-only within the series; a change of behaviour is announced at least three months ahead.

**Why it may help here, and why it may not.** A mutant first caught by a late binary would be caught when that test fails, without the binaries before it, and a hang would be one failed test within its own bound. But a runner has four cores, the sweep runs two mutants at once, and this tree's suite is what that page describes: fast unit tests and slow integration tests. No reading of this tree under nextest exists.

## Decision Drivers

- **A measured need**: ADR-0158's table.
- **The maintainers' choice** (2026-10-09): measure it in a round of its own.
- **Latest ≠ Newest** (whitepaper §2.1): a tool is held to the same test as a dependency.
- **No verdict weaker.** The sweep exists to list survivors. A faster sweep that catches less is a loss.
- **A gain written first**, as [ADR-0099](0099-the-engines-speed.md) asked of the engine's speed levers.
- **The gate is not touched**: the pull request's checks and the commands of `CLAUDE.md` stay on `cargo test`.

## Considered Options

1. **What is done now**: (a) a measurement, and the tool kept only by a rule written first; (b) the tool adopted on the documentation's word; (c) nothing.
2. **Where**: (a) the runtime's area of the weekly sweep alone; (b) both areas; (c) the gate as well.
3. **The comparison**: (a) two sweeps of the runtime's area on one commit, one under each tool, compared mutant by mutant; (b) the new sweep against the last one on `main`.
4. **A hang**: (a) nextest's bound for each test, derived on the runner from the suite's slowest test; (b) the sweep's bound for a mutant alone, as now.

## Decision Outcome

**Options 1(a), 2(a), 3(a) and 4(a).**

### The tool against Latest ≠ Newest

| The test | What was read | Held by |
| :--- | :--- | :--- |
| a stable specification | the stability policy above: an append-only public API within 0.9.x | this ADR |
| two years of third-party production use | not verified here | **brief 065, with its sources, before anything else** |
| documented failure modes | no doctests; straggling tests run on; some trees are slower | this ADR |
| what it costs the tree | a pinned binary in one job of the weekly workflow; no dependency, no change to a manifest, no change to what a developer runs | this ADR |

A sub-1.0 tool passes the first row only because it states a stability policy (`CLAUDE.md`, principle 4). **If the second row cannot be shown, the round stops there.**

### The measurement (Specified; brief 065 builds it)

- **An input of the manual dispatch** chooses the tool the runtime's sweep runs its tests under, `cargo` unless given. The schedule and every dispatch that does not name it run as now.
- **nextest at an exact version**, installed in the sweep's job by a means whose integrity is checked.
- **A profile for the sweep** in nextest's configuration: fail fast; and a bound for each test, derived by the job from the unmutated suite's slowest test on that runner, as the bound for a mutant is derived from the suite ([ADR-0058](0058-the-weekly-sweep-and-its-timeouts.md)).
- **Before any mutant**: the unmutated suite passes under nextest, and it runs the tests `cargo test` runs, counted, with whatever it does not run named.
- **Two dispatches at `scope=mutants` on one commit** of the round's branch, one under each tool, with the same shards.
- **The comparison**, from each job's `outcomes.json`: every mutant's verdict under both, and its seconds building and testing under both.

### The rule for keeping it (written before any dispatch)

nextest is kept for the runtime's area **only if all three hold**:
1. **No verdict is weaker.** Every mutant caught under `cargo` is caught under nextest. A mutant that timed out under `cargo` is caught or times out. None is missed that was not.
2. **The area's machine time**, every mutant's seconds building and testing summed over the shards, **is at most three quarters** of what it was under `cargo` in the same pair.
3. **The unmutated suite passes under nextest in every shard**, so no test of the runtime needs to share a process with another.

- **Kept**: the runtime's sweep runs under nextest from then on, the state crates' stays on `cargo`, and the measurement's ADR says so with the pair's tables.
- **Not kept**: the input, the installation and the profile are removed, as [ADR-0100](0100-the-sweep-without-the-gate.md) removed a lever that did not pay, and the reading stays in the ADR.
- **Three quarters** is where the change pays for itself: a pinned tool more to keep, against a sweep that is no longer on any merge's path (ADR-0158).

**No prediction.** cargo-mutants' own page says a tree of this shape may be slower. The two effects this ADR expects, the late catches and the hangs, are read separately.

**Readings, no clause:**
- the 72 and the 16 of ADR-0158's table, by name where the mutants are the same, under each tool;
- the share of the area's time that is building, under each tool;
- each shard's wall time under each tool;
- every mutant whose verdict differs, with both.

### The round's stopping rule

1. **One round**: brief 065. No file under `src/` changes.
2. **It stops before any dispatch**, and is a finding, if two years of third-party production use cannot be shown, if the unmutated suite fails under nextest, or if nextest does not run a test `cargo test` runs other than a doctest.
3. **Kept or not kept** by the rule above, read once. A verdict that differs is listed and is not re-run until it agrees.
4. **The rule's three quarters and the profile's derivation do not move after the first dispatch.**

## Consequences

- Good: the tool is adopted on a reading of this tree, or not at all.
- Good: the gate, the commands and the state crates' sweep are untouched whichever way it reads.
- Good: the sweep of brief 063's dispatch is on record, with no survivor.
- Bad: two sweeps of the runtime's area, about sixty hours of machine time, for one comparison. The runners are not metered.
- Bad: one pair. Runners differ in speed within a run, by more than two to one in ADR-0153's evidence, so the machine time's ratio carries that noise. The rule's margin of a quarter is the room for it.
- Bad: if kept, the weekly workflow holds a second pinned tool.
- Neutral: a round with no engine question in it. The learning line's next round is brief 064's.

## Alternatives considered and why rejected

- **Option 1(b), adopted on the documentation's word**: the documentation says some trees are slower, and names this tree's shape.
- **Option 1(c), nothing**: 64 per cent of the area's time is 88 mutants.
- **Option 2(b), both areas**: the state crates' sweep is 56 minutes of machine time in one job.
- **Option 2(c), the gate**: the gate's commands are the ones a developer runs, and its tests are not what is slow.
- **Option 3(b), against the last sweep on `main`**: another commit and other runners. The pair on one commit removes the first.
- **Option 4(b), the mutant's bound alone**: the 16 hangs then wait out the whole bound as now, which is a third of what the tool could save.
- **A watchdog in the runtime's tests**: the hangs are in every test that ticks an executor, not in the injector's own, so the watchdog would wrap the suite.
- **Test binaries renamed so that the late catchers run first**: an order held by file names, which the next test file breaks.

## Confirmation

`briefs/065_the-sweeps-tests-under-nextest.md` runs the measurement. Whitepaper 4.100.0 carries §9's rows for this ADR and ADR-0159, and F-67. `npm run spec` holds the links, the rows, the brief's sections and the version.
