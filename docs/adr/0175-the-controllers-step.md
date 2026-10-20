---
status: accepted
date: 2026-10-13
depends-on: ADR-0174
title: The controller's step
kind: protocol
hypothesis: H-37
summary: 'The line under an unreliable reward is closed with no rule of the engine changed, and the question is asked of the criticality controller''s step: five steps from a sixteenth to one in 256. H-37 is predicted no at every step.'
decision-makers: VirtualCortex maintainers
---

# ADR-0175: The controller's step — H-36's stopping rule taken: the line under an unreliable reward closed with what the configuration holds stated and no rule of the engine changed, the readings by kind not ordering the candidates; then the operating regime, the maintainers' choice: the tree already holds that under the criticality controller at its step of an eighth the instrument is drowned, so the question is asked of the step — five steps from a sixteenth to one in 256, the controller on from the lead-in to the arms, the first at which the lead-in settles in its sums and in its gain and the frozen block passes both measures, then H-25's two arms under it; predicted no at every step, with the reason and what would refute it; nothing under `src/` changed; H-37; brief 071 runs it once

## Context and Problem Statement

[ADR-0174](0174-three-in-four-on-the-other-networks-measured.md) read H-36 as no: at three in four the no is not one network's. Its stopping rule, step 5, names an ADR on a single error's weight and the gate's clamp. This is that ADR. **The maintainers chose on 2026-10-13 to change no rule of the engine for it, to have what the configuration holds written down, and to take the operating regime next.**

**No sweep is owed.** ADR-0174 records the schedule's, and brief 070's dispatch ran none.

### What the learning configuration holds of an unreliable reward

Three rounds read it, each on H-25's configuration and schedule:

| Round | The reward right | Networks | Mappings learned | Reversals crossed inside their span | A reversal's blocks, of the network's own under the true reward |
| :--- | :--- | :--- | ---: | ---: | ---: |
| H-28 | seven times in eight | seed 22 | 8 of 8 | 6 of 6 | 1.13 |
| H-35 | three times in four | seed 22 | 7 of 8 | 5 of 6 | 1.42 |
| H-36 | three times in four | seeds 23 and 24 | 8 and 6 of 8 | 6 and 5 of 6 | 1.31 and 1.42 |

- **A reward right seven times in eight is held**: every mapping learned and revised, the couplings bounded, the network held, the critic holding three quarters of $2p - 1$.
- **At three in four the first mapping is learned on every network, in all six arms**, and the critic holds a half of $2p - 1$ in all forty-eight cells, with the couplings bounded and the network held.
- **At three in four a revision over three reversals does not hold inside the schedule**: twenty-one mappings of twenty-four learned and sixteen reversals of eighteen crossed. Two networks of three fail, each by mappings a few trials short that were rising at their end.
- **The boundary** for a revision inside H-20's schedule lies between the two shares.
- `Feedback::ThreeInFour` stays in the tree, a feedback of the task; the six arms at it stay weekly tests.

**Why no rule of the engine is changed.** ADR-0174 says of the two candidates that its readings *"do not order"* them:
- the arm that lost most to misleading punishments, 53 per cent, failed, and the next, 51, learned every mapping;
- the learning signal does not tell a mapping that failed from one that held;
- the gate's ceiling is met by a true reward in 38 to 47 per cent of them on every network, the one that learned all eight among them;
- the one reading that stands apart in the mappings that failed, a misleading reward's move of the old answer's pair, may be their effect.

The three mappings that failed stand at the same two places of the schedule, in one arm, with stimulus B the one not turned, and the same trials mislead on every network. The coin's own rule does not settle it: over the last 128 trials of the second mapping it misleads 20 of B's 64 presentations and 18 of A's 64, and over the fourth's 13 of B's 74 and 22 of A's 54. That is computed here from `Task::misleading_in_four_at` at seed 27, which returns the tree's 1 913; it is no table of the tree.

**What would reopen the line**: a second seed of the task, which tells the sequence's part from the network's; or a reading that orders the candidates.

