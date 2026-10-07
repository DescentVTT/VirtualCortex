---
status: accepted
date: 2026-10-07
depends-on: ADR-0148
decision-makers: VirtualCortex maintainers
---

# ADR-0149: A reward right seven times in eight, measured — H-28 is yes: on H-25's configuration and schedule with the reward's sign the outcome's in seven trials of eight and its opposite in one, the coin misleading in 949 of the run's 7 680 trials, every mapping was learned in both arms, 113 to 123 of each mapping's last 128 from the assignment and 107 to 124 from the mirrored assignment, no coupling passed 1.30, the highest 1.181 and 1.187, the network outside the couplings stayed within two parts in ten thousand of the image's, and each stimulus's mean value stood within 0.13 of the reward of three quarters of 2p − 1 and nearer it than 2p − 1 in all sixteen; the reversals passed 40 of 64 in 14 to 24 blocks, 0.90 to 1.50 of H-25's and five of six under the naive four thirds, and the learning signal was 0.87 to 1.03 of H-25's and not three quarters of it, because a true reward delivered an error of 0.30 to 0.52 of the reward where H-25's delivered 0.08 to 0.18, while a misleading punishment, its signal below the gate's floor in two of three, took from the answer's pair 1.9 times what a true reward gave, 27 per cent of it in all; H-28's stopping rule at step 3, and the next decision named and not taken

## Context and Problem Statement

[ADR-0147](0147-a-reward-right-seven-times-in-eight.md) asked the named learning configuration the first task it had not been asked, a reward that is right seven times in eight, and wrote **H-28** before any run. [ADR-0148](0148-a-reward-right-seven-times-in-eight-built.md) built the feedback: `Feedback::SevenInEight`, its coin three bits of the trial's own draw. Brief 061 runs H-28 once. This ADR is the round's protocol, committed before any rewarded run, and then its readings: **H-28 is yes**, at step 3 of its stopping rule.

