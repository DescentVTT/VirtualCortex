---
status: accepted
date: 2026-10-12
depends-on: ADR-0168
title: Twice the size, measured
kind: measured
hypothesis: H-34
summary: 'H-34 is yes, on both networks, in both arms and in all five clauses: at 2 048 units from seeds 22 and 23 every rule gave a value and H-25''s arms learned every mapping. The learning configuration is its rules'' at twice the size.'
decision-makers: VirtualCortex maintainers
---

# ADR-0169: Twice the size, measured — H-34 is yes, on both networks, in both arms and in all five clauses: on ADR-0044's generator at 2 048 units from seeds 22 and 23 every rule gave a value — the rotation 0, the gain 1.75 and the cancel's 6 messages as at 1 024 units, the settling at the 100th and the 105th window, the cancel's span 10 and 21 ticks, the target period 59 599 and 59 119 ticks, the critic's shift and scale 9 and 3 by ADR-0168's extension — and the frozen block passed both measures; H-25's two arms then learned every mapping, 120 to 127 of each mapping's last 128, with no coupling past 1.30 of its image's, the highest 1.169 and 1.151, every reversal within 9 to 15 blocks of its flip and the network outside the couplings held; step 3 of the stopping rule, the learning configuration is its rules' at twice the size as at the first; the protocol written before any rule was run at 2 048 units, the derivation given its size with every pinned number at 1 024 units as it was but the one assertion ADR-0168 names, and the injector ring, the one capacity of a run's configuration the rules' ranges could reach at 2 048 units, sized by the units; no file under `src/` changed

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
- the readings' rules over tables written by hand;
- since the pins, over the two networks' pinned tables: each rule over the table its derivation pinned returning what the derivation pinned; every cancel the pick tried inside the injector ring; each arm's readings by their rules over its 120 blocks, a selection or a tie and a value for every presentation of every block; the critic's reading written for every arm at either size, a count a trial of each mapping's span and the units counted the sources drawn; and the verdict and its step as written.

It synthesizes no network at 2 048 units and runs no rule on one. In H-33's gate the one assertion ADR-0168 names moves.

### The calibration, as it ran (2026-10-11, UTC)

On the developer machine, at the protocol's commit `fd0f025` with nothing uncommitted, pushed at 18:29:56Z:
- **Before the commit**, through `derive_at` and `site_arm` at 1 024 units: the rules on seed 22 returned `RULES_1024`, every table on the way was the table its round pinned and the image's CRC-64 was H-23's, `0x11586a76f7415129`; and H-33's arm on seed 23 from the assignment reproduced its tables.
- **Every pinned number of the tree**: the tree's 102 whole-domain tests but the six at 2 048 units, from 18:30:13Z to 22:22:43Z. Ninety-six ran under `cargo test --workspace --release --locked -- --ignored exhaustive --skip at_2048_units --skip on_seed_23 --skip on_seed_24`; H-33's six ran beside them, each in a process of its own with its dumps kept. All 102 passed. The workspace's other tests passed before it in the debug profile, in the release profile and on the MSRV, 732 passed and 148 ignored in each.
- **At 1 024 units the rules returned the constants the tree holds on seeds 22, 23 and 24**, and the four arms of seeds 23 and 24 reproduced their tables through `site_arm` and `fed_run_at`.
- **No rule had run at 2 048 units**: the first did at 22:23:13Z.

### The derivations: clause 0 holds on both networks

Each network's derivation ran once, the two side by side from 22:23:13Z, in 426 and 440 s. Every rule gave a value on both, and the frozen block passed both measures on both.

