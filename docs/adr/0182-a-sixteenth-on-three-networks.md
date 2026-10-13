---
status: accepted
date: 2026-10-13
depends-on: ADR-0181
decision-makers: VirtualCortex maintainers
---

# ADR-0182: A sixteenth, on three networks — H-38's stopping rule taken at the step and the networks: the protocol took the largest step that holds, an eighth, and that is the one step at its loop's edge, the rate changing fifteen per cent for one of gain; a sixteenth's lead-in is quiet and no arm has run under it, and the yes is one network's; so under the rate's rule at a step of a sixteenth, on seeds 22, 23 and 24 at 1 024 units, each with the target its own derivation returns with the gain held, H-25's two arms on each; predicted yes, with what would refute it and four predicted readings; nothing under `src/` changed; H-39; brief 073 runs it once

## Context and Problem Statement

[ADR-0181](0181-a-rate-the-controller-holds-measured.md) read H-38 as yes: with the controller holding the settled network's rate the engine learns and revises. Its stopping rule, step 3, names an ADR on what the engine carries by default, the rule and its step, on the other networks and the other size under it, and on the questions left. **The maintainers took the step and the networks first, on 2026-10-13: a sixteenth, on three networks.**

**One sweep is running**: the one of brief 072's dispatch, run [38417864748](https://github.com/DescentVTT/VirtualCortex/actions/runs/38417864748). It had not ended when this was written; by [ADR-0158](0158-the-sweep-off-the-next-decisions-path.md) the first ADR merged after it ends records it.

**What H-38 read of the step**, on seed 22:
- **Every step of the six holds clause 0**, and the protocol took the largest, an eighth.
- **The rate changes fifteen per cent for one per cent of gain**, where [ADR-0178](0178-a-rate-the-controller-holds.md) had taken nine to fourteen. By ADR-0178's arithmetic a gain that stands off its target by $\delta$ stands at $(1 - \kappa S)\,\delta$ a window later: $-0.88\,\delta$ at an eighth, $+0.06\,\delta$ at a sixteenth.
- **An eighth is at its loop's edge.** Its gain alternates; a window's count ranges from 1 228 to 3 375 in its lead-in and from 1 880 to 2 882 through its arms; and with the weights frozen the alternation grows over the eight windows read, from 182 above the target to 721 below.
- **A sixteenth is well inside it.** Its lead-in's count stays within 2 067 and 2 507, where the gain held's stays within 2 132 and 2 462.
- **No arm has run under a sixteenth.** ADR-0181: *"the readings order the two steps' lead-ins and not their learning."*

**What H-38 read of the learning**, under an eighth: every mapping learned, 118 to 124 of its last 128; every reversal in 15 to 19 blocks; the count held at its target, 2 306 a window, where the gain held's arms hold 2 814 to 2 838; the gain two per cent below its image's, and not moved by a reversal.

**What it read beside**: the inhibitory sum ends at 0.78 of its image's, where the gain held leaves 0.85, and is still falling at the arm's end. ADR-0181 gives an account and did not vary it: the gain holds the population's count at the settled rate while the task has its own units fire more, so the rest fire below the rate the inhibitory rule drives each unit toward.

**The other networks.** H-33 read H-25's configuration on seeds 23 and 24 with the gain held: every mapping learned, the reversals in 12 to 21 blocks. Their derivations return targets of 59 510 and 59 179 ticks, where seed 22's returns 58 201: 2 255, 2 268 and 2 306 spikes a window.

## Decision Drivers

- **The maintainers' choice** (2026-10-13): a sixteenth, on three networks.
- **A default is not chosen from a step no arm has run under**, nor from the one step at its loop's edge.
- **A yes on one network is that network's**, as H-33 and H-36 read of earlier results.
- **Two questions, each with its own comparison**: seed 22 at a sixteenth stands beside seed 22 at an eighth; seeds 23 and 24 at a sixteenth stand beside seed 22 at a sixteenth, and each beside its own arms with the gain held.
- **A round without a build.** The rule is in the tree; no file under `src/` changes.
- **A prediction is written**, with what would refute it.
- Latest ≠ Newest: nothing is adopted.

## Considered Options

1. **The step**: (a) a sixteenth; (b) an eighth, the protocol's; (c) both, on every network.
2. **The networks**: (a) seeds 22, 23 and 24 at 1 024 units; (b) seed 22 alone; (c) those and the two at 2 048 units.
3. **The target on a network**: (a) the period its own derivation returns with the gain held; (b) seed 22's on all three.
4. **The clauses**: (a) clause 0 and H-25's four, on each network; (b) a pooled verdict.

## Decision Outcome

**Options 1(a), 2(a), 3(a) and 4(a).**

### The runs (Specified; brief 073 builds them in the tests)

On each of the three networks, ADR-0044's generator at 1 024 units from seeds 22, 23 and 24:
- **the target**: the period H-25's derivation returns on that network with the gain held, 58 201, 59 510 and 59 179 ticks;
- **the derivation under the rate's rule at a step of a sixteenth**, by ADR-0178's rules: the lead-in settled in its sums and in its gain's mean within 320 windows, then the quiet run, the cancel, the frozen block's two measures, the inhibitory target period and the critic's constants. Seed 22's is the one brief 072's scan pinned;
- **H-25's two arms** on H-20's schedule from that derivation's image, the step and the target carried through every trial.

