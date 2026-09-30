---
status: proposed
date: 2026-09-30
depends-on: ADR-0138
decision-makers: VirtualCortex maintainers
---

# ADR-0139: The address drawn, built — ADR-0138's drawing as `Executor::address_drawn(targets)`, a call between ticks that writes the addressed set's sources from the critic's own counts, every unit whose count is not zero, and its targets exactly as given, refused while the critic is unset (`AddressError::NoCritic`) or its window is unset (`AddressError::NoWindow`) and whole for a target outside the arena; `Executor::draws` the check the call and the task both read; the task's `Delivery::Drawn` calling it with the selected readout's units and with none at a tie, refused by `Task::check` before any tick on an engine that cannot draw; nothing of the image, the records or a rule changed, and every run under the global and the addressed deliveries the run it was, bit for bit

## Context and Problem Statement

[ADR-0138](0138-the-address-drawn.md) decided the drawing and left its placement to brief 058. Its decision:
- *"Between ticks, the executor sets the address's sources to the units whose critic count is not zero, and its targets to the units the caller gives. It is refused while the critic or its window is unset."*
- *"A delivery beside the two, which calls it with the selected readout's units, and with no targets at a tie, as the addressed delivery does. The global and the addressed deliveries are untouched."*
- *"The task writes the address between a trial's last tick and its reward, so the counts it draws from are that trial's window. Nothing of the image or the records changes, and no format moves."*
- Option 3(a): the drawing lives in the executor, *"so that the per-trial part of the address is written from the engine's own state by the engine. The task gives only the body's channel its own selection chose."*

Brief 058's empowerment names the choices: the call's name and signature and how it is refused; the delivery's name and how the task composes it. What was read first (principle 2, and the brief's directive that the engine is read before a description of it is trusted):

1. **The address** (`runtime/cortex-runtime/src/executor.rs`): two flags a unit in the executor's shared state, `sources` and `targets`, every one set at birth. `Executor::address(sources, targets)` clears each side and sets the units named, refused whole with both sides as they were when a unit on either side is outside the arena (`AddressError::NoSuchUnit`); `address_all` sets every flag. The fan-out reads the flags (`Modulations::for_synapse`): an excitatory synapse whose source and target are both flagged consolidates under the signal, any other under the baseline alone. Nothing else writes the flags, and the image does not carry them.
2. **The counts** ([ADR-0131](0131-the-critic-built.md), [ADR-0134](0134-the-critics-window-built.md)): `Executor::features`, one `u32` a unit while the critic is set and none while it is unset, raised in `merge_spikes` for each merged spike when `in_window` holds — always with the window unset, and with it set only fewer ticks than its length after `window_opened`. `reward` with the critic set zeroes every count and opens the window at the clock. The configuration refuses a window without the critic (`ConfigError::WindowWithoutCritic`), and so does the loader, so a set window always has a critic beside it.
3. **The task's order** (`runtime/cortex-runtime/src/task.rs`, `Task::trial`): the trial's ticks, then the readout's count and the selection, then the addressed set written by the delivery's match whatever the feedback, then `exec.reward` unless the feedback is withheld. So a call made in the match reads the counts before the reward zeroes them: the spikes of the trial's window, the window having opened at the previous trial's reward.
4. **The check** (`Task::check`) runs at the start of every trial and refuses, before any injection or tick, what a run cannot do; the task's refusals of the executor surface as `TaskError::Address`.

## Decision Drivers

- ADR-0138's decision and brief 058's standing directives: the sources from the executor's own counts and never from the task's stimulus; the targets the body's channel; no rule of the engine changed; nothing of the image or the records; unset, bit for bit; no dependency, no float, no `unsafe` beyond ADR-0023's invariant.
- **A structural boundary beats a reviewed one** (principle 5): the sources are written by the executor from a field the task cannot write, so no task can hand the drawing a label; an engine that cannot draw is refused by name, by the call and by the task's check alike.
- A run that cannot draw is refused before it spends a tick.

## Considered Options

1. **The call**: (a) `Executor::address_drawn(targets)`, the sources written from the counts and the targets as given, in one call; (b) a call that writes the sources alone, the targets through `address`'s target side; (c) a flag that makes `reward` draw the sources itself.
2. **The refusal**: (a) two variants of `AddressError`, `NoCritic` and `NoWindow`, and `Executor::draws`, the check the call and the task both read; (b) one variant for both.
3. **Where the task refuses**: (a) in `Task::check`, before any tick; (b) at the trial's end, where the call refuses.
4. **The measurement's harness**: (a) the composer's oracle takes the sources the executor drew, held first to the counts the harness's own critic oracle keeps; (b) the composer re-derives the sources from the stimulus units it replays.

## Decision Outcome

**Options 1(a), 2(a), 3(a) and 4(a).**