### The operating regime, and what the tree already holds

Every learning result was read with the gain held, at the 1.75 a calibration picked, where the population fires at 1.7 Hz. The engine's criticality controller ([ADR-0036](0036-criticality-control.md)) was off. Once a window of $2^{17}$ ticks, eight trials, it moves the gain by $g \leftarrow g\,(1 - \kappa\,\operatorname{clamp}(\hat\sigma - 1, -1, 1))$, with $\hat\sigma$ the lag-one slope of the window's thirty-two bins and $\kappa$ its step; a window at one spike a unit a bin or more, 24.4 Hz, is read as saturated, $\hat\sigma$ at its ceiling of 16.

**Under the controller at a step of an eighth the instrument is drowned.** [ADR-0077](0077-the-background-side.md) read it on seed 22 at 1 024 units, and the tree pins it (`BACKGROUND_LEAD_IN_1024`, `BACKGROUND_MEASURES_1024`; `the_controller_at_1024_units_exhaustive`):

| Window | The gain it ran at | Spikes | A unit | The estimate | The gain after |
| ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 1.750 | 2 446 | 1.8 Hz | 0.287 | 1.906 |
| 2 | 1.906 | 7 918 | 5.9 Hz | 0.409 | 2.047 |
| 3 | 2.047 | 13 799 | 10.3 Hz | 0.339 | 2.216 |
| 4 | 2.216 | 22 710 | 16.9 Hz | 0.390 | 2.385 |
| 5 | 2.385 | 31 858 | 23.7 Hz | 0.258 | 2.606 |
| 6 | 2.606 | 44 560 | 33.2 Hz | 16, saturated | 2.280 |
| 7 | 2.280 | 21 427 | 16.0 Hz | 0.000 | 2.565 |
| 8 | 2.565 | 39 315 | 29.3 Hz | 16, saturated | 2.245 |

- **The estimate never reads near one.** From 1.8 to 23.7 Hz it reads 0.26 to 0.41; over the thirty-five windows it reads 0.00 to 0.41 or its ceiling, in fourteen of them, and nothing between; and with the gain held at 1.75 it reads 0.00 to 0.45 over ninety-six windows. So the controller raises the gain until the saturation rule fires, and from the sixth window the gain alternates between 2.21 and 2.80 and the rate between 12 and 41 Hz.
- **The frozen block fails both measures** there: the volley is 48.4 of 51 units a presentation, the stimulus is seen in 51 trials of 64 where 56 are asked, and the readouts fire seven to twenty-four times the 0.28 spikes a trial they fire with the gain held.
- **Three ADRs looked for another estimate and left the record and the controller as they are**: [ADR-0047](0047-second-prior-and-the-estimator.md), [ADR-0051](0051-the-estimator-at-4096-units.md) and [ADR-0054](0054-the-causal-count-inside-the-loop.md).

**H-25's rules with the controller on are that candidate.** The derivation's settled network is `candidate_from` at a step, and its cancel is derived at the gain's rule's 1.75 on the prior's network. At a step of an eighth on seed 22 it is ADR-0077's second candidate through the frozen block, which fails. **A round that turned the controller on at its step would reproduce a pinned no.**

**What the step has not been asked.** ADR-0077 took ADR-0055's eighth, *"the step already measured on this network size and not searched"*. A smaller step moves the gain less a window. Whether it changes where the controller holds the network, or only how far it swings about it, has not been read.

## Decision Drivers

- **The maintainers' choices** (2026-10-13): the line under an unreliable reward closed; the operating regime next, asked of the controller's step.
- **A rule of the engine is not changed where the readings do not say which.**
- **A pinned result is not run again as a question.**
- **A lever that costs no build is read before a rule is changed**: the step is a parameter of the image.
- **One network**: seed 22 at 1 024 units, where ADR-0077's two candidates are pinned.
- **A round without a build.** No file under `src/` changes.
- **A prediction is written**, with its reason and what would refute it: the tables above give one.
- Latest ≠ Newest: the controller is ADR-0036's, run as built; nothing is adopted.

