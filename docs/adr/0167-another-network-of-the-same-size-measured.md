---
status: proposed
date: 2026-10-11
depends-on: ADR-0165
decision-makers: VirtualCortex maintainers
---

# ADR-0167: Another network of the same size, measured — H-33 is yes, on both networks, in both arms and in all five clauses: on ADR-0044's prior from seeds 23 and 24 every rule gave a value — the rotation 0, the gain 1.75 and the cancel's 6 messages as on seed 22, the settling at the 106th and the 99th window, the cancel's span 8 and 12 ticks, the target period 59 510 and 59 179 ticks — and the frozen block passed both measures; H-25's two arms then learned every mapping, 115 to 126 of each mapping's last 128, with no coupling past 1.30 of its image's, the highest 1.183 and 1.195, every reversal within 12 to 21 blocks of its flip and the network outside the couplings held; step 3 of the stopping rule, the learning configuration is the rules' and not one network's; the protocol written before any rule was run on a network of another seed, the harness's constants as what the rules return for a prior with every pinned number of seed 22 as it was, and the cancel's rule run as the tree holds it, a pick among three candidates; no file under `src/` changed

## Context and Problem Statement

[ADR-0165](0165-another-network-of-the-same-size.md) wrote H-33 before any run: on ADR-0044's generator at 1 024 units from seeds 23 and 24, each constant the learning configuration took from the network is derived again by its rule, and H-25's two arms are run. This ADR is the measurement's: the protocol, committed before any rule was run on a network of another seed, and then what was read.

**H-33**, restated from ADR-0165 and pinned in the tests. On each of the two networks, with the configuration its rules derive, H-25's two arms on H-20's schedule:
- **clause 0, the rules derive a configuration**: every rule gives a value and the frozen block passes both measures;
- **clause 1, the learning holds**: every mapping is learned, at least 80 of its last 128 trials correct;
- **clause 2, the couplings stay bounded**: no stimulus–readout coupling passes 1.30 of its image's at any block's end;
- **clause 3, the revision is in time**: each of the three reversals passes 40 of 64 within 23 blocks of its flip;
- **clause 4, the network holds**: at every block's end the summed magnitude of every excitatory synapse outside the four couplings lies within 0.75 and 1.25 of the image's.

Yes when all five hold on both networks, in both arms. **No prediction for the verdict**, and this round adds none.

**The last dispatched sweep** ([ADR-0158](0158-the-sweep-off-the-next-decisions-path.md)): run 38111246283's, which ADR-0165 wrote down. The one dispatch since, 38119792253, was [ADR-0166](0166-the-sweep-after-the-shards.md)'s check of its own change and was cancelled before any job of it ended. No sweep's outcome is owed here.

What was read first (principle 2), on 2026-10-11, in `runtime/cortex-runtime/tests/instrument/harness.rs` and `tests/inhibition.rs`:

