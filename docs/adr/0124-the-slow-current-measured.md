---
status: proposed
date: 2026-09-28
depends-on: ADR-0123
decision-makers: VirtualCortex maintainers
---

# ADR-0124: The slow current measured — brief 052's measurement: ADR-0117's assembly of 64 members marked for ADR-0123's slow current, (A) under ADR-0019's short-term plasticity and (B) marked facilitating under ADR-0114's set (ii) as well, at recurrent weights of an eighth to a half of Q1.15's range and three input shifts placed by an arithmetic of the slow rule itself, the slow potential leaking over $2^{13}$ ticks and its gate opening from the drive's mean standing to the threshold's base, on ADR-0117's substrate, geometry, protocol, measures, thresholds, conditions and rules, every weight frozen; the grid, the shifts, the arithmetic and the rule that picks the cells run under the conditions written before any run; the backgrounds read first, and there the slow current, fed by the prior's synapses alone, raises the unwired members' rate from ADR-0117's 1.6 Hz to 4 to 54 Hz, since its gate at the drive's mean standing opens on every upward excursion of the soma and so lowers the distance a fluctuation must cover to the threshold by the ratio of the slow potential to its critical level; the kick's volley is a full one at every arm and shift and its second clause fails at every one, at shifts 0 and 1 because the members' own background alone passes its mark, so by the protocol no cell runs, and the maintainers chose to stop the round there rather than move the measure after a run; the next decision named and not taken

## Context and Problem Statement

[ADR-0122](0122-a-slow-voltage-gated-current.md) took a slow, voltage-gated current as the one candidate for persistence the four rounds before it had not tried, and [ADR-0123](0123-the-slow-current-built.md) built it. Brief 052 measures it on ADR-0117's assembly. This ADR is the measurement's protocol, committed before any run, and then its readings.

What was read before anything was written (principle 2):

1. **ADR-0117's substrate and ADR-0120's refactors** (`runtime/cortex-runtime/tests/assembly.rs`): `members`, `grown`, `wire_drawn`, the kick, `layout`, `span_protocol_counting`, `run_050`, `holding`, `failed`, `bursts_049`, `usable_neighbours`, the conditions (`Condition`, `DELAY_SEED_050`, `drained_image`, `frozen_again`, `stimulus_task`, `STIMULUS_AT`) and the pins of ADR-0117's cell (`GRID_RUNS_050`, `GRID_050`, `GRID_BURSTS_050`, `CORE_READ_050`).
2. **What the slow input is on the engine** (ADR-0123): the positive efficacies of the synapses' messages that land in a marked unit's basal compartment, after the gain. In a background run the members are unwired, so it is the prior's excitatory basal synapses alone; in a cell it is those and the assembly's 32 recurrent synapses a member. The drive, the kick, the release's cancel and the task's stimulus are injected messages, which give it nothing.
3. **The kick is ADR-0112's**: a ramp of injected messages and a reset. It gives the slow potential nothing directly; in a cell the volley's recurrent spikes do, which is how a kick can start a hold.
4. **ADR-0019's constants never prime a member** (ADR-0113): from any state, a silence leaves a member releasing at least the unkicked product, so under arm (A) there is no priming for the release to outlast (`fade(StpClass::REFERENCE)` is none). Under set (ii) the release of six stretches outlasts the class's fade (ADR-0115).
5. **The protocol reads no slow potential yet.** `span_protocol_counting` gains a reading of the records, which no input of a run sees: while the engine carries a slow current, every window's sums over the members and its ticks of the slow potential and of the gate's opening at the soma as each tick left them. Without one it reads nothing, and every earlier run runs as it ran.

## Decision Drivers

- Brief 052's standing directives:
  - no rule of the engine changes but ADR-0123's slow current, and every weight is frozen in every measured run — the excitatory baseline zero, the inhibitory baseline, the signed gate and the dopamine signal at rest or unset, no reward — each run's weights at its end shown equal to its start;
  - ADR-0117's substrate, geometry, protocol, spans, release, stretch, measures, thresholds, conditions and rules unchanged; the marks and the slow constants are the difference;
  - ADR-0117's cell reproduced with nothing marked for the slow current, bit for bit, before any cell of this round;
  - the grid, the slow constants, the arithmetic and the rule that picks the cells run under the conditions written before any run and moved by none; the grid may be widened before the first run and never narrowed;
  - the arithmetic computed first, with the slow rule itself, and the three input shifts placed by it;
  - the kick read on the engine under each arm and condition before those cells;
  - no float, every loop ended by construction; the runtime's gate grows by at most one test; no shard of the weekly job past 60 per cent of its bound under the regenerated deal.