**Nothing under `src/` changes.**

### H-39 (Hypothesis; written before any run)

**On each of the three networks, with the controller holding that network's settled rate at a step of a sixteenth:**
- **clause 0, the step holds**: the lead-in settles in its sums and in its gain's mean, every later rule gives a value and the frozen block passes both measures;
- **clauses 1 to 4, in both arms**: H-25's four — every mapping learned, at least 80 of its last 128 trials correct; no coupling past 1.30 of its image's; each of the three reversals passing 40 of 64 within 23 blocks of its flip; the excitatory sum outside the four couplings within 0.75 and 1.25 of the image's.

**Yes** when all five hold on all three networks; otherwise **no**, with the network, the arm and the clause.

**Predicted: yes.**
- **The reason.** Under an eighth, at its loop's edge, both arms on seed 22 hold every clause, and a sixteenth's loop leaves a deviation at a sixteenth of itself a window later. With the gain held seeds 23 and 24 learn as seed 22 does, and each network's target is its own settled rate by one rule.
- **What would refute it**: a network whose rate is so much steeper in the gain than seed 22's that a sixteenth is at its edge there; the inhibitory sum, which falls further under the rule, falling far enough on a network to move the couplings; or a sixteenth's slower answer to the task's count letting a reversal's blocks run past twenty-three.

**Four readings are predicted**; a miss is recorded as one:
1. **the step holds on seeds 23 and 24**, as brief 072's scan read it on seed 22;
2. **through each of the six arms the gain stays within five per cent of 1.75** after every window;
3. **a sixteenth does not alternate as an eighth does**: in each arm consecutive windows stand on opposite sides of the target in fewer than 600 pairs of 959, where an eighth's arms read 750 and 758;
4. **the inhibitory sum ends lower under the rule than with the gain held**, in each of the six arms beside the same network's arm with the gain held.

**Readings, no clause:**
- **seed 22 at a sixteenth beside seed 22 at an eighth**: each mapping's last 128, each reversal's blocks, the highest coupling, a window's count and its scatter, the gain's band and mean, and the inhibitory sum's course;
- **each network beside its own arms with the gain held** (H-25's and H-33's): the same, with the value and its troughs;
- per network: the derivation's readings as brief 072's scan read them, the steepness among them;
- over the three networks: the mappings learned of twenty-four, the reversals' blocks, and the inhibitory sum's end under the rule beside its end with the gain held.

### H-39's stopping rule

1. **One round**: brief 073 builds the derivations and the arms in the tests and runs H-39. No file under `src/` changes. The networks, the targets, the step and H-39's constants are committed before any lead-in is run under the rule on seed 23 or seed 24 and before any arm is run at a sixteenth.
2. **A calibration** stops the round before any such run, and is a finding, if a pinned number of the tree moves: H-38's six steps and two arms among them, and seed 22's derivation at a sixteenth the one pinned.
3. **Yes**: the configuration learns and revises under the rate's rule at a sixteenth on three networks. The next decision is an ADR on what the engine carries by default — the rule, the step and a target's rule for a network no derivation has settled — and on the other size and the questions left, a reward delayed past a trial among them, named and not taken.
4. **No on clause 0, on a network**: the step or the target's rule does not transfer. The next decision is an ADR on that network's derivation, with the steepness read.
5. **Otherwise no on seed 22**: a sixteenth learns worse there than an eighth did. The next decision is an ADR on the step, with what the two steps' arms read apart.
6. **Otherwise no on seed 23 or seed 24 alone**: the rule's yes is seed 22's. The next decision is an ADR on what differs, each network beside its own arms with the gain held.
7. **No constant moves after such a run, and there is no second attempt, no other step, no other target and no other network.**

## Consequences

- Good: the step a default would carry is read under the task before it is chosen.
- Good: H-38's yes is asked of the two other networks of the size in the same round, each with its own comparison.
- Good: no build.
- Bad: the size is left. The two networks at 2 048 units are four arms of an hour each, and are the next decision's.
- Bad: one schedule, one generator and one target's rule.
- Bad: the inhibitory sum's fall under the rule is read again and still not varied.
- Bad: six arms and two derivations in the weekly job, about 12 000 s. The table goes from 101 602 s to about 113 600, about 46 per cent of a shard's bound at the last run's ratios.
- Neutral: seed 22's derivation at a sixteenth is already a weekly test.

## Alternatives considered and why rejected

- **Option 1(b), an eighth**: at fifteen per cent of rate for one of gain it is at its edge on seed 22, and on a network a little steeper it does not converge. A no there would not say whether the step or the network read it.
- **Option 1(c), both steps on every network**: twelve arms, six of them for a step a default would not carry.
- **Option 2(b), seed 22 alone**: the cleanest reading of the step, and the networks a round later.
- **Option 2(c), the networks at 2 048 units too**: four more arms of about an hour each before the step is read under the task.
- **Option 3(b), seed 22's target on all three**: a target is a network's own settled rate by its rule; another's is a second change.
- **Option 4(b), a pooled verdict**: the question is whether the yes is one network's.

## Confirmation

`briefs/073_a-sixteenth-on-three-networks.md` runs H-39 once. Whitepaper 4.116.0 carries H-39 in §11.1 with its stopping rule, and §9's row. `npm run spec` holds the links, the rows, the brief's sections and the version.
