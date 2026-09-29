---
status: proposed
date: 2026-09-29
depends-on: ADR-0131
decision-makers: VirtualCortex maintainers
---

# ADR-0132: A critic of the engine's own, measured — brief 055's protocol, committed before any rewarded run: the critic's step shift and scale placed by an arithmetic written first, a step of $2^{-9}$ and a scale of $2^{-2}$, so that one stimulus volley moves the value of the same volley by 51/2 048 of the error, the least sum of the two shifts at which that is no more than the task critic's thirty-second; the image each arm decodes, H-21's with the critic written and nothing else, shown by a masked check; H-21's first block reproduced with the critic unset before it; H-21's two arms run once from it with the task's critic replaced by the engine's; H-22's clauses as integer rules and the readings beside them

## Context and Problem Statement

[ADR-0130](0130-a-critic-of-the-engines-own.md) took a critic of the engine's own and wrote **H-22** before any run: on H-21's configuration — H-20's configuration, schedule and arms with the inhibitory rule's target at the settled network's rate — with the task's critic replaced by the engine's, (1) every mapping is learned, (2) no coupling passes 1.30 of its image's at any block's end and (3) over each mapping's last 128 trials the mean error on the correct trials is at most half the reward's magnitude, in both arms. [ADR-0131](0131-the-critic-built.md) built the critic. Brief 055 runs H-22 once. This ADR is the round's protocol, committed before any rewarded run, and then its readings.

