---
status: accepted
date: 2026-10-09
depends-on: ADR-0155
decision-makers: VirtualCortex maintainers
---

# ADR-0156: A punishment the value does not soften, measured: H-30 is no, on clauses 1 and 5, where a yes was predicted — H-29's two arms from H-29's image with the flag's byte written and nothing else, each H-29's trial for trial up to its fourth or its seventh trial; the first reversal passed in 19 and 22 blocks where H-29's took 31 and 30, and its mapping was learned; the second and the third never passed 40 of 64 and their mappings were not learned, in both arms; the punishments under a value below zero, about half of all trials, were received whole, the signal stood at the gate's lower bound in half the trials after a flip, and the six couplings' sum sank to 0.93 to 0.96 of the image's with the readouts' answers; H-30's stopping rule reaches step 5, and the next decision, the one ADR-0153's step 5 names, is named and not taken

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

- **With the parameter unset, every pinned number of the tree holds.** The whole-domain tests of the tree ran on the developer machine at the build's sources before any rewarded run of the protocol, from 23:09Z on 2026-10-08 to 01:29Z on 2026-10-09: 89 of 89 passed, H-29's two arms among them. The six whole-image pins moved with the format and nothing else did (ADR-0155).
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

### What was read

The arms ran once each on the developer machine, from 01:30Z to 01:52Z on 2026-10-09, after the calibration's first half had ended green. Every oracle held at every one of each arm's 7 680 trials: the critic's value, error and weights; the hand rule of what the modulator received and the signal's course from it; the address; and every excitatory synapse of the arena. The tables are pinned from that run (`WHOLE_*_1024`). H-29's two arms ran beside them and held their trials by this round's reader to `ANSWERED_TRIALS_1024`.

### The verdict

**H-30 is no, on clauses 1 and 5, in both arms.** ADR-0154 predicted a yes.

| Clause | From the assignment | From the mirrored assignment |
| :--- | :--- | :--- |
| 1, each mapping's last 128 | 119, 116, **69**, **60** correct | 119, 113, **60**, **67** correct |
| 2, the highest coupling | 1.242 of its image's | 1.252 |
| 3, the excitatory sum outside the six | 0.9996 to 1.0000 of the image's | 0.9997 to 1.0000 |
| 4, the value against $2p - 1$ | within 0.22 of the reward | within 0.22 |
| 5, each reversal's blocks to 40 of 64 | 19, **never**, **never** | 22, **never**, **never** |

- **Clause 1 fails for the second and the third reversals' mappings in both arms**, by the count and by a stimulus. From the assignment the third mapping read A at 42 of 62 and B at 27 of 66, and the fourth A at 42 of 54 and B at 18 of 74. From the mirrored assignment the third read A at 21 of 62 and B at 39 of 66, and the fourth A at 37 of 54 and B at 30 of 74.
- **The first mapping and the first reversal's hold in both.** The first mapping is H-29's within a trial or two: 119 and 119, where H-29 read 118 and 119. The first reversal's mapping read 116 and 113, where H-29 read 87 and 85; the second of those was the mapping H-29 failed.
- **Clause 5 holds for the first reversal and fails for the two after it.** The first passed 40 of 64 in its 19th and its 22nd block, inside H-25's 23, where H-29's passed in its 31st and its 30th. The second and the third never passed it in their 32 blocks: their best blocks read 36 and 34 of 64 from the assignment, and 36 and 35 from the mirrored assignment. H-29's passed in 31 and 21, and in 23 and 23.
- **Clauses 2, 3 and 4 hold.** Clause 4 holds where a stimulus's accuracy is under a half: B's mean value stood at −0.21 and −0.52 of the reward against −0.18 and −0.51 over the last 128 trials of the assignment arm's third and fourth mappings. The weights move by the critic's own error, and the value held the expected reward below zero.

### The predicted readings

None was asserted. Each is read by its rule against H-29's pinned tables of the same arm.

