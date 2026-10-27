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
lacks one or holds them out of order; the contract is `.spec-brief.json`'s
([ADR-0185](../docs/adr/0185-the-spec-tools-upgraded-and-spec-brief.md)).

| Section | What it holds |
| :--- | :--- |
| `## Mission` | One paragraph: the state of the tree when the brief is done. |
| `## Standing directives` | The repository's always-on rules, restated. Latest ≠ Newest is always among them (`CLAUDE.md` principle 4), applied to what the round could be tempted to depend on; `npm run spec:briefs` fails a live brief whose section does not name it. What else the round does not take on, a mechanism or a change beside its own, is its scope, written in `## Not empowered`, not this directive ([ADR-0187](../docs/adr/0187-the-upgrade-suite-planned.md)). |
| `## Context` | Facts the brief rests on, each with a path, re-derived on the stated date. Line numbers move; symbol names and quoted sentences are what to re-derive. |
| `## Deliverables` | A checkbox list. `spec-graph` reads each box as an obligation with real state. |
| `## Not empowered` | The cheap wrong moves this round must not make. |
| `## Architectural empowerment` | The licence to override the brief's *instructions* with a better decision, recorded as an ADR. It never reaches the whitepaper's invariants or the constraints in `CLAUDE.md`. |
| `## Verification` | The commands and the expected results, the mutation gate on the diff among them ([ADR-0030](../docs/adr/0030-verification-governance.md)). |
| `## Report` | What the closing message must state, always including what was not done. |

A live brief may also carry `<!-- @assert-* -->` directives that state its **precondition** (the
defect it fixes still exists). `spec-guard` runs them: when the precondition stops holding, CI fails
until the brief is archived or rewritten, which is how a stale brief is caught. It runs them because `npm run spec:guard` passes `--ignore-status`: a live brief is
`proposed`, and spec-guard 0.6.0 and later otherwise report a proposed document's directives
without executing them.

## What a brief is not

**A record of what happened.** Outcomes live in the ADR the round writes, in the changelog, in the
whitepaper's §11 dispositions and in the code. Do not add a status file here.

## Lifecycle

| Phase | Where | Front matter |
| :--- | :--- | :--- |
| Live | `briefs/NNN_title.md` | `status: proposed`; from brief 075, `affectedFiles` and `protectedFiles` |
| Executed | `briefs/archive/NNN_title.md` | `status: archived` |

Numbers are three digits, allocated in order, never reused. Take the next free one with `ls briefs briefs/archive`.

**A brief's scope** ([ADR-0191](../docs/adr/0191-a-rounds-scope-eighteen-shards-and-the-reproduction-first.md)). From brief 075 the front matter lists, in
spec-brief's globs, what the round may write, `affectedFiles`, and what it may not, `protectedFiles`,
each with at least one pattern:

```yaml
affectedFiles: [runtime/menteion-runtime/tests/**, docs/**, briefs/**, CHANGELOG.md, CLAUDE.md, README.md, scripts/exhaustive-costs.tsv]
protectedFiles: [crates/**/src/**, runtime/menteion-runtime/src/**, .github/workflows/**, scripts/arm-run.sh, scripts/exhaustive-shard.sh, scripts/exhaustive-costs.mjs, rust-toolchain.toml]
```

A test-only round protects the sources, as above. Every round protects the workflow, the run
scripts and the toolchain unless the brief is for them. `npm run spec:scope` fails a brief from 075
without both lists, and one that protects less than its live copy on `origin/main`.

## Archiving one

A round archives its brief with `spec-brief` ([ADR-0185](../docs/adr/0185-the-spec-tools-upgraded-and-spec-brief.md)):

1. Disposition every deliverable in place: `[x]` when done, or a `**Delegated to** ...`,
   `**Accepted debt:** ...` or `**Rejected:** ...` note under the box.
2. Run `npx --no-install @descent-vtt/spec-brief archive <NNN> --strict --pr <n> --summary "<one paragraph>"`.
   The summary says what the round did in one paragraph: the ADRs written, the findings closed,
   the verdict. Whatever more there is to say belongs in the ADR. The command:
   - refuses an open box, naming each one;
   - from brief 075, measured from `origin/main` (`archiving.base`), refuses a protected file the
     round changed and, being strict, a file outside `affectedFiles`;
   - moves the file to `briefs/archive/` and sets `status: archived`;
   - writes the frozen banner, which ends *"The body below describes the tree before execution and
     is not maintained."*;
   - gives every relative link one `../`, and rewrites the links other live briefs hold to it;
   - writes an `integrity` hash into the front matter. It commits nothing.
3. Commit the result with the round.

