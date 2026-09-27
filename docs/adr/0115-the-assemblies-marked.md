---
status: accepted
date: 2026-09-27
depends-on: ADR-0114
decision-makers: VirtualCortex maintainers
---

# ADR-0115: The assemblies marked, measured — brief 049's measurement with ADR-0114's class: ADR-0112's assemblies of 16, 32 and 64 units at six recurrent weights under each of ADR-0113's two sets, every member marked and every weight frozen, run through a lead-in and eight rounds of an unkicked span, a hold span with ADR-0112's kick, a release derived from the class and a tail, by rules written before any run; the arithmetic with the class's own step agreeing with ADR-0113's tables but for one sentence of its prose (F-55); one cell usable — 64 units under set (ii) at a quarter of the range: kicked, it holds as a train of bursts refreshed inside the class's primed window; unkicked, it stays quiet; released, it lets go — while every other cell never holds or runs away; the next decision, a readout gated by the context, named and not taken

## Context and Problem Statement

[ADR-0112](0112-an-assembly-that-holds.md) read no assembly usable on ADR-0077's settled network: a kick set off one population burst, the burst drained the members' pool, and the strong cells burst again of their own. [ADR-0113](0113-a-facilitating-class-of-synapses.md) chose a class of synapses whose facilitation outlasts their depression, and [ADR-0114](0114-the-facilitating-class-built.md) built it: a unit marked `FLAG_FACILITATING` steps its short-term state under the image's class. Brief 049 asks the question ADR-0112 asked, with the members marked, and over spans long enough to tell a hold the network refreshes from a kick's priming fading.

What was read before anything was written (principle 2):

1. **ADR-0112's substrate is reusable as it is** (`runtime/cortex-runtime/tests/assembly.rs`): `members`, `wire`, `grown`, the kick (`inject_before`, `note_kick_spikes`, `KICK_MESSAGE_Q16`, `KICK_RESET_Q16`), the cancel's constants, `settled`, `delivered`, `rise` and `least_together`. Its `protocol` reads epochs of one kind each and does not.
2. **The short-term readings stepped `step_stp` unconditionally.** `stp_sums`, `steady` and `stp_course` stepped every member under ADR-0019's constants. Brief 049 requires every oracle and reading that steps a member's short-term state to step a marked member under the class. They now do so through one function, `stepped_pair`, which selects as the executor's turn does. On an unmarked unit it is `step_stp`, so ADR-0112's pinned tables stand, and its gate passes unchanged.
3. **A mark changes a member's output, not its input.** Unwired, a marked member receives what an unmarked one does. Its synapses onto the prior's targets release under the class, which changes the rest of the network slightly. So a background is read per size with that size's members marked, not once for all sizes.
4. **The kick is the membrane's, and the membrane is untouched.** ADR-0112's kick fires every member once whatever its short-term state; a mark changes only what the kick's spikes then release.
5. **The class's step agrees with ADR-0113's replica** (the gate, below): the steady pairs pair for pair, and the course after a burst to ADR-0113's hundredths. One sentence of ADR-0113's prose does not hold as written, finding **F-55**. It gives the primed window as *"from about 7 400 to 106 000 ticks"* for set (i) and *"from about 4 100 to 117 500"* for set (ii), with the product *"above the unkicked one"*. By the class's step the product is above the unkicked one only to 101 429 and 113 388 ticks. From there it equals the unkicked product exactly — the pair is the unkicked steady pair, $(113, 255)$ and $(63, 255)$ — until 106 358 and 117 742, and then falls below. ADR-0113's two ends are where the product stops being at or above, not above. The explanation is that plateau. It changes no decision: the release below is derived from the fade from the ceiling of both factors, which bounds both readings.

## Decision Drivers

- Brief 049's standing directives:
  - every weight frozen: the excitatory baseline zero, the inhibitory baseline and the signed gate unset, no reward, every weight at each run's end shown equal to its value at the start;
  - the grid, the spans, the measures, their thresholds and the release written before any run, and moved by none;
  - the arithmetic computed first, with the class's own step;
  - no float, oracles included; every loop ends by construction;
  - the runtime's gate grows by at most one test; the runs are weekly `exhaustive` tests, dealt so that no shard passes 60 per cent of its bound.
- ADR-0113's arithmetic: a kick's priming fades below the unkicked product by $2^{17}$ ticks, so a hold read after that is refreshed, and a release must keep the members silent about that long.
- One change at a time: ADR-0112's placement, wiring, delays, kick, drive and measures' thresholds, with the class and the spans the only differences.

## Considered Options

