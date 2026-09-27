---
status: accepted
date: 2026-09-27
depends-on: ADR-0113
decision-makers: VirtualCortex maintainers
---

# ADR-0114: The facilitating class built — ADR-0113's mark placed as bit 2 of a unit's `flags`, `FLAG_FACILITATING`, which `integrate` leaves as it finds it; the class's $U$ and two shifts a parameter of the image behind a flag byte at `[32..36)` of the modulator section, refused outside what the rule resolves, a mark refused while no class is set, and the image format moved from 16 to 17; `step_stp_class` beside `step_stp`, which is untouched and which it is bit for bit at ADR-0019's constants; each worker holding the class from `Executor::new`, so that no word, no allocation and no syscall is added to the tick; unset, every run is the run it was, bit for bit, and the two whole-image pins re-pinned with the format as ADR-0095 did

## Context and Problem Statement

[ADR-0113](0113-a-facilitating-class-of-synapses.md) decided the class and left its placement to brief 049. Its decision:
- *"A bit of `DendriticSuperNeuron::flags`, beside `FLAG_BURST_MODE` and `FLAG_INHIBITORY`. `integrate` sets and clears its own bit by mask and leaves the others."*
- *"The class's $U$ in Q0.8 and its two time constants as shifts, a parameter of the image (whitepaper §8.3), each refused outside what the rule resolves: a shift from 1 to 16, and a $U$ from 1 to 255."* Unset, no class exists and the loader refuses a marked unit; set with no unit marked, nothing reads it.
- *"The class's step sits beside `step_stp`, which is not touched."* At ADR-0019's constants it is `step_stp` bit for bit. The executor calls it for a marked unit where it calls `step_stp` now, *"once per spike, no word, no allocation and no syscall added to the tick."*
- *"`FORMAT_VERSION` moves from 16 to 17. A pin of a whole image moves with the format number and nothing else."*

Brief 049's empowerment names the choices: which bit marks a unit, where the constants sit in the image and how the loader refuses, the step's name and signature, and how the executor selects it. What was read first (principle 2):

1. **`flags` has six free bits.** `FLAG_BURST_MODE` (0x01) is `integrate`'s, set when a plateau begins and cleared when it ends, each by a mask on its own bit (`membrane.rs`). `FLAG_INHIBITORY` (0x02) is written by the synthesis and read by `Polarity::of_flags` at the fan-out. No other code reads or writes `flags`.
2. **The workers are spawned in `Executor::new`, and the loader sets the image's parameters after it.** `Image::decode` builds the executor from the configuration and then reads the modulator record into it: the baseline, the target period, the inhibitory baseline and the signed gate reach the workers through words the coordinator publishes each tick (`Shared::modulation`, `istdp_alpha`). A class that each worker holds as its own field has to be known when the worker is made.
3. **The modulator record has room.** Its `[26..28)` and `[32..64)` are reserved and refused unless zero ([ADR-0094](0094-the-signed-gate-built.md)).
4. **Two tests pin an image's whole bytes, header included.** `REVERSAL_IMAGE_CRC_1024`, H-17's inhibited image, which [ADR-0095](0095-an-image-pin-moves-with-its-format.md) re-pinned at format 16 beside the CRC H-17 read at 15; and `PUNISHED_IMAGE_CRC_1024`, the signed image that H-18, H-19 and H-20 decode, pinned at format 16. No other test pins bytes of a header: the determinism pin hashes the arena's records.
5. **The modulator section's doc comment in `cortex-connectome` was stale.** It described a baseline and 44 reserved bytes, the record of format 11; the target period (ADR-0053), the inhibitory baseline (ADR-0086) and the signed gate (ADR-0094) had filled bytes it called reserved. It is a comment of the tree, and it is corrected here to list the record as it is.

## Decision Drivers

- ADR-0113's decision and brief 049's standing directives: `step_stp` untouched; unset, bit for bit; no rule of the engine changes but the class; no dependency, no float, no `unsafe` beyond ADR-0023's invariant.
- **A structural boundary beats a reviewed one** (principle 5): the class's step held to `step_stp` by a property test over the lattice, the class's validity refused by the configuration and the loader alike, and the mark refused at load while no class is set.
- **The tick reads no new word**: the class is fixed for a run, so each worker can hold it rather than read it from the coordinator.
- ADR-0095: a pin of a whole image moves with the format and is re-pinned in the round that moves it, the rest of the image held to the pin before it.

## Considered Options

