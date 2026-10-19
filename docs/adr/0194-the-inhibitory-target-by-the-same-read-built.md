---
status: proposed
date: 2026-10-19
depends-on: ADR-0192
decision-makers: VirtualCortex maintainers
---

# ADR-0194: The inhibitory target by the same read, built — ADR-0192's flag as `Config::istdp_target_read`, a flag of the read: set beside a read of the rate's target, the read's last window sets the inhibitory rule's period to the period it sets the rate's to, through `set_istdp_target_period`, and the flag is spent; a mean whose period the inhibitory rule refuses ends the read refused, by a range pattern in `read_step`; refused without a read, with `ConfigError::IstdpReadWithoutRead`; at the modulator section's `[26]`, beside a read under way and nowhere else, `[27]` reserved; the image format moved from 23 to 24; unset, every run is the run it was, the six whole-image pins that carry a pin written back re-pinned by the mask and the eighteen H-40 read at format 23 held by the version written back

## Context and Problem Statement

[ADR-0192](0192-the-inhibitory-target-by-the-same-read.md) decided the flag and left its placement to brief 075. Its decision:
- *"Unset, every run is the run it was, bit for bit. An image of format 23 reads as unset."*
- *"Set beside the read: at the read's last window the engine sets the inhibitory rule's period to the period it sets the rate's to, through the path an image's period takes, `set_istdp_target_period`."*
- *"Set without the read: refused."* And *"With it set, a read's mean refused if the inhibitory rule would refuse the period it makes."*
- *"Its byte is the modulator section's `[26]`; `[27]` stays reserved. The image's format becomes 24."*

Brief 075's empowerment names the choices: the flag's name in `Config`, its encoding in `[26]`, where the executor sets the inhibitory period at the read's end, and the error a refused flag or a refused period raises, and where.

What was read first (principle 2), on 2026-10-19, at `main` `417af36`:

1. **The read's close** (`runtime/cortex-runtime/src/executor.rs`, `read_close`, `read_step`). At the read's last window `read_step` returns `ReadStep::Target { period, count }` from `read_target_of`, or `ReadStep::Refused`; `read_close` then sets `rate_target_period_ticks` and `rate_target` and ends the read, or ends it refused. It touches nothing of the inhibitory rule.
2. **The inhibitory rule's period in the engine.** `Config::istdp_target_period_ticks`, default 20 000, is refused by `Executor::new` outside `ISTDP_PERIOD_MIN_TICKS..=ISTDP_PERIOD_MAX_TICKS`, 100 to 1 000 000. `set_istdp_target_period(period)` refuses the same range, stores the period and publishes `istdp_alpha_q1_15(period)` to `shared.istdp_alpha`, which every worker reads at its next tick; the loader calls it with the image's `[20..24)`.
3. **What a read accepts that the inhibitory rule does not.** `rate_target_of` refuses a period only where its count for the units is no spike a window or past `RATE_TARGET_MAX`. So a read can end on a period past 1 000 000 — at 1 024 units a mean of 134 spikes a window or fewer — or below 100, where the units times $2^{17}$ over the mean is shorter.
4. **The modulator section** (`runtime/cortex-runtime/src/image.rs`). `MODULATOR_RESERVED` is `[26..28)`, refused when not zero; every other byte of the section is a field's. The read under way is `[39]`, `[54..56)` and `[60..64)`, set on the engine `Executor::new` builds by `set_target_read`, the configuration's read set to none first.
5. **The whole-image pins** (`tests/inhibition.rs`, `tests/assembly.rs`). Six images carry a pin as they are now beside a pin written back to the format their hypothesis read ([ADR-0095](0095-an-image-pin-moves-with-its-format.md)), and the gate restates the five of `tests/inhibition.rs` as literals. H-40's three derivations pinned their six images each at format 23 (`READ_CRCS`), held by `read_pins_held` to `Derivation::crcs`, the images at the current format. H-38's and H-39's forty-eight are held at format 22 by the version written back (`RULED_READ_FORMAT`, ADR-0189), and twenty-five at 21 (ADR-0180).

## Decision Drivers

- ADR-0192's decision and brief 075's standing directives: the flag in the engine; unset bit for bit; no change to the inhibitory rule's form, the rate's rule, the read's span or a state crate's record; `Config`'s default unset; no dependency, no float, no `unsafe`.
- **An image is the run** (§8.3): a read with the flag written mid-read reads back to the same run.
- **The engine after the read is the engine its image loads**: an image written after the read carries both periods and nothing of the read, as ADR-0189 holds for the rate's.
- **A structural boundary beats a reviewed one** (principle 5): the flag without a read is refused by one rule for the configuration and the loader.
- **Every mutant in the changed lines caught** ([ADR-0030](0030-verification-governance.md)): bounds as range patterns where a comparison would leave a mutant.
- **No capacity reached silently**: a period the inhibitory rule refuses is refused by name, not clamped.

