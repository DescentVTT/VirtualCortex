---
status: proposed
date: 2026-09-29
depends-on: ADR-0125
decision-makers: VirtualCortex maintainers
---

# ADR-0126: The gate raised, measured — brief 053's measurement of ADR-0123's slow current with its lower gate voltage at 0.7, 0.8 and 0.9 of the threshold, on ADR-0124's substrate, arms, shifts, protocol, measures and kick, every weight frozen; the quiet soma's distribution read first on both arms' substrates with nothing of the slow current, ADR-0117's background among them bit for bit; an arithmetic of the slow rule itself, written before any run, that reads what the gate does to a quiet soma as an effective threshold, the larger of the gate's lower voltage and $1 - s \cdot 128/257$, so that at shift 2 none of the three voltages moves it; the eighteen backgrounds and controls, the core grid only where the kick passes, and ADR-0125's rule that pauses the line applied as written; the quiet soma stands above 0.7, 0.8 and 0.9 of the threshold on 6.0, 1.8 and 0.41 per cent of its ticks, so no voltage lies above the band its fluctuations reach; the gate raised lowers every background, to 2.9 to 5.4 Hz at 0.9, and the kick passes at shift 2 at 0.8 and 0.9 under both arms; there every one of the sixteen cells fails, fifteen igniting without a kick and none letting go, because a wired assembly's own slow potential makes the voltage itself its members' threshold and the drive carries the soma across it; ADR-0125's case 2, and the line pauses, the facilitating class and the slow current staying in the engine unset bit for bit and the next decision named and not taken

## Context and Problem Statement

[ADR-0125](0125-the-gate-raised.md) took the slow current's gate voltages as the last try of the rule held by the network, and wrote before any run the rule that pauses the line. Brief 053 measures it. This ADR is the measurement's protocol, committed before any run, and then its readings.

What was read before anything was written (principle 2):

1. **The slow current** (`crates/cortex-core/src/dynamics/membrane.rs`, [ADR-0123](0123-the-slow-current-built.md)). `SlowCurrent::gate_q16` is $\operatorname{clamp}((v - V_{lo}) / (V_{hi} - V_{lo}), 0, 1)$, read on the soma as the tick found it, and `integrate_slow` adds $\gamma(v) \cdot s / 16$ to the soma. The slow potential's own update reads no gate. So raising $V_{lo}$ changes what the slow potential gives the soma and nothing of the slow potential itself: [ADR-0124](0124-the-slow-current-measured.md)'s levels of the slow potential (`ARITHMETIC_052`) are this round's at every voltage.
2. **ADR-0124's substrate** (`runtime/cortex-runtime/tests/assembly.rs`): `ARMS_052`, `SHIFTS_052`, `WEIGHTS_052`, `SIZE_052`, `current_052`, `slow_marked`, `engine_052`, `run_052`, `backgrounds_controls_052`, `cell_read_052`, `slow_read`, `Pinned052`, its pinned tables, and ADR-0117's rules as it applied them. Its $V_{lo}$ is a constant, `V_LO_052`, the drive's mean standing as the oracle reads it, 28 561 (0.436).
3. **ADR-0117's background**, which brief 053 names for the distribution's run, marks its 64 members facilitating under set (ii): `run_050` builds every run of brief 050, background or not, through `marked_engine_drawn` with `SET_050`. The brief's parenthesis calls it *"64 members marked for nothing"*. That is F-58, below.
4. **The kick's measure** (ADR-0112, `fires_every_member_once`): a volley of at least 496 of 512 spikes over eight kicks, and at most 51 of the members' spikes in the pair window after the span. ADR-0124's backgrounds alone gave that window 42 to 562.
5. **The protocol reads no soma.** `span_protocol_counting` gains a reading of the records that no input of a run sees (`span_protocol_reading`, `SomaRead`): over every tick after the lead-in, how many of the members' somatic potentials, as each tick left them, were above each of 0.5, 0.6, 0.7, 0.8, 0.9 and 0.95 of the threshold's base, their sum and their count. The three middle levels are the three voltages, so a soma above one is a soma the gate at that voltage is open on. Without the reading, every earlier run runs as it ran.

## Decision Drivers

- Brief 053's standing directives:
  - no rule of the engine changes, $V_{lo}$ being a parameter of the image;
  - every weight frozen in every measured run, and each run's weights at its end shown equal to its start;
  - ADR-0124's substrate, arms, slow constants but $V_{lo}$, protocol, spans, release, stretch, measures, thresholds and kick unchanged, and the kick not re-derived to pass a background that fails it;
  - ADR-0117's cell reproduced bit for bit before any other run;
  - ADR-0125's rule applied as written and not amended;
  - the grid, the voltages, the rule that picks the cells and the arithmetic written before any run and moved by none, the grid widened only before;
  - no float, every loop ended by construction; the runtime's gate grows by at most one test; no shard of the weekly job past 60 per cent of its bound.

