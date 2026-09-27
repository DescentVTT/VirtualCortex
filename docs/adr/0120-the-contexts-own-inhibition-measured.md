---
status: accepted
date: 2026-09-28
depends-on: ADR-0119
decision-makers: VirtualCortex maintainers
---

# ADR-0120: The context's own inhibition measured — brief 051's measurement: ADR-0117's assembly of 64 marked members given sixteen added inhibitory synapses a member from the prior's inhibitory units drawn over the ring, at recurrent weights of a quarter to three quarters of Q1.15's range against added inhibitory weights of a quarter, a half and the whole of it, under ADR-0117's four conditions, every weight frozen and ADR-0117's substrate, protocol, measures, thresholds and rules unchanged, the grid, the draw, the rules and the arithmetic written before any run; no cell of the core grid is usable — at a quarter of the range the inhibition keeps the assembly from holding at every inhibitory weight, and from three eighths up every cell ignites without a kick — so no cell is robust and there is no cell for the second round; the inhibition moves the assembly's threshold rather than widening the gap between holding and igniting, and its sources fire 1.4 to 2.0 times faster while the assembly bursts; the next decision, among a feedback inhibition, the context's isolation, the class's constants and the inhibitory drain, is named and not taken

## Context and Problem Statement

[ADR-0119](0119-an-inhibition-of-the-contexts-own.md) took the context's own inhibition among the branches [ADR-0117](0117-the-region-measured.md) named for a region no condition leaves robust. Brief 051 measures it and changes no rule of the engine. This ADR is its protocol, committed before any run, and then its readings.

What was read before anything was written (principle 2):

1. **ADR-0117's substrate** (`runtime/cortex-runtime/tests/assembly.rs`): the geometry (`R0_MASK_050`, `R1_MASK_050`, `context_geometry`), `members`, set (ii) (`SET_050`), `marked`, `grown`, `wire_drawn`, the kick, `layout`, `span_protocol_under`, `run_050`, `backgrounds_controls_050`, `holding`, `failed`, `bursts_049`, the conditions (`Condition`, `DELAY_SEED_050`, `drained_image`, `frozen_again`, `stimulus_task`, `STIMULUS_AT`), the rules (`robust_cells`, `usable_neighbours`, `round_two_cell`, `failures_050`) and the pins (`GRID_RUNS_050`, `GRID_050`, `CONDITION_RUNS_050`, `CONDITION_GRID_050`). At 64 units a member sends and receives 32 recurrent synapses (`fan`).
2. **The prior's inhibitory units** (`crates/cortex-connectome/src/prior.rs`, `runtime/cortex-runtime/src/synthesis.rs`): unit $i$ when $(i + 1) \bmod 5 = 0$, 204 of them at 1 024 units, at places 4, 9, 14 and 19 of each period of twenty. Synthesis sets their `FLAG_INHIBITORY`. The prior draws an inhibitory synapse's magnitude as an excitatory weight of 6 000 to 12 000 times 255/16 and saturates it at the width, so every one leaves at $-32\,767$, `-i16::MAX`, one LSB above $-1.0$. Each of the four places is a readout place of the task's geometry and of ADR-0116's (the gate).
3. **What a spike releases** (`runtime/cortex-runtime/src/executor.rs`): at a presynaptic spike the executor walks the unit's whole chain, releases every slot under the unit's pair (`release_all`), stores the efficacy in the slot (`last_release_q16`) and delivers it after the slot's delay. The polarity every rule reads is the unit's flag (`Polarity::of_flags`), so a synapse appended to an inhibitory unit's chain is an inhibitory synapse to every rule.
4. **`wire_drawn` asserts that the arena holds exactly the prior's blocks and the assembly's.** Brief 051's arena holds the inhibition's after them, so the wiring is split: `wire_among` wires the assembly into an arena that holds at least those, and `wire_drawn` keeps its check and calls it. Every earlier run wires as it did.
5. **ADR-0117's readings count the members' spikes and the rest's**, not the inhibitory units'. The sources' spikes are counted in every window by `span_protocol_counting`, which `span_protocol_under` now calls with no units to count: a reading of the train, which no input of a run sees.
6. **An unwired assembly draws no delay.** The inhibition's draw is its own seed's under every condition, so condition (b)'s backgrounds and controls would be the core's bit for bit, as under ADR-0117.
7. **H-20's arm dominates condition (c)'s cost**: ADR-0117's test of condition (c), the arm and ten runs, took 1 890 s on the hosted runners, where a run took 60 to 100 s.

## Decision Drivers

