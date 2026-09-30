---
status: accepted
date: 2026-09-29
depends-on: ADR-0131
decision-makers: VirtualCortex maintainers
---

# ADR-0132: A critic of the engine's own, measured — H-22 is no, on clause 3 alone: with ADR-0131's critic at a step of $2^{-9}$ and a scale of $2^{-2}$, placed by an arithmetic committed before any run, H-21's schedule learned every mapping in both arms, 89 to 125 of each mapping's last 128, with no coupling past 1.30 of its image's, the highest 1.186 and 1.185; over each mapping's last 128 trials the engine's value on the correct trials stood at 0.495 to 0.935 of the reward, and in one mapping, the assignment arm's second, at 0.495, below the half clause 3 asks; the value per stimulus is about $2p - 1$ of the reward, a stimulus's expected reward, so for such a critic the clause is an accuracy of three quarters (F-61); after the first and third flips the value fell to −0.92 to −1.03 of the reward, deeper than the task critic's −0.75 to −0.95, and those reversals took 22 to 30 blocks against H-21's 14 to 23; the inhibitory course H-21's; the assertion held; H-22's stopping rule at step 5, and the next decision named and not taken

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

### The order of the work, as the history holds it

1. **The critic built** (`8eed216` on `main`; `9e04f87` on the branch before the rebase that merged pull request #164, 14:09Z on 2026-09-29): ADR-0131, its tests and the four whole-image pins restated. The workspace in the debug profile: 677 passed, 113 ignored.
2. **The protocol** (`26e9484`; `b589eb7`, 14:15Z, pushed to pull request #164, opened as a draft before any run): the rules, the constants, the arithmetic, the gate, the arms with empty pins, and this ADR's protocol.
3. **The calibration**, before any rewarded run, with the critic unset, from the release build of `b589eb7` (`26e9484`): **every one of the 73 whole-domain tests of the weekly job passed**, each in a process of its own — 63 eight at a time from 14:22Z to 15:31Z, H-21's two arms among them in 2 479 and 2 961 s under that load, and the ten timed tests of ADR-0097 and ADR-0102 one at a time to 15:33Z; the workspace's tests passed in the release profile (678 passed, 115 ignored) and on the MSRV (678 passed); the pull request's checks on `b589eb7` (`26e9484`) passed, the determinism pin on AArch64 among them, and the mutation gate on the changed lines: 66 mutants, 58 caught, 8 unviable, none missed.
4. **The arms**, once, side by side from 15:34Z, the calibration held in each — H-21's first block reproduced table by table with the critic unset — and the masked check passing (eleven bytes differ from H-21's image, three of the critic's and eight of the seal; the image's CRC-64 `0xbf6797857fda0809`); both ran their 7 680 trials and stopped at the first empty table at 15:55Z, 1 284 and 1 274 s. The tables were written from those dumps, and a second run side by side from 15:58Z reproduced every table and passed, in 1 311 and 1 308 s (on the developer machine, a ratio and not admissible); the pins are `9473933` (`492e409`).

No constant, clause or rule moved after the first rewarded trial, and there was no second attempt: the second run is the pinned tables' reproduction.

### The readings

**The verdict** (`VALUED_1024`), by the rule committed first, over the pinned tables:

- **Clause 1 holds in both arms**: each mapping's last 128 trials, **121, 96, 120 and 100** correct from the assignment and **125, 89, 121 and 113** from the mirrored assignment, against the mark of 80 (H-21's: 121, 119, 115, 119 and 124, 113, 115, 119).
- **Clause 2 holds in both arms**: no coupling above 1.30 of its image's at any block's end. The highest per mapping, as a fraction of the image coupling:

  | Arm | First mapping | Second | Third | Fourth |
  | :--- | :--- | :--- | :--- | :--- |
  | Assignment first | 1.164, block 22, A→R0 | 1.151, block 24, A→R0 | **1.186**, block 86, A→R0 | 1.166, block 88, A→R0 |
  | Mirrored first | 1.164, block 23, B→R0 | 1.156, block 24, A→R1 | **1.185**, block 87, B→R0 | 1.181, block 118, B→R1 |

  H-21's highest were 1.190 and 1.193.
- **Clause 3 holds in seven of the eight mappings and fails in one**: the mean error on the correct trials of each mapping's last 128, against the bound of half the reward, 32 768 (`ERRORS_VALUED_1024`):

  | Arm | First mapping | Second | Third | Fourth |
  | :--- | ---: | ---: | ---: | ---: |
  | Assignment first | 7 695 (0.117) | **33 121 (0.505)** | 8 272 (0.126) | 29 737 (0.454) |
  | Mirrored first | 4 239 (0.065) | 28 301 (0.432) | 7 344 (0.112) | 17 152 (0.262) |

  So the engine's value on those correct trials stood at 0.495 to 0.935 of the reward, and at 0.495 in the assignment arm's second mapping, the one below the half.
