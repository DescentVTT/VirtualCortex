---
status: accepted
date: 2026-09-12
decision-makers: VirtualCortex maintainers
depends-on: ADR-0023
---

# ADR-0035: Multirate stepping as a cadence — a power-of-two period and a phase, a mask on the tick; the population spike tally; the idle-tick benchmark; conservative lookahead left to a measurement

## Context and Problem Statement

The proposal of brief 018 named a multirate frontier: "hierarchical timing wheels and epoch dividers" to step slow rules (neuromodulatory, metabolic, sleep) at coarse multiples of the 10 µs tick, and "conservative parallel discrete-event simulation with axonal lookahead" so that regional thread pools step ahead of one another within the minimum axonal delay before a rendezvous. It stated as a baseline finding that fast and slow nodes "synchronize at every single tick, inducing severe barrier skew, core idling, and atomic bus contention".

The tree contradicts the finding and lacks two things the frontier needs. The executor (ADR-0023) runs units and synapse blocks and nothing else: no homeostatic, thalamic, immune or metabolic rule is composed, so no slow node synchronises at any tick. The cost of the four barrier waits per tick on many workers is unmeasured; `docs/benchmarks/README.md` lists "the loop across several workers (the barriers' cost with contention)" as not measured, and ADR-0023's consequences already name the waits as "the dominant cost of an idle tick and a T-3 measurement subject". And nothing in the executor counts how many units fired in a tick, which every population-level rule (the branching ratio of ADR-0036 first) needs. What is the form of a slow rule's schedule, what does the executor publish for population rules, and what decides the lookahead question?

## Decision Drivers

- §8.3: a run is `(image, seed, trace)` and bit-identical on any worker count; a slow rule's schedule must be a function of the tick, not of a thread's progress.
- TC-5 and ADR-0023: no allocation, no blocking but the barrier, no new phase and no new barrier for a rule that runs once in thousands of ticks.
- ADR-0013's two reasons for power-of-two rings: slot selection is a mask, and the wrap of the clock is exact.
- ADR-0033: the loader resumes the writer's clock, so a schedule on the tick continues where an image was written.
- ADR-0010: a change to the executor's synchronisation is made from a measurement on the reference platform, not from a premise.
- "Latest ≠ Newest": conservative lookahead (Chandy and Misra 1979; Bryant 1977) and the minimum-delay communication interval of NEST (Morrison et al. 2005) are decades old with documented failure modes; they are admissible when needed, and needed only when measured.

## Considered Options

1. A hierarchical timing wheel for slow rules: each rule an event re-scheduled at its next due tick.
2. **A cadence: a period of $2^k$ ticks and a phase below it; a rule is due at the ticks whose low $k$ bits are the phase. The executor's coordinator runs slow rules between ticks on their cadence; workers publish per-tick counts at the end of a phase and the coordinator sums them. An idle-tick benchmark on one, two and four workers. Conservative lookahead as an open question decided by that benchmark on the reference platform.**
3. Conservative lookahead now: units assigned to workers, each worker stepping its region for the minimum delay between rendezvous.

## Decision Outcome

Option 2.

- **`Cadence`** (`cortex-core`, `dispatch/cadence.rs`). `Cadence::new(period_shift, phase)` is `None` for a shift of 64 or more (the clock's width) and for a phase at or beyond the period; `is_due(tick)` is `tick & (period − 1) == phase`; `period()` and `phase()` read back. A power of two so that the test is a mask and $2^{64}$ is a multiple of every period: the wrap of the `u64` clock is exact, as it is for the wheel's rings (ADR-0013). A cadence is a function of the tick alone, so a rule stepped on one runs at the same ticks on every worker count and after a reload (ADR-0033), and it adds no barrier: multirate stepping is a *schedule*, not a scheduler. Tests at both bounds, across the wrap, and a property walk (every tick is due on exactly one phase of a period).
- **Where a slow rule runs.** Between ticks, on the coordinator, after the tick's last barrier and before the next tick's first: the place the modulator's decay (ADR-0032) and the clock sweep (ADR-0024) already run. A rule that must touch every unit runs inside phase 1 by the turn holder, as the gain of ADR-0036 does; a rule that must touch every block runs inside phase 2 by the chain's owner. No phase is added.
- **The population tally.** At the end of the turns phase each worker stores the number of units it fired into its slot of `Shared::spikes`; after the tick the coordinator sums the slots. A sum of integers is the same in any order, so the count is the same on any worker count, which the exit test of ADR-0036 holds on one and four workers. The count feeds `HomeostaticDrivePool::count_activity` (ADR-0036); it is the tally that whitepaper §5.2.16 and R-12 called Specified.
- **The benchmark.** `executor/idle_tick/{1,2,4}` in `benches/cortex-bench`: an executor of eight units at rest, one `tick` per iteration, so the four barrier waits, the coordinator's publication of the modulation and the gain and the tally are all it measures. A developer-machine figure is recorded in brief 018's report as not admissible (ADR-0010); the admissible run is the protocol of `docs/benchmarks/README.md` on the reference platform at its worker count.
- **Conservative lookahead is not adopted, and not rejected.** Its precondition is a static assignment of units to workers, which the work-stealing deque of ADR-0023 does not have: a unit is run by whichever worker pops it, so no worker owns a region it could step ahead in. Its benefit is one rendezvous per minimum delay instead of four barrier waits per tick. Whitepaper §11.1 carries the question with the decision rule: the idle tick at the reference platform's worker count, from an admissible run, against the tick's budget; if the waits are a material fraction of the 10 µs, the next executor round decides ownership and the lookahead together as a new decision on the executor's ownership model, and a connectome would then carry its minimum delay in the header.
- **What is not built.** No hierarchical wheel for slow rules: a periodic rule needs no data structure, only a mask on the tick; the wheel exists for events (deliveries), whose times are data. No barrier is removed: merging the tick's last wait with the next tick's first is a candidate for the same executor round, after the measurement.

### Consequences

- Good: a slow rule's schedule is one type, tested once, with no barrier and no allocation; the tally is one atomic store per worker per tick and one sum per tick.
- Good: the question the proposal raised has its measurement in the tree, and a rule for what the number decides.
- Bad: the benchmark's developer-machine figure is not a Measured number and the whitepaper's T-3 column stays empty (finding F-13 stands).
- Bad: a cadence's period is a power of two; a rule that wants every 1 000 ticks takes 1 024.

## Alternatives considered and why rejected

- **Option 1** re-schedules every slow rule as a wheel event, with a token per rule and a drain per tick, to express what a mask expresses in one instruction.
- **Option 3** builds a static ownership of units over the work-stealing model of ADR-0023 on a premise the tree does not support and a number nobody has measured.
- **A cadence with an arbitrary period** (`tick % period`): a division per check and a schedule that breaks at the clock's wrap unless the period divides $2^{64}$.
- **The tally by a shared atomic increment per spike**: a contended cache line on the hot path, for a count that a per-worker store and one sum give without contention.

## Confirmation

`cortex-core`: three unit tests and the property walk of `cadence.rs`; `npx spec-guard` asserts `Cadence` exists (§5.2.1). `cortex-runtime`: the exit test of ADR-0036 (`tests/criticality.rs`) holds that the tally equals the units that fired in every bin, that the bin and the window close on their cadences (one tick short of the boundary nothing has moved; at it the gain has), and that the loop is bit-identical on one and four workers. `cargo bench -p cortex-bench --bench hot_path -- --test` executes `executor/idle_tick` on one, two and four workers in CI.
