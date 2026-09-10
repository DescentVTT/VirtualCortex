---
status: proposed
date: 2026-09-10
---

# Brief 017 — Three-factor plasticity: an eligibility trace per synapse, consolidated by the modulator; the tick duration in the header

> A self-contained prompt for one round of work. Read `briefs/README.md`, then this file, then
> the files it names. Frozen when executed; the outcome lives in the ADR that accepts it, the
> changelog and the code.

## Mission

Replace the Specified row "Three-factor plasticity" of whitepaper §8.8 with an implemented
integer rule: at a presynaptic spike, the STDP pairing amount of [ADR-0022](../docs/adr/0022-synapse-fan-out-and-stdp.md)
enters an **eligibility trace** per synapse instead of the weight, the trace decays with its own
time constant, and a **modulator** derived from the dopamine signal of `cortex-neuromod` decides
what fraction of the trace is **consolidated** into the weight. With the modulator at 1.0 the rule
is ADR-0022's rule, so the reference runs that carry no reward keep their behaviour; with the
modulator low, a pairing is remembered for hundreds of milliseconds and a reward that arrives
later consolidates it (the distal-reward problem, Izhikevich 2007). The executor composes the
modulator: one `NeuromodulatorState` per engine, a reward input between ticks, a decay per tick,
the modulation published to the workers before the fan-out phase, and the record persisted in
the image. In the same format bump, the header carries the tick duration, closing the open
question of §11.1 that [ADR-0013](../docs/adr/0013-timing-wheel-geometry.md) and
[ADR-0024](../docs/adr/0024-cortex-image-and-clock-sweep.md) left to "the round that changes the
header next".

## Standing directives

- `CLAUDE.md` in full: the repository wins over the document; label every claim; Latest ≠
  Newest; a structural boundary beats a reviewed one; say what you did not do.
- No `f32`/`f64`, no heap types, no `unsafe` in a state crate; no dependency in a state crate
  (TC-2); plain `+ - * / %` is a build error everywhere (`clippy::arithmetic_side_effects`).
- A rule is an integer rule with a boundary test and a test over the lattice of
  `testkit/prop.rs`; the mutation gate on the changed lines ([ADR-0030](../docs/adr/0030-verification-governance.md))
  must pass in CI; a survivor is a missing boundary test or an equivalent mutant removed by
  restructuring, never an `exclude_re` without a reason.
- A change to a record bumps `CortexFileHeader::FORMAT_VERSION`, the connectome test that pins
  it, the record's table in whitepaper §5.2 and the changelog, in the same pull request.
- A rule change is an ADR first (`status: proposed`, accepted on merge). Never allocate an ADR
  number here: the executing round takes the next free one (`ls docs/adr`).
- A deliberate change to the dynamics moves the pins (`PINNED_ARENA_HASH`, `PINNED_SPIKE_COUNT`)
  only with the reason stated in the ADR and the test that shows the old and the new value at
  the boundary.
- No marketing vocabulary; no claim that a rule "learns" or "understands"; the document says which
  rule moved which field.
- Commit only on a branch; open a pull request; Conventional Commits with a body; merge only
  with every check green.

## Context

Re-derived on 2026-09-10 against `main` at `400dbae`.

- `crates/cortex-core/src/dynamics/neuron.rs`: `SynapseBlock` is 64 bytes: `target_neuron_ids`
  `[0..16)`, `weights_q1_15` `[16..24)`, `delays_ticks` `[24..32)`, `next_block_idx` `[32..36)`
  (index + 1), `last_spike_tick` `[36..40)`, `last_release_q16` `[40..56)`, `apical_mask` `[56]`,
  `_reserved: [u8; 7]` `[57..64)`. Seven reserved bytes cannot hold four Q1.15 traces.
- `crates/cortex-core/src/dynamics/synapse.rs`: `step_stdp(slot, t, q)` moves the weight by the
  nearest-neighbour pair rule, saturating after each of its two terms; `step_stdp_all` runs every
  slot then stamps; `window(amplitude, ticks)` uses `stp_decay_factor_q16(ticks, STDP_TAU_SHIFT)`;
  `link` refuses only `u32::MAX`; `MAX_TOKEN_BLOCK` is $2^{26} - 1$ (finding F-23), so no chain
  index above 26 bits can be delivered and the loader (`too_many_blocks`) refuses an arena with
  more blocks.
- `crates/cortex-core/src/dynamics/plasticity.rs`: `stp_decay_factor_q16(elapsed, tau_shift)`
  computes $(1 - 2^{-\text{shift}})^{\text{elapsed}}$ in Q16.16 for a shift of at most 16.
- `crates/cortex-neuromod/src/lib.rs`: `NeuromodulatorState` is 16 bytes (`dopamine_rpe: i32`
  Q16.16 signed, three unsigned Q16.16 fields); `reward(rpe)` adds saturating; `decay_dopamine
  (shift)` moves the signal toward zero by $2^{-\text{shift}}$ and at least one LSB. The crate
  comment says the three-factor rule is Specified. Nothing in the runtime composes the crate.
