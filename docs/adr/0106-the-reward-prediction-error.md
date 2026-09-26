---
status: accepted
date: 2026-09-26
depends-on: ADR-0096
decision-makers: VirtualCortex maintainers
---

# ADR-0106: The reward-prediction error — of the four questions H-18's stopping rule named, the reward-prediction error is taken: the task keeps an expected reward for each stimulus, and the dopamine signal receives the outcome's reward less that expectation, which then moves toward the reward by a thirty-second of the error; written as H-19 before any run: H-18's configuration, schedule and arms with the critic set, asking whether the engine learns, revises and settles, where H-18's couplings never stopped rising; the tension the rules predict between settling and revising stated before the run, and no prediction for the verdict

## Context and Problem Statement

[ADR-0096](0096-the-punished-pair-measured.md) read H-18 as yes. On [ADR-0077](0077-the-background-side.md)'s settled network at 1 024 units, the configuration below learned the mapping in 128 of the last 128 trials before the flip, and learned the flipped mapping in 128 of the last 128 at the 4 608th trial, in both arms:
- every excitatory synapse under the reward's gate with the signed gate set ([ADR-0094](0094-the-signed-gate-built.md));
- every inhibitory synapse under a baseline of its own at 0.5 ([ADR-0086](0086-the-inhibitory-baseline-built.md));
- the reward addressed to the synapses from the presented stimulus onto the selected readout ([ADR-0068](0068-the-reward-addressed.md)).

Its stopping rule's third step named the next decision: an ADR choosing among the reward-prediction error, a schedule of reversals, the operating regime and another size. [ADR-0105](0105-the-speed-line-closed.md) closed the speed line, and the maintainers chose the reward-prediction error.

What the tree does today, read on 2026-09-26:

- **The reward is the outcome, every trial.** `Task::trial` (`runtime/cortex-runtime/src/task.rs`) under `Feedback::Answer` delivers `+reward_q16` for a correct selection and `−reward_q16` otherwise, a tie included, through `Executor::reward`. The modulator's `reward(reward_prediction_error_q16)` adds its argument to `dopamine_rpe`. The field and the argument are named for a prediction error, and they receive the reward itself. The learning line's reward is 1.0 (`REWARD_Q16`), and a trial is $2^{14}$ ticks, one dopamine time constant.
- **So a learned mapping is reinforced for ever.** H-18's answer pairs, read from its pinned tables (`PUNISHED_BLOCKS_1024`, as a reading of the image's coupling):
  - With every trial of the last four blocks before the flip correct, the four answer pairs rose by **3.7, 7.6, 4.2 and 9.5 per cent** of the image's coupling over trials 1 281 to 1 536.
  - The new answer's pairs rose by **7.1, 8.8, 4.9 and 6.5 per cent** over the last 256 trials of the run.

  ADR-0096 recorded it: "the rise has no stop but the rail". The dopamine signal sat at its fixed point, 1.712 after each reward, through every correct block.
- **The revision was bought by punishment that did not fade.** After the flip, every error delivered −1.0 again, the signal went to −1.712 and the signed modulation clamped at −1. The old answer's pair fell trial after trial until each stimulus crossed, 14 to 20 blocks after the flip: 900 to 1 300 trials, about 450 to 650 punished presentations of each stimulus.

## Decision Drivers

- **The prediction error is what dopamine neurons report** (Schultz, Dayan and Montague 1997). A reward that is expected changes nothing, and a surprise does. Reward-modulated STDP needs its reward's mean taken away, stimulus by stimulus, to learn anything but the drift of its own correlations (Frémaux, Sprekeler and Gerstner 2010).
- **The unbounded rise is the defect H-18 left.** A schedule of reversals, another size or another regime would each inherit couplings that rise to the rail.
- **One change at a time.** H-18's configuration, schedule, seeds and arms are kept, so that H-18's pinned arms are this hypothesis's control, trial for trial until the critic makes them differ.
- **Where the critic lives.** The readout, the selection and the reward's sign are the task's (`task.rs`), composed by the runtime around the engine. A critic that predicts the task's reward for the task's stimulus is the task's too. A critic of the engine's own, a value in a record over the engine's own representation of a state, is a larger question than this hypothesis needs.

## Considered Options

For the prediction:

1. **An expected reward per stimulus**, the Rescorla–Wagner critic of a one-step episode: $\delta = r - V_s$, then $V_s \leftarrow V_s + \delta/2^k$.
2. **One expected reward for all trials**, the running mean of the reward. It cancels the drift of the whole task and not each stimulus's.
3. **An expected reward per stimulus and readout**, a Q-value, with the selection reading it. That changes the selection as well as the reward, which is two changes at once.

For where it lives: the task (the runtime), or a record of the engine and the image.

## Decision Outcome

**Option 1, in the task.**

- **The rule.** A critic in `Task`, off unless set, holds `expected_q16[s]` for each stimulus, Q16.16, zero at the run's start.
  - At each trial, after the selection, the outcome's reward is $r = \pm$`reward_q16` exactly as `Feedback::Answer` gives it, a tie an error.
  - The prediction error is $\delta = r - V_s$, saturating.
  - $\delta$ is what `Executor::reward` receives, and it is recorded as the trial's reward.
  - Then $V_s \leftarrow V_s + (\delta \gg 5)$: an arithmetic shift, saturating, which keeps $V_s$ within $[-r, r]$.

  The shift is **5**. $V_s$ forgets over 32 presentations of its stimulus, about one block of 64 trials, the unit the criterion reads. The address, the delivery and every other rule are unchanged. With the critic unset every pinned number of every round stands, H-18's arms included.
