---
status: accepted
date: 2026-09-22
depends-on: ADR-0081
decision-makers: VirtualCortex maintainers
---

# ADR-0082: Plasticity everywhere — of the candidates H-14's stopping rule named, the baseline at which every synapse consolidates is chosen and written as H-15 before any run: H-14's configuration with the modulation baseline at 0.5 instead of zero (brief 027's, the baseline ADR-0077's lead-in settled under and H-12's rewarded runs used), every other constant H-14's; the drift the unrewarded consolidation brings predicted from ADR-0077's terms to favour readout 1 and read in a withheld arm; the note beside H-12 for a yes and the stopping rule written first; the runs in a test binary of their own, named so that the round robin leaves `instrument.rs` alone in its shard

## Context and Problem Statement

[ADR-0081](0081-the-reinforced-form-measured.md) read H-14 yes: on the network [ADR-0077](0077-the-background-side.md) settled, with the modulation baseline at zero, the task as [ADR-0059](0059-a-task-a-readout-and-a-reward.md) and [ADR-0068](0068-the-reward-addressed.md) built it — the reward addressed to the synapses from the presented stimulus onto the readout the engine selected, its sign the outcome's — moved the selection to the answer in 128 of the last 128 trials in the assignment and in the mirrored assignment over 1 536 trials. H-14's stopping rule ([ADR-0080](0080-the-reinforced-form.md), whitepaper §11.1) names what follows a yes: an ADR that chooses what the learning line asks next among, at least, the baseline at which every synapse consolidates, the assignment reversed within a run, the operating regime and another size. This is that ADR. It chooses and builds nothing.

**Why the baseline first.** Of the conditions under which H-13 and H-14 read yes, the baseline at zero is the most artificial: under it a synapse consolidates only under the dopamine term and only where the task addressed it, so nothing in the arena moves but the pairs a reward reached. That made the attribution exact, and it is also a network in which ADR-0055's depression cannot settle a weight, the inhibitory rule does not act, and nothing the background pairs is ever kept. Whether the selection still moves to the answer when every synapse consolidates part of what it pairs is the question on which every other candidate depends: a reversal, a regime or a size measured only at a baseline of zero would inherit the same qualification.

**Which baseline.** The executor's default is 1.0 (`Config::default`, ADR-0022's rule: every pairing consolidated at once), under which a reward adds nothing and the task refuses to deliver one (`TaskError::RewardAtCeiling`). The baseline at which every synapse consolidates and a reward still has room is below it; the one the tree has used is **0.5** — brief 027's `BASELINE_Q16`, the modulation of every rewarded run H-12 made, and the baseline under which ADR-0077's lead-in settled the network H-13 and H-14 ran on. No other value has a reading behind it, and choosing one now would be a constant picked for this question; 0.5 is taken as the tree has it.

**What 0.5 does, read from the rules** (`Modulations::for_synapse`, `NeuromodulatorState::modulation`, `reward`, `decay_dopamine`). Every synapse that is not addressed consolidates half its pending trace at each presynaptic spike. The signal at a trial's end lies between −0.712 and 0.712, the fixed points of a punishment and of a reward every trial (`decay_dopamine` reads the magnitude only, so the two are symmetric; ADR-0079 read the positive one). After a correct selection the addressed pair — the answer's — consolidates under `clamp(0.5 + signal, 0, 1)` with the signal at least 0.288 after the reward, so **under 0.788 to 1.0** at the next volley where the rest of the arena consolidates under 0.5: a reward now adds up to half a trace to what the background's consolidation already takes. After a wrong selection the addressed pair — the wrong one — consolidates under the same rule with the signal at most 0.712 − 1.0, so **under at most 0.212** for the trial after it, where under a baseline of zero it consolidated nothing: the punishment, which at zero only withheld consolidation, now lowers it below the rest of the arena's.

**What the background's consolidation will do, read from ADR-0077.** On the settled network the trace on the four stimulus–readout pairs over the frozen block was net potentiation on average for every pair, and larger onto readout 1: **+0.4, +1.6, +0.8 and +1.7 per synapse per trial** on A→R0, A→R1, B→R0 and B→R1, their sums after the block +0.7, +14.5, +8.4 and +13.6. At a baseline of 0.5 half of that is consolidated whatever the selection, and ADR-0079 read the rise as positive feedback — a higher coupling, a larger response, a larger potentiation. H-14's shuffled arm, whose reward carried no information, read the same lean at a baseline of zero: over 1 536 trials A→R1 rose to 1.258 of the image's against A→R0's 1.036, B→R1 to 1.168 against B→R0's 1.095, and A went to readout 1 in 60 of its last 61 presentations. So the drift this baseline adds is expected to favour readout 1 for both stimuli, and in each rewarded arm the stimulus whose answer is readout 0 — A in the assignment, B in the mirrored assignment — is the one the reward must carry against it.

