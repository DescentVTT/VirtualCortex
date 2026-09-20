---
status: accepted
date: 2026-09-20
amends: ADR-0008
decision-makers: VirtualCortex maintainers
---

# ADR-0064: The documentation gate is one command and the whitepaper's version is checked — CI runs `npm run spec` rather than a list of steps that drifted from it, the document's two version declarations must agree, and a change to the document moves its version

## Context and Problem Statement

Two defects of the same shape, four days apart, neither found by a check.

**The version.** The whitepaper declares its version and its date twice: in the front matter that `spec-guard` reads and in the Document control table that a reader reads. Brief 027's round bumped the front matter to 4.15.0 / 2026-09-19 and left the table at 4.14.0 / 2026-09-14. The document disagreed with itself for a day and a half and was corrected by brief 028's round, which noticed it while doing something else; nothing in the tree would have said so. Nor is there any rule about when the version moves at all: [ADR-0008](0008-documentation-governance.md) governs the whitepaper and says nothing about its version, so a pull request may change a table, add a finding or restate a claim under a version that has not moved since the week before — which makes the number a decoration rather than a reading.

**The gate.** `npm run spec` is the command CONTRIBUTING.md's verification list names and CLAUDE.md's commands repeat, and the `specs` job in CI ran a hand-written list of the same checks, step by step. On 2026-09-20 brief 028's round added `scripts/check-decisions.mjs` after F-41, wired it into `npm run spec` as `spec:decisions` and `spec:decisions:test`, and named it in both documents — and did not add it to the workflow. For two days the tree carried a check that three documents called blocking and that ran on a developer machine only. That is CLAUDE.md's own warning back verbatim: "a check here and not in CI is a gate nobody enforces". It is recorded as **F-44**, and the disagreement it left is one the tree cannot see, because a list of YAML steps and a list of npm scripts have no relation a script can read.

## Decision Drivers

- A structural boundary beats a reviewed one (CLAUDE.md principle 5); both defects are reviews that did not happen.
- A list kept in two files drifts. The fix is one list, not a rule about keeping two in step.
- The repository wins over the document (principle 1): a version that says 4.15.0 while the tree's table says 4.14.0 is the document disagreeing with itself, which is worse than either.
- A new rule names where it is enforced, or it is written as a description and not as a requirement.

## Considered Options

1. **Leave both as review rules**, and record the version disagreement as a finding with a sentence. This is what F-40's lesson was, and F-41 is F-40 happening again; the sentence did not prevent the repeat.
2. **A check on the two declarations only.** It closes the disagreement and leaves the version free to stand still while the document changes under it.
3. **A check on the declarations, plus a version that must move on a pull request that changes the document, plus one CI step that is `npm run spec`.** Every rule enforced where it is stated.
4. **Generate the workflow's steps from `package.json`.** The drift is real, but generated CI is a build step to maintain for a list of five lines.

## Decision Outcome

**Option 3**, in three parts.

**The gate is one step.** The `specs` job runs `npm run spec` and nothing else, so the list of documentation checks lives in `package.json` alone and a check added there is in CI by construction. Each check already prints its own name, file and line, which is what a per-step name bought. `scripts/check-scripts.test.mjs` (`npm run spec:scripts:test`) holds the list itself: every `spec:*` script and every `*:test` script must be run by `spec`, `spec` must run nothing but scripts the file defines, and a script that takes an argument — `mutants:timeouts`, which needs a sweep's output directory — must not be in it.

**The two declarations must agree.** `scripts/check-version.mjs` (`npm run spec:version`) reads the front matter's `version:` and `date:` and the Document control table's `| Version |` and `| Date |` rows and fails with both line numbers when they differ, or when the version is not `MAJOR.MINOR.PATCH`. Against the whitepaper as it stood at `bbb831d` it fails with exactly the two lines that were wrong.

**A change to the document moves its version.** On a pull request the same script is run again with the base's copy of the whitepaper, written out with `git show`; if the file differs and the version has not moved forward, the check fails. The scale, now written down in CONTRIBUTING.md:

- **patch** — a correction that changes no claim: a typo, a link, formatting, a transcription that was already true;
- **minor** — new or changed content: a finding, a row, a section, a status, a number;
- **major** — a restructuring that supersedes, as 3.0.0 superseded Specification 2.8.0 and 4.0.0 superseded 3.0.0.

The date moves with the version. `spec-guard`'s own assertions are unaffected: they read the document's claims, not its header.

### Consequences

- Good: the version becomes a reading again — the document says which of it you have, in both places, and it moves whenever the content does.
- Good: a check cannot exist locally and not in CI. The class of defect F-44 names is closed by construction rather than by a fourth document saying to remember.
- Good: two open pull requests that both bump the whitepaper collide on the version line, which is a conflict a human resolves by reading both, rather than a silent second merge under the same number.
- Neutral: the version will move faster than before, several times a week. SemVer on a living document is a statement about the document, not a release; a patch bump for a typo costs a line.
- Bad: the per-check CI step names are gone from the checks tab, so a failure reads as "the documentation gate" until the log is opened. The log's first line names the check.
- Bad: the bump check needs the base commit, so the `specs` job checks out the full history (`fetch-depth: 0`) where it took one commit.

## Alternatives considered and why rejected

- **Declaring the version once.** The front matter is what tools read and the table is what a reader reads; dropping either loses a real audience, and a generated table is a build step for one row.
- **Bumping only on "substantive" changes.** A rule whose predicate is a judgement is a rule that is argued about in review; "the file changed" is a predicate git computes.
- **A `version` that tracks the ADR number or the date.** Both were considered and both encode something the version is not: a document can change three times in a day and can go a week without changing while ADRs accumulate.
- **Keeping the per-check CI steps and adding `spec:decisions` to them.** It fixes the instance and leaves the mechanism: the next check added to `npm run spec` drifts the same way.

## Confirmation

- `scripts/check-version.mjs` and `scripts/check-version.test.mjs` (thirteen cases); `scripts/check-scripts.test.mjs` (four cases); both inside `npm run spec`.
- `.github/workflows/ci.yml`: the `specs` job's two steps, and its checkout's `fetch-depth: 0`.
- `package.json`: `spec` runs `spec:guard`, `spec:graph`, `spec:briefs`, `spec:briefs:test`, `spec:decisions`, `spec:decisions:test`, `spec:version`, `spec:version:test`, `spec:scripts:test`, `spec:deps` and `mutants:timeouts:test`.
- CONTRIBUTING.md's Documentation rules: the version bullet and the scale.
- Whitepaper §11: F-43 and F-44; Appendix B's V-5 row.
