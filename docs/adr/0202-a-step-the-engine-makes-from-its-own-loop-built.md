---
status: accepted
date: 2026-10-20
depends-on: ADR-0200
title: A step the engine makes from its own loop, built
kind: built
summary: 'ADR-0200''s probe is built as `Config::step_probe`: after the read, 16 windows at g₀·(1 ∓ 1/64), D the higher windows'' counts less the lower ones'', and the step 4 096·c*/D, refused at D ≤ 0 or past 0x8000. The image moved from format 24 to 25, and unset every run is the run it was.'
decision-makers: VirtualCortex maintainers
---

# ADR-0202: A step the engine makes from its own loop, built

**In brief**
- **Decision.** The probe of [ADR-0200](0200-a-step-the-engine-makes-from-its-own-loop.md) is `Config::step_probe`, a state of the read (`TargetRead::Probing`) and the modulator section's `[27]`, at format 25.
- **Why.** Brief 078 builds it before H-42 runs. Each choice its empowerment left to the round is recorded below.
- **Left open.** Whether the probe replaces step = 1/16 in `Config::reads_its_targets()`: H-42's to read, and the next decision's to take.

## Context and Problem Statement

ADR-0200 decided the probe and left its placement to brief 078.
Its decision, in its words:
- *"Set beside the read, the read's end starts the probe instead of the rule. Set without the read, it is refused."*
- *"Unset, every run is the run it was, bit for bit, and an image of format 24 reads as unset."*
- *"The probe holds the gain for 16 windows, in the order −, +, +, −, four, eight and four windows"*, its state *"in the read's bytes"*.
- *"κ = (1/4)/S, written as the step in Q0.16, which is 4 096·c*/D"*; refused *"when D ≤ 0, or when κ would pass `CONTROL_STEP_MAX_Q0_16`"*.

What was read first (principle 2), on 2026-10-20 at `main` `62195dc`:
1. **The read's close** (`runtime/cortex-runtime/src/executor.rs`). `tally` calls `read_close` at each window while `target_read` is set. `read_close` holds the record's gain (`HomeostaticDrivePool::hold`) and moves the read by `read_step`. At its last window it sets the rate's target, with ADR-0194's flag the inhibitory period too, and the rule runs from the next window.
2. **The gain the workers use.** `tick` stores the record's `synaptic_gain_q16` into `shared.gain` before each tick's first barrier. Every worker scales its sums by that word and by nothing else.
3. **The bytes.** The modulator section's `[27]` is its last reserved byte. The read's state is `[39]`, `[54..56)` and `[60..64)`, zero once the read has ended. The homeostasis record holds the step at `[20..22)` and the gain at `[28..32)`, and has no reserved byte.
4. **The whole-image pins.** Six constants and the gate's literals hold images at format 24. H-41's eighteen derivation CRCs are held at the current format (`both_pins_held`).
5. **`cargo-mutants` 27.1.0 makes no mutant inside `Executor::new`**, so a refusal there is held by its tests alone.

## Decision Drivers

- ADR-0200's decision, and brief 078's standing directives: unset bit for bit, no change to `crates/cortex-homeostasis/`, the read or `Config::reads_its_targets()`, no dependency, no float, no `unsafe`.
- **An image is the run** (§8.3): an image written during the probe reads back to the same run.
- **The engine after the probe is the engine its image loads.**
- **A structural boundary beats a reviewed one** (principle 5): bounds as range patterns where a comparison would leave a mutant.
- **No capacity reached silently**: a step past its bound is refused by name, not clamped.

## Considered Options

1. **Where g₀ is kept through the probe**: (a) the record's gain held at g₀, the workers given it moved by the window's factor; (b) the record's gain moved, and g₀ kept beside it.
2. **The probe's state**: (a) a variant of the read, `TargetRead::Probing { left, sum }`, in the read's bytes beside the target; (b) a field of the executor beside the read.
3. **The probe's gain at the controller's bounds**: (a) the product as it is, which may pass a bound by 1/64 of it; (b) clamped to the bounds.
4. **A refused probe**: (a) `TargetRead::Refused`, the read's target kept as a reading; (b) the target cleared, as a refused read has none.
5. **The flag without a read**: (a) a new `ConfigError::StepProbeWithoutRead`; (b) `IstdpReadWithoutRead` again.
6. **The whole-image pins**: (a) the six re-pinned by the format's mask and H-41's eighteen held by the version written back to 24; (b) all re-pinned.

## Decision Outcome

**Options 1(a), 2(a), 3(a), 4(a), 5(a) and 6(a).**

### The parameter and the probe

- **`Config::step_probe: bool`**, false by default, which is unset.
  - Refused without a read (`ConfigError::StepProbeWithoutRead`), after the read's own refusals and ADR-0194's flag's.
  - `Config::default()` and `Config::reads_its_targets()` leave it unset.
