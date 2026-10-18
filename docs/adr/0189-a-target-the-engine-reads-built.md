---
status: proposed
date: 2026-10-18
depends-on: ADR-0188
decision-makers: VirtualCortex maintainers
---

# ADR-0189: A target the engine reads, built — ADR-0188's read as `Config::rate_read_windows`, a span in windows, zero unset; through it the gain held at every window's end by `HomeostaticDrivePool::hold`, the rate's rule at a step of zero with the window's count returned, and the counts of its last sixteen summed; at its last window the mean to nearest made a period to nearest and the period's count, the engine then the engine its image loads; a mean the rate's target refuses ending the read refused, the gain held, its image refused; the read under way at the modulator section's `[39]`, `[54..56)` and `[60..64)`, its windows left and a 48-bit sum; a read beside a period refused by the configuration and the loader; the image format moved from 22 to 23; unset, every run is the run it was, the six whole-image pins that carry a pin written back re-pinned by the mask and the forty-eight read at format 22 held by the version written back

## Context and Problem Statement

[ADR-0188](0188-a-target-the-engine-reads.md) decided the read and left its placement to brief 074. Its decision:
- *"Set, the engine holds its gain for 128 windows from its first window: neither the rate's rule nor the estimate's moves it, whatever the step."*
- *"At the 128th window's end the target's count is their mean, to nearest. The period the image carries at `[56..60)` becomes the units times $2^{17}$ over that count, to nearest, so that an image written after the read is the configuration H-39 ran."*
- *"The read's span and its progress are part of the image, in the modulator section's reserved bytes: the windows read so far and the sum."*
- *"A read set beside a period already set is refused: an image has one target."* And *"A count `rate_target_of` would refuse stops the run with its error."*

Brief 074's empowerment names the choices: which reserved bytes, the width of the sum, how the windows read so far are counted, whether the executor or a method on the pool closes a held window, and the error a refused read raises and where.

What was read first (principle 2), on 2026-10-18, at `main` `f1e5a5c`:

1. **The record** (`crates/cortex-homeostasis/src/lib.rs`). `HomeostaticDrivePool` has no byte free. `regulate_rate(target)` stores the estimate by its slope where the window has one, moves the gain through `move_gain`, clears the window through `clear_window`, and at a step of zero leaves the gain as it was.
2. **The composition** (`runtime/cortex-runtime/src/executor.rs`, `tally`). On the window's cadence it calls `regulate_rate(count)` while the engine holds a count and `regulate(saturation_ceiling(units))` while it holds none, then `step_sleep`. `Executor::tick` returns nothing: no error can leave a tick.
3. **The modulator section** (`runtime/cortex-runtime/src/image.rs`). Nine bytes are reserved, `[26..28)`, `[39]`, `[54..56)` and `[60..64)`. **`[39]` lies inside the slow current's bytes**: `slow_current_of` reads an unset slow current as `[37..48)` all zero, so a byte written at `[39]` on an image with the slow current unset would be refused as the slow current's.
4. **A window's count.** `window_spikes()` is the closed bins', at most thirty-two at the bin's cap, `RATE_TARGET_MAX`, 536 870 880. Sixteen of them are at most 8 589 934 080, which takes thirty-three bits.
5. **The whole-image pins** (`tests/inhibition.rs`, `tests/assembly.rs`). Six images carry a pin as they are now beside a pin written back to the format their hypothesis read ([ADR-0095](0095-an-image-pin-moves-with-its-format.md)). Twenty-five are held at format 21 by the version written back ([ADR-0180](0180-a-rate-the-controller-holds-built.md)). Forty-eight more were read at format 22 and pinned as they were then: the six images of each of H-38's six steps (`RATE_CRCS`) and of H-39's derivations on seeds 23 and 24 (`SEEDED_RATE_CRCS`); `rate_pins_held` held them to `Derivation::crcs`, the images at the current format.

## Decision Drivers

- ADR-0188's decision and brief 074's standing directives: the read in the engine; unset bit for bit; no change to a rule's form, a state crate's layout or a default; no dependency, no float, no `unsafe`.
- **An image is the run** (§8.3): a read in progress written mid-read reads back to the same run.
- **The engine after the read is the configuration H-39 ran**: an image written after it carries the period and nothing of the read.
- **A structural boundary beats a reviewed one** (principle 5): a read beside a period is refused by one rule in the configuration and the loader alike.
- **Every mutant in the changed lines caught** ([ADR-0030](0030-verification-governance.md)): bounds as range patterns where a comparison would leave a mutant no test can tell apart.
- **No capacity reached silently**: a sum the bytes cannot hold, or a target the rule refuses, is refused by name.

