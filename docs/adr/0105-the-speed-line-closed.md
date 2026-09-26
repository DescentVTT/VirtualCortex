---
status: accepted
date: 2026-09-26
depends-on: ADR-0104
decision-makers: VirtualCortex maintainers
---

# ADR-0105: The speed line closed — of ADR-0099's levers the sweep without the gate is kept (ADR-0102) and the membrane's rule in lanes is not (ADR-0104); the engine now integrates the reference network at 1 024 units on two workers faster than a tick of real time on a developer machine, and what is left of the line is named with the need that would take each; the learning line resumes from H-18's named next decision (ADR-0106)

## Context and Problem Statement

[ADR-0098](0098-the-integration-model.md) kept per-tick service and named five levers on the engine's speed. The maintainers chose to take the three that change no rule before the learning line resumed. [ADR-0099](0099-the-engines-speed.md) ordered those three and wrote their acceptance and their measure. The line has run three rounds since:

- [ADR-0100](0100-the-sweep-without-the-gate.md) built the sweep without the gate and did not keep it on ADR-0099's workload, whose census of every unit between ticks was in the timed ticks (F-50). [ADR-0101](0101-the-sweep-measured-again.md) resolved F-50 and fixed the workload. [ADR-0102](0102-the-sweep-timed-with-no-census.md) timed the same code on it and **kept** it: 0.517, 0.527, 0.745 and 0.643 of the base's wall time per tick at [ADR-0097](0097-the-active-set-measured.md)'s runs (a) to (d), every reading of behaviour bit for bit. The weekly job's whole-domain tests fell from 26 945 s to 15 263 s of the cost table.
- [ADR-0103](0103-the-working-layout.md) took the working layout on the reading that the integration was the turn. [ADR-0104](0104-the-membrane-in-lanes.md) built the membrane's rule in lanes and did not keep it: 1.209, 1.132, 1.211 and 1.122, as it predicted before the run. It found F-51: the reading ADR-0103 was written on compared a one-unit chain with a turn from other sessions. Read in one session, the integration is about two thirds of a turn.

After the sweep, the reference network at 1 024 units on two workers takes about 5.7 µs of wall time per 10 µs tick at run (a): faster than real time. At 4 096 units it takes about 30 µs. (These are a developer machine's medians, not admissible.) The learning line runs at 1 024 units.

## Decision Drivers

- **ADR-0078.** A lever waits for a measured need. The need the line was opened on was ADR-0097's budget, and the learning line's own runs now meet it at the size they use.
- **The maintainers' order** (ADR-0098): the speed first, then the learning line.
- **What is left** is either small at the tree's sizes, or a decision of its own about the architecture or the build.

## Considered Options

1. Close the line and resume the learning line.
2. Take the lookahead, ADR-0099's third lever.
3. Take another of ADR-0104's named options: the rest of the turn, a persistent working copy, or a wider baseline.

## Decision Outcome

**Option 1.** The line is closed. What is left is named with the need that would take each:

| Lever | What it is | The need that would take it |
| :--- | :--- | :--- |
| The lookahead (§11.1) | synchronise once per window of the shortest delay, units advanced a window from one load | a size past the caches or the reference platform's worker count, where ADR-0097's arena traffic and the barriers bind; at 1 024 units the barriers are about 4 per cent of a tick on two workers |
| The rest of the turn | the mail check, the batch's sort, sum and scaling, the rest check and the gate byte, about a third of a turn (F-51) | a round whose runs a turn bounds, with that third read in one session first |
| A persistent working copy | the integrated fields kept in lanes across ticks, so that nothing is gathered and stored per tick | a need the transient lanes of ADR-0104 cannot meet, and an ADR amending quality goal 2 and [ADR-0001](0001-64-byte-pod-records.md), with every reader of a record re-argued |
| A wider baseline | a `target-cpu` or target features beyond SSE2 and NEON | a decision about every build and CI job, not a lever's |
| The rest condition, a longer tick | ADR-0098's two rule changes | not taken; every pinned number would move |

The learning line resumes from H-18's named next decision. [ADR-0106](0106-the-reward-prediction-error.md) takes it: the reward-prediction error.

### Consequences

- Good: the tree keeps the one lever that paid, held bit for bit, and records the two readings that did not, with their reasons (F-50, F-51).
- Good: the learning line's runs, and the weekly job, run in about half the time they took before the line opened.
- Bad: Appendix A's real-time reach is about twice ADR-0098's estimate, still of the order of $10^4$ units at ADR-0044's density; the levers that would move it are the ones left above.
- Neutral: F-49's disposition stands (ADR-0098); §11.1's question on the engine's speed is closed with this table, and the lookahead's question stays open as it was.

## Alternatives considered and why rejected

- **The lookahead now**: it can remove at most the barriers' share at the sizes the learning line runs, about 4 per cent of a tick, below what ADR-0099's protocol can read.
- **The rest of the turn now**: about a third of a turn, at a size that already runs faster than real time.
- **A persistent working copy, or a wider baseline, now**: each is a decision about the architecture or the build, and needs its own ADR and its own need.

## Confirmation

Whitepaper 4.57.0: §11.1's question on the engine's speed is closed with this ADR, and §9 carries its row. `npm run spec` holds the links and the rows.
