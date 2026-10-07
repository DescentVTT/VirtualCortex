---
status: proposed
date: 2026-10-07
depends-on: ADR-0141
decision-makers: VirtualCortex maintainers
---

# ADR-0142: The readouts' own competition, measured — H-26 is no, as predicted, and at the bar: on H-25's two arms, reproduced bit for bit with two shadows of the composer's stimulus–readout synapses beside them, one under the drawn address held to the composer at every trial and one with the address's targets released to every unit, the released shadow's consolidation on the pairs of the readout not selected, signed against the answer, came to 0.486 to 0.602 of the learning signal the run delivered, past half in six of the eight mappings and under it in two; the readout not selected carried the selected one's eligibility by magnitude and a quarter to a half of it on net, moved as much weight, and fired 0.57 to 0.62 of its spikes in the readout window and 0.95 to 0.97 over the trial; net of what the shadow's selected side gained the release kept 0.51 to 0.63 of the signal; H-26's stopping rule at step 4, and the next decision, an ADR on the lateral competition §5.2.5 specifies with this round's cost per block as its need and the attention-gated feedback as its fallback, named and not taken

## Context and Problem Statement

[ADR-0141](0141-the-readouts-own-competition.md) began the second step [ADR-0138](0138-the-address-drawn.md) planned, a neural form of the address's target side, by measuring with nothing changed, and wrote **H-26** before any run: on H-25's runs, reproduced bit for bit, an open-loop shadow replays the stimulus–readout synapses as if the address's targets were every unit; the **cost** of a mapping is the shadow's consolidation on the pairs of the readout not selected, signed against the answer and summed over the mapping; the **signal** is the answer pairs' net consolidation less the other pairs' under the run; and H-26 holds when the cost is at most half the signal in every mapping of both arms. The prediction, written first, is no. Brief 059 runs it once. This ADR is the round's protocol, committed before the run, and then its readings.

What was read before anything was written (principle 2, and brief 059's directive that the engine is read before a description of it is trusted, the brief's and ADR-0141's included):

1. **What an address reaches** (`runtime/cortex-runtime/src/executor.rs`, the fan-out and `Modulations::for_synapse`): at a presynaptic spike an excitatory slot consolidates under `addressed` when its unit is a source and the slot's target a target, and under `at_rest` otherwise; the baseline is zero, so `at_rest` moves nothing. With every unit a target the second half is always true: **a synapse is addressed exactly when its source was drawn**. The selection enters the address nowhere else, so a released address is written at a tie as at any trial — `Executor::address_drawn` handed every unit — where the run's has no target and consolidates nothing.
2. **When** (`runtime/cortex-runtime/src/task.rs`, `Task::trial`): the address is written between a trial's last tick and its reward, and the fan-out reads it from the next tick. So what a trial consolidates falls under the address and the signal of the trial before it. Before the first trial every unit is a source and a target and the signal is at rest; the first trial's drawing names no source, its window having closed in the harness's lead-in ([ADR-0140](0140-the-address-drawn-measured.md)).
3. **The composer** (`runtime/cortex-runtime/tests/instrument/harness.rs`, `Composer::observe`): per stimulus–readout synapse and presynaptic spike since the last reading — the trace's decay since the block's stamp, the pair rule's two terms against the target's last spike, the stamp, then the consolidation under the baseline plus the signal the executor published at that tick where the synapse is addressed. Under the drawn delivery a synapse is addressed when its source is among the units the executor drew and its readout the one the last trial selected. The record's trace and weight of all 3 188 are held to it at every trial.
4. **A consolidation moves the trace as well as the weight** (`consolidated`, `consolidated_signed`): what the weight absorbs is taken out of the trace, and under the signed gate a punishment moves the weight against the trace and spends the trace by as much. The excitatory depression is scaled by the weight ([ADR-0055](0055-a-weight-that-settles.md)). So a synapse the release consolidates carries, from then on, another trace and another weight than the run's, and consolidates otherwise the next time the run's address reaches it. **The released shadow is not the run plus a term**: it needs its own state for every synapse, and its selected side parts from the run's as its weights do.
5. **Where the consolidation went** ([ADR-0137](0137-the-reward-unaddressed-measured.md), `Went`, `Network::replay`): per block, the weight raised and the weight lowered, each consolidation summed on the side of its own sign, in the answer's pairs — each stimulus onto its answer *under the mapping in force at the trial the consolidation fell in* — in the other two pairs, and outside. H-25's two arms pin it whole (`DRAWN_WENT_1024`).
6. **The signal is therefore already in the tree.** By ADR-0141's definition and ADR-0140's table, the answer pairs' net less the other pairs' per mapping:

   | Arm | First mapping | Second | Third | Fourth |
   | :--- | ---: | ---: | ---: | ---: |
   | Assignment first | 2 678 216 | 5 024 810 | 5 166 932 | 5 506 390 |
   | Mirrored first | 2 738 289 | 4 977 285 | 4 811 435 | 5 135 182 |

   Half of each is the bar the cost is read against: 1 339 108, 2 512 405, 2 583 466 and 2 753 195 from the assignment, and 1 369 144, 2 488 642, 2 405 717 and 2 567 591 from the mirrored, the odd ones rounded down. The bar is fixed before the shadow exists.
7. **The gate's cost**: H-25's gate test, eight trials on the instrument's network with both oracles, runs in nine seconds in the debug profile on the developer machine (a ratio, not admissible).

## Decision Drivers

- Brief 059's standing directives: nothing of the engine changes and no file under `src/`; H-25's runs reproduced bit for bit, the shadow reading the run and never writing to it; the shadow the composer's rule with one change; H-26's rule written before the run and not moved after it; no second attempt; no float; every loop ends by construction; the gate grows by at most one test; no shard of the weekly job past 60 per cent of its bound; no pinned number of an earlier round moves.
- Latest ≠ Newest: no dependency, no tool, no rule; the shadow is a second state for a function the tree already holds.
- **A structural boundary beats a reviewed one** (principle 5): "the composer's rule with one change" is one function called twice with one flag, not two writings kept alike by review; "never written back" is the shadow never being handed the executor.
- ADR-0141 leaves four things of its rule to the protocol — a tie, the trial a consolidation is entered at, the cost's sign and the shadow's selected side — and each is fixed here before the run.

## Considered Options

1. **How the shadow is built on the composer**:
   - (a) the composer's rule taken out of `Composer::observe` as a function of the synapses' state and of what a reading replays from (`advance`), called for the composer's own synapses and again for each shadow's copy of them;
   - (b) a second composer beside the first, with a flag that stops it holding the record;
   - (c) a second network's oracle (brief 057's `replay_block`) kept to the pairs.
