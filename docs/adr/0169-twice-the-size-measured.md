---
status: proposed
date: 2026-10-12
depends-on: ADR-0168
decision-makers: VirtualCortex maintainers
---

# ADR-0169: Twice the size, measured — H-34's protocol, written before any rule is run at 2 048 units: the derivation given its size, with every pinned number at 1 024 units as it was but the one assertion ADR-0168 names; the critic's rule as ADR-0168 writes it; the injector ring, the one capacity of a run's configuration the rules' ranges could reach at 2 048 units, sized by the units; no file under `src/` changed

## Context and Problem Statement

[ADR-0168](0168-twice-the-size.md) wrote H-34 before any run: on ADR-0044's generator at 2 048 units from seeds 22 and 23, each constant of the learning configuration is derived by its rule with the size given where 1 024 was written, and H-25's two arms are run. This ADR is the measurement's: the protocol, committed before any rule is run at the new size, and then what was read.

**H-34**, restated from ADR-0168 and pinned in the tests. On each of the two networks at 2 048 units, with the configuration its rules derive, H-25's two arms on H-20's schedule:
- **clause 0, the rules derive a configuration**: every rule gives a value and the frozen block passes both measures, every stimulus unit firing once and the stimulus seen in 56 trials of 64;
- **clause 1, the learning holds**: every mapping is learned, at least 80 of its last 128 trials correct;
- **clause 2, the couplings stay bounded**: no stimulus–readout coupling passes 1.30 of its image's at any block's end;
- **clause 3, the revision is in time**: each of the three reversals passes 40 of 64 within 23 blocks of its flip;
- **clause 4, the network holds**: at every block's end the summed magnitude of every excitatory synapse outside the four couplings lies within 0.75 and 1.25 of the image's.

Yes when all five hold on both networks, in both arms. **No prediction for the verdict**, and this round adds none.

**The last dispatched sweep** ([ADR-0158](0158-the-sweep-off-the-next-decisions-path.md)): run 38111246283's, which ADR-0165 wrote down. The one dispatch since that ran to its end, 38145828467, was brief 067's at `scope=exhaustive` and ran no sweep. No sweep's outcome is owed here.

What was read first (principle 2), on 2026-10-12, in `runtime/cortex-runtime/tests/inhibition.rs`, `tests/instrument/harness.rs` and `src/`:

1. **`derive` writes its size in sixteen places**: the prior, three configurations, two geometries, three composed blocks, the candidate's engine, the lead-in, two decodings of the frozen image, the target period's units and two decodings of the signed image.
2. **Four functions beneath it write the size too.** `probe_task_on` builds the probe's network and its silent drive at 1 024; `target_lead_in` decodes the settled image and lays the drive at 1 024; `fed_run_on` holds its sets to the geometry at 1 024 and runs at 1 024; and `earned_run_held_on` holds its `units` to 1 024 by an assertion, written when the cancel and the synapse counts beneath it were constants and kept when brief 067 made them an argument.
3. **Nothing else on the path names a size.** The bodies of the 153 functions reachable from `derive` and from an arm, in the two files, were read by a script for a size written in place, for `rotation(units)`, `gain(units)` and `prior(units)`, and for a table named for a size. It is not in the tree. Beside the four above it found what only dumps a reading beside seed 22's, and `config`, which reads the prior's block count by the units.
4. **`rotation(units)` and `gain(units)` panic at another size**, and are called only by the functions whose forms brief 067 added. A derivation calls the forms.
5. **The size enters a derivation through six things**: the geometry's whole periods, and so the stimulus set's units, which the frozen block's two clauses and the critic's rule read; the drive's messages a tick; the units of the target period's rule; the units a fraction at the target is taken of; the train's capacity; and the tables the composer keeps a unit. It does not enter the cancel's oracle, which steps one unit, the settling's criterion, which is a fraction of the sum, or any rule's range.
6. **The critic's rule** (`critic_for`) is ADR-0167's: the least sum with `units × 32 ≤ 2^sum`, ADR-0132's scale, and nothing where the shift's dead band is not under one per cent of the reward. The gate pins a stimulus of 65 units as nothing.
7. **The capacities of a run's configuration** (`config`), and what the engine does at each:

   | Capacity | In `config` | Reached when | What the engine does |
   | :--- | :--- | :--- | :--- |
   | the injector ring | 4 096 entries, whatever the size | more messages are injected before one tick than it holds; worker 0 empties at most one ring's worth a tick | `InjectError::Full`, which `Task::trial`, `Drive::step` and `run_driven` return |
   | a worker's nodes | 65 536, whatever the size | a tick takes more than a wheel slot's tokens, a ring's worth and a ripple's | the process aborts |
   | the train | the units times the most spikes a unit can hold in a trial, 328 | a window is read after the ring let part of it go | the spike is let go and counted in `train_overwritten` |
   | a worker's trace | 0 | never: with no capacity a worker records nothing and `WorkerReport::dropped` counts nothing | — |

   The wheel slot is not `config`'s: a slot of the harness's engine holds 2 048 tokens whatever the units (`Engine = Executor<2048>`, ADR-0013), and a full slot aborts the process. `tests/reference.rs` runs ADR-0044's network at 4 096 units on the same type.