## Considered Options

1. **The flag's form**: (a) `Config::istdp_target_read: bool`, beside `rate_read_windows`; (b) a field of `TargetRead::Reading`; (c) a set of targets the read writes, an enum.
2. **Its lifetime**: (a) the read's: set only beside a read under way, spent when the read ends, either way; (b) kept after the read, so that an image after it carries the flag.
3. **Where a period the inhibitory rule refuses is refused**: (a) in `read_step`, a range pattern beside the flag, so the read ends `TargetRead::Refused` before anything is set; (b) in `read_close`, after the rate's target is set; (c) the period clamped into the rule's range.
4. **The error of the flag without a read**: (a) a new `ConfigError::IstdpReadWithoutRead`, raised by `Executor::new` after the read's own refusals and by the loader beside no read under way; (b) `RateReadOutOfRange` again.
5. **The byte**: (a) `[26]`, one while set and zero while not, any other value a byte the writer never produces; `[27]` reserved; (b) a bit of `[39]`.
6. **The whole-image pins**: (a) the six that carry a pin written back re-pinned by the mask, H-40's eighteen at format 23 held by the version written back, every other as it was; (b) all re-pinned.

## Decision Outcome

**Options 1(a), 2(a), 3(a), 4(a), 5(a) and 6(a).**

- **The parameter** (`runtime/cortex-runtime/src/executor.rs`): `Config::istdp_target_read: bool`, false by default, which is unset.
  - Set, `Executor::new` holds it beside the read it starts (`Executor::istdp_target_read`, true while the read under way will set the inhibitory period).
  - Refused without a read (`ConfigError::IstdpReadWithoutRead`), after the read's own refusals: a read short of sixteen windows is `RateReadOutOfRange` and one beside a period `RateReadBesideTarget`, whatever the flag. The refusal is a pattern over the pair `(flag, read)`, so no operator is left for a mutant to move.
  - *Why not a field of `TargetRead::Reading`* (option 1(b)): the tree compared `TargetRead::Reading { left, sum }` in sixteen places, H-40's harness among them; a third field would rewrite each, and a refused read has no flag to carry. *Why not a set of targets* (option 1(c)): a second target is the only one ADR-0192 decided.
- **The flag is the read's** (option 2(a)). `read_close` spends it at the read's last window, whether the read ends in a target or refused. So the engine holds it only beside a read under way, the writer writes it only there, and an image written after the read is the configuration with both periods set, the engine its image loads. *Why not kept* (option 2(b)): an image after the read would carry a flag with no read to act on, which the loader would then have to accept beside no read, and the flag without a read would no longer be one refusal.
- **The read's close with the flag** (`read_step`, `read_close`). `read_step(left, sum, spikes, units, inhibitory)` takes the flag; at the last window a target is the read's where the flag is unset, and where it is set only where the period lies in `ISTDP_PERIOD_MIN_TICKS..=ISTDP_PERIOD_MAX_TICKS`, a range pattern of the two constants (option 3(a)). Otherwise the read ends `ReadStep::Refused`, as one the rate's target refuses: `TargetRead::Refused`, no target set, the gain held from there on, and `Image::encode` refusing the engine's image with `ConfigError::RateTargetOutOfRange`, ADR-0189's path. At `ReadStep::Target` with the flag set, `read_close` sets the rate's period and count as before and then the inhibitory period through `set_istdp_target_period`, which publishes the depression to the workers for the next tick; a refusal there, which `read_step` has made unreachable, aborts as a violated invariant. *Why not in `read_close`* (option 3(b)): the rate's target would be set and then taken back, two writes for one refusal. *Why not a clamp* (option 3(c)): a period the rule refuses would be a target reached silently.
  - At 1 024 units the inhibitory rule accepts a read's mean from 135 spikes a window, a period of 994 205 ticks; H-40's three read 2 314 to 2 331, periods 57 579 to 58 002.
