---
status: accepted
date: 2026-10-12
depends-on: ADR-0173
title: Three in four on the other networks, measured
kind: measured
hypothesis: H-36
summary: 'H-36 is no, on clause 1 on one network of the two: seed 23 learned every mapping in both arms, and seed 24 learned six of eight. Over the three networks twenty-one mappings of twenty-four were learned and sixteen reversals of eighteen crossed.'
decision-makers: VirtualCortex maintainers
---

# ADR-0174: Three in four on the other networks, measured — H-36 is no, on clause 1 on one network of the two: on ADR-0044's generator at 1 024 units from seeds 23 and 24, each under the configuration its rules derived, with the reward's sign the outcome's in three trials of four and the same 1 913 trials misleading as on seed 22, seed 23 learned every mapping in both arms, 121, 115, 121, 119 and 121, 97, 117, 112 of each mapping's last 128, and seed 24 learned six of eight, 121, 71, 116, 76 from the assignment with the second nine short and the fourth four, and 107, 119, 112, 112 from the mirrored assignment; no coupling passed 1.30 on either, the highest 1.136 to 1.195, the network outside the couplings stayed within four parts in ten thousand of the image's, and each stimulus's mean value stood within 0.22 of the reward of a half of 2p − 1; eleven reversals of twelve passed 40 of 64 inside their 32 blocks, a mean of 1.31 and of 1.42 of each network's own under the true reward, and over the three networks twenty-one mappings of twenty-four were learned and sixteen reversals of eighteen crossed, at 1.38; three predicted readings held and the fourth missed, misleading punishments taking back more than a half of what true rewards gave in two arms of four; H-36's stopping rule at step 5, seed 24 not revised by the rule that splits it, one trial of one block from the other branch, and the next decision named and not taken; the tree's two arm bodies made one form given where it runs and under what feedback, every arm of H-28, H-33, H-34 and H-35 a call of it, no pinned number moved and no file under `src/` changed

## Context and Problem Statement

[ADR-0173](0173-three-in-four-on-the-other-networks.md) read the need before a rule of the engine is changed for it: the reward right three times in four, which H-35 read as no on seed 22 by five trials of one mapping, asked of the two other networks of the size, seeds 23 and 24. It wrote **H-36** before any run, with no prediction for its verdict, a forecast by H-35's own ratios and four predicted readings. Brief 070 runs H-36 once and changes no file under `src/`. This ADR is the round's protocol, committed before any rewarded run at three in four on either network, and then its readings: **H-36 is no**. Seed 23 reads yes on all four clauses; on seed 24 clause 1 fails in two mappings of the arm from the assignment; the stopping rule is at step 5, seed 24 not revised by the rule that splits it.

What was read before anything was written (principle 2, and brief 070's directive that the engine and the harness are read before a description of them is trusted, the brief's and ADR-0173's included):

1. **The tree holds an arm in two bodies** (`tests/inhibition.rs`), and an arm of a network that is not seed 22's under a feedback that is not the true reward's is in neither.
   - `site_arm(site, arm)`, H-33's and H-34's ([ADR-0167](0167-another-network-of-the-same-size-measured.md), [ADR-0169](0169-twice-the-size-measured.md)): the network derived by its rules (`derive_at`) and held to its pinned derivation; 7 680 trials through `fed_run_at` under `Feedback::Answer`; the critic's step read beside the run at every trial; twelve readings (`SeededRead`); nine pinned tables and the critic's reading (`ArmPins`).
   - `misled_arm_at(share, arm)`, H-28's and H-35's ([ADR-0149](0149-a-reward-right-seven-times-in-eight-measured.md), [ADR-0172](0172-a-reward-right-three-times-in-four-measured.md)): seed 22's images held by their CRCs and H-25's first block under the true feedback; 7 680 trials through `fed_run`, which is `fed_run_at` at 1 024 units with the constants the tree holds for seed 22; the share's feedback and coin; H-28's readings and brief 069's (`MisledRead`, `BesideRead`); twenty-three pinned tables (`MisledPins`).
2. **What `misled_arm_at` and its rules name of seed 22**: the rotation, the critic, the window, the target period and the settled image's clock; the image's sums, its four couplings and H-24's cells; clause 2 read against `IMAGE_COUPLINGS_1024` inside `misled_at`, and again in the readings `first_over` and `highest`; the stimulus set's units inside `once_blocks`; H-25's tables wherever a reading stands beside the true reward's; and H-28's blocks in the reading of the first block that is not H-28's. The gate's checks over an arm's pinned tables, `misled_tables`, name the same.
3. **`derive` at seed 22 returns those constants and writes H-23's image.** The weekly test `the_rules_return_the_tree_s_constants_on_seed_22_at_1024_units_exhaustive` holds what a derivation makes there — the rotation, the cancel and the synapse counts, the gain, the sums, the couplings, the period, the window and the critic — to the constants `misled_arm_at` names, and the image to H-23's CRC. So the instrument's configuration, as what a derivation makes (`Made`), is those constants with the image the arm has held by its CRC.
4. **`SeededRead` is `MisledRead`'s, twelve numbers of it.** Each is read by the rule H-28's arm reads it by, against the network's image where H-28's rule names seed 22's: the correct trials, the first block past the bound, the crossings, the first block out of the band, each stimulus's values, the highest couplings, the troughs, the tally, the blocks that fired once, whether the inhibitory sum fell in every block, the reach and the sums after the run.
5. **What H-33 did not pin.** An arm of H-33 pins the sight's blocks, the earned blocks, the engine's value and the drawn sources by block, and four hashes. It pins no table of where the consolidation went, of the moves by a trial's kind, of the errors delivered or of the clamp's part. So the learning signal, a true reward's error and how often the clamp acted **under the true reward on seeds 23 and 24** are in no table of the tree, and H-35 read each of them beside H-25's.
6. **Under `Feedback::Answer` the reward's sign is the outcome's** (`Task::trial` in `src/task.rs`): the rule of the two misleading feedbacks with a coin that misleads at no trial. `fed_run_at` takes the feedback as a value and runs one path for all of them.
7. **Two phrases of ADR-0173's predicted readings are read here before the run**, and written as rules:
   - reading 3, *"for each stimulus in each mapping that clause 1 holds for and whose accuracy over those trials is at least three quarters"*: the accuracy is the stimulus's, over its own trials of the mapping's last 128. The cell ADR-0173 cites as the miss it excepts is a stimulus's, 34 of its 64.
   - reading 4, *"between a third and a half"*: both bounds counted.

**The sweep brief 070 names.** The schedule's weekly run on `main`, run [38234325824](https://github.com/DescentVTT/VirtualCortex/actions/runs/38234325824): looked for at 14:40Z on 2026-10-12, when this protocol was begun, twelve of its sweep's thirteen jobs had ended, each green, and `runtime-1` was running. Its record is in the evidence below.

## Decision Drivers

- Brief 070's standing directives: no file under `src/` and no file of a state crate changes; one change from H-35, the network, and seen from H-33 one too, the reward's reliability; the networks seeds 23 and 24 at 1 024 units, each under the configuration ADR-0167 pinned; the same trials misleading on every network; H-36's clauses, constants and four predicted readings committed before the first rewarded run at three in four and not moved after it; no second attempt; no prediction of the verdict added; no float; every loop ends by construction; the gate grows by at most one test; no pinned number of an earlier round moves.
- **Every arm of the tree must be the arm it was.** H-33's four arms, H-34's four, H-28's two and H-35's two are in the calibration and in the weekly job, and H-33's and H-35's are what H-36 is read beside.
- **One body for an arm**, so that a difference between a network's arm under the true reward and its arm at three in four is the feedback's, and a difference between seed 22's arm at three in four and another network's is the network's, and neither is a second copy's.
- **A reading beside the network's own is read from the tree**, not from a log.
- Latest ≠ Newest: no dependency, no tool, no rule of the engine.

## Considered Options

1. **How an arm of a site takes a share**: (a) one body, `arm_on(ground, share, arm)`, given where it runs and under what feedback, with `site_arm` and `misled_arm_at` each a call of it; (b) `misled_arm_at`'s body given a site, `site_arm`'s left beside it; (c) a third body for a site at a share.
2. **What the true reward is to the form**: (a) no share — `Feedback::Answer`, a coin that misleads at no trial, the reliability's part one; (b) a `Share` value for it, with pinned tables it does not have.
3. **How a network's first block under the true feedback is held** (H-36's stopping rule, step 2): (a) sixty-four trials through the `fed_run_at` the arm then runs, held to the first row of each table by block H-33's arm pins; (b) H-33's whole arm run again inside each arm.
4. **How a reading stands beside the network's own under the true reward where H-33 pinned no table of it**: (a) H-33's arms, run through the form, read by H-28's rules too, and that reading pinned from the calibration's run of them (`ArmRead`), which the dispatch reproduces; (b) beside H-25's on seed 22 alone; (c) from the calibration's dumps.
5. **The verdict on a network**: (a) H-35's four clauses by H-35's rules against the network's own image (`misled_on`); (b) against seed 22's.
6. **The step of the stopping rule**: (a) a rule over the two networks' verdicts and crossings, with H-35's split read per network (`others_step`); (b) from the tables by eye.
7. **How the four arms are dealt into tests**: (a) four weekly tests, each deriving its network first; (b) one test a network.