| Predicted reading | Held | What was read |
| :--- | :--- | :--- |
| (a) the old answer lets go sooner | 8 of 12 | it led for 9, 10, 9, 16, 11, 5 blocks from the assignment (H-29: 24, 21, 22, 16, 6, 12) and for 11, 13, 13, 4, 7, 7 from the mirrored assignment (H-29: 22, 13, 5, 14, 16, 17) |
| (b) every reversal faster than its own | 2 of 6 | the first reversal of each arm; the four others never passed |
| (c) the value recovers sooner | 7 of 12 | blocks below minus half the reward: 7, 10, 17, 16, 6, 21 (H-29: 22, 19, 21, 21, 5, 18) and 10, 12, 19, 8, 10, 12 (H-29: 24, 11, 6, 15, 13, 12) |
| (d) the old answer's coupling below the image's at the mapping's end | 12 of 12 | 0.855 to 0.965 of the image's; in H-29's failed mapping it had ended at 1.010 |

- **(a) and (c) held at every flip and stimulus of the first reversal but one each, and at about half of those after it.**
- **(d) held everywhere, and says less than it was written to.** The old answer's coupling fell below the image's because every coupling did; see below.

### What the rule changed

The two rounds differ in one byte of the image, and each arm is H-29's trial for trial up to its fourth or its seventh trial. What follows differs between them because of the rule.

- **The punishments it reached.** A punished trial under a value below zero received the reward itself, −1.000, where the critic's error at the same trials averaged −0.44 to −0.87 of it. There were 3 801 of them in the assignment arm's 7 680 trials and 3 955 in the mirrored arm's: about half of all trials.

  | Arm | Mapping | Punished under a value below zero, H-29 → H-30 | Mean received in H-29 | The critic's mean error in H-30 | Mean received in H-30 |
  | :--- | :--- | ---: | ---: | ---: | ---: |
  | assignment | first | 137 → 137 | −0.87 | −0.87 | −1.00 |
  | | second | 1 417 → 820 | −0.30 | −0.49 | −1.00 |
  | | third | 1 472 → 1 456 | −0.38 | −0.44 | −1.00 |
  | | fourth | 972 → 1 388 | −0.52 | −0.53 | −1.00 |
  | mirrored | first | 292 → 260 | −0.77 | −0.74 | −1.00 |
  | | second | 1 229 → 922 | −0.34 | −0.46 | −1.00 |
  | | third | 1 070 → 1 454 | −0.49 | −0.51 | −1.00 |
  | | fourth | 1 081 → 1 319 | −0.45 | −0.58 | −1.00 |

- **The punishments of the old answer**, the twelve rows of ADR-0153's table. In H-29 they delivered a mean of −0.31 to −0.67 of the reward. In H-30 they delivered −1.00 to −1.07, a little past the reward where some met a value above zero:

  | Arm | Flip, stimulus | Trials that selected the old answer, H-29 → H-30 | Of H-30's, under a value below zero | Mean received, H-29 → H-30 |
  | :--- | :--- | ---: | ---: | ---: |
  | assignment | first, A | 604 → 287 | 234 | −0.33 → −1.07 |
  | | first, B | 554 → 312 | 256 | −0.38 → −1.06 |
  | | second, A | 466 → 365 | 333 | −0.34 → −1.03 |
  | | second, B | 419 → 444 | 429 | −0.46 → −1.01 |
  | | third, A | 192 → 313 | 289 | −0.67 → −1.01 |
  | | third, B | 368 → 309 | 309 | −0.51 → −1.00 |
  | mirrored | first, A | 615 → 338 | 290 | −0.31 → −1.05 |
  | | first, B | 352 → 307 | 249 | −0.63 → −1.07 |
  | | second, A | 187 → 402 | 383 | −0.65 → −1.02 |
  | | second, B | 404 → 349 | 328 | −0.48 → −1.02 |
  | | third, A | 401 → 286 | 268 | −0.50 → −1.01 |
  | | third, B | 364 → 341 | 333 | −0.57 → −1.00 |

  After the first flip the old answer was selected less often than in H-29 at all four rows, about half as often at three. After the second and the third it was selected more often at three rows of eight and less often at five.
- **The first reversal was shortened**, in both arms, by 12 and by 8 blocks, and its mapping's last block read 58 and 55 correct of 64, where H-29's read 43 and 43.
- **The reversals after it were lost.** In H-29 the second and the third reversals' mappings were learned in all four cases, with 82 to 114 of their last 128. In H-30 none was, and none passed 40 of 64 in any block.

### The signal

