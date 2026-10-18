---
status: accepted
date: 2026-10-17
amends: ADR-0008
decision-makers: VirtualCortex maintainers
---

# ADR-0185: The spec tools upgraded, and spec-brief admitted — spec-guard 0.11.0 to 0.26.0 and spec-graph 0.8.0 to 0.17.0 read the tree as before, but spec-graph reads an archived document as a record from 0.9.0, so an archived brief with an open box no longer fails, as `briefs/README.md` said it did; spec-brief 0.8.0 holds the briefs' contract from `.spec-brief.json`, archives a round in one command that refuses an open box and writes an integrity hash, and fails an archived brief changed since; every archived brief carries that hash, `scripts/check-archive.mjs` fails one without it, and `scripts/check-briefs.mjs` retires; `spec-guard prove` joins the gate; spec-harness is not adopted

## Context and Problem Statement

[ADR-0008](0008-documentation-governance.md) pinned `@descent-vtt/spec-guard` and `@descent-vtt/spec-graph` exactly, as this project's own documentation tools. The tree held them at 0.11.0 and 0.8.0. Their author has released spec-guard 0.26.0 and spec-graph 0.17.0, and two new members of the family: spec-brief 0.8.0, for the briefs' lifecycle, and spec-harness 0.18.0, for an agent's round. The maintainers asked on 2026-10-17 which of them this tree should take.

**What the new versions read on this tree**, with the flags and configuration it already uses:

| Tool | Old | New |
| :--- | :--- | :--- |
| spec-guard | 578 of 578 assertions held | 578 of 578 held, the same count. It also names 8 directive-shaped comments it does not run; each is an example in inline code. |
| spec-graph | consistent; 5 159 relations, 41 open | consistent; 4 976 relations, 41 open |

**What changed underneath.**
- **spec-guard, 0.12.0 to 0.26.0**: Markdown is read by a CommonMark scanner; patterns it cannot read are refused; `archived` withholds a document as `proposed` does; unexpected errors exit 2. No flag the tree uses was removed. With `--ignore-status`, every document's directives still run.
- **spec-graph, 0.9.0**: `archived` is read as a record phase, not a retired one. Tried on a corpus of one archived brief with an open box: **0.8.0 exits 1, 0.17.0 exits 0.**
  - `briefs/README.md`'s step 3 and `CONTRIBUTING.md` say that spec-graph fails CI on an archived brief with an open box. Taken alone, the upgrade would make both sentences false, and the guard they name would go unenforced.
  - No archived brief holds an open box today, so nothing fails either way.
- **spec-graph's false positives** this tree has written around: one is fixed (*"amended by"* in an archived banner). Three are not: *"blocks"* near a link, *"left to"* after a link, and *"replaced by … (ADR-N)"*.

**spec-brief 0.8.0** reads its contract from a configuration file: the sections, their order, a section that must be a checklist, a phrase a section must contain, and the status words. With `.spec-brief.json` written for this tree, the 73 briefs pass with no change to any of them. It was tried before this file was written:
- a copy of brief 073 without *"Latest ≠ Newest"* fails `must-contain`;
- archiving that copy with its boxes open is refused, each box named;
- archiving it with its boxes ticked writes the banner and an `integrity` hash, and rewrites 11 relative links;
- one word changed after that fails `archive-freeze`.

Its collision check reads scopes the briefs here do not declare. Brief 065 and brief 073 must not run side by side because both dispatch the weekly workflow, not because they write one file.

**spec-harness 0.18.0** asks for what this tree does not have:
- merge commits and rulings in commits signed by an allowed key;
- briefs that declare `affectedFiles` and `protectedFiles` and carry a `## Rulings` section;
- branches named `brief/{id}`.

It also has limits of its own:
- Its guard sees Claude Code's Edit and Write and not a shell's write, which is how this tree's rounds make most edits.
- Its hook runs before and after each edit, and on Windows its own measurements put one call at 1.0 to 1.9 s, and 13.7 to 20.9 s under load.
- A configuration it cannot load holds every write.

