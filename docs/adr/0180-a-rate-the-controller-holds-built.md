---
status: accepted
date: 2026-10-13
depends-on: ADR-0178
title: A rate the controller holds, built
kind: built
summary: ADR-0178's rule is built as `HomeostaticDrivePool::regulate_rate` beside `regulate`, its target the period `Config::rate_target_period_ticks`, zero unset. The image format moved from 21 to 22, and unset every run is the run it was, bit for bit.
decision-makers: VirtualCortex maintainers
---

# ADR-0180: A rate the controller holds, built — ADR-0178's rule as `HomeostaticDrivePool::regulate_rate` beside `regulate`, the window's count over a target the caller names in the estimate's place, with the step, the clamp, the rounding and the bounds the two share; the estimate still read by its slope and kept, the saturation rule not consulted; the target a period in ticks, `Config::rate_target_period_ticks`, at the modulator section's `[56..60)`, zero unset, the executor making the window's count of it for its units and refusing a period no window can be held to; the image format moved from 21 to 22; unset, every run is the run it was, bit for bit, the six whole-image pins that carry a pin written back re-pinned with the format, and the twenty-five read at format 21 alone held by the version written back

## Context and Problem Statement

[ADR-0178](0178-a-rate-the-controller-holds.md) decided the rule and left its placement to brief 072. Its decision:
- *"Once a window, where the estimate's rule moves the gain, the rate's rule moves it by the same form: $g \leftarrow g\,(1 - \kappa\,\operatorname{clamp}(c / c^{*} - 1,\, -1,\, 1))$, with $c$ the window's spikes and $c^{*}$ the target's count for a window."*
- *"The target is a period in ticks, a parameter of the image: a unit's mean interval at the target rate. The executor makes of it the window's count, the units times the window's ticks over the period. Zero is unset."*
- *"Unset, the controller is ADR-0036's, bit for bit."*
- *"Set, the saturation rule is not consulted … The estimate is still computed and kept in the record as a reading; it moves nothing."*

Brief 072's empowerment names the choices: the rule's name and signature and how it stands beside `regulate`; where the target sits in the image, the record being full, and what the loader refuses; how the executor makes the window's count, and what it does where the count would be zero.

What was read first (principle 2), on 2026-10-13:

1. **The record** (`crates/cortex-homeostasis/src/lib.rs`, `HomeostaticDrivePool`). Sixty-four bytes, every one a named field, none reserved. `regulate(saturation_per_bin)` takes the saturation ceiling from its caller and holds no ceiling of its own. A window's spikes are nowhere a field: `is_saturated` reads them as `sum_prev` and `last_activity` together, the pairs' first members and the last bin, which are every closed bin once.
2. **The composition** (`runtime/cortex-runtime/src/executor.rs`, `tally`). On the window's cadence it calls `regulate(saturation_ceiling(units))` and then `step_sleep`. The gain the workers scale by is published from the record before every tick. Nothing outside `cortex-homeostasis` reads the record's `branching_ratio_q16`.
3. **The modulator section** (`runtime/cortex-runtime/src/image.rs`). After [ADR-0155](0155-a-punishment-the-value-does-not-soften-built.md) thirteen bytes are reserved: `[26..28)`, `[39]` and `[54..64)`. The section already carries one period in ticks, the inhibitory rule's at `[20..24)`, and every parameter the last eight formats added. The loader reads the section before it builds the executor and passes what it finds through the configuration, so `Executor::new` refuses an image's parameter by the rule it refuses the configuration's.
4. **The whole-image pins** (`tests/inhibition.rs`, `tests/assembly.rs`). Six images are pinned twice, as they are now and with the header's version written back to the format their hypothesis read ([ADR-0095](0095-an-image-pin-moves-with-its-format.md)'s check). Twenty-five more are pinned once, at format 21, the format they were read at: H-30's image, and the six images of each of H-33's two derivations and of H-34's two. H-37's five steps wrote no image.
5. **A kick above a gain of one.** `tests/criticality.rs` designs a window by kicking distinct units, one spike a kick. At a gain of 1.03 a kick fires its unit twice, so a designed window holds its counts only while the gain is at most 1.0. The estimate's test never met it: its first window lowers the gain.

## Decision Drivers

