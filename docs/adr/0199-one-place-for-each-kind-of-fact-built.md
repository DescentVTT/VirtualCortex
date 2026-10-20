---
status: proposed
date: 2026-10-20
depends-on: ADR-0197
title: One place for each kind of fact, built
kind: built
summary: Brief 077 builds ADR-0197. Where the work stands is in docs/STATUS.md and each hypothesis's text and rounds in docs/research/. Every ADR carries a title, a summary and a kind, the tables are written from them, and a ratchet holds new text to the limits.
decision-makers: VirtualCortex maintainers
---

# ADR-0199: One place for each kind of fact, built

**In brief**
- **Decision.** ADR-0197's homes exist, the text that moved went verbatim with its directives, and two checks join the gate: `spec:shape` and `spec:index`.
- **Why.** ADR-0197 decided it, and brief 077 is its round. Each choice the brief left to the round is recorded below.
- **Left open.** §5.2's layout tables generated from the code, which ADR-0197 named. The closed questions of §11.1 that no hypothesis numbers stay as they were written.

## Context and Problem Statement

[ADR-0197](0197-one-place-for-each-kind-of-fact.md) gives each kind of fact one home and a check on the shape of new text.
Brief 077 builds it, on `main` at `54cc458`, after brief 076 merged.
Its architectural empowerment leaves the round six choices: the key for an ADR's kind, the layout of `docs/STATUS.md` and of a report, where §6.5's account is split, the markers and the generator, and how the check reads "added or modified".

**The sweep of brief 076's dispatch, recorded here**, as [ADR-0158](0158-the-sweep-off-the-next-decisions-path.md) asks of the first ADR that merges after a sweep ends.
Dispatch 39280026945, on `8892e52`, ended its thirteen sweep jobs green at 22:42Z on 2026-10-19, after brief 076's pull request had merged.

| Outcome | Mutants |
| :--- | ---: |
| Caught (2 418 in the state crates, 1 332 in the runtime) | 3 750 |
| Missed | 0 |
| Unviable | 157 |
| Timeouts | 26 |
| All | 3 933 |

- **Six mutants more than ADR-0197's 3 927.** `Config::reads_its_targets`, ADR-0198's, gives six, and each is caught.
- **Twenty-two timeouts are where [ADR-0062](0062-the-first-complete-sweeps-list.md) reads them as inherent**: the three chain iterators of `cortex-core`, six; the injector, twelve; the barrier's wait, one; the workers' stop and run, three.
- **Three are in `Image::decode`'s loading**, ADR-0197's three.
  - `image.rs:1167:24` and `image.rs:1175:24` were still finishing tests, F-42's kind by [ADR-0063](0063-the-sweep-reads-its-own-timeouts.md)'s evidence.
  - `image.rs:1144:44` had stopped finishing tests, none of its unreported tests the mutant's own module's.
  - Each shared its window with an injector mutant that timed out: 1 860, 1 246 and 2 604 s.
- **One is new**: `store.rs:143:9`, `Induction::discoveries` returning an empty slice.
  - It was still finishing tests, F-42's kind, beside an injector timeout for 1 108 s.
  - It was not among the timeouts of ADR-0197's sweep.
- **There is no survivor**, so the sweep leaves no finding.

## Decision Drivers

- ADR-0197's decision, and brief 077's standing directives: text that moves keeps its words, no directive is lost, no accepted ADR's body or archived brief changes, no code changes, no dependency.
- A structural boundary beats a reviewed one (principle 5): what can be checked is a check, and the rest is a description in `CONTRIBUTING.md`.
- The repository wins over the document (principle 1): a disagreement found on the way is a finding.

## Considered Options