2. **How it is held to the composer under the drawn address**: (a) a shadow under the drawn address replayed beside the released one through the whole run, held to the composer at every trial; (b) a calibration run of its own before the arms.
3. **Where the arms run**: (a) riding on H-25's two weekly tests, H-25's tables held before the shadow is read; (b) two weekly tests of their own.
4. **A tie**: (a) both readouts are readouts not selected, and what the release consolidates onto either enters the cost; (b) ties left out of the cost.
5. **The shadow's selected side**: (a) outside the cost, as ADR-0141 wrote it, and read beside it; (b) the cost taken as the run's signal less the shadow's whole signal.

## Decision Outcome

**Options 1(a), 2(a), 3(a), 4(a) and 5(a).** Everything from here to the weekly tests and their cost was committed before the run; the order of the work, the readings and the step of the stopping rule follow it.

### The shadow (option 1(a))

- **`advance`** (`tests/instrument/harness.rs`) is the body of `Composer::observe`'s oracle, moved and not rewritten: the synapses' state in, and a `Replay` — each unit's spikes, the cursor, the trial's first tick, the signal's course, the pair the last trial addressed, the drawn sources, the baseline, the signed gate and the global delivery — beside it; the terms, what each pair consolidated, the same by direction (`Shifted`, `[stimulus][readout][raised, lowered]`) and the addressed pair's moves out.
- **One change**: `Replay::released`. Under the drawn delivery a synapse is addressed when its source is among the units the executor drew *and* its readout the one the last trial selected; with `released` set, when its source is among them, whatever its readout and whatever was selected, a tie included. Unset, the predicate is the one it was, case by case, and `released` is refused without the drawn sources.
- **A `Shadow`** is a copy of the composer's synapses — their stamps, traces and weights as the record held them when the run began — with `released` set or unset, and what each trial consolidated into it. `Composer::observe` advances its own synapses, then each shadow's from the same `Replay` with the shadow's flag. Nothing else is shared: the shadows hold no reference to the executor, and the record is held to the composer's synapses alone, as before.
- **Why not a second composer** (option 1(b)): it would collect the train a second and third time and need a second flag to stop holding the record, two things to keep alike by review. **Why not the network's oracle** (option 1(c)): it is the tree's other writing of the rule and is held to the record as the composer is, but ADR-0141 names the composer's, which has replayed these 3 188 synapses since H-13, and the network's oracle already rides in the same arms as the independent check.
- **Every earlier run is the run it was**: with no shadow taken `advance` is called once, with `released` unset, where the loop stood. The calibration below holds every pinned number to that.