- `runtime/cortex-runtime/src/executor.rs`: in phase 2 (`phase_fan_out`) the worker that owns
  the spiking unit walks its chain and calls `block.step_stdp_all(now, posts)` then
  `block.release_all(u, r)`; `Config` has nine fields with a `Default`; `Shared` carries the
  per-tick `now: AtomicU32` the workers read after the barrier.
- `runtime/cortex-runtime/tests/differential.rs` wires every synapse at `i16::MAX` (the rail) and
  pins the arena hash and the spike count of a 20 000-tick run on one and four workers (checked on
  x86-64 and AArch64). At the rail, ADR-0022's sequence clips the potentiation term and then
  applies the depression term, so a weight at the rail drifts down by the depression amount at
  every pairing whose target fired after the previous presynaptic spike.
- `runtime/cortex-runtime/src/image.rs`: `Image::encode` writes the neuron and synapse sections
  always, the delta and amendment sections when the executor has any; `Image::decode` knows the
  record size per section kind and refuses any other kind (`ImageError::Directory`); the
  synapse loader refuses a block whose reserved bytes are not zero (`ReservedNotZero`).
- `crates/cortex-connectome/src/lib.rs`: `CortexFileHeader` `[60..64)` is `_padding: u32`,
  "Reserved; MUST be zero", refused by `validate` when not zero; `FORMAT_VERSION` is 10; the
  section kinds end at `SECTION_AMENDMENT` (41). No constant in the workspace states the tick
  duration: `crates/cortex-core/src/dispatch/wheel.rs` says "10 µs" in a comment.
- `docs/WHITEPAPER.md` §8.4: "Tick sizes and the wheel geometry are `cortex-core` constants; the
  record types do not encode them … where it lives is an open question (§11.1)"; §11.1: "The
  `mmap` round, which changes the header next, owns it."
- `docs/WHITEPAPER.md` §8.8: the row "Three-factor plasticity | `cortex-neuromod` | $\Delta W =
  \eta \cdot e_{ij} \cdot M$ … | Specified"; reference 5 is Frémaux and Gerstner 2016.
- Preconditions, checked by `spec-guard` until this brief is archived:

<!-- @assert-absence target="crates/cortex-core" symbol="eligibility_q1_15" reason="brief 017 precondition: no eligibility trace exists yet" -->

<!-- @assert-absence target="crates/cortex-connectome" symbol="tick_ns" reason="brief 017 precondition: the header does not carry the tick duration yet" -->

## Deliverables

1. [ ] **The record.** `SynapseBlock` `[56..64)` becomes `eligibility_q1_15: [i16; 4]`, the
   eligibility trace per slot (Q1.15, signed, saturating). The apical mask moves into bits 28–31
   of the chain word at `[32..36)`, whose bits 0–27 keep the next block index + 1 (0 = end of
   chain); `link` refuses an index the 28 bits cannot hold, `unlink`, `set_synapse` and
   `clear_synapse` preserve the other field, `clear_synapse` zeroes the slot's trace, and
   `encode`/`decode` round-trip both. `FORMAT_VERSION` becomes 11 with the reason in its list; a
   version-10 image MUST NOT be read as version 11 (the mask at byte 56 would read as a trace).
2. [ ] **The rule** (`cortex-core`, one ADR): `decay_eligibility(elapsed)` multiplies every trace
   by $(1 - 2^{-16})^{\text{elapsed}}$ (`stp_decay_factor_q16` with a new
   `ELIGIBILITY_TAU_SHIFT` of 16, about 655 ms at 10 µs), rounded to nearest, and by at least
   one LSB toward zero when `elapsed > 0`, so a trace reaches zero exactly; `step_stdp` adds the
   pairing amount to the slot's trace instead of the weight and returns the trace; `step_stdp_all`
   decays by the ticks since the block's stamp (no decay without a stamp on record), pairs every
   slot, then stamps; `consolidate(slot, m_q16)` clamps `m` to $[0, 1]$ (`MODULATION_ONE_Q16` =
   65 536), moves `round(trace × m)` into the weight, saturating, and takes **what the weight
   absorbed** out of the trace (so the trace and the weight conserve their sum and a weight at
   the rail keeps its pending change); `consolidate_all(m)` for the four slots. With `m` = 1.0 the
   pairing's two terms sum in the trace before the weight saturates: this is the one result that
   differs from ADR-0022, and only at the rail; the ADR states it, a test shows both values, and
   the pins move with that reason.
3. [ ] **The modulator** (`cortex-neuromod`): `modulation(&self, baseline_q16) -> i32` =
   `clamp(baseline + dopamine_rpe, 0, 1.0)`; `DOPAMINE_TAU_SHIFT` (14, about 164 ms) as the
   recommended per-tick decay; `encode`/`decode` of the 16 bytes, little-endian, field by field.
   The crate's comment and §5.2.14 stop saying the rule is Specified.
