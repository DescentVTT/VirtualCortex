---
status: accepted
date: 2026-10-09
depends-on: ADR-0154
decision-makers: VirtualCortex maintainers
---

# ADR-0155: A punishment the value does not soften, built — ADR-0154's rule as `ValueCritic::received_q16` beside the critic's error, the parameter as `Config::whole_punishment`, a flag byte at the modulator section's `[51]`; set, a reward below zero that meets a value below zero reaches the modulator whole and the critic's weights move by the error as they did; the engine's reading names the error and what the modulator received, and a trial records the second; refused without the critic by the configuration and the loader; the image format moved from 20 to 21; unset, every run is the run it was, bit for bit, and the six whole-image pins re-pinned with the format under a masked check

## Context and Problem Statement

[ADR-0154](0154-a-punishment-the-value-does-not-soften.md) decided the rule and left its placement to brief 063. Its decision:
- *"the critic's error is $r - V$, and it moves the weights, as it does now;"*
- *"the modulator receives $r - V$, unless $r$ and $V$ are both below zero; then it receives $r$."*
- *"The rule is `cortex-neuromod`'s, beside `ValueCritic::error_q16`. The executor's reward path calls it. The parameter is a flag in the modulator section's reserved bytes, so the image's format goes to 21. It is refused without the critic."*

Brief 063's empowerment names the choices: the rule's name and signature, and whether the prediction's record gains a field for what the modulator received; which reserved byte holds the flag, and the parameter's name; and how the harness's oracles replay a reward whose delivered error is not the critic's.

What was read first (principle 2), on 2026-10-09:

1. **The reward path** (`runtime/cortex-runtime/src/executor.rs`, `Executor::reward`). With the critic set it forms the value, takes `ValueCritic::error_q16(reward, value)`, moves every counted unit's weight by `critic.step` of the error, records `Prediction { value_q16, error_q16 }`, opens the window again and hands the error to `NeuromodulatorState::reward`. One number, the error, does three things there: it moves the weights, it is recorded, and it is what the modulator receives.
2. **The task's record** (`runtime/cortex-runtime/src/task.rs`, `Task::trial`). Under the engine's critic a trial's `Outcome::reward_q16` is the reading's `error_q16`, documented as *"the prediction error the modulator received"*. Every learning harness since H-22 reads the delivered reward from that field.
3. **The modulator section** (`runtime/cortex-runtime/src/image.rs`). After [ADR-0134](0134-the-critics-window-built.md) fourteen bytes are reserved: `[26..28)`, `[39]`, `[51]` and `[54..64)`. `[51]` lies between the critic's constants at `[48..51)` and its window at `[52..54)`. Every flag of the section is a byte of its own, zero unset and one set.
4. **The harness's oracles** (`tests/inhibition.rs`, `answered_run`; `tests/instrument/harness.rs`, `value_step`). The critic's oracle returns the value, the error and the weights after, and the run holds the trial's recorded reward to the error. The network's oracle (`Network::replay_each`) replays every excitatory synapse under the signal it reads from the record after each trial, so it needs no notion of what was delivered. Nothing in the run held the record's signal to what the modulator was said to receive.

## Decision Drivers

- ADR-0154's decision and brief 063's standing directives: the one mechanism is what the modulator receives at a reward below zero under a value below zero; unset, bit for bit; set, nothing changes where the value is at or above zero, and the weights move by the error in every case; no dependency, no float, no `unsafe` beyond ADR-0023's invariant.
- **A structural boundary beats a reviewed one** (principle 5): the parameter refused without the critic by the configuration and the loader alike; the critic's error and what the modulator received two fields of one reading, so that neither can be read as the other.
- **Every mutant in the changed lines caught** ([ADR-0030](0030-verification-governance.md)): a rule whose comparisons each have a case that fails when it moves.
- ADR-0095: a pin of a whole image moves with the format and is re-pinned in the round that moves it, the rest of the image held to the pin before it.

## Considered Options

