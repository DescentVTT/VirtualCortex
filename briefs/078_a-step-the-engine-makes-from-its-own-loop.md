---
status: proposed
date: 2026-10-20
affectedFiles: [runtime/cortex-runtime/src/**, runtime/cortex-runtime/tests/**, crates/cortex-connectome/src/**, docs/**, briefs/**, CHANGELOG.md, CLAUDE.md, README.md, scripts/exhaustive-costs.tsv]
protectedFiles: [crates/cortex-core/src/**, crates/cortex-homeostasis/src/**, benches/**, .github/workflows/**, scripts/arm-run.sh, scripts/exhaustive-shard.sh, scripts/exhaustive-costs.mjs, scripts/check-archive.mjs, scripts/check-scope.mjs, scripts/check-explorations.mjs, scripts/check-doc-shape.mjs, scripts/adr-index.mjs, .spec-brief.json, package.json, package-lock.json, rust-toolchain.toml, Cargo.toml, Cargo.lock]
---

# Brief 078: A step the engine makes from its own loop

## Mission

**This brief builds one thing in the engine and runs one hypothesis once.**
[ADR-0200](../docs/adr/0200-a-step-the-engine-makes-from-its-own-loop.md) gives the read a probe:
- with a flag set, the read's end holds the gain at ±1/64 of the read's for 16 windows;
- it reads the steepness S of the network's count;
- the rate's rule then runs at step κ = (1/4)/S, not at 1/16.

ADR-0200 wrote **H-42** before any run, in [H-42's report](../docs/research/H-42.md), and **predicts yes** with four readings.

When the round is done, the tree holds:
- the probe in the engine, unset bit for bit, its flag at the modulator section's `[27]`, the image at format 25;
- on each of seeds 22, 23 and 24 at 1 024 units, the derivation with the probe set and H-25's two arms under it, as weekly `exhaustive` tests;
- H-42's verdict per network, arm and clause, and the step of its stopping rule reached, with the next decision it names and does not take.

It is the first round under [ADR-0197](../docs/adr/0197-one-place-for-each-kind-of-fact.md)'s documents. Its readings go to H-42's report, its measured ADR is short, and it rewrites `docs/STATUS.md` ([briefs/README.md](README.md#a-rounds-documents)).

The scope is the front matter's ([ADR-0191](../docs/adr/0191-a-rounds-scope-eighteen-shards-and-the-reproduction-first.md)): `spec-brief archive --strict` refuses the round outside it.

**Do not run this round beside brief 065's**: both dispatch the weekly workflow.

---

## Standing directives

- **Latest ≠ Newest** (whitepaper §2.1, `CLAUDE.md` principle 4). The round depends on nothing new: no crate, no tool and no version bump. The probe is the engine's own measure, given to the engine's own rule.
- **The probe is ADR-0200's.**
  - Its flag is `[27]`, set only beside the read; without the read it is refused.
  - Unset, every run is the run it was, bit for bit, at format 25, and an image of format 24 reads as unset.
  - Its 16 windows run at g₀·(1 − 1/64), then g₀·(1 + 1/64), then g₀·(1 − 1/64): four, eight and four windows, g₀ the read's held gain.
  - It keeps one signed sum D, the + windows' counts less the − windows'. Its state is in the read's bytes, `[39]`, `[54..56)` and `[60..64)`.
  - At its end, κ = 4 096·c*/D in Q0.16, c* the read's target count; the gain returns to g₀ and the rule runs from the next window.
  - It is refused when D ≤ 0 or κ would pass `CONTROL_STEP_MAX_Q0_16`, and a refused probe ends as a refused read ends.
- **The networks are seeds 22, 23 and 24 at 1 024 units, under `Config::reads_its_targets()` with the probe's flag set.** No other network, size, λ or probe is run.
- **A refused read or probe, a lead-in that does not settle, a rule that gives no value, or a frozen block that fails a measure is a reading of H-42** (its clause 0). It stops that network, not the round, and it is not repaired.
- **H-42's clauses, its constants and its four predicted readings are committed before any lead-in is run with the probe set**, and do not move after it. There is no second attempt.
- **The runs are dispatches, as `briefs/README.md`'s "Evidencing a round" says.**
  - The calibration excludes the round's own tests.
  - The derivations, the arms and any trial run at `scope=arms`, once.
  - The reproduction is dispatched as soon as the tables are pinned, at `scope=both` since a file under `src/` changes, and the documents are written while it runs.
  - A change after the calibration is calibrated again, or shown in the ADR to change no run.
  - No whole-domain test runs on the developer machine.
- **The branch is named `claude/brief-078-…`**, never rebased or force-pushed once a dispatch has run on it. Every commit is signed.
- **The prediction is ADR-0200's**: yes. The round reports it against what was read, and each predicted reading as held or missed.
- **The engine and the harness are read before a description of them is trusted**, this brief's and ADR-0200's included.
- **No capacity of a run's configuration is reached silently.**
- No `f32`/`f64`, oracles included; every operation on a state field saturates or wraps by name ([ADR-0029](../docs/adr/0029-structural-enforcement.md)). Every loop ends by construction ([ADR-0062](../docs/adr/0062-the-first-complete-sweeps-list.md)). No `unsafe`.
- A change to a record's or a section's bytes bumps the image's version, updates its table in whitepaper §5.2, and gets a changelog entry, in the same pull request.
- A heavy run is an `#[ignore]`d test whose name contains `exhaustive`. The runtime's gate grows by at most one test ([ADR-0061](../docs/adr/0061-the-learning-runs-leave-the-gate.md)). A new rule carries a test over the lattice of `testkit/prop.rs`, and the mutation gate catches every mutant in the lines the change touches ([ADR-0030](../docs/adr/0030-verification-governance.md)).
- **The documents are ADR-0197's**: new ADRs in its shape, the tables written by `node scripts/adr-index.mjs`, new text within `npm run spec:shape`'s limits.
- **The pull request leaves draft with the commit that asks for the merge**, which sets the round's ADRs to `accepted`.
- **No shard of the weekly job passes 60 per cent of its bound**, 180 minutes, at eighteen shards ([ADR-0179](../docs/adr/0179-the-bounds-rule-restated.md), ADR-0191). One that does is reported with its job's minutes.
- No pinned number of an earlier round moves, but the whole-image pins by the format's mask.
- Conventional Commits with a real body; never commit on `main`; the required checks keep their names.

## Context

Re-derived on 2026-10-20 against `main` at `26cadb7`. Line numbers move; the symbols and the quoted sentences are what to re-derive.

1. **The rate's rule** (`crates/cortex-homeostasis/src/lib.rs`, protected):
   - `regulate_rate(target)` reads c/c* through `rate_over_target_q16` and moves the gain through `move_gain`, by `control_step_q0_16`.
   - `hold()` holds the gain, keeps σ̂ as a reading and returns the window's spikes.
   - `GAIN_MIN_Q16` is 0.25, `GAIN_MAX_Q16` 4 and `CONTROL_STEP_MAX_Q0_16` 0x8000.
   - A bin is 2^12 ticks and a window 2^5 bins.
2. **The engine** (`runtime/cortex-runtime/src/executor.rs`):
   - `tally` closes each bin, and at the window's cadence calls `read_close` while a read is under way, else `regulate_rate` when a target is set.
   - `read_close` holds the gain and steps the read through `read_step`. At its last window `ReadStep::Target` sets the rate's target and, with ADR-0194's flag, the inhibitory period.
   - `TargetRead::Reading { left, sum }` is the read's state, and `shared.gain` publishes the gain to the workers each tick.
   - `Config::reads_its_targets()` is ADR-0196's default, at step 0x1000 (ADR-0198).
3. **The image** (`runtime/cortex-runtime/src/image.rs`, `crates/cortex-connectome/src/lib.rs`):
   - `MODULATOR_RESERVED` is `[27..28)`, and `FORMAT_VERSION` is 24.
   - The read's state is at `[39]`, `[54..56)` and `[60..64)`, and its flag for the inhibitory target at `[26]`.
   - The homeostasis section holds the step at `[20..22)` and the gain at `[28..32)`.
4. **The derivation and its readers** (`runtime/cortex-runtime/tests/inhibition.rs`, `tests/instrument/harness.rs`):
   - `derive_under` and `candidate_under` run a settling; `UNDER_BOTH_TARGETS` is H-41's.
   - `Settling::settled` applies H-39's criteria to the windows after the read.
   - `opposite_pairs` counts the alternation, and `gain_read` the gain's band, mean and counts.
5. **H-41's runs, pinned**: the derivations and six arms under `UNDER_BOTH_TARGETS`, with `BOTH_*` tables, are this round's twins up to the read's end.
6. **The figures ADR-0200 rests on**:
   - S read at 15.0, 21.4 and 16.0 by the first window's move (ADR-0181, ADR-0183);
   - opposite pairs 591 to 623 of 959 under step = 1/16 (H-39 to H-41);
   - the count's standard deviation 74 to 82, and 72 to 78 with the gain held.
7. **The process** (ADR-0184, ADR-0191, ADR-0197, `briefs/README.md`):
   - eighteen weekly shards;
   - `scope=arms` jobs on AArch64, each test's log kept as `arm-<n>`;
   - `spec-brief archive <NNN> --strict` measured from `origin/main`;
   - `docs/STATUS.md` rewritten whole, the readings in H-42's report, and a short measured ADR.
8. **No sweep is unrecorded**: the last, run 39280026945's, ADR-0199 recorded. This round's own sweep is recorded by the first ADR that merges after it ends (ADR-0158).

## Deliverables

- [ ] **The probe, built in the engine, in a new ADR at the next free number (`ls docs/adr`), kind `built`.**
  - ADR-0200's flag at `[27]`, its 16 windows, its sum, its step and its refusals, the gain returned to g₀.
  - The image at format 25, its table in §5.2.
  - Unset, every run the run it was, bit for bit; a format-24 image read as unset.
  - Its tests:
    - by hand at the edges, the refusals among them;
    - over the lattice against an oracle;
    - an image written during the probe reading back to the same run;
    - the step set exactly at the probe's last window and not before.
  - The whole-image pins moved by the format's mask alone.
- [ ] **H-42's protocol, in a new ADR at the next free number, kind `protocol`, before any lead-in is run with the probe set.**
  - The networks, the read, the probe and λ.
  - What holds a derivation with the probe set, where none has a pin.
  - H-42's clauses and constants, restated from ADR-0200 and pinned in the tests.
  - The four predicted readings as rules over the tables.
- [ ] **The calibration**, before any such run: a dispatch at `scope=exhaustive` excluding this round's own tests. Every pinned number of the tree holds at format 25 with the flag unset, H-41's derivations and six arms among them.
- [ ] **The derivations with the probe set on the three networks**, each a weekly `exhaustive` test, run once by a dispatch at `scope=arms`. Pinned from that run:
  - the lead-in by window;
  - the read, the probe's D, S and κ;
  - what each rule returned, or that it returned nothing;
  - the frozen block's readings.
- [ ] **The arms**: on each network whose clause 0 holds, H-25's two, each a weekly `exhaustive` test, run once by a dispatch at `scope=arms`, their tables pinned per block.
- [ ] **The reproduction**, dispatched at `scope=both` as soon as the arms' tables are pinned. Its eighteen whole-domain shards are green and reproduce every pinned number, and the cost table is regenerated from them.
- [ ] **H-42's report**, `docs/research/H-42.md`: its `verdict:` set, the readings' tables, each network beside its twin under H-41, and the dispatches that are their evidence.
- [ ] **The measured ADR**, kind `measured`, short:
  - the verdict per network, arm and clause, in one table;
  - each of the four predicted readings, held or missed, in a line;
  - the step of the stopping rule reached, and the next decision it names, not taken.
- [ ] **The gate.** At most one runtime test: H-42's clauses and its predicted readings' rules at their edges, over tables written by hand.
- [ ] **The documents, in the same pull request.**
  - `docs/STATUS.md` rewritten whole.
  - Whitepaper: §5.2's modulator table at format 25, and the tables written by `node scripts/adr-index.mjs`.
  - `CHANGELOG.md`, one entry of at most 400 characters a change.
  - `README.md` and `docs/zh-TW/README.md` as the result requires.
  - The whitepaper's version in both declarations, with its date.
- [ ] **The brief archived** with `spec-brief archive 078 --strict`, every deliverable dispositioned. The round's ADRs are set to `accepted` by the commit that asks for the merge, and the pull request is marked ready for review with it.

## Not empowered

- **The round's scope** is the front matter's. It writes only inside `affectedFiles`, and never into `protectedFiles`. A file it must change outside the scope is handled as `briefs/README.md` says.
  - The rate's rule, `hold`, `move_gain` and the gain's bounds do not change; `crates/cortex-homeostasis/` is protected.
  - The read, its span and the inhibitory target by it are H-41's.
  - `Config::reads_its_targets()` and `Config::default()` do not change.
- **λ = 1/4, the probe's ±1/64, its 4 + 8 + 4 windows and the networks are ADR-0200's.** None is fitted to the result, and none other is run.
- No rule that derives a constant is changed so that a network's read, probe or settling holds.
- H-42's clauses, its constants and its predicted readings do not move after such a run. There is no second attempt, and a run is not repeated for a check of the harness.
- No whole-domain test runs on the developer machine.
- No pinned number of an earlier round moves, but the whole-image pins by the format's mask.
- No float anywhere, no dependency, no `unsafe`, and no rebase of the branch once a dispatch has run on it.

## Architectural empowerment

The executing session may replace any instruction above with a better decision, recorded in its ADR. That covers:
- the flag's name in `Config`, how the probe's state packs into the read's bytes, and where the executor starts and ends it;
- the fixed-point form of g₀·(1 ± 1/64), of the return to g₀ and of κ, each rounding stated and tested;
- the error a refused flag or probe raises, and where;
- how the harness sets the flag, and how a derivation and an arm carry it, every call of the tree the call it was;
- how a derivation with the probe set on a network with no pin is held: what is asserted by rule, and what is pinned from the one run;
- how the three derivations and six arms are dealt into tests and dispatches;
- how each reading is computed and pinned, the opposite pairs by `opposite_pairs`;
- a file the round must change and its scope does not hold, within this brief's mission, widened as `briefs/README.md` says.

It may not:
- write into a protected file, or shrink `protectedFiles`;
- change λ, the probe's size or windows, the read, the step's bound, a network, the task or the schedule;
- move a clause, a constant or a predicted reading after such a run;
- run a whole-domain test on the developer machine;
- reach the standing directives, the whitepaper's invariants or the constraints in `CLAUDE.md`.

If the calibration fails, the round stops there as H-42's stopping rule says: the finding is written, and no such run is made.

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
gh workflow run ci.yml --ref <this round's branch> -f scope=arms -f tests='<the derivations, then the arms>'
gh workflow run ci.yml --ref <this round's branch> -f scope=both
node scripts/exhaustive-costs.mjs from <the reproduction's downloaded artifacts> --run <its id> > scripts/exhaustive-costs.tsv
node scripts/adr-index.mjs
npx --no-install @descent-vtt/spec-brief archive 078 --strict --pr <n> --summary "<one paragraph>"
```

- The calibration's shards are green with the round's tests excluded.
- Each run's job exits 0 with one test passed and its log kept.
- The reproduction's eighteen whole-domain shards are green and reproduce every pinned number.
- In the history, H-42's constants and its predicted readings precede every dispatch that runs a lead-in with the probe set, and every commit a dispatch ran on is an ancestor of the pull request's head.
- The archive passes `--strict`: nothing outside `affectedFiles` and nothing in `protectedFiles` changed.

## Report

The closing message states:
- the probe as built, the format, and that every run with it unset is the run it was;
- the protocol, and when it was committed;
- per network, the probe's D, S and κ, and the lead-in's settling beside H-41's;
- H-42's verdict per network, arm and clause, and the prediction and each predicted reading against what was read;
- each network's opposite pairs and gain band beside its twin under H-41;
- the step of the stopping rule reached and the next decision it names, not taken;
- each dispatch with its run id and commit: the calibration, the runs and the reproduction;
- any widening of `affectedFiles`, and why;
- that no whole-domain test ran on the developer machine, that each run ran once, and that no pinned number moved but by the format's mask;
- each shard's job time against its bound and the regenerated cost table;
- that the pull request is ready for review;
- what was not done and why.
