---
status: accepted
date: 2026-10-07
depends-on: ADR-0144
decision-makers: VirtualCortex maintainers
---

# ADR-0145: The gate's output delivered, measured — H-27 is no, on clauses 1 and 3: with the address's targets released to every unit and the channel not selected held from the readout window's close to tick 4 096 by one message at the bound every 512 ticks — the least an oracle of the membrane's rule derives, checked on a frozen block where the held readout fired 5 spikes within the span against the selected readout's 1 494 — the consolidation on the side not selected turned from a cost of half the signal into a gain, the eligibility onto the held readout net below zero, and the run's learning signal was 1.02 to 1.59 of H-25's; but the learning was slower than H-25's: from the assignment every mapping learned, 83 to 96 of each mapping's last 128, with two reversals at 25 and 24 blocks, and from the mirrored assignment two mappings unlearned, 72 and 75, with one reversal never passing 40 of 64; no coupling past 1.30 and the network outside the couplings within 0.999 and 1.001 of the image's; the control, the release alone, no as predicted, its signal 0.30 to 0.59 of H-25's; under the hold the four couplings fell as a whole to 0.93 to 0.97 of the image's where H-25's rose, the readouts fired a sixth less and the inhibitory sum fell to 0.78; H-27's stopping rule at step 5, and the next decision, an ADR on the attention-gated feedback with these readings as its need, named and not taken

## Context and Problem Statement

[ADR-0143](0143-the-gates-output-delivered.md) delivered the gate's output to the network, released the address's targets, and wrote **H-27** before any run with the release alone as its control. [ADR-0144](0144-the-gates-output-built.md) built both: `Hold`, a parameter of the task, and `Delivery::Released`. Brief 060 derives the hold's schedule, checks it on a frozen run, and runs H-27 once. This ADR is the round's protocol, committed before any rewarded run, and then its readings.

What was read before anything was written (principle 2, and brief 060's directive that the engine is read before a description of it is trusted, the brief's and ADR-0143's included):

1. **The membrane's rule** (`crates/cortex-core/src/dynamics/membrane.rs`, `integrate`): the basal compartment leaks $2^{-9}$ of itself a tick and takes its input; the soma leaks $2^{-11}$ and moves a sixteenth of its difference to each compartment, so under a standing basal potential it stands at about half of it; a soma at or above the threshold fires; inside the refractory window, 200 ticks, inputs are dropped.
2. **The drive** (`runtime/cortex-runtime/src/synthesis.rs`, `Drive::unit_at`): eight messages of 0.125 a tick over the 1 024 units, each unit named by a hash of the clock's tick and the message's index; 0.21875 after the gain of 1.75. The drive's messages into a unit in a trial are therefore a function of the trial's first tick. A run from ADR-0077's settled image starts at tick 12 585 412 — the lead-in's 96 windows and the quiet run's 2 500 ticks — and its trial of index $j$ at $12\,585\,912 + 16\,384\,j$, after the instrument's lead-in of 500.
3. **One message** (`spike_message`, the executor's `scaled`): clamped at −2.0, −3.5 after the gain (F-47). A message injected before trial tick $k$ lands on tick $k + 1$.
4. **The readouts.** Each is nine places of the geometry's twenty over 51 periods: **459 units, 357 of them excitatory and 102 inhibitory**, and every inhibitory unit of the network, 204, lies in one of the two (`geometry`; [ADR-0137](0137-the-reward-unaddressed-measured.md)'s census). ADR-0143 wrote *"The readouts are 357 units each"*, which is each readout's excitatory units (**F-63**). So a hold reaches 459 units, and with them half the network's inhibitory units.
5. **The ring**: the harness's injector holds 4 096 messages a tick. A hold's messages into 459 units and the drive's eight fit while a hold is at most eight messages a time.
6. **The frozen run** ([ADR-0077](0077-the-background-side.md), `frozen_from`, `taught_run`): the settled image with the baseline at zero and the reward withheld; nothing consolidates, and its first block is pinned (`BACKGROUND_1024`).
7. **H-25's arm and its harness** ([ADR-0140](0140-the-address-drawn-measured.md), [ADR-0142](0142-the-readouts-own-competition-measured.md)): `drawn_arm`, both oracles held at every trial; the composer's rule as `advance`, with `Replay::released` the predicate the released delivery writes; the network's oracle reading the executor's addressed set, whatever wrote it; both replaying the train, so that a hold reaches them only through the spikes it leaves.

## Decision Drivers

- Brief 060's standing directives: the schedule derived and not fitted, by an oracle of the membrane's rule, checked on a frozen run before any rewarded one; the span ADR-0143's; H-27's clauses and constants committed before the first rewarded run and not moved after it; no second attempt; no float; every loop ends by construction; the gate grows by at most one test for the measurement; no shard of the weekly job past 60 per cent of its bound; no pinned number of an earlier round moves.
- **A structural boundary beats a reviewed one** (principle 5): the hold's two numbers are not written down and defended, they are what a function returns, and the test holds the constants to the function.
- Latest ≠ Newest: no dependency, no tool, no rule.
- The readings must say which of the two effects ADR-0143 names was the larger: the background removed, or the window's response kept.

## Considered Options

1. **What the oracle steps**:
   - (a) `integrate` alone on one unit, for every unit of both readouts in every trial of the frozen block, from the most standing potential a unit carries without firing, under the drive's messages as they land on that unit in that trial;
   - (b) a unit at rest under the drive;
   - (c) a bound on a unit's synaptic input from the image's wiring, every afferent at once;
   - (d) each unit's input recorded from a frozen run.
2. **The schedule's form**: (a) messages at the bound on a cadence from the close, the cadence a power of two; (b) every tick; (c) an onset and a hold of different sizes.
3. **Where the frozen check runs**: (a) in the calibration of each arm that carries the hold; (b) a weekly test of its own.
4. **How the cost and the signal are read closed-loop**: (a) the composer's own consolidation folded by H-26's `shade_of`, held to the network's oracle block by block; (b) a shadow under the drawn address beside the run.
5. **What the hold's arms pin**: (a) H-25's tables whole and brief 060's beside them; (b) the clauses' inputs alone.

## Decision Outcome

**Options 1(a), 2(a), 3(a), 4(a) and 5(a).** Everything from here to the weekly tests and their cost was committed before any rewarded run; the order of the work, the readings and the step of the stopping rule follow it.

### The hold, derived (`least_hold`, option 1(a) and 2(a))

- **The form.** `messages` messages at the bound, −2.0, into every unit of the channel held, before the tick the window closes at, 600, and before every `every`-th tick after it below tick 4 096; `every` a power of two from 1 to $2^{12}$, a mask on the ticks since the close as the engine's own cadences are masks on the tick ([ADR-0035](0035-cadence-and-the-population-tally.md)). The longest is longer than the span: one delivery, at the close.
- **The oracle** (`held_alone`): `integrate` stepped alone on one unit at the base threshold from the tick after the close through tick 4 096. It starts from `EXTREME_STANDING`, the most standing potential a unit carries without firing ([ADR-0076](0076-two-injections.md): the basal compartment one LSB below twice the threshold, the soma one LSB below it). Each tick it takes, in one batch scaled by the gain as the executor scales a turn's sum, the drive's messages that land on it and the hold's when the hold was due before the tick before.
- **Over what** (`first_fired`): every unit of readout 0 and of readout 1, 918, in each of the frozen block's 64 trials, the drive as `Drive::unit_at` lands it on that unit at that trial's ticks. A hold silences when no unit fires in any span.
- **The least** (`least_hold`): the least messages a time with which the densest cadence, every tick, silences; then, at that count, the longest cadence that still does. The cadences nest and the rule is monotone in its input while the unit does not fire, so a cadence that silences is silenced by every shorter one.
- **What it returned**, computed before any network was run with a hold:

  | Hold | The first unit to fire: trial, unit, tick |
  | :--- | :--- |
  | none | trial 0, unit 1, tick 634 |
  | one message, at the close only (every $2^{12}$) | trial 0, unit 45, tick 3 951 |
  | one message every 2 048 ticks | trial 0, unit 337, tick 1 672 |
  | one message every 1 024 ticks | trial 6, unit 239, tick 1 312 |
  | **one message every 512 ticks** | **none, in 64 trials** |

  One message a time silences at every tick, and no message does not. **The hold is one message at the bound every 512 ticks** (`HOLD`, `HOLD_MESSAGES`, `HOLD_EVERY`): due before ticks 600, 1 112, 1 624, 2 136, 2 648, 3 160 and 3 672, landing a tick later, seven times a trial, 3 213 messages into the channel held.
- **The constants are the function's.** Each hold arm asserts `least_hold` over the whole block equal to `HOLD` before it reads anything; the gate holds the constants at the lattice's edge by the table's units. No number of the hold was chosen.
- **What the hold leaves in a unit, by the leak's arithmetic** (an integer replica of `integrate`'s basal line; Implemented arithmetic, not a measurement). For a unit otherwise at rest, in thresholds:

  | | After each landing | Just before each next | At tick 4 096 | Mean over the span |
  | :--- | :--- | :--- | ---: | ---: |
  | Basal potential | −3.50, −4.79, −5.26, −5.44, −5.50, −5.52, −5.53 | −1.29, −1.77, −1.94, −2.00, −2.03, −2.04 | −2.42 | −3.24 |

  The compartment is back within half a threshold of rest at tick 4 907 and within one message of the drive at tick 5 335. **The hold outlasts its span by about 1 200 ticks**, the basal leak's tail: a channel held is below rest for nearer three tenths of the trial than the fifth ADR-0143 counted. Clause 4 and the inhibitory sum's course are the readings for it.
