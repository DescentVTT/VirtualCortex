# Contributing to VirtualCortex

Thank you for considering a contribution. This file is the map; the reasoning behind each rule lives in the [whitepaper](docs/WHITEPAPER.md) and in the [ADRs](docs/adr/README.md).

## Before you start

1. Read the whitepaper's [How to read this document](docs/WHITEPAPER.md#how-to-read-this-document) section. The four status labels (Implemented, Specified, Target, Hypothesis) govern everything written here.
2. Check [§11 Risks and technical debt](docs/WHITEPAPER.md#11-risks-and-technical-debt). Many good first contributions are already numbered there.
3. Anything that changes an architectural rule needs an ADR first (below).

## Workflow

1. Fork and branch from `main`. Branch names are free-form; commit messages are not.
2. Make the change, including its documentation, in the same pull request. A layout change without its whitepaper table and changelog entry is incomplete.
3. Run the full verification locally (below). CI runs the same commands.
4. Open a pull request. Describe what changed, which finding or milestone it addresses, and how you verified it.

### Commit messages

[Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/): `type(scope): summary`, where `type` is one of `feat`, `fix`, `docs`, `refactor`, `test`, `chore`, `ci` and `scope` is a crate name without the `cortex-` prefix (`core`, `hippocampus`), or `whitepaper`, `adr`, `briefs`, `workspace`, `specs`, `ci`.

```text
docs(whitepaper): transcribe SynapseBlock layout from source
feat(core): add saturating Q16.16 helpers
fix(cerebellum): compare prediction with delayed observation (F-8)
```

## Code rules

These are the technical constraints of whitepaper §2.2. They are checked where a tool can check them.

| Rule | How it is checked |
| :--- | :--- |
| Stable Rust, edition 2024, minimum supported version 1.85 ([ADR-0009](docs/adr/0009-rust-edition-and-msrv.md)); no nightly features. | `rust-toolchain.toml` pins the toolchain CI and contributors build with; a second CI job builds and tests on the `rust-version` in `Cargo.toml`; `spec-guard` holds the edition, the floor and the thirty-two inheriting manifests. |
| State crates declare no dependencies. The benchmark crate may carry the harness as a dev-dependency and nothing else ([ADR-0014](docs/adr/0014-benchmark-harness.md)). | `Cargo.toml` review; `cargo tree -e normal -p <crate>`; CI builds `--locked`. |
| A new state crate passes the admission test of [ADR-0016](docs/adr/0016-thirty-two-crate-architecture.md): an ADR names the gap, no existing record owns the quantity, the mechanism is in whitepaper §8.8 with the layout, every Q-format fits its width, and §1.5 and §8.10 are intact or moved by that ADR first. | `spec-guard` holds the member count at 32 in the whitepaper and the README; the pull request that adds a crate moves them and carries the ADR that admits it. |
| Performance figures are Measured only from an admissible run recorded under `docs/benchmarks/results/` ([ADR-0010](docs/adr/0010-measured-or-target.md)). | Review; the results file's `admissible:` line. |
| Every public function and associated constant has at least one unit test, and every state crate carries a `#[cfg(test)]` module. | The module: `spec-guard`, one directive per crate in whitepaper §1.6. The per-item rule: review; no tool checks it yet, so a PR that adds a public item without a test is rejected on review. |
| Every primary record is `#[repr(C)]`; arena records are `align(64)` and exactly 64 bytes; size and alignment are asserted in a `const _: () = { ... }` block. | `cargo check` fails otherwise. |
| No `f32` or `f64` anywhere under `crates/`. Use Q16.16 (whitepaper §8.1). | `spec-guard` directive in the whitepaper. |
| Every state crate is `#![no_std]`; no `Box`, `Vec`, `String` or thread spawning in state crates. | `spec-guard` directives (the `no_std` count is asserted at 32). |
| A record without atomics derives `Clone, Copy, Debug, PartialEq, Eq`; a control record derives `Debug` only. | Review; whitepaper §8.2 rule L-5. |
| No `unsafe` without an ADR naming the invariant and the test. | `spec-guard` directive; review. |
| Arithmetic on Q16.16 state fields is saturating, or explicitly wrapping for phase counters. | Review, until a lint exists (finding F-4). |
| Trailing padding is an explicit `_reserved` or `padding` byte array. | Review. |
| Changing any field of any record, including reserved bytes, bumps `CortexFileHeader::version` and gets a changelog entry. | Review. |

Formatting (`cargo fmt`) and lints (`cargo clippy -D warnings`) run in CI as blocking checks. Run them before pushing.

## Documentation rules

The whitepaper is the canonical architecture document and is governed by [ADR-0008](docs/adr/0008-documentation-governance.md).

- **Label every claim.** A sentence about the system carries Implemented, Specified, Target or Hypothesis. If you cannot decide, it is Specified at best.
- **Requirement keywords** (MUST, SHOULD, MAY) follow BCP 14 and are written in capitals only when meant that way.
- **Layout tables are transcribed from source**, never from memory. If you change a record, change its table in the same commit.
- **Numbers are Measured or Target** ([ADR-0010](docs/adr/0010-measured-or-target.md)). A Measured figure links to a benchmark in this repository and the command that produced it. Do not write "tested on".
- **Make claims executable.** When you write that something exists in the tree, add a `<!-- @assert-count ... min="1" -->` directive under the sentence; when you write that something is gone, add `<!-- @assert-absence ... -->`. See the [spec-guard README](https://www.npmjs.com/package/@descent-vtt/spec-guard) for the directive syntax.
- **English is canonical.** The Traditional Chinese file is a reader's guide; do not add layouts, figures or new claims to it.
- **Findings, not silent fixes.** If the document and the tree disagree, record a numbered finding in §11 and fix it in a separate, visible step.

### Briefs

Rounds of work are written as numbered, self-contained prompts in [`briefs/`](briefs/README.md). A brief says what done looks like, re-derives its facts with paths and a date, lists deliverables as checkboxes, states what the round is not empowered to do and what it may override (as an ADR), and says how it is verified and reported. When executed it is moved to `briefs/archive/` with a frozen banner and every deliverable dispositioned; `spec-graph` fails CI on an archived brief that still holds open work. See that README for the sections every brief carries; `npm run spec:briefs` enforces them.

### Architecture decision records

`docs/adr/` holds one [MADR](https://adr.github.io/madr/) file per decision. To propose one: copy the newest file, take the next number, set `status: proposed`, fill in context, drivers, options, outcome and confirmation, and open a pull request. It becomes `accepted` on merge with that status. Superseding a decision means adding `supersedes: ADR-NNNN` to the new record and `superseded-by:` to the old one; `spec-graph` reports the omission. Never renumber, never delete.

## Verification

All of these must pass before a change is called done.

```bash
cargo check --workspace --all-targets --locked
cargo test --workspace --locked
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo bench -p cortex-bench --bench hot_path --locked -- --test   # benchmarks execute; no timing asserted
cargo +1.85 check --workspace --all-targets   # the MSRV floor (ADR-0009); rustup toolchain install 1.85 once
cargo +1.85 test --workspace
npm ci
npm run spec                        # spec-guard + spec-graph + check-briefs
```

`npm run spec:guard` alone runs the executable assertions; `npm run spec:graph` alone runs the cross-document checks; `npm run spec:briefs` alone checks the live briefs. All exit non-zero with a file and line number when something is wrong.

## Definition of done

A change is done when all of the following hold:

- code compiles on the pinned toolchain and on the MSRV ([ADR-0009](docs/adr/0009-rust-edition-and-msrv.md)) with no new warnings;
- layout assertions and tests pass;
- whitepaper tables and status labels reflect the change;
- an ADR is added or amended if a rule changed;
- there is a changelog entry under `Unreleased`;
- `npm run spec` is green.