1. **The mark**: (a) bit 2 of `flags`, `0x04`; (b) a higher bit.
2. **The class in the image**: (a) the modulator section's `[32..36)`, a flag byte and the three bytes; (b) three bytes at `[32..35)` with $U = 0$ read as unset; (c) a section of its own.
3. **The class in the executor**: (a) a field of each worker, copied from the configuration in `Executor::new`, the loader reading the class before it builds the executor; (b) a word in `Shared`, stored by the loader and read by the turn.
4. **The step**: (a) `step_stp_class(elapsed, class)` beside `step_stp`, a copy of its body with the class's constants; (b) `step_stp` rewritten to call a parameterised step.
5. **A marked unit while no class is set**, inside a running executor (a test can mark one in memory): (a) it steps under ADR-0019's constants, the mark read by nothing; (b) the tick aborts.

## Decision Outcome

**Options 1(a), 2(a), 3(a), 4(a) and 5(a).**

- **The mark** (`crates/cortex-core/src/dynamics/membrane.rs`): `FLAG_FACILITATING`, `0x04`, the next free bit, beside the other two. `integrate` is not touched. It sets and clears `FLAG_BURST_MODE` by mask, so every other bit stands as it was. Two tests hold that: a plateau begun and ended and a spike without one, with the mark, the inhibitory flag, both and every other bit set; and the million-tick walk of the property test, which now runs with every bit but the burst bit set and asserts them unmoved after every tick.
- **The class and the rule** (`crates/cortex-core/src/dynamics/plasticity.rs`):
  - `StpClass { u, tau_f_shift, tau_d_shift }`, three `u8`, `Clone, Copy, Debug, PartialEq, Eq`. It is not a record, since its bytes sit in a record of the runtime's. `StpClass::REFERENCE` is ADR-0019's $(51, 14, 15)$, asserted valid at compile time. `is_valid` is $U$ from 1 to 255 and each shift from 1 to 16:
    - a $U$ of zero never facilitates and never releases;
    - a shift of zero is no time constant;
    - a shift above 16 is one `stp_decay_factor_q16` reads as 16 (ADR-0028), so a class that named it would name a time constant the rule does not run.
  - `DendriticSuperNeuron::step_stp_class(&mut self, elapsed_ticks, class) -> (u8, u8)`: `step_stp`'s body line for line, the class's $U$ and shifts in place of `STP_U`, `STP_TAU_F_SHIFT` and `STP_TAU_D_SHIFT`. `step_stp` is not touched.
  - The module's documentation says which unit steps under which.
- **The executor** (`runtime/cortex-runtime/src/executor.rs`):
  - `Config::stp_class: Option<StpClass>`, `None` by default. `Executor::new` refuses a class that is not valid (`ConfigError::StpClassOutOfRange`) and copies the class into each worker's own `stp_class` field and into the executor's, read back by `Executor::stp_class()`.
  - In the turn, where the spike's step was `u.step_stp(elapsed)`, a `match` on the worker's class with a guard on the unit's mark: `step_stp_class(elapsed, class)` for a marked unit while a class is set, `step_stp(elapsed)` otherwise.
  - Unset, the match's first arm is never taken, and every run is the run it was, bit for bit. The tick gains one comparison per spike, and no word, no allocation and no syscall.
  - A unit marked in memory while no class is set steps under ADR-0019's constants: the mark is read only with a class, and no image can carry that state (below).
- **The image** (`runtime/cortex-runtime/src/image.rs`; `crates/cortex-connectome/src/lib.rs`):
  - The modulator section's `[32]` is the class's flag, `STP_CLASS_SET` (1) while set and zero while unset, and `[33]`, `[34]` and `[35]` are its $U$, $\tau_f$ shift and $\tau_d$ shift, zero while unset. `[26..28)` and `[36..64)` stay reserved, 30 bytes.
  - `FORMAT_VERSION` moves from 16 to **17**, with its note in the version history, and the section's doc comment now lists the record as it is (Context, item 5).
  - The loader reads the class (`stp_class_of`) before it builds the executor, with the record count's check moved up beside it, and passes it through the configuration: the image's class, set or unset, outranks the configuration's (§8.3).
  - A flag of zero with the three bytes zero is unset: a format-16 record's bytes read as unset under the new version.
  - A flag of one with a class the rule does not resolve is refused as the configuration's is (`ImageError::Config(ConfigError::StpClassOutOfRange)`). Any other flag, or a byte beside a zero flag, is refused as one the writer never produces (`ImageError::ReservedNotZero { section: 42, index: 0 }`).
  - A unit whose record carries the mark while no class is set is refused (`ImageError::MarkWithoutClass(unit)`).
  - A version-16 header is refused as every foreign version is (`HeaderError::ForeignVersion(16)`, L-6).