1. **The harness keys a network by its unit count.** `prior(units)` writes the seed 22; `rotation(units)` and `gain(units)` return the constant written for 256 or 1 024 units. Nine functions of a run's path lay the four sets as `geometry(units, rotation(units))`: `task`, `run_on_scheduled`, `Composer::new`, `volley_once`, `after_quiet`, `lead_in_until_settled`, `taught_run`, `reach` and `earned_run_held`.
2. **A rewarded run takes three things from the network** (`earned_run_held`): the rotation; the cancel, `cancel_of(CANCEL_PICKED_1024)`; and the synapses from each stimulus set onto each readout set, which it holds the composer's count to, `SYNAPSES_1024`.
3. **Clause 2 and the highest coupling read the instrument's image** (`first_over`, `highest`): `IMAGE_COUPLINGS_1024`. Clause 4 reads `image_outside()`, the same image's.
4. **The cancel's rule is a pick among three candidates, and ADR-0165's table names one of them.** ADR-0165 and brief 067 write the cancel's messages as *"the least messages that leave a unit firing once"*, `least_cancel` at the extreme standing, 6. The tree (ADR-0076, `CANCELS`, `cancel_pick`): three candidates in a fixed order — the least count that leaves a unit at rest firing once, 3; the least that leaves a unit at the most standing potential firing once, 6; twice that, 12 — and the cancel is the first whose probe on a unit at rest fires once and whose frozen block on the prior's network fires once by ADR-0074's measure. On seed 22 the first left the stimulus its spikes after the opening and the second was picked. So 6 is what the pick returned there; on another network it may return another candidate.
5. **The span is read from the calibration's block.** `VOLLEY_TICKS_1024` is the census of ADR-0072's composition run: the prior's network at the picked gain, not yet settled, F-46's stimulus with no cancel. That run is the gain's calibration run read through the composer.
6. **The candidates' frozen blocks were run on the prior's network** (`the_two_injections_at_1024_units_exhaustive`), before ADR-0077 settled it. The settled network then took ADR-0076's pick as given.
7. **The settling's range is 160 windows** (`LEAD_IN_EXTENDED`), not the 80 of `LEAD_IN_BOUND`: seed 22 settled at the 96th.
8. **`quiet` fails its test** where the network is still active after one window of ticks. On a network no round has run on, that is a rule returning nothing.
9. **The frozen block's two measures** are ADR-0074's `fires_once` and the sight. The harness's sight, `calibrated`, asks 56 trials of 64 and each readout's total after the volley above its total before. ADR-0165's clause 0 writes the first of the two. ADR-0072's sign, which the settled network failed on seed 22 at 53 of 64, is not among the measures every round since H-13 has stood on.
10. **The critic's split is a judgment over a table of three scales** ([ADR-0132](0132-a-critic-of-the-engines-own-measured.md)), with its reasons written: the dead band under one per cent of the reward, the floor's drift, the volley's share of the rail. Only the sum of the shift and the scale is a formula of the stimulus's units.
11. **A rotation leaves 51 or 50 periods.** At 1 024 units the rotations 0 to 4 leave 51 whole periods and 5 to 19 leave 50. Either volley times 32 lies between $2^{10}$ and $2^{11}$.
12. **The rotation's rule has no range in the tree.** The gate holds that every rotation below the written one fails; at 256 units the first that passes is 17.
13. **Every step of seed 22's derivation has a pinned table from the round that made it**: `CALIBRATION_1024`, `VOLLEY_TICKS_1024`, `SYNAPSES_1024`, `CANCELLED_1024`, `PROBED_CANCEL_1024`, `BACKGROUND_LEAD_IN_1024`, `QUIET_1024`, `BACKGROUND_1024` with its counts and census, `TARGET_LEAD_IN_1024`, and the CRC-64 of H-17's, H-20's, H-21's, H-22's and H-23's images.
14. **The arena's size does not read the seed** (`blocks_for`): a prior's blocks are its units times its synapses a unit over a block's slots.

## Decision Drivers

- H-33's stopping rule: the seeds, the order of the derivations and the constants committed before any rule is run on a new network; no constant moves after one; no second attempt and no other seed.
- **One change from H-25**: the prior's seed.
- **A rule is run as the code has it**, on a network it was not written on: where ADR-0165's table and the tree differ, the tree is the rule (principle 1).
- **No pinned number of seed 22 moves**, and that is to hold by construction before it is checked by a run.
- **A rule that returns nothing is a reading**, so a derivation returns it and does not fail on it.
- **Seed 22 calibrates the derivation**: the function that derives on a new network is run on the instrument's prior and held to the tree's tables.
- The round is evidenced as [ADR-0150](0150-a-round-waits-for-what-it-checks.md) says. Latest ≠ Newest: nothing is adopted.

## Considered Options

1. **How a constant becomes what a rule returns for a prior**: (a) each function of a run's path that read a constant of the network gains a form that is given it, and the function it was calls that form with seed 22's constants; (b) every call site of the tree given the network; (c) a network in force held in thread-local state; (d) readers written again for the new networks.
2. **The cancel's messages**: (a) the tree's pick among three candidates, each derived on the network; (b) ADR-0165's table, the least count at the extreme standing.
3. **The frozen block's sight in clause 0**: (a) ADR-0165's words, 56 trials of 64, with the harness's `calibrated` read beside it; (b) `calibrated`.
4. **The critic's arithmetic for another volley**: (a) the sum by ADR-0132's inequality over the stimulus's units, the scale ADR-0132 took, and nothing returned where the reason ADR-0132 gave for the split does not hold; (b) ADR-0132's constants carried over; (c) the split chosen again by a table.
5. **How a derivation with no pinned table is held**: (a) by the rule, run again in every test that needs the network, and by the first run's tables pinned; (b) by its end, the image's CRC alone.
6. **How the derivations and the arms are dealt into tests**: (a) a test for seed 22's derivation, one for each new network's, and one for each arm, each arm deriving its own network first; (b) an image kept between tests.
7. **What an arm on a new network runs**: (a) H-25's run with no shadow beside it (`fed_run_on`), the composer, the critic's oracle and the network's oracle held at every trial; (b) H-25's arm whole, H-26's shadows and H-23's replication with it.
8. **The gate**: (a) one test — the rules at their edges, the rules over seed 22's pinned tables, the clauses and the steps over tables written by hand — that runs nothing on a new network; (b) a few trials on a new network.

