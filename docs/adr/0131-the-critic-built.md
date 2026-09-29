---
status: proposed
date: 2026-09-29
depends-on: ADR-0130
decision-makers: VirtualCortex maintainers
---

# ADR-0131: The critic built — ADR-0130's value weight placed in the unit record's last reserved bytes, `value_weight`, an `i16` at `[52..54)`; the constants a parameter of the image behind a flag byte at the modulator section's `[48..51)`, a step's shift of at most 30 and a weight's scale of at most 14, refused outside them and without a train; the value, the error and the step a pure rule in `cortex-neuromod` beside the modulator; the features each unit's spikes since the previous reward, counted as the coordinator merges them into the train; `Executor::reward` forming the error from the value, delivering it and moving every weight between ticks while the critic is set, and the reward itself while it is not; the discovery loop's own reward, a unit with spikes pending and a task with a critic of its own each placed; the image format moved from 18 to 19; unset, every run is the run it was, bit for bit, and the four whole-image pins re-pinned with the format under a masked check as ADR-0095 did

## Context and Problem Statement

[ADR-0130](0130-a-critic-of-the-engines-own.md) decided a critic of the engine's own and left its placement to brief 055. Its decision:
- *"Each unit carries a value weight $w_i$, signed, in its record's `[52..54)`, the `_reserved` field today"*, zero in every image written before the critic and in every run without it; the format moves from 18 to 19.
- The features: *"$c_i$, the unit's spikes since the previous reward the engine received, read from the executor's own train."*
- At a reward $r$ with the critic set: the value $V = \sum_i w_i c_i$, the error $\delta = r - V$ delivered to the modulator in the reward's place, and every unit that fired moving its weight by $(\delta \cdot c_i) \gg k$, saturating. Unset, `Executor::reward` is what it is today, bit for bit.
- The constants — the set flag, the step's shift and the weight's scale — a parameter of the image, refused where the rule does not resolve them.
- The rule pure, in a state crate beside the modulator; the executor composing it and writing the weights between ticks (axiom A3).
- The task refusing its own critic while the engine's is set.

Brief 055's empowerment names the choices: the field's name, the weight's width and scale, where the constants sit and how the loader refuses, which state crate holds the rule and how the executor composes it, and how the features are counted within "each unit's spikes since the previous reward". What was read first (principle 2):

1. **`[52..54)` is `_reserved: u16`**, "MUST be zero": written by `encode`, `decode` and `restore_plain_fields`, read by `is_at_rest_image`, and named nowhere else. It is the unit record's last reserved field.
2. **`Executor::reward` is `self.modulator.reward(x)`** and has two callers in the crate: `Task::trial`, and the discovery loop, which delivers the committed inventions' reward to the modulator when it is positive ([ADR-0043](0043-discovery-path.md), [ADR-0052](0052-the-term-arena-in-the-image.md)).
3. **The train** ([ADR-0050](0050-the-train-inside-the-executor.md)) is filled between ticks by `merge_spikes`, which sorts the tick's fired units and pushes each into a ring of `train_capacity`, letting the oldest go when it is full. With no train (`train_capacity` zero) the tick keeps no fired slot and the merge does nothing. A task refuses a train that cannot hold one trial's spikes.
4. **The clock sweep** evicts a unit that is idle, at rest and quiet for the bound; its record goes to the write-ahead log and its slot keeps its id and its last spike stamp; the loader of an image reads an evicted unit's record from the log, and re-hydration restores every plain field from it.
5. **The modulator record has room**: `[48..64)` are reserved since ADR-0123, sixteen bytes.
6. **Four tests pin an image's whole bytes, header included**, as ADR-0123 found them: `REVERSAL_IMAGE_CRC_1024` and `PUNISHED_IMAGE_CRC_1024` in `tests/inhibition.rs`, `H20_IMAGE_CRC` in `tests/assembly.rs`, the second restated, and the fifth field of `DRAINED_050`, the frozen image H-20's arm leaves. The gate tests of H-19, H-20 and H-21 restate the second.

## Decision Drivers

