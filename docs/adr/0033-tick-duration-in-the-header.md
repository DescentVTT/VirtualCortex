---
status: accepted
date: 2026-09-10
decision-makers: VirtualCortex maintainers
depends-on: ADR-0024
---

# ADR-0033: The tick in the image header — its duration at `[60..64)`, refused when it is not this build's, and the clock at `[40..48)`, which the loader resumes so that every stamp keeps its meaning

## Context and Problem Statement

Every `*_ticks` field and every `*_tick` stamp of every record counts fine ticks, and whitepaper §8.4 says the tick is 10 µs; but until this decision the value lived in a comment of `dispatch/wheel.rs`, the record types did not encode it, and §8.4 and §11.1 carried the open question "Should tick sizes be recorded in `CortexFileHeader` so that an image is self-describing?", reopened when [ADR-0013](0013-timing-wheel-geometry.md) fixed the geometry as constants and the loader of [ADR-0024](0024-cortex-image-and-clock-sweep.md) did not decide where the duration lives, leaving it to "the round that changes the header next". [ADR-0032](0032-three-factor-plasticity.md) bumps the format to 11 for the synapse block and the modulator section; the header's `[60..64)` was `_padding`, reserved and zero.

A second gap showed while brief 017's exit test was written. The image carries stamps (`last_soma_spike_tick`, a block's `last_spike_tick`), every rule reads an interval as the wrapping difference between the clock and a stamp (§8.4), and the loader started the clock at zero: a unit that spiked at tick 8 000 before the write read, at the loaded engine's tick 10, as having spiked $2^{32} - 7\,990$ ticks ago, so its short-term plasticity had fully relaxed, its pairings paired with nothing, its eligibility trace decayed to zero at the first spike, and a never-spiked unit's quiet time restarted from zero for the sweep. A trial's two forks ([ADR-0031](0031-policy-amendment.md)) shared the restarted clock and so agreed with each other, and the round-trip test wrote its image before any spike, so nothing had shown it. The header's `[40..48)` was `layers_offset`, "the byte offset of the laminar section", which no writer or loader has used since [ADR-0024](0024-cortex-image-and-clock-sweep.md) gave the image a directory: a section is found through its entry.

## Decision Drivers

- §8.7: an image with a foreign meaning MUST fail closed. A delay of 300 in an image written for a 100 µs tick is 30 ms, not 3 ms; a loader that cannot tell the difference reads one as the other.
- §8.4: a stamp means something only against the clock it was taken from. A loaded image that restarts the clock changes the meaning of every stamp it carries, silently.
- Principle 6: a sentence that says a constant exists gets an assertion; "the tick is 10 µs" was a comment.
- One format bump per round: deciding both in the round that already moves the version costs no second bump.
- TC-2: `cortex-connectome` declares no dependency, so it cannot compare the header with `cortex-core`'s constant; the runtime, which has both, does.

## Considered Options

1. Leave the tick a constant of the code, the header padded, and the clock restarted at zero; an image is meaningful only to the build that wrote it, and its stamps only until the first spike.
2. **`tick_ns: u32` at `[60..64)`, set by the writer from `cortex_core::TICK_NS`, refused at zero by `validate` and refused by the loader when it differs from `TICK_NS`; and `written_tick: u64` at `[40..48)` (where `layers_offset` was), the executor's tick at the write, which the loader resumes the clock at.**
3. The whole geometry in the header (fine tick, ticks per coarse slot, ring lengths), and every stamp rebased to zero by the loader.

## Decision Outcome

Option 2.

- `cortex-core` gains `TICK_NS` (10 000), beside the wheel's geometry constants of [ADR-0013](0013-timing-wheel-geometry.md).
- `CortexFileHeader::new` takes the tick duration and the clock as its fifth and sixth arguments; `[60..64)` is `tick_ns`, `[40..48)` is `written_tick`; `validate` refuses a zero tick (`HeaderError::ZeroTick`, after the padding); the header's own crate stores what it is told.
- `Image::encode` writes `TICK_NS` and `Executor::ticks()`; `Image::decode` refuses `tick_ns != TICK_NS` with `ImageError::TickMismatch(tick_ns)` right after the header validates, and, once the arenas are loaded and before any unit is woken, resumes the clock at `written_tick` (`Executor::resume_clock`, the loader's alone). A loaded engine's `ticks()` is the writer's, its `now` the same `u32`, and every interval a rule reads is the one the writer would have read.
- A fork of a trial ([ADR-0031](0031-policy-amendment.md)) therefore starts at the tick the image was written, as the live engine did: the two forks still agree with each other, and now with the engine. The trials of `tests/amendment.rs` that set a quiet bound in fork ticks set it in engine ticks (an image written at tick 3 000 has quiet units at 3 501 ticks, not 501, at the fork's first sweep).
- Format version 11 ([ADR-0032](0032-three-factor-plasticity.md) carries the bump); a version-10 image had `[60..64)` zero, which `ZeroTick` would refuse even before the version check refused it, and its `[40..48)` zero, which would have resumed at tick 0 as before.

### Consequences

- Good: an image says what a tick is, and a build that counts another tick refuses it instead of misreading every delay and stamp; §8.4's sentence and §11.1's question close.
- Good: a loaded engine continues where the writer stopped: the same intervals, the same relaxation, the same pairings, the same quiet times; brief 017's exit test holds a loaded engine and the original to the same weight and trace after the same spike, at the same tick.
- Good: no record moved; both fields were reserved or unused.
- Bad: the geometry (ring lengths, ticks per coarse slot) is still the code's; an image written for another geometry with the same tick would be refused only by the over-horizon check, and only where a delay reaches the difference. Option 3's first half is the change when a second geometry exists.
- Bad: a sweep's quiet bound in a trial now counts from the engine's clock, which is what it should count from; a test that assumed a fork at tick zero had to say so.

## Alternatives considered and why rejected

- **Option 1** keeps the failure principle 6 names, and keeps a loader that changes the meaning of what it loads.
- **Option 3's geometry** spends three more words of the header on values no second configuration needs yet; [ADR-0013](0013-timing-wheel-geometry.md) fixes the geometry, and the format has flags (`reserved_flags`) and a version for the day it does not. **Rebasing every stamp to zero** at load touches every record twice (once to find a stamp, once to move it), needs a rule per stamp for "no spike on record" (zero, which a rebase must not produce), and gives the loaded engine a history no other engine had; resuming the clock is one store.
- **The tick as a `cortex-connectome` constant compared by `validate`**: two constants for one number in two crates that may not depend on each other; the runtime is where they meet.
- **The clock in the modulator section or a section of its own**: the clock is not a record of a crate; it is the header's, like the counts.

## Confirmation

`cortex-connectome`: a header round-trips `tick_ns` at `[60..64)` and `written_tick` at `[40..48)`; a zero tick is refused after the padding; `FORMAT_VERSION` is 11. `runtime/cortex-runtime/tests/image.rs`: a header sealed with 20 000 ns is `TickMismatch(20_000)`, one with 0 is `Header(ZeroTick)`; `TICK_NS` is 10 000. `runtime/cortex-runtime/tests/modulation.rs`: an image's header carries `TICK_NS` and the writer's tick, a loaded engine's `ticks()` is the writer's, and the loaded engine and the original reach the same weight and trace after the same spike. `runtime/cortex-runtime/tests/amendment.rs`: the trials' quiet bounds count from the written tick. A whitepaper directive asserts `TICK_NS` exists in `cortex-core` and `tick_ns` in `cortex-connectome`.
