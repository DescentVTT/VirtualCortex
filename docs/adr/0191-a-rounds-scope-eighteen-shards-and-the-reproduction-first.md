---
status: accepted
date: 2026-10-19
amends: ADR-0150
decision-makers: VirtualCortex maintainers
---

# ADR-0191: A round's scope, eighteen shards, and the reproduction first — from brief 075 a brief declares what its round may write and what it may not, and `spec-brief archive --strict` refuses a round that wrote outside the one or into the other, a check holding that the scope is declared and that its protected patterns never shrink; the weekly whole-domain tests are dealt to eighteen shards, where a dispatch's slowest read 111 minutes of 180 at twelve; a round dispatches its reproduction as soon as its tables are pinned and writes its readings while it runs; a change after the calibration is calibrated again or shown to change no run

## Context and Problem Statement

Brief 074 was the first round under [ADR-0184](0184-the-runs-off-the-developer-machine.md), [ADR-0185](0185-the-spec-tools-upgraded-and-spec-brief.md) and [ADR-0186](0186-merge-commits.md) ([ADR-0190](0190-a-target-the-engine-reads-measured.md)). It ran from the brief pasted at 09:13Z to the pull request ready at about 15:40Z on 2026-10-18. The maintainers asked on 2026-10-18 for four changes to the process. This ADR takes three of them; the fourth, an exploration lane, is its own decision.

**What a round may write is checked by nobody.**
- A brief says what its round may not do in `## Not empowered`, and the reviewing session reads the diff against it.
- spec-brief reads two fields of a brief's front matter, `affectedFiles`, the scope its round may write, and `protectedFiles`, what it may not. Its archive refuses a round that changed a protected file, and under `--strict` one that changed a file outside the scope, measured against a base branch. The briefs here declare neither, so the archive checks nothing.
- spec-harness's own [ADR-0005](https://github.com/DescentVTT/spec-harness/blob/main/docs/adr/0005-a-guard-is-a-guardrail.md) names the archive as the gate, since a tool's hook cannot see a shell's write.
- **Tried** on a scratch branch never pushed, with a scratch brief 075 measured from `origin/main`:
  - a change to `.github/workflows/ci.yml`, protected, refused as `protected-file`;
  - a change to `README.md`, outside the scope, refused as `out-of-scope`;
  - a change to a test inside the scope, passed;
  - `README.md` added to `affectedFiles`, no longer refused;
  - the base read from `.spec-brief.json`'s `archiving.base`.
- What spec-brief does not hold:
  - that a brief declares a scope at all, since a brief with none passes the check over nothing;
  - that a round does not shrink its brief's protected patterns before archiving, since the refusal then has nothing to refuse.

**The weekly shards.**
- Brief 074's reproduction, dispatch 39131009704, dealt the 146 whole-domain tests to twelve shards. Shard 9's job took 111.3 minutes, 62 per cent of its 180, and shard 10's 108.1, 60 per cent. Neither held a test of that round: the runners' spread put them there (ADR-0190).
- By [ADR-0179](0179-the-bounds-rule-restated.md)'s rule, *"when a shard of a dispatch reads past 60 per cent of it, 108 minutes of the job's time, under a deal made from a table that knew its tests … the bound or the deal is then the workflow's decision."*
- The table after that run, 146 tests and 114 511 s, plans 9 544 s for each of twelve shards, and 6 365 s for each of eighteen. The longest test, `the_address_drawn_on_seed_22_from_the_assignment_at_2048_units_exhaustive`, reads 3 674 s, a floor no deal moves.
- A round runs the whole-domain tests twice, as its calibration and as its reproduction, so the slowest shard's job bounds it twice.
- The account runs twenty jobs at once. The sweep's thirteen jobs wait for the shards ([ADR-0166](0166-the-sweep-after-the-shards.md)), so eighteen shards do not contend with them.

**The reproduction's order.**
- Brief 074's plan pinned the arms' tables, wrote ADR-0190's readings and documents, and then dispatched the reproduction. At the maintainers' word, its session dispatched first and wrote while the reproduction ran.
- A reproduction needs the code and the pinned numbers, not the documents. `merge_round.sh`, the reviewing session's gate, already refuses a pull request whose code changed after the reproduction's commit.

**A change after the calibration.**
- Brief 074's calibration ran on `ddc7f2c`. Then `9f97fab` changed `src/` before the derivations ran: the image's read sum joined from its bytes, to remove the mutation gate's one survivor.
- ADR-0189 and ADR-0190 say it changes no run, and the reproduction, on the arms' commit, reproduced every pinned number.
- Nothing said what a round does there, and ADR-0184 says the calibration is *"made on the commit the round's runs will be made from"*.

**A sweep running.** The sweep of dispatch 39131009704 had not ended when this was written. The first ADR that merges after it ends records it ([ADR-0158](0158-the-sweep-off-the-next-decisions-path.md)).

## Decision Drivers

- **A structural boundary beats a reviewed one** (principle 5): what a round may write is refused at its archive, whatever wrote it.
- **A check over nothing is not a pass**: a brief with no scope fails, as an archive with no brief does.
- **The rule ADR-0179 wrote** for a shard past 60 per cent.
- **A round waits for what it checks, and no longer** ([ADR-0150](0150-a-round-waits-for-what-it-checks.md)).
- **What a calibration showed holds of the commit it ran on.**

## Considered Options

1. **The scope**:
   - (a) declared from brief 075, archived with `--strict` against `origin/main`, with a check that it is declared and that protection never shrinks;
   - (b) declared, with the archive not strict;
   - (c) not declared, as now.