1. **The spans' lengths**: the brief's defaults (sixteen epochs unkicked, sixteen for the hold) or shorter ones for the weekly job's time.
2. **The lead-in**: ADR-0112's one epoch, or long enough for the image's short-term state, written under ADR-0019's constants, to relax to the class's own.
3. **The release**: (a) ADR-0076's cancel repeated, at the start of every window but the last two of a span whose length is derived from the class; (b) a volley into nearby inhibitory units; (c) ADR-0112's single cancel.
4. **The release's length**: from the state ADR-0113's one burst leaves; or from the ceiling of both factors, which bounds every state a hold can leave.
5. **The background**: once per set for all sizes, or per size with that size's members marked.
6. **The grid**: the brief's six weights and three sizes, or widened.
7. **The tables**: every window in full as ADR-0112's, or every stretch in full and every window by a hash.
8. **Where the tests live**: `tests/assembly.rs` beside ADR-0112's, or a binary of their own with the substrate moved into a shared module.

## Decision Outcome

**Options 1 (the brief's spans), 2 (a long lead-in), 3(a), 4 (from the ceiling), 5 (per size), 6 (the brief's grid, not widened), 7 (stretches in full, windows by hash) and 8 (`tests/assembly.rs`).** Everything below is committed before any run of the measurement, with the arithmetic the gate holds.

### The grid

- **Sizes**: 16, 32 and 64 units, at ADR-0112's placement (places 5 and 16 of each period of twenty, the sizes nested), wiring (each member to the next $\min(\text{size} - 1, 32)$ members, basal) and delays (the prior's local band, drawn by `mix64` of seed 48).
- **Weights** (`WEIGHTS_049`): 0x2000, 0x3000, 0x4000, 0x5000, 0x6000 and `i16::MAX` in Q1.15: 0.25, 0.375, 0.5, 0.625, 0.75 and the top of the range. ADR-0112's four are among them.
- **Sets** (`SETS`): ADR-0113's (i) $(U, \tau_f, \tau_d) = (51/256, 2^{16}, 2^{13})$ and (ii) $(26/256, 2^{16}, 2^{13})$.
- **Marks** (`marked`): every member of the size's assembly marked `FLAG_FACILITATING` in its record and the set's class written in the modulator record, each section re-sealed. These are the test's own marks, as `grown` and `wire` are its own wiring, and the loader takes them. No other unit is marked.
- **Not widened.** Over the six weights, the members' spikes that must land together to fire a target at the primed peak run from five down to two under set (i) and from eight down to two under set (ii) (the arithmetic below), so the grid spans the range the priming acts over. A size of 128 gives a member no more synapses than 64 does (ADR-0112).

### The protocol

Each run starts from ADR-0077's frozen settled image, decoded at its written tick, under ADR-0044's drive with the gain 1.75 carried in the image and the controller off. Epochs are $2^{14}$ ticks, read in windows of 2 048.

- **A lead-in of sixteen epochs** (`LEAD_IN_EPOCHS_049`), read by no rule: $2^{18}$ ticks, four of the class's $\tau_f$. Over it, the short-term state the image's members carry from ADR-0019's constants relaxes to within two per cent of the class's own.
- **Eight rounds** (`layout`), each of four spans:
  - **an unkicked span** of sixteen epochs: the drive alone;
  - **a hold span** of sixteen epochs, with ADR-0112's kick from its first tick: the ramp of 1/64 on the first 160 ticks and each member's reset of −23 062 on the tick after its kick spike's refractory window;
  - **a release span** of $2S$ epochs (below), with ADR-0076's cancel — six messages at −2.0 into every member before each of nine ticks — at the start of every window but the last two (`cancel_due`);
  - **a tail** of four epochs: the drive alone.
- **The stretch**: two consecutive epochs of a span, from its first. Every span is a whole number of stretches.
- **The runs**: a run is 384 epochs under set (i) and 400 under set (ii), 15.4 and 16 times ADR-0112's 25.
  - **the background**, per set and size (`class_run` with `background`): the frozen image with the members marked and unwired, over the same ticks with no kick and no release;
  - **the control**, per set and size: the image grown by the assembly's blocks, the members marked and unwired, the protocol;
  - **the cell**, per set, size and weight: the grown image, the members marked and wired at the weight, the protocol.

  Every run is dumped before any is held to its table. Every weight of the arena at each run's end is asserted equal to its value at the start.

### The release, derived from the class before any run

A kick's priming is the members' release fraction held high while their pool recovers. The release must keep the members from firing until no member can be primed. Two facts of the rule make that a bound, which the gate holds:
- the pair after a silence is non-decreasing in each factor it starts from;
- from the ceiling of both factors, $(255, 255)$, the pool stays full while $u$ only relaxes, so the product falls monotonically with the silence.

So:

- **The fade** (`fade`): the least silence after which a member at the ceiling releases with a product below the unkicked one, the product of the steady pair at the background's 56 818 ticks. By the class's step it is **135 785 ticks** under set (i) and **180 192** under set (ii).
  - The gate holds the bound: at the fade, no pair of the lattice of `testkit/prop.rs` releases at or above the unkicked product.
  - One tick earlier, the ceiling still does.