- ADR-0130's decision and brief 055's standing directives: the engine's own inputs only; unset, bit for bit; the weights written between ticks; no dependency, no float, no `unsafe` beyond ADR-0023's invariant.
- **A structural boundary beats a reviewed one** (principle 5): the rule held to an `i128` oracle over the lattice; the constants refused by the configuration and the loader alike; a weight refused at load while no critic is set; the axiom by `&mut self`.
- ADR-0095: a pin of a whole image moves with the format and is re-pinned in the round that moves it, the rest of the image held to the pin before it.

## Considered Options

1. **The weight**: (a) `value_weight: i16`, signed, its scale a parameter; (b) a `u16` offset from a midpoint; (c) Q1.15 as the synaptic weights are.
2. **The features**: (a) counted as the coordinator merges a tick's spikes into the train, one count a unit, zeroed at every reward; (b) the train scanned back from its end at every reward to the previous reward's tick.
3. **The rule's crate**: (a) `cortex-neuromod`, beside the modulator the error goes to; (b) `cortex-core`, beside the record; (c) `cortex-basal-ganglia`, the crate of action selection.
4. **The constants in the image**: (a) `[48]` a flag, `[49]` the shift, `[50]` the scale; (b) the two constants packed into one byte.
5. **The discovery loop's reward** with the critic set: (a) to the modulator itself, as before; (b) through the critic.
6. **A unit with spikes since the previous reward**, quiet and at rest: (a) not evicted while its count is pending; (b) evicted, its slot keeping the weight and re-hydration keeping the slot's; (c) evicted as before.
7. **The task under the engine's critic**: (a) it delivers the outcome's reward and records the error the modulator received and the engine's value; (b) it delivers the outcome's reward and records it.

## Decision Outcome

**Options 1(a), 2(a), 3(a), 4(a), 5(a), 6(a) and 7(a).**

- **The weight** (`crates/cortex-core/src/dynamics/neuron.rs`, `serial.rs`): `value_weight: i16` at `[52..54)`, signed, so that a unit can predict a punishment as well as a reward. `encode`, `decode` and `restore_plain_fields` carry it. `is_at_rest_image` no longer reads the field: the weight is what the critic has learned, a parameter like a synapse's weight, and a unit at rest may carry any; the loader refuses one while the image carries no critic (below). The record has no reserved byte left.
- **The rule** (`crates/cortex-neuromod/src/lib.rs`): `ValueCritic { shift, scale }`, `Clone, Copy, Debug, PartialEq, Eq`, not a record. The widths, written first:
  - a weight is an `i16` whose LSB predicts $2^{-s}$ of a Q16.16 LSB of reward a spike; a count is a `u32`, saturating;
  - `value_q16(pairs)` sums $w_i c_i$ in `i64` — each product within $2^{15} \cdot 2^{32} = 2^{47}$ — saturating, takes the floor over $2^{s}$ by an arithmetic shift and clamps to the `i32` of a Q16.16 reward;
  - `error_q16(reward, value)` is the reward less the value, saturating;
  - `step(weight, error, count)` is the weight plus the floor of $\delta c / 2^{k}$, the product within $2^{31} \cdot 2^{32} = 2^{63}$ in `i64`, clamped to the `i16`. A count of zero moves nothing; a positive error moves a weight up by nothing or more, a negative one down by one LSB or more, the floor's.
  - `is_valid`: a shift of at most `CRITIC_SHIFT_MAX`, 30 — at 31 or more one spike under any error of the `i32` lattice moves a weight by nothing or by the floor's one LSB down, the error's sign and not a step — and a scale of at most `CRITIC_SCALE_MAX`, 14 — at 15 or more one spike at the positive rail predicts less than one LSB. Both bounds are compile-time assertions over the widths. A shift or a scale past its bound handed to the rule directly is taken as the bound.

  `cortex-neuromod` holds the rule because the error it forms is the modulator's input (option 3(a)); the record's field is `cortex-core`'s, and the rule reads it as a number. The crate stays without dependencies.
