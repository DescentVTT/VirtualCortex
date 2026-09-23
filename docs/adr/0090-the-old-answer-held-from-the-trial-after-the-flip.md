---
status: accepted
date: 2026-09-23
amends: ADR-0089
decision-makers: VirtualCortex maintainers
---

# ADR-0090: The old answer held from the trial after the flip — ADR-0089's assertion amended before any run: the first trial under the second mapping still consolidates the last reward of the first, so the pair that trial's predecessor addressed may move in it, and the assertion holds the old answer's pairs from the end of the 1 537th trial, the other stimulus's old pair from the end of the 1 536th, and every excitatory synapse outside the four stimulus–readout pairs at the image's; the criterion, the arms, the length, the flip's trial, the prediction and the stopping rule are unchanged

## Context and Problem Statement

[ADR-0089](0089-the-assignment-reversed.md) wrote H-17 and, beside its criterion, an assertion: after the flip the pair that was the answer before it "neither rises nor falls: it can consolidate only when it is selected and correct, which it never is again, and nothing under the gate depresses it — so its coupling at the 3 072nd trial equals its coupling at the 1 536th, bit for bit, in both arms. If it fails, a finding against ADR-0080's derivation as this ADR extends it." Brief 040 gives the executing round the licence to amend H-17's assertion by an ADR committed before the first rewarded run when it finds a defect in it. Reading the engine before trusting the description of it, this round found one: **the trial index is one too early.**

The rule the assertion rests on is right, and it is what makes the index wrong. Three facts of the tree:

1. **The delivery is one trial late by construction.** `Task::trial` runs the trial's ticks, reads the selection, then writes the addressed set (`Executor::address`) and the reward (`Executor::reward`) between the trial's last tick and the next trial's first. `Executor::address` says the set holds "from the next tick … until the set is written again", which is the end of the next trial; so a trial's reward is consolidated during the trial after it, at the presynaptic spikes of the stimulus it presented, under `clamp(0 + signal, 0, 1)`. ADR-0079's oracle replays exactly this (`Composer::observe` consolidates a synapse under the course of the signal the last trial's reward left, where the last trial's delivery addressed it) and has equalled the record at every trial of every arm since brief 036.
2. **The 1 536th trial is rewarded, in both arms.** The first 1 536 trials of an H-17 arm are H-16's arm of the same assignment bit for bit: the same image, the same task, the same trials, and nothing differs until the flip, which comes after them. H-16 read 128 of the last 128 correct in both assignments ([ADR-0087](0087-inhibition-off-the-reward-gate-measured.md)), so the 1 536th trial selects its stimulus's old answer and is rewarded; its delivery addresses the synapses from that stimulus onto its old answer's readout, and the signal it leaves is 1.712 at the fixed point, at the modulation's ceiling for 9 694 of the next trial's 16 384 ticks and never below 0.712 in it.
3. **The stimulus's units fire in the next trial whatever it presents.** The 1 536th trial presents B and the 1 537th presents A (`Task::stimulus_at` with `SEED` 27: the low bit of `mix64` at indexes 1 535 and 1 536 reads 1 and 0). B's units fire in the 1 537th only from the network's own activity — in H-16's last block of the assignment the B set fired 2 500 spikes over 64 trials of which 1 779 were its 35 volleys, about eleven spikes a trial beyond them — and each such spike consolidates the pending trace of that unit's synapses onto B's old answer's readout under the ceiling.

So in both arms the 1 537th trial — the first under the second mapping — moves B's old answer pair by what those spikes consolidate, and the assertion as ADR-0089 wrote it fails by the very rule it rests on, whatever the engine does after the flip. Run as written, the round would open a finding against ADR-0080's derivation that is the assertion's defect and not the derivation's.

## Decision Drivers

- H-17's rule: a defect found in the criterion, the assertion or the stopping rule before the first rewarded run is written as an ADR amending ADR-0089 and committed before that run; never after it.
- The assertion should state what the derivation implies, no more and no less, so that its failure would be a finding about the engine.
- Nothing but `mirrored` may change at the flip (ADR-0089), so the carry-over is a property of the run to be read, not a thing to be removed.

## Considered Options