## Considered Options

1. **The span**: (a) a parameter, `Config::rate_read_windows`, the span in windows, zero unset and at least sixteen when set, the runs setting 128; (b) a flag, the span a constant of 128.
2. **How the windows read so far are counted**: (a) the windows left, from the span down to one; (b) the windows read, up from zero, beside the span.
3. **Where the read's state sits in the image**: (a) the modulator section's `[39]`, `[54..56)` and `[60..64)`; (b) the homeostasis record; (c) a section of its own.
4. **The sum's width**: (a) forty-eight bits, the low thirty-two at `[60..64)` and the high sixteen at `[54..56)`; (b) thirty-two bits, saturating; (c) forty bits, the high byte at `[39]` and the windows left elsewhere.
5. **Who closes a held window**: (a) a method on the pool, `HomeostaticDrivePool::hold`, the rate's rule at a step of zero with the window's count returned; (b) the executor setting the step to zero for the read's windows; (c) `regulate_rate` with the window's own count as its target.
6. **The count the rule holds the windows to after the read**: (a) the period's count, `rate_target_of(units, period)`; (b) the mean itself.
7. **A mean whose period the rate's target refuses**: (a) the read ends refused, no target, the gain held from there on, and the writer refuses the engine's image with the error; (b) the process aborts; (c) the engine falls back to the estimate's rule.
8. **The whole-image pins**: (a) the six that carry a pin written back re-pinned by the mask, the twenty-five read at format 21 and the forty-eight read at format 22 held by the version written back; (b) all seventy-nine re-pinned.

## Decision Outcome

**Options 1(a), 2(a), 3(a), 4(a), 5(a), 6(a), 7(a) and 8(a).**

- **The parameter** (`runtime/cortex-runtime/src/executor.rs`): `Config::rate_read_windows: u8`, zero by default, which is unset.
  - Set, `Executor::new` starts the read under way with the span's windows left and nothing summed: `TargetRead::Reading { left, sum: 0 }`, read back by `Executor::target_read`.
  - Refused below sixteen, the windows it sums (`ConfigError::RateReadOutOfRange`), and beside a period already set (`ConfigError::RateReadBesideTarget`), after the period's own refusal. Both are range patterns over the pair, so neither edge leaves a comparison for a mutant to move.
  - *Why a span and not a flag* (option 1(b)): the span is what the read is. The image holds it as the windows left, so a flag would still need the byte, and the span's edges — sixteen, every window summed; seventeen, the first not — are tested on an engine in seconds where 128 windows take minutes. The runs set 128, as ADR-0188 has it. ADR-0188's stopping rule names an ADR on the span; this leaves it a parameter of the image, not a constant of the code.
