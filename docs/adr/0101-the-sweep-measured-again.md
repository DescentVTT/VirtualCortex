---
status: accepted
date: 2026-09-26
depends-on: ADR-0100
amends: [ADR-0099]
decision-makers: VirtualCortex maintainers
---

# ADR-0101: The sweep measured again — of ADR-0100's named options the second is taken: the sweep without the gate, as ADR-0100 built it, timed again on ADR-0097's four runs with no census in the timed ticks, against ADR-0099's bounds unchanged; the workload is chosen after the first reading failed, and this record says so; the census stays in the tests of behaviour; a lever's gain is read on a workload whose instrument reads no record the engine does not; F-50 resolved; brief 044

## Context and Problem Statement

[ADR-0100](0100-the-sweep-without-the-gate.md) built the sweep without the gate and did not keep it. It held every reading of behaviour bit for bit. It read 0.406, 0.369, 1.178 and 0.488 of the base's wall time per tick at [ADR-0097](0097-the-active-set-measured.md)'s runs (a) to (d), and (c) is above [ADR-0099](0099-the-engines-speed.md)'s bound of 1.10. Two diagnostics beside the criterion read the sweep faster at (c) too: 0.657 on one worker, and 0.748 on two with the workload's per-tick census taken out.

The census is `read` in `runtime/cortex-runtime/tests/active.rs`. Before every timed tick it reads the gate byte of every unit on the coordinator's thread, worker 0, and holds the tick's turns to that count. The census is outside the timed call but not outside its cache. The whitepaper records this as F-50: "The measure reads the instrument beside the engine, and no rule of the engine reads every record between ticks." ADR-0100 named five options and took none.

**This ADR's author chose that workload.** ADR-0099 took ADR-0097's runs because they print their wall time. It did not read what `read` does between the ticks it times. The flaw was in the measure ADR-0099 fixed, not in the round that applied it.

## Decision Drivers

- **The engine's workload is what a gain is for.** `.gate()` is called by `tests/active.rs` and `tests/differential.rs` and by no file under `src/`. Nothing `Executor::tick` runs reads every unit's record between ticks; the clock sweep and the image writer read many, and neither runs in a tick. One harness step does: the instrument's `quiet` (`tests/instrument/harness.rs`) calls `Executor::is_quiescent`, which reads every unit's mailbox head, between the ticks of the quiet run before an image is written. That is a phase of preparing an image, not of the trials the learning line runs. A tick timed after a census of every record is not a tick the engine takes in the runs the line is for.
- **Pre-registration.** A workload chosen after its first reading failed is the move pre-registration exists to prevent. What makes it admissible here is written down:
  - the reason it names is independent of the verdict it wants, since the census is not the engine's work;
  - the bounds do not move;
  - the old workload is read again beside the new one and published;
  - this record states the order in which the choice was made.
- **ADR-0065's precedent.** The instrument was recalibrated after H-12 read chance, and the question was asked again through it, the recalibration stated as such.
- **ADR-0099's acceptance stands**: every reading of behaviour bit for bit, a pin that holds the scheduler's own bytes restated only under the masked check, and any other moved pin a stop.
- **The per-turn breakdown** ADR-0100 read: on one worker the sweep's turn at (a) is about 10.7 ns against the base's 20.8, which is the integration alone. The working layout's need depends on the sweep being in the tree.

## Considered Options

ADR-0100's five:

1. The working layout without the sweep.
2. **The sweep again under a workload without the census in the timed ticks.**
3. The sweep with a balance for the sparse case: a lock-free structure the tree does not have.
4. The lookahead next.
5. The speed line closed and the learning line resumed.

## Decision Outcome

**Option 2.**

