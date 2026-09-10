---
status: accepted
date: 2026-09-10
decision-makers: VirtualCortex maintainers
---

# ADR-0009: Rust edition 2024 and a pinned MSRV

## Context and Problem Statement

All eighteen crates declared `edition = "2021"` and no `rust-version`. The README badge claimed "Rust 2024/2026"; there is no 2026 edition. The workspace builds on `rustc 1.97.1`. Which edition and minimum supported Rust version (MSRV) does the project commit to, which toolchain does CI build with, and where is each enforced?

## Decision Drivers

- "Latest is not Newest": the 2024 edition was stabilised in Rust 1.85 (February 2025) and has been the default for new crates for over a year; it is the current stable edition, not a moving target. The same rule applies to the toolchain: the compiler CI uses should be a deliberate pin, not whatever `stable` resolves to on the day.
- An MSRV lets downstream users and CI know what is supported; `cargo` refuses an older compiler with a clear message, and `clippy` gates its suggestions on it.
- The edition affects `unsafe` hygiene (`unsafe_op_in_unsafe_fn` by default) and `static mut` lints, both relevant to future SIMD and `mmap` code.

## Considered Options

1. Stay on edition 2021 indefinitely.
2. **Move all crates to edition 2024 through `[workspace.package]` inheritance, declare `rust-version = "1.85"`, and pin the working toolchain in `rust-toolchain.toml`.**
3. Track the newest edition automatically as it appears.

## Decision Outcome

Option 2, accepted on 2026-09-10 with the change that applies it.

- **Edition 2024, inherited.** `[workspace.package] edition = "2024"`; every manifest keeps `edition.workspace = true`. `cargo fix --edition` was run on the 2021 tree first and changed no source file: the state crates use no construct the edition migrates. rustfmt's 2024 style edition reordered `use` lists in five files (version sort, uppercase before lowercase); that is the whole diff to the source.
- **MSRV 1.85, inherited.** `[workspace.package] rust-version = "1.85"`; every manifest gains `rust-version.workspace = true`. 1.85 is the edition floor. The open question below is answered with it: the maintainers' matrix has one toolchain, the pin, so "the oldest toolchain in the matrix" would be 1.97 and would forbid nothing the floor does not. The one place the code already avoided a newer API, `u64::is_multiple_of` (1.87) in the timing wheel, needs no `#[allow]` any more: `clippy` reads `rust-version` and does not suggest it.
- **Resolver 3.** The workspace manifest declares the MSRV-aware resolver (Rust 1.84+), so that a future `cargo update` prefers dependency versions within the floor. The lock file did not change: the harness pinned by [ADR-0014](0014-benchmark-harness.md) already respected it.
- **Toolchain pin.** `rust-toolchain.toml` names `1.97.1` with `rustfmt` and `clippy`. rustup selects it in this checkout and installs it on first use. It is moved in its own pull request, after the whitepaper's "Toolchain verified against" line has been re-verified.
- **Enforcement.** The CI job `msrv` reads `rust-version` from `Cargo.toml`, selects that toolchain with `rustup override set` (a directory override outranks the pin file) and runs `cargo check --workspace --all-targets --locked` and `cargo test --workspace --locked`; the `rust` job installs the pinned toolchain from the file. Three `spec-guard` directives under whitepaper TC-8 hold the edition, the floor and the eighteen inheriting manifests.

### Consequences

- Good: each policy is one line in one table, and moving it is a pull request rather than a drift.
- Good: CI and contributors build with the same compiler; a difference between `rustfmt` or `clippy` versions cannot make a local green run push red.
- Good: `cargo` refuses a compiler older than 1.85 with a message naming the floor, instead of failing somewhere inside a build.
- Bad: contributors on toolchains older than 1.85 cannot build; acceptable given the toolchain in use is 1.97.
- Bad: two versions to maintain, the floor and the pin. The floor moves rarely and only for a feature the code needs; the pin moves on a cadence the maintainers choose.

## Alternatives considered and why rejected

- **Option 1**: edition 2021 is stable, but the 2024 defaults the project wants for future `unsafe` code would be opt-in lints only, and the README's edition claim would stay false.
- **Option 3**: the newest edition is exactly what "Latest ≠ Newest" rules out. An edition is admissible once it has a year of production use; 2024 has it and its successor, when there is one, will not on the day it appears.
- **MSRV equal to the pin (1.97)**: a `rust-version` that high says nothing the code needs; it would refuse downstream users on every earlier stable for no reason.
- **`channel = "stable"` in the toolchain file**: not a pin. CI would change compiler on every Rust release day, and the "Toolchain verified against" line would be false six weeks out of six.

## Confirmation

On the change that accepted this decision (2026-09-10):

- `cargo fix --edition --workspace --all-targets` on the 2021 tree changed no file.
- `cargo metadata --no-deps` reports `edition = "2024"` and `rust_version = "1.85"` for all nineteen packages.
- On `rustc 1.85.0`, installed for the purpose: `cargo check --workspace --all-targets --locked` and `cargo test --workspace --locked` pass, 70 tests, including the benchmark crate and its pinned harness.
- On the pinned `rustc 1.97.1`: check, test (debug and release), `fmt --check`, `clippy -D warnings` and the benchmark smoke run pass.
- CI: the `msrv` job described above; the `rust` job builds with the toolchain from `rust-toolchain.toml`. Whitepaper TC-8 is Implemented with three executable assertions; finding F-5 is closed.

## Open Questions

- [x] Should the MSRV be 1.85 (edition floor) or the oldest toolchain in the maintainers' CI matrix at the time of adoption? Answered: 1.85, the edition floor; see the Decision Outcome.
