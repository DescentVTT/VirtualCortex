---
status: accepted
date: 2026-09-10
decision-makers: VirtualCortex maintainers
---

# ADR-0002: Q16.16 fixed-point arithmetic; no IEEE-754 on the hot path

## Context and Problem Statement

Membrane potentials, synaptic efficacies, modulator levels and gating variables need fractional values. Floating point is the default choice and is wrong for this engine: `(a + b) + c != a + (b + c)` in IEEE-754, and the order of a SIMD reduction differs between AVX-512 and SVE2 code paths and between compiler versions. The engine must be bit-exact across platforms (quality goal 1).

## Decision Drivers

- Reproducibility of scientific runs and of embodied-control incidents.
- Cross-platform differential testing must be able to compare state hashes.
- Integer SIMD lanes are wider and cheaper than floating-point lanes for 32-bit values.

## Considered Options

1. `f32` everywhere with fast-math disabled and a fixed reduction order.
2. Posits or other alternative real-number formats.
3. **Q16.16 signed fixed point in `i32` (or `u32` for non-negative quantities), with `i64` widening for multiplication and saturating arithmetic on state fields.**

## Decision Outcome

Option 3. `f32` and `f64` MUST NOT appear in any crate under `crates/`. Narrower fields use a stated Q-format (`u8` short-term-plasticity variables are Q0.8). The numeric rules are in whitepaper §8.1.

### Consequences

- Good: associative, bit-exact, and identical on every target.
- Good: overflow behaviour is defined (saturate or wrap, chosen per field), not implementation-defined.
- Bad: dynamic range of ±32 768 and resolution of 2⁻¹⁶; models must be scaled into that range.
- Bad: current update functions use plain operators rather than saturating ones (finding F-4); this ADR is the rule they are to be brought into line with.

## Confirmation

`npx spec-guard` asserts the absence of `f32` and `f64` in `crates/` (whitepaper §2.2 directives). A future clippy configuration will deny `float_arithmetic` in state crates.