- **The release's span** (`release_rule`): the least whole number of stretches $S$ whose guaranteed silence $(16S - 4) \cdot 2\,048 + 3\,510$ ticks is at least the fade. That silence runs from the span's second window, whose cancel catches a member the first found refractory, to `RELEASE_RECOVERED` (3 510) after the last cancel, at the start of the span's third window from the end. The result is **five stretches (ten epochs) under set (i) and six (twelve) under set (ii)**. One stretch fewer leaves each fade uncovered.
- **The release by the oracle** (`release_span_oracle`: `integrate` alone under the drive's mean input and the cancels, from the drive's mean standing and from ADR-0076's extreme):
  - no spike in any tick of the span;
  - the basal potential no lower than −190.1;
  - the soma back within a tenth of the threshold of the drive's mean standing 3 519 ticks after the last cancel, before the tail opens 4 096 ticks after it.
- **The release on the engine**: in the gate, one member at rest with no drive keeps the oracle's basal potential after every tick of two windows of cancels.

### The measures (thresholds ADR-0112's)

The background of a run (`background_049`) is the members' and the rest's spikes over every stretch after the lead-in. Every comparison is in integers, as ADR-0112's.

- **A held stretch**: the members' spikes over it at least five times the background (`HOLD_TIMES`).
- **Holds**: in at least 7 of 8 rounds (`OF_EIGHT_MIN`), every stretch of the hold span's second half (its fifth to eighth stretch, epochs nine to sixteen after the kick) is held. By ADR-0113's arithmetic, confirmed below, a kick's priming has faded there unless a burst refreshed it.
- **Ignites**: a round whose unkicked span holds any stretch. A usable cell ignites in at most 1 of 8 (`IGNITIONS_MAX`).
- **Lets go**: in at least 7 of 8 rounds, every stretch of the tail is at most twice the background (`LET_GO_TIMES`).
- **Spills**: over the held stretches of the hold spans' second halves, the rest of the network's spikes are more than twice the rest's background (`SPILL_TIMES`). If none is held, spills is not read.
- **Usable** (`Holding::usable`): holds, lets go, ignites in at most one round, and does not spill.
- **What failed** (`failed`), where a cell is not usable: never holding, running away (ignites in more than one round, or spills), not letting go.
- **The kick** fires every member once by ADR-0112's measure (`kicked_once_049` through `fires_every_member_once`, which ADR-0112's `kicked_once` now calls), read on each control's eight kicks. If it does not, no cell is run and the kick is derived again.

**What the thresholds read by chance**, for a Poisson background at 1.75 Hz a member, ADR-0112's settled rate: a quiet assembly's stretch expects 9.2, 18.4 and 36.7 spikes at 16, 32 and 64 units.
- It reads *held* by chance with a probability below $10^{-15}$ per stretch.
- It fails *quiet* in one stretch with a probability of 0.30 per cent at 16 units, 0.01 at 32 and below $10^{-5}$ at 64. A tail of two stretches fails with 0.59, 0.02 and $10^{-5}$ per cent.
- So a quiet cell of sixteen fails *lets go* by chance with a probability of about $10^{-3}$, and the larger ones with less.

### Readings, no clause

