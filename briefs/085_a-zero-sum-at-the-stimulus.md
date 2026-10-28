---
status: proposed
date: 2026-10-28
affectedFiles: [crates/menteion-core/src/**, runtime/menteion-runtime/src/**, runtime/menteion-runtime/tests/**, testkit/**, docs/**, briefs/**, CHANGELOG.md, CLAUDE.md, README.md, scripts/exhaustive-costs.tsv]
protectedFiles: [crates/menteion-connectome/**, docs/explorations/0*.md, docs/benchmarks/results/**, benches/**, .github/workflows/**, .cargo/**, Cargo.toml, Cargo.lock, rust-toolchain.toml, clippy.toml, .spec-brief.json, package.json, package-lock.json, scripts/arm-run.sh, scripts/exhaustive-shard.sh, scripts/exhaustive-costs.mjs, scripts/check-additive.mjs, scripts/check-archive.mjs, scripts/check-scope.mjs, scripts/check-explorations.mjs, scripts/check-doc-shape.mjs, scripts/adr-index.mjs]
---

# Brief 085: A zero sum at the stimulus, and a gentler depression

<!-- @assert-absence target="runtime/menteion-runtime/src/executor.rs" symbol="fn set_zero_sum" reason="brief 085 precondition: main holds no group's sum; the round that builds the zero sum archives this brief in the same pull request" -->

## Mission

**This brief builds [ADR-0226](../docs/adr/0226-a-zero-sum-at-the-stimulus.md)'s two rules and runs H-46 once.**
Under the engine's default the couplings rise through every schedule of a long run. M12 asks for them bounded ([Appendix C](../docs/WHITEPAPER.md#appendix-c-roadmap)).

When the round is done, the tree holds:
- the zero sum at the stimulus, set by the host, each synapse's dropped fraction carried, and the depression's exponent as a configuration's, both unset by default and each the engine as it is when unset, held by property tests;
- H-46's protocol committed before any arm ran, its six arms and six controls run over sixteen schedules, and its verdict;
- the twelve long runs kept as `…_paused` tests, and a short form of each, its first schedule, in the weekly job.

It writes a built ADR, a protocol ADR and a measured ADR, adds its readings to [H-46's report](../docs/research/H-46.md) and rewrites `docs/STATUS.md` ([briefs/README.md](README.md#a-rounds-documents)).
The scope is the front matter's ([ADR-0191](../docs/adr/0191-a-rounds-scope-eighteen-shards-and-the-reproduction-first.md)).

---

## Standing directives

- **Latest ≠ Newest** (whitepaper §2.1, `CLAUDE.md` principle 4). The round depends on nothing new: no crate, no tool, no version of either.
- **Unset is the engine as it is.** With both rules unset no number moves, and the format stays 26. A pinned number that moves stops the round as a finding (H-46's stopping rule, step 2).
- **H-46 is ADR-0226's**: its clauses, its constants and its three predicted readings are committed in the protocol ADR and pinned in the tests before any arm runs, and nothing among them moves after.
- **No `unsafe` beyond ADR-0023's.** No `f32`/`f64`; every loop ends by construction ([ADR-0062](../docs/adr/0062-the-first-complete-sweeps-list.md)); the mutation gate catches every mutant in the lines the change touches ([ADR-0030](../docs/adr/0030-verification-governance.md)); a new rule carries a test over the lattice.
- **The runs are dispatches** ([briefs/README.md](README.md#evidencing-a-round)): the calibration at `scope=exhaustive` excluding the round's own tests, and the arms at `scope=arms`, dispatched together on the protocol's commit; the calibration read first. No whole-domain test runs on the developer machine.
- **The pull request opens as a draft with the round's first push** ([ADR-0223](../docs/adr/0223-the-gate-shortened-and-explorations-kept-as-tags.md)), and leaves draft with the commit that asks for the merge, which sets the round's ADRs to `accepted`.
- **The branch is named `claude/brief-085-…`**, never rebased or force-pushed once a dispatch has run on it.
- **The documents are ADR-0197's**: the new ADRs in its shape, the tables written by `node scripts/adr-index.mjs`, new text within `npm run spec:shape`'s limits.
- **No shard of the weekly job passes 60 per cent of its bound**, 240 minutes ([ADR-0179](../docs/adr/0179-the-bounds-rule-restated.md)). One that does is reported with its job's minutes.
- **GitHub without gh.** Dispatches, the pull request, logs and artifacts go through the REST API with the repository's own token (`~/.claude/menteion-tools/ghapi.sh` and its README); `git push` uses the same token. The token is never printed or put on a command line.
- **The maintainer is notified** as `briefs/README.md` says: when a question only they can answer stops the round, and when the pull request is ready for review.
- Conventional Commits with a real body; never commit on `main`.

## Context

Re-derived on 2026-10-28 against `main` at `2c48eee`. Line numbers move; the symbols are what to re-derive.

1. **The depression** (`crates/menteion-core/src/dynamics/synapse.rs`): ADR-0055's excitatory depression, *"`amount × magnitude / 2^13`"*, its reference magnitude a quarter of the width, 0x2000.
2. **The explorations' code**, written for explorations and not reviewed:
   - the tag `explore/011-zero-sum-at-the-stimulus`: `Executor::set_zero_sum`, its groups and the scan of a group's chains;
   - the tag `explore/012-the-zero-sums-rounding`: `set_zero_sum_rounding`, `ZeroSumRounding`, each synapse's fraction carried;
   - the tag `explore/013-the-carried-sum-and-the-exponent`: `DepressionExponent` in `synapse.rs`, `Executor::set_depression_exponent`, the integer square roots;
   - their harness in `tests/inhibition.rs`: the long run, the readings by schedule, the eighth image carried with its CRC-64 under `tests/explore-0NN/`.
3. **The runs to start from**: H-42's derivations on seeds 22, 23 and 24, pinned; H-25's two arms and their four clauses; H-20's schedule; exploration 007's long run in two jobs of eight schedules.
4. **The served-on-arrival engine** ([ADR-0225](../docs/adr/0225-a-unit-served-on-arrival-built.md)): an eight-schedule job took 101 to 110 minutes on AArch64 in explorations 011 to 013.
5. **The paused tests** ([ADR-0127](../docs/adr/0127-the-paused-line-leaves-the-weekly.md)): a test renamed `…_paused` leaves the weekly job, keeps its pins and runs by name at `scope=arms`.
6. **The process** ([briefs/README.md](README.md#evidencing-a-round)): `npm run round:additive` reads the change, which is not only additive; the reproduction runs the round's own weekly tests alone when nothing the calibration ran has changed; the cost table is regenerated with `--keep`.
7. **The schedule** runs on Wednesday at 20:17 UTC and takes most of the organisation's places for hours. A dispatch made then waits.

## Deliverables

- [ ] **The two rules, built, in a new ADR at the next free number (`ls docs/adr`), kind `built`.**
  - The zero sum at the stimulus: groups set by the host, each group's held value its sum when set, scaled back after a tick's consolidations with each synapse's fraction carried, no weight past the rail.
  - The depression's exponent: one, a half or a quarter, by integer square roots, one being ADR-0055's rule.
  - Unset, every run the run it was, bit for bit; neither rule in the image.
  - Their tests: by hand at the edges; over the lattice, the carried scaling against an exact rational oracle and each exponent against an oracle of its power; unset against the rule as it is.
- [ ] **H-46's protocol, in a new ADR at the next free number, kind `protocol`, before any arm runs.**
  - The networks, the arms and the controls, the sixteen schedules in two jobs, and how the eighth image is carried.
  - H-46's clauses and constants, restated from ADR-0226 and pinned in the tests; the three predicted readings as rules over the tables.
  - The calibration's check of H-46's stopping rule, step 2, as a test: each control's first schedule H-42's arm bit for bit.
  - The output of `npm run round:additive` on the protocol's commit.
- [ ] **The calibration and the first halves, dispatched together on the protocol's commit**: `scope=exhaustive` excluding this round's own tests, every pinned number held, and the twelve first halves at `scope=arms`. The calibration is read first.
- [ ] **The second halves**, dispatched on the commit that adds the twelve eighth images, with their CRC-64s, and nothing else that runs.
- [ ] **The tables pinned per schedule** once both halves are read: each arm's and control's highest coupling, its clauses, its reversals and its synapses by band of the width.
- [ ] **The long runs renamed `…_paused`**, every pin kept, and **twelve short forms**, each run's first schedule, as weekly `…_exhaustive` tests pinned from the same runs.
- [ ] **The reproduction**, dispatched as soon as the tables are pinned: `scope=exhaustive` with `only` naming the twelve short forms, when nothing the calibration ran has changed since its commit; otherwise every test. Its shards are green, and the cost table is regenerated from the calibration's and the reproduction's artifacts.
- [ ] **H-46's report**: its `verdict:` set, the readings' tables, each arm beside its control, and the dispatches that are their evidence.
- [ ] **The measured ADR**, kind `measured`, short: the verdict per network, arm and clause in one table; each predicted reading held or missed in a line; the step of the stopping rule reached, and the next decision it names, not taken.
- [ ] **The documents, in the same pull request.**
  - `docs/STATUS.md` rewritten whole; F-86's row as H-46's verdict leaves it.
  - Whitepaper: where ADR-0055's depression and the consolidation are described, the two rules; Appendix C's M12 row; the tables written by `node scripts/adr-index.mjs`; its version in both declarations, with its date.
  - `CHANGELOG.md`, one entry of at most 400 characters a change.
- [ ] **The brief archived** with `spec-brief archive 085 --strict`, every deliverable dispositioned. The round's ADRs are set to `accepted` by the commit that asks for the merge, and the pull request is marked ready for review with it.

## Not empowered

- **The round's scope** is the front matter's. It writes only inside `affectedFiles`, and never into `protectedFiles`. A file it must change outside the scope is handled as `briefs/README.md` says.
- **No number moves** with both rules unset, and no format, record or image byte changes.
- **No constant of H-46 moves** after any arm runs: not the exponent, the rounding, the groups, the arms, the controls, the networks or the schedules. There is no second attempt.
- **Neither rule enters `Config::reads_its_targets()` or `Config::default()`.** The default's adoption is the next decision's.
- No dependency, tool version, toolchain, MSRV or edition changes. No change to the workflow or the run scripts.
- No whole-domain test runs on the developer machine, and no rebase of the branch once a dispatch has run on it.
- No accepted ADR and no archived brief changes.

## Architectural empowerment

The executing session may replace any instruction above with a better decision, recorded in its ADR. That covers:
- the names of the setters, the configuration's field and the types;
- how a group's sum is read and where the carried fractions live, within the standing directive on `unsafe` and outside the image;
- when in a tick the scaling runs, so long as it follows the consolidations it answers, and it is the same on both architectures and both services;
- the integer form of each power, held to an oracle over the lattice;
- the order of the commits, so long as each builds and the protocol precedes every arm;
- a file the round must change and its scope does not hold, within this brief's mission, widened as `briefs/README.md` says.

It may not:
- change H-46's clauses, constants or readings, or run a second attempt;
- write into a protected file, or shrink `protectedFiles`;
- run a whole-domain test on the developer machine;
- reach the standing directives, the whitepaper's invariants or the constraints in `CLAUDE.md`.

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
npm run round:additive -- origin/main <the protocol's commit>
# ci.yml on this round's branch, inputs {"scope":"exhaustive","exclude":"<the round's own tests>"}
# ci.yml on this round's branch, inputs {"scope":"arms","tests":"<the twelve first halves>"}
# ci.yml on this round's branch, inputs {"scope":"arms","tests":"<the twelve second halves>"}
# ci.yml on this round's branch, inputs {"scope":"exhaustive","only":"<the twelve short forms>"}
node scripts/exhaustive-costs.mjs from <their downloaded artifacts> --run '<the calibration> and <the reproduction>' --keep scripts/exhaustive-costs.tsv > costs.new
mv costs.new scripts/exhaustive-costs.tsv
node scripts/adr-index.mjs
npx --no-install @descent-vtt/spec-brief archive 085 --strict --pr <n> --summary "<one paragraph>"
```

- `round:additive` exits 1, and the protocol ADR quotes it.
- The calibration's eighteen shards are green, and each control's first schedule is H-42's arm bit for bit.
- The reproduction's shards are green and reproduce every short form's pins.
- The archive passes `--strict`: nothing outside `affectedFiles` and nothing in `protectedFiles` changed.

## Report

The closing message states:
- the two rules as built, and their tests;
- H-46's verdict per network, arm and clause, and each predicted reading held or missed;
- the dispatches with their run ids and commits, each shard's and arm's job time against its bound;
- the step of the stopping rule reached, and the next decision it names;
- what `round:additive` read;
- any widening of `affectedFiles`, and why;
- that no whole-domain test ran on the developer machine;
- that the pull request is ready for review;
- what was not done and why.
