---
status: accepted
date: 2026-10-25
depends-on: ADR-0209
title: A reward past the trial, its address kept, measured
kind: measured
hypothesis: H-43
summary: 'H-43 = yes at step 3: on seeds 22, 23 and 24, in both arms, at one trial''s delay and at two, H-25''s four clauses held, 118 to 128 of 128 correct. The address named the rewarded pair in 0.948 to 0.964 of rewards. Readings 1 and 3 held; reading 2 missed on three network-arms of six.'
decision-makers: VirtualCortex maintainers
---

# ADR-0210: A reward past the trial, its address kept, measured

**In brief**
- **Decision.** H-43 reads yes, as [ADR-0207](0207-a-reward-past-the-trial-its-address-kept.md) predicted. Its stopping rule reaches step 3. Predicted readings 1 and 3 held, and reading 2 missed.
- **Why.** With the address kept, a reward one or two trials late reached its trial's pair in 95 % of rewards, and every mapping was learned. Without it, exploration 001 learned nothing at the same delays.
- **Left open.** Step 3's next decision, the reward right three times in four asked again on the default with a control at step zero, is named here and not taken.

## Context and Problem Statement

ADR-0207 wrote H-43, [ADR-0208](0208-a-reward-past-the-trial-its-address-kept-built.md) built the kept addresses and the delay, and [ADR-0209](0209-a-reward-past-the-trial-its-address-kept-its-protocol.md) committed the protocol on `e087b47` before any arm ran at a delay.
Brief 080 ran it once.
The readings' tables and the dispatches are in [H-43's report](../research/H-43.md).

## Decision Outcome

**H-43 = yes in all twelve arms, step 3 of its stopping rule.**

| Network | Arm | Delay | Clause 1: learned | Clause 2: couplings | Clause 3: reversals | Clause 4: outside sum |
| :--- | :--- | ---: | :--- | :--- | :--- | :--- |
| Seed 22 | Assignment first | 1 | yes, 118 to 122 | yes, 1.204 | yes, 11 to 13 blocks | yes |
| | | 2 | yes, 119 to 123 | yes, 1.195 | yes, 13 to 14 blocks | yes |
| | Mirrored first | 1 | yes, 121 to 125 | yes, 1.223 | yes, 13 to 14 blocks | yes |
| | | 2 | yes, 120 to 124 | yes, 1.213 | yes, 14 to 16 blocks | yes |
| Seed 23 | Assignment first | 1 | yes, 120 to 127 | yes, 1.209 | yes, 10 to 12 blocks | yes |
| | | 2 | yes, 122 to 125 | yes, 1.208 | yes, 10 to 12 blocks | yes |
| | Mirrored first | 1 | yes, 120 to 128 | yes, 1.177 | yes, 9 to 13 blocks | yes |
| | | 2 | yes, 122 to 127 | yes, 1.171 | yes, 12 to 14 blocks | yes |
| Seed 24 | Assignment first | 1 | yes, 121 to 124 | yes, 1.277 | yes, 11 to 13 blocks | yes |
| | | 2 | yes, 122 to 127 | yes, 1.257 | yes, 12 to 14 blocks | yes |
| | Mirrored first | 1 | yes, 120 to 126 | yes, 1.223 | yes, 13 to 15 blocks | yes |
| | | 2 | yes, 119 to 126 | yes, 1.230 | yes, 12 to 15 blocks | yes |

**The three predicted readings:**
1. Held: the address each reward wrote named the rewarded trial's pair in 0.948 to 0.964 of each arm's rewards; on time, 0.951 to 0.965.
2. Missed: at one trial's delay the reversals summed fewer blocks than on time on seed 23 from the mirrored assignment, 34 against 37, and on seed 24 from the assignment, 35 against 37. At two, seed 24 from the mirrored assignment summed fewer than at one, 41 against 42.
3. Held: each mapping's last block's mean value was 0.836 to 0.985 of the reward.

**The stopping rule** reaches step 3. Its next decision is the reward right three times in four asked again on the default, with a control at step zero; then a delay the engine is not told. This ADR does not take it.

**The calibration** held before any arm ran at a delay.
- Dispatch 40164773490 at `scope=exhaustive` on `e087b47` ran the tree's other 166 tests in 18 shards, H-43's 18 excluded, each green.
- The six twins, dispatch 40164780397 on `e087b47`, were each H-42's arm on its network bit for bit: blocks, trace and all 7 680 values.

**The evidence.**
- The arms ran once each at `scope=arms` on AArch64, dispatch 40170518252 on `f87b02f`, and were pinned in `4bc7f7e`, pull request #232.
- The reproduction, dispatch 40171928796 at `scope=both` on `4bc7f7e`, ran 184 tests in 18 shards on x86-64, each green, and reproduced every pinned table, the twins' and the arms' among them.
- Its jobs took 54.0 to 95.3 minutes, the longest 53 % of its 180-minute bound. The cost table is regenerated from it: 184 tests, 152 906 s.
- No commit under `runtime/`, `crates/` or `benches/` follows `4bc7f7e`.

**The sweep of that dispatch had not ended** when this ADR asked for the merge. The first ADR that merges after it ends records it ([ADR-0158](0158-the-sweep-off-the-next-decisions-path.md)). No earlier sweep is unrecorded: brief 079's is [ADR-0207](0207-a-reward-past-the-trial-its-address-kept.md)'s, and no weekly schedule has run since 2026-10-19.

## Consequences

- Good: the engine credits a late reward to the trial that earned it. At one or two trials' delay it learned as on time, where with no address kept it learned nothing.
- Good: the delay cost little: the reversals summed 215 blocks on time, 221 at one trial's delay and 238 at two.
- Bad: the host names the delay. A world that does not say it is the next question's, and the trace's 0.605 over two trials bounds how far the kept address can reach.
- Neutral: reading 2's misses are reversals one to three blocks faster at a delay than without one. On time, the two arms of one network differ by up to six blocks, 34 and 40 on seed 22.

## Confirmation

`tests/inhibition.rs` holds the tables and reads the verdict, its step and the three readings from them in the gate (`kept_tables`, `KEPT_1024`, `KEPT_STOP_1024`, `KEPT_READINGS_READ_1024`).