- **The first half** (`Holding::first_half`): the rounds whose hold span's first half is held in every stretch, where the kick's priming alone could hold.
- **The bursts** (`bursts_049`, ADR-0112's burst window: the members firing at least once each in a window):
  - the burst windows in each kind of span;
  - the intervals between consecutive burst windows within a hold span, in windows, counted in bins of 1, 2, 3–4, 5–8, 9–16, 17–32, 33–64 and 65–128.
- **The short-term state**: the members' sums of $u$ and $R$ at each window's end, pinned in every window's hash and in full at each stretch's end.
- **The product** a member's next spike would release with, per member per window and per mille of the unkicked product:
  - over the held stretches of the hold spans' second halves;
  - over the unkicked spans.

### The tables

- **Per stretch, in full** (`StretchRow`): the members' spikes, the rest's, the members' burst windows, $\sum u$ and $\sum R$ at the stretch's end, and the sum over its windows of $\sum uR$; 192 rows a run under set (i), 200 under set (ii).
- **Per window, by a hash** (`windows_hash`, FNV-1a over every window's five numbers). A run has 3 072 or 3 200 windows, and 48 runs in full would be about 150 000 rows; the hash pins every one of them, and each run's windows are in its dump.
- **Per round**, each hold span's kick: the members' spikes in its first `KICK_SPAN` ticks and in the pair window after.
- The readings above, per run.

### The arithmetic, before any run, with the class's step

Every number below is computed by the gate test with `step_stp_class` (through `stepped_pair`) and pinned there. Each agrees with ADR-0113's replica where ADR-0113 gives it, with F-55 as the one sentence that does not.

**The steady pairs** (`CLASS_STEADY_049`), at ADR-0112's eight intervals, pair for pair with ADR-0113's table where it has the rate:

| Rate | (i) pair | $uR$ | (ii) pair | $uR$ |
| :--- | :--- | ---: | :--- | ---: |
| at rest | (92, 255) | 23 460 | (49, 255) | 12 495 |
| 1.76 Hz: the unkicked product | (113, 255) | 28 815 | (63, 255) | 16 065 |
| 5 Hz | (149, 242) | 36 058 | (93, 247) | 22 971 |
| 10 Hz | (182, 197) | 35 854 | (128, 211) | 27 008 |
| 20 Hz | (208, 130) | 27 040 | (163, 146) | 23 798 |
| 40 Hz | (226, 74) | 16 724 | (199, 81) | 16 119 |
| 100 Hz | (242, 31) | 7 502 | (225, 33) | 7 425 |
| 200 Hz | (248, 15) | 3 720 | (234, 16) | 3 744 |
| 400 Hz | (250, 8) | 2 000 | (243, 8) | 1 944 |

**The course after ADR-0113's burst** (`BURST_COURSE_049`: four spikes 220 ticks apart, the first one background interval after the unkicked steady state's last spike), per mille of the unkicked product, each within five per mille of ADR-0113's hundredths:

| $t$ (ticks) | 220 | 2 048 | 8 192 | 16 384 | 32 768 | 65 536 | 131 072 | 262 144 |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| (i) | 108 | 428 | 1 053 | 1 332 | 1 350 | 1 150 | 929 | 823 |
| (ii) | 430 | 745 | 1 325 | 1 554 | 1 515 | 1 238 | 952 | 809 |

**The primed window** (`PRIMED_049`):
- under set (i), above the unkicked product from 7 389 to 101 429 ticks after the burst, and equal to it to 106 358; the peak is 1.383 times it at 24 319 ticks, the pair $(164, 243)$;
- under set (ii), above from 4 046 to 113 388, equal to 117 742; the peak is 1.601 times at 19 242 ticks, $(109, 236)$.

**What one spike delivers** (`DELIVERED_049`), after the gain, at each weight. The first three columns are at the unkicked pair and the last three at the primed peak's: the delivery, the soma's peak it raises a unit at the drive's mean standing (0.436 of the threshold) to, and the least number of such spikes landing together that fires the unit. The deliveries are in units of the threshold.

| Set | Weight | Unkicked: delivered | Peak | Together | Primed: delivered | Peak | Together |
| :--- | :--- | ---: | ---: | ---: | ---: | ---: | ---: |
| (i) | 0.25 | 0.192 | 0.526 | 7 | 0.266 | 0.560 | 5 |
| (i) | 0.375 | 0.289 | 0.571 | 5 | 0.399 | 0.623 | 4 |
| (i) | 0.5 | 0.385 | 0.616 | 4 | 0.532 | 0.685 | 3 |
| (i) | 0.625 | 0.481 | 0.661 | 3 | 0.665 | 0.747 | 2 |
| (i) | 0.75 | 0.577 | 0.706 | 3 | 0.798 | 0.810 | 2 |
| (i) | 1.0 | 0.769 | 0.796 | 2 | 1.064 | 0.934 | 2 |
| (ii) | 0.25 | 0.107 | 0.486 | 12 | 0.172 | 0.516 | 8 |
| (ii) | 0.375 | 0.161 | 0.511 | 8 | 0.258 | 0.556 | 5 |
| (ii) | 0.5 | 0.214 | 0.536 | 6 | 0.343 | 0.597 | 4 |
| (ii) | 0.625 | 0.268 | 0.561 | 5 | 0.429 | 0.637 | 3 |
| (ii) | 0.75 | 0.322 | 0.587 | 4 | 0.515 | 0.677 | 3 |
| (ii) | 1.0 | 0.429 | 0.637 | 3 | 0.687 | 0.758 | 2 |

**No single spike fires a unit at the drive's mean standing in any cell, primed or not.** The priming lowers the number of members' spikes that must land together by one or two at most weights. It lowers it by none at the top weight of set (i), where two suffice unprimed, and by three and four at the two lightest weights of set (ii). ADR-0112's reading was that a hold, if one exists, is the fluctuations riding above a mean below threshold; that still holds. At the background's 1.76 Hz a spike delivers 1.31 times what it delivers under ADR-0019's constants under set (i), and 0.73 times under set (ii) (ADR-0113's table).

**The release** as above: fades of 135 785 and 180 192 ticks, spans of five and six stretches, and the oracle's release silent through the span and recovered before the tail.

### The gate (`the_facilitating_class_the_arithmetic_the_spans_the_rules_and_a_marked_assembly_on_the_engine`)

