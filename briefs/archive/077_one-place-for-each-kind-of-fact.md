---
status: archived
date: 2026-10-19
affectedFiles: [docs/**, briefs/**, README.md, CLAUDE.md, CONTRIBUTING.md, CHANGELOG.md, package.json, .spec-graph.json, scripts/check-doc-shape.mjs, scripts/check-doc-shape.test.mjs, scripts/adr-index.mjs, scripts/adr-index.test.mjs]
protectedFiles: [crates/**, runtime/**, benches/**, .github/workflows/**, Cargo.toml, Cargo.lock, rust-toolchain.toml, package-lock.json, .spec-brief.json, scripts/arm-run.sh, scripts/exhaustive-shard.sh, scripts/exhaustive-costs.mjs, scripts/exhaustive-costs.tsv, scripts/check-archive.mjs, scripts/check-scope.mjs, scripts/check-explorations.mjs, scripts/check-decisions.mjs, scripts/check-version.mjs, scripts/check-deps.mjs]
integrity: sha256-9201bfab6d6e48ba3637b1c5d20d2de731528c1f39223162a2cd555b5813ecd9
---

<!-- spec-brief:banner -->
> **Executed 2026-10-20 (UTC) in pull request [#224](https://github.com/DescentVTT/VirtualCortex/pull/224).** Built ADR-0197, recorded in ADR-0199: docs/STATUS.md holds where the work stands, and docs/research/ a report for each of H-1 to H-41 and for two lines with no number, their text moved verbatim from the whitepaper's §11.1 and §6.5 with 266 directives. Every ADR's front matter carries a title, a summary, a kind and its hypothesis, the index, §9 and the hypotheses' tables are written from it by scripts/adr-index.mjs (spec:index), and scripts/check-doc-shape.mjs (spec:shape) holds new text to ADR-0197's limits. Whitepaper 5.0.0; F-72 to F-74 recorded and resolved; brief 076's sweep recorded with no survivor. The body below describes the tree before execution and is not maintained.
<!-- /spec-brief:banner -->

# Brief 077: One place for each kind of fact

## Mission

**This brief changes the documents and adds two checks. It changes no code of the engine, no test and no pin.**
[ADR-0197](../../docs/adr/0197-one-place-for-each-kind-of-fact.md) gives each kind of fact one home:
- where the work stands: `docs/STATUS.md`;
- what a hypothesis read: `docs/research/H-n.md`;
- why a thing was decided: an ADR with a short title and a summary.

Every other document links to them, and a check holds the shape of new text.

When the round is done, the tree holds:
- `docs/STATUS.md`, at most 60 lines, written from the standing as `CLAUDE.md` gives it now;
- `docs/research/README.md` and one report for each hypothesis H-1 to H-41, each hypothesis's text from §11.1
  moved into it verbatim;
- the whitepaper with its executive summary, §1.6, §6.5, §8.8's status cells and §11.1 made short and linked;
- `README.md`'s Status, `CLAUDE.md`'s "Where the work stands" and `docs/zh-TW/README.md` linking to
  `docs/STATUS.md`, carrying no status of their own;
- `title:`, `summary:`, `kind:` and, where there is one, `hypothesis:` in the front matter of every ADR, each H1
  and body byte for byte;
- the index and §9 generated from that front matter, held by `spec:index`;
- the shape check, `spec:shape`, on the lines a change adds or modifies;
- the writing rules in `CONTRIBUTING.md`, and the rules for a round's documents in `briefs/README.md`.

The scope is the front matter's ([ADR-0191](../../docs/adr/0191-a-rounds-scope-eighteen-shards-and-the-reproduction-first.md)):
`spec-brief archive --strict` refuses the round outside it.

**Run it after brief 076 has merged, and beside no other round.** Every round writes the whitepaper, `CLAUDE.md` and
`README.md`.

---

## Standing directives

- **Latest ≠ Newest** (whitepaper §2.1, `CLAUDE.md` principle 4). The round depends on nothing new: no package and no
  version bump. The two checks are Node scripts with no dependency, like `scripts/check-scope.mjs`.
- **Text that moves, moves verbatim.**
  - A sentence moved from the whitepaper to a report keeps its words, its links and the directives under it.
  - A sentence is shortened only where it stays.
  - Nothing is reworded on the way.
- **No directive is lost.**
  - The count of `@assert` comments over the scanned documents is the same before and after, or each one removed is
    named with the claim that went with it.
  - `spec:guard` and `spec:prove` pass.
- **History is not rewritten.**
  - An accepted ADR's H1 and body stay byte for byte; only its front matter gains keys.
  - No archived brief changes, since `spec:archive` holds their hashes.
  - `CHANGELOG.md`'s entries stay; one is added.
- **Every summary is drawn from its ADR.** It is taken from the ADR's H1 and decision outcome, claims nothing the ADR
  does not, and is at most 300 characters.
- **No code changes.** No file under `crates/`, `runtime/` or `benches/` is written, and no whole-domain test or
  dispatch is needed.
- **The branch is named `claude/brief-077-…`**, never rebased or force-pushed once its pull request is open. Every
  commit is signed.
- **The documents are read before a description of them is trusted**, this brief's and ADR-0197's included. So are
  spec-graph's and spec-guard's sources in `node_modules/@descent-vtt/`.
- **The pull request leaves draft with the commit that asks for the merge**, which sets the round's ADR to
  `accepted`.
- Conventional Commits with a real body; never commit on `main`; the required checks keep their names.

## Context

Re-derived on 2026-10-19 against `main` at `43e89b5`. Line numbers move; the symbols and the quoted sentences are
what to re-derive.

1. **The whitepaper** (`docs/WHITEPAPER.md`, 4.132.0 on ADR-0196's branch):
   - 1 045 K characters, 3 315 lines, 85 lines over 2 000 characters, 598 `@assert` comments.
   - The longest lines are §6.5's scenario R-5 paragraph (32 594), two of §11.1's hypotheses (18 475 and 17 569),
     the executive summary (10 935), a row of §5.2 (9 961) and a row of §8.8 (8 987).
   - §11.1 has 42 hypothesis items, `- [x] **H-n …` or `- [ ] **H-n …`, for H-1 to H-41. H-12 has two items, and
     its report takes both. There are 19 other items, the open questions among them.
   - §12 is the Glossary.
2. **Where status is written today**: `README.md`'s Status (one line of 34 919 characters), `CLAUDE.md`'s "Where the
   work stands", the executive summary, §1.6, §8.8, §11.1, the index and §9, `CHANGELOG.md` and
   `docs/zh-TW/README.md`.
3. **The checks** (`package.json`):
   - `npm run spec` chains them, and `spec:scripts:test` holds the chain to every `spec:*` and `*:test` script.
   - `spec:guard` and `spec:prove` scan `docs/**/*.md`, `briefs/*.md`, `README.md`, `CONTRIBUTING.md` and
     `CLAUDE.md`.
   - `spec:decisions` (`scripts/check-decisions.mjs`) asks every ADR for a row in §9 and in the index, by the row's
     opening link only.
   - `spec:scope` (`scripts/check-scope.mjs`) reads the base through `git show`, with `SPEC_SCOPE_BASE` defaulting
     to `origin/main`. The shape check reads it the same way.
4. **spec-graph** (`.spec-graph.json`):
   - `patterns` cover `docs/**/*.md` and the root documents.
   - `historyPatterns` are `CHANGELOG.md` and `docs/explorations/[0-9]*.md`.
   - `strict` is true.
   - Its check passed with `title:`, `summary:`, `kind:` and `hypothesis:` in an ADR's front matter, tried on
     ADR-0196 and reverted. Read how it uses each before relying on it.
5. **The ADRs**: 196 before ADR-0197.
   - 17 end in `-built.md` and 36 in `-measured.md`.
   - The H1's median is 561 characters.
   - None has `title:`, `summary:` or `kind:` today.
6. **The briefs' contract** (`.spec-brief.json`, protected): eight sections in order, and an archive by
   `spec-brief archive --strict`.

## Deliverables

- [x] **The round's ADR at the next free number (`ls docs/adr`)**, in ADR-0197's shape: what was built, and each
  choice made under the empowerment below.
- [x] **`docs/STATUS.md`**, at most 60 lines.
  - Where the work stands, written from `CLAUDE.md`'s "Where the work stands" and ADR-0196.
  - The current line, the last verdicts with their links, what does not exist, and the next decision.
- [x] **`docs/research/`.**
  - `README.md`: one table of every hypothesis — H-n, the question in at most 160 characters, the verdict, its ADRs,
    its report.
  - `H-1.md` to `H-41.md`: each with its §11.1 text moved verbatim, its directives with it.
  - The reports added to spec-graph's `historyPatterns`.
    H-23 and H-26 are excepted by name, since spec-graph reads their verbatim text as a handover into a record ([ADR-0199](../../docs/adr/0199-one-place-for-each-kind-of-fact-built.md)).
- [x] **The whitepaper**:
  - the executive summary, at most 300 words;
  - §1.6 and §8.8's status cells, at most 400 characters each: their label and their links;
  - §6.5's account of the rounds moved verbatim into the reports it belongs to, a short statement and links left;
  - §11.1's hypotheses as one table linking the reports, the open items kept as short items;
  - §9 generated;
  - the version in both declarations, with its date.
- [x] **`README.md`, `CLAUDE.md` and `docs/zh-TW/README.md`.**
  - README's Status: the four labels, each "Today" cell short, and a line to `docs/STATUS.md`.
  - CLAUDE.md's "Where the work stands": a link to `docs/STATUS.md` beside the table of where to read.
  - The zh-TW guide: a guide to the whitepaper's structure, its status replaced by the link.
- [x] **The ADRs' front matter.** `title:`, `summary:`, `kind:` and `hypothesis:` added to ADR-0001 to ADR-0196.
  - Each H1 and body is shown byte for byte its copy on the base, by the shape check.
  - ADR-0197 carries them already.
    ADR-0198, written after ADR-0197 without them, took them too.
- [x] **`scripts/adr-index.mjs` with its tests**, as `spec:index` and `spec:index:test`.
  - It writes the index's table (ID, title, kind, H-n, summary, status), and §9's (ID, title, status) between markers.
  - With `--check` it fails when the committed tables are not what it writes.
- [x] **`scripts/check-doc-shape.mjs` with its tests**, as `spec:shape` and `spec:shape:test`.
  - ADR-0197's limits on the lines a change adds or modifies against `origin/main`, in the living documents it names.
  - An accepted ADR's body equal to its base copy.
  - Its tests: each limit at its edge; a line unchanged from the base passing whatever its length; an archived brief
    and the base's CHANGELOG entries not read; an accepted ADR's body changed failing; an unreadable base failing.
- [x] **`npm run spec`** chains the four new scripts, and `spec:scripts:test` passes.
- [x] **`CONTRIBUTING.md`**: ADR-0197's writing rules, each named as checked (by which script) or held by review.
- [x] **`briefs/README.md`**: from brief 078, every round rewrites `docs/STATUS.md`. A round that reads a hypothesis
  writes its report and a short measured ADR. New ADRs take ADR-0197's shape.
- [x] **`CHANGELOG.md`**: one entry, at most 400 characters.
- [x] **The brief archived** with `spec-brief archive 077 --strict`, every deliverable dispositioned. The round's ADR is
  set to `accepted` by the commit that asks for the merge, and the pull request is marked ready for review with it.

## Not empowered

- **The round's scope** is the front matter's. It writes only inside `affectedFiles`, and never into
  `protectedFiles`. A file it must change outside the scope is handled as `briefs/README.md` says.
- **No accepted ADR's H1 or body changes**, and no archived brief changes.
- **No sentence that moves is reworded.** No claim is dropped unless a directive and its claim are named as removed,
  with the reason.
- **No limit of ADR-0197 is loosened** to let the round's own text pass.
- No code, test or pin of the engine changes, no dispatch is made, and no dependency is added.

## Architectural empowerment

The executing session may replace any instruction above with a better decision, recorded in its ADR. That covers:
- the key that holds an ADR's kind, if spec-graph reads `kind:` as something else;
- how `docs/STATUS.md` is laid out within its 60 lines;
- how a report is laid out around its moved text, and where §6.5's account is split between reports;
- the markers around §9's generated table, and how the generator writes the index;
- how the shape check finds a paragraph, a list item and a cell, and how it reads "added or modified" from the
  diff;
- a file the round must change and its scope does not hold, within this brief's mission, widened as
  `briefs/README.md` says.

It may not:
- write into a protected file, or shrink `protectedFiles`;
- reword moved text, change an accepted ADR's body, or change an archived brief;
- loosen a limit, or add a dependency;
- reach the standing directives, the whitepaper's invariants or the constraints in `CLAUDE.md`.

## Verification

Before each push, on the developer machine:

```bash
npm ci
npm run spec
```

No file under `crates/`, `runtime/` or `benches/` changes, which the scope holds, so the cargo lines of `CLAUDE.md`'s
first block change nothing here. The pull request's gate runs them all, and every check passes on the last commit.

```bash
npx --no-install @descent-vtt/spec-brief archive 077 --strict --pr <n> --summary "<one paragraph>"
```

- `spec:guard` and `spec:prove` pass, with the directive count reported before and after.
- `spec:graph` passes with the reports as history.
- `spec:index --check` and `spec:shape` pass on the round's own diff.
- The archive passes `--strict`: nothing outside `affectedFiles` and nothing in `protectedFiles` changed.

## Report

The closing message states:
- the size of the whitepaper, `README.md`, `CLAUDE.md`, the index and the zh-TW guide, before and after;
- the count of directives before and after, and any removed with its claim;
- the reports written, and where §6.5's account went;
- that every accepted ADR's H1 and body is byte for byte its base copy;
- the kind key chosen, and why, if not `kind:`;
- the two checks, and what each holds;
- any widening of `affectedFiles`, and why;
- that the pull request is ready for review;
- what was not done and why.