**Where the runs go.** ADR-0081 measured `instrument.rs` at 4 047 s of its shard's 7 200 (56 per cent), a twenty-minute test being the binary's critical path, and read that the next measuring round's runs go to a binary of their own. `scripts/exhaustive-shard.sh` sorts the binaries that hold an `exhaustive` test by path and gives shard $k$ every binary whose position $NR$ has $NR \bmod 3 = k$; the binaries are now `cortex_core`, `instrument`, `learning` and `reference`, so `instrument` is alone in its shard. A new binary whose name sorts **after** `instrument` moves `reference` (1 241 s) or itself into `instrument`'s shard — past 73 per cent on this runner — while one whose name sorts **before** `instrument` leaves it alone and pairs the new binary with `learning` or `reference`.

## Decision Drivers

- H-14's stopping rule, applied as written: an ADR chooses among its named candidates; this one chooses and builds nothing.
- One variable against H-14 — the baseline — so that a difference in the outcome is the baseline's; every other constant, arm and mark is H-14's.
- **Nothing chosen after a rewarded run**: H-15's configuration, arms, criterion, readings, prediction and stopping rule are written here before its first run.
- The drift is predicted from readings, not feared or assumed: ADR-0077's terms and H-14's shuffled arm say which way it leans, and a withheld arm reads it.
- [ADR-0073](0073-the-whole-domain-tests-sharded.md) and ADR-0081's reading: no shard past 60 per cent of its bound, and `instrument.rs`'s shard not grown.
- Latest ≠ Newest (§2.1): nothing adopted.

## Considered Options

