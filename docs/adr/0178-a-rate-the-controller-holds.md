---
status: accepted
date: 2026-10-13
depends-on: ADR-0177
decision-makers: VirtualCortex maintainers
---

# ADR-0178: A rate the controller holds — H-37's stopping rule taken at what the controller regulates: its estimate reads near one at no rate and what holds the network is the saturation rule, so the controller is given a second rule, ADR-0036's option 2 reopened: the gain moved by the window's count over a target in the estimate's place, the target a period in ticks of the image and zero unset; ADR-0036 set it aside for want of a canonical target, and H-21 has since given one, the settled network's own rate by the rule that derives the inhibitory target; the controller as built bit for bit where it is unset; six steps scanned under it, an eighth to one in 256, and H-25's two arms under the largest that holds; predicted yes, with the arithmetic and what would refute it; H-38; brief 072 builds the rule and runs it once

## Context and Problem Statement

[ADR-0177](0177-the-controllers-step-measured.md) read H-37 as no at every step, as predicted. Its stopping rule, step 5, names *"an ADR on what the controller regulates, its estimate or a rate"*. **The maintainers took a rate on 2026-10-13.**

**No sweep is owed.** Brief 071's dispatch ran the whole-domain shards alone, and ADR-0174 records the last sweep that ended.

**What H-37 read of the controller as built**, on seed 22 at 1 024 units, over 1 276 windows at seven steps:
- **The estimate reads near one at no rate**: 0 to 0.498 at every rate from 1.6 to 24.4 Hz, and its ceiling from there up. So a controller that rests where its estimate reads one finds no such rate.
- **What holds the network is the saturation rule**, at one spike a unit a bin, 24.4 Hz: half the windows at the ceiling and half below, the gain a step up and a step down in turn, at every step from an eighth to one in 256.
- **The step sets the width of the swing, not the rate.**
- **At that rate the instrument is drowned**: the readouts fire thirteen times what they fire with the gain held, and the stimulus is seen in 46 to 51 trials of 64.

By its guard the controller as built is a rate controller, at a rate nobody chose.

**[ADR-0036](0036-criticality-control.md) considered a rate controller** as its option 2, *"a per-image target rate, the gain moved by the rate error (Turrigiano's rule at the population)"*, and set it aside: *"Option 2 needs a target rate per image with no canonical value; $\sigma = 1$ is canonical. A rate ceiling is kept as the guard for the regime the slope cannot read."*

**Two things have been read since.**
- **$\sigma = 1$ is canonical and the estimate does not find it.** Three ADRs looked for another estimate and left the record as it is ([ADR-0047](0047-second-prior-and-the-estimator.md), [ADR-0051](0051-the-estimator-at-4096-units.md), [ADR-0054](0054-the-causal-count-inside-the-loop.md)).
- **A target rate has a rule.** [ADR-0128](0128-a-target-the-network-fires-at.md) gave the inhibitory rule its target as the settled network's own rate, and H-21 read it as yes. On seed 22 the rule returns a period of 58 201 ticks, 1.72 Hz a unit; on the other networks read, 59 119 to 59 599. Every learning result since stands on it.

**The arithmetic of a rate rule on this network**, from H-37's tables:
- **The target as a window's count.** A period of 58 201 ticks is $1\,024 \times 2^{17} / 58\,201$, 2 306 spikes a window. With the gain held the ninety-six windows hold 2 132 to 2 462.
- **The rate is steep in the gain.** At a step of one in 256 the rate goes from 1.8 to 3.5 Hz as the gain goes from 1.755 to 1.851, and from 3.5 to 5.9 Hz as it goes on to 1.960; at an eighth the second window, at a gain 8.9 per cent above the first's, holds 3.2 times its spikes. The rate changes by nine to fourteen per cent for one per cent of gain, most where the network is quietest.
- **So the step has a bound.** Under a rule that moves the gain by $\kappa$ times the rate's error, a gain that stands $\delta$ from where the rate is on target stands $(1 - \kappa S)\,\delta$ from it a window later, with $S$ that steepness. At an eighth and $S$ of fourteen that is $-0.75\,\delta$: it converges, alternating, and at $S$ of sixteen and more it does not. At a sixteenth it is $+0.1\,\delta$ to $+0.4\,\delta$ over the steepness read.
- **The task adds a fifth to the count.** A trial's volley is 51 spikes and the readouts answer with about twelve more, about 500 a window of eight trials over the 2 306. At that steepness the gain that holds the target with the task running is two to three per cent below the one that holds it without.

**The literature.** A rate held by a multiplicative gain is homeostatic scaling at the population (Turrigiano 2008). Under an external drive such a population rests below criticality by the drive's share of its rate (Zierenberg, Wilting and Priesemann 2018), which is where the instrument reads. That is a prior and not a verdict: the engine's gain is one number for the whole network, its arithmetic is integer, it moves once a window of eight trials, and the task's stimulus is part of the count.

## Decision Drivers

