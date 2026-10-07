---
status: proposed
date: 2026-10-07
depends-on: ADR-0144
decision-makers: VirtualCortex maintainers
---

# ADR-0145: The gate's output delivered, measured — brief 060's protocol, committed before any rewarded run: the hold derived by an oracle of the membrane's rule as one message at the bound into every unit of the channel held, before the tick the readout window closes at and before every 512th tick after it below tick 4 096, held to the engine's own probe and checked on a frozen block, where the readout not selected fired 5 spikes within the span against the selected readout's 1 494; H-27's two arms under the released delivery with the hold and the control's two with the release alone, H-25's four clauses, and the readings

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

## Consequences

- Good: the hold is what a function of the membrane's rule and the drive returns, held to the engine by a probe and to the network by a check that passed with room: 5 spikes against 1 494.
- Good: the control reads what the gate's output adds to the release alone, closed-loop, where ADR-0142 could read the release only on H-25's trajectory.
- Good: the cost and the signal are read by H-26's fold on the run's own consolidation, held to the second oracle.
- Bad: the hold outlasts its span by about 1 200 ticks, and reaches half the network's inhibitory units. Both were read before any rewarded run and neither is in ADR-0143.
- Bad: the oracle models the drive and a standing potential, not the synapses; that the hold silences the network's units is the frozen check's reading on one block, not the oracle's.
- Neutral: four more weekly arms, each pinned whole.

## Alternatives considered and why rejected

- **A unit at rest, a bound from the wiring, a recorded input** (options 1(b) to 1(d)), **every tick or two phases** (options 2(b), 2(c)): see above.
- **A weekly test of its own for the check** (option 3(b)): the check is a calibration of the arms that carry the hold and costs them two frozen blocks.
- **A shadow under the drawn address beside the run** (option 4(b)): H-25 is that run, pinned.
- **The clauses' inputs alone** (option 5(b)): the readings are what the round hands the next decision.

## Confirmation

- `runtime/cortex-runtime/tests/inhibition.rs`: `GATED_ARMS`, `GATED_DELIVERY`, `GATED_PREDICTED`, `ALONE_PREDICTED`, `HOLD`, `HOLD_UNTIL`, `HOLD_CLOSE`, `HOLD_SPAN`, `HOLD_MESSAGES`, `HOLD_EVERY`, `HOLD_CHECK_TIMES`, `SETTLED_TICK`, `trial_start`, `drive_landings`, `held_alone`, `first_fired`, `hold_of`, `least_hold`, `Gated`, `gated`, `step_of`, `Fired`, `Quieted`, `quieted_blocks`, `hold_checked`, `selected_signal`, `GatedRead`, `Along`, `along_hash`, `frozen_held`, `hold_calibration`, `drawn_first_block`, `held_run`, `gated_arm`, `hold_probe`, `hold_alone`, `BARE_FROZEN_1024`, `HELD_FROZEN_1024`; the four weekly tests and the gate's test named above.
- `runtime/cortex-runtime/tests/instrument/harness.rs`: `Composer::released`, `HeldRead`, `hold_times`, `earned_run_held` (ADR-0144).
- Whitepaper §11 (F-63) and §11.1 (H-27); `CHANGELOG.md`.