- **The windows left** (option 2(a)): a countdown by one tested for zero, as every loop of the tree ends. A window is one of the last sixteen exactly when sixteen or fewer are left before it closes, so the span is needed only to start the count, and an image need not carry it.
- **The read's close** (`Executor::read_close`, `read_step`). On the window's cadence, while the engine holds a read, `tally` calls `read_close` in the rules' place; otherwise it calls them as it did, in one `else`.
  - `HomeostaticDrivePool::hold()` closes the window with the gain held (option 5(a)): the estimate stored by its slope where the window has one, the window's spikes returned, the window cleared, the gain left whatever the step. It is `regulate_rate` at a step of zero in every field, with the count returned in the gain's place; a test holds the two equal over 20 000 windows of the lattice. The method changes none of the record's bytes. *Why not the step set to zero* (option 5(b)): the step is the record's and the image's; zeroing it for the read would write a step the image does not carry, or make the image carry the read's step. *Why not the window's own count as its target* (option 5(c)): a target of the count is a reading of one, and its rounding would move the gain by an LSB on windows the rule's quotient does not round to one.
  - `read_step(left, sum, spikes, units)` is the arithmetic, a pure function: the window's spikes added to the sum where sixteen or fewer windows are left, one window fewer, and at the last — one left, or none — the target of the sum.
  - The target (`read_target_of`): the mean, the sum over sixteen to nearest with a half going up (`(sum + 8) >> 4`); the period, the units times the window's $2^{17}$ ticks over the mean, to nearest with a half going up, as `rate_target_of` rounds its inverse; and **the period's count, `rate_target_of(units, period)`** (option 6(a)). The engine then holds that period and that count, as though `rate_target_period_ticks` had set them, and the read is none. So an image written after the read is the configuration H-39 ran, and the engine after the read is the engine its image loads. At 1 024 units the period's count is the mean for every mean up to 11 601 a window, five times any rate the tree runs; from 11 602 it is one above. At one unit the two part from a mean of 375. *Why not the mean* (option 6(b)): an engine holding the mean would part from the engine its image loads wherever the two counts part.
  - **A mean whose period is refused** (option 7(a)): a mean of no spike, a period of no tick, a period past the width (32 768 units at a spike a window), or a period `rate_target_of` refuses. The read ends `TargetRead::Refused`: no target, and the gain held at every window from there on as through the read. `Image::encode` refuses the engine's image with `ImageError::Config(ConfigError::RateTargetOutOfRange)`, the error ADR-0188 names, so no image carries a refused read and a run that writes its image stops there. *Why not an abort* (option 7(b)): a refused read is an outcome a run can meet, not a violated invariant, and an abort cannot be tested in a process. *Why not the estimate's rule* (option 7(c)): a second rule ADR-0188 did not decide.
- **The image** (`runtime/cortex-runtime/src/image.rs`):
  - The modulator section's `[39]` is the windows left, a `u8`, zero while no read is under way (option 3(a)). The sum is a 48-bit count, its low thirty-two bits at `[60..64)` and its high sixteen at `[54..56)` (option 4(a)), every byte zero while none is. `[26..28)` stay reserved and are refused when not zero.
  - `[39]` is taken out of the slow current's bytes: `slow_current_of` reads an unset slow current as `[37..39)` and `[40..48)` all zero. The slow current's bytes are the ones they were; `[39]` was reserved within them.
  - The writer writes a read under way and refuses a refused one. The loader reads the read beside the rate's target, builds the engine with the configuration's read set to none — the image's read, under way or none, outranks the configuration's (§8.3) — and sets the image's read on it (`Executor::set_target_read`), refused beside the image's period with `ConfigError::RateReadBesideTarget`. A sum beside no window left, or above what the windows of its last sixteen already closed can count (`read_sum_max`: sixteen less the windows left, at `RATE_TARGET_MAX` each), is a record the writer never produces and is refused as one (`ImageError::ReservedNotZero`).
  - `FORMAT_VERSION` moves from 22 to **23**, with its note in the version history. A version-22 header is refused as every foreign version is (`HeaderError::ForeignVersion(22)`, L-6); its zeros at the read's seven bytes read as no read under the new version.
  - *Why not forty bits* (option 4(c)): the windows left would move to `[26]`, and the reserved pair would split. *Why not thirty-two saturating* (option 4(b)): sixteen windows of 2 048 units firing every tick pass thirty-two bits, and a saturated sum would be a target reached silently. *Why not the homeostasis record or a section of its own* (options 3(b), 3(c)): the record is full, and ADR-0180 put the rate's target in this section for the reason the read goes beside it.