## Considered Options

1. **The release under arm (A)**: (a) ADR-0117's layout, six stretches, for both arms; (b) a release derived for ADR-0019's constants.
2. **The input shifts**: placed by a rule written before the arithmetic was read, or chosen from its reading.
3. **The slow reading**: (a) a sum over every tick of the window; (b) a sample at the window's end.
4. **The weekly tests**: how the core grid's runs are dealt; when the conditions' tests are written.

## Decision Outcome

**Options 1(a), 2 by a rule, 3(a), and 4 below.** Everything below is committed before any measurement run, with the arithmetic and the gate.

### The arms, the constants and the grid

- **The assembly**: ADR-0117's cell's 64 members at ADR-0112's placement on ADR-0116's geometry (`SIZE_052`), each sending and receiving 32 recurrent synapses, the delays of seed 48.
- **The arms** (`ARMS_052`): every member marked `FLAG_SLOW`; under (A) no class, so the members' synapses release under ADR-0019's constants; under (B) every member marked `FLAG_FACILITATING` as well, the image carrying ADR-0114's set (ii), $(U, \tau_f, \tau_d) = (26/256, 2^{16}, 2^{13})$.
- **The slow constants** (`current_052`): $\tau_s = 2^{13}$ ticks (`LEAK_SHIFT_052`); $V_{lo}$ the soma's standing under the drive's mean input as the oracle reads it, 28 561 (0.436, `DRIVE_STANDING`), held by the gate to `drive_standing`; $V_{hi}$ the threshold's base, 1.0; the input shift $g$ at each of three values (`SHIFTS_052`), placed below.
- **The weights** (`WEIGHTS_052`): 0x1000, 0x2000, 0x3000 and 0x4000, an eighth, a quarter, three eighths and a half of the range. ADR-0117's weight is the second.
- **The grid**: 2 arms × 3 shifts × 4 weights, twenty-four cells, run first under the core condition.
- **Backgrounds and controls**: one of each per arm and shift, as ADR-0117's: the background marked and unwired, not grown, the drive alone; the control grown, marked and unwired, with the kick and the release. Each cell is read against its arm's and shift's background.
- **The release** (option 1(a)): ADR-0117's layout for both arms, six stretches. Its silence, 191 926 ticks, outlasts set (ii)'s fade, 180 192, and sixteen of the slow potential's time constants, 131 072; under ADR-0019's constants there is nothing to fade (Context, item 4). The gate holds all three.

### The protocol, the measures and the thresholds

ADR-0117's, unchanged, for every run: a lead-in of sixteen epochs; eight rounds of an unkicked span, a hold span with ADR-0112's kick, a release of six stretches and a tail; `holding`, `failed`, `bursts_049`, `HOLD_TIMES`, `LET_GO_TIMES`, `SPILL_TIMES`, `OF_EIGHT_MIN` and `IGNITIONS_MAX`. A run is 400 epochs. Every run is dumped before any is held to its table, and every weight of the arena at each run's end is asserted equal to its value at the start. The tables are pinned as ADR-0117's — the stretches in full, the windows by hash, under (d) the epochs by hash — and each run's slow reading beside them (`Pinned052`, `SlowRead`): by kind of span (the lead-in, the unkicked spans, the hold spans' first and second halves, the release spans and the tails) the sums over the members and the span's ticks of the slow potential and of the gate's opening, and every window's sums as one hash (option 3(a)). Bursts under arm (A) are read against ADR-0019's unkicked product (`class_052`).

### The rules, before any run