- **The image** (`runtime/cortex-runtime/src/image.rs`):
  - The modulator section's `[26]` is the flag, `ISTDP_READ_SET`, one, while set, and zero otherwise (option 5(a)). `MODULATOR_RESERVED` becomes `[27..28)`, refused when not zero. *Why not a bit of `[39]`* (option 5(b)): `[39]` is the windows left, a count to 255, and a bit taken from it would halve the span.
  - The writer writes `[26]` from `Executor::istdp_target_read`, so only beside a read under way.
  - The loader reads `[26]` (`istdp_target_read_of`): zero unset, one set, any other value `ImageError::ReservedNotZero`. It builds the engine with the configuration's flag unset, as it does the read — the image's flag, set or unset, outranks the configuration's (§8.3) — sets the image's read on it, and then the flag (`Executor::set_istdp_target_read`), refused beside no read under way with `ImageError::Config(ConfigError::IstdpReadWithoutRead)` (option 4(a)), the configuration's error.
  - `FORMAT_VERSION` moves from 23 to **24**, with its note in the version history. A version-23 header is refused as every foreign version is (`HeaderError::ForeignVersion(23)`, L-6); its zero at `[26]` reads as unset under the new version.
- **Unset, every run is the run it was.** With the flag unset `read_step` returns what it returned, `read_close` sets what it set, the writer leaves `[26]` zero where a format-23 writer left it, and the loader sets nothing. Only the header's version and seal differ.
- **The tests.**
  - `executor.rs`, by hand: a read's last window at 1 024 units at a mean of 2 330, 135 and 134 a window, periods 57 604, 994 205 and 1 001 625; at 15 625 units at 2 048 and 2 047, the rule's widest period, 1 000 000, and 1 000 489; at 25 units at 32 768 and 33 099, its shortest, 100, and 99 — each the read's with the flag unset, and with it set the read's or refused; no spike; a window before the last; a `left` of zero. The configuration: unset by default; set beside a read, the flag held and the period the default; refused without a read alone and beside a period, and after the read's own refusals.
  - `executor.rs`, over the lattice: the 4 000 reads of ADR-0189's walk with the flag set in every other four, held at the last window to the oracle of ADR-0188's text in `i128` and, where the flag is set, ADR-0192's refusal beside it; the walk meets both of the inhibitory rule's bounds.
  - `tests/criticality.rs`: an engine of 64 units with the flag and one without through ADR-0189's seventeen designed windows, one at every window of the read — the record held, the read's progress, the rate's target — the inhibitory period the default 20 000 on both and the flag set, until at the read's last window the flagged engine holds 26 214 for both periods and the flag spent; its image loading with both periods, no read and no flag. Sixteen windows of eight spikes, a mean whose period, 1 048 576, the rate's target accepts and the inhibitory rule refuses: the read ends in that target unset and refused with the flag, the gain held after and the image refused. A recurrent network written mid-window in its read's third window with the flag: its image carrying `[26]` and loading with the flag whatever the configuration says, the loaded engine and one on four workers reading and setting both periods window for window as the engine that wrote it, and the same image with `[26]` cleared reading alike and ending with the default period.
  - `tests/image.rs`: `[26]` zero with the flag unset and the image's unset outranking the configuration's flag; set beside a read just begun at `[26]` and no other byte of the section, read back whatever the configuration says; beside a read under way with a sum, read back and written again the same; refused beside no read, alone and beside a period, whatever the configuration says; refused at any other value; `[27]` refused; the image equal to its version written back to 23 but for the version and the seal; a version-23 header refused. The earlier tests that held `[26]` reserved now hold `[27]`.
  - `crates/cortex-connectome`: the version's assertion reads 24. `tests/differential.rs` and `tests/contention.rs` name the field as unset.
