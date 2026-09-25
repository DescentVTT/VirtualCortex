---
status: accepted
date: 2026-09-25
depends-on: ADR-0098
decision-makers: VirtualCortex maintainers
---

# ADR-0099: The engine's speed — the three levers that change no rule, taken in the order the sizes the tree runs can read them: the sweep without the gate first, a working layout of the integrated fields for the vector units second, the lookahead third; each held to every reading of behaviour bit for bit, a pin that also holds the scheduler's own bytes restated only when it is shown equal with those bytes masked, and each kept only if a gain written before its first timed run is read; brief 043 builds the first

## Context and Problem Statement

[ADR-0098](0098-the-integration-model.md) kept per-tick service and named five levers on the engine's speed. The three that change no rule are the lookahead, a sweep without the gate, and a working layout of the integrated fields for the vector units. The maintainers chose to take those three before the learning line resumes, and ADR-0098 left their order, their acceptance and the measure of their gain to this ADR.

What a turn costs, and where, is known only from a developer machine (not admissible; `docs/benchmarks/results/2026-09-25-dancr-win11.md`, [ADR-0097](0097-the-active-set-measured.md)):

- the gate's schedule, begin and end: 9.79 ns;
- the integration: 11.03 ns;
- one message pushed and drained: 4.70 ns;
- the barriers and the coordinator of an idle tick on two workers: 468 to 543 ns;
- the runs' own figure: 24 to 25 ns of one worker per turn, and up to 38 ns in an earlier run of the same tests, so two sessions on one machine differ by half.

At the sizes the tree runs, the reference network at 1 024 units on two workers takes 12.7 µs a tick. The barriers are about 4 per cent of that. The unit arena (64 KB) and its synapse blocks (512 KB) fit the caches.

Three facts of the tree bear on the order and the acceptance:

- **Axiom A3** (whitepaper §4): "At most one worker touches a record in any tick, enforced by a compare-and-swap gate" ([ADR-0017](0017-mailbox-and-gate-protocol.md)). A sweep without the gate keeps the invariant and moves its enforcement.
- **The determinism pin** (`PINNED_ARENA_HASH` in `runtime/cortex-runtime/tests/differential.rs`) hashes every unit's 64 image bytes, "atomics as plain values". So it holds the gate byte and the mailbox head, which is a node index from a worker's pool, beside the dynamics. Its companion `PINNED_SPIKE_COUNT` holds the dynamics alone. A change of scheduling can move the hash with no change to the dynamics, as ADR-0054 moved it for a field the loop writes.
- **The executor's `unsafe`** in phase 1 rests on "this worker took `unit` from a deque, where it was put by the one `try_schedule` that moved its gate to scheduled".

## Decision Drivers

- **No reading of behaviour may move.** The tree's pins are the acceptance, and every earlier round's readings stay true.
- **ADR-0010.** A speed-up is read on a developer machine and is never Measured. Two sessions differ by half, so a gain must be read by alternating the base and the change in one session, under a criterion written before the first timed run.
- **ADR-0078.** Each lever is justified by the share of the turn it removes, from the bench, not by its name.
- **One brief at a time.** Each lever is one round with its own ADR.

## Considered Options

For the order:

1. The lookahead first: the largest lever at Appendix A's scale. It needs units owned by workers, and at 1 024 units it can remove about 4 per cent of a tick and none of the memory traffic, since the arenas fit the caches.
2. **The sweep without the gate first**: the gate is about two fifths of a turn wherever the fraction served is near one, which is every size the tree runs under ADR-0044's drive. It introduces the ownership the lookahead needs.
3. The working layout first: it needs a contiguous range served in order to feed the vector units, which the sweep introduces.

For the acceptance: every pin held literally; or every reading of behaviour held literally and a pin that also holds the scheduler's own bytes restated under a stated check.

## Decision Outcome

**Option 2, then 3, then 1, with the second form of acceptance.**