- **The maintainers' choice** (2026-10-13): a rate.
- **A measured need**: the estimate's rule holds this network at its guard, at every step.
- **A target with a rule**, already in the tree and already read.
- **Unset, bit for bit**: the controller as built is pinned by ADR-0053's day, ADR-0077's candidate and H-37's five steps.
- **The same rule, one quantity replaced**: the step, the clamp, the rounding, the gain's bounds and the window stay.
- **One network first**: seed 22 at 1 024 units.
- **A prediction is written**, with its arithmetic and what would refute it.
- Latest ≠ Newest: scaling to a target rate is decades old, in the literature and in practice; the engine's own ADR-0036 weighed it.

## Considered Options

1. **What the controller regulates**: (a) a rate, beside the estimate; (b) another estimate; (c) a gain of each unit's own, §8.8's; (d) nothing, the gain held.
2. **The target**: (a) a period of the image, zero unset; (b) the inhibitory rule's own period, with a flag; (c) a rate chosen for every image.
3. **Its value in H-38**: (a) the settled network's rate with the gain held, the period H-25's derivation returns; (b) a rate chosen; (c) the prior's first window.
4. **The step**: (a) scanned, an eighth and H-37's five; (b) a sixteenth, by the arithmetic; (c) an eighth, as built.
5. **What a step is held to**: (a) the lead-in settled in its sums and in its gain's mean, then the frozen block's two measures; (b) H-37's rule as it was.
6. **The networks**: (a) seed 22 at 1 024 units; (b) every network read.

## Decision Outcome

**Options 1(a), 2(a), 3(a), 4(a), 5(a) and 6(a).**

### The rule (Specified; brief 072 builds it)

- **Once a window, where the estimate's rule moves the gain, the rate's rule moves it by the same form**: $g \leftarrow g\,(1 - \kappa\,\operatorname{clamp}(c / c^{*} - 1,\, -1,\, 1))$, with $c$ the window's spikes and $c^{*}$ the target's count for a window. It is the estimate's rule with the count over its target in the estimate's place: the step $\kappa$, the clamp, the rounding and the gain's bounds are the ones the record has.
- **The target is a period in ticks, a parameter of the image**: a unit's mean interval at the target rate. The executor makes of it the window's count, the units times the window's ticks over the period. **Zero is unset.**
- **Unset, the controller is ADR-0036's**, bit for bit: the estimate, the saturation rule and every run of the tree.
- **Set, the saturation rule is not consulted**: a window above its target lowers the gain by the clamp. The estimate is still computed and kept in the record as a reading; it moves nothing.
- **Where it lives**: the rule in `cortex-homeostasis` beside the estimate's; the composition in the executor. The image's format moves from 21.

### The scan and the arms (Specified; brief 072 builds them in the tests)

On ADR-0044's prior at 1 024 units from seed 22:

- **The target**: the period H-25's derivation returns with the gain held, 58 201 ticks, the settled network's own rate.
- **The steps**: an eighth, a sixteenth, one in 32, one in 64, one in 128 and one in 256, in that order.
- **The settled network under a step**: the lead-in from the prior at the gain's rule's 1.75, the rate's rule on from its first window. It runs until ADR-0055's criterion holds over the excitatory sums, each of sixteen windows within 75 per ten thousand of the sum sixteen windows before, and the gain's mean over those sixteen windows is within 75 per ten thousand of its mean over the sixteen before; or 320 windows have run.
- **Then H-25's rules**, the step and the target carried in the image: the quiet run; the cancel; the frozen block's two measures; the inhibitory rule's target period; the critic's window and constants.
- **A step holds** when every rule gives a value and the frozen block passes both measures. **The step taken is the largest that holds.**
- **The arms**: H-25's two on H-20's schedule from that step's image, the rate's rule on through every trial.

**Why the gain is held to its mean.** ADR-0177 read that ADR-0055's criterion over each window's gain *"reads a gain that alternates as settled"* once the swing fits its tolerance, and under a rate's rule the gain moves every window by the count's own scatter. What the criterion is for is a gain that has stopped drifting, and the mean of sixteen windows beside the mean of the sixteen before reads that.

### H-38 (Hypothesis; written before any run)

**On seed 22 at 1 024 units, with the controller holding the settled network's rate:**
- **clause 0, a step holds**: at one step of the scan at least, the lead-in settles in its sums and in its gain's mean, every later rule gives a value and the frozen block passes both measures, every stimulus unit firing once and the stimulus seen in 56 trials of 64;
- **clauses 1 to 4, at the largest step that holds, in both arms**: H-25's four — every mapping learned, at least 80 of its last 128 trials correct; no coupling past 1.30 of its image's; each of the three reversals passing 40 of 64 within 23 blocks of its flip; the excitatory sum outside the four couplings within 0.75 and 1.25 of the image's.

**Yes** when all five hold; otherwise **no**, with the clause, and for clause 0 what each step returned.