## Decision Outcome

**Options 1(a), 2(a), 3(a), 4(a), 5(a), 6(a), 7(a) and 8(a).**

### The harness (Implemented in the tests)

- **`prior_seeded(units, seed)`** is ADR-0044's generator with the seed given; `prior(units)` is it at `PRIOR_SEED`, 22.
- **Each function that read a constant of the network gains a form that is given it**, and the function it was calls that form with what it read before, so every call of the tree is the call it was:

  | The function | Its form | What it is given |
  | :--- | :--- | :--- |
  | `task` | `task_over` | the four sets |
  | `run_on_scheduled` | `run_on_scheduled_over` | the four sets, held to the task's own |
  | `Composer::new` | `Composer::over` | the four sets |
  | `compose_on` | `compose_on_over` | the four sets |
  | `volley_once`, `after_quiet`, `fires_once` | `volley_once_of`, `after_quiet_of`, `fires_once_of` | the stimulus set's units |
  | `cancel_of`, `least_cancel` | `cancel_over`, `least_cancel_over` | the span |
  | `probe_task` | `probe_task_on` | the prior and the gain |
  | `candidate` | `candidate_from` | the prior and the gain |
  | `lead_in_until_settled` | `lead_in_until_settled_over` | the two readout sets |
  | `earned_run_held` | `earned_run_held_on` | `Derived`: the rotation, the cancel, the synapses |
  | `fed_run` | `fed_run_on` | `Derived` |
  | `first_over`, `highest`, `scheduled`, `drawn` | `first_over_of`, `highest_of`, `scheduled_of`, `drawn_of` | the image's four couplings |
  | `reach_by_polarity` | `reach_over` | the four sets |

- **`Derived`** is what a rewarded run takes from its network. `DERIVED_1024` is seed 22's: the rotation written, ADR-0076's pick over ADR-0076's span, and ADR-0072's counts. A compile-time assertion holds `CANCEL_PICKED_1024` to the candidate it names.
- **Two assertions are new on the old path** and read nothing: `run_on_scheduled_over` holds the sets it is given to the task's own, and `fed_run_on` holds its sets to the rotation's.

### The rules, their ranges and what each returns when nothing passes

`derive(seed)` runs them in ADR-0165's order and stops at the first that returns nothing. `Rules` holds what each returned.

| # | The constant | The rule, as the tree holds it | The range | Nothing when |
| ---: | :--- | :--- | :--- | :--- |
| 1 | the rotation | `first_balanced`: the first rotation at which the four couplings by the prior's census are `balanced` | 0 to 19, one period (`ROTATIONS`) | no rotation of the period is balanced |
| 2 | the gain | `first_calibrated`: the first gain whose calibration block on the prior's network is `calibrated` | 1.75, then 2.0 (`GAINS`) | neither is |
| 3 | the settled network | `settled_within` over the lead-in of `candidate_from` under the drive alone; then `quiet_within` | 160 windows (`LEAD_IN_EXTENDED`); one window of quiet ticks (`QUIET_BOUND`) | no window settles, or the network does not fall quiet |
| 4 | the cancel's offset | `CANCEL_OFFSET`, the refractory window and one | a constant of the membrane | — |
| 4 | the cancel's span | `span_of` the census of the picked gain's calibration block | the ticks before the readout window opens | the block holds no volley spike |
| 4 | the cancel's candidates | `cancel_candidates`: `least_cancel_over` at rest and at `EXTREME_STANDING`, and twice the second | 1 to 32 messages (`LEAST_SCAN`) | either scan finds none |
| 4 | the cancel's messages | `first_firing_once`: the first candidate whose probe fires once and whose frozen block on the prior's network fires once | the three candidates, in order | none does |
| 4 | the frozen block | `frozen_measures` over a frozen block from the settled image under the cancel: `fires_once_of`, and the sight at `SEEN_MIN` | 64 trials | a measure fails: clause 0 fails and the network stops |
| 5 | the target period | `target_period` over the settled image frozen under the reference drive | $2^{20}$ ticks | never: the rule clamps to ADR-0053's bounds |
| 6 | the critic's window | `shortest_delay` of the image | every synapse | the image carries none |
| 6 | the critic | `critic_for` the stimulus set's units | sums of 0 to 44 | see below |

