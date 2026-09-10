---
status: archived
date: 2026-09-10
---

> **Executed 2026-09-10 in pull request #26.** Writes
> [ADR-0022](../../docs/adr/0022-synapse-fan-out-and-stdp.md); R-1 step 6 is Implemented,
> milestone M3's exit test passes as `crates/cortex-core/tests/oscillator.rs`, the image format
> is version 6 and finding F-23 (the token width against Appendix A) is opened. The report is in
> the pull request and in `CHANGELOG.md`. The body below describes the tree before execution and
> is not maintained, apart from relative links, which gained one `../` so that they still resolve
> from `archive/`.

# Brief 013 — Fan-out through `SynapseBlock` chains and pair-based STDP (milestone M3)

## Mission

A spike leaves a unit: `cortex-core` walks the unit's `SynapseBlock` chain by index, bounded
and without allocation, yielding each target with its weight and delay, so that the caller can
schedule each into the wheel (delay > 0) or push it into a mailbox (delay = 0); and the synaptic
weights learn: a pair-based STDP rule on `SynapseBlock::last_spike_tick` and the unit's
`last_soma_spike_tick`, with exponential windows by the binary exponentiation of
[ADR-0019](../../docs/adr/0019-short-term-plasticity.md), in saturating Q1.15. An ADR fixes the
walk's termination, the token encoding for a delayed delivery, the STDP window, amplitudes and
pairing rule. Milestone M3's exit test passes: a three-neuron delayed oscillator whose period is
exact to the tick. Whitepaper R-1 step 6 is Implemented.

## Standing directives

- **The repository wins over the document.** Record a disagreement as a numbered finding in
  whitepaper §11; do not fix it silently.
- **Verify before asserting.** Read the file; run the command.
- **Label every claim** Implemented, Specified, Target or Hypothesis.
- **Latest ≠ Newest.** Stable Rust only; no dependencies
  ([ADR-0005](../../docs/adr/0005-crate-per-subsystem.md)); no `unsafe`; no floating point
  ([ADR-0002](../../docs/adr/0002-q16-16-fixed-point.md)).
- **Say what you did not do** in the closing report.
- Branch and pull request; Conventional Commits with a real body; run every command in
  `CLAUDE.md` before pushing.

## Context

Re-derived against `main` (`20c6243`) on 2026-09-10.

- `crates/cortex-core/src/dynamics/neuron.rs`: `SynapseBlock { target_neuron_ids: [u32; 4],
  weights_q1_15: [i16; 4], delays_ticks: [u16; 4], next_block_idx: u32, last_spike_tick: u32,
  _reserved: [u8; 24] }`; `DendriticSuperNeuron::synapse_slab_idx` is the first block. Whitepaper
  §5.2.1: "blocks chain by index; sentinel for end", the sentinel unnamed. Nothing walks a chain.
- `synaptic_efficacy_q16(w_q1_15, u_q0_8, r_q0_8)` and `step_stp` give the release per spike
  ([ADR-0012](../../docs/adr/0012-synaptic-weight-q1-15.md), ADR-0019); `integrate` stamps
  `last_soma_spike_tick` ([ADR-0018](../../docs/adr/0018-membrane-integration.md));
  `ticks_since_spike` is the wrap-safe difference (§8.4).
- `FlatTimingWheel::schedule(delay_ticks, token)` takes a 28-bit token and refuses
  `ZeroDelay`, `BeyondHorizon` (2 560 ticks) and `TokenTooLarge`
  ([ADR-0013](../../docs/adr/0013-timing-wheel-geometry.md)); §6.2: the connectome loader MUST
  reject a delay beyond the horizon at load. A delayed delivery must carry enough to deliver:
  the target unit and the synapse's efficacy or its block and slot; 28 bits hold a unit index
  or a block offset, not both.
