---
status: proposed
date: 2026-09-29
depends-on: ADR-0128
decision-makers: VirtualCortex maintainers
---

# ADR-0129: A target the network fires at, measured — H-21 is yes: ADR-0128's rule, committed before it ran, read the settled network at 18 449 spikes over $2^{20}$ ticks, 1.718 Hz a unit, and gave a target period of 58 201 ticks, where the inhibitory rule's depression per spike is 23 against the default's 67; with it written into H-20's image and nothing else, H-20's schedule kept the network's inhibitory sum at or above 0.848 of the settled image's at every block's end in both arms and learned every mapping, 113 to 124 of each mapping's last 128, no coupling past 1.30 of its image's, the highest 1.190 and 1.193; the sum rose to 1.01 of the image's over the first mapping and then fell, and was still falling at the run's end by about 0.15 per cent of the image's a block, so the drain slowed rather than stopped; the inhibition onto the stimulus units, which fire at 4.5 Hz, rose to the rail by the second mapping, while onto the inhibitory units and the readouts, at 2.2 and 1.75 Hz, it fell to 0.65 and 0.82 of the image's; the settled image's inhibition was already below the rail, 45 of its 6 528 synapses at it (F-59), and the rule moved it both ways, 833 and 856 at the end; the reversal speeds H-20's within a block but for one; the assertion held; H-21's stopping rule at step 3, and the next decision named and not taken

## Context and Problem Statement

[ADR-0128](0128-a-target-the-network-fires-at.md) took the inhibitory drain among H-20's four open questions and wrote **H-21** before any run: on H-20's configuration, schedule and arms, with the inhibitory rule's target at the settled network's own rate, (1) the network's inhibitory sum stays at or above half of the settled image's at every block's end and (2) every mapping is learned with no coupling past 1.30 of its image's, in both arms. Brief 054 runs it once and changes no rule. This ADR is the round's protocol, committed before any rewarded run, and then its readings.