## Considered Options

1. **The line under an unreliable reward**: (a) closed, with what it holds stated; (b) a second seed of the task; (c) the gate's clamp widened; (d) a smaller weight on one error.
2. **The operating regime**: (a) the controller's step scanned; (b) the controller on at its step of an eighth; (c) a controller that regulates a rate; (d) another gain held; (e) not asked.
3. **The steps**: (a) five, by halving from a sixteenth to one in 256; (b) one smaller step; (c) a finer scan.
4. **What a step is held to**: (a) the lead-in settled in its sums and in its gain, then the frozen block's two measures; (b) the frozen block's measures alone.
5. **The arms**: (a) H-25's two under the largest step that holds, the controller on throughout; (b) none in this round.

## Decision Outcome

**Options 1(a), 2(a), 3(a), 4(a) and 5(a).**

### The scan (Specified; brief 071 builds it in the tests)

On ADR-0044's prior at 1 024 units from seed 22, for each step of the scan, the controller on from the lead-in's first window:

- **The steps**: a sixteenth, one in 32, one in 64, one in 128 and one in 256, in that order. An eighth is ADR-0077's. One in 256 is the smallest halving that can carry the gain from 1.75 to its ceiling of 4.0 inside the lead-in's bound, $(1 + \kappa)^{320} \ge 4.0 / 1.75$.
- **The rules before the settled network are H-25's**, and do not read the step: the rotation, and the gain's rule, whose 1.75 is the gain the lead-in starts at.
- **The settled network under a step**: the lead-in runs until ADR-0055's criterion holds over the excitatory sums and over the gains, each of sixteen windows within 75 per ten thousand of the value sixteen windows before, or 320 windows have run. The rule returns the window, or nothing.
- **Then H-25's rules, the step carried in the image**: the quiet run; the cancel, derived as it is; the frozen block's two measures, every stimulus unit firing once and the stimulus seen in 56 trials of 64; the target period; the critic's window and constants.
- **A step holds** when every rule gives a value and the frozen block passes both measures. **The step taken is the largest that holds.**
- **The arms**: where a step is taken, H-25's two arms on H-20's schedule from its image, the controller on at that step through every trial.

**Nothing under `src/` changes.** With the step at zero the derivation is seed 22's, and at an eighth its lead-in and frozen block are ADR-0077's second candidate's.

### H-37 (Hypothesis; written before any run)

**On seed 22 at 1 024 units, with the criticality controller on:**
- **clause 0, a step holds**: at one step of the scan at least, the lead-in settles in its sums and in its gain, every later rule gives a value and the frozen block passes both measures;
- **clauses 1 to 4, at the largest step that holds, in both arms**: H-25's four — every mapping learned, at least 80 of its last 128 trials correct; no coupling past 1.30 of its image's; each of the three reversals passing 40 of 64 within 23 blocks of its flip; the excitatory sum outside the four couplings within 0.75 and 1.25 of the image's.

**Yes** when all five hold; otherwise **no**, with the clause, and for clause 0 what each step returned.

**Predicted: no, on clause 0, at every step.**
- **The reason.** The estimate reads under a half at every rate the tables hold below the saturation rule's, so at any step the controller raises the gain until that rule fires, at 24.4 Hz. A smaller step narrows the swing about that rate and does not lower it. At a background of that order the volley is short and the stimulus is not seen, as at an eighth.
- **What would refute it**: a window of a lead-in whose estimate reads between a half and two. An estimate near one below saturation would stop the gain there, and nothing in the tables says none exists at a gain that is nearly still.

**Three readings are predicted**; a miss is recorded as one:
1. **in every lead-in the saturation rule fires**, the estimate at its ceiling in one window at least;
2. **no window of any lead-in reads an estimate between a half and two**;
3. **every lead-in ends loud**: its last window holds more than five times the 2 308 spikes the gain held leaves in its ninety-sixth.

