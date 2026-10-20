---
status: accepted
date: 2026-10-12
depends-on: ADR-0171
title: A reward right three times in four, measured
kind: measured
hypothesis: H-35
summary: 'H-35 is no on clause 1: with the reward''s sign the outcome''s in three trials of four, seven mappings of eight were learned, the fourth from the assignment at 75, five short. Five reversals of six passed 40 of 64 in 19 to 26 blocks and the sixth in none of its 32.'
decision-makers: VirtualCortex maintainers
---

# ADR-0172: A reward right three times in four, measured — H-35 is no on clause 1: on H-25's configuration and schedule with the reward's sign the outcome's in three trials of four and its opposite in one, the coin misleading in 1 913 of the run's 7 680 trials, seven mappings of eight were learned, 115, 80 and 110 of each mapping's last 128 from the assignment with the fourth at 75, five short, and 117, 94, 110 and 97 from the mirrored assignment; no coupling passed 1.30, the highest 1.111 and 1.126, the network outside the couplings stayed within three parts in ten thousand of the image's, and each stimulus's mean value stood within 0.22 of the reward of a half of 2p − 1; five reversals of six passed 40 of 64 in 19 to 26 blocks and the sixth in none of its 32, its best block 38, a mean of 1.42 of H-25's, nearer H-28's reading carried in proportion than the naive scaling; the learning signal was 0.46 to 0.73 of H-25's, because a true reward's larger error met the gate's ceiling in 47 per cent of them and moved the answer's pair what it moved at seven in eight, while misleading punishments, each costing 1.3 true rewards, took back 42 and 44 per cent and misleading rewards put back 41 and 43 per cent of what true punishments took; two predicted readings held and the third missed in one cell of fourteen; H-35's stopping rule at step 5, not revised by the rule that splits it, two trials of one block from the other branch, and the next decision named and not taken; H-28's harness given the share as a value, no pinned number moved

## Context and Problem Statement

[ADR-0170](0170-a-reward-right-three-times-in-four.md) asked the learning configuration a less reliable reward, right three times in four, and wrote **H-35** before any run, with no prediction for its verdict, three predicted readings and a rule that splits a no on the learning. [ADR-0171](0171-a-reward-right-three-times-in-four-built.md) built the feedback: `Feedback::ThreeInFour`, its coin two of the misleading coin's three bits. Brief 069 runs H-35 once. This ADR is the round's protocol, committed before any rewarded run, and then its readings: **H-35 is no**, on clause 1 in one mapping of eight, at step 5 of its stopping rule, not revised by the rule that splits it.

