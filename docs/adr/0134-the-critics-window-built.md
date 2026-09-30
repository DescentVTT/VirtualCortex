---
status: accepted
date: 2026-09-30
depends-on: ADR-0133
decision-makers: VirtualCortex maintainers
---

# ADR-0134: The critic's window built — ADR-0133's window as `Config::critic_window_ticks`, a length in ticks at the modulator section's `[52..54)`, zero unset; a spike counted, where ADR-0131 counts it as the coordinator merges a tick's spikes, only when its tick is fewer than the length after the tick the window opened at — the previous reward the critic took, or the engine's start, its build or its load, before the first; the length read from the image by `shortest_delay`, the least delay of any slot that holds a synapse, and the loader refusing a window that is not that length and a window without the critic; the image format moved from 19 to 20; unset, every run is the run it was, bit for bit, and the five whole-image pins re-pinned with the format under a masked check

## Context and Problem Statement

[ADR-0133](0133-the-critics-window.md) decided the critic's window and left its placement to brief 056. Its decision:
- *"With the window set, a unit's spike counts toward its feature only when it falls within $W$ ticks after the previous reward the engine received; before the first reward, within $W$ ticks after the engine's start."* Every other part of [ADR-0131](0131-the-critic-built.md)'s critic is kept.
- *"$W$ is the shortest delay of any synapse the image carries, read by a rule written before any run."*
- *"$W$ is a parameter of the image beside ADR-0131's constants. Zero, or unset, means every spike since the previous reward ... The loader refuses a window the rule does not resolve. The image format moves from 19 to 20."*
- *"The count is taken where ADR-0131 takes it, as the coordinator merges a tick's spikes, and the window is a comparison of the tick with the previous reward's."*

Brief 056's empowerment names the choices: the parameter's width and bytes and how the loader refuses; how the executor keeps the previous reward's tick and how the window gates the count; and how the rule reads the shortest delay from the image. What was read first (principle 2):

1. **The count** (`runtime/cortex-runtime/src/executor.rs`): `merge_spikes(now)` runs after a tick's last barrier and before the clock moves, so `self.tick` is the merged spikes' tick. It counts each merged spike against its unit (`features`) before it pushes it into the ring. `reward` zeroes every count while the critic is set.
2. **The reward's tick.** `reward` is an input between ticks. A reward taken with the clock at $T$ comes before tick $T$ runs, so the first spikes after it are tick $T$'s. "Within $W$ ticks after the reward" is therefore the ticks $T$ to $T + W - 1$.
3. **The engine's start.** A new executor's clock is 0. The loader resumes the clock at the image's tick (`resume_clock`, [ADR-0033](0033-tick-duration-in-the-header.md)), and ADR-0131's counts start at the load: *"an engine built from an image counts from its load, as the train starts empty at its load."*
4. **The delays** (`crates/cortex-core/src/dynamics/synapse.rs`, `runtime/cortex-runtime/tests/differential.rs`): a delay is a `u16` per slot; a slot holds a synapse when its target is not `SLOT_EMPTY`; the loader refuses a delay at or beyond the wheel's horizon, 2 560 ticks. The existing test `a_delayed_synapse_arrives_delay_ticks_after_the_spike_and_a_zero_delay_one_the_next_tick` holds that a spike at tick $t$ through a delay $d \ge 1$ is integrated at $t + d$, and through a delay of zero at $t + 1$. So through a synapse of delay $d$ a spike at $t$ can cause another no sooner than tick $t + d$, and within $d$ ticks after a reward no spike of the window caused another.
5. **The modulator section** has fourteen bytes free after ADR-0131: `[26..28)`, `[39]` and `[51..64)`.

## Decision Drivers

- ADR-0133's decision and brief 056's standing directives: the one mechanism an integer comparison on the clock; the window opening at the engine's own reward and its length a rule of the image's own synapses; unset, bit for bit; no dependency, no float, no `unsafe` beyond ADR-0023's invariant.
- **A structural boundary beats a reviewed one** (principle 5): the window refused without the critic by the configuration and the loader alike; its length held by the loader to the rule, so that no image runs with a window its anatomy does not give.
- ADR-0095: a pin of a whole image moves with the format and is re-pinned in the round that moves it, the rest of the image held to the pin before it.

## Considered Options

1. **The parameter**: (a) a `u16` at `[52..54)`, zero unset, the width of a delay; (b) a flag byte and a `u32`; (c) a flag alone, the length computed by the loader.
2. **The opening**: (a) a tick the executor keeps, set at a reward the critic takes, at the build and at the load; (b) the tick of the last `Prediction`.
3. **Where the length's rule lives**: (a) the runtime's image module, `shortest_delay(blocks)`, which the loader and the measurement both call; (b) `cortex-core`, beside the synapse; (c) the tests alone.
4. **What the loader holds the length to**: (a) the rule's reading of the image being loaded; (b) a bound only, below the wheel's horizon.

## Decision Outcome

**Options 1(a), 2(a), 3(a) and 4(a).**

