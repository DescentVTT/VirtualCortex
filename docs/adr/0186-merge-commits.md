---
status: proposed
date: 2026-10-18
amends: ADR-0150
decision-makers: VirtualCortex maintainers
---

# ADR-0186: Merge commits — `main` took every pull request by a rebase, which gives each commit a new hash and drops its signature, so the hashes an ADR cites and the commits a dispatch ran on are not `main`'s; `main` now takes a pull request by a merge commit, the ruleset allows that method alone and no longer requires a linear history, and a branch is brought up to date by merging `main` into it, never by a rebase once a run has been made on it or its pull request is open; signed commits follow

## Context and Problem Statement

**How `main` takes a pull request today.**
- `main` holds 473 commits, none of them a merge commit and none signed.
- The ruleset "Main Protect" requires a linear history and allows the merge, squash and rebase methods. Every round has merged by a rebase.

**What a rebase does to a round's evidence.**
- **The hashes change.** A rebase merge writes each commit again on top of `main`. [ADR-0150](0150-a-round-waits-for-what-it-checks.md) therefore had each ADR cite *"the hashes its branch holds"*, and relied on GitHub keeping a merged pull request's commits under `refs/pull/<n>/head`. That ref holds the head the pull request had when it merged. It does not hold a commit that a rebase of the branch, before the pull request or during it, left behind.
- **ADR-0184 puts this on the evidence path.** A round's tables now come from dispatches, and each dispatch runs on a commit. "The protocol was committed before the arms ran" is read from that commit's place in the history. ADR-0184's own trials ran on `1378111` and `e56e8b5`. Rebasing its branch onto `main` after brief 073 would leave both commits in no branch, and the runs would name commits that history no longer holds.
- **The signature goes.** GitHub's documentation says that with a rebase merge *"the commits in the head branch are added to the base branch without commit signature verification"*. GitHub cannot sign the rewritten commit, since it does not hold the committer's key. The maintainers chose on 2026-10-17 to sign commits once the merge method allows it.

**What a merge commit does.** The branch's commits enter `main` with their hashes and signatures. One commit on `main` records the merge. `git log --first-parent main` reads one entry per pull request, which is the linear view a rebase gave.

**Applied first.** On 2026-10-18 the maintainers decided this and had the ruleset changed at once, before this ADR merged. The ruleset no longer requires a linear history, and it allows the merge method alone. Brief 073's pull request and those of ADR-0184, ADR-0185 and this ADR merge that way. This ADR records the decision; the change to the ruleset is already made.

## Decision Drivers

- **A run's commit stays in the history** that the round's ADR points into (ADR-0184).
- **An ADR's hashes are `main`'s.** ADR-0150's citing of branch hashes was a workaround for the rebase.
- **Signed commits are possible**, which a rebase merge rules out.
- **A structural boundary beats a reviewed one** (principle 5): the merge method is the ruleset's, not a habit.

## Considered Options

1. (a) merge commits only; (b) squash; (c) rebase, as now.
2. Bringing a branch up to date: (a) by merging `main` into it; (b) by a rebase.

## Decision Outcome

**Options 1(a) and 2(a).**

- **`main` takes a pull request by a merge commit** (`gh pr merge <n> --merge`). The ruleset allows that method alone, and the merge commit's message is GitHub's default.
- **The ruleset no longer requires a linear history.** It still forbids deleting `main` and force-pushing to it. It still requires a pull request and the three named checks, `Rust (check, test, fmt, clippy)`, `Rust (minimum supported version)` and `Documentation (spec-guard, spec-graph)`. The bypass list is unchanged.
- **A branch is brought up to date by merging `main` into it**, or by the app's sync, which merges. **It is not rebased, and not force-pushed, once a run has been made on any of its commits or its pull request is open.** No ruleset holds a branch other than `main`, so this is a practice, and the reviewer checks it: every commit an ADR cites, and every commit a dispatch it cites ran on, is an ancestor of the pull request's head (`git merge-base --is-ancestor`).
- **ADR-0150's item 5 becomes**: the ADR cites the round's commits by their hashes, which are `main`'s once it merges. No second pull request follows.
- **Before a merge, the branch holds `main`.** Its merge state reads `CLEAN`, so the merge commit's tree is the tree the gate checked. The ruleset does not require it, since its strict status-check policy is off, and the reviewer checks it.
- **Signed commits follow, as their own step.**
  - A single SSH signing key, registered with GitHub, signs every commit made on the developer machine.
  - The ruleset's required signatures are turned on once commits are signed.
  - The key is the maintainer's to make and register. Until then, commits stay unsigned.
  - A merge commit that GitHub makes is GitHub's.

## Consequences

- Good: the commits an ADR cites, and the commits its dispatches ran on, are in `main`'s history.
- Good: a signature made on a branch survives the merge.
- Good: the pull request a commit came in by is one `--first-parent` step away.
- Bad: `main`'s history is no longer a single line; reading it as one takes `--first-parent`.
- Bad: a branch's every commit lands on `main` as written, a fixup included. A round already writes no commit it would squash.
- Bad: "no rebase once a run has been made" binds every branch, and no ruleset holds a branch other than `main`; the reviewer holds it.
- Neutral: the repository's own settings still allow squash and rebase; the ruleset is what refuses them.

## Alternatives considered and why rejected

- **Option 1(b), squash**: one commit per pull request on `main`, and every hash a round cites gone.
- **Option 1(c), rebase**: what ADR-0150 worked around, and it rules out signatures.
- **Option 2(b), rebasing a branch**: it rewrites the commits a dispatch ran on.

## Confirmation

- The ruleset "Main Protect" (id 22666663), as read back after the change on 2026-10-18:
  - rules `deletion`, `non_fast_forward`, `pull_request` with `allowed_merge_methods: ["merge"]`, and `required_status_checks` with the three checks;
  - no `required_linear_history`;
  - enforcement `active`, the bypass list as it was.
- `briefs/README.md`'s "Evidencing a round", item 5; `CLAUDE.md`'s and `CONTRIBUTING.md`'s workflow; this decision's rows in §9 and the index.
- The first merge commit on `main` is the merge of the first pull request after this change.
