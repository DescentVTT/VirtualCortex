---
status: accepted
date: 2026-09-10
decision-makers: VirtualCortex maintainers
amended-by: ADR-0016
---

# ADR-0005: One crate per subsystem with no inter-crate dependencies among state crates

> Amended by [ADR-0016](0016-thirty-two-crate-architecture.md) (2026-09-10), which admits fourteen further crates under a six-part admission test and locks the count at thirty-two. The decision, one crate per subsystem with empty dependency tables, stands; the count of eighteen below describes the founding partition.

## Context and Problem Statement

The engine models many functionally distinct brain systems. They could live in one crate as modules, or each in its own crate. The choice affects compile times, testability of layout contracts, and how a future runtime composes them.

## Decision Drivers

- Each subsystem's record layout is an independent ABI contract that should be testable in isolation.
- A crate boundary makes an accidental dependency visible in `Cargo.toml`.
- Reviewers and coding agents need a one-to-one map from subsystem to source path.

## Considered Options

1. A single `cortex` crate with one module per subsystem.
2. A handful of crates grouped by layer (core, subcortical, cortical, systems).
3. **One crate per subsystem, `cortex-<name>`, exporting one primary record and, where settled, one deterministic update function. State crates MUST NOT depend on each other; a future `cortex-runtime` crate composes them.**

## Decision Outcome

Option 3. Eighteen crates today. Dependencies are permitted only from a runtime crate downward, and only on a vetted allow-list (initially: `crossbeam-epoch` for ADR-0011, `rustix` or `nix` for `mmap`, `bytemuck` for `Pod` derives), each addition recorded in the changelog.

### Consequences

- Good: layout tests and `spec-guard` directives are per crate; a failure names the subsystem.
- Good: a subsystem can be replaced or removed without touching the others.
- Bad: eighteen `Cargo.toml` files to keep consistent. Mitigated: version, edition, authors, license and repository are inherited from `[workspace.package]` (finding F-9, closed), so each manifest carries only its name and description.
- Bad: cross-subsystem dynamics (neuromodulator gating of plasticity, salience tagging of hippocampal replay) must be expressed in the runtime crate, not in the state crates.

## Confirmation

`npx spec-guard` asserts that `Cargo.toml` lists exactly eighteen `crates/cortex-` members. Every crate's `[dependencies]` table is empty, which `cargo tree` confirms.
