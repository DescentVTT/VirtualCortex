---
status: accepted
date: 2026-10-13
depends-on: ADR-0180
title: A rate the controller holds, measured
kind: measured
hypothesis: H-38
summary: 'H-38 is yes, as ADR-0178 predicted: on seed 22 at 1 024 units clause 0 holds at every step of the six, and under an eighth H-25''s two arms hold H-25''s four clauses. Each of the four predicted readings held, and the steepness reads fifteen, an eighth at its loop''s edge.'
decision-makers: VirtualCortex maintainers
---

# ADR-0181: A rate the controller holds, measured — H-38 is yes, as ADR-0178 predicted: on seed 22 at 1 024 units with the controller holding 58 201 ticks a spike, clause 0 holds at every step of the six, each lead-in settling at the window its sums do and its frozen block passing both measures; under an eighth, the largest, H-25's two arms hold H-25's four clauses, 118 to 124 of each mapping's last 128 correct and each reversal passed in 15 to 19 blocks, with the count held at its target and the gain two per cent below its image's, alternating; each of the four predicted readings held; the steepness reads fifteen, which puts an eighth at its loop's edge and a sixteenth well inside it; the inhibitory sum ends at 0.78 of its image's where the gain held leaves 0.85; H-38's stopping rule reaches step 3, and the next decision, on what the engine carries by default, is named and not taken

## Context and Problem Statement

[ADR-0178](0178-a-rate-the-controller-holds.md) gave the controller a second rule and wrote H-38 before any run, predicting yes. [ADR-0180](0180-a-rate-the-controller-holds-built.md) built the rule. Brief 072 runs H-38 once, and asks that its clauses, its constants, the lead-in's bound and criteria and the four predicted readings be committed before any lead-in is run under the rate's rule.

**This ADR is written in two parts.** The protocol below was committed first (`5caf9a2`, pushed 2026-10-13T09:51:46Z). The readings were added after the run of each step and each arm, and nothing of the protocol moved then.

**No sweep is owed.** ADR-0174 records the last that ended, and brief 071's dispatch ran the whole-domain shards alone.

What was read first (principle 2), on 2026-10-13:

1. **H-37's harness** (`runtime/cortex-runtime/tests/inhibition.rs`). `derive_under(units, seed, settling, name)` runs H-25's rules in ADR-0165's order under a `Settling`, which carried the controller's step, whether the settled network's rule reads each window's gain, and the bound. Its candidate's engine is built by `candidate_from` with the step in its configuration; the image `at_gain` reloads it from carries what the configuration held.
2. **No arm has run under a step.** ADR-0177's five steps stopped at the settled network's rule or at the frozen block. `arm_on` runs an arm on the instrument's network or on a `Site`, and its calibration holds a site's derivation to tables an earlier round pinned whole. It asserts the gain at the load and reads it nowhere after.
3. **A trial and a window.** A trial is 16 384 ticks and a window 131 072, so a block of sixty-four trials is eight windows. An image is written a quiet run after its lead-in's last window, so an arm's trials do not begin at a window's first tick: a window ends inside one trial in eight. *Not read before the protocol was committed, and found by the arms' run:* the run steps 500 ticks under the drive between the load and an arm's first trial (`LEAD_IN`, `run_on_scheduled_over`).
4. **The record's count of its open window** is its closed bins' spikes and its open bin's (`HomeostaticDrivePool::window_spikes`, `bin_activity`), which an image carries.
5. **The gain held's lead-in against H-38's target**, by the rule below, before any lead-in under the rate's rule: ninety-six windows of 2 132 to 2 462 spikes, thirty-two of them above 2 306, and 35 897 spikes over the last sixteen, 2 244 a window, 2.7 per cent below the target. The target is the settled image's rate frozen; the lead-in's last windows, its weights still moving, fire a little below it.

## Decision Drivers

- ADR-0178's protocol and brief 072's standing directives: one network, one target, six steps in their order, a bound of 320 windows; no rule that derives a constant changed so that a step holds; no constant moved after a lead-in is run under the rate's rule, and no second attempt.
- **The rule that reads a gain as settled must not read a swing as drift**, which is why ADR-0178 holds the gain to its mean; and it must be an integer rule with its edges written.
- **A run under the rule is held to the rule window by window**, in the lead-ins and in the arms, by an oracle that is not the record's code.
- **Every derivation and every arm of the tree is the one it was**: the harness that stands is given the target, and the calibration runs before any lead-in under the rule.
- A heavy run is a weekly test; the gate grows by one test for the measurement ([ADR-0061](0061-the-learning-runs-leave-the-gate.md)).

## Considered Options

