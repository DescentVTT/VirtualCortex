---
status: accepted
date: 2026-10-11
depends-on: ADR-0163
decision-makers: VirtualCortex maintainers
---

# ADR-0164: An exploration the value gates, measured — H-32 is no, on clause 1 in both arms: the first reversal among three is learned in both arms and sooner, 113 and 117 of its last 128 where H-29 read 87 and 85, the second is learned in neither, 68 and 79, and the third is lost in one; the couplings bounded, the network held and the critic holding the expected reward; the regime's onset as derived and its level lower, the gate's reading spread among three; a reward on a drawn selection teaching its pair about as one on the gate's does; the gate itself relearning the first reversal sooner than in H-29 and the later ones no sooner; step 5 of the stopping rule, the line stops among three answers, and the exploration stays in the tree unset; the protocol written before any rewarded run of it, and each arm leaving H-29's at the trial pinned beforehand from H-29's own trials, the fourth and the tenth

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

### The calibration, as it ran (2026-10-09)

- **Unset, every pinned number held.** The tree's 93 other whole-domain tests passed on the developer machine at the protocol's commit, `92bb4b1`, from 13:33Z to 16:44Z: H-29's, H-30's and H-31's six arms among them, through `answered_run` as it now is.
- **Set, each arm left H-29's at the trial pinned beforehand.** Each arm's first block, run unset, was H-29's pinned first block, table for table; run set, it was the unset run trial for trial up to the pinned trial and drew there.

  | Arm | The trial | H-29's trial there | H-32's |
  | :--- | :--- | :--- | :--- |
  | From the assignment | index 3, value −1 600 | counts 1, 4, 3; readout 1 selected, wrong; received −63 936 | the same counts and value; readout 2 drawn, wrong; received −63 936 |
  | From the mirrored assignment | index 9, value −4 863 | counts 2, 6, 1; readout 1 selected, correct; received 70 399 | the same counts and value; readout 2 drawn, wrong; received −60 673 |

  From the assignment the two trials differ in the selection alone, and so in the address the trial wrote: both are punished, by the same error. From the mirrored assignment the draw turned a reward into a punishment.
- **Then the arms ran once**, side by side, from 16:44Z to 17:06Z, every oracle and hand rule held at every one of their 7 680 trials.
- **The dumps were read a third time**, by the rule written in another language beside SplitMix64's finaliser. At every trial of both arms the trials drawn, their channels, what is correct and what the modulator received are that writing's, and the hash of the readings with their draws is the dump's.

### H-32's verdict: no, on clause 1 in both arms

The arms ran on 2026-10-09. Their dumps were read, and what follows written, on 2026-10-11.

**Clause 1 fails in both arms.** Each mapping's last 128 trials, with each stimulus's correct selections of its presentations, and H-29's beside:

| Arm | The first mapping | The first reversal's | The second reversal's | The third reversal's |
| :--- | :--- | :--- | :--- | :--- |
| From the assignment | 118 (A 59 of 61, B 59 of 67) | 113 (61 of 64, 52 of 64) | **68** (33 of 62, 35 of 66) | **77** (51 of 54, **26 of 74**) |
| H-29's | 118 | 87 | 82 | 109 |
| From the mirrored assignment | 118 (59 of 61, 59 of 67) | 117 (59 of 64, 58 of 64) | **79** (35 of 62, 44 of 66) | 110 (51 of 54, 59 of 74) |
| H-29's | 119 | **85** (A 30 of 64) | 99 | 114 |

- The first reversal's mapping is learned in both arms, the one H-29 failed among them.
- The second reversal's is learned in neither: 68, and 79 where 80 are asked.
- The third reversal's is lost from the assignment, where B selects its answer in 26 of 74 presentations.
- Over the run five mappings of eight are learned. H-29 learned seven, H-31 six and H-30 four.

**Clause 2 holds.** The highest coupling is 1.260 of its image's from the assignment and 1.254 from the mirrored assignment (H-29: 1.242 and 1.261).

**Clause 3 holds.** The excitatory sum outside the six couplings stays within two parts in ten thousand of the image's.

