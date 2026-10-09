---
status: proposed
date: 2026-10-09
depends-on: ADR-0157
decision-makers: VirtualCortex maintainers
---

# ADR-0161: The span a choice among three is given, measured — H-31's protocol written before any trial past H-29's second flip: the schedule a parameter of a run in the harness, given to H-29's and H-30's readers where they took H-20's as constants; ADR-0157's schedule of 168 blocks; H-29's two arms from H-29's image, each held inside its own run, at the end of its 56th block, to H-29's first 3 584 trials; H-29's four clauses by H-29's rules; ADR-0157's four predicted readings as rules

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

- **Every pinned number of the tree holds.** The tree's whole-domain tests run on the developer machine at the protocol's commit, the two arms left out, before either arm runs. H-29's and H-30's arms are among them, read through the readers as they now are.
- **Each arm is H-29's arm trial for trial for its first 3 584 trials, held inside its own run.** `answered_run` calls a closure after each block's last trial, with the blocks and the trials read so far, before the next trial runs. An arm's closure acts once, at the end of the 56th block (`sized_calibration`), and holds:
  - the 56 blocks to H-29's pinned first 56, table for table;
  - the hash of every reading of the 3 584 trials to `ANSWERED_HEAD_1024`;
  - the trials by brief 063's reader on H-20's schedule to H-29's pinned ones over the first two mappings and the first flip (`before_second_flip`).

  A failure panics there, with no trial past H-29's second flip run. The arm then asserts that the closure held the calibration exactly once.
- **`ANSWERED_HEAD_1024`** is the hash of H-29's first 3 584 trials' readings, `0x385ef2444d22cb8d` from the assignment and `0x386c12a91850a707` from the mirrored assignment. It was first written from the dumps of H-29's two arms of 2026-10-09, each dump checked against the arm's pinned hash of its 7 680 trials and against its pinned accuracy sequence's by a second writing of the hash. H-29's own arms dump it and hold it from this round on, so the calibration's first half reproduces it before either arm of H-31 is held to it. No pinned number of H-29's moves.

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

## Consequences

- Good: the schedule is a parameter from here on, and the next schedule costs one constant.
- Good: the calibration's order is structural, and the arm reports that it was held once.
- Good: H-29's arms hold one more number each, the hash of their first 3 584 trials, and nothing of theirs moves.
- Neutral: two weekly tests of about 1.4 times H-29's cost, and one test in the gate.
- Neutral: twenty readers and the run gain an argument, and every call of H-29's and H-30's passes H-20's schedule.

## Alternatives considered and why rejected

- **Readers for the longer schedule beside H-29's** (option 1(b)), **the constants moved** (option 1(c)), **a separate calibration run** (option 2(b)), **a weekly test for the calibration** (option 2(c)), **the blocks and the reader's trials alone** (option 3(b)), **all 168 blocks pinned** (option 4(b)), **a verdict of its own** (option 5(b)), **each arm split** (option 6(b)), **two ADRs** (option 7(b)): see above.

## Confirmation

- `runtime/cortex-runtime/tests/inhibition.rs`: `Schedule`, `SCHEDULE`, `SIZED_SCHEDULE`, `SIZED_ARMS`, `six`, `six_at_ends`, `inhibitory_at_ends`, `at_the_32nd`, `added_blocks`, `failed_learned`, `within_sized`, `six_held`, `ends_between`, `SizedRead`, `sized_read`, `before_second_flip`, `sized_calibration`, `sized_arm`, the two weekly tests and the gate's test named above.
- The pins: `ANSWERED_HEAD_1024`.
- Whitepaper §9; `CHANGELOG.md`.
