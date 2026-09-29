---
status: accepted
date: 2026-09-29
depends-on: ADR-0126
decision-makers: VirtualCortex maintainers
---

# ADR-0128: A target the network fires at — with the rule held by the network paused, the learning line takes the inhibitory drain among H-20's four open questions: the inhibitory rule's target rate, a parameter of the image left at its default of 5 Hz while the settled network fires at about 1.7, is set to the settled network's own rate, and H-21 asks before any run whether H-20's schedule then keeps its inhibition and still learns every mapping; brief 054 runs it once, nothing of the engine changed

## Context and Problem Statement

[ADR-0126](0126-the-gate-raised-measured.md) paused the rule held by the network, and the next decision returned to H-20's four open questions: the operating regime, another size, the inhibitory drain and a critic of the engine's own ([ADR-0110](0110-a-schedule-of-reversals-measured.md)). **The maintainers took the inhibitory drain on 2026-09-29.**

**The drain, as measured.** Under the learning configuration, every inhibitory synapse consolidates under a baseline of its own, 0.5 ([ADR-0085](0085-inhibition-off-the-reward-gate.md), [ADR-0086](0086-the-inhibitory-baseline-built.md)), and the network's inhibitory sum falls:
- 0.41 of the settled image's under H-15 and H-16 ([ADR-0083](0083-plasticity-everywhere-measured.md), [ADR-0087](0087-inhibition-off-the-reward-gate-measured.md));
- 0.17 under H-17 and 0.11 under H-18;
- a tenth under H-19;
- under H-20 it fell to 0.120 and 0.122 by the second flip and levelled at about **0.072 and 0.078** from the 111th block ([ADR-0110](0110-a-schedule-of-reversals-measured.md)).

A network with a fourteenth of its inhibition is not the network the learning started on. [ADR-0117](0117-the-region-measured.md) read its members' background 13 per cent faster on the image H-20's arm leaves.

