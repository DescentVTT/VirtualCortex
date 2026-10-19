---
status: accepted
date: 2026-10-19
decision-makers: VirtualCortex maintainers
---

# ADR-0193: An exploration lane — a question asked with no prediction, its runs `#[ignore]`d tests named `…_explore` on a branch `explore/NNN-…` that is kept, dispatched at `scope=arms` as often as it needs, and written down as a note in `docs/explorations/` marked exploratory, with its branch, commit and runs; a note suggests the next question and decides nothing, and an ADR cites one only in its Context, which a check holds; a change under `src/` stays on its branch

## Context and Problem Statement

Every question this tree has asked of the engine went through a measured round: a decision ADR wrote a hypothesis and its prediction before any run, and one round built, ran once, pinned and reproduced it ([ADR-0150](0150-a-round-waits-for-what-it-checks.md), [ADR-0184](0184-the-runs-off-the-developer-machine.md)). A round takes six to nine hours and asks one change. That is what makes a verdict trustworthy, and it is slow for finding which question to ask: a parameter's range, whether a mechanism does anything at all on these networks, what a network does under a change nobody has a prediction for.

On 2026-10-18 the maintainers asked for an exploration lane beside the rounds, its results labelled as exploratory and never as evidence, a promising one promoted to a round with its own protocol.

**What the tree already has for it.**
- **`scope=arms`** runs any `#[ignore]`d test by name on the repository's host, each in a job of its own, its output kept ([ADR-0184](0184-the-runs-off-the-developer-machine.md)). It lists `--ignored --list`, so it finds a test whatever its suffix.
- **The weekly job's listing is `--ignored exhaustive`.** [ADR-0127](0127-the-paused-line-leaves-the-weekly.md) kept a paused line's runs out of it by naming them `…_paused`. A third suffix keeps an exploration's runs out of it the same way.
- **Merge commits keep a branch's commits in `main`'s history** ([ADR-0186](0186-merge-commits.md)). An exploration's branch is not merged, so its commits stay only as long as the branch does.

## Decision Drivers

- **Speed where nothing is decided.** Find the question fast, then answer it slowly.
- **No verdict without a protocol written before its runs.** The lane must not become a way around that.
- **A structural boundary beats a reviewed one** (principle 5): "a note is not evidence" is held by a check.
- **A rule changes only by an ADR** (`CLAUDE.md`'s workflow).

## Considered Options

1. **Where an exploration's code lives**: (a) on a branch `explore/NNN-…`, kept, never merged unless a later round takes it; (b) merged into `main` as `…_explore` tests; (c) anywhere, unrecorded.
2. **Its record**: (a) a note in `docs/explorations/`, marked exploratory, with its branch, commit and runs, merged as a documents-only pull request; (b) a section of a decision ADR; (c) none.
3. **What holds the line**: (a) a check, `scripts/check-explorations.mjs`, that every note says what it is and where it came from, and that an ADR cites one only in its Context; (b) review.

## Decision Outcome

**Options 1(a), 2(a) and 3(a).**

### An exploration

- **A question with no prediction**, asked when the maintainers want one. The reviewing session may run one by dispatch alone, or a session may be given one as a prompt. It is not a brief: it has no deliverables, no archive and no verdict.
- **Its branch** is `explore/NNN-<slug>` from `main`, NNN the note's number. It is kept, never deleted, so that every commit its runs name stays in a branch.
- **Its runs** are `#[ignore]`d tests whose names end in `_explore`, in the test files where the harness they use lives. They are dispatched at `scope=arms` on the branch, as often as the question needs, and none is in the weekly job. A test whose name ends in `_explore` contains neither `exhaustive` nor `paused`.
- **A change under `src/` stays on the branch.** An exploration may try a rule, but the rule enters `main` only by a decision ADR and a measured round.
- **Its record is a note**, `docs/explorations/NNN-<slug>.md`, merged alone in a documents-only pull request:
  - front matter `status: exploratory`;
  - lines naming the branch (`**Branch**:`), the commit the runs ran on (`**Commit**:`) and the runs (`**Runs**:` with a link to each run of the host);
  - what was asked, what ran, what it printed, and what it suggests, every figure exploratory.

  `docs/explorations/README.md` holds the template.

### What a note may do

- **It may be cited by a decision ADR in its "Context and Problem Statement"**, as the reason a question is asked.
- **It may not be cited anywhere else in an ADR.** A verdict rests on runs made after a protocol was committed; a note's runs were made before any.
- **Its numbers are not a hypothesis's, nor pinned.** A question it suggests is asked again by a measured round, from that round's own protocol and runs.

### What holds it

- **`scripts/check-explorations.mjs`** (`npm run spec:explorations`, with its tests in `spec:explorations:test`) fails:
  - a note not marked exploratory, or one naming no branch, no commit or no run of the host;
  - a file in `docs/explorations/` that is neither a note nor the README;
  - a link from an ADR into `docs/explorations/` outside its "Context and Problem Statement".
- **spec-graph reads the notes as history** (`historyPatterns`), so a note's "suggests" opens no obligation.

## Consequences

- Good: a question can be found in hours of dispatched runs, with nothing pinned and no protocol, where a round takes most of a day.
- Good: the line between a suggestion and a verdict is a check, not a habit.
- Good: an exploration's code and its runs stay findable through a kept branch and a note.
- Bad: kept branches accumulate.
- Bad: an exploration's code is not held by the gate. It is compiled on its branch and nowhere else, and may not build a year later.
- Bad: what an exploration suggests can still shape which hypothesis is asked and how it is predicted. Its prediction is written before its own runs, and a reader can see in the Context which note suggested it.
- Neutral: nothing of the engine, the weekly job or a round's protocol changes.

## Alternatives considered and why rejected

- **Option 1(b), merged `…_explore` tests**: code the gate compiles and nobody guards, in `main`, for questions whose answers are not evidence.
- **Option 1(c)**: a suggestion with no record of what produced it.
- **Option 2(b), a section of a decision ADR**: it would place exploratory numbers inside the document that carries a verdict.
- **Option 3(b), review alone**: the line most likely to blur under a good result is the one review would have to hold.

## Confirmation

`scripts/check-explorations.mjs` and its nine tests; `package.json` (`spec:explorations`, `spec:explorations:test`); `.spec-graph.json` (`historyPatterns`); `docs/explorations/README.md`; `CLAUDE.md`'s and `CONTRIBUTING.md`'s commands, Appendix B and its directives; this decision's rows in §9 and the index.