1. **The rule's form in code**: (a) the error against the value's part at or above zero wherever the reward is below zero, one comparison, on the reward; (b) two comparisons, the reward's and the value's, as ADR-0154's sentence reads; (c) a method of the critic that takes the flag.
2. **Where the parameter lives**: (a) a flag of the configuration and of the image, read by the executor's `reward`; (b) a third field of `ValueCritic`.
3. **The reading's record**: (a) `Prediction` gains `received_q16` beside `error_q16`; (b) `error_q16` becomes what the modulator received.
4. **The flag's byte**: (a) `[51]`, between the critic's constants and its window; (b) the first byte of the tail, `[54]`; (c) a second value of the critic's own flag at `[48]`.
5. **The harness**: (a) the run holds what the modulator received to a hand rule written from ADR-0154's text, and the record's signal to the signal the last reward left, decayed over the trial, plus it; (b) the run reads the engine's reading and holds nothing more.

## Decision Outcome

**Options 1(a), 2(a), 3(a), 4(a) and 5(a).**

- **The rule** (`crates/cortex-neuromod/src/lib.rs`): `ValueCritic::received_q16(reward_q16, value_q16)`, beside `error_q16`. It is `error_q16(reward, max(value, 0))` where the reward is below zero and `error_q16(reward, value)` everywhere else. Where the reward and the value are both below zero that is the reward less zero, the reward; where the reward is below zero and the value is not, it is the reward less the value; and the saturation is `error_q16`'s. So it is ADR-0154's rule number for number, and it compares the reward alone.
  - *Why not two comparisons* (option 1(b)): at a value of exactly zero the reward less the value is the reward, so `value < 0` and `value <= 0` give one result everywhere, and no test can tell them apart. The form taken has no comparison on the value to move.
  - *Why not a method that takes the flag* (option 1(c)): the flag is the image's and the executor's, as the critic's window is. The rule is a pure function of two numbers.
- **The parameter** (`runtime/cortex-runtime/src/executor.rs`): `Config::whole_punishment: bool`, unset by default. `Executor::new` refuses it while the critic is unset (`ConfigError::WholePunishmentWithoutCritic`); `Executor::whole_punishment` reads it back. A third field of `ValueCritic` (option 2(b)) would enter every literal of the critic's constants across the tree and its validity rule, for a flag the rule's arithmetic does not read.
- **The reward path** (`Executor::reward`): the value, the error and the step of every counted weight by the error are ADR-0131's, untouched. What goes to `NeuromodulatorState::reward` is the error while the parameter is unset and `received_q16(reward, value)` while it is set. With the critic unset `reward` is the modulator's own, as before ADR-0131.
- **The reading** (option 3(a)): `Prediction { value_q16, error_q16, received_q16 }`. `error_q16` stays the critic's error, what the weights moved by; `received_q16` is what the modulator received, the error while the parameter is unset. Redefining `error_q16` (option 3(b)) would leave the critic's own error unread on the engine, where clause 4 of every hypothesis since H-23 and the harness's oracle of the weights stand on it.
- **The trial's record** (`Task::trial`): under the engine's critic `Outcome::reward_q16` is the reading's `received_q16`. Unset it is the error, the number it was.
- **The image** (`runtime/cortex-runtime/src/image.rs`):
  - The modulator section's `[51]` is the flag, `1` while set and zero while unset (option 4(a)). It sits beside the critic whose reading it is a rule of, and is a byte of its own as every flag of the section is. A second value of the critic's flag (option 4(c)) would make one byte mean two parameters; the tail's first byte (option 4(b)) would separate the flag from the critic by the window.
  - `[26..28)`, `[39]` and `[54..64)` stay reserved, 13 bytes, and are refused when not zero. A flag that is neither zero nor one is refused as a byte the writer never produces.
  - The loader reads the flag beside the critic and its window, before it builds the executor, and passes it through the configuration: the image's, set or unset, outranks the configuration's (§8.3). A flag without the critic is refused as the configuration's is (`ImageError::Config(ConfigError::WholePunishmentWithoutCritic)`).
  - `FORMAT_VERSION` moves from 20 to **21**, with its note in the version history, and the section's doc comment lists the record as it is. A version-20 header is refused as every foreign version is (`HeaderError::ForeignVersion(20)`, L-6); its zero at `[51]` reads as unset under the new version.