**A file the round must change and its scope does not hold** ([ADR-0191](../docs/adr/0191-a-rounds-scope-eighteen-shards-and-the-reproduction-first.md)):
1. within the brief's mission and its architectural empowerment, the round widens its brief's
   `affectedFiles` in a commit of its own, the file and the reason in its message, and its ADR says
   so; `npm run spec:scope` prints each widening for the review;
2. protected, or past the empowerment, the round does not widen: it stops at that point and says
   what it needs and why, for the next decision;
3. allowed by the maintainers in the reviewing session, the allowance and its reason go into the
   round's ADR, and the round then widens.

**An archived brief does not change.** `npm run spec:briefs` fails one changed since its hash was
written, and `npm run spec:archive` fails one that carries no hash. To reopen a round, run
`spec-brief unarchive <NNN>`, which takes the banner and the hash off. `spec-guard` does not run on
`archive/`, so a precondition directive left there runs nowhere. `spec-graph` still checks the links
of archived briefs. Since 0.9.0 it reads an archived brief as a record, and an open box there no
longer fails it, which is why the refusal and the hash are spec-brief's.

## A round's documents

From brief 078, a round writes its result once, in the place [ADR-0197](../docs/adr/0197-one-place-for-each-kind-of-fact.md) gives it, and links to it from everywhere else.
- **Every round rewrites [`docs/STATUS.md`](../docs/STATUS.md) whole**: where the work stands, the last verdicts with their links, what does not exist, and the next decision, in at most 60 lines.
- **A round that reads a hypothesis writes its report**, `docs/research/H-n.md`, in its pull request: the front matter `hypothesis:`, `question:` and `verdict:`, the hypothesis as written before any run, its readings' tables and the dispatches that are their evidence.
- **Its measured ADR is short**: the verdict per network, arm and clause in one table, each predicted reading as held or missed in a line, and the stopping rule's step reached with the next decision it names. The protocol ADR and the built ADR are as they were.
- **A new ADR takes ADR-0197's shape**: `title:`, `summary:`, `kind:` and, where it has one, `hypothesis:` in its front matter, its H1 `# ADR-NNNN: ` and the title, and **In brief** first.
- **The tables are written, not edited**: `node scripts/adr-index.mjs` writes the index, §9 and the hypotheses' tables from front matter, and `npm run spec:index` fails a hand edit.
- **New text meets the limits** `npm run spec:shape` holds ([CONTRIBUTING.md](../CONTRIBUTING.md#where-each-fact-goes-and-the-shape-of-new-text)), and the `CHANGELOG.md` entry is at most 400 characters.

## When the session needs the maintainer

A round runs for hours, and the maintainer is often away from it.
A session that executes a brief and cannot go on without the maintainer sends a push notification, Claude Code's `PushNotification`, in one line that names what it needs.
- **It notifies** when a question only the maintainer can answer stops the round, when a permission or a sign-in waits for them, and when the pull request is ready for review.
- **It does not notify** for progress, for a dispatch started or ended, or for a step it can take itself.
- It waits for no reply it does not need: a step the brief or its empowerment already decides is taken.

## Evidencing a round

How a round that measures reaches its merge ([ADR-0150](../docs/adr/0150-a-round-waits-for-what-it-checks.md)).
A brief written before that ADR, or before [ADR-0184](../docs/adr/0184-the-runs-off-the-developer-machine.md), may ask
for more waiting than this or for a run on the developer machine; where the two differ, this
section is the rule.

1. **No whole-domain test runs on the developer machine**
   ([ADR-0184](../docs/adr/0184-the-runs-off-the-developer-machine.md)). A round's runs are dispatches on its branch:
   - **The calibration is what the change needs**
     ([ADR-0215](../docs/adr/0215-the-calibration-a-change-needs.md)), read by `npm run
     round:additive -- origin/main <the protocol's commit>` and quoted in the protocol ADR:
     - exit 0, the change only adds: the round's own calibration tests, those its stopping rule's
       step 2 names, at `scope=arms` beside the arms, and no dispatch at `scope=exhaustive`;
     - exit 1: `gh workflow run ci.yml --ref <branch> -f scope=exhaustive -f exclude='<test>
       ...'`, naming every test of the round's own. A name the listing lacks fails every shard
       before any test runs.
   - **The arms run once**, and so do the round's lead-ins and any trial of its harness on an arm
     the tree pins: `gh workflow run ci.yml --ref <branch> -f scope=arms -f tests='<test> ...'`.
     Each test runs in a job of its own on AArch64 with its output kept as the artifact
     `arm-<n>`; read it with `gh run download <run> -n arm-<n>`.
   - **The calibration and the arms are dispatched together**, on the protocol's commit
     ([ADR-0211](../docs/adr/0211-the-round-shortened.md)). The calibration is read first, with
     any twin its stopping rule holds bit for bit to a pinned arm. No arm's log is read until
     every calibration shard or test and every twin is green; one that fails stops the round as
     a finding, and its arms are void.
   - **The pull request opens as a draft with the round's first push**, and at the latest with
     the protocol's commit ([ADR-0223](../docs/adr/0223-the-gate-shortened-and-explorations-kept-as-tags.md)),
     so that its gate, the mutation gate included, runs while the calibration does. A survivor
     found before the protocol's commit may be killed under `src/`; one found after it is killed
     by a new test where one can catch it, since a change under `src/` then is the change after
     the calibration below.
   - **The tables are written from those logs and committed.** The ADR names the run, the commit
     it ran on, and the job each table came from. The run's commit holds the protocol.
   - **A change after the calibration** ([ADR-0191](../docs/adr/0191-a-rounds-scope-eighteen-shards-and-the-reproduction-first.md)): a change to a file
     under `src/`, or to a test the calibration ran, made after the calibration's commit and before
     a run, is calibrated again by a second dispatch, or the ADR says why it changes no run and the
     reproduction is the check.
   - **The dispatch at `scope=exhaustive` after the tables are pinned reproduces them**, on x86-64.
     A table it does not reproduce stops the round as a finding. **It is dispatched as soon as the
     tables are pinned and pushed**; the readings, the documents and the archive are written while
     it runs, and after its commit no file under `runtime/`, `crates/` or `benches/` and no
     `Cargo.toml` or `Cargo.lock` changes (ADR-0191).
   - **It runs the round's own tests alone** (ADR-0211): `gh workflow run ci.yml --ref <branch> -f
     scope=exhaustive -f only='<test> ...'`, every test of the round's own. That holds when,
     from the calibration's commit to its own, no file under `src/` changed and no test or helper
     the calibration ran; the round reads it from `git diff`, and the merge's reviewer again.
     Otherwise it runs every test, with no `only`.
   - Everything else before the arms is unchanged: the protocol committed first, the calibration
     read before any rewarded run, and constants that do not move after one.

   Before a push the developer machine runs only the first block of `CLAUDE.md`'s commands, and
   the pull request's gate runs the rest.
2. **A round never dispatches the sweep** (ADR-0211, amending
   [ADR-0075](../docs/adr/0075-the-dispatch-scope-follows-the-diff.md)): its calibration and its
   reproduction are at `scope=exhaustive`, whatever its diff. The schedule sweeps the whole tree
   every week, on Wednesday at 20:17 UTC (ADR-0223), and the pull request's mutation gate meets
   every line the round changed.
3. **The round asks for its merge when the reproduction's whole-domain shards are green** and the
   pull request's gate is green on its last commit. It waits for no sweep.
4. **The cost table** is regenerated from the calibration's and the reproduction's shards'
   artifacts together, downloaded under one directory: `node scripts/exhaustive-costs.mjs from
   <dir> --run '<calibration> and <reproduction>' --keep scripts/exhaustive-costs.tsv`, written to
   a new file and moved over the table. A shard whose dispatch named `only` ran its tests alone
   and is not read, so the round's own tests keep the table's default until a full deal measures
   them ([ADR-0220](../docs/adr/0220-the-cost-of-a-test-run-alone.md), F-82).
5. **The ADR cites the round's commits by their hashes, with the pull request's number**, and
   they are `main`'s once it merges: `main` takes a pull request by a merge commit
   ([ADR-0186](../docs/adr/0186-merge-commits.md)). A branch is brought up to date by merging `main` into it,
   never by a rebase or a force-push once a run has been made on it or its pull request is open, so
   every commit a dispatch ran on stays in the history. No second pull request follows.
6. **The commit that asks for the merge sets the round's ADRs to `accepted`**, in their files and in
   the index.
7. **The sweep's outcome, the schedule's since ADR-0211, is read when it ends and written down by
   the first ADR that merges after it has ended**, a decision's or a round's
   ([ADR-0158](../docs/adr/0158-the-sweep-off-the-next-decisions-path.md)). No ADR waits for it: one
   written before it ends names the run and says so. The session that writes an ADR looks whether
   the last dispatched sweep has ended and whether an ADR has recorded it. A survivor is a finding
   there, and the next round's list.

## Writing one

Copy the newest live brief and replace every section. Re-derive the context on the day you write
it and say so. Write the empowerment clause for *this* round: what it may decide is specific to the
work and is the one part that must be thought about rather than templated.

**Never allocate an ADR number in a brief.** Write "a new ADR at the next free number
(`ls docs/adr`)"; the executing round takes the number when it writes the file. `spec-graph`
reports a citation of an ADR that does not exist, which is how the first three briefs were caught
naming one.