| The rule returned | Seed 22, 1 024 | Seed 23, 1 024 | Seed 24, 1 024 | **Seed 22, 2 048** | **Seed 23, 2 048** |
| :--- | ---: | ---: | ---: | ---: | ---: |
| The rotation | 0 | 0 | 0 | **0** | **0** |
| — the four couplings by census there, the largest over the smallest | 1.039 | 1.074 | 1.059 | 1.038 | 1.021 |
| The gain | 1.75 | 1.75 | 1.75 | **1.75** | **1.75** |
| — its calibration block's trials seen, of 64 | 62 | 63 | 62 | 63 | 63 |
| The settled network, at window | 96 | 106 | 99 | **100** | **105** |
| — the excitatory sum settled, of the prior's | 0.925 | 0.920 | 0.925 | 0.922 | 0.919 |
| — the inhibitory sum settled, of the prior's | 0.775 | 0.751 | 0.767 | 0.764 | 0.754 |
| — the quiet run, ticks | 2 500 | 2 519 | 2 424 | 2 538 | 2 489 |
| The cancel's span, ticks | 9 | 8 | 12 | **10** | **21** |
| The least messages at rest, and at the extreme standing | 3, 6 | 3, 6 | 3, 6 | **3, 6** | **3, 6** |
| The cancel's messages, the pick | 6, the second candidate | 6, the second | 6, the second | **6, the second** | **6, the second** |
| — the first candidate's stimulus spikes after the opening | 2 717 | 2 745 | 2 746 | 5 461 | 5 438 |
| The frozen block: the stimulus's spikes after the opening | 0 | 0 | 1 | 0 | 1 |
| The frozen block: trials seen, of 64 | 62 | 64 | 64 | **64** | **64** |
| The settled network's spikes over the target's lead-in | 18 449 | 18 043 | 18 144 | 36 032 | 36 325 |
| — a unit | 18.02 | 17.62 | 17.72 | 17.59 | 17.74 |
| The target period, ticks | 58 201 | 59 510 | 59 179 | **59 599** | **59 119** |
| The critic's window, ticks | 100 | 100 | 100 | **100** | **100** |
| The critic's shift and scale | 9, 2 | 9, 2 | 9, 2 | **9, 3** | **9, 3** |

- **Four constants came back as at 1 024 units on both networks**: the rotation, the gain, the cancel's messages and the critic's window. **The critic's constants are the extension's**, 9 and 3 for a stimulus of 102 units, as ADR-0168's table gives them. The three that differed among the networks at 1 024 units differ here too, inside or beside the range read there: the window the network settles at, the cancel's span and the target period.
- **The two rules ADR-0168 named as least known returned what they returned at 1 024 units.** The rotation's scan stopped at its first rotation, the census's four couplings within 3.8 and 2.1 per cent of one another. The gain's first candidate, 1.75, was calibrated, the stimulus seen in 63 trials of 64 on each network. Neither rule was read returning another value.
- **The settled network is the same network a unit.** The excitatory and the inhibitory sums settle at the same fractions of the prior's (the two inhibitory fractions at 1 024 units are read here from the pinned sums, 0.7755 and 0.7515; ADR-0167's table gives them a thousandth higher), the quiet run takes as long, and a unit fires 17.6 and 17.7 times over the target's lead-in where it fired 17.6 to 18.0, so the target period lies among the three read at 1 024.
- **The cancel's span is 21 ticks on seed 23**, the longest read: the volley's spikes fall on ticks 1 to 8 but for two units once each on ticks 10 and 11 and one unit once on tick 21, of 6 514 volley spikes. On seed 22 it is 10, ticks 1 to 7 and one unit each on 9 and 10. Over 10 or 21 ticks the oracle's least counts are the same, 3 and 6.
- **The pick took the second candidate on both**, for the reason it did at 1 024: under the first the stimulus fires again after the opening, 5 461 and 5 438 spikes over the block where a tenth of a spike a unit a presentation allows 652. The widest cancel tried was 6 messages into each of 102 units before a tick, 612 messages and the drive's 16, of the ring's 8 192; the 4 096 every run had would have held it.
- **The synapses a rewarded run holds**, from each stimulus onto each readout: 1 549, 1 621, 1 600 and 1 581 on seed 22; 1 595, 1 577, 1 613 and 1 576 on seed 23; twice the 767 to 824 read at 1 024.
- **The image each arm decodes**: CRC-64 `0x3300859e3fabf553` on seed 22 and `0xf1ea58899baaaab1` on seed 23.

**The frozen block from the settled image**, sixty-four trials, beside the three networks' at 1 024:

| | Seed 22, 1 024 | Seed 23, 1 024 | Seed 24, 1 024 | **Seed 22, 2 048** | **Seed 23, 2 048** |
| :--- | ---: | ---: | ---: | ---: | ---: |
| The volley's spikes a presentation, A and B, of the stimulus's units | 50.82, 50.83 of 51 | 50.71, 50.90 | 50.82, 50.87 | 101.65, 101.77 of 102 | 101.74, 101.70 |
| The readouts' spikes after the volley, readout 0 and 1 | 645, 637 | 700, 693 | 683, 673 | 1 267, 1 345 | 1 409, 1 325 |
| The readouts' spikes before the injection | 258, 256 | 250, 224 | 224, 232 | 489, 425 | 487, 473 |
| Trials seen, of 64 | 62 | 64 | 64 | 64 | 64 |
| ADR-0072's sign, trials of 64 | 53 | 61 | 56 | 62 | 64 |
| A's selections: readout 0, readout 1, tie | 14, 15, 5 | 12, 18, 4 | 20, 10, 4 | 15, 16, 3 | 25, 9, 0 |
| B's selections: readout 0, readout 1, tie | 12, 15, 3 | 17, 10, 3 | 10, 18, 2 | 9, 19, 2 | 16, 13, 1 |
| The margin a trial, A and B, spikes | 3.09, 4.03 | 3.15, 4.27 | 3.62, 4.30 | 4.32, 4.90 | 5.79, 5.23 |
| The two counts a trial, A and B, spikes | 19.2, 21.0 | 20.1, 23.7 | 20.4, 22.1 | 40.0, 41.7 | 41.0, 44.6 |
| The margin, of the two counts | 0.161, 0.192 | 0.157, 0.180 | 0.177, 0.195 | 0.108, 0.118 | 0.141, 0.117 |

- **The readouts fire twice the spikes**, 40 to 45 a trial where they fired 19 to 24, as twice the units do.
- **The margin grows less than the counts**: 4.3 to 5.8 spikes a trial where it was 3.1 to 4.3, so it is 0.11 to 0.14 of the two counts where it was 0.16 to 0.20. A selection is read on twice the spikes and stands further from a tie in spikes, and **the ties fall** from 2 to 5 of a stimulus's trials to 0 to 3.
- **The frozen networks lean, as at 1 024.** Seed 22's selects readout 1 more for both stimuli, by one trial for A and in 19 trials to 9 for B; at 1 024 units it leant to readout 1 for both too. Seed 23's selects readout 0 for both, A in 25 of its 34 trials; at 1 024 it leant to the mirrored assignment.
- **The sign, which seed 22's settled network failed at 53 at 1 024 units, reads 62 and 64.** It is no measure of clause 0.

### H-34's verdict: yes, on both networks