What was read before anything was written (principle 2, and brief 054's directive that the engine is read before a description of it is trusted, the brief's and ADR-0128's included):

1. **The rule** (`crates/cortex-core/src/dynamics/synapse.rs`). On an inhibitory block `step_stdp` takes `istdp_alpha_q1_15` from the slot's trace at every presynaptic spike and adds $A_+ (1 - 2^{-11})^{|\Delta t|}$ for each of the two nearest pairings, whichever way round. `istdp_alpha_q1_15(period)` is $2 A_+ 2^{11} / \text{period} = 1\,343\,488 / \text{period}$, truncated, the period clamped to `[ISTDP_PERIOD_MIN_TICKS, ISTDP_PERIOD_MAX_TICKS]`, 100 to 1 000 000 ticks: 67 at the default 20 000. `consolidate` moves the rounded share of the trace the modulation names into the weight's magnitude, clamped to $[0, 2^{15} - 1]$, and takes out of the trace only what the magnitude absorbed. A magnitude at `i16::MAX`, the rail, therefore keeps a positive trace in the trace, where it decays.
2. **The target in the executor and the image** (`runtime/cortex-runtime/src/executor.rs`, `image.rs`). The modulator section's `[20..24)` holds the period, little-endian. The loader reads it with `set_istdp_target_period`, which refuses a period outside the bounds (`ConfigError::IstdpPeriodOutOfRange`) and publishes the depression to the workers at once. The image's period outranks the configuration's (§8.3). The harness never sets one, so ADR-0077's settled image carries the default, 20 000, and so does every image built from it.
3. **H-20** (`runtime/cortex-runtime/tests/inhibition.rs`): `signed_images` builds ADR-0077's settled engine step by step and H-18's signed image, whose CRC-64 is `PUNISHED_IMAGE_CRC_1024`; `schedule_run` runs `earned_run_scheduled` with the flips before the trials of index 1 536, 3 584 and 5 632 (`SCHEDULE_FLIPS`) and the critic of shift 5 (`CRITIC_AT_START`); `scheduled` is its criterion; its readings are the rules around them. H-20 pinned the 64 blocks after its second flip and held the first 56 to H-19's tables.
4. **The inhibitory baseline**, 0.5 in H-20's image ([ADR-0086](0086-the-inhibitory-baseline-built.md)): every inhibitory synapse consolidates half its trace at each of its presynaptic spikes, whatever the reward.
5. **The rail, and the settled image (F-59).** The prior writes every inhibitory weight at $-32\,767$, one LSB above $-1.0$: its inhibitory sum, `PRIOR_SUMS_1024`, is 213 902 976, which is 6 528 synapses at the rail. **The settled image is not the prior.** ADR-0077's lead-in ran 96 windows of $2^{17}$ ticks under the modulation baseline 0.5 with the inhibitory rule at the default target, and the image it left carries an inhibitory sum of 165 876 268 (`QUIET_1024`), 0.775 of the prior's. Every learning run from H-15 to H-20 started from that image, and H-21 does too. So the rule has already weakened a quarter of the inhibition, and a synapse below the rail onto a unit that fires above the new target can be potentiated back toward the rail. ADR-0128 reads the start as the rail: *"a unit at or above that rate keeps its inhibition, since the rail allows no more"*, its first *Bad* consequence, and its stopping rule's step 4, which names *"inhibition below the rail"* as the change after a no. How many of the settled image's synapses are at the rail is read by this round.
6. **ADR-0057's rule** (`spikes_and_at_target` in the harness): a unit is at the target when its spikes in a window of $2^{17}$ ticks are within a factor of two of $2^{17} / \text{period}$, inclusive both ways — six at the default period, so 3 to 12; two at a period near 59 000, so 1 to 4.
7. **The train and the trial.**
   - `Executor::train` takes `&mut self`, since it makes the ring contiguous, while `earned_run_scheduled` hands its `after` a shared `&Engine`.
   - A trial is `TRIAL_TICKS`, $2^{14}$ ticks. So a block of 64 trials is $2^{20}$ ticks, the length of the target's lead-in, and ADR-0057's window is eight trials.
   - The train holds the most spikes a trial can produce (`spikes_per_unit(TRIAL_TICKS)` a unit), so a trial read from it at its end is whole.
8. **The geometry at 1 024 units** tiles every period of twenty with the four sets (A at place 0, B at 11, the readouts at the other eighteen). With every fifth unit inhibitory, the arena holds 204 inhibitory units, 102 excitatory units of the stimulus sets, 714 of the readout sets and four past the last whole period.

## Decision Drivers

- Brief 054's standing directives:
  - no rule of the engine changes; the target period is a parameter of the image (ADR-0053);
  - one change from H-20, the target period, every other byte of the image each arm decodes H-20's, shown by a masked check;
  - the target read by ADR-0128's rule and not chosen, the rule committed before it is computed;
  - H-21's clauses and constants ADR-0128's, committed before the first rewarded run;
  - no constant moves after a rewarded run, and no second attempt;
  - no float; every loop ends by construction; the gate grows by at most one test; no shard of the weekly job past 60 per cent of its bound; no pinned number of an earlier round moves.
- **The engine read before a description of it is trusted**: item 5 above is F-59.
- Latest ≠ Newest: no dependency, no tool, no rule.

## Considered Options

1. **How the target reaches the arms' engine**: (a) H-20's image with the four bytes of the period written and the section re-sealed, as H-16's and H-18's images wrote their flags; (b) the configuration's `istdp_target_period_ticks` at decode; (c) a settled engine run again with the target from its first tick.
2. **The masked check**: (a) every position at which the two images differ lies in the period's four bytes or the modulator's entry in the section table, and writing the old period back gives H-20's image bit for bit; (b) a count of differing bytes alone.
3. **The lead-in**: (a) on ADR-0077's zero image, the frozen form of the settled image, from the tick it was written at; (b) on the arms' image.
4. **How a run's train is read**: (a) a sibling of `earned_run_scheduled` whose `after` is handed the executor mutably, which `earned_run_scheduled` calls with a shared view; (b) `after`'s type changed at every caller; (c) the population's spikes added to the harness's `Block`.
5. **What is pinned**: (a) the whole run of each arm; (b) the blocks and a hash of the rest.
6. **Readings beyond ADR-0128's**: the inhibition by class and the synapses at the rail per block; the spikes by class per block; none.
7. **How many ADRs**: one; a protocol ADR and a readings ADR.

## Decision Outcome

Options 1(a), 2(a), 3(a), 4(a), 5(a), 6 both, and 7 one. Everything below is committed before any rewarded run.

### The target's rule, committed before it is computed

- **The rule** (`target_period`). The settled network's population rate is `spikes` of `units` units over `ticks` ticks. In hertz, at the 10 µs tick, it is $10^5 \cdot \text{spikes} / (\text{units} \cdot \text{ticks})$. The period is 100 000 ticks over that rate, which is $\text{units} \cdot \text{ticks} / \text{spikes}$. It is rounded to the nearest tick from the exact quotient, a half up, and clamped to ADR-0053's bounds; a network that did not fire gets the longest.
  - The period is rounded once, from the quotient, not from a rate rounded first.
  - By the rule the default's own rate gives the default: 5 Hz is 53 687 spikes at 1 024 units over $2^{20}$ ticks, and 20 000 ticks.
- **The lead-in** (`target_lead_in`, option 3(a)). It runs on ADR-0077's zero image — the modulation baseline zero and the inhibitory baseline unset, so nothing consolidates — decoded and driven by ADR-0044's drive for $2^{20}$ ticks from the tick the image was written at.
  - It runs in eight windows of $2^{17}$ ticks. The train is asserted to hold each window whole, as `settling` asserts it, and every unit's spikes are counted per window.
  - Every weight is asserted unmoved after it.
  - On the arms' image (option 3(b)) the inhibitory synapses would consolidate under their baseline, and the rate read would be a network moving under the default target.
- **Pinned before any rewarded run**, in a commit of its own after the lead-in's first run: the spikes per window (`TARGET_LEAD_IN_1024`) and the target (`TARGET_PERIOD_1024`). With them go the settled network's readings below that need the target.
- **What the rule gave**, read on that first run, which stopped at the empty pin before any rewarded trial:
  - the lead-in's windows held 2 367, 2 310, 2 349, 2 261, 2 274, 2 312, 2 338 and 2 238 spikes, **18 449 in all, 1.718 Hz a unit**;
  - **the target period is 58 201 ticks**, $1,073,741,824 / 18,449 = 58,200.5$ rounded up, where the depression per spike is **23** against the default's 67. Every period from 58 412 down to 55 979 gives 23, so the rule balances where a target fires at $23 / 1,343,488$ a tick, 1.712 Hz;
  - the gate holds the rule over the pinned spikes.

### The image each arm decodes (options 1(a) and 2(a))

- **`targeted_image`** writes the period into H-20's image, the modulator section's `[20..24)`, and re-seals the section.
- **`only_the_target`** is the masked check: every position that differs lies in those four bytes or in the modulator's entry in the section table, which holds the seal; and writing H-20's period back gives H-20's image bit for bit, whose CRC-64 is pinned.
- **Each arm asserts before its decode**: H-20's image by its CRC, the masked check, and then the decoded engine's period, sums and four couplings.
- **The other two options.** The configuration (option 1(b)) would change nothing, since the image's period outranks it. A settled engine run again (option 1(c)) would be another image, not H-20's with one change.

### H-21, restated as integer rules (ADR-0128's clauses and constants)

- **Clause 1, the drain stops** (`drained_below`, `first_below`): a block fails when its inhibitory sum is below half of the settled image's, `sum × 100 < image × 50`, with `DRAIN_FLOOR_PER_CENT` at 50. The clause holds for a run of 120 blocks in which no block fails.
- **Clause 2, the learning holds**: H-20's `scheduled`, unchanged — each mapping's last 128 trials at least 80 correct, and no coupling above 1.30 of its image's at any block's end.
- **The verdict** (`Targeted`): `held` and `below` per arm, `learning` (H-20's `Scheduled`), and `yes` when both clauses hold in both arms. `TARGET_PREDICTED` is `None`: ADR-0128 predicts no verdict.

### The assertion, beside the verdict

H-20's (`punished_held`): no excitatory synapse outside the four stimulus–readout pairs moves from the image to the run's end. The oracle is held to the record's weights, traces and signal at every trial inside the harness. A false assertion is a finding, reported beside the verdict and not in place of it.

### The readings, no clause (ADR-0128's, and option 6)

- **The inhibitory sum and the network's rate at every block's end**: the sum is the harness's `Block`; the rate is each block's spikes by class (`Watched::spikes`), read from the train at every trial's end (option 4(a)), so the population's spikes over a block of $2^{20}$ ticks are its rate in hertz over 10 737.
- **The fraction of units at the target** by ADR-0057's rule (`at_target`):
  - at the start, over each of the lead-in's eight windows, at the default period and at the rule's (`TARGET_AT_START_1024`);
  - over the eight trials before each flip and the run's last eight, at the engine's period (`Watched::at_target`).
- **The reversal speeds** beside H-20's 13 to 23: H-20's rules, `crossings`, `crossed_scheduled` and `first_new_scheduled`.
- **The settled network's rates** before the first trial (`rates`), from each unit's spikes over the lead-in:
  - the counts at 0, 10, 25, 50, 75, 90 and 100 per cent of the units, and the units that did not fire;
  - per class, the units and those at or above the target, `count × period ≥ ticks`.
- **The inhibition by class** (`inhibition_by_class`), F-59's need and the need of the stopping rule's step 4 should clause 1 fail: the inhibitory magnitudes summed by the class of the unit they reach, and the synapses at the rail. It is read on the image and at every block's end.
- **H-20's other readings**, by its rules: the tallies, the settle measure, the highest coupling per mapping, the punishment's course, the moves by mapping, the blocks in which the stimulus fired once, and the sums after.
- **The course beside H-20's**: each block's inhibitory sum as a fraction of the image's, dumped beside H-20's own course from its pinned tables (`h20_course`).
- ADR-0109's two predicted readings are not restated: ADR-0128 writes none.

### The calibration, before any rewarded run (H-21's stopping rule, step 2)

In each arm's test, before the arm's first rewarded trial:
- the settled engine is held to ADR-0077's tables step by step, and H-20's image to its CRC (`signed_images`);
- a frozen block from the zero image is held to ADR-0077's frozen run (`calibration_holds`);
- then the lead-in runs, and its spikes and the target it gives are held to their pins;
- then the masked check.

A failure at any of them stops the test there. Before the round's first rewarded run every whole-domain test that reaches `earned_run_scheduled` runs on the new harness, since option 4(a) routes it through the sibling.

### The gate

`the_target_s_rule_the_clauses_of_h_21_and_the_image_s_patch`, one test, no run:
- the constants, H-20's restated;
- the target's rule over sixteen rows written by hand: the rounding, the bounds, a network that did not fire, and 5 Hz giving the default;
- clause 1 at half of the image's sum and one LSB below, at the first, the 56th and the last block, and the verdict naming the clause, the arm and the block;
- the readings' rules over counts and a train written by hand;
- the classes and the inhibition by class on the instrument's network, where every inhibitory synapse sits at the rail;
- the patch on that network's image: the positions it moves, the loader reading the period, the old period written back giving the image, a byte moved elsewhere failing the check, and periods of 99 and 1 000 001 refused by the loader.

### The weekly tests and their cost

- **Two weekly tests**, one per arm: `a_target_the_network_fires_at_from_the_assignment_at_1024_units_exhaustive` and `a_target_the_network_fires_at_from_the_mirrored_assignment_at_1024_units_exhaustive`.
- **Each is H-20's arm and a little more**: H-20's arms took 1 794 and 1 690 s on the hosted runners by the cost table, and the lead-in adds one block's ticks on a frozen network.
- **The plan stays inside the budget.** The table after ADR-0127 is 71 tests and 20 405 s, about 24 per cent of the bound at six shards. The two arms add about 3 600 s, and the plan rises to about 28 per cent, under the directive's 60.

### The order of the work, as the history holds it

1. **The protocol** (`e6b6340` on the branch, committed at 02:05Z on 2026-09-29 and pushed to pull request #161, opened as a draft before any run): the rules, the constants, the gate, the arms with empty pins, and this ADR's protocol.
2. **The target** (`1bbbf59`, 02:10Z): the lead-in's first run, in the assignment arm's test, stopped at its empty pin after the calibration held and before the image was written; the target and the settled network's readings pinned in a commit of their own, before any rewarded trial.
3. **The earlier tests on the new harness**, before any rewarded run: the eleven whole-domain tests that reach `earned_run_scheduled` — H-14's reinforced form, H-15's plasticity everywhere, and H-16 to H-20's nine arms — ran from the release build of `e6b6340`, four at a time, each in a process of its own, from about 02:08 to 03:00Z. **All eleven passed**, every table pinned, H-20's two arms in 1 576 and 1 580 s.
4. **The arms**, once, side by side from 10:42Z, the calibration and the lead-in held to their pins in each and the masked check passing (ten bytes differ from H-20's image, two of the period's and eight of the seal; the image's CRC-64 `0xafe7eff2d59da51f`); both ran their 7 680 trials and stopped at the first empty table at 11:04Z, 1 327 and 1 325 s. The tables were written from those dumps, and a second run side by side from 11:06Z reproduced every table and passed, in 901 and 911 s (on the developer machine, a ratio and not admissible).

No constant, clause or rule moved after the first rewarded trial, and there was no second attempt: the second run is the pinned tables' reproduction.

### The readings

**The verdict** (`TARGET_1024`), by the rule committed first, over the pinned tables:

- **Clause 1 holds in both arms.** No block's inhibitory sum fell below half of the settled image's; the lowest stood at **0.848** of it in both, at the last block (`LOWEST_1024`: 8 478 and 8 477 per ten thousand at block 119).
- **Clause 2 holds in both arms**, as H-20's did:
  - each mapping's last 128 trials: **121, 119, 115 and 119** correct from the assignment and **124, 113, 115 and 119** from the mirrored assignment, against the mark of 80 (H-20's: 122, 121, 113, 115 and 125, 115, 118, 120);
  - no coupling above 1.30 of its image's at any block's end; the highest per mapping, as a fraction of the image coupling:

  | Arm | First mapping | Second | Third | Fourth |
  | :--- | :--- | :--- | :--- | :--- |
  | Assignment first | 1.153, block 22, A→R0 | 1.136, block 55, A→R1 | 1.181, block 86, A→R0 | **1.190**, block 119, A→R1 |
  | Mirrored first | 1.162, block 22, B→R0 | 1.155, block 54, B→R1 | 1.183, block 87, B→R0 | **1.193**, block 118, B→R1 |

  H-20's highest were 1.166 and 1.174.
- `Targeted { held: [true; 2], below: [None; 2], learning: Scheduled { learned: [[true; 4]; 2], bounded: [true; 2], over: [None; 2], yes: true }, yes: true }`. **H-21 is yes**, with its scope: this task, these two arms, this schedule of three flips over 7 680 trials, ADR-0077's settled network at 1 024 units with the gain 1.75 held and the controller off, H-20's learning configuration, and the inhibitory rule's target at 58 201 ticks. ADR-0128 wrote no prediction for it.

**The inhibitory sum's course beside H-20's** (each block's sum as a fraction of the settled image's):

| Block (end of) | 1st | 24th, first flip | 56th, second flip | 88th, third flip | 120th, end |
| :--- | ---: | ---: | ---: | ---: | ---: |
| H-21 from the assignment | 1.002 | 1.004 | 0.953 | 0.898 | **0.848** |
| H-20 from the assignment | 0.975 | 0.423 | 0.120 | 0.081 | 0.072 |
| H-21 from the mirrored | 1.002 | 1.001 | 0.951 | 0.896 | **0.848** |
| H-20 from the mirrored | 0.975 | 0.412 | 0.122 | 0.087 | 0.078 |

- **It rose first**, to 1.011 and 1.010 of the image's at the 13th and the 11th block, and stood at or above the image's until the 27th and the 25th.
- **Then it fell, and it had not levelled at the run's end.** Over the last 32 blocks it fell by 15.6 and 14.9 per ten thousand of the image's a block, and over the last 16 by 14.8 and 13.6; it fell in 105 and 108 of the 119 steps between blocks (`FALLS_TARGET_1024` false: it rose in the first blocks). H-20's fell by about 240 per ten thousand a block over its first mapping and levelled near 0.07.
- So the drain slowed by more than an order of magnitude and did not stop within the run. ADR-0128's account, that it *"should then stop where the units sit at the target"*, is not what the run read; clause 1, a floor of one half over 120 blocks, held with a margin of 0.35.

**Where the inhibition went** (`TARGET_INHIBITION_1024`: the inhibitory magnitudes by the class of the unit they reach, and the synapses at the rail), as fractions of the settled image's:

| At the end of | Onto inhibitory units | Onto stimulus units | Onto readout units | Synapses at the rail |
| :--- | ---: | ---: | ---: | ---: |
| the image | 1 | 1 | 1 | 45 |
| the first mapping (assignment; mirrored) | 0.917; 0.928 | 1.253; 1.252 | 0.987; 0.981 | 544; 590 |
| the second | 0.815; 0.827 | 1.281; 1.281 | 0.935; 0.929 | 709; 744 |
| the third | 0.718; 0.740 | 1.281; 1.281 | 0.879; 0.872 | 743; 796 |
| the fourth | **0.653; 0.664** | **1.281; 1.282** | **0.825; 0.823** | **833; 856** |

The four units past the last period, in no set, went as the readouts did, to 0.858 and 0.881.

**The network's rate** (`TARGET_SPIKES_1024`, the spikes by class per block over its $2^{20}$ ticks):

- the population at 2.03 to 2.20 Hz a unit per block, against the frozen lead-in's 1.718, the task's stimulus and response included;
- per mapping, the inhibitory units at 2.18 to 2.28 Hz, the stimulus units at 4.45 to 4.46, the readout units at 1.72 to 1.78, against the target's 1.712 (the rate at which a depression of 23 balances).

So the rule did what its target says where a class's rate stands well above the target: the stimulus units' inhibition rose to the rail by the second mapping and stayed, 1.281 of the image's being as far as the rail lets it rise. Where the rate stands near the target, the readouts, it fell slowly. And where it stands above the target by a third, the inhibitory units, it fell fastest of the three. The rule balances at the rate $\alpha / (2 \tau A_+)$ only while the postsynaptic spikes are independent of the presynaptic ones, and an inhibitory synapse delays the spike it inhibits; that the balance sits above the target where the target is inhibited hardest is **an account of this reading, a Hypothesis**, not a measurement: the round reads the rates and the sums by class and not the pairings.

**The fraction of units at the target** by ADR-0057's rule:
- at the start, over the frozen lead-in's eight windows: 0.36 to 0.41 at the default period and **0.77 to 0.80** at the rule's (`TARGET_AT_START_1024`);
- over the eight trials before each flip and the run's last eight, at the rule's period: **0.721, 0.723, 0.731 and 0.713** from the assignment and **0.721, 0.719, 0.729 and 0.695** from the mirrored (`TARGET_AT_TARGET_1024`).

**The settled network's rates before the first trial** (`TARGET_RATES_1024`, the lead-in's $2^{20}$ ticks): the units' spike counts at 0, 10, 25, 50, 75, 90 and 100 per cent are 6, 11, 14, 17, 21, 25 and 50; none was silent; **411 of 1 024 fired at or above the target** — 122 of the 204 inhibitory units, 31 of the 102 stimulus units, 257 of the 714 readout units and one of the four others.

**The reversal speeds** (`CROSSINGS_TARGET_1024`, blocks to 40 of 64, the crossing block counted), beside H-20's:

| Arm | First mapping | First reversal | Second | Third |
| :--- | ---: | ---: | ---: | ---: |
| H-21 from the assignment | 6 | 19 | 20 | 16 |
| H-20 from the assignment | 6 | 19 | 21 | 16 |
| H-21 from the mirrored | 3 | 23 | 14 | 14 |
| H-20 from the mirrored | 3 | 23 | 13 | 18 |

Each reversal passed 40 of 64 in the 14th to the 23rd block after its flip. Every stimulus selected its new answer within 106 trials of each flip (`FIRST_NEW_TARGET_1024`; ADR-0109's bound of 128 is no clause here).

**The learning's other readings**, by H-20's rules:
- the errors and ties per mapping (`TALLY_TARGET_1024`, `[correct, wrong, tied]`): from the assignment 1 250, 222, 64; 945, 963, 140; 1 001, 903, 144; 1 115, 810, 123; from the mirrored 1 271, 197, 68; 879, 1 070, 99; 1 302, 633, 113; 1 230, 728, 90;
- H-19's settle measure over each mapping's last 256 trials (`SETTLE_TARGET_1024`), the answer pairs' moves as per cents of their image couplings: up to 4.65 from the assignment and 2.91 from the mirrored, so in no mapping did both pairs settle, as in H-20;
- the punishment's course (`STRONG_BY_FLIP_TARGET_1024`): 242 and 285, 239 and 334, 209 and 287 strong punishments per flip and stimulus from the assignment, 248 and 242, 206 and 220, 231 and 201 from the mirrored;
- the stimulus fired once by ADR-0074's measure in all 120 blocks of both runs.

**The assertion held in both arms** (`REACH_TARGET_1024`): no excitatory synapse outside the four stimulus–readout pairs moved from the image to the end of the run — 3 187 and 3 188 of the pairs' synapses moved and none outside them, the inhibitory synapses 6 490 and 6 485 of 6 528 by their own baseline — and the oracle equalled the record's weights, traces and signal at every one of the 7 680 trials of each arm. In every block the inhibition by class sums to the block's inhibitory sum and the stimulus class's spikes from the train are the stimulus sets' own spikes the harness counts, which the gate holds over the pinned tables.

### H-21's stopping rule: step 3 reached, and the next decision it names

Step 1 (one round, the target's rule and H-21's constants committed before the first rewarded run, then the runs) and step 2 (the calibration reproduced ADR-0077's settled image, and no pinned number of an earlier test moved — the eleven tests through the changed harness passed before the rewarded run) are met above. **Step 3 is reached: H-21 is recorded yes with its scope in whitepaper §11.1, and the learning configuration is named with the target — ADR-0077's settled network at 1 024 units, the excitatory synapses under the reward's gate with the signed gate set, the inhibitory ones under a baseline of 0.5 with the inhibitory rule's target at the settled network's own rate by ADR-0128's rule (58 201 ticks here), and a critic of one expectation per stimulus with a shift of 5. The next decision, by the rule, is an ADR choosing among H-20's other three open questions — the operating regime, another size, and a critic of the engine's own. It is not taken here.** The readings it has beside it: the inhibitory sum still falling at the run's end, by about 0.15 per cent of the image's a block, so a run longer than 120 blocks is unread; the inhibition onto the inhibitory units falling fastest although they fire above the target; the stimulus units' inhibition at the rail. Steps 4 and 5 did not arise. Step 6 is kept.

### F-59, resolved

The settled image's inhibition was not at the rail: 0.775 of the prior's sum, the rule having run at the default target through ADR-0077's lead-in, and **45 of its 6 528 inhibitory synapses sat at the rail** (`TARGET_IMAGE_INHIBITION_1024`). Under the rule's target the run moved them both ways: 544 and 590 were at the rail by the first flip and 833 and 856 at the end, and the inhibition onto the stimulus units rose by 28 per cent, to the rail. ADR-0128's account and its step 4 read the start as the rail; step 4 did not arise, and its wording stays ADR-0128's. The finding is resolved by this reading; ADR-0128 and brief 054 are records and are not edited.

### The evidence

To be written from the weekly dispatched on this round's branch.

## Consequences

- Good: with its target at the network's own rate, the inhibitory rule no longer drains the network to a fourteenth of its inhibition over H-20's schedule; the sum ends at 0.85 of the image's, and the learning is H-20's, every mapping learned and every coupling bounded.
- Good: the target is read by a rule committed before the lead-in ran, and the image each arm decodes is H-20's in every other byte, so every difference from H-20 is the target's.
- Good: the inhibition by class says where the rule moved it — up to the rail onto the stimulus units, down onto the inhibitory units and the readouts.
- Neutral: the reversal speeds and the accuracy are H-20's within a few trials; the target neither helped nor hurt the learning as clause 2 reads it.
- Bad: the drain slowed and did not stop; whether it levels, and where, over a run longer than 120 blocks is unread.
- Bad: the rule's target is one rate for every unit, and the inhibitory units, above it, lose inhibition fastest; the account offered is a Hypothesis.
- Bad: one seed, one size, one schedule, as H-20's.

## Confirmation

- `runtime/cortex-runtime/tests/instrument/harness.rs`: `earned_run_observed`, which `earned_run_scheduled` calls.
- `runtime/cortex-runtime/tests/inhibition.rs`: `TARGET_LEAD_IN_TICKS`, `TARGET_ARMS`, `TARGET_PREDICTED`, `DRAIN_FLOOR_PER_CENT`, `target_period`, `drained_below`, `first_below`, `lowest`, `Targeted`, `targeted`, `classes_of`, `inhibition_by_class`, `each_spike`, `at_target`, `rates`, `targeted_image`, `only_the_target`, `target_lead_in`, `target_run`, `target_arm`; `TARGET_LEAD_IN_1024`, `TARGET_PERIOD_1024`, `TARGET_RATES_1024`, `TARGET_AT_START_1024`, `TARGET_IMAGE_INHIBITION_1024`, `TARGET_BLOCKS_1024` and the arms' other tables, `TARGET_1024`; the two weekly tests and the gate's test named above.
- Every pinned number of ADR-0065 to ADR-0128 unchanged, and the determinism pin; the image format 18.
- Whitepaper §11.1: H-21 checked yes with its scope and its stopping rule's step; §11's F-59 resolved; §9's row.
