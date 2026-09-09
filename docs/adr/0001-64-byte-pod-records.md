---
status: accepted
date: 2026-09-10
decision-makers: VirtualCortex maintainers
---

# ADR-0001: 64-byte cache-line POD records as the unit of state

## Context and Problem Statement

Every neural unit, synapse block and subsystem state in VirtualCortex is read and written millions of times per second by core-pinned workers. The memory subsystem transfers data in 64-byte cache lines; a record that straddles two lines costs two transfers, and a record that carries a pointer costs a dependent load on every traversal. What should the unit of state be?

## Decision Drivers

- The memory wall dominates: a DRAM miss costs ~256 cycles on the reference platform (whitepaper §1.1).
- Hardware prefetchers stream linear arrays; they cannot follow pointers.
- The same bytes must be usable in memory and on disk without a decode pass (ADR-0007).
- Bit-exact determinism across x86-64 and AArch64 requires an explicit, stable layout.

## Considered Options

1. Rust structs with natural layout and `Box`/`Vec`-owned adjacency lists.
2. Struct-of-arrays with separate columns per field.
3. **`#[repr(C, align(64))]` records of exactly 64 bytes, linked by 32-bit indices, with explicit reserved padding.**

## Decision Outcome

Option 3. Every primary state record is `#[repr(C)]`; arena records are `align(64)` and exactly 64 bytes; cross-record links are indices into arenas; trailing padding is an explicit byte array that MUST be zero; size and alignment are asserted at compile time in a `const _: () = { ... }` block.

### Consequences

- Good: one line per record, prefetch-friendly, no pointer chasing, identical layout in memory and in the `.cortex` image.
- Good: layout drift is a compile error, not a runtime surprise.
- Bad: records containing atomics (`DendriticSuperNeuron`, `EmbodimentRingBuffer`) cannot be `Copy` and are handled as control records (whitepaper §8.2, rule L-5).
- Bad: some fields are wider or narrower than ideal to hit 64 bytes; the 16-bit synaptic weight is an open finding (F-3).
- Struct-of-arrays remains available inside a subsystem where SIMD gathers need it; the 64-byte record is the interchange unit, not a ban on SoA scratch buffers.

## Confirmation

`cargo check` fails if any assertion block fails. `npx spec-guard` verifies that each crate still defines its record (whitepaper §5.2 directives).