The four arms ran once each, side by side from 22:32:56Z to 23:36:03Z, in 3 761 to 3 786 s (on the developer machine, a ratio and not admissible; four arms at 1 024 units side by side took 1 915 to 1 935 s there in ADR-0167's round). Each derived its network again, held the derivation to the pins above before its first trial, ran its 7 680 trials with every oracle held at every trial, held the value of the window's counts to the engine's own and the units counted to the sources drawn at every trial, dumped everything and stopped at its first empty table. The tables were written from those dumps, and every filled table was held to the text of its dump by a script. The pins are `352e4d1`.

| Network, arm | Clause 1: correct of each mapping's last 128 | Clause 2: the highest coupling, of its image's | Clause 3: blocks to 40 of 64 after each flip | Clause 4: the outside sum, of the image's |
| :--- | :--- | :--- | :--- | :--- |
| **Seed 22 at 2 048, from the assignment** | 121, 122, 123, 125 | 1.133, block 88, A→R0 | 15, 15, 10 | 0.9999 to 1.0000 |
| **Seed 22 at 2 048, from the mirrored assignment** | 124, 125, 127, 123 | 1.169, block 119, A→R0 | 13, 13, 12 | 0.9999 to 1.0000 |
| **Seed 23 at 2 048, from the assignment** | 126, 127, 126, 125 | 1.137, block 119, B→R0 | 13, 9, 12 | 0.9999 to 1.0001 |
| **Seed 23 at 2 048, from the mirrored assignment** | 121, 120, 127, 126 | 1.151, block 82, A→R1 | 14, 12, 14 | 0.9998 to 1.0000 |
| *Seed 22 at 1 024 (H-25), from the assignment* | *122, 124, 119, 116* | *1.181* | *19, 21, 16* | — |
| *Seed 22 at 1 024 (H-25), from the mirrored assignment* | *124, 118, 123, 117* | *1.191* | *20, 15, 14* | — |
| *Seed 23 at 1 024 (H-33), from the assignment* | *126, 124, 124, 123* | *1.183* | *15, 13, 13* | *0.9999 to 1.0000* |
| *Seed 23 at 1 024 (H-33), from the mirrored assignment* | *122, 115, 118, 126* | *1.182* | *16, 14, 12* | *0.9999 to 1.0001* |

- `TWICE_2048`: on each network `Seeded { derived: true, arms: Some(Drawn { learning: Scheduled { learned: [[true; 4]; 2], bounded: [true; 2], over: [None; 2], yes: true }, revised: [[true; 3]; 2], left: [None; 2], held: [true; 2], yes: true }), yes: true }`; `TWICE_STEP_2048` is 3.
- **H-34 is yes**, with its scope: H-25's task, arms and schedule; 2 048 units; ADR-0044's generator with every parameter but the size; the drive's rule at seed 3 and the task's seed 27; two networks, seeds 22 and 23; and the critic's constants ADR-0168's extension gives a stimulus of 102 units. ADR-0168 wrote no prediction for it.
- **No clause is near its edge.** The least-learned mapping reads 120 of its last 128 where 80 are asked; the highest coupling stands at 1.169 where 1.30 is the bound; the slowest reversal takes 15 blocks where 23 are allowed.
- **Two things changed in the tests with the pins, beside the tables and the gate's checks over them**, neither a reading. `config` asserts that the ring it sizes holds the widest injection at its size, and the gate holds `TWICE_PREDICTED` to none at run time: on the MSRV toolchain a use inside a compile-time assertion alone left `widest_injection` and the constant reported as never used. The assertion in `config` reads two functions of the size and is true at every size; the dispatched run reproduces every table on that code.

### The readings

**The reversals are no slower at twice the size, and on these two networks they are sooner.** The twelve take 9 to 15 blocks; the eighteen read at 1 024 units took 12 to 21. On the same seed: seed 22's six average 13.0 blocks where they averaged 17.5 at 1 024, and seed 23's 12.3 where they averaged 13.8. Two networks a size are too few to call it the size's doing.

**The first mapping** passes 40 of 64 in its 4th and 2nd block on seed 22, from the assignment and from the mirrored assignment, and in its 1st and 4th on seed 23; H-25 read 6 and 4, and H-33 4 and 3, and 1 and 8. On seed 23, whose frozen network selects readout 0 for stimulus A in 25 of 34 trials, which is A's answer under the assignment, the arm that starts from the assignment reads 40 correct in its first block and the mirrored one 24. Seed 22's two arms read 32 and 31.

**The couplings move less.** Over the four arms the highest stands at 1.133 to 1.169 of its image's and the lowest at 0.929 to 0.957; at 1 024 units the six arms' highest stood at 1.181 to 1.195 and their lowest at 0.895 to 0.955. At each mapping's end the answer's two pairs stand at 1.07 to 1.16 of their image's and the other two at 0.93 to 1.01, where they stood at 1.11 to 1.20 and at 0.89 to 1.01. The mappings are learned as well or better with the couplings moved less far.

**The critic's constants, ADR-0168's extension, give the step ADR-0132's gave.** Read at every trial from the weights before and after the reward and the window's counts, and held to the engine's own value at every one of the 30 720 trials at this size and the 30 720 at 1 024:

| Per mapping, over the four arms | At 1 024 units, seeds 23 and 24, under 9 and 2 | **At 2 048 units, seeds 22 and 23, under 9 and 3** |
| :--- | ---: | ---: |
| What a presentation moves the value by, parts in ten thousand of its error | 253 to 256 | **253 to 255** |
| — the rule's, the volley's units over $2^{\text{shift} + \text{scale}}$ | 249 | 249 |
| The units the window counts a trial | 52.40 to 52.50 | **104.87 to 104.99** |
| — the volley's | 51 | 102 |
| Trials whose value moved against its error's sign | 0 | **0** |

- The step is a fortieth of the error at either size, as ADR-0168's arithmetic had it; what it reads over the rule's 249 is the units the window counts beside the volley's.
- **The value beside $2p - 1$.** Over each mapping's last 128 trials each stimulus's mean value lies within 0.10 of the reward of $2p - 1$, in all thirty-two cases; at 1 024 units H-33 read within 0.14, and H-23's bound, a reading here, is a quarter. After a flip the value's trough is −0.59 to −0.81 of the reward, where H-25 and H-33 read −0.56 to −0.89, and it stays above zero from the 10th to the 16th block after a flip, where they read the 13th to the 24th.

**The drawn sources** are the presented stimulus's units in 99.66 to 99.68 per cent of their presentations, as at 1 024, with 3.25 to 3.26 other units a trial where 1 024 units read 1.62: twice the background units fire in the window.

**The inhibitory sum** rises first, to 1.017 to 1.021 of the image's by the 15th to 17th block, and then falls through the run to 0.892 and 0.891 on seed 22 and 0.902 and 0.902 on seed 23. At 1 024 units it ended at 0.848 and 0.850 on seed 22, 0.908 and 0.910 on seed 23 and 0.896 and 0.897 on seed 24. It did not fall in every block in any arm.

**The stimulus** fires once by ADR-0074's measure in all 120 blocks of each arm. **Ties** are 28 to 65 trials a mapping, where H-33 read 49 to 117. **The reach**: of the four pairs' synapses, 6 349 and 6 350 of 6 351 ended away from the image's weight on seed 22, and 6 360 and 6 358 of 6 361 on seed 23; 19 987 to 21 276 excitatory synapses outside the pairs ended away from it too, and their sum stayed within two parts in ten thousand of the image's.

**The capacities were not reached.**
- *The injector ring*: every injection's result was taken, and none was refused in any run. The widest cancel tried carried 6 messages a unit, 628 messages before a tick with the drive's, of the ring's 8 192.
- *The train*: every reading asserted its window held. The most spikes one trial's ticks put in the train were 857 to 888 over the four arms, of the train's 671 744; at 1 024 units, 447 to 471 of 335 872.
- *A worker's trace*: each arm's test reaches the shutdown of its executor only when every table holds, which the first run's did not; the dispatched run's four arms did, and each held every worker's report to no entry dropped, as H-33's four at 1 024 units did in the calibration.
- *A worker's nodes and a wheel slot*: no test's process aborted.

### What is read from the tables and not measured

- **Why the reversals are sooner and the couplings move less** was not varied. The frozen block reads a selection on twice the spikes with fewer ties, and a selection that needs less of a coupling's change to turn would give both; no arm was run that separates it from the two networks' own difference.
- **Whether the rules transfer to this size where they would return another value** is read for the same three as at 1 024 and for the critic's. The rotation, the gain, the cancel's messages and the critic's window came back as on all three networks at 1 024, so those rules were tested here in that they returned a value and the value worked.
- **The critic's extension is read at one stimulus size**, 102 units. Its step is ADR-0132's there; 129 units and more, where the scale is 4, are not read.
- **Two networks.** A yes on two is not a rate.

### The step reached

**Step 3 of H-34's stopping rule: yes.** The learning configuration is its rules' at twice the size as at the first: on two networks at 2 048 units the rules derived it, with the critic's constants by ADR-0168's extension, and it learned and revised as at 1 024.

**The next decision is an ADR choosing among a larger size still, which needs the weekly job's bound decided first, the operating regime, a reward delayed past a trial and a less reliable reward.** It is named and not taken. Steps 4 to 7 did not arise, and step 8 is kept: no constant moved after a rule was run at the new size, there was no second attempt, and no other seed and no other size was run.

### The evidence

- **The dispatch's scope.** The diff changes no file under `src/`, so by [ADR-0075](0075-the-dispatch-scope-follows-the-diff.md) the weekly was dispatched on this round's branch at **`scope=exhaustive`**, as brief 068 asks: run [38185502052](https://github.com/DescentVTT/VirtualCortex/actions/runs/38185502052) at `352e4d1`, the pins, at 23:45:52Z on 2026-10-11. It ran the twelve whole-domain shards and no sweep, so no sweep's outcome is owed for it.
- **Every whole-domain shard is green**, 23:45:56Z on 2026-10-11 to 01:16:51Z on 2026-10-12. All 108 `exhaustive` tests passed, each exit 0 with one test reported. The 102 before this round reproduced their pinned numbers on the hosted runners through the derivation given its size, the sites and the ring sized by the units; the rules on seed 22 at 1 024 units returned the tree's constants there, in 236 s, and H-33's two derivations and four arms reproduced their tables, the critic's reading among them. **The two derivations at 2 048 units reproduced their tables**, in 476 and 489 s. **The four arms reproduced theirs**, each deriving its network again first, and each held every worker's report to no entry dropped at its end: 2 151 and 3 333 s on seed 22, from the assignment and from the mirrored assignment, and 3 580 and 2 879 s on seed 23.
- **An arm's seconds beside twice an arm's at 1 024.** In the same run the six arms of H-25 and H-33 at 1 024 units took 1 462 to 1 886 s, 1 714 on average, and twice that is 3 428. The four at 2 048 took 2 151 to 3 580 s, 2 986 on average: 1.74 times, under ADR-0168's expectation of twice. The hosted runners differ by 29 per cent between like arms at 1 024 units, 1 462 s against 1 886, which is wider than that difference; on the developer machine, four arms side by side, the ratio read 1.96. **The longest arm is 50 per cent of a shard's bound by itself**, and the shortest 30.
- **The shards**, dealt by the table before this round, which did not know the six tests and costed each at 900 s:

  | Shard | Job | Its two heaviest tests (s) | Tests | Their seconds summed | Tests' wall time |
  | ---: | ---: | :--- | ---: | ---: | ---: |
  | 0 | 37.3 min | H-31 from the assignment 1 924; H-29 from the mirrored 1 418 | 8 | 4 285 | 2 158 s, 30 % |
  | 1 | 66.0 min | H-31 from the mirrored 2 043; H-25 from the assignment 1 825 | 9 | 7 121 | 3 868 s, 54 % |
  | 2 | 59.4 min | H-22 from the assignment 1 644; H-22 from the mirrored 1 633 | 8 | 6 369 | 3 469 s, 48 % |
  | 3 | 72.7 min | **H-34 on seed 22 from the mirrored 3 333**; H-30 from the mirrored 1 698 | 10 | 8 133 | 4 262 s, 59 % |
  | 4 | 62.3 min | H-33 on seed 23 from the mirrored 1 886; H-24 from the mirrored 1 830 | 9 | 7 196 | 3 641 s, 51 % |
  | 5 | 56.1 min | H-25 from the mirrored 1 823; H-21 from the mirrored 1 796 | 9 | 6 017 | 3 271 s, 45 % |
  | 6 | 54.5 min | H-33 on seed 24 from the assignment 1 815; H-20 from the mirrored 1 358 | 9 | 5 679 | 3 173 s, 44 % |
  | 7 | 43.5 min | **H-34 on seed 22 from the assignment 2 151**; H-26 from the assignment 1 079 | 9 | 5 069 | 2 542 s, 35 % |
  | 8 | 61.3 min | **H-34 on seed 23 from the mirrored 2 879**; H-30 from the assignment 1 423 | 9 | 6 909 | 3 594 s, 50 % |
  | 9 | 56.5 min | H-28 from the mirrored 1 823; H-28 from the assignment 1 822 | 10 | 6 083 | 3 295 s, 46 % |
  | 10 | 90.9 min | **H-34 on seed 23 from the assignment 3 580**; H-27 from the assignment 1 873 | 10 | 10 409 | **5 368 s, 75 %** |
  | 11 | 56.5 min | H-32 from the mirrored 1 816; H-23 from the mirrored 1 802 | 8 | 6 220 | 3 295 s, 46 % |

  The percentages are of the job's bound, 120 minutes; every shard ran its tests two at a time, at 1.79 to 1.99 of their summed seconds over the wall. **One shard passed 60 per cent of its bound: shard 10, at 75**, 5 368 s of 7 200. The arm that put it there is H-34's on seed 23 from the assignment, 3 580 s, which the deal had costed at 900 s and set beside nine other tests; the shard's ten summed 10 409 s. Shard 3, with the arm of 3 333 s, read 59. As brief 068 directs, the round reports it and goes on: a shard's bound against its longest test is the workflow's next decision, which ADR-0168 names.
- **The shards started at once.** All twelve started four seconds after the dispatch, and the last ended 1 h 31 m after it.
- **The cost table is regenerated from this run** (`node scripts/exhaustive-costs.mjs from <artifacts> --run 38185502052`): 108 lines, 79 490 s, where the table before it held 102 and 66 471; ADR-0168 expected about 82 000. ADR-0092's deal plans each of the twelve shards at 6 623 to 6 627 s summed, about 3 330 to 3 700 s of wall time at the run's ratios, **46 to 51 per cent of the bound**, inside the brief's 60. A shard that holds the longest arm cannot read under its 50.
- **The pull request's gate** ([200](https://github.com/DescentVTT/VirtualCortex/pull/200)) is green in every job on `fd0f025`, the protocol, and on `6bf744d`, the readings, which holds the Rust sources of `352e4d1`; the determinism pin on AArch64 and the MSRV are among them. The gate's run on `352e4d1` itself was cancelled by the push of `6bf744d` seven minutes after it. **The mutation gate on the changed lines** had nothing to mutate, no line under `src/` being changed, and said so: `No mutants to filter`.
- **On the developer machine**, at `352e4d1`: the workspace's tests passed in the debug profile, in the release profile and on the MSRV, 732 passed and 148 ignored in each; the check, the format, Clippy, the documentation, the benchmarks and `npm run spec` exited 0; 108 whole-domain tests are listed; and `cargo mutants --in-diff` listed no mutant. Before any rule ran at 2 048 units, the tree's 102 other whole-domain tests passed there at `fd0f025`, 18:30:13Z to 22:22:43Z on 2026-10-11. The two derivations then ran once from 22:23:13Z, and the four arms once, 22:32:56Z to 23:36:03Z.
- **The commits**, by the hashes the branch holds, in pull request 200: `fd0f025`, the protocol, the derivation given its size and the critic's rule; `352e4d1`, the derivations' and the arms' tables and the gate's checks over them; `6bf744d`, these readings.

## Consequences

- Good: the second thing the line's results were scoped by, the size, is read at one step: at twice the units, on two networks, the rules derived the configuration and it learned and revised as at the first size.
- Good: the derivation that ran at the new size is the one held at 1 024 units to every table eight rounds pinned, on three seeds, by a calibration that ran it there first.
- Good: the critic's rule past 64 units, which ADR-0168 wrote from ADR-0132's reasons before any run, is read doing what it was written to do: the step a presentation takes of its error is the one it takes at 1 024 units, to the third figure.
- Good: what could fail at the new size for the harness's own reason was looked for before anything ran there, four sizes written in place and one capacity, and nothing did.
- Good: the critic's reading is taken from the weights and the counts at every trial and held to the engine's own value, at both sizes by one piece of code.
- Bad: the two rules ADR-0168 named as least known, the rotation's and the gain's, returned what they returned at 1 024 units. A rule that would have to return another value at a size has still not been read doing so.
- Bad: the injector ring was sized for a cancel no run tried. The widest carried 628 messages before a tick, which the ring every run had would have held; the sizing covers the rule's range and was not needed by these two networks.
- Bad: two networks, one step in size, one generator and one drive. That the reversals are sooner and the couplings move less is read on them and its cause is not.
- Bad: each arm derives its network before its first trial, about a seventh of an arm again, at twice the cost. An arm alone read 30 to 50 per cent of a weekly shard's bound in the dispatch, and the shard dealt the longest beside nine other tests read 75. The regenerated table plans each shard at 46 to 51; a shard's bound against its longest test stays the workflow's next decision.
- Bad: the harness could not be run at 2 048 units before the protocol was committed, so that it held there rested on its holding at 1 024 and on a reading of its code. It held.
- Neutral: six weekly tests, four of them arms, and one test in the gate.
- Neutral: the frozen networks lean at this size too, and the first mapping follows the lean where there is one.

## Alternatives considered and why rejected

- **Option 1(b), a parameter of `derive`**: three calls changed where none needs to be.
- **Option 2(b), a second path for the new size**: a path no test at 1 024 units would hold to the tree's tables.
- **Option 4(b), the ring left at 4 096**: a refusal would be the harness's and would read as a failed test of a rule. **Option 4(c), the scan narrowed**: a rule's range changed for a size, which is not this round's.
- **Option 5(b), the CRC alone**: a derivation that moved would say only that it had.
- **Option 6(b), an image kept between tests**: the weekly shards run each test in a process of its own.
- **Option 7(b), the step estimated from the value's course**: between two presentations of a stimulus the other stimulus's trials move the background units' weights, so the value's change is not one presentation's.
- **Option 8(b), trials at the new size in the gate**: the gate runs on every push, and no rule is to run at the new size before the protocol is committed.

## Confirmation

`runtime/cortex-runtime/tests/inhibition.rs` holds `derive_at`, the critic's rule, the sites, the six weekly tests at 2 048 units with their pinned tables, and the gate's test, which holds the rules over those tables; `TWICE_2048` and `TWICE_STEP_2048` hold the verdict and its step as written, and `SEEDED_CRITIC` the critic's reading at 1 024 units. `tests/instrument/harness.rs` holds `injector_entries` and `widest_injection` with their compile-time assertions. Whitepaper 4.107.0 carries H-34's verdict in §11.1 and this ADR's row in §9. `npm run spec` holds the links, the rows and the version.