### How it is held (option 2(a)), and the calibration (H-26's stopping rule, step 2)

`earned_run_shadowed` (the harness; `earned_run_delivered` is it with no shadow) takes two shadows before the first tick: **one under the drawn address as the run writes it**, and **one released**.

- **The shadow under the drawn address is the composer, at every trial**: what `advance` returned for it equal to the composer's, number by number, and its every synapse's stamp, trace and weight the composer's — and so the record's. That is brief 059's third calibration, *"the shadow, run under the drawn address in place of the released one, reproduces the composer's moves"*, run through all 7 680 trials of each arm rather than in a run of its own (option 2(b)), beside the very readings it calibrates.
- **The released shadow** is held to the stamps alone, which no weight enters: it replayed the composer's spikes. At the run's end its `Shifted` rows are held to its own weights: the image's couplings plus every trial's nets are the shadow's summed weights.
- **The run's own consolidation by the shadow's fold** is held to the network's oracle at every block: nothing on the pairs of the readout not selected — under the drawn address the readout that lost is never consolidated — and the selected side `Went`'s answer pairs and other pairs, direction by direction.
- **In the tree**, before the arms: the workspace's tests in the debug and the release profile and on the MSRV, and every whole-domain test of the weekly job, each in a process of its own from the release build of this protocol's commit.
- **In each arm's test**: H-25's calibration as it stands, then H-25's run with the shadows beside the composer, then **every one of H-25's pinned tables and readings held, before anything of the shadow is dumped or read**. A run that is not H-25's bit for bit stops there, as a finding.

### Where the arms run (option 3(a))

`drawn_arm` calls `shadowed_run` (`tests/inhibition.rs`; `delivered_run` is it with no shadow) and, after its last assertion, `released_reading`. So H-26 rides on `the_address_drawn_from_the_assignment_at_1024_units_exhaustive` and `the_address_drawn_from_the_mirrored_assignment_at_1024_units_exhaustive`: the run H-26 reads is H-25's by construction and not by a second run held to the first, H-25's pins are untouched, and the weekly job grows by what two more passes over 3 188 synapses a trial cost. Two tests of their own (option 3(b)) would run each arm twice a week, about 3 500 s more, for a reading the first run already carries.

### H-26, restated as integer rules (ADR-0141's rule)

- **The fold** (`shade_of`, `Shade`): a trial's `Shifted` by
  - **the side** of the address in force, written at the trial before's end: the pairs of the readout that trial selected, and the pairs of the readout it did not;
  - **the pairs**: each stimulus onto its answer under the mapping in force at the trial the consolidation fell in, and each stimulus onto the other readout, as `Went` reads them;
  - the weight raised and the weight lowered.
- **A tie** (option 4(a)): nothing was selected, so both readouts' pairs lie on the side not selected. The run's address has no target there and the released one has every unit; leaving ties out (option 4(b)) would leave out consolidation only the release makes.
- **The trial a consolidation is entered at**: the trial it fell in, in that trial's block and under that trial's mapping — `Went`'s convention, so that the cost and the signal are sums over the same trials signed the same way. The first trial after each flip is thus signed by the new mapping though its address and its signal are the old mapping's last: three trials of 7 680 an arm, alike on both sides of the rule.
- **The cost** (`cost_of`): on the side not selected, the other pairs' net less the answer pairs' net. A rise onto the other readout and a fall from the answer are cost, as ADR-0141 wrote; a fall from the other readout and a rise onto the answer count against it, so the cost is a net and may be below zero.
- **The selected side enters nothing** (option 5(a)). It is the run's own address. In the shadow it parts from the run's as the shadow's weights do; that is read beside the cost and is not the cost. Taking the cost as the run's signal less the shadow's whole signal (option 5(b)) would be another rule than the one ADR-0141 committed.
- **The signal** (`signal_of`): per block, `Went`'s answer pairs' net less its other pairs' net, by the network's oracle; `SIGNAL_RELEASED_1024` is the table above, held in the gate to `DRAWN_WENT_1024`.
- **The rule** (`affordable`, `COST_TIMES`): twice the cost at most the signal, the product saturating.
- **The verdict** (`Released`, `released`): per arm and mapping the cost and the signal summed over the mapping's blocks (`MAPPINGS`), and `held`; `yes` when every mapping of both arms holds, over a shadow and a run of 120 blocks each. `RELEASED_PREDICTED` is no, ADR-0141's, dumped beside the reading and never asserted.