**Against Latest ≠ Newest.** All four tools are below 1.0, and the oldest was begun six weeks ago. They do not pass principle 4's test for a dependency, which asks for two years of third-party production use. ADR-0008 admitted the first two as the project's own tools, and this admits spec-brief on the same ground, which this ADR now states rather than leaves implied:
- they run in the documentation gate, outside the engine's build and off the hot path;
- each has a written pre-1.0 policy: a change that can turn a passing check red comes only in a minor release, and a patch only fixes or reports less;
- they declare no runtime dependency, and `npm audit signatures` verifies the provenance attestations of all three;
- their mutation scores, in their own repositories, are 99.26 % (spec-guard), 96.60 % (spec-graph) and 97.83 % (spec-brief's core);
- they are pinned to exact versions, so a minor release reaches this tree only by a decision.

## Decision Drivers

- **The repository wins over the document** (principle 1): a sentence that says CI fails something must stay true.
- **A structural boundary beats a reviewed one** (principle 5): an archived brief should be unable to change without a check failing, which is more than the open-box guard ever held.
- **A check over nothing is not a pass**: an assertion that cannot fail should be found by a tool, not by a later round (the empty-scope trap).
- **One place for the briefs' contract**, not a script and a document that can drift.

## Considered Options

1. spec-guard: (a) 0.26.0; (b) stay at 0.11.0.
2. spec-graph: (a) 0.17.0, with the open-box guard given to spec-brief; (b) 0.17.0, with the guard added to `scripts/check-briefs.mjs`; (c) stay at 0.8.0.
3. spec-brief: (a) admitted, for the contract and for archiving; (b) not admitted.
4. spec-harness: (a) admitted; (b) not now.
5. `spec-guard prove` in the gate: (a) yes; (b) no.

## Decision Outcome

**Options 1(a), 2(a), 3(a), 4(b) and 5(a).**

- **The pins**: spec-guard 0.26.0, spec-graph 0.17.0 and spec-brief 0.8.0, each exact.
- **`.spec-brief.json`**:
  - the eight sections of `briefs/README.md`, in order;
  - `## Deliverables` a checklist;
  - `## Standing directives` must contain *"Latest ≠ Newest"* in every live brief;
  - `proposed` and `archived` the only status words;
  - archiving closes the boxes of `## Deliverables`, by a tick or by a note that starts `**Delegated`, `**Accepted debt` or `**Rejected`;
  - a banner of one paragraph;
  - the integrity hash.
- **The gate**, `npm run spec`:
  - `spec:briefs` is `spec-brief lint`: the contract over the live briefs, and the hash over every archived brief that carries one;
  - `spec:archive` is `node scripts/check-archive.mjs`, which fails an archived brief that carries no hash, and fails an archive with no brief in it; its tests are `spec:archive:test`;
  - `spec:prove` is `spec-guard prove` over the documents `spec:guard` reads. It shows each rule in force a violation of itself and fails on any that still passes. On this tree it read 578 rules, each seen to fail, in 64 s.
- **A round archives its brief** with `npx --no-install @descent-vtt/spec-brief archive <id> --pr <n> --summary "<one paragraph>"`, after ticking or dispositioning every deliverable, and commits the result.
  - The command refuses an open box, moves the file, sets the status, writes the banner and the hash, and rewrites the links.
  - The banner is one paragraph; what more the round has to say is in its ADR.
- **Every archived brief carries the hash** that spec-brief's own `integrityOf` computes. That covers the 71 archived when this was written, and brief 073's. Brief 073 was archived by its round under the steps before this ADR, and gained its hash when this branch took `main`. Each gained one line in its front matter and nothing else, and `spec-brief lint` then read every brief with no finding.
- **`scripts/check-briefs.mjs` and its tests retire.**
  - The contract they checked is `.spec-brief.json`'s.
  - Their floor at brief 028 has no case left: every live brief is numbered past it, and numbers only grow.
- **spec-harness is not adopted.** It is weighed again when this tree merges with merge commits and signs its commits, and when its guard sees a shell's writes.
- **spec-brief's `matrix` and `schedule` are not used.** The briefs here declare no scope.
- **`briefs/README.md`'s archiving steps, `CONTRIBUTING.md`, the READMEs and Appendix B** say this.

## Consequences

- Good: an archived brief cannot change without the gate failing, where before only an open box in it did.
- Good: an assertion that cannot fail is found by the gate.
- Good: the briefs' contract is in one file, and a round archives in one command that refuses what step 3 asked a person to check.
- Bad: the gate takes about a minute more, the time `prove` takes.
- Bad: an archive banner is one paragraph. The banners of rounds 032 to 073 run to several.
- Bad: reopening an archived brief, even to mend a link, is `spec-brief unarchive` and a new archive.
- Bad: three tools to keep at exact pins. Under their policy a minor release may turn this gate red, which is why each is a decision.
- Neutral: three of spec-graph's false positives remain, and the phrasing around them stays as it is.

## Alternatives considered and why rejected

- **Option 1(b) or 2(c), staying**: what the tree reads does not change, and what it gains (`prove`, CommonMark reading, the fixed false positive) is left behind.
- **Option 2(b), the guard in `check-briefs.mjs`**: it would restore the open-box guard alone, by a parser of this tree's own for what spec-brief already reads, and would leave an archived brief otherwise free to change.
- **Option 3(b)**: archiving would stay six manual steps with one of them unchecked.
- **Option 4(a), spec-harness now**: the tree has none of what it requires, and its guard does not see the writes this tree's rounds make.
- **Option 5(b)**: the empty-scope trap stays a thing a round finds by hand.

## Confirmation

- `package.json` and `package-lock.json`: the three exact pins, and the scripts `spec:prove`, `spec:briefs`, `spec:archive` and `spec:archive:test` in `npm run spec`.
- `.spec-brief.json`, `scripts/check-archive.mjs` and `scripts/check-archive.test.mjs`, nine tests. `scripts/check-briefs.mjs` and its tests are removed.
- The archived briefs, one front-matter line each: the 71, and brief 073's.
- **Tried**:
  - the whole gate on this tree with the new pins;
  - an edit to an archived brief fails `archive-freeze`, and its restoration passes;
  - the two old versions against the new on the same tree;
  - spec-graph's two versions on an archived brief with an open box;
  - spec-brief's lint and archive on copies of brief 073.
- Whitepaper Appendix B: the versions, the V-4 row and its directives; this decision's rows in §9 and the index.