**Predicted: yes.**
- **The reason.** The target is the rate the network already fires at with the gain held at 1.75, so the rule has nothing to correct but the count's scatter and the task's fifth; by the arithmetic above a step of a sixteenth converges at every steepness the tables give, and the gain the task asks for is two to three per cent lower. The instrument is then where it was read.
- **What would refute it**: a steepness of the rate in the gain far above what the tables give, so that the steps that converge are too slow to settle inside their lead-in; the gain's scatter moving the volley or the sight; or the controller's moves through a reversal, where the count changes as the selection does.

**Four readings are predicted**; a miss is recorded as one:
1. **a step of a sixteenth holds**, whatever an eighth does;
2. **under the rule the saturation rule's count is reached in no window** of any lead-in, 32 768 spikes;
3. **through each arm the gain stays within five per cent of 1.75**, 1.6625 to 1.8375, after every window;
4. **the task lowers the gain**: its mean through each arm is below the gain its image carries.

**Readings, no clause**, per step, beside the gain held and beside H-37's steps:
- the gain's course and band, the count's course beside its target, and the window each criterion holds at;
- the steepness of the rate in the gain, as the tables give it;
- the estimate, kept as a reading, by the rate of its window;
- the sums' courses, and the inhibitory rule's target period derived under the rule beside 58 201;
- the frozen block's readings;
- through each arm: the gain's course by block, lowest and highest, and through each reversal; the count by window; H-25's readings, each beside H-25's with the gain held; the inhibitory sum's course beside it.

### H-38's stopping rule

1. **One round**: brief 072 builds the rule, its tests and its ADR, then runs H-38. The rule, the target, the steps, the bound, the criteria and H-38's constants are committed before any lead-in is run under the rate's rule.
2. **A calibration** stops the round before any such run, and is a finding, if a pinned number of the tree moves: with the target unset every run is the run it was, H-37's five steps and ADR-0077's candidate among them.
3. **Yes**: the engine learns and revises with its controller on. The next decision is an ADR on what the engine carries by default, the rule and its step, on the other networks and the other size under it, and on the questions left, a reward delayed past a trial among them, named and not taken.
4. **A step holds and clause 1, 2, 3 or 4 fails**: the instrument reads under the rule and the learning does not hold there. The next decision is an ADR on what the controller's moves do to it, with the gain's course through the arms.
5. **No step holds**: the rate's rule does not hold this network where the instrument reads. The next decision is an ADR on why, with the steepness read and what each step returned.
6. **No constant moves after a lead-in is run under the rate's rule, and there is no second attempt, no other target, no other step and no other network.**

## Consequences

- Good: the engine gets a controller that is asked to hold the regime every learning result was read in.
- Good: the target is derived by a rule the tree already holds and has read, on five networks.
- Good: the rule is the one the record has, with one quantity replaced; unset, nothing moves.
- Good: ADR-0036's objection, that a target rate has no canonical value, is answered by a rule the tree has since read.
- Bad: a rule in a state crate and a format of the image, on a prediction. If it reads no, the rule stays in the tree unset, as the whole punishment and the exploration do.
- Bad: one network and one size. A yes is seed 22's until the rules are run under it elsewhere.
- Bad: the controller then holds a rate and no longer looks for criticality. What the engine's regime should be is still the instrument's, not an argument's.
- Bad: two rules on one quantity. The inhibitory rule drives each unit toward the same rate, slowly; the gain holds the population at it, fast. What they do to each other through a run is a reading here.
- Bad: a build under `src/` and in a state crate, so the dispatch runs the sweep after the shards.
- Neutral: the weekly job grows by six derivations and two arms.

## Alternatives considered and why rejected

- **Option 1(b), another estimate**: three ADRs, and H-37's 1 276 windows.
- **Option 1(c), a gain of each unit's own**: §8.8 writes it into bytes of the unit record that ADR-0131 has since given the critic's weight. It is a layout decision first and a rule on every spike's path, where the population's rule costs one division a window.
- **Option 1(d), the gain held**: the engine would carry a controller that no learning result can be read under.
- **Option 2(b), the inhibitory rule's own period with a flag**: one parameter for two rules that H-38 may read apart.
- **Option 2(c), a rate chosen for every image**: ADR-0036's objection, which a derived target answers.
- **Option 3(b), a rate chosen**: the same. **Option 3(c), the prior's first window**: one window's count, 2 446, where the rule's is a settled network's over $2^{20}$ ticks.
- **Option 4(b), a sixteenth by the arithmetic**: the arithmetic rests on a steepness read from lead-ins whose weights were falling, and a scan costs six lead-ins. **Option 4(c), an eighth as built**: by the same arithmetic it alternates, and past a steepness of sixteen it does not converge.
- **Option 5(b), H-37's rule as it was**: ADR-0177 read what it cannot tell.
- **Option 6(b), every network read**: ten more arms before one has read.

## Confirmation

`briefs/072_a-rate-the-controller-holds.md` builds the rule and runs H-38 once. Whitepaper 4.114.0 carries H-38 in §11.1 with its stopping rule, and §9's row. `npm run spec` holds the links, the rows, the brief's sections and the version.
