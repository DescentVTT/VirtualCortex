---
status: accepted
date: 2026-10-27
amends: [ADR-0193, ADR-0211]
title: The gate shortened, the schedule moved, and explorations kept as tags
kind: process
summary: The gate's Rust jobs skip two trees a green gate already tested, and the mutation gate runs in three shards. A round opens its pull request by its protocol. The schedule runs Wednesday at 20:17 UTC. A finished exploration's branch becomes a tag. F-85 is resolved.
decision-makers: Menteion maintainers
---

# ADR-0223: The gate shortened, the schedule moved, and explorations kept as tags

**In brief**
- **Decision.** Five changes to how a pull request and a round reach a merge, and none to the engine:
  1. the gate's four Rust jobs skip a pull request whose two trees, its base's and its merge's as those jobs read them, a green gate of this repository already tested;
  2. the mutation gate runs in three shards;
  3. a round's pull request opens as a draft with its first push, and at the latest with its protocol's commit, so that its gate runs while the calibration does;
  4. the schedule runs on Wednesday at 20:17 UTC, Thursday 04:17 in Taiwan;
  5. a finished exploration's branch becomes a tag of the same name, which a ruleset keeps.
- **Why.** Brief 082 took about six hours. Two of them were a reproduction of every test, because a change under `src/` for three mutation survivors came after the calibration. Its gate ran 51 minutes more on a last commit of documents and the cost table. The schedule began 5 to 7 hours late each week, and once not at all.
- **F-85** is recorded and resolved: the artifacts were kept 14 days where the workflow asked 90.

## Context and Problem Statement

**Brief 082's path**, read from its runs ([ADR-0219](0219-a-delay-the-engine-reads-measured.md)):

| 2026-10-25 to 26, UTC | |
| :--- | :--- |
| The decision merged | 18:51 |
| The calibration and the arms, 40226374616 | 19:30 to 21:38 |
| The reproduction of every test, 40237389553 | 21:50 to 23:57 |
| The gate on the last commit, 40247623536 | 00:05 to 00:56 |
| The merge | 00:58 |

- **The reproduction ran every test** ([ADR-0211](0211-the-round-shortened.md)). A mutation run on the developer machine found three survivors during the calibration, and a refactor under `src/` killed them at 20:10 (a3840cf), after the calibration's commit. The pull request, and with it the gate, opened at 21:58.
- **The last commit changed documents and `scripts/exhaustive-costs.tsv`**, which no Rust job reads. The gate ran its four Rust jobs again. Its mutation job read 83 mutants, an unmutated baseline of 18 minutes and the mutants in 33. The Rust job took 28 minutes, and 24 to 31 in the last three gates.
- **ADR-0211 skips the Rust jobs for a pull request of documents alone.** A pull request that changes code once, then documents, runs them on every push.

**The schedule** (`0 3 * * 1`, Monday 03:00 UTC):

| Week of | The run began | It ended |
| :--- | :--- | :--- |
| 2026-09-21 | 08:30 | 11:16 |
| 2026-09-28 | 09:16 | 12:00 |
| 2026-10-05 | no run | |
| 2026-10-12 | 10:07 | 15:00 |
| 2026-10-19 | 08:24 | 16:51 |
| 2026-10-26 | 08:25 | 15:06 |

GitHub's documentation says a schedule can be delayed under load, the start of each hour among the busiest times, and a queued job dropped. Of 121 dispatches from 2026-09-17 to 2026-10-26, by the weekday in Taiwan, 39 were on a Sunday, 34 on a Monday and 3 on a Thursday; 10 began between 04:00 and 09:00 there.

**The explorations.** [ADR-0193](0193-an-exploration-lane.md) keeps each exploration's branch, so that the commits its runs name stay reachable. Five branches are kept, `explore/001` to `explore/005`. Since 2026-10-27 the repository deletes a merged pull request's branch, and the ruleset "Main Protect" holds `main` alone, so nothing keeps an `explore/` branch from being deleted or moved.

**The artifacts.** `ci.yml` asks 90 days for each of its three uploads. The repository allowed 14, already so before the move to its own organisation: run 39222543243's artifacts, of 2026-10-19, expire on 2026-11-02. GitHub keeps the lesser without a warning. On 2026-10-27 the organisation's limit and the repository's were set to 90 (F-85).

## Decision Drivers

- **The gate's evidence is the input it tested.** A second run on the same input reads nothing the first did not.
- **A structural boundary beats a reviewed one** (principle 5): the reuse is a hash a job computes, not a reader's view of a diff, and a ruleset keeps the tags.
- **A doubt runs the jobs.** A fork, a lookup that fails, no fingerprint: each runs them.
- **Latest ≠ Newest**: no new dependency. The weekly sweep has run cargo-mutants' `--shard` since ADR-0058, and the artifacts API and tag rulesets are GitHub's own.

## Considered Options

1. **When the gate's Rust jobs skip**: (a) when the two trees' fingerprint is one a green gate kept; (b) when the commits since the pull request's last green head touch no file they read; (c) only for documents alone, as now.
2. **The mutation gate**: (a) three shards; (b) two; (c) one job, as now.
3. **When a round's pull request opens**: (a) as a draft with its first push, at the latest with its protocol's commit; (b) when the round asks for review.
4. **The schedule**: (a) Wednesday 20:17 UTC; (b) Friday 20:17 UTC; (c) Monday 03:17 UTC.
5. **A finished exploration's branch**: (a) a tag of its name, kept by a tag ruleset; (b) the branch, kept by a branch ruleset; (c) the branch, as now.

## Decision Outcome

**Options 1(a), 2(a), 3(a), 4(a) and 5(a)**, built in this pull request, but for 5's ruleset and conversion, made once it merges.

### The gate on two trees already tested

