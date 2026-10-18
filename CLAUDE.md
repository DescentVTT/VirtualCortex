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

It is past the **state-model stage**. This section says where the work stands and where to read
the rest; it is not the record of the rounds
([ADR-0150](docs/adr/0150-a-round-waits-for-what-it-checks.md)). A round changes it only where the
standing changes.

| To know | Read |
| :--- | :--- |
| What is built, subsystem by subsystem | Whitepaper [§1.6](docs/WHITEPAPER.md#16-implementation-status-at-a-glance) |
| Every hypothesis asked of the engine, with its verdict and stopping rule | Whitepaper [§11.1](docs/WHITEPAPER.md#111-hypotheses-and-open-questions) |
| What is known to be wrong | Whitepaper [§11](docs/WHITEPAPER.md#11-risks-and-technical-debt), findings F-n |
| Why each thing was decided, in order | [docs/adr/README.md](docs/adr/README.md) |
| What each round did | [CHANGELOG.md](CHANGELOG.md) |

**Where the work stands.**

- **The engine runs.** The records with their layout assertions, the update rules, synaptic fan-out
  with three-factor STDP, the executor on a pool of workers that each own a range of the unit arena,
  the `.cortex` image (format 23), the criticality controller, sleep with the episodic ledger and
  its replay, and the symbolic layer (hypervectors, categorial reduction, induction, the discovery
  path) are Implemented.
- **It learns.** On the reference network at 1 024 units the engine learns a two-alternative task,
  revises it through three reversals with its couplings bounded, and holds a reward that is right
  seven times in eight (H-13 to H-28). Asked a choice among three answers on that configuration,
  it learns the first mapping as it learns one between two and does not revise every mapping
  inside the schedule: H-29 read no, on one stimulus of one mapping of eight
  ([ADR-0153](docs/adr/0153-three-answers-measured.md)). With a punishment delivered whole where
  the critic's value is below zero, the first revision among three is in time and the two after
  it are lost: H-30 read no ([ADR-0156](docs/adr/0156-a-punishment-the-value-does-not-soften-measured.md)).
  Given half again the blocks a mapping, the first revision among three completes and the ones
  after it slow, the last not made by one stimulus in either arm, with a coupling past its bound:
  H-31 read no ([ADR-0161](docs/adr/0161-the-span-a-choice-among-three-is-given-measured.md)).
  With a selection drawn among the readouts with a probability equal to the part of the critic's
  value below zero, the first revision among three is made in both arms and sooner, the second in
  neither and the third in one: H-32 read no
  ([ADR-0164](docs/adr/0164-an-exploration-the-value-gates-measured.md)), and by its stopping rule the
  line among three answers stops there.
  On two other networks of the same size, ADR-0044's prior from seeds 23 and 24, with each constant
  of the configuration derived again by its rule, the engine learns and revises the two-alternative
  task as on the first: H-33 read yes
  ([ADR-0167](docs/adr/0167-another-network-of-the-same-size-measured.md)).
  At twice the size, 2 048 units, on two networks of that generator, with each constant derived by
  its rule and the critic's rule extended first past 64 units a stimulus (ADR-0168), it learns and
  revises the task as at 1 024: H-34 read yes
  ([ADR-0169](docs/adr/0169-twice-the-size-measured.md)).
  Under a reward right three times in four on the first network, seven mappings of eight are
  learned and one reversal of six is not made inside the schedule, by one stimulus of two, with the
  couplings bounded and the critic holding the expected reward: H-35 read no
  ([ADR-0172](docs/adr/0172-a-reward-right-three-times-in-four-measured.md)). The boundary lies
  between that share and seven in eight.
  Under the same share on the two other networks of the size, seeds 23 and 24, each with its
  constants derived by their rules, the first learns all eight mappings and the second six, one
  reversal of twelve not made inside the schedule; over the three networks twenty-one mappings of
  twenty-four are learned: H-36 read no
  ([ADR-0174](docs/adr/0174-three-in-four-on-the-other-networks-measured.md)). The no at three in
  four is not one network's.
  With the engine's criticality controller on, at five steps from a sixteenth to one in 256 on the
  first network, no step holds the network where the stimulus fires once and is seen: every step
  takes it to the saturation rule's rate, 24.4 Hz a unit, a smaller step narrowing the gain's
  swing there and not lowering it, and the estimate reads near one at no rate: H-37 read no, as
  predicted ([ADR-0177](docs/adr/0177-the-controllers-step-measured.md)).
  With the controller given a second rule, the gain moved by the window's count over a target
  (ADR-0178, ADR-0180), and the target the settled network's own rate, every step from an eighth
  to one in 256 holds the first network where the stimulus fires once and is seen; under an
  eighth, the largest, the engine learns and revises the two-alternative task as with the gain
  held, the count held at its target and the gain two per cent below its image's: H-38 read yes,
  as predicted ([ADR-0181](docs/adr/0181-a-rate-the-controller-holds-measured.md)). The rate
  changes fifteen per cent for one per cent of gain there, so an eighth is at that loop's edge
  and a sixteenth inside it.
  Under a sixteenth, on that network and on seeds 23 and 24, each with the target its own
  derivation with the gain held returns, the engine learns and revises the two-alternative task
  on every network, the count held at its target with the gain held's scatter: H-39 read yes, as
  predicted, and of its four predicted readings the alternation's missed, a sixteenth halving an
  eighth's and not removing it ([ADR-0183](docs/adr/0183-a-sixteenth-on-three-networks-measured.md)).
  With the target the engine reads for itself (ADR-0189): its gain held for 128 windows, the mean
  count of the last sixteen the target, then the rate's rule at a sixteenth. On those three
  networks the engine learns and revises the two-alternative task as under a derived target, the
  read targets 1.0 to 2.8 per cent above the derived ones and the count held at them: H-40 read
  yes, as predicted, and of its three predicted readings the first missed, the live count still
  rising through the read ([ADR-0190](docs/adr/0190-a-target-the-engine-reads-measured.md)).
  What the engine carries by default is not decided. Every learning result before H-38 stands
  with the gain held and the controller off.
  The task takes any number of readouts up to 64 (ADR-0152). The learning configuration is H-25's
  ([ADR-0146](docs/adr/0146-the-tag-the-address-already-is.md)):
  - the excitatory synapses under the reward's gate, with the signed gate set;
  - the inhibitory synapses under a baseline of their own, with the inhibitory rule's target at the
    settled network's rate;
  - a critic of the engine's own, counting each unit's spikes within the shortest synaptic delay
    after a reward;
  - the reward's address drawn by the engine: its sources the units that window counted, its targets
    the readout its own selection chose.

  Per trial the host gives the reward's sign. The readout sets are the body's interface.
- **What was asked and read as no stays in the tree, unset.** The rule held by the network is paused
  with the facilitating class and the slow current
  ([ADR-0126](docs/adr/0126-the-gate-raised-measured.md)); the address's neural target side is closed
  with the hold and the released delivery (ADR-0146); the whole punishment is a parameter of the
  image no configuration sets (ADR-0155, ADR-0156); the exploration is an option of the task no
  run's configuration sets (ADR-0163, ADR-0164); the controller's step under its estimate is zero
  in every run that learns (ADR-0077, ADR-0177), the rate's rule is set in H-38's, H-39's and
  H-40's runs and in no other (ADR-0181, ADR-0183, ADR-0190), and the read of its target in
  H-40's alone; the speed line is closed with the sweep kept
  and the lanes reverted ([ADR-0105](docs/adr/0105-the-speed-line-closed.md)).
- **What does not exist.** Every subsystem's real dynamics beyond these rules; the lookahead
  ADR-0099 ordered and a lever on the rest of the turn; the real-time mode's reach measured on the
  reference platform; the `mmap` path, the shared-memory mappings and core pinning; the tool broker;
  standardising apart; a target rate chosen for the engine's default; the per-unit gain of §8.8.
- **The next round** is the live brief in [briefs/](briefs/README.md), if there is one; the decision
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
   when executed; outcomes live in ADRs, the changelog and the code.

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
- State crates declare no dependencies (`npm run spec:deps` holds it). The runtime crate `runtime/cortex-runtime` composes
  them ([ADR-0023](docs/adr/0023-executor.md)) and is the only place `unsafe` is allowed, under
  that ADR's invariant: a `&mut` to a record never overlaps another reference to it.
- Changing any field of any record, including reserved bytes, bumps `CortexFileHeader::version`,
  updates the record's table in whitepaper §5.2, and gets a changelog entry.
- The engine never amends its own code. What it may amend by itself is a parameter in
  `cortex-executive`'s `REGISTRY`, through the four gates of `PolicyAmendment` and a trial in two
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
cargo bench -p cortex-bench --bench hot_path --locked -- --test
cargo +1.85 check --workspace --all-targets --locked   # the MSRV floor
cargo +1.85 test --workspace --locked
git diff main...HEAD > target/pr.diff && cargo mutants --workspace --in-diff target/pr.diff   # cargo-mutants 27.1.0
```

```bash
# 3. By dispatch alone, never on the developer machine
gh workflow run ci.yml --ref <branch> -f scope=exhaustive   # every whole-domain test, twelve shards; a calibration adds -f exclude='<the round's own tests>'
gh workflow run ci.yml --ref <branch> -f scope=arms -f tests='<test> ...'   # a round's arms, lead-ins and trials: one job a test on AArch64, its output kept
```

The mutation line is the gate a pull request meets: every mutant `cargo-mutants` can make in the
lines the change touches must be caught by a test ([ADR-0030](docs/adr/0030-verification-governance.md));
a new rule carries a test over the lattice of `testkit/prop.rs`. Before a release the whole-domain
tests run by a dispatch at `scope=exhaustive`. A round's runs are dispatches too: its calibration at
`scope=exhaustive` with its own tests excluded, and its arms at `scope=arms`, reproduced by the next
dispatch at `scope=exhaustive` ([briefs/README.md](briefs/README.md#evidencing-a-round)).

The `cargo bench ... -- --test` line executes each benchmark once and asserts no timing. A number
becomes Measured only through the protocol in `docs/benchmarks/README.md`; a developer-machine
figure is recorded there as not admissible and is never written into the whitepaper's tables.

`npm run spec` is `spec:guard` (executable assertions in the documents against `crates/`),
`spec:graph` (cross-document consistency: links, ADR lifecycle, open obligations) and
`spec:prove` (each assertion is shown a violation of itself and must fail it, so none passes over nothing), `spec:briefs` (`spec-brief lint` over `.spec-brief.json`: every live brief carries its mandatory sections in order and a Latest ≠ Newest standing directive, and no archived brief has changed since its hash), `spec:archive` (every archived brief carries that hash, [ADR-0185](docs/adr/0185-the-spec-tools-upgraded-and-spec-brief.md); `spec:archive:test` tests the checker), `spec:decisions` (every ADR has its row in whitepaper §9 and in `docs/adr/README.md`, F-41; `spec:decisions:test` tests the checker), `spec:version` (the whitepaper's front matter and its Document control table declare the same version and date, F-43; on a pull request CI runs it again against the base, and a change to the document with its version left behind fails, [ADR-0064](docs/adr/0064-the-documentation-gate-and-the-version.md)) `spec:deps` (state crates declare
no dependencies, TC-2) and `spec:costs` (every line of the whole-domain tests' cost table names a test in the tree, ADR-0092; `spec:costs:test` tests the deal). Each fails with a file and line.

CI runs `npm run spec`, one step and the same command, so a check inside it is enforced and a check outside it is enforced nowhere: `spec:decisions` was added to the gate and to three documents on 2026-09-20 and to the workflow not at all (F-44). `spec:scripts:test` holds the list to running every `spec:*` and every `*:test` script.

## Workflow

- Never commit on `main`. Branch, then pull request; Conventional Commits with a real body
  ([CONTRIBUTING.md](CONTRIBUTING.md#commit-messages)).
- `main` takes a pull request by a merge commit, the only method its ruleset allows. Bring a branch
  up to date by merging `main` into it; never rebase or force-push one once a run has been made on
  it or its pull request is open ([ADR-0186](docs/adr/0186-merge-commits.md)).
- A change to a record and the change to its documentation travel in the same PR.
- A rule change is an ADR first (`status: proposed`), accepted on merge.
- To execute a brief: read `briefs/README.md`, then the brief, then the files it names. When done,
  archive it as that README says, with the frozen banner and every deliverable dispositioned.

<!-- @assert-present file="docs/WHITEPAPER.md,docs/adr/README.md,briefs/README.md,CONTRIBUTING.md,CHANGELOG.md,.spec-brief.json,scripts/check-archive.mjs" -->
