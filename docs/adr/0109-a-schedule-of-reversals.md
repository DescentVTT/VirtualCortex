---
status: accepted
date: 2026-09-26
depends-on: ADR-0108
decision-makers: VirtualCortex maintainers
---

# ADR-0109: A schedule of reversals — H-19's stopping rule at step 4 on clause 3 alone asks what else keeps the couplings rising, and the answer is ADR-0108's two readings: the advantage the critic delivers while the selection still errs, and an error's carry-over; an expectation per stimulus and readout would remove the first by starving the learning itself and is rejected; H-19's configuration is kept as the one the engine learns and revises in, and the learning line takes H-18's second named question under it: written as H-20 before any run, H-19's arms and critic through three reversals, their first 56 blocks H-19's bit for bit, asking whether every mapping is learned and every coupling stays below 1.30 of its image's; the speed of each reversal read beside it; no prediction for the verdict; brief 047

## Context and Problem Statement

[ADR-0108](0108-the-reward-prediction-error-measured.md) read H-19 as no, clause 3 the one that failed. With the critic of [ADR-0106](0106-the-reward-prediction-error.md) set, the engine did three things, in both arms:
- it learned the mapping: 122 and 125 of the last 128 trials before the flip;
- it revised it: 119 and 121 of the last 128 at the 4 608th trial;
- it did not settle: over each mapping's last 256 trials, one answer pair of each mapping moved by 1.22 to 1.87 per cent of its image coupling. The other moved by −0.08 to +0.34, where H-18's moved by 3.6 to 9.5.

H-19's stopping rule, at step 4 on clause 3 alone, names the next decision: *an ADR on what else keeps the couplings rising, with this round's readings as its need*. This is that ADR.

ADR-0108 read what keeps them rising, as arithmetic over its tables:

1. **The advantage.** The expectation of a stimulus settles near $2p - 1$ for a fraction $p$ of its presentations correct. The selection stayed wrong or tied in two to five trials of 64, so a correct trial still delivers about $2(1 - p)$, a tenth, onto the answer pair. In an actor–critic this is the advantage of the correct selection over the stimulus's mean. It is the signal that drives the selection toward the answer while the selection still errs, and it falls as the errors fall.
2. **The carry-over.** An error delivers about −1.9. The dopamine signal carries it into the next trial (F-53), where it reaches the pair that trial addresses at below zero.

In each arm and each mapping, one answer pair balanced the two to within a third of a per cent, and the other did not.

## Decision Drivers

- **The verdict stands.** H-19 is no. Its clause 3 is not reread, rescaled or moved. What this ADR decides is what the learning line does next, with ADR-0108's readings as the need.
- **Do not remove a signal the learning needs.** A change that stops the rise by removing the advantage stops the learning with it.
- **H-18's stopping rule named four questions.** The reward-prediction error has been asked; a schedule of reversals, the operating regime and another size remain.
- **One change at a time**, and the previous hypothesis as the control. H-19's arms are pinned for 4 608 trials, and a run that starts as they do can be held to them bit for bit.

## Considered Options

1. **An expectation per stimulus and selected readout**, a tie its own selection: the answer's expectation is moved by the answer's trials alone, so the residual errors no longer hold it below the reward.
2. **A bound in the plasticity rule**: potentiation scaled down as a weight nears a ceiling, a soft bound (van Rossum, Bi and Turrigiano 2000).
3. **A trial-locked dopamine signal**: the carry-over removed, the modulator reset or shortened between trials.
4. **Keep H-19's configuration, and take a schedule of reversals under it**, reading over a longer run whether the couplings stay bounded.

## Decision Outcome

**Option 4.**

- **Why not option 1.** An expectation that only its own action's trials move converges on that action's reward whatever the selection's accuracy. Its error at the $n$-th such trial is $(31/32)^n$ of the first, so over a whole run it delivers **at most 32 full rewards** to the answer's pair, and at most 32 full punishments to each wrong selection's pair, however many trials remain wrong. By ADR-0108's tables, H-19's critic delivered about 100 and 50 full rewards to each stimulus's answer pair before the selection passed 40 of 64 in a block, at trials 384 and 192. That is an estimate: the correct trials of those blocks, times one less the mean of the expectations at the block's two ends. It is one and a half to three times the bound. The same bound applies to the first learning as to the last rise, so option 1 would stop the learning before it stops the rise. This is arithmetic from the rule, not a measurement. It is the reason an actor is taught by the advantage over the stimulus's mean and not by an action's own error.
- **Why not options 2 and 3 now.** Each changes a rule of the engine for a rise that ADR-0108 read as slow and falling: 3 to 50 times smaller than H-18's over the same spans. Neither is needed unless a longer run shows the couplings do not stay bounded. Option 2 is named as the next decision's first candidate if they do.
- **What keeps the couplings rising, recorded:**
  - the advantage the critic delivers while the selection still errs, which falls as it errs less;
  - an error's carry-over into the next trial, which pushes the other way.
  - H-19's configuration is the one the engine learns and revises in with the reward-prediction error: ADR-0077's settled network at 1 024 units, the excitatory synapses under the reward's gate with the signed gate set, the inhibitory ones under a baseline of 0.5, and a critic of one expectation per stimulus with a shift of 5.
