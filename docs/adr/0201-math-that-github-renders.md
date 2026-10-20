---
status: accepted
date: 2026-10-20
title: Math that GitHub renders
kind: process
summary: GitHub refuses \operatorname and lets Markdown eat a backslash before punctuation inside $...$. The math is written \mathrm, $`...`$ and ```math, the living documents, accepted ADRs and changelog are fixed once, and spec:shape holds every block to it.
amends: ADR-0197
decision-makers: VirtualCortex maintainers
---

# ADR-0201: Math that GitHub renders

**In brief**
- **Decision.** Math is written so GitHub renders it: `\mathrm` for an operator's name, `` $`...`$ `` inline and a ```` ```math ```` fence for a block whenever the TeX holds a backslash before punctuation. The living documents, the accepted ADRs and the changelog are fixed once, and `spec:shape` holds every block to it.
- **Why.** GitHub showed the whitepaper's formulas as error boxes, and some others as different formulas, with no error at all.
- **Left open.** The archived briefs keep their math as it was, frozen by their hashes.

## Context and Problem Statement

On 2026-10-20 the maintainers saw the whitepaper on GitHub show *"The following macros are not allowed: operatorname"* in place of formulas.
Read on GitHub's page and in the tree at `26cadb7`, two faults:

| Fault | What GitHub shows | Where |
| :--- | :--- | :--- |
| `\operatorname{...}` | an error box: GitHub's renderer refuses the macro | 12 boxes in the whitepaper; 37 uses in 16 files |
| a backslash before ASCII punctuation, `\,` `\;` `\{` `\}` `\\` `\%`, inside `$...$` or `$$...$$` | Markdown takes it as an escape before MathJax sees it: `[8\,180, 31\,986]` reads `[8, 180, 31, 986]`, `-\,dF/dt` reads `−, dF/dt`, `\{...\}` loses its braces, `\%` comments out the rest | 117 uses in 38 files |

They sit in 42 files and in 12 archived briefs.
Each archived brief is held by its hash ([ADR-0185](0185-the-spec-tools-upgraded-and-spec-brief.md)) and frozen since it was executed (principle 8).
39 of the 42 are accepted ADRs, whose bodies [ADR-0197](0197-one-place-for-each-kind-of-fact.md) holds byte for byte.

GitHub documents two forms that Markdown does not touch: `` $`...`$ `` inline and a ```` ```math ```` fence for a block.

## Decision Drivers

- **The maintainers' choice** (2026-10-20): the living documents, the accepted ADRs and the changelog are fixed; the archived briefs are not.
- **A fix of rendering, not of content**: the TeX stays byte for byte but for `\operatorname{` to `\mathrm{`.
- **A structural boundary beats a reviewed one** (principle 5): the rule is a check, and the exception it opens in ADR-0197's is a check too.

## Considered Options

1. **The escapes**: (a) GitHub's protected forms; (b) doubled backslashes, `\\,`, which another renderer would read as a line break; (c) no punctuation macros, `\,` dropped.
2. **The accepted ADRs**: (a) fixed, the body compared in a rendered form that reads the protected forms as the plain ones; (b) left as they are.
3. **The rule**: (a) every block, since after the fix none fails; (b) new text only, as ADR-0197's limits.

## Decision Outcome

**Options 1(a), 2(a) and 3(a).** Built in this pull request:

- **The fix**, once, in every Markdown file but the archived briefs:
  - `\operatorname{X}` becomes `\mathrm{X}`;
  - an inline `$...$` holding a backslash before punctuation becomes `` $`...`$ ``;
  - a block, `$$` lines or a line that is `$$...$$`, holding one becomes a ```` ```math ```` fence at its indentation.
- **`spec:shape` reads math**, in every block of the living documents and of every ADR, and in every changelog entry:
  - `\operatorname`, protected or not, and in a ```` ```math ```` fence, fails;
  - a backslash before ASCII punctuation in `$...$` or `$$...$$` fails.
- **ADR-0197's rule on an accepted ADR's body is amended.** The body may change where its rendered form stays the same. That form reads `` $`...`$ `` as `$...$`, a ```` ```math ```` fence and a `$$...$$` line as a `$$` block, and `\operatorname{` as `\mathrm{`. A change in a single character of TeX still fails.
- **A block whose only change is its math's form is not new text**, so a long line fixed in place is not held to the limits it was written before.
- **The writing rules** in `CONTRIBUTING.md` gain the two forms, checked by `spec:shape`.

## Consequences

- Good: every formula of the living documents and the ADRs renders on GitHub, and a new one that would not fails the gate.
- Good: the exception to ADR-0197 is exact: rendering only, checked.
- Bad: `` $`...`$ `` and ```` ```math ```` are GitHub's forms. A renderer that knows only `$...$` shows them as code.
- Bad: 22 faults stay in 12 archived briefs.
- Neutral: no code, test or pin of the engine changes.

## Alternatives considered and why rejected

- **Option 1(b), doubled backslashes**: right on GitHub, a line break and a stray mark in a renderer that leaves `$...$` alone.
- **Option 1(c), no punctuation macros**: `\{...\}` has no plain equivalent, and the TeX would change.
- **Option 2(b), the ADRs left**: 100 faults in 39 ADRs would keep showing wrong formulas, and the maintainers chose to fix them.
- **Option 3(b), new text only**: a formula moved back to `\operatorname` reads as the base's under the rendered form, so a ratchet would pass it.

## Confirmation

`npm run spec` runs `spec:shape` with the math rule and its tests, and passes on this pull request's tree.
On GitHub the whitepaper shows no error box.