- **Unset, every run is the run it was.** With no read the executor calls the two rules as it did; the record is the record it was; the image's seven bytes are zero where a format-22 writer left zero. Only the header's version and seal differ.
- **The tests.**
  - `crates/cortex-homeostasis`, by hand: `hold` at five steps from zero to beyond the bound over a silent window, a doubling window and a window at the cap, the gain at each rail, the estimate stored for a slope and kept for a constant window, never the saturation rule's ceiling, a window of no bin, the window cleared and the open bin kept. Over the lattice: 20 000 windows, `hold` equal in every field to `regulate_rate` at a step of zero under any target, and its count the window's spikes.
  - `executor.rs`, by hand: `read_target_of` at the three networks' derived counts, the least sum that is a spike, a half going up at 64 units, the width's edge at 32 767 and 32 768 units, the period's count at 11 601 and 11 602 a window at 1 024 units and at 374 and 375 at one unit, the shortest period and the next, the widest sum; `read_step` at 128, seventeen, sixteen, two and one window left, a half of a spike in the mean either side, a mean of no spike, a `left` of zero; `read_sum_max` at its edges; the configuration's default, its refusals at one and fifteen windows, beside a period and in order.
  - `executor.rs`, over the lattice: 4 000 reads of spans from sixteen to 255 at units from one to 65 536 over counts on and between their edges, stepped window by window and held at each window to the windows left and the sum of its last sixteen by hand, and at the last to an oracle of ADR-0188's text in `i128` by doubling.
  - `tests/criticality.rs`: an engine of 64 units under a read of seventeen windows at an eighth over designed windows — the first a growing one, not summed, its estimate kept and its gain held; sixteen summed to 5 127, a mean of 320 — held to a record fed the same counts and held in every field after each, the windows left and the sum read back after each, the period 26 214 and its count 320 set at the last, its image loading as the configuration with that period set, and the next window regulated by the rate's rule to 64 179 on both; a silent engine of one unit at the largest step holding its gain at 1.0 through 128 windows, ending refused, holding it after, and its image refused; a recurrent network written mid-window in its read's third window, its image loading with its windows left and its sum whatever the configuration says, and the loaded engine and one on four workers reading and regulating window for window as the engine that wrote it does.
  - `tests/image.rs`: the read's seven bytes zero with no read, the image's none outranking the configuration's read; a read just begun its span at `[39]` and nowhere else; nine reads under way with a sum read back and written again the same; each byte of the sum its own part; a sum beside no read, above what its windows count, and the forty-first bit refused; a read beside a period refused whatever the configuration says; `[26..28)` refused; the image equal to its version written back to 22 but for the version and the seal; a version-22 header refused. The earlier tests that held `[39]` reserved now hold the bytes that are.
  - `crates/cortex-connectome`: the version's assertions read 23. `tests/differential.rs` and `tests/contention.rs` name the field as unset.
- **Every pinned number unchanged with the read unset, but a whole-image pin with the format** (option 8(a)).
  - A probe on the prior's image at 1 024 units, not kept in the tree, read the mask from 22 to 23 as `0x411c870fc1e6e4da` on seeds 22, 23 and 24, each image 590 656 bytes. It read from 21 to 22 the mask ADR-0180 read, `0xc3258910422b2d6e`, and from 20 to 21 ADR-0155's, which is this one: 22 and 23 differ in the version's lowest bit, as 20 and 21 do, and a CRC is linear.
  - **The six images that carry a pin written back** are re-pinned as they are now, by that mask. The check that holds every other byte of each is the one it had: the image with its header's version written back to the format its hypothesis read.

    | Image | At format 23 | At format 22, the pin before | Held to |
    | :--- | :--- | :--- | :--- |
    | H-17's inhibited image, `REVERSAL_IMAGE_CRC_1024` | `0x8719681d9de299c9` | `0xc605ef125c047d13` | `0xd5579c31308388ad` at 15 |
    | H-18's signed image, H-20's, `PUNISHED_IMAGE_CRC_1024` and `H20_IMAGE_CRC` | `0x62fe5969d2ef312f` | `0x23e2de661309d5f5` | `0x3771636d385191ac` at 16 |
    | H-21's image, `TARGET_IMAGE_CRC_1024` | `0x7851dbe9bceecc28` | `0x394d5ce67d0828f2` | `0xafe7eff2d59da51f` at 18 |
    | H-22's image, `VALUED_IMAGE_CRC_1024` | `0x29cd2491d74f85e4` | `0x68d1a39e16a9613e` | `0xbf6797857fda0809` at 19 |
    | H-23's image, H-25's and H-29's, `WINDOWED_IMAGE_CRC_1024` | `0x93616469748c989d` | `0xd27de366b56a7c47` | `0x5044ed7936a7b5f3` at 20 |
    | The frozen image H-20's arm leaves, `DRAINED_050`'s fifth field | `0xc3f009aea83c6d47` | `0x82ec8ea169da899d` | `0xd763b4a58364291e` at 17 |

  - **The forty-eight read at format 22 keep their numbers**: H-38's six steps' six images each (`RATE_CRCS`) and H-39's two derivations' (`SEEDED_RATE_CRCS`). A derivation now reads the six CRCs written back to 22 as well (`Derivation::crcs_ruled`, `RULED_READ_FORMAT`), and `rate_pins_held` holds those to the round's pins, as `derivation_held` holds `crcs_read` to H-33's and H-34's. **The twenty-five read at format 21** keep theirs as ADR-0180 held them.
  - Re-pinning all seventy-nine (option 8(b)) would move seventy-three numbers that a check can hold where they are.
