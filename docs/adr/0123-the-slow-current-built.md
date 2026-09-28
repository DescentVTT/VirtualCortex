---
status: proposed
date: 2026-09-28
depends-on: ADR-0122
decision-makers: VirtualCortex maintainers
---

# ADR-0123: The slow current built — ADR-0122's mark placed as bit 3 of a unit's `flags`, `FLAG_SLOW`, which `integrate` and its sibling leave as they find it; the slow potential in the record's `[20..24)`, `v_slow`, reserved since format 4, zero in every unmarked unit and refused in an image otherwise; the constants a parameter of the image behind a flag byte at `[36..48)` of the modulator section, refused outside what the rule resolves, a mark refused while none is set, and the image format moved from 17 to 18; `integrate_slow` beside `integrate`, which is untouched and which it is bit for bit with no slow potential and no slow input; each worker holding the constants from `Executor::new`, an unmarked unit's turn gaining the test of its mark and nothing else; unset, every run is the run it was, bit for bit, and the four whole-image pins re-pinned with the format under a masked check as ADR-0095 did

## Context and Problem Statement

[ADR-0122](0122-a-slow-voltage-gated-current.md) decided the slow current and left its placement to brief 052. Its decision:
- *"A bit of `DendriticSuperNeuron::flags`, beside bits 0 to 2, which `integrate` and its sibling leave as they find them."*
- *"The slow potential $s$, Q16.16, in the record's `[20..24)`. It stays zero in every unmarked unit, so an image with no marked unit is the image it was."*
- The constants $k_s$, $g$, $V_{lo} < V_{hi}$ *"a parameter of the image in the modulator section's reserved bytes"*, each refused outside what the rule resolves; unset, a marked unit is refused.
- The input: for a marked unit, *"the positive efficacies of the synaptic messages landing in the basal compartment … scaled by the gain"*; an unmarked unit's turn adds nothing.
- The rule beside `integrate`: $s \leftarrow \operatorname{leak}(s, k_s) + \text{input} \cdot 2^{-g}$, in the refractory window too; the gate $\gamma(v) = \operatorname{clamp}((v - V_{lo})/(V_{hi} - V_{lo}), 0, 1)$ on the soma before this tick's update; the soma receives $\gamma(v) \cdot s \cdot 2^{-4}$, only while $s > 0$.
- *"`FORMAT_VERSION` moves from 17 to 18. A pin of a whole image moves with the format number and nothing else, restated under a masked check as ADR-0095 did."*

Brief 052's empowerment names the choices: which bit marks a unit, the field's name, where the constants sit and how the loader refuses, the rule's name and signature, and how the executor selects it. What was read first (principle 2):

1. **`flags` has five free bits.** Bit 0 is `integrate`'s, set and cleared by mask; bit 1 is written by the synthesis and read by `Polarity::of_flags`; bit 2 is ADR-0114's mark, read by the turn. No other code reads or writes `flags`.
2. **`[20..24)` is `_reserved_20`**, read by `is_at_rest_image` and written by `encode`, `decode` and `restore_plain_fields`; nothing else names it.
3. **The executor's census of rest is one function, `at_rest`**, read in three places: the turn, to keep a unit on the schedule; the loader, to wake a unit written awake; and the clock sweep, to leave a unit that is not at rest. A marked unit whose slow potential is not zero is not at rest in any of them: the turn must serve it so that its slow potential leaks, the loader must wake it, and the sweep must not evict it with a potential frozen in the log.
4. **The turn sums the whole batch into `basal` and `apical`**, and every message says whether a synapse sent it (`message_is_synaptic`, ADR-0054) and where it lands (`message_is_apical`).
5. **The modulator record has room**: `[26..28)` and `[36..64)` are reserved since ADR-0114, 30 bytes.
6. **Four tests pin an image's whole bytes, header included**: `REVERSAL_IMAGE_CRC_1024` and `PUNISHED_IMAGE_CRC_1024` in `tests/inhibition.rs`, re-pinned at format 17 by ADR-0114; `H20_IMAGE_CRC` in `tests/assembly.rs`, the second restated; and the fifth field of `DRAINED_050`, the CRC-64 of the frozen image H-20's arm leaves (ADR-0117), which ADR-0114 did not meet because it did not exist yet.

