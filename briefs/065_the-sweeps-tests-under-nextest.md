---
status: proposed
date: 2026-10-09
---

# Brief 065: The sweep's tests under nextest — ADR-0160's measurement: the runtime's area of the weekly sweep run twice on one commit, under `cargo test` and under `cargo-nextest`, compared mutant by mutant; the tool kept only by the rule written first, and removed otherwise

## Mission

**This brief measures one tool in one job of the weekly workflow and changes nothing of the engine.**
[ADR-0158](../docs/adr/0158-the-sweep-off-the-next-decisions-path.md) read that 88 mutants of the runtime's 1 320 take
64 per cent of its sweep: 72 wait for the test binaries `cargo test` runs last, and 16 hang to their bound.
[ADR-0160](../docs/adr/0160-the-sweeps-tests-under-nextest.md) names `cargo-nextest` as the candidate, holds it to
Latest ≠ Newest, and writes the rule for keeping it before any run.

When the round is done, the tree holds either:
- **kept**: the runtime's sweep running its tests under nextest at a pin, with the pair's tables in an ADR; or
- **not kept**: the workflow as it was, and the pair's tables in an ADR.

Either way the ADR states what was read and the rule's verdict.

**Run this round after brief 064's has merged**, not beside it: both dispatch the weekly workflow, and the account runs
twenty jobs at once.

---

## Standing directives

- **Latest ≠ Newest** (whitepaper §2.1, `CLAUDE.md` principle 4). The one tool is `cargo-nextest`, at an exact
  version, in the `mutants-weekly` job's runtime shards alone. **Nothing else is adopted**:
  - not in the pull request's gate, not in `CLAUDE.md`'s commands, not as something a developer installs;
  - no dependency in a manifest, and no change to `Cargo.lock`;
  - no other change to `cargo-mutants`, its pin or `.cargo/mutants.toml` beyond what the measurement needs;
  - no test of the runtime renamed, reordered or wrapped.
- **Two years of third-party production use is shown first**, with its sources, or the round stops (ADR-0160).
- **No file under `src/` and no test changes.** If a test fails under nextest, that is a finding, and the test is not
  changed to pass.
- **The rule for keeping the tool is ADR-0160's**, written before any dispatch, and its three quarters does not move.
- **A verdict that differs is listed, not re-run until it agrees.**
- **Unset, the workflow is the workflow it was**: the schedule and every dispatch that does not name the tool run under
  `cargo test`.
- **The engine is read before a description of it is trusted**, this brief's and ADR-0160's included; so is the tool.
  What its documentation says is checked against what it does on this tree.
- **The round is evidenced as `briefs/README.md`'s "Evidencing a round" says** where that applies: the ADR cites the
  pull request's commits, and the commit that asks for the merge sets it to `accepted`. This round's dispatches are
  its measurement; it has no arms and no whole-domain shards of its own.
- Every loop ends by construction ([ADR-0062](../docs/adr/0062-the-first-complete-sweeps-list.md)); a script that
  compares two sweeps is a script of this tree and is held to it.
- Conventional Commits with a real body; never commit on `main`; the required checks keep their names.

## Context

Re-derived on 2026-10-09 against `main` after ADR-0158, ADR-0159 and ADR-0160 merged. Line numbers move; the symbols
and the quoted sentences are what to re-derive.

1. **The sweep's job** (`.github/workflows/ci.yml`, `mutants-weekly`): one job for the state crates and twelve shards
   for the runtime, `--shard k/12 --sharding round-robin`. Each times the area's own suite and sets
   `MUTANT_TIMEOUT` to three times it plus thirty seconds, then runs
   `cargo mutants --file "<area>/**" <shard> --baseline=skip --timeout "$MUTANT_TIMEOUT" --jobs 2`. It checks that the
   shard covered its slice, *"an incomplete sweep is not a result"*, and writes `timeout-evidence.txt`
   (`scripts/read-timeouts.mjs`).
2. **The dispatch** (`workflow_dispatch`, input `scope`: `both`, `exhaustive`, `mutants`). ADR-0075's comment says a
   round whose diff *"changes `.cargo/mutants.toml` or the `mutants-weekly` job"* dispatches the sweep.