- **The executor** (`runtime/cortex-runtime/src/executor.rs`):
  - `Config::critic: Option<ValueCritic>`, `None` by default. `Executor::new` refuses constants that are not valid (`ConfigError::CriticOutOfRange`) and a critic with no train (`ConfigError::CriticWithoutTrain`), and allocates one count a unit while it is set, none while it is not. `Executor::critic`, `Executor::features` and `Executor::prediction` read them back.
  - **The features** (option 2(a)): after every tick `merge_spikes` counts each merged spike against its unit, before it pushes the spike into the ring. The count is therefore the unit's spikes since the previous reward whatever the ring has let go since, where a scan of the train (option 2(b)) would miss what the ring let go and would need a refusal for it. The executor's test holds the counts to the train's spikes since the previous reward where the ring holds them, and to the worker's own trace where it does not.
  - **`reward(r)`**: unset, `self.modulator.reward(r)`, as before. Set, the value of the counts under the units' weights, the error $r - V$ into the modulator, every unit's weight moved by the step, every count zeroed, and the value and the error kept as the last `Prediction`. The weights are written through `units_mut`, which `&mut self` makes exclusive: the tick takes `&mut self` too, so no worker holds a reference into the arena while `reward` runs, and every worker waits at the barrier (axiom A3; no new `unsafe`).
  - **The discovery loop's reward** (option 5(a)) goes to `self.modulator.reward` directly: it is the engine's own reward for an invention (ADR-0043), not a reward it receives, and passing it through the critic would take a value from it and cut a host's window between two of its rewards at every invention. Unset, the line is the call it replaces.
  - **The sweep** (option 6(a)) does not evict a unit whose count is not zero while the critic is set. A unit is evicted only when quiet, and it cannot fire while evicted, since a message re-hydrates it after the tick that delivered it; so an evicted unit's count is zero, its weight in the log is its weight, and the critic neither reads nor moves it. Option 6(b) would reach into re-hydration and the image's encoding of evicted units; option 6(c) would read a weight of zero where the unit's weight is in the log. Unset there is no count and the sweep is as it was.
