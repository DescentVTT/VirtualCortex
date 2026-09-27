---
status: accepted
date: 2026-09-27
depends-on: ADR-0116
decision-makers: VirtualCortex maintainers
---

# ADR-0117: The region measured — brief 050's measurement: ADR-0115's marked assemblies under set (ii) at 48, 64, 80 and 96 units and five weights from an eighth to three eighths of Q1.15's range on the settled network, and a subset of six cells under another draw of the delays, on the image H-20's arm leaves and with the task's stimulus presented every epoch, on the geometry F-54's decision gives the context, every weight frozen and ADR-0115's protocol, measures and thresholds unchanged, by rules for a robust cell and for the second round's cell written before any run; the drained image frozen with its dopamine signal put at rest as well (F-56); the usable region is one weight, a quarter of the range, at 64, 80 and 96 units, and no cell is robust — the task's stimulus ignites the context in half the rounds, so does the drained network, and 96 units fail under another draw of the delays — so there is no cell for the second round, and the next decision, among the branches ADR-0116 names for those failures, is named and not taken

## Context and Problem Statement

[ADR-0116](0116-the-region-before-the-readout.md) deferred ADR-0111's second round by one round and asked four things first: where the context sits beside the task, how wide [ADR-0115](0115-the-assemblies-marked.md)'s usable region is, whether it survives another seed, a drained network and the task's own stimulus, and which cell the second round builds on. Brief 050 measures them and changes no rule of the engine. This ADR is its protocol, committed before any run, and then its readings.

What was read before anything was written (principle 2):