What was read before anything was written (principle 2, and brief 055's directive that the engine is read before a description of it is trusted, the brief's and ADR-0130's included):

1. **H-21** (`runtime/cortex-runtime/tests/inhibition.rs`, [ADR-0129](0129-a-target-the-network-fires-at-measured.md)): `target_arm` builds ADR-0077's settled engine and H-20's signed image (`signed_images`), reads the target period by ADR-0128's rule, writes it into H-20's image (`targeted_image`) and runs `earned_run_observed` with the task's critic of shift 5 (`CRITIC_AT_START`) and H-20's flips (`SCHEDULE_FLIPS`). Its target is pinned, `TARGET_PERIOD_1024`, 58 201 ticks; its tables are pinned whole, 120 blocks an arm, the task critic's expectation per block among them (`TARGET_EXPECTED_1024`).
2. **The trial and the rewards** (`tests/instrument/harness.rs`): `run_on_scheduled` drives a lead-in of 500 ticks (`LEAD_IN`) before the first trial, then trials of $2^{14}$ ticks that abut, the task rewarding at every trial's end under `Feedback::Answer`. So the engine's first window is the lead-in and the first trial, and every later window is one trial.
3. **The reward**, `REWARD_Q16`, 1.0 (65 536), signed by the outcome; the task critic moves the presented stimulus's expectation by a thirty-second of the error (`CRITIC_SHIFT`, 5).
4. **The spikes of a trial at H-21's rates** (`TARGET_SPIKES_1024`, by class per block of 64 trials): in the first blocks about 72 of the 204 inhibitory units' spikes, 74 of the 102 stimulus units' — 51 of them the presented stimulus's volley, each of its units once (ADR-0076) — 204 of the 714 readout units' and one of the four others', **about 351 a trial**. About 300 are the background, over some 970 units, at about 0.3 a unit a trial.
5. **The task's outcome under the engine's critic** (ADR-0131): the task delivers the outcome's reward, `Outcome::reward_q16` records the error the modulator received and `Outcome::value_q16` the engine's value. The harness's composer is fed `reward_q16` to replay the modulator, so it replays the engine's error as it replayed the task critic's.
6. **The critic's rule** (ADR-0131): the value $\lfloor \sum_i w_i c_i / 2^s \rfloor$, the error $r - V$, each weight moved by $\lfloor \delta c_i / 2^k \rfloor$, the weight an `i16`, $k \le 30$, $s \le 14$.

## Decision Drivers

- Brief 055's standing directives:
  - the one mechanism ADR-0130's, and nothing else of the engine changed;
  - the critic reading only the engine's train and its rewards;
  - the shift and the scale placed by an arithmetic written first, and H-22's constants, clauses, shift and scale committed before the first rewarded run and not moved after it;
  - no second attempt; no float; every loop ends by construction; the gate grows by at most one test; no shard of the weekly job past 60 per cent of its bound; no pinned number of an earlier round moves but the whole-image pins ADR-0131 restated.
- Latest ≠ Newest: no dependency, no tool; the delta rule and the least-mean-squares account of its noise are textbook (Sutton and Barto 2018; Widrow and Stearns 1985).

## Considered Options

1. **The sum of the two shifts**, $k + s$: 10, 11 or 12.
2. **Its split**: a scale of 0, 2 or 4.
3. **How the critic reaches the arms' engine**: (a) H-21's image with the critic's three bytes written and the section re-sealed, as H-21 wrote its period; (b) the configuration's critic at decode.
4. **The calibration's reproduction of H-21**: (a) H-21's two weekly tests unchanged, and in each H-22 arm H-21's first block from H-21's image with the critic unset, held to H-21's tables; (b) H-21's two weekly tests alone.
5. **What is pinned**: the whole run of each arm, and the engine's value per block and every trial's by a hash.

## Decision Outcome

Options 1 at 11, 2 at 2, 3(a), 4(a) and 5. Everything below is committed before any rewarded run.

### The arithmetic that places the shift and the scale

Under the rule a reward's step moves the value of the same features by
$$\Delta V = \frac{1}{2^s} \sum_i \left\lfloor \frac{\delta c_i}{2^k} \right\rfloor c_i \approx \frac{\delta}{2^{k+s}} \sum_i c_i^2 .$$

- **The volley alone.** The presented stimulus's 51 units each fire once, so the volley moves the value of the same volley by $51\,\delta / 2^{k+s}$, and since the next presentation of the stimulus fires the same 51 units, that share carries to it whole. The task critic moves the stimulus's expectation by $\delta / 32$. **The least $k + s$ at which $51 / 2^{k+s} \le 1/32$ is 11**: $2^{11} = 2\,048 \ge 51 \times 32 = 1\,632 > 2^{10}$. At 11 the volley moves the value by $51/2\,048$ of the error, about a fortieth, 0.80 of the task critic's step; at 10 it would be 1.59 of it, and at 12, 0.40.
  - Worked: from weights of zero, an error of 1.0 moves each of the volley's weights by $65\,536 / 2^k$; at $k = 9$, $s = 2$ that is 128, and the volley's value then $51 \times 128 / 4 = 1\,632$, beside the task critic's 2 048.
- **The background in the same step.** About 300 background spikes add $\sum c_i^2 / 2^{11} \approx 380 / 2\,048$, 0.19, of the error to the value of the same trial's features, so one reward moves its own trial's value by about $431/2\,048 \approx 0.21$ of the error in all. The next trial's background is mostly other units: what carries to it is the background's mean, $\sum_i \lambda_i^2 / 2^{11} \approx 970 \times 0.09 / 2\,048 \approx 0.04$ of the error, a bias learned at about a twenty-fifth. The rest is noise in the weights.
- **The noise the background adds to a value in a trial.** At one weight $w$ on every background unit, 300 spikes carry $75 w$ and a Poisson count of $300 \pm 17$ moves that by $\pm 4.3 w$, 5.8 per cent of what it carries: with the background carrying the whole reward, $w = 874$, the value moves by about 3 700 from trial to trial, 5.7 per cent of the reward. The step's own noise, the least-mean-squares misadjustment $\mu \operatorname{tr} R / (2 - \mu \operatorname{tr} R)$ with $\mu \operatorname{tr} R \approx 0.21$, adds about 12 per cent to the least mean squared error a linear critic of these features can reach (Widrow and Stearns 1985). Clause 3's bound is half the reward, 32 768, an order of magnitude above either.
- **The split.** With $k + s = 11$, a scale $s$ puts the weight's rail, `i16::MAX`, at $32\,767 / 2^s$ of reward a spike, the volley's weights at $65\,536 \cdot 2^s / 51$ when it carries the whole reward, and the step's dead band at $2^k$ — a positive error below it moves no weight, a negative one moves each unit that fired one LSB down, $351 / 2^s$ of value a trial:

  | Scale $s$ | Shift $k$ | The rail a spike | The volley carrying 1.0, of the rail | The dead band |
  | :--- | :--- | ---: | ---: | ---: |
  | 0 | 11 | 0.50 of the reward | 0.039 | 3.1 % |
  | **2** | **9** | **0.125** | **0.157** | **0.78 %** |
  | 4 | 7 | 0.031 | 0.63 | 0.20 % |

  **The scale 2 and the shift 9** (`VALUED_CRITIC`): the dead band under one per cent of the reward, the floor's drift 88 of 65 536 a trial, 0.13 per cent, and the volley's share of the whole reward at a sixth of the rail, room for the value to gather on a sixth of the volley's units. A scale of 0 leaves a dead band of 3 per cent; a scale of 4 puts the volley's share at two thirds of the rail.

The gate holds each number of this section: the two inequalities as compile-time assertions, and the volley's step, the rail, the floor and the background's count over numbers written by hand.

### The image each arm decodes (option 3(a))

- **`valued_image`** writes the critic into H-21's image — the modulator section's `[48]` the flag, `[49]` the shift, `[50]` the scale — and re-seals the section. H-21's image is `targeted_image` of H-20's at `TARGET_PERIOD_1024`, pinned by its CRC-64 at format 19, `TARGET_IMAGE_CRC_1024` (`0xeefb68fd147b41c5`; `0xafe7eff2d59da51f` at 18, the CRC ADR-0129 read).
- **`only_the_critic`** is the masked check: H-21's image carries no critic; every position that differs lies in those three bytes or in the modulator's entry in the section table, which holds the seal; and zeros written back give H-21's image bit for bit.
- **Each arm asserts before its run**: the decoded engine's critic and target, every weight and every count zero, the sums and the four couplings the image's.
- The configuration (option 3(b)) would change nothing: the image's critic, set or unset, outranks it.

### H-22, restated as integer rules (ADR-0130's clauses and constants)

- **Clauses 1 and 2**: H-20's `scheduled`, unchanged — each mapping's last 128 trials at least 80 correct, and no coupling above 1.30 of its image's at any block's end.
- **Clause 3, the critic predicts** (`predicts`, `mapping_errors`): over each mapping's last 128 trials — 1 409 to 1 536, 3 457 to 3 584, 5 505 to 5 632 and 7 553 to 7 680 — the errors the correct trials delivered summed, $E$, and their count, $n$; the clause holds when $n > 0$ and $2E \le r\,n$, with $r$ the reward's magnitude, 1.0. A mean error below zero holds. A mapping with no correct trial among its last 128 does not.
- **The verdict** (`Valued`): `learning` (H-20's `Scheduled`), `predicted` per arm and mapping, and `yes` when all hold in both arms. `VALUED_PREDICTED` is `None`: ADR-0130 predicts no verdict.

### The assertion, beside the verdict

H-20's and H-21's (`punished_held`): no excitatory synapse outside the four stimulus–readout pairs moves from the image to the run's end. The oracle is held to the record's weights, traces and signal at every trial inside the harness. A false assertion is a finding, reported beside the verdict.

### The harness

`tests/instrument/harness.rs` gains the critic's rule written a second time, `value_step`, in `i128` by `div_euclid` and not by a shift, and `earned_run_valued`, `earned_run_observed` with the engine's critic held to it: with the critic set, the harness keeps its own weights from the record's at the start and its own counts from the executor's at the start and every spike of the train since the last reading, the lead-in's included; at every rewarded trial it forms the value, the error and the step, holds the outcome's value, the error it records and every unit's weight to them, and counts afresh. `earned_run_observed` calls it and asserts the engine carries no critic, so every earlier run is the run it was.

### The readings, no clause (ADR-0130's)

- **The engine's value beside the task critic's expectation**: every trial's value (`VALUED_VALUE_HASH_1024`) and, per block and stimulus, its trials, their values summed, its correct trials and their errors summed (`value_blocks`); each block's mean value per stimulus dumped beside H-21's expectation for that stimulus at the block's end (`TARGET_EXPECTED_1024`).
- **The weights by group at every block's end** (`weights_by_group`): the value weights summed over the inhibitory units, stimulus A's and B's, readout 0's and 1's and the others, with the lowest and the highest weight of any unit — which units carry the value.
- **The reversal speeds** beside H-20's and H-21's: `crossings`, `crossed_scheduled` and `first_new_scheduled`.
- **The inhibitory sum's course**, each block's as a fraction of the settled image's, beside H-21's.
- **H-21's other readings** by H-20's rules: the tallies, the settle measure, the highest coupling per mapping, the punishment's course, the moves by mapping, the blocks in which the stimulus fired once, and the sums after.

### The calibration, before any rewarded run (H-22's stopping rule, step 2)

- **In the tree, with the critic unset**: the workspace's tests in the debug and the release profile; every whole-domain test of the weekly job, H-21's two arms among them, in a process of its own from the release build of this commit; the determinism pin with them.
- **In each arm's test**, before its first rewarded trial: the settled engine held to ADR-0077's tables step by step and H-20's image to its CRC (`signed_images`); a frozen block from the zero image held to ADR-0077's frozen run (`calibration_holds`); H-21's image by its CRC; and **H-21's first block from H-21's image with the critic unset** (`h21_first_block`), 64 trials under the task's critic held to H-21's pinned first block, table by table (option 4(a)); then the masked check.

A failure at any of them stops the round there as a finding.

### The gate

`the_critic_s_arithmetic_the_clauses_of_h_22_and_the_image_s_patch`, one test, no run:
- the constants, H-21's restated, and H-21's image's CRC;
- this ADR's arithmetic over numbers written by hand: the volley's step beside the task critic's and at the sum of ten, the rail, the volley's share, the step's floor and the background's count;
- `value_step` over a table written by hand and held to the engine's rule over 2 000 seeded draws;
- clause 3 at half the reward and one LSB over, with no correct trial, over trials written by hand across the four mappings, and the verdict naming the clause, the arm and the mapping;
- the readings' rules over tables written by hand, and the groups on the instrument's network, 204 inhibitory units, 51 of each stimulus set, 357 of each readout set and four others;
- the patch on that network's image: the positions it moves, the loader reading the critic, zeros written back giving the image, a byte moved elsewhere failing the check, and constants past their bounds refused by the loader.

### The weekly tests and their cost

- **Two weekly tests**, one per arm: `a_critic_of_the_engines_own_from_the_assignment_at_1024_units_exhaustive` and `a_critic_of_the_engines_own_from_the_mirrored_assignment_at_1024_units_exhaustive`.
- **Each is H-21's arm and a block more**: H-21's arms took 1 705 and 1 790 s on the hosted runners by the cost table, and H-21's first block adds a sixtieth of a run.
- **The plan stays inside the budget.** The table after ADR-0129 is 73 tests and 23 654 s, about 28 per cent of the bound at six shards. The two arms add about 3 600 s, and the plan rises to about 32 per cent, under the directive's 60.

### The order of the work

1. The critic built, ADR-0131, and its tests, with the four whole-image pins restated.
2. This commit: the rules, the constants, the arithmetic, the gate, the arms with empty pins, and this ADR's protocol, before any run.
3. The calibration above, before any rewarded run.
4. The two arms, once, stopping at their first empty pin; the tables written from the dumps, and a second run to reproduce them.
5. This ADR's readings and the documents; then the weekly dispatched on the branch at `scope=both` and the cost table regenerated from it.

## Consequences

- Good: the shift and the scale are read off an arithmetic committed before the first rewarded run, against the task critic's step the comparison is with, so a difference from H-21 is the critic's and not a tuning.
- Good: every input the critic reads is the engine's own; the harness holds the engine's value, error and weights to a second writing of the rule at every trial.
- Neutral: the whole run is pinned, 120 blocks per arm, as H-21's.
- Bad: one seed, one size, one schedule, one pair of constants, as H-21's.

## Confirmation

- `runtime/cortex-runtime/tests/instrument/harness.rs`: `value_step`, `earned_run_valued`, which `earned_run_observed` calls.
- `runtime/cortex-runtime/tests/inhibition.rs`: `VALUED_ARMS`, `VALUED_PREDICTED`, `VALUED_CRITIC`, `VOLLEY_UNITS`, `TASK_STEP_SHIFT`, `PREDICTS_DIVISOR`, `CRITIC_AT`, `GROUPS`, `correct_errors`, `mapping_errors`, `predicts`, `Valued`, `valued`, `groups_of`, `weights_by_group`, `value_blocks`, `values_hash`, `with_critic_bytes`, `valued_image`, `only_the_critic`, `h21_first_block`, `valued_run`, `valued_arm`, `TARGET_IMAGE_CRC_1024`; the two weekly tests and the gate's test named above.
- Every pinned number of ADR-0065 to ADR-0129 unchanged, and the determinism pin; the image format 19.