- **The image** (`runtime/cortex-runtime/src/image.rs`; `crates/cortex-connectome/src/lib.rs`):
  - The modulator section's `[48]` is the critic's flag, `CRITIC_SET` (1) while set and zero while unset; `[49]` the shift and `[50]` the scale, zero while unset. `[26..28)`, `[39]` and `[51..64)` stay reserved, 16 bytes.
  - `FORMAT_VERSION` moves from 18 to **19**, with its note in the version history, and the section's doc comment lists the record as it is.
  - The loader reads the constants (`critic_of`) beside the class and the slow current, before it builds the executor, and passes them through the configuration: the image's, set or unset, outranks the configuration's (§8.3). A flag of zero with both bytes zero is unset, so a format-18 record reads as unset under the new version. A flag of one with constants the rule does not resolve is refused as the configuration's are (`ImageError::Config(ConfigError::CriticOutOfRange)`), and a set critic under a configuration with no train as the configuration's is (`ConfigError::CriticWithoutTrain`). Any other flag, or a byte beside a zero flag, is refused as one the writer never produces (`ImageError::ReservedNotZero { section: 42, index: 0 }`).
  - A unit whose record carries a weight while no critic is set is refused (`ImageError::ValueWithoutCritic(unit)`): nothing but the critic writes one.
  - The counts are not in the image: an engine built from an image counts from its load, as the train starts empty at its load (ADR-0059's accepted debt that a learning run is not resumable from an image covers them).
  - A version-18 header is refused as every foreign version is (`HeaderError::ForeignVersion(18)`, L-6).
- **The task** (`runtime/cortex-runtime/src/task.rs`): `TaskError::TwoCritics` refuses a task that carries a `Critic` on an engine whose critic is set, before anything is injected. With the engine's critic set the task delivers the outcome's reward (option 7(a)): `Outcome::reward_q16` is the error the modulator received, as it is under the task's critic, and the new `Outcome::value_q16` is the engine's value, none without the engine's critic or when the reward is withheld. With the engine's critic unset every field is as before.
- **The tests.**
  - `cortex-neuromod`: the constants at their bounds on both sides; the value worked by hand — the floor, a unit that did not fire, the `i64` rail held and not wrapped, the `i32` clamp, a scale past its bound; the error and the step worked by hand — a zero count, the floor on both sides, the count's product, the weight's rails, a shift past its bound; over the lattice, every weight, error and count of `testkit/prop.rs` at every shift against an `i128` oracle by `div_euclid`; and over 20 000 seeded draws the value against an exact `i128` sum floored, and the value of the same features after a step moving with the error's sign or not at all.
  - `cortex-core` (`serial.rs`): the weight's bytes, signed, round-tripped and restored; a unit at rest with any weight.
  - `executor.rs`: the critic unset by default and refused at the edges the rule does not resolve and without a train, and taken at every edge it does; with the critic unset, a reward over the lattice between ticks of a network that fires is the oracle's modulator's bit for bit, with no count and no reading and a weight written by hand never read or moved; with the critic set, in two runs of four units kicked in patterns — one whose ring holds every spike and one whose ring holds two — no weight moves at any tick, and at every reward the counts are the spikes since the previous reward, the reading is the oracle's value and error, the signal moves by the error, every weight moves by the oracle's step and every count starts again; the two runs end alike.
  - `task.rs`: `TwoCritics` before anything is injected, and the task's own critic on an engine without one; at weights of zero a trial under the engine's critic is the trial on an engine without it in every field but the value; the next trial's value is the moved weights times the spikes since the reward, and the error the reward less it; withheld, no value, no error, nothing moved and the spikes pending.
  - `tests/image.rs`: the constants and a weight written and read back, set beside the slow current and unset, the image's outranking the configuration's either way; the counts starting at the load; four constants at the resolved edges read back and four refused; a set critic refused without a train; six flag and byte patterns and two reserved bytes refused; a weight refused while unset, at the unit that carries it; a version-18 header refused. Evict, spike, re-hydrate with the critic set and a reward every fifth round ends in the same image as the run that never evicts, every weight included, with no evicted unit ever carrying a count; and a unit quiet and at rest with a spike pending stays through a sweep and goes at the first after a reward. The earlier tests' reserved-byte probes move from `[48]`, now the critic's flag, to `[51]`; the probe at the record's `[52..54)` reads `ValueWithoutCritic`.
  - `crates/cortex-connectome`: the version's two assertions read 19. `tests/differential.rs` and `tests/contention.rs` name the field as `None`.
- **Every pinned number unchanged with the critic unset, bar the four whole-image pins.** Each is re-pinned as ADR-0095 re-pinned H-17's, read from a run of this round, with every other byte of its image held to the pin before it by the header's version written back to 18 and the header resealed (`with_version`), and by the masked checks the tests already carry:
  - H-17's inhibited image reads `0x50af5c06f491f0fe` at format 18, the pin before this round, and `0xd5579c31308388ad` at 15, `REVERSAL_IMAGE_CRC_FORMAT_15_1024`; its format-19 CRC is `0x11b3db0935771424`, the new `REVERSAL_IMAGE_CRC_1024`.
  - H-18's signed image, H-20's, reads `0xb5486d72bb9c5818` at 18 and `0x3771636d385191ac` at 16, `PUNISHED_IMAGE_CRC_FORMAT_16_1024`; its format-19 CRC is `0xf454ea7d7a7abcc2`, the new `PUNISHED_IMAGE_CRC_1024` and `H20_IMAGE_CRC`, restated in the gate tests of H-19, H-20 and H-21.
  - The frozen image H-20's arm leaves reads `0x14463db5c14f0470` at 18 and ADR-0117's `0xd763b4a58364291e` at 17, `DRAINED_IMAGE_CRC_FORMAT_17`, which `drained_image` asserts; its format-19 CRC is `0x555ababa00a9e0aa`, `DRAINED_050`'s fifth field, read from a run of the arm that reproduced ADR-0110's sequence and sums and held H-20's image to its new CRC.
  - H-21's image, H-20's with the target period written, reads `0xafe7eff2d59da51f` at 18, the CRC ADR-0129 read, and `0xeefb68fd147b41c5` at 19, which brief 055's measurement pins as `TARGET_IMAGE_CRC_1024`.

  Every other whole-domain test reproduces its pinned numbers in the weekly dispatched on brief 055's branch; its run id is in brief 055's measurement ADR.
- **The mutation gate** ([ADR-0030](0030-verification-governance.md)): every mutant `cargo-mutants` makes in the changed lines is caught. The in-diff run's outcome is recorded with the measurement's evidence.

### Consequences

- Good: ADR-0130's critic exists in the rule, the executor and the image, and reads nothing the engine does not have: its features are its own train's spikes and its window its own rewards.
- Good: unset, it is the engine before it bit for bit by construction: `reward`'s first line returns the modulator's, the merge counts into an empty vector, the sweep reads an empty vector, and the image's bytes are zero where a format-18 writer left zero.
- Good: the constants are part of the image and outrank the configuration on load (§8.3). They are not a registry entry: the engine cannot amend them.
- Neutral: the format is 19 for a field and three bytes of the modulator record. A format-18 image is refused by its header, and its zeros read as unset under the new version. The unit record has no reserved byte left.
- Bad: with the critic set a reward walks every unit twice, once for the value and once for the step, and the merge counts every spike once more. Neither cost is measured by this round.
- Bad: a unit that fired since the previous reward is not evicted while the critic is set, so a host that rewards rarely keeps more units resident.
- Bad: the counts are not in the image, so an image written between two rewards loses the window's spikes so far, as it loses the train.

## Alternatives considered and why rejected

- **A `u16` from a midpoint** (option 1(b)) or **Q1.15** (option 1(c)): a signed field reads as every other signed field of the record, and the weight's meaning in reward is the scale's, a parameter, not the field's.
- **Scanning the train** (option 2(b)): exact only while the ring holds every spike since the previous reward, which a host that rewards rarely does not guarantee.
- **`cortex-core`** (option 3(b)): the rule reads the record's field as a number and writes the modulator's input; **`cortex-basal-ganglia`** (option 3(c)): selection is the actor's side, and ADR-0130's critic is not.
- **One packed byte** (option 4(b)): a shift of up to 30 and a scale of up to 14 need five and four bits, and a byte each reads as the class's and the slow current's shifts do.
- **The discovery's reward through the critic** (option 5(b)): see above.
- **Evicted with the weight kept in the slot** (option 6(b)) and **evicted as before** (option 6(c)): see above.
- **Recording the outcome's reward** (option 7(b)): the field says what the modulator received under the task's critic, and the oracle of every learning run reads it so.

## Confirmation

- `crates/cortex-neuromod/src/lib.rs`: `ValueCritic`, `ValueCritic::{is_valid, value_q16, error_q16, step}`, `CRITIC_SHIFT_MAX`, `CRITIC_SCALE_MAX`.
- `crates/cortex-core/src/dynamics/neuron.rs`, `serial.rs`: `value_weight`; `is_at_rest_image`.
- `runtime/cortex-runtime/src/executor.rs`: `Config::critic`, `ConfigError::{CriticOutOfRange, CriticWithoutTrain}`, `Prediction`, `Executor::{critic, features, prediction, reward}`, `merge_spikes`, `sweep`, `induce`.
- `runtime/cortex-runtime/src/image.rs`: `CRITIC_FLAG`, `CRITIC_SHIFT`, `CRITIC_SCALE`, `CRITIC_SET`, `MODULATOR_RESERVED`, `critic_of`, `ImageError::ValueWithoutCritic`.
- `runtime/cortex-runtime/src/task.rs`: `TaskError::TwoCritics`, `Outcome::value_q16`.
- `crates/cortex-connectome/src/lib.rs`: `FORMAT_VERSION` 19 and its note; `SECTION_MODULATOR`'s comment.
- `runtime/cortex-runtime/tests/inhibition.rs` and `tests/assembly.rs`: `REVERSAL_IMAGE_CRC_1024`, `PUNISHED_IMAGE_CRC_1024`, `H20_IMAGE_CRC`, `DRAINED_050`.
- Whitepaper §5.2.1 (the record's table), §5.2.2 (the format's version row), §5.2.14, §8.7, §8.8 (the modulator row), §9; `CHANGELOG.md`; the image format 19.