- **Not in the image.** The critic is the task's state, like the seed and the trial's index. No record field moves, and the format stays 16.
- **H-19**, written before any run, in §11.1:
  - *The run:* H-18's configuration with the critic set: ADR-0077's settled image; the excitatory baseline zero; the signed gate set; the inhibitory baseline 0.5; [ADR-0076](0076-two-injections.md)'s stimulus; the task as built, addressed, the reward 1.0; 4 608 trials with the mapping flipped between trial 1 536 and 1 537; H-18's two arms, from the assignment and from the mirrored assignment.
  - *Yes* when, in both arms:
    1. at least 80 of trials 1 409 to 1 536 are correct;
    2. at least 80 of trials 4 481 to 4 608 are correct;
    3. each of the two answer pairs of the first mapping moves by less than **1 per cent** of its image coupling over trials 1 281 to 1 536, and each of the second mapping's over trials 4 353 to 4 608. H-18 moved them by 3.7 to 9.5 per cent over the same trials.

    A tie is not correct.
  - *No* otherwise, with the clause that failed named.
  - *Asserted*, as for H-18: no excitatory synapse outside the four stimulus–readout pairs moves, and the oracle agrees with the record at every trial, fed the prediction error the trial delivered.
- **Predicted readings, not the verdict**:
  1. The first mapping's answer pairs stand below H-18's at the flip (1.372 to 1.441 of the image's).
  2. Each stimulus's expected reward falls below zero within 64 trials of the flip. By the rule $V_n = -1 + 2(31/32)^n$ over its presentations after the flip, starting from 1.0, it is below zero after 22 presentations.
- **The tension, stated before the run.** The rules give the critic two effects in opposite directions:
  - *It bounds the couplings.* Once a stimulus is answered correctly, $V_s \to 1$, $\delta \to 0$, the modulation of its pair falls to the baseline of zero, and nothing more is consolidated. This is clause 3.
  - *It shortens the punishment.* After the flip, $\delta_n = -2(31/32)^n$ at the stimulus's $n$-th presentation. It holds the signed modulation at −1 for about the first 22 presentations and fades by half every 22 after that: about 54 presentations' worth of full punishment in all, where H-18's revision took 450 to 650 of them.
  - *Against it:* a correct selection after the flip is reinforced by up to $\delta = +2$, against H-18's +1. The first mapping's pairs, bounded, stand lower than H-18's, so there is less to undo.

  Whether clause 2 holds is the question the round answers. **No prediction is written for the verdict**, and the critic's shift is not tuned to one.
- **Read, not argued:**
  - the trials after the flip whose modulation of the old answer's pair was at −0.5 or below;
  - each stimulus's expected reward by block;
  - the dopamine signal by block, which ADR-0096 read at 1.712 through every correct block;
  - the inhibitory sum, which the critic does not reach (its synapses are under their own baseline) and which H-18 left at 0.11 of the image's.
- **Its stopping rule, written with it:**
  1. One round, brief 046: the critic, its tests and its ADR, then the runs, the criterion committed before the first rewarded run.
  2. A calibration that does not reproduce ADR-0077's settled candidate, or a pinned number that moves with the critic unset, stops the round before any rewarded run and is a finding.
  3. Yes: H-19 recorded yes with its scope; the configuration the engine learns, revises and settles in named; and the next decision an ADR choosing among a schedule of reversals, the operating regime, another size and a critic of the engine's own.
  4. No: H-19 recorded no with its scope and the failed clause.
     - Clause 2 alone failing (learning and settling, not revising): the next decision is an ADR on an exploration in the selection (H-17's other named mechanism) under the critic, or on the critic's time scale, with this round's readings as the need.
     - Clause 1 failing: an ADR on why the prediction error does not learn where the reward did.
     - Clause 3 failing: an ADR on what else keeps the couplings rising.
  5. No constant moves after a rewarded run, the critic's shift included. Neither the flip's trial nor the run's length moves, and there is no second attempt at H-19 in this configuration.

### Consequences

- Good: the signal the modulator was named for is the signal it receives, and the unbounded rise H-18 left is asked about directly, with H-18's arms as the control.
- Good: the tension between settling and revising is written down before the run, with the arithmetic that sets it, so whichever way clause 2 reads is a measured answer, not a surprise.
- Bad: the critic is the task's, not the engine's. A trace recorded in one run does not carry it into an image, and a critic over the engine's own states is left for later.
- Bad: one shift, one size, one seed and one reversal.
- Neutral: the inhibitory drain under the baseline of 0.5 is not addressed; it is read beside.

## Alternatives considered and why rejected

- **One expected reward for the whole task** (option 2): with two stimuli learned at different speeds, the drift of one would be charged to the other, which is the case Frémaux et al. (2010) show a stimulus-specific critic is needed for.
- **A Q-value read by the selection** (option 3): it changes the selection and the reward together, so a verdict could not say which did what.
- **The critic in a record and the image**: a field, a format, a loader and a registry question, for a quantity the task already owns.
- **A schedule of reversals, another regime or another size first**: each would inherit couplings with no stop but the rail.

## Confirmation

`briefs/046_the-reward-prediction-error.md` builds the critic and runs H-19. Whitepaper 4.57.0 carries H-19 and its stopping rule in §11.1, H-18's stopping rule taken, and §9's row. `npm run spec` holds the links, the rows, the brief's sections and the version.