8. **Every injection of a run is made between ticks, for the next tick alone** (`Task::trial`): the stimulus's two messages a unit before a trial's first tick, a cancel's messages a unit before each tick of its span, and the drive's. The harness takes each result by `expect`, so a refused message fails its test.
9. **Every window read from the train is asserted held** where it is read: a lead-in's window and the target's (`lead_in_window`, `target_lead_in`), the composer's reading, the critic's oracle's and the network's oracle's at every trial.
10. **The widest injection a rule's range allows is a cancel's.** The pick tries the least count at rest, the least at the extreme standing and twice that, each scanned to 32 messages (`LEAST_SCAN`), so at most 64 messages into every unit of a stimulus before one tick. At 1 024 units that is $51 \times 64$ and the drive's 8, 3 272 messages, inside the ring. At 2 048 it is $102 \times 64$ and the drive's 16, **6 544, past the ring's 4 096**. On the three networks at 1 024 units the widest cancel tried carried 6 messages a unit; a count of 40 and more at 2 048 units would be refused.

## Decision Drivers

- H-34's stopping rule: the size, the seeds and the constants committed before any rule is run at the new size; no constant moves after one; no second attempt, no other seed and no other size.
- **One change from H-33**: the size.
- **No pinned number of the tree moves** but the one ADR-0168 names, and that is to hold by construction before a run checks it.
- **A rule is run as the code has it**, at a size it was not written at; the critic's alone is changed, exactly as ADR-0168 writes it.
- **A harness that fails is not a rule that returned nothing.** What can only fail for the harness's own reason at the new size is found before the protocol is committed, since nothing may be changed after a rule is run there.
- **No capacity is reached silently**, and the round shows each was not reached.
- The round is evidenced as [ADR-0150](0150-a-round-waits-for-what-it-checks.md) says. Latest ≠ Newest: nothing is adopted.

## Considered Options

1. **How the derivation takes its size**: (a) a form given the size, which `derive` calls with 1 024; (b) a parameter of `derive`, every call given 1 024.
2. **How the four functions beneath it are reached at another size**: (a) each as its smallest change — a form given the size for two, the prior's own size for the probe, and the assertion of 1 024 moved to the function that passes the constants pinned at 1 024; (b) a second path written for the new size.
3. **The critic's rule**: (a) as ADR-0168 writes it, the shift held at the largest whose dead band is under one per cent of the reward; nothing else is this round's.
4. **The injector ring at 2 048 units**: (a) sized by the units past 1 024, four entries a unit, with 1 024's and 256's as they were; (b) left at 4 096, a refusal failing the test; (c) the cancel's scan narrowed at the new size.
5. **How a derivation with no pinned table is held**: (a) by the rule, run again in every test that needs the network, and by the first run's tables pinned, as ADR-0167; (b) by its image's CRC alone.
6. **How the networks' derivations and arms are dealt into tests**: (a) a test for each network's derivation and one for each arm, each arm deriving its network first, as ADR-0167; (b) an image kept between tests.
7. **The critic's readings**: (a) read beside the run at every trial, from the weights and the window's counts, at 2 048 units and on H-33's four arms at 1 024; (b) estimated from the value's course alone.
8. **The gate**: (a) one test that synthesizes no network at 2 048 units; (b) a few trials at the new size.

## Decision Outcome

**Options 1(a), 2(a), 3(a), 4(a), 5(a), 6(a), 7(a) and 8(a).**

### The derivation given its size (Implemented in the tests)

