---
status: proposed
date: 2026-10-07
depends-on: ADR-0148
decision-makers: VirtualCortex maintainers
---

# ADR-0149: A reward right seven times in eight, measured — H-28's protocol, committed before any rewarded run: H-25's two arms from H-25's image with the one change the task's feedback, the coin misleading in 949 of the run's 7 680 trials, within its bounds; the four clauses as integer rules, the readings by the kind of each trial, and the calibration; the readings and the step of the stopping rule follow the run

## Context and Problem Statement

[ADR-0147](0147-a-reward-right-seven-times-in-eight.md) asked the named learning configuration the first task it had not been asked, a reward that is right seven times in eight, and wrote **H-28** before any run. [ADR-0148](0148-a-reward-right-seven-times-in-eight-built.md) built the feedback: `Feedback::SevenInEight`, its coin three bits of the trial's own draw. Brief 061 runs H-28 once. This ADR is the round's protocol, committed before any rewarded run, and then its readings.

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

## Consequences

- Good: every trial's reward is read back whole and held to a coin written twice, so no table of this round calls an error's sign a reward's.
- Good: H-25 stands beside the run block for block, table for table.
- Bad: the clamp's part is read from what the run did, not from a run without the clamp. A reading can say how often and by how much the signal lay beyond the gate; it cannot say what the learning would have been otherwise.
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
- Whitepaper §11.1 and §9; `CHANGELOG.md`.