- **The probe begins at the read's end** (`read_close`). Where the read makes its target and the flag is set, the read's state becomes `TargetRead::Probing { left: 16, sum: 0 }` in place of none. The rate's target, and the inhibitory period under ADR-0194's flag, are set as before.
- **The record's gain is held at g₀ through the probe** (option 1(a)).
  - Each window closes by `hold`, as through the read, so the record's gain stays the read's.
  - `Executor::gain_given_q16` is what `tick` stores in `shared.gain`: the record's gain, or under the probe the record's times 63/64 or 65/64.
  - The factor is 65/64 in the windows with 5 to 12 left, a range pattern (`probe_raises`), and 63/64 in the others.
  - The product rounds to nearest with a half going up: $(64 \pm 1) g_0 + 32$, shifted right by 6.
  - At 1.75 the two gains are exactly 112 896 and 116 480 in Q16.16.
  - *Why not option 1(b)*: $g_0 \cdot 63/64$ rounded is not one-to-one, so g₀ could not be read back from it. Keeping g₀ elsewhere would take bytes the image no longer has.
- **The sum** (`probe_step`). Each window's count is added in a higher window and taken in a lower one, saturating.
- **The step** (`probe_step_of`). At the 16th window, $\kappa = (c^{*} \ll 12 + D/2) / D$, to nearest with a half going up.
  - A sum below zero is no `u64` and a sum of zero divides nothing, so D ≤ 0 is refused with no comparison.
  - κ past `CONTROL_STEP_MAX_Q0_16` is refused by a range pattern, `0..=0x8000`.
  - The shift of 12 is tied by a constant assertion to λ = 1/4, 8 windows a side and the gain moved by 2/64.
  - A step that rounds to zero, S above 32 768, is accepted: ADR-0200 refuses only the two cases above.
- **The probe's end** (`probe_close`).
  - With a step, the record's `control_step_q0_16` becomes κ and the probe ends. The rate's rule runs from the next window at κ, against the read's target.
  - Refused, the read's state is `TargetRead::Refused` (option 4(a)). The gain is held from there on, the read's target is kept as a reading, and the step stays the configuration's.
  - Either end spends the flag. A read that ends refused spends it too, and no probe begins.
  - *Why not option 4(b)*: a refused probe's network made its target, and H-42 reports it. Clearing it would hide the read's result behind the probe's.
- **The probe's gain is not clamped** (option 3(a)). At g₀ = 4.0 the higher windows run at 4.0625, and at 0.25 the lower ones at 0.2461. The record's gain never leaves its bounds. A clamp would make S half the network's at either bound.

### The image

- **The modulator section's `[27]`** is the flag, `STEP_PROBE_SET`, one while the probe follows the read under way or is under way, zero otherwise. Any other value is `ReservedNotZero`. The section has no reserved byte left.
- **The probe's state is in the read's bytes beside the rate's target** (option 2(a)):
  - `[39]` its windows left, 1 to 16;
  - `[60..64)` and `[54..56)` its sum, forty-eight bits of two's complement, sign-extended by bytes and no comparison;
  - held to `probe_sum_bounds(left)`, each window closed at most `RATE_TARGET_MAX` either way.
- **The loader** reads the flag first. Beside a period, the read's bytes are the probe's; beside none, the read's. It builds the engine with the configuration's flag unset and then sets the image's (`Executor::set_step_probe`). The flag beside no read and no probe, or a probe beside no target, is `ConfigError::StepProbeWithoutRead` (option 5(a)).
- **The writer** refuses an engine whose read or probe ended refused. A read refused has no target, `RateTargetOutOfRange` as before. A probe refused has a target and no step, `ControlStepOutOfRange`.
- **`FORMAT_VERSION` moves from 24 to 25.** A version-24 header is refused as every foreign version is (L-6). Its zero at `[27]` reads as unset.
- **Unset, every run is the run it was.** `read_close` returns the rule where it returned it, `gain_given_q16` is the record's gain, and the writer leaves `[27]` zero. Only the header's version and seal differ.

### The tests

- **`executor.rs`, by hand.**
  - The gain: each window's factor by its windows left; the half at 16 416 going up both ways; one LSB either side; the controller's bounds.
  - The step: the sum's sign by window; κ = 1 024 at c* = 2 306 and D = 9 224; the bound 0x8000 accepted at c* = 8 and D = 1, at 32 768 over 4 096 and at 320 over 40; one past refused; D = 0, −1 and the width refused; a half going up; c* = 0 a step of zero; a `left` of zero the last.
  - The loader's bounds at every count of windows closed.
  - The configuration: unset by default and in `Config::reads_its_targets()`; set beside a read; refused without a read and after the read's own refusals.
  - The engine through `read_close`: a read's last window starting the probe; sixteen windows of counts drifting up by one, D = 9 224 and κ = 1 024 at the last and not before; a probe of equal counts refused, its image refused; a read refused spending the flag.
  - The tick: a silent engine set to the probe, `shared.gain` the window's at its first tick and its last, through all sixteen.
