---
status: accepted
date: 2026-09-10
decision-makers: VirtualCortex maintainers
depends-on: ADR-0023
---

# ADR-0029: Structural enforcement of the invariants that were held by review — workspace lints for `unsafe` and floats, a manifest check for dependencies, rustdoc as a gate, the arithmetic lint where it passes, pinned actions and normalised line endings

## Context and Problem Statement

Principle 5 of `CLAUDE.md` ranks the ways a rule can be held: a compile-time assertion beats a test, which beats an executable documentation directive, which beats a review comment. An audit of where each invariant of §2.2 is actually enforced found:

| Invariant | Held by | Rank |
| :--- | :--- | :--- |
| Record layout (TC-1) | `const _` blocks | compile-time |
| Edition and MSRV (TC-8) | workspace inheritance, the `msrv` job, spec-guard | structural |
| No `f32`/`f64` under `crates/` (TC-4) | spec-guard absence directives only; `runtime/` and `benches/` uncovered | directive |
| No `unsafe` under `crates/` (TC-9) | spec-guard absence directive only | directive |
| State crates declare no dependencies (TC-2) | review; `--locked` does not check it | nowhere |
| Saturating arithmetic on state fields (§8.1, F-4 "until a lint exists") | review | nowhere |
| Rustdoc builds cleanly | nothing ran `cargo doc`; F-22 was a rustdoc content defect | nowhere |

The same audit found three CI actions referenced by mutable tags, the runner image floating on `ubuntu-latest`, and every text file stored as LF but checked out as CRLF by one contributor's `core.autocrlf`, which produced a warning per file on every commit. Which of the stable practices that hold these should the workspace adopt, and where?

## Decision Drivers

- Principle 5: move each invariant to the strongest enforcement available today.
- Principle 4 (Latest ≠ Newest): `[workspace.lints]` (Cargo 1.74, 2023), `clippy::disallowed_types` (2021), `clippy::arithmetic_side_effects` (2022), `RUSTDOCFLAGS=-D warnings` and SHA-pinned actions are all stable and in wide third-party use for more than two years; nothing here is nightly, sub-1.0 without a policy, or a new dependency.
- The command set in `CLAUDE.md` and CI must stay the same set: a check in one and not the other is a gate nobody enforces.
- Zero new runtime or build dependencies.

## Considered Options

1. **Enforce with what the toolchain already provides**: workspace lints inherited by every crate, a zero-dependency manifest check beside the brief check, rustdoc as a blocking step, the arithmetic lint denied in the crates that already pass it, actions pinned to commits, `.gitattributes`.
2. Add `cargo-deny`, `cargo-hack`, Dependabot and Miri as well.
3. Leave enforcement to review and directives.

## Decision Outcome

Option 1.

- **`[workspace.lints]`** in `Cargo.toml`: `rust.unsafe_code = "forbid"` and `clippy.disallowed_types = "deny"` with `f32` and `f64` disallowed in `clippy.toml`; every state crate and the benchmark crate inherit it with `[lints] workspace = true`. The runtime crate, which holds the workspace's one `unsafe` (ADR-0023), carries its own `[lints]` table with the float lint alone. TC-4 and TC-9 are now compiler errors under `cargo clippy -D warnings`, in every crate of the workspace; the spec-guard directives stay as the document's side of the same rule.
- **`scripts/check-deps.mjs`** (zero dependencies, run by `npm run spec:deps` inside `npm run spec`): every manifest under `crates/` declares no dependency of any kind; the runtime depends on workspace crates by path and on nothing else; the benchmark crate depends on workspace crates by path and carries exactly one dev-dependency, the harness at an exact pin. TC-2 is a CI gate.
- **`cargo doc --workspace --no-deps` with `RUSTDOCFLAGS="-D warnings"`** is a step of the `rust` job and a line of the documented command set.
- **`#![deny(clippy::arithmetic_side_effects)]`** in the twelve state crates that pass it today (`cortex-attention`, `cortex-basal-ganglia`, `cortex-curiosity`, `cortex-ethics`, `cortex-executive`, `cortex-hippocampus`, `cortex-immune`, `cortex-predictive`, `cortex-salience`, `cortex-spatial`, `cortex-telemetry`, `cortex-tools`), held by a spec-guard count. The other twenty crates and the runtime carry 230 sites of plain arithmetic, most of them deliberate (index and counter arithmetic, wrapping hashes); they migrate crate by crate, each in a change that says which sites are meant to wrap. The lint closes F-4's "until a lint exists" for the crates that carry it.
- **Actions pinned to commits** (`actions/checkout` v5.1.0, `actions/setup-node` v5.0.0, `Swatinem/rust-cache` v2.9.2, each with its version in a comment) and the runner pinned to `ubuntu-24.04`. Moving a pin is a visible diff.
- **`.gitattributes`** (`* text=auto eol=lf`) and an `.editorconfig`: every text file is LF in the working copy on every platform; the index was already LF, so history does not change.