**Clause 4 holds.** Each stimulus's mean value over each mapping's last 128 trials stands within 0.21 of the reward of $2a - 1$, the bound a quarter. The widest are the second reversal's mappings, where the accuracy was still rising: −0.13 against 0.06, and −0.07 against 0.13.

**H-32 is no.** ADR-0162 wrote no prediction for the verdict.

### The predicted readings

| Reading | Held | What was read |
| :--- | :--- | :--- |
| (a) the new answer selected early | at 11 of 12 | Over the second to the eighth block after a flip a stimulus selected its new answer 37 to 62 times, where H-29's same stimulus did 7 to 51 times. The one that did not hold: A after the mirrored arm's second flip, 49 against 51. |
| (b) the value settles higher | at 12 of 12 | The lowest block mean after a flip is −0.51 to −0.72 of the reward, where H-29's were −0.58 to −0.95. |
| (c) every reversal faster | at 2 of 6 | The first reversal passes 40 of 64 in its 23rd and 18th block, where H-29's did in its 31st and 30th. The second never does from the assignment and does in its 32nd from the mirrored one (H-29: 31st and 23rd). The third does in its 23rd in both (H-29: 21st and 23rd). |
| (d) the exploration switches itself off | at 5 of 5 | No trial of the last 128 of a learned mapping is drawn. Of the three mappings not learned, of which the reading says nothing, 13, 24 and 5 were. |
| (e) the couplings not worn down | at 6 of 8 | The six couplings' sum ends the assignment arm's third and fourth mappings at 0.9889 and 0.9898 of the image's, just under the 0.99 asked, and stands at 0.998 or above at the other six ends. Its lowest over a run is 0.967 and 0.974 (H-29: 0.981 and 0.991; H-30's sank to 0.93). |

### The regime against what was read

ADR-0162 derived: no exploration until the value passes zero, about 25 presentations after a flip; then, while the gate still selects the old answer, three presentations in five drawn, the value at −0.6, and the selections one fifth the new answer, three fifths the old and one fifth the third.

| Arm, flip, stimulus | Presentations until the value is below zero | Trials the gate's own reading is the old answer | Of them drawn | Mean value there | Selected: new, old, third |
| :--- | ---: | ---: | ---: | ---: | :--- |
| Assignment, first, A | 29 | 351 | 0.45 | −0.39 | 0.14, 0.72, 0.15 |
| Assignment, first, B | 25 | 351 | 0.39 | −0.34 | 0.15, 0.74, 0.11 |
| Assignment, second, A | 28 | 455 | 0.48 | −0.54 | 0.15, 0.69, 0.17 |
| Assignment, second, B | 22 | 496 | 0.45 | −0.45 | 0.14, 0.68, 0.17 |
| Assignment, third, A | 3 | 217 | 0.36 | −0.30 | 0.12, 0.78, 0.11 |
| Assignment, third, B | 2 | 372 | 0.40 | −0.44 | 0.15, 0.72, 0.13 |
| Mirrored, first, A | 28 | 398 | 0.44 | −0.42 | 0.15, 0.72, 0.13 |
| Mirrored, first, B | 26 | 308 | 0.41 | −0.39 | 0.12, 0.71, 0.17 |
| Mirrored, second, A | 25 | 445 | 0.41 | −0.47 | 0.14, 0.71, 0.15 |
| Mirrored, second, B | 27 | 346 | 0.40 | −0.38 | 0.12, 0.74, 0.14 |
| Mirrored, third, A | 4 | 279 | 0.39 | −0.32 | 0.14, 0.72, 0.14 |
| Mirrored, third, B | 16 | 375 | 0.36 | −0.35 | 0.11, 0.76, 0.13 |

