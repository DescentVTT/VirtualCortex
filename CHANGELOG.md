# Changelog

All notable changes to this repository are documented here. The format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/). Crate versions follow [Semantic Versioning](https://semver.org); the architecture whitepaper carries its own version in its front matter.

This file is a historical record: `spec-graph` treats it as history, so nothing written here is an obligation.

## [Unreleased]

### Added

- **`CLAUDE.md`**: the switchboard for coding agents — eight principles, the code invariants, a reading map and the exact command set CI runs.
- **`briefs/`**: numbered, self-contained prompts for rounds of work, with a README defining the mandatory sections and the archive procedure; three seed briefs (001 saturating arithmetic, 002 `no_std` and derives, 003 synaptic weight Q-format); `scripts/check-briefs.mjs` enforces the sections and runs as `npm run spec:briefs` locally and in CI. Live briefs are also read by `spec-guard` (precondition directives) and `spec-graph` (deliverables as obligations).

### Changed

- **Findings F-9 and F-18 closed.** Crate metadata (`version`, `edition`, `authors`, `license`, `repository`) is inherited from `[workspace.package]`; each manifest keeps only `name` and `description`, and all eighteen now carry one. `cortex-sensory` gained the compile-time assertion that `SensoryEvent` is 8 bytes, 8-aligned; the whitepaper's assertion-block directive now requires 18.

- **Finding F-10 closed.** All crates formatted with `rustfmt` (whitespace and comment alignment only). `Default` implemented for `FlatTimingWheel` and `EmbodimentRingBuffer`, delegating to their `const fn new()`; `FabricPacketHeader::MAGIC` written as a byte-string literal. `cargo fmt --check` and `cargo clippy -D warnings` are now blocking in CI; the two advisory steps and their `continue-on-error` are gone.

- **Whitepaper 3.0.0.** Rewritten on the arc42 template with C4 views. Every claim now carries a status label (Implemented, Specified, Target, Hypothesis). Record layouts are transcribed from source with byte offsets. All performance figures are Targets with measurement protocols; the figures previously stated as results ("P99.99 < 35 ns", "> 120 M spikes/s", "< 100 ms cold boot") are withdrawn. The founding five axioms (virtual existence, state/compute decoupling, turn invariant, discrete delay, metabolic tiering) are restored as the solution strategy. Findings F-1 to F-17 record every disagreement between the previous specification and the tree.
- **README** rewritten to describe the repository as it is: a state-model-stage workspace, with a status table and a crate table.
- **Traditional Chinese document** replaced by a reader's guide (`docs/zh-TW/README.md`) that carries no layouts or figures, per ADR-0008. The former full translation (`docs/zh-TW/architecture-report.md`) is removed.

### Added

- Architecture decision records ADR-0001 to ADR-0011 in `docs/adr/` (MADR format).
- `CONTRIBUTING.md`, `SECURITY.md`, this changelog.
- Documentation verification tooling: `@descent-vtt/spec-guard` 0.4.0 and `@descent-vtt/spec-graph` 0.2.1 pinned in `package.json`; `.spec-graph.json` configuration; `npm run spec`.
- GitHub Actions workflow `ci.yml`: `cargo check`, `cargo test`, `spec-guard` and `spec-graph` as blocking checks; `cargo fmt` and `cargo clippy` as advisory checks until finding F-10 is closed.

### Fixed

- Every LaTeX expression in the previous whitepaper and translation contained control characters (interpreted `\t`, `\f`, `\a`, `\r`, `\b`, `\v`, `\n` escapes), so no equation rendered. Rewritten.
- Struct definitions reproduced in the previous whitepaper did not match the source for 15 of 19 types, including one 72-byte record described as 64 bytes. Replaced with tables transcribed from `crates/`.
- The README referred to a documentation checker by an absolute path on one developer's machine.

### Removed

- The claim of a "Rust 2026 edition"; no such edition exists. Edition policy is now ADR-0009 (proposed).
- The "Sovereign", "Grand", "Monumental" and "Fidelity 5.0/5.0" vocabulary and the "conscious" description of the workspace subsystem.

## Earlier history

Before this changelog existed, the repository's history is the git log: specification revisions 1.x to 2.8.0 between 2026-09-06 and 2026-09-10, culminating in the eighteen-crate workspace whose layouts this changelog's first entry documents.
