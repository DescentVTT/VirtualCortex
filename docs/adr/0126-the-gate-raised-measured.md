---
status: proposed
date: 2026-09-29
depends-on: ADR-0125
decision-makers: VirtualCortex maintainers
---

# ADR-0126: The gate raised, measured — brief 053's measurement of ADR-0123's slow current with its lower gate voltage at 0.7, 0.8 and 0.9 of the threshold, on ADR-0124's substrate, arms, shifts, protocol, measures and kick, every weight frozen; the quiet soma's distribution read first on both arms' substrates with nothing of the slow current, ADR-0117's background among them bit for bit; an arithmetic of the slow rule itself, written before any run, that reads what the gate does to a quiet soma as an effective threshold, the larger of the gate's lower voltage and $1 - s \cdot 128/257$, so that at shift 2 none of the three voltages moves it; the eighteen backgrounds and controls, the core grid only where the kick passes, and ADR-0125's rule that pauses the line applied as written

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

## Consequences

- Good: the last try is a parameter and a measurement, read by ADR-0124's protocol and measures beside ADR-0117's cell reproduced bit for bit.
- Good: the arithmetic says before the run where the gate raised can matter and where it cannot, and the predicted readings make that checkable.
- Bad: at shift 2, the gentlest background, the arithmetic says the gate raised moves nothing; if the kick fails there as it did at ADR-0124's voltage, no voltage of the three can rescue it.
- Neutral: whatever the case, the facilitating class and the slow current stay in the engine, unset bit for bit.

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
- the runs: `engine_under`, `slow_marked_under`, `run_053`, `quiet_run_053`, `quiet_soma_053`, `backgrounds_controls_053`, `pairs_053`, `Pinned053`, `held_053`, and the seven weekly tests;
- the gate `the_gate_raised_the_voltages_the_arithmetic_the_rules_and_a_marked_assembly_kicked_at_the_highest`, with `rules_053_at_their_edges`, `hand_pair`, `GATE_KICKED_053` and `GATE_SLOW_053`.
