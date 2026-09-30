---
status: proposed
date: 2026-09-30
depends-on: ADR-0134
decision-makers: VirtualCortex maintainers
---

# ADR-0135: The critic's window, measured — brief 056's protocol, committed before any rewarded run: the window's length read by its rule from H-21's image, the shortest delay of any synapse the image carries, 100 ticks; ADR-0132's arithmetic restated for the window's features, the volley's step unchanged and the background the window admits about two spikes; H-22's shift 9 and scale 2 kept; the image each arm decodes, H-22's with the window written and nothing else, shown by a masked check; H-22's first block reproduced with the window unset before it; H-22's two arms run once on it; H-23's four clauses as integer rules

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

### The order of the work

1. The window built, ADR-0134, and its tests, with the five whole-image pins restated.
2. This commit: the rules, the constants, the window's length, the arithmetic, the gate, the arms with empty pins, and this ADR's protocol, before any run.
3. The calibration above, before any rewarded run.
4. The two arms, once, stopping at their first empty pin; the tables written from the dumps, and a second run to reproduce them.
5. This ADR's readings and the documents; then the weekly dispatched on the branch at `scope=both` and the cost table regenerated from it.

## Consequences

- Good: the window's length is read by the rule the loader holds the image to, over the image the arms decode, and pinned before the first rewarded run, so it is the anatomy's and not a tuning.
- Good: every input the critic reads is the engine's own; the harness holds the engine's value, error, weights and window to a second writing of the rule at every trial.
- Good: only the window moves from H-22's configuration, so a difference from H-22 is the window's.
- Neutral: the whole run is pinned, 120 blocks per arm, as H-22's.
- Bad: one seed, one size, one schedule, one window length, as H-22's.

## Confirmation

- `runtime/cortex-runtime/tests/instrument/harness.rs`: `earned_run_valued`.
- `runtime/cortex-runtime/tests/inhibition.rs`: `WINDOWED_ARMS`, `REVERSALS_PREDICTED`, `WINDOW_TICKS_1024`, `REVERSAL_BLOCKS_MAX`, `HOLDS_DIVISOR`, `WINDOW_AT`, `TARGET_TICKS_PER_SPIKE`, `reversals_within`, `stimulus_values`, `holds_expected`, `Windowed`, `windowed`, `admitted_blocks`, `troughs`, `value_means`, `expected_means`, `with_window_bytes`, `only_the_window`, `h22_first_block`, `windowed_run`, `windowed_arm`, `VALUED_IMAGE_CRC_1024`, `VALUED_IMAGE_CRC_FORMAT_19_1024`, `WINDOWED_IMAGE_CRC_1024`; the two weekly tests and the gate's test named above.
- Every pinned number of ADR-0065 to ADR-0132 unchanged, and the determinism pin; the image format 20.