- ADR-0178's decision and brief 072's standing directives: one mechanism, the count over its target in the estimate's place; no change to the estimate, the saturation rule, the gain's bounds, the step's bound, the window or the bins; unset, bit for bit; no dependency, no float, no `unsafe`.
- **The state crate's invariants** (`CLAUDE.md`): the record stays sixty-four bytes with its size asserted; every operation on a state field saturates or wraps by name; the rule carries a test over the lattice.
- **A structural boundary beats a reviewed one** (principle 5): a period no window can be held to is refused by the configuration and the loader alike, by one function.
- **Every mutant in the changed lines caught** ([ADR-0030](0030-verification-governance.md)): bounds written as ranges and literals where a comparison would leave a mutant no test can tell apart.
- ADR-0095: a pin of a whole image moves with the format in the round that moves it, the rest of the image held to the pin before it.

## Considered Options

1. **How the rule stands beside `regulate`**: (a) a second function, `regulate_rate(target_per_window)`, the two sharing the move and the clearing; (b) `regulate` given what it regulates; (c) the target a field of the record, read by `regulate`.
2. **What the rule takes**: (a) the target as a window's count, the caller's, as the saturation ceiling is; (b) the period and the units.
3. **Where the target sits in the image**: (a) the modulator section's `[56..60)`, a `u32`; (b) a second record of the homeostasis section; (c) bytes carved from the record's own fields; (d) a section of its own.
4. **Unset**: (a) a period of zero; (b) a flag byte beside the period.
5. **What is refused**: (a) a period whose count for the engine's units is no spike a window, or more than a window's bins count; (b) a period outside the inhibitory rule's bounds; (c) nothing.
6. **The count of a period**: (a) to nearest, a half up; (b) truncated.
7. **A target of no spike, or a window of no bin, in the rule**: (a) no reading, the gain left; (b) any spike read as above the target.
8. **The whole-image pins**: (a) the six that carry a pin written back re-pinned as they are now, and the twenty-five read at format 21 alone held through the version written back; (b) all thirty-one re-pinned, each with a second constant.

## Decision Outcome

**Options 1(a), 2(a), 3(a), 4(a), 5(a), 6(a), 7(a) and 8(a).**

- **The rule** (`crates/cortex-homeostasis/src/lib.rs`): `HomeostaticDrivePool::regulate_rate(target_per_window) -> Option<u32>`, beside `regulate`.
  - The window's spikes are `window_spikes()`, `sum_prev` and `last_activity` together; `is_saturated` reads them through it.
  - The count over its target is `rate_over_target_q16(target)`, Q16.16: $(c \cdot 2^{16} + \lfloor c^{*} / 2 \rfloor) / c^{*}$, the quotient to nearest. It is 1.0 for a window at its target.
  - The gain moves by `move_gain(reading)`, which `regulate` now calls with its estimate: the error is the reading less one, clamped to one either way; $\kappa$ times it is rounded to nearest; the gain times one less that is rounded to nearest and clamped to `GAIN_MIN_Q16` and `GAIN_MAX_Q16`. These are the lines `regulate` had, moved into a function the two rules share. So the step, the clamp, the rounding and the bounds are one text.
  - A window at its target leaves the gain; one at twice its target or more lowers it by $\kappa$ whole; a silent one raises it by $\kappa$ whole. The rounding is to nearest with a half going up at each of the three steps, as `regulate`'s is, so a window one spike in 65 536 above its target moves the gain an LSB at the largest step and one as far below does not.
  - **The estimate is a reading.** `regulate_rate` stores `estimate_branching_ratio()` in `branching_ratio_q16` where the window has a slope and leaves the field where it has none. The saturation rule is not consulted: a window at one spike a unit a bin stores no ceiling, and its gain moves by its count.
  - The window is cleared as `regulate` clears it (`clear_window`), and the open bin is the next window's.
  - It returns the gain where a count was held to a target, and nothing where there was none.
  - *Why not `regulate` given what it regulates* (option 1(b)): every call of the tree would change, and the estimate's rule would gain a branch on its own path. With a second function the estimate's rule is one call of `move_gain` where it had the lines, and a test holds it to the lines it had (below).
  - *Why not a field of the record* (option 1(c)): the record is full. ADR-0036 gave the saturation ceiling to the caller for the same reason the target goes there: it is a property of the network's size, which the record does not hold.
