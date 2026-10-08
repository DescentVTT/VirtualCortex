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
  the `.cortex` image (format 21), the criticality controller, sleep with the episodic ledger and
  its replay, and the symbolic layer (hypervectors, categorial reduction, induction, the discovery
  path) are Implemented.
- **It learns.** On the reference network at 1 024 units the engine learns a two-alternative task,
  revises it through three reversals with its couplings bounded, and holds a reward that is right
  seven times in eight (H-13 to H-28). Asked a choice among three answers on that configuration,
  it learns the first mapping as it learns one between two and does not revise every mapping
  inside the schedule: H-29 read no, on one stimulus of one mapping of eight
  ([ADR-0153](docs/adr/0153-three-answers-measured.md)). The task takes any number of readouts up
  to 64 (ADR-0152). The learning configuration is H-25's
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
  with the hold and the released delivery (ADR-0146); the speed line is closed with the sweep kept
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
4. **Latest ≠ Newest.** A technology is admissible on the hot path only with a stable specification,
   two years of third-party production use, documented failure modes, and a mechanical-sympathy
   argument (whitepaper [§2.1](docs/WHITEPAPER.md#21-engineering-doctrine-latest--newest)). The
   same test applies to documentation standards and to dependencies. Nightly features, sub-1.0
   crates without a stability policy, and unreproduced performance claims fail it.
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

Run all of these before pushing. CI runs exactly the same set (plus the same tests on AArch64 and,
weekly, the whole-tree mutation run, which have no local form); a check here and not in CI is a
gate nobody enforces, and the reverse is a green local run and a red push.

```bash
cargo check --workspace --all-targets --locked
cargo test --workspace --locked
cargo test --workspace --release --locked
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --locked -- -D warnings
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --locked
cargo bench -p cortex-bench --bench hot_path --locked -- --test
cargo +1.85 check --workspace --all-targets --locked   # the MSRV floor; `rustup toolchain install 1.85` once
cargo +1.85 test --workspace --locked
npm ci
npm run spec
git diff main...HEAD > target/pr.diff && cargo mutants --workspace --in-diff target/pr.diff   # once: cargo install cargo-mutants --locked --version 27.1.0
cargo test --workspace --release --locked -- --ignored exhaustive   # before a release; the weekly job runs it in twelve shards, dealt by cost (ADR-0073, ADR-0092, ADR-0150)
```

The mutation line is the gate a pull request meets: every mutant `cargo-mutants` can make in the
lines the change touches must be caught by a test ([ADR-0030](docs/adr/0030-verification-governance.md));
a new rule carries a test over the lattice of `testkit/prop.rs`. `cargo test --workspace --release
--locked -- --ignored exhaustive` runs the whole-domain tests before a release.

The `cargo bench ... -- --test` line executes each benchmark once and asserts no timing. A number
becomes Measured only through the protocol in `docs/benchmarks/README.md`; a developer-machine
figure is recorded there as not admissible and is never written into the whitepaper's tables.

`npm run spec` is `spec:guard` (executable assertions in the documents against `crates/`),
`spec:graph` (cross-document consistency: links, ADR lifecycle, open obligations) and
`spec:briefs` (every live brief carries its mandatory sections, and from brief 028 a Latest ≠ Newest standing directive; `spec:briefs:test` tests the checker), `spec:decisions` (every ADR has its row in whitepaper §9 and in `docs/adr/README.md`, F-41; `spec:decisions:test` tests the checker), `spec:version` (the whitepaper's front matter and its Document control table declare the same version and date, F-43; on a pull request CI runs it again against the base, and a change to the document with its version left behind fails, [ADR-0064](docs/adr/0064-the-documentation-gate-and-the-version.md)) `spec:deps` (state crates declare
no dependencies, TC-2) and `spec:costs` (every line of the whole-domain tests' cost table names a test in the tree, ADR-0092; `spec:costs:test` tests the deal). Each fails with a file and line.

CI runs `npm run spec`, one step and the same command, so a check inside it is enforced and a check outside it is enforced nowhere: `spec:decisions` was added to the gate and to three documents on 2026-09-20 and to the workflow not at all (F-44). `spec:scripts:test` holds the list to running every `spec:*` and every `*:test` script.

## Workflow

- Never commit on `main`. Branch, then pull request; Conventional Commits with a real body
  ([CONTRIBUTING.md](CONTRIBUTING.md#commit-messages)).
- A change to a record and the change to its documentation travel in the same PR.
- A rule change is an ADR first (`status: proposed`), accepted on merge.
- To execute a brief: read `briefs/README.md`, then the brief, then the files it names. When done,
  archive it as that README says, with the frozen banner and every deliverable dispositioned.

<!-- @assert-present file="docs/WHITEPAPER.md,docs/adr/README.md,briefs/README.md,CONTRIBUTING.md,CHANGELOG.md,scripts/check-briefs.mjs" -->