1. **How the derivation takes the rule**: (a) the `Settling` carries the target and whether the rule reads the gain's mean, and `derive_under` builds its candidate under it; (b) a second derivation for the rate's rule.
2. **The gain's mean where the lead-in is shorter than two blocks of sixteen**: (a) both blocks whole windows of the lead-in, so no window below the thirty-second holds the criterion; (b) the prior's gain standing for the windows before the first.
3. **The arms' ground**: (a) a third ground of `arm_on`, the instrument's prior derived under the rate's rule, its calibration the step's own pinned derivation; (b) a `Site` given a settling.
4. **What an arm is read beside**: (a) H-25's two arms with the gain held, the tables the tree pins; (b) nothing.
5. **How the gain is read through an arm**: (a) at every trial's end, with each window's spikes from the train and the window's regulation held to the rule; (b) at each block's end.
6. **Whether every arm of the tree reads it**: (a) every arm run by `arm_on`, on every ground; (b) H-38's alone.

## Decision Outcome

**Options 1(a), 2(a), 3(a), 4(a), 5(a) and 6(a).**

### The protocol (written before any lead-in under the rate's rule)

- **The network**: ADR-0044's prior at 1 024 units from seed 22 (`SCAN_UNITS`, `PRIOR_SEED`).
- **The target**: 58 201 ticks (`RATE_TARGET_PERIOD`, which is `TARGET_PERIOD_1024`), 2 306 spikes a window at 1 024 units by ADR-0180's rule (`RATE_TARGET_COUNT`; the gate holds an engine's own count to it).
- **The steps**, in the order run: an eighth, a sixteenth, one in 32, one in 64, one in 128 and one in 256 (`RATE_STEPS`).
- **The settling** (`under_rate(step)`): the controller's step and the target in the candidate's configuration and so in every image written from it; the rate's rule on from the lead-in's first window; the bound 320 windows (`RATE_BOUND`).
- **The settled network's rule under the rate's rule** (`settled_in_mean`): the first window $w$, counted from one, at which both hold:
  - ADR-0055's criterion over the excitatory sums (`settled_in`), as it stands;
  - the gain's mean's criterion (`mean_settled_in`): with $A$ the gains after windows $w - 15$ to $w$ summed and $B$ the gains after windows $w - 31$ to $w - 16$ summed, $10\,000\,|A - B| < 75\,B$. The two means are over one count of windows, so their sums stand for them; the comparison is strict, as ADR-0055's is; a gain is the gain its window's regulation left.
  - **Both blocks are whole windows of the lead-in** (option 2(a)), so the rule gives no window below the thirty-second. ADR-0178 names *"the sixteen before"*; before the sixteenth window there are not sixteen, and a prior's gain standing for windows that did not run (option 2(b)) would hold a mean over one number sixteen times.
  - It returns nothing where no window of the 320 holds both.
- **Then H-25's rules**, as `derive_under` runs them: the quiet run, the cancel, the frozen block's two measures, the inhibitory rule's target period, the critic's window and constants. A rule that returns nothing stops the derivation there, and every rule after it returns nothing.
- **A step holds** when every rule gives a value and the frozen block passes both measures (`Rules::hold`): every stimulus unit firing once by ADR-0074's measure, and the stimulus seen in 56 trials of 64. **The step taken is the largest that holds**, the first in the scan's order (`ruled`).
- **The arms**: H-25's two on H-20's schedule from the taken step's image, under the true reward, the step and the target carried through every trial (`ruled_arm`).
- **H-38's clauses** are ADR-0178's, by the rules the tree holds: clause 0, `Rules::hold` at one step at least; clauses 1 to 4, `drawn_of` against the image the taken step's derivation wrote — at least 80 of each mapping's last 128 trials correct; no coupling past 1.30 of its image's; each reversal passing 40 of 64 within 23 blocks of its flip; the excitatory sum outside the four couplings within 0.75 and 1.25 of the image's. **Yes** when all five hold in both arms.
- **The stopping rule's steps** (`ruled_step`): 3, yes; 5, no step holds; otherwise 4.
- **The prediction**: yes (`RULED_PREDICTED`).
- **The four predicted readings as rules** (`rate_predicted`), each predicted to hold (`RATE_READINGS_PREDICTED`):
  1. the step of a sixteenth holds: `Rules::hold` at the scan's second step;
  2. no window of any of the six lead-ins holds 32 768 spikes or more, the saturation rule's count at 1 024 units;
  3. through each arm the gain after every window that ends inside it is within five per cent of 1.75: twenty times its distance from 114 688 at most 114 688, which is 108 954 to 120 422, both counted (`gain_within`);
  4. each arm's gains after those windows sum below as many times the gain its image carries, strictly (`mean_below`).

  The third and the fourth are read where arms run and are not read where none does.

### The harness (built before any lead-in under the rate's rule)