## Decision Outcome

**Options 1(a), 2(a), 3(a), 4(a), 5(a), 6(a) and 7(a).** Everything from here to the order of the work was committed before any rewarded run at three in four on either network.

### The form (`arm_on`, option 1(a))

- **Where an arm runs is a value** (`Ground`): the instrument's network, seed 22 at 1 024 units, or a `Site`.
  - On the instrument's network the calibration is `misled_arm_at`'s as it stood (`instrument_made`): ADR-0077's settled engine and H-20's to H-23's images by their CRCs, a frozen block held to ADR-0077's frozen run, and H-25's first block under the true feedback (`drawn_first_block`). It returns the constants the tree holds for seed 22 with H-23's image.
  - On a `Site` the calibration is `site_arm`'s as it stood (`site_made`) — the derivation by the rules, held to the site's pinned derivation, and no arm where clause 0 does not hold — and then the network's own first block under the true feedback (`own_first_block`, below), which is what `drawn_first_block` is in the instrument's.
- **Under what feedback is a value** (option 2(a)): a `Share`, or none. None is the true reward: `Feedback::Answer`, a coin that misleads at no trial, and the reliability's part one (`TRUE_RELIABILITY`), at which clause 4's rule is H-25's.
- **The body is the two bodies' union, and each number of it is read by the rule it was read by.** What `misled_arm_at` named of seed 22 is what the ground made: the rotation, the gain, the critic, the window, the period, the sums and the couplings. `first_over` and `highest` are read by `first_over_of` and `highest_of` against those couplings, `reach_by_polarity` by `reach_over` over the ground's sets, and `once_blocks` by `fires_once_of` for the ground's stimulus set. On the instrument's network each is the call it was, and the two assertions that are seed 22's alone — the settled image's clock and H-24's cells — are held there.
- **On every ground and under every feedback the run is held as either body held it**:
  - the drawn sources to the critic's oracle's counts and both oracles to the record at every trial (`fed_run_at`);
  - every trial's reward, read back as the error plus the engine's value, to the outcome's sign unless the coin misleads — `misled_arm_at`'s, now held under the true reward too;
  - the critic's step beside the run, the value of the window's counts under the weights before the reward the engine's own at every trial, and the units counted the sources drawn — `site_arm`'s, now held on seed 22 too. The reading reads the engine and writes nothing to it;
  - no worker's entry dropped, at the run's end.
- **What a run is held to follows where it ran and under what**: under a share, the share's tables (`MisledPins`, and `BesidePins` where it carries them); under the true reward on a `Site`, the site's (`ArmPins`). H-25's own arm on the instrument's network is `drawn_arm`'s and is not this form's.
- **`site_arm(site, arm)` is `arm_on` on the site under no share, and `misled_arm_at(share, arm)` is `arm_on` on the instrument's network under the share.** So H-33's, H-34's, H-28's and H-35's arms are each the call they were and read what the tree pins. What is added to them reads and does not write: the two holds above that each lacked, and on a site the first block before the arm.
- **A network as tables are read against it is a value too** (`Stand`): the stimulus set's units, the image's sums and couplings, its two arms under the true reward (`Own`), and the run the reading of the first block apart is read against. The instrument's is the constants the tree holds, with H-25's arms its own and H-28's blocks; a site's is its pinned derivation's and its two arms' pinned tables. `misled_tables_on(stand, …)` is the gate's loop given it, and `misled_tables` calls it with the instrument's.

A second body beside `site_arm` (option 1(b)) or a third (option 1(c)) would leave H-33's arm and the arm at three in four on its network in two bodies, and a difference between them could then be the copy's: ADR-0172's reason against a copy of the arm for a second share.

### H-36's arms (`others_arm`)

- **Four arms**, each a weekly test of its own: H-25's two on each of `seeded_site(0)` and `seeded_site(1)`, seeds 23 and 24, through `arm_on` on the site under `three_in_four_on(n)`.
- **`three_in_four_on(n)` is H-35's share** — the feedback, the coin as the harness writes it and as the task draws it, the reliability's part, the coin's bounds, its count by mapping and the count of the trials that mislead here alone — with the name the network's dumps carry, `infour23` and `infour24`, and the network's arms' pinned tables.
- **Each is H-33's arm of its network with the feedback the one change, and H-35's arm with the network the one change**: H-25's image of the network, the drawn delivery, H-20's flips, 7 680 trials, the engine's critic with its window, the task carrying no critic, no hold and no shadow; the task's seed 27.

### The calibration, before any rewarded run at three in four (H-36's stopping rule, step 2)

- **In the tree**: the workspace's tests in the debug and the release profile and on the MSRV; and every whole-domain test of the weekly job before this round, each from the release build of the protocol's commit — every pinned number of the tree, **H-33's four arms, H-34's four, H-28's two and H-35's two through the form among them**.
- **In each arm's test**, before its first rewarded trial:
  - **the coin's count** held to H-35's, 363, 514, 515 and 521 by mapping and 1 913 in all, within H-35's bounds; nested in seven in eight's, with 182, 252, 265 and 265 misleading here alone; and the harness's coin held to the task's at every trial of the schedule;
  - **the derivation** run again by the rules and held to the one ADR-0167 pinned, table by table, clause 0 holding;
  - **the network's first block under the true feedback** (`own_first_block`, option 3(a)): the arm's first sixty-four trials from H-25's image of the network under `Feedback::Answer`, through the `fed_run_at` the arm then runs, held to the first row of the sight's blocks, the earned blocks, the engine's value and the drawn sources that H-33's arm of the network pins.

