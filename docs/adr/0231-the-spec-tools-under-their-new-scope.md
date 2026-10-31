---
status: proposed
date: 2026-11-01
amends: [ADR-0008, ADR-0185]
title: The spec tools under their new scope
kind: process
summary: spec-guard 0.30.0, spec-graph 0.21.0 and spec-brief 0.12.0 under `@spec-family` replace ADR-0185's pins under `@descent-vtt`. ADR-0008's admission carries over, every step of `npm run spec` reads the tree as before, and records keep the old name. spec-harness stays out.
decision-makers: Menteion maintainers
---

# ADR-0231: The spec tools under their new scope

**In brief**
- **Decision.** The tree pins `@spec-family/spec-guard` 0.30.0, `@spec-family/spec-graph` 0.21.0 and `@spec-family/spec-brief` 0.12.0, exactly, in place of `@descent-vtt` at 0.26.0, 0.17.0 and 0.8.0. The admission of [ADR-0008](0008-documentation-governance.md) and the ground of [ADR-0185](0185-the-spec-tools-upgraded-and-spec-brief.md) read the new name. spec-harness is not adopted.
- **Why.** The family moved to a GitHub organization and an npm scope of its own, and its versions go on only there. Tried on this tree, every step of `npm run spec` stayed green and spec-guard's and spec-brief's readings did not change.
- **Left open.** Whether spec-harness ever joins, and when the old scope stops being published. Nothing in this tree waits on either.

## Context and Problem Statement

The tools' author moved them on 2026-10-26: "versions go on from 0.29.0 under the new name", in spec-guard's changelog, and the same for the other two.

**What each scope holds:**

| Scope | spec-guard | spec-graph | spec-brief | spec-harness |
| :--- | ---: | ---: | ---: | ---: |
| `@descent-vtt`, not deprecated | 0.29.0 | 0.20.0 | 0.11.0 | 0.21.0 |
| `@spec-family` | 0.30.0 | 0.21.0 | 0.12.0 | 0.22.0 |
| This tree before this ADR | 0.26.0 | 0.17.0 | 0.8.0 | none |

Each first release under the new scope says a run decides what the version before it decided. So the move carries the versions between the pins and the old scope's last, and those are what was tried.

**The trial**, on `main` at `e00b9bc`, in a clone, on Windows under Node 24:

| Tool | Before | After |
| :--- | :--- | :--- |
| spec-guard | 644 of 644 held | 644 of 644 held, the output differing only in the renamed lines |
| `spec-guard prove` | 644 seen to fail | 644 seen to fail |
| spec-graph | consistent, 30 open, 6 377 relations | consistent, 30 open, 6 375 relations |
| spec-brief | 85 clean | 85 clean |

**spec-graph's two fewer relations** are its reading, not the tree's: `amends` falls from 74 to 55, `references` rises from 4 634 to 4 652, `assumes` and `blocked-by` each fall by one, and `delegates-to` rises by one. Archived briefs that named a decision as amending ADR-0031 now read as references to it.

**spec-brief's archive** was tried on brief 084 taken back to its live copy: `archive --strict` wrote it again byte for byte, and refused a change to a protected file and one outside the brief's scope. The banner's link is read from `origin`, so a clone must have the repository's address there.

**What the newer versions change here:** spec-guard's fixes are to regex rules and to a work tree below the root, and this tree has no regex rule and no rule on `target="."`. spec-graph's fixes are false positives this tree does not trip.

