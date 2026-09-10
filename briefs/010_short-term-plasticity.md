---
status: proposed
date: 2026-09-10
---

# Brief 010 — Short-term plasticity in integers (milestone M5; `cortex-core`)

## Mission

`cortex-core` implements the Tsodyks–Markram short-term plasticity update of whitepaper §8.8 on
the two Q0.8 fields of `DendriticSuperNeuron`, `stp_u_rel` and `stp_r_ves`, in saturating
integer arithmetic with the shift forms the whitepaper gives; an ADR fixes the discretisation
(working width, rounding back to Q0.8, the parameters $U$, $\tau_f$, $\tau_d$ as shifts and where
they live, per-spike or per-tick stepping); tests show facilitation, depletion, recovery and the
steady state, and the path through `synaptic_efficacy_q16` end to end. The first mechanism of
milestone M5 is Implemented, and the tick-wrap rule of §8.4 has its first consumer.

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

- `crates/cortex-core/src/dynamics/neuron.rs`: `stp_r_ves: u8` at `[58]` and `stp_u_rel: u8` at
  `[59]`, both Q0.8 (whitepaper §8.1: 0..255 maps to 0..0.996). `synaptic_efficacy_q16(w_q1_15,
  u_q0_8, r_q0_8) -> i32` forms the exact `i64` product and shifts by 15
  ([ADR-0012](../docs/adr/0012-synaptic-weight-q1-15.md)); seven tests. Nothing updates the two
  fields.
- Whitepaper [§8.8](../docs/WHITEPAPER.md#88-biological-model-mapping), reference forms:
  $u_{n+1} = u_n + \big[ U \cdot (2^{16} - u_n) \gg \tau_f \big]$ and
  $R_{n+1} = R_n - \big[ (u_{n+1} R_n) \gg 16 \big] + \big[ (2^{16} - R_n) \gg \tau_d \big]$,
  written in a 16-bit fractional domain. The fields are 8 bits; the ADR decides whether the
  update is computed in Q16.16 from the Q0.8 fields and rounded back (resolution floor 1/256 per
  step, so a small decay term can round to zero: see the shift-decay lesson of
  [ADR-0016](../docs/adr/0016-thirty-two-crate-architecture.md), "a shift-based decay takes at
  least one LSB"), or whether the fields widen (a record change, rule L-6, format bump).
- The forms are per step. Biologically $u$ facilitates at each presynaptic spike and $R$ depletes
  at each spike and recovers between spikes with $\tau_d$; a per-tick form applies recovery every
  tick and the spike terms on spike ticks. The ADR decides per-spike with an elapsed-tick
  argument (using `SynapseBlock::last_spike_tick`, a `u32` that wraps, §8.4) or per-tick.
- Parameters: $U$, $\tau_f$, $\tau_d$ as right shifts. Where they live is a decision: constants
  in `cortex-core` (one synapse type), or bits of `DendriticSuperNeuron::flags`, or a per-column
  record. A new record is a crate-admission question ([ADR-0016](../docs/adr/0016-thirty-two-crate-architecture.md));
  constants are the smallest honest answer.
- Whitepaper [§8.4](../docs/WHITEPAPER.md#84-time-model): a comparison of two `u32` tick stamps
  is their wrapping difference read as signed, never `a < b`.
- Whitepaper §8.1: arithmetic on state fields is saturating; multiplication widens to `i64`.

<!-- @assert-absence target="crates/cortex-core" symbol="step_stp" word="true" reason="precondition: no short-term-plasticity update exists; archive this brief when it does" -->

## Deliverables

- [ ] A new ADR at the next free number (`ls docs/adr`), `status: proposed` in the PR: the working
      width and rounding, per-spike or per-tick, the parameters and where they live, the
      resolution floor and its consequence (the smallest recovery that still moves), and the
      format bump if the fields widen. Committed `accepted` per `docs/adr/README.md`.
- [ ] `DendriticSuperNeuron::step_stp(&mut self, ...)` (signature per the ADR: the elapsed ticks
      or a per-tick call plus a spike flag) updating `stp_u_rel` and `stp_r_ves` in place,
      saturating, never leaving $[0, 255]$; `const` parameters named for what they are.
- [ ] Tests: a spike train at a fixed interval facilitates $u$ toward its fixed point and
      depletes $R$, and both reach a steady state within a stated number of spikes; silence
      recovers $R$ to full and relaxes $u$ to $U$; the smallest step still moves (no stall at the
      Q0.8 floor); saturation at both ends; determinism (two records, same train, equal bytes);
      the efficacy of a depleted synapse through `synaptic_efficacy_q16` is below that of a
      rested one by the expected ratio.
- [ ] Whitepaper §5.2.1 (public API, status), §8.8 row (the update becomes Implemented; the rest stays Specified), §8.1 if a
      rounding rule is added, §5.2.2 if bumped, Appendix C M5 status; `CHANGELOG.md` entry;
      archive this brief.

## Not empowered

- Not to implement STDP, the three-factor rule or membrane integration (brief 011).
- Not to add a record or a crate; a per-column parameter record is an ADR-0016 question and is
  out of this round.
- Not to change `SynapseBlock`.

## Architectural empowerment

You may widen `stp_u_rel` and `stp_r_ves` to 16 bits if the ADR shows the Q0.8 floor makes the
dynamics unusable at the intended time constants, provided the record stays 64 bytes, the four
reserved bytes are what moves, and the format version is bumped. You may choose per-spike over
per-tick or the reverse. The empowerment reaches this brief's instructions, not the whitepaper's
invariants or `CLAUDE.md`.

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

State: the discretisation decided (width, rounding, stepping, parameters) in three sentences; the
steady state reached and after how many spikes; the resolution floor and how the smallest step
was kept from stalling; whether the format version moved; the state of milestone M5; and
anything in this brief that turned out to be wrong when re-derived.