**Readings, no clause**, per step, beside the gain held and beside an eighth:
- the gain's course: the window it first falls in, and the band it then keeps;
- the rate's course and the windows the saturation rule fires in;
- **the estimate by the rate of its window**, over every window of every lead-in;
- the excitatory and the inhibitory sum's courses, and the window each criterion holds at;
- the frozen block: the volley's units a presentation, the readouts' spikes before the injection and after the volley, the trials seen, and ADR-0072's sign;
- where arms run: the gain's course through the arm, the windows the saturation rule fires in, and H-25's readings.

### H-37's stopping rule

1. **One round**: brief 071 gives the derivation its step and runs H-37. No file under `src/` changes. The steps, the bound, the criterion and H-37's constants are committed before any lead-in is run under a step of the scan.
2. **A calibration** stops the round before any such run, and is a finding, if a pinned number of the tree moves: with the step at zero the derivation returns seed 22's constants; and at an eighth the lead-in it runs is ADR-0077's second candidate's, window for window, and the frozen block from that candidate's thirty-fifth window reads the measures ADR-0077 pinned.
3. **Yes**: the configuration learns and revises with the engine's controller on. The next decision is an ADR on the step the engine carries and on the questions left, a reward delayed past a trial and a larger size, named and not taken.
4. **A step holds and clause 1, 2, 3 or 4 fails**: the instrument can be read under the controller and the learning does not hold there. The next decision is an ADR on what the controller's moves do to it, with the gain's course through the arms.
5. **No step holds** (predicted): the step is not the lever. The next decision is an ADR on what the controller regulates, its estimate or a rate, with this round's readings of the estimate by rate and of the regime each step keeps. It is named and not taken.
6. **No constant moves after a lead-in is run under a step of the scan, and there is no second attempt, no other step and no other network.**

## Consequences

- Good: the line under an unreliable reward ends with its boundary stated, three rounds after it began, and no rule of the engine changed on readings that do not say which.
- Good: the regime is asked of a lever that needs no build, before a rule of the controller is changed.
- Good: a pinned no is not run again. The round begins where ADR-0077 ended.
- Good: whichever way it reads, the estimate by rate over five lead-ins is what a decision on the controller needs.
- Bad: the prediction is a no. If it holds, the round's yield is its readings.
- Bad: one network, one size and one generator.
- Bad: the gate's clamp and a single error's weight are named and not taken. What a misleading reward costs is read, and not what would lessen it.
- Bad: five derivations in the weekly job, each a lead-in of up to 320 windows, and two arms if a step holds.
- Neutral: the weekly job's bound is [ADR-0176](0176-a-shards-bound-by-its-longest-test.md)'s, in the same pull request.

## Alternatives considered and why rejected

- **Option 1(b), a second seed of the task**: a third round at one share, six more arms, and by either reading the configuration stands at an edge. It is what would reopen the line.
- **Options 1(c) and 1(d), the clamp or a single error's weight**: each a rule of the engine and a parameter of the image, and the readings do not say which, or that either helps.
- **Option 2(b), the controller on at an eighth**: pinned, and a no.
- **Option 2(c), a controller that regulates a rate**: a rule of a state crate and a format of the image, before the lever that costs neither is read. It is step 5's decision.
- **Option 2(d), another gain held**: it asks whether the configuration holds at another working point, and leaves the controller where it is.
- **Option 2(e), not asked**: the engine is to run with its controller on, and no learning result has it on.
- **Option 3(b), one smaller step**: a no at one step would leave the others as an objection. **Option 3(c)**: more lead-ins between steps whose reading the tables do not suggest differs.
- **Option 4(b), the frozen block alone**: eight windows can pass before a gain still rising has reached where the controller holds it.
- **Option 5(b), no arms in this round**: a step that holds would wait a round for the question it was scanned for.

## Confirmation

`briefs/071_the-controllers-step.md` runs H-37 once. Whitepaper 4.112.0 carries H-37 in §11.1 with its stopping rule, and §9's row. `npm run spec` holds the links, the rows, the brief's sections and the version.