**What the rule is told to do**, read on 2026-09-29:
- **The rule.** The inhibitory rule is Vogels et al. (2011)'s ([ADR-0049](0049-dale-principle-in-plasticity.md), [ADR-0053](0053-the-waking-day-and-the-target-period.md)). At an inhibitory synapse onto a target unit it loses $\alpha$ at every presynaptic spike and gains $A_+ f(|\Delta t|)$ at each pairing. It balances where the target fires at the rate $\rho_0$ that $\alpha = 2 \rho_0 \tau A_+$ names: above that rate the inhibition onto the target grows, below it weakens ([ADR-0057](0057-the-inhibitory-rule-from-below-the-rail.md)).
- **The target.** $\rho_0$ is a parameter of the image, the target period (`Config::istdp_target_period_ticks`, the modulator section's `[20..24)`). Its default is `ISTDP_TARGET_PERIOD_TICKS`, 20 000 ticks: **5 Hz**. The harness sets no other, so every learning run from H-15 on ran at 5 Hz.
- **The network's rate.** The settled network fires at about **1.7 Hz** a unit: ADR-0117's backgrounds read the rest at 1.68. So nearly every target sits below the rule's rate, and the rule weakens the inhibition onto it at every consolidation.
- **The rail.** The prior writes every inhibitory weight at the rail, one LSB above −1.0 ([ADR-0120](0120-the-contexts-own-inhibition-measured.md)). **From the rail the rule can only weaken inhibition** (ADR-0053: "the inhibitory weights only fall"). ADR-0057 read the rule from below the rail and chose no target: "no target chosen".

So the drain is at least in part the rule doing what it is told, pulling every unit toward 5 Hz by the only move the rail leaves it. A target at the network's own rate tells it something else:
- a unit at or above that rate keeps its inhibition, since the rail allows no more;
- a unit below it loses inhibition until it rises to the rate.

The drain should then stop where the units sit at the target, rather than at a fourteenth of the image's. That is the account; H-21 asks whether it holds, and whether learning survives it.

## Decision Drivers

- **A measured need**: the inhibitory sum at 0.07 to 0.08 of the image's under H-20, and a network measurably more excitable for it.
- **The maintainers' choice**: the inhibitory drain first among H-20's four.
- **Nothing of the engine changes.** The target is a parameter of the image (ADR-0053), and H-20's configuration, schedule, arms and critic are the tree's.
- **One change at a time.** Only the target moves; the rail and the rule stay.
- **H-20's own clauses** stay the measure of learning, so a yes cannot buy the inhibition with the learning.
- **The weekly job's budget**: after [ADR-0127](0127-the-paused-line-leaves-the-weekly.md) the plan is about a quarter of the bound, and two of H-20's arms cost about 3 500 s.

## Considered Options

1. **The target**: (a) the settled network's own rate, read by a rule written before any run; (b) a grid of targets; (c) a target below every unit's rate, under which the rule would never move a weight at the rail.
2. **The start**: (a) the prior's inhibition at the rail, as every learning run has had it; (b) inhibition started below the rail, as ADR-0057 read it, so that the rule can move both ways.
3. **The schedule**: (a) H-20's, 7 680 trials with three flips, both arms; (b) H-16's single mapping.

## Decision Outcome

**Options 1(a), 2(a) and 3(a).**

### The target (a parameter of the image; nothing of the engine changes)

- **Read by a rule written before any rewarded run**: the settled image's population rate under ADR-0044's drive, every unit counted, over a lead-in of $2^{20}$ ticks with every weight frozen. The target period is 100 000 ticks, one second at the 10 µs tick, over that rate in hertz, rounded to the nearest tick and within ADR-0053's bounds. By ADR-0117's reading it will be about 59 000 ticks, about 1.7 Hz; the rule, not that estimate, sets it.
- **Written into the image** H-20's arms decode, whose modulator section outranks the configuration (ADR-0053). Every other byte is H-20's.

### H-21 (Hypothesis; written before any run)

**On H-20's configuration, schedule and arms, with the inhibitory rule's target at the settled network's rate:**
- **clause 1, the drain stops**: in both arms, the network's inhibitory sum stays at or above **0.5** of the settled image's at every block's end;
- **clause 2, the learning holds**: in both arms, every mapping is learned (at least 80 of its last 128 trials correct) and no stimulus–readout coupling passes 1.30 of its image's at any block's end, as H-20's.

**Yes** when both hold in both arms; otherwise **no**, with the clause that failed. No prediction is made for the verdict.

**Readings, no clause**:
- the inhibitory sum and the network's rate at every block's end;
- the fraction of units at the target by ADR-0057's rule, at the start and at every flip;
- the reversal speeds (blocks to 40 of 64) beside H-20's 13 to 23;
- the settled network's rate distribution before the first trial, and so the share of units at or above the target, whose inhibition the rail keeps.

### H-21's stopping rule

1. **One round**: brief 054, the target's rule and H-21's constants committed before the first rewarded run, then the runs.
2. **A calibration** that does not reproduce ADR-0077's settled image, or a pinned number of an earlier test that moves, stops the round before any rewarded run and is a finding.
3. **Yes**: the learning configuration is named with the target, and the next decision is an ADR choosing among H-20's other three open questions — the operating regime, another size, a critic of the engine's own — named and not taken.
4. **No, clause 1 failed** (the drain did not stop at half): the drain is not the target's alone. The next decision is an ADR on the inhibitory rule's start (inhibition below the rail, ADR-0057) or its form, with this round's readings as its need.
5. **No, clause 2 failed** (the inhibition held and the learning did not): the target costs the learning. The next decision is an ADR weighing the two, with this round's readings as its need.
6. **No constant moves after a rewarded run, and there is no second attempt.**

## Consequences

- Good: the drain is asked at its cause, a parameter the tree has carried since ADR-0053 and never chosen, with nothing of the engine changed.
- Good: H-20's clauses stand beside the new one, so the inhibition cannot be kept at the learning's cost unread.
- Bad: from the rail the rule still only weakens. A unit below the target loses inhibition until it rises to it, so a clause of one half may still fail, and the fix would then be the start, not the target.
- Bad: one rate for every unit. The readouts fire above the network's mean during trials and the rest below it, and the rule acts per target.
- Neutral: two of H-20's arms in the weekly job, about 3 500 s, inside the budget ADR-0127 freed.

## Alternatives considered and why rejected

- **Option 1(b), a grid of targets**: one change at a time. A no on clause 1 names the start before another target.
- **Option 1(c), a target below every rate**: the rule would never move a weight at the rail, which is the inhibitory baseline switched off by another name. H-14 read learning without inhibitory plasticity already.
- **Option 2(b), inhibition below the rail**: it changes the settled image every learning reading starts from, so none of H-16 to H-20 would be the baseline. It is the next step if clause 1 fails.
- **Option 3(b), H-16's single mapping**: the drain's reading that matters is H-20's, across flips, and H-20's clauses are the learning's measure.

## Confirmation

`briefs/054_a-target-the-network-fires-at.md` runs H-21 once. Whitepaper 4.77.0 carries H-21 in §11.1 with its stopping rule, and §9's row. `npm run spec` holds the links, the rows, the brief's sections and the version.