- Brief 051's standing directives:
  - no rule of the engine changes, and every weight is frozen in every measured run — the excitatory baseline zero, the inhibitory baseline, the signed gate and the dopamine signal at rest or unset, no reward — each run's weights at its end, the added ones included, shown equal to its start;
  - Dale's principle: every added synapse leaves an inhibitory unit at a weight at or below zero;
  - ADR-0117's substrate, geometry, class, protocol, spans, release, stretch, measures, thresholds, conditions and rules unchanged, and its cell of 64 units at 0.25 with no added inhibition reproduced bit for bit under each condition before any other cell of that condition runs;
  - the grid, the sources' draw, the rules and the arithmetic written before any run and moved by none; the grid may be widened before the first run and never narrowed;
  - the kick read on the engine under every condition and inhibitory weight before those cells;
  - no float, every loop ended by construction; the runtime's gate grows by at most one test; no shard of the weekly job past 60 per cent of its bound under the regenerated deal.
- One difference from ADR-0117: the added inhibition, and the recurrent weights set against it.

## Considered Options

1. **The sources' draw**: (a) sixteen distinct inhibitory units a member over the whole ring, a unit within the prior's window of the member allowed and a source shared by several members allowed; (b) the window's units excluded; (c) no source shared, which the ring cannot give: 1 024 draws over 204 units.
2. **The whole of the range, negative**: (a) $-32\,768$, `i16::MIN`, exactly $-1.0$; (b) $-32\,767$, the prior's own inhibitory rail.
3. **Condition (b)'s backgrounds and controls**: (a) run again; (b) the core's.
4. **The grid**: the brief's twelve cells, or widened by a weight between two, a size of 96, or eight or thirty-two sources a member.
5. **The weekly tests**: how the 70 runs and condition (c)'s arm are dealt.

## Decision Outcome

**Options 1(a), 2(b), 3(b), 4 the brief's grid, not widened, and 5 ten tests.** Everything below is committed before any run of the measurement, with the arithmetic and the gate.

### The inhibition (the test's wiring)

- **The draw** (`sources_of`): for each of the 64 members, a partial Fisher–Yates shuffle of the 204 inhibitory units in ascending order, the swap of step $i$ drawn by `mix64` of the seed 51 (`INHIBITION_SEED`), the member and $i$, sixteen steps. The sixteen are distinct by construction. Every inhibitory unit of the ring can be drawn, the window's among them, and members share sources (option 1(a)): the brief's draw, and the one whose sources fire as the whole network does.
- **What the draw gives** (`SOURCES_051`, computed by the gate): 203 of the 204 inhibitory units are sources; a source sends 1 to 13 added synapses; 20 of the 1 024 draws fall within the prior's window of their member.
- **The blocks** (`inhibition_plan`, `wire_inhibition`): each source's added synapses, in the members' order, four to a block, 332 blocks appended after the assembly's 512. Each synapse is basal, at the cell's inhibitory weight, with a delay from the prior's local band, 100 to 300 ticks, drawn by `mix64` of seed 51, the source and the member (`delay_drawn`). A source's first block is chained after the last block of its own chain. A background and a control are grown by both sets of blocks and wire only the inhibition.
- **The sources' spikes** are counted in every window beside the rows (the union of the sources, 203 units), and pinned by kind of span with every window's count as one hash.

### The grid

- **Recurrent weights** (`RECURRENT_051`): 0x2000, 0x3000, 0x4000 and 0x6000, a quarter, three eighths, a half and three quarters of Q1.15's range.
- **Added inhibitory weights** (`INHIBITORY_051`): $-$0x2000, $-$0x4000 and $-$0x7FFF, a quarter, a half and the whole of the range. The whole is the prior's rail (option 2(b)): the prior never writes $-1.0$ itself.
- Twelve cells at 64 members, and beside them **ADR-0117's cell**, 64 units at 0.25 with no added inhibition (`CELL_050`), run through ADR-0117's own `run_050` under each condition and held to ADR-0117's pins: the stretches, the kicks, the windows' and epochs' hashes, the reading, the bursts and, under (d), the readouts.

### The conditions, the backgrounds and the controls

- **The conditions are ADR-0117's**, each built as ADR-0117 built it: the core (the settled image, the assembly's delays of seed 48); (b) the assembly's delays of seed 49 (`DELAY_SEED_050`); (c) the image H-20's arm from the assignment leaves, the arm held to ADR-0110's accuracy sequence and sums and the frozen image to ADR-0117's reading of it (`DRAINED_050`); (d) the task's stimulus at every epoch's middle. The inhibition's draw and delays are seed 51's under all four.
- **A background and a control per condition and inhibitory weight**: the background marked, grown and with the inhibition wired, the assembly not, no kick and no release; the control the same with the kick and the release. Each cell is read against its condition's background at its inhibitory weight.
- **Condition (b)'s are the core's** (option 3(b)), by item 6 above; its kick is the core control's.

### The protocol, the measures and the thresholds

