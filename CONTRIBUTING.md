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

[Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/): `type(scope): summary`, where `type` is one of `feat`, `fix`, `docs`, `refactor`, `test`, `chore`, `ci` and `scope` is a crate name without the `cortex-` prefix (`core`, `hippocampus`), or `whitepaper`, `adr`, `workspace`.

```text
docs(whitepaper): transcribe SynapseBlock layout from source
feat(core): add saturating Q16.16 helpers
fix(cerebellum): compare prediction with delayed observation (F-8)
```

## Code rules

These are the technical constraints of whitepaper §2.2. They are checked where a tool can check them.

| Rule | How it is checked |
| :--- | :--- |
| Stable Rust only; no nightly features. | CI builds on `stable`. |
| State crates declare no dependencies. | `Cargo.toml` review; `cargo tree`. |
| Every primary record is `#[repr(C)]`; arena records are `align(64)` and exactly 64 bytes; size and alignment are asserted in a `const _: () = { ... }` block. | `cargo check` fails otherwise. |
| No `f32` or `f64` anywhere under `crates/`. Use Q16.16 (whitepaper §8.1). | `spec-guard` directive in the whitepaper. |
| No `Box`, `Vec`, `String` or thread spawning in state crates. | `spec-guard` directives. |
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

### Architecture decision records

`docs/adr/` holds one [MADR](https://adr.github.io/madr/) file per decision. To propose one: copy the newest file, take the next number, set `status: proposed`, fill in context, drivers, options, outcome and confirmation, and open a pull request. It becomes `accepted` on merge with that status. Superseding a decision means adding `supersedes: ADR-NNNN` to the new record and `superseded-by:` to the old one; `spec-graph` reports the omission. Never renumber, never delete.

## Verification

All of these must pass before a change is called done.

```bash
cargo check --workspace --all-targets
cargo test --workspace
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
npm ci
npm run spec                        # spec-guard + spec-graph
```

`npm run spec:guard` alone runs the executable assertions; `npm run spec:graph` alone runs the cross-document checks. Both exit non-zero with a file and line number when something is wrong.

## Definition of done

A change is done when all of the following hold:

- code compiles on stable with no new warnings;
- layout assertions and tests pass;
- whitepaper tables and status labels reflect the change;
- an ADR is added or amended if a rule changed;
- there is a changelog entry under `Unreleased`;
- `npm run spec` is green.