- **The mutation gate** ([ADR-0030](0030-verification-governance.md)). On the pull request's run at `ddc7f2c`, 39120127812, `cargo-mutants` made 51 mutants in the changed lines: 42 caught, 8 unviable and one missed — the loader's `u64::from(high) << 32 | u64::from(low)` with `|` made `^`, which joins two halves that share no bit the same way and is the same function. `9f97fab` joins the sum from its bytes, `u64::from_le_bytes` over the four and the two, and the writer takes the same six bytes from the sum's own, so no operator is left for a mutant to move and every byte of the image is the byte it was. On the run at `8342095`, 39131012880, the last commit to change code, 47 mutants: 39 caught, 8 unviable, none missed. The measurement's ADR records the dispatched sweep's state with its evidence ([ADR-0190](0190-a-target-the-engine-reads-measured.md)).
- **What brief 074 did not empower and this does not take**: no rule's form changes, no record of a state crate changes its layout, and `Config`'s default leaves the read unset. The inhibitory rule's target is not touched.

### Consequences

- Good: ADR-0188's read exists in the engine and the image, and an engine whose read ended is the engine ADR-0180 built with a period set, its image that configuration's.
- Good: unset, the engine is the one before it by construction and by test: the executor calls the two rules as it did, in an `else`; `hold` is a new method and changes no other; the image's seven bytes are zero where a format-22 writer left zero.
- Good: an image written mid-read reads back to the same run, and a read beside a period is refused by one rule for the configuration and the loader.
- Neutral: the format is 23 for seven bytes. A format-22 image is refused by its header, and its zeros read as no read.
- Neutral: the span is a parameter where ADR-0188 wrote a constant; the runs set 128 and nothing else does.
- Bad: one parameter more, and the controller's state in two sections: the read's in the modulator section beside the target, the window's in the homeostasis record.
- Bad: a refused read leaves an engine that runs on with its gain held and cannot be written to an image; the run's harness must read `target_read` to see why.
- Bad: two bytes of the modulator section remain, `[26..28)`.

## Alternatives considered and why rejected

- **A flag** (option 1(b)), **the windows read beside the span** (option 2(b)), **the homeostasis record or a section of its own** (options 3(b), 3(c)), **thirty-two or forty bits** (options 4(b), 4(c)), **the step zeroed or the count as its own target** (options 5(b), 5(c)), **the mean held** (option 6(b)), **an abort or the estimate's rule** (options 7(b), 7(c)), **all pins re-pinned** (option 8(b)): see above.
- **The inhibitory rule's target set by the read as well**: ADR-0188's option 4(b), rejected there; brief 074 does not empower it.
- **ADR-0055's criterion in the engine**: ADR-0188's option 1(b), rejected there.

## Confirmation

- `crates/cortex-homeostasis/src/lib.rs`: `HomeostaticDrivePool::hold`; its tests by hand and over the lattice.
- `runtime/cortex-runtime/src/executor.rs`: `Config::rate_read_windows`, `ConfigError::{RateReadOutOfRange, RateReadBesideTarget}`, `TargetRead`, `READ_SUMMED_WINDOWS`, `read_target_of`, `ReadStep`, `read_step`, `read_sum_max`, `Executor::{target_read, set_target_read, read_close}`, `tally`; the tests named above and the lattice property `read_prop`.
- `runtime/cortex-runtime/src/image.rs`: `RATE_READ_LEFT`, `RATE_READ_SUM_LOW`, `RATE_READ_SUM_HIGH`, `MODULATOR_RESERVED`, `SLOW_CURRENT_BYTES`, `target_read_of`.
- `crates/cortex-connectome/src/lib.rs`: `FORMAT_VERSION` 23 and its note; `SECTION_MODULATOR`'s comment.
- `runtime/cortex-runtime/tests/criticality.rs`, `tests/image.rs`; `tests/inhibition.rs` and `tests/assembly.rs`: the six constants above, `RULED_READ_FORMAT`, `Derivation::crcs_ruled`.
- Whitepaper §5.2.2 (the format's version row), §5.2.16, §8.7, §8.8, §9; `CHANGELOG.md`; `CLAUDE.md`; the image format 23.