## Decision Drivers

- ADR-0122's decision and brief 052's standing directives: `integrate` untouched; unset, bit for bit; an unmarked unit's turn costs nothing new but a test of its mark; no dependency, no float, no `unsafe` beyond ADR-0023's invariant.
- **A structural boundary beats a reviewed one** (principle 5): the rule held to `integrate` by a property test over the lattice and to an `i64` oracle of ADR-0122's text; the constants refused by the configuration and the loader alike; the mark refused at load while no constants are set; a slow potential in an unmarked record refused at load.
- **The tick reads no new word**: the constants are fixed for a run, so each worker holds them, as ADR-0114's class is held.
- ADR-0095: a pin of a whole image moves with the format and is re-pinned in the round that moves it, the rest of the image held to the pin before it.

## Considered Options

1. **The mark**: (a) bit 3 of `flags`, `0x08`; (b) a higher bit.
2. **The field**: `v_slow`, an `i32` beside the other potentials, or a `u32`.
3. **The constants in the image**: (a) `[36..48)`: a flag byte, the two shifts, a reserved byte, the two voltages as `i32`; (b) the voltages as `u16` fractions of the base.
4. **The rule**: (a) `integrate_slow(basal, apical, slow, now, current)` beside `integrate`, `integrate`'s body line for line with the slow lines added; (b) `integrate` rewritten to call a parameterised rule.
5. **The gate's edges**: (a) a clamp of the distance above $V_{lo}$ to the span, then one division; (b) a comparison at each voltage.
6. **The census of rest**: (a) `at_rest` reads the slow potential, and the turn reads it only in a marked unit's arm; (b) the turn reads it for every unit.
7. **A marked unit while no constants are set**, inside a running executor (a test can mark one in memory): (a) it integrates under `integrate`, the mark read by nothing; (b) the tick aborts.

## Decision Outcome

**Options 1(a), 2 `v_slow: i32`, 3(a), 4(a), 5(a), 6(a) and 7(a).**

- **The mark** (`crates/cortex-core/src/dynamics/membrane.rs`): `FLAG_SLOW`, `0x08`, the next free bit. Neither rule touches it: `integrate` and `integrate_slow` set and clear `FLAG_BURST_MODE` by mask. The property test's walk runs with every bit but the burst bit set and asserts them unmoved after every tick.
- **The state** (`crates/cortex-core/src/dynamics/neuron.rs`, `serial.rs`): `v_slow: i32` at `[20..24)`, Q16.16, signed like the other potentials, so that a slow potential below zero is read as one the rule leaks to rest and gives the soma nothing. `encode`, `decode` and `restore_plain_fields` carry it. `is_at_rest_image` asks that it be zero in a unit not marked `FLAG_SLOW`, since nothing writes it there; a marked unit's may be anything.
- **The constants** (`membrane.rs`): `SlowCurrent { leak_shift, input_shift, v_lo_q16, v_hi_q16 }`, `Clone, Copy, Debug, PartialEq, Eq`, not a record. `is_valid`:
  - a leak shift from 1 to 16: zero is no time constant, and above 16 a slow potential below 1.0 leaks by one LSB a tick whatever its size, a line rather than a time constant;
  - an input shift from 0 to 16: above it a message of at most 1.0 gives less than one LSB;
  - $0 < V_{lo} < V_{hi} \le$ `THRESHOLD_BASE`.