- **`derive_at(units, seed, name)`** is `derive`'s body with the size given in each of the sixteen places. **`derive(seed, name)` calls it with 1 024**, so the three tests that call `derive` are the calls they were.
- **`target_lead_in_at(zero, size)`** and **`fed_run_at(units, …)`** are forms given the size; `target_lead_in` and `fed_run_on` call them with 1 024. `fed_run_at` also takes a reading of the caller's, called at each trial's end after the run's own; `fed_run_on` passes one that reads nothing.
- **`probe_task_on`** builds the probe at the prior's own size, `p.units`, where it wrote 1 024. Every prior it was given is at 1 024 units.
- **The assertion that a rewarded run is at 1 024 units moves from `earned_run_held_on` to `earned_run_held`**, the function that passes `DERIVED_1024`, the cancel and the counts pinned at that size. The form holds the size it is given to the engine's own. This is no pinned number; on every path the tree ran, the same assertion is made of the same value.
- **A network is a `Site`**: its size, its prior's seed, the name its dumps carry and its pinned tables. `site_derivation` and `site_arm` are brief 067's `seeded_derivation` and `seeded_arm` over a site; H-33's two networks are sites at 1 024 units (`seeded_site`) whose tables are the ones that round pinned, and H-34's two are sites at 2 048 (`twice_site`).

### The critic's rule (option 3(a), ADR-0168's)

`critic_for(units)`, with the sum found as before:
- **the shift is the lesser of the sum less ADR-0132's scale and `SHIFT_HELD`**, 9, which compile-time assertions hold to be the largest shift whose dead band is under one per cent of the reward;
- **the scale is the sum less the shift**;
- **nothing** where no sum of the range fits, where the sum is below ADR-0132's scale, and where `ValueCritic::is_valid` refuses.

| A stimulus of | The rule returns |
| :--- | :--- |
| no unit | nothing |
| 4 units | 5 and 2, as it did |
| 32 | 8 and 2, as it did |
| 33, 51, 64 | 9 and 2, as it did |
| 65, 101, 102, 128 | 9 and 3 |
| 129, 256 | 9 and 4 |
| 262 144 | 9 and 14 |
| 262 145 | nothing: the scale would be 15, past the loader's 14 |

- **One assertion of the tree moves**, the one ADR-0168 names: in H-33's gate a stimulus of 65 units read nothing and reads 9 and 3.
- The gate restates the rule as ADR-0167 wrote it and holds the two equal for every stimulus of 0 to 64 units; for 65 to 4 096 it holds that the old rule returned nothing and the new one a shift of 9, a scale past 2, and the least sum that fits.

### The rules, their ranges at 2 048 units and what each returns when nothing passes

`derive_at` runs them in ADR-0165's order and stops at the first that returns nothing. **No range moves with the size.**

| # | The constant | The rule | The range, at either size | What the size changes | Nothing when |
| ---: | :--- | :--- | :--- | :--- | :--- |
| 1 | the rotation | `first_balanced` over the prior's census | 0 to 19, one period | 102 whole periods from a rotation of at most eight, 101 from a later one | no rotation of the period is balanced |
| 2 | the gain | `first_calibrated` | 1.75, then 2.0 | the calibration block's network and its volley | neither is calibrated |
| 3 | the settled network | `settled_within`, then `quiet_within` | 160 windows; $2^{17}$ quiet ticks | the drive is 16 messages a tick | no window settles, or the network does not fall quiet |
| 4 | the cancel's span | `span_of` the census | the 100 ticks before the readout window opens | — | the block holds no volley spike |
| 4 | the cancel's candidates | `cancel_candidates` | 1 to 32 messages | nothing: the oracle steps one unit | either scan finds none |
| 4 | the cancel's messages | `first_firing_once` | the three candidates, in order | the clauses are a stimulus of 102 or 101 units' | none fires once |
| 4 | the frozen block | `frozen_measures` | 64 trials | the same | a measure fails: clause 0 fails and the network stops |
| 5 | the target period | `target_period` | $2^{20}$ ticks | the units of the exposure | never: the rule clamps |
| 6 | the critic's window | `shortest_delay` | every synapse | — | the image carries none |
| 6 | the critic | `critic_for` | sums of 0 to 44 | ADR-0168's split: 9 and 3 for 102 and for 101 units | see the table above |

### What holds a derivation at the new size

As [ADR-0167](0167-another-network-of-the-same-size-measured.md) held one on a new seed (option 5(a)): the rule itself, and the first run's tables. Each network's derivation is a weekly test that dumps everything before it holds anything and pins what each rule returned and every table it was read from. Each arm's test derives its network again and holds the derivation to those pins before its first trial. The dispatched run reproduces both.

### The capacities at 2 048 units (option 4(a))

