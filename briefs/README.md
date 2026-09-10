# Briefs

Numbered, self-contained prompts. Each one is a round of work written to be handed to a fresh
Claude Code session, or a human, with no other context. The shape is borrowed from the
maintainers' other repositories and trimmed to what a small repository needs.

## What a brief is

An **input**. It says what done looks like, re-derives the facts it rests on with file paths and a
date, lists deliverables, draws the scope boundary in both directions, and says how the work is
verified and how it is reported. It repeats the standing directives rather than referring to a
preamble, because the session that reads it may have read nothing else.

Every brief carries these sections, in this order. `npm run spec:briefs` fails when a live brief
lacks one.

| Section | What it holds |
| :--- | :--- |
| `## Mission` | One paragraph: the state of the tree when the brief is done. |
| `## Standing directives` | The repository's always-on rules, restated. |
| `## Context` | Facts the brief rests on, each with a path, re-derived on the stated date. Line numbers move; symbol names and quoted sentences are what to re-derive. |
| `## Deliverables` | A checkbox list. `spec-graph` reads each box as an obligation with real state. |
| `## Not empowered` | The cheap wrong moves this round must not make. |
| `## Architectural empowerment` | The licence to override the brief's *instructions* with a better decision, recorded as an ADR. It never reaches the whitepaper's invariants or the constraints in `CLAUDE.md`. |
| `## Verification` | The commands and the expected results, the mutation gate on the diff among them ([ADR-0030](../docs/adr/0030-verification-governance.md)). |
| `## Report` | What the closing message must state, always including what was not done. |

A live brief may also carry `<!-- @assert-* -->` directives that state its **precondition** (the
defect it fixes still exists). `spec-guard` runs them: when the precondition stops holding, CI fails
until the brief is archived or rewritten, which is how a stale brief is caught.

## What a brief is not

**A record of what happened.** Outcomes live in the ADR the round writes, in the changelog, in the
whitepaper's §11 dispositions and in the code. Do not add a status file here.

## Lifecycle

| Phase | Where | Front matter |
| :--- | :--- | :--- |
| Live | `briefs/NNN_title.md` | `status: proposed` |
| Executed | `briefs/archive/NNN_title.md` | `status: archived` |

Numbers are three digits, allocated in order, never reused. Take the next free one with `ls briefs briefs/archive`.

## Archiving one

1. Move the file to `briefs/archive/`. Set `status: archived`.
2. Prepend a frozen banner: execution date, pull request, findings closed, ADRs written, and the
   sentence *"The body below describes the tree before execution and is not maintained."*
3. Disposition every deliverable in place: `[x]` when done, or a `**Delegated to** ...`,
   `**Accepted debt:** ...` or `**Rejected:** ...` note under the box. `spec-graph` reports an
   archived brief with an open box as an orphaned obligation, and a delegation to a retired document
   as a ghost handover; both fail CI.
4. Remove the precondition directives, or leave them: `spec-guard` does not run on `archive/`.
5. Relative links gain one `../` so that they still resolve from `archive/`; `spec-graph` checks
   links in archived briefs. This changes no word, claim or figure, and the banner says it was
   done.
6. Do not edit the body otherwise. A frozen snapshot claims nothing about now and cannot drift.

## Writing one

Copy the newest live brief and replace every section. Re-derive the context on the day you write
it and say so. Write the empowerment clause for *this* round: what it may decide is specific to the
work and is the one part that must be thought about rather than templated.

**Never allocate an ADR number in a brief.** Write "a new ADR at the next free number
(`ls docs/adr`)"; the executing round takes the number when it writes the file. `spec-graph`
reports a citation of an ADR that does not exist, which is how the first three briefs were caught
naming one.
