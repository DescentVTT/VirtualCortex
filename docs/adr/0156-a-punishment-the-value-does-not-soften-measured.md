---
status: proposed
date: 2026-10-09
depends-on: ADR-0155
decision-makers: VirtualCortex maintainers
---

# ADR-0156: A punishment the value does not soften, measured — H-30's protocol written before any rewarded run: H-29's two arms from H-29's image with the flag's byte written and nothing else, H-29's four clauses by H-29's rules and each reversal within H-25's 23 blocks, ADR-0154's four predicted readings as rules against H-29's pinned tables, and the calibration's two readings pinned

## Context and Problem Statement

[ADR-0154](0154-a-punishment-the-value-does-not-soften.md) wrote H-30 before any run and [ADR-0155](0155-a-punishment-the-value-does-not-soften-built.md) built its rule. This ADR is the measurement's: the protocol, committed before the first rewarded run of it, and then what was read.

**H-30**, restated from ADR-0154 and pinned in the tests. On H-29's configuration, readouts, schedule and arms, with the whole punishment set:
- **clause 1, the learning holds**: in both arms every mapping is learned — at least 80 of its last 128 trials correct, and among them each stimulus selecting its answer in more than half of its presentations;
- **clause 2, the couplings stay bounded**: no stimulus–readout coupling passes 1.30 of its image's at any block's end;
- **clause 3, the network holds**: at every block's end the summed magnitude of every excitatory synapse outside the six couplings lies within 0.75 and 1.25 of the image's;
- **clause 4, the critic holds the expected reward**: for each stimulus and each mapping over its last 128 trials, the engine's mean value lies within a quarter of the reward of $(2p - 1)$ of it;
- **clause 5, the revision is in time**: each of an arm's three reversals passes 40 of 64 within 23 blocks of its flip, the crossing block counted.

Yes when all five hold in both arms. **Predicted: yes.**

What was read first (principle 2), on 2026-10-09:

1. **H-29's arm** (`tests/inhibition.rs`, `answered_arm`, `answered_run`): the image built link by link by `answered_images` and held by its CRC; the deal held to the readouts' rule by `deal_calibration`; 7 680 trials with the critic's oracle and the network's held at every trial; the tables `ANSWERED_*_1024` pinned per block, every trial's reading pinned by one hash.
2. **H-29's trials are not in the tree**, only their hash. ADR-0153's table of the punishments after a flip was read from the arms' dumps.
3. **H-25's clause 3** (`reversals_within`, `REVERSAL_BLOCKS_MAX` 23, `CROSSING_MARK` 40) reads an array of four crossings, and H-29's reader gives one (`answered_crossings`).
4. **The first trials of an arm.** Every value weight is zero in the image, so the first reward meets a value of zero. A punishment moves the counted units' weights down, and the next presentation of that stimulus meets a value below zero. The first trial at which a reward below zero meets a value below zero is therefore within the first trials, and up to it the parameter cannot act.

## Decision Drivers

- H-30's stopping rule: the constants and the calibration's readings committed before the first rewarded run; no constant moves after one; no second attempt.
- **One change from H-29**: the image's one byte. The readouts, the deal, the schedule, the seed and the arms are H-29's.
- **H-29 beside it**: every reading of H-30 has H-29's read by the same rule, from tables the tree holds.
- **The round is evidenced as ADR-0150 says**: the arms run once, and the dispatch reproduces them.

## Considered Options

1. **The calibration's comparison with H-29's arm**: (a) each arm's first block run twice in the arm's own test, from H-29's image and from the same image with the flag written, the unset block held to H-29's pinned first block and the two runs held to one another trial for trial; (b) H-29's trials pinned whole in the tree and the arm's held to them; (c) the unset block alone, held to H-29's pinned first block.
2. **How the arm is held to the calibration**: (a) the arm's first 64 trials asserted the calibration's set run's; (b) by determinism, unasserted.
3. **H-29's trial-level readings**: (a) read by the rule this round writes, in H-29's own arms, and pinned there; (b) from the dumps of the last round's runs, outside the tree.
4. **The verdict's type**: (a) H-29's verdict whole and clause 5 beside it; (b) a new verdict of five fields.
5. **The tests**: (a) two weekly tests, each arm's calibration inside it, and one test in the gate; (b) a third weekly test for the calibration.
6. **The ADRs**: (a) two, the build's and the measurement's; (b) one.

