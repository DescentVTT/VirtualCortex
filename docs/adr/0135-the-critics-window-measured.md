---
status: accepted
date: 2026-09-30
depends-on: ADR-0134
decision-makers: VirtualCortex maintainers
---

# ADR-0135: The critic's window, measured — H-23 is yes: with the engine's critic counting only within 100 ticks after each reward, the shortest delay of any synapse the image carries, read by its rule and committed before any run with H-22's shift 9 and scale 2, H-22's schedule learned every mapping in both arms, 115 to 124 of each mapping's last 128, with no coupling past 1.30 of its image's, the highest 1.200; each of the three reversals passed 40 of 64 in 14 to 20 blocks, within the 23 the task's critic let H-20 and H-21 take and against H-22's 10 to 30; and each stimulus's mean value over each mapping's last 128 trials stood within 0.12 of the reward of $2p - 1$, the quarter clause 4 allows; the prediction written first held; the window admitted the presented stimulus's volley and about 1.6 background spikes a trial, the value came to rest on each stimulus's own units, and after each flip it fell to −0.58 to −0.91 of the reward and came back as the task critic's did, to the block in eight cases of twelve and within four in the rest; the inhibitory course H-22's; the assertion held; H-23's stopping rule at step 3, and the next decision named and not taken

## Context and Problem Statement

[ADR-0133](0133-the-critics-window.md) took the critic's window and wrote **H-23** before any run: on H-22's configuration — H-21's configuration, schedule and arms with the engine's critic at a step shift of 9 and a scale of 2 ([ADR-0132](0132-a-critic-of-the-engines-own-measured.md)) — with the critic's window set by its rule, (1) every mapping is learned, (2) no coupling passes 1.30 of its image's at any block's end, (3) each of the three reversals passes 40 of 64 within 23 blocks of its flip, and (4) for each stimulus and mapping over its last 128 trials, the engine's mean value on that stimulus's trials lies within a quarter of the reward of $(2p - 1)$ of it, in both arms. Its prediction: clause 3 holds. [ADR-0134](0134-the-critics-window-built.md) built the window. Brief 056 runs H-23 once. This ADR is the round's protocol, committed before any rewarded run, and then its readings.

What was read before anything was written (principle 2, and brief 056's directive that the engine is read before a description of it is trusted, the brief's and ADR-0133's included):

1. **H-22** (`runtime/cortex-runtime/tests/inhibition.rs`): `valued_arm` builds ADR-0077's settled engine, H-20's signed image and H-21's image (`targeted_image`), writes the critic into it (`valued_image`) and runs `earned_run_valued` with no task critic and H-20's flips. Its tables are pinned whole, 120 blocks an arm, the engine's value per block and stimulus (`VALUED_VALUES_1024`) and the value weights by group (`VALUED_WEIGHTS_1024`) among them. The image it decodes, H-22's image, was dumped and not pinned: `0xbf6797857fda0809` at format 19, the CRC ADR-0132 read.
2. **The reward and the window's opening** (`tests/instrument/harness.rs`, `runtime/cortex-runtime/src/task.rs`): `run_on_scheduled` drives a lead-in of 500 ticks (`LEAD_IN`) and then trials of $2^{14}$ ticks that abut; the task rewards at a trial's end and injects the next stimulus before the next trial's first tick. So the engine's first window opens at its load and holds the lead-in's first ticks, and every later window opens at a trial's first tick.
3. **The volley's ticks** (`VALUED_CENSUS_1024`, the composer's census): over H-22's 7 680 trials an arm, the presented units' first spikes within 100 ticks of the trial's start — 390 415 of them from the assignment, about 51 a trial — fell 0 to 72 ticks after it, 99.8 per cent of them 1 to 6 ticks after it, and ADR-0076's cancel keeps each unit from firing again. A window of 100 ticks holds the whole volley.
4. **The delays** of the reference prior (`prior` of the harness, [ADR-0044](0044-reference-network.md)): the local band 100 to 300 ticks, the far band from 1 400. No rule of the engine moves a delay, so every image of the instrument's network carries the synthesised delays, and `shortest_delay` reads 100 ticks over the network the gate builds without running it.
5. **The settled network's rate** under H-21's target: 1.718 Hz a unit, a spike every 58 201 ticks (`TARGET_PERIOD_1024`, [ADR-0129](0129-a-target-the-network-fires-at-measured.md)).