- **The gain's candidates are run in order until one passes**, and so are the cancel's. A candidate after the one taken is not run; the rule's return is the same.
- **The rotation's range is one period** (option of brief 067's empowerment, the tree stating none): a rotation of twenty lays the same pattern one period on. A rotation that puts a stimulus on the units the prior makes inhibitory reads a coupling of zero, which `balanced` refuses, so the scan needs no rule of its own for them.
- **The cancel** is the tree's pick (option 2(a)). The census is the picked gain's calibration block's; the candidates are derived over that span at that gain; each candidate in order is probed on a unit at rest through the task and run for a frozen block on the prior's network. What ADR-0165's table names, the least count at the extreme standing, is the second candidate and is read beside the pick.
- **The critic** (option 4(a)), `critic_for(units)`:
  - the sum of the shift and the scale is the least at which `units × 32 ≤ 2^sum`, ADR-0132's inequality;
  - the scale is ADR-0132's, 2, and the shift the rest;
  - nothing is returned where the sum is below the scale, where the loader would refuse the constants, or where the shift's dead band, $2^k$, is not under one per cent of the reward, the first reason ADR-0132 gave for its split.

  For 33 to 64 units it is ADR-0132's own, a shift of 9 and a scale of 2. A rotation at 1 024 units leaves 51 or 50, so on these networks the rule returns ADR-0132's constants whatever the rotation; it is run and not assumed.
- **Clause 0's sight** is ADR-0165's (option 3(a)): the window after the volley holding more readout spikes than the window before the injection in at least 56 trials of 64. The harness's `calibrated` and ADR-0072's sign are read beside it.
- **The image** is written as H-25's arms write it: the settled network's image with the baseline at zero, then the inhibitory baseline at a half, the signed gate, the target period, the critic and the window, each onto the last. The target period's, the critic's and the window's masked checks hold each change to be the only one.

### What holds a derivation where seed 22 had a pinned table

- **On seed 22, the tables.** `the_rules_return_the_tree_s_constants_on_seed_22_at_1024_units_exhaustive` runs `derive` on the instrument's prior and holds:
  - what each rule returned to `RULES_1024`, the constants the tree holds — the rotation 0, the gain 1.75, the 96th window, the span 9, the least counts 3 and 6, the pick 6, both measures, the period 58 201, the window 100, and the critic's shift 9 and scale 2;
  - every table read on the way to the table the round that made it pinned (item 13 above);
  - the CRC-64 of the image with the inhibitory baseline, with the signed gate, with the target period, with the critic and with the window to H-17's, H-20's, H-21's, H-22's and H-23's images, the last the one H-25's arms decode.
- **On a new network, the rule and the first run** (option 5(a)). Each network's derivation is a weekly test of its own that dumps everything before it holds anything and pins what each rule returned and every table it was read from. Each arm's test runs the derivation again and holds it to those pins before its first trial. The dispatched run reproduces both.

### The protocol