4. [ ] **The composition** (`cortex-runtime`, depends on `cortex-neuromod` by path): `Config::
   modulation_baseline_q16` (default `MODULATION_ONE_Q16`, refused outside $[0, 1]$); the
   executor holds one `NeuromodulatorState`, exposes `modulator()` and `reward(rpe_q16)` between
   ticks (an input, like an injection; a run is still `(image, seed, trace)` plus the rewards the
   caller replays); at every tick it publishes `modulation(baseline)` to the workers before the
   phases and then decays the signal by `DOPAMINE_TAU_SHIFT`; phase 2 calls `consolidate_all(m)`
   after `step_stdp_all` and before `release_all`. The image gains section kind 42
   (`SECTION_MODULATOR`, one 64-byte record: the 16 bytes and 48 reserved zero bytes), written
   when the modulator is not at rest, refused when malformed (count, reserved bytes); a fork of
   the trial ([ADR-0031](../docs/adr/0031-policy-amendment.md)) decodes it like everything else,
   and the trial carries no rewards (Specified; say so).
5. [ ] **The tick in the header** (one ADR): `cortex-core` gains `TICK_NS` (10 000); the header's
   `[60..64)` becomes `tick_ns: u32`, taken by `CortexFileHeader::new`, refused by `validate` when
   zero, and refused by the loader when it differs from `TICK_NS` (a new `ImageError` variant).
   §8.4's sentence and the §11.1 question close.
6. [ ] **The exit test** (`runtime/cortex-runtime/tests/`): a two-unit network in which unit 0's
   spike makes unit 1 fire, with the baseline at 0: the weight does not move while the trace
   accumulates; a reward of 1.0 delivered between ticks consolidates the pending trace at the
   next presynaptic spike; the same reward delivered before any pairing changes nothing; the
   same reward delivered about $2^{17}$ ticks after the last pairing consolidates less than one
   delivered at once (the trace decayed); the modulator is written to and read from the image;
   a reward is not consolidated twice. With the baseline at 1.0 and no reward, the differential
   pins are restated and hold on one and four workers.
7. [ ] **Documents**: whitepaper §5.2.1 (the block's table, its API line and its rule paragraph),
   §5.2.2 (the header row, the version-11 sentence, the section kind), §5.2.14 (API, status, the
   rule with an executable assertion), §8.4, §8.7 (the section list), §8.8 (the STDP and
   three-factor rows), §11.1 (the tick question), §9 (the two ADRs), Appendix A if a row is
   needed; `README.md`, `CLAUDE.md`, the reader's guide and `CONTRIBUTING.md` where they name
   STDP or the format version; `CHANGELOG.md`; this brief archived with every box dispositioned.

## Not empowered

- To store the trace anywhere but the block (a per-synapse record quadruples the arena; ADR-0022
  rejected it), or to widen `NeuromodulatorState` (its width is a separate open question).
- To apply the trace to the weight anywhere but at the presynaptic spike, in phase 2, by the
  worker that owns the block (no reverse index exists; no other phase may hold a `&mut` to a
  block).
- To make the modulator amendable: a registry entry of [ADR-0031](../docs/adr/0031-policy-amendment.md)
  changes what the engine costs, never what it does; the baseline is the caller's argument.
- To add `#[allow]` for any lint, a dependency, a feature flag, or a nightly attribute.
- To move a pin without the ADR's reason and the test that shows the boundary.
- To decide the `mmap` path, core pinning, the broker, or the lexicon.

## Architectural empowerment

- The executing round may choose a different home for the apical bits (a bit per slot in the
  delay word, whose values are below $2^{12}$) if it names why in the ADR; the trace stays in
  the block at Q1.15.
- It may choose the trace's time constant, the dopamine decay, and the rounding of the
  consolidation differently, with the reason and the boundary test, and may choose a
  consolidation that scales by the interval since the last consolidation if it shows that the
  fixed fraction is wrong for a stated input.
- It may keep the section kind for the modulator out of the image and instead require the
  modulator at rest at a quiescent point, if it shows that a persisted signal breaks a stated
  invariant of the writer or the trial.
- It may leave deliverable 5 out if the header must not change this round for a reason the ADR
  states; the §11.1 question then stays open and says so.

## Verification

```bash
cargo check --workspace --all-targets --locked
cargo test --workspace --locked
cargo test --workspace --release --locked
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --locked -- -D warnings
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --locked
cargo bench -p cortex-bench --bench hot_path --locked -- --test
cargo +1.85 test --workspace --locked
npm run spec
git diff main...HEAD > target/pr.diff && cargo mutants --workspace --in-diff target/pr.diff
```

Every command exits 0; the last one reports no survivor in CI (a local Windows run marks
mutants whose build failed as unviable and is not the gate's truth). `npm run spec:deps` still
reports no dependency in a state crate. The two precondition directives above are gone with
the archived brief.

## Report

The closing message states: the record's new layout and why the mask moved; the rule with
its constants and the one result that differs from ADR-0022; which pins moved, from what to
what, and why; the exit test's outcome; the format version; what the mutation gate found on
the changed lines and how each survivor was answered; what was not done (the trial's rewards,
the columns' own modulators, the other three signals of the record) and why.
