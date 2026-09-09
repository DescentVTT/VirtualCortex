---
status: proposed
date: 2026-09-10
---

# Brief 005 — Decide the timing wheel's geometry and slot representation, and implement the drain

## Mission

`FlatTimingWheel` has power-of-two rings so that slot selection is a mask, a decided slot
representation, a `schedule` that routes a delay to the fine or coarse ring or rejects it beyond
the horizon, and an `advance` that returns and clears the current slot and cascades the coarse
ring into the fine ring; the decision is an ADR; tests prove delivery at exactly the requested
delay, the cascade, wrap-around and rejection; whitepaper finding **F-11** is Resolved and both
timing-wheel questions in §11.1 are closed.

## Standing directives

- **The repository wins over the document.** Record a disagreement as a numbered finding in
  whitepaper §11; do not fix it silently.
- **Verify before asserting.** Read the file; run the command.
- **Label every claim** Implemented, Specified, Target or Hypothesis.
- **Latest ≠ Newest.** Stable Rust only; no new dependencies
  ([ADR-0005](../docs/adr/0005-crate-per-subsystem.md)); no allocation
  ([ADR-0003](../docs/adr/0003-zero-allocation-hot-path.md)).
- **Say what you did not do** in the closing report.
- Branch and pull request; Conventional Commits with a real body; run every command in
  `CLAUDE.md` before pushing.

## Context

Re-derived against `main` on 2026-09-10.

- `crates/cortex-core/src/dispatch/wheel.rs`: `pub struct FlatTimingWheel { cursor: usize,
  fine_ring: [u64; 200], coarse_ring: [u64; 80] }`, 2 248 bytes, `Debug` and `Default`;
  `const fn new()`; `schedule_fine(delay_ticks, event_mask)` does
  `fine_ring[(cursor + delay_ticks) % 200] |= event_mask`. There is no `advance`, no coarse-ring
  insert, no horizon check and no test.
- [ADR-0004](../docs/adr/0004-two-tier-timing-wheel.md) fixes the structure (two tiers, 10 µs and
  100 µs slots) and records the two open points: ring length 200 is not a power of two, and a slot
  is a 64-bit lane mask rather than the list of `SynapseBlock` offsets the design intended.
- Whitepaper [§6.2](../docs/WHITEPAPER.md#62-scenario-r-2-timing-wheel-tick) specifies the tick:
  advance the cursor, read and clear the slot, dispatch every set bit; every tenth fine tick drain
  one coarse slot into the fine ring; a delay beyond the coarse horizon is a load-time error.
  [§8.4](../docs/WHITEPAPER.md#84-time-model) gives the horizons (2 ms fine, 8 ms coarse) and says
  tick sizes are configuration, not properties of the type.
- [§11.1](../docs/WHITEPAPER.md#111-hypotheses-and-open-questions) asks: power-of-two rings
  (256 / 64, giving 2.56 ms / 6.4 ms) so that slot selection is a mask? And should tick sizes be
  recorded in `CortexFileHeader`?
- Appendix A row 19 budgets the *designed* wheel at 8 MB per worker (1 024 slots × offset lists)
  against the implemented 2 248 bytes. The wheel is per-worker runtime state, not part of the
  `.cortex` image, so its shape does not touch `FORMAT_VERSION`.
- The delivery path the wheel feeds (mailbox push, gate) is Specified and not part of this round.

<!-- @assert-count target="crates/cortex-core" symbol="fine_ring: [u64; 200]" min="1" reason="precondition: F-11 is open and the ring is still 200 slots; archive this brief when the geometry changes" -->

## Deliverables

- [ ] A new ADR at the next free number (`ls docs/adr`), `status: proposed` in the PR, deciding:
      the fine and coarse ring lengths (powers of two) and the horizons they give at 10 µs / 100 µs;
      the slot representation (a 64-lane mask, an offset list of fixed capacity, or a mask that
      indexes a per-worker lane table) with the memory per worker and the cost of the cascade;
      and what a "lane" denotes. It must reconcile Appendix A row 19 with the decision.
- [ ] `FlatTimingWheel` rewritten to the decided geometry with `const` ring lengths and masks;
      `schedule(delay_ticks, payload) -> Result<(), BeyondHorizon>` routing to the fine or coarse
      ring; `advance(&mut self) -> Slot` returning and clearing the current fine slot and, at every
      coarse boundary, draining the next coarse slot into the fine ring; `schedule_fine` kept or
      removed as the ADR decides, with the whitepaper following.
- [ ] Tests: an event scheduled at delay `d` is delivered on exactly the `d`-th `advance` for
      `d` across both rings and both wrap boundaries; a coarse event is not delivered early; a
      delay beyond the horizon is rejected without mutating the wheel; two wheels fed the same
      sequence produce identical slots (determinism); the wheel never allocates (it is `no_std`
      already; state that no `alloc` is linked).
- [ ] Whitepaper §5.2.1 (`FlatTimingWheel` paragraph and public API), §6.2, §8.4 (horizons),
      Appendix A row 19, [ADR-0004](../docs/adr/0004-two-tier-timing-wheel.md) consequences
      (amended by the new ADR; say so in both), §11 F-11 Resolved, §11.1 both questions checked
      off with the decision (the `CortexFileHeader` question may be answered "no, tick sizes are
      worker configuration" or deferred with a reason).
- [ ] `CHANGELOG.md` entry under Unreleased.
- [ ] Archive this brief.

## Not empowered

- Not to implement the mailbox push, the gate, or `SynapseBlock` fan-out (R-1 steps 2–6).
- Not to add a dependency or any allocation; the wheel stays a fixed-size value.
- Not to change `SynapseBlock`, `DendriticSuperNeuron` or any record; the wheel is not in the
  image.
- Not to change the tick durations (10 µs / 100 µs); only the ring lengths and slot type.

## Architectural empowerment

If the analysis shows that two tiers are wrong for the delay distribution the connectome will
carry (for example that a single 1 024-slot ring at 10 µs, 10.24 ms horizon, is simpler and
cheaper than a cascade), you may decide that, provided the ADR shows the memory per worker, the
per-tick cost, and why the cascade was not worth it; in that case the new ADR retires ADR-0004
rather than amending it, and both files carry the corresponding front-matter fields. The
empowerment reaches this brief's instructions, not the whitepaper's invariants or `CLAUDE.md`.

## Verification

```bash
cargo test --workspace
cargo test --workspace --release
cargo clippy --workspace --all-targets -- -D warnings
cargo fmt --all -- --check
npm run spec                      # the precondition directive above must be gone (archived)
```

Brief 007 tests whatever wheel API this round leaves; brief 006 benchmarks it. Run this brief
before those two, or say in their reports that it had not landed.

## Report

State: the geometry and slot representation chosen with the one-sentence reason; memory per
worker before and after; the public API as it now stands; which tests exercise which boundary; the
state of F-11 and the two §11.1 questions; whether ADR-0004 was amended or superseded; and anything
in this brief that turned out to be wrong when re-derived.
