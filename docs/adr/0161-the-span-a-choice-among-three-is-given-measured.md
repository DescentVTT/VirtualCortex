---
status: proposed
date: 2026-10-09
depends-on: ADR-0157
decision-makers: VirtualCortex maintainers
---

# ADR-0161: The span a choice among three is given, measured: H-31 is no, on clauses 1 and 2 in both arms and on clause 4 in one, where a yes was predicted — the schedule a parameter of a run in the harness, given to H-29's and H-30's readers where they took H-20's as constants, and ADR-0157's schedule of 168 blocks; H-29's two arms from H-29's image, each H-29's trial for trial for its first 3 584 trials, held inside its own run; the first reversal's mapping learned at 48 blocks in both arms, the one H-29 failed among them; the second reversal slower than H-29's, 46 and 37 blocks where H-29's took 31 and 23; the third reversal's mapping not learned in either arm, one stimulus holding its old answer; an answer's coupling still rising at its 48th block and past the bound of 1.30 in both arms; across both rounds the old answer leading longer the higher its coupling stood at the flip, under a punishment softened most where it is selected most; H-31's stopping rule reaches step 5, and the next decision, the ADR on an exploration, is named and not taken

## Context and Problem Statement

[ADR-0157](0157-the-span-a-choice-among-three-is-given.md) wrote H-31 before any run and Specified its schedule. This ADR is the measurement's: the protocol, committed before any trial past H-29's second flip is run, and then what was read.

**H-31**, restated from ADR-0157 and pinned in the tests. On H-29's configuration, readouts and arms, with a mapping after a flip of 48 blocks:
- **clause 1, the learning holds**: in both arms every mapping is learned — at least 80 of its last 128 trials correct, and among them each stimulus selecting its answer in more than half of its presentations;
- **clause 2, the couplings stay bounded**: no stimulus–readout coupling passes 1.30 of its image's at any block's end;
- **clause 3, the network holds**: at every block's end the summed magnitude of every excitatory synapse outside the six couplings lies within 0.75 and 1.25 of the image's;
- **clause 4, the critic holds the expected reward**: for each stimulus and each mapping over its last 128 trials, the engine's mean value lies within a quarter of the reward of $(2p - 1)$ of it.

Yes when all four hold in both arms. **Predicted: yes.**