- **What the oracle does not model**, said before the check:
  - the synapses' messages: the oracle answers for the drive as it lands and the standing potential a unit can carry, not for any input (twenty of the drive's messages on one tick do not fire a held unit, and forty do);
  - a unit inside its refractory window at the close, whose inputs the rule drops until the window ends, and which is unheld from there to the next landing;
  - the network's answer to 459 silent units, half its inhibitory units among them.

  The frozen check reads the network for all three.
- **The engine's own probe** (`hold_probe`, `hold_alone`): on the instrument's network at rest at the gain 1.75, a task whose readouts are one unit each, one cued so that it is selected at the close; the other's basal and somatic potentials after the trial equal `integrate` stepped alone with the hold's messages landing a tick after each tick the hold was due at, for `HOLD` and for a denser hold of two messages every 64 ticks. The oracle's timing and scaling are the engine's.
- **Why not the others.** A unit at rest (option 1(b)) leaves out the standing potential the response has just built. A bound from the wiring (option 1(c)) silences against every afferent firing at once, an input the settled network does not make, by a hold several times deeper whose tail would reach past half the trial. A recorded input (option 1(d)) is a measurement feeding the derivation, taken from a network the hold itself changes. Every tick (option 2(b)) is one message a tick into a compartment that leaks over 512: a standing potential of −1 792 thresholds. Two phases (option 2(c)) are two more numbers with no rule that needs them: the first landing already takes a unit at the extreme to −1.5.

### The frozen check (`frozen_held`, `hold_checked`; H-27's stopping rule, step 2)