- `Valued { learning: Scheduled { learned: [[true; 4]; 2], bounded: [true; 2], over: [None; 2], yes: true }, predicted: [[true, false, true, true], [true; 4]], yes: false }`. **H-22 is no, on clause 3 alone**, in the assignment arm's second mapping, with its scope: this task, these two arms, this schedule of three flips over 7 680 trials, H-21's configuration at 1 024 units, and the critic at a step of $2^{-9}$ and a scale of $2^{-2}$. ADR-0130 wrote no prediction for it.

**The engine's value beside the task critic's expectation.** Per stimulus and mapping, over its last 128 trials: the stimulus's accuracy $p$, the value $2p - 1$ of the reward a critic of the stimulus's expected reward would hold, the engine's mean value, and its value on the correct trials:

| Arm, mapping | A: $p$ | $2p-1$ | mean value | value on correct | B: $p$ | $2p-1$ | mean value | value on correct |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Assignment, 1 | 0.951 | 0.902 | 0.906 | 0.901 | 0.940 | 0.881 | 0.867 | 0.866 |
| Assignment, 2 | 0.703 | 0.406 | 0.310 | 0.306 | 0.797 | 0.594 | 0.657 | 0.661 |
| Assignment, 3 | 0.968 | 0.935 | 0.913 | 0.924 | 0.909 | 0.818 | 0.844 | 0.824 |
| Assignment, 4 | 0.870 | 0.741 | 0.588 | 0.596 | 0.716 | 0.432 | 0.522 | 0.503 |
| Mirrored, 1 | 1.000 | 1.000 | 1.047 | 1.047 | 0.955 | 0.910 | 0.821 | 0.829 |
| Mirrored, 2 | 0.438 | −0.125 | −0.099 | −0.126 | 0.953 | 0.906 | 0.883 | 0.887 |
| Mirrored, 3 | 0.968 | 0.935 | 0.944 | 0.935 | 0.924 | 0.848 | 0.832 | 0.842 |
| Mirrored, 4 | 0.796 | 0.593 | 0.501 | 0.497 | 0.946 | 0.892 | 0.890 | 0.886 |

- **The engine learned a stimulus's expected reward**, the quantity the task's critic holds: its mean value follows $2p - 1$ within about 0.15 of the reward in every row, and its value on the correct trials is its mean value within 0.03. It does not tell a correct trial from a wrong one before the reward, though the readouts' spikes that decide the selection are among its features: with two stimuli and two readouts, a linear value over the units cannot hold the conjunction of a stimulus and its answer.
- **So clause 3 is an accuracy bound for such a critic** (F-61): the mean error on the correct trials is about $2(1 - p)$ of the reward, at most a half exactly when $p \ge 3/4$, 96 of 128, against clause 1's 80. The failing mapping had 96 correct, its stimuli at 0.703 and 0.797.
- **At the first mapping's end** the two critics stand together: the engine at 0.927 and 0.922 of the reward (assignment) and 1.044 and 0.910 (mirrored) at the 24th block, the task critic at 0.899, 0.903, 0.975 and 0.887 in H-21.
- **After a flip the engine's value falls deeper.** Over the eight blocks after the first and the third flips its mean value per stimulus fell to −0.92 to −1.03 of the reward, where H-21's task critic fell to −0.75 to −0.95 in the same blocks; after the second flip to −0.51 to −0.78, against −0.70 to −0.86. It came back above zero 17 to 30 blocks after the first and third flips — once not within 32 — and 4 to 13 after the second.

**The reversal speeds** (`CROSSINGS_VALUED_1024`, blocks to 40 of 64, the crossing block counted), beside H-21's and H-20's:

| Arm | First mapping | First reversal | Second | Third |
| :--- | ---: | ---: | ---: | ---: |
| H-22 from the assignment | 6 | **30** | 15 | **27** |
| H-21 from the assignment | 6 | 19 | 20 | 16 |
| H-20 from the assignment | 6 | 19 | 21 | 16 |
| H-22 from the mirrored | 4 | **30** | 10 | **22** |
| H-21 from the mirrored | 3 | 23 | 14 | 14 |
| H-20 from the mirrored | 3 | 23 | 13 | 18 |