- **The settling carries the rule** (option 1(a)). `Settling` gains the target and whether the rule reads the gain's mean; `GAIN_HELD`, `AT_AN_EIGHTH` and `under_step` carry no target and read no mean. `derive_under` builds its candidate by `candidate_under`, which is `candidate_from` with the target in the configuration, and holds the image to carrying it. With the target unset the configuration is the one it was, field for field, so every derivation of the tree is the derivation it was. A second derivation (option 1(b)) would be H-25's rules written twice.
- **The rate's rule as an oracle** (`rate_regulated`): ADR-0180's rule in `i128`, each rounding to nearest by doubling and not by the record's shifts. The gate holds it to numbers written by hand and to `HomeostaticDrivePool::regulate_rate` over 400 windows fed bin by bin.
- **A lead-in's reading** (`RateRead`, `rate_read`): its length; the window each criterion first holds at, and both; the gain's band over the lead-in and over its last sixteen windows; the least and the greatest count, and the last sixteen windows' count; the windows above the target; the windows at the saturation rule's count; the windows whose gain is not the oracle's from the gain before, the window's count, the target and the step; the last window; the sums; and the steepness.
- **The steepness** (`steepness`): each window's count against the gain it ran under, by the least-squares slope at the two means, as the per cent of rate a per cent of gain moves. It is the table's slope: the lead-in's weights move under it, and the gain answers the count of the window before.
- **Each step a weekly test** (`rate_step`): the derivation dumped whole, then held — what it makes before the settled network and its cancel to seed 22's constants; its first window to the gain held's in everything but the gain; every window's gain to the oracle; the train's capacity; and the tables pinned from its one run (`RATE_*`).
- **The arms' ground** (option 3(a), `Ground::Ruled`, `RuledSite`): the instrument's prior under the rate's rule at the taken step. Its calibration is that step's derivation, run again and held to the step's pinned tables as the step's own test holds it (`ruled_made`); a step that does not hold has no arm. A `Site` given a settling (option 3(b)) would need a pinned derivation in a `Site`'s shape for a network whose derivation before the settled network is seed 22's own, and would run `own_first_block`, which holds an arm under another reward to the network's arm under the true one: here the arm is that arm.
- **Read beside H-25's** (option 4(a), `ruled_stand`): the tables `arm_on` reads an arm beside are, on this ground, H-25's two with the gain held. So each reversal's block, the value, the troughs, the signal, the strong punishments, the couplings' and the inhibitory sum's courses, and the first block that is not H-25's are dumped beside H-25's by the lines the tree already has.
- **The controller beside every arm** (options 5(a) and 6(a), `GainWatch`): at every trial's end the gain, and each window's spikes from the train, the window open at the load beginning with what the image's record held of it. As committed it counted the spikes of a trial's ticks; since the arms' run it counts every spike since the load (see the readings). Then, on every ground:
  - the train's count of the open window is the record's own at every trial's end, 7 680 times an arm;
  - 960 windows end inside an arm, eight in every block;
  - every window's gain is the rule's from the gain before it: the oracle's from the window's count where the engine carries a target, and the gain before where it carries none;
  - with no target the step is zero and the gain is the image's throughout.

  So every arm the calibration runs through `arm_on` shows the gain held at every window, and shows the count H-38's arms are read by to be the engine's own.
- **An arm's reading of the controller** (`GainRead`, `gain_blocks`): the band; the first window outside predicted reading 3's band; the gains summed beside as many times the image's; the counts beside as many targets; the windows above the target; the windows not the rule's; per reversal the gain at the flip and the lowest and the highest over its twenty-three blocks; and by block the lowest, the highest and the last gain and the block's count. Pinned for H-38's arms.
- **The gate** (one test, `the_settled_network_s_rule_under_the_rate_s_rule_clause_0_of_h_38_and_the_predicted_readings_rules`): the constants; the oracle; the gain's mean's criterion at its edges — below the thirty-second window, exactly 0.75 per cent either way and one part inside, against the reference's own sum, a swing, a rise that stops — and the settled network's rule waiting for the sums; clause 0, the verdict and the stopping rule's steps over tables written by hand; the four readings' rules at their edges; the lead-in's reading, the steepness, the watch, the gain by block and the arm's reading over tables written by hand. It runs no lead-in.
- **Two tables read before the run**, functions of lead-ins the tree already pins:
  - the settled network's rule under the rate's rule over them (`RATE_BESIDE_SETTLED`): the gain held's at its ninety-sixth window, where its sums settle; ADR-0077's at an eighth at its thirty-fifth; H-37's at a sixteenth, one in 32 and one in 64 at their forty-fifth, forty-fourth and fifty-sixth; H-37's at one in 128 and one in 256 at none of the 71 and 114 windows they ran. So the mean reads as settled the three lead-ins ADR-0177's rule read as never settling, which is what it was written to do; their frozen blocks were not run.
  - the gain held's lead-in by the rate's reading (`HELD_RATE_READ`), item 5 of the context.

### The readings

Everything below was read on the developer machine on 2026-10-13, from one run of each step and, for the reason the last paragraph of this section gives, two of each arm, after the protocol above was committed and pushed (`5caf9a2`, 09:51:46Z). A developer machine's seconds are not Measured figures ([ADR-0010](0010-measured-or-target.md)); the tables are integers of a deterministic engine, which the dispatched weekly reproduces.

