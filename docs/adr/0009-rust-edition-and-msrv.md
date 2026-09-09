---
status: proposed
date: 2026-09-10
decision-makers: VirtualCortex maintainers
---

# ADR-0009: Rust edition 2024 and a pinned MSRV

## Context and Problem Statement

All eighteen crates declare `edition = "2021"` and no `rust-version`. The README badge claimed "Rust 2024/2026"; there is no 2026 edition. The workspace builds on `rustc 1.97.1`. Which edition and minimum supported Rust version does the project commit to?

## Decision Drivers

- "Latest is not Newest": the 2024 edition was stabilised in Rust 1.85 (February 2025) and has been the default for new crates for over a year; it is the current stable edition, not a moving target.
- An MSRV lets downstream users and CI know what is supported, and `cargo` enforces it.
- The edition affects `unsafe` hygiene (`unsafe_op_in_unsafe_fn` by default) and `static mut` lints, both relevant to future SIMD and `mmap` code.

## Considered Options

1. Stay on edition 2021 indefinitely.
2. **Move all crates to edition 2024 through `[workspace.package]` inheritance and declare `rust-version = "1.85"`; add a `rust-toolchain.toml` pinning a stable channel for reproducible CI.**
3. Track the newest edition automatically as it appears.

## Decision Outcome

Proposed: option 2. It is a code change and is recorded here rather than made silently; it becomes `accepted` when the maintainers apply it. `cargo fix --edition` performs the migration; the state crates use no constructs the edition changes.

### Consequences

- Good: `[workspace.package]` already carries version, authors, license and repository (finding F-9, closed) and the edition; adopting this decision is one edit to that table plus a `rust-version` line.
- Good: CI and contributors agree on a toolchain.
- Bad: contributors on toolchains older than 1.85 cannot build; acceptable given the toolchain in use is 1.97.

## Confirmation

When applied: `grep -r 'edition' crates/*/Cargo.toml` returns only inherited values, and `cargo metadata` reports `rust_version = "1.85"` for every crate. Whitepaper constraint TC-8 moves from Proposed to Implemented.

## Open Questions

- [ ] Should the MSRV be 1.85 (edition floor) or the oldest toolchain in the maintainers' CI matrix at the time of adoption?