- **The parameter** (`runtime/cortex-runtime/src/executor.rs`): `Config::critic_window_ticks: u16`, zero by default. `Executor::new` refuses a window while the critic is unset (`ConfigError::WindowWithoutCritic`). `Executor::critic_window_ticks` and `Executor::window_opened` read it back. A `u16` is the width of the delay it is read from; zero is unset, as a zero delay resolves no window (below), so no flag byte is needed (option 1(b)). A flag alone (option 1(c)) would leave the image's run to a computation the image does not state.
- **The opening** (option 2(a)): the executor keeps `window_opened`, the tick the window opened at. It is 0 at the build, the image's tick at the load (`resume_clock`), and the clock at every `reward` the critic takes. The discovery loop's own reward goes to the modulator itself (ADR-0131), so it opens no window. The last `Prediction` (option 2(b)) has no tick, and there is none before the first reward.
- **The rule** (`merge_spikes`, `in_window`): the count of ADR-0131 is taken for a merged tick only when the window is unset or `self.tick − window_opened < critic_window_ticks`. The clock only moves forward from an opening, so the difference is the ticks since. The value, the error, the step, the weights written between ticks and every count zeroed at a reward are ADR-0131's. The sweep's rule — a unit with a count pending is not evicted — reads the same counts.
- **The length's rule** (`runtime/cortex-runtime/src/image.rs`, re-exported as `cortex_runtime::shortest_delay`, option 3(a)): `shortest_delay(blocks)` is the least `delays_ticks` over every slot that holds a synapse, whichever unit's chain names its block; none when no slot does. An empty slot's delay is no synapse's. A synapse through the mailbox reads zero, which resolves no window. The rule reads the arena and not the prior, so a network wired by hand, a synthesised one and a learned one are read alike. `cortex-core` (option 3(b)) would put a rule of the image in a state crate that knows no image; the tests alone (option 3(c)) would leave the loader nothing to hold the parameter to.
- **The image**:
  - The modulator section's `[52..54)` is the window, a little-endian `u16`, zero while unset. `[26..28)`, `[39]`, `[51]` and `[54..64)` stay reserved, 14 bytes, and are refused when not zero.
  - The loader reads the window beside the critic, before it builds the executor, and passes it through the configuration: the image's, set or unset, outranks the configuration's (§8.3). A window without the critic is refused as the configuration's is (`ImageError::Config(ConfigError::WindowWithoutCritic)`).
  - After the synapse arena is loaded, a window that is set and not `shortest_delay` of that arena is refused (`ImageError::WindowNotShortestDelay { window, shortest }`, option 4(a)): one tick off on either side, an image with no synapse, and one whose shortest synapse goes through the mailbox. A bound alone (option 4(b)) would let an image carry a window its anatomy does not give, which is ADR-0133's excluded case of a length read from the task.
  - The window opens at the load: an image's run counts from the tick it was written at.
  - `FORMAT_VERSION` moves from 19 to **20**, with its note in the version history, and the section's doc comment lists the record as it is. A version-19 header is refused as every foreign version is (`HeaderError::ForeignVersion(19)`, L-6); its zeros at `[52..54)` read as unset under the new version.
- **The tests.**
  - `executor.rs`: the window unset by default, refused without the critic at every length and taken at every length with it, open at 0 at the build. Four units kicked at distances from 0 to 300 ticks after each reward, and before the first: a first run with the window unset holds its features, readings and weights to an oracle of ADR-0131's rule and reads the spikes' distances from each opening (from 2 to 304 ticks); then a run at every length on both sides of every distance, whose spikes are the first run's tick for tick, holds at every reward the features to the train's spikes fewer ticks after the opening than the length, the reading to the value and the error of those counts, every weight to the step of them, and the opening to the reward's tick. Some length leaves a spike out.
  - `tests/image.rs`: `shortest_delay` over arenas written by hand — no block, no synapse, an empty slot's delay, the least over several blocks, the wheel's last delay, a delay of zero; the window written at `[52..54)` and read back set and unset, the image's outranking the configuration's either way, opening at the load's tick, and the image written twice; five lengths refused as not the shortest delay, zero read as unset; a window without the critic refused; an image with no synapse and one with a synapse through the mailbox refused; three reserved bytes refused; a version-19 header refused. The earlier tests' version assertions read 20 and the previous format's header 19.
  - `crates/cortex-connectome`: the version's two assertions read 20. `tests/differential.rs` and `tests/contention.rs` name the field as zero.
  - The measurement's harness, `earned_run_valued` in `tests/instrument/harness.rs`, applies the window when the engine's is set: its oracle counts only the spikes fewer ticks than the length after the opening, asserts the run starts where the engine did with nothing pending, and holds the engine's opening to the reward's tick at every reward. Unset, it counts as before, so H-22's arms are the runs they were.
