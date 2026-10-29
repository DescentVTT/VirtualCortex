# CLAUDE.md

> **This file is the switchboard, not the rulebook.** It carries the principles that are not
> negotiable, the map, and the commands. The reasoning lives in the
> [whitepaper](docs/WHITEPAPER.md) and the [ADRs](docs/adr/README.md); the workflow lives in
> [CONTRIBUTING.md](CONTRIBUTING.md). Where this file and any of those disagree, they win and the
> disagreement is a defect in this file.

## What this repository is

A Rust workspace for a **deterministic, single-node neuromorphic virtual-actor engine**: spiking
neural computation on 64-byte cache-line records, Q16.16 fixed point, and a fixed pool of
core-pinned workers. Thirty-two crates, one per subsystem, no dependencies between them;
fourteen were admitted by [ADR-0016](docs/adr/0016-thirty-two-crate-architecture.md) on 2026-09-10.

Where the work stands, and the next decision, is in [docs/STATUS.md](docs/STATUS.md), which every
round rewrites whole. This file carries no status of its own
([ADR-0197](docs/adr/0197-one-place-for-each-kind-of-fact.md)).

| To know | Read |
| :--- | :--- |
| Where the work stands, and the next decision | [docs/STATUS.md](docs/STATUS.md) |
| What is built, subsystem by subsystem | Whitepaper [§1.6](docs/WHITEPAPER.md#16-implementation-status-at-a-glance) |
| Every hypothesis asked of the engine, with its verdict | Whitepaper [§11.1](docs/WHITEPAPER.md#111-hypotheses-and-open-questions); each one's text, stopping rule and readings in [docs/research/](docs/research/README.md) |
| What is known to be wrong | Whitepaper [§11](docs/WHITEPAPER.md#11-risks-and-technical-debt), findings F-n |
| Why each thing was decided, in order | [docs/adr/README.md](docs/adr/README.md) |
| What each round did | [CHANGELOG.md](CHANGELOG.md) |

**The next round** is the live brief in [briefs/](briefs/README.md), if there is one; the decision
behind it is the newest ADR in the index.

## The principles

Each one is here because breaking it has already produced a defect in this repository.

1. **The repository wins over the document.** Where a document and the tree disagree, the tree is
   authoritative; record the disagreement as a numbered finding in whitepaper §11 rather than
   fixing either side silently. Specification 2.8.0 described 15 of 19 records wrongly because
   nobody did this.
2. **Verify before asserting.** Read the file. Run the command. If you are about to write
   "presumably", go and look. The whitepaper's first executable assertion was written from memory
   and failed on the first run (F-18).
3. **Label every claim.** Implemented, Specified, Target or Hypothesis. A number is Measured only
   when a benchmark in this tree produced it on the reference platform from a committed command
   ([ADR-0010](docs/adr/0010-measured-or-target.md)); otherwise it is a Target with a protocol.
   Never write "tested on". Never reintroduce the withdrawn figures (86 billion neurons,
   P99.99 < 35 ns, 100 ms cold boot).
4. **Latest ≠ Newest.** A technology or a dependency is admissible on the hot path or in the
   engine's build only with a stable specification, two years of third-party production use,
   documented failure modes, and a mechanical-sympathy argument (whitepaper
   [§2.1](docs/WHITEPAPER.md#21-engineering-doctrine-latest--newest)). Documentation standards meet the
   same test; the project's own documentation tools are admitted by ADR-0008 and ADR-0185 instead.
   Nightly features, sub-1.0 crates without a stability policy, and unreproduced performance claims
   fail it. It governs what the tree depends on, not what the engine does: a mechanism enters by an
   ADR and a measured round ([ADR-0187](docs/adr/0187-the-upgrade-suite-planned.md)), and a brief's "nothing else is
   adopted" is that round's scope, stated on its own, not this principle.
5. **A structural boundary beats a reviewed one.** Compile-time assertion > test > executable
   documentation directive > review comment. When you add a rule, name where it is enforced; if
   nowhere, write it as a description, not a requirement.
6. **Make claims about the tree executable.** A sentence that says something exists gets a
   `<!-- @assert-count ... min="1" -->` under it; a sentence that says something is gone gets
   `<!-- @assert-absence ... -->`. `spec-guard` runs them in CI.
7. **Say what you did not do.** A completed task with an unstated gap is worse than an incomplete
   one. Finish everything not blocked, then name what is left and why.
8. **No intermediate documents that drift.** One canonical whitepaper in English; the Traditional
   Chinese file is a reader's guide with no layouts or figures; briefs are inputs and are frozen
   when executed; outcomes live in ADRs, the reports in `docs/research/`, the changelog and the
   code, and where the work stands in `docs/STATUS.md` alone ([ADR-0197](docs/adr/0197-one-place-for-each-kind-of-fact.md)).

## Invariants of the code

These are checked; the whitepaper §2.2 lists the constraint ids.

- Every primary record is `#[repr(C)]`; arena records are `align(64)` and exactly 64 bytes; size
  and alignment are asserted in a `const _: () = { ... }` block in the defining crate.
- No `f32` or `f64` anywhere in the workspace: a Clippy error under `[workspace.lints]`
  ([ADR-0029](docs/adr/0029-structural-enforcement.md)). Q16.16 in `i32`/`u32`; widen to `i64` to multiply;
  saturating arithmetic on state fields (whitepaper §8.1); plain `+ - * / %` is a Clippy error in
  every crate and every test crate root (`clippy::arithmetic_side_effects`, ADR-0029, brief 016).
  Sixteen-bit synaptic weights are Q1.15 and eight-bit plasticity factors are Q0.8
  ([ADR-0012](docs/adr/0012-synaptic-weight-q1-15.md)).
- Every state crate is `#![no_std]`. No `Box`, `Vec`, `String`, thread spawning or heap allocation in
  state crates; no syscalls on the hot path once a hot path exists.
- A record without atomics derives `Clone, Copy, Debug, PartialEq, Eq`; a record with atomics is a
  control record and derives `Debug` only (whitepaper §8.2, L-5).
- No `unsafe` without an ADR naming the invariant and the test; `unsafe_code` is forbidden by
  `[workspace.lints]` in every state crate and the benchmark crate (ADR-0029).
- Every loop ends by construction: a `for` over a range or a slice, a countdown by one tested for
  zero, a scan by `get`, a recursion whose depth argument falls to a stated bound; never by an
  ordering comparison alone on a value the body moves, and a test's loop never by the function it
  tests. A wait on another thread is the runtime's protocol and lives there only. The weekly
  sweep's `timeout.txt` is the evidence, read by the triage rule of
  [ADR-0062](docs/adr/0062-the-first-complete-sweeps-list.md): a directive-class timeout is a
  rewrite held to its previous form bit for bit, an inherent one is a detection and stays. The job
  writes `timeout-evidence.txt` beside it — each timeout's window, what ran in the shard's other
  slot and whether its test run was still finishing tests
  ([ADR-0063](docs/adr/0063-the-sweep-reads-its-own-timeouts.md), F-42); `npm run mutants:timeouts`
  is the same reading on a downloaded artifact.
- State crates declare no dependencies (`npm run spec:deps` holds it). The runtime crate `runtime/menteion-runtime` composes
  them ([ADR-0023](docs/adr/0023-executor.md)) and is the only place `unsafe` is allowed, under
  that ADR's invariant: a `&mut` to a record never overlaps another reference to it.
- Changing any field of any record, including reserved bytes, bumps `MenteionFileHeader::version`,
  updates the record's table in whitepaper §5.2, and gets a changelog entry.
- The engine never amends its own code. What it may amend by itself is a parameter in
  `menteion-executive`'s `REGISTRY`, through the four gates of `PolicyAmendment` and a trial in two
  forks of the image whose behaviour hashes must be equal ([ADR-0031](docs/adr/0031-policy-amendment.md));
  a registry entry is an ADR, and the veto gate's parameters are never one.
- Edition 2024 and MSRV 1.85 are decided by [ADR-0009](docs/adr/0009-rust-edition-and-msrv.md)
  and inherited from `[workspace.package]`; the toolchain CI builds with is pinned in
  `rust-toolchain.toml`. Moving any of the three is its own pull request, never a passing edit.
- The crate count is not a design parameter. A new state crate needs an ADR that names the gap
  it fills, no existing record owning the quantity, its mechanism in whitepaper §8.8 with the
  layout, formats that fit their widths, and the §1.5 and §8.10 boundaries intact or moved by
  that ADR first ([ADR-0016](docs/adr/0016-thirty-two-crate-architecture.md)); a quantity that belongs
  to an existing subsystem is a field in that record, not a crate. The `expected="32"`
  directives are the tripwire; the fourteen crates admitted on 2026-09-10 each passed that test.

## Where to read

| Question | File |
| :--- | :--- |
| What is the architecture? | [docs/WHITEPAPER.md](docs/WHITEPAPER.md) — arc42; §4 axioms, §5 per-crate layouts, §8 numeric and concurrency rules |
| Why was something decided? | [docs/adr/](docs/adr/README.md) — MADR, one file per decision |
| What is known to be wrong? | Whitepaper §11 — findings F-n, hypotheses H-n, open questions |
| What is the next round of work? | [briefs/](briefs/README.md) — self-contained prompts; `briefs/archive/` is what already ran |
| How do I contribute? | [CONTRIBUTING.md](CONTRIBUTING.md) — commits, code rules, documentation rules, definition of done |
| What changed? | [CHANGELOG.md](CHANGELOG.md) |

## Commands

The checks run in three places ([ADR-0184](docs/adr/0184-the-runs-off-the-developer-machine.md)). Before a push the
developer machine runs the first block, and the pull request's gate runs it again with the
second. The whole-domain tests run only by dispatch, the third block, and none of them runs on
the developer machine. Every line here runs in CI. A check run only on the developer machine would
be enforced by nobody; one that only the gate runs is enforced, and can turn a push red.

```bash
# 1. Before a push, on the developer machine; the pull request's gate runs these too
cargo check --workspace --all-targets --locked
cargo test --workspace --locked
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --locked -- -D warnings
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --locked
npm ci
npm run spec
```

```bash
# 2. The pull request's gate alone, with the tests again on AArch64
cargo test --workspace --release --locked
cargo bench -p menteion-bench --bench hot_path --locked -- --test
cargo +1.85 check --workspace --all-targets --locked   # the MSRV floor
cargo +1.85 test --workspace --locked
git diff main...HEAD > target/pr.diff && cargo mutants --workspace --in-diff target/pr.diff   # cargo-mutants 27.1.0
```

```bash
# 3. By dispatch alone, never on the developer machine
gh workflow run ci.yml --ref <branch> -f scope=exhaustive   # every whole-domain test, eighteen shards; a calibration adds -f exclude='<the round's own tests>'
gh workflow run ci.yml --ref <branch> -f scope=exhaustive -f only='<the round's own tests>'   # a round's reproduction, when nothing its calibration ran has changed
gh workflow run ci.yml --ref <branch> -f scope=arms -f tests='<test> ...'   # a round's arms, lead-ins and trials: one job a test on AArch64, its output kept
gh workflow run ci.yml --ref explore/NNN-<slug> -f scope=arms -f tests='<test>_explore ...'   # an exploration's runs, as often as it needs (docs/explorations/)
```

The mutation line is the gate a pull request meets: every mutant `cargo-mutants` can make in the
lines the change touches must be caught by a test ([ADR-0030](docs/adr/0030-verification-governance.md));
a new rule carries a test over the lattice of `testkit/prop.rs`. Before a release the whole-domain
tests run by a dispatch at `scope=exhaustive`. A round's runs are dispatches too: its calibration at
`scope=exhaustive` with its own tests excluded, made with its arms at `scope=arms`, and its
reproduction at `scope=exhaustive` with only its own tests; the whole-tree sweep runs on the weekly
schedule alone ([ADR-0211](docs/adr/0211-the-round-shortened.md),
[briefs/README.md](briefs/README.md#evidencing-a-round)). A round whose change only adds, by
`npm run round:additive -- origin/main <commit>`, calibrates with its own calibration tests alone
([ADR-0215](docs/adr/0215-the-calibration-a-change-needs.md)).

A pull request of documents alone skips the gate's Rust jobs, and so does one whose two trees, its
base's and its merge's as those jobs read them, a green gate already tested; the mutation gate runs
in three shards ([ADR-0223](docs/adr/0223-the-gate-shortened-and-explorations-kept-as-tags.md)).

The `cargo bench ... -- --test` line executes each benchmark once and asserts no timing. A number
becomes Measured only through the protocol in `docs/benchmarks/README.md`; a developer-machine
figure is recorded there as not admissible and is never written into the whitepaper's tables.

`npm run spec` is `spec:guard` (executable assertions in the documents against `crates/`),
`spec:graph` (cross-document consistency: links, ADR lifecycle, open obligations) and
`spec:prove` (each assertion is shown a violation of itself and must fail it, so none passes over nothing), `spec:briefs` (`spec-brief lint` over `.spec-brief.json`: every live brief carries its mandatory sections in order and a Latest ≠ Newest standing directive, and no archived brief has changed since its hash), `spec:archive` (every archived brief carries that hash, [ADR-0185](docs/adr/0185-the-spec-tools-upgraded-and-spec-brief.md); `spec:archive:test` tests the checker), `spec:explorations` (every note in `docs/explorations/` is marked exploratory with its branch, commit and runs, and no ADR cites one outside its Context, [ADR-0193](docs/adr/0193-an-exploration-lane.md); `spec:explorations:test` tests the checker), `spec:scope` (from brief 075 every brief declares `affectedFiles` and `protectedFiles`, and none protects less than its copy on `origin/main`, [ADR-0191](docs/adr/0191-a-rounds-scope-eighteen-shards-and-the-reproduction-first.md); `spec:scope:test` tests the checker), `spec:decisions` (every ADR has its row in whitepaper §9 and in `docs/adr/README.md`, F-41; `spec:decisions:test` tests the checker), `spec:version` (the whitepaper's front matter and its Document control table declare the same version and date, F-43; on a pull request CI runs it again against the base, and a change to the document with its version left behind fails, [ADR-0064](docs/adr/0064-the-documentation-gate-and-the-version.md)) `spec:deps` (state crates declare
no dependencies, TC-2) and `spec:costs` (every line of the whole-domain tests' cost table names a test in the tree, ADR-0092; `spec:costs:test` tests the deal). Each fails with a file and line.

It runs `spec:index` too (the ADR index, §9 and the hypotheses' tables are what
`scripts/adr-index.mjs` writes from front matter; `spec:index:test` tests it) and `spec:shape`
(ADR-0197's limits on the text a change adds or modifies against `origin/main`, and an accepted
ADR's body byte for byte its base copy; `spec:shape:test` tests it).

And it runs `spec:pins`: a round's pinned tables are in its own file, `tests/<binary>/pins/<round>.rs`, declared, with no test among them and no table left in the binary's file past 24 lines ([ADR-0227](docs/adr/0227-the-pinned-tables-in-a-file-a-round.md); `spec:pins:test` tests it).

CI runs `npm run spec`, one step and the same command, so a check inside it is enforced and a check outside it is enforced nowhere: `spec:decisions` was added to the gate and to three documents on 2026-09-20 and to the workflow not at all (F-44). `spec:scripts:test` holds the list to running every `spec:*` and every `*:test` script.

## Workflow

- Never commit on `main`. Branch, then pull request; Conventional Commits with a real body
  ([CONTRIBUTING.md](CONTRIBUTING.md#commit-messages)).
- `main` takes a pull request by a merge commit, the only method its ruleset allows. Bring a branch
  up to date by merging `main` into it; never rebase or force-push one once a run has been made on
  it or its pull request is open ([ADR-0186](docs/adr/0186-merge-commits.md)).
- A change to a record and the change to its documentation travel in the same PR.
- A rule change is an ADR first (`status: proposed`), accepted on merge.
- A question with no prediction is an exploration, not a round: `…_explore` tests on an
  `explore/NNN-…` branch, dispatched at `scope=arms`, written down as a note in `docs/explorations/`
  that decides nothing ([ADR-0193](docs/adr/0193-an-exploration-lane.md)). When the note merges, the
  branch becomes a tag of its name, which a ruleset keeps ([ADR-0223](docs/adr/0223-the-gate-shortened-and-explorations-kept-as-tags.md)).
- To execute a brief: read `briefs/README.md`, then the brief, then the files it names. When done,
  archive it as that README says, with the frozen banner and every deliverable dispositioned.
- From brief 075 a round writes inside its brief's `affectedFiles` and never into its
  `protectedFiles`; `spec-brief archive --strict` refuses it otherwise. It widens its scope only
  within its empowerment, in a commit of its own that says why ([ADR-0191](docs/adr/0191-a-rounds-scope-eighteen-shards-and-the-reproduction-first.md)).

<!-- @assert-present file="docs/WHITEPAPER.md,docs/adr/README.md,briefs/README.md,CONTRIBUTING.md,CHANGELOG.md,.spec-brief.json,scripts/check-archive.mjs" -->