- **The target is a count, and the caller's** (option 2(a)). The rule compares a window's spikes with a number of spikes. The units and the window's ticks are the executor's to know, as they are for the saturation ceiling.
- **A target of no spike, or a window of no bin, is no reading** (option 7(a)): the gain stays and the function returns nothing, as `regulate` does without an estimate and `update_branching_ratio` does without ancestors. Reading any spike as above a target of zero (option 7(b)) would make the unset value a rule; the executor never passes zero, and a caller that does is told nothing was measured.
- **The parameter** (`runtime/cortex-runtime/src/executor.rs`): `Config::rate_target_period_ticks: u32`, zero by default, which is unset (option 4(a)). `Executor::rate_target_period_ticks` reads it back and `Executor::rate_target_per_window` the count the engine holds, none while unset.
  - A flag byte (option 4(b)) would say twice what a period of zero says once: no rate has a period of no tick.
- **The count** (`rate_target_of(units, period)`): the units times the window's 131 072 ticks over the period, to nearest from the exact quotient, a half up (option 6(a)). It is the inverse of the rounding by which ADR-0128's rule makes a period of a count. At 58 201 ticks it is 577 at 256 units, 2 306 at 1 024 and 4 612 at 2 048.
- **What is refused** (option 5(a), `ConfigError::RateTargetOutOfRange`): a period whose count is zero, which is a period longer than twice the units times the window; and a count above `RATE_TARGET_MAX`, 536 870 880, the most a window's thirty-two bins count at their cap.
  - Below one spike a window the rule has no target to divide by. Above what a window counts no window can reach its target, and the gain would only rise.
  - The inhibitory rule's bounds (option 5(b)) are that rule's: its depression per spike must fit sixteen bits. Under them a period of a million ticks on one unit would still be no spike a window, so they would not spare the first refusal.
  - The bound's upper end is written as a literal tied to the record's constants by an assertion, and the test as a range pattern, so that neither end leaves a comparison for a mutant to move.
- **The composition** (`tally`): on the window's cadence, `regulate_rate(count)` while the engine holds a count and `regulate(saturation_ceiling(units))` while it holds none. Nothing else of the tick reads the target. The step is `control_step_q0_16` under either rule; at zero the gain stays.
- **The image** (`runtime/cortex-runtime/src/image.rs`):
  - The modulator section's `[56..60)` is the period, a `u32`, little-endian, zero while unset (option 3(a)). It is aligned at a multiple of four, as the section's other words are, and leaves `[54..56)` between it and the critic's window.
  - `[26..28)`, `[39]`, `[54..56)` and `[60..64)` stay reserved, nine bytes, and are refused when not zero.
  - The loader reads the period beside the critic, before it builds the executor, and passes it through the configuration: the image's, set or unset, outranks the configuration's (§8.3). `Executor::new` makes the count of the image's units and refuses the period as it refuses the configuration's (`ImageError::Config(ConfigError::RateTargetOutOfRange)`). So one period is a count on an image of more units and refused on one of fewer.
  - `FORMAT_VERSION` moves from 21 to **22**, with its note in the version history. A version-21 header is refused as every foreign version is (`HeaderError::ForeignVersion(21)`, L-6); its zeros at `[56..60)` read as unset under the new version.
  - *Why not the homeostasis section* (options 3(b) and 3(c)): the section is one record, and the loader holds it to one. A second record would be a second layout for one number. Carving bytes from the record's fields would change the record every pinned run decodes, and the controller's other parameter that is a property of the image and not of the window, the inhibitory rule's period, already sits in the modulator section.
  - *Why not a section of its own* (option 3(d)): a directory entry and a sealed section for four bytes, and a section the loader must require or default.
