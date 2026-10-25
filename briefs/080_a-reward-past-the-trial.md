---
status: proposed
date: 2026-10-25
affectedFiles: [runtime/cortex-runtime/src/**, runtime/cortex-runtime/tests/**, docs/**, briefs/**, CHANGELOG.md, CLAUDE.md, README.md, scripts/exhaustive-costs.tsv]
protectedFiles: [crates/**, benches/**, .github/workflows/**, scripts/arm-run.sh, scripts/exhaustive-shard.sh, scripts/exhaustive-costs.mjs, scripts/check-archive.mjs, scripts/check-scope.mjs, scripts/check-explorations.mjs, scripts/check-doc-shape.mjs, scripts/adr-index.mjs, .spec-brief.json, package.json, package-lock.json, rust-toolchain.toml, Cargo.toml, Cargo.lock]
---

# Brief 080: A reward past the trial

## Mission

**This brief builds one thing in the engine and runs one hypothesis once.**
[ADR-0207](../docs/adr/0207-a-reward-past-the-trial-its-address-kept.md) takes M10:
- with `Config::kept_addresses` set, each trial's close keeps its drawn address and its critic counts;
- a reward names the age of the trial it is for, and the engine consolidates under that trial's address.

ADR-0207 wrote **H-43** before any run, in [H-43's report](../docs/research/H-43.md), and **predicts yes** with three readings.

When the round is done, the tree holds:
- the kept addresses in the engine, unset bit for bit, and the task's delay;
- on each of seeds 22, 23 and 24 at 1 024 units, H-25's two arms at a delay of one trial and of two, under the default's network, as weekly `exhaustive` tests: twelve arms;
- H-43's verdict per network, arm, delay and clause, and the step of its stopping rule reached, with the next decision it names and does not take.

Its readings go to H-43's report, its measured ADR is short, and it rewrites `docs/STATUS.md` ([briefs/README.md](README.md#a-rounds-documents)).

The scope is the front matter's ([ADR-0191](../docs/adr/0191-a-rounds-scope-eighteen-shards-and-the-reproduction-first.md)): `spec-brief archive --strict` refuses the round outside it.

**Do not run this round beside brief 065's**: both dispatch the weekly workflow.

---

## Standing directives

- **Latest ≠ Newest** (whitepaper §2.1, `CLAUDE.md` principle 4). The round depends on nothing new: no crate, no tool and no version bump. The engine's own address and critic are kept longer.
- **The kept addresses are ADR-0207's.**
  - `Config::kept_addresses`, 0 unset, at most 4, requires the critic and its window.
  - A trial's close keeps the sources the critic counted, the targets and the counts, as the newest entry, and zeroes the counts and reopens the critic's window as a reward does today.
  - A reward for the trial of age a writes that entry's address, reads the value and moves the value weights from its counts, sends the error to the modulator, and spends the entry.
  - Unset, every run is the run it was, bit for bit. Kept and rewarded at age zero, every run is the run it was too.
  - Nothing of it is in the image, and the format stays 25.
- **The task's delay is ADR-0207's**: trial t's sign at the end of trial t + k, at age k; none at the first k trials' ends; the last k signs not given.
- **The runs are on seeds 22, 23 and 24 at 1 024 units, from the default's network** (H-42's derivations, pinned), `kept_addresses` 2, H-25's arms on H-20's schedule, at k = 1 and k = 2. No other network, delay or arm is run.
- **H-43's clauses, its constants and its three predicted readings are committed before any arm runs at a delay**, and do not move after it. There is no second attempt.
- **The runs are dispatches, as `briefs/README.md`'s "Evidencing a round" says.**
  - The calibration excludes the round's own tests.
  - The twin, an arm with the addresses kept and rewarded at age zero, is H-42's arm bit for bit on each network, or the round stops.
  - The arms and any trial run at `scope=arms`, once.
  - The reproduction is dispatched as soon as the tables are pinned, at `scope=both` since a file under `src/` changes, and the documents are written while it runs.
  - A change after the calibration is calibrated again, or shown in the ADR to change no run.
  - No whole-domain test runs on the developer machine.
- **The branch is named `claude/brief-080-…`**, never rebased or force-pushed once a dispatch has run on it. Every commit is signed.
- **The prediction is ADR-0207's**: yes. The round reports it against what was read, and each predicted reading as held or missed.
- **The engine and the harness are read before a description of them is trusted**, this brief's and ADR-0207's included. So is exploration 001's branch, `explore/001-the-old-noes-under-the-default`, whose tests ran the delay through the host.
- **No capacity of a run's configuration is reached silently.**
- No `f32`/`f64`, oracles included; every operation on a state field saturates or wraps by name ([ADR-0029](../docs/adr/0029-structural-enforcement.md)). Every loop ends by construction ([ADR-0062](../docs/adr/0062-the-first-complete-sweeps-list.md)). No `unsafe`.
- A heavy run is an `#[ignore]`d test whose name contains `exhaustive`. The runtime's gate grows by at most one test ([ADR-0061](../docs/adr/0061-the-learning-runs-leave-the-gate.md)). A new rule carries a test over the lattice of `testkit/prop.rs`, and the mutation gate catches every mutant in the lines the change touches ([ADR-0030](../docs/adr/0030-verification-governance.md)).
- **The documents are ADR-0197's and ADR-0201's**: new ADRs in their shape, the tables written by `node scripts/adr-index.mjs`, new text within `npm run spec:shape`'s limits and its math GitHub's.
- **The pull request leaves draft with the commit that asks for the merge**, which sets the round's ADRs to `accepted`.
- **No shard of the weekly job passes 60 per cent of its bound**, 180 minutes, at eighteen shards ([ADR-0179](../docs/adr/0179-the-bounds-rule-restated.md), ADR-0191). One that does is reported with its job's minutes.
- No pinned number of an earlier round moves.
- Conventional Commits with a real body; never commit on `main`; the required checks keep their names.

## Context

Re-derived on 2026-10-25 against `main` at `045952a`. Line numbers move; the symbols and the quoted sentences are what to re-derive.

1. **The reward's path** (`runtime/cortex-runtime/src/executor.rs`):
   - `address_drawn(targets)` writes the sources from the critic's counts (`features`, a `Vec<u32>` a unit) and the targets, between ticks; it is refused while the critic or its window is unset (`draws`).
   - `reward(reward_q16)` with the critic set reads the value from the counts, moves the value weights by the error times the counts, zeroes every count, reopens the critic's window (`window_opened`) and sends the error to the modulator.
   - The addressed and unaddressed modulations, the signed gate and `consolidate_signed` act at presynaptic spikes during the next trial.
2. **The task** (`runtime/cortex-runtime/src/task.rs`): `Task::trial` injects the stimulus, runs 16 384 ticks, reads the counts and the selection, writes the address, then gives the reward. `Feedback` is `Answer`, `Shuffled`, `Withheld`, `SevenInEight` or `ThreeInFour`.
3. **The trace** (`crates/cortex-core/src/dynamics/synapse.rs`, protected): `ELIGIBILITY_TAU_SHIFT` 16, τ = 65 536 ticks, four trials; a shift of 16 is the ceiling.
4. **The default's network**: `UNDER_THE_PROBE` and `PROBE_RULES` in `tests/inhibition.rs`, H-42's derivations pinned; ADR-0206's default writes their candidate byte for byte.
5. **Exploration 001** ([its note](../docs/explorations/001-the-old-noes-under-the-default.md)): on seed 22, a delay of one or two trials with no mechanism learned nothing; the address named the rewarded pair in 0.20 to 0.27 of rewards; the trace held 0.56 to 0.73. Its tests on `explore/001-the-old-noes-under-the-default` gave the reward through the host (`run_on_scheduled_over`, `Withheld`). Its figures are exploratory.
6. **H-42's arms** (ADR-0204): mappings 120 to 127 of 128 correct, reversals in 10 to 14 blocks, the address naming the rewarded pair 0.94 to 0.96 on time.
7. **The process** (ADR-0184, ADR-0191, ADR-0197, ADR-0201, `briefs/README.md`): eighteen weekly shards; the runtime's sweep over sixteen shards since ADR-0207; `spec-brief archive <NNN> --strict` measured from `origin/main`.
8. **The sweep**: brief 079's, run 40097581635, ADR-0207 records. This round's own sweep is recorded by the first ADR that merges after it ends (ADR-0158), and a scheduled weekly run's sweep too.

## Deliverables

- [x] **The kept addresses and the task's delay, built, in a new ADR at the next free number (`ls docs/adr`), kind `built`.**
  - ADR-0207's `Config::kept_addresses`, the trial's close and the reward of an age, with their refusals.
  - The task's delay.
  - Unset, every run the run it was, bit for bit; kept and rewarded at age zero, the same.
  - Its tests:
    - by hand at the edges, the refusals among them;
    - over the lattice against an oracle;
    - the twin at age zero against the run without the addresses kept;
    - an entry spent once, and a reward of an age not kept refused.
- [x] **H-43's protocol, in a new ADR at the next free number, kind `protocol`, before any arm runs at a delay.**
  - The networks, the delays, the arms.
  - H-43's clauses and constants, restated from ADR-0207 and pinned in the tests.
  - The three predicted readings as rules over the tables.
  - How the address each reward writes is read against the rewarded trial's pair.
- [x] **The calibration**, before any such run: a dispatch at `scope=exhaustive` excluding this round's own tests, every pinned number held; and the twin at age zero on each network, H-42's arm bit for bit.
- [x] **The arms**: twelve, each a weekly `exhaustive` test, run once by a dispatch at `scope=arms`, their tables pinned per block.
- [x] **The reproduction**, dispatched at `scope=both` as soon as the arms' tables are pinned. Its eighteen whole-domain shards are green and reproduce every pinned number, and the cost table is regenerated from them.
- [x] **H-43's report**, `docs/research/H-43.md`: its `verdict:` set, the readings' tables, each arm beside its on-time twin and exploration 001's arm at the same delay, and the dispatches that are their evidence.
- [x] **The measured ADR**, kind `measured`, short:
  - the verdict per network, arm, delay and clause, in one table;
  - each of the three predicted readings, held or missed, in a line;
  - the step of the stopping rule reached, and the next decision it names, not taken.
- [x] **The gate.** At most one runtime test: H-43's clauses and its predicted readings' rules at their edges, over tables written by hand.
- [x] **The documents, in the same pull request.**
  - `docs/STATUS.md` rewritten whole.
  - Whitepaper: where the reward's path and the task are described, the kept addresses and the delay; and the tables written by `node scripts/adr-index.mjs`.
  - `CHANGELOG.md`, one entry of at most 400 characters a change.
  - `README.md` and `docs/zh-TW/README.md` as the result requires.
  - The whitepaper's version in both declarations, with its date.
- [x] **The brief archived** with `spec-brief archive 080 --strict`, every deliverable dispositioned. The round's ADRs are set to `accepted` by the commit that asks for the merge, and the pull request is marked ready for review with it.

## Not empowered

- **The round's scope** is the front matter's. It writes only inside `affectedFiles`, and never into `protectedFiles`. A file it must change outside the scope is handled as `briefs/README.md` says.
  - The trace, its decay and its ceiling do not change; `crates/` is protected.
  - The default, the read, the probe and the critic's rule do not change.
- **`kept_addresses` 2, the delays 1 and 2, the arms and the networks are ADR-0207's.** None is fitted to the result, and none other is run.
- **No record, section or format changes.**
- H-43's clauses, its constants and its predicted readings do not move after such a run. There is no second attempt, and a run is not repeated for a check of the harness.
- No whole-domain test runs on the developer machine.
- No pinned number of an earlier round moves.
- No float anywhere, no dependency, no `unsafe`, and no rebase of the branch once a dispatch has run on it.

## Architectural empowerment

The executing session may replace any instruction above with a better decision, recorded in its ADR. That covers:
- the names of the field, the two calls and their errors, and how the entries are held in the executor;
- how the task carries its delay, and how a trial's close and a reward of an age are called from it;
- whether the trial's close takes the selection's targets or the address already drawn;
- how the twin, the arms and the readings are dealt into tests and dispatches, and what of exploration 001's tests is taken;
- how each reading is computed and pinned;
- a file the round must change and its scope does not hold, within this brief's mission, widened as `briefs/README.md` says.

It may not:
- write into a protected file, or shrink `protectedFiles`;
- change a delay, `kept_addresses`, a network, the task's feedback or the schedule;
- put the kept addresses in the image;
- move a clause, a constant or a predicted reading after such a run;
- run a whole-domain test on the developer machine;
- reach the standing directives, the whitepaper's invariants or the constraints in `CLAUDE.md`.

If the calibration or the twin fails, the round stops there as H-43's stopping rule says: the finding is written, and no such run is made.

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
gh workflow run ci.yml --ref <this round's branch> -f scope=arms -f tests='<the twins, then the arms>'
gh workflow run ci.yml --ref <this round's branch> -f scope=both
node scripts/exhaustive-costs.mjs from <the reproduction's downloaded artifacts> --run <its id> > scripts/exhaustive-costs.tsv
node scripts/adr-index.mjs
npx --no-install @descent-vtt/spec-brief archive 080 --strict --pr <n> --summary "<one paragraph>"
```

- The calibration's shards are green with the round's tests excluded.
- Each run's job exits 0 with one test passed and its log kept.
- The reproduction's eighteen whole-domain shards are green and reproduce every pinned number.
- In the history, H-43's constants and its predicted readings precede every dispatch that runs an arm at a delay, and every commit a dispatch ran on is an ancestor of the pull request's head.
- The archive passes `--strict`: nothing outside `affectedFiles` and nothing in `protectedFiles` changed.

## Report

The closing message states:
- the kept addresses and the delay as built, and that every run with them unset, or kept at age zero, is the run it was;
- the protocol, and when it was committed;
- H-43's verdict per network, arm, delay and clause, and the prediction and each predicted reading against what was read;
- each arm beside its on-time twin and exploration 001's arm;
- the step of the stopping rule reached and the next decision it names, not taken;
- each dispatch with its run id and commit: the calibration, the twins, the arms and the reproduction;
- any widening of `affectedFiles`, and why;
- that no whole-domain test ran on the developer machine, that each run ran once, and that no pinned number moved;
- each shard's job time against its bound and the regenerated cost table;
- that the pull request is ready for review;
- what was not done and why.