ADR-0117's, over a grid of three axes, `[arm][shift][weight]` (`Grid052`):
- **The cells the conditions run** (`condition_cells_052`): every cell usable in the core grid; if more than eight are (`CONDITION_CELLS_MAX`), the eight with the most usable neighbours in the core grid. A cell's neighbours are within its arm: the cells one weight step either way at its shift and one shift step either way at its weight, at most four; a diagonal is not one, nor the same cell in the other arm (`neighbours_052`). Ties go by ADR-0117's order: the lighter weight, then the smaller shift, then arm (A) (`tie_key`).
- **Robust** (`robust_052`): a cell the conditions ran that is usable under each of (b), (c) and (d).
- **The cell for the second round** (`round_two_052`): of the robust cells, the one with the most usable neighbours in the core grid, ties by the same order.
- **If none is robust**: what failed in each cell the conditions ran, in the core grid and under each condition, by ADR-0115's `failed` (`failures_052`).

The gate holds each rule at its edges over tables written by hand.

### The conditions

ADR-0117's, each built as ADR-0117 built it, for the cells the rule picks from the core grid's reading:
- **(b)**: the assembly's delays drawn with seed 49; its backgrounds and controls are the core's, since an unwired assembly draws no delay.
- **(c)**: the image H-20's arm from the assignment leaves, the arm held to ADR-0110's accuracy sequence and sums and the frozen image to ADR-0117's reading of it, its CRC held at format 17 (ADR-0123).
- **(d)**: the task's stimulus at every epoch's middle.

Under (c) and (d) each arm and shift that has a cell to run gets its own background and control, the kick read on the engine from the control before the cells. The weekly tests for the conditions are written once the core grid is pinned, their number following the cells the rule picks; the rule itself is this section's, committed first.

### Readings, no clause

- **The core grid's table per arm** and its usable region; the bursts of every cell.
- **The slow potential and the gate** per member and tick: over the unkicked spans and the hold spans' halves (`SlowRead`), and over the held stretches of the hold spans' second halves (`slow_held`), in the backgrounds, the controls and the cells.
- **Under (d)**, each readout's count per trial with the context held and quiet (`task_read`), and ADR-0065's measure on each background (`sight_050`).
- **The arithmetic beside what the runs read.**

### The arithmetic, before any run

Computed with the slow rule itself (`integrate_slow` on one unit): by the gate for what the settled image is not needed for, and on the settled image, before the core tests' first run, for the rest (`arithmetic_052`, held there to `ARITHMETIC_052`). A rate is ADR-0117's at 64 units in the core's background: the members' 1.613 Hz for the assembly's quiet state, and the rest's 1.679 Hz for the prior's sources.

**The prior's input** on the settled image (`ARITHMETIC_052`): 1 577 excitatory synapses of the rest land in the members' basal compartments, 24.6 a member, of mean weight 0.245; and the prior's far band joins members to one another by 28 more, which fire at the members' own rate. At the rest's pair steady at its rate, one of the rest's synapses delivers 0.145 of the threshold after the gain, 9 471; a member takes one every 2 417 ticks on average, 3.9 LSB a tick.

**What the gate needs, by the slow rule** (the gate): the critical level (`SLOW_CRITICAL_052`), the least slow potential that, held, fires a unit at the drive's mean standing under the drive's mean input from an eighth of the gap above it, is **74 589, 1.138 of the threshold**. A current under an open gate raises the soma's steady level by $s \cdot 128 / 257$, so the gap of 0.564 is met at 1.132: the rule and the algebra agree. The gate's opening (`GATE_052`) is nothing at the drive's mean standing, 0.25, 0.50 and 0.75 at a quarter, a half and three quarters of the gap above it, 0.82 at 0.9 and 1.0 at the threshold's base: at the standing the slow current gives the soma nothing, whatever its level.

**The per-spike release** of the held state over the quiet one (`PER_SPIKE_052`), what one spike delivers at the pair steady at 5, 10 and 20 Hz over what it delivers at the members' quiet rate: under arm (A) 0.81, 0.60 and 0.38, since ADR-0019's constants depress; under arm (B) 1.45, 1.71 and 1.51, the 1.4 to 1.6 the four rounds before were bounded by.

**The slow potential's levels**, by the slow rule stepped with the trains of messages each state gives a member (`slow_levels`), after sixteen time constants, the mean over sixteen more:

**Arm (A), ADR-0019's constants** (`ARITHMETIC_052`), in units of the threshold, at or above the critical level in bold: the background (the prior alone, the members at their quiet rate), the quiet state (the assembly's input beside it) and the held state at 5, 10 and 20 Hz; and the contrast, the held state's at 20 Hz over the quiet state's.

| Shift | Weight | Background | Quiet | 5 Hz | 10 Hz | 20 Hz | Contrast |
| ---: | :--- | ---: | ---: | ---: | ---: | ---: | ---: |
| 0 | 0.125 | 0.56 | 0.88 | **1.36** | **1.76** | **2.08** | 2.4 |
| 0 | 0.25 | 0.56 | **1.19** | **2.16** | **2.94** | **3.57** | 3.0 |
| 0 | 0.375 | 0.56 | **1.51** | **2.95** | **4.11** | **5.06** | 3.4 |
| 0 | 0.5 | 0.56 | **1.82** | **3.74** | **5.29** | **6.55** | 3.6 |
| 1 | 0.125 | 0.31 | 0.48 | 0.73 | 0.90 | 1.09 | 2.3 |
| 1 | 0.25 | 0.31 | 0.63 | 1.12 | **1.50** | **1.83** | 2.9 |
| 1 | 0.375 | 0.31 | 0.78 | **1.50** | **2.10** | **2.55** | 3.2 |
| 1 | 0.5 | 0.31 | 0.94 | **1.90** | **2.67** | **3.29** | 3.5 |
| 2 | 0.125 | 0.03 | 0.26 | 0.38 | 0.50 | 0.60 | 2.3 |
| 2 | 0.25 | 0.03 | 0.37 | 0.61 | 0.76 | 0.92 | 2.5 |
| 2 | 0.375 | 0.03 | 0.40 | 0.76 | 1.11 | **1.28** | 3.2 |
| 2 | 0.5 | 0.03 | 0.50 | 0.99 | **1.37** | **1.65** | 3.3 |