- **The rule** (`membrane.rs`): `DendriticSuperNeuron::integrate_slow(basal, apical, slow, now, current) -> bool`, `integrate`'s body line for line with three additions:
  - after the compartments, `v_slow = add(leak(v_slow, k_s), slow >> g)`, whether or not the unit is refractory;
  - before the soma's update, `SlowCurrent::current_q16(v_soma, v_slow)`, the gate read on the soma as the tick found it — nothing above it in the tick moves the soma — times the slow potential at no less than zero, shifted by 20: the gate's 16 fractional bits and the coupling's 4, one shift, floored;
  - that current added to the soma's update beside its coupling to the compartments.

  With $s = 0$ and no slow input the leak leaves zero, the input adds zero and the current is zero, so the tick is `integrate`'s bit for bit. `SlowCurrent::gate_q16` clamps the distance above $V_{lo}$ to $[0, V_{hi} - V_{lo}]$ before one division (option 5(a)): at either voltage the clamp and the division agree, so no comparison exists there for a mutant to move off its bound. `integrate` is not touched.
- **The executor** (`runtime/cortex-runtime/src/executor.rs`):
  - `Config::slow_current: Option<SlowCurrent>`, `None` by default. `Executor::new` refuses constants that are not valid (`ConfigError::SlowCurrentOutOfRange`) and copies them into each worker's own field and into the executor's, read back by `Executor::slow_current()`.
  - In the turn, where `integrate` was called, a `match` on the worker's constants with a guard on the unit's mark: for a marked unit while the constants are set, `slow_input(batch)` — the positive efficacies of the batch's synaptic messages that land in the basal compartment, summed, saturating — scaled by the gain as the basal sum is, then `integrate_slow`; otherwise `integrate`, as before.
  - The census (option 6(a)): `at_rest` is `membrane_at_rest(u) && u.v_slow == 0`, which the loader and the sweep read. The turn keeps a unit on the schedule while `slow_awake || !membrane_at_rest(u)`, where `slow_awake` is the marked arm's `v_slow != 0` and the unmarked arm's `false`. **An unmarked unit's turn gains the test of its mark and nothing else**: no word, no allocation, no syscall and no read of its slow potential.
  - A unit marked in memory while no constants are set integrates under `integrate` (option 7(a)); no image can carry that state.
- **The image** (`runtime/cortex-runtime/src/image.rs`; `crates/cortex-connectome/src/lib.rs`):
  - The modulator section's `[36]` is the flag, `SLOW_CURRENT_SET` (1) while set and zero while unset; `[37]` the leak shift, `[38]` the input shift, `[39]` reserved, `[40..44)` $V_{lo}$ and `[44..48)` $V_{hi}$, `i32` in Q16.16; every byte zero while unset. `[26..28)`, `[39]` and `[48..64)` stay reserved, 19 bytes.
  - `FORMAT_VERSION` moves from 17 to **18**, with its note in the version history, and the section's doc comment lists the record as it is.
  - The loader reads the constants (`slow_current_of`) beside the class, before it builds the executor, and passes them through the configuration: the image's, set or unset, outrank the configuration's (§8.3).
  - A flag of zero with every byte zero is unset: a format-17 record reads as unset under the new version. A flag of one with constants the rule does not resolve is refused as the configuration's are (`ImageError::Config(ConfigError::SlowCurrentOutOfRange)`). Any other flag, a byte beside a zero flag, or the reserved byte is refused as one the writer never produces (`ImageError::ReservedNotZero { section: 42, index: 0 }`).
  - A unit whose record carries the mark while no constants are set is refused (`ImageError::MarkWithoutSlowCurrent(unit)`); an unmarked unit with a slow potential is not at rest (`ImageError::NotAtRest(unit)`). A marked unit whose only departure from rest is its slow potential is woken on load and leaks it.
  - A version-17 header is refused as every foreign version is (`HeaderError::ForeignVersion(17)`, L-6).
