---
status: proposed
date: 2026-10-12
depends-on: ADR-0171
decision-makers: VirtualCortex maintainers
---

# ADR-0172: A reward right three times in four, measured — H-35's protocol, written before any rewarded run: H-28's harness given the share as a value, so that H-28's arms run through it with the constants they were written with and H-35's with a half where they had three quarters; the coin misleading in 1 913 of the run's 7 680 trials, 363, 514, 515 and 521 by mapping, H-28's 949 all among them; H-28's four clauses with the expected reward at a half of 2p − 1; the three predicted readings and step 5's split as integer rules; the readings beside H-28's and H-25's; the calibration and the gate; the readings follow the run

## Context and Problem Statement

[ADR-0170](0170-a-reward-right-three-times-in-four.md) asked the learning configuration a less reliable reward, right three times in four, and wrote **H-35** before any run, with no prediction for its verdict, three predicted readings and a rule that splits a no on the learning. [ADR-0171](0171-a-reward-right-three-times-in-four-built.md) built the feedback: `Feedback::ThreeInFour`, its coin two of the misleading coin's three bits. Brief 069 runs H-35 once. This ADR is the round's protocol, committed before any rewarded run; its readings are added to it when the arms have run.

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

**The sweep ADR-0170 names.** The schedule's weekly run on `main` of 2026-10-12 had not started when this protocol was written, at 05:04Z; no run of the schedule's event is listed after the one of 2026-10-05. This ADR looks again before its readings are written.

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
