---
status: accepted
date: 2026-10-19
title: One place for each kind of fact
kind: process
summary: Status lives in docs/STATUS.md, a hypothesis's readings in docs/research/, and a decision in an ADR with a short title and a summary. Every other document links to them, and a check holds the shape of new text.
decision-makers: VirtualCortex maintainers
---

# ADR-0197: One place for each kind of fact

**In brief**
- **Decision.** Each kind of fact has one home, and every other document links to it:
  - where the work stands: `docs/STATUS.md`;
  - what a hypothesis read: `docs/research/H-n.md`;
  - why a thing was decided: an ADR.

  New text meets a shape rule, which a check holds on the lines a pull request adds or changes.
- **Why.** The documents carry three jobs at once: explanation, audit trail and the contract CI checks. Each round appends its result in about seven places, so they have grown past reading.
- **Left open.** Generating §5.2's layout tables from the code.

## Context and Problem Statement

On 2026-10-19 the maintainers brought a proposal for the documents' readability. Measured on `main` at `43e89b5`, in characters:

| Measure | Value |
| :--- | :--- |
| `docs/WHITEPAPER.md` | 1 045 K, 3 315 lines, 85 lines over 2 000 |
| The whitepaper's longest line | 32 594, one paragraph of §6.5's scenario R-5 |
| The executive summary | one paragraph of 10 935 |
| Where the long lines are | of 401 K in lines over 2 000: 33 paragraphs, 34 list items (most of them §11.1's hypotheses, up to 18 475), 18 table rows (61 K) |
| Directives in the whitepaper | 598 `@assert` comments |
| `README.md`'s Status | one line of 34 919 |
| `docs/adr/README.md` | 135 K, a sentence of findings a row |
| ADR titles | 196 ADRs; the H1's median 561, the longest 1 635, 181 over 100 |
| A measured ADR | ADR-0195: 246 lines, 45 K |
| The whitepaper's version | 1.0 on 2026-09-09, 4.132.0 on 2026-10-19 |

**Where one round's result is written today:**
- `README.md`'s Status;
- `CLAUDE.md`'s "Where the work stands";
- the whitepaper's executive summary, §1.6, §8.8's status cells and §11.1;
- the ADR index and §9;
- `CHANGELOG.md`;
- `docs/zh-TW/README.md`.

Each is appended to, round after round.

**The proposal, checked against the tree:**
- **One home per fact.** It holds, and it is principle 8's own aim.
- **Measured ADRs turned into reports, cutting ADRs by two thirds.** It does not hold: 17 built and 36 measured ADRs are 27 per cent of 196. A measured ADR is also where a pre-registered hypothesis is read and its stopping rule's step taken, which ADR-0150 and ADR-0191 rest on.
- **Directives moved to a sidecar file.** spec-guard has no such thing: its source at 0.27.0 reads directives only from the documents it scans. Principle 6 also binds a directive to the sentence it backs. A comment is not shown when the Markdown is rendered.
- **Warnings first, then a block.** A check over every line would fail on the whole history before it could block anything.

**What may not be rewritten:**
- An accepted ADR is not edited (ADR-0192's F-69).
- An archived brief is held by its hash (ADR-0185).
- `CHANGELOG.md` is history to spec-graph.

**What is checked by row, not by text:** `spec:decisions` asks only that every ADR has a row in §9 and in the index, by the row's opening link.

**Tried before writing this:** spec-graph 0.17.0's check passes with `title:`, `summary:`, `kind:` and `hypothesis:` in an ADR's front matter. This was tried on ADR-0196 and reverted. How spec-graph reads each key is the brief's to read before relying on it.

**The sweep of brief 075's reproduction, recorded here**, as [ADR-0158](0158-the-sweep-off-the-next-decisions-path.md) asks of the first ADR that merges after a sweep ends. Dispatch 39202114129's thirteen sweep jobs ended green at 12:03Z.

- **3 927 mutants: 3 745 caught, none missed, 157 unviable, 25 timeouts.**
  - Of the caught, 2 418 are in the state crates and 1 327 in the runtime.
  - That is 11 more mutants than the 3 916 ADR-0191 recorded, which is about ADR-0194's flag.
- **Twenty-two timeouts are in the places [ADR-0062](0062-the-first-complete-sweeps-list.md) reads as inherent**:
  - `cortex-core`'s three chain iterators, six;
  - the injector, twelve;
  - the barrier's wait, one;
  - the workers' stop and run, three.
- **Three are in `Image::decode`'s loading**, code ADR-0052 wrote and not this round's:
  - `image.rs:1175:24`, `delete !` at the clause store, is of F-42's kind by [ADR-0063](0063-the-sweep-reads-its-own-timeouts.md)'s evidence: it was still finishing tests.
  - `image.rs:1167:24`, `delete !` at the term arena, and `image.rs:1144:44`, `||` for `&&` at the episodes, had stopped finishing tests by that evidence. In neither was the test that never reported one of the mutant's own module.
  - Each of the three shared its window with an injector mutant that timed out: 2 124, 1 648 and 2 518 s of overlap.
  - Each sits in a `for` over the section's records, so none can keep the decode from ending.
- **There is no survivor**, so the sweep leaves no finding.

## Decision Drivers

- **The maintainers' choices** (2026-10-19):
  - a measured ADR made short, its readings in a report;
  - the old ADRs' bodies untouched and their front matter completed once;
  - a brief after brief 076 and before the next decision's.
- **One home for each kind of fact** (principle 8).
- **A structural boundary beats a reviewed one** (principle 5): the shape is a check, and a rule that is not checked is written as a description.
- **History is not rewritten**: accepted ADRs' bodies, archived briefs and `CHANGELOG.md`'s entries.
- **Latest ≠ Newest**: no new tool. The checks are scripts in the tree with no dependency, like the others.

## Considered Options

1. **A measured ADR**: (a) short, with its readings in `docs/research/`; (b) a report only, no ADR.
2. **The old ADRs**: (a) front matter completed once, the H1 and the body byte for byte; (b) untouched, the index rows shortened by hand.
3. **The shape**: (a) a ratchet on the lines a change adds or modifies; (b) a warning over every line, then a block.
4. **The directives**: (a) under their sentences, moving with them; (b) a sidecar file.
5. **Who does it**: (a) a brief, after brief 076; (b) the decision session directly.

## Decision Outcome

**Options 1(a), 2(a), 3(a), 4(a) and 5(a).** Brief 077 builds all of it.

### One home for each kind of fact

| Kind of fact | Its home | How it is written |
| :--- | :--- | :--- |
| Where the work stands | `docs/STATUS.md` | rewritten whole by each round, at most 60 lines, no history |
| Why a thing was decided | an ADR | the decision, its options, and the next decision it names |
| What a hypothesis read | `docs/research/H-n.md` | one report a hypothesis; `docs/research/README.md` the table of all |
| What changed | `CHANGELOG.md` | one entry a change, at most 400 characters, linking its ADR |
| The architecture | the whitepaper | the stable design; a status cell gives its label and links |
| How to work | `CONTRIBUTING.md`, `CLAUDE.md`, `briefs/README.md` | rules and commands |

**What that changes:**
- **README.md's Status** keeps the four labels. Each "Today" cell is short, and a line links to `docs/STATUS.md`.
- **CLAUDE.md's "Where the work stands"** becomes a link to `docs/STATUS.md` beside the table of where to read. The principles, invariants, commands and workflow stay.
- **The whitepaper**:
  - The executive summary is at most 300 words.
  - Each cell of §1.6 and each status cell of §8.8 is at most 400 characters: its label and its links.
  - §6.5's account of the rounds moves into the reports it belongs to.
  - §11.1's hypotheses become one table: H-n, the question, the verdict, the ADRs, the report. Each hypothesis's text moves verbatim into its report. The open questions stay as short items.
- **docs/zh-TW/README.md** stays a guide to the whitepaper's structure, as principle 8 says. It carries no status, and links to `docs/STATUS.md`.
- **Text that moves, moves verbatim, with its directives.** A report is history to spec-graph once its round has merged.

### A measured ADR, from brief 078 on

- It keeps:
  - the verdict per network, arm and clause, in one table;
  - each predicted reading as held or missed, one line each;
  - the stopping rule's step reached, and the next decision it names.
- Its readings' tables and the evidence of its dispatches go to `docs/research/H-n.md`, written in the same pull request and linked from the ADR.
- The protocol ADR and the built ADR are unchanged.
- An exploration's note stays in `docs/explorations/` ([ADR-0193](0193-an-exploration-lane.md)). A research report is the evidence of a pre-registered round, and an ADR may cite it anywhere.

### An ADR's shape, from brief 078 on

- **Its front matter carries:**

  | Key | Value |
  | :--- | :--- |
  | `title:` | at most 80 characters, a noun phrase |
  | `summary:` | at most 300 characters, the conclusion |
  | `kind:` | one of `decision`, `protocol`, `built`, `measured`, `process` (CI and workflow among it) |
  | `hypothesis:` | the H-n it asks or reads, when it has one |

- **Its H1** is `# ADR-NNNN: ` and the title.
- **Its body opens with "In brief"**, three bullets: the decision, why, and what is left open. Then come MADR's sections, as now.

### The old ADRs: front matter, once

- Brief 077 adds `title:`, `summary:`, `kind:` and, where there is one, `hypothesis:` to the front matter of ADR-0001 to ADR-0196, once.
- Their H1 and body stay byte for byte. Each summary is drawn from the ADR's own H1 and decision outcome, and claims nothing the ADR does not.
- From then on an accepted ADR's body never changes. Its front matter gains only what a later ADR's lifecycle writes, `superseded-by:` among it. The shape check holds both.

### The index and §9, generated

- `scripts/adr-index.mjs` writes the index's table from the front matter, `npm run spec:index` fails when the committed tables are not what it writes, and `spec:decisions` stays.

  | Table | Columns |
  | :--- | :--- |
  | `docs/adr/README.md` | ID, title, kind, H-n, summary, status |
  | §9, between markers | ID, title, status |

### The shape check

`scripts/check-doc-shape.mjs`, as `spec:shape` with `spec:shape:test`, runs in `npm run spec`. It reads `origin/main` as `spec:scope` does.

**On the lines a change adds or modifies**, in the living documents, the limits hold:

| Text | Limit |
| :--- | :--- |
| A paragraph or a list item | 1 200 characters |
| A table cell | 400 characters; the index's 300 |
| An ADR's `title:` | 80 characters |
| An ADR's `summary:` | 300 characters |
| An ADR's `kind:` | one of the five |
| `docs/STATUS.md` | 60 lines |
| A `CHANGELOG.md` entry | 400 characters |

- **The living documents** are the whitepaper, `README.md`, `CLAUDE.md`, `CONTRIBUTING.md`, `briefs/README.md`, `docs/STATUS.md`, `docs/research/**`, `docs/zh-TW/**`, the index, a new ADR and a live brief.
- **Over whole files:** an accepted ADR's body equals its copy on the base, and its front matter gains only lifecycle keys.
- **Not read:** archived briefs, `CHANGELOG.md`'s entries on the base, code blocks, and the directive comments themselves.
- **Writing rules no check can hold** go into `CONTRIBUTING.md` as descriptions, held by review:
  1. the conclusion first;
  2. one thing a sentence, at most 35 words, and no clauses chained by semicolons;
  3. numbers and symbols, `step = 1/16` rather than *a sixteenth*;
  4. a mechanism's one short name, in §12's Glossary;
  5. results in tables, reasoning in paragraphs;
  6. one sentence a line in new text.

### The directives stay

- Each `@assert` stays under the sentence it backs (principle 6), and moves with it.
- No sidecar file is made.

### Named, not taken

§5.2's layout tables generated from the records' field comments, each of which names its offset, is its own ADR, after brief 077.

## Consequences

- Good: a reader knows where the work stands from one page of 60 lines. A round writes its result once, and links to it.
- Good: the whitepaper keeps the design, and new text cannot grow back past the limits.
- Good: the index reads as a list, and an ADR's conclusion is in its summary rather than its title.
- Bad: a large one-time move.
  - A lost directive or a broken link is the risk. `spec:guard`, `spec:prove` and `spec:graph` catch both, and the directive count is reported before and after.
- Bad: 196 summaries are new prose about old decisions. Each is drawn from its ADR's H1 and is reviewed.
- Bad: each round writes two more files, `docs/STATUS.md` and a report, with less text in each.
- Neutral: no engine code, test or pin changes.

## Alternatives considered and why rejected

- **Option 1(b), a report only**: a measured ADR takes its stopping rule's step, which is a decision. ADR-0150's merge-asking commit, ADR-0191's rules and spec-graph's obligations rest on it. It would also cut 18 per cent of ADRs, not two thirds.
- **Option 2(b), untouched**: the index could not be generated, and its rows would be written by hand once more.
- **Option 3(b), warnings first**: it fails on the history it does not change, and it blocks nothing until all of it is rewritten.
- **Option 4(b), a sidecar**: spec-guard does not read one, and a directive away from its sentence no longer says which claim it holds.
- **Option 5(b), the decision session**: a move of this size is a round, and a round is a brief run in its own session, one at a time.

## Confirmation

`briefs/077_one-place-for-each-kind-of-fact.md` builds it. `npm run spec` runs `spec:shape`, `spec:index` and their tests beside the checks it runs now.