- **The networks**: `SEEDS`, 23 and 24, in that order. Neither was synthesized, read or run before this ADR's commit.
- **The order**: on each network the derivations as the table above numbers them, then the arms.
- **The derivations' tests**: `the_rules_on_seed_23_at_1024_units_exhaustive` and `the_rules_on_seed_24_at_1024_units_exhaustive`.
- **The arms** (`SEEDED_ARMS`, H-25's two): `the_address_drawn_on_seed_{23,24}_from_the_{assignment,mirrored_assignment}_at_1024_units_exhaustive`, four weekly tests, each run once. An arm exists on a network whose clause 0 holds; on one where it does not, its two tests are removed with the pins and the ADR says so.
- **An arm's run** (option 7(a)): H-25's image of the network decoded and held to the derived configuration; then 7 680 trials under `Delivery::Drawn` with no task critic and H-20's flips (`fed_run_on` with no shadow and no hold), the task's seed 27 and the drive's 3. At every trial the drawn sources are held to the units the critic's oracle counted, and the composer and the network's oracle to the record.
- **The clauses**, by rule: `Rules::hold` for clause 0; `drawn_of`, H-25's `drawn` against the network's own image, for clauses 1 to 4. `seeded` is a network's verdict and `seeded_step` the step of ADR-0165's stopping rule the two reach: 3 for a yes; 4 for a no on clause 0 on either network; otherwise 5 for a no on clause 4; otherwise 6 for one on clause 1 or 2; otherwise 7 for clause 3 alone.
- **The constants**: 80 of 128 (`REWARDED_MIN`); 1.30 (`BOUND_PER_CENT`); 40 of 64 within 23 blocks (`CROSSING_MARK`, `REVERSAL_BLOCKS_MAX`); 0.75 and 1.25 (`BAND_QUARTERS`); 56 of 64 (`SEEN_MIN`). Each is H-25's or ADR-0165's and none moves.
- **The readings, no clause**:
  - *each derived constant beside seed 22's* (`RULES_1024`), with the tables beside seed 22's: the prior's sums, the synapses, the lead-in's length, the quiet run, the image's couplings, the target's lead-in;
  - *the frozen block*: the readouts' spikes after the volley and before the injection and the trials seen (`measure`), the selections by readout and the ties (`frozen_selections`), the stimulus's spikes after the opening, the sign and `calibrated`, beside seed 22's;
  - *per arm* (`SeededRead`): the correct trials of each mapping's last 128; the block each mapping passes 40 of 64 in, beside H-25's; the couplings' courses and each mapping's highest, beside H-25's 1.191; each stimulus's value over each mapping's last 128 beside $2p - 1$, and the value's troughs; the inhibitory sum's course; each mapping's trials by outcome; the reach; the drawn sources beside the host's.
- **An arm's pinned tables**, by block: the 120 blocks as H-25's are pinned; the earned blocks; the engine's value; the drawn sources; and in one number each the accuracy sequence, every trial's reading, every trial's value, every trial's drawn sources and the compositions.

### The calibration (H-33's stopping rule, step 2)

- **Every pinned number of the tree holds.** The tree's whole-domain tests run on the developer machine at this protocol's commit, the six tests of the new networks left out, before any rule is run on one. Every run of `tests/instrument.rs`, `tests/everywhere.rs` and `tests/inhibition.rs` passes through a form the harness gained.
- **On seed 22 the rules return the constants the tree holds**: the test above, among them.
- **A calibration that fails stops the round there** as a finding, and no rule is run on a new network.

### The gate

One test, `the_rules_of_a_derivation_the_clauses_of_h_33_and_the_readings_rules`:
- the seeds, that `prior_seeded` at 22 is `prior` and that a seeded prior differs from it in the seed alone, the ranges and every constant;
- each rule at its edges over tables written by hand — the rotation's at the first and the last rotation of the period and past it; the gain's; the candidates' over ADR-0076's span and over a span of no tick; the pick's; the frozen block's measures; `quiet_within` on a network at rest; the critic's at 51, 50, 33, 64, 32, 4, 65 and no unit; clause 0 with each rule unset in turn;
- each rule over the table the tree pins for seed 22, returning the constant the tree holds, where that needs no run: the rotation over the prior's census, the gain over `CALIBRATION_1024`, the settled window over ADR-0077's lead-in, the span over `VOLLEY_TICKS_1024`, the pick over `CANCELLED_1024`, the measures over ADR-0077's frozen block, the period over ADR-0129's lead-in, the window over the prior's arena, and the critic;
- H-33's verdict and each step of the stopping rule over tables written by hand, against an image written by hand: a yes, clause 0, clause 4, clause 1, clause 2 one past the bound and at it, clause 3 alone in its 24th block, and the steps' order where two fail;
- the readings' rules over tables written by hand;
- since the pins, over the two networks' pinned tables: each rule over the table its derivation pinned returning what the derivation pinned, each arm's readings by their rules over its 120 blocks, a selection or a tie and a value for every presentation of every block, and the verdict and its step as written.

It runs no rule on a network of another seed.

### The calibration, as it ran (2026-10-11)

On the developer machine, at the protocol's commit `9677dfe` with nothing uncommitted, pushed at 09:56Z:
- **The rules on seed 22**, run alone before the commit: the test passed on its first run, in 116 s. `derive` returned `RULES_1024`, every table on the way was the table its round pinned, and the image's CRC-64 was H-23's, `0x11586a76f7415129`.
- **Every pinned number of the tree**: `cargo test --workspace --release --locked -- --ignored exhaustive --skip on_seed_23 --skip on_seed_24`, 96 tests — the 95 the tree held and the rules on seed 22 — from 09:58:13Z to 13:24:07Z. All passed. The workspace's other tests in the release profile passed before it, 731.
- **No rule had run on a network of another seed**: the first did at 13:24:27Z.

### The derivations: clause 0 holds on both networks

Each network's derivation ran once, the two side by side, in 184 and 173 s. Every rule gave a value on both, and the frozen block passed both measures on both.

| The rule returned | Seed 22 | Seed 23 | Seed 24 |
| :--- | ---: | ---: | ---: |
| The rotation | 0 | 0 | 0 |
| — the four couplings by census there, the largest over the smallest | 1.039 | 1.074 | 1.059 |
| The gain | 1.75 | 1.75 | 1.75 |
| — its calibration block's trials seen, of 64 | 62 | 63 | 62 |
| The settled network, at window | 96 | 106 | 99 |
| — the excitatory sum settled, of the prior's | 0.925 | 0.920 | 0.925 |
| — the inhibitory sum settled, of the prior's | 0.776 | 0.752 | 0.767 |
| — the quiet run, ticks | 2 500 | 2 519 | 2 424 |
| The cancel's span, ticks | 9 | 8 | 12 |
| The least messages at rest, and at the extreme standing | 3, 6 | 3, 6 | 3, 6 |
| The cancel's messages, the pick | 6, the second candidate | 6, the second | 6, the second |
| — the first candidate's stimulus spikes after the opening | 2 717 | 2 745 | 2 746 |
| The frozen block: the stimulus's spikes after the opening | 0 | 0 | 1 |
| The frozen block: trials seen, of 64 | 62 | 64 | 64 |
| The settled network's spikes over the target's lead-in | 18 449 | 18 043 | 18 144 |
| The target period, ticks | 58 201 | 59 510 | 59 179 |
| The critic's window, ticks | 100 | 100 | 100 |
| The critic's shift and scale | 9, 2 | 9, 2 | 9, 2 |

- **Five constants came back as seed 22's on both networks**: the rotation, the gain, the cancel's messages, the critic's window and the critic's constants. Three did not: the window the network settles at, the cancel's span and the target period.
- **The rotation's scan stopped at its first rotation on all three.** The census's four couplings lie within 3.9, 7.4 and 5.9 per cent of one another.
- **The cancel's span is where the networks differ most.** The volley's spikes fall on ticks 1 to 9 on seed 22, 1 to 8 on seed 23, and on seed 24 on ticks 1 to 6 and, one unit each, on 9, 11 and 12. Over 8, 9 or 12 ticks the oracle's least counts are the same, 3 and 6.
- **The pick took the second candidate on all three**, for the reason it did on seed 22: under the first, the least count at rest, the stimulus fires again after the opening, 2 745 and 2 746 spikes over the block where a tenth of a spike a unit a presentation allows 326. So the tree's pick and the count ADR-0165's table names agree on these networks.
- **The settling ran past the eighty windows of `LEAD_IN_BOUND` on all three** and held within the 160.
- **The critic's arithmetic returned ADR-0132's constants**, as it had to with 51 units a stimulus at a rotation of zero.
- **The synapses a rewarded run holds**, from each stimulus onto each readout: 775, 806, 798 and 809 on seed 22; 824, 767, 791 and 799 on seed 23; 794, 782, 771 and 809 on seed 24.
- **The image each arm decodes**: CRC-64 `0x20e08eeecd746cb1` on seed 23 and `0x462f5c9f107e3cff` on seed 24.

**The frozen block from the settled image**, sixty-four trials, beside seed 22's (ADR-0077's):

