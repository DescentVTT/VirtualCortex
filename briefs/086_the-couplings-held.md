---
status: proposed
date: 2026-11-01
affectedFiles: [runtime/menteion-runtime/src/**, runtime/menteion-runtime/tests/**, testkit/**, docs/**, briefs/**, CHANGELOG.md, CLAUDE.md, README.md, scripts/exhaustive-costs.tsv]
protectedFiles: [crates/**, docs/explorations/0*.md, docs/benchmarks/results/**, benches/**, .github/workflows/**, .cargo/**, Cargo.toml, Cargo.lock, rust-toolchain.toml, clippy.toml, .spec-brief.json, package.json, package-lock.json, scripts/arm-run.sh, scripts/exhaustive-shard.sh, scripts/exhaustive-costs.mjs, scripts/check-additive.mjs, scripts/check-archive.mjs, scripts/check-scope.mjs, scripts/check-explorations.mjs, scripts/check-doc-shape.mjs, scripts/adr-index.mjs]
---

# Brief 086: The couplings held in a running configuration

<!-- @assert-absence target="runtime/menteion-runtime/src/executor.rs" symbol="fn holds_its_couplings" reason="brief 086 precondition: main holds no running configuration; the round that builds it archives this brief in the same pull request" -->

## Mission

**This brief puts what H-46 measured under one name and runs nothing new.**
H-46 read yes ([ADR-0230](../docs/adr/0230-a-zero-sum-at-the-stimulus-measured.md)).
[ADR-0232](../docs/adr/0232-the-couplings-held-in-a-running-configuration.md) puts its two rules in a configuration an engine runs a settled image under, and leaves the derivation H-42's.

When the round is done, the tree holds:
- `Config::holds_its_couplings()`, `Config::reads_its_targets()` with a quarter's exponent, and H-46's arm running under it;
- an exponent other than one refused beside a read, and refused on an image with a read or a probe under way;
- `Executor::coupling_sum`, the reading a host keeps each group's value from;
- one dispatch at `scope=exhaustive` whose eighteen shards are green, every pin held.

**There is no hypothesis.** Nothing runs under the name that H-46's arms did not run.

It writes a built ADR and rewrites `docs/STATUS.md` ([briefs/README.md](README.md#a-rounds-documents)).
The scope is the front matter's ([ADR-0191](../docs/adr/0191-a-rounds-scope-eighteen-shards-and-the-reproduction-first.md)).

**Do not run this round beside brief 065's**: both dispatch the weekly workflow.

---

## Standing directives

- **Latest ≠ Newest** (whitepaper §2.1, `CLAUDE.md` principle 4). The round depends on nothing new: no crate, no tool, no version of either.
- **The configuration is ADR-0232's**: `Config::holds_its_couplings()` is `Config::reads_its_targets()` with `depression_exponent` at `DepressionExponent::Quarter`, and no other field.
- **`Config::default()` and `Config::reads_its_targets()` build the engines they built.** No configuration of the tree but the new one changes, and every image is born as it was.
- **No pinned number of the tree moves.** A pin that moves stops the round as a finding, and the configuration is not merged until it is understood.
- **No derivation, lead-in or arm is run.** The one dispatch is the round's evidence that nothing moved. No whole-domain test runs on the developer machine.
- **The dispatch is made once the last commit under `src/` and `tests/` is pushed**, at `scope=exhaustive` with nothing excluded, since a file under `src/` changes. A change under either after it is dispatched again, or shown in the ADR to change no run.
- **No record, section or format changes.** The format stays 26. No state crate changes.
- **No `unsafe` beyond ADR-0023's.** No `f32`/`f64`; every operation on a state field saturates or wraps by name ([ADR-0029](../docs/adr/0029-structural-enforcement.md)); every loop ends by construction ([ADR-0062](../docs/adr/0062-the-first-complete-sweeps-list.md)).
- **The mutation gate catches every mutant in the lines the change touches** ([ADR-0030](../docs/adr/0030-verification-governance.md)); a new rule carries a test over the lattice of `testkit/prop.rs`.
- **The pull request opens as a draft with the round's first push** ([ADR-0223](../docs/adr/0223-the-gate-shortened-and-explorations-kept-as-tags.md)), and leaves draft with the commit that asks for the merge, which sets the round's ADR to `accepted`.
- **The branch is named `claude/brief-086-…`**, never rebased or force-pushed once a dispatch has run on it. Every commit is signed.
- **The engine and the harness are read before a description of them is trusted**, this brief's and ADR-0232's included.
- **The documents are ADR-0197's**: a new ADR in its shape, the tables written by `node scripts/adr-index.mjs`, new text within `npm run spec:shape`'s limits.
- **No shard of the weekly job passes 60 per cent of its bound**, 240 minutes ([ADR-0179](../docs/adr/0179-the-bounds-rule-restated.md)). One that does is reported with its job's minutes.
- **GitHub without gh.** Dispatches, the pull request, logs and artifacts go through the REST API with the repository's own token (`~/.claude/menteion-tools/ghapi.sh` and its README); `git push` uses the same token. The token is never printed or put on a command line.
- **The maintainer is notified** as `briefs/README.md` says: when a question only they can answer stops the round, and when the pull request is ready for review.
- Conventional Commits with a real body; never commit on `main`.

## Context

Re-derived on 2026-11-01 against `main` at `eacc1fd`. Line numbers move; the symbols are what to re-derive.

1. **The configurations** (`runtime/menteion-runtime/src/executor.rs`):
   - `Config::reads_its_targets()` is `Config::default()` with `modulation_baseline_q16` 0x8000, `synaptic_gain_q16` 0x0001_C000, `control_step_q0_16` 0x1000, `rate_read_windows` 128, `istdp_target_read` and `step_probe`.
   - Its test `reads_its_targets_is_the_default_with_adr_0196_s_five_values_the_probe_and_no_other` holds it field by field.
   - `Config::depression_exponent` defaults to `DepressionExponent::One`, and `Executor::depression_exponent` reads it.
2. **The decode** (`runtime/menteion-runtime/src/image.rs`): `Image::decode` sets the read, its flag and the probe to their unset values in the `Config` it builds on, then takes them, the baseline, the gain and the step from the image's bytes. The exponent is not in the image, so a decoded engine's is its configuration's.
3. **H-46's harness** (`runtime/menteion-runtime/tests/inhibition.rs`):
   - `zero_from(image, units, form)` decodes under `Config { kept_addresses: KEPT_ADDRESSES, depression_exponent: exponent, ..config(units, 2, 0) }`, the exponent `ZERO_EXPONENT` under `ZERO_ARM`.
   - `zero_groups(made, name)` reads each group's value with `couplings`, and asserts it equal to `made.couplings`.
   - `zero_schedule` calls `exec.set_zero_sum(&held, &groups.targets)` on each schedule's engine of an arm.
   - `zero_short` is the six `…_exhaustive` short forms' body, and `the_zero_sum_of_h_46_its_clauses_and_its_predicted_readings_rules` reads the pins of `tests/inhibition/pins/h46.rs`.
4. **The zero sum** (`Executor::set_zero_sum`, `hold_zero_sum`, `zero_sum_round`): `zero_sum_round` walks a group's chains and sums its synapses onto the targets. `Executor::zero_sum_groups` counts the groups held.
5. **The errors** (`ConfigError` and `ImageError::Config`): `Executor::new` returns a `ConfigError` for each refused field. `Image::decode` wraps one in `ImageError::Config`.
6. **The process**: eighteen weekly shards ([ADR-0191](../docs/adr/0191-a-rounds-scope-eighteen-shards-and-the-reproduction-first.md)), the bound 240 minutes; `spec-brief archive --strict` measured from `origin/main`. The tools are `@spec-family/…` ([ADR-0231](../docs/adr/0231-the-spec-tools-under-their-new-scope.md)).
7. **The sweep**: the schedule runs on 2026-11-04 at 20:17Z. If it has ended when this round's ADR merges, that ADR records it ([briefs/README.md](README.md#evidencing-a-round)).

## Deliverables

- [ ] **The configuration, the boundary and the reader, built, in a new ADR at the next free number (`ls docs/adr`), kind `built`.**
  - `Config::holds_its_couplings()`, and its test field by field against `Config::reads_its_targets()`.
  - `Executor::new` refusing an exponent other than one beside a read, by a `ConfigError` of its own.
  - `Image::decode` refusing, with the same error, an image with a read or a probe under way under such an exponent.
  - `Executor::coupling_sum(sources, targets)`, read between ticks.
  - A test over `testkit/prop.rs`'s lattice for each refusal, and one holding `coupling_sum` to a sum by hand.
- [ ] **H-46's harness under the name.**
  - `zero_from` takes an arm's exponent from `Config::holds_its_couplings()`.
  - `zero_groups` reads each value through `coupling_sum`, its assertion against `made.couplings` kept.
  - The six short forms and the verdict's test unchanged, each pin held.
- [ ] **The dispatch**, at `scope=exhaustive` with nothing excluded, once the last commit under `src/` and `tests/` is pushed. Its eighteen shards are green, and the cost table is regenerated from it.
- [ ] **The ADR's reading.**
  - What was built, and the two refusals.
  - The dispatch with its run id and commit, and each shard's job time against its bound.
  - Whether the schedule's sweep of 2026-11-04 has ended, and its record if it has.
- [ ] **The documents, in the same pull request.**
  - `docs/STATUS.md` rewritten whole.
  - Whitepaper: the running configuration beside the engine's default in §8, with an `@assert-count` for the new name; the depression's exponent's paragraph no longer "unset by default" where the name sets it; the tables written by `node scripts/adr-index.mjs`.
  - `CHANGELOG.md`, one entry of at most 400 characters.
  - The whitepaper's version in both declarations, with its date.
- [ ] **The brief archived** with `spec-brief archive 086 --strict`, every deliverable dispositioned. The round's ADR is set to `accepted` by the commit that asks for the merge, and the pull request is marked ready for review with it.

## Not empowered

- **The round's scope** is the front matter's. It writes only inside `affectedFiles`, and never into `protectedFiles`. A file it must change outside the scope is handled as `briefs/README.md` says.
- **No record, section or format changes**, and no state crate changes.
- **No value of `Config::default()` or `Config::reads_its_targets()` changes.**
- **`Config::holds_its_couplings()` sets the exponent and nothing else.** The kept addresses, the delay read and the sleep stay unset in it.
- **The zero sum's groups enter no configuration and no image.**
- **No pinned number moves**, and no constant of H-46's.
- **No derivation, lead-in or arm is run**, and no whole-domain test runs on the developer machine.
- No float anywhere, no dependency, no `unsafe` beyond ADR-0023's, and no rebase of the branch once a dispatch has run on it.

## Architectural empowerment

The executing session may replace any instruction above with a better decision, recorded in its ADR. That covers:
- the names of the configuration, the error and the reader;
- whether `coupling_sum` takes its targets as a list or as `zero_sum_round`'s flags, and whether the two share their walk;
- how `zero_from` reaches the name, every pin held;
- where in `Image::decode` the second refusal sits;
- a file the round must change and its scope does not hold, within this brief's mission, widened as `briefs/README.md` says.

It may not:
- write into a protected file, or shrink `protectedFiles`;
- change a value of the default, or what `Config::default()` builds;
- let an engine be born, or a read or a probe end, under an exponent other than one;
- run a derivation, a lead-in or an arm, or any whole-domain test on the developer machine;
- reach the standing directives, the whitepaper's invariants or the constraints in `CLAUDE.md`.

If a decoded image H-46 ran is refused, or the dispatch moves a pinned number, the round stops there: the finding is written, and the configuration is not merged until it is understood.

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

By dispatch, never on the developer machine. A `ci.yml` dispatch is `POST /repos/Menteion/Menteion/actions/workflows/ci.yml/dispatches` with a JSON body, through `ghapi.sh`:

```bash
# ci.yml on this round's branch, inputs {"scope":"exhaustive"}
node scripts/exhaustive-costs.mjs from <the dispatch's downloaded artifacts> --run '<its id>' --keep scripts/exhaustive-costs.tsv > costs.new
mv costs.new scripts/exhaustive-costs.tsv
node scripts/adr-index.mjs
npx --no-install @spec-family/spec-brief archive 086 --strict --pr <n> --summary "<one paragraph>"
```

- The dispatch's eighteen shards are green and hold every pinned number, the six short forms among them.
- Every commit the dispatch ran on is an ancestor of the pull request's head, and no commit under `src/` or `tests/` follows it unless a later dispatch, or the ADR, covers it.
- The archive passes `--strict`: nothing outside `affectedFiles` and nothing in `protectedFiles` changed.

## Report

The closing message states:
- the configuration as built, and that `Config::default()` and `Config::reads_its_targets()` build the engines they built;
- the two refusals and their tests;
- the dispatch with its run id and commit, and that every pinned number held;
- each shard's job time against its bound and the regenerated cost table;
- any widening of `affectedFiles`, and why;
- that no whole-domain test ran on the developer machine;
- that the pull request is ready for review;
- what was not done and why.
