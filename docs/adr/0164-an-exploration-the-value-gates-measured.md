---
status: proposed
date: 2026-10-09
depends-on: ADR-0163
decision-makers: VirtualCortex maintainers
---

# ADR-0164: An exploration the value gates, measured — H-32's protocol written before any rewarded run of it: H-29's two arms from H-29's image with the task's exploration set and nothing else; H-29's four clauses by H-29's rules; ADR-0162's five predicted readings and its regime as rules; the run's selection held at every trial to hand rules that read a drawn selection; each arm held to H-29's trial for trial up to the trial its coin first draws at, the fourth and the tenth, found from H-29's own trials before any run

## Context and Problem Statement

[ADR-0162](0162-an-exploration-the-value-gates.md) wrote H-32 before any run, and [ADR-0163](0163-an-exploration-the-value-gates-built.md) built the exploration in the task. This ADR is the measurement's: the protocol, committed before any rewarded run of it, and then what was read.

**H-32**, restated from ADR-0162 and pinned in the tests. On H-29's configuration, readouts, schedule and arms, with the exploration set:
- **clause 1, the learning holds**: in both arms every mapping is learned — at least 80 of its last 128 trials correct, and among them each stimulus selecting its answer in more than half of its presentations;
- **clause 2, the couplings stay bounded**: no stimulus–readout coupling passes 1.30 of its image's at any block's end;
- **clause 3, the network holds**: at every block's end the summed magnitude of every excitatory synapse outside the six couplings lies within 0.75 and 1.25 of the image's;
- **clause 4, the critic holds the expected reward**: for each stimulus and each mapping over its last 128 trials, the engine's mean value lies within a quarter of the reward of $(2a - 1)$ of it, with $a$ that stimulus's accuracy over those trials.

Yes when all four hold in both arms. **No prediction for the verdict.**

**The last dispatched sweep** ([ADR-0158](0158-the-sweep-off-the-next-decisions-path.md)): brief 064's round dispatched the whole-domain shards alone, and the sweep before it is written down by [ADR-0160](0160-the-sweeps-tests-under-nextest.md). No sweep's outcome is owed here.

What was read first (principle 2), on 2026-10-09:

1. **H-29's run holds its selection to the gate's hand rule** (`runtime/cortex-runtime/tests/inhibition.rs`, `answered_run`): at every trial, `outcome.selection` is `largest_alone` of the counts. With the exploration set that is false at every drawn trial.
2. **The critic's oracle's value is of the weights and the counts alone** (`tests/instrument/harness.rs`, `value_step`). The run forms it after the trial from the train, and it reads nothing of the selection or the reward.
3. **The address and the network's oracle read the record** (`answered_run`, `Network::replay_each`): the run holds the targets to the readout the trial's outcome names, and the oracle replays every excitatory synapse under the address and the signal it reads from the executor after each trial. Neither needs to know how the selection was made.
4. **H-29's trials are not in the tree**, only their hashes and the blocks. The last round's dumps of H-29's two arms hold every trial, and each reproduces its arm's pinned hash (`ANSWERED_READ_1024`).
5. **Until an arm's first drawn trial it is H-29's arm**, by ADR-0163's construction: no line a trial runs differs but the read of the option and of the value. So H-29's own trials say where each arm's coin first draws.
6. **H-32's stopping rule numbers its steps as H-29's does**: 3 for a yes, 4 for a no on clause 3, then 5, 6 and 7 for a no on clauses 1, 2 and 4. `answered_step` gives them.
7. **The harness's reward is 1.0** (`REWARD_Q16`), at which ADR-0163's comparison is the coin against the value's part below zero itself.

## Decision Drivers

- H-32's stopping rule: the bits and the constants committed before the first rewarded run; no constant moves after one; no second attempt.
- **One change from H-29**: the selection. The image, the readouts, the deal, the schedule, the seed and the arms are H-29's.
- **"By H-29's rules" is to be literal**: one writing of each rule, read by both rounds.
- **The engine is read before a description of it is trusted**: a drawn selection is held to a second writing of the rule at every trial, not to the task's.
- **H-29 beside it**: every reading of H-32 has H-29's read by the same rule, from tables the tree holds.
- The round is evidenced as [ADR-0150](0150-a-round-waits-for-what-it-checks.md) says. Latest ≠ Newest: nothing is adopted.

## Considered Options

