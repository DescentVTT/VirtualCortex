---
status: proposed
date: 2026-10-07
depends-on: ADR-0143
decision-makers: VirtualCortex maintainers
---

# ADR-0144: The gate's output delivered, built — ADR-0143's two mechanisms in the task: a `Hold`, a parameter of the task unset by default, under which the selection is made before the tick the readout window closes at and the channel the gate's own output holds, the one not selected, takes basal messages of negative efficacy into every unit of its set on a cadence from the close to the hold's last tick, nothing at a tie; and `Delivery::Released`, the drawn delivery with every unit a target, at a tie too, refused without the critic's window as the drawn one is; four refusals of a hold named by `Task::check`; no rule of the engine, no record and nothing of the image changed, and a task with no hold runs the trial it ran, held against the trial ADR-0139 left written out as an oracle

## Context and Problem Statement

[ADR-0143](0143-the-gates-output-delivered.md) decided the gate's output delivered to the network and the released address, and left their placement to brief 060. Its decision:
- *"When the selection is made. At the readout window's close, from the counts the selection reads today. The selection, and everything that depends on it, is what it was."*
- *"What is delivered. Into every unit of the channel the gate did not select, negative basal messages by a schedule, from the window's close to tick $2^{12}$ of the trial … Nothing is delivered at a tie, where no channel was selected."*
- *"Where it lives. In the task, beside the cancel: a parameter of the task, unset by default. Unset, a trial is the trial it was, bit for bit."*
- *"A delivery beside the three: the sources drawn by the engine as `Delivery::Drawn` draws them, and every unit a target, at a tie as at any trial. It is refused without the critic's window."*

Brief 060's empowerment names the choices: the names and the shape of the task's parameter and of the delivery, how each is refused, and how the trial is composed around a selection at the window's close. This ADR is the build. The schedule's numbers and H-27's protocol are the measurement's, in the ADR that follows this one.