- **It stood at or below the gate's lower bound far more often.** After the reward the signal was at or below −1.0 in 799, 1 208 and 1 044 of the 2 048 trials of the assignment arm's mappings after a flip, and in 884, 1 130 and 976 of the mirrored arm's. H-29 read 201, 181 and 216, and 237, 237 and 258. In the first mapping the two rounds read alike, 244 and 289 against 222 and 236.
- **It stood at or above the upper bound somewhat less**: 279, 296 and 284, and 288, 298 and 333, where H-29 read 330, 388 and 363, and 313, 378 and 372.
- **Its range did not widen**: −2.96 to 2.75 over both arms, where H-29's was −3.02 to 2.82.
- **A reward came to a signal further below zero.** After a punished trial the signal averaged −1.21 to −1.36, where H-29's averaged −0.39 to −0.63 after a flip. A rewarded trial received 1.31 to 1.41 of the reward in the last two mappings and left the signal at 1.12 to 1.20.

### The six couplings

- **Their sum sank below the image's.** At each mapping's end, as a fraction of the image's six summed:

  | Arm | Round | First | Second | Third | Fourth |
  | :--- | :--- | ---: | ---: | ---: | ---: |
  | assignment | H-29 | 1.053 | 1.031 | 1.008 | 1.023 |
  | | H-30 | 1.052 | 1.032 | **0.959** | **0.933** |
  | mirrored | H-29 | 1.053 | 1.022 | 1.029 | 1.048 |
  | | H-30 | 1.047 | 1.008 | **0.954** | **0.953** |

- **By the readouts' roles**, at each mapping's end after a flip, as fractions of the image's, the answer's, the old answer's and the third's:

  | Arm | Mapping | From A | From B |
  | :--- | :--- | :--- | :--- |
  | assignment | second | **1.240**, 0.965, 0.937 | **1.220**, 0.912, 0.912 |
  | | third | **1.112**, 0.955, 0.886 | **1.016**, 0.944, 0.843 |
  | | fourth | **1.061**, 0.925, 0.914 | **0.940**, 0.883, 0.872 |
  | mirrored | second | **1.203**, 0.931, 0.888 | **1.165**, 0.965, 0.920 |
  | | third | **0.974**, 0.947, 0.893 | **1.098**, 0.914, 0.901 |
  | | fourth | **1.080**, 0.855, 0.932 | **1.039**, 0.940, 0.862 |

  After the first reversal the answer's coupling stood at 1.165 to 1.240, higher than in H-29 at three of the four, where it read 1.098 to 1.176. After the second and the third it stood lower than the 1.119 to 1.233 H-29 read, twice below the image's, and the two other readouts' stood at 0.84 to 0.96.
- **The third readout's pairs** stood at 0.843 to 0.937 of the image's at the end of a mapping after a flip, where H-29 read 0.893 to 0.972.
- **Where the consolidation went**, by the network's oracle, per mapping after a flip, in millions:

  | Arm | Round | The answer's pairs, net | The other four pairs, net | All six, net |
  | :--- | :--- | :--- | :--- | :--- |
  | assignment | H-29 | +1.32, +1.12, +1.67 | −1.75, −1.57, −1.36 | −0.43, −0.46, +0.31 |
  | | H-30 | +1.78, +0.91, +0.89 | −2.18, −2.34, −1.42 | −0.40, −1.43, −0.53 |
  | mirrored | H-29 | +1.14, +1.51, +1.88 | −1.76, −1.38, −1.50 | −0.62, +0.13, +0.38 |
  | | H-30 | +1.46, +0.86, +1.11 | −2.23, −1.94, −1.12 | −0.77, −1.08, −0.00 |

  The learning signal, the answer's net less the others', was 4.0 and 3.7 million over the first reversal's mapping, above H-29's 3.1 and 2.9, and 2.2 to 3.3 over the two after it, against H-29's 2.7 to 3.4. What changed more is the sum: over the second reversal's mapping the six couplings lost 1.4 and 1.1 million, where H-29's lost 0.5 and gained 0.1, and they did not regain it.

### The readouts

