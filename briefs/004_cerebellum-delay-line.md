---
status: proposed
date: 2026-09-10
---

# Brief 004 — A real cerebellar forward model: compare the prediction made at t with the observation at t + d

## Mission

`CerebellarMicrozone::step_forward_model` predicts the sensory consequence of a motor command
and, `d` ticks later, compares that prediction with what actually arrived; the climbing-fibre
error drives the parallel-fibre weight so that the prediction converges on a linear plant; the
Purkinje output is produced rather than passed through; a convergence test proves it; whitepaper
finding **F-8** is Resolved and §5.2.6's status line no longer says "placeholder".

## Standing directives

- **The repository wins over the document.** Record a disagreement as a numbered finding in
  whitepaper §11; do not fix it silently.
- **Verify before asserting.** Read the file; run the command.
- **Label every claim** Implemented, Specified, Target or Hypothesis.
- **Latest ≠ Newest.** Stable Rust only; no new dependencies
  ([ADR-0005](../docs/adr/0005-crate-per-subsystem.md)). Q16.16, saturating, no floats
  ([ADR-0002](../docs/adr/0002-q16-16-fixed-point.md)).
- **Say what you did not do** in the closing report.
- Branch and pull request; Conventional Commits with a real body; run every command in
  `CLAUDE.md` before pushing.

## Context

Re-derived against `main` on 2026-09-10.

- `crates/cortex-cerebellum/src/lib.rs`: `step_forward_model(&mut self, current_sensory, motor_command) -> i32`
  computes `forward_model_pred = current_sensory + (motor_command >> 2)` and then
  `climbing_fiber_error = current_sensory - forward_model_pred` **from the same sample**, so the
  error is identically `-(motor_command >> 2)` and carries no information about the plant. The
  function returns `purkinje_output_rate`, which nothing ever writes. Whitepaper
  [§5.2.6](../docs/WHITEPAPER.md#526-cortex-cerebellum--forward-models) and F-8 say exactly this.
- Brief 001 made the arithmetic saturating and added four tests; one of them,
  `error_is_the_negated_command_quarter_and_drives_ltd`, **pins the placeholder** and must be
  replaced by this round, not kept.
- The record is 64 bytes (rule L-1): `microzone_id`, `purkinje_output_rate`, `mossy_fiber_input`,
  `granule_expansion_code` (u32), `climbing_fiber_error`, `ltd_synaptic_weight`,
  `forward_model_pred`, `lead_compensation_q16`, `_reserved: [u8; 32]`. The 32 reserved bytes are
  the only room for a delay line; repurposing them changes the record and therefore bumps
  `CortexFileHeader::FORMAT_VERSION` (rule L-6, [ADR-0007](../docs/adr/0007-cortex-image-format.md))
  and the §5.2.6 table.
- Intended dynamics, whitepaper [§8.8](../docs/WHITEPAPER.md#88-biological-model-mapping): granule
  expansion, Purkinje readout, climbing-fibre LTD, "prediction compared with delayed observation".
  Reference equation: $\Delta W_{\text{PF-PC}} = -\eta_{\text{LTD}} \cdot \text{PF}(t) \cdot \text{CF}(t) + \eta_{\text{LTP}} \cdot \text{PF}(t) \cdot [1 - \text{CF}(t)]$.
- Time model, [§8.4](../docs/WHITEPAPER.md#84-time-model): fine tick 10 µs; a plant delay `d` of a
  few ticks to a few hundred ticks is realistic.

<!-- @assert-count target="crates/cortex-cerebellum" symbol="error_is_the_negated_command_quarter_and_drives_ltd" min="1" reason="precondition: F-8 is open and the placeholder-pinning test still exists; archive this brief when it is replaced" -->

## Deliverables

- [ ] A delay line of `D` past predictions inside the record (a ring in the reserved bytes, `D`
      chosen and justified: eight `i32` slots fit; state what horizon that gives at 10 µs ticks
      and how a longer plant delay would be handled). The record stays exactly 64 bytes and every
      `const _` assertion still holds.
- [ ] `step_forward_model` rewritten so that at tick `t` it (a) forms the prediction for `t + d`
      from `current_sensory` and `motor_command` through a learned gain in `ltd_synaptic_weight`,
      (b) pushes it into the delay line, (c) computes `climbing_fiber_error` as the difference
      between the observation now and the prediction made `d` ticks ago, (d) applies the LTD/LTP
      rule to the gain with saturating Q16.16 arithmetic, and (e) writes and returns
      `purkinje_output_rate` as the compensation signal. `lead_compensation_q16` carries `d` or
      the lead; say which.
- [ ] Tests: the placeholder test replaced; a convergence test against a linear plant
      (`sensory(t + d) = sensory(t) + k · command(t)`) showing the error magnitude falls below a
      stated bound within a stated number of ticks for at least two values of `k` and of `d`;
      a test that the first `d` ticks produce no error (nothing to compare yet); saturation tests
      at the extremes as in brief 001.
- [ ] `CortexFileHeader::FORMAT_VERSION` bumped to 3 with its doc comment extended; whitepaper
      §5.2.2 and §8.7 updated to match.
- [ ] Whitepaper §5.2.6: layout table, function description, status line; §8.8 cerebellum row to
      Implemented (forward model and LTD) · Specified (granule expansion); §11 F-8 Resolved.
- [ ] `CHANGELOG.md` entry under Unreleased.
- [ ] Archive this brief.

## Not empowered

- Not to change the record size, alignment or any field outside the reserved bytes.
- Not to implement granule-layer sparse expansion or hashing; `granule_expansion_code` stays as it
  is and the gain remains a single scalar.
- Not to add a dependency, a floating-point type or a fixed-point library.
- Not to touch the other four update functions or their tests.

## Architectural empowerment

If you conclude that the delay line does not belong inside the 64-byte record (for example
because a realistic plant delay exceeds what eight slots give), you may instead specify a
separate per-microzone delay arena addressed by index, write a new ADR at the next free number
(`ls docs/adr`), `status: proposed`, with the memory cost per microzone and the cache argument,
and implement the in-record version anyway as the first step so that F-8 closes in this round.
The empowerment reaches this brief's instructions, not the whitepaper's invariants or `CLAUDE.md`.

## Verification

```bash
cargo test --workspace            # convergence test passes in debug
cargo test --workspace --release
cargo clippy --workspace --all-targets -- -D warnings
cargo fmt --all -- --check
npm run spec                      # the precondition directive above must be gone (archived)
```

## Report

State: the delay-line design (`D`, horizon, where `d` lives); the update rule as implemented and
its constants; the convergence figures (bound, ticks, `k`, `d`) as *measured by the test*, labelled
as a test result and not a performance figure; the format-version bump; the state of F-8; whether
an arena ADR was proposed; and anything in this brief that turned out to be wrong when re-derived.