**Against Latest ≠ Newest.** The ground ADR-0185 stated for the tools was read again on the new packages:
- they run in the documentation gate, outside the engine's build and off the hot path;
- [their pre-1.0 policy](https://github.com/spec-family/spec-core/blob/main/docs/adr/0009-versions-before-1-0.md) stands: a change that can turn a passing check red comes only in a minor release;
- each declares no runtime dependency, and `npm audit signatures` verifies 3 registry signatures and 3 attestations;
- spec-guard's README gives a mutation score of 99.27 % and spec-graph's 96.59 %; spec-brief's was not read again;
- they stay pinned to exact versions.

**spec-harness 0.22.0** was tried beside them. Its hook no longer costs what ADR-0185 reported: about 0.3 s before an edit and 0.12 to 0.18 s after one on the developer machine, under load, with outliers of 0.88 s and 1.7 s. It denied writes to protected paths. A shell's write is still caught only at a commit or a turn's end, protected paths still need a signed ruling, and what it adds here `spec-brief archive --strict` and `spec:scope` already hold.

## Decision Drivers

- **Releases reach this tree only by a decision** (ADR-0185): a scope that stops at a version is no reason to stop reading the versions after it.
- **The repository wins over the document** (principle 1): the documents that name a tool name the one installed.
- **A record is not rewritten** (ADR-0197, ADR-0185): an accepted ADR's body and an archived brief are frozen.

## Considered Options

1. Stay on `@descent-vtt` at the present pins.
2. Move to the old scope's last versions first, then to the new scope.
3. Move to `@spec-family` in one step.
4. Option 3, with spec-harness adopted too.

## Decision Outcome

**Chosen: option 3.**
- `package.json` and `package-lock.json` pin the three new packages. `.spec-brief.json`'s `$schema`, the imports in `scripts/check-scope.mjs` and its test, the living documents and the whitepaper's assertions name them.
- ADR-0008's admission of the project's own documentation tools, and ADR-0185's ground for it, hold for these packages under this name.
- spec-guard's and spec-graph's two uses in the gate, and every flag the tree passes them, are unchanged.
- Whitepaper §10's rows for V-3 and V-4 held cells of 750 and 743 characters, which the rename would have modified past ADR-0197's limit. What they said moved, word for word where it could, into two paragraphs under the table.

**Accepted ADRs, archived briefs and the changelog's older lines keep `@descent-vtt`.** They record what ran, and the old packages are still published.

<!-- @assert-absence target="package.json" symbol="@descent-vtt" reason="ADR-0231: no pin under the old scope" -->
<!-- @assert-absence target="package-lock.json" symbol="@descent-vtt" reason="ADR-0231: no resolved package under the old scope" -->
<!-- @assert-absence target=".spec-brief.json" symbol="@descent-vtt" reason="ADR-0231: the schema is the new package's" -->
<!-- @assert-absence target="scripts" symbol="@descent-vtt" glob="*.mjs" reason="ADR-0231: no script imports the old scope" -->

**No sweep is left unrecorded** that had ended when this ADR was written; the schedule's next runs on 2026-11-04 at 20:17Z.

## Consequences

- Good: a later release can reach this tree, by a decision, as one can no longer under the old scope.
- Good: the versions between the pins and the old scope's last are in the tree, with their fixes, at no change in any reading above.
- Bad: on the developer machine, under load, `prove` took 7.6 s to 14.0 s at its best and spec-graph 6.0 s to 9.2 s. These are indications from three alternated rounds, not Measured.
- Neutral: a reader following a frozen record's name finds the old package, which is still published.

## Amended

- **ADR-0008**: the two tools it admits by name are those packages under `@spec-family`.
- **ADR-0185**: its pins are the versions above. Its other decisions stand, spec-harness not adopted among them.
- Both are accepted and are not edited.

## Alternatives considered and why rejected

- **Option 1, stay.** Nothing forces the date, since `npm ci` still installs the old packages. But no later release arrives, and the distance grows with each one.
- **Option 2, two steps.** Each first release under the new scope says it decides as the last under the old, and the trial read none of the steps apart, so the second pull request would test nothing the first had not.
- **Option 4, with spec-harness.** The speed that kept it out is gone, but the reason it adds little stands: rounds write by shell as well as by Edit, and the maintainers do not sign rulings with a hardware key.

## Confirmation

- `npm run spec` in the pull request's gate runs on Ubuntu under Node 22, which the trial did not; `npm ci` there installs the lock.
- spec-guard's directives pin the three versions in `package.json`, and this ADR's four hold the old scope out of the manifests and the scripts.
- `npm run spec:scope:test` runs `scripts/check-scope.mjs`'s import of the new package.