- **`executor.rs`, over the lattice** (`probe_prop`). 4 000 probes over targets, counts and gains drawn on and between their edges, held at every window to an oracle of ADR-0200's text in `i128`. The walk meets a step, a sum not above zero and a step past its bound.
- **`tests/criticality.rs`.**
  - Designed windows on 64 units: an engine with the flag beside one without, one through the read; the probe's sixteen windows held to an oracle record, the step set to 0x1000 at the last and not before; the image after it; the rule at that step from the next window.
  - The probe's last window from images written at it: D = 40, the bound, a step; 39, 0, −1 and −641 refused, the gain held after and the image refused.
  - A recurrent network at a gain of 1.5, written in the probe's third window and loaded on two and four workers, alike window for window to the step and after; the image with `[27]` cleared refused.
- **`tests/image.rs`**: `[27]` zero unset; set beside a read at `[27]` alone; the probe under way read back and written again at each bound of its sum, below zero among them; refused past them, at no window left or more than sixteen, beside nothing, at another value; a version-24 header refused. The tests that held `[27]` reserved now hold it as the probe's.
- **`crates/cortex-connectome`**: the version reads 25. `tests/differential.rs` and `tests/contention.rs` name the field unset.

### The pins

- **The mask.** Two images of one length differ in the version and the seal alone, and a CRC is linear. The move from 24 to 25 is the move from 22 to 23, since 24 ⊕ 25 = 22 ⊕ 23 = 1. A probe not kept in the tree read the two equal on three image lengths.
- **The six whole-image pins move by `0x411c870fc1e6e4da`**, each pin's own move from 22 to 23, and the gate's literals with them:

| Image | At format 25 | At format 24 |
| :--- | :--- | :--- |
| `REVERSAL_IMAGE_CRC_1024` | `0x2c071c14489fd8cf` | `0x6d1b9b1b89793c15` |
| `PUNISHED_IMAGE_CRC_1024`, `H20_IMAGE_CRC` | `0xc9e02d6007927029` | `0x88fcaa6fc67494f3` |
| `TARGET_IMAGE_CRC_1024` | `0xd34fafe069938d2e` | `0x925328efa87569f4` |
| `VALUED_IMAGE_CRC_1024` | `0x82d350980232c4e2` | `0xc3cfd797c3d42038` |
| `WINDOWED_IMAGE_CRC_1024` | `0x387f1060a1f1d99b` | `0x7963976f60173d41` |
| `DRAINED_050`, its fifth field | `0x68ee7da77d412c41` | `0x29f2faa8bca7c89b` |

- **H-41's eighteen keep their numbers**: a derivation reads its images written back to 24 as well (`Derivation::crcs_both_targets`, `BOTH_TARGETS_FORMAT`).
- *Why not option 6(b)*: it would move numbers a check can hold where they are.

### The mutation gate

On pull request #227's run 39376981604 at `1fc2f28`, `cargo-mutants` made 56 mutants in the changed lines: 51 caught, 5 unviable, none missed ([ADR-0030](0030-verification-governance.md)).
No file under `src/` changed after `5cbdf58`, the build's commit.

### What brief 078 did not empower, and this does not take

The rate's rule, `hold`, `move_gain`, the gain's bounds, the read, its span, λ, the probe's size and windows, and `Config::reads_its_targets()` do not change.

## Consequences

- Good: ADR-0200's probe exists in the engine and the image, and an engine after it is the engine its image loads, with the step it made.
- Good: unset, the engine is the one before it, by construction and by test.
- Good: the workers' gain is one word published in one place, so the probe touches no worker.
- Bad: the modulator section has no reserved byte left. A next flag needs a section of its own.
- Bad: the probe's gain may pass the controller's bounds by 1/64 of them.
- Neutral: a refused probe keeps the read's target, so a refused read and a refused probe tell apart by it.

## Confirmation

- `runtime/cortex-runtime/src/executor.rs`: `Config::step_probe`, `ConfigError::StepProbeWithoutRead`, `TargetRead::Probing`, `PROBE_WINDOWS`, `probe_raises`, `probe_gain_q16`, `probe_step_of`, `probe_step`, `probe_sum_bounds`, `Executor::{step_probe, gain_given_q16, set_step_probe, probe_close}`; the tests above and `probe_prop`.
- `runtime/cortex-runtime/src/image.rs`: `STEP_PROBE_FLAG`, `STEP_PROBE_SET`, `step_probe_of`, `probe_of`.
- `crates/cortex-connectome/src/lib.rs`: `FORMAT_VERSION` 25 and its note.
- Whitepaper §5.2.2, §5.2.16, §8.7 and §8.8; `CHANGELOG.md`.
