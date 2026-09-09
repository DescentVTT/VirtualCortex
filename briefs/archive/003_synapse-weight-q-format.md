---
status: archived
date: 2026-09-10
---

> **Executed 2026-09-10 in pull request #6.** Closes findings F-3 and F-12; writes
> [ADR-0012](../../docs/adr/0012-synaptic-weight-q1-15.md) (Q1.15) and bumps the image format to
> version 2. The report is in the pull request and in `CHANGELOG.md`. The body below describes the
> tree before execution and is not maintained, apart from relative links, which gained one `../`
> so that they still resolve from `archive/`.

# Brief 003 — Decide the Q-format of 16-bit synaptic weights and rename the index that calls itself a pointer

## Mission

`SynapseBlock::weights_q16` has a Q-format that fits sixteen bits, a name that says so, and a
whitepaper table and glossary that agree; `AgentPerspectiveState::intention_vector_ptr` is named as
the index it is; `CortexFileHeader::version` is bumped because two record ABIs changed in name
(and one in semantics); findings **F-3** and **F-12** are Resolved and the open question on the
weight format in whitepaper §11.1 is closed.

## Standing directives

- **The repository wins over the document.** Record a disagreement as a numbered finding in
  whitepaper §11; do not fix it silently.
- **Verify before asserting.** Read the file; run the command.
- **Label every claim** Implemented, Specified, Target or Hypothesis.
- **Latest ≠ Newest.** Stable Rust only; no new dependencies.
- **Say what you did not do** in the closing report.
- Branch and pull request; Conventional Commits with a real body; run every command in
  `CLAUDE.md` before pushing.

## Context

Re-derived against `main` on 2026-09-10.

- `crates/cortex-core/src/dynamics/neuron.rs`: `pub weights_q16: [i16; 4], // [16..24) 4 static
  weights (Q16.16)`. A 16-bit field cannot hold Q16.16. Whitepaper
  [§5.2.1](../../docs/WHITEPAPER.md#521-cortex-core--neural-state-and-dispatch) records this as F-3 and
  [§11.1](../../docs/WHITEPAPER.md#111-hypotheses-and-open-questions) asks: Q8.8 for range
  (±128, resolution 1/256) or Q1.15 for resolution (±1, resolution 1/32768)?
- The whitepaper's numeric model ([§8.1](../../docs/WHITEPAPER.md#81-numeric-model-q1616)) says
  narrower fields "use a stated Q-format" and gives Q0.8 for the `u8` STP variables.
- `crates/cortex-agency/src/lib.rs`: `pub intention_vector_ptr: u64, // 8 bytes (offset 8..16)`.
  Whitepaper rule L-3 forbids pointers in records and flags the name (F-12).
- `CortexFileHeader::version` is `u32`; no constant for the current version exists in
  `crates/cortex-connectome/src/lib.rs`. Whitepaper L-6 and [ADR-0007](../../docs/adr/0007-cortex-image-format.md)
  require a bump on any record change, including a rename that changes meaning.
- Biological context for the decision: synaptic efficacies are combined with `u8` Q0.8 STP factors
  and summed into a Q16.16 membrane potential; the product must widen anyway. Range matters more
  than resolution for a static base weight that STP scales down; resolution matters if weights are
  learned by STDP in place.

<!-- @assert-count target="crates/cortex-core" symbol="weights_q16" min="1" reason="precondition: F-3 is open; the field is still named as Q16.16" -->
<!-- @assert-count target="crates/cortex-agency" symbol="intention_vector_ptr" min="1" reason="precondition: F-12 is open" -->

## Deliverables

- [x] Decide Q8.8 or Q1.15 for `[i16; 4]` weights, with the argument written in a new ADR at the
      next free number (`ls docs/adr`; `status: proposed` in the PR, accepted on merge). The ADR
      must state how a weight is
      multiplied by the Q0.8 STP factors and accumulated into Q16.16 without loss of the
      intended bits.
      Q1.15; ADR-0012. The arithmetic is also implemented as `synaptic_efficacy_q16` with tests.
- [x] Rename the field to match (`weights_q8_8` or `weights_q1_15`); update its comment.
- [x] Rename `intention_vector_ptr` to `intention_vector_idx`; update its comment.
- [x] Add `pub const FORMAT_VERSION: u32` to `CortexFileHeader` (value 2; version 1 is the layout
      described by whitepaper 3.0.0) and cite it from the whitepaper §5.2.2 table.
      `MAGIC` was added alongside it, mirroring `FabricPacketHeader::MAGIC`.
- [x] Whitepaper: §5.2.1 and §5.2.12 tables; §8.1 narrower-fields sentence; §12 glossary entry
      for the chosen format; F-3 and F-12 Resolved; the §11.1 open question checked off with the
      decision.
- [x] `CHANGELOG.md` entry under Unreleased, including the format-version bump.
- [x] Archive this brief.

## Not empowered

- Not to widen the weights to `i32` or change `SynapseBlock`'s size, fan-out (4) or any other
  field. If the analysis shows sixteen bits are insufficient, say so in the ADR as an open
  question and stop.
- Not to implement STDP or STP dynamics; this round decides a representation.
- Not to introduce a fixed-point library.

## Architectural empowerment

If you find that the right answer is neither Q8.8 nor Q1.15 (for example a shared per-block
exponent, or Q4.12), you may decide it, provided the ADR shows the widening arithmetic and the
field stays sixteen bits. The empowerment reaches this brief's instructions, not the whitepaper's
invariants or `CLAUDE.md`.

## Verification

```bash
cargo check --workspace --all-targets   # every const _ assertion still holds
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo fmt --all -- --check
npm run spec                             # both precondition directives above must be gone
```

## Report

State: the format chosen and the one-sentence reason; the widening arithmetic; the new field names;
the format version; the state of F-3, F-12 and the §11.1 question; and anything in this brief that
turned out to be wrong when re-derived.