| Capacity | At 2 048 units | Could a run reach it | How the round shows it was not |
| :--- | :--- | :--- | :--- |
| the injector ring | **8 192 entries**, sized by the units | no: the widest injection the rules' ranges allow is 6 544 | every injection's result is taken by `expect`; a compile-time assertion holds `widest_injection` inside `injector_entries` at 256, 1 024 and 2 048 units |
| a worker's nodes | 65 536, as it was | no: a tick takes at most a slot's 2 048 tokens, a ring's 8 192 and a ripple's 24 | the gate holds that sum inside the pool; exhausting it aborts the test's process |
| the train | 671 744 spikes, by the units | no window is read past what it holds | each reading asserts its window held; each arm reads the most spikes one trial put in the train beside the capacity |
| a worker's trace | 0, as it was | there is none | each arm shuts its executor down at its end and holds every worker's `dropped` to zero |
| a wheel slot, the engine's | 2 048 tokens, as it was | a token is one synapse of a unit that fired, due at its delay; a slot fills when 2 048 fall due on one tick from one worker's units | a full slot aborts the test's process |

- **The injector ring is the one capacity sized by the units**, beside the train: `injector_entries(units)` is 4 096 up to 1 024 units, as every run had it, and four entries a unit past that. At 256 and at 1 024 units `config` returns what it returned.
- **Why it is sized and not left**: the ring does not enter the dynamics, it only bounds what one tick takes. Left at 4 096, a cancel of 40 messages a unit and more would fail its test on the harness's ring and not by any rule, and nothing may be changed after a rule is run at the new size.

### The protocol

