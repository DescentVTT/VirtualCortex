---
status: proposed
date: 2026-09-29
depends-on: ADR-0128
decision-makers: VirtualCortex maintainers
---

# ADR-0129: A target the network fires at, measured — brief 054's protocol, committed before any rewarded run: ADR-0128's rule for the inhibitory rule's target period, read on ADR-0077's settled image frozen under the reference drive over $2^{20}$ ticks; the image each arm decodes, H-20's with the period written and nothing else, shown by a masked check; H-20's two arms run once from it with H-20's critic and flips and pinned whole; H-21's clauses as integer rules, H-20's assertion beside them, and readings of the spikes and the inhibition by class per block, the fraction of units at the target before each flip and the settled network's rates; and F-59, the settled image's inhibition already below the rail

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

### The order of the work

1. This commit: the rules, the constants, the gate, the arms with empty pins, and this ADR's protocol, before any run.
2. The lead-in's first run, which stops at its pins before any rewarded trial; the target and the settled network's readings pinned in a commit of their own.
3. Every whole-domain test through `earned_run_scheduled` run on the new harness; then the two arms, once, stopping at their first empty pin; the tables written from the dumps, and a second run to reproduce them.
4. This ADR's readings and the documents; then the weekly dispatched on the branch and the cost table regenerated from it.

### F-59

The settled image's inhibition is not at the rail: 0.775 of the prior's sum, the rule having run at the default target through ADR-0077's lead-in. ADR-0128's account and its step 4 read the start as the rail. The round reads the synapses at the rail on the image and at every block's end. The stopping rule is ADR-0128's and is applied as written; its step 4, if it is reached, is read with this finding beside it.

## Consequences

- Good: the target is read by a rule committed before the lead-in ran, and the image each arm decodes is shown to be H-20's in every other byte, so a difference from H-20 is the target's.
- Good: the inhibition by class and at the rail per block say where the inhibition went, whatever clause 1 reads.
- Neutral: the whole run is pinned, 120 blocks per arm, since nothing of H-20 is replicated.
- Bad: one seed, one size, one schedule, as H-20's.

## Confirmation

- `runtime/cortex-runtime/tests/instrument/harness.rs`: `earned_run_observed`, which `earned_run_scheduled` calls.
- `runtime/cortex-runtime/tests/inhibition.rs`: `TARGET_LEAD_IN_TICKS`, `TARGET_ARMS`, `TARGET_PREDICTED`, `DRAIN_FLOOR_PER_CENT`, `target_period`, `drained_below`, `first_below`, `lowest`, `Targeted`, `targeted`, `classes_of`, `inhibition_by_class`, `each_spike`, `at_target`, `rates`, `targeted_image`, `only_the_target`, `target_lead_in`, `target_run`, `target_arm`; the two weekly tests and the gate's test named above.
- Every pinned number of ADR-0065 to ADR-0128 unchanged, and the determinism pin; the image format 18.