The first and the third reversals — the flips after which the value fell deepest — took 7 to 11 blocks longer than H-21's, and the second 4 to 5 blocks fewer. Fewer of the old answer's selections met a strong punishment (the signal at or below −0.5 after the reward; `STRONG_BY_FLIP_VALUED_1024`): 383, 383 and 425 from the assignment and 328, 262 and 381 from the mirrored, against H-21's 527, 573 and 496 and 490, 426 and 432. **An account of these readings, a Hypothesis and not a measurement**: with the value near −1.0 a wrong selection delivers an error near zero, so the punishment that revises the old answer under the signed gate (ADR-0093) fades while the old answer still wins, and a correct selection delivers nearly twice the reward; the value falls that deep because about half of it is carried by units both stimuli share (below), which every error of either stimulus moves. The round reads the value and the punishments' counts, not the modulation each synapse met.

**Which units carry the value** (`VALUED_WEIGHTS_1024`, the weights by group at each block's end). At the first flip, from the assignment, the sums were 160 407 on the 204 inhibitory units, 66 467 and 61 090 on the 51 units of each stimulus set, 131 023 and 152 695 on the 357 of each readout set and 3 683 on the four others; weighted by each group's spikes a trial, the presented stimulus's own units carry about a third of the value, the inhibitory units and the readouts, which both stimuli share, about a half, and the other stimulus's units about a tenth — an estimate from the sums and the classes' rates, not a reading unit by unit. A stimulus that is answered badly carries a value of its own below the rest: in the mirrored arm's second mapping, where A was right in 44 per cent of its trials, A's 51 units summed −121 655 and B's +162 737 at its end. No weight came near the rail: the extremes over the runs were −11 527 and 12 558, 0.38 of it.

**The inhibitory sum's course**, each block's as a fraction of the settled image's, is H-21's within 0.0022 at every block: it rose to 1.0105 and 1.0099 at the 14th and the 11th block, stood at or above the image's until the 27th and the 25th, and ended at **0.849 and 0.846** against H-21's 0.848 and 0.848, still falling. The inhibitory rule's course does not depend on which critic delivers the error.

**The assertion held** in both arms: no excitatory synapse outside the four stimulus–readout pairs moved (`REACH_VALUED_1024`: 3 188 and 3 187 synapses moved inside them, none outside). The oracle — the harness's second writing of the critic's rule among it — was held to the record's value, error, weights, counts, traces and signal at every one of the 15 360 trials.

### The step of the stopping rule reached

**Step 5, no on clause 3 alone**: *"the engine learns the task without learning to predict its reward. The next decision is an ADR on the critic's scale or window."* The configuration learned every mapping in both arms with its couplings bounded, and its critic, reading only the engine's own spikes and rewards, learned what the task's critic held — a stimulus's expected reward — but not enough of it to meet clause 3 in the one mapping learned at three quarters. **The next decision is an ADR on the critic's scale or window, with this round's readings as its need**: the value per stimulus at $2p - 1$, which no scale or window of a linear critic over these features exceeds on a correct trial; F-61, that clause 3 bounds a stimulus-level critic's accuracy rather than its prediction; and the deeper trough after a flip beside the slower reversals. It is named and not taken.

### The evidence

- **The dispatch's scope.** The diff changes files under `src/` — the record, the rule, the executor, the image and the task — so by [ADR-0075](0075-the-dispatch-scope-follows-the-diff.md) the weekly was dispatched on this round's branch at **`scope=both`**, as brief 055 asks: run [36597280260](https://github.com/DescentVTT/VirtualCortex/actions/runs/36597280260) at `492e409` (`9473933` on `main`), the pinned tables, the last commit that changes a test or a source file. The round's commits as `main` holds them, beside the branch's: `8eed216` (`9e04f87`) the critic built; `26e9484` (`b589eb7`) the protocol, before any run; `9473933` (`492e409`) the arms' tables and the gate's checks over them; `9704403` (`7ebc4bd`) this ADR's readings and the documents; `0ba8368` (`ba73c90`) this section, the cost table and the brief's archive.
- **Every exhaustive job is green.** All seventy-five `exhaustive` tests passed, each exit 0: the seventy-three before this round reproduced their pinned numbers on the hosted runners, the whole-image pins among them at format 19, and H-22's two arms reproduced their tables there as they did twice on the developer machine. The arms took **1 379 s from the assignment and 1 814 s from the mirrored**; H-21's took 1 800 and 1 804 s in the same run.
- **The shards**, dealt by the table before this round, which did not know H-22's arms and costed each at 900 s:

  | Shard | Job | Its two heaviest tests (s) | Tests | Their seconds summed | Tests' wall time |
  | ---: | ---: | :--- | ---: | ---: | ---: |
  | 0 | 44.4 min | H-22 from the mirrored 1 814; H-21 from the mirrored 1 804 | 12 | 5 200 | 2 601 s, 36 % |
  | 1 | 38.3 min | H-22 from the assignment 1 379; H-20 from the assignment 1 379 | 12 | 4 272 | 2 248 s, 31 % |
  | 2 | 40.8 min | H-21 from the assignment 1 800; H-18 from the mirrored 1 132 | 11 | 4 767 | 2 392 s, 33 % |
  | 3 | 40.2 min | H-20 from the mirrored 1 978; H-18 from the assignment 1 137 | 13 | 4 653 | 2 353 s, 33 % |
  | 4 | 40.0 min | H-15 1 126; H-16 1 116 | 14 | 4 682 | 2 343 s, 33 % |
  | 5 | 37.1 min | H-19 from the assignment 1 164; H-19 from the mirrored 1 146 | 13 | 4 306 | 2 163 s, 30 % |

  The percentages are of the job's bound, 120 minutes; every shard ran its tests two at a time, at 1.90 to 2.00 of their summed seconds over the wall.
- **The cost table is regenerated from this run** (`node scripts/exhaustive-costs.mjs from <artifacts> --run 36597280260`): 75 lines, 27 880 s, the seventy-three earlier tests at 1.044 of the table before. ADR-0092's deal plans each of the six shards at 4 646 to 4 648 s summed, about 2 350 s of wall time at the run's ratio, **about 33 per cent of the bound**, inside the brief's 60.
- **The mutation sweep.** Seven jobs, green: 3 578 mutants caught over the tree and **none missed**, 2 398 in the state crates and 1 180 in the runtime, 150 unviable. Every mutant the sweep made in the critic's code was caught — `ValueCritic`'s sixteen, `Executor::{critic, features, prediction, reward}`, the count in `merge_spikes`, the pending unit's clause in `sweep` and every one of `critic_of`'s. Its 22 timeouts are the known protocol ones, in the iterators of `cortex-core`, the injector, the barrier and the workers' loop, none in the critic's code, read by [ADR-0062](0062-the-first-complete-sweeps-list.md)'s triage as detections. The runtime's six shards took 1 h 37 min to 2 h 58 min against their bound of 330 minutes.
- **The pull request's gate** on `7ebc4bd` (`9704403`) and on `ba73c90` (`0ba8368`) is green in every job, the mutation gate on the changed lines among them: 66 mutants in the lines the round changes, 58 caught, 8 unviable, none missed, as on `b589eb7`.

## Consequences

- Good: the shift and the scale are read off an arithmetic committed before the first rewarded run, against the task critic's step the comparison is with, so a difference from H-21 is the critic's and not a tuning.
- Good: every input the critic reads is the engine's own; the harness holds the engine's value, error and weights to a second writing of the rule at every trial.
- Good: the engine's learning no longer needs a label only the host has: with its own critic it learned every mapping of H-21's schedule in both arms with its couplings bounded, and its value came to what the task's critic held, a stimulus's expected reward.
- Neutral: the whole run is pinned, 120 blocks per arm, as H-21's.
- Bad: H-22 is no. Clause 3 failed in one mapping of eight, by 353 of 65 536, and the reading beside it says why a critic of a stimulus's expected reward meets it only where a mapping is learned at three quarters (F-61), which the clause's text did not say; the verdict is read by the clause as written, and the reading does not change it.
- Bad: two of the three reversals were slower than H-21's by 7 to 11 blocks, and the account of why is a Hypothesis the round did not measure.
- Bad: one seed, one size, one schedule, one pair of constants, as H-21's.

## Confirmation

- `runtime/cortex-runtime/tests/instrument/harness.rs`: `value_step`, `earned_run_valued`, which `earned_run_observed` calls.
- `runtime/cortex-runtime/tests/inhibition.rs`: `VALUED_ARMS`, `VALUED_PREDICTED`, `VALUED_CRITIC`, `VOLLEY_UNITS`, `TASK_STEP_SHIFT`, `PREDICTS_DIVISOR`, `CRITIC_AT`, `GROUPS`, `correct_errors`, `mapping_errors`, `predicts`, `Valued`, `valued`, `groups_of`, `weights_by_group`, `value_blocks`, `values_hash`, `with_critic_bytes`, `valued_image`, `only_the_critic`, `h21_first_block`, `valued_run`, `valued_arm`, `TARGET_IMAGE_CRC_1024`; the two weekly tests and the gate's test named above.
- `runtime/cortex-runtime/tests/inhibition.rs`: the pinned tables `VALUED_BLOCKS_1024` to `SUMS_AFTER_VALUED_1024` and the verdict `VALUED_1024` (`9473933` on `main`; `492e409` on the branch).
- Every pinned number of ADR-0065 to ADR-0129 unchanged, and the determinism pin; the image format 19; finding F-61 in whitepaper §11.