- **`scripts/gate-fingerprint.mjs <base> <head>`** hashes what `git ls-tree -r` lists of each tree for the paths the Rust jobs read: those of `readByRust` ([ADR-0215](0215-the-calibration-a-change-needs.md)), `ci.yml` and `.gitattributes`. A blob, a mode or a path that changes changes the hash. Its six cases are `npm run gate:fingerprint:test`.
- **The two trees** are the base's and the merge's, which the jobs test and between which the mutation gate reads its lines. A merge of `main` into the branch, or `main` moving, changes the merge's tree, and so the hash, whenever it touches a file the jobs read.
- **The `changes` job** skips the four jobs, as it does for documents alone, when an artifact `gate-green-<hash>` exists, unexpired, from a run on this repository's own branches. A fork's pull request always runs them, since a fork's run executes the fork's workflow, which could keep any name.
- **The `gate-green` job** keeps that artifact, 90 days, when all four jobs ran and passed, each mutation shard included. A run that skipped them keeps nothing.

### The mutation gate in three shards

- **`--shard k/3 --sharding round-robin`**, one job a shard. Each runs its own baseline, which sets its timeouts. A shard dealt no mutant passes with no build: cargo-mutants 27.1.0 printed "Found 0 mutants" and exited 0 on one here.
- **On brief 082's list** that is about 18 + 11 = 29 minutes where it was 51, a Target until a gate reads it.
- **The mutation gate is not a required check**, so its new names change nothing in "Main Protect".

### A round's pull request, opened with its protocol

- **The pull request opens as a draft with the round's first push**, and at the latest with its protocol's commit (`briefs/README.md`). Its gate, the mutation gate included, runs on the host while the calibration does, and not on the developer machine ([ADR-0184](0184-the-runs-off-the-developer-machine.md)).
- **A survivor found before the protocol's commit** may be killed under `src/`. **One found after it is killed by a new test** where one can catch it. A change under `src/` then is a change after the calibration, ADR-0191's: it is calibrated again, or the reproduction runs every test.
- Held by description, not by a check: no event marks a protocol's commit.

### The schedule

- **`17 20 * * 3`**: Wednesday 20:17 UTC, Thursday 04:17 in Taiwan. Off the hour, on the day the maintainers dispatch least, at their quietest hours.
- How late it begins there is a Target until four runs read it.
- Its sweep is still recorded by the first ADR that merges after it ends ([ADR-0158](0158-the-sweep-off-the-next-decisions-path.md)).

### An exploration kept as a tag

- **When its note merges, one push makes the tag and deletes the branch**: `git push --atomic origin refs/remotes/origin/explore/NNN-<slug>:refs/tags/explore/NNN-<slug> :refs/heads/explore/NNN-<slug>`. The session that merges the note makes it.
- **A tag ruleset on `refs/tags/explore/*`** refuses a deletion and an update. It is the repository's setting, made once after this record merges, and the five branches are converted then.
- **A note's `**Branch**:` line keeps its name**, now a tag's, and every commit its runs name is reachable from it. `check-explorations.mjs` does not change.
- An exploration taken up again branches from its tag.

### F-85, and the sweep

- **F-85**, recorded and resolved by the limits of 2026-10-27. An artifact uploaded before that keeps its 14 days, so a run link older than two weeks reaches nothing, and the committed tables are the evidence.
- **The schedule's run of 2026-10-26** is recorded in [ADR-0221](0221-the-project-renamed-menteion.md). None has ended since.

## Consequences

- Good: a push that touches no file the Rust jobs read, after a green gate, costs the documentation job alone.
- Good: a round learns its survivors during the calibration, while a test can still kill them before the reproduction.
- Good: the schedule leaves the hour GitHub delays most, and the days the rounds dispatch most.
- Good: a finished exploration is a tag nobody can delete or move, and the branches list only live work.
- Bad: three mutation shards pay the baseline three times, about 54 minutes of machine time where it was 18.
- Bad: a round's gate, opened with its protocol, shares the account's places with the calibration's eighteen shards and the arms.
- Bad: a reused gate ran on the runner image of its day. The toolchain is pinned, the image is not.
- Bad: a gate that failed or was cancelled keeps nothing, so the next push runs all four jobs again.
- Neutral: no code, test or pin of the engine changes.

## Amended

- **ADR-0193**: an exploration's branch is kept while it runs, and becomes a tag of its name when its note merges.
- **ADR-0211**: the Rust jobs also skip two trees a green gate already tested.
- Each is accepted and is not edited.

## Alternatives considered and why rejected

- **Option 1(b), the commits since the last green head**: the jobs test the merge with the base, which changes when `main` moves with no commit on the branch.
- **Option 1(c)**: brief 082's 51 minutes.
- **Option 2(b), two shards**: about 35 minutes, past the Rust job's 24 to 31.
- **Option 2(c)**: 51 minutes on 83 mutants, and more on a round that builds.
- **Option 3(b), at review**: brief 082's two hours.
- **Option 4(b), Friday**: Saturday, in Taiwan, had 10 dispatches to Thursday's 3.
- **Option 4(c), Monday off the hour**: Monday had 34.
- **Option 5(b), a branch ruleset**: a branch names work that moves. A tag names a commit that does not.
- **Option 5(c)**: nothing keeps a branch.

## Confirmation

- `npm run spec` runs `gate:fingerprint:test` and the directives on the fingerprint, the shards and the schedule.
- This pull request's first gate runs the four jobs and keeps its fingerprint; its mutation shards, on a diff with no Rust line, are each dealt none. Its last commit changes this record alone, and its gate reuses the first.
- After the merge: the ruleset (`gh api repos/Menteion/Menteion/rulesets`) and the five tags (`git ls-remote --tags origin 'explore/*'`).
