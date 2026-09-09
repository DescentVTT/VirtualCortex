# CLAUDE.md

> **This file is the switchboard, not the rulebook.** It carries the principles that are not
> negotiable, the map, and the commands. The reasoning lives in the
> [whitepaper](docs/WHITEPAPER.md) and the [ADRs](docs/adr/README.md); the workflow lives in
> [CONTRIBUTING.md](CONTRIBUTING.md). Where this file and any of those disagree, they win and the
> disagreement is a defect in this file.

## What this repository is

A Rust workspace for a **deterministic, single-node neuromorphic virtual-actor engine**: spiking
neural computation on 64-byte cache-line records, Q16.16 fixed point, and a fixed pool of
core-pinned workers. Eighteen crates, one per subsystem, no dependencies between them.

It is at the **state-model stage**. The records, their compile-time layout assertions and five
small update functions exist; the executor, the timing-wheel drain path, the image loader, the
embodiment rings and every subsystem's real dynamics do not. The whitepaper's
[§1.6](docs/WHITEPAPER.md#16-implementation-status-at-a-glance) is the table of what is built;
[§11](docs/WHITEPAPER.md#11-risks-and-technical-debt) is the numbered list of what is wrong.

## The principles

Each one is here because breaking it has already produced a defect in this repository.

1. **The repository wins over the document.** Where a document and the tree disagree, the tree is
   authoritative; record the disagreement as a numbered finding in whitepaper §11 rather than
   fixing either side silently. Specification 2.8.0 described 15 of 19 records wrongly because
   nobody did this.
2. **Verify before asserting.** Read the file. Run the command. If you are about to write
   "presumably", go and look. The whitepaper's first executable assertion was written from memory
   and failed on the first run (F-18).
3. **Label every claim.** Implemented, Specified, Target or Hypothesis. A number is Measured only
   when a benchmark in this tree produced it on the reference platform from a committed command
   ([ADR-0010](docs/adr/0010-measured-or-target.md)); otherwise it is a Target with a protocol.
   Never write "tested on". Never reintroduce the withdrawn figures (86 billion neurons,
   P99.99 < 35 ns, 100 ms cold boot).
4. **Latest ≠ Newest.** A technology is admissible on the hot path only with a stable specification,
   two years of third-party production use, documented failure modes, and a mechanical-sympathy
   argument (whitepaper [§2.1](docs/WHITEPAPER.md#21-engineering-doctrine-latest--newest)). The
   same test applies to documentation standards and to dependencies. Nightly features, sub-1.0
   crates without a stability policy, and unreproduced performance claims fail it.
5. **A structural boundary beats a reviewed one.** Compile-time assertion > test > executable
   documentation directive > review comment. When you add a rule, name where it is enforced; if
   nowhere, write it as a description, not a requirement.
6. **Make claims about the tree executable.** A sentence that says something exists gets a
   `<!-- @assert-count ... min="1" -->` under it; a sentence that says something is gone gets
   `<!-- @assert-absence ... -->`. `spec-guard` runs them in CI.
7. **Say what you did not do.** A completed task with an unstated gap is worse than an incomplete
   one. Finish everything not blocked, then name what is left and why.
8. **No intermediate documents that drift.** One canonical whitepaper in English; the Traditional
   Chinese file is a reader's guide with no layouts or figures; briefs are inputs and are frozen
   when executed; outcomes live in ADRs, the changelog and the code.

## Invariants of the code

These are checked; the whitepaper §2.2 lists the constraint ids.

- Every primary record is `#[repr(C)]`; arena records are `align(64)` and exactly 64 bytes; size
  and alignment are asserted in a `const _: () = { ... }` block in the defining crate.
- No `f32` or `f64` anywhere under `crates/`. Q16.16 in `i32`/`u32`; widen to `i64` to multiply;
  saturating arithmetic on state fields (whitepaper §8.1).
- No `Box`, `Vec`, `String`, thread spawning or heap allocation in state crates; no syscalls on
  the hot path once a hot path exists.
- No `unsafe` without an ADR naming the invariant and the test.
- State crates declare no dependencies. A future runtime crate composes them.
- Changing any field of any record, including reserved bytes, bumps `CortexFileHeader::version`,
  updates the record's table in whitepaper §5.2, and gets a changelog entry.
- Edition and MSRV are decided by [ADR-0009](docs/adr/0009-rust-edition-and-msrv.md), which is
  `proposed`; do not change `[workspace.package]` edition in passing.

## Where to read

| Question | File |
| :--- | :--- |
| What is the architecture? | [docs/WHITEPAPER.md](docs/WHITEPAPER.md) — arc42; §4 axioms, §5 per-crate layouts, §8 numeric and concurrency rules |
| Why was something decided? | [docs/adr/](docs/adr/README.md) — MADR, one file per decision |
| What is known to be wrong? | Whitepaper §11 — findings F-n, hypotheses H-n, open questions |
| What is the next round of work? | [briefs/](briefs/README.md) — self-contained prompts; `briefs/archive/` is what already ran |
| How do I contribute? | [CONTRIBUTING.md](CONTRIBUTING.md) — commits, code rules, documentation rules, definition of done |
| What changed? | [CHANGELOG.md](CHANGELOG.md) |

## Commands

Run all of these before pushing. CI runs exactly the same set; a check here and not in CI is a
gate nobody enforces, and the reverse is a green local run and a red push.

```bash
cargo check --workspace --all-targets
cargo test --workspace
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
npm ci
npm run spec
```

`npm run spec` is `spec:guard` (executable assertions in the documents against `crates/`),
`spec:graph` (cross-document consistency: links, ADR lifecycle, open obligations) and
`spec:briefs` (every live brief carries its mandatory sections). Each fails with a file and line.

## Workflow

- Never commit on `main`. Branch, then pull request; Conventional Commits with a real body
  ([CONTRIBUTING.md](CONTRIBUTING.md#commit-messages)).
- A change to a record and the change to its documentation travel in the same PR.
- A rule change is an ADR first (`status: proposed`), accepted on merge.
- To execute a brief: read `briefs/README.md`, then the brief, then the files it names. When done,
  archive it as that README says, with the frozen banner and every deliverable dispositioned.

<!-- @assert-present file="docs/WHITEPAPER.md,docs/adr/README.md,briefs/README.md,CONTRIBUTING.md,CHANGELOG.md,scripts/check-briefs.mjs" -->