## Decision Outcome

**Options 1(a), 2(a), 3(a), 4(a), 5(a) and 6(a).**

### The protocol (written before any rewarded run)

- **The arms** (`WHOLE_ARMS`): H-29's two, each a weekly `exhaustive` test, `a_punishment_the_value_does_not_soften_from_the_{assignment,mirrored_assignment}_at_1024_units_exhaustive`.
- **The image.** H-29's image is built and held as H-29's arms build and hold it, by its CRC at format 21 and by its CRC with the header's version written back to 20, the one H-29 read (`WINDOWED_IMAGE_CRC_FORMAT_20_1024`). The image an arm decodes is that image with the modulator section's `[51]` written and the section re-sealed (`with_whole_byte`). A masked check holds that it differs from H-29's in that byte and the section's seal alone, and that writing zero back gives H-29's image bit for bit (`only_the_whole`). Its CRC is pinned: `0x9bd4d0dbfc9e78c0`, 9 bytes from H-29's.
- **The run** is H-29's (`answered_run`): H-29's deal, held again to the readouts' rule on the image; H-25's delivery; H-20's flips; H-29's seed. At every trial the run holds the critic's oracle, the hand rule of what the modulator received with the signal's course from it (ADR-0155), the address, and the network's oracle over every excitatory synapse.
- **The clauses**, by rule:
  - clauses 1 to 4 by H-29's `answered`, unchanged;
  - clause 5 by H-25's `reversals_within` over `answered_crossings`;
  - the verdict `Whole { answered, in_time, yes }` (`unsoftened`), and the step it reaches (`whole_step`): 3 for a yes; 4 for a no on clause 3; otherwise 5 for a no on clause 1; otherwise 6 for a no on clause 5; otherwise 7 for a no on clause 2; otherwise 8.
- **The predicted readings**, as rules and never asserted, each against H-29's pinned tables of the same arm:
  - (a) `lets_go_sooner`: per flip and stimulus, the old answer led for fewer blocks than in H-29 (`old_held` of each);
  - (b) `faster`: per reversal, fewer blocks to 40 of 64 than its own in H-29; one that never passes is not faster;
  - (c) `recovers_sooner`: per flip and stimulus, fewer of the mapping's blocks with the stimulus's mean value below minus half the reward than in H-29 (`below_half` of each; over H-29's pinned blocks the rule reads 5 to 24, ADR-0153's reading);
  - (d) `old_below_image`: per flip and stimulus, the coupling into the old answer's readout below the image's at the mapping's last block's end.
- **The readings, no clause.**
  - *By trial* (`trials_read`), per mapping: the punished trials whose value was below zero — how many, the errors the critic took, summed, and what the modulator received, summed; the punished trials whose value was at or above zero; the rewarded trials; per flip and stimulus the trials that selected the old answer, with those of them under a value below zero and the same two sums; the signal after the reward at or beyond each of the gate's bounds, with its lowest and highest; and the margin, the trials whose largest count led the next by at most two spikes.
  - *By block*, by H-29's reader over this run (`answered_read`): where the wrong selections went and the ties, the six couplings' courses by role with the third readout's among them, where the consolidation went, the inhibitory sum's course, the value's troughs.
  - **H-29's beside each.** The by-block readings of H-29 are its pinned `ANSWERED_READINGS_1024`. Its by-trial readings are read by `trials_read` in H-29's own arms and pinned as `ANSWERED_TRIALS_1024` (option 3(a)). The pin was first written from the dumps of the last round's two runs, each checked against the arm's pinned hash of every trial's reading by a second writing of the hash; H-29's arms hold it from this round on. No pinned number of H-29's moves.

### Why these options