The one runtime test this measurement adds:
- **The grid and the arithmetic**: dumped, then held to the pinned constants above and to ADR-0113's tables.
- **The spans**: their order, their lengths and whole stretches; the cancel's ticks at the edges; the interval bins.
- **The rules at their edges** over stretches written by hand: holds at five times exactly and one LSB below in one round and then two; lets go at twice exactly; a round ignites once however many stretches; spills over the second halves' held stretches only; nothing held, no spill read; what failed in each; the stretch rows from windows; and the kick's measure over eight kicks.
- **The marks on the instrument's network at rest**, marked by `marked` at each set and size:
  - the loader takes the marked image of each set and size, grown, the prior's blocks unchanged;
  - a mark with the class removed is refused at the first member, unit 5;
  - unwired under the drive for 16 384 ticks, every unit that fired steps as `stepped_pair` says: the members under the class, and every other unit under `step_stp`;
  - the release on the engine as above.
- **One marked assembly on the engine**: sixteen members at the top weight under set (i), wired on the instrument's network at rest and kicked through the protocol's own kick for 800 ticks. Every member fires once in the span, sixteen resets are scheduled, and no weight moves. Every member's short-term state after its spikes is the class's step over them. The members' spikes are pinned (`GATE_KICKED_049`).
- **Over the pinned tables**, once pinned: each run's stretches read again by the rules, each control's kick read again, and each control's lead-in and first unkicked span shown to be its background's.

### The order of the work

