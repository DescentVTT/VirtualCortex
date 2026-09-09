---
status: accepted
date: 2026-09-10
decision-makers: VirtualCortex maintainers
---

# ADR-0008: Documentation governance: arc42, MADR, BCP 14, executable assertions

## Context and Problem Statement

Between revisions 2.0 and 2.8 the architecture whitepaper grew to twenty-eight chapters, reproduced struct definitions that no longer matched the source for fifteen of nineteen types, stated unmeasured performance figures as results, and carried a full parallel translation that drifted and broke in lockstep. Coding agents and reviewers read the document as ground truth and were misled by it. How are the project's documents structured and kept honest?

## Decision Drivers

- The founding rule of the project, "Latest is not Newest", applies to documentation standards as much as to code: use mature templates with tooling and a reader base.
- Every claim about the source tree must be checkable by a tool that runs in CI.
- Every performance figure must be labelled as measured or as a target (ADR-0010).
- One canonical text; translations must not become second contracts.

## Considered Options

1. Free-form whitepaper, hand-maintained, in two languages.
2. Generated documentation only (`cargo doc`) with no architecture narrative.
3. **arc42 structure with C4 views; MADR decision records; BCP 14 requirement keywords; four status labels (Implemented, Specified, Target, Hypothesis) on every claim; `spec-guard` directives for every claim about the tree; `spec-graph` for cross-document consistency; English canonical, with a short Traditional Chinese reader's guide that contains no layouts or figures.**

## Decision Outcome

Option 3, effective with whitepaper 3.0.0.

- `docs/WHITEPAPER.md` is the canonical architecture document. `docs/zh-TW/README.md` is a reader's guide that points into it and is not a translation of it.
- `docs/adr/NNNN-title.md` holds one MADR record per decision with a `status:` front-matter field that `spec-graph` reads.
- Where the document and the repository disagree, the repository is authoritative and the disagreement is a numbered finding in whitepaper §11.
- `@descent-vtt/spec-guard` and `@descent-vtt/spec-graph` are pinned as exact-version dev dependencies in `package.json` and run in CI as blocking checks.

### Consequences

- Good: a stale claim is a failing build with a line number.
- Good: reviewers can see at a glance what exists, what is designed, and what is aspirational.
- Bad: two Node tools in a Rust repository; accepted because both have zero runtime dependencies, are auditable in an afternoon, and are already used by the maintainers' other projects.
- Bad: the whitepaper is longer per crate than a prose summary would be; the layout tables are the ABI and are worth the length.

## Confirmation

`npm run spec` executes both tools; `.github/workflows/ci.yml` runs them on every push and pull request. This ADR is itself checked by `spec-graph`.