- **The tests.**
  - `crates/cortex-homeostasis`, by hand: the window's spikes; the count over its target at one, a half, three, none, a third and two thirds, a half of an LSB either way, one above and one below a target of 65 536, the widest window over the least target and the widest, and none for no bin and for no target; the rule at its target at four steps, at twice it and more at two steps, one short of twice, at none, at a half and at half again, one above, one below and two below, the quotient's rounding carried to the gain, a step of zero, the gain at each bound and one short of each, the window cleared and the open bin kept; the estimate stored where there is a slope and kept where there is none, a window the saturation rule reads as saturated moved by its count alone, no bin and no target, and a step above its bound.
  - `crates/cortex-homeostasis`, over the lattice: 40 000 windows of up to thirty-one bins drawn over `U32_LATTICE` and small counts, at gains and steps on and between their bounds, each against a hand rule in `i128` written from this text — exact rounding to nearest by doubling, not the record's shifts — with the target at the count, one either side of it, at half and at twice it, over the lattice and at random; the estimate held to the slope or to what it was, the window cleared, the record well formed.
  - **Unset, a window is regulated as it was**: 20 000 windows over the same lattice under ceilings on both sides of the saturation rule, each regulated by `regulate` and by `regulate_as_built`, the function's body before this ADR kept word for word in the test, and the two records held equal in every field.
  - `executor.rs`: the count at twenty-eight pairs of units and period, the half going up, both refusals at both of their edges, the width and the saturating product; the parameter unset by default and the engine's count.
  - `tests/criticality.rs`: an engine of 64 units with a target of 320 spikes a window over five designed windows — at its target, a decaying window above it, the saturation rule's count, a growing window below it, none — held to a record fed the same counts under `regulate_rate` in every field after each, each gain a number written by hand (65 536, 64 179, 56 157, 57 912, 65 151); beside it a record under the estimate's rule fed the same counts, whose gain moves the other way in the second and the fourth window and whose estimate reads its ceiling in the third where the engine's keeps the slope it had. The count at 256, 1 024 and 2 048 units. A silent engine's gain, a step of zero under a drive, one worker and four. The target through an image written mid-window, the loaded engine regulating as the one that wrote it.
  - `tests/image.rs`: the period written at `[56..60)` and nowhere else, read back set and unset with the image's outranking the configuration's either way, the image written twice; the count of five periods on an image of 192 units; a period a tick past the last that is a count refused whatever the configuration says, and a count on an image of twice the units; nine reserved bytes refused, the period's four bytes each a period; unset, the image equal to the one with its version written back to 21 but for the version and the seal; a version-21 header refused.
  - `crates/cortex-connectome`: the version's assertion reads 22. `tests/differential.rs` and `tests/contention.rs` name the field as unset.
- **Every pinned number unchanged with the target unset, but a whole-image pin with the format** (option 8(a)).
  - At 1 024 units the prior's image is 590 656 bytes on each of the three seeds, and a run's images carry the sections it carries. Two formats' images of one length differ in the header's version and seal by one difference, and a CRC is linear, so one mask takes each image's CRC-64 from one format to the next. A probe on the prior's image, not kept in the tree, read `0xc3258910422b2d6e` from 21 to 22, and from 20 to 21 the mask ADR-0155 read from its runs, `0x411c870fc1e6e4da`. The six numbers below are the pins before under that mask. The calibration of the measurement's round then read them: five by the weekly tests that hold them, and the sixth by its paused test run to its reading ([ADR-0181](0181-a-rate-the-controller-holds-measured.md)).
  - **The six images that carry a pin written back** are re-pinned as they are now, by that mask. The check that holds every other byte of each is the one it had: the image with its header's version written back to the format its hypothesis read.

    | Image | At format 22 | At format 21, the pin before | Held to |
    | :--- | :--- | :--- | :--- |
    | H-17's inhibited image, `REVERSAL_IMAGE_CRC_1024` | `0xc605ef125c047d13` | `0x052066021e2f507d` | `0xd5579c31308388ad` at 15 |
    | H-18's signed image, H-20's, `PUNISHED_IMAGE_CRC_1024` and `H20_IMAGE_CRC` | `0x23e2de661309d5f5` | `0xe0c757765122f89b` | `0x3771636d385191ac` at 16 |
    | H-21's image, `TARGET_IMAGE_CRC_1024` | `0x394d5ce67d0828f2` | `0xfa68d5f63f23059c` | `0xafe7eff2d59da51f` at 18 |
    | H-22's image, `VALUED_IMAGE_CRC_1024` | `0x68d1a39e16a9613e` | `0xabf42a8e54824c50` | `0xbf6797857fda0809` at 19 |
    | H-23's image, H-25's and H-29's, `WINDOWED_IMAGE_CRC_1024` | `0xd27de366b56a7c47` | `0x11586a76f7415129` | `0x5044ed7936a7b5f3` at 20 |
    | The frozen image H-20's arm leaves, `DRAINED_050`'s fifth field | `0x82ec8ea169da899d` | `0x41c907b12bf1a4f3` | `0xd763b4a58364291e` at 17 |

  - **The twenty-five read at format 21 alone keep their numbers.** H-30's image (`WHOLE_IMAGE_CRC_1024`) and the six images of each derivation of H-33 and H-34 (`CRCS_SEEDED`, `CRCS_TWICE`) are held through the image with its header's version written back to 21 and resealed (`with_version`; `WHOLE_READ_FORMAT`, `DERIVED_READ_FORMAT`). A derivation now reads both, the six CRCs as written (`Derivation::crcs`) and the six written back (`crcs_read`), and `derivation_held` holds the second to the round's pins. That is ADR-0095's check with the pin it protects as its only constant: every byte of the image but the version and the seal is the image the round read.
  - Re-pinning all thirty-one with a second constant each (option 8(b)) would move twenty-five numbers that a check can hold where they are.