1. **How the run reads a selection that is sometimes drawn**: (a) `answered_run` takes the task's exploration, and holds every trial's draw and selection to hand rules over the critic's oracle's value; (b) a run of its own for H-32; (c) the run holds the record to the task's own functions.
2. **The hand rules' form**: (a) the coin by a shift and a remainder, the comparison as the coin against the value's part below zero itself at a reward of 1.0, the channel by the thirds of 32 768 written out; (b) the task's masks and its multiplication again.
3. **How the calibration against H-29 is made**: (a) each arm's first block run unset and set before the arm's run, in the arm's own test, the unset one held to H-29's pinned first block and the two to one another; (b) inside the arm's own run by a closure; (c) a weekly test of its own.
4. **Where the trial an arm first draws at comes from**: (a) from H-29's own trials by the rule written apart from the tree, before any run with the exploration set, and pinned; (b) read from the calibration's run and pinned after.
5. **What a trial's record gains in the harness**: (a) the draws and each trial's consolidation by pair kept beside the run, outside every hash and table of the rounds before; (b) a wider trial, and every pinned hash moved.
6. **The verdict**: (a) H-29's `Answered` by H-29's `answered`, its step by `answered_step`; (b) a verdict of its own.
7. **The tests**: (a) two weekly tests, each an arm of 7 680 trials, and one test in the gate; (b) a third weekly test for the calibration.

## Decision Outcome

**Options 1(a), 2(a), 3(a), 4(a), 5(a), 6(a) and 7(a).**

### The harness

- **`answered_run` takes the task's exploration**, beside its delivery. Every call of H-29's, H-30's and H-31's arms and gates passes it unset, and their pinned numbers do not move.
- **The selection, by the hand rules**, held at every trial:
  - the trial is drawn exactly where the exploration is set and `explores_by_hand` says the trial's coin is below the part of the critic's oracle's value below zero; with it unset no trial is drawn;
  - a drawn trial's selection is `channel_by_hand`'s, whatever the counts; every other trial's is `largest_alone` of the counts, as it was;
  - `correct` is that selection against the answer in force by the schedule's hand rule, as it was.
- **The hand rules** (option 2(a)), each a second writing and not the task's:
  - `coin_by_hand`: `mix64` of the seed and the trial's index shifted down sixteen places, its remainder over 65 536;
  - `explores_by_hand`: the value below zero and the coin below its part below zero, at most the reward. A compile-time assertion holds the harness's reward to 1.0, where that is ADR-0163's comparison;
  - `channel_by_hand`: the same draw shifted down thirty-three places, its remainder over 32 768, below 10 923 the first readout, below 21 846 the second, the third from there.
- **The order of a trial's checks.** The selection's assertions now follow the critic's oracle, whose value they read. The oracle's value is formed from the weights and the counts before the reward is read, so it stands on nothing the selection decides.
- **Everything else the run held, it holds**: the value, the error, every weight and the window against the critic's oracle; what the modulator received and the signal's course from it; the address; and every excitatory synapse against the network's oracle. At a drawn trial the address's targets are held to the drawn readout's units.
- **Beside the run** (option 5(a)): `AnsweredRun::drawn`, whether each trial was drawn, and `AnsweredRun::pairs`, each trial's consolidation by pair from the network's oracle. No hash and no table of an earlier round reads either.

### The protocol (written before any rewarded run of it)

- **The arms** (`EXPLORED_ARMS`): H-29's two, each a weekly `exhaustive` test, `an_exploration_the_value_gates_from_the_{assignment,mirrored_assignment}_at_1024_units_exhaustive`.
- **The image** is H-29's, built and held as H-29's arms build and hold it: by its CRC at format 21, `0x11586a76f7415129`, and by its CRC with the header's version written back to 20, the one H-29 read, `0x5044ed7936a7b5f3`. The engine decoded from it is asserted to carry the whole punishment unset.
- **The run** is H-29's (`answered_run`) on H-20's schedule, the one H-25 and H-29 ran on (`SCHEDULE`): H-29's deal, held again to the readouts' rule on the image; H-25's delivery; H-29's seed; the exploration set (`EXPLORED`).
- **The clauses**, by rule: H-29's `answered` over the two arms. The step is `answered_step`'s.
- **The predicted readings**, as rules and never asserted:
  - (a) `new_early`: per flip and stimulus, more selections of the new answer over the mapping's second to eighth blocks (`EARLY_BLOCKS`) than in H-29's same blocks, the presentations being the same trials';
  - (b) `settles_higher`: per flip and stimulus, the lowest block mean value of the mapping at or above −0.75 of the reward, read as `mean × 4 >= −3 × reward`;
  - (c) `faster`, brief 063's rule: per reversal, fewer blocks to 40 of 64 than its own in H-29;
  - (d) `switches_off`: per learned mapping, fewer than one trial in ten of its last 128 drawn, read as `drawn × 10 < 128`; a mapping not learned is not read;
  - (e) `not_worn`: per mapping, the six couplings summed at its last block's end at or above 0.99 of the image's six summed.