## Considered Options

1. **The distribution's run**: (a) ADR-0117's background as it is, marked facilitating under set (ii); (b) the members marked for nothing, as the brief's parenthesis says; (c) both.
2. **The distribution's form**: a histogram, or the fractions above each of the brief's six levels.
3. **The critical level at a raised voltage**: ADR-0124's definition, from an eighth of the gap above the drive's mean standing; or from an eighth of the gate's span above $V_{lo}$.
4. **The grid**: the three voltages; or widened before any run, by a voltage between two of them or above 0.9.
5. **The weekly tests**: how the runs are dealt.

## Decision Outcome

**Options 1(c), 2 the fractions, 3 from the gate's span, 4 the three voltages, and 5 as below.** Everything below is committed before any measurement run, with the arithmetic and the gate.

### The voltages and the grid

- **$V_{lo}$ at 0.7, 0.8 and 0.9 of the threshold's base**, floored in Q16.16: 45 875, 52 428 and 58 982 (`V_LOS_053`), with $V_{hi}$ at the base. Every other slow constant is ADR-0124's (`current_053`). At ADR-0124's voltage, `current_053` is `current_052` bit for bit, which the gate holds.
- **The arms, the assembly, the shifts and the weights are ADR-0124's**: (A) under ADR-0019's short-term plasticity and (B) under set (ii) as well; ADR-0117's 64 members; shifts 0, 1 and 2; weights 0.125, 0.25, 0.375 and 0.5 of the range.
- **Eighteen pairs**, one per arm, voltage and shift (`Pair053`), each as ADR-0124's:
  - the background marked and unwired, not grown, the drive alone;
  - the control grown, marked and unwired, with the kick and the release;
  - ADR-0124's protocol, layout and readings, and the soma's reading beside them (`run_053`).
- **The grid is not widened** (option 4). By the arithmetic below, a voltage between two of the three sits between their effective thresholds, and one above 0.9 moves shift 2's effective threshold, 0.94, by less than a hundredth; neither buys a reading the three do not bracket.

### The quiet soma's distribution (options 1(c) and 2)

On the settled image, frozen, the 64 members unwired and not grown, the drive alone over ADR-0124's layout, with nothing of the slow current (`quiet_run_053`):
- **under (A)'s substrate**: the members marked for nothing, so their synapses release under ADR-0019's constants;
- **under (B)'s**: marked facilitating under set (ii), which is ADR-0117's background itself, held to its pin (`CORE_RUNS_050`) bit for bit and to its reading (`CORE_READ_050`).

Each reads the fraction of the members' ticks after the lead-in with the soma above 0.5, 0.6, 0.7, 0.8, 0.9 and 0.95 of the threshold's base, the soma's mean, and the spikes. It is a reading, with no clause, taken before any run with the slow current. The six levels are the brief's; a histogram would add bins the voltages do not need.

### The arithmetic, before any run

It is computed with the slow rule itself: in the gate, and on the settled image in the first weekly test for the slow potential's levels. Pinned: `SLOW_CRITICAL_053`, `GATE_053`, `LEVELS_053` and `EFFECTIVE_053`.

**The critical slow potential at each voltage** (`slow_critical_at`, `fires_held`): the least slow potential that, held, fires a unit under the drive's mean input from an eighth of the gate's span above $V_{lo}$ (option 3).
- At ADR-0124's voltage the span is the gap above the drive's standing, so this is ADR-0124's definition. The gate reproduces ADR-0124's 74 589 from it, searched over a range four times wider.
- At the three voltages it is **317 851, 410 185 and 502 261: 4.85, 6.26 and 7.66 of the threshold**. The algebra, $8\,(\tfrac{257}{128} v_0 - b)$ at the starting soma $v_0$ and the drive's basal standing $b = 0.875$, gives 4.85, 6.25 and 7.66.
- ADR-0124's definition, from the drive's mean standing, has no value at a raised voltage: the gate is shut there, and no slow potential gives the soma anything.

**The gate's opening** (`gate_openings_053`), at the drive's mean standing, at $V_{lo}$, a quarter, a half and three quarters of the span above it, and at the base: **0, 0, 0.25, 0.50, 0.75 and 1.0** at each voltage, to the floors of `GATE_053`. At the drive's mean standing the gate is shut at every voltage. ADR-0124's was shut exactly there and open on every excursion above.

