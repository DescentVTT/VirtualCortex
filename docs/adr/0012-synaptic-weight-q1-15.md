---
status: accepted
date: 2026-09-10
decision-makers: VirtualCortex maintainers
---

# ADR-0012: Sixteen-bit synaptic base weights are Q1.15

## Context and Problem Statement

`SynapseBlock` stores four base weights in `[i16; 4]`. The field was commented as Q16.16, which needs 32 bits (whitepaper finding F-3). A weight is combined with the two `u8` Tsodyks–Markram factors (`stp_u_rel`, `stp_r_ves`, both Q0.8) and accumulated into a Q16.16 membrane potential. Which fixed-point format do the sixteen bits carry, and how is the product formed without losing the bits that matter?

## Decision Drivers

- The whitepaper's numeric model (§8.1): integer only, saturating accumulation, every narrow field with a stated Q-format.
- Whitepaper §8.8 specifies STDP acting on `SynapseBlock` in place (`last_spike_tick` exists for it), so weights are *learned*, not only loaded; resolution therefore matters as much as range.
- Biology: a single cortical synapse contributes a small fraction of the firing threshold (roughly 0.03–0.2 of it); reach comes from summation across synapses, not from any one weight.
- Mechanical sympathy ("Latest ≠ Newest", §2.1): the SIMD path will multiply thousands of weights per tick, and the instruction sets in scope have native Q15 multiply-high-with-rounding (`pmulhrsw` since SSSE3, 2006; `sqrdmulh` on AArch64).
- The block has no spare bits for a per-synapse exponent without widening the record; 24 reserved bytes exist.

## Considered Options

1. **Q8.8** — range ±128, resolution 1/256 ≈ 0.0039.
2. **Q1.15** — range [−1, 1 − 2⁻¹⁵], resolution 2⁻¹⁵ ≈ 3 × 10⁻⁵.
3. **Q4.12** — range ±8, resolution 2⁻¹² ≈ 2.4 × 10⁻⁴; a compromise with no native SIMD form.
4. **Q1.15 plus a shared per-block exponent** in the reserved bytes — range on demand, at the cost of a second multiply on every synapse.

## Decision Outcome

**Option 2, Q1.15**, named `weights_q1_15`. A weight is a signed fraction of the firing threshold; strong synapses sit near 0.2, so the format leaves five-fold headroom, and the summation over a unit's fan-in provides reach beyond 1.0.

Resolution decided it. A typical weight of 0.05 has 13 representable steps in Q8.8 and about 1 600 in Q1.15. In-place STDP applies changes of a fraction of a percent of the weight; in Q8.8 such a change is below one LSB and vanishes unless stochastic rounding is added, which the deterministic numeric model does not allow. Range, the argument for Q8.8, is served by summation and by STP scaling *down*, never up.

Q1.15 is also the canonical sixteen-bit fixed-point format in signal processing, with a documented failure mode (the single overflow case, −1 × −1) and native SIMD support on every target in scope, which is the admissibility test of whitepaper §2.1.

### The widening arithmetic

For a weight $w$ (Q1.15), release $u$ (Q0.8) and resource $R$ (Q0.8):

$$
e_{\text{Q16.16}} \;=\; \big\lfloor (w \cdot u \cdot R) \gg 15 \big\rfloor,
\qquad w \cdot u \cdot R \text{ formed exactly in } i64 .
$$

The product carries 15 + 8 + 8 = 31 fractional bits; one arithmetic shift by 15 yields 16. Nothing is rounded before the shift, so every bit of the weight survives, scaled by the coarser STP factors; the STP factors, not the weight, are the resolution floor of the path. The magnitude is bounded by 1.0, so the result always fits an `i32`, and the shift floors toward negative infinity, as §8.1 requires of all Q16.16 arithmetic. The accumulation into `v_basal` or `v_apical` is `saturating_add`.

This is implemented as `cortex_core::synaptic_efficacy_q16(w, u, r)`, a `const fn` with seven unit tests covering both extremes, the zero factors, an exact interior point, the one-LSB behaviour and the flooring direction.

### Consequences

- Good: in-place STDP has three orders of magnitude of usable resolution within a typical weight.
- Good: the SIMD implementation of the weight × STP product is one native instruction per lane on x86-64-v4 and AArch64 once the STP factors are rescaled to Q0.15 for that path.
- Bad: no single synapse can exceed one threshold unit. A connectome that needs one must express it as several synapses to the same target; the loader SHOULD warn when a source weight is clipped.
- Bad: the meaning of the weight bytes changed, so `CortexFileHeader::FORMAT_VERSION` is 2; version-1 images are refused until a migration tool exists (none is planned before the loader itself).
- The escape hatch, option 4, remains available in the reserved bytes if a future model needs range; adopting it would be a new ADR.

## Alternatives considered and why rejected

- **Q8.8**: the loss of STDP resolution described above; range it offers is not needed.
- **Q4.12**: splits the difference and gets neither the SIMD instruction nor a range anyone asked for.
- **Widening to `i32`**: changes `SynapseBlock`'s size or fan-out, which brief 003 forbade and which would halve synapse density for a resolution nobody needs.

## Confirmation

`cargo test -p cortex-core` runs the seven efficacy tests; `npx spec-guard` asserts `weights_q1_15` and `synaptic_efficacy_q16` exist in `cortex-core` (whitepaper §5.2.1 directives). The whitepaper §5.2.1 table and glossary name the format.
