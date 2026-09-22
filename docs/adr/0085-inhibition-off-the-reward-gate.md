---
status: accepted
date: 2026-09-22
depends-on: ADR-0083
decision-makers: VirtualCortex maintainers
---

# ADR-0085: Inhibition off the reward's gate — of the two routes H-15's stopping rule named, consolidation gated by the reward is chosen, and its named cost is taken first: an inhibitory synapse may consolidate under a baseline of its own, a parameter of the image, while every excitatory synapse stays under the reward's gate; written as H-16 before any run — H-14's configuration with that inhibitory baseline at 0.5, which splits H-15's no into its two parts; the reward-prediction error and the regime named for after it

## Context and Problem Statement

[ADR-0083](0083-plasticity-everywhere-measured.md) read H-15 no: with the modulation baseline at 0.5, the reinforced form reached 106 of the last 128 in the mirrored assignment and 78 in the assignment, two short. Both readings [ADR-0082](0082-plasticity-everywhere.md) predicted held: the withheld arm drifted to readout 1 for both stimuli, and the stimulus whose answer is readout 0 decided each arm. H-15's stopping rule, written before that outcome, names the next decision: **an ADR choosing between a consolidation gated by the reward alone as the configuration the engine learns in — the baseline at zero, with what it costs the plasticity that consolidates through the baseline, ADR-0055's settling and the inhibitory rule among it — and the operating regime.** This is that ADR.

What ADR-0083 read says the no had two parts, and the run could not tell them apart:

1. **The excitatory synapses' unrewarded consolidation.** Under 0.5 the task's own volleys carried every stimulus–readout pair up by a third to a half of the image's in twenty-four blocks, faster onto readout 1, under a positive feedback that reached every pair alike; "the reward's differential on a pair is a tenth of that drift and of the size of the drift's own lean". This is the unsupervised term a reward-modulated rule meets when its modulation does not have a mean of zero (Frémaux, Sprekeler and Gerstner 2010).
2. **The inhibitory rule's course.** The inhibitory sum fell in every block, from 0.974 of the image's after the first to **0.411** after the twenty-fourth, in all three arms alike — the rule of [ADR-0053](0053-the-waking-day-and-the-target-period.md) and [ADR-0057](0057-the-inhibitory-rule-from-below-the-rail.md) carrying the network toward its target period, at the rate ADR-0077's lead-in read (about 0.5 M per window) — and a readout's count in the task's window rose from about 10 per trial to 43 to 51. The regime the instrument reads in was moving under the task.

H-14 read yes with the baseline at zero, where neither part acts: nothing consolidates but what a reward reaches. So the route that keeps the reward's gate — H-14's configuration — is the one with evidence behind it, and **its named cost is the inhibitory rule**, which consolidates through the same modulation and so stops with it. Whether ADR-0055's settling of the excitatory synapses outside the rewarded pairs is also needed is not read anywhere: under the gate those synapses keep the image's weights, which is what the settled network is for.

**What the engine does today** (`runtime/cortex-runtime/src/executor.rs`, read for this ADR). In the fan-out every slot of a block consolidates under `Modulations::for_synapse` — `clamp(baseline + dopamine, 0, 1)` if its synapse is addressed, `clamp(baseline, 0, 1)` otherwise — and the block's polarity, its presynaptic unit's flag ([ADR-0049](0049-dale-principle-in-plasticity.md)), decides only which half of the width the weight moves in and which pair rule enters the trace. One baseline serves both polarities. The image's modulator section holds the modulator's 16 bytes, the baseline at `[16..20)`, the inhibitory rule's target period at `[20..24)` and **40 reserved bytes that must be zero** (`image.rs`).

## Decision Drivers

- H-15's stopping rule, applied as written: the choice is between its two routes, made by an ADR.
- The evidence: H-14's yes at a gate of zero, H-15's no at 0.5, and the two parts of that no, which one run can separate.
- **One variable** against each of H-14 and H-15, so the answer attributes.
- A mechanism is an ADR first (`CLAUDE.md`); a parameter that changes what a run does is part of the image (§8.3); a change to a record's bytes, reserved ones included, moves the format.
- Every pinned number of ADR-0065 to ADR-0083 and the determinism pin must stand: the new parameter, unset, is today's rule bit for bit.
- Latest ≠ Newest (§2.1): the separation of a homeostatic inhibitory rule from the reward is Vogels et al. (2011)'s rule as it was written — a plasticity that balances excitation, not one that learns from reward — and adopts nothing newer.