| | Seed 22 | Seed 23 | Seed 24 |
| :--- | ---: | ---: | ---: |
| The volley's spikes, A and B, of 1 734 and 1 530 | 1 728, 1 525 | 1 724, 1 527 | 1 728, 1 526 |
| The readouts' spikes after the volley, readout 0 and 1 | 645, 637 | 700, 693 | 683, 673 |
| The readouts' spikes before the injection | 258, 256 | 250, 224 | 224, 232 |
| Trials seen, of 64 | 62 | 64 | 64 |
| ADR-0072's sign, trials of 64 | 53 | 61 | 56 |
| A's selections: readout 0, readout 1, tie | 14, 15, 5 | 12, 18, 4 | 20, 10, 4 |
| B's selections: readout 0, readout 1, tie | 12, 15, 3 | 17, 10, 3 | 10, 18, 2 |

- **`calibrated` holds on all three**, each readout's total after the volley above its total before.
- **The sign, which seed 22's settled network failed at 53, reads 61 and 56**, at or past its mark of 56. It is no measure of clause 0.
- **The frozen networks lean differently.** Seed 22's selects readout 1 a little more for both stimuli. Seed 23's leans to the mirrored assignment, A onto readout 1 and B onto readout 0. Seed 24's leans to the assignment.

### H-33's verdict: yes, on both networks

