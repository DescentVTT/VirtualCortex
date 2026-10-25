---
status: accepted
date: 2026-10-25
depends-on: ADR-0213
title: A reward right three times in four, on the default, measured
kind: measured
hypothesis: H-44
summary: H-44 = yes at step 3, split (b). On seeds 22, 23 and 24, in both arms, H-35's four clauses held from the default's image, 90 to 127 of 128, and from the control's at step zero, 80 to 127. The network the default settles holds the share. Readings 1 and 2 missed, 3 held.
decision-makers: VirtualCortex maintainers
---

# ADR-0214: A reward right three times in four, on the default, measured

**In brief**
- **Decision.** H-44 reads yes, as [ADR-0212](0212-three-in-four-on-the-default.md) predicted. Its stopping rule reaches step 3, split (b): the control at step zero read yes too. Predicted readings 1 and 2 missed, and reading 3 held.
- **Why.** Under a reward right three times in four, every mapping was learned on the default's three networks, with the rule moving the gain and with the gain held. H-35 and H-36 read no on two of three other networks.
- **Left open.** Step 3's next decision, a delay the engine is not told, is named here and not taken.

## Context and Problem Statement

ADR-0212 wrote H-44, and [ADR-0213](0213-three-in-four-on-the-default-its-protocol.md) committed the protocol on `72959ae` before any arm ran at three in four.
Brief 081 ran it once.
The readings' tables and the dispatches are in [H-44's report](../research/H-44.md).

## Decision Outcome

**H-44 = yes in all six of the default's arms, step 3 of its stopping rule, split (b).**

| Network | Arm | Start | Clause 1: learned | Clause 2: couplings | Clause 3: outside sum | Clause 4: expected reward |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Seed 22 | Assignment first | default | yes, 105 to 122 | yes, 1.212 | yes | yes |
| | | control | yes, 94 to 121 | yes, 1.188 | yes | yes |
| | Mirrored first | default | yes, 90 to 122 | yes, 1.172 | yes | yes |
| | | control | yes, 88 to 121 | yes, 1.152 | yes | yes |
| Seed 23 | Assignment first | default | yes, 120 to 125 | yes, 1.231 | yes | yes |
| | | control | yes, 116 to 124 | yes, 1.152 | yes | yes |
| | Mirrored first | default | yes, 106 to 125 | yes, 1.178 | yes | yes |
| | | control | yes, 101 to 124 | yes, 1.156 | yes | yes |
| Seed 24 | Assignment first | default | yes, 90 to 127 | yes, 1.231 | yes | yes |
| | | control | yes, 80 to 127 | yes, 1.201 | yes | yes |
| | Mirrored first | default | yes, 92 to 121 | yes, 1.173 | yes | yes |
| | | control | yes, 101 to 118 | yes, 1.168 | yes | yes |

**The three predicted readings:**
1. Missed: no control mapping fell below 80 of its last 128; the lowest was 80, on seed 24 from the assignment.
2. Missed: the default's lowest mapping stood above the control's on five network-arms of six; on seed 24 from the mirrored assignment it was 92 against 101.
3. Held: all 18 of the default's reversals reached 40 of 64 inside their 32 blocks, the slowest in 28.

**The stopping rule** reaches step 3, split (b).
The control's yes reads the share as held by the network the default settles, with the rule not needed for it.
Its next decision is a delay the engine is not told; this ADR does not take it.

**What the rule did.** With the rule, the arms held their windows at their targets, 1.001 of them, and lowered the gain from 1.745–1.747 to as low as 1.703.
With the gain held, the same networks ran 24 to 26 % above their targets and learned as well: 110.5 of 128 on average against 112.9, and 356 blocks of reversal against 359.

**The calibration** held before any arm's log was read.
- The trial of the harness, dispatch 40199691326 on `72959ae`, ran the three calibration tests on AArch64, each green.
- Dispatch 40199683055 at `scope=exhaustive` on `72959ae` ran 187 tests in 18 shards, the twelve arms excluded and the three calibration tests among them, each green.
- Its jobs took 52.9 to 104.6 minutes, the longest 58 % of its 180-minute bound.
- On each network the derivation was H-42's, the coin H-35's, the control's image the default's but for the step's two bytes and the section's seal, and the default's first block under the true feedback H-42's arm's.

**The evidence.**
- The arms ran once each at `scope=arms` on AArch64, dispatch 40200561650 on `72959ae`, and were pinned in `55097b8`, pull request #237.
- The reproduction, dispatch 40208314220 at `scope=exhaustive` with `only` the twelve arms on `55097b8`, ran them in 18 shards on x86-64, each green, and reproduced every pinned table.
- Its jobs took 1.2 to 22.4 minutes. The cost table is regenerated from the calibration and the reproduction: 199 tests, 176 376 s.
- From `72959ae` to `55097b8` only the pinned tables and the verdict as read changed, at the end of `tests/inhibition.rs`; no test the calibration ran and nothing under `src/` changed.
- No commit under `runtime/`, `crates/` or `benches/` follows `55097b8`.

**No sweep is left unrecorded.** This round dispatched none ([ADR-0211](0211-the-round-shortened.md)). Brief 080's is [ADR-0212](0212-three-in-four-on-the-default.md)'s, the schedule's of 2026-10-19 is [ADR-0206](0206-the-probe-in-the-engines-default-built.md)'s, and the next schedule runs on 2026-10-26 at 03:00Z.

## Consequences

- Good: the engine's default learns a reward right three times in four on three networks, where with the gain held H-35 and H-36 read no on two of three other networks.
- Good: the control locates the difference in the network the default settles, not in the rule through the arm.
- Bad: the control's lowest mapping sat at the clause's edge, 80 of 128, and seed 23's default used 0.98 of clause 4's band; neither margin is wide.
- Neutral: the default's arms are exploration 001's, mapping for mapping, so the host's path and the task's feedback gave one run.

## Confirmation

`tests/inhibition.rs` holds the tables and reads the verdict, its step and the three readings from them in the gate (`on_default_tables`, `ON_DEFAULT_1024`, `ON_DEFAULT_STOP_1024`, `ON_DEFAULT_READINGS_READ_1024`).