What was read first (principle 2, and the brief's directive that the engine is read before a description of it is trusted):

1. **The trial** (`runtime/cortex-runtime/src/task.rs`, `Task::trial`): the stimulus injected, then for each tick the cancel where it is due, the drive's step and `Executor::tick`; after the last tick `Readout::count_window` over the task's `Window` and `Readout::select`; then the delivery's address, then the reward.
2. **The stamps** (`Executor::tick`): a tick stamps its spikes with the clock as the tick found it, so the tick run before trial tick `k + 1` stamps `start + k`. A window `from..from + ticks` is the ticks `k` in that range. Before the tick `from + ticks` runs, every tick of the window has run and its spikes are in the train (`merge_spikes`).
3. **`count_window`** passes over entries after the window and stops at the first before it ([ADR-0065](0065-the-instrument-recalibrated.md)). So the counts read before the tick the window closes at are the counts read after the trial's last tick, while the train holds the trial: `Task::check` refuses a train that cannot (`TrainTooSmall`).
4. **The gate** (`crates/cortex-basal-ganglia`, `compute_gating`): it writes each channel's net output, `gpi_snr_inhibition`, and selects where it falls below zero. `Readout::select` feeds each channel its own count as the direct drive and the other's as the indirect, so the channel not selected holds an output above zero, the selected one below, and at a tie both hold zero. Nothing read the field but the flag.
5. **An injection** (`Stimulus::send`, `Cancel`): a message injected before trial tick `k` is integrated on tick `k + 1`; the executor scales a turn's sum by the gain and one message's efficacy is clamped at −2.0 (F-47); the membrane rule drops an input that lands inside a unit's refractory window ([ADR-0018](0018-membrane-integration.md)).
6. **The drawn address** (`Executor::address_drawn(targets)`, [ADR-0139](0139-the-address-drawn-built.md)): the sources from the critic's counts, the targets exactly as given. Handed every unit, it writes the address ADR-0143 calls released; no call of the executor is missing.

## Decision Drivers

- ADR-0143's decision and brief 060's standing directives: nothing of the engine's rules, the records or the image changes; unset, bit for bit, selections included; no lateral wiring and no feedback tag; no dependency, no float, no `unsafe` beyond ADR-0023's invariant.
- **A structural boundary beats a reviewed one** (principle 5): which channel is held is the gate's own field, not a second reading of the counts; "the selection is made once, at the close" is the shape of the loop, not a comparison a mutant can move.
- Latest ≠ Newest (§2.1): a message the executor already carries, injected at ticks the task already controls, into a set the task already names. Nothing is adopted.
- The mutation gate ([ADR-0030](0030-verification-governance.md)): every mutant in the changed lines is caught by a test.

## Considered Options

1. **The parameter's shape**: (a) `Hold { until, every, messages, efficacy_q16 }`, due before the tick the window closes at and before every `every`-th tick after it below `until`; (b) a `Cancel`'s shape, every tick of a span; (c) a list of timed injections.
2. **Which channel is held**: (a) each channel whose `gpi_snr_inhibition` the last selection left above zero; (b) the index the selection did not return.
3. **Where the selection is made**: (a) with a hold, before the first tick the hold is due at, which is the tick the window closes at, once; without one, after the last tick, as before; (b) before the tick the window closes at in every trial.
4. **The released delivery**: (a) `Delivery::Released`, the task calling `Executor::address_drawn` with every unit; (b) a second call of the executor.
5. **What a trial reports**: (a) the hold's messages in `Outcome::held`; (b) nothing, the count left to the caller's arithmetic.

## Decision Outcome

**Options 1(a), 2(a), 3(a), 4(a) and 5(a).**

- **The hold** (`runtime/cortex-runtime/src/task.rs`, option 1(a)): `Hold { until, every, messages, efficacy_q16 }`, and `Task::hold: Option<Hold>`, none by default. `Hold::is_due(close, k)` is true when `k` is at or after the close, below `until`, and a whole number of cadences after the close; a cadence of zero is never due. The first message lands on the tick after the close and the last on `until` at the latest. A span of every tick (option 1(b)) cannot say "less often", and silencing a unit for thousands of ticks through a compartment that leaks over $2^9$ needs far fewer than one message a tick; a list (option 1(c)) is a shape with no rule in it.
- **What is held** (option 2(a)): `Readout::hold(inject, hold)` sends the hold's messages into every unit of each channel whose net output is above zero, and returns the messages sent. After a selection that is the channel not selected; at a tie both outputs are zero and nothing is sent; before any selection the channels are at rest and nothing is sent. The task reads the field the gate wrote and decides nothing itself. The other index (option 2(b)) would be the task's second reading of a decision the gate already holds.
- **The trial** (option 3(a)): before each tick, after the cancel and before the drive's step, a hold that is due there makes the selection if none is made yet — `count_window` over the train and `select` — and delivers. The first tick a hold is due at is the tick the window closes at, so the selection is made there, once, with every tick of the window in the train. After the last tick a trial with no selection yet makes it there, as before. **The counts and the selection are the ones the trial's end would read** (item 3 above). Selecting at the close in every trial (option 3(b)) would read the same counts and move a trial with no hold off the code it ran; the directive is that it does not move.
- **A task with no hold runs the trial it ran.** Its ticks reach no line the hold added; the read after the last tick is the two calls it was, in their order.
- **The released delivery** (option 4(a)): `Delivery::Released` beside the three. In the trial's match it calls `exec.address_drawn(0..units)`, whatever was selected. `Task::check` asks `exec.draws()` under it as under the drawn delivery, and refuses by `TaskError::Address(AddressError::NoCritic)` or `NoWindow` before any tick. The executor is not touched.
- **The refusals** (`Task::check`, after the window's own): `EmptyHold` (no message, or no cadence); `HoldNotNegative` (an efficacy of zero or above is a drive into the channel); `HoldBeforeClose` (`until` at or before the window's close: no tick of the hold is due — a window that is the whole trial closes with it, so no hold fits one); `HoldOutsideTrial` (`until` at or past the trial's length: the last message could land after the trial's last tick). A trial with a refused hold runs no tick.
- **What a trial reports** (option 5(a)): `Outcome::held`, the messages the hold delivered, zero without a hold and at a tie. The measurement reads it per trial against the hold's times.
- **A hold and a delivery are independent.** A hold can be set under any delivery, and the released delivery used with no hold: the measurement's control is the second.

### The tests

- `a_hold_is_due_on_its_cadence_from_the_close_and_at_no_other_tick`: the rule at its edges — the close itself, `until` on the cadence and off it, a cadence of one, of zero and longer than the span, a close at or after `until`, the top of the tick space.
- `the_gate_holds_the_channel_not_selected_and_neither_at_a_tie`: after each selection the outputs' signs, the messages sent, and every unit's basal potential two ticks on — three messages into each unit of the other readout and nothing anywhere else; at a tie and before a selection nothing; a ring that refuses a message stops the hold.
- `every_refusal_of_a_hold_is_named`: each refusal at its edge, their order, a window refused before the hold is read against it, and no tick run.
- `a_trial_with_no_hold_is_the_trial_before_the_hold`: `Task::trial` against the trial as ADR-0139 left it, written out in the test as the oracle, on twin engines over eight trials under a drive, a cancel, a sub-window, the critic's window and each of the three deliveries, a readout cued before some: the same outcome, units' fields, train, messages drained, addressed set, counts and signal after every trial, with each selection and a tie among them.
- `a_hold_selects_at_the_window_s_close_and_holds_the_channel_not_selected`: on sixteen armed units at a gain of 1.0, the held units' potentials after the trial are those of `integrate` stepped alone with −4.0 landing on the tick after each tick the hold was due at; every other unit, the train and the outcome but `held` are those of the same trial with no hold; the counts are the train's at the trial's end; a window closing on the tick after the cued readout's spike selects it, and one closing on the spike's own tick is a tie with the hold as without it.
- `the_released_delivery_addresses_the_units_the_critic_counted_onto_every_unit`: the sources the units the window counted, as the drawn delivery's from the same state, and the targets every unit at a tie and at a selection; refused before any tick without the critic and without its window; the two deliveries that draw nothing not refused there.
- `a_hold_delivers_at_its_times_and_at_no_other` (over `testkit/prop.rs`): over seeded holds and windows a cued trial delivers the hold's messages a hand rule counts and an uncued one none, the ring draining what was sent and no more; and `is_due` against the hand rule over the lattice of closes, ends, cadences and ticks.

**Every run under the three deliveries with no hold is the run it was, bit for bit.** The whole-domain tests reproduce their pinned numbers in the measurement's calibration and in the weekly dispatched on its branch; the measurement's ADR records both. No pin moves: the image, its format and every record are untouched.

### Consequences

- Good: the gate's net output, a field the engine computed and nothing read, now decides which channel is held; the task adds no decision of its own.
- Good: the selection is made at the close by the loop's shape, and the unset path is the code it was.
- Good: the released delivery needed no call the executor lacked.
- Neutral: the hold is the output nuclei's inhibition composed by the task over the body's channels. Whitepaper §5.2.5's lateral competition stays Specified, as ADR-0143 says.
- Bad: a unit inside its refractory window at the close drops the hold's messages until its window ends; with a cadence longer than a tick it is unheld until the next one lands. The measurement's frozen check reads what that leaves.
- Bad: `Task` grows a field, and every literal of it names one more.

## Alternatives considered and why rejected

- **A span of every tick** (option 1(b)) **or a list** (option 1(c)): see above.
- **The index the selection did not return** (option 2(b)): see above.
- **Selecting at the close in every trial** (option 3(b)): see above.
- **A second call of the executor** (option 4(b)): `address_drawn` already takes any targets.
- **No count in the outcome** (option 5(b)): the measurement would hold the hold's messages to arithmetic and not to the ring.

## Confirmation

- `runtime/cortex-runtime/src/task.rs`: `Hold`, `Hold::is_due`, `Readout::hold`, `Task::hold`, `Outcome::held`, `Delivery::Released`, `TaskError::{EmptyHold, HoldNotNegative, HoldBeforeClose, HoldOutsideTrial}`, `Task::check`, `Task::trial`, and the tests named above.
- `runtime/cortex-runtime/tests/instrument/harness.rs`: `Composer::released`, `HeldRead`, `hold_times`, `earned_run_held`.
- Whitepaper §5.2.5, §6.5, §8.8 and §9; `CHANGELOG.md`.