- **They came to answer less.** Over a mapping's last four blocks the answer's readout fired 7.7 to 11.3 spikes a presentation after the first reversal, where H-29's fired 6.3 to 9.1. After the third it fired 7.3 and 4.3 from the assignment and 6.6 and 5.3 from the mirrored assignment, where H-29's fired 12.1 and 7.4, and 10.8 and 8.9. The two other readouts fired 4.0 to 4.4, at or under the 4.4 to 5.7 a readout fires on the frozen network before anything is learned (ADR-0153).
- **The largest count** averaged 9.12, 8.05, 6.65 and 6.16 spikes over the assignment arm's four mappings and 8.89, 7.55, 6.40 and 6.19 over the mirrored arm's. H-29 read 9.14, 8.27, 7.31 and 8.00, and 9.03, 8.23, 7.62 and 7.91.
- **The margin** between the largest count and the next was at most two spikes in 40, 52, 66 and 71 per cent of a mapping's trials from the assignment and in 44, 55, 70 and 70 from the mirrored assignment; H-29 read 40 to 65.
- **Nothing was selected** in 10.5, 11.8, 17.9 and 18.5 per cent of a mapping's trials, and in 9.6, 13.2, 18.8 and 17.9; H-29 read 9.8 to 16.0.
- **Where the wrong selections went**: to the old answer, 58 to 74 per cent of them, where H-29 read 62 to 90. The third readout took 10 to 22 per cent of a mapping's trials, where H-29's took 6 to 19.

### The rest of the network

- **The inhibitory sum** rose to 1.010 and 1.008 of the image's and ended at 0.840 and 0.836, still falling; H-29's ended at 0.843 and 0.842.
- **Outside the six couplings** the excitatory sum moved by four parts in ten thousand at most.
- **The value's troughs after a flip** were −0.61 to −0.88 of the reward, where H-29's were −0.58 to −0.95.

### What this reads, and what it does not

- **The one change caused the difference.** The two rounds share every input but one byte, and each arm is H-29's until its fourth or its seventh trial. A faster first reversal and two lost ones after it are the rule's doing on this instrument.
- **ADR-0153's account held for the first reversal.** A punishment the value does not soften shortened it, from 31 and 30 blocks to 19 and 22, and the old answer was selected half as often. The reverse order ADR-0154 named, that a reversal fast for another reason keeps its punishments large, does not account for that.
- **What the prediction did not hold is what ADR-0154 named beside it**: *"what a signal held at its floor for many trials does to the pairs of the two other readouts."* Read from the tables: over the second and the third reversals' mappings about seven trials in ten are punished; each whole punishment moves the selected readout's pairs against their traces at the gate's full depth; the rewards, a quarter to a third of the trials, do not put back what that takes; and from the second reversal the six couplings stand under the image's. The readouts then answer a stimulus with little more than they answer anything, the counts lie within two spikes of one another in seven trials of ten, and the selection has little to read.
- **That is an account of these tables, and its last step is not measured.** No run of this round held the couplings' sum while delivering whole punishments, so the flat readouts are read as the reason the later reversals were lost, not shown to be.
- **It does not read the two-answer configuration.** No two-answer arm ran under the parameter, as brief 063 rules.

### The evidence

