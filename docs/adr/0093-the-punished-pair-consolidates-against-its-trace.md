---
status: accepted
date: 2026-09-24
depends-on: ADR-0091
decision-makers: VirtualCortex maintainers
---

# ADR-0093: The punished pair consolidates against its trace — of the two mechanisms H-17's stopping rule named, the depression under the gate is chosen: a signed gate, a parameter of the image, under which an addressed excitatory synapse consolidates under `clamp(baseline + dopamine, −1, 1)`, so that a punishment moves its weight against its trace's sign and spends the trace by as much; written as H-18 before any run — H-17's configuration with the signed gate set and a second mapping twice as long, the length derived from ADR-0091's readings; the exploration named for after it

## Context and Problem Statement

[ADR-0091](0091-the-assignment-reversed-measured.md) read H-17 no, as [ADR-0089](0089-the-assignment-reversed.md) predicted: an engine that learned the answer in 128 of the last 128 trials before the flip selected the new answer in 0 of the last 128 after it, in both arms. H-17's stopping rule names the next decision: **an ADR choosing between an exploration in the selection — a gate that sometimes takes the action it does not prefer — and a depression under the gate — a rule by which a punished or unrewarded pair loses what it gained — with this round's readings as the measured need each must meet.** This is that ADR. It chooses and builds nothing.

ADR-0091 wrote the need as arithmetic on its readings:

- The deterministic gate crossed under the counts' own variation **three times in the 3 072 trials after the two flips**, the counts' gap at 16 to 24 spikes a trial.
- **A single rewarded selection of the new answer consolidated 162 to 3 415** into its pair, against **a lead of 2.2 to 3.0 million** the first half built from about six hundred rewards per stimulus.
- **An exploration alone** would have to supply rewarded selections of the new answer on the order of the first half's rewards before the counts can cross.
- **A depression under the gate** would act on **a trace of +5 to +10 per synapse per trial** in the old pair that a punished trial already addresses, in each of the **1 534 punished trials**.

The engine, read for this decision (`crates/cortex-core/src/dynamics/synapse.rs`, `crates/cortex-neuromod/src/lib.rs`, `runtime/cortex-runtime/src/executor.rs`):

- `NeuromodulatorState::modulation` returns `clamp(baseline + dopamine, 0, 1)` — "there is no anti-Hebbian reversal: a dip below zero is clamped, as in Izhikevich 2007". Under the gate the baseline is zero, so after a punishment the addressed pair consolidates under zero: it is frozen, not depressed.
- `SynapseBlock::consolidate` clamps the modulation to `[0, 1]`, moves `round(|trace| × m)` into the magnitude with the trace's sign, clamps the magnitude to `[0, 2^15)`, and **takes what the magnitude absorbed out of the trace, so that the magnitude and the trace conserve their sum**.
- `Modulations::of` publishes the addressed modulation from `modulation(baseline)`, the one at rest from the baseline alone, and — since [ADR-0086](0086-the-inhibitory-baseline-built.md) — the inhibitory one; `for_synapse` picks by address and polarity.

## Decision Drivers

- H-17's stopping rule, applied as written: the choice is between its two mechanisms, with ADR-0091's need.
- **One variable** against H-17, so that the answer attributes.
- A mechanism is an ADR first; a parameter that changes what a run does is part of the image (§8.3); a record's bytes moving moves the format.
- Unset, every pinned number of the tree and the determinism pin stand, bit for bit.
- Latest ≠ Newest (§2.1): reward-modulated STDP with a signed reward is Florian (2007) and Frémaux, Sprekeler and Gerstner (2010) — the form whose failure mode, the unsupervised bias of a modulation that is not mean-free, [ADR-0083](0083-plasticity-everywhere-measured.md) already met in this tree.

## Considered Options

1. **A depression under the gate**: a signed gate under which an addressed excitatory synapse consolidates under `clamp(baseline + dopamine, −1, 1)`.
2. **An exploration in the selection**: a gate that takes the action it does not prefer with some probability, drawn from the trial's index and a seed.
3. **Both at once.**
4. **A reward-prediction error** in the delivery, the reward less its expectation.

## Decision Outcome

**Option 1.**

- **Why the depression.** ADR-0091's numbers: every one of the 1 534 punished trials after the flip already addresses the old answer's pair, whose trace holds +5 to +10 per synapse per trial; a depression puts every one of those trials to work against the lead. An exploration must first make the engine select the new answer and then be rewarded for it, about six hundred times against a lead that one reward moves by 162 to 3 415 — at a tenth of the trials that is thousands of trials of pure exploration, and a stochastic selection is the larger change to the engine.
- **Why not both** (option 3): two variables; a yes could not say which did it.
- **Why not the reward-prediction error** (option 4): the error changes the reward's size and mean, not the floor at zero the modulation is clamped to, so a punished pair under the gate is frozen with or without it. It is named again for after this round, where it bears on where the rise stops.