- **The onset is as derived** at the first two flips: 22 to 29 presentations. At the third it is 2 to 16, because the mapping before it had not been learned and the value stood near zero at the flip.
- **The regime settles lower than derived.** Where the gate's reading is the old answer, 36 to 48 trials in a hundred are drawn and the value stands at −0.30 to −0.54, where the derivation gives 60 and −0.6. Over a reversal's second to eighth blocks, all presentations counted, 46 to 55 in a hundred are drawn and the value stands at −0.44 to −0.57.
- **Where the derivation's premise differed from the run.** It took the gate to read one readout at a time, the old answer and then the new. In the run the gate's reading is spread: over the second reversal's ninth to sixteenth blocks it is the old answer at 44 to 55 presentations in a hundred, the new at 14 to 23, a tie at 13 to 16 and the third readout at the rest. The stimulus is then right more often than a draw alone would make it, the value stands higher, and less is drawn.
- **It switches itself off only once the gate's own reading is right about half the time**, since the value that gates it follows the accuracy. Over the second reversal's 25th to 32nd blocks the gate's own reading is right at 43 and 51 presentations in a hundred and 28 and 24 in a hundred are still drawn.

### The readings

**The draws.** 2 108 and 1 912 of an arm's 7 680 trials are drawn, 27 and 25 in a hundred: 2 and 8 in a hundred of the first mapping's, where the value dips below zero before the first answers are learned, and 26 to 43 of a mapping's after a flip. They fall on the three readouts alike, 714, 699 and 695 by role, and 612, 624 and 676, so 34 and 32 in a hundred are rewarded. 72 and 73 in a hundred are another readout than the gate's own reading.

**What a reward consolidated in the pair it reached**, summed weight a trial in the trial after it, a pair's image coupling being 3.1 to 3.4 million:

| Arm | The gate's selection rewarded | The gate's punished | A drawn selection rewarded | A drawn one punished |
| :--- | :--- | :--- | :--- | :--- |
| From the assignment | 2 774 trials, +1 644 | 2 088, −2 005 | 714, +1 308 | 1 394, −1 082 |
| From the mirrored assignment | 2 988, +1 706 | 2 066, −2 053 | 612, +1 758 | 1 300, −1 083 |

- A reward on a drawn selection raises the pair it reaches by 0.80 and 1.03 of what a reward on the gate's own selection does. ADR-0162 named this as unread on readouts of 204 units: it is now read, and a drawn readout that did not win the count is taught about as a selected one is.
- A punishment of a drawn selection lowers its pair by 0.54 and 0.53 of what a punishment of the gate's own does.
- One rewarded trial moves a pair by about five parts in ten thousand of its image's coupling.

**What the exploration changed, beside H-29.**