- **Every pinned number unchanged with the flag unset, but a whole-image pin with the format** (option 6(a)).
  - A probe on the prior's image at 1 024 units, not kept in the tree, read the mask from 23 to 24 as `0xea02f306149ba5dc` on seeds 22, 23 and 24, each image 590 656 bytes: two images of one length differ in the header's version and seal alone, and a CRC is linear.
  - **The six images that carry a pin written back** are re-pinned as they are now, by that mask, and the gate's literals with them. The check that holds every other byte of each is the one it had: the image with its header's version written back to the format its hypothesis read.

    | Image | At format 24 | At format 23, the pin before | Held to |
    | :--- | :--- | :--- | :--- |
    | H-17's inhibited image, `REVERSAL_IMAGE_CRC_1024` | `0x6d1b9b1b89793c15` | `0x8719681d9de299c9` | `0xd5579c31308388ad` at 15 |
    | H-18's signed image, H-20's, `PUNISHED_IMAGE_CRC_1024` and `H20_IMAGE_CRC` | `0x88fcaa6fc67494f3` | `0x62fe5969d2ef312f` | `0x3771636d385191ac` at 16 |
    | H-21's image, `TARGET_IMAGE_CRC_1024` | `0x925328efa87569f4` | `0x7851dbe9bceecc28` | `0xafe7eff2d59da51f` at 18 |
    | H-22's image, `VALUED_IMAGE_CRC_1024` | `0xc3cfd797c3d42038` | `0x29cd2491d74f85e4` | `0xbf6797857fda0809` at 19 |
    | H-23's image, H-25's and H-29's, `WINDOWED_IMAGE_CRC_1024` | `0x7963976f60173d41` | `0x93616469748c989d` | `0x5044ed7936a7b5f3` at 20 |
    | The frozen image H-20's arm leaves, `DRAINED_050`'s fifth field | `0x29f2faa8bca7c89b` | `0xc3f009aea83c6d47` | `0xd763b4a58364291e` at 17 |

  - **The eighteen H-40 read at format 23 keep their numbers** (`READ_CRCS`): a derivation now reads its six images written back to 23 as well (`Derivation::crcs_read_target`, `READ_TARGET_FORMAT`), and `read_pins_held` holds those to the round's pins, as `rate_pins_held` holds `crcs_ruled` to H-38's and H-39's. The forty-eight read at format 22 and the twenty-five at 21 keep theirs as ADR-0189 and ADR-0180 held them.
  - Re-pinning all ninety-seven (option 6(b)) would move ninety-one numbers a check can hold where they are.
- **The mutation gate** ([ADR-0030](0030-verification-governance.md)). No file under `src/` changed after `c92ad6d`. The pull request's runs on `c92ad6d` and `9e86d1f` were cancelled by the pushes after them, before their mutation jobs ended. On the run at `36c9b09`, 39194365915, `cargo-mutants` made 21 mutants in the changed lines: 16 caught, 5 unviable, none missed; and the same on the run at `c1b8d55`, 39202120251, the last commit to change code. The measurement's ADR records the dispatched sweep's state with its evidence ([ADR-0195](0195-the-inhibitory-target-by-the-same-read-measured.md)).
- **What brief 075 did not empower and this does not take**: the inhibitory rule's form, the rate's rule, the read's span and the estimate's rule do not change; no record of a state crate changes its layout; `Config`'s default leaves the flag unset.

### Consequences

- Good: ADR-0192's flag exists in the engine and the image, and an engine whose read ended with it is the engine ADR-0189 built with both periods set, its image that configuration's.
- Good: unset, the engine is the one before it by construction and by test: `read_step`'s target is the one it was, `read_close` sets what it set, the image's `[26]` is zero where a format-23 writer left zero.
- Good: an image written mid-read with the flag reads back to the same run, and the flag without a read is refused by one rule for the configuration and the loader.
- Neutral: the format is 24 for one byte. A format-23 image is refused by its header, and its zero reads as unset.
- Bad: one parameter more, and the read's state in the modulator section grows a byte.
- Bad: one byte of the modulator section remains, `[27]`.

## Alternatives considered and why rejected

- **A field of the read or a set of targets** (options 1(b), 1(c)), **the flag kept after the read** (option 2(b)), **the refusal after the rate's target is set, or a clamp** (options 3(b), 3(c)), **the read's own error** (option 4(b)), **a bit of `[39]`** (option 5(b)), **all pins re-pinned** (option 6(b)): see above.
- **The read always setting both periods**: ADR-0192's option 2(b), rejected there; it would move H-40's pinned derivations and arms.

## Confirmation

- `runtime/cortex-runtime/src/executor.rs`: `Config::istdp_target_read`, `ConfigError::IstdpReadWithoutRead`, `read_step`'s flag, `Executor::{istdp_target_read, set_istdp_target_read, read_close}`; the tests named above and the lattice property `read_prop`.
- `runtime/cortex-runtime/src/image.rs`: `ISTDP_READ_FLAG`, `ISTDP_READ_SET`, `MODULATOR_RESERVED`, `istdp_target_read_of`.
- `crates/cortex-connectome/src/lib.rs`: `FORMAT_VERSION` 24 and its note; `SECTION_MODULATOR`'s comment.
- `runtime/cortex-runtime/tests/criticality.rs`, `tests/image.rs`; `tests/inhibition.rs` and `tests/assembly.rs`: the six constants above and the gate's literals, `READ_TARGET_FORMAT`, `Derivation::crcs_read_target`.
- Whitepaper §5.2.2 (the format's version row), §5.2's read paragraph, §8.7, §8.8, §9; `CHANGELOG.md`; `CLAUDE.md`; the image format 24.