1. ADR-0114's build (`8e3ec71` and `f62b955` on `main`; `abb29be` and `5b38a5d` on the branch before the rebase that merged pull request #143).
2. This protocol, the rules, the oracles, the arithmetic pinned and the gate, committed before any run of the measurement (`c19edff` on `main`, `8f192d6` on the branch, pushed with pull request #143 opened as a draft before any run).
3. `the_kick_the_backgrounds_and_the_controls_under_set_i_at_1024_units_exhaustive` and `…_set_ii_…`: each set's three backgrounds and three controls. The kick is read on the engine with the members marked, and must fire every member once at every size under each set before any cell is run (pinned at `78dfe0f` on `main`, `fcb3830` on the branch).
4. The six cell tests (`a_marked_assembly_of_{16,32,64}_units_at_six_weights_under_set_{i,ii}_exhaustive`), against the backgrounds pinned in step 3 (pinned at `d193265` on `main`, `cf6e485` on the branch).
5. The readings below and the documents at 4.65.0 (`240fd35` on `main`, `339babd` on the branch), then the weekly dispatched on that commit, and its evidence, the cost table and the brief's archive.

### The readings

Every reading below is from the pinned tables of `tests/assembly.rs`, and a second run of every test reproduced them. The runs are in the order above, at the commits `main` holds after the rebase that merged pull request #143, the branch's in brackets:
- the kick, the backgrounds and the controls at `c19edff` (`8f192d6`), read against their pins at `78dfe0f` (`fcb3830`);
- the six cell tests at `78dfe0f` (`fcb3830`) side by side, read against their pins at `d193265` (`cf6e485`).

On a developer machine in the release profile each kick-and-background test took 337 to 344 s run two at a time, and each cell test 470 to 595 s run six at a time (a ratio, not admissible). No weight of any arena moved in any run.

- **The backgrounds** (`BACKGROUNDS_049`: each size's members marked and unwired, the drive alone over the protocol's ticks after the lead-in):
  - the members fired at 1.655, 1.659 and 1.634 Hz a member at 16, 32 and 64 units under set (i), and 1.623, 1.639 and 1.613 Hz under set (ii);
  - the rest fired at 1.68 to 1.76 Hz a unit.

  ADR-0112's unmarked members read 1.72 to 1.75 Hz over a run sixteen times shorter.
- **The kick, read on the engine with the members marked** (`KICKS_049`, each control's eight kicks). Every member fires once by ADR-0112's measure at every size under both sets:
  - the volley is 128, 256 and 512, every kick a full volley;
  - the after is 7, 18 and 31 under set (i) and 0, 5 and 10 under set (ii), against marks of 12.8, 25.6 and 51.2.

  Each control's lead-in and first unkicked span are its background's bit for bit. The kick was not derived again.
- **The controls** (`CONTROLS_049`: grown, marked, unwired, the kick and the release). At every size under both sets no stretch of a hold span is held, no unkicked span ignites, and every tail lets go. The only burst windows are the eight kicks'. The kick alone holds nothing.
- **The grid** (`GRID_049`, by the rules committed first, against the backgrounds pinned before any cell ran):

  | Set | Size | Weight | First half, of 8 | Holds, of 8 | Ignites, of 8 | Lets go, of 8 | Spills | Usable | What failed |
  | :--- | ---: | :--- | ---: | ---: | ---: | ---: | :--- | :--- | :--- |
  | (i) | 16 | 0.25 | 0 | 0 | 2 | 8 | no | no | never holding; running away |
  | (i) | 16 | 0.375 | 6 | 6 | 8 | 2 | no | no | never holding; running away; not letting go |
  | (i) | 16 | 0.5 to 1.0 | 8 | 8 | 8 | 0 | no | no | running away; not letting go |
  | (i) | 32 | every weight | 8 | 8 | 8 | 0 | no | no | running away; not letting go |
  | (i) | 64 | every weight | 8 | 8 | 8 | 0 | no | no | running away; not letting go |
  | (ii) | 16 | 0.25 | 0 | 0 | 0 | 8 | not read | no | never holding |
  | (ii) | 16 | 0.375 | 1 | 0 | 0 | 8 | no | no | never holding |
  | (ii) | 16 | 0.5 | 8 | 7 | 8 | 8 | no | no | running away |
  | (ii) | 16 | 0.625 | 8 | 8 | 8 | 3 | no | no | running away; not letting go |
  | (ii) | 16 | 0.75, 1.0 | 8 | 8 | 8 | 0 | no | no | running away; not letting go |
  | (ii) | 32 | 0.25 | 6 | 5 | 2 | 7 | no | no | never holding; running away |
  | (ii) | 32 | 0.375 | 8 | 8 | 8 | 2 | no | no | running away; not letting go |
  | (ii) | 32 | 0.5 to 1.0 | 8 | 8 | 8 | 0 | no | no | running away; not letting go |
  | **(ii)** | **64** | **0.25** | 6 | **8** | **0** | **8** | **no** | **yes** | — |
  | (ii) | 64 | 0.375 | 8 | 8 | 8 | 2 | no | no | running away; not letting go |
  | (ii) | 64 | 0.5 to 1.0 | 8 | 8 | 8 | 0 | no | no | running away; not letting go |

  **One cell is usable: 64 units under set (ii) at a quarter of the range.** Over the thirty-six cells, 5 never hold, 33 run away and 30 do not let go; nothing spills anywhere.
- **The usable cell, read** (its stretches and `CELL_BURSTS_049`):
  - **It holds by refreshing.** In every round the four stretches of the hold span's second half read 6.8 to 19.7 times the background. The hold spans hold 140 burst windows between them, and none falls in an unkicked span, a release or a tail. The intervals between consecutive burst windows are:
    - 26 of one window (a burst across a window's edge);
    - 3 of three to four;
    - 57 of five to eight;
    - 42 of nine to sixteen;
    - 3 of 17 to 32, and 1 of 33 to 64.

    So the bursts come mostly 10 000 to 33 000 ticks apart, around the primed peak's 19 242 ticks and well inside the window in which the class leaves the product above the unkicked one (4 046 to 113 388 ticks). Over the held stretches the members' product stands at 1.451 times the unkicked product, against 1.118 over the unkicked spans. Each kick sets off a burst: 3.8 spikes a member a kick in the pair window after its span.
  - **It pauses and resumes.** In two rounds one stretch of the first half fell to 2.1 and 1.1 times the background, and the next stretch held again: the priming outlasts a missed refresh. So the first half reads 6 of 8, while the second half holds in 8 of 8.
  - **It stays quiet unkicked.** Every stretch of every unkicked span reads 0.8 to 2.6 times the background.
  - **It lets go.** Every tail reads 0.9 to 1.5 times the background. The release kept the members silent: 19 spikes in all over the eight releases.
- **The border of the region.**
  - Under set (ii), at 32 units and 0.25, a stretch of the second half falls short in three rounds: in two the train of bursts dies, and in one it pauses for a stretch. In two rounds an unkicked span ignites in its second half and holds. At 64 units and 0.375 the assembly ignites in every round and does not let go.
  - Below 0.25 and above 64 units the grid does not reach, so the region is bounded on two sides and open on two.
  - Under set (i) no cell holds without also running away. The class's unkicked product is 1.31 times ADR-0019's there, and every cell from 16 units at 0.375 up ignites in all eight rounds.
- **What running away is.** At the top weight of 32 and 64 units under both sets, and at 0.75 of 64, the members fire in nearly every window: 1 022 to 1 024 of the unkicked spans' 1 024 are burst windows. Their product is 0.06 to 0.15 of the unkicked one, the pool drained, and the release silences them for its span only.
- **The arithmetic beside the readings.** The arithmetic said a hold, if any, would be bursts set off by the drive's fluctuations on an assembly whose members need several spikes together to fire a target: at the usable cell twelve at the unkicked product and eight at the primed peak. That is what the run read — a train of bursts whose timing sits inside the primed window and whose product, 1.45 of the unkicked, approaches the primed peak's 1.60 — and an unkicked assembly of the same cell that never bursts. The class did what ADR-0113's account said it would, at one cell of the grid.
- **The evidence.** [ADR-0075](0075-the-dispatch-scope-follows-the-diff.md)'s first clause for `scope=both` applies to the diff: files under `src/` changed, in `cortex-core`, `cortex-connectome` and `runtime/cortex-runtime`. The weekly was dispatched on this round's branch at **`scope=both`**: run [36295847527](https://github.com/DescentVTT/VirtualCortex/actions/runs/36295847527) at `339babd`, the documents commit on the branch (`240fd35` on `main`), after which only this evidence, the cost table and the brief's archive change. The round's commits as `main` holds them after the rebase that merged pull request #143: `8e3ec71` the class and `f62b955` its range held in one place; `c19edff` this ADR's protocol, before any run; `78dfe0f` the kick, the backgrounds and the controls pinned, before any cell; `d193265` the grid pinned; `240fd35` the readings and the documents at 4.65.0; `8a7726a` this evidence, the cost table and the brief's archive. On the branch they were `abb29be`, `5b38a5d`, `8f192d6`, `fcb3830`, `cf6e485`, `339babd` and `785ba8c`. **Every job it ran is green.**
  - **The whole-domain tests.** All seventy-nine passed on the hosted runners. The seventy-one before this round reproduced their pinned numbers. Among them, H-17's, H-18's, H-19's and H-20's arms read their images at format 17 with the CRCs ADR-0114 re-pinned, and each image, written back to its earlier version, read the CRC it read before. This round's eight reproduced their tables. [ADR-0092](0092-the-shards-dealt-by-cost.md)'s table did not know the eight and costed each at 900 s; on the runners they took 465 to 650 s. The four shards took:

    | Shard | Job | Tests | Their seconds summed | Tests' wall time | The shard's heaviest test (s) |
    | ---: | ---: | ---: | ---: | :--- | :--- |
    | 0 | 50 m 44 s | 20 | 5 721 | 2 989 s, 42 % | H-20 from the mirrored 1 733 |
    | 1 | 44 m 23 s | 19 | 5 203 | 2 611 s, 36 % | H-20 from the assignment 1 505 |
    | 2 | 53 m 50 s | 20 | 6 338 | 3 171 s, 44 % | H-19 from the mirrored 1 367 |
    | 3 | 52 m 43 s | 20 | 6 095 | 3 105 s, 43 % | H-18 from the mirrored 1 058 |

  - **The cost table is regenerated from this run's artifacts** (`scripts/exhaustive-costs.tsv`: seventy-nine lines, 23 357 s, its source line naming the run; `npm run spec:costs` passing). Replayed through the deal, it plans each shard at 5 839 to 5 840 s summed. At this run's ratio of summed seconds to wall time, 1.91 to 2.00, that is about 2 920 to 3 060 s of tests' wall time, 41 to 43 per cent of the bound, under the brief's 60.
  - **The mutation sweep**, all seven shards green: 3 637 mutants, 3 466 caught, 145 unviable, 26 timed out and **no survivor**, so none on the class.
    - No timeout is on the class's lines: `step_stp_class`, `StpClass::is_valid`, `stp_class_of`, the turn's selection or the mark's refusal.
    - Twenty-four are mutants the scheduled run of 2026-09-21 (35578231335) also timed out on: the iterators', the barrier's, the injector's, the workers' stop, and the loader's `terms` field.
    - Two are not: the loader's `clauses` field and the negation of `load_clause`, in code this round moved down and did not change. Two injector mutants that run timed out on were caught here. A timeout's list moves with the runner's timing (ADR-0063), and ADR-0062's triage of the two is the weekly job's reading, not this round's.

  The pull request's gate on `339babd` (`240fd35` on `main`; run [36295839144](https://github.com/DescentVTT/VirtualCortex/actions/runs/36295839144)) is green in every job: check, test, fmt and clippy; the AArch64 determinism pin, unmoved; the MSRV job; the documentation gate; and the mutation gate on the changed lines, 42 mutants of which 34 were caught and 8 unviable. The build's in-diff run on a developer machine had found one survivor, an equivalent guard in the loader, which `5b38a5d` (`f62b955` on `main`) removed.

  On the developer machine every command of brief 049's verification list exited 0 (a ratio, not admissible):
  - the workspace in the debug profile, in the release profile and on the MSRV toolchain in a target directory of its own: 649 passed in each and 79 ignored;
  - check, fmt, clippy, doc, the bench `--test`, `npm ci` and `npm run spec`;
  - `--list`, 79 tests;
  - the whole-domain suite, one process a test and eight at a time, 79 of 79 in 3 593 s of wall time.
- **Not done:**
  - The grid was not widened, before the runs or after; below 0.25 and at 128 units the region's extent is unread.
  - There is one seed of the delays, one placement and one drive, on the settled network and not a drained one.
  - No readout was gated, no context switched and no reward delivered.
- **The next decision, named and not taken.** By brief 049's branch for a usable region: **ADR-0111's second round, a readout gated by the context**, with F-54's geometry to solve first, since the usable assembly shares its units with the task's readouts at 1 024 units. Its need is this reading: one assembly that holds by refreshing, stays quiet unkicked and lets go, at one cell of the grid.

## Consequences

- Good: the question ADR-0112 asked is asked again with the one mechanism ADR-0113 chose, on the same substrate, over spans that separate a refreshed hold from a fading priming by the class's own arithmetic.
- Good: the release is derived from the class and bounded over every state a member can be in, not from one burst's course.
- Good: one cell holds a context by the network's own activity, stays quiet without a kick and lets go on a signal, by rules written first. ADR-0111's first round has a substrate, and its second has a measured need.
- Bad: the usable region is one cell, at the grid's lightest weight and largest size. It is a band, not a margin: 32 units at the same weight, and 64 at the next, fail. Its extent below 0.25 and above 64 units is unread.
- Bad: under set (i), the constants closest to Mongillo's, no cell is usable. The class's larger release at the background's rate makes every assembly heavy enough to hold also ignite without a kick.
- Neutral: the per-window tables are pinned by a hash, not in full. A reader has the stretches and the dumps, and a later round that needs a window's numbers reruns the test.
- Bad: F-55 — ADR-0113's primed window, as written, ends about 4 600 and 4 100 ticks later than the product stays strictly above the unkicked one.

## Alternatives considered and why rejected

- **Shorter spans** (option 1): a run is about 40 seconds on a developer machine, and the whole grid fits the weekly job under the directive's bound. A shorter unkicked span would make "ignites" more lenient, and a shorter hold would read the priming itself.
- **ADR-0112's one-epoch lead-in** (option 2): the first round's unkicked span and the backgrounds would read members whose release fraction is still relaxing from ADR-0019's steady state, 93 of 256, toward set (ii)'s 26.
- **A volley into nearby inhibitory units** (option 3(b)): it would reach the members through the prior's synapses at the prior's weights, a release whose strength is the network's, not derived. The cancel's effect on a member is the oracle's.
- **ADR-0112's single cancel** (option 3(c)): it holds a member below its standing for about 3 500 ticks, a thirtieth of the fade.
- **The release from one burst's state** (option 4): a hold of refreshing bursts leaves the release fraction higher than one burst does. By ADR-0113's integer replica, under set (i) a burst every 16 384 to 32 768 ticks leaves it at 0.82 to 0.88 at a burst's end, against 0.71 after one burst. A release derived from the ceiling is long enough whatever the hold left.
- **One background for all sizes** (option 5): the 48 members beyond sixteen, marked, change the network the sixteen fire in.
- **Every window in full** (option 7): about 150 000 rows of five numbers in a test file, for readings the stretches and the hash already pin.
- **A binary of its own** (option 8): ADR-0112's substrate would have to move into a shared module. Its short-term readings change here anyway, and in one file ADR-0112's tables and this round's are read by the same functions.

## Confirmation

`runtime/cortex-runtime/tests/assembly.rs`:
- the constants and types: `SETS`, `SET_NAMES`, `WEIGHTS_049`, `Span`, `LEAD_IN_EPOCHS_049`, `UNKICKED_EPOCHS`, `HOLD_EPOCHS`, `TAIL_EPOCHS`, `STRETCH_EPOCHS`, `CEILING`, `FADE_HORIZON`, `COURSE_TICKS`, `ADR_0113_STEADY`, `ADR_0113_COURSE`, `StretchRow`, `ClassWindow`, `Holding`, `BurstRead049`;
- the functions: `layout`, `stretches_of`, `cancel_due`, `fade`, `silence_of`, `release_rule`, `release_span_oracle`, `after_burst`, `burst_course`, `primed`, `delivered_049`, `stepped_pair`, `stp_course`, `steady_under`, `at_rest_under`, `next_pair`, `stp_sums`, `stretch_rows`, `background_049`, `holding`, `failed`, `kick_reading_049`, `kicked_once_049`, `fires_every_member_once`, `bursts_049`, `marked`, `marked_engine`, `span_protocol`, `windows_hash`, `class_run`;
- the gate `the_facilitating_class_the_arithmetic_the_spans_the_rules_and_a_marked_assembly_on_the_engine`, and the weekly `the_kick_the_backgrounds_and_the_controls_under_set_{i,ii}_at_1024_units_exhaustive` and `a_marked_assembly_of_{16,32,64}_units_at_six_weights_under_set_{i,ii}_exhaustive`;
- the tables `CLASS_AT_REST_049`, `CLASS_STEADY_049`, `UNKICKED_PRODUCT_049`, `BURST_COURSE_049`, `PRIMED_049`, `DELIVERED_049`, `FADE_049`, `RELEASE_STRETCHES_049`, `RUN_EPOCHS_049`, `RELEASE_SPAN_ORACLE_049`, `GATE_KICKED_049`; the runs' `BACKGROUND_049`, `CONTROL_049`, `CONTROL_KICKS_049`, `WINDOWS_HASH_049`, `BACKGROUNDS_049`, `KICKS_049`, `CONTROLS_049`, `CONTROL_BURSTS_049`, `CELLS_049`, `CELL_KICKS_049`, `CELL_HASH_049`, `GRID_049` and `CELL_BURSTS_049`; and the gate's reading of the usable cell (`over_the_049_tables`).

Whitepaper §11 carries F-55, and §11.1's question on a rule held by the network this reading.