1. **The kind's key**: (a) `kind:`; (b) another name.
2. **"Added or modified"**: (a) a block of text the base's blocks do not hold, links resolved to the file they reach; (b) the lines `git diff` marks.
3. **The hypotheses' tables**: (a) written by the generator from the reports' front matter; (b) written by hand in §11.1 and in `docs/research/README.md`.
4. **Two lines with no number**, the rule held by the network and the operating regime: (a) a report each; (b) left long in §11.1.
5. **Two reports spec-graph reads a handover into**: (a) left out of its history; (b) a baseline of accepted findings.

## Decision Outcome

**Options 1(a), 2(a), 3(a), 4(a) and 5(a).**

### What was built

| Home or check | What it holds | Where |
| :--- | :--- | :--- |
| Where the work stands | 60 lines from `CLAUDE.md`'s former section and ADR-0196 | [`docs/STATUS.md`](../STATUS.md) |
| What a hypothesis read | H-1 to H-41, a report each, and two lines with no number | [`docs/research/`](../research/README.md) |
| A decision's short form | `title:`, `summary:`, `kind:` and `hypothesis:` on ADR-0001 to ADR-0196 and ADR-0198 | the front matter, 676 lines |
| The tables | the index, §9, and the hypotheses' tables in §11.1 and the research index | `scripts/adr-index.mjs`, `spec:index` |
| The shape of new text | ADR-0197's limits, an accepted ADR's body | `scripts/check-doc-shape.mjs`, `spec:shape` |
| The rules | where each fact goes, what is checked and what review holds | `CONTRIBUTING.md`, `briefs/README.md` |

**The whitepaper**, 5.0.0, a restructuring by [ADR-0064](0064-the-documentation-gate-and-the-version.md)'s rule:
- the executive summary in 296 words;
- §1.6's three long cells and §8.8's seven status cells within 400 characters, each its label and its links;
- §6.5's paragraph split at its sentences, the mechanism kept in eight paragraphs and the account of the rounds moved;
- §11.1 a table of the hypotheses, its two long open items short, its other items as they were;
- §9 written from front matter;
- Appendix B's row V-8 for the two checks, and findings F-72 to F-74.

| Document | Before | After |
| :--- | ---: | ---: |
| `docs/WHITEPAPER.md` | 1 076 110 bytes, 3 321 lines | 615 555 bytes, 3 062 lines |
| `README.md` | 44 735 bytes, a line of 35 486 | 10 129 bytes |
| `CLAUDE.md` | 24 630 bytes | 16 355 bytes |
| `docs/adr/README.md` | 139 697 bytes | 75 470 bytes |
| `docs/zh-TW/README.md` | 131 771 bytes, a line of 114 693 | 9 879 bytes |

**The directives**: 602 before and 604 after. None was removed. The two added back Appendix B's row. Of the whitepaper's 599, 266 moved into the reports with the text they back.

### The choices under the empowerment

- **The key is `kind:`.** spec-graph 0.17.0 keeps a front matter key as `fm.kind`, and the kind of a node it builds is its own word, `document` or `item`. ADR-0197 carried the key already and the graph read it.
- **A report's layout.** Its front matter holds `hypothesis:`, `question:` and `verdict:`. Its H1 is the hypothesis's number and its own name. Then come the text as §11.1 held it, and the account of its rounds as §6.5 told it, each with its directives.
- **The question** is the hypothesis's own name, its bold title in §11.1 without the ADR and the date. A question written afresh would have been new prose about old text.
- **Where §6.5's account went.** It was split at the sentences where a round's account starts, from ADR-0069's to ADR-0128's.

  | Report | §6.5's account, from |
  | :--- | :--- |
  | H-12 | ADR-0069, through ADR-0077 and ADR-0078's choice |
  | H-13 and H-14 | ADR-0079 and ADR-0081 ran them |
  | H-15 to H-21 | the sentence where each protocol ADR wrote it: ADR-0082, 0085, 0089, 0093, 0106, 0109 and 0128 |