3. **The pinned tool**: `cargo-mutants` 27.1.0, whose `--help` lists `--test-tool <TEST_TOOL>`, *"Tool used to run
   test suites: cargo or nextest"*, and `--profile`. `.cargo/mutants.toml` holds `additional_cargo_test_args =
   ["--locked"]` and the excluded mutants.
4. **What ADR-0158 read** from run 37810004245, by each job's `outcomes.json`: the runtime's 1 320 mutants and 30.2
   hours of machine time; 404 minutes building; 1 056 mutants caught within 30 s; 72 caught after more than four
   minutes, 768 minutes, first caught by `no_alloc` 35, `reference` 20, `store` 8, `sleep` 4, `modulation` 2 and
   `image` 3; 16 timeouts, 393 minutes.
5. **The inherent timeouts** ([ADR-0062](../docs/adr/0062-the-first-complete-sweeps-list.md)): the injector, the
   barrier's wait and the workers' stop and run. Their evidence reads *"the run had stopped finishing tests"*.
6. **The runtime's suite**: `cargo test -p cortex-runtime --locked`, the library's tests and nineteen test binaries
   under `runtime/cortex-runtime/tests/`. `tests/no_alloc.rs` installs a global allocator and holds one test
   function (F-64). No doctest is written under `runtime/cortex-runtime/src`.
7. **The toolchain**: `rust-toolchain.toml` pins 1.97.1.
8. **What the tool's documentation says** (read 2026-10-09; ADR-0160 quotes it): nextest runs each test in a process
   of its own and tests of several binaries at once; it fails fast by default; straggling tests run to completion;
   it does not run doctests; `slow-timeout = { period, terminate-after }` terminates a test and counts it failed; its
   command line, machine-readable output and configuration format are append-only within 0.9.x.

## Deliverables

- [ ] **The tool against Latest ≠ Newest, in a new ADR at the next free number (`ls docs/adr`), before anything is
  installed.** Two years of third-party production use, with sources; the version taken and why that one; how it is
  installed and how its integrity is checked. If the use cannot be shown, the round stops here and the ADR says so.
- [ ] **The measurement built, behind an input.**
  - A `workflow_dispatch` input that names the tool the runtime's shards run their tests under, `cargo` unless given.
    The state crates' job and every run that does not name it are unchanged.
  - nextest installed at its exact version in those shards when the input asks for it.
  - A profile for the sweep in nextest's configuration: fail fast, and a bound for each test derived by the job from
    the unmutated suite's slowest test on that runner. The derivation is written in the ADR before any dispatch.
  - The job's own bound for a mutant derived as now, from the suite timed under the tool the shard uses.
- [ ] **The unmutated suite under nextest, before any mutant.**
  - It passes.
  - It runs the tests `cargo test -p cortex-runtime` runs: both lists counted and compared by name, with whatever
    nextest does not run named. A test that is not a doctest and is not run stops the round.
- [ ] **Two dispatches at `scope=mutants` on one commit of this round's branch**, one under each tool, with the same
  twelve shards. Both green by the sweep's own rule, which lists survivors and blocks nothing.
- [ ] **The comparison, by a script of this tree with its own test**, from each job's `outcomes.json`:
  - every mutant's verdict under both tools, and every mutant whose verdict differs, named;
  - every mutant's seconds building and testing under both, summed by shard and by area;
  - ADR-0158's 72 and 16, by name where the mutant is the same, under each tool;
  - each shard's wall time under each tool.
- [ ] **The rule read once**, in the ADR:
  1. no verdict weaker: every mutant caught under `cargo` caught under nextest, a timeout caught or timed out, none
     missed that was not;
  2. the area's machine time under nextest at most three quarters of `cargo`'s in the pair;
  3. the unmutated suite passing under nextest in every shard.
- [ ] **Kept or not kept, done in the same pull request.**
  - *Kept*: the runtime's shards run under nextest by default, the input stays as the way back, and the whitepaper's
    Appendix B and the workflow's header say what the sweep runs its tests under.
  - *Not kept*: the input, the installation and the profile are removed, and the workflow's `mutants-weekly` job is
    the one on `main` before this round, line for line.
- [ ] **The documents, in the same pull request.** Whitepaper §9 and Appendix B as the result requires; the ADR index
  and `CHANGELOG.md`; a finding in §11 for anything the tree's documents or ADR-0160 stated that the tool does not do;
  the whitepaper's version in both declarations, with its date
  ([ADR-0064](../docs/adr/0064-the-documentation-gate-and-the-version.md)).
- [ ] **The brief archived** as `briefs/README.md` says, every deliverable dispositioned, and the round's ADR set to
  `accepted` by the commit that asks for the merge.

## Not empowered

- **No rule of the engine, no file under `src/` and no test changes.**
- nextest is not adopted on a pair that fails the rule, and not adopted for part of the area.
- The rule's three quarters, and the derivation of the bound for a test, do not move after the first dispatch.
- No mutant is added to `.cargo/mutants.toml`'s exclusions to make a pair agree.
- The pull request's gate, `CLAUDE.md`'s commands and the state crates' sweep stay on `cargo test`.
- No change to the whole-domain shards, `scripts/exhaustive-shard.sh` or the cost table.
- No dependency, no change to `Cargo.lock`, and no `unsafe`.

## Architectural empowerment

The executing session may replace any instruction above with a better decision, recorded in its ADR. That covers:
- the input's name and shape;
- the version of nextest taken, and how it is installed and checked;
- the profile's keys and the derivation of the bound for a test, written before any dispatch;
- how the unmutated suite's two lists are taken and compared;
- the comparison script's shape, and where it lives beside `scripts/read-timeouts.mjs`;
- whether the two dispatches run at once or one after the other.

It may not:
- change what the sweep mutates, its bound's rule for a mutant or its completeness check;
- change the rule for keeping the tool;
- change a test so that it passes under nextest;
- reach the standing directives, the whitepaper's invariants or the constraints in `CLAUDE.md`.

## Verification

```bash
cargo check --workspace --all-targets --locked
cargo test --workspace --locked
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --locked -- -D warnings
npm ci
npm run spec
git diff main...HEAD > target/pr.diff && cargo mutants --workspace --in-diff target/pr.diff
gh workflow run ci.yml --ref <this round's branch> -f scope=mutants
gh workflow run ci.yml --ref <this round's branch> -f scope=mutants -f <the input>=nextest
node scripts/<the comparison> <the cargo run's artifacts> <the nextest run's artifacts>
```

Every command exits 0. Both dispatched sweeps reach a verdict in every shard and cover their slices. The comparison
names every mutant whose verdict differs. In the history, the Latest ≠ Newest reading, the profile's derivation and
the rule precede the first dispatch.

## Report

The closing message states:
- the tool against Latest ≠ Newest, with the sources for its use, its version and how it is installed;
- the unmutated suite under nextest: whether it passes, and the two lists compared;
- the pair: the verdicts that differ, the machine time by area and by shard, the 72 and the 16 under each tool, and
  the shards' wall times;
- the rule's three clauses, each read, and kept or not kept;
- what the workflow is after the round;
- what was not done and why.