- **Every pinned number unchanged with the window unset, bar the five whole-image pins.** Each is re-pinned as ADR-0095 re-pinned H-17's, read from a run of this round, with every other byte of its image held to the pin before it by the header's version written back and resealed (`with_version`):
  - H-17's inhibited image reads `0x11b3db0935771424` at format 19, the pin before this round, and `0xd5579c31308388ad` at 15, `REVERSAL_IMAGE_CRC_FORMAT_15_1024`; its format-20 CRC is `0x443ce10ddfc9b4a7`, the new `REVERSAL_IMAGE_CRC_1024`.
  - H-18's signed image, H-20's, reads `0xf454ea7d7a7abcc2` at 19 and `0x3771636d385191ac` at 16, `PUNISHED_IMAGE_CRC_FORMAT_16_1024`; its format-20 CRC is `0xa1dbd07990c41c41`, the new `PUNISHED_IMAGE_CRC_1024` and `H20_IMAGE_CRC`, restated in the gate tests of H-18 to H-21.
  - H-21's image reads `0xeefb68fd147b41c5` at 19, the CRC H-22 read, and `0xafe7eff2d59da51f` at 18, the CRC H-21 read, now held by a masked check of its own in H-22's arms (`TARGET_IMAGE_CRC_FORMAT_18_1024`); its format-20 CRC is `0xbb7452f9fec5e146`, the new `TARGET_IMAGE_CRC_1024`.
  - H-22's image, H-21's with the critic written, which ADR-0132 dumped and did not pin, reads `0xbf6797857fda0809` at 19, the CRC ADR-0132 read; its format-20 CRC is `0xeae8ad819564a88a`. Brief 056's measurement pins both.
  - The frozen image H-20's arm leaves reads `0x555ababa00a9e0aa` at 19, `0x14463db5c14f0470` at 18 and ADR-0117's `0xd763b4a58364291e` at 17, `DRAINED_IMAGE_CRC_FORMAT_17`, which `drained_image` asserts; its format-20 CRC is `0x00d580beea174029`, `DRAINED_050`'s fifth field, read from a run of the arm that reproduced ADR-0110's sequence and sums and held H-20's image to its new CRC.

  Every other whole-domain test reproduces its pinned numbers in the calibration of brief 056's measurement and in the weekly dispatched on its branch; the measurement's ADR records both.
- **The mutation gate** ([ADR-0030](0030-verification-governance.md)): every mutant `cargo-mutants` makes in the changed lines is caught. The in-diff run's outcome is recorded with the measurement's evidence.

### Consequences

- Good: ADR-0133's window exists in the executor and the image, and reads nothing the engine does not have: it opens at the engine's own reward and its length is its own anatomy's.
- Good: the loader holds the length to the rule, so an image cannot carry a window read from anything else, and a learned image carries the same window, since no rule moves a delay.
- Good: unset, the engine is the one before it bit for bit by construction: `in_window` is true before the comparison is read, the image's bytes are zero where a format-19 writer left zero, and the opening is written and read by nothing else.
- Neutral: the format is 20 for one parameter. A format-19 image is refused by its header, and its zeros read as unset under the new version.
- Bad: the length is a constant of the image, so its fit to a task is the host's: a host that presents its next situation later than one shortest delay after a reward gives the critic the background alone (ADR-0133).
- Bad: a window refused at the load for a wrong length says so only at the load; an engine built from a configuration may carry any length, and the tests that do are the executor's own.

## Alternatives considered and why rejected

- **A flag and a `u32`** (option 1(b)): the length is read from a `u16` delay, and zero is not a length the rule gives.
- **A flag alone, the length computed at the load** (option 1(c)): see above.
- **The tick of the last `Prediction`** (option 2(b)): see above.
- **The rule in `cortex-core`** (option 3(b)) **or in the tests alone** (option 3(c)): see above.
- **A bound only** (option 4(b)): see above.

## Confirmation

- `runtime/cortex-runtime/src/executor.rs`: `Config::critic_window_ticks`, `ConfigError::WindowWithoutCritic`, `Executor::{critic_window_ticks, window_opened, reward, resume_clock}`, `in_window`, `merge_spikes`.
- `runtime/cortex-runtime/src/image.rs`: `CRITIC_WINDOW`, `MODULATOR_RESERVED`, `critic_window_of`, `shortest_delay`, `ImageError::WindowNotShortestDelay`.
- `crates/cortex-connectome/src/lib.rs`: `FORMAT_VERSION` 20 and its note; `SECTION_MODULATOR`'s comment.
- `runtime/cortex-runtime/tests/instrument/harness.rs`: `earned_run_valued`.
- `runtime/cortex-runtime/tests/inhibition.rs` and `tests/assembly.rs`: `REVERSAL_IMAGE_CRC_1024`, `PUNISHED_IMAGE_CRC_1024`, `H20_IMAGE_CRC`, `TARGET_IMAGE_CRC_1024`, `TARGET_IMAGE_CRC_FORMAT_18_1024`, `DRAINED_050`.
- Whitepaper §5.2.2 (the format's version row), §5.2.14, §8.7, §8.8 (the critic's row), §9; `CHANGELOG.md`; the image format 20.
