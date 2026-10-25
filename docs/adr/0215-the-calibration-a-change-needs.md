---
status: accepted
date: 2026-10-25
amends: [ADR-0184, ADR-0211]
title: The calibration a change needs
kind: process
summary: A round whose change only adds, read so by scripts/check-additive.mjs, calibrates with its own calibration tests alone, beside its arms. Any other change calibrates the whole tree as before, and the schedule runs every test each week.
decision-makers: VirtualCortex maintainers
---

# ADR-0215: The calibration a change needs

**In brief**
- **Decision.** A round whose change only adds calibrates with its own calibration tests alone, dispatched beside its arms. A change only adds when `scripts/check-additive.mjs` reads every line a Rust build reads as whole items inserted into a package's `tests/`, under names its crate does not use. Any other change calibrates the whole tree, as before. The schedule runs every test each week.
- **Why.** Brief 081's calibration was the longest step of its path: 104.6 minutes for 187 tests its change could not reach, beside arms of 19 minutes. The weekly job grows by about 18 000 s a round.
- **Left open.** A change under `src/` still calibrates the whole tree. A calibration of only what a change to the source can reach is not attempted.

## Context and Problem Statement

[ADR-0184](0184-the-runs-off-the-developer-machine.md) made a round's calibration a dispatch of every whole-domain test but the round's own, to show that its build moves no number an earlier round pinned. [ADR-0211](0211-the-round-shortened.md) dispatched it beside the arms.

**Brief 081, the first round under ADR-0211** ([ADR-0214](0214-three-in-four-on-the-default-measured.md)), changed nothing under `src/` and added its harness and its tests at the end of `tests/inhibition.rs`:

| Step | Dispatched | Ended | Minutes |
| :--- | :--- | :--- | ---: |
| Its three calibration tests, at `scope=arms` | 13:53Z | 14:04Z | 10 |
| The calibration, 187 tests in 18 shards | 13:53Z | 15:38Z | 105 |
| The twelve arms | 14:05Z | 15:18Z | 73 |
| The reproduction, the twelve arms alone (`-f only`) | 15:44Z | 16:06Z | 22 |

- Each arm ran for about 19 minutes, but the twelve took 73: the calibration's eighteen shards held the places of the account's twenty. The tables then waited 20 minutes more for its last shard.
- The calibration's longest shard read 58 % of its 180-minute bound, against [ADR-0179](0179-the-bounds-rule-restated.md)'s line at 60.
- The weekly job holds 199 tests and 176 376 s. `inhibition` holds 137 of them and 94 % of their seconds.

**What can move a pinned number, and what cannot.** A test's number moves only if the code it runs changes. That code changes with a line under `src/`, a manifest or the toolchain; with a changed line of a test or of a helper; with a line inserted inside an existing item; and with a new item that shadows a name a glob import gave the code there, or a constant that turns a binding in a pattern into a comparison.
A whole item inserted at the top level under a name its crate does not use, or a new file, changes no line any earlier test runs.

## Decision Drivers

- **The maintainers' choice** (2026-10-25): E taken, now that ADR-0211 has run once and the tables stay where they are ([ADR-0212](0212-three-in-four-on-the-default.md)).
- **A structural boundary beats a reviewed one** (principle 5): the reading is a script with its tests, not a session's reading of a diff.
- **What each run checks is kept.** The schedule runs every whole-domain test every Monday. The round's own calibration tests still hold its stopping rule's step 2, and the reproduction still runs the round's tests on x86-64.
- **Latest ≠ Newest**: git and Node, as the other checks; no dependency.

## Considered Options

1. **What decides**: (a) a script over the diff; (b) the round's reading of its diff; (c) nothing, every round calibrating the whole tree.
2. **What a round that only adds calibrates**: (a) its own calibration tests; (b) those, and every whole-domain test of the binaries it touches.

## Decision Outcome

**Options 1(a) and 2(a)**, built in this pull request.

### The check

`node scripts/check-additive.mjs <base> <head>`, or `npm run round:additive -- <base> <head>`, reads `git diff` from the two commits' merge base. A file a Rust build reads is additive only when:
1. it lies under a package's `tests/` directory;
2. it is new, or every hunk of it inserts lines and removes none;
3. each insertion into a file that was there goes in at the top level after a line that ends an item, and is itself whole items: its brackets balanced, its first line an item's, its last ending one;
4. no item it inserts is a `use`, an `impl` for a type the base names, or an item under a name its crate can already see: the file's own text, the modules the test crates share under `tests/`, the library and the state crates.

Brackets are counted outside comments, strings and character literals, all three kinds, since a table's rows sit inside `[`. It exits 0 when the change only adds, and 1 with each file and reason when it does not. Its tests are `round:additive:test`, in `npm run spec`.

### The round's calibration

- **The round runs the check on its protocol's commit**, against `origin/main`, and its protocol ADR quotes the output.
- **When it exits 0**, the calibration is the round's own calibration tests, those its stopping rule's step 2 names, dispatched at `scope=arms` beside the arms. No dispatch at `scope=exhaustive` is made. No arm's log is read until every calibration test is green.
- **When it exits 1**, the calibration is ADR-0184's and ADR-0211's: every test but the round's own, at `scope=exhaustive`, beside the arms.
- **The reproduction is ADR-0211's**: `-f only` with the round's own tests, its calibration tests among them, when nothing the calibration ran has changed since.
- **The session that merges runs the check again** on the same commits.

### What does not change

- The schedule runs every whole-domain test and the sweep every Monday.
- The pull request's gate runs every test it ran.
- A round that changes `src/`, a manifest, a line that was there, or a name its crate sees calibrates the whole tree.

### What it saves (Target)

A round that only adds waits for its arms and its reproduction alone: about an hour, where brief 081's path took 2 h 13 min. Its arms also take the eighteen places the calibration's shards took.

## Consequences

- Good: a round that only adds no longer waits for tests its change cannot reach.
- Good: the boundary is a check with tests. On the tree's history it reads brief 081's protocol, `25f49e4...72959ae`, as additive, and its pins, brief 080's round and brief 079's as not.
- Bad: a pinned number such a round could still move, by a path the check does not read, shows on the schedule's Monday, not before the merge.
- Bad: the check reads Rust's text, not the compiler's output. A construct it misreads would most likely read as not additive, and cost a whole calibration; a misreading the other way would cost a week.
- Neutral: no code, test or pin of the engine changes.

## Amended

- **ADR-0184**: a round's calibration is every whole-domain test but its own *when its change does not only add*. Otherwise it is the round's own calibration tests.
- **ADR-0211**: the calibration dispatched beside the arms is the one this record names.
- Each is accepted and is not edited.

## Alternatives considered and why rejected

- **Option 1(b), a reading by the round**: a calibration skipped by a misreading costs a week, and ADR-0211's reading for the reproduction is already the one place a person decides.
- **Option 1(c), every round in full**: brief 081 paid 105 minutes for 187 tests its change could not reach, and the job grows by about 18 000 s a round.
- **Option 2(b), the binaries it touches**: nearly every round adds to `inhibition`, which holds 94 % of the whole-domain seconds, so it is nearly the whole calibration.

## Confirmation

- `npm run spec` runs `round:additive:test`, eighteen cases: the scan, the hunks, the insertions, the names, and the verdict on each kind of change.
- The check was run on the tree's history: `25f49e4...72959ae` reads additive in under a second; `72959ae...55097b8`, brief 080's round and brief 079's do not.
- The next round that only adds quotes the check's output in its protocol ADR, and its path's minutes are read in its record.