A failure at any of them stops the round there as a finding, with no rewarded run at three in four on either network.

**The first block is held on every arm of a site, the true reward's among them.** There it is the arm's own first sixty-four trials, run once before the arm on an engine of their own and held to the row the arm then pins. It costs H-33's and H-34's arms one block in 120 and changes nothing of their runs, and it puts the check where the calibration and every weekly run reach it: the step an arm at three in four rests on has reproduced on its network before that arm runs, and a failure of it there is the network's and not the check's.

H-33's whole arm inside each arm (option 3(b)) is twenty-five minutes of a run the calibration and the weekly job already hold, four times over.

### H-36, restated as integer rules (ADR-0173's clauses and constants)

- **Clauses 1 to 4 are H-35's, by H-35's rules, on each network against the network's own image** (`misled_on`, option 5(a)): `misled_at` with the image's couplings given, clause 2 read by `scheduled_of` against them and clause 3's band against the image's own outside sum; clause 4 at a half, `holds_expected_reward_at` at `IN_FOUR_RELIABILITY`. `misled_at` calls it with seed 22's couplings and reads what it read.
- **The verdict**: yes when all four hold in both arms on both networks; otherwise no, with the network, the arm and the clause.
- **The step** (`others_step`, option 6(a)), ADR-0173's steps 3 to 6:
  - 3 at a yes on both networks;
  - 4 at a no in which clause 3 fails on either network, whatever else failed;
  - otherwise 5 at a no on clause 1 or 2 on either network;
  - otherwise 6, a no on clause 4 alone.
- **The split, per network** (`split_on`): on a network where clause 1 or 2 fails, H-35's split by H-35's rule, `in_four_step`'s — *slower than the schedule* where clause 2 holds in both arms, the first mapping is learned in both and each of the six reversals has a crossing; *not revised* otherwise. It is read wherever clause 1 or 2 fails, whatever clause 3 read on that network.
- **The span among the candidates** (`OthersStep::span`): at step 5, true where every network that fails clause 1 or 2 reads *slower than the schedule*.
- `OTHERS_PREDICTED` is none: ADR-0173 predicts no verdict, and this round adds none.

### The four predicted readings, as rules (`others_predicted`)