## Decision Drivers

- Brief 056's standing directives:
  - the one mechanism ADR-0133's, built by ADR-0134, and nothing else of the engine changed;
  - the window opening at the engine's own reward and its length a rule of the image's own synapses, never the task's timing;
  - H-22's shift 9 and scale 2 kept, ADR-0132's arithmetic restated for the window's features before any run, and the round stopped before any rewarded run if the constants no longer resolve the rule;
  - H-23's clauses, constants and the window's rule and length committed before the first rewarded run and not moved after it;
  - no second attempt; no float; every loop ends by construction; the gate grows by at most one test; no shard of the weekly job past 60 per cent of its bound; no pinned number of an earlier round moves but the whole-image pins ADR-0134 restated.
- Latest ≠ Newest: no dependency, no tool.

## Considered Options

1. **Where the window's length is read**: (a) by `shortest_delay` over the arena the loader fills from H-22's image, in each arm before its run, and over the instrument's network in the gate; (b) from the prior's constants.
2. **How the window reaches the arms' engine**: (a) H-22's image with the window's two bytes written and the section re-sealed, as H-22 wrote its critic; (b) the configuration's window at decode.
3. **The calibration's reproduction of H-22**: (a) H-22's two weekly tests unchanged, and in each H-23 arm H-22's first block from H-22's image with the window unset, held to H-22's tables; (b) H-22's two weekly tests alone.
4. **What is pinned**: the whole run of each arm, the engine's value per block and every trial's by a hash, and the spikes the window admits per block by group.

## Decision Outcome

Options 1(a), 2(a), 3(a) and 4. Everything below is committed before any rewarded run.

### The window's length (option 1(a))

- **The rule**: `shortest_delay`, ADR-0134's, over the arena the loader fills from H-22's image; the loader holds the image's window to the same rule.
- **The length it reads**, pinned as `WINDOW_TICKS_1024`: **100 ticks**, the reference prior's local band's floor. The gate reads it over the instrument's network, whose delays every image of it carries, and finds every delay in the local band or the far one; each arm reads it again from H-22's image before its run and holds it to the pin.
- The prior's constants (option 1(b)) would state what the prior draws, not what the image carries.

### ADR-0132's arithmetic, restated for the window's features

ADR-0132 placed the shift and the scale so that one stimulus volley moves the value of the same volley by $51 / 2^{k+s} = 51/2\,048$ of the error, 0.80 of the task critic's thirty-second. The window changes the features, not the rule:

- **The volley.** The whole volley falls inside the window (Context, 3): its 51 spikes are counted as in H-22, and each presentation of the stimulus moves the value of its next presentation by $51/2\,048$ of the error, as in H-22. **The two inequalities stand**: $51 \times 32 \le 2^{11}$ and $51 \times 32 > 2^{10}$.
- **The background the window admits.** 1 024 units over 100 ticks at a spike every 58 201 ticks a unit: $102\,400 / 58\,201 = 1.76$ spikes a trial, beside the volley's 51. The step then moves its own trial's value by about $(51 + 1.76) / 2\,048$, 0.026 of the error, where H-22's 351 spikes moved it by about $431/2\,048$, 0.21. No readout and no inhibitory unit's response to the volley can come within the window, since the shortest delay is its length.
- **The dead band and the floor.** A positive error below $2^9 = 512$, 0.78 per cent of the reward, moves no weight, as in H-22. A negative error moves each unit that fired at least one LSB down: the window's 53 spikes lower the value by $53/4$, floored to 14 of 65 536 a trial, against H-22's 88.
- **The rail.** A weight at `i16::MAX` predicts 8 191 a spike, an eighth of the reward; the volley carries the whole reward at 5 141 a unit, 0.157 of the rail, as in H-22.
- **So the constants resolve the rule**: the volley's step is H-22's, the dead band is H-22's, and the background's share of the step falls from about seven times the volley's to a thirtieth of it. The round goes on to the calibration.

The gate holds each number of this section over numbers written by hand, and the two inequalities as compile-time assertions.