- **The networks**: `TWICE_SEEDS`, 22 and 23, at `TWICE_SIZE`, 2 048 units. This round synthesized, read and ran nothing at that size before this ADR's commit. ADR-0168 reports a probe, not in the tree, that ran six windows of the settling at 2 048 units for its seconds; it is the one run at that size before this commit that the tree records, and no rule's return was read from it.
- **The order**: on each network the derivations as the table above numbers them, then the arms.
- **The derivations' tests**: `the_rules_on_seed_22_at_2048_units_exhaustive` and `the_rules_on_seed_23_at_2048_units_exhaustive`.
- **The arms** (`TWICE_ARMS`, H-25's two): `the_address_drawn_on_seed_{22,23}_from_the_{assignment,mirrored_assignment}_at_2048_units_exhaustive`, four weekly tests, each run once. An arm exists on a network whose clause 0 holds; on one where it does not, its two tests are removed with the pins and this ADR says so.
- **An arm's run**: H-33's (`site_arm`) at the new size — the network derived again and held to its pins; H-25's image of it decoded and held to the derived configuration; then 7 680 trials under `Delivery::Drawn` with no task critic and H-20's flips, the task's seed 27 and the drive's 3, the drawn sources held to the units the critic's oracle counted and the composer and the network's oracle to the record at every trial.
- **The clauses**, by rule: `Rules::hold` for clause 0; `drawn_of`, H-25's `drawn` against the network's own image, for clauses 1 to 4; `seeded` a network's verdict and `seeded_step` the step of the stopping rule, whose steps 3 to 7 are numbered as ADR-0165's were.
- **The constants**: 80 of 128 (`REWARDED_MIN`); 1.30 (`BOUND_PER_CENT`); 40 of 64 within 23 blocks (`CROSSING_MARK`, `REVERSAL_BLOCKS_MAX`); 0.75 and 1.25 (`BAND_QUARTERS`); 56 of 64 (`SEEN_MIN`). Each is H-25's or ADR-0165's and none moves.
- **The readings, no clause**, as ADR-0168 lists them:
  - *each derived constant beside the three networks' at 1 024* (`RULES_1024`, `RULES_SEEDED`), with the tables beside seed 22's;
  - *the frozen block*: the readouts' spikes after the volley and before the injection and the trials seen (`measure`), the selections by readout and the ties (`frozen_selections`), and **the margin** (`frozen_margins`): by stimulus, the difference of the two readouts' counts summed in magnitude over its trials, beside the two counts summed; all beside the three networks' at 1 024;
  - *per arm* (`SeededRead`): the correct trials of each mapping's last 128; the block each mapping passes 40 of 64 in; the couplings' courses and each mapping's highest; each stimulus's value over each mapping's last 128 beside $2p - 1$, and the value's troughs; the inhibitory sum's course; the reach; the drawn sources beside the host's;
  - *the critic's step* (`Stepped`, `CriticRead`, option 7(a)): at every trial's end the window's counts by unit are read from the train, and the value of those counts under the weights before the reward and under the weights after it. The first is held to the value the engine read at that reward, at every trial, and the units counted to the sources the delivery drew. **What a presentation moves the value by** is the second less the first, summed in magnitude over a mapping's trials, over the errors summed in magnitude; the rule sets it at the volley's units over $2^{\text{shift} + \text{scale}}$, 249 parts in ten thousand at either size. **The units the critic counts a trial** are read beside the volley's, with the spikes it counts;
  - *the capacities*: the most spikes one trial's ticks put in the train, beside the train's;
  - *an arm's seconds* in the dispatch beside twice an arm's at 1 024.
- **The critic's step is read on H-33's four arms too**, at 1 024 units, by the same code: they dump it in the calibration, and it is pinned from there beside H-34's.
- **An arm's pinned tables**, by block: the 120 blocks; the earned blocks; the engine's value; the drawn sources; in one number each the accuracy sequence, every trial's reading, every trial's value, every trial's drawn sources and the compositions; the readings as written; and the critic's reading.

### The calibration (H-34's stopping rule, step 2)

- **Every pinned number of the tree holds.** The tree's whole-domain tests run on the developer machine at this protocol's commit, the six at 2 048 units left out, before any rule is run at the new size. H-33's seven are among them, so **at 1 024 units the rules return the constants the tree holds on seeds 22, 23 and 24**, and the four arms of seeds 23 and 24 reproduce their tables through `site_arm` and `fed_run_at`.
- **A calibration that fails stops the round there** as a finding, and no rule is run at the new size.

### The gate

One test, `the_critic_s_rule_at_its_edges_the_clauses_of_h_34_and_the_readings_rules`:
- the size, the seeds and the arms, that a prior at the new size differs from the one read at 1 024 units in its units alone, the ranges and every constant;
- the geometry at 2 048 units from every rotation of the period, the drive, and each capacity of `config` at 256, 1 024 and 2 048 units;
- the critic's rule at its edges and against the rule as ADR-0167 wrote it, and ADR-0132's three reasons under the extension at 102 units equal to what it chose at 51: a dead band of 512, a drift of 12 of 65 536 a trial, weights of 5 140 for the whole reward, and a step of 249 parts in ten thousand;
- clause 0's measures for a stimulus of 102 units at their edges; H-34's verdict and each step over tables written by hand, each of clauses 1 to 4 at its edge — 80 and 79 of 128, 1.30 of the image's and one past it, the 23rd block and the 24th, 0.75 and 1.25 of the outside sum and one past either;
- the readings' rules over tables written by hand.

It synthesizes no network at 2 048 units and runs no rule on one. In H-33's gate the one assertion ADR-0168 names moves.

## Consequences

- Good: the derivation that runs at the new size is the one held at 1 024 units to every table eight rounds pinned, on three seeds, by a calibration that runs it there first.
- Good: what could fail at the new size for the harness's own reason was looked for before anything ran there: four sizes written in place, and one capacity.
- Good: the critic's reading is taken from the weights and the counts at every trial and held to the engine's own value, at both sizes by one piece of code.
- Bad: the harness cannot be run at 2 048 units before the protocol is committed, so that it holds there rests on its holding at 1 024 and on a reading of its code.
- Bad: the injector ring is twice what it was at the new size. It bounds what a tick takes and does not enter a run that stays inside it; the round reads that it did.
- Bad: each arm derives its network before its first trial, about a seventh of an arm again, at twice the cost.
- Neutral: six weekly tests, four of them arms, and one test in the gate.

## Alternatives considered and why rejected

- **Option 1(b), a parameter of `derive`**: three calls changed where none needs to be.
- **Option 2(b), a second path for the new size**: a path no test at 1 024 units would hold to the tree's tables.
- **Option 4(b), the ring left at 4 096**: a refusal would be the harness's and would read as a failed test of a rule. **Option 4(c), the scan narrowed**: a rule's range changed for a size, which is not this round's.
- **Option 5(b), the CRC alone**: a derivation that moved would say only that it had.
- **Option 6(b), an image kept between tests**: the weekly shards run each test in a process of its own.
- **Option 7(b), the step estimated from the value's course**: between two presentations of a stimulus the other stimulus's trials move the background units' weights, so the value's change is not one presentation's.
- **Option 8(b), trials at the new size in the gate**: the gate runs on every push, and no rule is to run at the new size before the protocol is committed.

## Confirmation

`runtime/cortex-runtime/tests/inhibition.rs` holds `derive_at`, the critic's rule, the sites, the six weekly tests at 2 048 units with their tables empty until their one run, and the gate's test; `tests/instrument/harness.rs` holds `injector_entries` and `widest_injection` with their compile-time assertions. Whitepaper 4.107.0 carries this ADR's row in §9. `npm run spec` holds the links, the rows and the version.