- **The tests.**
  - `cortex-core` (`plasticity.rs`):
    - `a_class_is_valid_from_one_to_255_and_its_shifts_from_one_to_sixteen`: the edges on both sides of each constant, and `REFERENCE`.
    - `the_class_s_step_at_its_edges`, each pair and state computed by hand from the rule: a first spike (`u32::MAX`) at $U$ of 1, 26, 51 and 255; an empty pool with no time elapsed; shifts of 1 and 16 over one and two ticks and over $2^{16}$; and each time constant its own field's, with one shift at 1 and the other at 16.
    - Over the lattice of `testkit/prop.rs` and a seeded walk of 2 000 trains of a hundred spikes, `the_class_s_step_at_adr_0019_s_constants_is_step_stp_bit_for_bit`: the pair returned and the two fields left.
    - Over every pairing of $U$ in {1, 2, 26, 51, 254, 255} and shifts in {1, 2, 13, 15, 16}, the lattice of factors and intervals, and 400 000 seeded draws of a class, a state and an interval, `the_class_s_step_is_tsodyks_markram_at_any_constants_by_an_i64_oracle`. The oracle is written from the rule rather than from the step: each factor's deviation from its rest survives by the factor, rounded, at least one LSB smaller once time has passed; then facilitation, release and depletion. Its decay factor is the form before ADR-0062, which the existing property test holds to the present one.
  - `cortex-core` (`membrane.rs`): the two tests above on the flags.
  - `executor.rs`: `a_marked_unit_steps_under_the_class_and_an_unmarked_one_under_adr_0019_s_constants`:
    - the default is unset; `new` refuses five classes at the edges it does not resolve and takes three at the edges it does;
    - in one run under ADR-0113's set (ii), with unit 0 marked and both units fired in eight rounds 250 to 70 000 ticks long, the marked unit's factors are the class's step and the unmarked unit's `step_stp`, each on a copy from rest, spike by spike, with the intervals the executor's own train read;
    - each is shown to differ from the other step, so the test tells;
    - with no class, both units step as `step_stp` does.
  - `tests/image.rs`: `the_class_of_short_term_plasticity_is_written_to_and_read_from_the_image_and_a_record_left_zero_reads_as_unset`:
    - unset writes zeros, and reads as unset under a configuration that says set;
    - set, the flag and the three bytes are written beside the inhibitory baseline and the signed gate, and a unit's mark is written in its record; they read back under a configuration that says unset, and the image encodes again byte for byte;
    - five classes at the resolved edges read back;
    - seven out of range are refused as the configuration's are, and seven flag and byte patterns as bytes the writer never produces;
    - the reserved bytes 26, 27, 36 and 63 are refused;
    - a mark on unit 1 and on unit 0 while no class is set is refused at that unit, whatever the configuration says, and the inhibitory flag alone is no mark;
    - a version-16 header is refused.

    The two earlier tests' reserved-byte probes move from `[32]`, now the class's flag, to `[36]`. The inhibitory baseline's test reads the previous version as 16, and the signed gate's test the format as 17.
  - `crates/cortex-connectome`: the version's two assertions read 17. `tests/differential.rs` and `tests/contention.rs` name the field as `None`.
- **Every pinned number unchanged with no class set, bar the two whole-image pins.**
  - The workspace's tests in the debug profile pass: 648 passed and 71 ignored, where `main` read 641 and 71. The seven new ones are the tests above.
  - The determinism pin of [ADR-0030](0030-verification-governance.md) is unmoved.
  - Two pinned numbers move with the format and with nothing else. Each is re-pinned as ADR-0095 re-pinned H-17's, with every other byte of its image held to the pin before it.
  - The images the arms decode were built on this branch, at format 17, and read with the header's version written back and the header resealed (`with_version`):
    - H-17's inhibited image at 15 reads `0xd5579c31308388ad` and at 16 reads `0xd2965219775c394a`, the two CRCs pinned before. Its format-17 CRC is `0x938ad516b6badd90`, the new `REVERSAL_IMAGE_CRC_1024`. Each H-17 arm already asserts the format-15 CRC.
    - H-18's signed image at 16 reads `0x3771636d385191ac`, the CRC H-18, H-19 and H-20 read. Its format-17 CRC is `0x766de462f9b77576`, the new `PUNISHED_IMAGE_CRC_1024`. `PUNISHED_IMAGE_CRC_FORMAT_16_1024` holds the format-16 CRC, and each arm of H-18, H-19 and H-20 now asserts it on its image written back to 16. The gate tests of H-19 and H-20 name both.
  - So the claim that nothing but the header moved is held in the weekly job, not only here. The weekly dispatched on this round's branch runs every arm; its run id is in brief 049's measurement ADR.
