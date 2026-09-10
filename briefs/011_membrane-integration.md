---
status: proposed
date: 2026-09-10
---

# Brief 011 — Membrane integration: leak, threshold, refractory period (milestone M5; `cortex-core`)

## Mission

`DendriticSuperNeuron` integrates: per fine tick, the somatic potential leaks toward rest, takes
the basal and apical inputs, fires at the adaptive threshold, resets, and holds a refractory
window in which inputs are dropped; the BAC coincidence of whitepaper §8.8 (a somatic spike
within a window of apical depolarisation starts a calcium plateau of `bac_plateau_ticks`) is
implemented or explicitly left Specified with the window recorded. An ADR fixes the integer
discretisation of the leaky-integrate-and-fire equation in §8.8: leak as a right shift, the
resting and reset potentials, what happens at the tick of the spike, and how
`last_soma_spike_tick` is compared under the wrap rule of §8.4. Tests pin every rule. Milestone
M5 reaches the neuron; R-1 step 5 becomes Implemented.

## Standing directives

- **The repository wins over the document.** Record a disagreement as a numbered finding in
  whitepaper §11; do not fix it silently.
- **Verify before asserting.** Read the file; run the command.
- **Label every claim** Implemented, Specified, Target or Hypothesis.
- **Latest ≠ Newest.** Stable Rust only; no dependencies
  ([ADR-0005](../docs/adr/0005-crate-per-subsystem.md)); no `unsafe`; no floating point
  ([ADR-0002](../docs/adr/0002-q16-16-fixed-point.md)).
- **Say what you did not do** in the closing report.
- Branch and pull request; Conventional Commits with a real body; run every command in
  `CLAUDE.md` before pushing.

## Context

Re-derived against the `adr-0016-state-crate-admission` branch on 2026-09-10.

- `crates/cortex-core/src/dynamics/neuron.rs`: `v_soma`, `v_basal`, `v_apical`, `v_thresh`
  (`i32`, Q16.16, `[24..40)`), `bac_plateau_ticks: u16 [40..42)`, `refractory_ticks: u16 [42..44)`,
  `last_soma_spike_tick: u32 [44..48)`, `flags: u8 [57]` (burst mode, inhibitory), the two atomics
  and the reserved four bytes. No method updates any of them. The record is a control record; a
  worker that holds the turn (axiom A3) has `&mut` access to the plain fields, so the method
  takes `&mut self` and never touches the atomics (brief 009's domain).
- Whitepaper [§8.8](../docs/WHITEPAPER.md#88-biological-model-mapping): "Exponential leak per
  tick with integer decay; fire at threshold; hard refractory window" (Specified); the continuous
  equation with $g_L$, $E_L$, the conductances and $I_{\text{bAP}}$; BAC firing: a somatic spike
  within a coincidence window of apical depolarisation triggers a plateau of `bac_plateau_ticks`
  that converts single spikes into a burst.
- Whitepaper [§6.1](../docs/WHITEPAPER.md#61-scenario-r-1-lifecycle-of-one-spike) R-1 step 5:
  "integrate: v_basal, v_apical, v_soma; refractory; BAC coincidence" (Specified); below
  threshold reset the gate to idle, at or above emit, set `last_soma_spike_tick`, start
  `refractory_ticks`.
- Whitepaper [§8.1](../docs/WHITEPAPER.md#81-numeric-model-q1616): saturating arithmetic on
  state fields; a leak by right shift stalls at $2^k - 1$ LSB above rest unless the step takes at
  least one LSB ([ADR-0016](../docs/adr/0016-thirty-two-crate-architecture.md), lesson recorded).
- Whitepaper [§8.4](../docs/WHITEPAPER.md#84-time-model): the fine tick is 10 µs; `u32` tick
  stamps wrap; comparisons are wrapping differences read as signed.
- `cortex-basal-ganglia`, `cortex-workspace` and the fourteen crates of ADR-0016 show the house
  style for a rule with boundary tests: widen to `i64`, clamp, one `const` per parameter, a test
  that fails under plain arithmetic in a debug build.

<!-- @assert-absence target="crates/cortex-core" symbol="fn integrate" reason="precondition: no membrane integration exists in cortex-core; archive this brief when it does" -->

## Deliverables

- [ ] A new ADR at the next free number (`ls docs/adr`), `status: proposed` in the PR: the
      discrete update (leak shift per compartment, how basal and apical inputs enter the soma,
      the threshold test, the reset potential, the refractory length and whether inputs during
      it are dropped or accumulated), the BAC window and plateau (Implemented or Specified with
      the window stated), threshold adaptation (Implemented or Specified), and the wrap-safe
      comparison of `last_soma_spike_tick`. Committed `accepted` per `docs/adr/README.md`.
- [ ] `DendriticSuperNeuron::integrate(&mut self, basal_q16: i32, apical_q16: i32, now_tick: u32) -> bool`
      returning `true` on the tick the unit fires; `const` parameters named for what they are;
      doc comment stating the update in the whitepaper's notation.
- [ ] Tests: leak from a displaced potential reaches rest exactly (no stall above rest);
      constant input below threshold settles at a fixed point below threshold; sufficient input
      fires on a predictable tick and resets; inputs during the refractory window do not fire;
      the refractory countdown ends on the stated tick; saturation at both ends of every
      potential; `last_soma_spike_tick` is compared correctly across a `u32` wrap; determinism
      (two records, the same input trace, equal bytes after 10⁵ ticks); if BAC is Implemented,
      a coincidence within the window starts a plateau and one outside does not.
- [ ] Whitepaper §5.2.1 (public API, status), §6.1 step 5 Implemented, §8.8 rows (LIF and BAC),
      Appendix C M5 status; `CHANGELOG.md` entry; archive this brief.

## Not empowered

- Not to touch `gate_state`, `mailbox_head_ptr` or `mailbox_tag` (brief 009).
- Not to implement STDP, short-term plasticity (brief 010) or fan-out (milestone M3).
- Not to change the record's layout; the fields exist for this purpose. If a parameter needs a
  home the record does not have, it is a `const` in this round and the report says so.

## Architectural empowerment

You may choose a different discrete form from the continuous equation of §8.8 (for example a
single leak shift on the soma with the compartments as instantaneous inputs, or per-compartment
leaks) provided the ADR states the form, its fixed points and its stability at the 10 µs tick,
and provided every operation is integer, saturating and deterministic. The empowerment reaches
this brief's instructions, not the whitepaper's invariants or `CLAUDE.md`.

## Verification

```bash
cargo test --workspace
cargo test --workspace --release
cargo clippy --workspace --all-targets -- -D warnings
cargo fmt --all -- --check
cargo +1.85 check --workspace --all-targets
npm run spec                      # the precondition directive above must be gone (archived)
```

## Report

State: the discrete update as decided, in the whitepaper's notation; the tick a unit fires on for
a stated constant input and threshold; what BAC and threshold adaptation are (Implemented or
Specified) and why; the state of R-1 step 5 and of milestone M5; and anything in this brief that
turned out to be wrong when re-derived.