ADR-0117's, unchanged, for every run: the lead-in of sixteen epochs; eight rounds of an unkicked span, a hold span with ADR-0112's kick, a release of six stretches and a tail; `holding`, `failed`, `bursts_049`, `HOLD_TIMES`, `LET_GO_TIMES`, `SPILL_TIMES`, `OF_EIGHT_MIN` and `IGNITIONS_MAX`. A run is 400 epochs. Every run is dumped before any is held to its table, and every weight of the arena at each run's end, the added ones included, is asserted equal to its value at the start. The tables are pinned as ADR-0117's (the stretches in full, the windows by hash, under (d) the epochs by hash), and each run's sources' spikes by kind of span with their windows' hash (`Pinned051`).

### The rules, before any run

ADR-0117's, over a grid of recurrent and inhibitory weights (`Grid051`, `[recurrent][inhibitory]`):
- **Robust** (`robust_051`): a cell usable in the core grid and under each of (b), (c) and (d).
- **The cell for the second round** (`round_two_051`): of the robust cells, the one with the most usable neighbours in the core grid (`usable_neighbours`, now over any grid of readings). A cell's neighbours are the cells one recurrent step either way at its inhibitory weight and one inhibitory step either way at its recurrent weight, at most four; a diagonal is not one. Ties go to the lighter recurrent weight, then the lighter inhibition.
- **If no cell is robust**: what failed in each cell in the core grid and under each condition, each by ADR-0115's `failed` (`failures_051`).

The gate holds each rule at its edges over tables written by hand.

### Readings, no clause

- **The core grid's table** and its usable region, each condition's table, and the bursts of every cell (`bursts_049`).
- **The sources' rates and the members' under each condition and inhibitory weight**, from the backgrounds.
- **Under (d), each readout's count per trial with the context held and with it quiet** (`task_read`), for the backgrounds, the controls and the cells, and ADR-0065's measure on each background (`sight_050`).
- **Under (c), the members' input from the drained image's own prior synapses** at (c)'s rates (`DRAINED_INPUT_051`), beside the arithmetic's estimate of it.

### The arithmetic, before any run

Computed before any run: by the gate for the first three tables, and on the settled image, before the core test's first run, for the last two (`arithmetic_051`, held there to `ARITHMETIC_051`). Every rate is ADR-0117's at 64 units: the members' and the rest's of its backgrounds, the rest's standing for the sources', since ADR-0117 did not count the inhibitory units apart. An input is the basal potential it would hold at that rate, the input per tick times the basal leak's $2^9$ ticks, in units of the threshold; the drive's mean standing holds the basal at 0.875.

**One spike of a source** (`INHIBITION_DELIVERED_051`), at the pair of ADR-0019's constants steady at the rest's rate in the core, after the gain, and the soma's trough from the drive's mean standing, 0.436. One member's spike at 0.25 delivers 0.107 (ADR-0117's table), so one source's spike at a quarter of the range cancels about 1.4 of them, and at the whole about 5.5.

| Inhibitory weight | Delivered | Soma's trough |
| :--- | ---: | ---: |
| −0.25 | −0.148 | 0.367 |
| −0.5 | −0.296 | 0.297 |
| −1.0 | −0.591 | 0.159 |

**The assembly's input to a member** (`ASSEMBLY_INPUT_051`): 32 synapses, every member at the members' rate in the core, 1.613 Hz, at the class's steady pair and at the primed peak's.

| Recurrent weight | Unkicked | Primed |
| :--- | ---: | ---: |
| 0.25 | 0.028 | 0.045 |
| 0.375 | 0.042 | 0.068 |
| 0.5 | 0.056 | 0.091 |
| 0.75 | 0.084 | 0.136 |

**The added inhibition's input to a member** (`INHIBITION_INPUT_051`): sixteen synapses, every source at the rest's rate of each condition's background.

| Condition | −0.25 | −0.5 | −1.0 |
| :--- | ---: | ---: | ---: |
| the core | −0.020 | −0.041 | −0.081 |
| (c) | −0.023 | −0.045 | −0.090 |
| (d) | −0.024 | −0.049 | −0.097 |

**The prior's input to a member** (`ARITHMETIC_051`), every synapse of the settled image onto a member at its source's rate and pair; (c)'s scaled by the drained image's sums by polarity over the settled image's, an estimate the (c) test reads against the drained image itself.

| Condition | Excitatory | Inhibitory | Net |
| :--- | ---: | ---: | ---: |
| the core | 0.031 | −0.026 | 0.005 |
| (c) | 0.035 | −0.002 | 0.033 |
| (d) | 0.037 | −0.031 | 0.006 |

**What the added inhibition's rise takes back** (`ARITHMETIC_051`), of the prior's excitatory change from the core and of its net change:

| Condition | Of | −0.25 | −0.5 | −1.0 |
| :--- | :--- | ---: | ---: | ---: |
| (c) | the excitatory change, 0.004 | 0.62 | 1.25 | 2.50 |
| (c) | the net change, 0.027 | 0.08 | 0.16 | 0.33 |
| (d) | the excitatory change, 0.006 | 0.66 | 1.32 | 2.64 |
| (d) | the net change, 0.001 | 4.1 | 8.3 | 16.6 |

**What the arithmetic says, and does not:**
- **Every mean input is small beside the drive.** The prior's net input to a member is 0.005 of the threshold in the core against the drive's 0.875: the members fire by the drive's fluctuations, and ADR-0117's cells ignite by bursts, which no mean describes.
- **At the whole of the range the added inhibition is a larger mean input than either the prior gives a member**, 0.081 against its excitatory 0.031 and inhibitory 0.026: about the unkicked assembly's input at 0.75, 0.084, and nine tenths of the primed one's at 0.5, 0.091. At a quarter it is 0.020, three quarters of the unkicked assembly's at 0.25. The grid spans that balance.
- **Under (c) the drain removes nine tenths of the prior's inhibition onto the members**, so the net input rises by 0.027; the added inhibition's rise takes back 8 to 33 per cent of it, and it is not drained itself.
- **Under (d), by mean rates, the prior's inhibition rises with its excitation**, and the added inhibition's rise is 4 to 17 times the net change. But the rest's rate under (d) counts the stimulus's own units, whose volleys reach the members together, and a mean does not describe that; the sources' own rates are read from the backgrounds.
- The arithmetic places the grid. It makes no prediction of the verdict, and none is made.

### The gate (`an_inhibition_of_the_contexts_own_the_wiring_the_grid_the_arithmetic_the_rules_and_an_assembly_kicked_with_it`)

