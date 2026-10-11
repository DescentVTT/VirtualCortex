---
status: proposed
date: 2026-10-11
depends-on: ADR-0165
decision-makers: VirtualCortex maintainers
---

# ADR-0167: Another network of the same size, measured — H-33's protocol, written before any rule is run on a network of another seed: the harness's constants for the instrument's prior as what the rules return for a prior, with every call and every pinned number of seed 22 as it was; the derivation of H-25's configuration on a seeded prior, each constant by the rule that derived it and stopped at the first rule that returns nothing; the range each rule scans; the cancel's rule as the tree holds it, a pick among three candidates and not the one ADR-0165's table names; H-33's five clauses and the step its stopping rule reaches as rules; no file under `src/` changed

## Context and Problem Statement

[ADR-0165](0165-another-network-of-the-same-size.md) wrote H-33 before any run: on ADR-0044's generator at 1 024 units from seeds 23 and 24, each constant the learning configuration took from the network is derived again by its rule, and H-25's two arms are run. This ADR is the measurement's: the protocol, committed before any rule is run on a network of another seed, and then what was read.

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
- the readings' rules over tables written by hand.

It runs no rule on a network of another seed.

## Consequences

- Good: the function that derives on a new network is the one held on seed 22 to every table eight rounds pinned, down to the image's CRC.
- Good: no call of the tree changed. A pinned number of seed 22 can move only through a form's body, and every existing whole-domain test runs through one.
- Good: a rule that returns nothing is a value the derivation returns and a test pins.
- Bad: each arm's test derives its network before its first trial, about a seventh of an arm again.
- Bad: the cancel's rule is run as the tree has it and not as ADR-0165's table put it. Where the two would differ on a network, the configuration is the pick's, and ADR-0165's is a reading.
- Bad: the critic's split is restated by one of ADR-0132's three reasons. On these networks it cannot return anything but ADR-0132's constants, so the restatement is not tested by them.
- Neutral: seven weekly tests, four of them arms.

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

`runtime/cortex-runtime/tests/inhibition.rs` holds the rules, the derivation, the clauses and the seven weekly tests; the gate's test holds the rules at their edges and over seed 22's tables. The tables of the new networks are empty until their one run. `npm run spec` holds the links, the rows and the version.