- **The dispatch's scope.** The diff changes files under `src/` and a state crate — the executor, the image, the task, `cortex-neuromod` and `cortex-connectome` — so by [ADR-0075](0075-the-dispatch-scope-follows-the-diff.md) the weekly was dispatched on this round's branch at **`scope=both`**, as brief 063 asks: run [37872100407](https://github.com/DescentVTT/VirtualCortex/actions/runs/37872100407) at `d3add32`, the arms' tables, at 01:55:36Z on 2026-10-09.
- **Every whole-domain shard is green**, 01:55:40Z to 02:51:13Z. All ninety-one `exhaustive` tests passed, each exit 0 with one test reported. The eighty-nine before this round reproduced their pinned numbers on the hosted runners with the parameter unset, the five whole-image pins the weekly tests hold among them at format 21. **The two arms reproduced their tables there, in 1 798 s from the assignment and 1 729 s from the mirrored assignment.** H-29's arms, with their trials held by this round's reader, took 1 446 and 1 729 s.
- **The shards**, dealt by the table before this round, which did not know the two arms and costed each at 900 s:

  | Shard | Job | Its two heaviest tests (s) | Tests | Their seconds summed | Tests' wall time |
  | ---: | ---: | :--- | ---: | ---: | ---: |
  | 0 | 21.7 min | H-23 from the assignment 1 085; H-16's three arms 685 | 8 | 2 377 | 1 234 s, 17 % |
  | 1 | 55.5 min | H-24 from the mirrored 1 804; H-30 from the assignment 1 798 | 7 | 5 768 | 3 244 s, 45 % |
  | 2 | 40.7 min | H-29 from the mirrored 1 729; H-20 from the mirrored 1 726 | 7 | 4 699 | 2 357 s, 33 % |
  | 3 | 45.6 min | H-22 from the mirrored 1 710; H-21 from the assignment 1 709 | 8 | 5 105 | 2 652 s, 37 % |
  | 4 | 34.2 min | H-27 from the assignment 1 782; H-18 from the assignment 1 141 | 7 | 3 903 | 1 963 s, 27 % |
  | 5 | 43.1 min | H-30 from the mirrored 1 729; H-27 from the mirrored 1 370 | 7 | 4 476 | 2 498 s, 35 % |
  | 6 | 43.6 min | H-24 from the assignment 1 752; H-21 from the mirrored 1 743 | 8 | 4 588 | 2 525 s, 35 % |
  | 7 | 21.9 min | H-25 from the assignment 1 079; the control from the assignment 1 047 | 9 | 2 463 | 1 252 s, 17 % |
  | 8 | 45.1 min | H-22 from the assignment 1 779; H-28 from the assignment 1 750 | 7 | 4 892 | 2 620 s, 36 % |
  | 9 | 40.5 min | H-25 from the mirrored 1 768; H-23 from the mirrored 1 722 | 9 | 4 657 | 2 336 s, 32 % |
  | 10 | 32.8 min | H-28 from the mirrored 1 781; H-19 from the assignment 1 104 | 8 | 3 712 | 1 874 s, 26 % |
  | 11 | 31.7 min | the control from the mirrored 1 398; H-20 from the assignment 1 197 | 6 | 3 398 | 1 830 s, 25 % |

  The percentages are of the job's bound, 120 minutes; every shard ran its tests two at a time, at 1.78 to 1.99 of their summed seconds over the wall. **No shard passed 60 per cent of its bound**: the heaviest read 45, the shard the table had dealt H-24's slower arm and then given an arm of H-30 it costed at half its time.
- **The cost table is regenerated from this run** (`node scripts/exhaustive-costs.mjs from <artifacts> --run 37872100407`): 91 lines, 50 038 s. ADR-0092's deal plans each of the twelve shards at 4 169 to 4 170 s summed, about 2 100 to 2 340 s of wall time at the run's ratios, **29 to 33 per cent of the bound**, inside the brief's 60.
- **The sweep had not ended** when the shards did, and the merge does not wait for it (ADR-0150). Its job over the state crates had ended green at 02:20:00Z; its six jobs over the runtime were running. Its outcome is read when it ends and written down by the next decision's ADR, a survivor a finding there.
- **The pull request's gate** is green in every job on `b6114aa`, on `3a2ca9c` and on `7b0b28c`, the determinism pin on AArch64 and the MSRV among them. The mutation gate on the changed lines found 27 mutants: 19 caught and 8 unviable, none missed. The eight unviable are the tool's synthesized returns for `Image::decode` and `Task::trial`, whose `Executor` and `Outcome` have no default. The gate's run on `d3add32` was cancelled by the push of `7b0b28c` eight minutes after it; `7b0b28c` holds its Rust sources. The same gate on the developer machine read 18 caught by the tool and the same 8 unviable; the nineteenth, whose build the Windows linker's lock failed in every round, was caught by two tests when applied by hand.
- **On the developer machine**, at `d3add32`: the workspace's tests passed in the debug profile, in the release profile and on the MSRV, 718 passed and 131 ignored in each; the check, the format, Clippy, the documentation, the benchmarks and `npm run spec` exited 0; 91 whole-domain tests are listed. Before any rewarded run of the protocol the tree's 89 whole-domain tests passed there with the parameter unset, 23:09Z on 2026-10-08 to 01:29Z on 2026-10-09. The two arms then ran once, 01:30Z to 01:52Z, in 1 329 and 1 333 s side by side with H-29's two.

### The step of the stopping rule reached

**Step 5**: *"Otherwise no on clause 1: the punishment's size is not what the revision lacks. The next decision is the one ADR-0153's step 5 names, among the selection, an exploration and the readout's resolution, with both rounds' readings."* Clause 3 holds, so step 4 does not arise; clause 1 fails, so step 5 is reached before clause 5's step 6 is read. **The next decision is that ADR.** It is named and not taken. What this round hands it, beside what ADR-0153 handed:

- **The punishment's size was part of what the first revision lacked, and is not what the later ones lacked.** The step's sentence stands for the verdict; the first reversal's 19 and 22 blocks stand beside it.
- **A whole punishment costs the couplings more than the rewards restore**, on this schedule and these readouts: the six summed fell to 0.93 to 0.96 of the image's, and the readouts' answers flattened with them. Whatever the next decision takes, a punishment at full depth on seven trials in ten, as the later mappings had, has that cost here.
- **The ties and the margin grew with it**: nothing selected in 18 to 19 per cent of the trials of the last two mappings, and a margin of at most two spikes in 66 to 71 per cent. They are the readings ADR-0153's step 5 asks for, under a second condition.
- **The wrong selections spread**: a smaller share to the old answer, a larger one to the third readout and to nothing.
- **The parameter stays in the tree, unset.** The learning configuration is H-25's as it was, and H-29's reading of it among three answers stands.

Step 3, the ADR on whether the parameter joins the learning configuration, does not arise. Steps 4, 6, 7 and 8 did not arise; step 9 is kept: no constant moved after a rewarded run, and there was no second attempt.

## Consequences

- Good: the run that tells ADR-0153's account from its reverse, with one byte changed and H-29 beside it trial for trial until the rule first acts. The account held for the first reversal.
- Good: the no is located. The first revision is in time under a whole punishment; the second and the third are lost, and the tables say what moved: the six couplings' sum, the readouts' answers and the margin.
- Good: the critic stayed a predictor under the rule. Clause 4 holds in every mapping, with the value below zero where the accuracy is under a half.
- Good: every reading has H-29's beside it by the same rule from tables the tree holds, and the calibration shows where the parameter first acts, in the test that then runs the arm.
- Bad: H-30 is no, and a yes was predicted. The parameter does not join the learning configuration; it stays in the tree, unset, with its format.
- Bad: the last step of the account, that the flattened readouts are why the later reversals were lost, is read from the tables and not measured.
- Bad: one form of the rule and one schedule. A punishment bounded otherwise, or a whole one on a schedule with fewer punished trials, was not tried, and brief 063 does not empower either.
- Neutral: two weekly tests of about H-29's cost, and one test in the gate. H-29's arms gain one held table, their trials by this round's reader.
- Neutral: the calibration's two first blocks lengthen each arm by 128 trials under every oracle.

## Alternatives considered and why rejected

- **H-29's trials pinned whole** (option 1(b)), **the unset block alone** (option 1(c)), **the arm unasserted against its calibration** (option 2(b)), **H-29's by-trial readings outside the tree** (option 3(b)), **a new verdict** (option 4(b)), **a third weekly test** (option 5(b)), **one ADR** (option 6(b)): see above.
- **A two-answer arm under the parameter in this round**: brief 063 does not empower it; H-30's step 3 names it as the next decision's first question.

## Confirmation

- `runtime/cortex-runtime/tests/inhibition.rs`: `WHOLE_ARMS`, `with_whole_byte`, `only_the_whole`, `Whole`, `unsoftened`, `whole_step`, `TrialsRead`, `trials_read`, `below_half`, `lets_go_sooner`, `faster`, `recovers_sooner`, `old_below_image`, `WholeRead`, `whole_read`, `first_met`, `parts_at_first_met`, `whole_calibration`, `whole_arm`, the two weekly tests and the gate's test named above.
- The pins: `WHOLE_IMAGE_CRC_1024`, `WHOLE_FIRST_MET_1024`, `ANSWERED_TRIALS_1024`, the arms' `WHOLE_*_1024`, the verdict `WHOLE_1024` and its step `WHOLE_STEP_1024`, and the gate's checks over them (`trials_hold` among them).
- Whitepaper §8.8, §9 and §11.1; `CHANGELOG.md`; `CLAUDE.md`; `README.md`; `docs/zh-TW/README.md`.