- **The workload, amending ADR-0099's.** ADR-0097's four configurations are unchanged: the prior at seed 22, the instrument's configuration at the gain 1.75, two workers, (a) 1 024 units under ADR-0044's drive, (b) sixteen times sparser, (c) 256 times sparser, (d) 4 096 units, and the lead-in and four windows of $2^{17}$ ticks. **The timed ticks carry no census**: no test-side read of any unit's record between two timed ticks. Every row a timed run produces is still held to ADR-0097's pinned tables (the turns, the messages delivered and sent, the spikes, the fewest and the most turns in a tick, the ticks with every unit served), all of them counts the engine keeps. The census, ADR-0097's replica of the served set, stays in ADR-0097's four weekly tests as a test of behaviour; they are not the timed workload. This rule holds for every lever of ADR-0099's line: **a lever's gain is read on a workload whose instrument reads no record the engine does not read.**
- **The bounds, unchanged**: the change is kept if its median wall time per tick is at most **0.80** of the base's at (a) and at (d), and at most **1.10** at (b) and at (c).
- **The protocol, ADR-0099's and ADR-0100's, unchanged.** Each build is exported with `git archive` and built in a target directory of its own. Five pairs alternate the base and the change, the base first in pairs 1, 3 and 5. The four runs go in order within each build's turn, each run's figure is the median of its five readings, and every run's whole output is kept. The session starts only after the machine's total processor time has stayed below 15 per cent for thirty seconds with none of the round's processes running, and the total is sampled every five seconds through it.
- **The builds.** The base is `main` at the round's start with the census-free timing path added, and no sweep. The change is the base with the sweep re-applied. **The sweep is ADR-0100's as built** (`ff6a297` on `main`), re-applied by reverting its revert (`5aa57c5`). It may change only where `main` has moved under it, or where the mutation gate on its diff requires a change that moves neither behaviour nor a turn's work. Each such change is listed in the round's ADR, and all are committed before the first timed run. The mutation gate is read on the sweep's diff before that run, in CI on the round's pull request, so that nothing it forces follows the timing.
- **Read beside the criterion, not a criterion:** the same five pairs on ADR-0099's workload with the census, so that the old reading is repeated in the open; and one worker at (a) and (c).
- **Kept.** If kept, the sweep lands as ADR-0100 designed it:
  - axiom A3 enforced by ownership, with §4's row, §8.5 and every sentence that says the gate enforces it restated;
  - phase 1's `unsafe` restated with its invariant and its tests;
  - `deque.rs`, `Config::deque_capacity` and the stealing removed;
  - ADR-0017's and ADR-0023's decisions amended where the round's ADR says;
  - the weekly dispatch at the scope [ADR-0075](0075-the-dispatch-scope-follows-the-diff.md) gives the diff;
  - the cost table regenerated.
- **Not kept.** If not kept, the pull request carries the readings, the instrument and the round's ADR, not the sweep. There is no further attempt at the sweep in this form. The next decision is an ADR among options 1, 3, 4 and 5.
- **F-50 resolved** by this ADR: the workload rule above. ADR-0100's verdict stands as the reading of ADR-0099's workload.

### Consequences

- Good: the gain the line is for — the engine's turn, not the test's census — is what the criterion reads, and every later lever is read the same way.
- Good: the choice is made in the open. The old workload is read again beside the new one, the bounds are ADR-0099's, and the sweep's code is ADR-0100's.
- Bad: a workload chosen after a failure is weaker evidence than one chosen before any run. The reader has this record's word that the census is not the engine's work. The tree's evidence is the absence of `.gate()` under `src/`, and the one harness step that reads every record between ticks, the quiet run before an image, is not what the line times.
- Bad: ADR-0100's diagnostics predicted the sweep passes at (c). A round that reads what a diagnostic predicted is exposed to the same bias the diagnostics were kept out of the verdict to avoid. The criterion's session is new, and its readings, not the diagnostics, decide.
- Neutral: ADR-0097's census and its tables are untouched; only what is timed changes.

## Alternatives considered and why rejected

- **Option 1, the working layout without the sweep**: its premise is a contiguous range served in order, which only the sweep gives, and on the deque a turn still carries the gate's cost beside the integration.
- **Option 3, the sweep with a balance for the sparse case**: it answers a regression that ADR-0100's diagnostics place in the instrument. It would add a lock-free structure the tree does not have, for a need not shown.
- **Option 4, the lookahead next**: it needs ownership, which only the sweep gives.
- **Option 5, closing the line**: it leaves a turn at twice the integration's cost where ADR-0100 read it at the integration's.
- **Loosening the bound at (c)**: that moves a constant after a timed run.
- **Keeping the sweep on ADR-0100's diagnostics**: ADR-0100 rejected it, and rightly. Its diagnostics were five pairs with the load unsampled and read after the verdict. A new session under a workload written first is the reading.

## Confirmation

`briefs/044_the-sweep-measured-again.md` is the round that applies this workload and these bounds. Whitepaper 4.53.0 carries the decision: F-50 resolved in §11; §11.1's question on the engine's speed with this decision; and §9's row. `npm run spec` holds the links, the rows, the brief's sections and the version.
