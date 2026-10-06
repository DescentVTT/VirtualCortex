---
status: proposed
date: 2026-10-07
depends-on: ADR-0141
decision-makers: VirtualCortex maintainers
---

# ADR-0142: The readouts' own competition, measured — brief 059's protocol, committed before the run: two shadows of the composer's stimulus–readout synapses beside H-25's two arms, each with traces and weights of its own from the image's, replayed by the composer's own rule from the run's train, signal, selection and drawn sources and never written back, one under the drawn address as the run wrote it and held to the composer at every trial, one with the address's targets released to every unit; H-25's tables held first, then H-26's rule as integers — the released shadow's consolidation on the pairs of the readout not selected, signed against the answer, at most half the learning signal H-25's pinned table already holds, in every mapping of both arms — and the readings beside it

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

**Options 1(a), 2(a), 3(a), 4(a) and 5(a).** Everything below is committed before the run.

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

### The order of the work

1. **This protocol**: the harness's `advance`, `Shadow` and `earned_run_shadowed`, H-26's rule and readings, the gate and the arms' empty tables, committed and pushed with the pull request opened as a draft, before the run.
2. **The calibration**, from the release build of that commit.
3. **The arms, once.** The tables are written from that run's dumps, and a second run reproduces them.
4. **The readings**, by the rule committed first.

## Consequences

- Good: the cost is computed by the function the record is held to, with one flag changed, and the shadow that does not set the flag is the composer at every trial of the run it is read on.
- Good: H-26 is read on H-25's run by construction, at no weekly cost, and H-25's tables are held before it.
- Good: the signal's side of the rule was in the tree before the shadow was written.
- Bad: `Composer::observe` is edited, the oracle of every learning round since brief 032. The edit moves its loop into a function and adds a table by direction; every pinned number of the tree is the evidence that it changed nothing.
- Bad: the shadow is open-loop, as ADR-0141 says: it reads the release on H-25's trajectory.
- Neutral: H-25's two tests now assert H-26's tables after their own; a failure of the second kind fails a test named for the first.

## Alternatives considered and why rejected

- **A second composer** (option 1(b)) **or the network's oracle** (option 1(c)): see above.
- **A calibration run of its own** (option 2(b)): it would hold the shadow to the composer on another run than the one read.
- **Two weekly tests of their own** (option 3(b)): see above.
- **Ties left out** (option 4(b)) **and the cost as a difference of signals** (option 5(b)): see above.

## Confirmation

- `runtime/cortex-runtime/tests/instrument/harness.rs`: `advance`, `Replay`, `Shifted`, `Shadow`, `Composer::{shadow, eligible, shifted, shadows}`, `Shadowed`, `earned_run_shadowed`.
- `runtime/cortex-runtime/tests/inhibition.rs`: `RELEASED_ARMS`, `RELEASED_PREDICTED`, `COST_TIMES`, `Shade`, `shade_of`, `cost_of`, `signal_of`, `Costed`, `costed_blocks`, `affordable`, `Released`, `released`, `Heard`, `Eligible`, `Margins`, `apart`, `Beside`, `shadowed_run`, `released_reading`, `SIGNAL_RELEASED_1024`; the gate's test named above.
- Every pinned number of ADR-0065 to ADR-0140 unchanged, and the determinism pin; no file under `src/` changed.
