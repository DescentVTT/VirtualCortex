---
status: proposed
date: 2026-10-25
depends-on: ADR-0218
title: A delay the engine reads from its kept addresses, measured
kind: measured
hypothesis: H-45
summary: H-45 = yes at step 3. On seeds 22, 23 and 24, at one and two trials' delay untold, the engine moved once to the true age, at trials 128 to 4 269, and every mapping after it was learned, 118 to 127 of 128. The six controls never moved and were H-42's arms. All three readings held.
decision-makers: VirtualCortex maintainers
---

# ADR-0219: A delay the engine reads from its kept addresses, measured

**In brief**
- **Decision.** H-45 reads yes, as [ADR-0216](0216-a-delay-the-engine-reads.md) predicted, at step 3 of its stopping rule. All three predicted readings held.
- **Why.** In all twelve arms the engine's critics named the delay it was not told, and its one move went to the true age; on time it never moved.
- **Left open.** Step 3's next decision, the re-read of the targets by evidence at M12, is named here and not taken. The mappings consolidated at age zero before a late move are lost.

## Context and Problem Statement

ADR-0216 wrote H-45, [ADR-0217](0217-a-delay-the-engine-reads-built.md) built the rule, and [ADR-0218](0218-a-delay-the-engine-reads-its-protocol.md) committed the protocol on `b418b75` before any arm ran.
Brief 082 ran it once.
The readings' tables and the dispatches are in [H-45's report](../research/H-45.md).

## Decision Outcome

**H-45 = yes in all twelve arms and all six controls, step 3 of its stopping rule.**

| Network | Arm | Delay | Clause 1: the move | Clause 2: after the move's mapping | Clause 3: the control |
| :--- | :--- | ---: | :--- | :--- | :--- |
| Seed 22 | Assignment first | 1 | yes, trial 128, age 1 | yes, 122 to 124 | yes |
| | | 2 | yes, trial 129, age 2 | yes, 119 to 126 | |
| Seed 22 | Mirrored first | 1 | yes, trial 2 565, age 1 | yes, 119 to 122 | yes |
| | | 2 | yes, trial 251, age 2 | yes, 122 to 124 | |
| Seed 23 | Assignment first | 1 | yes, trial 2 144, age 1 | yes, 118 to 126 | yes |
| | | 2 | yes, trial 4 269, age 2 | yes, 119 | |
| Seed 23 | Mirrored first | 1 | yes, trial 1 448, age 1 | yes, 121 to 126 | yes |
| | | 2 | yes, trial 172, age 2 | yes, 122 to 125 | |
| Seed 24 | Assignment first | 1 | yes, trial 145, age 1 | yes, 124 | yes |
| | | 2 | yes, trial 309, age 2 | yes, 121 to 127 | |
| Seed 24 | Mirrored first | 1 | yes, trial 225, age 1 | yes, 121 to 126 | yes |
| | | 2 | yes, trial 274, age 2 | yes, 122 to 126 | |

Clause 2's numbers are the correct of each later mapping's last 128; its couplings, reversals and outside sum held too.
Each control's clause 3 is both arms on time, no move and H-42's arm bit for bit.

**The three predicted readings:**
1. Held: 9 arms of 12 moved inside the first mapping, at trials 128 to 1 448.
2. Held: all nine held H-25's four clauses over the whole arm.
3. Held: their reversals summed 0.76 to 1.08 of H-43's told arm's.

**The stopping rule** reaches step 3: the engine reads a delay of one or two trials it is not told.
Its next decision is the re-read of the targets by evidence, at M12; this ADR does not take it.

**What the move cost.** The three arms that moved after the first mapping lost it, at 32 to 42 of 128, consolidated at age zero.
Each move lost one reward per age moved, the move's own among them.

**The calibration** held before any arm's log was read.
- Dispatch 40226374616 at `scope=exhaustive` on `b418b75` ran the tree's other 199 tests in 18 shards with the rule unset, each green.
- The two step 2 checks, in dispatch 40226384795 on `b418b75`, were H-43's arms bit for bit with the rule set.
- **F-82**: the calibration's shard 10 of 18 took 128.1 minutes, 71 % of its 180-minute bound, and shards 3, 13 and 6 took 108.3 to 111.7.

**The evidence.**
- The arms ran once each at `scope=arms` on AArch64, dispatch 40226384795 on `b418b75`, and were pinned in `0694e76`.
- `a3840cf`, after the calibration's commit, rewrites two rules of the delay read for the mutation gate, their behaviour the same ([ADR-0217](0217-a-delay-the-engine-reads-built.md)). So the reproduction runs every test.
- `989ae23` writes the cost table from the calibration, so that the reproduction is dealt from what its tests take.
- The reproduction is dispatch 40237389553 at `scope=exhaustive` on `989ae23`, every test, on x86-64.

**No sweep is left unrecorded** that had ended when this ADR was written. This round dispatched none ([ADR-0211](0211-the-round-shortened.md)). The schedule's next runs on 2026-10-26 at 03:00Z; the first ADR that merges after it ends records it.

## Consequences

- Good: a reward late by a delay the engine is not told reaches the trial it is for, by a reading of the engine's own, on three networks.
- Good: on time the rule is the run it was in every control, so the lead of 1/32 made no wrong move where any move is one.
- Bad: before the move the engine consolidates at age zero, and a move after the first mapping leaves it unlearned.
- Bad: the leads at the move were 0.038 to 0.156 of the reward, the smallest within 1.22 of the bound.
- Neutral: every first move came at exploration 003's trial; the latch is what changed the two arms it moved twice in.

## Confirmation

`tests/inhibition.rs` holds the tables and reads the verdict, its step and the three readings from them in the gate (`untold_tables`, `UNTOLD_1024`, `UNTOLD_STOP_1024`, `UNTOLD_READINGS_READ_1024`).