The four arms ran once each, side by side from 13:28:52Z, in 1 915 to 1 935 s (on the developer machine, a ratio and not admissible). Each derived its network again, held the derivation to the pins above before its first trial, ran its 7 680 trials with every oracle held at every trial, dumped everything and stopped at its first empty table. The tables were written from those dumps, and every filled table was held to the text of its dump by a script. The pins are `2e6682e`.

| Network, arm | Clause 1: correct of each mapping's last 128 | Clause 2: the highest coupling, of its image's | Clause 3: blocks to 40 of 64 after each flip | Clause 4: the outside sum, of the image's |
| :--- | :--- | :--- | :--- | :--- |
| Seed 23, from the assignment | 126, 124, 124, 123 | 1.183, block 113, A→R1 | 15, 13, 13 | 0.9999 to 1.0000 |
| Seed 23, from the mirrored assignment | 122, 115, 118, 126 | 1.182, block 120, B→R1 | 16, 14, 12 | 0.9999 to 1.0001 |
| Seed 24, from the assignment | 117, 123, 120, 115 | 1.195, block 120, B→R0 | 21, 12, 15 | 0.9999 to 1.0001 |
| Seed 24, from the mirrored assignment | 124, 120, 121, 123 | 1.191, block 84, B→R0 | 17, 16, 14 | 0.9998 to 1.0000 |
| *H-25, seed 22, from the assignment* | *122, 124, 119, 116* | *1.181* | *19, 21, 16* | — |
| *H-25, seed 22, from the mirrored assignment* | *124, 118, 123, 117* | *1.191* | *20, 15, 14* | — |

- `SEEDED_1024`: on each network `Seeded { derived: true, arms: Some(Drawn { learning: Scheduled { learned: [[true; 4]; 2], bounded: [true; 2], over: [None; 2], yes: true }, revised: [[true; 3]; 2], left: [None; 2], held: [true; 2], yes: true }), yes: true }`.
- **H-33 is yes**, with its scope: H-25's task, arms and schedule; 1 024 units; ADR-0044's generator with every parameter but the seed; the drive's seed 3 and the task's 27; three networks of that generator, seeds 22, 23 and 24. ADR-0165 wrote no prediction for it.
- **Clause 3**, which ADR-0165 named as one of the two least known: the slowest reversal of the twelve took 21 blocks, two inside the bound, and the others 12 to 17.

### The readings

**The first mapping follows the frozen network's lean.** The first mapping passes 40 of 64 in its 4th and 3rd block on seed 23, from the assignment and from the mirrored assignment, and in its 1st and 8th on seed 24; H-25 read 6 and 4. On seed 24 the arm that starts from the assignment, which the frozen network leans to, reads 42 correct in its first block, and the mirrored one 20. On seed 23 the mirrored arm reads 37 in its first block and the other 24. Seed 22's frozen network selects readout 1 a little more for both stimuli, which is neither assignment's.

**The reversals** take 12 to 21 blocks on the new networks, and 14 to 21 on seed 22. By network the six reversals average 13.8 blocks on seed 23, 15.8 on seed 24 and 17.5 on seed 22.

**The couplings.** Over the four arms the highest stands at 1.182 to 1.195 of its image's and the lowest at 0.895 to 0.929; H-25's stood at 1.181 and 1.191, and 0.914 and 0.955. At each mapping's end the answer's two pairs stand at 1.12 to 1.20 of their image's and the other two at 0.89 to 1.01, where H-25's stood at 1.11 to 1.19 and at 0.91 to 1.00.

**The value beside $2p - 1$.** Over each mapping's last 128 trials each stimulus's mean value lies within 0.14 of the reward of $2p - 1$, in all thirty-two cases; H-23's bound, a reading here, is a quarter. After a flip the value's trough is −0.56 to −0.89 of the reward, where H-25 read −0.63 to −0.84, and it stays above zero from the 13th to the 21st block after a flip, where H-25's did from the 14th to the 24th.

**The inhibitory sum** rises first, to 1.017 to 1.021 of the image's by the 16th to 19th block, and then falls through the run to 0.908 and 0.910 on seed 23 and 0.896 and 0.897 on seed 24. H-25's rose to 1.010 and 1.011 by the 11th to 14th block and ended at 0.848 and 0.850. It did not fall in every block in any arm.

**The drawn sources** are the presented stimulus's units in 99.67 and 99.68 per cent of their presentations, with 1.62 to 1.63 other units a trial; H-25 read 99.66 per cent and about 1.6.