## Considered Options

1. **The reward's gate, with inhibition off it**, as H-16: every excitatory synapse consolidates under the reward's gate (the baseline at zero) and every inhibitory synapse under a baseline of its own (0.5), H-14's configuration otherwise.
2. **The reward's gate alone** — H-14 as the configuration, the inhibitory rule stopped — recorded without a further run.
3. **The operating regime**: a review of ADR-0036, ADR-0044 and the inhibitory target against the regime the instrument reads.
4. **A reward-prediction error** in the task's delivery (the reward less its expectation, into a field the modulator already calls `dopamine_rpe`).
5. **ADR-0055's settling kept for the excitatory synapses outside the rewarded pairs** as well, by a second excitatory baseline for synapses not addressed.

## Decision Outcome

**Option 1: the route that keeps the reward's gate, with inhibition off it.**

- **Why the gate and not the regime.** The gate is the configuration under which the engine has learned (H-14, 128 of 128 in both assignments), and H-15's no is attributable to the two things the gate switches off. The regime is the deeper question and stays §11.1's open item; it becomes the next decision if H-16 is a no, because then the inhibitory rule's course alone — the regime moving under the task — is what breaks the learning.
- **Why not stop at option 2.** H-14 learned with the inhibitory rule stopped; a configuration the engine learns in cannot stop its inhibitory plasticity for good, since that plasticity is what holds excitation in balance as couplings rise (ADR-0079 and ADR-0081 read the answer pairs still rising at 1.45 of the image's). Recording option 2 without a run would adopt a configuration whose cost is unread.
- **Why a baseline of its own for inhibition, and 0.5.** It is the smallest separation that answers the question: the inhibitory rule consolidates as it did under H-15, and the excitatory synapses as they did under H-14. 0.5 is the value under which ADR-0077's lead-in settled the network and H-15 read the inhibitory rule's course; no other value has a reading behind it.
- **Why not the reward-prediction error now** (option 4). It changes the reward's mean and brakes the rise as the accuracy climbs, which bears on where the positive feedback stops; it does not bear on either part of H-15's no, and it is a second variable. It is named for after H-16.
- **Why not option 5.** A second excitatory baseline would re-admit the unrewarded excitatory consolidation that part 1 of H-15's no is made of; ADR-0055's settling of the arena outside the rewarded pairs stays off under the gate, and that is said.

**The rule, decided here; its placement the round's.** A new parameter, **the inhibitory baseline**: when it is set, every synapse of an inhibitory block consolidates under `clamp(inhibitory baseline, 0, 1)`, addressed or not — the dopamine term never reaches it — and every excitatory synapse under `Modulations::for_synapse` as today; when it is unset, every synapse consolidates as today. It changes what a run does, so it is part of the image (§8.3): the round places it in the modulator section's reserved bytes or elsewhere, moves the image format from 14 to 15, reads a format-14 image as unset, and updates §5.2's table and the changelog, as `CLAUDE.md` requires of any change to a record's bytes. **Unset, every pinned number of the tree and the determinism pin stand bit for bit.** The round records the placement in its own ADR.

**H-16 (the reward's gate holds with inhibition off it)**, written into whitepaper §11.1 as a Hypothesis with the following, all fixed here:

- **The network and the calibration.** ADR-0077's settled engine, built as ADR-0077, ADR-0079, ADR-0081 and ADR-0083 build it; the calibration a frozen first block from the image with the baseline at zero and the inhibitory baseline unset, held to ADR-0077's frozen run (the stimulus firing once, the sight 62, the sign 53); a mismatch stops the round before any rewarded run and is a finding.
- **The constants.** H-14's: ADR-0076's stimulus, the instrument's geometry, `WINDOW`, `TRIAL_TICKS`, `SEED`, `REWARD_Q16` (1.0), `BLOCK`, `LAST_BLOCKS`, `REWARDED_MIN` (80), 1 536 trials, the gain 1.75, the controller off, **the baseline zero** — and the one constant moved: **the inhibitory baseline, 0.5**.
- **The delivery and the arms.** `Task::trial` with `Delivery::Addressed`; the assignment and the mirrored assignment under `Feedback::Answer`, and the reward withheld (`Feedback::Withheld`) as a reading bounded by no clause.
- **The criterion.** H-14's: at least 80 correct of the last 128 in both rewarded arms, a tie not correct. **Yes** when both hold, **no** otherwise.
- **The assertion, not a criterion.** In a rewarded arm every excitatory synapse outside the two answer pairs — the wrong pairs among them — ends the run as the image holds it, bit for bit (ADR-0080's derivation, which holds for the excitatory synapses under a baseline of zero whatever the inhibitory ones do); in the withheld arm every excitatory synapse does. The inhibitory synapses move. If it fails, a finding against this derivation.
- **The readings**: the selection per block and per stimulus; the four stimulus–readout couplings per block; the arena's inhibitory sum per block beside H-15's course; the readout units' rate and a readout's count in the task's window per block; the sight and whether the stimulus still fires once; the trace's composition on the answer pairs by the oracle at least in the first and the last block; the dopamine signal.
- **The prediction, a Hypothesis written before the run.** **Yes**: the lean that decided H-15 was the excitatory synapses' unrewarded consolidation, which the gate removes; the reinforced form crossed 40 of 64 by trial 448 and 384 under H-14, when H-15's inhibitory sum — if its fall was as even as its endpoints suggest, about 2.5 per cent of the image's a block — had fallen by about a sixth. Predicted readings: the inhibitory sum falls in every block, as under H-15; the withheld arm's excitatory weights stay the image's, so its selection stays where the frozen network's is. **If the prediction fails, the regime is what failed it**, and the readings say by how much the readouts had grown louder when the selection stopped moving.
- **H-16's stopping rule, written with it:**
  1. **One round** (brief 039): the mechanism, its tests and its ADR; then the calibration, the three arms, the criterion committed before the first rewarded run.
  2. **A calibration that does not reproduce, or a pinned number that moves with the parameter unset, stops the round** before any rewarded run; it is a finding, not an outcome.
  3. **Yes**: H-16 recorded yes with its scope, and the configuration the engine learns in is named — every excitatory synapse under the reward's gate, every inhibitory synapse under a baseline of its own. The next decision is an ADR choosing among, at least: the reward-prediction error (where the rise stops); the assignment reversed within a run (under a gate at zero nothing depresses a coupling the reward raised); the operating regime; and another size.
  4. **No**: H-16 recorded no with its scope, and the next decision is the operating regime, by an ADR — the inhibitory rule's target against the regime the instrument reads (ADR-0036, ADR-0053, ADR-0057), with this round's readings of how far the regime moved before the learning stopped.
  5. **No constant moves after a rewarded run**, there is no second attempt at H-16 in this configuration, and no other inhibitory baseline is tried after a no without a new ADR that argues it before its run.

### Consequences

- Good: H-15's no is split into its parts by one run with one variable against each of H-14 and H-15, whichever way it reads.
- Good: a yes names a configuration the engine can learn in without switching its inhibitory plasticity off, which neither H-14 nor H-15 could.
- Good: the mechanism is Vogels et al.'s rule as written, homeostatic and not reward-modulated, and unset it is today's rule bit for bit.
- Neutral: the image format moves to 15 for one parameter; a format-14 image reads as unset.
- Bad: ADR-0055's settling of the excitatory arena outside the rewarded pairs stays off under the gate; whether a long-running engine needs it is not read here.
- Bad: brief 039 builds a mechanism under `src/` and measures with it, so its dispatch includes the whole-tree sweep (ADR-0075's first clause) — a longer round than the three before it.

## Alternatives considered and why rejected

- **The reward's gate alone** (option 2): a configuration whose cost — no inhibitory plasticity — would be adopted unread.
- **The regime now** (option 3): the deeper question, and the one a no here would make next; asked first, it would leave H-15's two parts unseparated.
- **The reward-prediction error now** (option 4): a second variable, bearing on where the rise stops rather than on H-15's no.
- **A second excitatory baseline** (option 5): the unrewarded excitatory consolidation again.
- **Inhibition off the gate always**, with no parameter: every pinned run since ADR-0032 consolidated its inhibitory synapses under the shared modulation, so every pinned number with an inhibitory synapse and a baseline below 1.0 would move.

## Confirmation

- Whitepaper §11.1: H-16 with its configuration, criterion, assertion, readings, prediction and stopping rule, unchecked; H-15's stopping rule's item marked with the decision taken; §8.8's three-factor row naming the inhibitory baseline as decided and not yet built; the whitepaper's version moved in both declarations.
- `briefs/039_inhibition-off-the-reward-gate.md`: the live brief that builds the mechanism and runs H-16 under this ADR.
- No code, no constant, no test and no record changed by this ADR.