- **The harness** (option 5(a), `tests/inhibition.rs`):
  - `received_by_hand(whole, reward, value, error)` is ADR-0154's sentence with its two comparisons: the error, unless the engine carries the parameter and the reward and the value are both below zero; then the reward. `answered_run` holds every trial's recorded reward to it, over the oracle's value and error, and holds the engine's reading to the oracle's value and error.
  - `answered_run` also holds the record's signal after every reward to the signal the last reward left, decayed over the trial's ticks by `signal_course`, plus what the hand rule says the modulator received. That is the first place a run holds the modulator's reception by a second writing; H-29's arms hold it too, and no pinned number of theirs moves.
  - The network's oracle is unchanged: it reads the signal from the record.
- **The tests.**
  - `crates/cortex-neuromod`: the rule by hand at the four cases by sign, at zero on either side, one LSB below zero on each side, and at the width and beyond it; and over the lattice's pairs and 100 000 seeded pairs against a hand rule in `i64` written from ADR-0154's text with its two comparisons, with the critic's error held to the rule it was, the result never above the error, and equal to it exactly where either number is at or above zero.
  - `executor.rs`: the parameter unset by default, taken with the critic with or without its window, and refused without the critic. Then two engines of four armed units, one unset and one set, given the same kicks, the same weights written by hand before each window and the same rewards over thirteen cases — the four by sign, a reward of zero, a value of zero, one LSB below zero on each side, and the width on both sides. At every reward of both the value, the error and what the modulator received are an oracle's; each engine's modulator is an oracle's, decayed tick by tick and given what the rule says; the reading records all three; and every weight moves by the error on both engines alike. Three cases meet both below zero, and the two engines' signals part at the first.
  - `task.rs`: the same trials on two engines that carry the critic, unset and set. A first tie is punished against a value of zero and the two trials are one. The same stimulus then ties again against a value below zero: unset the trial records the error, set it records the reward, the two signals part by exactly the value, and the weights move alike. A rewarded trial against a value below zero then records the error on both.
  - `tests/image.rs`: the flag written at `[51]` and nowhere else, read back set and unset with the image's outranking the configuration's either way, beside the critic and its window and beside the critic alone, and the image written twice; four other flags refused; the flag without the critic refused whatever the configuration says; six reserved bytes refused; a version-20 header refused. The earlier tests' loops over reserved bytes name `[54]` where they named `[51]`, and their version assertions read 21 and the previous format's header 20.
  - `crates/cortex-connectome`: the version's two assertions read 21. `tests/differential.rs` and `tests/contention.rs` name the field as unset.
- **Every pinned number unchanged with the parameter unset, bar the six whole-image pins.** Each is re-pinned as ADR-0095 re-pinned H-17's, read from a run of this round, with every other byte of its image held to the pin before it by the header's version written back and resealed (`with_version`). The six images have one length, and the format's change moves the header's version and seal by one difference whatever else the header holds, so one mask, `0x411c870fc1e6e4da`, takes each from its format-20 CRC to its format-21 one, as a CRC's linearity says it must.

  | Image | At format 21 | At format 20, the pin before | Held to |
  | :--- | :--- | :--- | :--- |
  | H-17's inhibited image, `REVERSAL_IMAGE_CRC_1024` | `0x052066021e2f507d` | `0x443ce10ddfc9b4a7` | `0xd5579c31308388ad` at 15, the CRC H-17 read |
  | H-18's signed image, H-20's, `PUNISHED_IMAGE_CRC_1024` and `H20_IMAGE_CRC` | `0xe0c757765122f89b` | `0xa1dbd07990c41c41` | `0x3771636d385191ac` at 16, the CRC H-18 to H-20 read |
  | H-21's image, `TARGET_IMAGE_CRC_1024` | `0xfa68d5f63f23059c` | `0xbb7452f9fec5e146` | `0xafe7eff2d59da51f` at 18, the CRC H-21 read |
  | H-22's image, `VALUED_IMAGE_CRC_1024` | `0xabf42a8e54824c50` | `0xeae8ad819564a88a` | `0xbf6797857fda0809` at 19, the CRC H-22 read |
  | H-23's image, H-25's and H-29's, `WINDOWED_IMAGE_CRC_1024` | `0x11586a76f7415129` | `0x5044ed7936a7b5f3` | the same, now `WINDOWED_IMAGE_CRC_FORMAT_20_1024`, which H-29's and H-30's arms hold |
  | The frozen image H-20's arm leaves, `DRAINED_050`'s fifth field | `0x41c907b12bf1a4f3` | `0x00d580beea174029` | `0xd763b4a58364291e` at 17, `DRAINED_IMAGE_CRC_FORMAT_17` |

  The first five were read from one build of the images, each also at formats 20, 19, 18, 16 and 15, where every earlier pin was reproduced. The sixth was read from a run of H-20's arm in `tests/assembly.rs` that reproduced ADR-0110's sequence and sums and held the image to its format-17 CRC; its test is paused outside the weekly ([ADR-0127](0127-the-paused-line-leaves-the-weekly.md)), so no dispatch reproduces it. Every other whole-domain test reproduces its pinned numbers in the calibration of brief 063's measurement and in the weekly dispatched on its branch; the measurement's ADR records both.
