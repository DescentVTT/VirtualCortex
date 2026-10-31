---
status: proposed
date: 2026-11-01
affectedFiles: [runtime/menteion-runtime/src/**, runtime/menteion-runtime/tests/**, testkit/**, docs/**, briefs/**, CHANGELOG.md, CLAUDE.md, README.md, scripts/exhaustive-costs.tsv]
protectedFiles: [crates/**, docs/explorations/0*.md, docs/benchmarks/results/**, benches/**, .github/workflows/**, .cargo/**, Cargo.toml, Cargo.lock, rust-toolchain.toml, clippy.toml, .spec-brief.json, package.json, package-lock.json, scripts/arm-run.sh, scripts/exhaustive-shard.sh, scripts/exhaustive-costs.mjs, scripts/check-additive.mjs, scripts/check-archive.mjs, scripts/check-scope.mjs, scripts/check-explorations.mjs, scripts/check-doc-shape.mjs, scripts/adr-index.mjs]
---

# Brief 086: A running configuration for a settled image

<!-- @assert-absence target="runtime/menteion-runtime/src/executor.rs" symbol="fn runs_a_settled_image" reason="brief 086 precondition: main holds no running configuration; the round that builds it archives this brief in the same pull request" -->

## Mission

**This brief puts H-46's exponent under a name, gives the host what it needs for the groups, and adds no rule.**
H-46 read yes ([ADR-0230](../docs/adr/0230-a-zero-sum-at-the-stimulus-measured.md)).
[ADR-0232](../docs/adr/0232-a-running-configuration-and-m12-closed.md) names a configuration an engine runs a settled image under, leaves the zero sum's groups to the host, and leaves the derivation H-42's.

When the round is done, the tree holds:
- `Config::runs_a_settled_image()`, `Config::reads_its_targets()` with a quarter's exponent, and H-46's arm running under it, the kept addresses unset;
- an exponent other than one refused beside a read, and refused on an image with a read or a probe under way;
- `Executor::coupling_sum`, the reading a host keeps each group's value from;
- one dispatch at `scope=exhaustive` whose eighteen shards are green, and seed 22's two first halves run again at `scope=arms`, every pin they hold unmoved.

**There is no hypothesis.** The one thing the name changes from H-46's arm is that the kept addresses are unset, and every reward was at age zero, so no pinned row should move. The runs show it.

It writes a built ADR and rewrites `docs/STATUS.md` ([briefs/README.md](README.md#a-rounds-documents)).
The scope is the front matter's ([ADR-0191](../docs/adr/0191-a-rounds-scope-eighteen-shards-and-the-reproduction-first.md)).

**Do not run this round beside brief 065's**: both dispatch the weekly workflow.

---

## Standing directives

- **Latest ≠ Newest** (whitepaper §2.1, `CLAUDE.md` principle 4). The round depends on nothing new: no crate, no tool, no version of either.
- **The configuration is ADR-0232's**: `Config::runs_a_settled_image()` is `Config::reads_its_targets()` with `depression_exponent` at `DepressionExponent::Quarter`, and no other field.
- **The exponent alone is not what was measured.** The name's comment and the whitepaper's paragraph say that a host which sets no groups runs a state no run has read. It is a description, as `CLAUDE.md` principle 5 asks of a rule enforced nowhere.
- **`Config::default()` and `Config::reads_its_targets()` build the engines they built.** No configuration of the tree but the new one changes, and every image is born as it was.
- **No pinned number of the tree moves.** A pin that moves stops the round as a finding, and the configuration is not merged until it is understood.
- **No derivation, lead-in or arm is run beyond what the two dispatches run.** They are the round's evidence that nothing moved. No whole-domain test runs on the developer machine.
- **The dispatches are made once the last commit under `src/` and `tests/` is pushed**: `scope=exhaustive` with nothing excluded, since a file under `src/` changes, and `scope=arms` on seed 22's two first halves. A change under either directory after them is dispatched again, or shown in the ADR to change no run.
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
   - `Config::depression_exponent` defaults to `DepressionExponent::One`, and `Executor::depression_exponent` reads it. No public setter moves it.
2. **The decode** (`runtime/menteion-runtime/src/image.rs`):
   - `Image::decode` calls `Executor::new` with `rate_read_windows`, `istdp_target_read` and `step_probe` unset, and with the configuration's exponent.
   - It has the image's read and probe in hand before that call, and takes them, the baseline, the gain and the step from the image's bytes.
   - The exponent is not in the image, so a decoded engine's is its configuration's. A decode is refused with `CriticWithoutTrain` when the image's critic is set and `train_capacity` is zero, so the sizing is the caller's.
3. **H-46's harness** (`runtime/menteion-runtime/tests/inhibition.rs`):
   - `zero_from(image, units, form)` decodes under `Config { kept_addresses: KEPT_ADDRESSES, depression_exponent: exponent, ..config(units, 2, 0) }`, the exponent `ZERO_EXPONENT` under `ZERO_ARM`, and `zero_half` shares it.
   - `zero_groups(made, name)` reads each group's value with `couplings`, and asserts it equal to `made.couplings`.
   - `zero_schedule` calls `exec.set_zero_sum(&held, &groups.targets)` on each schedule's engine of an arm, and only of an arm.
   - `zero_short` is the six `…_exhaustive` short forms' body, and `the_zero_sum_of_h_46_its_clauses_and_its_predicted_readings_rules` reads the pins of `tests/inhibition/pins/h46.rs`.
4. **The kept addresses at age zero** (`runtime/menteion-runtime/src/task.rs`): with addresses kept a trial takes `keep_address` then `reward_kept(0)`, and without them `address_drawn` then `reward`. `reward_kept` at age zero is bit for bit a plain `reward`: `a_reward_kept_at_age_zero_is_the_drawn_reward_bit_for_bit` in `executor.rs` and `on_time_under_kept_addresses_a_trial_is_the_trial_it_was_bit_for_bit` in `task.rs` hold it at the gate, and neither path reads the exponent or the zero sum.
5. **The zero sum** (`Executor::set_zero_sum`, `hold_zero_sum`, `zero_sum_round`): `zero_sum_round` walks a group's chains and sums its synapses onto the targets. `Executor::zero_sum_groups` counts the groups held.
6. **The errors** (`ConfigError` and `ImageError::Config`): `Executor::new` returns a `ConfigError` for each refused field. `Image::decode` wraps one in `ImageError::Config`.
7. **The tests that set an exponent today**: `engine` in `executor.rs`'s unit tests, the decode beside it, and `zero_from`. None stands beside a read, and each must keep passing.
8. **The weekly job** (`scripts/exhaustive-shard.sh`) lists `--ignored exhaustive`, so a `…_paused` test runs only by name at `scope=arms`.
9. **The process**: eighteen weekly shards ([ADR-0191](../docs/adr/0191-a-rounds-scope-eighteen-shards-and-the-reproduction-first.md)), the bound 240 minutes; `spec-brief archive --strict` measured from `origin/main`. The tools are `@spec-family/…` ([ADR-0231](../docs/adr/0231-the-spec-tools-under-their-new-scope.md)).
10. **The sweep**: the schedule runs on 2026-11-04 at 20:17Z. If it has ended when this round's ADR merges, that ADR records it ([briefs/README.md](README.md#evidencing-a-round)).

## Deliverables

- [ ] **The configuration, the boundary and the reader, built, in a new ADR at the next free number (`ls docs/adr`), kind `built`.**
  - `Config::runs_a_settled_image()`, its comment saying what the exponent alone is, and its test field by field against `Config::reads_its_targets()`.
  - `Executor::new` refusing an exponent other than one beside a read, by a `ConfigError` of its own.
  - `Image::decode` refusing, with the same error, an image with a read or a probe under way under such an exponent.
  - `Executor::coupling_sum(sources, targets)`, read between ticks.
  - A test over `testkit/prop.rs`'s lattice for each refusal, and one holding `coupling_sum` to a sum by hand.
- [ ] **H-46's harness under the name.**
  - `zero_from` decodes the arm's form under `Config::runs_a_settled_image()`, its sizing by struct update and `kept_addresses` left at zero, the task on its plain path at age zero, its comment rewritten. The control's form stays what it was.
  - `zero_groups` reads each value through `coupling_sum`, its assertion against `made.couplings` kept.
  - The six short forms and the verdict's test unchanged, each pin held.
- [ ] **The dispatches**, once the last commit under `src/` and `tests/` is pushed.
  - `scope=exhaustive` with nothing excluded: its eighteen shards are green, and the cost table is regenerated from it.
  - `scope=arms` on seed 22's two first halves: each job green, its eight schedules' pinned rows held under the name.
- [ ] **The ADR's reading.**
  - What was built, and the two refusals.
  - Each dispatch with its run id and commit, and each job's time against its bound.
  - That the arms' ten other long halves read the name's form unrun, and what their rows rest on.
  - Whether the schedule's sweep of 2026-11-04 has ended, and its record if it has.
- [ ] **The documents, in the same pull request.**
  - `docs/STATUS.md` rewritten whole.
  - Whitepaper §5.2.16: the running configuration beside the engine's default, with an `@assert-count` for the new name and the sentence on the exponent alone.
  - Whitepaper §5.2.1: `coupling_sum` in the zero sum's paragraph, and the exponent's paragraph no longer "unset by default" where the name sets it.
  - The tables written by `node scripts/adr-index.mjs`.
  - `CHANGELOG.md`, one entry of at most 400 characters.
  - The whitepaper's version in both declarations, with its date.
- [ ] **The brief archived** with `spec-brief archive 086 --strict`, every deliverable dispositioned. The round's ADR is set to `accepted` by the commit that asks for the merge, and the pull request is marked ready for review with it.

## Not empowered

- **The round's scope** is the front matter's. It writes only inside `affectedFiles`, and never into `protectedFiles`. A file it must change outside the scope is handled as `briefs/README.md` says.
- **No record, section or format changes**, and no state crate changes.
- **No value of `Config::default()` or `Config::reads_its_targets()` changes.**
- **`Config::runs_a_settled_image()` sets the exponent and nothing else.** The kept addresses, the delay read and the sleep stay unset in it.
- **The zero sum's groups enter no configuration and no image.**
- **No path to H-25's learning configuration is built.** The harness's byte patches stay the harness's.
- **No pinned number moves**, and no constant of H-46's.
- **No derivation, lead-in or arm is run beyond the two dispatches**, and no whole-domain test runs on the developer machine.
- No float anywhere, no dependency, no `unsafe` beyond ADR-0023's, and no rebase of the branch once a dispatch has run on it.

## Architectural empowerment

The executing session may replace any instruction above with a better decision, recorded in its ADR. That covers:
- the names of the configuration, the error and the reader, none of which may claim the bound the zero sum gives;
- whether `coupling_sum` takes its targets as a list or as `zero_sum_round`'s flags, and whether the two share their walk;
- how `zero_from` reaches the name, every pin held;
- where in `Image::decode` the second refusal sits;
- a file the round must change and its scope does not hold, within this brief's mission, widened as `briefs/README.md` says.

It may not:
- write into a protected file, or shrink `protectedFiles`;
- change a value of the default, or what `Config::default()` builds;
- let an engine be born beside a read, or a read or a probe end, under an exponent other than one;
- run a derivation, a lead-in or an arm beyond the two dispatches, or any whole-domain test on the developer machine;
- reach the standing directives, the whitepaper's invariants or the constraints in `CLAUDE.md`.

If a decoded image H-46 ran is refused, or a dispatch moves a pinned number, the round stops there: the finding is written, and the configuration is not merged until it is understood.

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
# ci.yml on this round's branch, inputs {"scope":"arms","tests":"a_zero_sum_and_a_quarter_s_exponent_on_seed_22_from_the_assignment_first_half_paused a_zero_sum_and_a_quarter_s_exponent_on_seed_22_from_the_mirrored_assignment_first_half_paused"}
node scripts/exhaustive-costs.mjs from <the exhaustive dispatch's downloaded artifacts> --run '<its id>' --keep scripts/exhaustive-costs.tsv > costs.new
mv costs.new scripts/exhaustive-costs.tsv
node scripts/adr-index.mjs
npx --no-install @spec-family/spec-brief archive 086 --strict --pr <n> --summary "<one paragraph>"
```

- The exhaustive dispatch's eighteen shards are green and hold every pin they list, the six short forms among them.
- Seed 22's two first halves are green, each pinned row held by its job's assertion.
- Every commit a dispatch ran on is an ancestor of the pull request's head, and no commit under `src/` or `tests/` follows them unless a later dispatch, or the ADR, covers it.
- The archive passes `--strict`: nothing outside `affectedFiles` and nothing in `protectedFiles` changed.

## Report

The closing message states:
- the configuration as built, and that `Config::default()` and `Config::reads_its_targets()` build the engines they built;
- the two refusals and their tests;
- each dispatch with its run id and commit, and that every pin they hold was held;
- each job's time against its bound and the regenerated cost table;
- any widening of `affectedFiles`, and why;
- that no whole-domain test ran on the developer machine;
- that the pull request is ready for review;
- what was not done and why.