The one runtime test this measurement adds:
- the grid, ADR-0117's cell beside it, the inhibitory weights below zero and the whole at the prior's rail, the conditions ADR-0117's;
- the prior's inhibitory units, 204, each at a readout place of the new geometry;
- the draw: sixteen distinct inhibitory units a member, the plan the draw, and its reading (`SOURCES_051`);
- **the wiring on the instrument's network** (`inhibition_on_the_prior`), beside the same engine with nothing wired: every synapse the prior wrote where it was, with its target, weight, delay and compartment; each source's chain its 32 prior synapses and then its added ones as the plan says, basal, at the inhibitory weight, each delay in the local band as drawn; every member receiving sixteen from sixteen distinct inhibitory units and no other unit any; Dale's principle over the arena; and one source fired alone with no drive, nothing else firing and no weight moving, having released through every added synapse the efficacy its weight gives under the pair of its last spike;
- the arithmetic, dumped, then held;
- the rules at their edges, and the sources' reading by kind of span;
- **a marked assembly with its inhibition on the new geometry kicked for a few hundred ticks**: 64 members under set (ii) at 0.75 with the inhibition at the whole of the range, on the instrument's network at rest, kicked through the protocol's own kick for 800 ticks (`kicked_on`, the body of ADR-0117's gate function). Every member fires once in the span, every reset is scheduled, no weight moves, and every member's short-term state is the class's step over its spikes; the spikes are pinned (`GATE_KICKED_051`);
- **over the pinned tables**, once pinned: each run read again by the rules, the kick under every condition and inhibitory weight, and the reading.

### The weekly tests and their cost

Ten tests, each `#[ignore]`d with `exhaustive` in its name, each building the settled image once:
- **the backgrounds and controls** of the core, of (c) and of (d), each at the three inhibitory weights, with ADR-0117's cell reproduced first — under the core and (b) in the core's test, under (c) and (d) in theirs — so that all four reproductions and every background are pinned before any cell of this round runs; the core's test computes the arithmetic on the settled image before its first run;
- **the cells**: of the core, of (b) and of (d), each in two tests of six, the two lighter recurrent weights and the two heavier; of (c) in one test of twelve.

That is 70 runs of 400 epochs — 18 backgrounds and controls, 4 reproductions and 48 cells — and two arms of H-20. At ADR-0117's rates on the hosted runners, about 75 s a run and 1 100 s an arm, the round adds about 8 100 s: eight tests of 500 to 700 s, and condition (c)'s two of about 1 700 and 2 100 s. **Condition (c)'s tests cannot be of the 900 s the cost table gives a test it does not know**, since the arm alone is above it; they are two rather than one so that the dispatch, which costs every new test at 900 s, puts each on its shard at most about 1 200 s over its plan. The table sums 25 743 s over 88 tests, so the five shards would plan about 6 770 s each, near 3 400 s of wall time at ADR-0117's ratio of 1.97 to 2.00: about 47 per cent of the bound, under the brief's 60. The dispatch reads it, and the cost table is regenerated from it.

### The order of the work

1. This protocol, the rules, the arithmetic pinned and the gate, committed before any run of the measurement and pushed with the pull request opened as a draft.
2. The backgrounds' three tests, each reproducing ADR-0117's cell first and reading the kick on the engine at every inhibitory weight; pinned.
3. The cells' seven tests, each against the backgrounds pinned first; pinned.
4. The readings, the documents, the weekly dispatched on the round's branch at `scope=exhaustive`, its evidence and the cost table regenerated from it.

### The readings

Every reading below is from the pinned tables of `tests/assembly.rs`. The runs were made in the order above, from the protocol's commit, `f696974`:
- the three backgrounds' tests, ADR-0117's cell reproduced first in each, pinned at `7e55921` before any cell of this round ran;
- the seven cells' tests, against those pins, pinned at `6173ed3`.

On a developer machine in the release profile a run took about 100 s with six other tests beside it, and H-20's arm about 25 minutes (a ratio, not admissible). **No weight of any arena moved in any measured run, the added synapses' included.**

- **ADR-0117's cell, reproduced.** 64 units at 0.25 with no added inhibition, run through ADR-0117's `run_050`, held to ADR-0117's stretches, kicks, windows' and epochs' hashes, reading and bursts under each of the four conditions, and under (d) to its readouts: usable in the core and under (b), igniting in 2 rounds under (c) and in 4 under (d), as ADR-0117 read it. Condition (c)'s arm reproduced ADR-0110's accuracy sequence and sums, and its frozen image ADR-0117's reading of it.
- **The backgrounds** (the inhibition wired, the assembly not, the drive alone after the lead-in; ADR-0117's at 64 units beside them):

  | Condition | Inhibitory weight | Members, Hz | Rest, Hz | Sources, Hz |
  | :--- | :--- | ---: | ---: | ---: |
  | the core | none (ADR-0117) | 1.613 | 1.679 | — |
  | the core | −0.25 / −0.5 / −1.0 | 1.451 / 1.350 / 1.229 | 1.672 / 1.668 / 1.663 | 1.966 / 1.960 / 1.954 |
  | (c) | none (ADR-0117) | 1.818 | 1.897 | — |
  | (c) | −0.25 / −0.5 / −1.0 | 1.625 / 1.495 / 1.348 | 1.888 / 1.880 / 1.873 | 2.247 / 2.234 / 2.223 |
  | (d) | none (ADR-0117) | 1.735 | 2.077 | — |
  | (d) | −0.25 / −0.5 / −1.0 | 1.547 / 1.433 / 1.298 | 2.069 / 2.065 / 2.060 | 2.120 / 2.114 / 2.106 |

  The added inhibition lowers the members' background by 10 to 11, 16 to 18 and 24 to 26 per cent at the three weights, under every condition alike, and the rest's by 0.4 to 1.3 per cent. The sources, the prior's inhibitory units, fire faster than the rest, 1.95 to 1.97 Hz in the core; under (c) 14 per cent faster than in the core, under (d) 8 per cent.
- **The kick, read on the engine before each condition's cells** (each control's eight kicks) fires every member once by ADR-0112's measure at every inhibitory weight under every condition: the volley is 512, every kick a full one, and the after 6, 6 and 4 in the core and under (d) and 18, 18 and 14 under (c), against a mark of 51.2. It was not derived again. Each control's lead-in and first unkicked span are its background's bit for bit, and no control holds, ignites or fails to let go.
- **The core grid** (`GRID_051[0]`, by the rules committed first, against the backgrounds pinned before any cell ran):

  | Recurrent | Inhibitory | Holds, of 8 | Ignites, of 8 | Lets go, of 8 | What failed |
  | :--- | :--- | ---: | ---: | ---: | :--- |
  | 0.25 | −0.25 / −0.5 / −1.0 | 2 / 1 / 0 | 0 | 8 | never holding |
  | 0.375 | −0.25 / −0.5 / −1.0 | 8 | 8 / 8 / 5 | 3 / 5 / 6 | running away; not letting go |
  | 0.5 | −0.25 / −0.5 / −1.0 | 8 | 8 | 0 / 0 / 1 | running away; not letting go |
  | 0.75 | −0.25 / −0.5 / −1.0 | 8 | 8 | 0 | running away; not letting go |

  No cell spills. **No cell of the core grid is usable** (`USABLE_051` empty). The added inhibition moved the holding edge above a quarter of the range at every inhibitory weight — ADR-0117's usable cell, 64 units at a quarter, holds in 2, 1 and 0 rounds of 8 with it — and from three eighths up every cell ignites without a kick and does not let go. The heaviest inhibition is the nearest: at three eighths and the whole of the range the cell ignites in 5 rounds and lets go in 6. Between a quarter and three eighths the grid has no step.
- **The bursts** (`BURSTS_051`) say how. At a quarter the kick sets off the members' burst and the class's priming does not keep it: the hold spans hold 76, 23 and 8 burst windows at the three inhibitory weights, against ADR-0115's 140 without the inhibition, and the unkicked spans none. At three eighths the unkicked spans hold 238, 204 and 105 burst windows, and at a half 351, 340 and 313: the heavier inhibition makes the ignitions rarer and does not stop them. At three quarters the assembly bursts in almost every window outside the release: 1 019 to 1 024 of the unkicked spans' 1 024 at the two lighter inhibitions, 891 at the heaviest.
- **The sources follow the assembly.** In every cell that ignites in more than one round, the sources fire at 2.8 to 4.5 Hz over the unkicked spans, 1.4 to 2.0 times their background rate (the sources' reading of `CELL_RUNS_051`): the assembly's bursts drive the network the sources sit in, so the added inhibition rises with the assembly's own activity, after the prior's delays, and not only with the network's. It did not stop the ignitions.
- **The conditions** (`GRID_051[1..]`, each against its own backgrounds):

  | Condition | Recurrent | −0.25 | −0.5 | −1.0 |
  | :--- | :--- | :--- | :--- | :--- |
  | (b) seed 49 | 0.25 | holds 1 | holds 1 | holds 0 |
  | (b) seed 49 | 0.375 | ignites 8, lets go 3 | ignites 8, lets go 3 | ignites 6, lets go 6 |
  | (b) seed 49 | 0.5 | ignites 8, lets go 0 | ignites 8, lets go 1 | ignites 8, lets go 2 |
  | (b) seed 49 | 0.75 | ignites 8, lets go 0 | ignites 8, lets go 0 | ignites 8, lets go 0 |
  | (c) drained | 0.25 | holds 6 | holds 3 | holds 0 |
  | (c) drained | 0.375 | ignites 8, lets go 0 | ignites 8, lets go 3 | ignites 8, lets go 5 |
  | (c) drained | 0.5 | ignites 8, lets go 0 | ignites 8, lets go 0 | ignites 8, lets go 0 |
  | (c) drained | 0.75 | ignites 8, lets go 0 | ignites 8, lets go 0 | ignites 8, lets go 0 |
  | (d) stimulus | 0.25 | holds 5, ignites 1 | holds 3 | holds 0 |
  | (d) stimulus | 0.375 | ignites 8, lets go 0 | ignites 8, lets go 3 | ignites 7, lets go 7 |
  | (d) stimulus | 0.5 | ignites 8, lets go 0 | ignites 8, lets go 0 | ignites 8, lets go 0 |
  | (d) stimulus | 0.75 | ignites 8, lets go 0 | ignites 8, lets go 0 | ignites 8, lets go 0 |

  Every cell from three eighths up holds in all 8 rounds under every condition; none spills. At a quarter the drained network and the stimulus hold the assembly in more rounds than the core does, 6 and 5 at −0.25 against 2, and never in the 7 the rule asks. **No cell is robust** (`ROBUST_051` empty), **so there is no cell for the second round** (`ROUND_TWO_051` none). What failed (`FAILURES_051`): every cell at a quarter never holds under every condition; every heavier cell runs away under every condition and does not let go under every condition but one, three eighths at the whole of the range under (d), which lets go in 7 rounds and ignites in 7.
- **The arithmetic beside the readings.**
  - The arithmetic took the sources at the rest's rate; they fire 17 per cent faster, so the added inhibition's mean input is about a sixth above its table. Under (d) the sources rose 8 per cent, not the rest's 24, as the arithmetic's own caveat said; under (c) 14 per cent, as the rest's 13.
  - The members' background fell by 10, 16 and 24 per cent in the core where the added inhibition's mean input is 2, 5 and 9 per cent of the drive's standing: the members fire in the drive's tail, where a small shift of the mean moves the rate by more.
  - Under (c) and (d) the added inhibition took back at most a quarter of the members' excess: their background stood 12.7 per cent above the core's under (c) without it, and 12.0, 10.7 and 9.7 with it; under (d) 7.6 per cent without and 6.6, 6.1 and 5.6 with it. The arithmetic's estimate against the net change under (c), 8, 16 and 33 per cent, is near what was read there, 6, 16 and 24; its estimate against the mean excitatory change, and every estimate under (d), does not, since the members' excess under (d) is the stimulus's volleys and not a rise of the rest's mean.
  - The drained image's own prior synapses give the members 0.035 of excitatory and 0.002 of inhibitory input at (c)'s rates (`DRAINED_INPUT_051`, 2 274 and −110), beside the arithmetic's scaled estimate of 2 267 and −136.
  - No mean describes the failure: every cell that fails by running away fails by bursts, which is what ADR-0117 read.
- **Under (d), the readouts** (`BG_TASK_051`, `CELL_TASK_051`). The added inhibition leaves the backgrounds' readouts within about a tenth of a spike a trial of ADR-0117's, 7.67 to 7.68 and 9.30 to 9.32 on A's trials and 8.55 to 8.58 and 9.68 to 9.74 on B's with the context quiet, and ADR-0065's measure resolves the stimulus in 58 to 64 trials of 64 in every block at every inhibitory weight (`BG_SIGHT_051`). In the cell that holds most at a quarter, against −0.25 (5 rounds), the held context adds 2.0 and 1.5 spikes a trial to readout 0 and readout 1 on A's trials and 1.3 and 0.7 on B's: to both, as ADR-0117 read. In every cell that ignites the quiet spans are not quiet, and both readouts count 1.3 to 3.8 spikes a trial more than the background's there.
- **The evidence.** Only tests and documents change, so by [ADR-0075](0075-the-dispatch-scope-follows-the-diff.md) the weekly was dispatched on this round's branch at **`scope=exhaustive`**: run [36346275507](https://github.com/DescentVTT/VirtualCortex/actions/runs/36346275507) at `9937acc`, the documents commit, after which only this evidence, the cost table and the brief's archive change. **Every job it ran is green.**
  - **The whole-domain tests.** All ninety-eight passed on the hosted runners: the eighty-eight before this round reproduced their pinned numbers, ADR-0115's and ADR-0117's among them through the refactored functions, and this round's ten reproduced their tables. The cost table did not know the ten and costed each at 900 s; on the runners eight took 477 to 776 s, condition (c)'s backgrounds 1 678 s and its cells 2 680 s, 9 520 s in all against the protocol's estimate of about 8 100. The eighty-eight earlier tests took 30 743 s against the table's 25 743, 19 per cent more on these runners. The five shards took:

    | Shard | Job | Tests | Their seconds summed | Tests' wall time | The shard's heaviest test (s) |
    | ---: | ---: | ---: | ---: | :--- | :--- |
    | 0 | 60 m 48 s | 19 | 7 146 | 3 602 s, 50 % | ADR-0117's condition (c) 1 868 |
    | 1 | 71 m 14 s | 19 | 8 314 | 4 223 s, 59 % | H-20 from the assignment 1 761 |
    | 2 | 88 m 12 s | 20 | 10 462 | 5 242 s, 73 % | this round's condition (c) cells 2 680 |
    | 3 | 64 m 02 s | 20 | 7 450 | 3 786 s, 53 % | plasticity everywhere 1 115 |
    | 4 | 58 m 45 s | 20 | 6 891 | 3 468 s, 48 % | inhibition off the gate 1 092 |

    **Shard 2 passed the brief's 60 per cent in this run, at 73.** It drew this round's condition (c) cells, costed at 900 s and taking 2 680, 1 780 s over their plan where the protocol said about 1 200, beside H-20's arm from the mirrored assignment, which took 1 896 s against the table's 1 275. The brief's rule reads the regenerated deal, below; this run is what a round's dispatch costs when the table does not know its tests.
  - **The cost table is regenerated from this run's artifacts** (`scripts/exhaustive-costs.tsv`: ninety-eight lines, 40 263 s, its source line naming the run; `npm run spec:costs` passing). Replayed through the deal, it plans each shard at 8 052 to 8 053 s summed. At this run's ratio of summed seconds to wall time, 1.97 to 2.00, that is about 4 030 to 4 090 s of tests' wall time, **56 to 57 per cent of the bound**: under the brief's 60, with less room than after any round before, since the table now carries these runners' slower times for every test. A round that adds about 2 000 s more on runners as slow would pass it.
  - **No mutation sweep**, by the scope: the diff changes nothing under `src/`, so the sweep has nothing to find.

  The pull request's gate on `9937acc` is green in every job: check, test, fmt and clippy; the AArch64 determinism pin, unmoved; the MSRV job; the documentation gate; and the mutation gate on the changed lines, which found no mutant to make.

  On the developer machine every command of brief 051's verification list exited 0 (a ratio, not admissible):
  - the workspace in the debug profile, in the release profile and on the MSRV toolchain in a target directory of its own: 651 passed in each and 98 ignored;
  - check, fmt, clippy, doc, the bench `--test`, `npm ci` and `npm run spec`;
  - `--list`, 98 tests;
  - the in-diff mutation run, "No mutants to filter";
  - this round's ten whole-domain tests twice, the second run at `6173ed3` against the pins, every table reproduced.
- **Not done:**
  - The grid was not widened: nothing between a quarter and three eighths, where the region would lie if anywhere; no size of 96; no eight or thirty-two sources a member.
  - No feedback inhibition, no readout gated by the context, no switch on errors and no reward in a measured run.
  - The added synapses were frozen; whether a context's inhibitory synapses learn under the inhibitory baseline stays ADR-0119's open consequence.
- **The next decision, named and not taken.** By ADR-0119's branch for a round in which no cell is robust: the feedback inhibition (ADR-0119's option 1(b)), the context's isolation from the task's stimulus, the class's constants, or the inhibitory drain. An ADR choosing among them has as its need these readings:
  - an inhibition driven by the network's inhibitory units moves the assembly's threshold and not the gap between holding and igniting: at a quarter of the range nothing holds under any of the three inhibitory weights, and from three eighths up every cell ignites; the heaviest inhibition makes the ignitions rarer, 105 burst windows in the unkicked spans against 238, and does not stop them;
  - the sources fire 1.4 to 2.0 times faster while the assembly bursts, so part of this inhibition follows the assembly after the prior's delays, and it did not stop an ignition once begun;
  - under (c) and (d) it took back at most a quarter of the members' excess rate;
  - a cell between a quarter and three eighths under the heavier inhibition is unread.

  ADR-0111's second round is not taken.

## Consequences

- Good: the added inhibition is measured under the same conditions, protocol and rules as the reading it answers, beside ADR-0117's cell reproduced under each.
- Good: nothing of the engine changes; every earlier run wires and reads as it did, and its pins stand.
- Bad: no cell is usable. The inhibition the network drives lowered the members' excitability as a whole: at a quarter of the range nothing holds, and from three eighths up every cell ignites, so the region is no wider than ADR-0117's and lies, if anywhere, between two of the grid's steps.
- Bad: the sources are shared and fire together with the network, so a cell's inhibition is not sixteen independent inputs; the draw is one seed's.
- Neutral: the sources fire faster while the assembly bursts, so the inhibition ADR-0119 chose as the network's is in part the assembly's own already, after the prior's delays.
- Bad: the weekly job grows by 9 520 s on the runners by this round's dispatch, condition (c) paying H-20's arm twice, and the regenerated deal plans each shard at 56 to 57 per cent of its bound, near the brief's 60.
- Neutral: the arithmetic's rates for the sources are the rest's, which under (d) count the stimulus's own units; the backgrounds read the sources' own.

## Alternatives considered and why rejected

- **The window's units excluded** (option 1(b)): 20 of 1 024 draws; the brief's draw over the ring keeps the rule simpler and the window's units fire as the network does.
- **$-32\,768$** (option 2(a)): a weight the prior never writes; the whole of the range the network has is its rail.
- **(b)'s own backgrounds and controls** (option 3(a)): six runs whose every tick is the core's.
- **A wider grid** (option 4): the budget; a widening is a later round's, where the reading says it matters.
- **Condition (c) in one test**: one arm fewer, about 1 100 s, and one test of about 2 600 s, which the dispatch would cost at 900 s on one shard.

## Confirmation

`runtime/cortex-runtime/tests/assembly.rs`:
- the inhibition and the grid: `SIZE_051`, `CELL_050`, `SOURCES_PER_MEMBER`, `INHIBITION_SEED`, `RECURRENT_051`, `INHIBITORY_051`, `CONDITIONS_051`, `inhibitory_units`, `sources_of`, `inhibition_plan`, `inhibition_sources`, `inhibition_blocks`, `wire_inhibition`, `engine_051`, and `wire_among`, `span_protocol_counting` and `kicked_on` beside the functions that call them;
- the rules: `Grid051`, `robust_051`, `usable_neighbours`, `round_two_051`, `failures_051`;
- the arithmetic: `inhibition_delivered`, `assembly_input`, `inhibition_input`, `prior_input`, `arithmetic_051` and their tables `SOURCES_051`, `INHIBITION_DELIVERED_051`, `ASSEMBLY_INPUT_051`, `INHIBITION_INPUT_051`, `ARITHMETIC_051`;
- the runs: `run_051`, `reproduce_050`, `backgrounds_controls_051`, `cells_051`, the gate and the ten weekly tests;
- the runs' tables `BG_RUNS_051`, `BG_READ_051`, `BG_TASK_051`, `BG_SIGHT_051`, `DRAINED_INPUT_051`, `CELL_RUNS_051`, `GRID_051`, `BURSTS_051`, `CELL_TASK_051`, and the reading `USABLE_051`, `ROBUST_051`, `ROUND_TWO_051` and `FAILURES_051` (`over_the_051_tables`).

Whitepaper §11.1's question on a rule held by the network carries the reading, and §9 its row.