- **The mutation gate** ([ADR-0030](0030-verification-governance.md)): every mutant `cargo-mutants` makes in the changed lines is caught. The in-diff run's outcome is recorded with the measurement's evidence.

### Consequences

- Good: ADR-0113's class exists in the rule, the executor and the image. Unset, it is the rule before it bit for bit by construction: `step_stp` is not touched, the turn's first arm is never taken, and the image's bytes are zero where a format-16 writer left zero.
- Good: the tick reads no new word. The class is each worker's own field, so a marked unit's step costs the comparison that selects it and nothing the coordinator publishes.
- Good: the class is part of the image and outranks the configuration on load, so a run under it is defined by its image (§8.3). It is not a registry entry: the engine cannot amend it (F-37's reading holds, since the registry admits no plasticity parameter).
- Neutral: the format is 17 for a flag, three bytes and a bit of `flags`. A format-16 image is refused by its header, and its record's zeros read as unset under the new version.
- Neutral: the class can be set only when an executor is made, by its configuration or its image. A class set after `new` would need a word the tick reads, which is what this placement avoids.
- Bad: `step_stp_class` repeats `step_stp`'s body, so a later change to the rule has two copies to change. The property test that holds them equal at ADR-0019's constants is what keeps them one rule.
- Bad: the loader refuses a malformed modulator record count before it reads the neuron and synapse sections, where it refused it after. An image with both faults now names the modulator's first. No test depended on the order.

## Alternatives considered and why rejected

- **A higher bit** (option 1(b)): no reason to leave a hole at `0x04`.
- **$U = 0$ as unset** (option 2(b)): one byte fewer, but "unset" would then be a value of a field rather than a flag. The inhibitory baseline (ADR-0086) and the signed gate (ADR-0094) each have a flag byte, and the loader's refusals read the same way for all three.
- **A section of its own** (option 2(c)): a new section kind, directory entry and CRC for four bytes, where the modulator section already holds the image's plasticity parameters and has room.
- **A word in `Shared`** (option 3(b)): the turn would read an atomic per spike for a value fixed for the run, and ADR-0113 asked for no word added to the tick.
- **`step_stp` calling a parameterised step** (option 4(b)): it would change `step_stp`'s code, which ADR-0113 and the brief keep untouched. "Unset, bit for bit" would then rest on reading that change rather than on the test that holds two rules equal.
- **An abort** (option 5(b)): the loader already refuses an image with a mark and no class. A unit marked in memory is a test's own state, and stepping it as unmarked keeps "unset" meaning "every unit under ADR-0019's constants" without a branch in the tick that can end the process.

## Confirmation

- `crates/cortex-core/src/dynamics/membrane.rs`: `FLAG_FACILITATING`; `integrate` unchanged.
- `crates/cortex-core/src/dynamics/plasticity.rs`: `StpClass`, `StpClass::{REFERENCE, is_valid}`, `DendriticSuperNeuron::step_stp_class`; `step_stp` unchanged.
- `runtime/cortex-runtime/src/executor.rs`: `Config::stp_class`, `ConfigError::StpClassOutOfRange`, `Executor::stp_class`, the workers' `stp_class`, the selection in the turn.
- `runtime/cortex-runtime/src/image.rs`: `STP_CLASS_FLAG`, `STP_CLASS_U`, `STP_CLASS_TAU_F`, `STP_CLASS_TAU_D`, `STP_CLASS_SET`, `stp_class_of`, `ImageError::MarkWithoutClass`, the writer's lines and the loader's.
- `crates/cortex-connectome/src/lib.rs`: `FORMAT_VERSION` 17 and its note; `SECTION_MODULATOR`'s comment.
- `runtime/cortex-runtime/tests/inhibition.rs`: `REVERSAL_IMAGE_CRC_1024`, `PUNISHED_IMAGE_CRC_1024`, `PUNISHED_IMAGE_CRC_FORMAT_16_1024` and the arms' assertions.
- The tests named above; the mutation gate on the changed lines.
- Whitepaper §5.2.1 (the record's `flags`, the public API and short-term plasticity), §5.2.2 (the format's public API and version row), §8.7 (the container's version and the modulator section's line), §8.8 (the short-term plasticity row), §9; `CHANGELOG.md`; the image format 17.