**The stimulus** fires once by ADR-0074's measure in all 120 blocks of each arm. **Ties** are 49 to 117 trials a mapping. **The reach**: of the four pairs' synapses, 3 181 and 3 179 of 3 181 ended away from the image's weight on seed 23, and 3 155 and 3 153 of 3 156 on seed 24; 11 036 to 11 386 excitatory synapses outside the pairs ended away from it too, the drawn address reaching the background units the window counts, and their sum stayed within two parts in ten thousand of the image's.

### What is read from the tables and not measured

- **Why the first mapping's speed follows the frozen lean** was not varied: the four arms of the two new networks agree with it, seed 22's frozen network leans to one readout for both stimuli and so to neither assignment, and no arm was run from an image whose lean was changed.
- **Why the inhibitory sum ends higher on the new networks** is not known. Their target periods are longer, 59 510 and 59 179 ticks against 58 201, and their settled images hold less inhibition, 0.752 and 0.767 of the prior's against 0.776; neither was varied.
- **Whether the rules transfer where they would return another value** is read only for three of the eight. The rotation, the gain, the cancel's messages, the window and the critic came back as seed 22's, so on these networks those rules were tested in that they returned a value and the value worked, not in that they returned a different one.

### The step reached

**Step 3 of H-33's stopping rule: yes.** The learning configuration is the rules', not one network's: on two networks it was not built on, the rules derived it, and it learned and revised as on the first.

**The next decision is an ADR choosing among another size, the operating regime, a reward delayed past a trial and a less reliable reward.** It is named and not taken. Steps 4 to 7 did not arise, and step 8 is kept: no constant moved after a rule was run on a new network, there was no second attempt, and no other seed was run.

## Consequences

- Good: the caveat the line's last four decisions carried, one seeded prior, is answered for this generator at this size: of three networks read, the configuration learns and revises on each.
- Good: the rules that derived the configuration were run where they were not written, and each gave a value that worked.
- Good: the function that derives on a new network is the one held on seed 22 to every table eight rounds pinned, down to the image's CRC.
- Good: no call of the tree changed. A pinned number of seed 22 can move only through a form's body, and every existing whole-domain test runs through one.
- Good: a rule that returns nothing is a value the derivation returns and a test pins.
- Bad: each arm's test derives its network before its first trial, about a seventh of an arm again.
- Bad: the cancel's rule is run as the tree has it and not as ADR-0165's table put it. Where the two would differ on a network, the configuration is the pick's, and ADR-0165's is a reading.
- Bad: the critic's split is restated by one of ADR-0132's three reasons. On these networks it cannot return anything but ADR-0132's constants, so the restatement is not tested by them.
- Bad: three networks of one generator, one size and one drive are a small sample, and five of the eight constants came back the same on all three. A rule that would have to return another value has not been read doing so.
- Bad: no rule returned nothing, so the branch of a derivation that stops a network was run only by the gate, over tables written by hand.
- Neutral: seven weekly tests, four of them arms.
- Neutral: the first mapping's speed follows the frozen network's lean on each of the three, and the inhibitory sum ends higher on the two new ones. Both are readings.

## Alternatives considered and why rejected

- **Option 1(b), every call site given the network**: some three hundred calls in a file of two hundred thousand lines, each a place a pinned number could move.
- **Option 1(c), a network in force in thread-local state**: a test that did not set it would run on seed 22 and say nothing.
- **Option 1(d), readers written again**: a second run path, which no existing test would hold to seed 22's tables.
- **Option 2(b), ADR-0165's table**: it names what the pick returned on seed 22, not the rule that returned it.
- **Option 3(b), `calibrated`**: a stricter clause than ADR-0165 wrote. It is read beside.
- **Option 4(b), the constants carried over**: ADR-0165 asks for the arithmetic restated. **Option 4(c)**: a table is a judgment made after the network is seen.
- **Option 5(b), the CRC alone**: a derivation that moved would say only that it had.
- **Option 6(b), an image kept between tests**: the weekly shards run each test in a process of its own.
- **Option 7(b), H-25's arm whole**: H-26's shadows and H-23's replication read seed 22's tables.
- **Option 8(b), a few trials on a new network in the gate**: the gate runs on every push, and no rule is to run on a new network before the protocol is committed.

## Confirmation

`runtime/cortex-runtime/tests/inhibition.rs` holds the rules, the derivation, the clauses, the seven weekly tests and their pinned tables; `SEEDED_1024` and `SEEDED_STEP_1024` hold the verdict and its step as written. The gate's test holds the rules at their edges, over seed 22's tables and over the two networks' pinned tables. Whitepaper 4.105.0 carries H-33's verdict in §11.1 and this ADR's row in §9. `npm run spec` holds the links, the rows and the version.