### The readings, no clause (ADR-0141's)

Per block, pinned; per mapping, computed from them:

- **The released shadow** (`Costed`): its `Shade` whole — both sides, both pairs, raised and lowered — and the block's cost by the kind of the trial each address was written at: after a correct selection, after a wrong one, after a tie.
- **The readouts' spikes** (`Heard`): the selected readout's and the other's, in the readout window and over the whole trial, a tie's two counts apart.
- **The eligibility at each reward** (`Eligible`): the record's traces over the pair synapses whose source the executor drew at that trial's end, onto the selected readout, onto the other and, at a tie, onto both; above zero and below zero apart. Each trace is the record's at its block's last presynaptic spike: what it has decayed to when the next spike consolidates it is in the shadow and not here.
- **The cost and the signal by block**, and per mapping **the reversal's blocks apart from the learned ones** (`apart`): the mapping's blocks up to and including the first with 40 of 64 correct, as `crossings` counts them, and the blocks after it.
- **The count margin the gating decided by** (`Margins`): the trials by the larger count less the smaller, in the bins 0, 1, 2, 3 to 4, 5 to 8 and 9 or more, and the margins summed.
- **The shadow's couplings** at every block's end, beside the run's: where the release would have taken them to first order, and whether any passes H-20's bound.
- Every trial's released consolidation, eligibility and spikes over the trial, by one hash.

### The gate

`the_rule_of_h_26_the_shadow_s_rule_and_the_readings_rules`, one test:
- the arms, the prediction and the rule's constant, and the signal per mapping held to H-25's pinned table;
- the verdict over tables written by hand: a cost of exactly half the signal holding and one LSB past it not, named by arm and mapping; an odd signal; a signal of zero; a cost below zero; the product at the width; a shadow or a run that is not whole;
- the fold over a trial written by hand, under each selection, each mapping and a tie, the four signs of the cost, and the blocks' fold across the first trial, a wrong selection, a tie and a flip;
- the readings' rules over values written by hand;
- **the shadow's rule against the composer's on a run written by hand**: four synapses, one drawn source and one not onto a unit of each readout, two readings through `advance` — the first pairing nothing, the second with one trace above zero and one below — under a reward, a punishment and a tie, the amounts written from the rule's own terms: the composer consolidates the drawn source's synapse onto the selected readout and nothing else, the released shadow that and the drawn source's synapse onto the other readout, the source not drawn under neither;
- eight trials on the instrument's network under the drawn delivery with the shadows beside the composer and the same eight without them: **the same run, table for table and weight for weight**; the shadow under the drawn address held to the composer at every trial; the released one's selected side the run's until the release first consolidates something of its own, and its table summed to its own weights.

### The weekly tests and their cost

- **No weekly test is added.** H-25's two arms carry the shadows; the cost table's lines stand and its times are regenerated from this round's dispatch.
- **The dispatch's scope**: no file under `src/` changes, so by [ADR-0075](0075-the-dispatch-scope-follows-the-diff.md) the weekly is dispatched at `scope=exhaustive`.

### The order of the work, as the history holds it

