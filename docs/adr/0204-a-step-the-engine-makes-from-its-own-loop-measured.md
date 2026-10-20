---
status: accepted
date: 2026-10-20
depends-on: ADR-0203
title: A step the engine makes from its own loop, measured
kind: measured
hypothesis: H-42
summary: 'H-42 = yes at step 3: on seeds 22, 23 and 24 the probe read S = 15.0 to 15.2 and made step = 1/61 to 1/60, each lead-in settled, and both arms held H-25''s four clauses. All four predicted readings held: the opposite pairs fell to 485 to 527 of 959 from H-41''s 591 to 623.'
decision-makers: VirtualCortex maintainers
---

# ADR-0204: A step the engine makes from its own loop, measured

**In brief**
- **Decision.** H-42 reads yes, as [ADR-0200](0200-a-step-the-engine-makes-from-its-own-loop.md) predicted, and its four predicted readings held. Its stopping rule reaches step 3.
- **Why.** The probe's step held each network's count with less alternation than step = 1/16, and learning was not slowed.
- **Left open.** Step 3's next decision, an ADR that puts the probe in `Config::reads_its_targets()`, is named here and not taken.

## Context and Problem Statement

ADR-0200 wrote H-42, [ADR-0202](0202-a-step-the-engine-makes-from-its-own-loop-built.md) built the probe, and [ADR-0203](0203-a-step-the-engine-makes-from-its-own-loop-protocol.md) committed the protocol on `049ec66` before any lead-in with the probe set.
Brief 078 ran it once.
The readings' tables and the dispatches are in [H-42's report](../research/H-42.md).

## Decision Outcome

**H-42 = yes on all three networks, step 3 of its stopping rule.**

| Network | Clause 0: read, probe, settling, rules, frozen block | Arm | Clause 1: learned | Clause 2: couplings | Clause 3: reversals | Clause 4: outside sum |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Seed 22 | yes: S = 15.19, step 1 078, settled at 176 | Assignment first | yes, 120 to 123 | yes, 1.239 | yes, 11 to 12 blocks | yes |
| | | Mirrored first | yes, 120 to 125 | yes, 1.248 | yes, 13 to 14 blocks | yes |
| Seed 23 | yes: S = 15.10, step 1 085, settled at 176 | Assignment first | yes, 124 to 127 | yes, 1.206 | yes, 10 blocks | yes |
| | | Mirrored first | yes, 121 to 127 | yes, 1.199 | yes, 12 to 13 blocks | yes |
| Seed 24 | yes: S = 15.02, step 1 091, settled at 176 | Assignment first | yes, 123 to 127 | yes, 1.254 | yes, 11 to 13 blocks | yes |
| | | Mirrored first | yes, 122 to 127 | yes, 1.233 | yes, 12 to 13 blocks | yes |

**The four predicted readings:**
1. Held: each network's S was 15.02 to 15.19, inside 12 to 24.
2. Held: each arm's opposite pairs were 485 to 527 of 959, under 540; H-41's twins read 591 to 623.
3. Held: each arm's mean count a window was 0.13 to 0.15 % above its read target, inside 1 %.
4. Held: the gain after every window of each arm was 1.7032 to 1.7408, inside 5 % of 1.75.

**The stopping rule** reaches step 3. Its next decision is an ADR that puts the probe in `Config::reads_its_targets()` in step = 1/16's place, and then M10. This ADR does not take it.

**The evidence.**
- The calibration, dispatch 39366349243 at `scope=exhaustive` on `049ec66`, ran 156 tests in 18 shards with H-42's ten excluded. Every pin held at format 25 with the probe unset.
- The trial, the derivations and the arms ran once each at `scope=arms` on AArch64: dispatches 39373902002 and 39374289306 on `049ec66`, and 39374860385 on `158fde4`.
- The tables were pinned from those logs in `158fde4` and `1fc2f28`, pull request #227.
- The reproduction, dispatch 39376979724 at `scope=both` on `1fc2f28`, ran 166 tests in 18 shards on x86-64, each green, and reproduced every pinned table. Its jobs took 43.0 to 87.1 minutes, the longest 48 % of its 180-minute bound. The cost table is regenerated from it: 166 lines, 130 054 s.

**The sweep of that dispatch, recorded here.** It ended green before any ADR merged, so this one records it ([ADR-0158](0158-the-sweep-off-the-next-decisions-path.md)).

| Outcome | Mutants |
| :--- | ---: |
| Caught | 3 793 |
| Missed | 0 |
| Unviable | 158 |
| Timeouts | 28 |
| All | 3 979 |

- 46 mutants more than the 3 933 [ADR-0199](0199-one-place-for-each-kind-of-fact-built.md) recorded: the probe's code, ADR-0202's.
- 23 timeouts are where [ADR-0062](0062-the-first-complete-sweeps-list.md) reads them as inherent: ADR-0199's 22, and the barrier's second wait, `barrier.rs:41:55`, a spinning hang.
- 3 are in `Image::decode`, ADR-0199's three at their moved lines, each beside an injector mutant that timed out. Two were still finishing tests, and one had stopped with none of its unreported tests its own module's.
- `store.rs:148:9` was still finishing tests. These four are F-42's kind, by [ADR-0063](0063-the-sweep-reads-its-own-timeouts.md)'s evidence.
- `branching.rs:100:16` ran its whole window beside `barrier.rs:41:55`'s spinning hang. Its log ends in `tests/criticality.rs`'s tests on four workers, none of its own module's: F-42's kind.
- No survivor, so no finding.

## Consequences

- Good: the rate's rule no longer needs a step the designer chose. The engine read S near 15 on three networks and made step = 1/61 to 1/60.
- Good: the alternation ADR-0200 traced to κS near 1 fell by 12 to 21 % against each arm's twin, and the reversals were no slower: 215 blocks in all against the twins' 227.
- Bad: the count sat a little further above its target, 0.13 to 0.15 % against 0.03 to 0.04 %, a step about four times smaller holding a drift more slowly.
- Neutral: the lead-in is 16 windows longer, and every network settled at the first window the rule can read after the probe.

## Confirmation

`tests/inhibition.rs` holds the tables and reads the verdict, its step and the four readings from them in the gate (`probe_tables`, `PROBE_1024`, `PROBE_STOP_1024`, `PROBE_READINGS_READ_1024`).