### Consequences

- Good: an `f32`, an `unsafe` block or a dependency in a state crate fails the build or the spec step, not a review; a rustdoc warning fails CI; the CI inputs are reproducible.
- Good: no new dependency; the command set in `CLAUDE.md` and CI is still one set, one line and one script longer.
- Bad: `disallowed_types` is a Clippy lint, so the float rule is enforced by the `rust` job's Clippy step, not by `cargo check`; the MSRV job does not run Clippy. The spec-guard absence directives cover the gap for `crates/`.
- Bad: contributors who had CRLF working copies see every file rewritten once after this change; the diff is empty.
- Not done: Dependabot (an integration that opens pull requests; the repository owner's decision), `cargo-deny` or an advisory check (the only third-party dependency is the harness, dev-only; low value today), Miri for the arena invariant (nightly only, fails principle 4 as a gate), a Windows CI leg (the transient linker failure `LNK1104` on fresh test binaries would need a retry policy first), branch protection requiring the three checks (a repository setting, not a file; recommended to the owner: "Main Protect" today forbids deletion and force-push only, so a merge on a red check is possible, and one happened on 2026-09-10, PR #33).

## Alternatives considered and why rejected

- `missing_docs`: about 400 sites, 359 of them record fields whose meaning lives in the whitepaper's layout tables; a second copy that drifts (principle 8).
- `clippy::arithmetic_side_effects` workspace-wide now: 230 sites, most of them index and counter arithmetic that is meant to wrap or cannot overflow; denying it everywhere at once would replace a review with `#[allow]`s nobody reads.
- `clippy::undocumented_unsafe_blocks`: the seven call sites it flags all carry `SAFETY` comments in a `let … else` shape the lint's adjacency rule rejects; ADR-0023's spec-guard count holds the comments.
- `rustfmt.toml` with `newline_style = "Unix"`: unnecessary once Git normalises the working copy, and it would fail `cargo fmt --check` on a CRLF checkout that predates `.gitattributes`.
- Option 2: each tool is admissible, but none holds an invariant of §2.2 that Option 1 leaves open; they are listed in the whitepaper's open questions, not adopted.

## Confirmation

- `cargo clippy --workspace --all-targets -- -D warnings` passes with the lints active; a scratch check that adds `let _x: f32 = 0.0;` to a state crate fails with `disallowed_types`, and an `unsafe {}` block fails with `unsafe_code` (done once by hand on this change; not a committed test, since a failing crate cannot live in the workspace).
- `npm run spec:deps` passes; adding `serde = "1"` to a state crate's manifest fails it with the file and line (checked by hand the same way).
- Whitepaper §2.2 rows TC-2, TC-4 and TC-9 name the enforcement; Appendix B lists the manifest check as V-5 and the rustdoc step; a spec-guard directive counts the twelve `arithmetic_side_effects` denials.
- `.github/workflows/ci.yml` references no mutable action tag; `git ls-files --eol` shows `i/lf w/lf` for every text file after a fresh checkout.