- **Option 1(a).** The stopping rule asks that each arm *be* H-29's arm trial for trial up to the first trial at which the rule can act. H-29's trials are not in the tree, so the arm's test runs H-29's first block itself, holds its tables to H-29's pinned first block, and holds the set run to it trial for trial in every number read. Pinning H-29's 7 680 trials whole (option 1(b)) would add two tables of that length for a comparison that ends within the first block. The unset block alone (option 1(c)) would not show where the two part.
- **Option 2(a).** The arm decodes the image again and runs; asserting its first 64 trials against the calibration's set run makes the stopping rule's sentence a statement about the run that is read.
- **Option 3(a).** A reading stated beside H-29's must be H-29's by the same rule. Read outside the tree (option 3(b)), it would be a number no test reproduces.
- **Option 4(a).** Clauses 1 to 4 are H-29's by H-29's rules, so the verdict holds H-29's verdict as it is and cannot drift from it.
- **Option 5(a).** A third weekly test (option 5(b)) would make each arm depend on another test's having run, which ADR-0153 rejected for the same reason.
- **Option 6(a).** The build stands if H-30 reads no; its decision and its tests are its own record.

### The calibration (H-30's stopping rule, step 2)

- **With the parameter unset, every pinned number of the tree holds.** The whole-domain tests of the tree ran on the developer machine at the build's sources before any rewarded run of the protocol; the result is recorded with the evidence below. The six whole-image pins moved with the format and nothing else did (ADR-0155).
- **With the parameter set, each arm is H-29's up to the first trial at which a reward below zero meets a value below zero.** In each arm's test the first block runs from H-29's image and from the same image with the flag written (`whole_calibration`):
  - the unset block's tables are H-29's pinned first block, table for table;
  - up to the trial, the two runs are one in every number read of every trial;
  - at the trial they present the same stimulus to the same counts, selection and value; unset the modulator received the reward less the value and set the reward; and the two signals part by exactly the value (`parts_at_first_met`).

  | Arm | The first trial met | The value there | Received unset | Received set |
  | :--- | :--- | ---: | ---: | ---: |
  | from the assignment | the fourth, index 3 | −1 600, 0.024 of the reward | −63 936 | −65 536 |
  | from the mirrored assignment | the seventh, index 6 | −208, 0.003 of the reward | −65 328 | −65 536 |

  Both are pinned (`WHOLE_FIRST_MET_1024`). The arm's own first 64 trials are then asserted the calibration's.
- **The gate** (`the_clauses_of_h_30_the_whole_punishment_s_patch_and_the_readings_rules`): the constants as ADR-0154 wrote them; the hand rule at its cases and against the engine's rule; the image's patch on the instrument's network, read back by the loader and refused without the critic; clause 5 at its edge, 23 blocks and 24, for each reversal of each arm, and each clause's place in the stopping rule's order over tables written by hand; the readings' rules over tables and trials written by hand; and eight trials of the task with the parameter set beside the same eight unset, every oracle held at every trial of both and the two held to one another as the calibration holds them. On that network the two part at the fifth trial, against a value of −116.

## Consequences

- Good: one byte of the image is the one change, shown by a masked check, with H-29's image held as H-29 read it.
- Good: the calibration shows where the parameter first acts and that nothing differs before it, in the test that then runs the arm.
- Good: every reading has H-29's beside it by the same rule, from tables the tree holds.
- Neutral: two weekly tests of about H-29's cost, and one test in the gate.
- Neutral: H-29's arms gain one held table, their trials by this round's reader.
- Bad: the calibration's two first blocks lengthen each arm by 128 trials under every oracle.

## Alternatives considered and why rejected

- **H-29's trials pinned whole** (option 1(b)), **the unset block alone** (option 1(c)), **the arm unasserted against its calibration** (option 2(b)), **H-29's by-trial readings outside the tree** (option 3(b)), **a new verdict** (option 4(b)), **a third weekly test** (option 5(b)), **one ADR** (option 6(b)): see above.
- **A two-answer arm under the parameter in this round**: brief 063 does not empower it; H-30's step 3 names it as the next decision's first question.

## Confirmation

- `runtime/cortex-runtime/tests/inhibition.rs`: `WHOLE_ARMS`, `with_whole_byte`, `only_the_whole`, `Whole`, `unsoftened`, `whole_step`, `TrialsRead`, `trials_read`, `below_half`, `lets_go_sooner`, `faster`, `recovers_sooner`, `old_below_image`, `WholeRead`, `whole_read`, `first_met`, `parts_at_first_met`, `whole_calibration`, `whole_arm`, the two weekly tests and the gate's test named above.
- The pins: `WHOLE_IMAGE_CRC_1024`, `WHOLE_FIRST_MET_1024`, `ANSWERED_TRIALS_1024`.
