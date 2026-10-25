---
status: archived
date: 2026-10-25
affectedFiles: [runtime/cortex-runtime/tests/**, docs/**, briefs/**, CHANGELOG.md, CLAUDE.md, README.md, scripts/exhaustive-costs.tsv]
protectedFiles: [crates/**, benches/**, .github/workflows/**, scripts/arm-run.sh, scripts/exhaustive-shard.sh, scripts/exhaustive-costs.mjs, scripts/check-archive.mjs, scripts/check-scope.mjs, scripts/check-explorations.mjs, scripts/check-doc-shape.mjs, scripts/adr-index.mjs, .spec-brief.json, package.json, package-lock.json, rust-toolchain.toml, Cargo.toml, Cargo.lock]
integrity: sha256-0364bd4198247479e6c29743ed099b676431357b79bbf854d22c68d5234008ca
---

<!-- spec-brief:banner -->
> **Executed 2026-10-25 (UTC) in pull request [#237](https://github.com/DescentVTT/VirtualCortex/pull/237).** Brief 081 ran H-44 once: H-35's question under the engine's default on seeds 22, 23 and 24, beside a control whose image's step is zero. ADR-0213 committed the protocol (the control's image by a masked patch, three calibration tests, twelve arms) before any arm ran at three in four. ADR-0214 reads H-44 = yes at step 3, split (b): the default's six arms and the control's six all held H-35's four clauses, so the network the default settles holds the share. Predicted readings 1 and 2 missed, 3 held. Nothing under src/ changed. The body below describes the tree before execution and is not maintained.
<!-- /spec-brief:banner -->

# Brief 081: A reward right three times in four, on the default

## Mission

**This brief changes nothing in the engine and runs one hypothesis once.**
[ADR-0212](../../docs/adr/0212-three-in-four-on-the-default.md) asks H-35's question again under the engine's default:
- on seeds 22, 23 and 24, H-42's arms with the task's feedback `ThreeInFour`;
- beside each, a control from the same image with its step written zero, so that its gain is held through the arm.

ADR-0212 wrote **H-44** before any run, in [H-44's report](../../docs/research/H-44.md), and **predicts yes** with three readings.

When the round is done, the tree holds:
- the control's image, made by the tests from the default's;
- on each of seeds 22, 23 and 24 at 1 024 units, H-25's two arms under `ThreeInFour`, from the default's image and from the control's, as weekly `exhaustive` tests: twelve arms;
- H-44's verdict per network, arm and clause, the control's read beside it, and the step of its stopping rule reached, with the next decision it names and does not take.

Its readings go to H-44's report, its measured ADR is short, and it rewrites `docs/STATUS.md` ([briefs/README.md](../README.md#a-rounds-documents)).

The scope is the front matter's ([ADR-0191](../../docs/adr/0191-a-rounds-scope-eighteen-shards-and-the-reproduction-first.md)): `spec-brief archive --strict` refuses the round outside it.

**Do not run this round beside brief 065's**: both dispatch the weekly workflow.

---

## Standing directives

- **Latest ≠ Newest** (whitepaper §2.1, `CLAUDE.md` principle 4). The round depends on nothing new: no crate, no tool and no version bump. No mechanism is added to the engine.
- **The control is ADR-0212's.** It is the image an engine born under `Config::reads_its_targets()` settles on the network, with the homeostasis record's `control_step_q0_16` written zero and the image re-sealed, every other byte the image's. At zero the rate's rule leaves the gain.
- **The runs are on seeds 22, 23 and 24 at 1 024 units, from the default's network** (H-42's derivations, pinned): H-42's two arms, H-25's on H-20's schedule, with the task's feedback `Feedback::ThreeInFour`, from the default's image and from the control's. No other network, share, step or arm is run.
- **H-44's clauses, its constants and its three predicted readings are committed before any arm runs at three in four**, and do not move after it. There is no second attempt.
- **The runs are dispatches, as `briefs/README.md`'s "Evidencing a round" says** ([ADR-0211](../../docs/adr/0211-the-round-shortened.md)).
  - The calibration, excluding the round's own tests, and the twelve arms are dispatched together on the protocol's commit.
  - The calibration is read first. No arm's log is read until every calibration shard is green and every calibration check of H-44's stopping rule holds; one that fails stops the round, and its arms are void.
  - The arms run at `scope=arms`, once.
  - The reproduction is dispatched as soon as the tables are pinned, at `scope=exhaustive` with only the round's own tests, when nothing the calibration ran has changed since its commit; otherwise with every test. The documents are written while it runs.
  - No round dispatches the sweep. No whole-domain test runs on the developer machine.
- **The branch is named `claude/brief-081-…`**, never rebased or force-pushed once a dispatch has run on it. Every commit is signed.
- **The prediction is ADR-0212's**: yes. The round reports it against what was read, and each predicted reading as held or missed.
- **The engine and the harness are read before a description of them is trusted**, this brief's and ADR-0212's included. So is exploration 001's branch, `explore/001-the-old-noes-under-the-default`, whose tests ran this share through the host.
- **No capacity of a run's configuration is reached silently.**
- No `f32`/`f64`, oracles included; every operation on a state field saturates or wraps by name ([ADR-0029](../../docs/adr/0029-structural-enforcement.md)). Every loop ends by construction ([ADR-0062](../../docs/adr/0062-the-first-complete-sweeps-list.md)). No `unsafe`.
- A heavy run is an `#[ignore]`d test whose name contains `exhaustive`. The runtime's gate grows by at most one test ([ADR-0061](../../docs/adr/0061-the-learning-runs-leave-the-gate.md)).
- **The documents are ADR-0197's and ADR-0201's**: new ADRs in their shape, the tables written by `node scripts/adr-index.mjs`, new text within `npm run spec:shape`'s limits and its math GitHub's.
- **The pull request leaves draft with the commit that asks for the merge**, which sets the round's ADRs to `accepted`.
- **No shard of the weekly job passes 60 per cent of its bound**, 180 minutes, at eighteen shards ([ADR-0179](../../docs/adr/0179-the-bounds-rule-restated.md), ADR-0191), read from the calibration's shards. One that does is reported with its job's minutes.
- No pinned number of an earlier round moves.
- **The maintainer is notified** as `briefs/README.md` says: when a question only they can answer stops the round, and when the pull request is ready for review.
- Conventional Commits with a real body; never commit on `main`; the required checks keep their names.

## Context

Re-derived on 2026-10-25 against `main` at `14c1f67`. Line numbers move; the symbols and the quoted sentences are what to re-derive.

1. **The feedback** (`runtime/cortex-runtime/src/task.rs`): `Feedback::ThreeInFour`, its coin `MISLEADING_IN_FOUR_BITS`, `Task::misleading_in_four_at`; on H-25's task it misleads 1 913 of 7 680 trials ([ADR-0171](../../docs/adr/0171-a-reward-right-three-times-in-four-built.md)).
2. **The step** (`crates/cortex-homeostasis/src/lib.rs`, protected): `HomeostaticDrivePool::control_step_q0_16`, bytes `[20..22]` of the record, *"0 leaves the gain"*; both the rate's rule and ADR-0036's run at it. After the probe the default's image carries the step the probe made, about 1/61 to 1/60 ([ADR-0204](../../docs/adr/0204-a-step-the-engine-makes-from-its-own-loop-measured.md)).
3. **The image** (`runtime/cortex-runtime/src/image.rs`): `Image::encode`, `Image::decode` and their checksums. The tests already make an image with a section's bytes written and the section re-sealed, every other byte the image's (`tests/inhibition.rs`, the functions documented so).
4. **The default's network**: `UNDER_THE_PROBE` and `PROBE_RULES` in `tests/inhibition.rs`, H-42's derivations pinned. H-42's arms ([ADR-0204](../../docs/adr/0204-a-step-the-engine-makes-from-its-own-loop-measured.md)): mappings 120 to 127 of 128, reversals in 10 to 14 blocks, their blocks pinned.
5. **H-35's and H-36's arms** (`tests/inhibition.rs`): `arm_on`, `misled_on`, `own_first_block` and `others_step`; H-35's clauses and the split of its step 5 ([H-35](../../docs/research/H-35.md), [H-36](../../docs/research/H-36.md)).
6. **Exploration 001** ([its note](../../docs/explorations/001-the-old-noes-under-the-default.md)): `three_in_four_on_the_default_s_network_on_seed_*_explore` on its branch ran H-35's arm on H-42's three networks with the reward given through the host. It read every clause in every arm, the lowest mapping at 90 of 128. Its figures are exploratory.
7. **The process** ([ADR-0211](../../docs/adr/0211-the-round-shortened.md), ADR-0191, ADR-0197, ADR-0201, `briefs/README.md`): eighteen weekly shards; the calibration with the arms; the reproduction with `-f only`; the cost table from the calibration's and the reproduction's artifacts together; `spec-brief archive <NNN> --strict` measured from `origin/main`.
8. **The sweep**: brief 080's, run 40171928796, ADR-0212 records. The schedule's run every Monday at 03:00Z sweeps the whole tree and holds most of the account's twenty places for hours; a dispatch made then waits. The first ADR that merges after a scheduled sweep ends records it (ADR-0158).

## Deliverables

- [x] **H-44's protocol, in a new ADR at the next free number (`ls docs/adr`), kind `protocol`, before any arm runs at three in four.**
  - The control's image, how it is made from the default's, and its checks.
  - H-44's clauses and constants, restated from ADR-0212 and pinned in the tests.
  - The three predicted readings as rules over the tables.
  - The calibration's checks of H-44's stopping rule, step 2, as tests.
- [x] **The calibration and the arms, dispatched together on the protocol's commit**: a dispatch at `scope=exhaustive` excluding this round's own tests, every pinned number held, and the twelve arms at `scope=arms`, each a weekly `exhaustive` test run once. The calibration is read first.
- [x] **The arms' tables pinned per block**, once the calibration is green.
- [x] **The reproduction**, dispatched as soon as the tables are pinned: `scope=exhaustive` with `-f only` naming the round's own tests, when the diff from the calibration's commit changes no file under `src/` and no test or helper the calibration ran. Its shards are green and reproduce every pinned number, and the cost table is regenerated from the calibration's and the reproduction's artifacts.
- [x] **H-44's report**, `docs/research/H-44.md`: its `verdict:` set, the readings' tables, each default arm beside its control and exploration 001's arm, and the dispatches that are their evidence.
- [x] **The measured ADR**, kind `measured`, short:
  - the verdict per network, arm and clause, and the control's, in one table;
  - each of the three predicted readings, held or missed, in a line;
  - the step of the stopping rule reached, its split, and the next decision it names, not taken.
- [x] **The gate.** At most one runtime test: H-44's clauses and its predicted readings' rules at their edges, over tables written by hand.
- [x] **The documents, in the same pull request.**
  - `docs/STATUS.md` rewritten whole.
  - Whitepaper: the tables written by `node scripts/adr-index.mjs`, and what the result changes elsewhere in it.
  - `CHANGELOG.md`, one entry of at most 400 characters a change.
  - `README.md` and `docs/zh-TW/README.md` as the result requires.
  - The whitepaper's version in both declarations, with its date.
- [x] **The brief archived** with `spec-brief archive 081 --strict`, every deliverable dispositioned. The round's ADRs are set to `accepted` by the commit that asks for the merge, and the pull request is marked ready for review with it.

## Not empowered

- **The round's scope** is the front matter's. It writes only inside `affectedFiles`, and never into `protectedFiles`. A file it must change outside the scope is handled as `briefs/README.md` says.
  - No file under `src/` changes, unless the control's image cannot be made from the tests; then the widening is a commit of its own that says why.
  - The default, the read, the probe, the step's rule, the critic and the task's feedback do not change.
- **The networks, the share, the arms and the control are ADR-0212's.** None is fitted to the result, and none other is run.
- **No record, section or format changes.**
- H-44's clauses, its constants and its predicted readings do not move after such a run. There is no second attempt, and a run is not repeated for a check of the harness.
- No arm's log is read before the calibration is green.
- No whole-domain test runs on the developer machine.
- No pinned number of an earlier round moves.
- No float anywhere, no dependency, no `unsafe`, and no rebase of the branch once a dispatch has run on it.

## Architectural empowerment

The executing session may replace any instruction above with a better decision, recorded in its ADR. That covers:
- how the control's image is made and checked, and where its function lives in the harness;
- how the arms and the readings are dealt into tests and dispatches, and what of exploration 001's tests is taken;
- how each reading is computed and pinned, and how the control's gain is shown held;
- whether the reproduction runs only the round's own tests, read from the diff as `briefs/README.md` says;
- a file the round must change and its scope does not hold, within this brief's mission, widened as `briefs/README.md` says.

It may not:
- write into a protected file, or shrink `protectedFiles`;
- change a network, the share, an arm, the schedule or the control's one field;
- add a mechanism to the engine;
- move a clause, a constant or a predicted reading after such a run;
- run a whole-domain test on the developer machine;
- reach the standing directives, the whitepaper's invariants or the constraints in `CLAUDE.md`.

If the calibration fails, the round stops there as H-44's stopping rule says: the finding is written, and no arm's log is read into a table.

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

By dispatch, never on the developer machine:

```bash
gh workflow run ci.yml --ref <this round's branch> -f scope=exhaustive -f exclude='<this round's own tests>'
gh workflow run ci.yml --ref <this round's branch> -f scope=arms -f tests='<the twelve arms>'
gh workflow run ci.yml --ref <this round's branch> -f scope=exhaustive -f only='<this round's own tests>'
node scripts/exhaustive-costs.mjs from <both runs' downloaded artifacts> --run '<calibration> and <reproduction>' > scripts/exhaustive-costs.tsv
node scripts/adr-index.mjs
npx --no-install @descent-vtt/spec-brief archive 081 --strict --pr <n> --summary "<one paragraph>"
```

- The calibration's shards are green with the round's tests excluded, and every calibration check holds.
- Each arm's job exits 0 with one test passed and its log kept.
- The reproduction's shards are green and reproduce every pinned number.
- In the history, H-44's constants and its predicted readings precede every dispatch that runs an arm at three in four, and every commit a dispatch ran on is an ancestor of the pull request's head.
- The archive passes `--strict`: nothing outside `affectedFiles` and nothing in `protectedFiles` changed.

## Report

The closing message states:
- the control's image as made, and its checks;
- the protocol, and when it was committed;
- H-44's verdict per network, arm and clause, the control's beside it, and the prediction and each predicted reading against what was read;
- each default arm beside its control and exploration 001's arm;
- the step of the stopping rule reached, its split and the next decision it names, not taken;
- each dispatch with its run id and commit: the calibration, the arms and the reproduction, and whether the reproduction named `only`;
- any widening of `affectedFiles`, and why;
- that no whole-domain test ran on the developer machine, that each run ran once, and that no pinned number moved;
- each shard's job time against its bound and the regenerated cost table;
- that the pull request is ready for review;
- what was not done and why.
