---
status: proposed
date: 2026-10-21
affectedFiles: [runtime/cortex-runtime/src/**, runtime/cortex-runtime/tests/**, docs/**, briefs/**, CHANGELOG.md, CLAUDE.md, README.md, scripts/exhaustive-costs.tsv]
protectedFiles: [crates/**, benches/**, .github/workflows/**, scripts/arm-run.sh, scripts/exhaustive-shard.sh, scripts/exhaustive-costs.mjs, scripts/check-archive.mjs, scripts/check-scope.mjs, scripts/check-explorations.mjs, scripts/check-doc-shape.mjs, scripts/adr-index.mjs, .spec-brief.json, package.json, package-lock.json, rust-toolchain.toml, Cargo.toml, Cargo.lock]
---

# Brief 079: The probe in the engine's default

## Mission

**This brief changes one value of the engine's default and runs nothing new.**
H-42 read yes ([ADR-0204](../docs/adr/0204-a-step-the-engine-makes-from-its-own-loop-measured.md)).
[ADR-0205](../docs/adr/0205-the-probe-in-the-engines-default.md) puts the probe in `Config::reads_its_targets()`, so the step the engine makes from its own loop replaces step = 1/16 in the default.

When the round is done, the tree holds:
- `Config::reads_its_targets()` with `step_probe: true`, its other five values unchanged;
- the default's field-by-field test with the probe set, and its byte-for-byte test against H-42's candidate on seeds 22, 23 and 24 at 1 024 units;
- one dispatch at `scope=both` whose eighteen whole-domain shards are green, every other pin held.

**There is no hypothesis.** Nothing is run under the default that H-42 did not run.

The scope is the front matter's ([ADR-0191](../docs/adr/0191-a-rounds-scope-eighteen-shards-and-the-reproduction-first.md)): `spec-brief archive --strict` refuses the round outside it.

**Do not run this round beside brief 065's**: both dispatch the weekly workflow.

---

## Standing directives

- **Latest ≠ Newest** (whitepaper §2.1, `CLAUDE.md` principle 4). The round depends on nothing new: no crate, no tool and no version bump.
- **The default is ADR-0205's**: `Config::reads_its_targets()` gains `step_probe: true` and nothing else. The step stays 0x1000.
- **`Config::default()` builds the engine it built**, and no configuration of the tree but the default changes.
- **No pinned number of the tree moves but the default's two tests**: its field-by-field test, and its byte-for-byte test, which moves from H-41's candidate to H-42's.
- **No derivation, lead-in or arm is run.** The one dispatch is the round's evidence that nothing else moved.
- **The dispatch is made once the last commit under `src/` and `tests/` is pushed**, at `scope=both`, since a file under `src/` changes. A change under either after it is dispatched again, or shown in the ADR to change no run. No whole-domain test runs on the developer machine.
- **The branch is named `claude/brief-079-…`**, never rebased or force-pushed once a dispatch has run on it. Every commit is signed.
- **The engine and the harness are read before a description of them is trusted**, this brief's and ADR-0205's included.
- No `f32`/`f64`, oracles included; every operation on a state field saturates or wraps by name ([ADR-0029](../docs/adr/0029-structural-enforcement.md)). Every loop ends by construction ([ADR-0062](../docs/adr/0062-the-first-complete-sweeps-list.md)). No `unsafe`.
- The runtime's gate grows by no test: the default's byte-for-byte test is moved, not added ([ADR-0061](../docs/adr/0061-the-learning-runs-leave-the-gate.md)). The pull request's mutation gate catches every mutant in the lines it changes ([ADR-0030](../docs/adr/0030-verification-governance.md)).
- **The documents are ADR-0197's and ADR-0201's**: a new ADR in their shape, the tables written by `node scripts/adr-index.mjs`, new text within `npm run spec:shape`'s limits and its math GitHub's.
- **The pull request leaves draft with the commit that asks for the merge**, which sets the round's ADR to `accepted`.
- **No shard of the weekly job passes 60 per cent of its bound**, 180 minutes, at eighteen shards ([ADR-0179](../docs/adr/0179-the-bounds-rule-restated.md), ADR-0191). One that does is reported with its job's minutes.
- Conventional Commits with a real body; never commit on `main`; the required checks keep their names.

## Context

Re-derived on 2026-10-21 against brief 078's head, `9ac4475`. Line numbers move; the symbols and the quoted sentences are what to re-derive.

1. **The default** (`runtime/cortex-runtime/src/executor.rs`):
   - `Config::reads_its_targets()` is `Config::default()` with `modulation_baseline_q16` 0x8000, `synaptic_gain_q16` 0x0001_C000, `control_step_q0_16` 0x1000, `rate_read_windows` 128 and `istdp_target_read` true.
   - Its test `reads_its_targets_is_the_default_with_adr_0196_s_five_values_and_no_other` holds it field by field.
2. **The probe** (ADR-0202): `Config::step_probe`, refused without a read. While the read and the probe run, the gain is held. At the probe's end the step is written, and a refused probe ends the read refused.
3. **The tests** (`runtime/cortex-runtime/tests/inhibition.rs`):
   - `the_engine_s_default_writes_h_41_s_candidate_byte_for_byte_on_its_three_networks` builds the default at H-41's sizing and compares its image with `candidate_under(…, &UNDER_BOTH_TARGETS)`'s.
   - `UNDER_THE_PROBE` is H-42's settling, and `PROBE_SEEDS` and `PROBE_RULES` its networks and their pinned rules.
4. **H-42's readings** (ADR-0204): S 15.02 to 15.19, step 1 078 to 1 091, opposite pairs 485 to 527 of 959, the count 0.13 to 0.15 % above its target, the gain within 1.7032 and 1.7408.
5. **The process** (ADR-0184, ADR-0191, ADR-0197, ADR-0201, `briefs/README.md`): eighteen weekly shards; `spec-brief archive <NNN> --strict` measured from `origin/main`; `docs/STATUS.md` rewritten whole.
6. **The sweep**: brief 078's reproduction, run 39376979724, ended before any ADR merged, and ADR-0204 records it. This round's own sweep is recorded by the first ADR that merges after it ends (ADR-0158).

## Deliverables

- [ ] **The default, built, in a new ADR at the next free number (`ls docs/adr`), kind `built`.**
  - `Config::reads_its_targets()` with `step_probe: true`.
  - The field-by-field test with the probe set.
  - The byte-for-byte test moved to H-42's candidate: on each of seeds 22, 23 and 24 at 1 024 units, an engine born under the default with H-42's sizing writes the image `candidate_under(…, &UNDER_THE_PROBE)` writes, at the gain `PROBE_RULES` pinned.
- [ ] **The dispatch**, at `scope=both`, once the last commit under `src/` and `tests/` is pushed. Its eighteen whole-domain shards are green and reproduce every pinned number, and the cost table is regenerated from it.
- [ ] **The ADR's reading.**
  - What was built.
  - The byte-for-byte check per network.
  - The dispatch with its run id and commit, each shard's job time against its bound, and whether its sweep had ended.
- [ ] **The documents, in the same pull request.**
  - `docs/STATUS.md` rewritten whole.
  - Whitepaper: where the default is described, the probe in it; and the tables written by `node scripts/adr-index.mjs`.
  - `CHANGELOG.md`, one entry of at most 400 characters.
  - `README.md` and `docs/zh-TW/README.md` as the result requires.
  - The whitepaper's version in both declarations, with its date.
- [ ] **The brief archived** with `spec-brief archive 079 --strict`, every deliverable dispositioned. The round's ADR is set to `accepted` by the commit that asks for the merge, and the pull request is marked ready for review with it.

## Not empowered

- **The round's scope** is the front matter's. It writes only inside `affectedFiles`, and never into `protectedFiles`. A file it must change outside the scope is handled as `briefs/README.md` says.
- **No record, section or format changes**, and no state crate changes.
- **The default gains `step_probe: true` and nothing else**; `Config::default()` builds the engine it built.
- **No pinned number moves but the default's two tests.**
- **No derivation, lead-in or arm is run**, and no whole-domain test runs on the developer machine.
- No float anywhere, no dependency, no `unsafe`, and no rebase of the branch once a dispatch has run on it.

## Architectural empowerment

The executing session may replace any instruction above with a better decision, recorded in its ADR. That covers:
- how the byte-for-byte test is moved, and whether H-41's comparison is kept beside it with the probe unset;
- the test names, every pin of the tree but the default's two unchanged;
- a file the round must change and its scope does not hold, within this brief's mission, widened as `briefs/README.md` says.

It may not:
- write into a protected file, or shrink `protectedFiles`;
- change a value of the default but `step_probe`, or what `Config::default()` builds;
- run a derivation, a lead-in or an arm, or any whole-domain test on the developer machine;
- reach the standing directives, the whitepaper's invariants or the constraints in `CLAUDE.md`.

If the dispatch moves a pinned number but the default's two tests, the round stops there: the finding is written, and the default is not merged until it is understood.

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
gh workflow run ci.yml --ref <this round's branch> -f scope=both
node scripts/exhaustive-costs.mjs from <the dispatch's downloaded artifacts> --run <its id> > scripts/exhaustive-costs.tsv
node scripts/adr-index.mjs
npx --no-install @descent-vtt/spec-brief archive 079 --strict --pr <n> --summary "<one paragraph>"
```

- The dispatch's eighteen whole-domain shards are green and reproduce every pinned number.
- Every commit the dispatch ran on is an ancestor of the pull request's head, and no commit under `src/` or `tests/` follows it unless a later dispatch, or the ADR, covers it.
- The archive passes `--strict`: nothing outside `affectedFiles` and nothing in `protectedFiles` changed.

## Report

The closing message states:
- the default as built, and that `Config::default()` builds the engine it built;
- the byte-for-byte check per network;
- the dispatch with its run id and commit, and that every pinned number but the default's two held;
- each shard's job time against its bound and the regenerated cost table;
- any widening of `affectedFiles`, and why;
- that no whole-domain test ran on the developer machine;
- that the pull request is ready for review;
- what was not done and why.
