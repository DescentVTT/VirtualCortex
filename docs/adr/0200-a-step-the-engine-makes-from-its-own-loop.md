---
status: accepted
date: 2026-10-20
title: A step the engine makes from its own loop
kind: decision
hypothesis: H-42
summary: After its read, the engine moves its gain by ±1/64 for 16 windows, reads the steepness S of its own count, and runs the rate's rule at step = (1/4)/S instead of 1/16. H-42 asks it on seeds 22, 23 and 24, predicted yes, the count's alternation below 540 pairs of 959.
depends-on: ADR-0198
decision-makers: VirtualCortex maintainers
---

# ADR-0200: A step the engine makes from its own loop

**In brief**
- **Decision.** With a flag set, the engine probes its own loop after the read. It measures the steepness S, and sets its step to κ = λ/S with λ = 1/4. H-42 asks it on H-41's three networks.
- **Why.** At step = 1/16 the loop gain κS is about 1, so the rule undoes each window's noise in the next. That is the count's leftover alternation. The step is also the last per-network constant of the rate's rule.
- **Left open.** The default keeps step = 1/16 until H-42 reads yes. A re-read of the targets by evidence stays M12's.

## Context and Problem Statement

[ADR-0196](0196-the-engines-default.md) names this decision first: *"a step the engine makes from the steepness it measures, with a hypothesis that it removes the alternation"*.
The maintainers took it on 2026-10-20, in the order they set on 2026-10-19.

**The rate's rule** (`crates/cortex-homeostasis/src/lib.rs`, `regulate_rate` and `move_gain`):
- once a window, g ← g·[1 − κ·clamp(c/c* − 1, −1, 1)], with g in [0.25, 4];
- c is the window's spikes over the population, c* the target count;
- a window is 2^17 ticks;
- κ is `control_step_q0_16`, the homeostasis record's `[20..22)`, also the step of ADR-0036's controller.

**The steepness S**, how far the count moves for a move of the gain, in per cent for one per cent:
- at step = 1/8 and 1/16 on seed 22, the first window's move gave 15.0 ([ADR-0181](0181-a-rate-the-controller-holds-measured.md));
- on seeds 23 and 24 it gave 21.4 and 16.0 ([ADR-0183](0183-a-sixteenth-on-three-networks-measured.md));
- ADR-0183 puts step = 1/16 at its loop's edge at S = 32.

**The leftover alternation is noise chased, not a loop at its edge.**
Read from the pinned arms, the lag-one correlation ρ of the window's count and its pairs of windows on opposite sides of the target are:

| Gain | κS | ρ | Opposite pairs of 959 |
| :--- | :--- | :--- | :--- |
| held, as ADR-0183 read it | 0 | +0.08 to +0.30 | 385 to 446 crossings of the mean |
| step = 1/16 | about 0.94 | −0.36 to −0.39 (H-39) | 591 to 623 (H-39 to H-41) |
| step = 1/8 | about 1.9 | −0.79 to −0.80 (H-38) | 750 and 758 (H-38) |

- On seed 22, ρ falls with κS along one line, ρ ≈ ρ_held − 0.46·κS.
- The pairs match a Gaussian count's sign changes, 959·(1/2 − arcsin(ρ)/π): ρ = −0.37 gives 595 where 597 was read.
- At κS near 1 each window's random deviation is corrected whole, and the next window is pushed the other way.
- The count's standard deviation is 74 to 82 under step = 1/16 and 72 to 78 with the gain held, so the rule holds the count without widening it.

These readings are this record's, from tables ADR-0181 to ADR-0195 pinned. No run was made for them.

**Why S cannot be read from the rule's own swings:** the pinned table slopes of count against gain fall with the step, 15.7 at 1/8 and 5.8 at 1/256 ([ADR-0181](0181-a-rate-the-controller-holds-measured.md)).
A small step moves the gain too little for the noise, and the slope is diluted toward zero.
It is the textbook failure of a self-tuning regulator without enough excitation.

**What the image has room for:**
- the modulator section's last reserved byte, `[27]`;
- the read's state, `[39]`, `[54..56)` and `[60..64)`, which nothing uses once the read has ended;
- the image is at format 24, and the homeostasis record has no reserved byte.

## Decision Drivers

- **The maintainers' choices** (2026-10-20): an active probe, λ = 1/4, and H-41's three networks.
- **The order they set** (2026-10-19): the default, then this step, then M10.
- **A question that can read something**: the alternation is a pinned reading, and the model above predicts how far it falls.
- **One change, with twins**: everything up to the read's end is H-41's.
- **Unset is the run it was**, bit for bit.
- **Latest ≠ Newest**: the engine's own rule, given a step from its own measure. A self-tuning regulator is fifty years old, with its failure modes written down; nothing is depended on.

## Considered Options

1. **How S is read**:
   - (a) an active probe after the read;
   - (b) a regression on the rule's own swings;
   - (c) no probe, a smaller fixed step.