**The rule, decided here; its placement the round's.** A new parameter, **the signed gate**: when it is set, an **addressed excitatory** synapse consolidates under `m = clamp(baseline + dopamine, −1, 1)`, and:

- for `m ≥ 0` the rule is today's, bit for bit — `round(|trace| × m)` into the magnitude with the trace's sign, what the magnitude absorbed taken out of the trace;
- for `m < 0` the weight moves **against** the trace's sign by `round(|trace| × |m|)`, clamped as today to `[0, 2^15)` of the magnitude, and **the trace is spent toward zero by what the magnitude moved** — so a punished consolidation can never grow the trace it reads. Leaving the trace unchanged would consolidate it again at every presynaptic spike of the trial, and the conservation rule, applied to a negative transfer, would grow the trace as the weight falls; both would run away.

Every other synapse is as today: an unaddressed excitatory synapse under the baseline alone, an inhibitory one under its own baseline when that is set ([ADR-0085](0085-inhibition-off-the-reward-gate.md)). When the signed gate is unset, every synapse consolidates as today. The parameter changes what a run does, so it is part of the image: the round places it (the modulator section's reserved bytes are `[25..28)` and `[32..64)` after ADR-0086), moves the image format from 15 to 16, and updates §5.2's table and the changelog. `SynapseBlock::consolidate`'s callers all pass a modulation in `[0, 1]` today, so extending its domain to `[−1, 1]` changes none of them. The round records the placement in its own ADR.

**What the signed rule does that a reading must show.** A synapse whose trace is negative — a net-depression pairing, which ADR-0077 read in eleven trials of sixty-four on the settled network — is **potentiated** by a punishment under this rule, the product of two negatives. That is the signed rule as Florian and Frémaux et al. wrote it, not a defect, and H-18 reads how often it happens.

**H-18 (a punished pair loses what it gained, and the engine revises)**, written into whitepaper §11.1 as a Hypothesis with the following, all fixed here:

- **The network and the calibration.** ADR-0077's settled engine as ADR-0087 and ADR-0091 build it, the excitatory baseline zero, the inhibitory baseline 0.5, **the signed gate set**; the calibration a frozen first block from the image with every one of the three parameters unset, held to ADR-0077's frozen run; a mismatch — or any pinned number that moves with the signed gate unset — stops the round before any rewarded run and is a finding.
- **The constants.** H-17's — ADR-0076's stimulus, the geometry, `WINDOW`, `TRIAL_TICKS`, `SEED`, `REWARD_Q16` (1.0), `BLOCK`, `LAST_BLOCKS`, `REWARDED_MIN` (80), the gain 1.75, the controller off, the task as built, the flip between trial 1 536 and 1 537 by `mirrored` — and **the run 4 608 trials, seventy-two blocks, the second mapping 3 072 trials long**.
- **Why 4 608, derived before any run from ADR-0091's readings.** The lead is 2.2 to 3.0 million; a punished trial addresses the old pair's trace of +5 to +10 per synapse per trial over its ~800 synapses, under a modulation at −1 for more than half the trial and not above about −0.7 at its end (the signal's fixed point under a punishment every trial, −1.712 after the punishment and −0.712 at the trial's end, the mirror of the one ADR-0079 read under a reward every trial; `decay_dopamine` reads the magnitude only), so it moves about 2 800 to 8 000 per trial: **about 300 to 1 100 trials at the starting rate**. The rate falls as the pair falls — the volley's potentiation follows the readout's response ([ADR-0079](0079-the-rewards-direction-measured.md), [ADR-0081](0081-the-reinforced-form-measured.md)), and at the image's couplings the trace is +0.4 to +1.7 per synapse per trial (ADR-0077), three to twenty-five times less — so the last part is the slow part. A second mapping of 3 072 trials puts the criterion's last 128 about three times past the starting-rate estimate and admits a slowdown of that order; a run that has not revised by then is read as too slow for this rule alone, which the stopping rule turns into the next decision.
- **The arms.** Two runs from the one image, each its own test: the assignment first and the mirrored first.
- **The criterion.** Both clauses in both arms: (1) **it learned** — at least 80 correct of trials 1 409 to 1 536 under the first mapping; (2) **it revised** — at least 80 correct of trials 4 481 to 4 608 under the second, a tie not correct. **Yes** when both hold in both arms, **no** otherwise. The signed gate acts in the first half too — a wrong selection's pair is now depressed — so the first half is no longer H-16's run, and a failure of clause 1 is a no about the signed gate, not a failure to replicate.
- **The assertion, not a criterion.** No excitatory synapse outside the four stimulus–readout pairs moves in either arm (the gate at zero for every unaddressed synapse); and the oracle — ADR-0079's, extended to the signed branch as the round builds it — agrees with the record's weights, traces and signal at every trial.
- **The readings**: the selection per block and per stimulus; the four couplings per block; after the flip, the old answer pair's course, the trial at which the selection first crosses to the new answer and the trial at which it passes 40 of 64 per block; in the first half, where the wrong pairs end against the image's; the fraction of a punished trial's addressed synapses whose trace is negative and which the punishment therefore potentiated; the trace's composition on the answer pairs at least in the first block, the block of the flip and the last; the inhibitory sum, the sight, whether the stimulus still fires once, and the dopamine signal.
- **The prediction.** **No prediction is written for the verdict**: the starting-rate arithmetic says the lead can be spent inside the run, and the slowdown as the pair falls is the part nothing has measured. Predicted readings, Hypotheses: (a) after the flip the old answer pair falls, block on block on average, until the selection crosses; (b) its fall is fastest in the first blocks after the flip and slows as it falls; (c) in the first half the wrong pairs end below the image's couplings, depressed while they were selected early.
- **H-18's stopping rule, written with it:**
  1. **One round** (brief 041): the mechanism, its tests and its ADR; then the calibration, the two arms, the criterion committed before the first rewarded run.
  2. **A calibration that does not reproduce, or a pinned number that moves with the signed gate unset, stops the round** before any rewarded run; it is a finding, not an outcome.
  3. **Yes**: H-18 recorded yes with its scope, and the configuration the engine learns **and revises** in is named. The next decision is an ADR choosing among the reward-prediction error (where the rise stops, and a mean-free modulation), a schedule of reversals, the operating regime and another size.
  4. **No**: H-18 recorded no with its scope, and the next decision is an ADR on **an exploration in the selection**, on top of the signed gate or instead of it, with this round's readings — how far the old pair fell, how the rate fell, and how often a punishment potentiated — as its need.
  5. **No constant moves after a rewarded run**, there is no second attempt at H-18 in this configuration, and neither the flip's trial nor the run's length is moved after a run.

### Consequences

- Good: the one route ADR-0091's arithmetic favours is asked with one variable against H-17, under a criterion, an assertion, a prediction and a stopping rule written first.
- Good: the rule is the signed reward-modulated STDP of the literature, with its trace spent so that a punishment cannot run away; unset, it is today's rule bit for bit.
- Good: the first half now reads what a punishment does while the engine is still learning, which no run has read.
- Neutral: the image format moves to 16 for one flag.
- Bad: the signed rule potentiates a punished synapse whose trace is negative; how often, and whether it matters, is read here and not argued away.
- Bad: two arms of 4 608 trials are about one and a half times H-17's; [ADR-0092](0092-the-shards-dealt-by-cost.md) deals them by cost, and the round regenerates the cost table from its own dispatch.
- Bad: the length is an estimate from a starting rate and a named slowdown; a no that ends with the old pair still falling says "too slow for this rule alone", which is what the stopping rule sends to the exploration.

## Alternatives considered and why rejected

- **An exploration first** (option 2): about six hundred rewarded explorations against a lead one reward moves by at most 3 415; the larger change for the smaller effect.
- **Both at once** (option 3): two variables.
- **The reward-prediction error** (option 4): leaves the floor at zero; named for after.
- **A negative modulation with the trace left unchanged**, or with the conservation rule applied to a negative transfer: the first consolidates one trace again at every presynaptic spike of the trial, the second grows the trace as the weight falls; both run away.
- **The signed gate on every synapse**, addressed or not: under the gate an unaddressed synapse's modulation is the baseline, zero, so the sign would change nothing there; on inhibitory synapses it would put the dopamine term back where ADR-0085 took it out.

## Confirmation

- Whitepaper §11.1: H-18 with its configuration, criterion, assertion, readings, prediction and stopping rule, unchecked; H-17's stopping rule's item marked with the decision taken; §8.8's three-factor row naming the signed gate as decided and not built; the whitepaper's version moved in both declarations.
- `briefs/041_the-punished-pair.md`: the live brief that builds the mechanism and runs H-18 under this ADR.
- No code, no constant, no test and no record changed by this ADR.