- **The tests.**
  - `cortex-core` (`membrane.rs`):
    - `a_slow_current_is_valid_within_its_bounds_and_its_bit_is_its_own`: every constant's edges on both sides, and the mark apart from the other three bits.
    - `the_gate_is_shut_at_and_below_its_low_voltage_open_at_and_above_its_high_and_linear_between`, `the_current_is_the_gate_times_the_slow_potential_over_sixteen_and_nothing_below_zero`, `one_tick_of_the_slow_rule_is_worked_by_hand`, `the_slow_potential_leaks_by_its_shift_and_takes_its_input_by_its_shift_at_their_bounds`, `in_the_refractory_window_the_slow_potential_takes_its_input_and_the_compartments_drop_theirs` and `a_slow_potential_fires_a_unit_under_an_open_gate_that_the_same_inputs_do_not_fire`: each number worked by hand from the rule.
    - Over the lattice of `testkit/prop.rs` under five sets of constants, from three starts, and a seeded walk of a million ticks with every other flag set, `with_no_slow_potential_and_no_slow_input_the_slow_rule_is_integrate_bit_for_bit`.
    - Over the lattice of the soma, the slow potential and the slow input, the soma also at each gate's edges, and 400 000 seeded draws of a whole state, its inputs and its constants, `the_slow_rule_is_adr_0122_s_by_an_i64_oracle_at_any_state`: the oracle written from ADR-0018's text and ADR-0122's in `i64`, not from `integrate_slow`.
  - `cortex-core` (`serial.rs`): the slow potential's bytes, signed, round-tripped and restored; at rest in a marked unit and not in an unmarked one.
  - `executor.rs`: `a_marked_unit_takes_its_excitatory_synaptic_basal_input_into_its_slow_potential_and_an_unmarked_one_integrates_as_before`:
    - the default is unset; `new` refuses six constants at the edges the rule does not resolve and takes three at the edges it does;
    - in one run of 3 000 ticks at a gain of 1.75, both units fed the same batches of every kind of message — an excitatory and an inhibitory synapse's into the basal compartment, a synapse's into the apical one, injected messages into both — the marked unit is `integrate_slow` on a copy whose slow input is the positive efficacies of the synapses' basal messages alone, scaled by the gain, and the unmarked unit `integrate` on a copy, every field tick by tick; the slow current fires the marked unit more often;
    - with no constants the marked unit is `integrate`'s too, its slow potential zero;
    - a marked unit with a slow potential and nothing else stays on the schedule, leaking it, until it is zero, and an unmarked unit's turn does not read the field.
  - `tests/image.rs`: `the_slow_current_is_written_to_and_read_from_the_image_and_a_record_left_zero_reads_as_unset`, the section's bytes and the record's written and read back set and unset, beside the inhibitory baseline, the signed gate and the class; a marked unit woken on load; four constants at the resolved edges read back and ten refused as the configuration's are; twelve flag and byte patterns and three reserved bytes refused; a mark refused while unset, and the facilitating mark alone no mark; a slow potential in an unmarked record refused; a version-17 header refused. The three earlier tests' reserved-byte probes move from `[36]`, now the slow current's flag, to `[39]` and `[48]`.
  - `crates/cortex-connectome`: the version's two assertions read 18. `tests/differential.rs` and `tests/contention.rs` name the field as `None`.
- **Every pinned number unchanged with no unit marked, bar the four whole-image pins.** Each is re-pinned as ADR-0095 re-pinned H-17's, with every other byte of its image held to the pin before it by the header's version written back and the header resealed (`with_version`):
  - H-17's inhibited image reads `0xd5579c31308388ad` at format 15, the CRC H-17 read and `REVERSAL_IMAGE_CRC_FORMAT_15_1024`, and `0x938ad516b6badd90` at 17, the pin before this round; its format-18 CRC is `0x50af5c06f491f0fe`, the new `REVERSAL_IMAGE_CRC_1024`.
  - H-18's signed image reads `0x3771636d385191ac` at format 16, `PUNISHED_IMAGE_CRC_FORMAT_16_1024`, and `0x766de462f9b77576` at 17; its format-18 CRC is `0xb5486d72bb9c5818`, the new `PUNISHED_IMAGE_CRC_1024` and `H20_IMAGE_CRC`.
  - The frozen image H-20's arm leaves (ADR-0117) reads `0xd763b4a58364291e` at format 17, ADR-0117's pin, now `DRAINED_IMAGE_CRC_FORMAT_17`, which `drained_image` asserts on every run that makes it; its format-18 CRC is `DRAINED_050`'s fifth field.

  Every other whole-domain test reproduces its pinned numbers in the weekly dispatched on brief 052's branch; its run id is in brief 052's measurement ADR.