2. **λ**: (a) 1/4; (b) 1/8.
3. **The networks**: (a) H-41's three at 1 024 units; (b) those and seeds 22 and 23 at 2 048 units.

## Decision Outcome

**Options 1(a), 2(a) and 3(a).**

### The probe (Specified; brief 078 builds it)

- **A flag in the modulator section's `[27]`**, at format 25.
  - Set beside the read, the read's end starts the probe instead of the rule.
  - Set without the read, it is refused.
  - Unset, every run is the run it was, bit for bit, and an image of format 24 reads as unset.
- **The probe holds the gain for 16 windows, in the order −, +, +, −, four, eight and four windows**:
  - at g₀·(1 − 1/64) and g₀·(1 + 1/64), g₀ the gain the read held;
  - the count of each + window added to one signed sum, and the count of each − window taken from it;
  - its state is in the read's bytes, its windows left at `[39]` and its sum in `[54..56)` and `[60..64)`.
- **At its end**, with D the sum and c* the read's target count:
  - S = D / (2·(1/64)·8·c*) = 4D/c*;
  - κ = (1/4)/S, written as the step in Q0.16, which is 4 096·c*/D;
  - the gain returns to g₀, and the rate's rule runs from the next window at κ.
- **The probe is refused** when D ≤ 0, or when κ would pass `CONTROL_STEP_MAX_Q0_16`, which is S below 1/2. A refused probe ends as a refused read ends.
- **The order −, +, +, − cancels a count that drifts at a constant rate** through the 16 windows.

### The runs (Specified; brief 078 builds them in the tests)

On each of seeds 22, 23 and 24 at 1 024 units, under `Config::reads_its_targets()` with the probe's flag set:
- **the derivation**: H-41's, the read and every window up to the 128th its twin's, then the probe, then the rule, settled by H-39's two criteria within 320 windows;
- **H-25's two arms** on H-20's schedule from its image.

### H-42 (Hypothesis; written before any run)

The text, with its clauses and its stopping rule, is [H-42's report](../research/H-42.md).
- **Clause 0.** The read and the probe each give a value the rules accept, the lead-in settles, every later rule gives a value, and the frozen block passes both measures.
- **Clauses 1 to 4.** H-25's four, in both arms.
- **Yes** when all five hold on all three networks.

**Predicted: yes.** The step falls from about 1/16 to about 1/60, the loop gain κS from about 0.94 to 1/4.

**Four readings are predicted:**
1. each network's S between 12 and 24;
2. in each of the six arms, fewer than 540 opposite pairs of 959 (ρ above −0.19);
3. in each arm, the mean count a window within 1 % of its read target;
4. through each arm, the gain within 5 % of 1.75 after every window.

**What would refute it:**
- a probe biased by the network's own change after the read, giving an S far from 12 to 24;
- a loop at 1/4 too slow for the arms' drift, the count's mean more than 1 % from its target;
- learning slowed by a count less held, a reversal past 23 blocks.

### H-42's stopping rule

1. **One round**: brief 078 builds the probe with its tests and its ADR, then runs H-42 once. Its clauses, constants and readings are committed before any lead-in with the probe set.
2. **A calibration stops the round**, and is a finding, if a pinned number moves with the flag unset at format 25.
3. **Yes**: the next decision is an ADR that puts the probe in `Config::reads_its_targets()` in step = 1/16's place. Then M10.
4. **No on clause 0, on a network**: the probe does not give a step the network settles under. The next decision is an ADR on how S is read.
5. **Otherwise no**: a step from the engine's own loop learns worse than step = 1/16. The next decision is an ADR on λ.
6. **No constant moves after such a run.** There is no second attempt, no other λ, probe or network.

## Consequences

- Good: the rate's rule keeps no constant of the network's. λ = 1/4 is a property of the loop, not of the network.
- Good: a network much steeper than these gets a smaller step, where step = 1/16 would pass its edge at S = 32.
- Good: a clean twin, every window up to the 128th H-41's.
- Bad: the lead-in is 16 windows longer, and the gain moves ±1.6 % through them.
- Bad: the modulator section's last reserved byte is spent. A next flag needs a section of its own.
- Bad: S below 1/2 is refused. A network the − windows silence reads S low, since a count cannot fall below zero.
- Neutral: the default does not change in this round.

## Alternatives considered and why rejected

- **Option 1(b), a regression**: the pinned slopes fall from 15.7 to 5.8 as the step falls; the estimate is diluted where it is needed.
- **Option 1(c), a smaller fixed step**: it would remove the alternation, but it keeps a constant that depends on the network's S. The maintainers chose the more evolved mechanism on 2026-10-19.
- **Option 2(b), λ = 1/8**: the alternation would go further, and the count's drift would be held half as fast.
- **Option 3(b), 2 048 units**: the read has never run there, so it would ask two things at once.

## Confirmation

`briefs/078_a-step-the-engine-makes-from-its-own-loop.md` builds the probe and runs H-42 once.
[H-42's report](../research/H-42.md) holds the hypothesis as written before any run.
`npm run spec` holds the links, the tables, the brief's contract and scope, and the version.