**Arm (B), set (ii)** (`ARITHMETIC_052`), in units of the threshold, at or above the critical level in bold: the background (the prior alone, the members at their quiet rate), the quiet state (the assembly's input beside it) and the held state at 5, 10 and 20 Hz; and the contrast, the held state's at 20 Hz over the quiet state's.

| Shift | Weight | Background | Quiet | 5 Hz | 10 Hz | 20 Hz | Contrast |
| ---: | :--- | ---: | ---: | ---: | ---: | ---: | ---: |
| 0 | 0.125 | 0.56 | 0.79 | **1.59** | **2.98** | **4.83** | 6.1 |
| 0 | 0.25 | 0.56 | 1.01 | **2.59** | **5.35** | **9.00** | 8.9 |
| 0 | 0.375 | 0.56 | **1.23** | **3.60** | **7.72** | **13.17** | 10.7 |
| 0 | 0.5 | 0.56 | **1.45** | **4.60** | **10.08** | **17.35** | 11.9 |
| 1 | 0.125 | 0.30 | 0.41 | 0.84 | **1.52** | **2.45** | 6.0 |
| 1 | 0.25 | 0.30 | 0.52 | **1.33** | **2.71** | **4.53** | 8.7 |
| 1 | 0.375 | 0.30 | 0.64 | **1.83** | **3.89** | **6.62** | 10.3 |
| 1 | 0.5 | 0.30 | 0.76 | **2.33** | **5.07** | **8.70** | 11.5 |
| 2 | 0.125 | 0.03 | 0.25 | 0.48 | 0.77 | **1.25** | 5.0 |
| 2 | 0.25 | 0.03 | 0.27 | 0.72 | **1.38** | **2.27** | 8.5 |
| 2 | 0.375 | 0.03 | 0.37 | 0.97 | **1.99** | **3.36** | 9.1 |
| 2 | 0.5 | 0.03 | 0.39 | **1.21** | **2.57** | **4.38** | 11.1 |

**What the arithmetic says, and does not:**
- **The slow potential is a staircase** (F-57). The rule leaks it by the membrane's `leak`, $\lfloor s / 2^{13} \rfloor$ and at least one LSB a tick, so it settles where that leak meets the mean input a tick, at about $2^{13}$ LSB, 0.125 of the threshold, for each LSB of input a tick after the shift. Below one LSB a tick it cannot build at all: the background, 3.9 LSB a tick before the shift, holds 0.57, 0.31 and 0.03 of the threshold at shifts 0, 1 and 2, not a halving at each step. ADR-0122 did not say so; the arithmetic reads the rule itself, so the grid is placed by what the rule does.
- **The contrast follows the rate**, not the per-spike release: under arm (A) the held state at 20 Hz stands 2.3 to 3.6 times the quiet state's though a spike releases 0.38 of what it does quiet, and under arm (B) 5.0 to 11.9 times beside a per-spike ratio of 1.5. The prior's input, the same in both states, is what keeps arm (A)'s contrast near three.
- **At shift 0 the quiet state is near or past the critical level under arm (A)**: 0.88 of the threshold at an eighth of the range and 1.19 to 1.82 from a quarter up, past 1.138; under arm (B) 0.79 to 1.45, past it from three eighths up. The rule reads such a cell as bound to ignite, as ADR-0117's heaviest cells did. At shift 1 the held state at 20 Hz reaches the critical level from a quarter of the range up under arm (A) and at every weight under arm (B); at shift 2 from three eighths up under arm (A) and at every weight under arm (B).
- These are mean levels under regular trains. The members fire by the drive's fluctuations, so a quiet member's soma crosses $V_{lo}$ and opens the gate in the fluctuations' tail, and ignitions come by bursts, which no mean describes. The arithmetic places the grid; it makes no prediction of the verdict, and none is made.

### The input shifts, placed by a rule written before the arithmetic was read

**The middle shift is the largest at which, under arm (A) at ADR-0117's weight, a quarter of the range, the slow potential of the held state at 20 Hz is at or above the critical level; the three shifts are it and one either side, or 0, 1 and 2 if it is 0.** Arm (A) is the arm whose synapses depress, so the one where the slow potential's contrast is smallest; ADR-0117's weight is the grid's anchor; 20 Hz is the rate the four rounds read their held states against; and the largest such shift leaves the quiet state's slow potential furthest below the critical level while the held state still crosses it. One step either side is a factor of two, so the three shifts span a factor of four, as the four weights do.

The rule reads the arithmetic at arm (A) and a quarter of the range: at 20 Hz the held state stands at 233 793 at shift 0, **119 859 at shift 1** and 60 113 at shift 2, against the critical level of 74 589. The largest shift that reaches it is 1, and the three shifts are **0, 1 and 2** (`SHIFTS_052`, `SHIFT_PLACED_052`). A larger shift only shrinks the input, so none above 2 reaches it either. The gate holds the rule over the pinned table, and the core tests hold it again over the arithmetic computed on the settled image before their first run. The rule was written before the arithmetic was read, and the shifts were not moved after it.

### The gate (`a_slow_current_the_arms_the_grid_the_arithmetic_the_rules_and_a_marked_assembly_kicked_with_it`)

The one runtime test this measurement adds:
- the grid, the arms, the slow constants valid at every shift, $V_{lo}$ the oracle's standing;
- the release against both arms (Decision Outcome, the release);
- the arithmetic the settled image is not needed for, dumped, then held: the critical level, the gate's openings and the per-spike ratios;
- **the marks on the instrument's network** (`slow_marked_on_the_prior`): under each arm at each shift the image decodes with its slow current and class, the members marked and no other unit, the prior's blocks as they were; with the slow current unset in the record a member's mark is refused; under the drive, unwired, the members' slow potentials rise from the prior's synapses and every other unit's stays zero;
- the rules at their edges;
- **a marked assembly with the slow current kicked for a few hundred ticks**: arm (B) at the smallest shift and the heaviest weight, on the instrument's network at rest, kicked through the protocol's own kick (`kicked_on_keeping`, ADR-0117's gate function keeping its engine): every member fires once in the span, every reset is scheduled, no weight moves, every member's short-term state is the class's step over its spikes, and the volley charges every member's slow potential; the spikes and the slow potentials pinned (`GATE_KICKED_052`, `GATE_SLOW_052`);
- **over the pinned tables**, once pinned.

### The weekly tests and their cost

Six tests for the core grid, each `#[ignore]`d with `exhaustive` in its name, each building the settled image once:
- per arm, the backgrounds and controls at the three shifts, six runs, the arithmetic held before the first; under arm (A) ADR-0117's cell reproduced first, a seventh;
- per arm, the cells at the two lighter weights and at the two heavier, six runs each.

That is 37 runs of 400 epochs. At ADR-0120's rates on the hosted runners, about 80 to 130 s a run, the core adds about 3 700 to 4 800 s in six tests of about 500 to 900 s. The conditions' cost follows the core's reading: at most eight cells under each of three conditions, their backgrounds and controls under (c) and (d), and H-20's arm under (c). The cost table, regenerated from ADR-0120's dispatch, sums 40 263 s over 98 tests, about 6 710 s a shard at six shards (ADR-0121); the core's six tests add about 4 200 s to it. The conditions' dealing is written with their tests, and the round's whole cost is read from its dispatch and regenerated into the table.

### The order of the work

1. ADR-0123's build, its tests and the whole-image pins re-pinned, committed first.
2. This protocol, the rules, the arithmetic pinned, the shifts placed and the gate, committed before any measurement run and pushed with the pull request opened as a draft.
3. The core's backgrounds and controls, ADR-0117's cell reproduced first; the kick read on the engine under each arm and shift; pinned.
4. The core grid's cells against those pins; pinned.
5. The conditions' tests for the cells the rule picks, then their runs, each condition's backgrounds and controls before its cells; pinned.
6. The readings, the documents, the weekly dispatched on the round's branch at `scope=both`, its evidence and the cost table regenerated from it.

### The readings

Every reading below is from the pinned tables of `tests/assembly.rs`. On the branch the build is `2038c6f`, the edge its mutation gate found `e7eeb61`, and this protocol `d0cb546`, all before any run; the backgrounds and controls were pinned at `3428e1e`, and a second run of both tests there reproduced every table. On a developer machine in the release profile a run took about a minute with one other test beside it (a ratio, not admissible). **No weight of any arena moved in any measured run.**

- **ADR-0117's cell, reproduced** with nothing marked for the slow current, first in arm (A)'s test: held to ADR-0117's stretches, kicks and windows, and read against ADR-0117's background to ADR-0117's reading and bursts — holds in 8, ignites in 0, lets go in 8, usable.
- **The arithmetic held** in both core tests to `ARITHMETIC_052` before their first run, and the rule placed the same shifts.
- **The backgrounds** (the members marked and unwired, the drive alone after the lead-in), `BG_READ_052`; the slow potential and the gate's opening per member and tick over the unkicked spans, from `BG_RUNS_052`'s slow readings; beside them the distance to the threshold an upward excursion must cover under the gate, $0.564 \cdot (1 - s / s_c)$:

  | Arm | Shift | Members, Hz | Rest, Hz | Slow potential | Gate | Distance to the threshold |
  | :--- | ---: | ---: | ---: | ---: | ---: | ---: |
  | ADR-0117, no slow current | — | 1.61 | 1.68 | — | — | 0.564 |
  | (A) | 0 | 31.20 | 1.84 | 0.562 | 0.162 | 0.286 |
  | (A) | 1 | 11.12 | 1.82 | 0.294 | 0.142 | 0.418 |
  | (A) | 2 | 4.07 | 1.75 | 0.114 | 0.125 | 0.508 |
  | (B) | 0 | 53.64 | 2.68 | 0.809 | 0.182 | 0.163 |
  | (B) | 1 | 15.66 | 2.28 | 0.367 | 0.149 | 0.382 |
  | (B) | 2 | 4.32 | 1.82 | 0.122 | 0.126 | 0.504 |

  **The slow current does not keep the quiet state quiet.** Fed by the prior's synapses alone, with no assembly wired, it raises the members' rate 2.5 to 33 times. The mean slow potential stays below the critical level everywhere — 0.11 to 0.81 against 1.138 — as the arithmetic said, and the soma's mean hardly moves, but the members fire in the drive's tail. There the gate is open, and an open gate adds $\gamma(v) \cdot s / 2$ to the soma's steady level, so it multiplies every upward excursion by $1 / (1 - s / s_c)$: the distance a fluctuation must cover to the threshold falls from 0.564 to 0.51 at the smallest slow potential and to 0.16 at the largest, and the rate climbs as that distance shrinks. The gate is open by 0.13 to 0.18 on average at the drive's standing, because $V_{lo}$ sits at the soma's mean and every upward excursion crosses it. Under (B) the members' faster firing through the prior's 28 member-to-member synapses, facilitating under set (ii), and the rest's rise to 2.3 to 2.7 Hz add to the slow potential: 0.81 against the arithmetic's 0.56 at shift 0. At shift 2 the slow potential reads 3.4 to 4.8 times the arithmetic's level, because the prior's messages arrive at random and a cluster lifts the slow potential off the staircase's lowest step, which the arithmetic's regular train cannot (F-57).
- **The kick, read on the engine** from each control's eight kicks, by ADR-0112's measure (`KICK_AFTER_052`, held by the gate over the pinned tables):

  | Arm | Shift | Volley, of 512 | After, the control | After, the background's | Mark |
  | :--- | ---: | ---: | ---: | ---: | ---: |
  | (A) | 0 | 509 | 327 | 327 | 51 |
  | (A) | 1 | 510 | 126 | 116 | 51 |
  | (A) | 2 | 512 | 62 | 42 | 51 |
  | (B) | 0 | 501 | 678 | 562 | 51 |
  | (B) | 1 | 509 | 241 | 164 | 51 |
  | (B) | 2 | 512 | 67 | 45 | 51 |

  The volley fires every member once at every arm and shift, within the measure's tolerance. The second clause — the members' spikes in the pair window after the span, at most a tenth of a spike a member a kick — **fails at every one**. At shifts 0 and 1 the members' own background alone gives that window more than the mark, so no kick could pass there. At shift 2 the background gives it 42 and 45, under the mark, and the control's members fire 62 and 67: the kick's volley, echoed through the prior onto members whose gate is open, leaves about 0.04 of a spike a member a kick beyond their background, where ADR-0117's kick left fewer spikes than its background (10 against about 17). Each control's lead-in and first unkicked span are its background's bit for bit, the slow potential with them; no control holds, ignites or fails to let go by the rules, and under (B) at shift 0 the unwired members' high rate makes 756 of the control's 1 024 unkicked windows bursts by ADR-0112's reading (`bursts_049`). The release's cancel shuts the gate, to 0.002 on average over the release spans, while the slow potential, fed by the prior, falls only to 0.48 and 0.49 at shift 0 against 0.56 and 0.81 unkicked.
- **No cell ran.** By the protocol the kick must fire every member once by ADR-0112's measure, or be derived again, before any cell of its arm runs. No kick can meet the second clause where the members' background alone passes it, and moving the measure after these runs is outside what brief 052 empowers. The maintainers chose on 2026-09-28 to stop the round here and write the backgrounds as its reading, rather than amend the measure before the cells or run them against a failed kick. So the core grid, the rule that picks the conditions' cells, and the conditions (b), (c) and (d) did not run; the rules are held at their edges by the gate, as written first. The four weekly tests of the core grid's cells were removed before the dispatch; the two of the backgrounds and controls stay.
- **The arithmetic beside the readings.** The mean levels of the slow potential were near what the runs read under (A) at shifts 0 and 1 (0.562 and 0.294 against 0.565 and 0.307), and the critical level held: no background's mean slow potential reached it. What the arithmetic of means could not say is what the gate does to a fluctuation. It placed the grid on the held state's mean crossing the critical level and the quiet state's mean staying below it, and the quiet state's rate is set by its tail, where the gate is open.

**The next decision, named and not taken.** By brief 052's branch for a round with no robust cell, what the readings name:
- **the gate's voltages**: $V_{lo}$ above the band the drive's fluctuations reach, so that the gate stays shut on the quiet state's excursions and opens only on the sustained depolarisation of a held state; the backgrounds read the gate open by 0.13 to 0.18 on average at the drive's standing;
- **the input's shift beyond the grid**: the staircase (F-57) floors a mean input below one LSB a tick to almost nothing, so at a shift of 3 the same arithmetic, stepped there before the shifts were placed and not pinned, puts the background's slow potential under a hundredth of the threshold, while arm (B)'s held state at 20 Hz at a quarter of the range just reaches the critical level and arm (A)'s reaches it at no weight of the grid; the runs read shift 2's background 3.4 to 4.8 times the regular train's level, so a larger shift's would sit above its arithmetic too;
- **the slow time constant**, which the readings do not single out;
- **or the line paused**, after five rounds.

An ADR choosing among them has as its need these readings: an unwired member's rate rises with $s / s_c$ as its distance to the threshold shrinks, 2.5 times at a tenth of the critical level and 33 times at seven tenths, and a kick leaves a small excess beyond the background even where the background passes the kick's measure. ADR-0111's second round is not taken.

### The evidence

The weekly is dispatched on this round's branch at `scope=both`, since `src/` changes ([ADR-0075](0075-the-dispatch-scope-follows-the-diff.md)); its run, the shards' times and the cost table regenerated from it are recorded here once it is green.

## Consequences

- Good: the slow current is measured by ADR-0117's substrate, protocol and measures, beside ADR-0117's cell reproduced bit for bit, and the reading is the mechanism's own: what the gate does to the quiet state's fluctuations, with a number for it.
- Good: nothing of the engine changes in the measurement; every earlier run wires and reads as it did, the slow reading of the protocol empty for them.
- Bad: no cell ran, so whether the slow current holds a kicked assembly, and whether it widens the facilitating class's one cell, is not read. The grid, its rule for the conditions and the conditions stay written and unrun.
- Bad: the arithmetic of means placed the shifts where the quiet state's mean stays below the critical level; the quiet state's rate is set by its fluctuations, which the arithmetic did not read.
- Neutral: the kick's measure (ADR-0074, ADR-0112) presumes a background of a spike or two a second; under the slow current its second clause measures the members' background as much as the kick.

## Alternatives considered and why rejected

- **The kick's second clause read net of the members' background**, amended before the cells: it would have let the cells of (A) at every shift and of (B) at shift 2 run, and still stopped (B) at shifts 0 and 1, where the kick leaves 116 and 77 spikes beyond the background against a mark of 51. It moves a threshold after a run, which brief 052 does not empower; the maintainers chose to stop instead.
- **The cells run against a failed kick**: every cell would be read against a background of 4 to 54 Hz, where holding asks for five times it, and the kick's failure would stand under every reading.
- **A kick derived again to suppress the members after the volley**: a kick that holds the members below their background for a pair window would pass the clause by hiding the very activity a hold is made of.

## Confirmation

`runtime/cortex-runtime/tests/assembly.rs`:
- the arms, the constants and the grid: `SIZE_052`, `ARMS_052`, `LEAK_SHIFT_052`, `V_LO_052`, `V_HI_052`, `WEIGHTS_052`, `SHIFTS_052`, `CONDITION_CELLS_MAX`, `current_052`, `class_052`;
- the marks: `slow_marked`, `slow_marks_held`, `engine_052`;
- the rules: `Grid052`, `Cell052`, `tie_key`, `neighbours_052`, `usable_052`, `condition_cells_052`, `robust_052`, `round_two_052`, `failures_052`;
- the arithmetic: `slow_level`, `assembly_train`, `prior_onto_members`, `train_of`, `slow_levels`, `fires_held`, `slow_critical`, `gate_openings`, `per_spike_052`, `arithmetic_052`, `shift_placed`, and their pins `SLOW_CRITICAL_052`, `GATE_052`, `PER_SPIKE_052`, `SHIFT_PLACED_052`, `ARITHMETIC_052`;
- the runs: the slow reading of `span_protocol_counting`, `slow_read`, `Pinned052`, `held_052`, `slow_held`, `run_052`, `cell_read_052`, `backgrounds_controls_052`, `backgrounds_held_052`, `reproduce_050_in_the_core`, `core_backgrounds_052` and the two weekly tests;
- the gate `a_slow_current_the_arms_the_grid_the_arithmetic_the_rules_and_a_marked_assembly_kicked_with_it`, with `slow_marked_on_the_prior`, `rules_052_at_their_edges`, `kicked_on_keeping`, `GATE_KICKED_052`, `GATE_SLOW_052`, `background_after` and `over_the_052_tables`;
- the runs' tables `BG_RUNS_052`, `BG_READ_052` and `KICK_AFTER_052`.

Whitepaper §11 carries F-57, and §11.1's question on a rule held by the network the reading.