- **The mutation gate** ([ADR-0030](0030-verification-governance.md)): every mutant `cargo-mutants` makes in the changed lines is caught. The in-diff run's outcome is recorded with the measurement's evidence.

### Consequences

- Good: ADR-0122's current exists in the rule, the executor and the image. Unset, it is the engine before it bit for bit by construction: `integrate` is not touched, the turn's first arm is never taken, and the image's bytes are zero where a format-17 writer left zero.
- Good: the tick reads no new word, and an unmarked unit's turn gains one test of its mark.
- Good: the constants are part of the image and outrank the configuration on load (§8.3). They are not a registry entry: the engine cannot amend them.
- Neutral: the format is 18 for a field, a bit of `flags` and twelve bytes of the modulator record. A format-17 image is refused by its header, and its zeros read as unset under the new version.
- Bad: `integrate_slow` repeats `integrate`'s body, so a later change to the membrane rule has two copies to change; the property test that holds them equal with no slow potential is what keeps them one rule, as ADR-0114's test keeps `step_stp_class` to `step_stp`.
- Bad: a marked unit's turn walks its batch twice, once for the sums and once for its slow input. Its cost is not measured by this round.

## Alternatives considered and why rejected

- **A higher bit** (option 1(b)): no reason to leave a hole at `0x08`.
- **A `u32` field**: the rule's input is never negative, but an image's field is read before the rule runs, and a signed potential is what every other potential of the record is.
- **The voltages as `u16` fractions** (option 3(b)): $V_{hi}$ at the base is 1.0, which a Q0.16 fraction cannot hold, and the rule reads Q16.16.
- **`integrate` calling a parameterised rule** (option 4(b)): it would change `integrate`'s code, which ADR-0122 and the brief keep untouched.
- **A comparison at each voltage** (option 5(b)): at either voltage the comparison and the division agree, so a mutant moving `<` to `<=` there would survive every test.
- **The turn reading every unit's slow potential** (option 6(b)): one read and one comparison in every unmarked unit's turn, where the directive allows only the test of its mark.
- **An abort** (option 7(b)): the loader already refuses an image with a mark and no constants; stepping a unit marked in memory as unmarked keeps "unset" meaning "every unit under `integrate`" without a branch in the tick that can end the process.

## Confirmation

- `crates/cortex-core/src/dynamics/membrane.rs`: `FLAG_SLOW`, `SlowCurrent`, `SlowCurrent::{is_valid, gate_q16, current_q16}`, `DendriticSuperNeuron::integrate_slow`; `integrate` unchanged.
- `crates/cortex-core/src/dynamics/neuron.rs`, `serial.rs`: `v_slow`; `is_at_rest_image`.
- `runtime/cortex-runtime/src/executor.rs`: `Config::slow_current`, `ConfigError::SlowCurrentOutOfRange`, `Executor::slow_current`, the workers' `slow_current`, `slow_input`, `membrane_at_rest`, the selection in the turn.
- `runtime/cortex-runtime/src/image.rs`: `SLOW_CURRENT_FLAG`, `SLOW_CURRENT_LEAK`, `SLOW_CURRENT_INPUT`, `SLOW_CURRENT_V_LO`, `SLOW_CURRENT_V_HI`, `SLOW_CURRENT_SET`, `MODULATOR_RESERVED`, `slow_current_of`, `ImageError::MarkWithoutSlowCurrent`.
- `crates/cortex-connectome/src/lib.rs`: `FORMAT_VERSION` 18 and its note; `SECTION_MODULATOR`'s comment.
- `runtime/cortex-runtime/tests/inhibition.rs` and `tests/assembly.rs`: `REVERSAL_IMAGE_CRC_1024`, `PUNISHED_IMAGE_CRC_1024`, `H20_IMAGE_CRC`, `DRAINED_IMAGE_CRC_FORMAT_17`, `with_version`.
- Whitepaper §5.2.1 (the record's table, the public API, the membrane), §5.2.2 (the format's version row), §8.7, §8.8 (the membrane row), §9; `CHANGELOG.md`; the image format 18.