- **The mutation gate** ([ADR-0030](0030-verification-governance.md)): on the pull request's run at `5caf9a2` `cargo-mutants` made 43 mutants in the changed lines; 39 were caught, 4 were unviable and none was missed. The measurement's ADR records the dispatched sweep's state with its evidence ([ADR-0181](0181-a-rate-the-controller-holds-measured.md)).

### Consequences

- Good: ADR-0178's rule exists in the engine and the image, and is the estimate's rule with one quantity replaced: the two share the text that moves the gain.
- Good: unset, the engine is the one before it by construction and by test. The executor calls `regulate` as it did; `regulate` is held to its previous body over 20 000 windows; the image's four bytes are zero where a format-21 writer left zero.
- Good: the record is the record it was, sixty-four bytes and every field where it stood, so every image's homeostasis section is the bytes it was.
- Good: the refusal is one function for the configuration and the loader, and reads the image's own units.
- Neutral: the format is 22 for four bytes. A format-21 image is refused by its header, and its zeros read as unset under the new version.
- Neutral: under the rate's rule `branching_ratio_q16` holds a window's slope, or the last window's that had one. It never holds the saturation rule's ceiling, so a reader of the field must know which rule is set. Nothing in the engine reads it.
- Bad: one parameter more in the image and the configuration. H-38 read yes; no configuration the tree carries by default sets it yet.
- Bad: the target is a count for the engine's units at its construction. An engine whose arena grew would hold a stale count; no arena grows today.
- Bad: the controller's parameters sit in two sections, the step in the homeostasis record and the target in the modulator section.

## Alternatives considered and why rejected

- **`regulate` given what it regulates** (option 1(b)), **a field of the record** (option 1(c)), **the period and the units as the rule's arguments** (option 2(b)), **the homeostasis section** (options 3(b) and 3(c)), **a section of its own** (option 3(d)), **a flag byte** (option 4(b)), **the inhibitory rule's bounds** (option 5(b)), **any spike above a target of zero** (option 7(b)), **all thirty-one pins re-pinned** (option 8(b)): see above.
- **Nothing refused** (option 5(c)): a target of no spike would reach the rule as a division it refuses every window, and the engine would run with a target set and a controller that never moves.
- **The count truncated** (option 6(b)): at 59 179 ticks, seed 24's period, 1 024 units make 2 267.996 spikes a window, and truncation would write 2 267. The period is itself a count rounded to nearest; its inverse rounds the same way.
- **Another form of the error, an integral term or a second time constant**: ADR-0178 did not decide them, and brief 072 does not empower them.

## Confirmation

- `crates/cortex-homeostasis/src/lib.rs`: `HomeostaticDrivePool::{regulate_rate, rate_over_target_q16, window_spikes}`, `move_gain`, `clear_window`; the tests named above, `regulate_as_built` and `rate_by_hand` among them.
- `runtime/cortex-runtime/src/executor.rs`: `Config::rate_target_period_ticks`, `ConfigError::RateTargetOutOfRange`, `RATE_TARGET_MAX`, `rate_target_of`, `Executor::{rate_target_period_ticks, rate_target_per_window}`, `tally`.
- `runtime/cortex-runtime/src/image.rs`: `RATE_TARGET`, `MODULATOR_RESERVED`, `rate_target_of`.
- `crates/cortex-connectome/src/lib.rs`: `FORMAT_VERSION` 22 and its note; `SECTION_MODULATOR`'s comment.
- `runtime/cortex-runtime/tests/criticality.rs`, `tests/image.rs`; `tests/inhibition.rs` and `tests/assembly.rs`: `REVERSAL_IMAGE_CRC_1024`, `PUNISHED_IMAGE_CRC_1024`, `H20_IMAGE_CRC`, `TARGET_IMAGE_CRC_1024`, `VALUED_IMAGE_CRC_1024`, `WINDOWED_IMAGE_CRC_1024`, `DRAINED_050`, `WHOLE_READ_FORMAT`, `DERIVED_READ_FORMAT`, `Derivation::crcs_read`.
- Whitepaper §5.2.2 (the format's version row), §5.2.14, §5.2.16, §8.7, §8.8, §9; `CHANGELOG.md`; `CLAUDE.md`; the image format 22.