- *The new answer's selections.* Over each mapping after a flip the stimuli selected their answers 945, 555 and 841 times from the assignment (H-29: 478, 491 and 945) and 950, 642 and 929 times from the mirrored assignment (H-29: 657, 859 and 810). Over a run 3 488 and 3 601 trials are correct, where H-29's were 3 085 and 3 420.
- *The old answer leads for fewer blocks*: 2 to 12 after a flip, where H-29's led for 5 to 24.
- *The punishments of the old answer are fewer and harder.* After the first flip a stimulus selected its old answer 281 to 344 times and received −0.67 to −0.72 of the reward a time, where H-29's selected it 352 to 615 times and received −0.31 to −0.63. After the second flip: 350 to 425 times at −0.54 to −0.68 (H-29: 187 to 466 times at −0.34 to −0.65). The regime gave −0.4.
- *The old answer's coupling* stands at 1.18 to 1.26 of its image's before the first two flips and ends the mapping after at 0.92 to 1.00 (H-29's: 1.10 to 1.26, and 0.95 to 1.01).

**The gate's own reading**, read from the runs' trials, which the pinned hashes hold. The share of presentations at which the largest count alone is the answer, over a reversal's blocks 2 to 8, 9 to 16, 17 to 24 and 25 to 32, with H-29's:

| Reversal | From the assignment | H-29's | From the mirrored assignment | H-29's |
| :--- | :--- | :--- | :--- | :--- |
| The first | 0.07, 0.27, 0.52, 0.83 | 0.04, 0.10, 0.22, 0.57 | 0.09, 0.23, 0.58, 0.81 | 0.08, 0.19, 0.45, 0.57 |
| The second | 0.06, 0.14, 0.25, 0.43 | 0.10, 0.14, 0.24, 0.48 | 0.11, 0.23, 0.30, 0.51 | 0.16, 0.33, 0.52, 0.68 |
| The third | 0.20, 0.26, 0.45, 0.56 | 0.19, 0.33, 0.57, 0.76 | 0.13, 0.27, 0.51, 0.79 | 0.11, 0.22, 0.46, 0.79 |

- **In the first reversal the gate itself relearns sooner than in H-29.**
- **In the second and the third it relearns at H-29's pace or more slowly**, though the new answer is selected more often early in each.
- **The clause fails by the gate's own reading, not by the draws laid over it.** Over the last 128 trials of the three mappings not learned the gate's own reading is the answer at 70, 71 and 79 trials, where the selections are right at 68, 77 and 79.
- **What the draws cost and found.** Over a mapping after a flip a draw took a correct reading of the gate's away 66 to 119 times and found the answer where the gate's reading was another 142 to 225 times. Over the second reversal's last sixteen blocks the two are about even from the assignment, 74 against 78, and the draws cost more than they found from the mirrored one, 76 against 53.

**The couplings at the second flip.** The readout that becomes the new answer at the second flip is the one that was the third readout through the first reversal, where a third of the draws fell on it and were punished. It ended the first reversal at 0.91 to 0.93 of its image's, where H-29's ended at 0.94 to 0.96. The answer it has to pass stood at 1.19 to 1.26, where H-29's stood at 1.10 to 1.18. By the second reversal's end the new answer's coupling had reached 1.05 to 1.17 and the old had fallen to 0.94 to 0.99.

**What did not change.** The ties of the gate's own reading are 10 to 16 in a hundred of a mapping's trials, as in H-29; the margin and the largest count read as H-29's; and the inhibitory sum follows H-29's course to within three parts in a thousand, ending at 0.842 and 0.839 of the image's.

### What is read from the tables and not measured

- **Three of the tables above are read by scripts outside the tree**: the gate's own reading by span of blocks, what the draws cost and found, and the shares drawn by span. The trials they read are the ones the pinned hashes hold, and no test recomputes them. Every other number above is a pinned table's or a rule's in `tests/inhibition.rs`.
- **Why the first reversal gains and the later ones do not is not measured.** No run varied where the couplings stood at a flip, or switched the exploration on for one reversal alone.
- **An account that fits the rows above**: a reversal is a gap between two couplings to be crossed at about five parts in ten thousand a rewarded trial, whoever makes the selection. The exploration brings the rewarded selections of the new answer forward and punishes the old answer harder, which closed the first gap inside 32 blocks. It also punishes the third readout, which is the next reversal's answer, and it learns the first reversal's answer higher, so the second gap is 0.25 to 0.33 of an image coupling where H-29's was 0.16 to 0.24. And while the gate's own reading is right less than half the time the draws go on, at a third right.
- **Another reading the rows do not exclude**: the later reversals are slower in H-32 for a reason that is not the exploration's, since H-31's later reversals were slower too.

### The step reached

**Step 5: the line stops among three answers.** Clause 3 holds and clause 1 does not.

Four rounds have now read no on a choice among three: H-29, and three that each changed one thing from its configuration.

| Round | The one change | Mappings learned, of eight | What it bought | What it cost |
| :--- | :--- | ---: | :--- | :--- |
| H-29 | three readouts where H-25 had two | 7 | — | — |
| H-30 | a punishment the value does not soften | 4 | the first reversal in 19 and 22 blocks | the two after it, the couplings worn to 0.93 |
| H-31 | a mapping after a flip of 48 blocks | 6 | the first reversal's mapping learned | the reversals after it slower, a coupling past 1.30 |
| H-32 | a selection drawn where the value is below zero | 5 | the first reversal's mapping learned, in 23 and 18 blocks | the second reversal's mapping in both arms, the third's in one |

**The next decision, named and not taken**: an ADR that states what the learning configuration holds of a choice among three — a first choice learned as between two, and a revision that is slow and does not hold over three — and returns the line to the list H-28's stopping rule left: a less reliable reward, a reward delayed past a trial, another size and the operating regime. No further mechanism is tried on three answers without a reading that names it.

What this round hands that ADR:
- each of the three changes tried after H-29 bought the first reversal and none held the two after it;
- a reward on a readout that did not win the count teaches its pair about as a reward on the selected one does;
- the selection's lack of noise is not, alone, what the revision lacks: over the second and the third reversals' second to eighth blocks the new answer was selected 1.6 times as often as in H-29, 403 times against 252, and the gate relearned those reversals no sooner.

**The exploration stays in the tree, unset**, as the whole punishment does: an option of the task that no run's configuration sets.

Steps 3, 4, 6 and 7 did not arise, and step 8 is kept: no constant moved after a rewarded run, and there was no second attempt.

### The evidence

- **The dispatch's scope.** The diff changes two files under `src/`, `task.rs` and `lib.rs`, so by [ADR-0075](0075-the-dispatch-scope-follows-the-diff.md) the weekly was dispatched on this round's branch at **`scope=both`**, as brief 066 asks: run [38111246283](https://github.com/DescentVTT/VirtualCortex/actions/runs/38111246283) at `c2e3e50`, the arms' tables, at 04:20:04Z on 2026-10-11. It ran the twelve whole-domain shards and the sweep's thirteen jobs.
- **Every whole-domain shard is green**, 04:21:27Z to 06:16:45Z. All ninety-five `exhaustive` tests passed, each exit 0 with one test reported. The ninety-three before this round reproduced their pinned numbers on the hosted runners, H-29's, H-30's and H-31's arms through `answered_run` as it now is. **The two arms reproduced their tables there, in 1 756 s from the assignment and 2 160 s from the mirrored assignment**, each holding its calibration first. H-29's arms took 1 370 and 856 s, H-30's 1 768 and 1 769, and H-31's 2 061 and 2 128.
- **The shards**, dealt by the table before this round, which did not know the two arms and costed each at 900 s:

  | Shard | Job | Its two heaviest tests (s) | Tests | Their seconds summed | Tests' wall time |
  | ---: | ---: | :--- | ---: | ---: | ---: |
  | 0 | 45.3 min | H-28 from the mirrored 1 767; H-27 from the mirrored 1 558 | 7 | 4 967 | 2 643 s, 37 % |
  | 1 | 28.8 min | H-25 from the assignment 1 079; H-24 from the assignment 961 | 8 | 3 173 | 1 665 s, 23 % |
  | 2 | 47.0 min | H-31 from the mirrored 2 128; H-28 from the assignment 1 371 | 8 | 5 353 | 2 753 s, 38 % |
  | 3 | 39.2 min | H-25 from the mirrored 1 782; H-19 from the assignment 1 130 | 9 | 4 510 | 2 258 s, 31 % |
  | 4 | 31.9 min | H-29 from the assignment 1 370; H-16's arms 900 | 7 | 3 632 | 1 836 s, 26 % |
  | 5 | 40.9 min | H-21 from the mirrored 1 393; the control from the mirrored 1 387 | 7 | 4 202 | 2 380 s, 33 % |
  | 6 | 33.7 min | H-23 from the assignment 1 388; H-22 from the assignment 1 386 | 8 | 3 799 | 1 952 s, 27 % |
  | 7 | 37.5 min | H-30 from the assignment 1 768; H-20 from the assignment 1 753 | 8 | 4 284 | 2 161 s, 30 % |
  | 8 | 36.8 min | H-21 from the assignment 1 402; H-18 from the assignment 877 | 6 | 3 568 | 2 135 s, 30 % |
  | 9 | 71.8 min | H-32 from the mirrored 2 160; H-23 from the mirrored 2 147 | 10 | 7 266 | 4 211 s, 58 % |
  | 10 | 51.4 min | H-27 from the assignment 1 800; H-32 from the assignment 1 756 | 8 | 5 338 | 2 997 s, 42 % |
  | 11 | 52.9 min | H-30 from the mirrored 1 769; H-22 from the mirrored 1 762 | 9 | 5 572 | 3 103 s, 43 % |

  The percentages are of the job's bound, 120 minutes; every shard ran its tests two at a time, at 1.67 to 2.00 of their summed seconds over the wall. **No shard passed 60 per cent of its bound.** The heaviest read 58: the deal had given shard 9 an arm it costed at 900 s, which took 2 160, beside a test that read 2 147 where the table held 1 374.
- **The runners were slow to start.** The dispatch's twenty-five jobs wait for twenty places (ADR-0158), and on this run the last whole-domain shard started at 05:23:53Z, an hour after the dispatch.
- **A test's seconds depend on the runner it drew.** In this run H-29's two arms read 1 370 and 856 s where the last dispatch read 1 733 and 1 588, and H-23's read 1 388 and 2 147. On the developer machine the two arms of H-32 ran side by side in 22 minutes.
- **The cost table is regenerated from this run** (`node scripts/exhaustive-costs.mjs from <artifacts> --run 38111246283`): 95 lines, 55 664 s, where the table before it held 93 and 48 211. ADR-0092's deal plans each of the twelve shards at 4 636 to 4 641 s summed, about 2 320 to 2 780 s of wall time at the run's ratios, **32 to 39 per cent of the bound**, inside the brief's 60.
- **The sweep had not ended when the shards did**, at 06:17Z ([ADR-0158](0158-the-sweep-off-the-next-decisions-path.md)). Four of its thirteen jobs had, each green and with no mutant missed: the state crates' (2 404 caught, 81 unviable, 6 timeouts) and the runtime's shards 2, 5 and 9 (105, 105 and 108 caught). Nine were running. Its outcome is written down by the first ADR that merges after it has ended; a survivor is a finding there.
- **The pull request's gate** ([196](https://github.com/DescentVTT/VirtualCortex/pull/196)) is green in every job on `92bb4b1`, the protocol, and on `3be8af0`, which holds the Rust sources of `c2e3e50`; the determinism pin on AArch64 and the MSRV are among them. The gate's run on `c2e3e50` itself was cancelled by the push of `3be8af0` eleven minutes after it. **The mutation gate on the changed lines** read 36 mutants, 32 caught and four unviable, none missed, on both.
- **On the developer machine**, at `c2e3e50`: the workspace's tests passed in the debug profile, in the release profile and on the MSRV, 730 passed and 135 ignored in each; the check, the format, Clippy, the documentation, the benchmarks and `npm run spec` exited 0; 95 whole-domain tests are listed. Before either arm ran, the tree's 93 other whole-domain tests passed there at `92bb4b1`, 13:33:42Z to 16:44:16Z on 2026-10-09. The two arms then ran once, 16:44:41Z to 17:06:32Z.
- **The commits**, by the hashes the branch holds, in pull request 196: `c0537bf`, the build; `92bb4b1`, the protocol and the run that reads a drawn selection; `c2e3e50`, the arms' tables and the gate's checks over them; `3be8af0`, these readings.

### Consequences

- Good: the run with the exploration set is held to as much as H-29's was, and at every drawn trial to a second writing of the rule.
- Good: where each arm leaves H-29's was known, and pinned, before any run with the exploration set.
- Good: H-29's, H-30's and H-31's arms run through the same `answered_run`, so the calibration with the exploration unset is also a check that the run's changes moved nothing.
- Neutral: two weekly tests more, each an H-29 arm with two blocks of calibration before it, 1 756 and 2 160 s on the hosted runners, and one test in the gate.
- Good: the question three stopping rules named is answered by a run: a selection that leaves the largest count, gated by the value with nothing to tune, does what was derived to the selections and the value, and does not make the revision among three hold.
- Good: the line among three answers ends on a stopping rule written before the run, after four rounds, and no fifth mechanism is tried without a reading that names it.
- Bad: why the first reversal gains and the later ones do not is read from the tables and not measured.
- Bad: an option stays in the task that no run's configuration sets.
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