1. **The substrate is ADR-0115's** (`runtime/cortex-runtime/tests/assembly.rs`): `members`, `wire`, `grown`, `marked`, the kick, `layout`, `span_protocol`, `class_run`, `background_049`, `holding`, `failed`, `bursts_049`, `SETS`, `RELEASE_STRETCHES_049` and `delay_of`. At every size of brief 050's grid a member sends and receives 32 synapses (`fan` is `min(size - 1, 32)`), so the sizes differ in the members' number and not in their wiring.
2. **The harness is compiled into four binaries**, and one of them (`tests/instrument.rs`) does not allow an unused item, so a geometry added to the harness and read by no test of that binary fails `clippy -D warnings` there (read by adding one and building).
3. **An unwired run draws no delay.** `delay_of` is reached only from `wire`, which neither a background nor a control calls. So condition (b)'s background and control would be the core grid's bit for bit.
4. **An image carries the modulator's dopamine signal** (`Image::encode` writes the modulator's sixteen bytes at `[0..16)` of its section; the loader reads them back). H-20's arm ends on a trial whose reward left a signal. Decoded under the baseline of zero, an excitatory synapse's modulation is the baseline plus the signal clamped to $[0, 1]$ (`NeuromodulatorState::modulation`), so a positive signal would consolidate every pending trace at its next presynaptic spike until the signal decays to rest — by a $2^{-14}$ of itself a tick and at least one LSB, 46 420 ticks from 1.0 — and a measured run's weights would move. ADR-0116 and brief 050 name two bytes to unset for a frozen drained image, the inhibitory baseline and the signed gate, and not this third one: **F-56**, resolved below.
5. **The task presents its stimulus as `Task::trial` does**: before the trial's first tick, `Stimulus::inject` puts the shape's messages into every unit of the drawn set; before each tick, `Stimulus::cancel_at` injects the cancel where it is due and then the drive steps. The stimulus of trial $n$ is `Task::stimulus_at(n)`, the low bit of `mix64(SEED ^ n)`. Both calls are public, so a protocol can present the task's stimulus with the task's own code, and the gate holds it to `Task::trial`.
6. **The stimulus reaches the context's places through the prior**, by the gate's count over the prior's synapses, which the settled image keeps with their weights moved: stimulus A's units send 105 synapses onto place 5 over the ring and 95 onto place 16, and B's 108 and 91. At 48, 64 and 80 units every member receives at least one synapse from a stimulus unit, and at 96 units 95 of the 96 do (`STIMULUS_REACH_050`).
7. **H-20's arm runs from H-20's image by `run_on_scheduled`**; `earned_run_scheduled` wraps it with an oracle that reads the engine and never writes it (`run_on_scheduled`'s `observe`, "it reads and never writes"). The image is the settled image with three bytes and the section's seal changed (`inhibited_image`, `signed_image` in `tests/inhibition.rs`), pinned there by its CRC-64, `PUNISHED_IMAGE_CRC_1024`; ADR-0110 pins the arm's accuracy sequence and its sums after the last block.

## Decision Drivers

- Brief 050's standing directives:
  - no rule of the engine changes, and every weight is frozen in every measured run — the excitatory baseline zero, the inhibitory baseline and the signed gate unset, no reward — each run's weights at its end shown equal to its start;
  - ADR-0115's protocol, spans, release, stretch, measures and thresholds unchanged, and its two cells in the grid reproduced bit for bit before any other cell runs;
  - the grid, the subset, the conditions and the rules written before any run and moved by none; the grid may be widened before the first run and never narrowed;
  - the kick read on the engine under every condition before its cells;
  - no float, every loop ended by construction; the runtime's gate grows by at most one test; no shard of the weekly job past 60 per cent of its bound.
- One difference at a time from ADR-0115: under each condition, one thing differs from the core grid.

## Considered Options

1. **Where the geometry lives**: (a) the harness, beside `geometry`, as ADR-0116 wrote; (b) `tests/assembly.rs`, beside the only runs that read it.
2. **Condition (b)'s background and control**: (a) run again with the seed; (b) the core grid's.
3. **Condition (c)'s image**: (a) H-20's arm from the assignment; (b) H-18's arm, cheaper; each (i) through `earned_run_scheduled` with its oracle, or (ii) through `run_on_scheduled` alone, held to ADR-0110's pins. Frozen with (x) the two bytes ADR-0116 names or (y) those and the signal. ADR-0116's option 3(b), the inhibitory weights scaled, as a reading beside it or not.
4. **Condition (d)'s timing**: the stimulus (a) at every epoch's first tick, where the kick falls at the start of a hold span, or (b) at every epoch's middle.
5. **The grid**: the brief's, or widened.
6. **The weekly tests**: one per size and condition, or fewer, each building the settled image once.

## Decision Outcome

**Options 1(b), 2(b), 3(a)(ii)(y) without 3(b), 4(b), 5 (the brief's grid, not widened) and 6 (nine tests).** Everything below is committed before any run of the measurement, with the arithmetic and the gate.

### The geometry (ADR-0116's decision, built)

- **Readout 0 at `0xAA28A`** (`R0_MASK_050`: 1, 3, 7, 9, 13, 15, 17 and 19) and **readout 1 at `0x45554`** (`R1_MASK_050`: 2, 4, 6, 8, 10, 12, 14 and 18), each the task's mask without the place the context takes; the stimuli at places 0 and 11 as the task's. `context_geometry` returns the four sets over `geometry`'s whole periods from the rotation, and `context_places` the places given up — 102 units at 1 024.
- **Compile-time assertions** hold the masks: eight places each, disjoint, disjoint from `PLACES`, and the two stimuli, the two readouts and the two places covering the period.
- **The gate holds the rest**: the four sets well-formed and pairwise disjoint; 51, 51, 408 and 408 units; the places disjoint from each; 1 020 units covered; a task over the new readouts passing `Task::check`; and every member of every size of the grid at a place given up and in no set, the sizes nested.
- **In `tests/assembly.rs`** (option 1(b)): the harness is compiled into `tests/instrument.rs`, which allows no unused item, and no test there reads the new geometry. `geometry` is unchanged and every earlier test keeps it. The round that runs the task on the new geometry — ADR-0111's third round needs H-20's schedule on it without a context — moves it into the harness beside `geometry`, where that test uses it.

### The core grid

- **ADR-0077's settled image at 1 024 units, delay seed 48, no task.** Set (ii), $(U, \tau_f, \tau_d) = (26/256, 2^{16}, 2^{13})$.
- **Sizes** (`SIZES_050`): 48, 64, 80 and 96 units at ADR-0112's placement, nested — the first $\text{size}/2$ periods' places 5 and 16.
- **Weights** (`WEIGHTS_050`): 0x1000, 0x1800, 0x2000, 0x2800 and 0x3000 in Q1.15, from 0.125 to 0.375 by sixteenths.
- **Each size has its own background and control** (ADR-0115's: the background marked, unwired and not grown, the drive alone; the control grown, marked and unwired, the kick and the release).
- **ADR-0115's two cells** are 64 units at 0.25 and 0.375 (`ADR_0115_CELLS`). The test of 64 units runs them first and holds each to ADR-0115's pinned stretches, kicks, windows' hash, reading and bursts before its other three weights run. 64 units' background and control are held to ADR-0115's pins as well. The pins are ADR-0115's own constants, named, not copied.

### The subset and the three conditions

**The subset** (`SUBSET_SIZES`, `SUBSET_WEIGHTS`): 64 and 96 units at 0.1875, 0.25 and 0.3125, six cells. Under each condition one thing differs from the core grid:

- **(b) another seed**: the assembly's delays drawn with seed 49 (`DELAY_SEED_050`, `wire_drawn`). Every target, weight and link is seed 48's; 3 062 of 3 072 delays differ at 96 units (`SEEDED_WIRING_050`). Its background and control are the core grid's (option 2(b)): an unwired run draws no delay, so a run of them with seed 49 is the core's run bit for bit, and the kick read on the engine under (b) is the core control's. The brief asked for each condition's own; this is that, without running the same ticks twice.
- **(c) a drained network**: the image H-20's arm from the assignment leaves at its 7 680th trial, frozen (`drained_image`). The test builds the settled image, writes H-20's image and holds it to `PUNISHED_IMAGE_CRC_1024` (restated as `H20_IMAGE_CRC`). It runs the arm through `run_on_scheduled` with H-20's task — F-46's shape with ADR-0076's cancel, the answer's feedback, the addressed delivery, the assignment first, the flips before the trials of index 1 536, 3 584 and 5 632, and H-19's critic of shift 5 — and holds the arm to ADR-0110's accuracy sequence and sums after its last block (restated as `H20_TRACE`, `H20_SUMS`) before anything else (option 3(a)(ii)). It then quiets the engine as every image is quieted, writes the image, and freezes it (`frozen_again`): **the signal put at rest, the inhibitory baseline and the signed gate unset** (option 3(y), F-56). The frozen image is read — its sums, its CRC, and the couplings from each stimulus into each place given up, beside the settled image's — and pinned. The arm learned on the task's geometry, where places 5 and 16 were readout places, so the stimuli's couplings onto the members were answer pairs there: a confound, read and recorded as one. Each subset size has its own background and control on this image.
- **(d) the task's stimulus**: on the settled image, the task's stimulus as H-20's arms present it (`stimulus_task`: F-46's shape, ADR-0076's cancel, `CANCEL_PICKED_1024`) at **every epoch's middle** (`STIMULUS_AT`, 8 192 ticks), A or B by the task's draw for the epoch's index in the run, weights frozen, no trial run and no reward (option 4(b)). Each subset size has its own background and control with the stimulus.
  - At the middle, the kick's reading — its span and the pair window after it, ticks 1 to 2 248 of a hold span — closes 5 444 ticks before the window a trial reads before its stimulus opens (compile-time).
  - The stimulus's cancel, its window after it and the prior's far band all end inside the epoch (compile-time and the gate).
  - The next epoch's kick comes 8 192 ticks after the stimulus, past the far band, so no stimulus-driven synapse lands in a kick's reading.
  - At the epoch's first tick a hold span's stimulus would land on the kick and be read as it.
  - `span_protocol_under` presents it with `Stimulus::inject` and `Stimulus::cancel_at`, before the drive's step, as `Task::trial` does. The gate holds one epoch of it to one `Task::trial` on the instrument's network at rest (`presented_as_the_task_presents`): the two trains are one bit for bit, the stimulus is the trial's, and the epoch's counts after the stimulus, each eight-place readout with the place it gave up, are the trial's.

### The protocol, the measures and the thresholds

ADR-0115's, unchanged, for every run: a lead-in of sixteen epochs; eight rounds of an unkicked span of sixteen epochs, a hold span of sixteen with ADR-0112's kick, a release of six stretches (set (ii)) and a tail of four; stretches of two epochs; `holding`, `failed`, `bursts_049`, `HOLD_TIMES`, `LET_GO_TIMES`, `SPILL_TIMES`, `OF_EIGHT_MIN` and `IGNITIONS_MAX`. A run is 400 epochs. Every cell is read against its size's background under its condition. Every run is dumped before any is held to its table, and every weight of the arena at each run's end is asserted equal to its value at the start. The tables are pinned as ADR-0115's: the stretches in full, the windows by a hash (`windows_hash`), and under (d) the epochs by a hash (`epochs_hash`).

### The rules, before any run

- **Robust** (`robust_cells`): a cell of the subset usable in the core grid and under each of (b), (c) and (d).
- **The cell for the second round** (`round_two_cell`): of the robust cells, the one with the most usable neighbours in the core grid (`usable_neighbours`). A cell's neighbours are the cells one weight step either way at its size and one size step either way at its weight, at most four; a diagonal is not one. Ties go to the lighter weight, then the smaller size.
- **If no cell is robust**: what failed in each subset cell in the core grid and under each condition, each by ADR-0115's `failed` (`failures_050`); the next decision follows ADR-0116's branch for what failed.

The gate holds each rule at its edges over tables written by hand.

### Readings, no clause

- **The core grid's table** and its usable region; the bursts of every cell (`bursts_049`).
- **Under (d), the readouts' counts per trial with the context held and with it quiet** (`task_read`): for every run, by kind of span — unkicked, the hold's first half, its second half, the release, the tail — and by stimulus, the trials and the eight-place readouts' spikes in the task's window after the stimulus (`WINDOW`, ticks 100 to 599 after it). Held is the hold's second half, quiet the unkicked span.
- **Under (d), ADR-0065's measure on each background** (`sight_050`, `resolves`): over the 384 trials after the lead-in in six blocks of 64, the harness's `calibrated` over the readouts' spikes after the stimulus against the window before it — for the eight-place readouts, and beside them for the nine-place readouts, each eight-place readout with its place.
- **Under (c), the drained image**: the arm's sums, the ticks the quiet run took, the signal the image carried, the frozen image's sums and CRC, and the stimuli's couplings into each place given up on the settled and the drained image.

### The arithmetic, before any run

Computed by the gate and pinned there:

**What one spike of a marked member delivers under set (ii)** (`DELIVERED_050`), in units of the threshold as ADR-0115's table reads it — at the unkicked steady pair and at the primed peak's, the delivery, the soma's peak it raises a unit at the drive's mean standing to, and the least number of such spikes landing together that fires it. The rows at 0.25 and 0.375 are ADR-0115's (`DELIVERED_049`), held so.

| Weight | Unkicked: delivered | Peak | Together | Primed: delivered | Peak | Together |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: |
| 0.125 | 0.054 | 0.461 | 23 | 0.086 | 0.476 | 15 |
| 0.1875 | 0.080 | 0.473 | 15 | 0.129 | 0.496 | 10 |
| 0.25 | 0.107 | 0.486 | 12 | 0.172 | 0.516 | 8 |
| 0.3125 | 0.134 | 0.499 | 9 | 0.215 | 0.536 | 6 |
| 0.375 | 0.161 | 0.511 | 8 | 0.258 | 0.556 | 5 |

**The stimulus's reach** (`STIMULUS_REACH_050`, on the prior's synapses): from A and from B onto the members, 94 and 100 at 48 units, 123 and 123 at 64, 159 and 160 at 80, and 191 and 190 at 96; every member reached at the first three sizes and 95 of 96 at the fourth. Over the ring, A sends 105 synapses onto place 5 and 95 onto place 16, and B 108 and 91, about two from each stimulus onto each context unit.

**The new draw** (`SEEDED_WIRING_050`): 1 532 of 1 536, 2 042 of 2 048, 2 553 of 2 560 and 3 062 of 3 072 delays differ between seeds 48 and 49 at the four sizes, every one in the prior's local band.

### The gate (`the_region_before_the_readout_the_geometry_the_conditions_the_rules_and_a_marked_assembly_on_the_new_geometry`)

The one runtime test this measurement adds:
- the geometry and the members as above;
- the grid, the subset and ADR-0115's two cells in it;
- the arithmetic, dumped, then held;
- condition (c)'s patches on the instrument's network (`the_patches_on_the_prior`): H-20's flags written and read back, frozen again the image before them bit for bit, and a reward delivered and written into an image, which carries its signal until `frozen_again` puts it at rest;
- condition (d)'s presentation held to `Task::trial`;
- the rules at their edges;
- **a marked assembly on the new geometry kicked for a few hundred ticks**: 96 members under set (ii) at 0.375, on the instrument's network at rest, kicked through the protocol's own kick for 800 ticks (`marked_assembly_kicked`, ADR-0115's gate function with the set, size and weight its arguments). Every member fires once in the kick's span and is in no set of the new geometry, every reset is scheduled, no weight moves, and every member's short-term state is the class's step over its spikes. Its spikes are pinned (`GATE_KICKED_050`).
- **over the pinned tables**, once pinned: each run read again by the rules, the kick at every size under each condition, and the robust cells and the cell for the second round.

### The weekly tests and their cost

Nine tests, each `#[ignore]`d with `exhaustive` in its name, each building the settled image once:
- the core grid's backgrounds and controls (eight runs);
- one test per size of the core grid (five runs each);
- the subset under (b) (six runs);
- the subset under (c): the settled image, H-20's arm, then two backgrounds, two controls and six cells;
- the backgrounds and controls under (d) (four runs);
- the subset under (d) (six runs).

That is 54 runs of 400 epochs and one arm of H-20. At ADR-0115's hundred seconds a run on the hosted runners and ADR-0110's 1 505 s for the arm with its oracle, the round adds about 7 000 s. The present table sums 23 357 s over 79 tests, so the four shards would plan about 7 600 s each, near 3 900 s of wall time at ADR-0115's ratio of 1.91 to 2.00: about 54 per cent of the bound, under the brief's 60. The dispatch reads it, and the cost table is regenerated from it.

### The order of the work

1. This protocol, the rules, the arithmetic pinned and the gate, committed before any run of the measurement and pushed with the pull request opened as a draft.
2. The core grid's backgrounds and controls, 64 units' held to ADR-0115's pins; the kick read on the engine at every size.
3. The test of 64 units: ADR-0115's two cells first, held to ADR-0115's pins, then its other three weights; then the other sizes and the subset under (b).
4. The backgrounds and controls under (d), the kick read on the engine; then the subset under (d).
5. The subset under (c) whole, its kick read on the engine before its cells inside the one test.
6. The readings, the documents, the weekly dispatched on the round's branch at `scope=exhaustive`, its evidence and the cost table regenerated from it.

### The readings

Every reading below is from the pinned tables of `tests/assembly.rs`. The runs were made in the order above, from the protocol's commit, `0887e4e` on `main` (`04fda04` on the branch); at the commits `main` holds after the rebase that merged pull request #146, the branch's in brackets:
- the core grid's backgrounds and controls, (d)'s backgrounds and controls and the test of 64 units, side by side, pinned at `ed82828` (`dfc778b`);
- the other three sizes and the subset under (b) and (d), pinned at `fd5593a` (`936bdd5`);
- the subset under (c), started once the test of 64 units had held ADR-0115's two cells, pinned at `7fca0cf` (`eacc7b4`).

A second run of all nine tests at `7fca0cf` (`eacc7b4`) reproduced every table. On a developer machine in the release profile a run took about 65 s with two other tests beside it and about 85 s with four; H-20's arm took about 24 minutes beside five other tests (a ratio, not admissible). **No weight of any arena moved in any measured run.**

- **ADR-0115's two cells, reproduced.** The test of 64 units ran 0.25 and 0.375 first and held each to ADR-0115's pinned stretches, kicks, windows' hash, reading and bursts before its other weights ran; 64 units' background and control are ADR-0115's bit for bit.
- **The backgrounds** (the members marked and unwired, the drive alone after the lead-in):
  - the core grid's members fired at 1.601, 1.613, 1.643 and 1.648 Hz a member at 48, 64, 80 and 96 units, and the rest at 1.670 to 1.683 Hz a unit;
  - under (c) the members fired at 1.818 and 1.841 Hz at 64 and 96 units and the rest at 1.897 and 1.886, about 13 per cent above the settled network's;
  - under (d) the members fired at 1.735 and 1.767 Hz, 7 per cent above the core's, and the rest at 2.077, 24 per cent above, the stimulus's own units among them.
- **The kick, read on the engine before each condition's cells** (each control's eight kicks). It fires every member once by ADR-0112's measure at every size under every condition, and was not derived again:
  - in the core grid the volley is 384, 512, 640 and 768, every kick a full volley, and the after 7, 10, 16 and 19 against marks of 38.4 to 76.8;
  - under (c) the volley is 512 and 767 (one member missing from one kick at 96 units, within the measure's tolerance), and the after 25 and 35;
  - under (d) the volley is 512 and 768, and the after 6 and 14;
  - under (b) the kick is the core control's (an unwired run draws no delay).

  Each control's lead-in and first unkicked span are its background's bit for bit, and no control holds, ignites or fails to let go.
- **The core grid** (`GRID_050`, by the rules committed first, against the backgrounds pinned before any cell ran):

  | Size | Weight | First half, of 8 | Holds, of 8 | Ignites, of 8 | Lets go, of 8 | Spills | Usable | What failed |
  | ---: | :--- | ---: | ---: | ---: | ---: | :--- | :--- | :--- |
  | 48 to 96 | 0.125, 0.1875 | 0 | 0 | 0 | 8 | not read | no | never holding |
  | 48 | 0.25 | 4 | 5 | 0 | 8 | no | no | never holding |
  | **64** | **0.25** | 6 | **8** | **0** | **8** | **no** | **yes** | — |
  | **80** | **0.25** | 7 | **7** | **0** | **8** | **no** | **yes** | — |
  | **96** | **0.25** | 8 | **8** | **0** | **8** | **no** | **yes** | — |
  | 48 | 0.3125 | 8 | 8 | 8 | 6 | no | no | running away; not letting go |
  | 64 | 0.3125 | 8 | 8 | 7 | 6 | no | no | running away; not letting go |
  | 80 | 0.3125 | 8 | 8 | 8 | 7 | no | no | running away |
  | 96 | 0.3125 | 8 | 8 | 8 | 6 | yes | no | running away; not letting go |
  | 48 to 80 | 0.375 | 8 | 8 | 8 | 1 or 2 | no | no | running away; not letting go |
  | 96 | 0.375 | 8 | 8 | 8 | 2 | yes | no | running away; not letting go |

  **The usable region is one weight, a quarter of the range, at 64, 80 and 96 units.** It is bounded on every side the grid reaches:
  - a sixteenth lighter nothing holds at any size;
  - a sixteenth heavier every size ignites in 7 or 8 rounds;
  - at 48 units the quarter holds in 5 rounds and ignites in none.

  The usable cells hold as ADR-0115's did, a train of bursts in the hold spans and none anywhere else (`GRID_BURSTS_050`): 140, 131 and 158 burst windows at 64, 80 and 96 units, their intervals mostly five to sixteen windows apart (99 of 132, 97 of 123 and 110 of 150), and the members' product over the held stretches 1.451, 1.420 and 1.401 times the unkicked one, against 1.118 to 1.123 over the unkicked spans.
- **The conditions** (`CONDITION_GRID_050`, each against its own backgrounds):

  | Condition | Size | 0.1875 | 0.25 | 0.3125 |
  | :--- | ---: | :--- | :--- | :--- |
  | (b) seed 49 | 64 | never holds | **usable**: holds 7, ignites 1, lets go 8 | ignites 8, lets go 7 |
  | (b) seed 49 | 96 | never holds | holds 6, ignites 0, spills | ignites 8, lets go 6, spills |
  | (c) drained | 64 | never holds | holds 7, **ignites 2**, lets go 7 | ignites 8, lets go 3 |
  | (c) drained | 96 | never holds | holds 8, **ignites 4**, lets go 7, spills | ignites 8, lets go 2, spills |
  | (d) stimulus | 64 | never holds | holds 8, **ignites 4**, lets go 8 | ignites 8, lets go 3 |
  | (d) stimulus | 96 | never holds | holds 8, **ignites 4**, lets go 8 | ignites 8, lets go 5 |

  **No cell is robust** (`ROBUST_050` empty), **so there is no cell for the second round** (`ROUND_TWO_050` none). The two cells usable in the core that the subset holds, 64 and 96 units at a quarter, fail by ADR-0115's `failed` (`FAILURES_050`):
  - 64 units: usable under (b), running away under (c) and under (d);
  - 96 units: never holding and running away (its hold spills) under (b), running away under (c) and under (d).

  Under (c) and (d) both cells hold in 7 or 8 rounds and ignite without a kick: the unkicked spans of 64 and 96 units hold 23 and 44 burst windows under (d), none in the core grid. Neither condition keeps a cell from holding; each tips it into igniting.
- **Condition (c)'s image** (`DRAINED_050`): H-20's arm reproduced ADR-0110's accuracy sequence and its sums after the last block exactly. The quiet run before the image took 2 541 ticks and moved no weight. **The image carried a dopamine signal of 21 860, 0.33, from the arm's last reward**, which `frozen_again` put at rest (F-56). The frozen image's inhibitory sum is 0.0723 of the settled image's and its excitatory sum 1.0049. The couplings from the stimuli onto the places the readouts give up moved as the arm's last mapping, the mirrored one, rewarded them: A onto place 16 and B onto place 5, both readout places of their answers there, rose by 18.8 and 5.6 per cent of the settled image's; A onto place 5 and B onto place 16 fell by 8.3 and 3.8 per cent. That is the confound written before the run, read.
- **Under (d), the readouts with the context held and with it quiet** (`STIMULUS_TASK_050`, `STIMULUS_CELL_TASK_050`): readout 0's and readout 1's spikes per trial in the window after the stimulus, A's trials and B's.

  | Run | Quiet (unkicked), A | Quiet, B | Held (the hold's second half), A | Held, B |
  | :--- | :--- | :--- | :--- | :--- |
  | background, 64 units | 7.65, 9.41 | 8.61, 9.73 | 8.46, 10.07 | 8.89, 9.81 |
  | 64 units at 0.25 | 7.80, 9.55 | 8.79, 9.82 | 10.29, 11.46 | 13.03, 13.11 |
  | background, 96 units | 7.58, 9.36 | 8.56, 9.79 | 8.39, 10.07 | 8.81, 9.78 |
  | 96 units at 0.25 | 9.59, 11.18 | 8.84, 10.15 | 10.04, 12.04 | 15.67, 15.53 |

  **A held context reaches both readouts, by about the same number of spikes**: 1.8 and 1.4 more a trial on A's trials and 4.1 and 3.3 on B's at 64 units, 1.7 and 2.0 and 6.9 and 5.8 at 96, over the background's held spans. The members sit at places 5 and 16, each inside the prior's window of both readouts' places, so the prior carries the context to both. The quiet row at 96 units holds its ignitions. A readout gated by the context would have to make that input selective; the prior does not.
- **Under (d), ADR-0065's measure on the backgrounds** (`STIMULUS_SIGHT_050`): the eight-place readouts resolve the stimulus from the window before it in all six blocks at both sizes, the trials seen 61, 64, 64, 63, 58 and 62 of 64 at 64 units and 62, 64, 64, 63, 58 and 62 at 96, each readout's spikes after above its spikes before. The nine-place readouts beside them see 62 to 64. Giving up a place each costs the measure at most four trials of a block.
- **The evidence.** Only tests and documents change, so by [ADR-0075](0075-the-dispatch-scope-follows-the-diff.md) the weekly was dispatched on this round's branch at **`scope=exhaustive`**: run [36313742568](https://github.com/DescentVTT/VirtualCortex/actions/runs/36313742568) at `376c702`, the documents commit on the branch (`8a95d66` on `main`), after which only this evidence, the cost table and the brief's archive change. The round's commits as `main` holds them after the rebase that merged pull request #146 are `0887e4e` (this protocol, before any run), `ed82828` (the core grid's and (d)'s backgrounds and controls, and 64 units, ADR-0115's two cells reproduced first), `fd5593a` (the core grid and the subset under (b) and (d)), `7fca0cf` (the subset under (c), and the reading in the gate), `8a95d66` (the readings and the documents at 4.67.0) and `9f9e343` (this evidence, the cost table and the brief's archive). On the branch they were `04fda04`, `dfc778b`, `936bdd5`, `eacc7b4`, `376c702` and `d6961f5`. **Every job it ran is green.**
  - **The whole-domain tests.** All eighty-eight passed on the hosted runners: the seventy-nine before this round reproduced their pinned numbers, ADR-0115's among them through the refactored functions, and this round's nine reproduced their tables. The cost table did not know the nine and costed each at 900 s; on the runners they took 259 to 795 s, and condition (c)'s 1 890 s. The four shards took:

    | Shard | Job | Tests | Their seconds summed | Tests' wall time | The shard's heaviest test (s) |
    | ---: | ---: | ---: | ---: | :--- | :--- |
    | 0 | 50 m 30 s | 22 | 5 955 | 2 983 s, 41 % | H-20 from the mirrored 1 275 |
    | 1 | 71 m 04 s | 22 | 8 347 | 4 216 s, 59 % | condition (c) 1 890 |
    | 2 | 41 m 18 s | 22 | 4 873 | 2 440 s, 34 % | H-18 from the mirrored 907 |
    | 3 | 56 m 33 s | 22 | 6 568 | 3 340 s, 46 % | plasticity everywhere 1 122 |

    Shard 1 drew condition (c) and H-20's arm from the assignment together under the old table's default cost, and came within a point of the brief's 60 per cent.
  - **The cost table is regenerated from this run's artifacts** (`scripts/exhaustive-costs.tsv`: eighty-eight lines, 25 743 s, its source line naming the run; `npm run spec:costs` passing). Replayed through the deal, it plans each shard at 6 435 to 6 436 s summed. At this run's ratio of summed seconds to wall time, 1.97 to 2.00, that is about 3 220 to 3 270 s of tests' wall time, 45 per cent of the bound, under the brief's 60.
  - **No mutation sweep**, by the scope: the diff changes nothing under `src/`, so the sweep has nothing to find.

  The pull request's gate on `376c702` (`8a95d66` on `main`) is green in every job: check, test, fmt and clippy; the AArch64 determinism pin, unmoved; the MSRV job; the documentation gate; and the mutation gate on the changed lines, which found no mutant to make.

  On the developer machine every command of brief 050's verification list exited 0 (a ratio, not admissible):
  - the workspace in the debug profile, in the release profile and on the MSRV toolchain in a target directory of its own: 650 passed in each and 88 ignored;
  - check, fmt, clippy, doc, the bench `--test`, `npm ci` and `npm run spec`;
  - `--list`, 88 tests;
  - the in-diff mutation run, "No mutants to filter";
  - this round's nine whole-domain tests twice, the second run at `7fca0cf` (`eacc7b4`) against the pins.
- **Not done:**
  - The grid was not widened: nothing between 0.1875 and 0.25, nor between 0.25 and 0.3125, where the region's edges are, and no size above 96.
  - ADR-0116's option 3(b), the settled image's inhibitory weights scaled, was not run beside condition (c).
  - The geometry is in `tests/assembly.rs`, not the harness; no test runs the task on it.
  - No readout was gated, no context switched and no reward delivered in a measured run.
- **The next decision, named and not taken.** By ADR-0116's branch for a round in which no cell is robust, the next decision follows from what failed, and all three conditions failed a cell the core grid holds usable:
  - under (d), the context's isolation from the task's stimulus: a placement beyond the prior's window from the stimulus places, which needs another geometry;
  - under (c), the inhibitory drain (one of H-20's four open questions), or an inhibition of the context's own;
  - under (b), at 96 units only, the class's constants: the region is not one draw's at 64 units and is at 96.

  An ADR choosing among them has as its need these readings: the region is one sixteenth of the range wide between never holding and igniting, both (c) and (d) push the usable cells over the igniting edge rather than below the holding one, and a held context reaches both readouts alike through the prior. ADR-0111's second round is not taken.

## Consequences

- Good: the region's width, and whether it survives three changes one at a time, are read by ADR-0115's own rules on the geometry the second round would use.
- Good: condition (d) presents the stimulus with the task's own code, held to `Task::trial` bit for bit.
- Good: condition (c)'s image is H-20's arm by its accuracy sequence and sums, and frozen by a rule the gate holds.
- Good: the eight-place readouts still resolve the task's stimulus by ADR-0065's measure, so the geometry F-54's decision chose costs the readout little.
- Bad: no cell is robust. The region is one weight wide, and the task's stimulus and a drained network each tip its cells into igniting, so ADR-0111's second round has no cell to build on as the rules define one.
- Bad: a held context reaches both readouts alike through the prior, which a gated readout would have to undo.
- Bad: the weekly job grows by 5 793 s on the runners by this round's dispatch, and one test, condition (c)'s, runs for 1 890 s there, the longest in the table.
- Neutral: the geometry lives in `tests/assembly.rs` for now, not in the harness as ADR-0116 wrote.
- Neutral: condition (c)'s image carries the old geometry's answer pairs onto the members' places; the couplings are read, and the confound stands.

## Alternatives considered and why rejected

- **The geometry in the harness now** (option 1(a)): it would fail `clippy -D warnings` in `tests/instrument.rs`, which no test of this round touches, or need an allowance the harness has nowhere else.
- **(b)'s own background and control** (option 2(a)): four runs of about 100 s whose every tick is the core grid's.
- **H-18's arm** (option 3(b)): not needed for the budget, and not the network H-20's schedule leaves.
- **The arm with its oracle** (option 3(a)(i)): the oracle writes nothing, and ADR-0110's accuracy sequence and sums identify the run.
- **Two bytes frozen** (option 3(x)): the image would carry the last reward's signal into every measured run (F-56).
- **ADR-0116's option 3(b), the inhibitory weights scaled**: allowed as a reading and not taken; the drain as the learning rule leaves it is what the second round would meet.
- **The stimulus at every epoch's start** (option 4(a)): each hold span's first stimulus would fall on the kick and be read as it.
- **A wider grid** (option 5): the budget is near the bound; a widening is a later round's, if the region's edge is where it matters.

## Confirmation

`runtime/cortex-runtime/tests/assembly.rs`:
- the geometry, grid and conditions: `R0_MASK_050`, `R1_MASK_050`, `CONTEXT_PLACE_MASKS`, `context_geometry`, `context_places`, `SIZES_050`, `WEIGHTS_050`, `SET_050`, `ADR_0115_CELLS`, `SUBSET_SIZES`, `SUBSET_WEIGHTS`, `DELAY_SEED_050`, `Condition`, `STIMULUS_AT`;
- the rules: `robust_cells`, `usable_neighbours`, `round_two_cell`, `failures_050`;
- condition (c): `h20_image`, `frozen_again`, `h20_task`, `drained_image`, `H20_IMAGE_CRC`, `H20_TRACE`, `H20_SUMS`;
- condition (d): `stimulus_task`, `span_protocol_under`, `task_read`, `sight_050`, `resolves`, `epochs_hash`;
- the runs: `delay_drawn`, `wire_drawn`, `marked_engine_drawn`, `run_050`, `backgrounds_controls_050`;
- the gate and the nine weekly tests; the arithmetic's tables `DELIVERED_050`, `STIMULUS_REACH_050`, `SEEDED_WIRING_050`, `PRESENTED_050`, `GATE_KICKED_050`;
- the runs' tables `CORE_RUNS_050`, `CORE_READ_050`, `GRID_RUNS_050`, `GRID_050`, `GRID_BURSTS_050`, `DRAINED_050`, `DRAINED_RUNS_050`, `DRAINED_READ_050`, `STIMULUS_RUNS_050`, `STIMULUS_READ_050`, `STIMULUS_TASK_050`, `STIMULUS_SIGHT_050`, `CONDITION_RUNS_050`, `CONDITION_GRID_050`, `CONDITION_BURSTS_050`, `STIMULUS_CELL_TASK_050`, and the reading `ROBUST_050`, `ROUND_TWO_050` and `FAILURES_050` (`over_the_050_tables`).

Whitepaper §11 carries F-56, and §11.1's question on a rule held by the network the reading.
