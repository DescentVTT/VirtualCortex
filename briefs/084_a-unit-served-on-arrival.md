---
status: proposed
date: 2026-10-28
affectedFiles: [crates/menteion-core/src/**, runtime/menteion-runtime/src/**, runtime/menteion-runtime/tests/**, testkit/**, benches/**, scripts/exhaustive-costs.tsv, docs/**, briefs/**, CHANGELOG.md, README.md, CLAUDE.md]
protectedFiles: [crates/menteion-connectome/**, docs/explorations/0*.md, docs/benchmarks/results/**, .github/workflows/**, .cargo/**, Cargo.toml, Cargo.lock, rust-toolchain.toml, clippy.toml, .spec-brief.json, scripts/arm-run.sh, scripts/exhaustive-shard.sh, scripts/exhaustive-costs.mjs, scripts/check-additive.mjs, scripts/check-scope.mjs, scripts/adr-index.mjs]
---

# Brief 084: A unit served on arrival

<!-- @assert-absence target="crates/menteion-core/src/dynamics/membrane.rs" symbol="quiet_cannot_fire" reason="brief 084 precondition: main serves every awake unit every tick; the round that builds service on arrival archives this brief in the same pull request" -->

## Mission

**This brief builds [ADR-0224](../docs/adr/0224-a-unit-served-on-arrival.md)'s service on arrival and runs every test once.**
The executor serves 99.99 % of the reference network's units on every tick ([ADR-0097](../docs/adr/0097-the-active-set-measured.md)).
[Exploration 008](../docs/explorations/008-a-unit-served-on-arrival.md) found a condition under which a unit cannot fire with no input, and an exact catch-up for its quiet ticks.

When the round is done, the tree holds:
- the condition and the catch-up in `menteion-core`, each held by property tests over `testkit/prop.rs`'s lattice;
- an executor that takes a unit off the schedule under the condition and brings it up to date bit for bit when a message reaches it or its state is read;
- per-tick service kept as the tests' reference, and a differential test holding the two to each other;
- one dispatch at `scope=exhaustive` with every test on the round's last code commit, every shard green and every pinned number held;
- the two services timed on AArch64 in one process, and x86-64 read from the regenerated cost table.

It writes a built ADR and rewrites `docs/STATUS.md` ([briefs/README.md](README.md#a-rounds-documents)).
The scope is the front matter's ([ADR-0191](../docs/adr/0191-a-rounds-scope-eighteen-shards-and-the-reproduction-first.md)).

---

## Standing directives

- **Latest ≠ Newest** (whitepaper §2.1, `CLAUDE.md` principle 4). The round depends on nothing new: no crate, no tool, no version of either, no `target-cpu` and no target feature.
- **No number moves.** No rule, record, constant, test assertion or image byte changes, and the format stays 26. The service changes when the work is done, never what it computes. A pinned number that moves stops the round as a finding.
- **Per-tick service is the model** ([ADR-0098](../docs/adr/0098-the-integration-model.md), amended by ADR-0224). The catch-up is the quiet tick's own rule, `integrate(0, 0, t)`, or a form held to it field by field.
- **No `unsafe` beyond ADR-0023's**, unless the round's ADR restates [ADR-0023](../docs/adr/0023-executor.md)'s invariant for the write and names its test.
- **The whole-domain run is one dispatch** ([ADR-0215](../docs/adr/0215-the-calibration-a-change-needs.md), [ADR-0211](../docs/adr/0211-the-round-shortened.md)): `scope=exhaustive` with every test, on the round's last commit that changes a file under `crates/`, `runtime/`, `benches/` or `testkit/`. After that commit, none of them changes. The alternation runs at `scope=arms` on the same commit. No reproduction and no sweep. No whole-domain test runs on the developer machine.
- **The pull request opens as a draft with the round's first push** ([ADR-0223](../docs/adr/0223-the-gate-shortened-and-explorations-kept-as-tags.md)), and leaves draft with the commit that asks for the merge, which sets the round's ADR to `accepted`.
- **The branch is named `claude/brief-084-…`**, never rebased or force-pushed once a dispatch has run on it.
- No `f32`/`f64`; every loop ends by construction ([ADR-0062](../docs/adr/0062-the-first-complete-sweeps-list.md)); the mutation gate catches every mutant in the lines the change touches ([ADR-0030](../docs/adr/0030-verification-governance.md)); a new rule carries a test over the lattice.
- **The documents are ADR-0197's**: the new ADR in its shape, the tables written by `node scripts/adr-index.mjs`, new text within `npm run spec:shape`'s limits.
- **No shard of the weekly job passes 60 per cent of its bound**, 240 minutes ([ADR-0179](../docs/adr/0179-the-bounds-rule-restated.md)). One that does is reported with its job's minutes.
- **GitHub without gh.** The developer machine has no gh login. Dispatches, the pull request, logs and artifacts go through the REST API with the repository's own token (`~/.claude/menteion-tools/ghapi.sh` and its README); `git push` uses the same token. The token is never printed or put on a command line.
- **The maintainer is notified** as `briefs/README.md` says: when a question only they can answer stops the round, and when the pull request is ready for review.
- Conventional Commits with a real body; never commit on `main`.

## Context

Re-derived on 2026-10-28 against `main` at `e33e0ed`. Line numbers move; the symbols are what to re-derive.

1. **The membrane** (`crates/menteion-core/src/dynamics/membrane.rs`): `integrate`, `integrate_slow`, `leak`, and the constants `SOMA_LEAK_SHIFT` 11, `BASAL_LEAK_SHIFT` 9, `APICAL_LEAK_SHIFT` 10, `COUPLING_SHIFT` 4, `PLATEAU_COUPLING_SHIFT` 2, `THRESHOLD_DECAY_SHIFT` 12. Every decay takes at least one LSB.
2. **Exploration 008's code**, on the tag `explore/008-a-unit-served-on-arrival` at `e547c45`, written for an exploration and not reviewed:
   - `quiet_cannot_fire` and `quiet_catch_up` in `membrane.rs`, with their property tests;
   - the executor's mode `MENTEION_EXPLORE_008`, the asleep map, the catch-up at a turn and at a read, in `executor.rs`;
   - `runtime/menteion-runtime/tests/arrival.rs`, the differential test at 256 units;
   - the timed workloads in `tests/inhibition.rs`, `scripts/explore-008/`.
3. **The executor** (`runtime/menteion-runtime/src/executor.rs`):
   - `Executor::turns`, "Turns served by every worker so far (ADR-0097)";
   - `Executor::units`, under `&self`, an `unsafe` slice with ADR-0023's comment that between ticks no worker holds a reference into an arena;
   - `units_mut` and `arenas_mut` under `&mut self`;
   - `Executor::sweep`, which reads the unit arena directly, not through `units`;
   - the schedule each worker serves its range from ([ADR-0102](../docs/adr/0102-the-sweep-timed-with-no-census.md)).
4. **What reads the turn count.** ADR-0097's tests (`runtime/menteion-runtime/tests/active.rs`) read `exec.turns()` tick by tick as the units per-tick service serves: each tick's turns against the units whose gate is scheduled, the gate test's one turn or none a tick, and per-tick rows pinned in `NETWORK_ROWS` and `CONTROL_ROWS`. `tests/contention.rs` holds the workers' summed turns to `exec.turns()`. Under service on arrival a tick serves about 1 % of those units.
5. **What exploration 008 left open**: the arena brought up to date under `&self`; a reward's write of the critic's weights through `units_mut` putting every asleep unit back on the schedule, although no quiet tick reads `value_weight`; where AArch64's time goes, 0.771 against x86-64's 0.581.
6. **The cost table** (`scripts/exhaustive-costs.tsv`): 229 444 s for the weekly tests, 95.6 % of it `inhibition`'s. Its shards run on x86-64.
7. **The process** ([briefs/README.md](README.md#evidencing-a-round)): `npm run round:additive` reads the change; the cost table is regenerated from the dispatch's artifacts with `--keep`; `spec-brief archive <NNN> --strict` is measured from `origin/main`.
8. **The schedule** runs on Wednesday at 20:17 UTC and takes most of the organisation's places for hours. A dispatch made then waits.

## Deliverables

- [x] **The service, in a new ADR at the next free number (`ls docs/adr`), kind `built`.**
  - The condition and the catch-up in `menteion-core`, with their property tests over the lattice and a driven walk.
  - The executor: a unit off the schedule under the condition, back on it at a message or an activation, brought up to date at its next turn and before any read of the arena, `Executor::sweep`'s included, and brought up to date and put back on the schedule before any write.
  - A unit marked `FLAG_SLOW` under the image's slow current never sleeps.
  - Per-tick service kept as a construction option for the tests, in neither `Config` nor the image.
  - The differential test holding the two services to each other at 256 units, under ADR-0044's drive and under ADR-0097's drive 256 times sparser: after every tick for 2 048 ticks, then every 4 999 ticks over 32 768.
  - `Executor::turns` counting the turns served, as each worker's count does, and a second count of the steps caught up at a turn and at a read; over a run, once a read has brought every asleep unit up to date, the two together held to per-tick service's turns by the differential test.
  - ADR-0097's tests run on the per-tick reference, their assertions unchanged.
  - How the arena is brought up to date between ticks: under `&mut self`, or under ADR-0023's invariant restated with its test.
- [x] **The dispatch at `scope=exhaustive`**, every test, on the round's last code commit, every shard green: every pinned number held. The cost table regenerated from its artifacts, and its seconds read beside the table before it, test by test.
- [x] **The alternation on AArch64**, at `scope=arms`: exploration 008's 16-block workload, and the reference network at 1 024 units under ADR-0097's drive 256 times sparser, each under both services on one runner, three rounds, inside one test since an arms job runs one. The test's name holds neither `exhaustive`, which would put it in the weekly job, nor `_explore`, the exploration lane's ([ADR-0193](../docs/adr/0193-an-exploration-lane.md)).
- [x] **The documents, in the same pull request.**
  - The whitepaper's sentences on per-tick service, the active set and the integration model, saying per-tick service carried out on arrival; its version in both declarations, with its date; the tables written by `node scripts/adr-index.mjs`.
  - `docs/STATUS.md` rewritten whole.
  - `CHANGELOG.md`, one entry of at most 400 characters.
- [x] **The brief archived** with `spec-brief archive 084 --strict`, every deliverable dispositioned. The round's ADR is set to `accepted` by the commit that asks for the merge, and the pull request is marked ready for review with it.

## Not empowered

- **The round's scope** is the front matter's. It writes only inside `affectedFiles`, and never into `protectedFiles`. A file it must change outside the scope is handled as `briefs/README.md` says.
- **No number moves**: no rule, constant, record, image byte or test assertion. A test that reads per-tick service's active set runs on the reference instead.
- **No setting of the service** in `Config`, the image, an environment variable or a feature flag. The reference is the tests'.
- No dependency, no tool version, no toolchain, no MSRV, no edition and no target feature changes. No change to the workflow or the run scripts.
- No whole-domain test runs on the developer machine, and no rebase of the branch once a dispatch has run on it.
- No accepted ADR and no archived brief changes.

## Architectural empowerment

The executing session may replace any instruction above with a better decision, recorded in its ADR. That covers:
- the catch-up: `quiet_catch_up` as exploration 008 wrote it, a faster exact form held to `integrate(0, 0, t)` over the lattice, or `integrate` itself where a form fails a property, saying which and why;
- how the arena comes up to date between ticks, within the standing directive on `unsafe`;
- a reward's write of the critic's weights that leaves asleep units asleep, held by the differential test;
- the reference's form, the timed test's name and how the alternation runs in one process;
- the order of the commits, so long as each builds and the dispatch runs on the last code commit;
- a file the round must change and its scope does not hold, within this brief's mission, widened as `briefs/README.md` says.

It may not:
- move a pinned number, or change a rule, a record, the image or the format;
- write into a protected file, or shrink `protectedFiles`;
- run a whole-domain test on the developer machine;
- reach the standing directives, the whitepaper's invariants or the constraints in `CLAUDE.md`.

If the dispatch or the differential test finds two bytes apart, the round stops there: the finding is written, and nothing is pinned anew.

## Verification

Before each push, on the developer machine:

```bash
cargo check --workspace --all-targets --locked
cargo test --workspace --locked
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --locked -- -D warnings
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --locked
npm ci
npm run spec
```

The pull request's gate runs those and the rest: the release tests, the benchmarks once, the MSRV, AArch64 and the mutation gate on the changed lines. Every check passes on the last commit.

By dispatch, never on the developer machine. Each `ci.yml` dispatch is `POST /repos/Menteion/Menteion/actions/workflows/ci.yml/dispatches` with a JSON body, through `ghapi.sh`:

```bash
npm run round:additive -- origin/main <the last code commit>
# ci.yml on this round's branch, inputs {"scope":"exhaustive"}: every test
# ci.yml on this round's branch, inputs {"scope":"arms","tests":"<the alternation's test>"}
node scripts/exhaustive-costs.mjs from <its downloaded artifacts> --run '<the dispatch>' --keep scripts/exhaustive-costs.tsv > costs.new
mv costs.new scripts/exhaustive-costs.tsv
node scripts/adr-index.mjs
npx --no-install @descent-vtt/spec-brief archive 084 --strict --pr <n> --summary "<one paragraph>"
```

- `round:additive` exits 1, and the ADR quotes it.
- The dispatch's eighteen shards are green, every test run, on the last code commit.
- Each on-arrival run of the alternation is held to its round's per-tick run, as [ADR-0224](../docs/adr/0224-a-unit-served-on-arrival.md) lists: the arena's bytes at the end, the messages, the spikes, and its turns and steps together against the per-tick turns. The 16-block runs also hold their pins. A sparse run lasts at least a minute.
- The ADR gives each round's seconds under both services and both drives. On-arrival service reads below per-tick service in every round under the reference drive, and under the sparse drive no slower than the spread of per-tick service's own three runs.
- The archive passes `--strict`: nothing outside `affectedFiles` and nothing in `protectedFiles` changed.

## Report

The closing message states:
- the service as built: the condition, the catch-up, how the arena comes up to date, and the reference;
- the property tests and the differential test, with what each holds;
- the dispatch with its run id and commit, each shard's job time against its bound, and that every pinned number held;
- the alternation's seconds on AArch64, and the cost table's seconds beside the table before it on x86-64;
- what `round:additive` read;
- any widening of `affectedFiles`, and why;
- that no whole-domain test ran on the developer machine;
- that the pull request is ready for review;
- what was not done and why.