- **The directives moved by what they back.** §6.5's that hold the mechanism stay. Those of a round's tests went to its report, and §8.8's 146 went to the reports of H-24 to H-39 by the ADR each cites first.
- **`docs/STATUS.md`** has five parts: the engine, what it learns in one table, what was read as no and kept unset, what does not exist, and the next decision ADR-0196 names.
- **The markers** are `<!-- adr-index: begin NAME -->` and `<!-- adr-index: end NAME -->`, with no `@`, so that neither spec tool reads them as a directive.
- **The hypotheses' tables are written too**, from the reports' front matter and the ADRs whose `hypothesis:` names them. Two tables written by hand would have been one fact in two places. An ADR cell past 400 characters gives its first and last ADR and the count, as H-12's does.
- **"Added or modified".** The check splits each document into paragraphs, list items and table cells. A block over its limit passes only when the base's blocks hold its text, every relative link resolved to the file it reaches. A sentence moved verbatim is not new text, and a paragraph grown by one word is.
- **The ADRs.** An ADR not accepted on the base is new: its title, summary and kind are checked, with its H1 and its **In brief**. An accepted one keeps its body byte for byte. Its front matter changes only in `status:`, `superseded-by:` and `amended-by:`, and takes ADR-0197's four keys once, while its base copy has no `title:`.
- **The hypothesis an ADR names** is the one it writes, amends or reads. Where §11.1's item cites an ADR its own text does not number, the ADR names the item's hypothesis: ADR-0021 H-4, ADR-0034 H-7, and ADR-0059, ADR-0060 and ADR-0070 H-12.
- **Two lines with no number** got a report each, `a-rule-held-by-the-network.md` and `the-operating-regime.md`. Their open items stay in §11.1, short, with a link.
- **H-23 and H-26 are not history to spec-graph.** Their verbatim text sets a bare ADR beside a verb spec-graph reads as a relation: *"blocks slower than H-21's (ADR-0132)"* and *"what ADR-0141 left to the protocol"*. In a record, those relations become a ghost handover from an accepted ADR. The text may not be reworded. A `@spec-ignore` drops a whole file in 0.17.0, and a baseline would record a misreading as debt. `.spec-graph.json` excepts the two by name.

### The findings recorded

- **F-72**: ADR-0197 says spec-guard's source is at 0.27.0, and the tree pins 0.26.0. Its source has no sidecar either.
- **F-73**: the Traditional Chinese guide named milestones M1 to M9, and Appendix C lists M1 to M15.
- **F-74**: `README.md` said the repository is at the state-model stage, and `CLAUDE.md` said it is past it.

## Consequences

- Good: a reader learns where the work stands from 60 lines, and a hypothesis's whole record from one report.
- Good: the index and §9 are short and cannot drift from the ADRs, and new text cannot grow past the limits.
- Bad: two reports are not history, by a misreading of spec-graph's, and are named as such in its configuration.
- Bad: 197 summaries are new prose about old decisions, drawn from each ADR's H1 and decision outcome. They are reviewed with this record.
- Neutral: no engine code, test or pin changed, and no dispatch was made.

## Alternatives considered and why rejected

- **Option 2(b), the diff's lines**: a sentence moved from the whitepaper to a report is an added line to `git diff`, so the move this round makes would fail the check it adds.
- **Option 3(b), by hand**: the same table in two documents, which is what ADR-0197 removes.
- **Option 4(b), left long**: an open item of 18 525 characters is the shape ADR-0197 ends.
- **Option 5(b), a baseline**: it records as debt a reading that is the tool's, and a new file outside the brief's scope.

## Confirmation

- `npm run spec` runs `spec:index`, `spec:index:test`, `spec:shape` and `spec:shape:test` beside the checks it ran before. `spec:scripts:test` holds that.
- `spec:guard` passes 604 directives and `spec:prove` shows each one fail.
- `spec:shape` passes on this round's own diff, and every accepted ADR's body is its copy on `main`.
- `spec-brief archive 077 --strict` passes: nothing outside `affectedFiles` changed and nothing in `protectedFiles`.