- **H-20**, written before any run, in §11.1:
  - *The run:* H-19's configuration exactly, each arm from H-19's image with its critic, over **7 680 trials**. The mapping flips between trials **1 536 and 1 537, 3 584 and 3 585, and 5 632 and 5 633**: four mappings, the first 1 536 trials long and each later one 2 048 (32 blocks). ADR-0108 read the selection passing 40 of 64 in a block 19 and 23 blocks after H-19's flip, and 57 to 59 of 64 correct 32 blocks after it, so a mapping of 32 blocks leaves room to learn.
  - *Asserted:*
    - the first 56 blocks of each arm are H-19's arm's first 56 blocks bit for bit (the same image, critic and trials up to the second flip);
    - no excitatory synapse outside the four stimulus–readout pairs moves;
    - the oracle agrees with the record at every trial.
  - *Yes* when, in both arms:
    1. at least 80 of the last 128 trials of each of the four mappings are correct: trials 1 409–1 536, 3 457–3 584, 5 505–5 632 and 7 553–7 680, a tie not correct;
    2. at the end of every block of the run, every one of the four stimulus–readout couplings stands at or below **1.30** of its image coupling. H-19's stood at or below 1.165 at every block's end; H-18's passed 1.30 in the 15th and 18th blocks and reached 1.55.
  - *No* otherwise, with the clause, the mapping and the block named.
- **Predicted readings, not the verdict:**
  1. After each of the three flips, each stimulus selects its new answer within 128 trials. H-19 did so within 3 to 68.
  2. Each mapping's first four blocks after its flip (for the first mapping, its first four blocks) hold fewer correct trials than its last four.
- **Read, not argued:**
  - *The speed of each reversal:* the blocks from each flip to the first block with 40 of 64 correct under the new mapping, and each stimulus's crossing block, beside H-19's first reversal (19 and 23 blocks).
  - The errors and ties per mapping; H-19's settle measure over each mapping's last 256 trials.
  - Each expectation and the dopamine signal by block.
  - The inhibitory sum, which H-19 left at a tenth of the image's and still falling, and which a run 1.67 times as long takes further.
- **No prediction for the verdict.** Two forces, each read by ADR-0108, meet a run 1.67 times as long:
  - the inhibitory drain under the baseline of 0.5 makes the network more excitable block after block;
  - the pairs alternate between answer and old answer at each flip, each time starting from where the last punishment left them.
- **Its stopping rule, written with it:**
  1. One round, brief 047: the schedule in the harness, the constants and rules committed before the first rewarded run, then the runs.
  2. Any of the following stops the round before any rewarded run and is a finding: a calibration that does not reproduce ADR-0077's settled candidate and H-19's image; a pinned number that moves; or a run whose first 56 blocks are not H-19's.
  3. Yes: H-20 recorded yes with its scope, and the configuration named as one that learns across a schedule of reversals with its couplings bounded. The next decision is an ADR choosing among the operating regime, another size, the inhibitory drain and a critic of the engine's own.
  4. No:
     - clause 1 failing at a later mapping: an ADR on what the schedule degrades, with the reversal speeds and the inhibitory sum as the need;
     - clause 2 failing: an ADR on a bound in the plasticity rule (option 2), with the block and the pair that passed 1.30 as the need.
  5. No constant moves after a rewarded run, the flips and the run's length included. There is no second attempt at H-20 in this configuration.

### Consequences

- Good: H-19's verdict stands as written, and what it measured decides the next step: the advantage is kept because the learning needs it.
- Good: the first 56 blocks are H-19's, so the run reproduces the previous hypothesis before it asks a new one, and a divergence there is a finding, not a result.
- Good: whether the couplings stay bounded is read over three reversals and 7 680 trials, not 256, with a bound that separates H-18's behaviour from H-19's.
- Bad: the settling H-19 asked for is not asked again. A pair may keep moving by 1 to 2 per cent over 256 trials and still pass clause 2, so a slow rise under 1.30 is read, not ruled out.
- Bad: one seed, one size, and a schedule of fixed lengths. A learning set in the sense of reversals learned faster than the first, if the speeds show one, is a reading of one run, not a finding about the engine.

## Alternatives considered and why rejected

- **Option 1, an expectation per stimulus and readout**: above. It stops the learning with the rise.
- **Option 2, a soft bound, now**: a rule change for a rise read as slow and falling. It is the named candidate if clause 2 fails.
- **Option 3, a trial-locked dopamine signal**: it removes the force that pushes against the rise, so it would make the rise faster, not slower.
- **Reading H-19's clause 3 again over a longer span**: that moves a clause after a run.
- **The operating regime or another size first**: each would carry the couplings' course over a longer run as an unknown. The schedule reads that course, and is the question H-18 named beside them.

## Confirmation

`briefs/047_a-schedule-of-reversals.md` runs H-20. Whitepaper 4.60.0 carries H-19's step 4 taken, H-20 and its stopping rule in §11.1, and §9's row. `npm run spec` holds the links, the rows, the brief's sections and the version.
