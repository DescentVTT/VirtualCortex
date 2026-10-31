---
status: accepted
date: 2026-11-01
depends-on: ADR-0229
title: A zero sum at the stimulus, and a gentler depression, measured
kind: measured
hypothesis: H-46
summary: H-46 = yes at step 3. On seeds 22, 23 and 24 over sixteen schedules, all six arms held H-25's four clauses in every schedule, the highest coupling 1.157 to 1.215. All six controls passed 1.30 by the third schedule, reaching 1.98 to 2.11. All four readings held; F-86 is resolved.
decision-makers: Menteion maintainers
---

# ADR-0230: A zero sum at the stimulus, and a gentler depression, measured

**In brief**
- **Decision.** H-46 reads yes, as [ADR-0226](0226-a-zero-sum-at-the-stimulus.md) predicted, at step 3 of its stopping rule. All four predicted readings held, and F-86 is resolved.
- **Why.** In all six arms H-25's four clauses held in each of the sixteen schedules, at the default's pace. In all six controls a coupling passed 1.30 by the third schedule.
- **Left open.** Step 3's next decision: whether the two rules enter the engine's default, whether M12 closes, and then M11. It is named here and not taken. The arms' couplings have not settled in shape.

## Context and Problem Statement

ADR-0226 wrote H-46, [ADR-0228](0228-a-zero-sum-at-the-stimulus-built.md) built the two rules, and [ADR-0229](0229-a-zero-sum-at-the-stimulus-its-protocol.md) committed the protocol on `ea7ca6a` before any arm ran.
Brief 085 ran it once.
The readings' tables and the dispatches are in [H-46's report](../research/H-46.md).

## Decision Outcome

**H-46 = yes in all six arms and all six controls, step 3 of its stopping rule.**

| Network | Arm | Clause 1: every mapping learned | Clause 1: no coupling past 1.30 | Clause 1: every reversal within 23 | Clause 1: the outside sum | Clause 2: the control past 1.30 |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Seed 22 | Assignment first | yes, 122 | yes, 1.172 | yes, 13 | yes | yes, schedule 2, to 2.072 |
| Seed 22 | Mirrored first | yes, 122 | yes, 1.186 | yes, 13 | yes | yes, schedule 3, to 2.061 |
| Seed 23 | Assignment first | yes, 123 | yes, 1.157 | yes, 10 | yes | yes, schedule 3, to 2.019 |
| Seed 23 | Mirrored first | yes, 122 | yes, 1.177 | yes, 11 | yes | yes, schedule 3, to 2.083 |
| Seed 24 | Assignment first | yes, 123 | yes, 1.215 | yes, 12 | yes | yes, schedule 2, to 1.979 |
| Seed 24 | Mirrored first | yes, 125 | yes, 1.212 | yes, 12 | yes | yes, schedule 3, to 2.108 |

Each number is the run's worst over its sixteen schedules: the lowest correct of 128, the highest coupling against H-42's image, the longest reversal in blocks.
The outside sum is pinned as its clause, within 0.75 and 1.25 of H-42's image's.

**The four predicted readings:**
1. Held: each arm's reversals over schedules 9 to 16 summed 103 to 112 blocks, 0.88 to 1.06 of its control's, under 1.25.
2. Held: each arm's highest coupling was 1.157 to 1.215 of H-42's image's, under 1.25.
3. Held, on three schedules alone: the sixteenth of seed 22's two arms and of seed 24's mirrored arm, at 13, 12 and 12 blocks. No control and no earlier schedule read it.
4. Held: each arm's inhibitory sum stood at 0.486 to 0.522 of H-42's image's at every block's end of schedules 9 to 16, inside 0.45 to 0.65.

**The stopping rule** reaches step 3: the couplings are bounded over sixteen schedules on three networks, M12's exit test for them, and reading 4 reads the inhibitory sum.
Its next decision is whether the two rules enter the engine's default, whether M12 closes, and then M11; this ADR takes none of them.

**F-86 is resolved.**
ADR-0037's reason, that nothing in the tree showed weight saturation, does not hold over long runs.
Under the default the most synapses of one coupling at the rail were 88 to 123 of its 767 to 824 by the eighth schedule, and 169 to 219 by the sixteenth.
Under the two rules they were 76 to 92, and 115 to 132.
ADR-0037's decision stands on its other reasons, and this round adopts no downscaling.

**The calibration** held before any arm's log was read.
- Dispatch 41199135859 at `scope=exhaustive` on `ea7ca6a` ran the tree's other 221 tests in 18 shards with both rules unset, each green, in 36.2 to 53.1 minutes of the 240-minute bound.
- Each control's first schedule, in dispatch 41199138073 on `ea7ca6a`, was H-42's arm bit for bit, by a check that fails its own job.

**The evidence.**
- The first halves ran at `scope=arms` on AArch64, dispatch 41199138073 on `ea7ca6a`, in 101.9 to 110.8 minutes.
- The second halves ran on `e10d992`, which adds the twelve eighth images and nothing else that runs, dispatch 41211071986, in 92.8 to 99.5 minutes.
- The tables are pinned in `15e454c`, pull request #260. From `ea7ca6a` to it no file under `src/` changed and no test or helper the calibration ran.
- The reproduction ran on `15e454c`. Dispatch 41219386178 at `scope=exhaustive`, `only` the six short forms, was green on x86-64 in 1.1 to 15.7 minutes, each short form holding its arm's first row.
- Dispatch 41219389039 at `scope=arms` ran the twenty-four halves again on AArch64, each job green in 93.1 to 112.0 minutes of 240, every pinned row held by its own job's assertion.
- The cost table is written from the calibration: 221 tests, 91 431 s. ADR-0220's tool does not read a shard that ran its tests alone, so the six short forms have no line, and each is costed at 2 400 s until a full deal measures it.
- No commit under `runtime/`, `crates/` or `benches/` follows `15e454c`.

**No sweep is left unrecorded** that had ended when this ADR was written. This round dispatched none ([ADR-0211](0211-the-round-shortened.md)). The schedule's next runs on 2026-11-04 at 20:17Z; the first ADR that merges after it ends records it.

## Consequences

- Good: a rule on the stimulus's own synapses bounds the couplings on three networks, with no cap on a single synapse and at the default's pace.
- Good: the arms' inhibitory sum moved at most 0.010 over schedules 9 to 16, where the controls' rose 0.033 to 0.074.
- Bad: the arms' couplings have not settled in shape. Their bottom eighth's share rose about 0.02 a schedule to the last, to 0.46 to 0.52, and the middle of the width emptied.
- Bad: reading 3 stands on the last schedule alone, so what a split costs in speed is hardly read.
- Bad: no run reads past sixteen schedules.
- Neutral: the six short forms join the weekly job, at 532 to 842 s each run alone on x86-64.

## Confirmation

`runtime/menteion-runtime/tests/inhibition/pins/h46.rs` holds the twelve runs' rows, the verdict, its step and the four readings, and `the_zero_sum_of_h_46_its_clauses_and_its_predicted_readings_rules` reads them by the protocol's rules at the gate.
[H-46's report](../research/H-46.md) holds the readings' tables and the dispatches.
The six `…_exhaustive` short forms hold each arm's first row on the weekly job.
`npm run spec` holds the links, the tables, the brief's scope and the version.