1. **Amend the trial index**: the old answer's pairs held from the end of the 1 537th trial, and the one the 1 536th trial did not address from the end of the 1 536th.
2. **Keep ADR-0089's form** and report its failure as a finding.
3. **Remove the carry-over from the run**: withhold the 1 536th trial's reward, or clear the addressed set at the flip.
4. **Assert on the couplings' sums** instead of synapse by synapse.

## Decision Outcome

**Option 1.** The assertion of H-17, as amended, in both arms — still an assertion and not a criterion, still computed beside the verdict and not in place of it:

- **(a) The old answer's pairs hold after the carry-over.** Every synapse from a stimulus onto the readout that was its answer before the flip has, at the end of the 3 072nd trial, the weight it had at the end of the **1 537th** — the first trial after the flip, in which the 1 536th trial's reward is consolidated. After it such a pair can consolidate only in the trial after one that selected it and was rewarded, and under the second mapping selecting it is wrong, so it never is again; a punishment leaves the signal below zero for the whole next trial and a tie addresses nothing (ADR-0080's derivation).
- **(b) The carry-over reaches one pair.** In the 1 537th trial the only one of the two old answer pairs that can move is the one the 1 536th trial's delivery addressed — the stimulus it presented onto the readout it selected, when it was rewarded — so the other stimulus's old answer pair has, at the end of the 3 072nd trial, the weights it had at the end of the **1 536th**.
- **(c) Nothing outside the four pairs.** No excitatory synapse outside the four stimulus–readout pairs moves from the image to the end of the run: [ADR-0085](0085-inhibition-off-the-reward-gate.md)'s assertion over the pairs a run with a flip can reach, which are all four. The inhibitory synapses move under their own baseline, as H-16 read, and are outside the clause. Brief 040's sentence "no synapse outside the four stimulus–readout pairs moved" is read as this clause: the inhibitory synapses cannot be meant, since the configuration H-17 runs moves them by its rule.

If (a), (b) or (c) fails, it is a finding against ADR-0080's derivation as ADR-0089 and this decision extend it, reported beside the verdict. What the carry-over moved in the 1 537th trial — the synapses and the coupling of the pair (b) names — is a reading.

**Nothing else of H-17 changes**: the network and the calibration, the constants, the two arms, 3 072 trials with the mapping flipped once between the 1 536th and the 1 537th by `Task::mirrored` and nothing else, the criterion's two clauses, the readings, the prediction of no and the stopping rule are ADR-0089's as written.

A consequence of fact 2 is stated here because it was read for this decision, and the round uses it: **the first half of each arm is H-16's arm bit for bit**, so clause 1 of the criterion is H-16's 128 before the run, and the round holds each arm's first twenty-four blocks, their compositions, their earned readings and their readings' hash to ADR-0087's pinned tables — a replication stronger than clause 1, whose failure would be the failure to replicate H-16 that H-17's criterion names.

### Consequences

- Good: the assertion says what the rule implies, so a failure of it would be a finding about the engine rather than about the sentence.
- Good: the carry-over, which ADR-0089 did not see, becomes a reading: what the last reward of the first mapping does to the old answer after the mapping has turned.
- Neutral: the amendment moves one trial index and names one pair; the brief's clause about the synapses outside the pairs is read as the excitatory clause H-16 held.
- Bad: one more sentence H-17 carries, and ADR-0089's text stands with the index it had; the whitepaper's H-17 names this decision beside it.

## Alternatives considered and why rejected

- **Keep ADR-0089's form** (option 2): the finding it would produce is known before the run to be the assertion's own, which is a finding of nothing.
- **Remove the carry-over** (option 3): withholding a reward or clearing the address at the flip changes the task at the flip, which ADR-0089 forbids, and removes a reading the run gives for free.
- **Sums instead of synapses** (option 4): weaker than bit for bit, and a sum can hold while synapses move in both directions.

## Confirmation

- Whitepaper §11.1's H-17: the assertion as amended here, with this decision named; §9's row; the whitepaper's version moved in both declarations.
- `runtime/cortex-runtime/tests/inhibition.rs`, in the commit that holds H-17's constants before any rewarded run: the assertion's three clauses as integer rules over the weights at the end of the 1 536th and the 1 537th trials and at the run's end.