What was read before anything was written (principle 2, and brief 061's directive that the engine is read before a description of it is trusted, the brief's and ADR-0147's included):

1. **H-25's arm and its harness** ([ADR-0140](0140-the-address-drawn-measured.md), `tests/inhibition.rs`, `drawn_arm`; `tests/instrument/harness.rs`, `earned_run_held`): H-23's image by its CRC, the engine's critic with its window, the drawn delivery, H-20's flips, 7 680 trials; the drawn sources held to the critic's oracle's counts and both oracles to the record at every trial. Since [ADR-0142](0142-the-readouts-own-competition-measured.md) the arm carries H-26's shadows beside it, which read the run and write nothing.
2. **What a trial's reading holds** (`EarnedTrial`): the stimulus, the counts, the selection, whether it was correct, the reward delivered, what the trial consolidated, and the signal before and after the reward. Under the engine's critic the reward delivered is **the error the modulator received**, the reward less the engine's value ([ADR-0131](0131-the-critic-built.md)). The reward the task gave is that error plus the value, which the run returns per trial beside the readings.
3. **The tables that read a delivery's sign** read the error's: `earned_blocks`' positive deliveries, `moves_blocks`' split, `value_blocks`' errors on correct trials. Under a true reward the error's sign is the outcome's while the value stays within the reward. Under this feedback a correct trial's error is below zero wherever its reward misled.
4. **The moves** (`Composer::moves`, [ADR-0096](0096-the-punished-pair-measured.md)): what a trial's consolidation did to the pair the trial before addressed — the presented stimulus onto the selected readout, none at a tie. After a correct selection that is the answer's pair; after a wrong one, the stimulus onto the readout that is not its answer.
5. **The signed gate** ([ADR-0094](0094-the-signed-gate-built.md)): an addressed synapse consolidates under the dopamine signal clamped to $[-1, 1]$, the baseline being zero. The signal a reward leaves is what the next trial's volley, a few ticks on, consolidates under.
6. **The schedule** (`SCHEDULE_FLIPS`, `SPANS`, `MAPPINGS`): flips before the trials of index 1 536, 3 584 and 5 632; four mappings of 24, 32, 32 and 32 blocks.

## Decision Drivers

- Brief 061's standing directives: one change from H-25, the reward's truth; H-28's clauses and constants committed before the first rewarded run and not moved after it; no second attempt; no float; every loop ends by construction; the gate grows by at most one test for the measurement; no shard of the weekly job past 60 per cent of its bound; no pinned number of an earlier round moves.
- **The reward must be read back whole.** A reading that calls an error's sign a reward's would be wrong on every trial whose value is beyond the reward.
- The readings must say what ADR-0147 left unpredicted: the clamp's part, against the naive scaling.
- Latest ≠ Newest: no dependency, no tool, no rule.

## Considered Options

1. **How a trial's reward is read**: (a) each trial's kind from the selection and the harness's own coin, with the reward the task gave read back as the error plus the value and held to that kind at every trial; (b) the sign of the delivery, as the earlier tables read it; (c) a field added to the trial's outcome.
2. **The earlier tables under the new feedback**: (a) kept by their rules, as H-25's, with brief 061's tables by kind beside them; (b) rewritten to read the kind.
3. **How the arms are built**: (a) H-25's arm through `fed_run`, with no shadow beside it; (b) with H-26's shadows.
4. **How the clamp's part is read**: (a) per kind, the errors delivered, the trials whose reward left the signal beyond the gate's bounds with the excess, and the addressed pair's moves in the trial after; (b) a second run with the gate's clamp widened.
5. **What the arms pin**: (a) H-25's tables whole and brief 061's beside them; (b) the clauses' inputs alone.

## Decision Outcome

**Options 1(a), 2(a), 3(a), 4(a) and 5(a).** Everything from here to the weekly tests and their cost was committed before any rewarded run; the order of the work, the readings and the step of the stopping rule follow it.

### The coin over the schedule (H-28's stopping rule, step 2)

- **The count** (`coin_of`, `misleading_by_mapping`): the coin is misleading in **181, 262, 250 and 256** trials of the four mappings' 1 536, 2 048, 2 048 and 2 048, **949 of 7 680**, 12.36 per cent. Both arms draw from the one seed, 27, so the count is both arms'.
- **Its bounds** (`MISLEADING_BOUNDS`, `coin_within`): 880 to 1 040, both counted. **949 is within them.**
- **Where the numbers come from**: an oracle written apart from the tree, SplitMix64's finaliser in another language with bits 48 to 50 read by a shift and a mask. It reproduces the tree's two earlier pins at seed 27 first. The tests hold the harness's coin (`misleads`) to those counts and to the task's at every trial of the schedule.
- **By stimulus**: A is misled in 99, 137, 127 and 133 of its 772, 1 005, 1 006 and 1 022 trials, and B in 82, 125, 123 and 123 of its 764, 1 043, 1 042 and 1 026. Over each mapping's last 128 trials the coin misleads 13, 17, 13 and 20 times.
- The bits were fixed before the count was computed (ADR-0148).

### The arms (`misled_arm`)

- **Two arms** (`MISLED_ARMS`), each a weekly test of its own: H-25's two, from the assignment and from the mirrored assignment. Each is H-25's arm with the feedback changed and nothing else: H-23's image, asserted by its CRC; `Delivery::Drawn`; H-20's flips; 7 680 trials; the engine's critic with its window; the task carrying no critic.
- **Through `fed_run`** (option 3(a)): `held_run` under a feedback, with no hold and no shadow. H-26's shadows read a release this round does not ask about, and cost a second replay of every pair synapse.
- **Both oracles at every trial**: the composer's 3 188 pair synapses and the network's 26 240 excitatory synapses, each held to the record's traces and weights. The drawn sources are held to the critic's oracle's counts, the targets to the selected readout's units, and the engine's value, error and weights to the harness's critic.
- **Every trial's reward** is held twice: inside the harness, the reward the task delivered against `correct != misleads(…)`; and in the arm, the error plus the value against the same rule, read back from the run's tables.

### H-28, restated as integer rules (ADR-0147's clauses and constants)

- **Clauses 1 and 2** are H-20's, `scheduled`: each mapping's last 128 trials at least 80 correct, and no coupling above 1.30 of its image's at any block's end. A trial is correct by its selection.
- **Clause 3** is H-24's band, `left_band`: the excitatory sum outside the four couplings within three quarters and five quarters of the image's at every block's end, over 120 blocks.
- **Clause 4** (`holds_expected_reward`): for a stimulus's $n$ trials over a mapping's last 128, $c$ of them correct, their values summing to $V$, with the reward $r$:

  $$|4V - 3\,(2c - n)\,r| \le r\,n$$

  and at least one trial. It is $|V/n - \tfrac{3}{4}(2p - 1)\,r| \le r/4$ with $p = c/n$, cleared of its divisions. `RELIABILITY` is the three quarters, $2q - 1$ at $q = 7/8$.
- **The verdict** (`Misled`, `misled`): yes when all four hold in both arms.
- **The step** (`misled_step`): 3 at a yes; 4 at a no in which clause 3 fails in an arm, whatever else failed beside it; otherwise 5 at a no on clause 1 or 2, whatever clause 4 read; otherwise 6, a no on clause 4 alone.
- `MISLED_PREDICTED` is yes, ADR-0147's. It is dumped beside the reading and never asserted.

### The readings, no clause (ADR-0147's)

Per block, pinned; per mapping, computed from them (`MisledRead`):

- **A trial's kind** (`reward_kind`), by its outcome and the harness's coin: a true reward, a true punishment, a misleading reward, a misleading punishment.
- **The trials by outcome and by the reward received** (`Outcomes`): the coin not misleading and misleading, by correct, wrong and tied; over each mapping and over its last 128 trials.
- **What a reward of each kind moved** (`KindMoves`, option 4(a)): the addressed pair's moves in the trial after, by the kind of the trial before. After a misleading punishment that is the answer's pair; after a misleading reward, the pair the wrong selection reached.
- **The errors delivered by kind** (`KindErrors`), and **the clamp's part** (`Beyond`): by kind, the trials whose reward left the signal above 1.0 or below −1.0, and the excess in all.
- **The value beside what it could hold** (`beside_expected`, `nearer_expected`): each stimulus's summed value over a mapping's last 128 trials beside three quarters of $(2p - 1)$ of the reward and beside $(2p - 1)$ of it, and which it is nearer; the rewards each stimulus received there (`rewards_received`); the value's troughs after each flip.
- **The reversal speeds** (`speeds`): each reversal's blocks to its crossing, H-25's, and three times the first less four times the second, above zero where the reversal took longer than four thirds of H-25's.
- **Where the consolidation went** (`Went`, the network's oracle) and the learning signal per mapping; **the couplings' separation** at each mapping's end; **the inhibitory sum's** lowest and last; each beside H-25's.
- **H-25's tables whole**, by their rules (options 2(a) and 5(a)), so that every table of H-25 has its counterpart here block for block. Where one reads a delivery's sign, it reads the error's (item 3 above), and the reading says so.

### The calibration, before any rewarded run (H-28's stopping rule, step 2)

- **In the tree**: the workspace's tests in the debug and the release profile and on the MSRV; every whole-domain test of the weekly job before this round, each in a process of its own from the release build: every pinned number of the tree.
- **In each arm's test**, before its first rewarded trial: the coin's count held to its pin and its bounds, and the harness's coin to the task's at every trial; ADR-0077's settled engine and H-20's to H-23's images by their CRCs; a frozen block held to ADR-0077's frozen run; **H-25's first block from H-23's image under the true feedback** (`drawn_first_block`), table by table, the network's oracle beside it, through the same `fed_run` the arm then runs.

A failure at any of them stops the round there as a finding.

### The gate

`the_clauses_of_h_28_the_coin_s_count_and_the_readings_rules`, one test:
- the arms, the feedback, the delivery, the prediction and the constants;
- the coin's count per mapping and in all, its bounds at their edges, the harness's coin against the task's over the schedule and against the apart oracle's first sixty-four trials;
- clause 4 at its edges, at a full accuracy, at none, at a half and at fifty-six of sixty-four; and what it does not tell apart;
- the verdict over tables written by hand, and the step it reaches: a yes; clause 1 at 79 and at 80; clause 1 beside clause 4; clause 2; clause 4 alone; the band at its edge and past it, alone and beside clauses 1 and 4; a run not whole;
- the readings' rules over tables written by hand: the kinds, the outcomes, the moves by kind, the rewards received, the speeds, the separation, the inhibitory sum, the errors by kind and the clamp's part;
- eight trials on the instrument's network under the drawn delivery with the reward right seven times in eight, and the same eight under the answer's feedback: every reward held to the coin, both oracles to the record, the two runs one until the first misleading trial and that trial's reward the opposite.

The build's tests are ADR-0148's, in `task.rs`.

### The weekly tests and their cost

- **Two weekly tests**: `a_reward_right_seven_times_in_eight_from_the_{assignment, mirrored_assignment}_at_1024_units_exhaustive`.
- **Each is H-25's arm** with the calibration's first block and without the shadows. H-25's arms took 1 796 and 1 081 s in ADR-0145's dispatch. The table after ADR-0145 is 85 tests and 39 759 s, about 47 per cent of the bound at six shards; two more arms at that cost plan about 50 per cent, under the directive's 60.
- **The dispatch's scope**: ADR-0148 changed files under `src/`, so by [ADR-0075](0075-the-dispatch-scope-follows-the-diff.md) the weekly is dispatched at `scope=both`.

### The order of the work, as the history holds it

1. **The bits before the count.** The coin's three bits were fixed and written into the task (ADR-0148). Then the oracle apart from the tree was run, once: it reproduced the tree's two pins at seed 27 and returned the count, 949. Had the count been outside its bounds the round would have stopped there as a finding.
2. **The rules before their result.** The clauses, the step, the readings' rules, the arms and the gate's test were written with every pinned table of an arm empty. The gate's test passed in 18 s in the debug profile on the developer machine (a ratio, not admissible); its eight rewarded trials under the feedback on the instrument's unsettled network are the harness's check and read nothing of H-28.
3. **The build and the protocol committed and pushed** (`9b9be41` and `2c9bc74` on `main`; `cc09710` and `a9a08c2` on the branch before the rebase that merged pull request #184, pushed at 10:38:28Z on 2026-10-07 with the pull request opened as a draft on them at 10:39:22Z), before any rewarded trial of an arm. The pull request's checks passed on `a9a08c2` (`2c9bc74`): the check, the tests in the debug and the release profile, the format, Clippy, the documentation and the benchmarks; the MSRV; the determinism pin on AArch64; and the mutation gate on the changed lines, 9 mutants in ADR-0148's lines, 8 caught and 1 unviable, none missed.
4. **The calibration.** **Every one of the 85 whole-domain tests of the weekly job before this round passed**, each in a process of its own and each exit 0 with one test reported: 75 eight at a time from 10:22:47Z to 12:06:23Z, H-25's two arms and every earlier run through `Task::trial` among them, and the ten of ADR-0097 and ADR-0102 one at a time to 12:09:44Z. They ran from a release build of the working tree made at 10:17Z, twenty-one minutes before that tree was committed as `a9a08c2` (`2c9bc74`); the test file was last written at 10:15:45Z and the tree was clean at the commit.
5. **The arms**, once, side by side from 12:11:00Z to 12:27:05Z, from that same release build. In each the calibration held: the coin's count, ADR-0077's settled engine and H-20's to H-23's images by their CRCs, ADR-0077's frozen block, and **H-25's first block from H-23's image under the true feedback reproduced table by table**. Each ran its 7 680 trials with the drawn sources held to the critic's oracle, every reward held to the coin and both oracles held to the record at every trial, and stopped at the first empty table, in 961 and 956 s. The tables were written from those dumps (`eea8607`; `f1b1f76`), and a second run side by side from 12:40:34Z reproduced every table and passed, in 1 274 and 1 268 s (on the developer machine, a ratio and not admissible).

No constant, clause or rule moved after the first rewarded trial of an arm, and there was no second attempt: the second run is the pinned tables' reproduction. Between the two runs the gate's test gained its checks over the pinned tables and the verdict's two constants, written from the rule's own output over those tables. A reproduction begun at 12:32:33Z was stopped six minutes in, unread, so that the last of those checks could be written first; the run counted is from the build of the test file as committed.

### The readings

**The verdict** (`MISLED_1024`), by the rule committed first, over the pinned tables:

| Clause | From the assignment | From the mirrored assignment |
| :--- | :--- | :--- |
| 1, each mapping's last 128 correct, at least 80 | 123, 113, 122, 119 | 124, 107, 122, 114 |
| 2, the highest coupling over its image's, at most 1.30 | 1.181, A onto readout 0 at the 88th block | 1.187, A onto readout 1 at the 87th block |
| 3, the excitatory sum outside the couplings, 0.75 to 1.25 of the image's | 0.99998 to 1.00012 | 0.99998 to 1.00015 |
| 4, each stimulus's mean value within 0.25 of three quarters of $2p - 1$ | all eight; the farthest 0.104 | all eight; the farthest 0.131 |

**H-28 is yes: all four clauses hold in both arms.** H-25 read 122, 124, 119, 116 and 124, 118, 123, 117 under clause 1, and 1.181 and 1.191 under clause 2.

**The prediction, against the reading.** ADR-0147 predicted yes, and said the clamp's part was not predicted. The verdict is as predicted. The second mapping is the weakest in both arms, 113 and 107, where H-25's was 124 and 118.

**The trials by outcome and by the reward received** (`MISLED_OUTCOMES_1024`; true rewards, true punishments, misleading rewards, misleading punishments):

| Mapping | From the assignment | Its last 128 | From the mirrored | Its last 128 |
| ---: | :--- | :--- | :--- | :--- |
| 1 | 1 073, 282, 38, 143 | 110, 5, 0, 13 | 1 057, 298, 37, 144 | 112, 3, 1, 12 |
| 2 | 805, 981, 157, 105 | 100, 11, 4, 13 | 695, 1 091, 159, 103 | 93, 18, 3, 14 |
| 3 | 937, 861, 123, 127 | 109, 6, 0, 13 | 1 126, 672, 98, 152 | 109, 6, 0, 13 |
| 4 | 900, 892, 119, 137 | 101, 7, 2, 18 | 828, 964, 136, 120 | 94, 14, 0, 20 |

- Over a run, **512 and 519 correct selections were punished and 437 and 430 wrong ones or ties rewarded**, 949 in each.
- A correct selection was punished in 12.1 and 12.3 per cent of the correct trials, the coin's share.
- A trial tied in 5.0 to 6.7 per cent of a mapping's trials, as under H-25 (4.4 to 7.0).

**The value** (`stimulus_values`, `rewards_received`; per mapping and stimulus over the mapping's last 128 trials, as fractions of the reward):

| | Mean value | Three quarters of $2p - 1$ | $2p - 1$ | Mean reward received |
| :--- | :--- | :--- | :--- | :--- |
| Assignment, A | 0.656, 0.593, 0.751, 0.619 | 0.676, 0.609, 0.726, 0.722 | 0.902, 0.812, 0.968, 0.963 | 0.672, 0.625, 0.839, 0.519 |
| Assignment, B | 0.740, 0.451, 0.559, 0.605 | 0.705, 0.539, 0.636, 0.588 | 0.940, 0.719, 0.848, 0.784 | 0.761, 0.625, 0.576, 0.676 |
| Mirrored, A | 0.714, 0.362, 0.811, 0.525 | 0.725, 0.375, 0.726, 0.611 | 0.967, 0.500, 0.968, 0.815 | 0.738, 0.312, 0.839, 0.370 |
| Mirrored, B | 0.608, 0.622, 0.506, 0.453 | 0.683, 0.633, 0.636, 0.568 | 0.910, 0.844, 0.848, 0.757 | 0.791, 0.688, 0.576, 0.541 |

- **The critic holds the expected reward, not the outcome's.** In all sixteen the value is nearer three quarters of $2p - 1$ than $2p - 1$ (`nearer_expected` above zero in each). Under H-25 the same sixteen stood at $2p - 1$: 0.69 to 0.98 against 0.68 to 1.00.
- **What clause 4 could not tell apart, the reading does.** H-25's own rule, a quarter of the reward either side of $2p - 1$, holds eight of these sixteen and fails eight.
- The value follows the reward the stimulus received, which the coin moves about its expectation: over those trials the mean reward a stimulus received was 0.31 to 0.84 of the reward, and its mean value 0.36 to 0.81.
- **The troughs after each flip are shallower**: −0.37 to −0.75 of the reward, against H-25's −0.63 to −0.84, in eleven of the twelve.

**The reversal speeds** (`speeds`; the blocks to the first of a mapping's blocks with 40 of 64 correct):

| | First reversal | Second | Third |
| :--- | :--- | :--- | :--- |
| From the assignment | 19, H-25's 19 | 19, H-25's 21 | 20, H-25's 16 |
| From the mirrored | 24, H-25's 20 | 14, H-25's 15 | 21, H-25's 14 |

- **Five of the six are under four thirds of H-25's**, the naive scaling; the sixth, 21 blocks against 14, is over it, four thirds of 14 being 18.7. As ratios: 1.00, 0.90, 1.25 and 1.20, 0.93, 1.50; their mean is 1.13.
- Every reversal passed within its 32 blocks, the slowest in 24. H-25's bound of 23 blocks is not a clause here; one reversal is a block past it.
- The first mapping was learned as fast as under H-25, in 6 and 4 blocks.
- Per stimulus, the selection first went to the new answer more often than to the old 9 to 23 blocks after a flip, against H-25's 9 to 23.

**The learning signal** (`signal_of` over `MISLED_WENT_1024`, the network's oracle; the answer pairs' net consolidation less the other pairs', per mapping, over H-25's):

- From the assignment: **1.03, 0.93, 0.87, 0.93**. From the mirrored: **0.98, 0.87, 0.92, 0.96**.
- **The naive scaling says 0.75.** The signal was not scaled by the reliability.
- In the answer's pairs the weight raised was 1.0 to 1.6 of H-25's and the weight lowered 1.1 to 1.7, and their net 0.72 to 1.12 of it: more was moved each way for about the same net.

**What the clamp's part was.** Two effects pull against the naive scaling, and both are read (option 4(a)).

*A true reward is worth more.* The critic holds about three quarters of what H-25's held, so a correct trial that is truly rewarded still delivers an error:
- over a learned mapping's last 128 trials a true reward's error was **0.30 to 0.52 of the reward** (`MISLED_KIND_ERRORS_1024`), where under H-25 the error on a correct trial was 0.08 to 0.18;
- in the trial after a true reward the answer's pair gained 3 030 to 4 150 a trial (`MISLED_KIND_MOVES_1024`), against H-25's 1 570 to 2 620 after a reward: **1.3 to 2.0 times**.

That is the effect ADR-0147 named and did not size: *"true rewards and true punishments keep delivering errors that the certain task's critic had spent."*

*A misleading punishment is capped, and still costs twice a reward.*
- Its error was −1.09 to −1.48 of the reward by mapping, and −1.48 to −1.70 over a learned mapping's last 128 trials, as the arithmetic said: below −1 for any value above zero.
- **It left the signal below the gate's floor in 68 per cent of them**, by 0.50 to 0.54 of the reward on average (`MISLED_BEYOND_1024`). There the clamp passed on −1 and no more.
- In the trial after it the answer's pair lost 6 690 and 6 740 a trial over the run, **1.9 and 2.0 times what a true reward gave**.
- One correct trial in eight was punished, so misleading punishments took back **27 and 28 per cent** of what true rewards gave the answer's pairs: 3.43 of 12.85 million from the assignment, 3.50 of 12.68 million from the mirrored.

*The same on the wrong side.* A misleading reward's error was above the reward, 1.1 to 1.3 of it after the first mapping, because a wrong selection comes mostly after a flip, where the value is below zero. It left the signal above the ceiling in 62 and 64 per cent of them. In the trial after it the pair the wrong selection reached gained 4 510 and 4 530 a trial, against the 3 630 and 3 350 a true punishment took from it; misleading rewards put back 18 and 19 per cent of what true punishments took.

*Over the whole run* the signal a reward left lay beyond the gate's bounds after **37.8 and 37.0 per cent of the trials**: above the ceiling after 1 429 and 1 380, below the floor after 1 471 and 1 462, by 0.40 to 0.47 of the reward on average.

**What this does not say.** These are readings of the run as it was. The run has one gate; what the learning would have been with a wider clamp, or with none, was not run and is not derived (option 4(b)).

**Where else the run stands beside H-25.**
- **The couplings' separation** at each mapping's end, the answer's coupling less the other's, was 0.64 to 1.11 of H-25's per stimulus, lowest at the end of the second mapping, 0.64 to 0.86.
- **The inhibitory sum** ended at 0.850 and 0.845 of the image's, its lowest, as H-25's ended at 0.848 and 0.850.
- **The drawn sources** were 99.66 and 99.67 per cent of the presented stimulus's units and 1.63 and 1.64 others a trial, H-25's.
- **The first new selection** after a flip came within 3 to 190 trials; H-25's within 3 to 127.
- Over whole mappings fewer trials were correct than under H-25 in seven of the eight, by 2 to 25 per cent; the eighth had 2 per cent more.

**What H-25's tables say here, and what they do not.** Three of them read a delivery's sign, which under the engine's critic is the error's (item 3 of the context):
- `MISLED_EARNED_1024`'s positive deliveries are the trials whose error was above zero;
- `MISLED_MOVES_1024` splits the addressed pair's moves by the sign of the error before;
- `MISLED_VALUES_1024` sums the errors on correct trials, misleading punishments among them: over a mapping's last 128 that mean is 0.08 to 0.25 of the reward, which is not a true reward's.

**In these two runs every error had its reward's sign.** The engine's value never passed the reward: the 4 152 and 4 136 deliveries above zero are the 3 715 and 3 706 true rewards and the 437 and 430 misleading ones, and none was exactly zero. So the first two tables read here what they read under H-25. The gate holds that block by block: the positive deliveries are the rewards of either kind, and the moves after a positive delivery and after a negative one are `MISLED_KIND_MOVES_1024`'s after a reward of either kind and after a punishment of either kind. It is a reading of these runs and not a property of the rule.

**The oracles held** at every one of the 15 360 trials: the drawn sources to the units the harness's critic oracle counted from the train, the targets to the selected readout's units, the engine's value, error and weights to the harness's critic, the reward to the coin, and the composer's and the network's traces and weights to the record.

### The evidence

- **The dispatch's scope.** The diff changes files under `src/` — the task, ADR-0148's feedback — so by [ADR-0075](0075-the-dispatch-scope-follows-the-diff.md) the weekly was dispatched on this round's branch at **`scope=both`**, as brief 061 asks: run [37626835338](https://github.com/DescentVTT/VirtualCortex/actions/runs/37626835338) at `ed43fbc` (`f574991` on `main`), the readings' commit, whose tests are those of `f1b1f76` (`eea8607`), the pinned tables. The round's commits as `main` holds them, beside the branch's: `9b9be41` (`cc09710`) the build; `2c9bc74` (`a9a08c2`) the protocol, before any rewarded run; `eea8607` (`f1b1f76`) the arms' tables and the gate's checks over them; `f574991` (`ed43fbc`) this ADR's readings and the documents; `a01d721` (`fe81b0f`) this section, the cost table and the brief's archive.
- **Every exhaustive job is green.** All eighty-seven `exhaustive` tests passed, each exit 0 with one test reported: the eighty-five before this round reproduced their pinned numbers on the hosted runners, and the two arms reproduced their tables there, in **1 857 s from the assignment and 1 102 s from the mirrored assignment**; H-25's arms took 1 817 and 1 837 s in the same run.
- **The shards**, dealt by the table before this round, which did not know the two arms and costed each at 900 s:

  | Shard | Job | Its two heaviest tests (s) | Tests | Their seconds summed | Tests' wall time |
  | ---: | ---: | :--- | ---: | ---: | ---: |
  | 0 | 62.7 min | H-27 from the mirrored 1 495; H-22 from the assignment 1 406 | 14 | 7 060 | 3 701 s, 51 % |
  | 1 | 64.7 min | H-20 from the mirrored 1 919; H-24 from the mirrored 1 818 | 14 | 7 569 | 3 794 s, 53 % |
  | 2 | 84.2 min | H-24 from the assignment 1 979; H-27 from the assignment 1 887 | 15 | 9 806 | 4 965 s, 69 % |
  | 3 | 58.1 min | H-23 from the assignment 1 416; H-23 from the mirrored 1 414 | 14 | 6 505 | 3 425 s, 48 % |
  | 4 | 41.7 min | H-28 from the mirrored 1 102; the control from the assignment 1 095 | 15 | 4 833 | 2 451 s, 34 % |
  | 5 | 76.2 min | H-25 from the mirrored 1 837; H-22 from the mirrored 1 819 | 15 | 8 992 | 4 507 s, 63 % |

  The percentages are of the job's bound, 120 minutes; every shard ran its tests two at a time, at 1.90 to 2.00 of their summed seconds over the wall. **Two shards of this run passed 60 per cent of their bound**, the third at 69 and the sixth at 63. The third drew H-28's arm from the assignment, costed at 900 s and running 1 857; and the eighty-five earlier tests ran at 1.051 of the table's seconds on this run's runners. The brief's directive is on the regenerated deal, below, which holds it; this run's own deal did not.
- **The cost table is regenerated from this run** (`node scripts/exhaustive-costs.mjs from <artifacts> --run 37626835338`): 87 lines, 44 765 s. ADR-0092's deal plans each of the six shards at 7 460 to 7 461 s summed, 3 730 to 3 930 s of wall time at the run's ratios, **about 53 per cent of the bound**, inside the brief's 60 and above the 50 planned earlier in this ADR. What is left under 60 per cent is about 6 000 s summed, three arms of this size at this run's speed.
- **The mutation sweep.** Seven jobs, green: 3 801 mutants over the tree, **3 626 caught and none missed**, 2 398 in the state crates and 1 228 in the runtime, 151 unviable and 24 timeouts. Every mutant the sweep made in the round's code was caught or did not compile: `Task::misleading_at`'s seven and the feedback's one in `Task::trial` caught; `Task::trial`'s whole body unviable, an `Outcome` having no default. **Twenty-two of the timeouts are the known protocol ones** — the iterators of `cortex-core`, the injector, the barrier, `stop_workers` and the workers' loop. By [ADR-0063](0063-the-sweep-reads-its-own-timeouts.md)'s evidence every one of the runtime's sixteen among them had stopped finishing tests or hung a test of its own module, and the six of the state crates are without the evidence to say, their bound being under cargo's sixty seconds; by [ADR-0062](0062-the-first-complete-sweeps-list.md)'s triage they are detections. **Two timeouts are not on that list**, both in `runtime-4` and neither in a file the round changed: `image.rs:1017:24: delete ! in Image::decode` and `store.rs:148:9: replace Induction::first_invention -> Option<&Discovery> with None`. Each ran beside a hung mutant of the injector in the shard's other slot and was still finishing tests, in `learning` and in `reference`, when its bound of 1 779 s arrived, with no test of its own module hung: **F-42's kind**, a slow pass starved by the hang beside it, which the evidence reports and does not classify. Both were caught in ADR-0145's sweep. Run alone on the developer machine after the sweep, both are caught: the first by the tool, its tests failing after 116 s; the second applied by hand, the tool's scratch build being refused twice by the Windows linker, with three tests of `reference` failing after 252 s and the file restored. F-42 stays open as it was: the class recurs, not the mutant. The runtime's six shards took 1 h 42 min to 4 h 06 min against their bound of 330 minutes, the longest the one with the two; the state crates' 20 minutes.
- **The pull request's gate** on `a9a08c2` (`2c9bc74`), on `ed43fbc` (`f574991`) and on `fe81b0f` (`a01d721`) is green in every job, the determinism pin on AArch64 and the MSRV among them; the mutation gate on the changed lines found 9 mutants in each, 8 caught and 1 unviable, none missed. The unviable is the tool's synthesized return for `Task::trial`, whose `Outcome` has no default. The same run on the developer machine read 6 caught and 3 unviable, two of them the Windows linker's refusal to overwrite a test binary and not the mutant's, and none missed.
- **On the developer machine**, at `ed43fbc` (`f574991`): the workspace's tests passed in the debug profile, in the release profile and on the MSRV, 701 passed and 127 ignored in each; the check, the format, Clippy, the documentation, the benchmarks and `npm run spec` exited 0; 87 whole-domain tests are listed.

### The step of the stopping rule reached

**Step 3**: *"Yes: the configuration learns and revises under a reward right seven times in eight. The next decision is an ADR choosing among a less reliable reward, more answers than two, a reward delayed past a trial, another size and the operating regime, named and not taken."* **The next decision is that ADR.** It is named and not taken. What this round hands it:

- **Seven in eight is not at the schedule's edge.** The reliability was derived as the least the schedule holds under a slowdown of four thirds. The slowdown read was 1.13 on average and the slowest reversal took 24 of 32 blocks. A less reliable reward on this schedule is not ruled out by this reading; by ADR-0147's arithmetic four in five would have put 21 blocks at 35.
- **Why the slowdown is under the naive one**: the critic's value settles lower, so each true reward teaches more, by 1.3 to 2.0 times in the answer's pair. By ADR-0147's arithmetic that gain grows as the reliability falls; this round read it at one reliability.
- **What stands against it**: a misleading punishment costs about two true rewards in the answer's pair even with the clamp passing on no more than −1 in two of three. At four in five one correct trial in five is punished. Which of the two is the larger there is not read.
- **The second mapping** is where the margin is thinnest: 113 and 107 of 128, and the mirrored arm's first reversal at 24 blocks.

Steps 4, 5 and 6 did not arise; step 7 is kept.

## Consequences

- Good: the first reading of the learning configuration under a reward that is not always true is a yes, with H-25 beside it block for block.
- Good: the reading says why the slowdown is under the naive one, from tables in the tree: what a true reward moved, what a misleading punishment moved, and how often the gate's clamp acted.
- Good: every trial's reward is read back whole and held to a coin written twice, so no table of this round calls an error's sign a reward's.
- Good: H-25 stands beside the run block for block, table for table.
- Bad: the clamp's part is read from what the run did, not from a run without the clamp. A reading can say how often and by how much the signal lay beyond the gate; it cannot say what the learning would have been otherwise.
- Bad: one reliability, one seed and one size. Whether seven in eight is near an edge is not read; the reading above says only that the schedule was not strained.
- Bad: two more arms in the weekly job, each about H-25's cost.

## Alternatives considered and why rejected

- **The sign of the delivery** (option 1(b)): under the engine's critic it is the error's.
- **A field in the outcome** (option 1(c)): a change to `Outcome` for what the error and the value already give.
- **The earlier tables rewritten** (option 2(b)): H-25's tables would lose their counterparts.
- **H-26's shadows beside the arms** (option 3(b)): see above.
- **A second run with the clamp widened** (option 4(b)): a change to the signed gate, which the brief does not empower, and a second configuration where the round has one.
- **The clauses' inputs alone** (option 5(b)): the readings ADR-0147 names could not be recomputed from the tree.

## Confirmation

- `runtime/cortex-runtime/tests/inhibition.rs`: `MISLED_ARMS`, `MISLED_FEEDBACK`, `MISLEADING_BOUNDS`, `MISLEADING_BY_MAPPING_1024`, `RELIABILITY`, `holds_expected_reward`, `Misled`, `misled`, `misled_step`, `MisledRead`, `misled_arm`, the two weekly tests and the gate's test named above.
- `runtime/cortex-runtime/tests/instrument/harness.rs`: `misleads`.
- The arms' pinned tables, `MISLED_*_1024`, the verdict `MISLED_1024` and its step `MISLED_STEP_1024`, and the gate's checks over them.
- Whitepaper §11.1 and §9; `CHANGELOG.md`; `CLAUDE.md`; `README.md`; `docs/zh-TW/README.md`.