1. **The baseline at 0.5, as H-15**: H-14's configuration, arms and criterion with the baseline at 0.5; a withheld arm read beside them; the runs in a binary of their own named before `instrument`.
2. **The assignment reversed within a run**, at the baseline of zero.
3. **The operating regime** (§11.1's open item).
4. **Another size.**
5. **Another baseline** between zero and 1.0.
6. **The runs in `instrument.rs`**, or in a binary of their own under any name.

## Decision Outcome

**Option 1.**

- **Why not the reversal first** (option 2). Under a baseline of zero, as ADR-0080 derived and ADR-0081 held, a wrong selection's pair never consolidates and nothing depresses a coupling the reward raised: the answer pairs stood at 1.35 to 1.45 of the image's after 1 536 trials and every wrong pair at the image's. A reversed assignment would ask the old answer's pair to lose what it gained, and at zero nothing can take it away; at 0.5 the punishment lowers the wrong pair's consolidation to at most 0.212 against the arena's 0.5, which is the first form of the rule in which unlearning has a mechanism at all. The reversal is better asked after the baseline, and under whichever baseline this round's answer recommends.
- **Why not the regime or a size first** (options 3 and 4). Both would be measured at a baseline of zero and inherit its qualification; the regime is a review of two accepted ADRs, and 256 units are unusable for this instrument (ADR-0070).
- **Why 0.5 and no other** (option 5). It is the only baseline below the ceiling with a reading behind it, and the one ADR-0077's network settled under; a value chosen between zero and 0.5 for this question would be a constant fitted to a hoped-for outcome.
- **Why a binary of its own, named before `instrument`** (option 6). ADR-0081 read that another run of this size in `instrument.rs` would pass the 60 per cent the briefs use, and the round robin above says a name sorting after `instrument` puts two heavy binaries in one shard. The harness ADR-0077, ADR-0079 and ADR-0081 built lives in `instrument.rs`; the round shares it with the new binary rather than copying it, and states the shards' binaries after the change. A rule that balanced the shards by their measured times would be an amendment to ADR-0073 and is not taken here; the name is the smallest change that keeps every shard under its bound.

**H-15 (a reward the engine earns moves its selection when every synapse consolidates)**, written into whitepaper §11.1 as a Hypothesis with the following, all fixed here:

- **The network and the calibration.** ADR-0077's settled engine, built as ADR-0077, ADR-0079 and ADR-0081 build it, saved as an image with the modulation baseline at **0.5**, BASELINE_Q16, the lead-in's own — and, for the calibration, as the image with the baseline at zero. Before any rewarded run a frozen first block from the zero image is held to ADR-0077's frozen run — the stimulus firing once, the sight 62, the sign 53 — as ADR-0079 and ADR-0081 held it. A mismatch stops the round before any rewarded run and is a finding, not an outcome.
- **The constants.** H-14's: ADR-0076's stimulus; the instrument's geometry, `WINDOW`, `TRIAL_TICKS`, `SEED`, `REWARD_Q16` (1.0), `BLOCK`, `LAST_BLOCKS`, `REWARDED_MIN` (80); **1 536 trials**; the gain 1.75 as the image holds it, the controller off. The one constant moved: **the baseline, 0.5**.
- **The delivery.** `Task::trial` with `Delivery::Addressed` and the arm's feedback, unchanged.
- **The arms.** Three runs of the same 1 536 trials from the 0.5 image: **the assignment** and **the mirrored assignment** (`Feedback::Answer`), and **the reward withheld** (`Feedback::Withheld`: every synapse consolidating under 0.5 and no reward), a reading. The shuffled reward is not run; H-14 read it at zero, and at 0.5 the withheld arm is the reading of what the background's consolidation does alone, which is this question. The round may add it if its binary's budget allows.
- **The criterion.** H-14's: over the last 128 trials, the correct selections number **at least 80 in the assignment and in the mirrored assignment** (a tie is not correct). H-15 is **yes** when both hold and **no** otherwise. The withheld arm is bounded by no clause: under the positive feedback ADR-0079 read, plasticity without a reward can lock a stimulus onto either readout, so its accuracy is not binomial; the information control is the mirrored pair, whose two arms start from one image and differ only in the answer's mapping.
- **The assertion, not a criterion.** In a rewarded arm the signal at every trial's end lies between −0.712 and 0.712; in the trial after a wrong selection the addressed pair consolidates under at most 0.212 (0.5 + 0.712 − 1.0), and in the trial after a correct one under at least 0.788 (0.5 − 0.712 + 1.0); held by the oracle against the record at every trial. If it fails, a finding against this derivation.
- **The readings**, in every arm: the selection per block and per stimulus; the four stimulus–readout couplings per block; the arena's excitatory and inhibitory sums per block (the settling ADR-0077 stopped, resumed under the task); the sight per block and the stimulus's volley and its spikes after it per block (whether the stimulus still fires once as every synapse of the stimulus units consolidates); the trace's composition on the four pairs by ADR-0079's oracle at least in the first and the last block; and the dopamine signal at the trials' ends.
- **The prediction, a Hypothesis written before the run.** **No prediction is written for the verdict.** Two readings are predicted: (a) in the withheld arm the four couplings rise and the two onto readout 1 rise faster, so its selection drifts toward readout 1 for both stimuli (from ADR-0077's terms and H-14's shuffled arm); (b) in each rewarded arm the stimulus whose answer is readout 0 — A in the assignment, B in the mirrored — reaches its answer later than the other or not at all, and is where the verdict is decided.
- **H-15 and H-12, written before the outcome.** If H-15 is yes, the note ADR-0081 wrote beside H-12's closing sentence gains one sentence: the answer holds under the baseline H-12's rewarded runs used, so what separated those runs from these yeses is the settled network, ADR-0076's stimulus and the run's length — the pairing statistics H-12's closing sentence named as its cause — and not the baseline. If H-15 is no, nothing is added beside H-12.
- **H-15's stopping rule, written with it:**
  1. **One round** (brief 038): the calibration, the three arms, the criterion committed before the first rewarded run.
  2. **A calibration that does not reproduce stops the round** before any rewarded run; it is a finding, not an outcome.
  3. **Yes**: H-15 recorded yes with its scope and the sentence beside H-12. The next decision is an ADR that chooses among, at least: the assignment reversed within a run, at this baseline; where the positive feedback stops; the operating regime; and another size. It is not ordered here.
  4. **No**: H-15 recorded no with its scope. The next decision is an ADR that chooses between a consolidation gated by the reward alone as the configuration the engine learns in — the baseline at zero, with what it costs the plasticity that consolidates through the baseline, ADR-0055's settling and the inhibitory rule among it — and the operating regime.
  5. **No constant moves after a rewarded run**, and there is no second attempt at H-15 in this configuration; a baseline between zero and 0.5 is not tried after a no without a new ADR that argues it before its run.

### Consequences

- Good: the one condition that made H-13's and H-14's attribution exact, and that no engine running its own plasticity would meet, is removed and nothing else is changed.
- Good: the drift is predicted from readings in the tree, so a no can say whether it was the drift that beat the reward, and on which stimulus.
- Good: the reversal, asked next after a yes, is asked under the first form of the rule in which a punishment can lower a coupling's consolidation.
- Good: the shard budget ADR-0081 read is kept by a rule written before the round, not discovered after its dispatch.
- Neutral: a yes adds a sentence beside H-12 that confirms its closing sentence's diagnosis rather than contradicting it.
- Bad: under 0.5 the attribution is no longer exact — every synapse moves — and the readings must carry what the assertion carried at zero.
- Bad: the new binary's name is chosen to steer a round robin; a rule that balanced shards by their measured times would not need it, and is left for its own decision.

## Alternatives considered and why rejected

- **The reversal first** (option 2): at zero nothing can unlearn; asked after the baseline.
- **The regime or a size first** (options 3, 4): each would inherit the baseline's qualification.
- **Another baseline** (option 5): no reading behind any other value.
- **The runs in `instrument.rs`** (option 6): past 60 per cent by ADR-0081's reading; **a binary named after `instrument`**: two heavy binaries in one shard.
- **The shuffled arm instead of the withheld one**: at 0.5 the question is what the background's consolidation does, which the withheld arm reads without a random dopamine on top of it.
- **A clause bounding the withheld arm**: non-binomial under positive feedback, as ADR-0080 found for the shuffled arm.

## Confirmation

- Whitepaper §11.1: H-15 with its configuration, criterion, assertion, readings, prediction, relation to H-12 and stopping rule, unchecked; H-14's stopping rule's item marked with the next decision taken; the whitepaper's version moved in both declarations.
- `briefs/038_plasticity-everywhere.md`: the live brief that runs H-15 under this ADR.
- No code, no constant of the engine, no test and no record changed by this ADR.
