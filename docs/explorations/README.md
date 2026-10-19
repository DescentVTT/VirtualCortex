# Explorations

An exploration asks a question with no prediction: a scan of a parameter, a mechanism tried on a branch, a look at
what a network does under a change. It runs on the repository's host and is written down here as a note. **A note is
not evidence.** A verdict rests only on a hypothesis written before its runs and measured by a round
([ADR-0193](../adr/0193-an-exploration-lane.md)).

| | A measured round | An exploration |
| :--- | :--- | :--- |
| Written first | a decision ADR, a hypothesis and its prediction | the question, in the note |
| Its runs | once, after the protocol is committed | as many as the question needs |
| Its code | merged, its numbers pinned in the weekly job | on its branch; merged only if a later round takes it |
| Its record | an ADR, accepted on merge | a note here, `status: exploratory` |
| What it can decide | a verdict, a stopping rule's step | nothing: it can only suggest the next question |

## Running one

1. **Branch** as `explore/NNN-<slug>` from `main`, NNN the note's number (`ls docs/explorations`). The branch is kept,
   never deleted, so that the commits its runs name stay in a branch.
2. **Write the runs as `#[ignore]`d tests whose names end in `_explore`.** Neither `--ignored exhaustive`, the weekly
   job's listing, nor `--ignored paused` names them; `scope=arms` runs any `#[ignore]`d test by name:
   `gh workflow run ci.yml --ref explore/NNN-<slug> -f scope=arms -f tests='<test>_explore ...'`. Each test runs in a
   job of its own on AArch64, its output kept as the artifact `arm-<n>`.
3. **A change under `src/` stays on the branch.** A rule changes only by an ADR and a measured round; an exploration
   may try one, and its note says so.
4. **Write the note** as `docs/explorations/NNN-<slug>.md` and merge it alone, a documents-only pull request:

```markdown
---
status: exploratory
date: YYYY-MM-DD
---

# NNN: <the question>

**Branch**: `explore/NNN-<slug>`
**Commit**: `<the commit the runs ran on>`
**Runs**: [<run id>](https://github.com/DescentVTT/VirtualCortex/actions/runs/<run id>)

What was asked, what ran, what it printed, and what it suggests. Every figure is exploratory.
```

## What a note may and may not do

- **May** be cited by a decision ADR in its "Context and Problem Statement", as the reason it asks what it asks.
- **May not** be cited anywhere else in an ADR, and its numbers are not a hypothesis's, nor pinned. A question an
  exploration suggests is asked again by a measured round, from its own protocol.
- `npm run spec:explorations` holds both: every note marked exploratory with its branch, commit and runs, and no ADR
  citing a note outside its Context.