- **The order.** Brief 043 builds the sweep without the gate. The working layout comes second, the lookahead third, each by its own brief. Each brief's ADR may reorder the rest on its own readings.
- **The acceptance**, for all three levers:
  - *Behaviour bit for bit.* Every reading of behaviour holds without exception. That covers every potential, window, stamp and threshold, the short-term factors, every weight and trace, the spike train and the spike count, the messages delivered and the turns served summed over the workers, every pinned number of every round in the gate and in the weekly whole-domain tests, the AArch64 job, and the differential test at one worker and four.
  - *Scheduler state.* A pin that also holds the scheduler's own bytes — the gate byte, a mailbox head's node index — may be restated only if, before it is restated, the round computes that pin with those bytes masked on the base and on the change, finds the two equal, and records both values beside the old and new pin. The spike count or any behaviour reading beside it must not move. A per-worker share that no test asserts is not a pin.
  - *Stop.* Any other pin that moves stops the round. That is a finding, the change is not merged, there is no re-pin and no second attempt in that round, and the next decision is an ADR.
  - *Standing rules.* No record field and no image format moves: a working layout lives beside the record, not in the image, and a quantity a lever needs that the image already implies, such as the shortest delay, is derived at load. A lever that needs a format change stops and goes to an ADR. Any `unsafe` stays in the runtime under a restated invariant with its test ([ADR-0023](0023-executor.md)). No dependency, no nightly feature, no `std::simd`. The mutation gate on the diff applies.
- **Axiom A3.** The single-writer invariant stays. The sweep's round may move its enforcement from the compare-and-swap gate to ownership by a worker. It restates the axiom's enforcement in §4 and ADR-0017's role for the gate, and names the test that holds it.
- **The measure of the gain, written before any timed run of a change.**
  - *Workload:* ADR-0097's four runs in `tests/active.rs`, which print the wall time per tick. They are (a) 1 024 units under ADR-0044's drive, (b) sixteen times sparser, (c) 256 times sparser, and (d) 4 096 units.
  - *Protocol:* the base (main at the round's start) and the change are built in release in two target directories. Five pairs of runs, alternating base and change, are made in one session on one otherwise idle machine. Each run's figure is the median of its five readings.
  - *Criterion:* the change is **kept** if its wall time per tick is at most **0.80** of the base's at (a) and at (d), and at most **1.10** of the base's at (b) and at (c). Otherwise it is not kept. The round then merges its readings and the tests that hold behaviour, not the change, and an ADR decides whether the next lever proceeds without it.
  - The figures are a developer machine's, recorded under `docs/benchmarks/README.md` as not admissible and never written into §10.2. The weekly job's shard times are read beside them as a secondary reading, subject to runner variance of F-45's kind, and are not a criterion. The criterion's constants do not move after a timed run.
- **What each round reads for the next.** After the sweep, a per-turn breakdown: the integration's share of a turn on the sweep, from the bench beside the runs. That share is the measured need the working layout's ADR weighs, together with quality goal 2. The lookahead's ADR weighs the barriers' share at the reference platform's worker count and the arena's traffic past the caches.
- **The line's end.** After the three levers, or at a stop, the learning line resumes from H-18's named next decision (ADR-0098).

### Consequences

- Good: every earlier reading stays true, because behaviour is held bit for bit and the one exception, the scheduler's own bytes, is shown equal with those bytes masked before any pin is restated.
- Good: a gain is read against a criterion written first, in a protocol that alternates base and change, so neither session noise nor the round's hopes decide it.
- Good: the first lever pays at the sizes the tree runs, and it builds the ownership the lookahead needs.
- Bad: the lever with the largest reach at Appendix A's scale comes last.
- Bad: a lever can be built and not kept. The criterion is meant to allow that.
- Neutral: the reference platform is still unmeasured. These rounds' gains are ratios on one developer machine, not figures of the platform.

## Alternatives considered and why rejected

- **The lookahead first**: at the sizes the tree runs it can remove about 4 per cent of a tick. Its gain would be below the noise of the protocol at the sizes the tree can time.
- **Every pin literally**: the determinism pin holds the gate byte and the mailbox heads. Any change of who serves a unit, or of the order in which a worker's pool hands out nodes, moves it with no change to the dynamics. Holding it literally would forbid the sweep, not protect behaviour.
- **A gain read from the weekly job**: its shards run on runners whose speed varied by a factor of two in ADR-0097's dispatch (one H-18 arm took 4 121 s against its twin's 1 806). It is kept as a secondary reading.
- **No criterion**: without a threshold written first, any change that is not slower would be kept. The protocol's noise is large enough for that to keep nothing real.

## Confirmation

`briefs/043_the-sweep-without-the-gate.md`, the round that builds the first lever under this acceptance and this criterion. Whitepaper 4.51.0: §11.1's question on the engine's speed records the order and the acceptance, and §9 carries this ADR's row. `npm run spec` holds the links, the rows, the brief's sections and the version.