1. **The protocol** (`e1c300b` on the branch, pushed at 19:00Z on 2026-10-06 with pull request #177 opened as a draft on it, before the run): the harness's `advance`, `Shadow` and `earned_run_shadowed`, H-26's rule and readings, the gate, the arms' empty tables and this ADR's protocol. The pull request's checks passed on it, the determinism pin on AArch64 and the MSRV among them.
2. **The calibration**, from the release build of `e1c300b`: the workspace's tests passed in the debug profile (688 passed, 121 ignored), in the release profile (688 passed, 121 ignored) and on the MSRV (688 passed), and the check, Clippy, the documentation and the format exited 0. **Every one of the 79 other whole-domain tests of the weekly job passed**, each in a process of its own and each exit 0 with one test reported: 69 eight at a time from 19:07:10Z to 21:01:17Z, H-23's and H-24's four arms and every earlier run through `Composer::observe` among them, and the ten of ADR-0097 and ADR-0102 one at a time from 21:01:51Z to 21:05:00Z.
3. **The arms**, once, side by side from 21:05:20Z to 21:34:28Z. In each, H-25's calibration held; H-25's 7 680 trials ran with the two shadows beside the composer, the one under the drawn address held to the composer at every trial; **every one of H-25's pinned tables and readings was reproduced**, and only then were the shadow's tables dumped. Each test stopped at the first empty table, in 1 747 s. The tables were written from those dumps (`e8f92ee`), and a second run side by side from 21:37:58Z reproduced every table of H-25 and of H-26 and passed, in 1 663 and 1 658 s (on the developer machine, a ratio and not admissible).

No constant, clause or rule moved after the run, and there was no second attempt: the second run is the pinned tables' reproduction.

### The readings

**The verdict** (`RELEASED_1024`), by the rule committed first, over the pinned tables (`RELEASED_COSTED_1024`, `DRAWN_WENT_1024`):

| Arm, mapping | Cost | Signal | Half the signal | Cost over signal | Cost at most half |
| :--- | ---: | ---: | ---: | ---: | :--- |
| Assignment, 1 | 1 500 853 | 2 678 216 | 1 339 108 | 0.560 | no |
| Assignment, 2 | 2 583 722 | 5 024 810 | 2 512 405 | 0.514 | no |
| Assignment, 3 | 3 069 735 | 5 166 932 | 2 583 466 | 0.594 | no |
| Assignment, 4 | 3 073 513 | 5 506 390 | 2 753 195 | 0.558 | no |
| Mirrored, 1 | 1 423 361 | 2 738 289 | 1 369 144 | 0.520 | no |
| Mirrored, 2 | 2 420 291 | 4 977 285 | 2 488 642 | **0.486** | **yes** |
| Mirrored, 3 | 2 387 992 | 4 811 435 | 2 405 717 | **0.496** | **yes** |
| Mirrored, 4 | 3 090 173 | 5 135 182 | 2 567 591 | 0.602 | no |

- `Released { held: [[false; 4], [false, true, true, false]], yes: false }`. **H-26 is no**: the cost is past half the signal in six of the eight mappings, the assignment's four and the mirrored's first and fourth.
- **It is a no at the bar.** The cost is 0.486 to 0.602 of the signal: from 0.7 per cent under its bar to 20.4 per cent over it, and within 5 per cent of it in four of the eight.
- Its scope: this task, these two arms and this schedule at 1 024 units, on H-25's trajectory, open-loop.

**The prediction, against the reading.** ADR-0141 predicted no, because without a competition the readout that lost would carry much of the eligibility. The premise held more fully than the verdict. By magnitude the readout not selected carried as much eligibility as the one selected, and the release moved as much weight on it. But the net of what it moved was about half the signal and not all of it, because the net of that eligibility, the part above zero less the part below, was a quarter to a half of the selected readout's.

**Where the cost came from** (the shade's side not selected, per mapping; the weight raised and the weight lowered with their signs):

| Arm, mapping | Answer's pairs: raised | lowered | net | Other pairs: raised | lowered | net |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: |
| Assignment, 1 | 2 642 225 | −3 832 632 | −1 190 407 | 6 222 339 | −5 911 893 | +310 446 |
| Assignment, 2 | 6 958 360 | −8 556 573 | −1 598 213 | 8 741 717 | −7 756 208 | +985 509 |
| Assignment, 3 | 6 520 500 | −8 361 263 | −1 840 763 | 9 147 223 | −7 918 251 | +1 228 972 |
| Assignment, 4 | 5 930 843 | −7 669 617 | −1 738 774 | 8 816 841 | −7 482 102 | +1 334 739 |
| Mirrored, 1 | 2 496 262 | −3 580 370 | −1 084 108 | 6 543 713 | −6 204 460 | +339 253 |
| Mirrored, 2 | 6 316 264 | −7 645 110 | −1 328 846 | 8 496 920 | −7 405 475 | +1 091 445 |
| Mirrored, 3 | 5 552 431 | −6 984 007 | −1 431 576 | 8 052 946 | −7 096 530 | +956 416 |
| Mirrored, 4 | 6 108 856 | −8 116 002 | −2 007 146 | 9 333 075 | −8 250 048 | +1 083 027 |

- **Both moves ADR-0141 named are there, and both count.** The answer's pairs fell on net in every mapping and the other pairs rose in every mapping; the cost is the second net less the first. The answer's fall is the larger part, 55 to 79 per cent of the cost.
- **As much weight moved on the readout not selected as on the one selected**: 18.6 to 32.0 million a mapping against the shadow's 17.6 to 31.4 million on its selected side, 0.97 to 1.06 of it.
- **By the kind of the trial the address was written at** (`KINDS_RELEASED_1024`): after a correct selection, where the release raises the other readout's coupling, 32 to 54 per cent of the cost; after a wrong one, where it lowers the answer's, 42 to 52 per cent; after a tie, where it reaches both, from nothing to 16 per cent. Per trial of its kind a wrong selection costs more than a correct one in seven of the eight mappings, and nine and eleven times as much in each arm's first (3 546 and 3 923 a trial against 397 and 362), where wrong selections are fewest, 213 and 189 of 1 536.
- **Every block's cost is above zero**, from 10 282 to 186 982 a block; it is past half that block's own signal in 83 and 72 of the 120 blocks, and past the whole of it in 14 and 23.

**The reversal's blocks apart from the learned ones** (`APART_RELEASED_1024`, the cost over the signal):

| Arm | Mapping 1: up to the crossing, past it | Mapping 2 | Mapping 3 | Mapping 4 |
| :--- | :--- | :--- | :--- | :--- |
| Assignment first | 0.490, 0.603 | 0.445, 0.633 | 0.542, 0.715 | 0.558, 0.559 |
| Mirrored first | 0.428, 0.549 | 0.414, 0.638 | 0.428, 0.627 | 0.422, 0.859 |

Up to the crossing the cost is 0.41 to 0.56 of the signal, and past it 0.55 to 0.86. Past the crossing the signal per block is lower in every mapping, by 15 to 54 per cent, while the cost per block is lower in four and higher in four: once a mapping is learned the release takes the larger share of a smaller signal.

**What the verdict rests on.** Two of the protocol's choices decide how the rule reads, and each was fixed before the run:
- **The tie** (option 4(a)). With the ties' part left out, the assignment's first mapping and the mirrored's first would hold beside the mirrored's second and third, at 0.468, 0.439, 0.469 and 0.482; the assignment's second would fail by 335 in 2.5 million, and the assignment's third and fourth and the mirrored's fourth at 0.549, 0.558 and 0.546. The verdict would be no.
- **The shadow's selected side** (option 5(a)). In the shadow the selected side's own signal, its answer pairs' net less its other pairs', was **1.007 to 1.147 of the run's**: under the release the selected side delivered more than the run's did. So the shadow's whole signal, its selected side's less the cost, was **0.511 to 0.626 of the run's**, and the difference of the two signals 0.374 to 0.489 of the run's, under half in every mapping. By the measure option 5(b) would have been, every mapping holds, the mirrored's third narrowly; by the rule ADR-0141 committed, six do not. The rule does not move, and H-26 is no. Why the selected side gains is not derived here: the run leaves the traces onto the readout it did not select to decay until that readout is next selected, and the shadow consolidates them at once, so the two consolidate different traces on the same synapses.

| Arm | The shadow's selected side over the run's signal, per mapping | The shadow's whole signal over the run's |
| :--- | :--- | :--- |
| Assignment first | 1.146, 1.138, 1.138, 1.109 | 0.586, 0.623, 0.544, 0.551 |
| Mirrored first | 1.146, 1.051, 1.007, 1.147 | 0.626, 0.565, 0.511, 0.545 |

**The readouts' spikes** (`RELEASED_HEARD_1024`, per trial that selected a readout):

| Arm | In the readout window: selected, other | other over selected | Over the whole trial: selected, other | other over selected |
| :--- | :--- | :--- | :--- | :--- |
| Assignment first | 13.9 to 15.6, 8.5 to 9.1 | 0.58 to 0.62 | 140.3 to 143.8, 135.2 to 137.7 | 0.958 to 0.965 |
| Mirrored first | 15.0 to 16.4, 8.9 to 10.2 | 0.57 to 0.62 | 141.9 to 146.0, 135.2 to 139.6 | 0.952 to 0.963 |

The selected readout is by the rule the one that counted more in the readout window, 500 ticks of the trial's 16 384, and there it leads by five to seven spikes a trial. Over the whole trial it leads by the same five to seven: outside the window the two fire alike. Nothing makes the readout that lost fire less, as ADR-0141 read from the task.

**The eligibility at each reward** (`RELEASED_ELIGIBLE_1024`, the record's traces from the sources drawn at that trial's end, summed per mapping, the readout not selected over the selected one):

| Arm | Above zero | Below zero | Magnitude | Net |
| :--- | :--- | :--- | :--- | :--- |
| Assignment first | 0.82 to 0.98 | 1.10 to 1.20 | 0.93 to 1.05 | 0.33 to 0.49 |
| Mirrored first | 0.80 to 0.97 | 1.13 to 1.20 | 0.93 to 1.06 | 0.25 to 0.52 |

The readout not selected carries a little less trace above zero and a little more below it than the selected one, the same amount in all, and so a net that is a quarter to a half of the selected readout's: 2 654 to 4 447 a trial against 5 658 to 10 772. That net is above zero in every mapping, and after a correct selection the release raises the other readout's coupling.

**The count margin the gating decided by** (`RELEASED_MARGINS_1024`): 4.4 to 7.0 per cent of a mapping's trials tied; the margin was one or two spikes in a further 18 to 25 per cent, so at most two in 23 to 31 per cent; nine or more in 18 to 30 per cent; the mean 5.0 to 6.4 spikes.

**The shadow's couplings** (`RELEASED_COUPLINGS_1024`): at each mapping's end each stimulus's coupling onto its answer stood above its coupling onto the other readout by 0.079 to 0.163 of its image coupling in the shadow, against the run's 0.152 to 0.231, 0.44 to 0.65 of the run's separation in fourteen of the sixteen and 0.78 and 0.95 in the other two. The shadow's highest coupling was 1.154 and 1.144 of its image's, under the run's 1.181 and 1.191. These are where H-25's spikes would have taken a released rule; a released run would have other spikes.

**The oracles held** at every one of the 15 360 trials: the shadow under the drawn address to the composer, number by number and synapse by synapse, and so to the record; the released shadow's stamps to the composer's; the composer and the network's oracle to the record, the drawn sources to the critic's oracle, as in H-25. At every block the run's own consolidation by the shadow's fold was nothing on the pairs of the readout not selected and, on the selected side, the network's oracle's answer pairs and other pairs, direction by direction. At each run's end the released shadow's tables summed to its own weights. The gate holds the pinned tables to one another and to H-25's block by block: the kinds to the block's cost, the shadow's couplings to what its shade holds, the readouts' spikes in the window and the ties to H-25's blocks, and the margins to the selected readout's count less the other's.

### The evidence

- **The dispatch's scope.** The diff changes no file under `src/` — two files under `tests/`, the documents and the brief — so by [ADR-0075](0075-the-dispatch-scope-follows-the-diff.md) the weekly was dispatched on this round's branch at **`scope=exhaustive`**, as brief 059 asks: run [37546642489](https://github.com/DescentVTT/VirtualCortex/actions/runs/37546642489) at `20367ed`, the readings' commit, whose tests are those of `e8f92ee`, the pinned tables. The round's commits on the branch: `e1c300b` the protocol, before the run; `e8f92ee` the arms' tables and the gate's checks over them; `20367ed` this ADR's readings and the documents; and the commit that carries this section, the cost table and the brief's archive.
- **Every exhaustive job is green.** All eighty-one `exhaustive` tests passed, each exit 0 with one test reported: the seventy-nine others reproduced their pinned numbers on the hosted runners, and **H-25's two arms reproduced H-25's tables and H-26's there, in 1 025 s from the assignment and 1 319 s from the mirrored**.
- **What the shadows cost is inside the runners' variance.** In the same run H-24's arms, the same run's structure without the shadows, took 1 771 and 1 778 s on two other runners, and H-23's mirrored arm took 1 297 s beside H-25's mirrored at 1 319 on the same one.
- **The shards**, dealt by the table before this round:

  | Shard | Job | Its two heaviest tests (s) | Tests | Their seconds summed | Tests' wall time |
  | ---: | ---: | :--- | ---: | ---: | ---: |
  | 0 | 55.4 min | H-24 from the assignment 1 771; H-20 from the mirrored 1 759 | 14 | 6 505 | 3 266 s, 45 % |
  | 1 | 34.4 min | H-25 from the assignment 1 025; H-21 from the mirrored 1 003 | 14 | 3 990 | 2 021 s, 28 % |
  | 2 | 51.9 min | H-24 from the mirrored 1 778; H-22 from the assignment 1 612 | 14 | 5 530 | 3 056 s, 42 % |
  | 3 | 43.4 min | H-23 from the assignment 1 438; H-22 from the mirrored 1 303 | 13 | 5 002 | 2 557 s, 36 % |
  | 4 | 59.0 min | H-21 from the assignment 1 760; H-20 from the assignment 1 722 | 13 | 6 594 | 3 482 s, 48 % |
  | 5 | 42.6 min | H-25 from the mirrored 1 319; H-23 from the mirrored 1 297 | 13 | 4 954 | 2 504 s, 35 % |

  The percentages are of the job's bound, 120 minutes; every shard ran its tests two at a time, at 1.81 to 1.99 of their summed seconds over the wall. No shard passed 60 per cent of its bound.
- **The cost table is regenerated from this run** (`node scripts/exhaustive-costs.mjs from <artifacts> --run 37546642489`): 81 lines, 32 575 s, the same eighty-one tests at 0.829 of the table before. These runners were faster than ADR-0140's: the seventy-nine others read 0.844 of their earlier seconds, and H-25's two arms, now with the shadows, 0.614 and 0.727 of theirs. ADR-0092's deal plans each of the six shards at 5 428 to 5 430 s summed, about 2 810 s of wall time at the run's ratio, **about 39 per cent of the bound**, inside the brief's 60. The table is one run's seconds on shared runners; ADR-0140's read 46 per cent for the same tests.
- **The mutation gate.** No file under `src/` changed, so the diff holds no mutant: `cargo mutants --in-diff` against `origin/main` reads "No mutants to filter", on the developer machine and in the pull request's job.
- **The pull request's gate** on `e1c300b` and on `20367ed` is green in every job, the determinism pin on AArch64 and the MSRV among them.

### The step of the stopping rule reached

**Step 4**: *"No: the release costs more than half the learning signal."* **The next decision is an ADR on the lateral competition whitepaper §5.2.5 specifies, with this round's cost per block as its need and the attention-gated feedback named as its fallback.** It is named and not taken. What this round hands it:

- **The need by the rule**: the cost is 0.486 to 0.602 of the signal, so in the worst mapping a sixth of the cost stands between the release and the bar. Per block the cost and the signal are `RELEASED_COSTED_1024` and `DRAWN_WENT_1024`.
- **What a competition would act on**: the readout that lost already fires 0.57 to 0.62 of the winner's spikes in the readout window and 0.95 to 0.97 over the trial, and carries the winner's eligibility by magnitude and a quarter to a half of it on net.
- **What the rule did not read**: net of what the shadow's selected side gains, the release's first-order loss is 0.37 to 0.49 of the signal, under half in every mapping. The next ADR weighs that reading with the verdict; this one does not.

Step 3 did not arise; step 5 is kept.

## Consequences

- Good: the cost is computed by the function the record is held to, with one flag changed, and the shadow that does not set the flag is the composer at every trial of the run it is read on.
- Good: H-26 is read on H-25's run by construction, at no weekly cost, and H-25's tables are held before it.
- Good: the signal's side of the rule was in the tree before the shadow was written.
- Good: the second step's build is handed a measured gap: the cost per block, the readouts' spikes in and out of the window, and the eligibility the readout that lost carries, by magnitude and on net.
- Bad: `Composer::observe` is edited, the oracle of every learning round since brief 032. The edit moves its loop into a function and adds a table by direction; every pinned number of the tree is the evidence that it changed nothing.
- Bad: the shadow is open-loop, as ADR-0141 says: it reads the release on H-25's trajectory.
- Bad: the verdict is at the bar, and two readings beside it lean the other way — two mappings hold, and net of what the shadow's selected side gains the first-order loss is under half the signal in every mapping. The rule committed first reads no and is kept; the next ADR has both.
- Neutral: H-25's two tests now assert H-26's tables after their own; a failure of the second kind fails a test named for the first.

## Alternatives considered and why rejected

- **A second composer** (option 1(b)) **or the network's oracle** (option 1(c)): see above.
- **A calibration run of its own** (option 2(b)): it would hold the shadow to the composer on another run than the one read.
- **Two weekly tests of their own** (option 3(b)): see above.
- **Ties left out** (option 4(b)) **and the cost as a difference of signals** (option 5(b)): see above.

## Confirmation

- `runtime/cortex-runtime/tests/instrument/harness.rs`: `advance`, `Replay`, `Shifted`, `Shadow`, `Composer::{shadow, eligible, shifted, shadows}`, `Shadowed`, `earned_run_shadowed`.
- `runtime/cortex-runtime/tests/inhibition.rs`: `RELEASED_ARMS`, `RELEASED_PREDICTED`, `COST_TIMES`, `Shade`, `shade_of`, `cost_of`, `signal_of`, `Costed`, `costed_blocks`, `affordable`, `Released`, `released`, `Heard`, `Eligible`, `Margins`, `apart`, `Beside`, `shadowed_run`, `released_reading`, `SIGNAL_RELEASED_1024`, `RELEASED_1024` and the pinned tables `RELEASED_{COSTED, HEARD, ELIGIBLE, MARGINS, COUPLINGS, HASH}_1024`, `COST_RELEASED_1024`, `KINDS_RELEASED_1024` and `APART_RELEASED_1024`; the gate's test named above.
- Every pinned number of ADR-0065 to ADR-0140 unchanged, and the determinism pin; no file under `src/` changed.