**The calibration** (H-38's stopping rule, step 2). The tree's 120 whole-domain tests but this round's eight, each in a process of its own from the release build at `5caf9a2`, eight at a time, from 10:01:23Z to 13:42:33Z: 120 of 120 passed, with the target unset and the image at format 22. So every pinned number of the tree holds, H-37's five steps and ADR-0077's two candidates among them. Within it:
- the five whole-image pins ADR-0180 moved by its mask that a weekly test reads are read by those tests; the sixth, the frozen image H-20's arm leaves, was read by its paused test, `the_subset_on_the_image_h_20_leaves_paused`, run to its reading and stopped there: `DRAINED_050` field for field, the CRC `0x82ec8ea169da899d`;
- the sixteen arms `arm_on` runs each read their gain held in all 960 windows, and the train's count of the open window the record's own at every trial's end.

**The scan** (13:42:59Z to 13:47:08Z, the six steps side by side, 234 to 245 s each). **Clause 0 holds at every step.**

| Step | Settled at window | Gain's band over the lead-in | Spikes a window | Last sixteen windows, a window | Windows above the target | Carried gain | Frozen block: seen of 64 | Gain after it | Inhibitory target period |
| :--- | ---: | :--- | :--- | ---: | ---: | :--- | ---: | ---: | ---: |
| An eighth | 102 | 1.6854 to 1.7891 | 1 228 to 3 375 | 2 268 | 51 of 102 | 1.7557 | 64 | 1.7283 | 58 955 |
| A sixteenth | 96 | 1.7419 to 1.7599 | 2 067 to 2 507 | 2 303 | 46 of 96 | 1.7505 | 62 | 1.7259 | 58 188 |
| One in 32 | 100 | 1.7461 to 1.7571 | 2 101 to 2 492 | 2 310 | 50 of 100 | 1.7511 | 64 | 1.7271 | 58 419 |
| One in 64 | 102 | 1.7479 to 1.7545 | 2 124 to 2 459 | 2 315 | 52 of 102 | 1.7507 | 64 | 1.7283 | 58 172 |
| One in 128 | 100 | 1.7485 to 1.7538 | 2 141 to 2 464 | 2 321 | 54 of 100 | 1.7511 | 63 | 1.7355 | 58 457 |
| One in 256 | 101 | 1.7491 to 1.7529 | 2 146 to 2 447 | 2 329 | 52 of 101 | 1.7515 | 63 | 1.7416 | 57 852 |
| The gain held | 96 | 1.7500 | 2 132 to 2 462 | 2 244 | 32 of 96 | 1.7500 | 62 | 1.7500 | 58 201 |

- **Every lead-in settles, at the window its sums do.** The gain's mean's criterion holds from the thirty-second window at every step; the sums' holds at windows 96 to 102, where it holds with the gain held at 96.
- **Every later rule gives a value**, and the cancel is ADR-0076's at every step.
- **The frozen block passes both measures at every step**: every stimulus unit fires once, and the stimulus is seen in 62 to 64 trials of 64 where 56 are asked.
- **No window of the 601 holds the saturation rule's count.** The greatest holds 3 375 spikes, at an eighth; 32 768 would be one spike a unit a bin.
- **In all 601 windows the gain is the oracle's** from the gain before, the window's spikes, the target and the step.
- **The step taken is an eighth**, the largest that holds.

**What each lead-in reads of the controller.**
- **The gain's band narrows with the step and stays about 1.75**: 5.9 per cent wide at an eighth, 1.0 at a sixteenth, 0.63, 0.38, 0.30 and 0.22 below. Its mean over sixteen windows is 1.7464 to 1.7532 at every step.
- **The count is held to its target.** Over each lead-in's last sixteen windows the count stands 1.6 per cent below the target to 1.0 above it; with the gain held it stands 2.7 per cent below. At a sixteenth and below the count's range is close to the gain held's own, 2 067 to 2 507 at a sixteenth against 2 132 to 2 462.
- **At an eighth the gain alternates.** The first six windows hold 2 446, 2 064, 2 524, 2 120, 2 532 and 1 959 spikes, the gain going 1.7367, 1.7595, 1.7387, 1.7562, 1.7347, 1.7674. Over the 102 windows the count ranges from 1 228 to 3 375, half the target to half again.
- **The steepness is fifteen.** At the first step the two largest steps read it apart from the scatter. The gain held's second window holds 2 329 spikes. At an eighth the gain leaves the first window 0.76 per cent lower and the second holds 2 064, 11.4 per cent fewer. At a sixteenth it leaves 0.38 per cent lower and the second holds 2 196, 5.7 per cent fewer. Each is 15.0 per cent of rate for one per cent of gain. By the tables' slope it is 15.7 at an eighth and 16.7 at a sixteenth; at the smaller steps, whose gain moves less than a window's count scatters, the slope reads 12.0, 8.9, 6.7 and 5.8, and the first step reads no one number: 5.7, 2.3, 7.3 and −19.3.
  - ADR-0178 took nine to fourteen from H-37's rising lead-ins. Fifteen is past it and short of the sixteen at which, by its arithmetic, an eighth does not converge: a deviation stands at $1 - 15/8$, $-0.88$ of itself a window later. So at an eighth the loop is at its edge, and at a sixteenth a deviation stands at 0.06 of itself.
- **With the weights frozen, an eighth's alternation grows over the eight windows read.** The target's lead-in, the settled image frozen under the drive, holds 2 488, 2 089, 2 571, 1 996, 2 642, 1 878, 2 964 and 1 585 spikes at an eighth: 182 above the target, then 217 below, to 658 above and 721 below. At a sixteenth it holds 2 207 to 2 381, and with the gain held 2 238 to 2 367. In the lead-in and in the arms, where the weights move, the alternation stays bounded. Eight windows do not say where a frozen network at an eighth ends; the rule's clamp bounds a move at an eighth of the gain.
- **The estimate, kept as a reading, reads as it read with the gain held**: zero in 241 windows of the 601, 0.479 at most, near one in none. No window stores the ceiling, the saturation rule not being consulted.
- **The readouts fire less in the frozen block**, 1 038 to 1 251 spikes after the volley where the gain held's block holds 1 282, because the task's spikes are in the count and the gain falls through the block: by 1.6 per cent at an eighth and 0.56 at one in 256.
- **The sums** end at 0.757 to 0.772 of the prior's inhibitory and 0.915 to 0.924 of its excitatory, beside 0.775 and 0.925 with the gain held.

**The arms**, H-25's two from the image the eighth's derivation wrote, the step and the target carried through every trial. Each arm first ran that derivation again and held it to the tables the scan pinned.

| | From the assignment | H-25's, the gain held | From the mirrored | H-25's, the gain held |
| :--- | :--- | :--- | :--- | :--- |
| Correct of each mapping's last 128 | 122, 123, 118, 124 | 122, 124, 119, 116 | 118, 118, 124, 119 | 124, 118, 123, 117 |
| First mapping passes 40 of 64 at block | 6 | 6 | 4 | 4 |
| Reversals, blocks from the flip | 18, 19, 16 | 19, 21, 16 | 15, 15, 15 | 20, 15, 14 |
| Highest coupling, of the image's | 1.191 | 1.181 | 1.220 | 1.191 |
| Outside sum, of the image's | 0.9999 to 1.0001 | | 0.9999 to 1.0000 | |
| Blocks whose stimulus fired once | 120 of 120 | | 120 of 120 | |
| Inhibitory sum at the end, of its image's | 0.782 | 0.848 | 0.783 | 0.850 |
| Spikes a window, mean | 2 306.1 | | 2 306.0 | |

**Through each arm the controller reads:**
- **The count is the target's.** Over 960 windows the arms hold 2 213 833 and 2 213 746 spikes where 960 targets are 2 213 760: 2 306.1 and 2 306.0 a window. With the gain held, the arms the calibration ran on this network, H-28's and H-35's, hold 2 814 to 2 838 a window: the task adds 22 to 23 per cent to the settled network's 2 306, where ADR-0178 reckoned a fifth. Here the gain gives it back.
- **The gain stands two per cent below the image's.** Its mean is 1.7215 and 1.7210, 1.95 and 1.98 per cent below the 1.7557 the image carries, and 1.6 and 1.7 per cent below 1.75. Its band is 1.6993 to 1.7401 and 1.6985 to 1.7407: 0.5 to 2.9 per cent below 1.75 after every window.
- **It alternates.** Consecutive windows stand on opposite sides of the target in 750 and 758 pairs of 959. A window holds 1 880 to 2 882 spikes, with a standard deviation of 111 to 143 by mapping. The window the load opens in is the greatest, and the swing it starts shrinks over the windows after it: 1 881, 2 610, 2 082 and 2 444 in the first arm.
- **A reversal does not move it.** The gain at each flip is 1.7365, 1.7267 and 1.7194 in the first arm and 1.7339, 1.7289 and 1.7188 in the second; over the twenty-three blocks after each it stays within 1.6985 and 1.7391. The sixteen windows after a flip hold a mean gain within 0.06 per cent of the sixteen before.
- **Every window's gain is the oracle's**, and the train's count of the open window is the record's own at every trial's end.

**The inhibitory sum falls further under the rule.** It ends at 0.782 and 0.783 of its image's, falling in every block from the second; with the gain held it rises to 1.01 of its own image's over its first eleven to fourteen blocks and ends at 0.848 and 0.850. The two rules meet here: the gain holds the population's count at the settled rate while the task has its own units fire more, so the count the rest fire is below the rate the inhibitory rule drives each unit toward. That is an account this round did not vary; what was read is the sum. Clause 4 reads the excitatory sum outside the four couplings, which stays at the image's.

### H-38's verdict

**Yes.** By `ruled` over the six pinned derivations and the two arms' pinned blocks against the image the eighth's derivation wrote:
- **clause 0**: a step holds — all six do, and the step taken is an eighth;
- **clause 1**: every mapping learned, 118 to 124 of each mapping's last 128 where 80 are asked;
- **clause 2**: no coupling past 1.30 of its image's, the highest 1.220;
- **clause 3**: each reversal passes 40 of 64 within 23 blocks of its flip, in 15 to 19;
- **clause 4**: the outside sum within 0.75 and 1.25 of the image's, at 0.9999 to 1.0001.

**The prediction was yes, and it held.** The four predicted readings:

| | Predicted | Read |
| :--- | :--- | :--- |
| 1. A step of a sixteenth holds | holds | **held**: it settles at window 96, the stimulus seen in 62 trials of 64 |
| 2. No window of a lead-in at the saturation rule's count | none | **held**: the greatest of 601 holds 3 375 spikes of 32 768 |
| 3. The gain within five per cent of 1.75 through each arm | 1.6625 to 1.8375 | **held**: 1.6985 to 1.7407 over 1 920 windows |
| 4. Each arm's mean gain below the gain its image carries | below | **held**: 1.7215 and 1.7210 below 1.7557 |

**What ADR-0178 said would refute it, against what was read.**
- *A steepness far above the tables'*: read at fifteen, where ADR-0178 took nine to fourteen. It is above, and it puts an eighth at the edge ADR-0178's arithmetic names, not past it.
- *The gain's scatter moving the volley or the sight*: the volley fires once in every block of the frozen blocks and of both arms, and the sight holds 62 to 64 of 64.
- *The controller's moves through a reversal*: the gain's mean does not move at a flip.

**ADR-0178's arithmetic, against what was read.** A sixteenth converges at every steepness read, as it said. An eighth it called an alternation that converges at fourteen and not at sixteen: at fifteen it alternates and is bounded with the weights moving, and grows over eight windows with them frozen. The task was to ask for a gain two to three per cent lower: it asks for two.

**What the readings do not say.**
- **Nothing of a step but an eighth under the task.** The protocol runs the arms under the largest step that holds, and that is the one step whose loop is at its edge. A sixteenth's lead-in is the quieter by every reading above, and no arm ran under it.
- **Nothing of another network, another size or another target.**
- **Nothing of a longer run**: an arm is 960 windows, and the inhibitory sum is still falling at its end, as it is with the gain held.
- **Nothing of the frozen network at an eighth past eight windows.**

**A defect of this round's harness, found by the arms' run and repaired.** The run steps 500 ticks under the drive before an arm's first trial (`LEAD_IN`, `run_on_scheduled_over`). The engine tallies them; the watch, as the protocol's commit had it, counted a trial's ticks alone. In each of H-38's arms two spikes fell there, so the watch's count of the window the load opens in read two short, 2 880 and 2 874, and its two checks failed after every table was dumped: seven trials whose open window's count was not the record's, and one window whose gain was not the oracle's from the watch's count. In the sixteen arms of the calibration no spike fell there, and both checks held.
- The watch now counts from the load. The two arms were run a second time for that reading, from 14:24:56Z to 14:50:32Z, the first having run from 13:49:57Z to 14:19:09Z: every table of the first run reproduced, and the first window read 2 882 and 2 876 spikes, the one count from which the oracle gives the engine's gain. Both checks hold in both arms.
- So the arms ran twice, where the round's rule is once. The engine's run is one run: no parameter, constant or criterion moved between the two, and the second's tables are the first's but for the watch's count of that window.

### The evidence

- **The dispatch's scope.** The diff changes files under `src/` — `cortex-homeostasis`, `cortex-connectome` and the runtime's executor and loader — so by [ADR-0075](0075-the-dispatch-scope-follows-the-diff.md) the weekly was dispatched on this round's branch at **`scope=both`**, as brief 072 asks: run [38417864748](https://github.com/DescentVTT/VirtualCortex/actions/runs/38417864748) at `384738c`, the pins' commit, at 14:57:33Z. The round's commits, in pull request #208: `0f08b1d` the rule built ([ADR-0180](0180-a-rate-the-controller-holds-built.md)); `5caf9a2` the protocol, before any lead-in under the rate's rule; `384738c` the six steps' and the two arms' tables, the watch counting from the load and the gate's checks over the tables; `af37240` the readings and the documents; and the commit that carries this evidence, which changes no test.
- **Every whole-domain shard is green.** All 128 `exhaustive` tests passed on the hosted runners, each exit 0 with one test reported, 101 602 s summed. The 120 before this round reproduced their pinned numbers at format 22 with the target unset, H-37's five steps and the calibration at an eighth among them, and the sixteen arms `arm_on` runs each held their gain in all 960 windows and their windows' counts to the record. **This round's eight reproduced their tables there**: the steps of an eighth, a sixteenth, one in 32, one in 64, one in 128 and one in 256 in 185, 225, 230, 227, 232 and 279 s, and the two arms in 1 834 and 2 269 s, each after deriving its network again, with the watch's two checks holding in both.
- **The shards**, dealt by the table before this round, which did not know the eight and costed each at 900 s:

  | Shard | Job, of its bound | Its two heaviest tests (s) | Tests | Their seconds summed | Tests' wall time, of the bound |
  | ---: | ---: | :--- | ---: | ---: | ---: |
  | 0 | 64.4 min, 36 % | `the_address_drawn_on_seed_22_from_the_assignment_at_2048_units` 3 782; `the_critic_from_the_assignment_at_1024_units` 1 161 | 10 | 7 450 | 3 782 s, 35 % |
  | 1 | 76.6 min, 43 % | `the_address_drawn_on_seed_23_from_the_assignment_at_2048_units` 3 698; `a_critic_with_a_window_from_the_mirrored_assignment_at_1024_units` 1 803 | 11 | 8 839 | 4 505 s, 42 % |
  | 2 | 55.7 min, 31 % | `the_address_drawn_on_seed_23_from_the_mirrored_assignment_at_2048_units` 2 814; `the_release_alone_from_the_mirrored_assignment_at_1024_units` 1 371 | 10 | 6 433 | 3 270 s, 30 % |
  | 3 | 73.6 min, 41 % | `a_reward_right_three_times_in_four_on_seed_23_from_the_mirrored_assignment_at_1024_units` 1 859; `three_answers_from_the_assignment_at_1024_units` 1 792 | 10 | 8 594 | 4 336 s, 40 % |
  | 4 | 83.0 min, 46 % | `the_address_drawn_on_seed_22_from_the_mirrored_assignment_at_2048_units` 2 939; `a_reward_right_three_times_in_four_on_seed_24_from_the_mirrored_assignment_at_1024_units` 1 500 | 12 | 9 648 | 4 909 s, 45 % |
  | 5 | 74.5 min, 41 % | `an_exploration_the_value_gates_from_the_mirrored_assignment_at_1024_units` 1 822; `a_reward_right_three_times_in_four_from_the_mirrored_assignment_at_1024_units` 1 820 | 10 | 8 035 | 4 396 s, 41 % |
  | 6 | 77.5 min, 43 % | `the_address_drawn_on_seed_23_from_the_mirrored_assignment_at_1024_units` 1 878; `a_target_the_network_fires_at_from_the_assignment_at_1024_units` 1 828 | 11 | 9 137 | 4 574 s, 42 % |
  | 7 | 91.0 min, 51 % | `the_address_drawn_on_seed_23_from_the_assignment_at_1024_units` 1 885; `the_address_drawn_from_the_mirrored_assignment_at_1024_units` 1 815 | 11 | 10 319 | 5 373 s, 50 % |
  | 8 | 87.7 min, 49 % | `a_reward_right_three_times_in_four_on_seed_23_from_the_assignment_at_1024_units` 2 317; `the_address_drawn_under_the_rate_s_rule_from_the_mirrored_at_1024_units` 2 269 | 12 | 10 311 | 5 173 s, 48 % |
  | 9 | 72.1 min, 40 % | `the_address_drawn_on_seed_24_from_the_assignment_at_1024_units` 1 852; `the_address_drawn_under_the_rate_s_rule_from_the_assignment_at_1024_units` 1 834 | 9 | 8 465 | 4 239 s, 39 % |
  | 10 | 42.7 min, 24 % | `a_reward_right_three_times_in_four_on_seed_24_from_the_assignment_at_1024_units` 1 094; `the_gate_s_output_delivered_from_the_assignment_at_1024_units` 1 091 | 11 | 4 612 | 2 508 s, 23 % |
  | 11 | 85.0 min, 47 % | `the_span_a_choice_among_three_is_given_from_the_mirrored_assignment_at_1024_units` 2 394; `the_address_drawn_on_seed_24_from_the_mirrored_assignment_at_1024_units` 1 848 | 11 | 9 759 | 5 018 s, 46 % |

  The percentages are of the job's bound, 180 minutes ([ADR-0179](0179-the-bounds-rule-restated.md)); each shard ran its tests two at a time, at 1.83 to 2.00 of their summed seconds over the wall. The twelve started within six seconds of each other, four seconds after the dispatch, and the last ended at 16:28:36Z, 1 h 31 min after it. **No shard's job passed 60 per cent of its bound**: the most is 91.0 minutes, 51 per cent. That is the slowest shard's job the tree has read, by a tenth of a minute past the 90.9 ADR-0179 took, and its rule gives the bound it has.
- **The cost table is regenerated from this run** (`node scripts/exhaustive-costs.mjs from <artifacts> --run 38417864748`): 128 lines, 101 602 s, where it held 120 and 93 020. The 120 before this round summed to 96 121 s here, a thirtieth more than the last run read them at; this round's eight are 5 481 s of the rest. ADR-0092's deal plans each of the twelve shards at 8 466 to 8 468 s summed, about 4 460 s of wall time at this run's ratios: **41 per cent of the bound**. No shard is planned past 60.
- **The sweep had not ended when the round asked for its merge** ([ADR-0158](0158-the-sweep-off-the-next-decisions-path.md)). Its thirteen jobs, `crates` and `runtime-0` to `runtime-11`, started at 16:28:40Z, four seconds after the last shard ended ([ADR-0166](0166-the-sweep-after-the-shards.md)), and none had ended. The first ADR that merges after it ends records it. No sweep was owed: [ADR-0174](0174-three-in-four-on-the-other-networks-measured.md) records the schedule's last, and no weekly run was scheduled or dispatched with a sweep between it and this one.
- **The pull request's gate** on `5caf9a2` and on `af37240` is green in every job, the determinism pin on AArch64 and the MSRV among them. **The mutation gate on the changed lines** made 43 mutants on each: 39 caught, 4 unviable, none missed. The gate on `384738c` was cancelled by the push of `af37240`, which changes documents and whose tests are its.
- **On the developer machine**, at `384738c`: the workspace's tests passed in the debug profile, in the release profile and on the MSRV, 752 passed and 168 ignored in each; the check, the format, Clippy, the documentation, the benchmarks and `npm run spec` exited 0; and 128 whole-domain tests are listed. `cargo mutants --in-diff` over the round's diff made the same 43 mutants and exited 0: 27 caught and 16 unviable, twelve of the sixteen the Windows linker's lock (`LNK1104` in their logs) on mutants the hosted run caught, and none missed. The six steps were not run a second time there: each ran once, each arm's calibration ran its step's derivation again, and the dispatch is their reproduction. The arms ran twice, as the readings say.

### The step of the stopping rule reached

**Step 3: yes.** The engine learns and revises with its controller on, the controller holding the settled network's rate.

**The next decision, named and not taken**, is ADR-0178's: *"an ADR on what the engine carries by default, the rule and its step, on the other networks and the other size under it, and on the questions left, a reward delayed past a trial among them."* What this round's readings hand it:
- the step taken by the protocol, an eighth, is at the loop's edge on this network: it holds every clause, and a window's count ranges from 1 880 to 2 882 through its arms and from 1 228 to 3 375 in its lead-in, where a sixteenth's lead-in stays within 2 067 and 2 507;
- no arm has run under a sixteenth, so the readings order the two steps' lead-ins and not their learning;
- under the rule the inhibitory sum ends lower, 0.78 of its image's against 0.85;
- the default's target needs a rule for a network no derivation has settled: here it is a period the gain held's derivation returned.

### Consequences

- Good: the criterion that reads the gain as settled is an integer rule with its edges in the gate, and what it reads of seven pinned lead-ins is known before any lead-in under the rule.
- Good: a run under the rule is held to the rule in every window of every lead-in and every arm, by an oracle written from ADR-0180's text.
- Good: every arm the tree runs through `arm_on` now shows its gain held and its windows' counts the engine's own, each week.
- Bad: `arm_on` reads more on every ground, and its dumps grow by the windows of the arm.
- Bad: the steepness is one number for a lead-in whose weights and gain both move.
- Good: the engine has a controller it learns and revises under. Every learning result before this stood with the gain held and the controller off.
- Good: the prediction was written with its arithmetic, and the readings say where the arithmetic was short: the steepness is fifteen, not nine to fourteen.
- Bad: the protocol's step is the largest that holds, and the largest is the one at its loop's edge. A yes at an eighth says less of a sixteenth's learning than an arm under a sixteenth would.
- Bad: the watch the protocol's commit added missed 500 ticks of every arm, and the arms ran twice for it. A check of the watch on a network whose first 500 ticks hold a spike would have found it before the run; the calibration's sixteen arms held none there.
- Neutral: the weekly job grows by eight tests, 5 481 s on the dispatched run: six derivations of three to five minutes and two arms of thirty and thirty-eight. The regenerated deal plans every shard at 41 per cent of its bound.

## Alternatives considered and why rejected

- **A second derivation** (option 1(b)), **the prior's gain for windows that did not run** (option 2(b)), **a `Site` given a settling** (option 3(b)): see above.
- **Read beside nothing** (option 4(b)): ADR-0178's readings ask for each of H-25's readings beside H-25's with the gain held, and the lines exist.
- **The gain at each block's end** (option 5(b)): predicted reading 3 is of the gain after every window, and a block's end is one window in eight.
- **H-38's arms alone** (option 6(b)): the watch's count would then be checked against the record only in the two arms it is needed for, and the gain held would stay asserted at the load alone.

## Confirmation

- `runtime/cortex-runtime/tests/inhibition.rs`: `Settling`, `candidate_under`, `derive_under`; `RATE_STEPS`, `RATE_TARGET_PERIOD`, `RATE_TARGET_COUNT`, `RATE_BOUND`, `under_rate`; `rate_regulated`, `mean_settled_in`, `settled_in_mean`, `steepness`, `RateRead`, `rate_read`; `GainWatch`, `windows_of`, `gain_blocks`, `GainRead`, `gain_read`, `gain_within`, `gains_within`, `mean_below`; `Ruled`, `ruled`, `ruled_step`, `rate_predicted`, `RULED_PREDICTED`, `RATE_READINGS_PREDICTED`; `Ground::Ruled`, `RuledSite`, `ruled_stand`, `ruled_made`, `ruled_arm`; `rate_step`; the six tests `the_rate_s_rule_at_a_step_of_…_at_1024_units_exhaustive` and the two `the_address_drawn_under_the_rate_s_rule_from_the_…_at_1024_units_exhaustive`; the gate's test and `rate_tables`; `RATE_BESIDE_SETTLED`, `HELD_RATE_READ`; the pinned tables `RATE_*` and `RULED_*`; `RULED_1024`, `RULED_STEP_1024`, `RATE_READINGS_READ_1024`.
- Whitepaper §1.6, §8.8, §11.1 (H-38 with its verdict and step), §9; `CHANGELOG.md`; `CLAUDE.md`; `README.md`; `docs/zh-TW/README.md`.