- **The run**: ADR-0077's frozen block — the settled image with the baseline at zero, the reward withheld, ADR-0076's stimulus, 64 trials — with the task carrying `HOLD`. With no hold the same function reproduces ADR-0077's pinned block and sequence, asserted.
- **The span** (`HOLD_SPAN`): the trial's ticks 600 to 4 096, both counted — the tick the selection is made before, which no message can reach, and the last one a message can land on.
- **The rule** (`HOLD_CHECK_TIMES`): over the block's trials that selected a readout, ten times the spikes of the readout not selected within the span at most the selected readout's there. A tie holds nothing and enters neither side.
- **The reading**, taken once, before any rewarded run, in the calibration of H-27's arm from the assignment (`BARE_FROZEN_1024`, `HELD_FROZEN_1024`); the readouts' spikes summed over the block, a tie's two readouts together:

  | | In the readout window: selected, other, at ties | Within the span | Over the whole trial | Trials that selected | Messages |
  | :--- | :--- | :--- | :--- | ---: | ---: |
  | No hold (ADR-0077's block) | 689, 463, 130 | 1 737, **1 793**, 474 | 7 962, 7 583, 2 099 | 56 | 0 |
  | The hold | 673, 448, 180 | 1 494, **5**, 637 | 7 389, 4 936, 2 796 | 54 | 173 502 |

  **The check holds**: within the span the readout not selected fired 5 spikes against the selected readout's 1 494, a third of one per cent, where with no hold it fired 1 793 against 1 737. No weight moved in either block.
- **What the block reads beside the rule**, per trial that selected a readout:
  - the held readout fired 91.4 spikes over the whole trial against 135.4 with no hold, 0.67 of the selected readout's where with no hold it fired 0.95: 32 fewer within the span and about 12 fewer after it, the tail the arithmetic above gives;
  - the selected readout fired 27.7 spikes within the span against 31.0 with no hold, a tenth fewer: with the other channel silent it loses excitation as well as inhibition;
  - in the readout window both blocks read alike, 12.5 and 8.3 against 12.3 and 8.3.

  The first trial is a tie in both blocks and the second selects readout 0 from the same counts; its span reads 36 and 36 with no hold and 27 and 0 with it. From the third trial the counts part, and the block ends with 10 ties against 8.

### The arms (`gated_arm`)

- **Four arms** (`GATED_ARMS`), each a weekly test of its own: H-27's two — `Delivery::Released` with `HOLD`, from the assignment and from the mirrored assignment — and the control's two, `Delivery::Released` alone. Each is H-25's arm of the same first mapping with nothing else changed: H-23's image, H-20's flips, 7 680 trials, the engine's critic with its window, the task carrying no critic.
- **Both oracles at every trial**: the composer's 3 188 pair synapses, addressed where the source was drawn whatever the readout (`Composer::released`), and the network's 26 240 excitatory synapses under the executor's addressed set, each held to the record's traces and weights. The drawn sources are held to the critic's oracle's counts, every unit to being a target, and the hold's messages in each trial to its times, its messages and the units of the readout not selected, none at a tie.

### H-27, restated as integer rules (ADR-0143's clauses and constants)

- **The four clauses are H-25's**, function for function: `drawn` over two arms — `scheduled` (each mapping's last 128 trials at least 80 correct; no coupling above 1.30 of its image's at any block's end), `reversals_within` of `crossings` (each reversal past 40 of 64 within 23 blocks) and `left_band` (the excitatory sum outside the four couplings within three quarters and five quarters of the image's at every block's end, over 120 blocks).
- **The two verdicts** (`Gated`, `gated`): H-27's over the two arms with the hold, the control's over the two without.
- **The step** (`step_of`): 3 at a yes; 4 at a no in which clause 4 fails in an arm, whatever else failed beside it; otherwise 5.
- `GATED_PREDICTED` is none, ADR-0143's; `ALONE_PREDICTED` is no. Both are dumped beside the reading and never asserted.

### The readings, no clause (ADR-0143's)

Per block, pinned; per mapping, computed from them:

- **The cost and the signal closed-loop** (`Costed`, option 4(a)): the composer's own consolidation folded by the address in force — the pairs of the readout the last trial selected and of the one it did not — and by the mapping in force, raised and lowered. Per mapping (`GatedRead`): the cost on the side not selected, `cost_of`, H-26's; the selected side's signal, its answer pairs' net less its other pairs'; and the run's whole signal by the network's oracle, the second less the first. Both sides together are held to the network's oracle's `Went`, block by block and direction by direction.
- **The readouts' spikes by where they fell** (`Quieted`): in the readout window, within the span and over the whole trial, the selected readout's, the other's and both at a tie.
- **The eligibility at each reward** (`Eligible`, ADR-0142's rule): the record's traces from the sources drawn at that trial's end onto the selected readout, the other and both at a tie, above zero and below.
- **The count margins** (`Margins`), and **the hold's messages** per block.
- **The spikes by class, the inhibitory sum's course**, the couplings' separation, the reversal speeds, the value and the troughs, beside H-25's: H-25's tables whole, by their rules (option 5(a)).
- Every trial's consolidation, eligibility, spikes within the span and over the trial, and hold's messages, by one hash.

### The calibration, before any rewarded run (H-27's stopping rule, step 2)

- **In the tree**: the workspace's tests in the debug and the release profile and on the MSRV; every whole-domain test of the weekly job before this round, each in a process of its own from the release build, with no task carrying a hold: every pinned number of the tree.
- **In each arm's test**, before its first rewarded trial: ADR-0077's settled engine and H-20's to H-23's images by their CRCs; a frozen block held to ADR-0077's frozen run; **H-25's first block from H-23's image under the drawn delivery** (`drawn_first_block`), table by table, the network's oracle beside it.
- **In each arm that carries the hold**: `least_hold` over the block equal to `HOLD`; the frozen block with no hold, ADR-0077's; **the frozen check**.

A failure at any of them stops the round there as a finding.

### The gate

`the_clauses_of_h_27_the_hold_s_rule_and_the_readings_rules`, one test:
- the arms, the delivery, the predictions and the constants; the hold's due ticks and times;
- the two verdicts over tables written by hand, and the step each reaches: a yes; clause 1 failing in the control alone; a reversal that never crosses; the network leaving the band, alone and beside clause 1; a run not whole;
- the oracle over values written by hand — a unit at rest, at the extreme, with a drive message on the tick after the close, with the hold beside it, under a burst the hold does not stop; the drive's landings against `Drive::unit_at`;
- the constants at the lattice's edge by the units of the table above, the hold silent over the gate's first eight trials, and the rule over one trial;
- the engine's probe held to the rule stepped alone;
- the frozen check's rule at its edge, and the readings' folds, over values written by hand;
- the geometry: 459 units a readout, 102 of them inhibitory, 204 inhibitory units in all;
- eight trials on the instrument's network under the released delivery with the hold, and eight without it: the drawn sources held to the critic's oracle's counts, every unit a target, the hold's messages to its times, both oracles to the record, and some consolidation on the pairs of the readout not selected.

The build's tests are ADR-0144's, in `task.rs`.

### The weekly tests and their cost

- **Four weekly tests**: `the_gate_s_output_delivered_from_the_{assignment, mirrored_assignment}_at_1024_units_exhaustive` and `the_release_alone_from_the_{assignment, mirrored_assignment}_at_1024_units_exhaustive`.
- **Each is H-25's arm** with the calibration's first block and, in the two with the hold, the oracle over the block and two frozen blocks. H-25's arms took 1 025 and 1 319 s in ADR-0142's dispatch. The table after ADR-0142 is 81 tests and 32 575 s, about 39 per cent of the bound at six shards; four more arms plan about 48 per cent, under the directive's 60.
- **The dispatch's scope**: ADR-0144 changed a file under `src/`, so by [ADR-0075](0075-the-dispatch-scope-follows-the-diff.md) the weekly is dispatched at `scope=both`.

### The order of the work, as the history holds it

1. **The rule before its result.** The hold's form, the oracle, what it steps over and the rule of the least were written (`least_hold`, `first_fired`, `held_alone`) before the oracle was first run; it was run once, returned the table above, and the constants were written from it. No network had run with a hold.
2. **The frozen check**, from the release build of the tree as it then stood, on 2026-10-07 from 02:14Z: the calibration of H-27's arm from the assignment — ADR-0077's settled engine, the images, ADR-0077's frozen block, H-25's first block under the drawn delivery, `least_hold` over the block, the frozen block with no hold and with the hold — which dumped the check's reading and stopped at its empty tables, before the arm's first rewarded trial. The tables were pinned from that dump.
3. **The gate's test** was run after it, 22.6 s in the debug profile on the developer machine (a ratio, not admissible); its eight rewarded trials with the hold on the instrument's unsettled network are the harness's check and read nothing of H-27. `held_alone`'s scan of the drive's landings was then rewritten as a partition of the slice, so that the file holds no `while`, and the gate's test passed again; each hold arm asserts `least_hold` over the whole block again before it reads anything.
4. **The build and the protocol pushed** (`db5b04a` and `a7236ac` on `main`; `1ee0424` and `af856fa` on the branch before the rebase that merged pull request #180, pushed at 02:39:45Z with the pull request opened as a draft on them), before any rewarded trial of an arm. The pull request's checks passed on `af856fa` (`a7236ac`): the check, the tests in the debug and the release profile, the format, Clippy, the documentation and the benchmarks; the MSRV; the determinism pin on AArch64; and the mutation gate on the changed lines, 23 mutants in ADR-0144's lines, 22 caught and 1 unviable, none missed.
5. **The calibration**, with no task carrying a hold. On the developer machine the workspace's tests passed in the debug profile (696 passed, 125 ignored), and the check, Clippy, the documentation and the format exited 0. **Every one of the 81 whole-domain tests of the weekly job before this round passed**, each in a process of its own and each exit 0 with one test reported: 71 eight at a time from 02:24:18Z to 03:44:49Z, H-25's two arms with their shadows and every earlier run through `Task::trial` among them, and the ten of ADR-0097 and ADR-0102 one at a time to 03:47:47Z. They ran from a release build of the working tree made at 02:22Z, a quarter of an hour before that tree was committed: the tree `af856fa` (`a7236ac`) holds, but for `held_alone`'s scan, rewritten after the build, which no calibrated test calls.
6. **The arms**, once, side by side from 03:48:59Z to 04:14:35Z, from a release build of `af856fa` (`a7236ac`) in a directory of its own. In each the calibration held: ADR-0077's settled engine and H-20's to H-23's images by their CRCs, ADR-0077's frozen block, and H-25's first block from H-23's image under the drawn delivery reproduced table by table; in the two with the hold, `least_hold` over the block returned `HOLD`, the frozen block with no hold was ADR-0077's, and the frozen check read again what it had read, 5 against 1 494. Each ran its 7 680 trials with the drawn sources held to the critic's oracle, every unit a target, the hold's messages held to its times and both oracles held to the record at every trial, and stopped at the first empty table, in 1 475 to 1 526 s. The tables were written from those dumps (`d3884d0`; `1774b86`), and a second run side by side from 04:28:10Z reproduced every table and passed, in 1 472, 1 480, 1 442 and 1 438 s (on the developer machine, a ratio and not admissible).

No constant, clause or rule moved after the first rewarded trial of an arm, and there was no second attempt: the second run is the pinned tables' reproduction.

### The readings

**The verdict** (`GATED_1024`), by the rule committed first, over the pinned tables. **H-27, the released delivery with the hold:**

| Arm | Correct of each mapping's last 128 | Highest coupling per mapping, of its image's | Blocks to 40 of 64: the first mapping; each reversal | The excitatory sum outside the couplings, of the image's |
| :--- | :--- | :--- | :--- | :--- |
| Assignment first | 96, 87, 83, 87 | 1.166, 1.154, 1.088, 1.076 | 11; **25**, **24**, 20 | 0.9990 to 1.0001 |
| Mirrored first | 117, **72**, 95, **75** | 1.198, 1.178, 1.202, 1.161 | 10; **none**, 17, **30** | 0.9991 to 1.0009 |

- **Clause 1** holds from the assignment and **fails from the mirrored assignment**, in its second and fourth mappings: 72 and 75 against the mark of 80.
- **Clause 2 holds in both arms**: the highest coupling 1.166 and 1.202 of its image's.
- **Clause 3 fails in both arms.** From the assignment the first two reversals pass 40 of 64 in their 25th and 24th blocks, two and one past the bound of 23, and the third in its 20th. From the mirrored the first never passes it in its 32 blocks, the second passes in its 17th and the third in its 30th.
- **Clause 4 holds in both arms.**
- Where clause 1 failed it failed on one stimulus (`GATED_EARNED_1024`): over the last 128 trials of the mirrored arm's second and fourth mappings stimulus B selected its answer in 45 and 49 of its trials, and stimulus A in 27 and 26, its old answer in 36 and 23.
- `Drawn { learning: Scheduled { learned: [[true; 4], [true, false, true, false]], bounded: [true; 2], over: [None; 2], yes: false }, revised: [[false, false, true], [false, true, false]], left: [None; 2], held: [true; 2], yes: false }`. **H-27 is no**, on clauses 1 and 3, with its scope: this task, these two arms, this schedule of three flips over 7 680 trials, H-25's configuration at 1 024 units, this hold.

**The control, the released delivery alone:**

| Arm | Correct of each mapping's last 128 | Highest coupling per mapping | Blocks to 40 of 64 | Outside the couplings |
| :--- | :--- | :--- | :--- | :--- |
| Assignment first | 80, **71**, **69**, **73** | 1.058, 1.056, 1.038, 1.029 | 14; **none**, **28**, **none** | 0.9995 to 1.0009 |
| Mirrored first | 97, **63**, 93, **69** | 1.110, 1.097, 1.121, 1.108 | 16; **none**, 16, **none** | 0.9992 to 1.0011 |

**The control is no, on clauses 1 and 3, as ADR-0143 predicted.** It learned the first mapping from the image in both arms, at the mark from the assignment (80) and at 97 from the mirrored, and from the mirrored its third, the first again (93). Of the six reversals one passed 40 of 64 within the bound, that return.

H-25's, for reference ([ADR-0140](0140-the-address-drawn-measured.md)): 122, 124, 119, 116 and 124, 118, 123, 117 correct; the first mappings crossing in 6 and 4 blocks, the reversals in 19, 21, 16 and 20, 15, 14.

**The prediction, against the reading.** ADR-0143 made none for H-27 and said why: the account of the basal ganglia predicts yes, and the response already made in the window and the pair rule's last spike pull the other way. What was read is neither side's whole: the account's premise held — the held channel carried no net eligibility, and the release stopped costing — and the learning still fell short of H-25's. For the control ADR-0143 predicted no on clause 1 or 3, from ADR-0142's first-order reading; it is no on both.

**What the hold did to the readouts' spikes** (`GATED_QUIETED_1024`; per trial that selected a readout, the range over the four mappings):

| Arms | In the readout window: selected, other | Within the span: selected, other | Over the whole trial: selected, other | Other over selected: window, span, trial |
| :--- | :--- | :--- | :--- | :--- |
| With the hold | 12.2 to 14.6, 8.2 to 9.7 | 27.3 to 28.2, **0.10 to 0.12** | 132.8 to 136.3, 91.0 to 93.6 | 0.64 to 0.67, 0.004, 0.68 to 0.69 |
| The release alone | 12.1 to 14.1, 8.2 to 9.9 | 31.3 to 32.8, 31.2 to 32.5 | 137.6 to 143.0, 134.1 to 138.7 | 0.66 to 0.70, 0.98 to 1.00, 0.97 to 0.98 |
| H-25 (ADR-0142) | 13.9 to 16.4, 8.5 to 10.2 | not read | 140.3 to 146.0, 135.2 to 139.6 | 0.57 to 0.62, —, 0.95 to 0.97 |

Through 7 680 trials of each arm the hold did what the frozen block read: within the span the held readout fired a tenth of a spike a trial against the selected readout's 27 to 28. Outside the span it fired 91 to 94 a trial against the selected readout's 106 to 108, 0.86 of it, where with the release alone it fires 103 to 106 against 106 to 110, 0.96 to 0.97: the tail the leak's arithmetic gave. The selected readout fired about four spikes fewer within the span than with the release alone. The hold delivered 3 213 messages in every trial that selected a readout, 2 921 to 3 006 a trial on average.

**The cost and the signal, closed-loop** (`GATED_COSTED_1024`, `GATED_WENT_1024`; per mapping, the run's own consolidation folded by the address in force, both sides together held to the network's oracle at every block):

| Arm, mapping | The selected side's signal | The cost on the side not selected | The run's whole signal | Over H-25's signal |
| :--- | ---: | ---: | ---: | ---: |
| With the hold, assignment, 1 to 4 | 2 923 454; 4 099 861; 3 553 675; 4 476 689 | −1 092 500; −1 582 668; −1 752 607; −1 132 501 | 4 015 954; 5 682 529; 5 306 282; 5 609 190 | 1.50, 1.13, 1.03, 1.02 |
| With the hold, mirrored, 1 to 4 | 3 198 678; 3 179 513; 4 010 091; 4 060 927 | −1 160 000; −2 198 996; −1 107 562; −1 428 930 | 4 358 678; 5 378 509; 5 117 653; 5 489 857 | 1.59, 1.08, 1.06, 1.07 |
| The release alone, assignment, 1 to 4 | 3 118 642; 4 380 141; 4 173 381; 4 800 045 | +1 933 216; +2 502 817; +2 624 390; +2 827 079 | 1 185 426; 1 877 324; 1 548 991; 1 972 966 | 0.44, 0.37, 0.30, 0.36 |
| The release alone, mirrored, 1 to 4 | 3 545 018; 3 916 660; 5 043 594; 5 507 472 | +1 933 732; +2 054 706; +3 195 839; +3 348 838 | 1 611 286; 1 861 954; 1 847 755; 2 158 634 | 0.59, 0.37, 0.38, 0.42 |

- **Under the release alone the cost is 0.52 to 0.63 of the selected side's signal**, in every mapping: H-26's open-loop reading, 0.49 to 0.60 of the run's signal, stands closed-loop. What is left is 0.30 to 0.59 of H-25's signal, under ADR-0142's first-order 0.51 to 0.63 in seven mappings of eight: a release that learns less also earns less.
- **Under the hold the cost is below zero in every mapping**, −0.25 to −0.69 of the selected side's signal: what the release consolidates on the side not selected adds to the signal. There the other pairs fell on net in all eight mappings, by 0.56 to 1.51 million, and the answer's pairs rose on net in seven, by 0.08 to 1.02 million. The run's whole signal is **1.02 to 1.59 of H-25's**.
- By the kind of the trial the address was written at (`GatedRead::kinds`), under the hold the gain comes after correct selections and after wrong ones alike; after a tie, where nothing is held, the release costs in five mappings of eight.

**Which of ADR-0143's two effects was the larger: the background removed.** The eligibility at each reward from the sources drawn there (`GATED_ELIGIBLE_1024`), the readout not selected over the selected one, the range over the four mappings:

| Arm | Above zero | Below zero | Magnitude | Net |
| :--- | :--- | :--- | :--- | :--- |
| With the hold, assignment | 0.78 to 0.84 | 0.91 to 0.97 | 0.85 to 0.90 | −0.46 to −0.03 |
| With the hold, mirrored | 0.75 to 0.82 | 0.90 to 0.92 | 0.82 to 0.87 | −0.93 to +0.12 |
| The release alone, assignment | 0.95 to 0.97 | 0.98 to 1.00 | 0.96 to 0.98 | 0.74 to 0.79 |
| The release alone, mirrored | 0.92 to 0.95 | 0.97 to 0.98 | 0.94 to 0.96 | 0.53 to 0.78 |

Against the release alone, mapping by mapping, the hold left the held readout 0.85 to 0.92 of its eligibility above zero and 0.98 to 1.02 of its eligibility below zero. So the potentiation the held readout would have paired from its spikes in the span is what the hold took; had keeping the window's response as the last spike been the larger effect, the part above zero would have risen. The net onto the readout not selected went from a half to four fifths of the selected readout's to below zero in seven mappings of eight, and a trace below zero is lowered by a reward and raised by a punishment under the signed gate: the cost's sign.

One reading stands beside this. Under either released arm the eligibility at a reward is under H-25's on both readouts, 0.36 to 0.51 of it above zero and 0.44 to 0.62 below. The release spends both readouts' traces at every trial, and the drawn address spent the selected readout's alone; whether that is the whole of the difference is not derived.

**Why the learning is slower under the hold than under H-25's address is not derived here.** The signal is not smaller. What differs is read:

- **The couplings fell as a whole.** The four couplings summed, of the image's, at each mapping's end, and each mapping's answer couplings and other couplings:

  | Run | The four summed | The answer's couplings, A and B | The other couplings | Their separation |
  | :--- | :--- | :--- | :--- | :--- |
  | With the hold, assignment | 1.008, 0.957, 0.934, 0.929 | 1.17, 1.16; 1.02, 1.02; 1.09, 1.06; 1.03, 0.98 | 0.85, 0.87; 0.92, 0.87; 0.80, 0.80; 0.85, 0.85 | 0.32, 0.29; 0.11, 0.15; 0.29, 0.26; 0.17, 0.13 |
  | With the hold, mirrored | 1.018, 0.993, 1.022, 0.968 | 1.20, 1.17; 1.00, 1.07; 1.20, 1.15; 1.02, 1.02 | 0.86, 0.85; 0.97, 0.94; 0.87, 0.86; 0.91, 0.92 | 0.34, 0.32; 0.03, 0.13; 0.33, 0.29; 0.11, 0.10 |
  | H-25, assignment | 1.047, 1.042, 1.036, 1.058 | 1.16, 1.14; 1.15, 1.11; 1.17, 1.12; 1.17, 1.15 | 0.94, 0.96; 1.00, 0.91; 0.94, 0.92; 0.99, 0.92 | 0.22, 0.19; 0.15, 0.20; 0.23, 0.20; 0.17, 0.23 |
  | H-25, mirrored | 1.069, 1.071, 1.077, 1.088 | 1.17, 1.17; 1.14, 1.17; 1.16, 1.18; 1.19, 1.18 | 0.97, 0.96; 0.99, 0.99; 1.00, 0.96; 0.99, 1.00 | 0.20, 0.21; 0.16, 0.19; 0.16, 0.22; 0.21, 0.19 |

  Under the hold the first mapping separates the couplings by more than H-25 did, 0.29 to 0.34 against 0.19 to 0.22, and does it by lowering the other couplings to 0.85 where H-25 left them at 0.94 to 0.97. The reversal then starts from further away and ends nearer, 0.03 to 0.15 at the second mapping's end against H-25's 0.15 to 0.20, and the return to the first mapping parts them by 0.26 to 0.33 again: the couplings keep a lean toward the mapping the run began with, which H-25's do not. The lowest coupling of a run is 0.795 and 0.848 of its image's, against H-25's 0.914 and 0.955.
- **The selection is made from less.** The selected readout's count in the window is 12.2 to 14.6 a trial against H-25's 13.9 to 16.4, the mean margin 3.6 to 4.6 spikes against 5.0 to 6.4, and 6.4 to 9.1 per cent of the trials tie against 4.4 to 7.0.
- **The network fires less and the inhibitory rule answers** (`GATED_SPIKES_1024`, the blocks' inhibitory sums), as ADR-0143 said it would:

  | Run | Rate over the run: inhibitory, stimulus, excitatory readout units (Hz) | The inhibitory sum, of the image's, at each mapping's end |
  | :--- | :--- | :--- |
  | With the hold | 1.82 and 1.85, 4.42 and 4.42, 1.44 and 1.45 | 0.978, 0.915, 0.847, **0.781** and 0.978, 0.913, 0.845, **0.781** |
  | The release alone | 2.17 and 2.20, 4.46 and 4.46, 1.72 and 1.74 | 1.001, 0.950, 0.893, 0.842 and 1.002, 0.951, 0.899, 0.850 |
  | H-25 | 2.22 and 2.26, 4.46 and 4.46, 1.74 and 1.75 | 1.005, 0.953, 0.898, 0.848 and 1.002, 0.952, 0.897, 0.850 |

  Under the hold the readouts' units fire a sixth less, the inhibitory ones among them, and the inhibitory sum falls from the first block, by about 0.2 per cent of the image's a block at the end against H-25's 0.13, with no sign of a floor. The release alone leaves the rates and the inhibitory sum where H-25 had them.
- **The release reaches further outside the couplings than the drawn address did**, with the hold or without (`GATED_CELLS_1024`, `GATED_WENT_1024`). Outside the four couplings 14 to 21 per cent of the answer pairs' weight moved a mapping, against H-25's 1.1 to 1.8, netting within a third of a million of zero; at the runs' ends 17 794 to 19 657 of the 23 052 synapses there stood off their image weight, against H-25's 11 325 and 11 822. Two cells moved that H-25 left untouched: a readout unit's synapses onto stimulus units, 2 084 to 2 200 of 2 452, and all 69 synapses from one stimulus unit onto another, where the largest single move from the image's weight was 22 449 to 26 071 of the width's 32 767, against H-25's 1 197 anywhere. Clause 4 reads the sum, which stayed within a thousandth of the image's.
- **The reward's net in the answer's pairs** is 6.8 to 9.4 per cent of the weight it moved there under the hold and 1.3 to 4.2 under the release alone, against H-25's 13.3 to 15.8.

**The drawn sources** (`GATED_SOURCED_1024`) are H-25's: 99.66 or 99.67 per cent of the presented stimulus's units and 1.62 to 1.66 others a trial, in all four arms.

**The value and the punishments.** Each stimulus's mean value over each mapping's last 128 trials stood within a quarter of the reward of $2p - 1$ in every mapping and stimulus of the four arms but one, the mirrored hold arm's first mapping for stimulus B, at 0.285. After each flip the value fell to −0.11 to −0.86 of the reward under the hold and −0.20 to −0.69 under the release alone, against H-25's −0.63 to −0.84. The strong punishments per flip were 553 to 761 under the hold and 633 to 722 under the release alone, against H-25's 451 to 579.

**The oracles held** at every one of the 30 720 trials: the drawn sources to the units the harness's critic oracle counted from the train, every unit to being a target, the hold's messages to its times, the composer's traces, weights and signal over the pairs under the released address, the network's traces, weights and stamps over all 26 240 excitatory synapses, and the engine's value, error, weights and window to the harness's critic. The gate holds the pinned tables to one another block by block: each coupling the one before plus what the composer consolidated, the composer's fold over both sides of the address the network's oracle's direction by direction, the outside's moves the change in the cells' sum, the readouts' spikes in the window the block's counts, the ties the margins' first bin, and the hold's messages 3 213 in every trial that selected.

### The evidence

- **The dispatch's scope.** The diff changes a file under `src/` — the task, ADR-0144's hold and delivery — so by [ADR-0075](0075-the-dispatch-scope-follows-the-diff.md) the weekly was dispatched on this round's branch at **`scope=both`**, as brief 060 asks: run [37574044466](https://github.com/DescentVTT/VirtualCortex/actions/runs/37574044466) at `433ec47` (`5ba9200` on `main`), the readings' commit, whose tests are those of `1774b86` (`d3884d0`), the pinned tables. The round's commits as `main` holds them, beside the branch's: `db5b04a` (`1ee0424`) the build; `a7236ac` (`af856fa`) the protocol, before any rewarded run; `d3884d0` (`1774b86`) the arms' tables and the gate's checks over them; `5ba9200` (`433ec47`) this ADR's readings and the documents; `67837d9` (`997354d`) this section, the cost table and the brief's archive.
- **Every exhaustive job is green.** All eighty-five `exhaustive` tests passed, each exit 0 with one test reported: the eighty-one before this round reproduced their pinned numbers on the hosted runners with no task carrying a hold, and the four arms reproduced their tables there, in **1 085 s and 1 997 s with the hold and 1 802 s and 1 704 s with the release alone**; H-25's arms took 1 796 and 1 081 s in the same run.
- **The shards**, dealt by the table before this round, which did not know the four arms and costed each at 900 s:

  | Shard | Job | Its two heaviest tests (s) | Tests | Their seconds summed | Tests' wall time |
  | ---: | ---: | :--- | ---: | ---: | ---: |
  | 0 | 61.8 min | H-25 from the assignment 1 796; H-24 from the mirrored 1 379 | 15 | 6 720 | 3 629 s, 50 % |
  | 1 | 62.3 min | the control from the mirrored 1 704; H-21 from the mirrored 1 687 | 14 | 7 302 | 3 667 s, 51 % |
  | 2 | 66.5 min | H-27 from the mirrored 1 997; H-21 from the assignment 1 804 | 14 | 7 831 | 3 921 s, 54 % |
  | 3 | 67.8 min | H-20 from the mirrored 1 971; the control from the assignment 1 802 | 14 | 7 968 | 4 005 s, 56 % |
  | 4 | 40.6 min | H-20 from the assignment 1 194; H-25 from the mirrored 1 081 | 14 | 4 671 | 2 386 s, 33 % |
  | 5 | 45.4 min | H-27 from the assignment 1 085; H-23 from the assignment 1 084 | 14 | 5 267 | 2 676 s, 37 % |

  The percentages are of the job's bound, 120 minutes; every shard ran its tests two at a time, at 1.85 to 2.00 of their summed seconds over the wall. No shard passed 60 per cent of its bound, the two that drew an arm costed at 900 s and running 1 800 to 2 000 included.
- **The cost table is regenerated from this run** (`node scripts/exhaustive-costs.mjs from <artifacts> --run 37574044466`): 85 lines, 39 759 s, the eighty-one earlier tests at 1.018 of the table before. ADR-0092's deal plans each of the six shards at 6 626 to 6 627 s summed, about 3 380 s of wall time at the run's ratio, **about 47 per cent of the bound**, inside the brief's 60 and beside the 48 planned above.
- **The mutation sweep.** Seven jobs, green: 3 793 mutants over the tree, **3 620 caught and none missed**, 2 398 in the state crates and 1 222 in the runtime, 151 unviable and 22 timeouts. Every mutant the sweep made in the round's code was caught or did not compile: `send`'s two, `Hold::is_due`'s six, `Readout::hold`'s five and the six in the hold's refusals in `Task::check` caught; `Task::trial`'s whole body unviable, an `Outcome` having no default. Its 22 timeouts are the known protocol ones — the iterators of `cortex-core`, the injector, the barrier, `stop_workers` and the workers' loop — none in code the round changed. By [ADR-0063](0063-the-sweep-reads-its-own-timeouts.md)'s evidence every one of the runtime's sixteen had stopped finishing tests or hung a test of its own module, and the six of the state crates are without the evidence to say, their bound being under cargo's sixty seconds; by [ADR-0062](0062-the-first-complete-sweeps-list.md)'s triage they are detections. The runtime's six shards took 2 h 12 min to 3 h 33 min against their bound of 330 minutes, the state crates' 25 minutes.
- **The pull request's gate** on `af856fa` (`a7236ac`), on `433ec47` (`5ba9200`) and on `997354d` (`67837d9`) is green in every job, the determinism pin on AArch64 and the MSRV among them; the mutation gate on the changed lines found 23 mutants in each, 22 caught and 1 unviable, none missed. The unviable is the tool's synthesized return for `Task::trial`, whose `Outcome` has no default. The same run on the developer machine read 11 caught and 12 unviable, eleven of them the Windows linker's refusal to overwrite a test binary and not the mutant's, and none missed.
- **On the developer machine**, at `433ec47` (`5ba9200`): the workspace's tests passed in the debug profile, in the release profile and on the MSRV, 696 passed and 125 ignored in each; the check, the format, Clippy, the documentation, the benchmarks and `npm run spec` exited 0; 85 whole-domain tests are listed. The MSRV's tests were first run beside the calibration, eight whole-domain tests at once, and failed once there in `no_alloc.rs`, which this round does not touch: one run of six read two allocations. On the idle machine they pass. That is F-64, opened by this round and not fixed by it.

### The step of the stopping rule reached

**Step 5**: *"Otherwise no on clause 1, 2 or 3: the gate's output after the window does not make the eligibility specific enough. The next decision is an ADR on the attention-gated feedback, the fallback ADR-0141 named, with this round's readings as its need."* Clause 4 held in every arm, so step 4 did not arise. **The next decision is an ADR on the attention-gated feedback.** It is named and not taken. What this round hands it:

- **The need is not the signal's size.** With the gate's output delivered the released target side delivered a learning signal as large as the addressed one's, and its unselected side stopped costing. The eligibility onto the losing readout was made net negative; it was not made small, 0.82 to 0.90 of the selected readout's by magnitude, and the release spent it at every trial.
- **What the learning fell short by**: two reversals of three from the assignment, by one and two blocks; from the mirrored a reversal that never crossed and one that took 30 blocks, and its second and fourth mappings at 72 and 75 of 128.
- **What a hold costs the network it stands on**: readouts firing a sixth less, an inhibitory sum falling by a fifth over the run and still falling, the four couplings 3 to 7 per cent under the image's as a whole, and a selection made from a margin more than a quarter smaller.
- **What the release reaches whatever is done about the losing readout**: the synapses from the drawn sources onto every unit, among them a stimulus unit's synapses onto stimulus units, which moved by up to three quarters of the width.
- **The control**: the release alone learns the first mapping and reverses once in six.

Steps 3 and 4 did not arise; step 6 is kept.

## Consequences

- Good: the hold is what a function of the membrane's rule and the drive returns, held to the engine by a probe and to the network by a check that passed with room, 5 spikes against 1 494, and through four arms of 7 680 trials it silenced the held readout within the span as the frozen block read.
- Good: the control reads what the gate's output adds to the release alone, closed-loop: a signal three times the control's and all four mappings learned from the assignment where the control learned one.
- Good: the question ADR-0143 left open is answered by a reading: of its two effects the background removed was the larger, and the held readout's eligibility went net below zero.
- Good: the cost and the signal are read by H-26's fold on the run's own consolidation, held to the second oracle.
- Bad: H-27 is no. With the gate's output delivered the released target side learns less well than the efference copy it was to replace, in both arms.
- Bad: the hold outlasts its span by about 1 200 ticks and reaches half the network's inhibitory units; the readouts fire a sixth less under it and the inhibitory sum falls by a fifth and is still falling at the run's end. No clause holds the inhibitory sum.
- Bad: the oracle models the drive and a standing potential, not the synapses; that the hold silences the network's units is the frozen check's and the arms' reading, not the oracle's.
- Bad: why a signal as large as H-25's learns more slowly is not derived. The couplings' fall, the weaker selection and the lower rates are read beside it, and none is shown to be the cause.
- Bad: one seed, one size, one schedule, one hold.
- Neutral: four more weekly arms, each pinned whole.

## Alternatives considered and why rejected

- **A unit at rest, a bound from the wiring, a recorded input** (options 1(b) to 1(d)), **every tick or two phases** (options 2(b), 2(c)): see above.
- **A weekly test of its own for the check** (option 3(b)): the check is a calibration of the arms that carry the hold and costs them two frozen blocks.
- **A shadow under the drawn address beside the run** (option 4(b)): H-25 is that run, pinned.
- **The clauses' inputs alone** (option 5(b)): the readings are what the round hands the next decision.

## Confirmation

- `runtime/cortex-runtime/tests/inhibition.rs`: `GATED_ARMS`, `GATED_DELIVERY`, `GATED_PREDICTED`, `ALONE_PREDICTED`, `HOLD`, `HOLD_UNTIL`, `HOLD_CLOSE`, `HOLD_SPAN`, `HOLD_MESSAGES`, `HOLD_EVERY`, `HOLD_CHECK_TIMES`, `SETTLED_TICK`, `trial_start`, `drive_landings`, `held_alone`, `first_fired`, `hold_of`, `least_hold`, `Gated`, `gated`, `step_of`, `Fired`, `Quieted`, `quieted_blocks`, `hold_checked`, `selected_signal`, `GatedRead`, `Along`, `along_hash`, `frozen_held`, `hold_calibration`, `drawn_first_block`, `held_run`, `gated_arm`, `hold_probe`, `hold_alone`, `BARE_FROZEN_1024`, `HELD_FROZEN_1024`; the four weekly tests and the gate's test named above.
- `runtime/cortex-runtime/tests/instrument/harness.rs`: `Composer::released`, `HeldRead`, `hold_times`, `earned_run_held` (ADR-0144).
- `GATED_1024`, `GATED_STEPS_1024` and the pinned tables `GATED_{BLOCKS, TRACES, COMPOSITIONS, EARNED, READ, CENSUS, MOVES, STRONG, AT_FLIPS, VALUES, VALUE_HASH, WEIGHTS, ADMITTED, SPIKES, CELLS, WENT, SOURCED, SOURCES_HASH, COSTED, QUIETED, ELIGIBLE, MARGINS, MESSAGES, ALONG_HASH, READINGS}_1024`.
- Every pinned number of ADR-0065 to ADR-0142 unchanged, and the determinism pin; the image format 20.
- Whitepaper §11 (F-63) and §11.1 (H-27); `CHANGELOG.md`.