### The image each arm decodes (option 2(a))

- **`with_window_bytes`** writes the window into H-22's image — the modulator section's `[52..54)` — and re-seals the section. H-22's image is `valued_image` of H-21's at `VALUED_CRITIC`, pinned by its CRC-64 at format 20, `VALUED_IMAGE_CRC_1024` (`0xeae8ad819564a88a`), and at 19, `VALUED_IMAGE_CRC_FORMAT_19_1024` (`0xbf6797857fda0809`, the CRC ADR-0132 read), by the header's version written back.
- **`only_the_window`** is the masked check: H-22's image carries no window; every position that differs lies in the window's two bytes or in the modulator's entry in the section table, which holds the seal; and zeros written back give H-22's image bit for bit.
- **The image each arm decodes** is pinned by its CRC-64, `WINDOWED_IMAGE_CRC_1024`.
- **Each arm asserts before its run**: the decoded engine's critic, window, target and opening at the load; every weight and every count zero; the sums and the four couplings the image's.
- The configuration (option 2(b)) would change nothing: the image's window, set or unset, outranks it.

### H-23, restated as integer rules (ADR-0133's clauses and constants)

- **Clauses 1 and 2**: H-20's `scheduled`, unchanged — each mapping's last 128 trials at least 80 correct, and no coupling above 1.30 of its image's at any block's end.
- **Clause 3, the engine revises** (`reversals_within`, `REVERSAL_BLOCKS_MAX`): for each of the second, third and fourth mappings, `crossings` — the blocks from the mapping's first to the first with at least 40 correct, that block counted — is at most 23. A reversal that never crosses does not hold. 23 is the slowest reversal H-20 and H-21 read, H-21's first from the mirrored assignment; the gate computes it from their pinned tables.
- **Clause 4, the critic holds a stimulus's expected reward** (`holds_expected`, `stimulus_values`, `HOLDS_DIVISOR`): over each mapping's last 128 trials — 1 409 to 1 536, 3 457 to 3 584, 5 505 to 5 632 and 7 553 to 7 680 — and for each stimulus, its trials $n$, its correct trials $c$ and the engine's values at their rewards summed, $V$; the clause holds when $n > 0$ and $4\,\lvert V - (2c - n)\,r \rvert \le r\,n$, with $r$ the reward's magnitude, 1.0.
- **The verdict** (`Windowed`): `learning` (H-20's `Scheduled`), `revised` per arm and reversal, `held` per arm, mapping and stimulus, and `yes` when all hold in both arms. The prediction, `REVERSALS_PREDICTED`, is that clause 3 holds; it is dumped beside the reading and never asserted.

### The assertion, beside the verdict

H-20's to H-22's (`punished_held`): no excitatory synapse outside the four stimulus–readout pairs moves from the image to the run's end. The oracle is held to the record's weights, traces and signal at every trial inside the harness, and the engine's value, error, weights and window's opening to the harness's critic. A false assertion is a finding, reported beside the verdict.

### The harness

`earned_run_valued` (ADR-0134) applies the engine's window when it is set: its oracle counts only the train's spikes fewer ticks than the window after the opening, asserts that the run starts where the engine did with nothing pending, and holds the engine's opening to the reward's tick at every reward. With the window unset it counts as it did for H-22.

### The readings, no clause (ADR-0133's)

- **The window's length** as the rule read it, and **the spikes it admits per trial**, by block, presented stimulus and group (`windowed_run`, `admitted_blocks`), from the train at every trial's reward.
- **The engine's value beside $2p - 1$, H-21's task critic and H-22's engine critic**: every trial's value by a hash, per block and stimulus its trials, values, correct trials and their errors (`value_blocks`), and per mapping and stimulus the counts of clause 4 (`stimulus_values`).
- **The trough after each flip** (`troughs`): per flip and stimulus the lowest block mean of the value over the mapping the flip put in force, beside H-22's by the same rule and H-21's task critic's expectation; and **the strong punishments per flip** (`strong_by_flip`) beside H-21's and H-22's.
- **The weights by group at every block's end** (`weights_by_group`).
- **The reversal speeds** (`crossings`, `crossed_scheduled`, `first_new_scheduled`) beside H-20's, H-21's and H-22's.
- **The inhibitory sum's course**, each block's as a fraction of the settled image's, beside H-22's.
- **H-22's other readings** by H-20's rules: the tallies, the settle measure, the highest coupling per mapping, the moves by mapping, the blocks in which the stimulus fired once, and the sums after.

### The calibration, before any rewarded run (H-23's stopping rule, step 2)

- **In the tree, with the window unset**: the workspace's tests in the debug and the release profile; every whole-domain test of the weekly job, H-22's two arms among them, each in a process of its own from the release build of this commit; the determinism pin with them.
- **In each arm's test**, before its first rewarded trial: the settled engine held to ADR-0077's tables step by step and H-20's image to its CRC (`signed_images`); a frozen block from the zero image held to ADR-0077's frozen run (`calibration_holds`); H-21's image by its CRC at format 20 and at 18; H-22's image by its CRC at 20 and at 19; and **H-22's first block from H-22's image with the window unset** (`h22_first_block`), 64 trials under the engine's critic held to H-22's pinned first block, table by table, the value and the weights by group among them (option 3(a)); then the window's length read and held to the pin, and the masked check.

A failure at any of them stops the round there as a finding.

### The gate

`the_window_s_rule_its_arithmetic_the_clauses_of_h_23_and_the_image_s_patch`, one test, no run:
- the constants, H-22's restated, and the images' CRCs; clause 3's bound computed from H-20's and H-21's pinned crossings;
- the window's rule over networks written by hand and over the instrument's network at 1 024 units;
- this ADR's arithmetic over numbers written by hand: the volley's step, the background the window admits, the step with it, the floor, the dead band and the rail;
- clause 3 at 23 and 24 blocks and a reversal that never crosses, and H-21's and H-22's pinned crossings read by it; clause 4 at a quarter of the reward either side and one LSB past, with no trial, and over trials written by hand across the four mappings; the verdict naming the clause, the arm, the mapping and the stimulus;
- the readings' rules over tables written by hand, and H-22's and H-21's troughs by the same rule;
- the patch on the instrument's network's image with the critic written: the positions it moves, the loader reading the window and opening it at the load, zeros written back giving the image, a byte moved elsewhere failing the check, three lengths the rule does not give refused by the loader, and a window without the critic refused.

### The weekly tests and their cost

- **Two weekly tests**, one per arm: `a_critic_with_a_window_from_the_assignment_at_1024_units_exhaustive` and `a_critic_with_a_window_from_the_mirrored_assignment_at_1024_units_exhaustive`.
- **Each is H-22's arm and a block more**: H-22's arms took 1 379 and 1 814 s on the hosted runners by the cost table, and H-22's first block adds a sixtieth of a run.
- **The plan stays inside the budget.** The table after ADR-0132 is 75 tests and 27 880 s, about 33 per cent of the bound at six shards. The two arms add about 3 300 s, and the plan rises to about 37 per cent, under the directive's 60.

### The order of the work, as the history holds it

1. **The window built** (`2d73307` on `main`; `6bf0411` on the branch before the rebase that merged pull request #168, 01:28Z on 2026-09-30): ADR-0134, its tests and the five whole-image pins restated, each read from a run of this round with its masked check passing — the frozen image H-20's arm leaves from a run of that arm that reproduced ADR-0110's sequence and sums. The workspace in the debug profile: 683 passed, 117 ignored.
2. **The protocol** (`1d3cd72`; `216e325`, 01:29Z, pushed to pull request #168, opened as a draft before any run at 01:29:58Z): the rules, the constants, the window's length, the arithmetic, the gate, the arms with empty pins, and this ADR's protocol.
3. **The calibration**, before any rewarded run, with the window unset, from the release build of `216e325` (`1d3cd72`): **every one of the 75 whole-domain tests of the weekly job passed**, each in a process of its own and each exit 0 with one test reported — 65 eight at a time from 01:32Z to 02:27Z, H-22's two arms among them in 1 649 and 1 646 s under that load, and the ten tests of ADR-0097 and ADR-0102 one at a time to 02:29Z; the workspace's tests passed in the release profile (683 passed, 117 ignored) and on the MSRV (683 passed); the pull request's checks on `216e325` (`1d3cd72`) passed, the determinism pin on AArch64 among them, and the mutation gate on the changed lines: 31 mutants, 27 caught, 4 unviable, none missed.
4. **The arms**, once, side by side from 02:36:33Z, the calibration held in each — H-22's first block reproduced table by table with the window unset — the window's rule reading 100 ticks from H-22's image and the masked check passing (nine bytes differ from H-22's image, one of the window's and eight of the seal; the image's CRC-64 `0x5044ed7936a7b5f3`); both ran their 7 680 trials and stopped at the first empty table at 02:58:04Z, 1 291 s. The tables were written from those dumps (`2ffbffd`; `35af0f5`), and a second run side by side from 03:00:25Z reproduced every table and passed, in 1 330 and 1 330 s (on the developer machine, a ratio and not admissible).

No constant, clause or rule moved after the first rewarded trial, and there was no second attempt: the second run is the pinned tables' reproduction.

### The readings

**The verdict** (`WINDOWED_1024`), by the rule committed first, over the pinned tables:

- **Clause 1 holds in both arms**: each mapping's last 128 trials, **120, 121, 120 and 115** correct from the assignment and **124, 116, 116 and 123** from the mirrored assignment, against the mark of 80 (H-22's: 121, 96, 120, 100 and 125, 89, 121, 113; H-21's: 121, 119, 115, 119 and 124, 113, 115, 119).
- **Clause 2 holds in both arms**: no coupling above 1.30 of its image's at any block's end. The highest per mapping, as a fraction of the image coupling:

  | Arm | First mapping | Second | Third | Fourth |
  | :--- | :--- | :--- | :--- | :--- |
  | Assignment first | 1.158, block 23, A→R0 | 1.136, block 55, A→R1 | 1.200, block 86, A→R0 | **1.200**, block 118, A→R1 |
  | Mirrored first | 1.164, block 23, A→R1 | 1.172, block 53, B→R1 | 1.180, block 87, B→R0 | **1.198**, block 118, B→R1 |

  H-22's highest were 1.186 and 1.185; H-21's 1.190 and 1.193.
- **Clause 3 holds in both arms**: each reversal passed 40 of 64 within 23 blocks of its flip (`CROSSINGS_WINDOWED_1024`, the crossing block counted):

  | Arm | First mapping | First reversal | Second | Third |
  | :--- | ---: | ---: | ---: | ---: |
  | **H-23 from the assignment** | 6 | **19** | **19** | **16** |
  | H-22 from the assignment | 6 | 30 | 15 | 27 |
  | H-21 from the assignment | 6 | 19 | 20 | 16 |
  | H-20 from the assignment | 6 | 19 | 21 | 16 |
  | **H-23 from the mirrored** | 4 | **20** | **15** | **14** |
  | H-22 from the mirrored | 4 | 30 | 10 | 22 |
  | H-21 from the mirrored | 3 | 23 | 14 | 14 |
  | H-20 from the mirrored | 3 | 23 | 13 | 18 |

- **Clause 4 holds in every mapping and stimulus of both arms**: over each mapping's last 128 trials, per stimulus, its accuracy $p$, the value $2p - 1$ of the reward a critic of the stimulus's expected reward would hold, and the engine's mean value (`STIMULUS_VALUES_WINDOWED_1024`):

  | Arm, mapping | A: $p$ | $2p-1$ | mean value | B: $p$ | $2p-1$ | mean value |
  | :--- | ---: | ---: | ---: | ---: | ---: | ---: |
  | Assignment, 1 | 0.934 | 0.869 | 0.886 | 0.940 | 0.881 | 0.898 |
  | Assignment, 2 | 0.984 | 0.969 | 0.880 | 0.906 | 0.812 | 0.813 |
  | Assignment, 3 | 0.968 | 0.935 | 0.918 | 0.909 | 0.818 | 0.815 |
  | Assignment, 4 | 1.000 | 1.000 | 0.980 | 0.824 | 0.649 | 0.627 |
  | Mirrored, 1 | 0.984 | 0.967 | 0.949 | 0.955 | 0.910 | 0.795 |
  | Mirrored, 2 | 0.844 | 0.688 | 0.751 | 0.969 | 0.938 | 0.954 |
  | Mirrored, 3 | 0.952 | 0.903 | 0.889 | 0.864 | 0.727 | 0.788 |
  | Mirrored, 4 | 0.944 | 0.889 | 0.887 | 0.973 | 0.946 | 0.935 |

  The largest distance from $2p - 1$ was 0.115 of the reward, for B in the mirrored arm's first mapping, against the quarter the clause allows.
- `Windowed { learning: Scheduled { learned: [[true; 4]; 2], bounded: [true; 2], over: [None; 2], yes: true }, revised: [[true; 3]; 2], held: [[[true; 2]; 4]; 2], yes: true }`. **H-23 is yes**, with its scope: this task, these two arms, this schedule of three flips over 7 680 trials, H-21's configuration at 1 024 units, the critic at a step of $2^{-9}$ and a scale of $2^{-2}$, and a window of 100 ticks.

**The account's prediction against the reading.** ADR-0133 predicted that clause 3 holds, the value resting on each stimulus's own units and falling after a flip as the task critic's did. It held in both arms, and the readings below bear out each part of the account.

**What the window admitted** (`WINDOWED_ADMITTED_1024`): about 52.4 spikes a trial in both arms — the presented stimulus's own units 50.8, its volley; the inhibitory units 0.40 to 0.41; the other stimulus's units 0.08 to 0.10; each readout set 0.54 to 0.58; the four others 0.007 — so about 1.6 background spikes beside the volley, against the 1.76 the arithmetic wrote first. Where H-22's features held some 300 background spikes a trial beside the volley, these hold under two.

**Where the value came to rest** (`WINDOWED_WEIGHTS_1024`): on each stimulus's own units. At the run's end the 51 units of A and of B summed 259 429 and 179 775 from the assignment and 243 399 and 245 641 from the mirrored assignment — a value of 0.99 and 0.69 of the reward for a volley of each, and 0.93 and 0.94 — while the 204 inhibitory units summed −432 and 13 397, each readout set −5 915 to 6 599, and the four others under 1 000. H-22's value sat about half on units both stimuli share (ADR-0132); here it sits on the stimulus the window saw.

**The value after each flip, beside H-22's and H-21's task critic** (`troughs`, the lowest block mean over the mapping the flip put in force, per stimulus, of the reward; for H-21 the task critic's expectation at a block's end):

| Arm, flip | H-23 A | H-23 B | H-22 A | H-22 B | H-21 A | H-21 B |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: |
| Assignment, 1 | −0.733 | −0.711 | −0.975 | −0.938 | −0.864 | −0.820 |
| Assignment, 2 | −0.754 | −0.746 | −0.739 | −0.749 | −0.841 | −0.782 |
| Assignment, 3 | −0.676 | −0.749 | −1.001 | −0.994 | −0.751 | −0.785 |
| Mirrored, 1 | −0.844 | −0.792 | −0.976 | −0.988 | −0.948 | −0.821 |
| Mirrored, 2 | −0.581 | −0.849 | −0.508 | −0.776 | −0.696 | −0.860 |
| Mirrored, 3 | −0.907 | −0.619 | −1.034 | −0.924 | −0.855 | −0.753 |

The block of the new mapping in which each stimulus's value, having fallen below zero, came back above it — H-23's, H-21's task critic's and H-22's, by the same rule:

| Arm | H-23, flips 1 / 2 / 3 (A, B) | H-21 | H-22 |
| :--- | :--- | :--- | :--- |
| Assignment | (19, 18) / (16, 20) / (14, 21) | (19, 18) / (16, 18) / (14, 20) | (30, 23) / (9, 13) / (25, 22) |
| Mirrored | (24, 17) / (11, 17) / (19, 12) | (24, 17) / (11, 14) / (15, 12) | (none in 32, 21) / (4, 12) / (26, 17) |

**The strong punishments per flip** (`STRONG_BY_FLIP_WINDOWED_1024`, the old answer selected and the signal at or below −0.5 after the reward, both stimuli): 543, 567 and 554 from the assignment and 509, 437 and 441 from the mirrored, beside H-21's 527, 573 and 496 and 490, 426 and 432, and H-22's 383, 383 and 425 and 328, 262 and 381. With the value no longer sinking below the task critic's after a flip, a wrong selection again delivers the punishment that revises the old answer under the signed gate, as many as H-21's.

**The inhibitory sum's course**, each block's as a fraction of the settled image's: it rose to 1.0109 and 1.0101 at the 14th and the 11th block and ended at **0.849 and 0.848**, against H-22's 0.849 and 0.846 and H-21's 0.848 and 0.848, still falling.

**The assertion held** in both arms: no excitatory synapse outside the four stimulus–readout pairs moved (`REACH_WINDOWED_1024`: 3 186 and 3 187 synapses moved inside them, none outside). The oracle — the harness's second writing of the critic's rule and its window among it — was held to the record's value, error, weights, window's opening, counts, traces and signal at every one of the 15 360 trials.

### The evidence

- **The dispatch's scope.** The diff changes files under `src/` — the executor, the image and the connectome's version — so by [ADR-0075](0075-the-dispatch-scope-follows-the-diff.md) the weekly was dispatched on this round's branch at **`scope=both`**, as brief 056 asks: run [36664142217](https://github.com/DescentVTT/VirtualCortex/actions/runs/36664142217) at `35af0f5` (`2ffbffd` on `main`), the pinned tables, the last commit that changes a test or a source file. The round's commits as `main` holds them, beside the branch's: `2d73307` (`6bf0411`) the window built; `1d3cd72` (`216e325`) the protocol, before any run; `2ffbffd` (`35af0f5`) the arms' tables and the gate's checks over them; `668cdd0` (`9344d7a`) this ADR's readings and the documents; `831d433` (`6e11b90`) this section, the cost table and the brief's archive.
- **Every exhaustive job is green.** All seventy-seven `exhaustive` tests passed, each exit 0 with one test reported: the seventy-five before this round reproduced their pinned numbers on the hosted runners, the whole-image pins among them at format 20, and H-23's two arms reproduced their tables there as they did on the developer machine. The arms took **1 362 s from the assignment and 1 845 s from the mirrored**; H-22's took 1 356 and 1 003 s in the same run.
- **The shards**, dealt by the table before this round, which did not know H-23's arms and costed each at 900 s:

  | Shard | Job | Its two heaviest tests (s) | Tests | Their seconds summed | Tests' wall time |
  | ---: | ---: | :--- | ---: | ---: | ---: |
  | 0 | 34.6 min | H-20 from the mirrored 1 612; H-16 923 | 14 | 3 934 | 2 025 s, 28 % |
  | 1 | 24.8 min | H-22 from the mirrored 1 003; H-14 643 | 12 | 2 858 | 1 446 s, 20 % |
  | 2 | 49.4 min | H-23 from the mirrored 1 845; H-21 from the mirrored 1 782 | 13 | 5 688 | 2 901 s, 40 % |
  | 3 | 41.9 min | H-21 from the assignment 1 763; H-18 from the assignment 1 101 | 13 | 4 870 | 2 448 s, 34 % |
  | 4 | 45.7 min | H-23 from the assignment 1 362; H-22 from the assignment 1 356 | 12 | 5 337 | 2 696 s, 37 % |
  | 5 | 54.1 min | H-20 from the assignment 2 345; H-19 from the mirrored 1 127 | 13 | 6 371 | 3 190 s, 44 % |

  The percentages are of the job's bound, 120 minutes; every shard ran its tests two at a time, at 1.94 to 2.00 of their summed seconds over the wall.
- **The cost table is regenerated from this run** (`node scripts/exhaustive-costs.mjs from <artifacts> --run 36664142217`): 77 lines, 29 058 s, the seventy-five earlier tests at 0.927 of the table before. ADR-0092's deal plans each of the six shards at 4 842 to 4 844 s summed, about 2 460 s of wall time at the run's ratio, **about 34 per cent of the bound**, inside the brief's 60.
- **The mutation sweep.** Seven jobs, green: 3 595 mutants caught over the tree and **none missed**, 2 398 in the state crates and 1 197 in the runtime, 150 unviable. Every mutant the sweep made in the window's code was caught — `in_window`'s seven, `Executor::{critic_window_ticks, window_opened, resume_clock}`, `critic_window_of`, `shortest_delay`'s three and the field the loader passes. Of its 24 timeouts, 23 are the known protocol ones, in the iterators of `cortex-core`, the injector, the barrier, the workers' loop and `stop_workers`; the one besides, `delete !` in the loader's check of a term node (`image.rs:1017`), is in code the round did not change and not in the window's: it ran beside an injector mutant that timed out, while the instrument binary, whose images carry no term, was still running, and by [ADR-0063](0063-the-sweep-reads-its-own-timeouts.md)'s evidence the run had stopped finishing tests. By [ADR-0062](0062-the-first-complete-sweeps-list.md)'s triage the timeouts are detections. The runtime's six shards took 1 h 51 min to 3 h 2 min against their bound of 330 minutes.
- **The pull request's gate** on `216e325` (`1d3cd72`) and on `6e11b90` (`831d433`) is green in every job, the mutation gate on the changed lines among them: 31 mutants in the lines the round changes, 27 caught, 4 unviable, none missed.

### The step of the stopping rule reached

**Step 3, yes**: *"the engine's own critic is named as one that learns, revises as fast as the task's critic let it, and holds a stimulus's expected reward. The next decision is an ADR choosing among the operating regime, another size, the rule held by the network reopened on this configuration, and a critic carried by a population (ADR-0130's option 1(c)), named and not taken."* The configuration the engine learns, revises and predicts in is named with the window: H-21's configuration — every excitatory synapse under the reward's gate with the signed gate set, every inhibitory one under a baseline of its own, the inhibitory rule's target at the settled network's rate — with the engine's critic at a step of $2^{-9}$ and a scale of $2^{-2}$ counting only within the shortest synaptic delay after each reward. **The next decision is an ADR choosing among the operating regime, another size, the rule held by the network reopened on this configuration, and a critic carried by a population.** It is named and not taken.

## Consequences

- Good: the window's length is read by the rule the loader holds the image to, over the image the arms decode, and pinned before the first rewarded run, so it is the anatomy's and not a tuning.
- Good: every input the critic reads is the engine's own; the harness holds the engine's value, error, weights and window to a second writing of the rule at every trial.
- Good: only the window moves from H-22's configuration, so a difference from H-22 is the window's.
- Good: the engine's critic, reading only its own spikes and rewards, now does what the task's critic did in H-21: it learns a stimulus's expected reward, falls after a flip no deeper than the task critic's, and lets the reversals run at H-21's speed.
- Neutral: the whole run is pinned, 120 blocks per arm, as H-22's.
- Bad: one seed, one size, one schedule, one window length, as H-22's; the window's fit rests on the task presenting its stimulus at the reward, and a host that presents it later gives the critic the background alone (ADR-0133).

## Confirmation

- `runtime/cortex-runtime/tests/instrument/harness.rs`: `earned_run_valued`.
- `runtime/cortex-runtime/tests/inhibition.rs`: the pinned tables `WINDOWED_BLOCKS_1024` to `SUMS_AFTER_WINDOWED_1024` and the verdict `WINDOWED_1024`.
- `runtime/cortex-runtime/tests/inhibition.rs`: `WINDOWED_ARMS`, `REVERSALS_PREDICTED`, `WINDOW_TICKS_1024`, `REVERSAL_BLOCKS_MAX`, `HOLDS_DIVISOR`, `WINDOW_AT`, `TARGET_TICKS_PER_SPIKE`, `reversals_within`, `stimulus_values`, `holds_expected`, `Windowed`, `windowed`, `admitted_blocks`, `troughs`, `value_means`, `expected_means`, `with_window_bytes`, `only_the_window`, `h22_first_block`, `windowed_run`, `windowed_arm`, `VALUED_IMAGE_CRC_1024`, `VALUED_IMAGE_CRC_FORMAT_19_1024`, `WINDOWED_IMAGE_CRC_1024`; the two weekly tests and the gate's test named above.
- Every pinned number of ADR-0065 to ADR-0132 unchanged but the whole-image pins ADR-0134 restated, and the determinism pin; the image format 20.