**The effective threshold** (`effective_threshold`): what a quiet soma must reach to fire, with the slow potential held at a level.
- The unit's basal and slow potentials are held. The soma rises from rest toward its steady level without overshoot, and it fires when that level, the slow current's share included, reaches the threshold.
- The effective threshold is the soma's standing, without the slow current, under the least basal potential at which the unit fires; it is found by bisection. With no slow potential it is the threshold's base (65 535, the standing's floor one LSB below).
- It is read at ADR-0124's six backgrounds' mean slow potentials over their unkicked spans (`LEVELS_053`, from `BG_RUNS_052`). In units of the threshold:

| ADR-0124's background | Slow potential | At 0.436 (ADR-0124) | At 0.7 | At 0.8 | At 0.9 |
| :--- | ---: | ---: | ---: | ---: | ---: |
| none | 0 | 1.000 | 1.000 | 1.000 | 1.000 |
| (A) at shift 0 | 0.562 | 0.720 | 0.720 | 0.800 | 0.900 |
| (A) at shift 1 | 0.294 | 0.854 | 0.854 | 0.854 | 0.900 |
| (A) at shift 2 | 0.114 | 0.943 | 0.943 | 0.943 | 0.943 |
| (B) at shift 0 | 0.809 | 0.597 | 0.700 | 0.800 | 0.900 |
| (B) at shift 1 | 0.367 | 0.817 | 0.817 | 0.817 | 0.900 |
| (B) at shift 2 | 0.122 | 0.939 | 0.939 | 0.939 | 0.939 |

**What the rule does to a quiet soma: the effective threshold is the larger of $V_{lo}$ and $1 - s \cdot 128/257$.**
- Below $V_{lo}$ the gate gives nothing.
- Above it, a soma whose standing without the current is $u$ settles at $V_{lo} + (u - V_{lo}) / (1 - k)$, with $k = s \cdot 128 / (257\,(1 - V_{lo}))$. That reaches the threshold exactly when $u \ge 1 - s \cdot 128/257$, whatever $V_{lo}$ is. Once $k \ge 1$ the soma runs away from any standing above $V_{lo}$.
- At ADR-0124's voltage this is its distance to the threshold, $0.564\,(1 - s/s_c)$.

So **raising the gate moves the threshold a quiet fluctuation must reach only where $V_{lo}$ is above $1 - 0.498\,s$**. That is, only where the slow potential is above $(1 - V_{lo}) \cdot 257/128$: 0.60, 0.40 and 0.20 of the threshold at the three voltages. There it moves it to $V_{lo}$ exactly.
- At shift 2, where ADR-0124's backgrounds held 0.11 and 0.12, none of the three voltages moves it.
- At shift 1, only 0.9 moves it.
- At shift 0, 0.8 and 0.9 move it, and 0.7 only under (B).
- At 0.9, every quiet and every held state of ADR-0124's table (below) is above 0.20, and so is every background but shift 2's. For all of these the effective threshold is 0.9 itself: whether a member fires is whether its soma crosses 0.9. The quiet soma's distribution reads how often that happens without the current.

**The slow potential's levels** do not read the gate. ADR-0124's table (`ARITHMETIC_052`) is this round's at every voltage:
- the background, the quiet state and the held state at 5, 10 and 20 Hz, at each arm, shift and weight;
- their contrast, 2.3 to 3.6 under (A) and 5.0 to 11.9 under (B).

The first weekly test holds `slow_levels_under` to it at each of the three voltages, on the settled image, before any run.

**Predicted readings, written before any run.** They are readings, with no clause:
1. Where a voltage leaves ADR-0124's effective threshold unmoved, the background's members fire within a quarter of ADR-0124's rate at that arm and shift. That is eleven pairs:
   - (A) at 0.7 at every shift, at 0.8 at shifts 1 and 2, and at 0.9 at shift 2;
   - (B) at 0.7 and 0.8 at shifts 1 and 2, and at 0.9 at shift 2.
2. At each arm and shift, the backgrounds' members fire no faster at a higher voltage.
3. Every background's members fire faster than their arm's quiet substrate without the current, since no effective threshold of the table reaches the threshold's base.

The arithmetic makes no prediction of the kick. At shift 2 the background alone gave the pair window 42 and 45 of the mark's 51 at ADR-0124's voltage. What the kick adds beyond the background — 20 and 22 spikes there — is not an arithmetic of means.

### The rules, before any run

- **The kick passes** (`passes_053`) by ADR-0112's measure over the control's kicks.
- **The pairs whose cells run** (`cell_pairs_053`), as ADR-0125 writes them:
  - every pair whose kick passes;
  - if more than four pass (`CELL_PAIRS_MAX`), the four whose backgrounds' members fire slowest, ties to the higher $V_{lo}$, then the larger shift, then arm (A);
  - in the grid's order.
- **The cells**: each picked pair's four weights, wired as ADR-0124's with ADR-0112's delays, each read against its pair's own background by ADR-0117's rules — holds, ignites, lets go, spills and usable — under the core condition only.
- **The cells the next round's conditions would run** (`condition_cells_053`), as ADR-0124's rule picks them:
  - every usable cell;
  - if more than eight, the eight with the most usable neighbours. A cell's neighbours are within its arm and voltage, one weight step or one shift step away, among the cells that ran;
  - ties to the lighter weight, then the smaller shift (ADR-0124's order), then the higher voltage, then arm (A).
- **ADR-0125's rule applied** (`case_053`, `Case053`):
  - no pair picked is case 1, and the line pauses;
  - cells that ran with none usable is case 2, and the line pauses;
  - usable cells are case 3: the next round runs ADR-0117's conditions (b), (c) and (d) on the cells `condition_cells_053` names;
  - case 4, a robust cell, needs that round.

### F-58

Brief 053 names the distribution's run as ADR-0117's background and describes it as *"the settled image, 64 members marked for nothing, the drive alone"*. ADR-0117's background marks its members facilitating under set (ii). The round reads both (option 1(c)): ADR-0117's background as it is, held to its pin bit for bit, and the members marked for nothing, arm (A)'s substrate. So each arm has its own quiet soma without the current, and the brief's reference is honoured as the tree has it. Resolved here; the brief is frozen when archived and is not edited.

### The gate (`the_gate_raised_the_voltages_the_arithmetic_the_rules_and_a_marked_assembly_kicked_at_the_highest`)

The one runtime test this measurement adds:
- the voltages as floored fractions of the base, above ADR-0124's, valid at every shift, and among the soma's levels;
- ADR-0124's definitions at its own voltage, bit for bit: `current_053`, and the critical level searched over the wider range;
- the arithmetic, dumped, then held to its pins;
- the effective threshold with no slow potential the base at every voltage, and with one never below the voltage and never above the base;
- the soma's reading at its edges: a soma at a level is not above it, one LSB more is;
- the marks at each voltage and arm on the instrument's network, the prior's blocks as they were;
- the rules at their edges, over tables written by hand:
  - the kick's measure at 51 and 52 spikes after the span and at volleys of 496 and 495;
  - no pass, case 1; three passes, every one; six, the four slowest;
  - the cut falling on each tie in turn, and a slower background outranking the ties;
  - case 2 and case 3; the neighbours within an arm and a voltage; twelve usable cells cut to eight; the tie key;
- **a marked assembly with the gate at 0.9, kicked for a few hundred ticks**: ADR-0124's gate cell — (B), shift 0, a half of the range — on the instrument's network at rest, through the protocol's own kick (`kicked_on_keeping`). Every member fires once in the span, every reset is scheduled, no weight moves, and the volley charges every member's slow potential. The volley is ADR-0124's gate's spike for spike. What follows differs: at that weight the assembly fires again from tick 267 where at ADR-0124's voltage it did from 258, 251 spikes in the span against 256 (`GATE_KICKED_053`, `GATE_SLOW_053`);
- over the pinned tables, once pinned.

### The weekly tests and their cost

Seven tests, each `#[ignore]`d with `exhaustive` in its name, each building the settled image once:
- `the_gate_raised_the_quiet_soma_and_adr_0117s_cell_exhaustive`: the slow potential's levels held at each voltage, ADR-0117's cell reproduced first, then the two distribution runs, three runs in all;
- six tests of the backgrounds and controls, one per arm and voltage, three shifts, six runs each.

That is 39 runs of 400 epochs. At ADR-0124's rates on the hosted runners, about 110 to 120 s a run with the image built, they are tests of about 400 and 720 s, about 4 700 s together.

If any pair passes, the cells follow in one test per picked pair, its four weights, about 470 s each and at most four, so at most about 1 900 s. They are written once the pairs are pinned; the rule that picks their pairs is this section's, committed first.

The cost table regenerated from ADR-0124's dispatch sums 42 699 s over 100 tests. With the round's tests at these rates it would be about 47 400 s without cells and 49 300 s with sixteen. At ADR-0124's ratio of summed seconds to wall time, 1.90 to 1.99, a shard would run 55 to 58 and 57 to 60 per cent of its bound. The round's whole cost is read from its dispatch and regenerated into the table.

### The order of the work

1. This protocol, the soma's reading, the arithmetic pinned and the gate, committed before any run, and pushed with the pull request opened as a draft.
2. The first weekly test: the slow potential's levels held, ADR-0117's cell reproduced, the quiet soma's runs; pinned.
3. The eighteen pairs, each with its kick read on the engine; pinned; the rule picks the pairs.
4. The cells' tests for the picked pairs, if any, written then; their runs; pinned.
5. The readings, ADR-0125's case, the documents, the weekly dispatched on the round's branch at `scope=exhaustive` ([ADR-0075](0075-the-dispatch-scope-follows-the-diff.md): only tests change), and the cost table regenerated from it.

### The readings

Every reading below is from the pinned tables of `tests/assembly.rs`. The protocol was committed at `68d5e71` before any run; the runs were pinned at `2640f5c`, and a second run of every weekly test there reproduced every table. On a developer machine in the release profile a pair's test took about 690 s with six others beside it, and a cell's test about 390 s with three (a ratio, not admissible). **No weight of any arena moved in any measured run.**

- **ADR-0117's cell, reproduced** first, with nothing marked for the slow current: held to ADR-0117's stretches, kicks and windows, and read against ADR-0117's background to its reading and bursts — holds in 8, ignites in 0, lets go in 8, usable.
- **The slow potential's levels** held to ADR-0124's table at each of the three voltages on the settled image, as the rule's text said they would be.
- **The quiet soma** (`QUIET_READ_053`), the members' ticks after the lead-in, 402 653 184 of them under each arm's substrate, with nothing of the slow current:

  | Substrate | Above 0.5 | Above 0.6 | Above 0.7 | Above 0.8 | Above 0.9 | Above 0.95 | Mean soma | Members, Hz |
  | :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
  | (A): marked for nothing | 32.9 % | 15.5 % | 5.97 % | 1.84 % | 0.407 % | 0.138 % | 0.436 | 1.61 |
  | (B): ADR-0117's background, bit for bit | 32.9 % | 15.5 % | 5.97 % | 1.84 % | 0.406 % | 0.138 % | 0.436 | 1.61 |

  The two substrates differ in the fourth significant figure. **None of the three voltages lies above the band the quiet soma's excursions reach**: the soma stands above 0.7 on one tick in seventeen, above 0.8 on one in fifty-four and above 0.9 on one in 246. The mean is the drive's standing, 0.436, as the oracle reads it.
- **The backgrounds** (`BG_READ_053`), the members marked and unwired, the drive alone after the lead-in; the members' rate beside ADR-0124's at its voltage:

  | Arm | Shift | 0.436 (ADR-0124) | 0.7 | 0.8 | 0.9 |
  | :--- | ---: | ---: | ---: | ---: | ---: |
  | (A) | 0 | 31.20 | 18.15 | 11.06 | 5.28 |
  | (A) | 1 | 11.12 | 8.90 | 6.85 | 4.21 |
  | (A) | 2 | 4.07 | 3.83 | 3.50 | 2.85 |
  | (B) | 0 | 53.64 | 24.64 | 12.89 | 5.44 |
  | (B) | 1 | 15.66 | 11.06 | 7.70 | 4.31 |
  | (B) | 2 | 4.32 | 3.97 | 3.59 | 2.87 |

  Over the unkicked spans, per member and tick:
  - **the slow potential is ADR-0124's**, from 0.110 to 0.751 of the threshold: 0.557, 0.291 and 0.113 under (A) at 0.7 against ADR-0124's 0.562, 0.294 and 0.114; under (B) it falls with the members' rate, 0.751 at shift 0 against 0.809;
  - **the gate's mean opening** falls from ADR-0124's 0.13 to 0.18 to 0.017 to 0.020 at 0.7, 0.006 to 0.008 at 0.8 and 0.002 to 0.003 at 0.9;
  - **the soma stands above the voltage**, where the gate is open, on 3.6 to 5.6 per cent of the ticks at 0.7, 1.1 to 1.6 at 0.8 and 0.30 to 0.33 at 0.9 — close to the quiet soma's 5.97, 1.84 and 0.41;
  - the rest of the network fires at 1.73 to 2.52 Hz, against 1.75 to 2.68 at ADR-0124's voltage.
- **The predicted readings, against the run.**
  1. **Failed as written.** Of the eleven pairs whose effective threshold the voltage leaves unmoved, five fire within a quarter of ADR-0124's rate — (A) at 0.7 at shifts 1 and 2 (0.80 and 0.94 of it) and at 0.8 at shift 2 (0.86), (B) at 0.7 and 0.8 at shift 2 (0.92 and 0.83) — and six fire slower still: (A) at 0.7 at shift 0 (0.58), at 0.8 at shift 1 (0.62) and at 0.9 at shift 2 (0.70); (B) at 0.7 and 0.8 at shift 1 (0.71 and 0.49) and at 0.9 at shift 2 (0.66). None fires faster. The split follows $k = s \cdot 128/(257\,(1 - V_{lo}))$ at the measured slow potential: the five within a quarter have $k$ at most 0.48, the six below it at least 0.55. The arithmetic's effective threshold is a steady level; the closer $k$ is to one, the more slowly a soma above $V_{lo}$ approaches it — as $8/(1 - k)$ ticks for the coupling's sixteenths — and the drive's excursions do not last long enough for it to be reached. That account is the rule's algebra, not a separate measurement.
  2. **Held**: at each arm and shift the rate falls at every step of the voltage, ADR-0124's included.
  3. **Held**: every background fires faster than its substrate's 1.61 Hz; the slowest, 2.85 and 2.87 Hz, at shift 2 at 0.9.
- **The kick, read on the engine** from each control's eight kicks by ADR-0112's measure (`KICK_AFTER_053`): the volley is full at every pair (at least 508 of 512). The members' spikes in the pair window after the span, the control's and what the background alone gives it, against the mark of 51:

  | Arm | Shift | 0.7 | 0.8 | 0.9 |
  | :--- | ---: | ---: | ---: | ---: |
  | (A) | 0 | 241 (190) | 204 (115) | 160 (55) |
  | (A) | 1 | 108 (93) | 91 (71) | 62 (44) |
  | (A) | 2 | 56 (40) | **48 (36)** | **39 (29)** |
  | (B) | 0 | 430 (258) | 324 (135) | 182 (57) |
  | (B) | 1 | 186 (115) | 124 (80) | 72 (45) |
  | (B) | 2 | 57 (41) | **47 (37)** | **38 (30)** |

  **The kick passes at four pairs, in bold: shift 2 at 0.8 and 0.9 under both arms.** What the kick leaves beyond the background falls with the voltage at shift 2: 20 and 22 spikes at ADR-0124's, 16 at 0.7, 12 and 10 at 0.8, 10 and 8 at 0.9. At shifts 0 and 1 the background alone passes the mark at every voltage but 0.9 at shift 1, where the kick's excess of 18 and 27 still fails it. Each control's lead-in and first unkicked span are its background's bit for bit, the slow potential with them.
- **The pairs whose cells run** (`CELL_PAIRS_053`): four pass, so all four, and no tie is cut.
- **The core grid** (`CELLS_053`), sixteen cells, each against its pair's background; the members' rate over the unkicked spans and the slow potential there per member and tick:

  | Pair | Weight | Holds, of 8 | Ignites, of 8 | Lets go, of 8 | Unkicked, Hz | Background, Hz | Slow potential |
  | :--- | :--- | ---: | ---: | ---: | ---: | ---: | ---: |
  | (A) at 0.8 | 0.125 | 0 | 4 | 0 | 15.6 | 3.50 | 0.50 |
  | (A) at 0.8 | 0.25 | 8 | 8 | 0 | 33.2 | 3.50 | 0.88 |
  | (A) at 0.8 | 0.375 | 8 | 8 | 0 | 48.7 | 3.50 | 1.26 |
  | (A) at 0.8 | 0.5 | 8 | 8 | 0 | 64.2 | 3.50 | 1.65 |
  | (A) at 0.9 | 0.125 | 0 | 0 | 0 | 6.6 | 2.85 | 0.40 |
  | (A) at 0.9 | 0.25 | 0 | 8 | 0 | 15.3 | 2.85 | 0.76 |
  | (A) at 0.9 | 0.375 | 8 | 8 | 0 | 28.4 | 2.85 | 1.18 |
  | (A) at 0.9 | 0.5 | 8 | 8 | 0 | 38.8 | 2.85 | 1.57 |
  | (B) at 0.8 | 0.125 | 8 | 8 | 0 | 52.4 | 3.59 | 1.39 |
  | (B) at 0.8 | 0.25 | 8 | 8 | 0 | 109.3 | 3.59 | 2.91 |
  | (B) at 0.8 | 0.375 | 8 | 8 | 0 | 169.1 | 3.59 | 4.53 |
  | (B) at 0.8 | 0.5 | 8 | 8 | 0 | 230.6 | 3.59 | 6.26 |
  | (B) at 0.9 | 0.125 | 4 | 8 | 1 | 17.3 | 2.87 | 0.87 |
  | (B) at 0.9 | 0.25 | 8 | 8 | 0 | 58.0 | 2.87 | 2.48 |
  | (B) at 0.9 | 0.375 | 8 | 8 | 0 | 89.5 | 2.87 | 3.90 |
  | (B) at 0.9 | 0.5 | 8 | 8 | 0 | 155.8 | 2.87 | 5.82 |

  **No cell is usable.** Fifteen ignite without a kick, in four to eight rounds of eight, and none lets go: by `failed`, four never hold, fifteen run away and sixteen do not let go. Nothing spills.

  **The wired assembly has one state, and the kick does not move it.** In every cell the hold spans' second halves fire within three per cent of the unkicked spans, but one: (B) at 0.9 at an eighth of the range, 18.6 Hz against 17.3, the only cell whose tail after the release drops toward the background, to 6.8 Hz, before its unkicked spans fire at 17.3 again. The recurrent synapses feed each member's slow potential from its fellows' background firing, and the slow potential over the unkicked spans stands at 0.40 to 6.26 of the threshold. That is above $(1 - V_{lo}) \cdot 257/128$ — 0.40 at 0.8 and 0.20 at 0.9 — in every cell. So by the arithmetic, and as its protocol said of every quiet state at 0.9 before the run, **the voltage itself is every member's threshold**. The drive alone lifts the soma above it on 0.3 to 1.6 per cent of the ticks. Each member that crosses fires, feeds its fellows' slow potentials, and the assembly sits at its own rate, 2.3 to 64 times its background, with nothing for a kick to switch on or a release to switch off. At an eighth of the range under (A) at 0.9 that rate, 6.6 Hz, is between twice and five times the background: the rules read it as neither held nor let go.

**ADR-0125's rule applied** (`case_053`, held by the gate over the pinned tables): cells ran and none is usable in the core. That is **case 2, and the line pauses.** As ADR-0125 writes the pause:
- **what was built stays, unset bit for bit**: the facilitating class (ADR-0114) and the slow current (ADR-0123) are in the engine, and with no unit marked every run is the run it was. The weekly dispatched for this round reproduced every earlier pinned number (below);
- **the readings of the six rounds are its record**: no assembly held (ADR-0112); one cell under the facilitating class (ADR-0115); that region one weight wide and not robust (ADR-0117); an inhibition that moved the threshold and not the gap (ADR-0120); a slow current whose gate let the quiet state's fluctuations through (ADR-0124); and here, the gate raised above the soma's mean and still inside the band its fluctuations reach, so that a wired assembly's quiet state crosses it as readily as its held state does;
- **the next decision returns to H-20's four open questions** — the operating regime, another size, the inhibitory drain, a critic of the engine's own — **or ADR-0111's option 2, the rule held in the symbolic layer.** It is named and not taken.

What the six rounds say together, as a reading and not a clause: on this network, at this size, with the drive this round's quiet soma reads, every mechanism tried that lets a wired assembly sustain its firing also lifts the same assembly's quiet state, because the quiet state's own recurrent input feeds the same mechanism, and the drive's fluctuations reach any voltage the held state could be told apart by.

### The evidence

Only tests change, so by [ADR-0075](0075-the-dispatch-scope-follows-the-diff.md) the weekly was dispatched on this round's branch at **`scope=exhaustive`**: run [36460574405](https://github.com/DescentVTT/VirtualCortex/actions/runs/36460574405) at `2640f5c`, the runs pinned, after which only the documents, the cost table and the brief's archive change. **Every job it ran is green.**
- **The whole-domain tests.** All 111 passed on the hosted runners, each in a process of its own reporting one test passed. The 100 before this round reproduced their pinned numbers with no unit marked: the whole-image pins, and H-17's to H-20's arms, among them. This round's eleven reproduced their tables: the quiet soma in 469 s, the backgrounds and controls in 708 to 784 s, the cells in 531 to 554 s, 6 992 s together against the protocol's estimate of about 6 600. The six shards took:

  | Shard | Job | Tests | Their seconds summed | Tests' wall time | The shard's heaviest test (s) |
  | ---: | ---: | ---: | ---: | :--- | :--- |
  | 0 | 63 m 52 s | 18 | 7 276 | 3 772 s, 52 % | ADR-0120's condition (c) cells 2 816 |
  | 1 | 67 m 00 s | 17 | 7 917 | 3 960 s, 55 % | ADR-0120's condition (c) backgrounds 2 348 |
  | 2 | 84 m 26 s | 17 | 9 439 | 5 008 s, 70 % | ADR-0117's condition (c) 2 500 |
  | 3 | 71 m 14 s | 19 | 8 312 | 4 213 s, 59 % | H-20 from the mirrored 1 690 |
  | 4 | 81 m 45 s | 20 | 9 643 | 4 849 s, 67 % | H-20 from the assignment 1 794 |
  | 5 | 76 m 00 s | 20 | 8 834 | 4 499 s, 62 % | plasticity everywhere 1 146 |

  Three shards passed the brief's 60 per cent in this run. It dealt by the table from ADR-0124's dispatch, which costed this round's eleven tests at 900 s each where they took 6 992 s together, and the older tests ran 4.1 per cent over their table, 44 429 s against 42 699.
- **The cost table is regenerated from this run's artifacts** (`scripts/exhaustive-costs.tsv`: 111 lines, 51 421 s, its source line naming the run; `npm run spec:costs` passing). Replayed through the deal at six shards, it plans every shard at 8 569 to 8 571 s summed. At this run's ratio of summed seconds to wall time, 1.88 to 2.00 and 1.96 overall, that is 4 285 to 4 559 s of tests' wall time: **59.5 to 63.3 per cent of the bound, about 61 at the run's overall ratio — past brief 053's 60 per cent.** The protocol's own estimate, 57 to 60 with sixteen cells, did not count the older tests' drift. The round is not empowered to change the shard count or the scripts, so **this standing directive is not met**, and it is recorded rather than met by dropping pinned runs. The levers are the known two, named for a decision of its own and not taken: a seventh shard (the lever of ADR-0118 and ADR-0121), or fewer copies of H-20's arm, which the condition (c) tests of ADR-0117 and ADR-0120 each rebuild, 2 348 to 2 816 s apiece. The longest test, ADR-0120's condition (c) cells, is 39 per cent of the bound alone.
- **The pull request's gate** on `68d5e71` and on `c2338b5` is green in every job: check, test, fmt and clippy; the AArch64 determinism pin, unmoved; the MSRV job; the documentation gate; and the mutation gate on the changed lines, which has no mutant to make, since only tests and documents change. Locally `cargo mutants --in-diff` reports no mutant to filter.

On the developer machine every command of brief 053's verification list exited 0 (a ratio, not admissible):
- the workspace in the debug profile, in the release profile and on the MSRV toolchain in a target directory of its own: 665 passed in each and 111 ignored. The MSRV test's first attempt failed to link one test binary (LNK1104, a file held open on this platform) and passed on the retry;
- check, fmt, clippy, doc, the bench `--test`, `npm ci` and `npm run spec`;
- `--list`, 111 tests;
- every weekly test of this round twice, the second run at `2640f5c` against the pins.

## Consequences

- Good: the last try is a parameter and a measurement, read by ADR-0124's protocol and measures beside ADR-0117's cell reproduced bit for bit, and it ends the line by a rule written before the run.
- Good: the arithmetic written first said where the gate could matter and why a wired quiet state would cross it at 0.9, and it did; its prediction for the backgrounds' rates failed in six pairs of eleven, in a way the rule's own algebra accounts for.
- Good: the kick passes where ADR-0124's could not, so the cells were read, and the reading is about the assembly and not the measure.
- Bad: the line paused leaves ADR-0111's need open: H-20's reversals still take 13 to 23 blocks with no representation of the rule in force.
- Bad: the weekly job's regenerated deal plans every shard at about 61 per cent of its bound, past the briefs' 60, with the levers named and not taken.
- Neutral: the facilitating class and the slow current stay in the engine, unset bit for bit, for a later need.

## Alternatives considered and why rejected

- **Option 1(a) or 1(b) alone**: one arm would have no quiet soma of its own, and 1(b) alone would leave ADR-0117's background, the one the brief names, unread.
- **A histogram**: bins the voltages do not need; the six fractions place the three voltages against the band, which is what ADR-0125 asks.
- **ADR-0124's critical level from the drive's standing**: it has no value where the gate is shut at the standing.
- **A voltage between two of the three, or above 0.9**: the arithmetic brackets both, as said.
- **Reusing ADR-0124's runs**: none is this round's bit for bit, since every run carries a different voltage.

## Confirmation

`runtime/cortex-runtime/tests/assembly.rs`:
- the voltages, the grid and the arithmetic: `V_LOS_053`, `current_053`, `SOMA_LEVELS_053`, `slow_critical_at`, `least_firing`, `fires_held`, `gate_openings_053`, `fires_standing`, `soma_standing`, `effective_threshold`, `background_levels_052`, `effective_053`, `slow_levels_under`, and their pins `SLOW_CRITICAL_053`, `GATE_053`, `LEVELS_053` and `EFFECTIVE_053`;
- the soma's reading: `SomaRead`, `soma_add`, `span_protocol_reading`;
- the rules: `Pair053`, `Read053`, `CELL_PAIRS_MAX`, `passes_053`, `cell_pairs_053`, `neighbours_053`, `usable_053`, `tie_key_053`, `condition_cells_053`, `Case053`, `case_053`;
- the runs: `engine_under`, `slow_marked_under`, `run_053`, `quiet_run_053`, `quiet_soma_053`, `backgrounds_controls_053`, `pairs_053`, `cells_053`, `Pinned053`, `held_053`, and the eleven weekly tests — the quiet soma, six of the backgrounds and controls, and four of the cells, one per picked pair;
- the gate `the_gate_raised_the_voltages_the_arithmetic_the_rules_and_a_marked_assembly_kicked_at_the_highest`, with `rules_053_at_their_edges`, `hand_pair`, `GATE_KICKED_053`, `GATE_SLOW_053` and `over_the_053_tables`;
- the runs' tables `QUIET_053_A`, `QUIET_READ_053`, `BG_RUNS_053`, `BG_READ_053`, `KICK_AFTER_053`, `CELL_PAIRS_053`, `CELL_RUNS_053`, `CELLS_053`, `CELL_BURSTS_053` and `CELL_HELD_053`.

Whitepaper §11 carries F-58, and §11.1's question on a rule held by the network the reading and the line paused.
