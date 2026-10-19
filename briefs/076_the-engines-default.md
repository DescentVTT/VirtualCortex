---
status: proposed
date: 2026-10-19
affectedFiles: [runtime/cortex-runtime/src/**, runtime/cortex-runtime/tests/**, docs/**, briefs/**, CHANGELOG.md, CLAUDE.md, README.md, scripts/exhaustive-costs.tsv]
protectedFiles: [crates/**, benches/**, .github/workflows/**, scripts/arm-run.sh, scripts/exhaustive-shard.sh, scripts/exhaustive-costs.mjs, scripts/check-archive.mjs, scripts/check-scope.mjs, scripts/check-explorations.mjs, .spec-brief.json, rust-toolchain.toml, Cargo.toml, Cargo.lock]
---

# Brief 076: The engine's default — ADR-0196's decision built: the gain an engine starts with as a field of the configuration, `Config::reads_its_targets()` with H-41's five values and `Config::default()` the bare engine it was; the default shown byte for byte H-41's candidate on seeds 22, 23 and 24 at 1 024 units, and every pin held by one dispatch; no hypothesis, no derivation and no arm run

## Mission

**This brief builds two things in the engine and runs nothing new.** H-41 read that the engine can read both of its
network's targets ([ADR-0195](../docs/adr/0195-the-inhibitory-target-by-the-same-read-measured.md)).
[ADR-0196](../docs/adr/0196-the-engines-default.md) makes what H-41 measured of the network's own regime the
engine's default for an image no derivation has settled. It does so in a named configuration, leaving
`Config::default()` the bare engine that every pin runs.

When the round is done, the tree holds:
- `Config::synaptic_gain_q16`, the gain `Executor::new` starts an engine at, default 1.0, refused outside 0.25 to 4,
  the image's outranking it;
- `Config::reads_its_targets()`, `Config::default()` with ADR-0196's five values;
- a test that an engine born under it writes, on each of seeds 22, 23 and 24 at 1 024 units, the image H-41's
  candidate writes before its lead-in;
- one dispatch at `scope=both` whose eighteen whole-domain shards are green, every pinned number held.

**There is no hypothesis.** Nothing is run under the default that H-41 did not run, so there is nothing to predict and
no stopping rule to reach.

The scope is the front matter's ([ADR-0191](../docs/adr/0191-a-rounds-scope-eighteen-shards-and-the-reproduction-first.md)):
`spec-brief archive --strict` refuses the round outside it. `briefs/README.md` says what to do with a file the scope
does not hold.

**Do not run this round beside brief 065's**: both dispatch the weekly workflow.

---

## Standing directives

- **Latest ≠ Newest** (whitepaper §2.1, `CLAUDE.md` principle 4). The round depends on nothing new: no crate, no
  tool and no version bump. The default gathers the engine's own rules under one name.
- **The default is ADR-0196's.**
  - It is `Config::default()` with these values and no other:

    | Field | Value |
    | :--- | :--- |
    | `modulation_baseline_q16` | `0x8000` |
    | `synaptic_gain_q16` | `0x0001_C000` |
    | `control_step_q0_16` | `0x1000` |
    | `rate_read_windows` | 128 |
    | `istdp_target_read` | set |

  - The sizing is the caller's, and the learning configuration stays unset in it.
  - None of the five is fitted, and none other is added.
- **The gain's field is ADR-0196's.**
  - Its default is `GAIN_ONE_Q16`, so `Config::default()` builds the engine it built.
  - It is refused outside `GAIN_MIN_Q16` to `GAIN_MAX_Q16`.
  - For an engine built from an image, the image's gain outranks it.
  - No record and no section changes, and the image stays at format 24.
- **No pinned number of the tree moves**, not even by a mask. T-1 is spelled with the gain's field at its default,
  and its pin holds.
- **No derivation, lead-in or arm is run.** The one dispatch is the round's evidence that nothing moved.
- **The dispatch is made once the last commit under `src/` and `tests/` is pushed**, at `scope=both`, since a file
  under `src/` changes. A change under either after it is dispatched again, or shown in the ADR to change no run. No
  whole-domain test runs on the developer machine.
- **The branch is named `claude/brief-076-…`**, never rebased or force-pushed once a dispatch has run on it. Every
  commit is signed.
- **The engine and the harness are read before a description of them is trusted**, this brief's and ADR-0196's
  included.
- No `f32`/`f64`, oracles included; every operation on a state field saturates or wraps by name
  ([ADR-0029](../docs/adr/0029-structural-enforcement.md)). Every loop ends by construction
  ([ADR-0062](../docs/adr/0062-the-first-complete-sweeps-list.md)). No `unsafe`.
- The runtime's gate grows by at most one test ([ADR-0061](../docs/adr/0061-the-learning-runs-leave-the-gate.md)): the
  byte-for-byte check, if it fits the gate's time, or else an `#[ignore]`d test whose name contains `exhaustive`. A new
  refusal carries a test over the lattice of `testkit/prop.rs`. The pull request's mutation gate catches every mutant
  in the lines it changes ([ADR-0030](../docs/adr/0030-verification-governance.md)).
- **The pull request leaves draft with the commit that asks for the merge**, which sets the round's ADR to `accepted`.
- **No shard of the weekly job passes 60 per cent of its bound**, 180 minutes, at eighteen shards
  ([ADR-0179](../docs/adr/0179-the-bounds-rule-restated.md), ADR-0191). One that does is reported with its job's
  minutes.
- Conventional Commits with a real body; never commit on `main`; the required checks keep their names.

## Context

Re-derived on 2026-10-19 against brief 075's head, `c1b8d55`. Line numbers move; the symbols and the quoted sentences
are what to re-derive.

1. **The configuration** (`runtime/cortex-runtime/src/executor.rs`):
   - `Config` derives `Clone, Debug, PartialEq, Eq`.
   - Its `Default` sets the step 0, `rate_target_period_ticks` 0, `rate_read_windows` 0, `istdp_target_read` false,
     `istdp_target_period_ticks` `ISTDP_TARGET_PERIOD_TICKS` (20 000) and `modulation_baseline_q16`
     `MODULATION_ONE_Q16`.
   - `Executor::new` validates each field and returns a `ConfigError`, among them `ControlStepOutOfRange`,
     `RateReadOutOfRange`, `RateReadBesideTarget` and `IstdpReadWithoutRead`.
   - It builds the homeostasis record as `HomeostaticDrivePool { control_step_q0_16, sleep_shift,
     ..HomeostaticDrivePool::new() }`, so the gain starts at `GAIN_ONE_Q16`.
2. **The gain** (`crates/cortex-homeostasis/src/lib.rs`, protected):
   - `GAIN_ONE_Q16`, `GAIN_MIN_Q16` (0.25) and `GAIN_MAX_Q16` (4).
   - The record's `synaptic_gain_q16` sits at the homeostasis section's `[28..32)`.
   - Only an image sets it today: `Executor::set_homeostasis`, from `Image::decode`.
3. **The image** (`runtime/cortex-runtime/src/image.rs`):
   - `Image::decode` forces the configuration's read and flag off, then takes the read, the flag, the step, the gain
     and both periods from the image's bytes.
   - `FORMAT_VERSION` is 24.
4. **The harness** (`runtime/cortex-runtime/tests/instrument/harness.rs`, `tests/inhibition.rs`):
   - `config(units, workers, baseline)` builds on `..Config::default()`.
   - `at_gain` synthesizes the prior into an engine built from a configuration, then reloads it with the gain
     patched into its homeostasis record.
   - `GAINS` is `[0x0001_C000, 0x0002_0000]`, and `BASELINE_Q16` is `0x8000`.
   - `candidate_under(p, units, gain, settling)` builds H-41's candidate from `config(units, 2, BASELINE_Q16)`, the
     settling's step, read and flag, at the calibrated gain. `UNDER_BOTH_TARGETS` is that settling.
5. **T-1** (`runtime/cortex-runtime/tests/differential.rs`,
   `the_random_network_hashes_to_the_pinned_value_on_every_architecture`) spells every field of the configuration out.
6. **The calibration** took its first candidate, 1.75, on every network derived: seeds 22, 23 and 24 at 1 024 units,
   and seeds 22 and 23 at 2 048 units ([ADR-0167](../docs/adr/0167-another-network-of-the-same-size-measured.md),
   [ADR-0169](../docs/adr/0169-twice-the-size-measured.md)).
7. **The process** (ADR-0184, ADR-0191, `briefs/README.md`): eighteen weekly shards; `spec-brief archive <NNN>
   --strict` measured from `origin/main`; `npm run spec:scope` prints each widening of `affectedFiles`.
8. **The sweep**: brief 075's reproduction, run 39202114129, was dispatched at `scope=both`. Its sweep is recorded by
   the first ADR that merges after it ends (ADR-0158). If that ADR is this round's, it records it.

## Deliverables

- [ ] **The gain's field and the default, built in the engine, in a new ADR at the next free number (`ls docs/adr`).**
  - `Config::synaptic_gain_q16`, default `GAIN_ONE_Q16`. A new `ConfigError` refuses it outside the controller's
    bounds, and `Executor::new` starts the homeostasis record's gain at it.
  - For an engine built from an image, the image's gain outranks it.
  - `Config::reads_its_targets()`, with ADR-0196's five values.
  - Its tests:
    - the refusal at its edges by hand, and over the lattice;
    - `Config::default()` building the engine it built, the gain 1.0;
    - `reads_its_targets()` field by field against `Config::default()`;
    - an engine built from it reading (128 windows left, the sum 0), at 1.75, its flag set, its image read back to
      the same engine;
    - an image's gain outranking the configuration's.
- [ ] **H-41's candidate, byte for byte.** On each of seeds 22, 23 and 24 at 1 024 units, the image of an engine
  built from `reads_its_targets()` with H-41's sizing, its arenas synthesized from the network's prior, equals the
  image of H-41's candidate before its lead-in.
- [ ] **The dispatch**, at `scope=both`, once the last commit under `src/` and `tests/` is pushed. Its eighteen
  whole-domain shards are green and reproduce every pinned number, and the cost table is regenerated from it.
- [ ] **The ADR's reading.**
  - What was built, and that `Config::default()` builds the engine it built.
  - The byte-for-byte check per network.
  - The dispatch with its run id and commit, each shard's job time against its bound, and whether the sweep had
    ended.
  - Brief 075's sweep, if this ADR is the first to merge after it ends.
- [ ] **The documents, in the same pull request.**
  - Whitepaper: §8.8's rate row naming the default as Implemented; where the configuration's fields are described,
    the gain's; and §9.
  - The ADR index and `CHANGELOG.md`.
  - `CLAUDE.md` where the standing changes: what the engine carries by default, and *"a target rate chosen for the
    engine's default"* among what does not exist.
  - `README.md` and `docs/zh-TW`'s reader's guide as the result requires.
  - The whitepaper's version in both declarations, with its date.
- [ ] **The brief archived** with `spec-brief archive 076 --strict`, every deliverable dispositioned. The round's ADR
  is set to `accepted` by the commit that asks for the merge, and the pull request is marked ready for review with it.

## Not empowered

- **The round's scope** is the front matter's. It writes only inside `affectedFiles`, and never into
  `protectedFiles`. A file it must change outside the scope is handled as `briefs/README.md` says: widened in a
  commit of its own when within this brief's empowerment, stopped at when protected or past it.
- **No record, section or format changes**, and no state crate changes.
- **`Config::default()` builds the engine it built.** No pinned number moves, not even by a mask.
- **The default's values are ADR-0196's.** None is fitted, none other is added, and the learning configuration is
  not part of it.
- **No derivation, lead-in or arm is run**, and no whole-domain test runs on the developer machine.
- No float anywhere, no dependency, no `unsafe`, and no rebase of the branch once a dispatch has run on it.

## Architectural empowerment

The executing session may replace any instruction above with a better decision, recorded in its ADR. That covers:
- the names of the gain's field, of the default and of the new error, and where the refusal is made;
- whether `reads_its_targets()` is a `const fn`;
- how the byte-for-byte check is built, and whether H-41's `candidate_under` builds its configuration from
  `reads_its_targets()`, every pin of the tree unchanged;
- a file the round must change and its scope does not hold, within this brief's mission, widened as
  `briefs/README.md` says.

It may not:
- write into a protected file, or shrink `protectedFiles`;
- change a value of the default, add one, or put the learning configuration in it;
- change what `Config::default()` builds;
- run a derivation, a lead-in or an arm, or any whole-domain test on the developer machine;
- reach the standing directives, the whitepaper's invariants or the constraints in `CLAUDE.md`.

If the dispatch moves a pinned number, the round stops there: the finding is written, and the default is not merged
until it is understood.

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

The pull request's gate runs those and the rest: the release tests, the benchmarks once, the MSRV, AArch64 and the
mutation gate on the changed lines. Every check passes on the last commit.

By dispatch, never on the developer machine:

```bash
gh workflow run ci.yml --ref <this round's branch> -f scope=both
node scripts/exhaustive-costs.mjs from <the dispatch's downloaded artifacts> --run <its id> > scripts/exhaustive-costs.tsv
npx --no-install @descent-vtt/spec-brief archive 076 --strict --pr <n> --summary "<one paragraph>"
```

- The dispatch's eighteen whole-domain shards are green and reproduce every pinned number.
- Every commit the dispatch ran on is an ancestor of the pull request's head, and no commit under `src/` or `tests/`
  follows it unless a later dispatch, or the ADR, covers it.
- The archive passes `--strict`: nothing outside `affectedFiles` and nothing in `protectedFiles` changed.

## Report

The closing message states:
- the gain's field and the default as built, and that `Config::default()` builds the engine it built;
- the byte-for-byte check per network;
- the dispatch with its run id and commit, and that every pinned number held;
- each shard's job time against its bound and the regenerated cost table;
- brief 075's sweep, if this round's ADR recorded it;
- any widening of `affectedFiles`, and why;
- that no whole-domain test ran on the developer machine;
- that the pull request is ready for review;
- what was not done and why.