- **The call** (`runtime/cortex-runtime/src/executor.rs`, option 1(a)): `Executor::address_drawn(targets)` between ticks. It asks `draws` first; then it refuses the whole call for a target outside the arena, the set standing as it was; then it writes every unit's source flag from its count, set exactly where the count is not zero, and writes the target side as `address` writes it, every flag cleared and the units named set. It reads the counts and writes nothing else: no count, no weight, no signal and no window moves. The address it leaves is read by the fan-out as any address is. A call that writes the sources alone (option 1(b)) would leave the addressed set half-written between two calls; a flag read by `reward` (option 1(c)) would tie the address to the reward, which the task writes whatever the feedback (ADR-0068), and would move the rule of `reward`, which ADR-0138 does not change.
- **The refusal** (option 2(a)): `AddressError::NoCritic` while the critic is unset — there is no count to draw from — and `AddressError::NoWindow` while the critic is set and its window unset — the counts would be every spike since the previous reward, the whole network's, which is the global delivery's reach and not an address (ADR-0138). `Executor::draws` returns the refusal or `Ok`, the critic named first. One variant for both (option 2(b)) would not say which of the two the engine lacks.
- **The delivery** (`runtime/cortex-runtime/src/task.rs`): `Delivery::Drawn` beside `Global` and `Addressed`. In the trial's match it calls `exec.address_drawn` with the selected readout's units, and with none at a tie, whatever the feedback. The task gives the channel its readout selected, the body's interface; it gives no stimulus. `Global` and `Addressed` write what they wrote.
- **The task's refusal** (option 3(a)): `Task::check` asks `exec.draws()` under the drawn delivery, after its other refusals, and refuses by `TaskError::Address(AddressError::NoCritic)` or `NoWindow` before any injection or tick. At the trial's end (option 3(b)) the trial's ticks would already have run on an engine that could not draw.
- **Withheld feedback**: the task writes the drawing and delivers no reward, so no count is zeroed and no window opens again. The next trial's drawing names the units of the window the last reward opened, or the engine's start. The drawn delivery is written for rewarded runs, and a withheld control under it reads that.
- **The measurement's harness** (`runtime/cortex-runtime/tests/instrument/harness.rs`, option 4(a)): under `Delivery::Drawn`, `earned_run_delivered` holds the executor's sources at every trial's end to exactly the units whose count its own critic oracle holds not zero, read from the train within the window before the reward zeroes them, and its targets to the selected readout's units, none at a tie; it hands the composer those sources (`Composer::drawn`), whose oracle then consolidates a stimulus–readout synapse under the signal where its readout was the one selected and its source among them — a unit of the presented stimulus the window did not count is not, and a unit of the other stimulus's set it did count is. `Composer::drawn` is none under the other deliveries, where the composer decides as it decided. Re-deriving the sources from the stimulus units (option 4(b)) would miss the background units the window admits, which are what the drawing adds.
- **The tests.**
  - `executor.rs`: the call refused without the critic and with the critic and no window, the set standing as it was, at birth and after an address; with both set, four units, two kicked at the window's opening and one after it closed, the counts held to the train's spikes within the window, the sources exactly the units with a count, the targets exactly the units given, the arena's last unit taken as a target and one past it refused whole, none at a tie; the call moving no count, no weight and no signal; after a reward no source; `address_all` writing over it.
  - `task.rs`: under `Delivery::Drawn` on sixteen armed units with the critic and a window, a trial's sources exactly the units the train shows fired within the window after its opening, whatever the stimulus presented: the stimulus's volley in a window of thirty ticks; no source when the ticks between two trials closed the window, the cued readout still the target; the cued readout's units drawn beside the stimulus's in a window that spans them; with the feedback withheld the counts standing and the sources theirs; the targets the selected readout's, none at a tie; refused before any tick without the critic and without the window.
- **Every run under the global and the addressed deliveries is the run it was, bit for bit**, by construction: their arms of the match are untouched, the new arm is reached by no task that does not name it, and the harness's composer decides as it decided while `Composer::drawn` is none. The whole-domain tests reproduce their pinned numbers in the calibration of brief 058's measurement and in the weekly dispatched on its branch; the measurement's ADR records both. No pin moves: the image, its format and every record are untouched.
- **The mutation gate** ([ADR-0030](0030-verification-governance.md)): every mutant `cargo-mutants` makes in the changed lines is caught. The in-diff run's outcome is recorded with the measurement's evidence.

### Consequences

- Good: the reward's sources are written by the engine from the spikes it was given, in a field the host cannot write; per trial the host gives the reward's sign and the channel the engine's own selection chose.
- Good: a run that cannot draw is refused by name before it runs a tick, and the refusal says which half the engine lacks.
- Good: the global and the addressed deliveries are the runs they were, so every learning run pinned since H-14 stands beside the drawn one.
- Neutral: the targets are still the task's call over the body's readout sets, an efference copy, and ADR-0138 says so; a neural form of that side is the maintainers' second step.
- Bad: the drawing inherits the window's fit (ADR-0133): a host that presents its next situation later than one shortest delay after a reward gives the drawing the background alone.
- Bad: under withheld feedback no window reopens, so the drawing names the last rewarded window's units; a control under the drawn delivery must read it so.

## Alternatives considered and why rejected

- **A call for the sources alone** (option 1(b)) **or a flag read by `reward`** (option 1(c)): see above.
- **One refusal for both halves** (option 2(b)): see above.
- **The refusal at the trial's end** (option 3(b)): see above.
- **The composer re-deriving the sources** (option 4(b)): see above.

## Confirmation

- `runtime/cortex-runtime/src/executor.rs`: `AddressError::{NoCritic, NoWindow}`, `Executor::{draws, address_drawn}`, and the tests `the_address_drawn_is_the_units_the_window_counted_onto_the_units_given` and `an_addressing_is_exact_and_refused_whole_outside_the_arena`.
- `runtime/cortex-runtime/src/task.rs`: `Delivery::Drawn`, `Task::check`, `Task::trial`, `TaskError::Address`, and the test `the_drawn_delivery_addresses_the_units_the_critic_counted_onto_the_selected_readout`.
- `runtime/cortex-runtime/tests/instrument/harness.rs`: `Composer::drawn`, `earned_run_delivered`.
- Whitepaper §6.5 (the loop as the runtime composes it) and §9; `CHANGELOG.md`.