- **The regime**, restated from ADR-0162 as the numbers the run is read against and never held to (`REGIME_PRESENTATIONS`, `REGIME_FIFTHS`): no exploration until the value passes zero, about 25 presentations after a flip; then, while the gate still selects the old answer, three presentations in five drawn, the new answer selected in one of five, the old in three and the third in one, with the value at −0.6 of the reward. What is read against it (`regime`), per flip and stimulus:
  - the presentations from the flip to the first whose value is below zero;
  - from that presentation on, over the trials at which the gate's own reading is the old answer's readout: how many, those drawn, the value summed, and what they selected by role.
- **The readings, no clause** (`ExploredRead`).
  - *By block* (`ExploredBlock`, `explored_blocks`), per stimulus: the trials drawn; the value at them, summed; the drawn selections by the readouts' roles — the answer's, which are those rewarded, and the two others', which are those punished; and the drawn selections that were not the gate's own reading. Summed by mapping (`explored_by_mapping`), with the trials drawn over each mapping's last 128 (`drawn_last`).
  - *What a reward consolidated in the pair it reached* (`reached_by_mapping`), by the kind of trial — the gate's selection rewarded, the gate's punished, a drawn one rewarded, a drawn one punished: the trials of the kind, and over the trial after each what the network's oracle consolidated in the pair of the stimulus presented and the readout selected, raised and lowered. A trial's address stands until the next trial's end and its reward's signal decays over the next trial, so what the next trial consolidates in that pair is that reward's.
  - *The old answer*: the blocks it leads (`old_held`, H-29's reader); and its coupling at the block before each flip and at the end of the mapping the flip put in force (`old_couplings`).
  - *By H-29's reader* (`answered_read`): the block each mapping passes 40 of 64 in, and each stimulus's; where the selections went by role; the six couplings' courses and their highest; the value's troughs; where the consolidation went; the inhibitory sum's course.
  - *By brief 063's reader* (`trials_read`): the margin of the gate's own counts, the punished and the rewarded trials, the trials that selected the old answer, the signal at the gate's bounds. With them the ties of the gate's own reading (`gate_ties`).
  - *The sums*: the six couplings' sum at each mapping's end with its lowest, highest and last over the run; the inhibitory sum at each mapping's end.
  - **H-29's beside each**, from its pinned `ANSWERED_BLOCKS_1024`, `ANSWERED_READINGS_1024` and `ANSWERED_TRIALS_1024`.
- **The run's pinned tables**: the 120 blocks as H-29's are pinned; the exploration by block; the hash of the accuracy sequence; the hash of every trial's reading with its draw (`explored_hash`); the couplings after the first trial of each new mapping; and the readings.

### The calibration (H-32's stopping rule, step 2)

- **With the exploration unset, every pinned number of the tree holds.** The tree's whole-domain tests run on the developer machine at the protocol's commit, the two arms left out, before either arm runs. H-29's, H-30's and H-31's arms are among them, read through `answered_run` as it now is.
- **With it set, each arm is H-29's arm trial for trial up to the first trial at which the value is below zero and the coin draws, and differs from it there** (`explored_calibration`, `parts_at_first_drawn`). Each arm's test first runs the arm's first block twice from H-29's image, with the exploration unset and with it set, every oracle and hand rule held at every trial of both, and holds:
  - the unset block to H-29's pinned first block, table for table;
  - the two runs one, trial for trial in every number read, up to the first trial the hand rule draws at on the unset run, with no trial drawn before it;
  - at that trial the same stimulus, counts and value in both; the unset run's selection the largest count alone; the set run's trial recorded drawn and its selection the hand rule's channel;
  - the two trials different in a number read, which they are exactly where the channel is not the readout the gate selected;
  - the trial's index, the value there and the channel to their pins (`EXPLORED_FIRST_DRAWN_1024`).

  The arm's own run then holds its first block to the calibration's set block.
- **The pins were written before any run with the exploration set**, from the last round's dumps of H-29's two arms, each first checked against its arm's pinned hash, by ADR-0163's rule written in another language beside SplitMix64's finaliser:

  | Arm | The trial, by index | The value there | The coin | The channel drawn | H-29 selected |
  | :--- | ---: | ---: | ---: | ---: | ---: |
  | From the assignment | 3, the fourth | −1 600 | 168 | readout 2 | readout 1, wrong |
  | From the mirrored assignment | 9, the tenth | −4 863 | 2 389 | readout 2 | readout 1, correct |

  In both arms the first trial drawn is also the first whose channel is not H-29's selection, so each arm leaves H-29's there. The trial of index 3 draws so early because its coin is 168 of 65 536: any value below −0.0026 of the reward draws at it, and the value there is −0.024.
- **A calibration that fails stops the round there** as a finding: the build stays, and no arm's run of 7 680 trials is made.

### The gate

One test, `the_clauses_of_h_32_the_exploration_s_hand_rules_and_the_readings_rules`:
- the arms, the delivery, the exploration, the predictions and every constant as ADR-0162 fixed them;
- the hand rules against the oracle written apart from the tree — the coin of the first ten trials and the channel of the first sixteen at the harness's seed — at their cases, and against the task's own rule over every one of a run's 7 680 trials at fifteen values a trial;
- H-32's verdict by H-29's rules with the step it reaches, over tables written by hand: clause 1 at its edge, and each clause's place in the stopping rule's order;
- the readings' rules and the predicted readings' over tables and trials written by hand;
- twelve trials of the task with three readouts on the instrument's network with the exploration set beside the same trials with it unset, every oracle and hand rule held at every trial of both and the two runs held to one another as the calibration holds them. On that network the tenth trial is drawn, onto readout 2 where the gate selected readout 0.

### Consequences

- Good: the run with the exploration set is held to as much as H-29's was, and at every drawn trial to a second writing of the rule.
- Good: where each arm leaves H-29's was known, and pinned, before any run with the exploration set.
- Good: H-29's, H-30's and H-31's arms run through the same `answered_run`, so the calibration with the exploration unset is also a check that the run's changes moved nothing.
- Neutral: two weekly tests more, each an H-29 arm with two blocks of calibration before it.
- Bad: the hand rules' form is the rule's at a reward of 1.0 alone. A run at another reward would need the comparison written out.

## Alternatives considered and why rejected

- **A run of its own for H-32** (option 1(b)): a second writing of H-29's run would no longer be H-29's by construction.
- **The record held to the task's own functions** (option 1(c)) and **the task's masks again** (option 2(b)): the run would hold the task to itself.
- **The calibration inside the arm's run** (option 3(b)): H-31 needed that because its arm was H-29's for 56 blocks. Here an arm leaves H-29's within its first ten trials, and a first block run twice is cheaper to read.
- **A weekly test for the calibration** (option 7(b)): four blocks more a week for what each arm's test already holds.
- **The trial read from the calibration and pinned after** (option 4(b)): the pin would be the engine's own word. Written first from H-29's trials, it is a prediction the calibration can fail.
- **A wider trial** (option 5(b)): every hash of H-29's, H-30's and H-31's trials would move.

## Confirmation

- `runtime/cortex-runtime/tests/inhibition.rs`: `answered_run`, `AnsweredRun::{drawn, pairs}`, `coin_by_hand`, `explores_by_hand`, `channel_by_hand`, `EXPLORED_ARMS`, `EXPLORED`, `ExploredBlock`, `explored_blocks`, `explored_by_mapping`, `drawn_last`, `reached_by_mapping`, `old_couplings`, `Regime`, `regime`, `gate_ties`, `new_early_counts`, `new_early`, `settles_higher`, `switches_off`, `not_worn`, `ExploredRead`, `explored_read`, `explored_hash`, `first_drawn`, `parts_at_first_drawn`, `explored_calibration`, `explored_arm`, the two weekly tests, the gate's test, and `EXPLORED_FIRST_DRAWN_1024`.
- Whitepaper §9 and §11.1; `CHANGELOG.md`.
