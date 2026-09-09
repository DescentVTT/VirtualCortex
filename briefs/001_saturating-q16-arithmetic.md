---
status: proposed
date: 2026-09-10
---

# Brief 001 — Saturating Q16.16 arithmetic in the five update functions

## Mission

Every arithmetic operation on a Q16.16 state field in `crates/` uses saturating (or, for phase
counters, explicitly wrapping) operations, each of the five update functions has a unit test that
exercises its boundary, and whitepaper finding **F-4** is Resolved. Finding **F-14** (no test
exercises any update function) is narrowed to whatever functions this brief does not test.

## Standing directives

- **The repository wins over the document.** Record a disagreement as a numbered finding in
  whitepaper §11; do not fix it silently.
- **Verify before asserting.** Read the file; run the command.
- **Label every claim** Implemented, Specified, Target or Hypothesis. No number is Measured without
  a committed benchmark ([ADR-0010](../docs/adr/0010-measured-or-target.md)).
- **Latest ≠ Newest.** Stable Rust only; no new dependencies in state crates
  ([ADR-0005](../docs/adr/0005-crate-per-subsystem.md)).
- **Say what you did not do** in the closing report.
- Branch and pull request; Conventional Commits with a real body; run every command in
  `CLAUDE.md` before pushing.

## Context

Re-derived against `main` on 2026-09-10.

- Whitepaper [§8.1](../docs/WHITEPAPER.md#81-numeric-model-q1616) requires saturating arithmetic on
  state fields and names the five offenders: `compute_gating` (`crates/cortex-basal-ganglia/src/lib.rs`),
  `step_forward_model` (`crates/cortex-cerebellum/src/lib.rs`), `evaluate_threat`
  (`crates/cortex-salience/src/lib.rs`), `step_ignition` (`crates/cortex-workspace/src/lib.rs`) and
  `update_circadian_tick` (`crates/cortex-homeostasis/src/lib.rs`).
- The functions use plain `+`, `-`, `+=` and `-=`, which panic on overflow in debug builds and wrap
  in release. `update_circadian_tick` masks its phase with `& 0xFFFF`, which is the one place where
  wrap is the intended semantics; it should say so with `wrapping_add` rather than rely on the mask
  alone.
- Constants in use: `0x0001_0000` = 1.0, `0x0001_8000` = 1.5, `0x0002_0000` = 2.0.
- `step_forward_model` also has a separate defect (F-8: the error is computed from the sample it
  predicted from). **Not this brief's** — see Not empowered.
- Only four crates have tests today, all layout tests (`cortex-agency`, `cortex-executive`,
  `cortex-immune`, `cortex-predictive`).

<!-- @assert-absence target="crates" symbol="saturating_add" glob="*.rs" reason="precondition: F-4 is open; archive this brief when it closes" -->

## Deliverables

- [ ] `compute_gating`: `saturating_add` / `saturating_sub`; test that `i32::MAX` drives do not panic
      and that the selection sign is preserved at saturation.
- [ ] `step_forward_model`: saturating operations only; keep the current (placeholder) formula,
      because F-8 is out of scope; test the `i32::MIN` shift case (`>> 2` of a negative value is
      arithmetic and must stay so).
- [ ] `evaluate_threat`: no arithmetic changes needed unless you find some; add the threshold test
      (2.0 exactly, 2.0 plus one LSB, conditioned weight at 1.0 exactly).
- [ ] `step_ignition`: `saturating_add` on `ignition_potential`; test that repeated evidence cannot
      overflow past `IGNITION_THRESHOLD` into a negative value.
- [ ] `update_circadian_tick`: `wrapping_add` then mask; test the wrap at `0xFFFF` and the sleep
      gate at `0xC000` and at `sensory_fatigue == 0x8000_0000`.
- [ ] Whitepaper §11: F-4 Resolved with a one-line description; F-14 narrowed to name any function
      left untested. §5.2 rows for the five crates: update the "Logic" wording if it changed.
- [ ] `CHANGELOG.md` entry under Unreleased.
- [ ] Archive this brief.

## Not empowered

- Not to fix F-8 (the cerebellum's error computation) or F-3 (the weight Q-format). Both change
  semantics or layout and have their own rounds.
- Not to add a dependency, a `build.rs`, or a macro crate for fixed-point arithmetic. Five call
  sites do not justify one; if a helper is wanted, it is a private `fn` in the crate that needs it.
- Not to change any record layout or any `const _` assertion.
- Not to lower or remove a clippy lint to make the build green.

## Architectural empowerment

If, while doing this, you conclude that Q16.16 state fields should be a newtype (`Q16(i32)`) with
saturating operator impls in `cortex-core`, you may propose it: write a new ADR at the next free
number (`ls docs/adr`), `status: proposed`, with the option analysis, and stop short of applying it
across crates in this round. The five
functions still get saturating operations in plain `i32` so that F-4 closes regardless of how the
ADR is decided. The empowerment reaches this brief's instructions, not the whitepaper's invariants
or `CLAUDE.md`.

## Verification

```bash
cargo test --workspace            # new tests pass in debug (overflow checks on)
cargo test --workspace --release  # and in release
cargo clippy --workspace --all-targets -- -D warnings
cargo fmt --all -- --check
npm run spec                      # the precondition directive above must be gone or archived
```

## Report

State: which functions now saturate, which wrap and why; which tests were added and what boundary
each exercises; the state of F-4 and F-14 after the round; whether a newtype ADR was proposed; and
anything in this brief that turned out to be wrong when re-derived.