2. **The shards**: (a) eighteen, the bound 180 minutes; (b) twelve, with the bound raised to 210; (c) twelve, as now.
3. **The reproduction**: (a) dispatched as soon as the tables are pinned; (b) after the readings, as brief 074 planned.
4. **A change after the calibration**: (a) calibrated again, or shown in the ADR to change no run and checked by the reproduction; (b) always calibrated again; (c) nothing said.

## Decision Outcome

**Options 1(a), 2(a), 3(a) and 4(a).**

### A round's scope

- **From brief 075, a brief declares `affectedFiles` and `protectedFiles` in its front matter**, each a list of spec-brief's globs with at least one pattern. The brief's writer sets them. A test-only round protects `crates/**/src/**` and `runtime/cortex-runtime/src/**`. Every round protects `.github/workflows/**`, `scripts/arm-run.sh`, `scripts/exhaustive-shard.sh`, `scripts/exhaustive-costs.mjs` and `rust-toolchain.toml` unless its brief is for them.
- **`.spec-brief.json` measures from `origin/main`** (`archiving.base`). A round archives with `npx --no-install @descent-vtt/spec-brief archive <NNN> --strict --pr <n> --summary "..."`. The archive refuses a protected file the round changed and, being strict, a file outside the scope.
- **`scripts/check-scope.mjs`** (`npm run spec:scope`, with its tests in `spec:scope:test`) fails:
  - a brief from 075, live or archived, that declares either list empty or not at all;
  - a brief whose protected patterns are fewer than its live copy's on the base. It prints, without failing, each pattern by which a brief widened its `affectedFiles` since the base, for the review.
  - A base git cannot read fails the check. The base is `origin/main`, or `SPEC_SCOPE_BASE`.
- **When a file the round must change is outside its scope**:
  1. **Within the brief's mission and its architectural empowerment**: the round widens its brief's `affectedFiles` in a commit of its own, the file and the reason in its message, and its ADR says so. The reviewer reads each widening, which the check prints.
  2. **Protected, or past the empowerment**: the round does not widen. It stops at that point and says what it needs and why. The next decision takes it.
  3. **Allowed by the maintainers in the reviewing session**: the allowance and its reason are written into the round's ADR, and the round then widens. No signed ruling stands in for it: the maintainers do not use a key no process can use.
- Briefs 065 and the archived ones are not edited.

### Eighteen shards

- **The weekly job deals the whole-domain tests to eighteen shards**, by cost as [ADR-0092](0092-the-shards-dealt-by-cost.md) deals them, each running its tests two at a time.
- The bound stays 180 minutes, and ADR-0179's rule reads it as before.
- At the table after brief 074, a shard plans 6 365 s summed, about 3 200 s of wall time where the longest test's 3 674 s is the floor.
- The runtime's sweep stays in twelve shards ([ADR-0158](0158-the-sweep-off-the-next-decisions-path.md)).

### The reproduction first

- **A round dispatches its reproduction as soon as its tables are pinned and pushed.** It writes its readings, its documents and its archive while the reproduction runs.
- **After the reproduction's commit, no file under `runtime/`, `crates/` or `benches/` and no `Cargo.toml` or `Cargo.lock` changes.** `merge_round.sh` refuses a pull request where one did.

### A change after the calibration

- **A change to a file under `src/`, or to a test the calibration ran, made after the calibration's commit and before a run**: the round dispatches the calibration again on the new commit, or its ADR says why the change changes no run. The reproduction is then the check.
- **Amending ADR-0150 and ADR-0184**: `briefs/README.md`'s "Evidencing a round" and "Archiving one" say this. Every other item of theirs stands.

## Consequences

- Good: a round that wrote what its brief did not allow cannot archive, however the file was written.
- Good: a brief that protects something cannot unprotect it within its own round.
- Good: a whole-domain pass's slowest shard falls from about 90 to 110 minutes toward the longest test's hour, twice a round.
- Good: a round no longer waits an hour for documents before its reproduction starts.
- Bad: a brief's writer must name its scope. A scope too narrow costs the round a widening commit, which the reviewer reads.
- Bad: eighteen shards of twenty concurrent jobs leave two for anything else running then. An arms dispatch beside a reproduction waits.
- Bad: the scope check reads `origin/main`, so a stale local copy compares with an older base. CI's checkout fetches it fresh.
- Neutral: the sweep's shards are unchanged.

## Alternatives considered and why rejected

- **Option 1(b), not strict**: a file outside the scope would be a warning, read by the reviewer as now.
- **Option 1(c)**: the archive spec-brief gives would check nothing.
- **Option 2(b), twelve with 210 minutes**: the wait stays at its length and only the bound grows.
- **Option 2(c)**: ADR-0179's rule asks the workflow to decide, and this is the decision.
- **Option 3(b)**: an hour of the round spent waiting on documents the reproduction does not read.
- **Option 4(b), always calibrated again**: ninety minutes more for a change the reproduction checks anyway, where the ADR can say why the change changes nothing.
- **Option 4(c)**: ADR-0184's sentence would be broken by the next round as by brief 074's.

## Confirmation

- `.spec-brief.json` (`archiving.base`); `scripts/check-scope.mjs` and its ten tests; `package.json` (`spec:scope`, `spec:scope:test`).
- `.github/workflows/ci.yml`: eighteen shards, their names and the shard script's count.
- `briefs/README.md`: the brief's front matter, "Archiving one" and "Evidencing a round"; `CLAUDE.md`, `CONTRIBUTING.md` and Appendix B: the shards and `spec:scope`.
- This decision's rows in §9 and the index.
- The scope was tried on a scratch branch, as the Context says, and the deal at eighteen on the listing dispatch 39131009704 kept.