- **The mutation gate** ([ADR-0030](0030-verification-governance.md)): every mutant `cargo-mutants` makes in the changed lines is caught, 19 of 27, the other eight unviable. The in-diff run's outcome is recorded with the measurement's evidence ([ADR-0156](0156-a-punishment-the-value-does-not-soften-measured.md)).

### Consequences

- Good: ADR-0154's rule exists in the engine and the image, reads nothing the engine does not have, and is one function of two numbers beside the rule it departs from.
- Good: unset, the engine is the one before it bit for bit by construction: the modulator is handed the error itself, the image's byte is zero where a format-20 writer left zero, and the new field of the reading holds the error.
- Good: the critic stays a predictor. Its weights move by its own error in every case, so the value holds a stimulus's expected reward below zero as above it.
- Good: a run now holds what the modulator received by a second writing, and the signal by its course from it, at every trial.
- Neutral: the format is 21 for one byte. A format-20 image is refused by its header, and its zero reads as unset under the new version.
- Bad: one parameter more in the image and the configuration, and a rule that stays in the tree unset if H-30 reads no.
- Bad: the reading has two numbers where it had one. A reader that wants the delivered reward must take `received_q16`; `Task::trial` does, and the type does not stop another reader from taking the error.

## Alternatives considered and why rejected

- **Two comparisons** (option 1(b)), **a method that takes the flag** (option 1(c)), **a field of the critic** (option 2(b)), **`error_q16` redefined** (option 3(b)), **another byte** (options 4(b) and 4(c)): see above.
- **A harness that reads the engine's reading alone** (option 5(b)): the run would hold the engine to itself. The rule would then be checked only where it is written.
- **A floor at another level, or a rule for rewards at or above zero**: ADR-0154 rejected them, and brief 063 does not empower them.

## Confirmation

- `crates/cortex-neuromod/src/lib.rs`: `ValueCritic::received_q16` and its two tests.
- `runtime/cortex-runtime/src/executor.rs`: `Config::whole_punishment`, `ConfigError::WholePunishmentWithoutCritic`, `Prediction::received_q16`, `Executor::{whole_punishment, reward}`.
- `runtime/cortex-runtime/src/task.rs`: `Task::trial`, `Outcome::reward_q16`.
- `runtime/cortex-runtime/src/image.rs`: `WHOLE_PUNISHMENT_FLAG`, `WHOLE_PUNISHMENT_SET`, `MODULATOR_RESERVED`, `whole_punishment_of`.
- `crates/cortex-connectome/src/lib.rs`: `FORMAT_VERSION` 21 and its note; `SECTION_MODULATOR`'s comment.
- `runtime/cortex-runtime/tests/inhibition.rs` and `tests/assembly.rs`: `received_by_hand`, `answered_run`, `REVERSAL_IMAGE_CRC_1024`, `PUNISHED_IMAGE_CRC_1024`, `H20_IMAGE_CRC`, `TARGET_IMAGE_CRC_1024`, `VALUED_IMAGE_CRC_1024`, `WINDOWED_IMAGE_CRC_1024`, `WINDOWED_IMAGE_CRC_FORMAT_20_1024`, `DRAINED_050`.
- Whitepaper §5.2.2 (the format's version row), §5.2.14, §6.5, §8.7, §8.8 (the critic's row), §9; `CHANGELOG.md`; `CLAUDE.md`; the image format 21.