What was read before anything was written (principle 2, and brief 069's directive that the engine and the harness are read before a description of them is trusted, the brief's and ADR-0170's included):

1. **H-28's harness** ([ADR-0149](0149-a-reward-right-seven-times-in-eight-measured.md), `tests/inhibition.rs`) wrote seven in eight as constants:
   - `MISLED_FEEDBACK`, the task's feedback;
   - `RELIABILITY`, three quarters, inside `holds_expected_reward`, `beside_expected` and `nearer_expected`;
   - `MISLEADING_BOUNDS` and `MISLEADING_BY_MAPPING_1024`, inside `coin_within` and the arm;
   - `coin_of`, which calls the harness's coin, `misleads`;
   - the arm's twenty-three pinned tables, each named `MISLED_…_1024` in `misled_arm` and again in the gate's test.
2. **The arm** (`misled_arm`): the coin's count and the oracle's coin against the task's; ADR-0077's settled engine and H-20's to H-23's images by their CRCs; a frozen block; H-25's first block from H-23's image under the true feedback (`drawn_first_block`); then 7 680 trials through `fed_run`, every trial's reward read back as the error plus the engine's value and held to the coin. Everything is dumped and the readings computed before a pinned table is held.
3. **A trial's reading** (`EarnedTrial`): the stimulus, the counts, the selection, whether it was correct, the error the modulator received, what the trial consolidated, and the signal before and after the reward. The reward the task gave is the error plus the value.
4. **A reversal's speed** (`crossings`): per mapping, the blocks from its first to the first of its blocks with at least 40 of 64 correct, that block counted; none where none of its blocks did. A mapping after a flip has 32 blocks, so a crossing there is inside its 32 exactly when there is one.
5. **H-25's six reversals** (`CROSSINGS_DRAWN_1024`): 19, 21 and 16 blocks from the assignment and 20, 15 and 14 from the mirrored assignment. H-28's: 19, 19 and 20, and 24, 14 and 21.
6. **The step of H-28's stopping rule** (`misled_step`) is the rule ADR-0170 writes for H-35 in its steps 3 to 6, but for step 5's split.

**The sweep ADR-0170 names.** The schedule's weekly run on `main` of 2026-10-12 had not started when this protocol was written, at 05:04Z; no run of the schedule's event is listed after the one of 2026-10-05. Looked for again at 09:13Z, when the readings were written, it had still not started. It started at 10:07:28Z, run 38234325824; its sweep had not started when this round asked for its merge, and the first ADR merged after it ends records it.

## Decision Drivers

- Brief 069's standing directives: one change from H-28, the coin's share; H-35's clauses, constants, predicted readings and step 5's split committed before the first rewarded run and not moved after it; no second attempt; no prediction of the verdict added; no float; every loop ends by construction; the gate grows by at most one test for the measurement; no pinned number of an earlier round moves.
- **H-28's arms must be the arms they were.** They are in the calibration and in the weekly job, and they are what H-35 is read beside.
- **One harness for both shares**, so that a difference between the two runs is the share's and not a second copy's.
- **A no is to say which no**, by a rule in the tree before the run.
- Latest ≠ Newest: no dependency, no tool, no rule of the engine.

## Considered Options

1. **How the harness is given the share**: (a) a value, `Share`, carried through the one arm and the one set of rules, with H-28's share built from the constants H-28 was written with; (b) a copy of the arm and the gate's checks for the new share; (c) the constants turned into arrays indexed by a share.
2. **How predicted reading 2 is read**: (a) the sum of the six ratios as an exact fraction, compared with six times 1.13; (b) a rounded mean.
3. **How step 5's split is read**: (a) a rule over the verdict and the crossings, `in_four_step`; (b) from the tables by eye after the run.
4. **What the arms pin beside H-28's tables**: (a) the misleading trials per block by whether seven in eight misled there too, and the readings beside H-28's and H-25's (`BesideRead`); (b) H-28's tables alone.
5. **The gate's checks over the pinned tables**: (a) H-28's loop taken out as a function given the share, called for both; (b) a second copy.

## Decision Outcome

**Options 1(a), 2(a), 3(a), 4(a) and 5(a).** Everything from here to the order of the work was committed before any rewarded run.

### The harness, given the share (`Share`, option 1(a))

- **A share** is a value: the name its dumps carry; the task's feedback; the coin as the harness writes it and as the task draws it; the reliability's part in an expected reward, $2q - 1$ as a fraction; the bounds the coin's count is held to and its count per mapping; the count per mapping of the trials that mislead at it and not at seven in eight; the prediction written for the verdict, if one was; each arm's pinned tables (`MisledPins`); and this round's further pins, which H-28's arms do not carry.
- **`seven_in_eight()`** is H-28's, built from `MISLED_FEEDBACK`, `misleads`, `RELIABILITY`, `MISLEADING_BOUNDS`, `MISLEADING_BY_MAPPING_1024`, `MISLED_PREDICTED` and the `MISLED_…_1024` tables. **`three_in_four()`** is H-35's.
- **The rules gain a form that is given what they read**, and the rule that stood calls it with H-28's constant, so every call of the tree is the call it was: `holds_expected_reward_at`, `misled_at`, `beside_expected_at` and `nearer_expected_at` take the reliability's part; `coin_of_by` the oracle's rule; `coin_within_of` the bounds.
- **The arm** is `misled_arm_at(share, arm)`: `misled_arm`'s body with the share's feedback, coin, bounds, count, reliability and pins in the places it named H-28's. `misled_arm` calls it with `seven_in_eight()`. Nothing else of it is touched.
- **The gate's checks over an arm's pinned tables** (option 5(a)) are `misled_tables(share, verdict, signed)`: the loop of H-28's gate's test, taken out. H-28's test calls it with its share. One of its checks rests on a reading of H-28's two runs and not on a rule — every error had its reward's sign — so it is held for the arms `signed` names: both of H-28's.
- **No pinned number of H-28 moves.** The calibration runs H-28's two arms through the form.

A copy (options 1(b) and 5(b)) would let the two shares' arms drift apart, and a difference between H-35 and H-28 could then be the copy's. Arrays indexed by a share (option 1(c)) would rewrite every one of H-28's constants.

### The coin over the schedule (H-35's stopping rule, step 2)

- **The count** (`coin_of_by`, `misleading_by_mapping`): the coin is misleading in **363, 514, 515 and 521** trials of the four mappings' 1 536, 2 048, 2 048 and 2 048, **1 913 of 7 680**, 24.91 per cent. Both arms draw from the one seed, 27.
- **Its bounds** (`IN_FOUR_BOUNDS`): 1 760 to 2 080, both counted. **1 913 is within them.**
- **Nested**: every one of H-28's 949 is among the 1 913. The 964 beside them — **182, 252, 265 and 265** by mapping (`IN_FOUR_ALONE_BY_MAPPING_1024`) — are the misleading trials whose bit 50 is set, and none is without it. The first is the eighth trial of the run, so the run leaves H-28's in its first block.
- **Where the numbers come from**: ADR-0171's oracle written apart from the tree, which reproduced the tree's pins of seven in eight first. The tests hold the harness's coin (`misleads_in_four`) to those counts and to the task's at every trial of the schedule.
- **By stimulus**: A is misled in 183, 250, 264 and 279 of its 772, 1 005, 1 006 and 1 022 trials, and B in 180, 264, 251 and 242 of its 764, 1 043, 1 042 and 1 026. Over each mapping's last 128 trials the coin misleads 26, 38, 35 and 35 times, where at seven in eight it misled 13, 17, 13 and 20.
- **Per block** the coin misleads in 9 to 25 of 64 trials.
- The bits are ADR-0170's, on `main` before the count was computed.

### The arms (`in_four_arm`)

- **Two arms** (`IN_FOUR_ARMS`), each a weekly test of its own: H-25's two, from the assignment and from the mirrored assignment. Each is H-28's arm at the share of three in four: H-23's image, asserted by its CRC; `Delivery::Drawn`; H-20's flips; 7 680 trials; the engine's critic with its window; the task carrying no critic; no hold and no shadow.
- **Both oracles at every trial**, the drawn sources held to the critic's oracle's counts, and the engine's value, error and weights to the harness's critic, as in H-28's.
- **Every trial's reward** is held twice: inside the harness, the reward the task delivered against `correct != misleads_in_four(…)`; and in the arm, the error plus the value against the same rule.

### H-35, restated as integer rules (ADR-0170's clauses and constants)

- **Clauses 1 and 2** are H-20's, `scheduled`: each mapping's last 128 trials at least 80 correct, and no coupling above 1.30 of its image's at any block's end. A trial is correct by its selection.
- **Clause 3** is H-24's band, `left_band`: the excitatory sum outside the four couplings within three quarters and five quarters of the image's at every block's end, over 120 blocks.
- **Clause 4** (`holds_expected_reward_at` at `IN_FOUR_RELIABILITY`, a half): for a stimulus's $n$ trials over a mapping's last 128, $c$ of them correct, their values summing to $V$, with the reward $r$:

  $$|2V - (2c - n)\,r| \le \tfrac{1}{2}\,r\,n$$

  and at least one trial. It is $|V/n - \tfrac{1}{2}(2p - 1)\,r| \le r/4$ with $p = c/n$. The rule clears the division: both sides are read times four.
- **The verdict** (`Misled`, `misled_at`): yes when all four hold in both arms.
- **The step** (`in_four_step`): 3 at a yes; 4 at a no in which clause 3 fails in an arm, whatever else failed beside it; otherwise 5 at a no on clause 1 or 2, whatever clause 4 read; otherwise 6, a no on clause 4 alone. They are `misled_step`'s.
- **Step 5's split** (`Split`, option 3(a)), as ADR-0170 writes it:
  - **(a) slower than the schedule** (`Split::Slower`) when clause 2 holds in both arms, the first mapping is learned in both arms, and each of the six reversals has a crossing;
  - **(b) not revised** (`Split::NotRevised`) in every other case.
- `IN_FOUR_PREDICTED` is none: ADR-0170 predicts no verdict, and this round adds none.

### The three predicted readings, as rules (`in_four_predicted`)

1. **The first mapping learned in both arms**: `mapping_correct` of the first mapping at least 80 in each.
2. **The six reversals slower than H-28's on average** (option 2(a)): with each reversal counted at its crossing, or at its 32 blocks where it did not cross (`counted`), the sum of the six ratios to H-25's (`ratios`) is an exact fraction, its divisor the product of H-25's six, and the reading holds when a hundred times the sum is above 113 times six times the divisor. It is false where one of H-25's never crossed. H-28's own six sum to 6.788, a mean of 1.131.
3. **The value nearer a half of $2p - 1$ than $2p - 1$** (`nearer_expected_at` above zero), for each stimulus in each mapping clause 1 holds for. A mapping it does not hold for is not read; a value as near either is not nearer.

`IN_FOUR_READINGS_PREDICTED` is the three, each predicted to hold. They are dumped beside what is read and never asserted; a miss is recorded as one.

### The readings, no clause (ADR-0170's)

Per block, pinned; per mapping, computed from them. H-28's tables whole, by H-28's rules (`MisledRead`), so that every table of H-28 and of H-25 has its counterpart here block for block:

- **the trials by outcome and by the reward received** (`Outcomes`), over each mapping and over its last 128 trials;
- **what a reward of each kind moved** (`KindMoves`) in the pair the trial before addressed; **the errors delivered by kind** (`KindErrors`); **the clamp's part** (`Beyond`), the trials whose reward left the signal beyond the gate's bounds and by how much;
- **the value beside what it could hold** (`beside_expected_at`, `nearer_expected_at`), the rewards each stimulus received, the value's troughs after each flip;
- **the reversal speeds** (`speeds`), the learning signal per mapping, the couplings' separation at each mapping's end, the inhibitory sum's lowest and last.

And this round's, beside them (`BesideRead`, option 4(a)):

- **each reversal beside the two accounts** (`beside_accounts`): the blocks it is counted at, H-25's, and a hundred times the first less 200 and less 139 times the second — above zero where it took longer than twice H-25's, and than 1.39 of it. **Which account the run stands nearer** is read from the mean of the six ratios beside 2 and beside 1.39, the midpoint 1.695;
- **the first block that is not H-28's** (`first_apart`), by the sight's table;
- **the misleading trials by whether seven in eight misled there too**, by outcome (`nested_blocks`), per block and per mapping;
- **the trials whose error had not its reward's sign** (`errors_against`): where there is none, the tables that read a delivery's sign read the reward's, as in H-28's runs;
- **what the rewards moved over the run** (`taken_back`): the net move of the answer's pair after a true reward and after a misleading punishment, and of the pair a wrong selection reached after a true punishment and after a misleading reward. Over H-28's pinned tables the rule reads 12.85 and −3.43 million from the assignment and 12.68 and −3.50 from the mirrored assignment, ADR-0149's 27 and 28 per cent.

### The calibration, before any rewarded run (H-35's stopping rule, step 2)

- **In the tree**: the workspace's tests in the debug and the release profile and on the MSRV; every whole-domain test of the weekly job before this round, each from the release build: every pinned number of the tree, **H-28's two arms under `SevenInEight`, through the form, among them**.
- **In each arm's test**, before its first rewarded trial:
  - the coin's count held to its pin and its bounds, and the harness's coin to the task's at every trial;
  - **every trial of the 7 680 misleading at seven in eight held misleading here**, and the trials misleading here alone to their count per mapping;
  - ADR-0077's settled engine and H-20's to H-23's images by their CRCs; a frozen block held to ADR-0077's frozen run;
  - **H-25's first block from H-23's image under the true feedback** (`drawn_first_block`), table by table, through the same `fed_run` the arm then runs.

A failure at any of them stops the round there as a finding.

### The gate

`the_clauses_of_h_35_the_coin_s_count_the_predicted_readings_and_the_split`, one test:
- the arms, the feedback, the delivery, the predictions and the constants; H-28's share held to the constants H-28 was written with;
- the coin's count per mapping and in all, its bounds at their edges and neither share's count within the other's; the harness's coin against the task's over the schedule and against the apart oracle's first sixty-four trials; nested, with the count of the trials misleading here alone and their bit 50;
- clause 4 at a half at its edges — at a full accuracy, at none, at a half, at fifty-six of sixty-four and at two of three — and what it tells apart here: at a full accuracy a value at $2p - 1$ fails it, where H-28's clause held both;
- the forms at H-28's share against H-28's rules;
- the verdict over tables written by hand, with the step and the split: a yes; clause 4 alone; clause 1 at 79 and at 80; clause 1 beside clause 4; a reversal that never crosses, and one that crosses in its thirty-second block with 79 of its last 128; the first mapping not learned; clause 2; the band at its edge and past it; a run not whole;
- the reversals' rules: the count at 32, the two accounts to the block, H-28's six against H-25's, and predicted reading 2 at its mark and one block past it;
- the three predicted readings' rules over tables written by hand;
- the readings' rules over tables written by hand;
- eight trials on the instrument's network under the drawn delivery with the reward right three times in four, and the same eight at seven in eight: every reward held to its share's coin, both oracles to the record, the two runs one through the seventh trial — two misleading rewards among them — and the eighth's reward the opposite.

The build's tests are ADR-0171's, in `task.rs`.

### The weekly tests and their cost

- **Two weekly tests**: `a_reward_right_three_times_in_four_from_the_{assignment, mirrored_assignment}_at_1024_units_exhaustive`.
- **Each is H-28's arm.** H-28's arms read 1 822 and 1 823 s in the table after ADR-0169, which holds 108 tests and 79 490 s. Two more at that cost make about 83 100 s, as ADR-0170 planned: 48 to 54 per cent of a shard's bound at the last run's ratios.
- **The dispatch's scope**: ADR-0171 changed a file under `src/`, so by [ADR-0075](0075-the-dispatch-scope-follows-the-diff.md) the weekly is dispatched at `scope=both`.

### The order of the work, as the history holds it

1. **The bits before the count.** The coin's two bits are ADR-0170's, merged to `main` at `d090b30` before this round began. Then the oracle apart from the tree was run: it reproduced the tree's pins of seven in eight and returned the count, 1 913, and the nesting. Had the count been outside its bounds, or a trial of H-28's 949 not among them, the round would have stopped there as a finding.
2. **The rules before their result.** The feedback, its tests, the share, the clauses, the step and its split, the predicted readings' rules, the readings' rules, the arms and the gate's test were written with every pinned table of an arm empty. The gate's test passed on the developer machine in the debug profile; its eight rewarded trials under the feedback on the instrument's unsettled network are the harness's check and read nothing of H-35.
3. **H-28's arm through the form, before the commit.** A new check of an arm fails only after its whole run, and an arm of H-35 runs once. So H-28's arm from the assignment was run through `misled_arm_at` from the release build of the working tree, from 04:39:16Z to 05:03:41Z on 2026-10-12: it reproduced every pinned table of H-28 and passed, in 1 461 s beside the workspace's tests (on the developer machine, a ratio and not admissible). It is a run at seven in eight; no trial of it is at three in four. The calibration runs both of H-28's arms again from the committed build.
4. **The build and the protocol committed and pushed** before any rewarded trial of an arm at three in four: the build as `a2f33b8`, then this protocol. On the working tree they were committed from, the check, the format, Clippy, the workspace's tests in the debug profile, in the release profile and on the MSRV — 737 passed and 150 ignored in each — the documentation, the benchmarks and `npm run spec` exited 0, and 110 whole-domain tests are listed.
5. **The calibration.** **Every one of the 108 whole-domain tests of the weekly job before this round passed**, each in a process of its own and each exit 0 with one test reported, eight at a time from 05:07:44Z to 08:30:28Z on 2026-10-12, from a release build of `4254054` with the tree clean. H-28's two arms under `SevenInEight` ran through `misled_arm_at` among them, in 1 873 and 1 947 s, and reproduced every pinned table of H-28; H-25's two arms, the six at 2 048 units and every earlier run through `Task::trial` were among them too.
6. **The arms**, once, side by side from that same build, from 08:32:46Z to 08:57:23Z. In each the calibration held before the first rewarded trial: the coin's count; every trial misleading at seven in eight misleading here, and 182, 252, 265 and 265 misleading here alone; ADR-0077's settled engine and H-20's to H-23's images by their CRCs; ADR-0077's frozen block; and **H-25's first block from H-23's image under the true feedback reproduced table by table**. Each ran its 7 680 trials with the drawn sources held to the critic's oracle, every reward held to the coin and both oracles held to the record at every trial, dumped every table and stopped at the first empty one, in 1 467 s each (on the developer machine, a ratio and not admissible). The tables were written from those dumps and committed as `1488edb`.

No constant, clause or rule moved after the first rewarded trial of an arm, and there was no second run: the weekly dispatched on `1488edb` is the pinned tables' reproduction. With the tables, the gate's test gained its checks over them and the verdict's three constants, written from the rules' own output over those tables.

### The readings

**The verdict** (`IN_FOUR_1024`), by the rule committed first, over the pinned tables:

| Clause | From the assignment | From the mirrored assignment |
| :--- | :--- | :--- |
| 1, each mapping's last 128 correct, at least 80 | 115, 80, 110, **75** | 117, 94, 110, 97 |
| 2, the highest coupling over its image's, at most 1.30 | 1.111, A onto readout 0 at the 23rd block | 1.126, B onto readout 0 at the 88th block |
| 3, the excitatory sum outside the couplings, 0.75 to 1.25 of the image's | 0.9997 to 1.0000 | 0.9999 to 1.0002 |
| 4, each stimulus's mean value within 0.25 of a half of $2p - 1$ | all eight; the farthest 0.137 | all eight; the farthest 0.217 |

**H-35 is no: clause 1 fails in one mapping of eight**, the fourth from the assignment, 75 of its last 128 trials correct where 80 are asked. The second from the assignment is on the mark, 80. Clauses 2, 3 and 4 hold in both arms. H-28 read 123, 113, 122, 119 and 124, 107, 122, 114 under clause 1 and 1.181 and 1.187 under clause 2; H-25 read 122, 124, 119, 116 and 124, 118, 123, 117, and 1.181 and 1.191.

ADR-0170 predicted no verdict. Of its two accounts, the naive scaling said no on clause 1 in the mappings after a flip, and the verdict is that. The speeds, below, stand nearer the other.

**The three predicted readings** (`IN_FOUR_READINGS_READ_1024`), each predicted to hold:

| Predicted reading | Read |
| :--- | :--- |
| 1, the first mapping learned in both arms | **Held**: 115 and 117 of its last 128, crossing 40 of 64 in the sixth block in each |
| 2, the mean of the six reversals' ratios to H-25's above 1.13 | **Held**: 1.42, the one that did not cross counted at its 32 blocks |
| 3, the value nearer a half of $2p - 1$ than $2p - 1$ in each mapping clause 1 holds for | **A miss, in one cell of fourteen** |

The miss: in the second mapping from the assignment, learned at 80 of 128, stimulus B was correct in 34 of its 64 trials. A half of $2p - 1$ is 0.031 of the reward there and $2p - 1$ is 0.063, and its mean value was 0.082, above both and nearer the second by 0.031. ADR-0170 said of this region that *"both are near zero and the band holds either"*; the predicted reading did not except it, so it is a miss. In the other thirteen cells the value is nearer the half.

**The trials by outcome and by the reward received** (`IN_FOUR_OUTCOMES_1024`; true rewards, true punishments, misleading rewards, misleading punishments):

| Mapping | From the assignment | Its last 128 | From the mirrored | Its last 128 |
| ---: | :--- | :--- | :--- | :--- |
| 1 | 837, 336, 113, 250 | 91, 11, 2, 24 | 833, 340, 103, 260 | 94, 8, 3, 23 |
| 2 | 587, 947, 335, 179 | 63, 27, 21, 17 | 593, 941, 305, 209 | 65, 25, 9, 29 |
| 3 | 835, 698, 239, 276 | 77, 16, 2, 33 | 814, 719, 227, 288 | 82, 11, 7, 28 |
| 4 | 466, 1 061, 359, 162 | 52, 41, 12, 23 | 614, 913, 312, 209 | 70, 23, 8, 27 |

- Over a run, **867 and 966 correct selections were punished and 1 046 and 947 wrong ones or ties rewarded**, 1 913 in each.
- A correct selection was punished in 24.1 and 25.3 per cent of the correct trials, the coin's share.
- A trial tied in 6.0 to 8.0 per cent of a mapping's trials; under H-28 in 5.0 to 6.7.
- Over whole mappings the correct trials were 0.61 to 1.04 of H-28's, fewer in six of the eight.

**The reversal speeds** (`crossings`, `beside_accounts`; the blocks to the first of a mapping's blocks with 40 of 64 correct):

| | First reversal | Second | Third |
| :--- | :--- | :--- | :--- |
| From the assignment | 26; H-28's 19, H-25's 19 | 20; H-28's 19, H-25's 21 | **none in its 32**, its best block 38; H-28's 20, H-25's 16 |
| From the mirrored | 24; H-28's 24, H-25's 20 | 19; H-28's 14, H-25's 15 | 24; H-28's 21, H-25's 14 |
| Twice H-25's, the naive scaling | 38 and 40 | 42 and 30 | 32 and 28 |
| 1.39 of H-25's, H-28's reading in proportion | 26.4 and 27.8 | 29.2 and 20.9 | 22.2 and 19.5 |

- As ratios to H-25's: 1.37, 0.95 and at least 2.00, and 1.20, 1.27 and 1.71. **Their mean is 1.42** with the one that did not cross counted at its 32 blocks; the five that crossed average 1.30.
- **The speeds stand nearer H-28's reading carried in proportion than the naive scaling**: the mean is nearer 1.39 than 2, the midpoint 1.695. Four of the six are under 1.39 of H-25's and five are under twice it; the sixth did not cross.
- **Neither account is the run's.** The naive scaling put three of the six past their 32 blocks and a fourth at its last; one is. The other put all six inside, the slowest at the edge; five are inside, by 6 to 13 blocks.
- **A crossing is not a mapping learned.** After their crossings the mappings rose less than H-28's did: the six mappings after a flip ended at 80, 110 and 75, and 94, 110 and 97 of 128, where H-28's ended at 113, 122 and 119, and 107, 122 and 114. The second mapping from the assignment crossed at its 26th block and held 43 and 37 in its last two.
- **The mapping that failed was still rising.** Its 32 blocks held 6, 7, 6, 9, 10, 14, 9, 10, 14, 10, 14, 14, 19, 13, 13, 24, 19, 19, 21, 31, 25, 22, 28, 23, 23, 26, 29, 35, 32, 28, 38 and 37 correct.
- **In it one stimulus had turned and the other had not.** Over its last 128 trials A was correct in 42 of 54 and B in 33 of 74. B's selection went to its new answer more often than to its old only in the 31st block after the flip, and at the mapping's end B's coupling onto its answer stood below its coupling onto the other readout.
- Per stimulus, the selection first went to the new answer more often than to the old 5 to 31 blocks after a flip; under H-28, 9 to 23.

**The learning signal** (`signal_of` over `IN_FOUR_WENT_1024`, the network's oracle; the answer pairs' net consolidation less the other pairs', per mapping, over H-25's):

- From the assignment: **0.71, 0.56, 0.56, 0.46**. From the mirrored: **0.73, 0.62, 0.62, 0.65**.
- **The naive scaling says 0.5.** H-28 read 0.87 to 1.03 where it said 0.75. The signal is above the naive half in seven of the eight and under it in the mapping that failed; it is 0.49 to 0.75 of H-28's.

**What a reward of each kind moved, and why the signal fell as it did.** Three things are read (`IN_FOUR_KIND_MOVES_1024`, `IN_FOUR_KIND_ERRORS_1024`, `IN_FOUR_BEYOND_1024`, `taken_back`).

*A true reward's error grew, and what it moved did not.*
- The critic holds about a half of $2p - 1$, so over a mapping's last 128 trials a true reward's error was **0.56 to 0.93 of the reward**, where H-28's was 0.30 to 0.52 and H-25's 0.08 to 0.18.
- In the trial after a true reward the answer's pair gained 2 970 to 4 210 a trial by mapping, 3 280 and 3 860 over a run. Under H-28 it gained 3 030 to 4 150, 3 460 and 3 420 over a run.
- **A true reward left the signal above the gate's ceiling in 46.6 per cent of them in each arm**, by 0.37 of the reward on average; under H-28 in 31 and 30 per cent. There the clamp passed on 1 and no more.

ADR-0149 handed on that *"each true reward teaches more"* as the reliability falls, and that it had read that gain at one reliability. Read at a second, the error was 1.6 to 2.4 times H-28's, and the pair's gain a trial is H-28's.

*A misleading punishment costs less than at seven in eight, and there are twice as many.*
- Its error was −0.90 to −1.24 of the reward by mapping, and −1.12 to −1.47 over a mapping's last 128 trials; under H-28, −1.48 to −1.70 there.
- It left the signal below the gate's floor in 53 and 52 per cent of them, by 0.47 of the reward on average; under H-28 in 68 per cent.
- In the trial after it the answer's pair lost 4 370 and 4 970 a trial over the run, **1.33 and 1.29 times what a true reward gave**; under H-28, 6 690 and 6 740, 1.9 and 2.0 times.
- One correct trial in four was punished, so misleading punishments took back **42 and 44 per cent** of what true rewards gave the answer's pairs: 3.79 of 8.95 million from the assignment, 4.80 of 11.01 million from the mirrored. Under H-28 they took back 27 and 28 per cent. At H-28's cost a trial ADR-0170's arithmetic gave 65.

*The pair a wrong selection reached is put back at twice H-28's rate.*
- A misleading reward's error was 1.05 to 1.21 of the reward after the first mapping. It left the signal above the ceiling in 56 and 55 per cent of them.
- In the trial after it the pair the wrong selection reached gained 4 250 and 4 320 a trial, against the 3 350 and 3 470 a true punishment took from it.
- One wrong selection in four was rewarded, so misleading rewards put back **43 and 41 per cent** of what true punishments took from those pairs: 3.81 of 8.84 million and 3.61 of 8.83 million. Under H-28 they put back 18 and 19 per cent.

*Over the whole run* the signal a reward left lay beyond the gate's bounds after **47.3 and 46.9 per cent of the trials**: above the ceiling after 1 854 and 1 848, below the floor after 1 775 and 1 753, by 0.40 of the reward on average. Under H-28, after 37.8 and 37.0 per cent.

**What this does not say.** These are readings of the run as it was. The run has one gate and one critic; what the learning would have been with a wider clamp, another step of the critic or a smaller weight on one error was not run and is not derived.

**The value** (`stimulus_values`, `rewards_received`; per mapping and stimulus over the mapping's last 128 trials, as fractions of the reward):

| | Mean value | A half of $2p - 1$ | $2p - 1$ | Mean reward received |
| :--- | :--- | :--- | :--- | :--- |
| Assignment, A | 0.450, 0.234, 0.197, 0.141 | 0.385, 0.219, 0.290, 0.278 | 0.770, 0.438, 0.581, 0.556 | 0.443, 0.375, 0.194, −0.037 |
| Assignment, B | 0.450, 0.082, 0.405, 0.030 | 0.410, 0.031, 0.424, −0.054 | 0.821, 0.063, 0.848, −0.108 | 0.463, 0.250, 0.273, 0.027 |
| Mirrored, A | 0.528, 0.284, 0.406, 0.037 | 0.451, 0.203, 0.435, 0.148 | 0.902, 0.406, 0.871, 0.296 | 0.508, 0.281, 0.484, 0.000 |
| Mirrored, B | 0.366, 0.049, 0.211, 0.215 | 0.381, 0.266, 0.288, 0.338 | 0.761, 0.531, 0.576, 0.676 | 0.522, 0.031, 0.303, 0.378 |

- **The critic holds the expected reward.** In fifteen of the sixteen the value is nearer a half of $2p - 1$ than $2p - 1$; the sixteenth is the miss above.
- **Clause 4 tells the three apart here**, as ADR-0170 said it would. H-25's rule, a quarter of the reward either side of $2p - 1$, holds four of these sixteen. H-28's clause, at three quarters, holds thirteen. H-35's holds all sixteen.
- The value follows the reward the stimulus received, which the coin moves about its expectation: the mean reward a stimulus received there was −0.04 to 0.52 of the reward, and its mean value 0.03 to 0.53.
- **The troughs after each flip are shallower again**: −0.28 to −0.61 of the reward, against H-28's −0.37 to −0.75 and H-25's −0.63 to −0.84, each of the twelve shallower than H-28's.

**The trials that mislead here and did not at seven in eight** (`IN_FOUR_ALONE_1024`; by outcome: correct, wrong, tied):

| Mapping | Assignment: at seven in eight too | Here alone | Mirrored: at seven in eight too | Here alone |
| ---: | :--- | :--- | :--- | :--- |
| 1 | 131, 38, 12 | 119, 52, 11 | 129, 43, 9 | 131, 39, 12 |
| 2 | 83, 150, 29 | 96, 138, 18 | 102, 143, 17 | 107, 133, 12 |
| 3 | 132, 104, 14 | 144, 97, 24 | 137, 97, 16 | 151, 96, 18 |
| 4 | 82, 154, 20 | 80, 163, 22 | 103, 138, 15 | 106, 146, 13 |

- The two halves of the coin met the same outcomes: of the misleading trials, the correct ones were 45 and 46 per cent of each half from the assignment and 50 and 51 from the mirrored.
- **The run leaves H-28's in its first block.** The eighth trial is the first that misleads here alone, and the first block's row of the sight's table is not H-28's in either arm. Its count of correct trials is H-28's, 33 and 29; the second block's is not, 31 against 33 and 30 against 32.

**Where else the run stands beside H-28 and H-25.**
- **The couplings rose less.** The highest was 1.111 and 1.126 of its image's; H-28's 1.181 and 1.187.
- **The couplings' separation** at each mapping's end, the answer's coupling less the other's, was −0.06 to 0.93 of H-28's per stimulus. It was below zero once: B at the end of the mapping that failed.
- **The inhibitory sum** ended at 0.845 and 0.844 of the image's, its lowest, as H-28's ended at 0.850 and 0.845.
- **The drawn sources** were 99.67 and 99.66 per cent of the presented stimulus's units and 1.63 others a trial in each arm, H-25's.
- **The first new selection** after a flip came within 1 to 40 trials; H-28's within 3 to 190.

**In these two runs every error had its reward's sign** (`errors_against` reads none in either arm). The engine's value never passed the reward: the 3 771 and 3 801 deliveries above zero are the 2 725 and 2 854 true rewards and the 1 046 and 947 misleading ones. So the tables of H-25 that read a delivery's sign read here what they read under H-25, and the gate holds that block by block for both arms. It is a reading of these runs and not a property of the rule.

**The oracles held** at every one of the 15 360 trials: the drawn sources to the units the harness's critic oracle counted from the train, the targets to the selected readout's units, the engine's value, error and weights to the harness's critic, the reward to the coin, and the composer's and the network's traces and weights to the record.

### The evidence

- **The dispatch's scope.** The diff changes a file under `src/` — the task, ADR-0171's feedback — so by [ADR-0075](0075-the-dispatch-scope-follows-the-diff.md) the weekly was dispatched on this round's branch at **`scope=both`**, as brief 069 asks: run [38228081172](https://github.com/DescentVTT/VirtualCortex/actions/runs/38228081172) at `1488edb`, the pins' commit. The round's commits, in pull request #202: `a2f33b8` the build; `4254054` the protocol, before any rewarded run at three in four; `1488edb` the arms' tables and the gate's checks over them; `b62d4e9` the readings and the documents; and the commit that carries this section, the cost table and the brief's archive.
- **Every whole-domain shard is green.** All 110 `exhaustive` tests passed on the hosted runners, each exit 0 with one test reported, 83 129 s summed. The 108 before this round reproduced their pinned numbers, H-28's two arms through the harness given the share among them, in 1 805 and 1 830 s. **H-35's two arms reproduced their tables there, in 1 807 s from the assignment and 1 827 s from the mirrored assignment.** H-25's arms took 1 108 and 1 834 s in the same run.
- **The shards**, dealt by the table before this round, which did not know the two arms and costed each at 900 s:

  | Shard | Job | Its two heaviest tests (s) | Tests | Their seconds summed | Tests' wall time |
  | ---: | ---: | :--- | ---: | ---: | ---: |
  | 0 | 61.6 min | H-34, seed 23 from the assignment 3 570; H-22 from the mirrored 1 836 | 8 | 7 011 | 3 603 s, 50 % |
  | 1 | 65.5 min | H-34, seed 22 from the mirrored 2 250; H-22 from the assignment 1 852 | 8 | 5 973 | 3 833 s, 53 % |
  | 2 | 66.4 min | H-34, seed 23 from the mirrored 3 706; H-30 from the mirrored 1 842 | 10 | 7 739 | 3 888 s, 54 % |
  | 3 | 79.8 min | H-34, seed 22 from the assignment 3 730; H-29 from the assignment 2 119 | 8 | 9 383 | 4 692 s, 65 % |
  | 4 | 56.6 min | H-31 from the mirrored 1 723; H-27 from the mirrored 1 456 | 9 | 6 357 | 3 316 s, 46 % |
  | 5 | 55.6 min | H-31 from the assignment 1 869; H-20 from the mirrored 1 410 | 9 | 6 281 | 3 257 s, 45 % |
  | 6 | 66.9 min | H-33, seed 23 from the mirrored 1 871; H-32 from the assignment 1 813 | 10 | 7 309 | 3 922 s, 54 % |
  | 7 | 38.9 min | H-20 from the assignment 1 247; H-27 from the assignment 1 033 | 10 | 4 535 | 2 271 s, 32 % |
  | 8 | 71.0 min | H-33, seed 23 from the assignment 1 890; H-33, seed 24 from the assignment 1 882 | 8 | 8 233 | 4 173 s, 58 % |
  | 9 | 38.4 min | H-25 from the assignment 1 108; H-32 from the mirrored 1 101 | 9 | 4 439 | 2 230 s, 31 % |
  | 10 | 70.3 min | H-33, seed 24 from the mirrored 1 892; H-24 from the assignment 1 842 | 11 | 7 985 | 4 126 s, 57 % |
  | 11 | 68.0 min | H-25 from the mirrored 1 834; H-30 from the assignment 1 832 | 10 | 7 884 | 3 990 s, 55 % |

  The percentages are of the job's bound, 120 minutes; each shard ran its tests two at a time, at 1.56 to 2.00 of their summed seconds over the wall. The twelve started from 25 seconds to 10 minutes after the dispatch, the later ones behind the pull request's own gate, and the last ended at 10:31:25Z, 1 h 22 min after it.

  **One shard of this run passed 60 per cent of its bound**, the fourth at 65. It holds one of H-34's arms, which read 3 730 s on this run's runner where the table held 2 151, beside H-29's arm at 2 119; brief 069's directive excepts a shard that holds one of H-34's arms and is on the regenerated deal, below. The two shards that drew this round's arms, the ninth and the eleventh, each with an arm costed at 900 s and running 1 827 and 1 807, read 58 and 57 per cent.
- **The cost table is regenerated from this run** (`node scripts/exhaustive-costs.mjs from <artifacts> --run 38228081172`): 110 lines, 83 129 s, where it held 108 and 79 490. ADR-0092's deal plans each of the twelve shards at 6 926 to 6 930 s summed, about 3 460 to 3 730 s of wall time at this run's ratios, **48 to 52 per cent of the bound**. No shard is planned past 60, those that hold H-34's arms among them. An arm at 2 048 units read 2 250 to 3 730 s in this run and 2 151 to 3 580 in the last: its seconds on a hosted runner are a range, and a shard's bound against its longest test stays the workflow's decision that ADR-0168 names.
- **The sweep.** Its thirteen jobs start when the shards have ended ([ADR-0166](0166-the-sweep-after-the-shards.md)): they were queued at 10:31:25Z and **had not started when this was written**, at 10:35Z. By [ADR-0158](0158-the-sweep-off-the-next-decisions-path.md) the first ADR merged after it ends records its outcome. The schedule's weekly run on `main`, which ADR-0170 names, started at 10:07:28Z as run 38234325824, at `d090b30`; its shards were running and its sweep had not started. No sweep that has ended is without its record: the last before these, of run 38111246283, is in [ADR-0165](0165-another-network-of-the-same-size.md).
- **The pull request's gate** on `4254054` and on `b62d4e9` is green in every job, the determinism pin on AArch64 and the MSRV among them. The mutation gate on the changed lines found 12 mutants in each, 8 caught and 4 unviable, none missed: `Task::misleading_in_four_at`'s seven and the feedback's one in `Task::trial` caught; the four unviable are the tool's synthesized returns for `Task::trial`, whose `Outcome` has no default. The same run on the developer machine read 5 caught and 7 unviable, three of them the Windows linker's refusal to overwrite a test binary and not the mutant's; each of those three, applied by hand, fails the task's unit tests. The gate on `1488edb` was cancelled by the push of `b62d4e9`, whose tests are its.
- **On the developer machine**, at `1488edb`: the workspace's tests passed in the debug profile, in the release profile and on the MSRV, 737 passed and 150 ignored in each; the check, the format, Clippy, the documentation, the benchmarks and `npm run spec` exited 0; 110 whole-domain tests are listed.

### The step of the stopping rule reached

**Step 5, branch (b)** (`IN_FOUR_STEP_1024`): a no on clause 1 with clause 3 held, and by the rule that splits it, written before the run, **not revised**. Clause 2 holds in both arms and the first mapping is learned in both, but one of the six reversals, the third from the assignment, has no block with 40 of its 64 trials correct inside its 32.

ADR-0170's step 5(b): *"The configuration does not hold a reward wrong one time in four, and the boundary lies between this share and seven in eight. The next decision is an ADR on a single error's weight, the gate's clamp and the critic's step among its candidates, with this round's readings by kind as its need."* **The next decision is that ADR.** It is named and not taken.

**How near the other branch the rule read.** The rule is the one written first and its reading stands. What the tables say beside it, for the decision it names:
- the reversal that did not cross had its best block at 38, two short of the 40 that would have made the branch (a), in the 31st of its 32 blocks, and its counts were still rising;
- the other five crossed with 6 to 13 blocks to spare, and seven of the eight mappings were learned, one of them on the mark;
- so the run stands between the two branches as the stopping rule drew them: revised in five reversals of six, and in the sixth by one stimulus of two.

**What this round hands that ADR**, by kind:
- **The gate's clamp, on the side that teaches.** A true reward's error was 0.56 to 0.93 of the reward and left the signal above the ceiling in 47 per cent of them; the answer's pair gained a trial what it gained at seven in eight. The gain ADR-0149 expected to grow as the reliability fell did not grow.
- **A single error's weight.** A misleading punishment took 1.3 times what a true reward gave, where at seven in eight it took 1.9 to 2.0; there were twice as many, and they took back 42 and 44 per cent.
- **The wrong side, which ADR-0149 read small.** Misleading rewards put back 41 and 43 per cent of what true punishments took from the pair a wrong selection reached, where at seven in eight they put back 18 and 19. After a flip that pair is the old answer's.
- **The critic's step.** The value held a half of $2p - 1$ within 0.22 in every cell, so the error a reward delivers is the one the arithmetic gives. Nothing in these readings says the critic's step is what failed.
- **The schedule.** The mapping that failed was rising at its end. ADR-0170 said of a no of the first kind that it *"is as much the schedule's as the engine's"*; the rule read the second kind, at the first's edge.

Steps 3, 4 and 6 did not arise; step 7 is kept.

## Consequences

- Good: the second share is read, and with seven in eight beside it the two give a slope: the learning signal 0.87 to 1.03 of H-25's at seven in eight and 0.46 to 0.73 at three in four, the reversals 1.13 and 1.42 of H-25's.
- Good: the two accounts are told apart by a run. Neither is the run's: the speeds stand nearer H-28's reading carried in proportion, and the verdict is the naive scaling's.
- Good: the no says which no, by a rule written before the run, and the tables say how near the other branch it stands.
- Good: the readings by kind name what moved between the two shares: the gate's ceiling met by true rewards, a misleading punishment's smaller cost, and the wrong side's put-back at twice the rate.
- Good: H-28's harness carries a second share with no pinned number of H-28 moved, so a third share is a value and not a third copy.
- Bad: one share, one network and one schedule. The boundary lies between this share and seven in eight and is not located.
- Bad: the verdict turns on five trials of one mapping and the branch on two trials of one block. A second network or seed could read either the other way; none was run.
- Bad: the clamp's part is read from what the run did, not from a run without it.
- Bad: two more arms in the weekly job, each about H-28's cost, 1 807 and 1 827 s in the dispatch. The regenerated table, 110 tests and 83 129 s, plans each shard at 48 to 52 per cent of its bound.
- Neutral: a predicted reading missed in a cell ADR-0170's own text called undecidable; the miss is recorded as one.

## Alternatives considered and why rejected

- **A copy of the arm and the gate's checks** (options 1(b) and 5(b)) **or arrays indexed by a share** (option 1(c)): see above.
- **A rounded mean for predicted reading 2** (option 2(b)): the mark, 1.13, is H-28's own mean to two places, 1.131; a rounding could have read H-28 itself either way.
- **Step 5's split read by eye** (option 3(b)): the branch decides which ADR is next, and this run's stands two trials from the other.
- **H-28's tables alone** (option 4(b)): the trials that mislead here alone, the first block that is not H-28's and what the rewards moved over the run could not be recomputed from the tree.

## Confirmation

- `runtime/cortex-runtime/tests/inhibition.rs`: `Share`, `MisledPins`, `seven_in_eight`, `three_in_four`, `misled_arm_at`, `misled_tables`, `holds_expected_reward_at`, `misled_at`, `beside_expected_at`, `nearer_expected_at`, `coin_of_by`, `coin_within_of`; `IN_FOUR_ARMS`, `IN_FOUR_FEEDBACK`, `IN_FOUR_RELIABILITY`, `IN_FOUR_BOUNDS`, `IN_FOUR_BY_MAPPING_1024`, `IN_FOUR_ALONE_BY_MAPPING_1024`; `in_four_step`, `Split`, `in_four_predicted`, `ratios`, `counted`, `beside_accounts`, `BesideRead`, `first_apart`, `nested_blocks`, `errors_against`, `taken_back`; the two weekly tests and the gate's test named above.
- `runtime/cortex-runtime/tests/instrument/harness.rs`: `misleads_in_four`.
- The arms' pinned tables, `IN_FOUR_*_1024`, the verdict `IN_FOUR_1024`, its step `IN_FOUR_STEP_1024` and the predicted readings as read, `IN_FOUR_READINGS_READ_1024`, with the gate's checks over them.
- Whitepaper §11.1 and §9; `CHANGELOG.md`; `CLAUDE.md`; `README.md`; `docs/zh-TW/README.md`.
