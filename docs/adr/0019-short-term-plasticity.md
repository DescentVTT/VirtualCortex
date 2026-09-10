---
status: accepted
date: 2026-09-10
decision-makers: VirtualCortex maintainers
depends-on: ADR-0012
---

# ADR-0019: Short-term plasticity — event-driven Tsodyks–Markram on the Q0.8 fields, exponentials by binary exponentiation

## Context and Problem Statement

`DendriticSuperNeuron` carried `stp_u_rel` and `stp_r_ves`, the Q0.8 release fraction and resource of the Tsodyks–Markram model, and `synaptic_efficacy_q16` combined them with a Q1.15 weight ([ADR-0012](0012-synaptic-weight-q1-15.md)); nothing updated them. Whitepaper §8.8 gave per-step shift forms in a 16-bit domain. Brief 010 asked which width the update is computed in, whether it is stepped per tick or per spike, where $U$, $\tau_f$ and $\tau_d$ live, and how the Q0.8 floor is kept from stalling the dynamics.

## Decision Drivers

- The engine is event-driven: a synapse's state matters only when a spike crosses it, and the interval since the previous spike is known (`ticks_since_spike`, [ADR-0018](0018-membrane-integration.md)). A per-tick update would touch every unit's two bytes every 10 µs for nothing.
- Exponential relaxation over an arbitrary interval must not cost that interval in operations, and must not need a table or a division ([ADR-0002](0002-q16-16-fixed-point.md)).
- The fields are one byte each; the record has four reserved bytes. Widening is a layout change (rule L-6) that the resolution does not yet justify.
- Every relaxation toward a target must reach it ([ADR-0016](0016-thirty-two-crate-architecture.md)'s lesson).

## Considered Options

1. Per-tick shift updates on every unit, as §8.8's forms read literally.
2. **Per presynaptic spike with the elapsed interval; the relaxation factor $(1 - 2^{-k})^{\Delta t}$ in Q16.16 by binary exponentiation; the fields stay Q0.8, computed in Q16.16, rounded to nearest, at least one LSB per relaxation; constants in `cortex-core`.**
3. Widen the fields to Q0.16 in the reserved bytes.

## Decision Outcome

Option 2.

- **Stepping.** `step_stp(elapsed_ticks)` is called once per presynaptic spike of the unit, with the ticks since its previous spike (`u32::MAX` for a first spike after a long rest). The runtime calls it before `integrate` stamps the new spike, or keeps the previous stamp.
- **Relaxation.** For each field, the fraction of its deviation that survives the interval is $(1 - 2^{-k})^{\Delta t}$ with $k$ the time constant's shift, computed in Q16.16 by squaring, at most 32 multiplications, and exactly 1.0 for a zero interval and 0 once nothing survives. The field moves toward its target by the vanished fraction of the gap, rounded to nearest, and by at least one LSB when the gap is not zero and the interval is not, so a rest is reached exactly and a short interval still moves; a zero interval moves nothing. `u` relaxes toward $U$; $R$ toward 255, the Q0.8 rest.
- **Spike.** $u \leftarrow u + \operatorname{round}(U(256 - u)/256)$, capped at 255; the release uses the facilitated $u$ and the recovered $R$, and the method returns that pair for `synaptic_efficacy_q16`; then $R \leftarrow R - \operatorname{round}(uR/256)$, floored at zero.
- **Constants.** $U = 51/256 \approx 0.2$, $\tau_f = 2^{14}$ ticks (164 ms), $\tau_d = 2^{15}$ ticks (328 ms), as `const` items of `cortex-core`: one synapse type for the whole engine. Per-type constants are a record question under ADR-0016's test.
- **Where the state lives.** Per presynaptic unit, shared by its fan-out, as the record has it; a per-synapse state would be a `SynapseBlock` change and is not needed until a mechanism asks for it.

### Consequences

- Good: a spike costs two exponentiations of at most 32 steps and a handful of multiplications, whatever the interval; a silent unit costs nothing.
- Good: the fixed point under a 50 Hz train is reached within sixty spikes and is a test; the first spike from rest gives exactly (92, 255) and leaves $R = 163$, which a reader can check by hand.
- Bad: Q0.8 resolution. A relaxation smaller than half an LSB rounds to nothing except for the one-LSB floor, which over-recovers a deficit of a few LSB in a short interval; at the magnitudes the model runs at the floor is invisible, and the alternative is a stall near rest.
- Bad: the truncating products of the exponentiation bias the factor slightly low: at one time constant it is 0.36 rather than 0.3679. Bounded and tested; a rounding product would double the operations.
- Bad: one set of constants; heterogeneity waits for a record.

## Alternatives considered and why rejected

- **Option 1** spends two byte updates per unit per tick for a quantity that changes only at spikes, and its per-tick shift with an 8-bit field would stall at once ($255 \gg 15 = 0$).
- **Option 3** buys 256× resolution for a layout change and a format bump that no mechanism has asked for; the reserved bytes stay reserved.
- **A lookup table for the exponential** is a 64 KB array in a `no_std` crate for a factor squaring produces in 32 steps.

## Confirmation

Six tests in `cortex-core` (`dynamics/plasticity.rs`): the factor is 1.0 at zero elapsed, near $e^{-1}$ at one time constant, zero for a long interval and monotone; the first spike from rest gives (92, 255) and leaves 163; a 50 Hz train reaches a fixed point within sixty spikes with $u$ facilitated, $R$ depleted and the efficacy through `synaptic_efficacy_q16` below the rested one; silence restores the resource and the baseline exactly and one tick still moves the last LSB; the fields never leave [0, 255]; two units given the same train stay identical over 10⁴ spikes. `npx spec-guard` asserts `step_stp` exists.
