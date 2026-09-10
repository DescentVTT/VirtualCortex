---
status: proposed
date: 2026-09-10
---

# Brief 016 — Migrate the remaining crates to `clippy::arithmetic_side_effects`

> A self-contained prompt for one round of work. Read `briefs/README.md`, then this file, then
> the files it names. Frozen when executed; the outcome lives in the ADR that accepts it, the
> changelog and the code.

## Mission

Move every state crate and the runtime under `#![deny(clippy::arithmetic_side_effects)]`, so
that whitepaper §8.1 (every operation on a state field saturates, or wraps by name) is a
compiler error everywhere and not a review item, and close finding F-4 rather than narrow it
again. ADR-0029 denied the lint in the twelve crates that passed it and three more followed on
2026-09-10; seventeen crates and the runtime remain.

## Standing directives

- `CLAUDE.md` in full: the repository wins over the document; label every claim; Latest ≠
  Newest; a structural boundary beats a reviewed one; say what you did not do.
- No `f32`/`f64`, no heap types, no `unsafe` in a state crate (the workspace lints hold this).
- A change to a rule's arithmetic is a change to the rule only if the result differs for some
  input. A conversion that preserves every result (`x + 1` → `x.wrapping_add(1)` where the
  wrap cannot occur, `a - b` → `a.saturating_sub(b)` where `a >= b` is already checked) is
  not a rule change and needs no ADR; a conversion that changes a result is a rule change and
  needs one, with the boundary test that shows it.
- Every converted site is named in the commit body with the operation it now performs by name
  (saturating, wrapping, checked with its fallback) and why that is the right one.
- The mutation gate on the changed lines (ADR-0030) must pass; a converted line whose mutants
  survive is a line without a boundary test.
- Commit only on a branch; open a pull request; Conventional Commits with a body.

## Context

Counts of sites the lint reports with `cargo clippy -p <crate> --all-targets -- -W
clippy::arithmetic_side_effects` on 2026-09-10 (deduplicated by span; "non-test" is before the
first `#[cfg(test)]`):

| Crate | Sites | Non-test | Notes |
| :--- | ---: | ---: | :--- |
| `cortex-sensory` | 1 | 0 | a test counter (`produced += 1`) |
| `cortex-homeostasis` | 1 | 1 | a division by a count already checked non-zero |
| `cortex-neuromod` | 1 | 1 | `decay_dopamine`'s `d - d.signum() * step` in `i64` (bounded) |
| `cortex-thalamus` | 1 | 1 | an `i32 × i32` product widened to `i64` |
| `cortex-arithmetic` | 2 | 1 | `wrapping_rem` after the zero check (`checked_rem(..).unwrap_or(0)` also gives `MIN % -1 = 0`) |
| `cortex-imagination` | 2 | 2 | the wander fraction and its product |
| `cortex-knowledge` | 2 | 2 | guarded subtractions in `note_anomaly` (`abs_diff`) |
| `cortex-symbolic` | 2 | 2 | `rebase`'s literal `u16::MAX as u32 + 1` and the modular sum |
| `cortex-workspace` | 3 | 3 | ignition and persistence steps |
| `cortex-connectome` | 4 | 4 | the CRC loop and the section arithmetic |
| `cortex-social` | 5 | 5 | the trust and sincerity averages |
| `cortex-cerebellum` | 7 | 6 | the ring and the learning step |
| `cortex-affect` | 8 | 8 | the load, mood, stake and mirth averages |
| `cortex-linguistic` | 11 | 11 | the recurrent cell and the role loops |
| `cortex-reasoning` | 16 | 15 | the term arena's indices and bounds |
| `cortex-embodiment` | 30 | 30 | the voice's series and the ring buffer |
| `cortex-core` | 94 | 90 | integration, plasticity, the wheel, the encodings |
| `cortex-runtime` | 167 | 167 | the phases, the deque, the injector, the loader |

The lint's own exemptions: a division or remainder by a non-zero literal, and shifts by a
literal below the width, are not reported. Index arithmetic on `usize` is reported and is the
bulk of the runtime's count.

## Deliverables

1. The nine crates with at most three sites converted and denied, in one pull request: each
   site converted to the named operation the table suggests (or a better one, with the reason),
   the boundary test that fails under the plain operation where one is missing, and
   `#![deny(clippy::arithmetic_side_effects)]` in the crate's `lib.rs`.
2. The five crates with four to sixteen sites, one pull request each or two per request, the
   same way; `cortex-reasoning`'s index arithmetic may use `checked_*` with `Malformed` as the
   fallback where an index is data.
3. `cortex-embodiment` and `cortex-core`, one pull request each; the voice's series keep their
   pinned values (the render pin of `vocal.rs` and the coefficient pins must not move unless a
   result was wrong, in which case the change says so).
4. The runtime last: index arithmetic on `usize` by `wrapping_*` where the bound is asserted a
   line above, `checked_*` where it is data, with the differential and contention tests and the
   arena pin unchanged.
5. Whitepaper: the F-4 row closes ("Resolved: the lint is denied in every crate"); the
   directive's `min` rises with each request and ends at `expected="32"` plus the runtime;
   `CONTRIBUTING.md`'s row names the lint alone; the changelog entry per request lists the sites.

## Not empowered

- To change a result a rule produces without an ADR and a test that shows the old and the new
  value at the boundary.
- To add `#[allow(clippy::arithmetic_side_effects)]` anywhere: a site that cannot be written by
  name is a finding, not an exemption.
- To move a pin (`PINNED_ARENA_HASH`, `PINNED_SPIKE_COUNT`, `PINNED_RENDER_HASH`, the resonator
  coefficients) without saying which result changed and why it was wrong.
- To add a dependency, a feature flag, or a nightly attribute.

## Architectural empowerment

- A helper such as `fn add_q16(a: i32, delta: i64) -> i32` (widen, clamp) may be shared within
  a crate; it may not become a crate, since state crates declare no dependencies (TC-2).
- Where a `usize` index is computed from data, the computation may be replaced by a lookup
  that cannot overflow (`get(i)` with the fallback the loader already uses).

## Verification

```bash
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --locked
cargo test --workspace --release --locked
cargo +1.85 test --workspace --locked
npm run spec
git diff main...HEAD > target/pr.diff && cargo mutants --workspace --in-diff target/pr.diff
```

Every command exits 0; the last one reports no survivor. The whitepaper directive that counts
`deny(clippy::arithmetic_side_effects)` rises with each request; when the last request lands,
`cargo clippy -p <every crate> -- -W clippy::arithmetic_side_effects` reports nothing.

## Report

The closing message states, per request: which sites were converted and to what, which
conversions changed a result (with the ADR), which pins moved and why, the mutation gate's
outcome on the changed lines, and what was not done and why.