1. **The first mapping learned in all four arms**: `in_four_predicted`'s first reading, on each network.
2. **On each network the six reversals slower than its own under the true reward**: `in_four_predicted`'s second with the network's own crossings where H-35 read H-25's — a hundred times the sum of the six ratios above 113 times six times its divisor, the sum an exact fraction, a reversal that does not cross counted at its 32 blocks. It holds when it holds on each of the two.
3. **The value nearer a half of $2p - 1$ than $2p - 1$** (`nearer_read`): over the cells read, a stimulus in a mapping clause 1 holds for whose accuracy over its trials there is at least three quarters, $4c \ge 3n$, the stimulus having at least one trial. It holds when every cell read is nearer (`nearer_expected_at` above zero); the count of the cells read is reported beside it.
4. **What the misleading trials undo** (`undone`, over `taken_back`'s four sums), in each arm: misleading punishments taking back at least a third and at most a half of what true rewards gave the answer's pairs, and misleading rewards putting back at least a third and at most a half of what true punishments took from the pair a wrong selection reached.

`OTHERS_READINGS_PREDICTED` is the four, each predicted to hold. They are dumped beside what is read and never asserted; a miss is recorded as one.

**The rules read over H-35's pinned tables on seed 22**, where ADR-0173 took their marks, before any run here: the first mapping learned; a mean ratio of 1.417, which ADR-0172 writes as 1.42; ten cells read of sixteen and all ten nearer; and both parts of the fourth in both arms. The gate holds that.

### The readings, no clause (ADR-0173's)

Per block, pinned; per mapping, computed from them. **H-35's tables whole, by H-35's rules**, on each network, so that every table of H-35 has its counterpart here block for block, and beside them:

- **each reversal beside the network's own under the true reward** (`beside_accounts`, `speeds`) and **beside ADR-0173's forecast** (`beside_forecast`): a hundred times its blocks less 142, 171 and 200 times its own;
- **the first block that is not the network's own under the true reward** (`first_apart`, in `BesideRead::leaves`): on seed 22 H-35 read it against H-28's;
- **the errors delivered and the clamp's part by kind over each mapping's last 128 trials** (`last_errors`, `last_beyond`), which H-35 computed from its blocks;
- **the network's own arms under the true reward, read by H-28's rules** (`ArmRead`, option 4(a)): H-33's four arms run through the form read the learning signal, what a reward moved, a true reward's error and the clamp's part as H-35's arms read them, and that reading is pinned from the calibration's run of them. No pinned number of H-33 moves; the dispatch reproduces the new ones;
- **over the three networks** (`over_networks`): the mappings learned of twenty-four, the reversals that crossed of eighteen, and the sum of the eighteen ratios to each network's own as an exact fraction.

Beside H-25's alone (option 4(b)), a signal that fell on seed 23 could be the network's and not the share's. From the dumps (option 4(c)), the readings beside could not be recomputed from the tree.

### The gate

`the_clauses_of_h_36_on_each_network_the_split_and_the_four_predicted_readings`, one test:
- the networks, the arms, the predictions and the constants; each network's share H-35's but for its name and its pins, the coin on it H-35's at every trial and the task's seed 27;
- the instrument's stand against the constants the tree holds, and each site's against ADR-0167's pins, with H-33's reversals as ADR-0173 lists them;
- clause 4's rule at the whole reliability against H-25's rule; the readings over a mapping's last 128 trials over tables written by hand;
- the verdict on a network over tables written by hand on that network's image: a yes; clause 2 at 1.30 of the network's own coupling and one past it, and a coupling past the bound on one network and within it on another; the form at the instrument's image against `misled_at`;
- the step and the split per network: a yes on both; clause 4 alone; clause 1 on one network, slower than the schedule, with the span among the candidates; one network not revised; clause 2; clause 3 on either network, with the split still read on the other;
- the four predicted readings' rules at their edges: the first mapping in every arm; the mark on each network against its own; a cell at three quarters read and one trial under it not, ADR-0172's miss read by ADR-0170's rule and not by this one; a third and a half, each counted, and one past each;
- the forecast's rule to the block; the reading over the networks by hand;
- the rules over H-35's pinned tables on seed 22.

It runs nothing on a network of another seed: a derivation is a weekly test's.

### The weekly tests and their cost

- **Four weekly tests**: `a_reward_right_three_times_in_four_on_seed_{23, 24}_from_the_{assignment, mirrored_assignment}_at_1024_units_exhaustive`.
- **Each is H-33's arm with its network's derivation before it.** H-33's four read 1 871 to 1 892 s in the last dispatch, and the table holds 110 tests and 83 129 s. Four more at that cost make about 90 650 s, as ADR-0173 planned: about 7 550 s summed a shard, 52 to 59 per cent of a shard's bound at the ratios every shard of the last three runs read but one.
- **The dispatch's scope**: no file under `src/` changes, so by [ADR-0075](0075-the-dispatch-scope-follows-the-diff.md) the weekly is dispatched at `scope=exhaustive`.

### The order of the work, as the history holds it

1. **The rules before their result.** The form, H-36's share on each network, the verdict's rule, the step and the split, the four predicted readings' rules, the readings' rules, the four arms and the gate's test were written with every pinned table of an arm of H-36 empty. The gate's test passed on the developer machine in the debug profile, with the gate's tests of H-28, H-33, H-34 and H-35 beside it; it runs no trial.
2. **Seven arms of the tree through the form, before the commit.** A new check of an arm fails only after its whole run, and an arm of H-36 runs once. So seven arms an earlier round pinned were run through `arm_on` from the release build of the working tree, side by side from 14:59:29Z to 15:42:36Z on 2026-10-12: H-33's four, on seeds 23 and 24; H-35's two; and H-28's from the assignment. **Each reproduced every pinned table and passed**, in 2 509 to 2 584 s beside one another (on the developer machine, a ratio and not admissible). Under the true reward every trial's reward was held to the outcome's sign for the first time, and on seed 22 the critic's step to the engine's value; both held at every trial of every arm. None of the seven is a trial at three in four on seed 23 or seed 24. H-35's two are runs at three in four on seed 22: they reproduce H-35's pinned tables, and nothing of H-35 is read from them again.
3. **The first block on every arm of a site, before the commit.** The step was then put in a site's calibration for every arm, and H-33's arm on seed 24 from the mirrored assignment was run from that build to the end of its calibration, from 15:27Z to 15:34Z: its first sixty-four trials, on an engine of their own, reproduced the first row of each table the arm pins. It was stopped there. One of H-34's arms, begun with the seven, was still running when the protocol was committed; the calibration runs all four of H-34's from the committed build.
4. **The protocol committed and pushed** before any rewarded trial at three in four on seed 23 or seed 24. On the working tree it was committed from, the format, Clippy over the runtime's targets, the check on the MSRV, the five gate tests named above in the debug profile and `npm run spec` exited 0, and 114 whole-domain tests are listed, four of them this round's.
5. **The calibration.** **Every one of the 110 whole-domain tests of the weekly job before this round passed**, each in a process of its own and each exit 0 with one test reported, eight at a time from 15:56:29Z to 20:20:05Z on 2026-10-12, from a release build of `34e975c` with the tree clean. Through `arm_on` among them, each reproducing every pinned table: H-33's four arms, in 2 424 to 2 925 s; H-34's four at 2 048 units, in 5 259 to 5 300 s; H-28's two, in 2 550 and 2 745 s; and H-35's two, in 2 749 and 2 759 s (on the developer machine, a ratio and not admissible). On each of the eight arms of a site the network's first block under the true feedback reproduced before the arm, at 2 048 units as at 1 024. H-33's four arms dumped their reading by H-28's rules, each line the one the arm had dumped before the commit from another build.
6. **The arms**, once, side by side from that same build, from 20:20:42Z to 20:51:48Z. In each the calibration held before the first rewarded trial: the coin's count, 363, 514, 515 and 521, with 182, 252, 265 and 265 misleading here alone; the derivation, table by table the one ADR-0167 pinned; and **the network's first block under the true feedback, H-33's**. Each ran its 7 680 trials with every reward held to the coin, the critic's step to the engine's value, the drawn sources to the critic's oracle and both oracles to the record at every trial, dumped every table and stopped at the first empty one, in 1 850 to 1 865 s (on the developer machine, a ratio and not admissible). The tables were written from those dumps and committed as `a3b223c`, with H-33's arms' reading by H-28's rules from the calibration's logs.

No constant, clause or rule moved after the first rewarded trial of an arm, and there was no second run: the weekly dispatched on `a3b223c` is the pinned tables' reproduction. With the tables, the gate's test gained its checks over them and the verdict's four constants, written from the rules' own output over those tables.

### The readings

**The verdict** (`OTHERS_1024`), by the rules committed first, over the pinned tables:

| Clause | Seed 23, from the assignment | Seed 23, from the mirrored | Seed 24, from the assignment | Seed 24, from the mirrored |
| :--- | :--- | :--- | :--- | :--- |
| 1, each mapping's last 128 correct, at least 80 | 121, 115, 121, 119 | 121, 97, 117, 112 | 121, **71**, 116, **76** | 107, 119, 112, 112 |
| 2, the highest coupling over its image's, at most 1.30 | 1.180, B onto readout 1 at the 85th block | 1.136, A onto readout 1 at the 24th | 1.159, B onto readout 1 at the 88th | 1.195, B onto readout 0 at the 88th |
| 3, the excitatory sum outside the couplings, 0.75 to 1.25 of the image's | 0.9999 to 1.0004 | 0.9999 to 1.0001 | 0.9998 to 1.0001 | 0.9997 to 1.0001 |
| 4, each stimulus's mean value within 0.25 of a half of $2p - 1$ | all eight; the farthest 0.195 | all eight; the farthest 0.206 | all eight; the farthest 0.148 | all eight; the farthest 0.216 |

**H-36 is no. On seed 23 all four clauses hold in both arms. On seed 24 clause 1 fails in two mappings of eight**, the second and the fourth from the assignment, 71 and 76 of their last 128 trials correct where 80 are asked. Clauses 2, 3 and 4 hold in all four arms.

Under the true reward the same arms read 126, 124, 124, 123 and 122, 115, 118, 126 on seed 23, and 117, 123, 120, 115 and 124, 120, 121, 123 on seed 24. On seed 22 H-35 read 115, 80, 110, 75 and 117, 94, 110, 97.

ADR-0173 predicted no verdict.

**Over the three networks** (`OTHERS_OVER_1024`): **twenty-one mappings of twenty-four learned** — seven, eight and six on seeds 22, 23 and 24 — and **sixteen reversals of eighteen crossed** 40 of 64 inside their 32 blocks. The three mappings that failed are all in the arm from the assignment, in its second or its fourth mapping: seed 22's fourth, and seed 24's second and fourth.

**The four predicted readings** (`OTHERS_READINGS_READ_1024`), each predicted to hold:

| Predicted reading | Read |
| :--- | :--- |
| 1, the first mapping learned in all four arms | **Held**: 121, 121, 121 and 107 of its last 128 |
| 2, on each network the mean of the six reversals' ratios to its own under the true reward above 1.13 | **Held**: 1.31 on seed 23 and 1.42 on seed 24, the reversal that did not cross counted at its 32 blocks |
| 3, the value nearer a half of $2p - 1$ than $2p - 1$ in every cell read | **Held**: twenty-seven cells read, fifteen on seed 23 and twelve on seed 24, and every one nearer |
| 4, in each arm the misleading trials undoing between a third and a half | **A miss, in two arms of four**: misleading punishments took back 50.9 per cent on seed 23 from the mirrored assignment and 52.6 on seed 24 from the assignment |

The miss is on the reading's first part. Misleading punishments took back 48.6 and 50.9 per cent of what true rewards gave the answer's pairs on seed 23, and 52.6 and 44.8 on seed 24; H-35 read 42 and 44 on seed 22. Its second part held in all four: misleading rewards put back 37.3 and 34.9 per cent on seed 23 and 40.0 and 40.3 on seed 24 of what true punishments took, where H-35 read 43 and 41. ADR-0173 wrote of the reading that what the misleading trials undo *"is the share's and not the network's"*. Over the six arms of three networks the one share took back 42 to 53 per cent: the network has a part in it.

Of the five cells the third reading does not read, two are not nearer the half: A in the second mapping from the mirrored assignment on seed 23, correct in 41 of 64, and B in the second mapping from the assignment on seed 24, correct in 31 of 64. ADR-0170's reading, without the condition, would have missed in the first.

**The reversal speeds** (`crossings`, `beside_forecast`; the blocks to the first of a mapping's blocks with 40 of 64 correct):

| | First reversal | Second | Third |
| :--- | :--- | :--- | :--- |
| Seed 23, from the assignment | 17; its own 15 | 19; its own 13 | 16; its own 13 |
| Seed 23, from the mirrored | 21; its own 16 | 14; its own 14 | 21; its own 12 |
| Seed 24, from the assignment | 32; its own 21 | 12; its own 12 | **none in its 32**, its best block 39; its own 15 |
| Seed 24, from the mirrored | 18; its own 17 | 16; its own 16 | 25; its own 14 |
| ADR-0173's forecast at 1.42 of its own, seed 23 | 21.3 and 22.7 | 18.5 and 19.9 | 18.5 and 17.0 |
| The same, seed 24 | 29.8 and 24.1 | 17.0 and 22.7 | 21.3 and 19.9 |

- As ratios to each network's own: 1.13, 1.46, 1.23 and 1.31, 1.00, 1.75 on seed 23, **a mean of 1.31**; 1.52, 1.00, at least 2.13 and 1.06, 1.00, 1.79 on seed 24, **a mean of 1.42** with the one that did not cross counted at its 32 blocks. Seed 22's mean is 1.42 too. **Over the eighteen it is 1.38**; over these twelve, 1.37.
- **Beside the forecast.** At H-35's mean ADR-0173 put every reversal inside its 32 blocks, one on seed 24 at their edge. On seed 23 all six crossed, in 14 to 21 blocks, four of them under 1.42 of their own. On seed 24 the one forecast at the edge, 29.8 blocks, crossed in its thirty-second; four crossed in 12 to 25; and the one forecast at 21.3 did not cross.
- **Three reversals of twelve took exactly their own blocks**: the second from the mirrored assignment on seed 23, and the second in each arm on seed 24.
- **The third reversal is the slow one in three arms of four**: 1.75, at least 2.13 and 1.79 of its own. On seed 22 the third was the slowest in both arms, at least 2.00 and 1.71.
- **A crossing is not a mapping learned**, as on seed 22: seed 24's first reversal from the assignment crossed with 41 in its last block and held 71 over its last two.
- **The two mappings that failed were still rising.** Seed 24's second from the assignment held 0, 1, 3, 4, 2, 2, 6, 6, 6, 9, 8, 9, 20, 15, 20, 16, 13, 18, 19, 14, 20, 24, 28, 20, 23, 31, 32, 29, 32, 39, 30 and 41 correct in its 32 blocks; its fourth 4, 4, 2, 9, 3, 8, 8, 10, 6, 11, 12, 11, 16, 13, 18, 16, 16, 18, 23, 23, 24, 27, 28, 25, 25, 34, 32, 39, 31, 34, 38 and 38.
- **In them neither stimulus had turned as far as it turned elsewhere, and B less.** Over the second mapping's last 128 trials A was correct in 40 of 64 and B in 31 of 64; over the fourth's, A in 35 of 54 and B in 41 of 74. B's coupling onto its answer stood above its coupling onto the other readout by a twentieth of what parted them under the true reward, at the end of each. On seed 22 it was B that had not turned.
- The first mapping passed 40 of 64 in its 7th and 3rd block on seed 23 and its 2nd and 11th on seed 24; under the true reward in its 4th and 3rd, and its 1st and 8th.

**The trials by outcome and by the reward received** (`OTHERS_OUTCOMES`; true rewards, true punishments, misleading rewards, misleading punishments):

| Mapping | Seed 23, assignment | Seed 23, mirrored | Seed 24, assignment | Seed 24, mirrored |
| ---: | :--- | :--- | :--- | :--- |
| 1 | 861, 312, 92, 271 | 957, 216, 63, 300 | 1 028, 145, 43, 320 | 701, 472, 151, 212 |
| 2 | 775, 759, 253, 261 | 723, 811, 257, 257 | 394, 1 140, 368, 146 | 843, 691, 213, 301 |
| 3 | 776, 757, 255, 260 | 883, 650, 210, 305 | 1 078, 455, 168, 347 | 808, 725, 247, 268 |
| 4 | 801, 726, 245, 276 | 720, 807, 277, 244 | 447, 1 080, 362, 159 | 625, 902, 309, 212 |

- Over a run, **1 068, 1 106, 972 and 993 correct selections were punished and 845, 807, 941 and 920 wrong ones or ties rewarded**, 1 913 in each: a correct selection was punished in 24.8 to 25.2 per cent of the correct trials, the coin's share.
- A trial tied in 3.1 to 7.4 per cent of a mapping's trials; under the true reward in 3.2 to 5.7.
- Over whole mappings the correct trials were 0.48 to 1.07 of the network's own under the true reward: 0.55 and 0.48 in the two that failed, and above one once, the third from the assignment on seed 24.

**The learning signal** (`signal_of` over `OTHERS_WENT`, the network's oracle; the answer pairs' net consolidation less the other pairs', per mapping, over the network's own under the true reward, `SEEDED_WHOLE`):

- Seed 23: **0.79, 0.75, 0.77, 0.81** from the assignment and **0.88, 0.78, 0.75, 0.72** from the mirrored.
- Seed 24: **0.93, 0.67, 0.63, 0.63** from the assignment and **0.68, 0.64, 0.75, 0.67** from the mirrored.
- **The naive scaling says 0.5, and every one of the sixteen is above it.** On seed 22 H-35 read 0.46 to 0.73 of H-25's.
- **The signal does not tell the mappings that failed from one that held.** From the assignment on seed 24 it is 0.67 and 0.63 in the second and the fourth mapping, which failed, and 0.63 in the third, learned at 116 of 128.

**What a reward of each kind moved** (`OTHERS_KIND_MOVES`, `OTHERS_KIND_ERRORS`, `OTHERS_BEYOND`, `taken_back`), over the six arms of the three networks, seed 22's from H-35's pinned tables:

| | Seed 22 | Seed 23 | Seed 24 |
| :--- | :--- | :--- | :--- |
| The answer's pair, a trial after a true reward | 3 280 and 3 860 | 4 640 and 4 490 | 4 320 and 4 510 |
| The same under the true reward on the network | — | 1 900 and 1 880 | 1 990 and 2 240 |
| The same pair, a trial after a misleading punishment | −4 370 and −4 970 | −6 780 and −6 790 | −6 890 and −6 070 |
| As a multiple of what a true reward gave | 1.33 and 1.29 | 1.46 and 1.51 | 1.59 and 1.34 |
| Taken back by misleading punishments, per cent | 42 and 44 | 49 and 51 | 53 and 45 |
| The pair a wrong selection reached, a trial after a true punishment | −3 350 and −3 470 | −4 650 and −4 650 | −4 460 and −4 290 |
| The same pair, a trial after a misleading reward | 4 250 and 4 320 | 5 410 and 5 220 | 5 450 and 5 240 |
| Put back by misleading rewards, per cent | 43 and 41 | 37 and 35 | 40 and 40 |

*A true reward's error grew, and on these networks what it moved grew with it.*
- Over a mapping's last 128 trials a true reward's error was **0.47 to 1.05 of the reward**, where under the true reward on the same networks it is 0.05 to 0.16. H-35 read 0.56 to 0.93 on seed 22.
- In the trial after a true reward the answer's pair gained **2.0 to 2.4 times what it gains under the true reward on its network**.
- **A true reward left the signal above the gate's ceiling in 41.5, 40.1, 38.1 and 45.7 per cent of them**, by 0.32 to 0.37 of the reward on average; under the true reward on these networks in 16.6, 16.1, 18.9 and 20.6. On seed 22 H-35 read 46.6 in each arm.

*A misleading punishment costs more here than on seed 22.*
- It took from the answer's pair 1.34 to 1.59 times what a true reward gave, where on seed 22 it took 1.29 and 1.33.
- It left the signal below the gate's floor in 62.2, 59.7, 60.8 and 54.2 per cent of them, by 0.47 to 0.51 of the reward; on seed 22 in 53 and 52.

*The pair a wrong selection reached is put back at about seed 22's rate.*
- A misleading reward left the signal above the ceiling in 55.5, 57.7, 60.6 and 53.3 per cent of them; a true punishment left it below the floor in 43.0, 43.7, 39.6 and 44.3, where under the true reward on these networks it does in 34 to 37.
- In the trial after a misleading reward that pair gained 1.12 to 1.22 times what a true punishment took from it.

*Over the whole run* the signal a reward left lay beyond the gate's bounds after **46.4, 46.0, 44.3 and 47.2 per cent of the trials**; under the true reward on these networks after 22.8 to 26.4, and on seed 22 at this share after 47.3 and 46.9.

*In the mappings that failed the wrong side moved most.* By mapping from the assignment on seed 24, a trial after each kind:

| Mapping | Correct of its last 128 | The answer's pair after a true reward | After a misleading punishment | The pair a wrong selection reached after a true punishment | After a misleading reward |
| ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 121 | 4 910 | −9 920 | −3 920 | 2 050 |
| 2 | **71** | 2 780 | −2 630 | −4 710 | 5 730 |
| 3 | 116 | 4 780 | −7 590 | −3 000 | 2 450 |
| 4 | **76** | 3 210 | −3 160 | −4 840 | 6 740 |

In the two that failed a misleading reward moved the pair it reached by 2.1 times what a true reward moved the answer's pair, and by more than a true punishment took from it; in the two that held it moved it by half as much, and a misleading punishment took 2.0 and 1.6 times what a true reward gave, where in the two that failed it took less than one. **Over the twenty-four mappings of the three networks that ratio is highest in these two**, 2.10 and 2.06. Seed 22's mapping that failed and its mapping on the mark read 1.40 and 1.48, the fifth and the fourth highest; the twenty learned above the mark read 0.42 to 1.65. **Which way this runs is not told by these tables**: in a mapping that has not turned, the pair a wrong selection reaches is the old answer's, which is the stronger coupling, and a larger move of it may be what a mapping not yet turned looks like as much as what keeps it from turning.

**What this does not say.** These are readings of the runs as they were. Each run has one gate, one critic and one sequence of trials: what the learning would have been with a wider clamp, a smaller weight on one error, another step of the critic or another task seed was not run and is not derived.

**The sequence is every network's.** The task's seed is 27 on all three, so the stimuli and the 1 913 misleading trials are the same trial for trial. The three mappings that failed stand at the same two places of the schedule, the second and the fourth, in the one arm, on two networks of three. On seed 23 that arm learned those two mappings at 115 and 119 under the same sequence, so the sequence alone does not fail them; and on seed 24 the arm from the mirrored assignment learned them at 119 and 112. What part the sequence has beside the network's is not told apart: no second seed of the task was run.

**The value** (`stimulus_values`; per mapping and stimulus over the mapping's last 128 trials):

- **The critic holds the expected reward.** In all thirty-two cells the mean value is within 0.22 of the reward of a half of $2p - 1$, and in thirty it is nearer the half than $2p - 1$. The two that are not are the cells named above, at accuracies of 0.64 and 0.48.
- **Clause 4 tells the three apart here too.** H-25's rule, a quarter of the reward either side of $2p - 1$, holds seven of these thirty-two. H-28's clause, at three quarters, holds twenty-four. H-35's holds all.
- **The critic's step is what it is under the true reward**: over every mapping of every arm a presentation moved the value by 2.56 per cent of its error, where the same arms under the true reward read 2.54 to 2.56. No trial's value moved against its error.
- **The troughs after each flip are shallower**: −0.10 to −0.60 of the reward, against −0.56 to −0.89 under the true reward on these networks, each of the twenty-four shallower than its own.

**Where else the runs stand beside their own under the true reward.**
- **The couplings rose about as far.** The highest was 1.136 to 1.195 of its image's; under the true reward 1.182 to 1.195. On seed 22 H-35 read 1.111 and 1.126 where H-25 read 1.181 and 1.191.
- **The couplings' separation** at each mapping's end, the answer's coupling less the other's, was 0.05 to 1.21 of its own under the true reward per stimulus, and above zero in every one of the thirty-two.
- **The inhibitory sum** ended at 0.9055 and 0.9056 of the image's on seed 23 and 0.8903 and 0.8933 on seed 24, its lowest; under the true reward at 0.9082 and 0.9101, and 0.8957 and 0.8968.
- **The drawn sources** were 99.67 and 99.68 per cent of the presented stimulus's units and 1.62 others a trial in each arm, as under the true reward.
- **The first new selection** after a flip came within 0 to 193 trials; under the true reward within 4 to 158.
- **The runs leave their own under the true reward in the first block**: the third trial is the first the coin misleads at, and the first block's row of the sight's table is not H-33's in any arm.

**In these four runs every error had its reward's sign** (`errors_against` reads none in any arm): the engine's value never passed the reward. So the tables of H-33 and H-25 that read a delivery's sign read here what they read there, and the gate holds that block by block for all four arms. It is a reading of these runs and not a property of the rule.

**The oracles held** at every one of the 30 720 trials: the drawn sources to the units the harness's critic oracle counted from the train, the targets to the selected readout's units, the engine's value, error and weights to the harness's critic, the value of the window's counts to the engine's own, the reward to the coin, and the composer's and the network's traces and weights to the record.

### The evidence

- **The dispatch's scope.** The diff changes no file under `src/` — the runtime's tests, the documents and the cost table — so by [ADR-0075](0075-the-dispatch-scope-follows-the-diff.md) the weekly was dispatched on this round's branch at **`scope=exhaustive`**, as brief 070 asks: run [38307712843](https://github.com/DescentVTT/VirtualCortex/actions/runs/38307712843) at `a3b223c`, the pins' commit. The round's commits, in pull request #204: `34e975c` the protocol, before any rewarded run at three in four on either network; `a3b223c` the arms' tables, H-33's arms' reading by H-28's rules and the gate's checks over them; `f3f73a3` the readings and the documents; and the commit that carries this section, the cost table and the brief's archive.
- **Every whole-domain shard is green.** All 114 `exhaustive` tests passed on the hosted runners, each exit 0 with one test reported, 81 785 s summed. The 110 before this round reproduced their pinned numbers, the twelve arms of them that now run through the one body among them: H-33's four in 1 149 to 1 802 s, H-34's four at 2 048 units in 2 165 to 3 554 s, H-28's two in 1 001 and 1 728 s, and H-35's two in 1 249 and 1 779 s. **H-36's four arms reproduced their tables there**: on seed 23 in 1 245 s from the assignment and 1 827 s from the mirrored assignment, and on seed 24 in 1 784 and 1 779 s. H-33's four held their reading by H-28's rules, pinned from the calibration.
- **The shards**, dealt by the table before this round, which did not know the four arms and costed each at 900 s:

  | Shard | Job | Its two heaviest tests (s) | Tests | Their seconds summed | Tests' wall time |
  | ---: | ---: | :--- | ---: | ---: | ---: |
  | 0 | 67.1 min | H-34, seed 22 from the assignment 3 554; H-20 from the assignment 1 683 | 8 | 7 813 | 3 949 s, 55 % |
  | 1 | 42.9 min | H-34, seed 23 from the mirrored 2 165; H-21 from the assignment 1 071 | 9 | 5 002 | 2 516 s, 35 % |
  | 2 | 71.4 min | H-34, seed 23 from the assignment 2 427; **H-36, seed 23 from the mirrored 1 827** | 9 | 6 981 | 4 206 s, 58 % |
  | 3 | 82.1 min | H-34, seed 22 from the mirrored 3 069; H-32 from the assignment 1 785 | 10 | 8 956 | 4 846 s, 67 % |
  | 4 | 59.7 min | brief 060's control from the assignment 1 808; H-29 from the assignment 1 388 | 10 | 6 478 | 3 503 s, 49 % |
  | 5 | 48.2 min | H-33, seed 24 from the mirrored 1 415; H-20 from the mirrored 1 342 | 9 | 5 629 | 2 831 s, 39 % |
  | 6 | 40.7 min | H-33, seed 23 from the assignment 1 149; H-28 from the mirrored 1 001 | 11 | 4 613 | 2 386 s, 33 % |
  | 7 | 57.3 min | H-33, seed 24 from the assignment 1 431; H-32 from the mirrored 1 385 | 9 | 6 306 | 3 373 s, 47 % |
  | 8 | 69.6 min | H-33, seed 23 from the mirrored 1 802; H-27 from the mirrored 1 775 | 10 | 7 938 | 4 100 s, 57 % |
  | 9 | 52.5 min | H-31 from the assignment 1 703; H-29 from the mirrored 1 315 | 9 | 6 136 | 3 091 s, 43 % |
  | 10 | 74.2 min | H-31 from the mirrored 2 112; **H-36, seed 24 from the assignment 1 784** | 10 | 8 384 | 4 372 s, 61 % |
  | 11 | 64.7 min | **H-36, seed 24 from the mirrored 1 779**; H-30 from the mirrored 1 751 | 10 | 7 549 | 3 797 s, 53 % |

  The percentages are of the job's bound, 120 minutes; each shard ran its tests two at a time, at 1.66 to 1.99 of their summed seconds over the wall. The twelve started within five seconds of the dispatch, no sweep and no other run holding a place, and the last ended at 22:21:06Z, 1 h 22 min after it.

  **Two shards of this run passed 60 per cent of their bound.** The fourth read 67: it holds one of H-34's arms, 3 069 s on this run's runner, which brief 070's directive excepts and reports with the arm's seconds. The eleventh read 61: it drew one of this round's arms, costed at 900 s and running 1 784, beside H-31's at 2 112. The other three shards that drew an arm of this round, the third, the tenth and the twelfth, read 58, 43 and 53; the tenth's, H-36's on seed 23 from the assignment, ran 1 245 s and is its third heaviest.
- **The cost table is regenerated from this run** (`node scripts/exhaustive-costs.mjs from <artifacts> --run 38307712843`): 114 lines, 81 785 s, where it held 110 and 83 129. ADR-0092's deal plans each of the twelve shards at 6 814 to 6 817 s summed, about 3 430 to 4 110 s of wall time at this run's ratios, **48 to 57 per cent of the bound**. No shard is planned past 60.

  **The table is one run's, and this run's runners were fast.** The 110 tests before this round summed to 75 150 s here where the last run read them at 83 129, a tenth less: H-33's four arms read 1 149 to 1 802 s where they read 1 871 to 1 892. At the last run's seconds for those 110 with this round's four, 89 760 s, the deal would plan 7 480 s a shard, 52 to 63 per cent at these ratios, near what ADR-0173 reckoned. So the plan holds under 60 on this run's table and not on the last one's, and the workflow's decision ADR-0168 named stays due before the round after this one.
- **The schedule's sweep, written down here** as [ADR-0158](0158-the-sweep-off-the-next-decisions-path.md) asks of the first ADR that merges after a sweep has ended. Run [38234325824](https://github.com/DescentVTT/VirtualCortex/actions/runs/38234325824), the schedule's weekly on `main` at `d090b30`, the tree before ADR-0171's feedback: its sweep ended at 15:00:53Z on 2026-10-12, green in all thirteen jobs, after ADR-0173 had merged and before this ADR.
  - **3 849 mutants over the tree, 3 671 caught and none missed**: 2 404 caught in the state crates and 1 267 in the runtime, 154 unviable and 24 timeouts. It is the count the sweep of run 38111246283 read: no file under `crates/` or the runtime's `src/` differs between the two commits.
  - **In `runtime/cortex-runtime/src/task.rs`**: 208 caught and 10 unviable, none missed and none timed out.
  - **Twenty-two of the timeouts** are in the places [ADR-0062](0062-the-first-complete-sweeps-list.md) reads as inherent: `cortex-core`'s three chain iterators, six; the injector, twelve; the barrier's wait, one; the workers' stop and run, three.
  - **The other two are of F-42's kind**, by [ADR-0063](0063-the-sweep-reads-its-own-timeouts.md)'s evidence, each a mutant of `Image::decode` starved by a hung mutant of the injector in its shard's other slot:
    - `image.rs:1045:24: delete ! in Image::decode`, F-42's own mutant, in `runtime-11`: 1 354 s against a bound of 1 332, 1 273 of them beside the hung mutant. The sweep of run 38228081172 timed it out too, as ADR-0173 records, and the one of run 38111246283 caught it.
    - `image.rs:1053:24: delete ! in Image::decode`, in `runtime-0`: 1 716 s against a bound of 1 698, 1 624 of them beside the hung mutant. The sweep of run 38228081172 caught it, in `runtime-0`.

    Each log ends in `learning` with a test reported after the last long-running warning: the run was still finishing tests, with no test of the mutant's own module hung, which the job's evidence names as F-42's kind. A timeout is a detection and not a survivor, and F-42 stands as it is written.
  - **Its twelve shards over the runtime took 54 m to 2 h 18 m each.** The thirteen jobs started from 27 s to 1 h 25 m after its last whole-domain shard ended, the sweep of brief 069's dispatch holding places beside them, and the sweep ended 4 h 53 m after the run began.
  - There is no survivor, so it leaves no finding. **No sweep that has ended is without its record**, and this round dispatches none: its scope is `exhaustive`.
- **The pull request's gate** on `34e975c` and on `f3f73a3` is green in every job, the determinism pin on AArch64 and the MSRV among them. The mutation gate on the changed lines found no mutant to make, in each: no line under `src/` changed. The gate on `a3b223c` was cancelled by the push of `f3f73a3`, whose tests are its.
- **On the developer machine**, at `a3b223c`: the workspace's tests passed in the debug profile, in the release profile and on the MSRV, 738 passed and 154 ignored in each; the check, the format, Clippy, the documentation, the benchmarks and `npm run spec` exited 0; 114 whole-domain tests are listed; and `cargo mutants --in-diff` over the round's diff found no mutant to make.

### The step of the stopping rule reached

**Step 5** (`OTHERS_STEP_1024`): a no on clause 1 on seed 24, with clause 3 held on both networks. Seed 23 reads yes and has no split. **Seed 24 reads *not revised***, by H-35's rule: clause 2 holds in both arms and the first mapping is learned in both, but one of its six reversals, the third from the assignment, has no block with 40 of its 64 trials correct inside its 32.

ADR-0173's step 5: *"the no at three in four is not one network's. The next decision is the ADR H-35's step 5(b) named, on a single error's weight and the gate's clamp, with three networks' readings by kind as its need; where every network that fails reads slower than the schedule, the span a mapping is given is among its candidates."* **The next decision is that ADR.** Both networks that fail, seeds 22 and 24, read *not revised*, so by the rule the span a mapping is given is not among its candidates. It is named and not taken.

**How near the other branch the rule read.** The rule is the one written first and its reading stands. What the tables say beside it, for the decision it names:
- seed 24's reversal that did not cross had its best block at 39, one short of the 40 that would have made the branch *slower than the schedule*, in the 28th of its 32 blocks, and held 38 and 38 in its last two;
- its other reversal that failed crossed, in its last block;
- on seed 22 the reversal that did not cross had its best block at 38;
- so on both networks that fail, the mappings that fail were rising at their end, and the branch turns on one and two trials of one block.

**What the three networks hand that ADR**, by kind:
- **The count.** Twenty-one mappings of twenty-four learned and sixteen reversals of eighteen crossed; two networks of three fail, each in the arm from the assignment, and the third learns all eight with its lowest at 97. The reversals take 1.31 to 1.42 of each network's own under the true reward.
- **The gate's clamp, on the side that teaches.** A true reward left the signal above the ceiling in 38 to 47 per cent of them on the three networks, where under the true reward on seeds 23 and 24 it does in 16 to 21. On those two the answer's pair still gained 2.0 to 2.4 times what it gains under the true reward.
- **A single error's weight.** A misleading punishment took 1.29 to 1.59 times what a true reward gave, and misleading punishments took back 42 to 53 per cent. The arm that took back most, 53, is one that failed; the arm that took back 51 learned every mapping.
- **The wrong side.** Misleading rewards put back 35 to 43 per cent of what true punishments took from the pair a wrong selection reached. Of the twenty-four mappings, the four at or under the mark hold four of the five highest ratios of a misleading reward's move of that pair to a true reward's move of the answer's, 1.40 to 2.10. Which is the cause and which the effect is not told.
- **The critic's step.** The value held a half of $2p - 1$ within 0.22 in all forty-eight cells of the three networks, and its step is the true reward's. Nothing in these readings says the critic's step is what failed.
- **The learning signal.** It was 0.46 to 0.93 of the true reward's over the three networks, and on seed 24 it does not tell the mappings that failed from one that held.
- **What is not told apart.** The same trials mislead on every network, and the mappings that failed stand at the same places of the schedule. A second seed of the task was not run.

Steps 3, 4 and 6 did not arise; step 7 is kept.

## Consequences

- Good: the need is read on three networks before a rule of the engine is changed. The no at three in four is not one network's: two of three fail, and the third learns all eight mappings with its lowest at 97.
- Good: the no on seed 24 says which no, by a rule written before the run, and the tables say how near the other branch it stands.
- Good: every reading by kind has the network's own arm under the true reward beside it, by tables of the tree: the clamp met by true rewards in 38 to 46 per cent of them where it is met in 16 to 21, a true reward's error 0.47 to 1.05 of the reward where it is 0.05 to 0.16, and the critic's step unchanged.
- Good: an arm of the tree is one body. A network or a share is a value, and an arm of any network at any share is a call.
- Good: each arm is held more than it was: on seed 22 the critic's step is held to the engine's value at every trial, under the true reward every trial's reward is held to the outcome's sign, and on a site the network's first block is held before the arm.
- Bad: one share, one size, one generator and one sequence of trials. The mappings that failed stand at the same places of the schedule on both networks that fail, and the sequence's part is not told from the network's.
- Bad: the verdict on seed 24 turns on nine and four trials of two mappings, and the branch on one trial of one block.
- Bad: the readings by kind do not order the candidates. The arm that lost most to misleading punishments failed and the next learned every mapping; the learning signal does not tell a mapping that failed from one that held; and the one reading that does stand apart in the mappings that failed, the wrong side's move, may be their effect.
- Bad: twelve arms of earlier rounds run through a body that is not the one they were pinned from. What holds them is their pins, in the calibration and in every weekly run.
- Bad: four more arms in the weekly job, 1 245 to 1 827 s each in the dispatch. The regenerated table, 114 tests and 81 785 s, plans each shard at 48 to 57 per cent of its bound; on the last run's seconds for the same tests it would plan 52 to 63, and two shards of this run read 61 and 67.
- Neutral: a predicted reading missed. What misleading punishments take back is not the share's alone: 42 to 53 per cent over six arms.

## Alternatives considered and why rejected

- **A second or a third body** (options 1(b) and 1(c)), **a share for the true reward** (option 2(b)), **H-33's whole arm inside each arm** (option 3(b)), **readings beside H-25's alone or from the dumps** (options 4(b) and 4(c)): see above. A share for the true reward would carry twenty-three pinned tables H-33's arms do not have.
- **The verdict against seed 22's image** (option 5(b)): clause 2 is a coupling against its image's, and another network has another image.
- **The step read by eye** (option 6(b)): the step and the split decide which ADR is next.
- **One test a network** (option 7(b)): two arms and a derivation in one test, past a quarter of a shard's bound, and one arm's failure hiding the other's tables.

## Confirmation

- `runtime/cortex-runtime/tests/inhibition.rs`: `Ground`, `Stand`, `Own`, `ArmRead`, `arm_on`, `instrument_made`, `site_made`, `own_first_block`, `instrument_stand`, `site_stand`, `misled_on`, `misled_tables_on`, `last_errors`, `last_beyond`; `OTHERS_SEEDS`, `OTHERS_ARMS`, `three_in_four_on`, `NetworkArms`, `pinned_network`, `split_on`, `others_step`, `OthersStep`, `nearer_read`, `undone`, `others_predicted`, `beside_forecast`, `over_networks`, `Over`; the four weekly tests and the gate's test named above.
- The arms' pinned tables, `OTHERS_*`, each `[network][arm]`; H-33's arms' reading by H-28's rules, `SEEDED_WHOLE`; the verdict `OTHERS_1024`, its step `OTHERS_STEP_1024`, the predicted readings as read, `OTHERS_READINGS_READ_1024`, and the reading over the networks, `OTHERS_OVER_1024`, with the gate's checks over them.
- Whitepaper §11.1 and §9; `CHANGELOG.md`; `CLAUDE.md`; `README.md`; `docs/zh-TW/README.md`.
