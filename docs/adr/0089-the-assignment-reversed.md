---
status: accepted
date: 2026-09-23
depends-on: ADR-0087
decision-makers: VirtualCortex maintainers
---

# ADR-0089: The assignment reversed — of the candidates H-16's stopping rule named, the reversal is chosen and written as H-17 before any run: H-16's configuration over 3 072 trials with the answer's mapping flipped at the half, in both directions; predicted **no**, because the gate is a deterministic comparison and under the reward's gate a pair consolidates only when it is selected and correct, so an engine that has learned selects the old answer and earns nothing — and the round measures how seldom it selects the other, which is the measured need the mechanism after it requires

## Context and Problem Statement

[ADR-0087](0087-inhibition-off-the-reward-gate-measured.md) read H-16 yes and named the configuration the engine learns in: every excitatory synapse under the reward's gate (the modulation baseline at zero), every inhibitory synapse under a baseline of its own. H-16's stopping rule names the next decision — an ADR choosing among **the reward-prediction error**, **the assignment reversed within a run**, **the operating regime** and **another size**, not ordered. This is that ADR. It chooses and builds nothing.

Three facts of the tree, read for this decision, decide which candidate is worth a round.

1. **The selection is a deterministic comparison.** `Readout::select` puts each readout's count into a `BasalGangliaChannelState` as its own direct drive and the other's as its indirect drive, and `compute_gating` selects a channel when `other − own < 0` — the larger count, with a tie selecting neither (`runtime/cortex-runtime/src/task.rs`, `crates/cortex-basal-ganglia/src/lib.rs`). **There is no exploration in it**: no noise, no temperature, no sampling. The engine tries the action it already prefers, every trial.
2. **Under the gate a pair consolidates only when it is selected and correct**, and nothing depresses a coupling the reward has raised: the signal at a trial's end is at most 0.712 and a punishment leaves it negative, so the wrong selection's pair spends the next trial under a modulation of zero ([ADR-0080](0080-the-reinforced-form.md)'s derivation, held on every arm of [ADR-0081](0081-the-reinforced-form-measured.md) and ADR-0087).
3. **What a learned engine looks like.** At the end of H-14's runs the answer readout counted **28.5 and 27.8 spikes** in the task's window against the other's **12.8 and 11.6**, and the selection was 128 of the last 128 (ADR-0081); H-16 read the same verdict. The answer's readout is about twice as loud as the other.

Put together, they say what happens when the mapping is reversed under this configuration: the engine goes on selecting the readout it learned, which is now wrong; a wrong selection earns no reward; nothing consolidates; nothing decays; and the engine has no way to discover that the other readout is now the answer — unless the counts happen to cross, which the gap of about sixteen spikes makes rare. **A system that learns but cannot revise is worth knowing about before anything else is built on it**, and the number that matters — how often the engine selects the readout it did not learn — is measurable in one round with no new mechanism.

## Decision Drivers

- H-16's stopping rule, applied as written: the choice is among its four candidates, made by an ADR.
- A mechanism needs a measured need ([ADR-0078](0078-the-rewards-direction.md) deferred the structural rule for exactly this reason). The reversal produces the need for whatever follows it: an exploration in the selection, or a depression under the gate.
- Cost: the reversal changes no engine rule — the task's `mirrored` flag is flipped between trials — where the reward-prediction error is a mechanism and another size is a longer run of everything.
- **Nothing chosen after a rewarded run**: H-17's configuration, arms, length, criterion, assertion, prediction and stopping rule are written here.
- A predicted no is worth a round only when the no is a reading nothing else gives (ADR-0080 refused a predicted no that would have measured only speed). Here the no's number — the trials in which the engine selects the other readout at all, and the rewards it earns after the reversal — is the need the next mechanism must meet.

## Considered Options

1. **The assignment reversed within a run**, as H-17.
2. **The reward-prediction error**: the reward less its expectation, so that consolidation stops as the accuracy climbs.
3. **The operating regime** (§11.1's open item).
4. **Another size** (4 096 units; 256 is unusable for this instrument, [ADR-0070](0070-where-256-units-settle.md)).

## Decision Outcome

**Option 1.**

- **Why the reversal and not the reward-prediction error** (option 2). The error brakes a rise that has not yet met anything: after 1 536 trials the answer pairs stand at 1.35 to 1.45 of the image's, with weights around 9 000 to 11 000 of the 32 767 the width allows, and ADR-0081 read the eligibility pending at any block's end within 74 per synapse, so nothing is at a rail. Revision is the property the configuration has not been asked for at all, and the error does not give it: it would lower what a correct selection consolidates, not let an engine that never selects the other readout find it. It is named for after this round.
- **Why not the regime or a size** (options 3, 4). Both are larger, and both would measure a configuration whose revision property is unknown.
- **Why the prediction is written as a no.** Facts 1 to 3 above; the round is run because the size of the no is the reading — how often a deterministic gate crosses under the counts' own variation, and whether a single rewarded trial after the reversal happens at all — and because a yes would be worth more still.

**H-17 (the engine revises a selection it has learned)**, written into whitepaper §11.1 as a Hypothesis with the following, all fixed here:

- **The network and the calibration.** ADR-0077's settled engine as ADR-0087 builds it, the modulation baseline zero, the inhibitory baseline 0.5; the calibration a frozen first block from the image with both parameters unset, held to ADR-0077's frozen run; a mismatch stops the round before any rewarded run and is a finding.
- **The constants.** H-16's: ADR-0076's stimulus, the instrument's geometry, `WINDOW`, `TRIAL_TICKS`, `SEED`, `REWARD_Q16` (1.0), `BLOCK`, `LAST_BLOCKS`, `REWARDED_MIN` (80), the gain 1.75, the controller off — and **the run 3 072 trials, forty-eight blocks, the answer's mapping flipped once, between trial 1 536 and trial 1 537**, by the task's `mirrored` flag and nothing else.
- **The arms.** Two runs from the one image: **the assignment first** (A onto readout 0, B onto 1 for the first half; the mirrored mapping for the second) and **the mirrored first** (the same, the other way round). No withheld arm: ADR-0087 read what the configuration does unrewarded over 1 536 trials, and this round's question is about a run that has learned.
- **The criterion.** Both clauses in both arms: (1) **it learned** — at least 80 correct of trials 1 409 to 1 536 under the first mapping; (2) **it revised** — at least 80 correct of the last 128 trials (2 945 to 3 072) under the second mapping, a tie not correct. H-17 is **yes** when both hold in both arms and **no** otherwise. A round in which clause 1 fails is a failure to replicate H-16 and is reported as one, not as an answer to H-17.
- **The assertion, not a criterion.** After the flip the pair that was the answer before it neither rises nor falls: it can consolidate only when it is selected and correct, which it never is again, and nothing under the gate depresses it — so its coupling at the 3 072nd trial equals its coupling at the 1 536th, bit for bit, in both arms. If it fails, a finding against ADR-0080's derivation as this ADR extends it.
- **The readings — the measured need.** After the flip, per block: the trials in which the engine selected the readout that is now the answer, the rewards it earned, the ties, and the two readouts' counts for each stimulus with their gap; the new answer pair's coupling, if it moves at all; before the flip, the same readings, so that the two halves are read alike. And the inhibitory sum, the sight and whether the stimulus still fires once, as in H-16.
- **The prediction, a Hypothesis written before the run.** **No**, by facts 1 to 3: the gate selects the larger count, the counts differ by about sixteen spikes in a learned run, so after the flip the engine selects the old answer in nearly every trial, earns nearly nothing, and consolidates nearly nothing; clause 2 fails in both arms and the new answer pair's coupling stays at or near the image's. What the round adds is the size: the trials of 1 536 in which the other readout is selected at all, and the rewards earned.
- **H-17's stopping rule, written with it:**
  1. **One round** (brief 040): the calibration, the two arms, the criterion committed before the first rewarded run.
  2. **A calibration that does not reproduce stops the round** before any rewarded run; it is a finding, not an outcome.
  3. **Yes**: H-17 recorded yes with its scope — the engine learns and revises in this configuration — and the next decision is an ADR choosing among the reward-prediction error, the operating regime and another size.
  4. **No**: H-17 recorded no with its scope, **and the next decision is an ADR choosing between an exploration in the selection** — a gate that sometimes takes the action it does not prefer, which `cortex-basal-ganglia`'s rule has no term for — **and a depression under the gate** — a rule by which an unrewarded or punished pair loses what it gained, which the modulation's clamp at zero forbids today — with this round's readings as the measured need each must meet.
  5. **No constant moves after a rewarded run**, there is no second attempt at H-17 in this configuration, and the flip's trial is not moved after a run.

### Consequences

- Good: the property a learning engine is judged by after "does it learn" — "can it change its mind" — is asked before anything else is built on the configuration H-16 named.
- Good: no mechanism is built to meet a need nobody measured; whichever way H-17 reads, the round produces the number the next decision needs.
- Good: the round costs two runs and no engine change, so its dispatch is the exhaustive tests alone.
- Neutral: the prediction is a no, and the ADR says why in terms of the two rules that produce it.
- Bad: if the no is as complete as predicted — no rewarded trial at all after the flip — the round reads a zero, and a zero is a thin measurement, though it is the sharpest form of the need.
- Bad: H-17 asks about one reversal at one point in one run; a schedule of reversals, or a reversal before the first learning has saturated, is not asked.

## Alternatives considered and why rejected

- **The reward-prediction error first** (option 2): brakes a rise that has met no rail, and does not give revision; named for after.
- **The regime or another size** (options 3, 4): larger, and both would measure a configuration whose revision property is unknown.
- **Building an exploration or a depression now, without the round**: a mechanism without a measured need, which ADR-0078 refused for the structural rule.
- **A withheld arm**: ADR-0087 read the configuration unrewarded; this round's question is about a run that has learned.
- **Flipping the mapping earlier, before the first half has learned**: then clause 1 does not hold and the round measures nothing about revision.

## Confirmation

- Whitepaper §11.1: H-17 with its configuration, criterion, assertion, readings, prediction and stopping rule, unchecked; H-16's stopping rule's item marked with the decision taken; the whitepaper's version moved in both declarations.
- `briefs/040_the-assignment-reversed.md`: the live brief that runs H-17 under this ADR.
- No code, no constant, no test and no record changed by this ADR.