- Whitepaper [§8.8](../../docs/WHITEPAPER.md#88-biological-model-mapping): STDP "pre-before-post
  potentiates; post-before-pre depresses; windowed by tick difference" (Specified); three-factor
  plasticity (`cortex-neuromod`) multiplies an eligibility trace by a modulator (Specified, out
  of this round). Whitepaper §8.1: weights are Q1.15 in $[-1, 1)$, arithmetic saturating.
- `stp_decay_factor_q16(elapsed, tau_shift)` (ADR-0019) is the exponential window any pair
  rule needs; reuse it rather than a second exponentiation.
- The M3 exit test (Appendix C): three units in a ring with delays $d_1, d_2, d_3$ drive each
  other above threshold; the observed period is $d_1 + d_2 + d_3$ plus three integration
  latencies, exact to the tick and reproducible. The harness is single-threaded and lives in
  `crates/cortex-core/tests/`, as `mailbox.rs` does; the executor (brief 012) is not needed.


## Deliverables

- [x] A new ADR at the next free number (`ls docs/adr`), `status: proposed` in the PR: the chain
      sentinel (`u32::MAX` or 0 with the `+ 1` encoding of ADR-0017, decided against the image
      at rest of §8.7); the walk's bound (the arena length, so a cycle terminates); the token
      encoding for a delayed delivery (block offset and slot, with the efficacy re-read at
      delivery, or unit index with the efficacy carried elsewhere) and what a zero delay does;
      the STDP form: nearest-neighbour or all-to-all pairing, $A_+$ and $A_-$ in Q1.15,
      $\tau_+ = \tau_- = 2^{11}$ ticks (20 ms) or as decided, the update on a presynaptic spike
      (depression against the last postsynaptic spike) and on a postsynaptic spike
      (potentiation against `SynapseBlock::last_spike_tick`), saturation at $[-1, 1)$, and the
      wrap rule for both stamps. Committed `accepted` per `docs/adr/README.md`.
- [x] `SynapseBlock::{is_end, SENTINEL}` and a `FanOut` iterator over `&[SynapseBlock]` from a
      first index, yielding `(target, weight_q1_15, delay_ticks, block_idx, slot)`, bounded by
      the arena, refusing an index outside it; `DendriticSuperNeuron::fan_out(&self, blocks)`.
- [x] `SynapseBlock::step_stdp(&mut self, slot, pre_tick, post_tick)` (or the pair of
      pre/post methods the ADR decides), saturating, with the window from
      `stp_decay_factor_q16`; `SynapseBlock::last_spike_tick` stamped on a presynaptic spike.
- [x] Tests: the walk yields every synapse of a three-block chain once and stops at the
      sentinel; a cyclic or out-of-arena chain terminates; delays beyond the horizon are the
      loader's problem and the walk yields them unchanged; STDP: pre-before-post potentiates
      and post-before-pre depresses by the window's amounts at 0, 5, 20 and 100 ms; a weight
      saturates at both ends; the stamps compare correctly across the tick wrap; determinism.
- [x] `crates/cortex-core/tests/oscillator.rs`: the three-neuron delayed oscillator, single
      thread, wheel plus mailboxes plus `integrate`, period exact to the tick over a hundred
      cycles for three delay triples; the period is asserted, not printed.
- [x] `benches/cortex-bench`: `synapse/fan_out_x4` and `synapse/step_stdp`; the README follows.
- [x] Whitepaper §5.2.1 (API, the sentinel in the layout table, the rules), §6.1 step 6 and
      §6.2 (the token as encoded), §8.8 (STDP row Implemented), §8.4 if the wrap rule gains a
      second consumer, Appendix C M3; `CHANGELOG.md`; archive this brief.

## Not empowered

- Not to implement the executor, threads or the deque (brief 012); the oscillator test is
  single-threaded.
- Not to implement three-factor plasticity or the eligibility trace; the modulator's role is
  Specified and named in the ADR as the next step.
- Not to change `DendriticSuperNeuron`'s layout; `SynapseBlock`'s 24 reserved bytes may be used
  only with the format bump of rule L-6 and the ADR's argument.

## Architectural empowerment

You may decide that a delayed delivery carries the block offset and slot (re-reading the weight
at delivery, so that STDP between scheduling and delivery is seen) rather than a precomputed
efficacy; you may decide a different window or pairing rule if the ADR shows the fixed points
and the saturation argument; you may add `eligibility` bytes to `SynapseBlock`'s reserved area
under the format bump if the ADR shows the three-factor rule needs them now. The empowerment
reaches this brief's instructions, not the whitepaper's invariants or `CLAUDE.md`.

## Verification

```bash
cargo test --workspace
cargo test --workspace --release
cargo clippy --workspace --all-targets -- -D warnings
cargo fmt --all -- --check
cargo bench -p cortex-bench --bench hot_path -- --test
cargo +1.85 check --workspace --all-targets
npm run spec                      # the precondition directive above must be gone (archived)
```

## Report

State: the sentinel, the token encoding and the STDP form as decided; the oscillator's period
for each delay triple and how many cycles held it; whether the format version moved; what the
three-factor rule still needs; and anything in this brief that turned out to be wrong when
re-derived.
