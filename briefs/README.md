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
| `## Standing directives` | The repository's always-on rules, restated. Latest ≠ Newest is always among them (`CLAUDE.md` principle 4), applied to what the round could be tempted to adopt; `npm run spec:briefs` fails a live brief whose section does not name it. |
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
| Live | `briefs/NNN_title.md` | `status: proposed` |
| Executed | `briefs/archive/NNN_title.md` | `status: archived` |

Numbers are three digits, allocated in order, never reused. Take the next free one with `ls briefs briefs/archive`.

## Archiving one

A round archives its brief with `spec-brief` ([ADR-0185](../docs/adr/0185-the-spec-tools-upgraded-and-spec-brief.md)):

1. Disposition every deliverable in place: `[x]` when done, or a `**Delegated to** ...`,
   `**Accepted debt:** ...` or `**Rejected:** ...` note under the box.
2. Run `npx --no-install @descent-vtt/spec-brief archive <NNN> --pr <n> --summary "<one paragraph>"`.
   The summary says what the round did in one paragraph: the ADRs written, the findings closed,
   the verdict. Whatever more there is to say belongs in the ADR. The command:
   - refuses an open box, naming each one;
   - moves the file to `briefs/archive/` and sets `status: archived`;
   - writes the frozen banner, which ends *"The body below describes the tree before execution and
     is not maintained."*;
   - gives every relative link one `../`, and rewrites the links other live briefs hold to it;
   - writes an `integrity` hash into the front matter. It commits nothing.
3. Commit the result with the round.

**An archived brief does not change.** `npm run spec:briefs` fails one changed since its hash was
written, and `npm run spec:archive` fails one that carries no hash. To reopen a round, run
`spec-brief unarchive <NNN>`, which takes the banner and the hash off. `spec-guard` does not run on
`archive/`, so a precondition directive left there runs nowhere. `spec-graph` still checks the links
of archived briefs. Since 0.9.0 it reads an archived brief as a record, and an open box there no
longer fails it, which is why the refusal and the hash are spec-brief's.

## Evidencing a round

How a round that measures reaches its merge ([ADR-0150](../docs/adr/0150-a-round-waits-for-what-it-checks.md)).
A brief written before that ADR, or before [ADR-0184](../docs/adr/0184-the-runs-off-the-developer-machine.md), may ask
for more waiting than this or for a run on the developer machine; where the two differ, this
section is the rule.

1. **No whole-domain test runs on the developer machine**
   ([ADR-0184](../docs/adr/0184-the-runs-off-the-developer-machine.md)). A round's runs are dispatches on its branch:
   - **The calibration**, before any rewarded run: `gh workflow run ci.yml --ref <branch> -f
     scope=exhaustive -f exclude='<test> ...'`, naming every test of the round's own. A name the
     listing lacks fails every shard before any test runs.
   - **The arms run once**, and so do the round's lead-ins and any trial of its harness on an arm
     the tree pins: `gh workflow run ci.yml --ref <branch> -f scope=arms -f tests='<test> ...'`.
     Each test runs in a job of its own on AArch64 with its output kept as the artifact
     `arm-<n>`; read it with `gh run download <run> -n arm-<n>`.
   - **The tables are written from those logs and committed.** The ADR names the run, the commit
     it ran on, and the job each table came from. The run's commit holds the protocol.
   - **The dispatch at `scope=exhaustive` after the tables are pinned reproduces them**, on x86-64.
     A table it does not reproduce stops the round as a finding.
   - Everything else before the arms is unchanged: the protocol committed first, the calibration
     before any rewarded run, and constants that do not move after one.

   Before a push the developer machine runs only the first block of `CLAUDE.md`'s commands, and
   the pull request's gate runs the rest.
2. **The reproduction's scope follows the diff**
   ([ADR-0075](../docs/adr/0075-the-dispatch-scope-follows-the-diff.md)): `scope=both` when a file
   under `src/` changed, `scope=exhaustive` otherwise. The calibration is at `scope=exhaustive`
   always.
3. **The round asks for its merge when the reproduction's whole-domain shards are green** and the
   pull request's gate is green on its last commit. It does not wait for the sweep. Its ADR names
   the dispatch and says the sweep had not ended.
4. **The cost table** is regenerated from the reproduction's shards' artifacts, which the run gives
   as each shard uploads them.
5. **The ADR cites the round's commits by the hashes its branch holds, with the pull request's
   number.** They are not rewritten after the merge, and no second pull request follows.
6. **The commit that asks for the merge sets the round's ADRs to `accepted`**, in their files and in
   the index.
7. **The sweep's outcome is read when it ends and written down by the first ADR that merges after
   it has ended**, a decision's or a round's
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
