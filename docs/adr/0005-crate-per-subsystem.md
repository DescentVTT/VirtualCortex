---
status: accepted
date: 2026-09-10
decision-makers: VirtualCortex maintainers
---

# ADR-0005: One crate per subsystem with no inter-crate dependencies among state crates

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
- Bad: eighteen `Cargo.toml` files to keep consistent; today four carry metadata and fourteen do not (finding F-9). Workspace inheritance (`[workspace.package]`) is the fix.
- Bad: cross-subsystem dynamics (neuromodulator gating of plasticity, salience tagging of hippocampal replay) must be expressed in the runtime crate, not in the state crates.

## Confirmation

`npx spec-guard` asserts that `Cargo.toml` lists exactly eighteen `crates/cortex-` members. Every crate's `[dependencies]` table is empty, which `cargo tree` confirms.