**The last dispatched sweep** ([ADR-0158](0158-the-sweep-off-the-next-decisions-path.md)): run [37872100407](https://github.com/DescentVTT/VirtualCortex/actions/runs/37872100407)'s, of brief 063's dispatch, ended at 06:03Z on 2026-10-09 and is written down by [ADR-0160](0160-the-sweeps-tests-under-nextest.md). No sweep's outcome is owed here.

What was read first (principle 2), on 2026-10-09:

1. **The schedule is four constants** (`runtime/cortex-runtime/tests/inhibition.rs`): `SCHEDULE_FLIPS` and `SCHEDULE_TRIALS`, and `SPANS` and `MAPPINGS` derived from them. Twenty readers of H-29's and H-30's read them directly, and the run (`answered_run`) flips where `SCHEDULE_FLIPS` holds the trial's index.
2. **The task's flip** (`src/task.rs`, `Task::flip`) moves each answer on one readout and knows no schedule: the harness calls it. The two-readout harness's `run_on_scheduled` already takes its flips as a slice.
3. **H-29's trials are not in the tree**, only the hash of each arm's 7 680 (`ANSWERED_READ_1024`). Its blocks are pinned one by one (`ANSWERED_BLOCKS_1024`), and its trials by brief 063's reader per mapping (`ANSWERED_TRIALS_1024`).
4. **Before the trial of index 3 584 the two schedules hold one flip, the same one**, before the trial of index 1 536. A run on either executes the same trials until then.
5. **H-31's stopping rule numbers its steps as H-29's does**: 3 for a yes, 4 for a no on clause 3, then 5, 6 and 7 for a no on clauses 1, 2 and 4. `answered_step` gives them.
6. **A run's blocks are read only when it ends.** `answered_run` builds its task, its oracles and its tallies inside one call and returns after the last trial, so nothing outside it can hold a run part-way.

## Decision Drivers

- H-31's stopping rule: the schedule and the constants committed before any trial past H-29's second flip; no constant moves after one; no second attempt.
- **No file under `src/` changes.** The schedule is the harness's.
- **Every run on H-20's schedule is the run it was.** H-29's and H-30's arms read what they read, table for table.
- **"By H-29's rules" is to be literal**: one writing of each rule, read by both rounds.
- **The calibration comes before the trials it guards by construction**, not by the order in which someone ran two commands (principle 5).
- **H-29 beside it**: every reading of H-31 has H-29's read by the same rule, from tables the tree holds.
- The round is evidenced as [ADR-0150](0150-a-round-waits-for-what-it-checks.md) says. Latest ≠ Newest: nothing is adopted.

## Considered Options

1. **How the schedule becomes a parameter**: (a) a `Schedule` value given to each of H-29's and H-30's readers and to the run, H-20's passed wherever they took the constants; (b) readers for the longer schedule written beside H-29's; (c) the four constants moved.
2. **Where the calibration against H-29's first 3 584 trials is held**: (a) inside the arm's own run, at the end of its 56th block, by a closure the run calls after each block's last trial; (b) a run of 3 584 trials before the arm's run, in the same test; (c) a weekly test of its own for each arm.
3. **What the arm is held to there**: (a) H-29's pinned first 56 blocks, the hash of H-29's first 3 584 trials' readings, and H-29's trials by brief 063's reader over the first two mappings and the first flip; (b) the blocks and the reader's trials alone.
4. **The arms' pinned blocks**: (a) the 112 from H-29's second flip, the first 56 being H-29's own table; (b) all 168 pinned again.
5. **The verdict**: (a) H-29's `Answered` by H-29's `answered` on ADR-0157's schedule, its step by `answered_step`; (b) a verdict of its own.
6. **The tests**: (a) two weekly tests, each an arm of 10 752 trials, and one test in the gate; (b) each arm split in two.
7. **The ADRs**: (a) one, the measurement's; (b) two.

## Decision Outcome

**Options 1(a), 2(a), 3(a), 4(a), 5(a), 6(a) and 7(a).**

### The schedule in the harness

- **`Schedule`**: a run's three flips, each the index of the trial it comes before, and the run's trials. Its four mappings' trials (`spans`), their blocks (`mappings`) and the run's blocks (`blocks`) are derived from those by one rule.
- **`SCHEDULE` is H-20's**, from `SCHEDULE_FLIPS` and `SCHEDULE_TRIALS`. A compile-time assertion holds its spans and its mappings to `SPANS` and `MAPPINGS`, bound by bound, and its blocks to `SCHEDULE_BLOCKS`, so a reader given it reads what it read of the constants.
- **The twenty readers take the schedule as their first argument**: `answers_at`, `answers_of`, `onto_the_other_s`, `answered_last`, `answered`, `answered_crossings`, `answered_tally`, `by_role`, `each_crossed`, `first_new_answered`, `old_held`, `course_by_role`, `answered_highest`, `answered_troughs`, `went_by_mapping`, `below_zero_first`, `answered_read`, `trials_read`, `below_half` and `trials_hold`. `answered_run` takes it beside its trials. Every call in H-29's and H-30's arms and gates passes `SCHEDULE`.
- **`SIZED_SCHEDULE` is ADR-0157's**: the flips before the trials of index 1 536, 4 608 and 7 680; 10 752 trials, 168 blocks; the mappings' blocks 0 to 24, 24 to 72, 72 to 120 and 120 to 168. All of it is asserted at compile time, with the span's two derivations: 48 is half again 32, and the least whole number of blocks at or above 31 × 32 / 21.
- **No reader of H-20's to H-28's is touched**, and no pinned number is.

### The protocol (written before any trial past H-29's second flip)

- **The arms** (`SIZED_ARMS`): H-29's two, each a weekly `exhaustive` test, `the_span_a_choice_among_three_is_given_from_the_{assignment,mirrored_assignment}_at_1024_units_exhaustive`.
- **The image** is H-29's, built and held as H-29's arms build and hold it: by its CRC at format 21, `0x11586a76f7415129`, and by its CRC with the header's version written back to 20, the one H-29 read, `0x5044ed7936a7b5f3`. The engine decoded from it is asserted to carry the whole punishment unset.
- **The run** is H-29's (`answered_run`) on `SIZED_SCHEDULE`: H-29's deal, held again to the readouts' rule on the image; H-25's delivery; H-29's seed. At every trial the run holds the task's answers to the hand rule of the schedule it was given (`answers_at`), the critic's oracle, the hand rule of what the modulator received with the signal's course from it, the address, and the network's oracle over every excitatory synapse.
- **The clauses**, by rule: H-29's `answered` over the two arms on `SIZED_SCHEDULE`. A mapping's last 128 trials are then the run's 23rd and 24th blocks, its 71st and 72nd, its 119th and 120th and its 167th and 168th. The step is `answered_step`'s.
- **The predicted readings**, as rules and never asserted:
  - (a) `failed_learned`: the mirrored arm's second mapping is learned by each stimulus, by clause 1's own rule (`learned_by_each`);
  - (b) `within_sized`: per reversal, 40 of 64 within 35 blocks of its flip, the crossing block counted — H-25's `reversals_within` with the bound half again, the half block counted whole (`SIZED_REVERSAL_BLOCKS_MAX`);
  - (c) `six_held`: per mapping, the six couplings summed at its last block's end at or above the image's six summed;
  - (d) `ends_between`: the inhibitory sum at the run's end within 0.75 and 0.80 of the image's, both bounds held, read in integers as `75 × image ≤ 100 × sum ≤ 80 × image`.
- **The readings, no clause** (`SizedRead`).
  - *By block, by H-29's reader on the longer schedule* (`answered_read`): the block each mapping passes 40 of 64 in, and each stimulus's; where the wrong selections went and the ties; the six couplings' courses by role; the value's troughs; where the consolidation went; the inhibitory sum's course.
  - *By trial, by brief 063's reader* (`trials_read`): the margin, the punished and the rewarded trials, the trials that selected the old answer, the signal at the gate's bounds.
  - *What the added span bought*: each mapping after a flip over its 31st and 32nd blocks, the last two H-20's schedule gave it, beside its last two (`at_the_32nd`); and its correct trials by block from its 32nd to its 48th (`added_blocks`).
  - *The sums*: the six couplings' sum at each mapping's end with its lowest, highest and last over the run; the inhibitory sum at each mapping's end, and the first block at whose end it stood below half of the image's, H-21's bar (`drained_below`).
  - **H-29's beside each**, from its pinned `ANSWERED_READINGS_1024` and `ANSWERED_TRIALS_1024`.

### The calibration (H-31's stopping rule, step 2)

- **Every pinned number of the tree holds.** The tree's whole-domain tests ran on the developer machine at the protocol's commit, `934332c`, the two arms left out, before either arm: 91 of 91 passed, from 07:01:13Z to 09:19:28Z on 2026-10-09. H-29's and H-30's arms are among them, read through the readers as they now are.
- **Each arm is H-29's arm trial for trial for its first 3 584 trials, held inside its own run.** `answered_run` calls a closure after each block's last trial, with the blocks and the trials read so far, before the next trial runs. An arm's closure acts once, at the end of the 56th block (`sized_calibration`), and holds:
  - the 56 blocks to H-29's pinned first 56, table for table;
  - the hash of every reading of the 3 584 trials to `ANSWERED_HEAD_1024`;
  - the trials by brief 063's reader on H-20's schedule to H-29's pinned ones over the first two mappings and the first flip (`before_second_flip`).

  A failure panics there, with no trial past H-29's second flip run. The arm then asserts that the closure held the calibration exactly once. **Both arms held it**, at 09:27:42Z and 09:27:43Z, eight minutes into their runs.
- **`ANSWERED_HEAD_1024`** is the hash of H-29's first 3 584 trials' readings, `0x385ef2444d22cb8d` from the assignment and `0x386c12a91850a707` from the mirrored assignment. It was first written from the dumps of H-29's two arms of 2026-10-09, each dump checked against the arm's pinned hash of its 7 680 trials and against its pinned accuracy sequence's by a second writing of the hash. H-29's own arms dump it and hold it from this round on, so the calibration's first half reproduced it, in both of H-29's arms, before either arm of H-31 was held to it. No pinned number of H-29's moves.

### The gate

One test, `the_clauses_of_h_31_the_schedule_s_hand_rule_and_the_readings_rules`:

- **the schedule's hand rule**: ADR-0157's schedule as written; H-20's by the same rule equal to the four constants; on either, a mapping's trials its blocks' and a mapping starting at a flip; the answers in force at each flip's edges, with H-20's second and third flips no flips of the longer schedule; and at every one of the 3 584 trials before H-29's second flip one mapping in force on the two schedules, and two at the trial of index 3 584;
- **the clauses at their edges**, over runs of 168 blocks written by hand: clause 1 at 80 and at 79 of 128 for each mapping of each arm, and at half of a stimulus's presentations for the mapping H-29 failed; the blocks H-20's schedule read and this one does not, unread; clause 2 at the bound and one past it, in a block past H-20's 120th; clause 3 at both edges of the band; clause 4 at the quarter and one past it; each clause's place in the stopping rule's order; and a run of H-20's length under this schedule, or of this length under H-20's, holding nothing;
- **the predicted readings' rules** at their edges, (b) beside H-25's bound over H-29's own crossings and (d) at where H-29's sum ended;
- **the readings' rules** over tables and trials written by hand, with `at_the_32nd` held to `answered_last` on H-20's schedule over H-29's pinned arms;
- **eight trials** of the task on the instrument's network on a schedule written by hand, its three flips before the trials of index 2, 5 and 6: the run flips where the schedule it is given says, the hand rule and both oracles held at every trial.

### Why these options

- **Option 1(a).** H-31's clauses are H-29's by H-29's rules. A second set of readers (option 1(b)) would be a second writing of each rule, free to drift from the first. Moving the constants (option 1(c)) would move every run that reads them.
- **Option 2(a).** The stopping rule asks that the calibration stop the round before any trial past H-29's second flip. Held in the run, between the trial of index 3 583 and the next, that order is the code's. A separate run first (option 2(b)) would add 3 584 trials under every oracle to each arm, a third of its cost, and a weekly test of its own (option 2(c)) would make each arm depend on another test's having run, which ADR-0153 rejected.
- **Option 3(a).** *"Trial for trial"* is a statement about every trial. The blocks' tables and the reader's sums (option 3(b)) are what brief 064 names, and they are held; the hash of every reading of every trial is the statement itself, and costs two numbers that H-29's own arms now hold.
- **Option 4(a).** One table is then the pin of both rounds' first 56 blocks, and cannot disagree with itself. Pinning them twice (option 4(b)) would add two tables of that length that say what the calibration already holds.
- **Option 5(a).** The rule and the step's numbering are H-29's, so nothing new can drift from them.
- **Option 6(a).** An arm is one run of 10 752 trials; splitting it (option 6(b)) is for a test that outruns what a shard's deal can balance, and the arm is about 1.4 times H-29's.
- **Option 7(a).** There is no build to record apart from the measurement.

### What was read

The arms ran once each on the developer machine, from 09:19:48Z to 09:40:14Z on 2026-10-09, after the calibration's first half had ended green. Every oracle held at every one of each arm's 10 752 trials: the task's answers against the hand rule of the schedule; the critic's value, error and weights; the hand rule of what the modulator received and the signal's course from it; the address; and every excitatory synapse of the arena. The tables are pinned from that run: the 112 blocks from H-29's second flip (`SIZED_LATER_1024`), the two hashes, the couplings after each flip's first trial and the readings (`SIZED_*_1024`).

### The verdict

**H-31 is no: on clauses 1 and 2 in both arms, and on clause 4 in one.** ADR-0157 predicted a yes.

| Clause | From the assignment | From the mirrored assignment |
| :--- | :--- | :--- |
| 1, each mapping's last 128 | 118, 120, 85, **96** correct | 119, 117, 118, **72** correct |
| 2, the highest coupling | **1.318** of its image's | **1.320** |
| 3, the excitatory sum outside the six | 0.9999 to 1.0000 of the image's | 0.9999 to 1.0000 |
| 4, the value against $2p - 1$ | within 0.11 of the reward, but once **0.31** | within 0.06 |

- **Clause 1 fails for the third reversal's mapping in both arms, by one stimulus.**
  - From the assignment it read 96 of 128, with A at 65 of 66 and B at 31 of its 62: exactly half, where the rule asks for more than half. It fails by one presentation.
  - From the mirrored assignment it read 72, with A at 64 of 66 and B at 8 of 62. No block of the mapping passed 40 of 64.
- **The first mapping, the first reversal's and the second reversal's hold in both.** The first reversal's mapping read 120 and 117, where H-29 read 87 and 85 at 32 blocks. The second reversal's read 85 and 118; from the assignment A stood at 30 of 54, three presentations over half.
- **Clause 2 fails in both arms.** From the assignment A's coupling into readout 0, its answer under the last mapping, stood past 1.30 of its image's at the end of six of the run's last ten blocks, from the 159th, and at 1.318 at the last. From the mirrored assignment B's into readout 2 stood past it at the end of the third mapping's last four blocks, the 117th to the 120th, at up to 1.320, and A's into readout 1 at the end of five of the run's last six, 1.320 at the last. H-29's highest were 1.242 and 1.261.
- **Clause 3 holds**: outside the six couplings the excitatory sum moved by one part in ten thousand at most.
- **Clause 4 fails once.** Over the last 128 trials of the assignment arm's fourth mapping, B's mean value stood at −0.31 of the reward against a $2p - 1$ of zero. B's accuracy was rising there: it first selected its answer in more than half of a block's presentations in the mapping's 48th block. Everywhere else the value stood within 0.11 of the reward of $2p - 1$, below zero where the accuracy was under a half: B's read −0.79 against −0.74 in the mirrored arm's fourth mapping.

### The predicted readings

None was asserted.

| Predicted reading | Held | What was read |
| :--- | :--- | :--- |
| (a) the mapping H-29 failed is learned | yes | from the mirrored assignment the second mapping read 117 of 128, A at 55 of 60 and B at 62 of 68 |
| (b) every reversal within 35 blocks | 3 of 6 | 31, **46** and 21 blocks from the assignment; 30, **37** and **never** from the mirrored assignment |
| (c) the six couplings' sum at or above the image's at each mapping's end | 8 of 8 | 1.053, 1.063, 1.035 and 1.062 of the image's; 1.053, 1.051, 1.072 and 1.060 |
| (d) the inhibitory sum ends within 0.75 and 0.80 of the image's | 2 of 2 | 0.7850 and 0.7858 |

- **(b)'s third crossing from the assignment, 21 blocks, is not a revision.** 40 of 64 counts both stimuli together, and A alone carried it: A selected its new answer in more than half of a block's presentations from its 16th block, B from its 48th.

### What the added span bought

Each mapping after a flip over its 31st and 32nd blocks, where H-20's schedule would have ended it, beside its last two; and H-29's own reversal of the same number at its end.

| Arm | Reversal | At its 32nd block | At its 48th | H-29's, at its 32nd and last |
| :--- | :--- | :--- | :--- | :--- |
| assignment | first | 87: A 45 of 64, B 42 of 64 | **120**: A 57 of 60, B 63 of 68 | 87, the same trials |
| | second | 27: A 4 of 72, B 23 of 56 | **85**: A 30 of 54, B 55 of 74 | 82 |
| | third | 83: A 71 of 71, B 12 of 57 | 96: A 65 of 66, B 31 of 62 | 109 |
| mirrored | first | 85: A 30 of 64, B 55 of 64 | **117**: A 55 of 60, B 62 of 68 | 85, the same trials |
| | second | 45: A 18 of 72, B 27 of 56 | **118**: A 51 of 54, B 67 of 74 | 99 |
| | third | 68: A 62 of 71, B 6 of 57 | 72: A 64 of 66, B 8 of 62 | 114 |

- **The first reversal used the span.** Its mapping rose from 87 and 85 at its 32nd block to 120 and 117, and its blocks from the 32nd to the 48th read 43 rising to 61, and 43 rising to 60, correct of 64. The mapping H-29 failed is learned.
- **The second reversal needed the span, where H-29's did not.** At its 32nd block it stood at 27 and 45 of 128; H-29's second reversal, which left a mapping held for 32 blocks, stood at 82 and 99 there. It passed 40 of 64 in its 46th and its 37th block, where H-29's passed in its 31st and its 23rd.
- **The third reversal did not use it.** One stimulus was right nearly every time by the 32nd block and the other was not: from the assignment its blocks from the 32nd on read 30 to 46 correct of 64 and 50 in the last; from the mirrored assignment 28 to 39.
- **H-29's fourth mapping was learned, 109 and 114, and H-31's is not.** Over the run H-29 learned seven mappings of eight, and H-31 six.

### The coupling a flip finds, and how long the old answer leads

**A mapping held for 48 blocks leaves its answer's coupling higher than one held for 32, and the old answer then leads for longer.** Per flip and stimulus: the coupling from the stimulus into the readout it has to leave, as a fraction of its image's at the end of the last block before the flip; and the blocks from the flip in which that readout was selected more often than either other (`old_held`).

| Arm | Flip, stimulus | H-29: the coupling at the flip, the blocks the old answer led | H-31 |
| :--- | :--- | :--- | :--- |
| assignment | first, A | 1.241, 24 | the same trials |
| | first, B | 1.213, 21 | the same trials |
| | second, A | 1.167, 22 | 1.282, **39** |
| | second, B | 1.157, 16 | 1.276, **31** |
| | third, A | 1.119, 6 | 1.123, 9 |
| | third, B | 1.120, 12 | 1.204, **30** |
| mirrored | first, A | 1.261, 22 | the same trials |
| | first, B | 1.191, 13 | the same trials |
| | second, A | 1.098, 5 | 1.277, **34** |
| | second, B | 1.176, 14 | 1.252, **27** |
| | third, A | 1.202, 16 | 1.235, 21 |
| | third, B | 1.136, 17 | 1.320, **48**, the whole mapping |

- **The two rise together.** Over the twenty rows that are not one run read twice, their rank correlation is 0.91: 0.76 over H-29's twelve and 0.93 over H-31's eight after its first flip. Every row whose coupling stood at 1.25 or above led for 22 blocks or more; every row under 1.17 led for 22 or fewer.
- **Before H-31's second flip the answer's couplings stood at 1.25 to 1.28**, where H-29's stood at 1.10 to 1.18. The mapping before it had been held half again as long and learned further, 120 and 117 against 87 and 85.
- **The stimulus that failed the third reversal is the one whose coupling stood highest**, in both arms: B at 1.204 beside A's 1.123, and B at 1.320 beside A's 1.235. From the mirrored assignment B's old answer led in every one of the mapping's 48 blocks, and its coupling ended the run at 1.126 of the image's, with B's into its new answer at 0.953.

### The punishments of the old answer

The softened punishment is softest where the old answer is selected most. Per flip and stimulus: the trials that selected the old answer, of the stimulus's presentations in the mapping, and the mean of what the modulator received at them, as a fraction of the reward.

| Arm | Flip | A | B |
| :--- | :--- | :--- | :--- |
| assignment | first | 616 of 1 509, −0.36 | 591 of 1 563, −0.45 |
| | second | 991 of 1 524, **−0.21** | 707 of 1 548, −0.40 |
| | third | 251 of 1 558, −0.50 | 789 of 1 514, **−0.28** |
| mirrored | first | 650 of 1 509, −0.37 | 365 of 1 563, −0.67 |
| | second | 791 of 1 524, −0.29 | 575 of 1 548, −0.50 |
| | third | 479 of 1 558, −0.43 | 1 167 of 1 514, **−0.16** |

- **The critic comes to expect the punishment.** The stimulus's block mean value fell to −0.75 to −0.99 of the reward after a flip, and stood below minus half of it in 10 to 47 of a mapping's 48 blocks: in 47 for B after the mirrored arm's third flip. What a punished trial then delivers is the reward less a value near it.
- **ADR-0153 read this in H-29**, at −0.31 to −0.67 of the reward. Here the two slowest rows received −0.21 and −0.16, over 991 and 1 167 trials.

### The six couplings

- **Their sum stayed above the image's**, 1.035 to 1.072 at each mapping's end, and never fell under 0.996.
- **By the readouts' roles** at each mapping's end after a flip, as fractions of the image's, the answer's, the old answer's and the third's:

  | Arm | Mapping | From A | From B |
  | :--- | :--- | :--- | :--- |
  | assignment | second | **1.282**, 0.976, 0.957 | **1.276**, 0.953, 0.925 |
  | | third | **1.123**, 0.996, 0.946 | **1.204**, 1.029, 0.909 |
  | | fourth | **1.318**, 1.014, 0.935 | **1.069**, 1.041, 0.992 |
  | mirrored | second | **1.277**, 0.986, 0.923 | **1.252**, 0.963, 0.930 |
  | | third | **1.235**, 1.029, 0.971 | **1.320**, 0.991, 0.896 |
  | | fourth | **1.320**, 0.993, 0.981 | **0.953**, 1.126, 0.980 |

- **A learned answer's coupling was still rising at its 48th block.** For a stimulus that had revised it ended a mapping at 1.12 to 1.32, nine of the ten at 1.20 or above, where H-29's ended at 1.10 to 1.23 after 32 blocks; and it is what passed the bound.
- **An old answer's coupling ended a mapping at or near the image's**, 0.95 to 1.04, and at 1.126 where the revision never came.
- **Where the consolidation went**, by the network's oracle, per mapping after a flip, in millions: the answer's pairs net +2.09, +1.48 and +1.76 from the assignment and +1.94, +2.30 and +1.39 from the mirrored assignment; the other four pairs −1.90, −2.01 and −1.23, and −1.99, −1.88 and −1.62; the six together +0.20, −0.54 and +0.53, and −0.05, +0.43 and −0.23.

### The selection

- **The readouts answered as strongly as in H-29, or more.** The largest count averaged 9.14, 8.98, 8.21 and 9.39 spikes over the assignment arm's four mappings and 9.03, 8.80, 8.82 and 9.47 over the mirrored arm's, where H-29 read 9.14, 8.27, 7.31 and 8.00, and 9.03, 8.23, 7.62 and 7.91.
- **Nothing was selected** in 9.0 to 13.0 per cent of a mapping's trials, where H-29 read 9.8 to 16.0.
- **The margin** between the largest count and the next was at most two spikes in 40 to 52 per cent of a mapping's trials, where H-29 read 40 to 65.
- **The wrong selections went to the old answer**: 73 to 87 per cent of them after a flip. The third readout took 5.9 to 12.7 per cent of a mapping's trials.
- **In the mappings that failed**, over their last four blocks the stimulus that had revised drew 14.2 and 13.1 spikes a presentation from its answer's readout against 4.8 to 5.7 from the others. The one that had not drew 5.9 from its answer's and 5.2 from its old answer's from the assignment, and 4.3 and 7.0 from the mirrored assignment.

### The rest of the network

- **The inhibitory sum** stood at 1.002, 0.923, 0.844 and 0.785 of the image's at the four mappings' ends from the assignment and at 0.999, 0.921, 0.844 and 0.786 from the mirrored assignment, falling by about 0.0012 a block over the last mapping. It never came near H-21's bar of a half. At its 120th block each arm stood at 0.844, where H-29's ended at 0.843 and 0.842 after as many: the fall follows the trials run, not the schedule.
- **Outside the six couplings** the excitatory sum moved by one part in ten thousand at most.
- **The signal after a reward** stood at or below the gate's lower bound in 147 to 356 of a mapping's 3 072 trials after a flip, and at or above its upper bound in 262 to 437.

### What this reads, and what it does not

- **The one change is the schedule.** Each arm is H-29's trial for trial for 3 584 trials, and from there the two rounds differ in when the flips come and in nothing else.
- **The prediction was wrong, and what ADR-0157 wrote beside it is what was read**: *"the second and third reversals leave a mapping held for 48 blocks, not 32, and a mapping learned for longer may be harder to leave."* The span bought the first reversal, whose mapping H-29 had cut at 87 and 85 of 128. The mappings after it were left more slowly than H-29's, and the last was not left by one stimulus in either arm.
- **Read from the tables**: a mapping held longer is learned further; its answer's coupling stands higher at the flip; the old answer then leads for more blocks; while it leads, the critic's value falls toward the punishment and the punishment that would unlearn it shrinks, to a fifth of the reward in the slowest rows. The span is therefore not a room a revision is given and no more. It also deepens what the next revision has to undo.
- **That is an account of twenty rows of two rounds, and it is not measured.** No run held the coupling at a flip while varying how long the mapping before it lasted, or the reverse. The two rounds share an image, a seed and a deal.
- **The assignment arm's fourth mapping fails clause 1 by one presentation**, as H-29's one failed mapping did by three. The mirrored arm's does not turn on a presentation: 72 of 128, B at 8 of 62, and no block at 40 of 64.
- **It does not read a longer span.** A mapping of 64 blocks would give the last reversal more room and the one after it more to undo; which wins was not run, and brief 064 does not empower it.
- **The account ADR-0157 cited**, that a learner driven by a prediction error holds a choice among three given the span, does not hold here at this span. Where the engine's premise differs is where ADR-0157 said it might: the selection has no noise of its own, so the new answer is found only while the old one is being unlearned, and the punishment that unlearns it is softened by the same value that makes the first learning stable.

### The evidence

*This section is written when the dispatch's whole-domain shards have ended.*

### The step of the stopping rule reached

**Step 5**: *"Otherwise no on clause 1: the configuration does not hold a choice among three even with the span. The next decision is the ADR on an exploration that ADR-0153's step 5 and ADR-0156's steps 5 and 6 name, with three rounds' readings."* The calibration held, so step 2 did not stop the round. Clause 3 holds, so step 4 does not arise; clause 1 fails, so step 5 is reached before clause 2's step 6 and clause 4's step 7 are read. **The next decision is that ADR.** It is named and not taken. What this round hands it, beside what ADR-0153 and ADR-0156 handed:

- **The span is not what the revision lacks.** Given half again the blocks, the first revision among three completes and the later ones slow down or fail; over the run one mapping fewer is learned than on H-20's schedule.
- **How long a mapping was held is part of how hard it is to leave.** The old answer's coupling at the flip and the blocks it then leads rise together across both rounds, and the softened punishment is weakest exactly there. A mechanism that finds the new answer without first unlearning the old one would not pay that cost; that is the exploration's case, read from tables and not measured.
- **The whole punishment answered the same need at another price** (ADR-0156): it shortened the first revision and wore the six couplings down. Here the couplings' sum stayed above the image's throughout.
- **The readouts were not what failed.** The largest count, the ties and the margin read as in H-29 or better, and a stimulus that had revised drew two and a half to three times the spikes from its answer's readout that it drew from another.
- **The bound of clause 2 is passed at this span.** A learned answer's coupling was still rising at its 48th block and stood at 1.32 by the run's end. Step 6, an ADR on the bound at this readout's size, is not reached, since clause 1 comes before it; the reading stands for whichever ADR sizes a schedule next.
- **ADR-0151's second step, the third stimulus, has no yes to stand on.** H-29, H-30 and H-31 have each read no.

Steps 3, 4, 6 and 7 did not arise; step 8 is kept: no constant moved after a trial past H-29's second flip, and there was no second attempt.

## Consequences

- Good: the baseline ADR-0157 asked for is read. Given half again the span, the configuration completes a first revision among three and does not hold the later ones.
- Good: the no says where. The span bought the mapping H-29 failed; it slowed the second reversal and lost the third, and the tables name what stood higher at those flips.
- Good: the reading has H-29's beside it by one rule, and each arm is H-29's until the schedules part, held in the run itself.
- Good: the schedule is a parameter from here on, and the next schedule costs one constant.
- Good: H-29's arms hold one more number each, the hash of their first 3 584 trials, and nothing of theirs moves.
- Bad: H-31 is no, where a yes was predicted, the second prediction of a yes in two rounds to read no. Three rounds have now read no on a choice among three, and ADR-0151's second step waits.
- Bad: the account of why, that a mapping held longer is harder to leave, is read from twenty rows and not measured.
- Bad: the assignment arm's failure of clause 1 is by one presentation of one stimulus. The mirrored arm's is not, and clause 2 fails in both, so the verdict does not turn on it; the reading of that arm's last mapping does.
- Bad: a longer span was not run, and the exploration is still deferred.
- Neutral: two weekly tests of about 1.4 times H-29's cost, and one test in the gate.
- Neutral: twenty readers and the run gain an argument, and every call of H-29's and H-30's passes H-20's schedule.

## Alternatives considered and why rejected

- **Readers for the longer schedule beside H-29's** (option 1(b)), **the constants moved** (option 1(c)), **a separate calibration run** (option 2(b)), **a weekly test for the calibration** (option 2(c)), **the blocks and the reader's trials alone** (option 3(b)), **all 168 blocks pinned** (option 4(b)), **a verdict of its own** (option 5(b)), **each arm split** (option 6(b)), **two ADRs** (option 7(b)): see above.
- **A longer span, or a second run with a constant moved**: H-31's stopping rule rules both out, and brief 064 does not empower either.

## Confirmation

- `runtime/cortex-runtime/tests/inhibition.rs`: `Schedule`, `SCHEDULE`, `SIZED_SCHEDULE`, `SIZED_ARMS`, `six`, `six_at_ends`, `inhibitory_at_ends`, `at_the_32nd`, `added_blocks`, `failed_learned`, `within_sized`, `six_held`, `ends_between`, `SizedRead`, `sized_read`, `before_second_flip`, `sized_calibration`, `sized_arm`, the two weekly tests and the gate's test named above.
- The pins: `ANSWERED_HEAD_1024`, the arms' `SIZED_LATER_1024`, `SIZED_TRACES_1024`, `SIZED_READ_1024`, `SIZED_AT_FLIPS_1024` and `SIZED_READINGS_1024`, the verdict `SIZED_1024` and its step `SIZED_STEP_1024`, and the gate's checks over them (`sized_blocks`, `trials_hold` among them).
- Whitepaper §8.8, §9 and §11.1; `CHANGELOG.md`; `CLAUDE.md`; `README.md`; `docs/zh-TW/README.md`.
