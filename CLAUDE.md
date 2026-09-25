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

It is past the **state-model stage**: the records, their compile-time layout assertions, small
update rules in twenty-six crates, synaptic fan-out with three-factor STDP (an eligibility
trace per synapse, consolidated by the modulator; ADR-0032), and the executor that runs them
on a pool of workers (`runtime/cortex-runtime`, ADR-0023), the `.cortex` image writer and
loader and the clock sweep (ADR-0024), the policy amendment's trial in two forks of the image
and its commit (ADR-0031), an image that says what a tick is and at which tick it was
written (ADR-0033), and the criticality controller (the population's spikes tallied every
tick, the branching ratio estimated by lag-one regression once per window, a bounded global
synaptic gain applied by every turn, all on a cadence that is a mask on the tick; ADR-0035,
ADR-0036), sleep as a state machine stepped once per window (a two-process pressure, a
circadian phase whose sixteen bits are a day, three stages, a wake as an input; ADR-0037) and
the episodic ledger (tagged patterns appended and never overwritten, replayed into the network
on a ripple cadence in slow-wave sleep and depotentiated in REM; ADR-0038), the hypervector body with the vector-symbolic algebra as integer rules over its words (ADR-0039), syntax as categorial reduction over the term arena and the runtime's composition of a category sequence into a frame sealed as a hypervector and read back (ADR-0040), induction on the term arena (definite clauses as terms, the least general generalisation, absorption, identification and intra-construction with a predicate invented from a reserved band; ADR-0041), the convergents of a polynomial continued fraction as the scratchpad's first expression with a bounded coefficient search (ADR-0042), and the runtime's discovery path (a clause store's description length as the free energy the valence rule reads, an invention's drop as the modulator's reward, a prover frame's certificate into a theorem; ADR-0043), the reference network (a seeded anatomical prior in `cortex-connectome`, the runtime's synthesis, a drive that is a function of the tick and a causal branching-ratio oracle by perturbation of forks; the estimator, the controller and a night's replay measured at 256 and 1 024 units; ADR-0044) and the executive clause search with a bounded proof search (ADR-0045), the lexicon on ids between a host's token ids and the frame in both directions (ADR-0046), the estimator measured on a second prior with the record's estimate cross-checked against the spike train and no line added (ADR-0047) and episodes tagged from the spike train and bound to a rewarded invention (ADR-0048), Dale's principle in the plasticity rule (a block's polarity from its unit's flag, the weight within its half of the width, the symmetric inhibitory rule; ADR-0049), the executor's own spike train (every worker's spikes of a tick merged in unit order into a bounded ring, the capture and the discovery loop over it; ADR-0050) and the estimator measured at 4 096 units with no line added (ADR-0051), the term arena in the image and the discovery loop inside the tick (the clause store, the term nodes, the induction record, the affect state and the episode's binding to an invented predicate as sections of the image, format 14; a commit's outputs instantiated so the store reads without the binding table; the search on a cadence while awake from a cursor; ADR-0052), the inhibitory target period as a parameter of the image with a day measured at 256 and 1 024 units (the registry's behaviour gate admits no plasticity parameter, F-37; ADR-0053) and the causal count inside the loop measured against the oracle at three sizes and left as a reading (ADR-0054), the excitatory depression scaled by the weight's magnitude so that a weight settles under stationary pairing instead of draining, read over a day at 256 and 1 024 units (ADR-0055), the term arena compacted onto the clause store at every entry into slow-wave sleep (ADR-0056) and the inhibitory rule read from below the rail with the fraction of units at the target as the reading a chosen target needs (ADR-0057), the two-alternative task composed on the executor (a stimulus drawn from the trial's index and a seed, a readout that counts a trial's spikes per set from the executor's own train and selects through `cortex-basal-ganglia`'s gating rule, a reward whose sign is the outcome's; ADR-0059) and its measurement on the reference network at 256 and 1 024 units, where the accuracy stayed at chance under a criterion written first and four controls, so the loop as composed does not move that behaviour at those sizes (ADR-0060), the instrument recalibrated and the question asked again through it (a stimulus that fires each of its units once, a readout window from the prior's delay band, a trial of one dopamine time constant, a gain picked by a calibration with the weights frozen; the readout resolves the stimulus from the background and the accuracy still stays at chance, so the no is about the rule on this task at these sizes; ADR-0065, ADR-0066), and the reward addressed (the dopamine term reaches the synapses from the stimulus presented onto the readout the engine selected, every other synapse consolidating under the baseline alone; the couplings from a stimulus into its rewarded readout part from the other two by a few per cent, downward, as the trace's sign said before the run, and the accuracy still stays at chance at 1 024 units with the global form reproduced beside it; ADR-0068, ADR-0069), and where 256 units settle under the drive alone, read over eighty windows on the instrument's executor (brief 026's clause first holds at the ninth window while the excitatory sum is still falling at the eightieth, at 0.55 of the prior's, and behind the lead-in the rule derived the instrument's measure is below its mark from the first block, so a learning round's criterion stays at 1 024 units; ADR-0070), and what the trace is made of (the eligibility on a stimulus's synapses into a readout, read from the record at 1 024 units with the weights frozen and split by what paired it through an oracle that replays the pair rule over the executor's own train and agrees with the record at every reading: the volley's pairing is net potentiation and the background's net depression of four to five times it, because each stimulus unit fires about twice more within one pair window after its volley and the rule depresses once per presynaptic spike, F-46; a ladder of gains below the present one passes at no rung under the sight and the sign, so no rewarded run was made and the gain stays; ADR-0072), and the one change F-46 names tried (one message at the unit's threshold, of 1.25 and of 1.125, each read over a frozen run at 1.75 by a measure written first; the executor scales an injected message by the gain, F-47, so none fires every unit once under the drive, the trace is net depression under each and there is no rewarded run; what the criterion requires, one to three spikes per trial on the answer readout, is read off the trials; ADR-0074), and the two injections (a cancel on the task's stimulus, negative basal messages into the same units inside the trial; the brief's cancel at `REFRACTORY_TICKS` lands inside the refractory window, which the membrane rule drops, so the cancel is derived from the rule — the tick after the window, over the nine ticks the volley spreads, six messages at the bound per tick — and under it every unit fires once and not again; the volley's depression goes to nothing and the background's halves, the volley's potentiation halves too because the response measured before was to three volleys, the terms sum to −2.0 per synapse per trial, the sight passes and the sign reads 13 of 64, so there is no rewarded run and the stimulus side of H-12 is exhausted under its stopping rule; ADR-0076), and the background side (the network settled before the task under the gain held — ADR-0055's criterion as an integer rule, held at the ninety-sixth window with the excitatory sum at 0.925 of the prior's and the readout units' rate the prior's — and the criticality controller on at its step, settled at the thirty-fifth at 0.453 with the gain swinging between 2.2 and 2.8 and the readouts seven to twenty-four times louder, as predicted; each alone, run quiet, saved as an image with the baseline at zero and read over a frozen run by three measures written first; the settled network fires once, is seen at 62 of 64 and its trace is net potentiation on average for the first time at a gain at which the readout sees, +0.4 per synapse per trial, but the sign reads 53 against 56; the controller fails all three; no candidate passes, there is no rewarded run, and by the rule's step 3 H-12 is closed as a no for this task in this regime; ADR-0077), and the executor's own count of the turns it serves, with the active set measured on the reference network (one message of 0.125 keeps a unit awake 2 503 ticks at the gain 1.75, an oracle of the membrane's rule and the engine agreeing, so under ADR-0044's drive the executor serves 99.99 per cent of the units on every tick at 1 024 and 4 096 units while 0.0018 per cent fire, and at drives sixteen and 256 times sparser, where the network never fires, 72.0 and 7.38 per cent beside the floor written first; the budget Appendix A's unit count, real-time tick and input density must fit together written for per-tick service and for service on arrival, and F-49 opened; ADR-0097), and the integration model decided against that budget (per-tick service kept, Appendix A's three targets met in two modes — a real-time mode with a body attached, whose unit count is what the budget allows, a Target of about 2.6 × 10⁴ units on 64 workers at ADR-0044's density, and an offline mode whose unit count is what memory holds, run slower than real time; F-49 resolved; ADR-0098) exist; the real-time mode's reach measured on the reference platform, the `mmap` path, the shared-memory mappings, core pinning, the tool broker, standardising apart, a chosen target rate, the per-unit gain of §8.8, a reward that measurably changes a behaviour (ADR-0060 read none, ADR-0066 read none through the calibrated instrument, and ADR-0069 read none with the reward addressed, the couplings parting by a few per cent downward; ADR-0072 measured what the trace is made of and walked the gain's ladder to no passing rung; ADR-0074 tried the stimulus that fires each unit once in three shapes and read none that does under the drive, the trace still net depression under each; ADR-0076 built the two injections, read every unit firing once and the trace still net depression by 2.0 per synapse per trial, and exhausted the stimulus side; ADR-0077 tried the background side in both its named settings, the settled network reading the sign at 53 of 64 with the trace net potentiation on average and the controller failing every measure, and under the rule's third step closed H-12 as a no for this task in this regime; ADR-0078 chose the first of the three routes the rule named — a task that asks for a sign rather than a difference — and wrote it as H-13 before any run: on the settled network with the baseline at zero, a reward delivered to the synapses from the presented stimulus onto its assigned readout, asked to raise those couplings and the response the engine reads against the same trials unrewarded, with its criterion and its own stopping rule; the structural rule is deferred, and the operating regime against ADR-0036's criticality is the open question a no on H-13 makes next; ADR-0079 ran H-13 once, the calibration reproducing ADR-0077's settled candidate first, and read **yes**: the assigned couplings rose by 12 to 15 per cent over 512 trials and were still rising, every synapse outside the pairs the image's bit for bit, and the assigned readout's response was above the same trial's with the reward withheld in 128 and 127 trials of the last 128 by six spikes per trial, in both assignments, the selection following it in 106 and 115 — a reading that reopens nothing of H-12; the next decision, by H-13's stopping rule, is an ADR on the reinforced form, a delivery the selection decides, and the operating regime stays open; ADR-0080 took it and wrote H-14 before any run: the task as built, the reward addressed to the selected readout with the outcome's sign, on the settled image with the baseline at zero over 1 536 trials, 80 correct of the last 128 in both assignments, a wrong selection's pair shown by the rules never to consolidate and the shuffled reward read for lock-in; ADR-0081 ran H-14 once, the calibration reproducing ADR-0077 first, and read **yes**: 128 of the last 128 correct in both assignments, the selection crossing 40 of 64 by trial 448 and 384, the answer couplings at 1.35 to 1.45 of the image's and still rising, every synapse outside them the image's bit for bit, the wrong pairs never consolidating as derived; the shuffled reward locked stimulus A onto one readout; the next decision, by H-14's stopping rule, is an ADR choosing what the learning line asks next among the baseline at which every synapse consolidates, the assignment reversed within a run, the operating regime and another size, not taken; ADR-0082 took the baseline first and wrote H-15 before any run: H-14's configuration with the baseline at 0.5, so that every synapse consolidates half of what it pairs, beside a withheld arm, the drift predicted toward readout 1 and no prediction for the verdict, the runs in a test binary of their own; ADR-0083 ran H-15 once, the calibration reproducing ADR-0077 first, and read **no**: 78 of the last 128 correct in the assignment against 80 and 106 in the mirrored, every coupling carried up by a third to a half of the image's by the unrewarded consolidation and faster onto readout 1, the withheld arm drifting there for both stimuli and the readout-0 stimulus deciding each arm as predicted, the reward's part two to nine per cent, the assertion true on every arm, the inhibitory sum at 0.41 of the image's; nothing written beside H-12; the next decision, by H-15's stopping rule, an ADR choosing between the baseline at zero as the configuration the engine learns in and the operating regime, not taken; the harness shared as one module between `instrument.rs` and the new `everywhere.rs`; ADR-0085 took the reward's gate and wrote H-16 before any run: every excitatory synapse under the gate and every inhibitory one under a baseline of its own, 0.5, a parameter of the image that brief 039 builds, splitting H-15's no into its excitatory and inhibitory parts; ADR-0086 built it — the inhibitory baseline in the modulator section's reserved bytes as a flag and a value, format 15, unset today's rule bit for bit — and ADR-0087 ran H-16 once, the calibration reproducing ADR-0077 first, and read **yes**: 128 of the last 128 correct in both assignments, the crossing at 512 and 256 beside H-14's 448 and 384, the inhibitory sum falling to 0.41 of the image's as under H-15 while every excitatory synapse outside the answer pairs stayed the image's bit for bit, so H-15's no was the excitatory synapses' unrewarded consolidation; the configuration the engine learns in named — every excitatory synapse under the reward's gate, every inhibitory one under a baseline of its own — and the next decision, by H-16's stopping rule, an ADR choosing among the reward-prediction error, the assignment reversed within a run, the operating regime and another size, not taken; the runs in `tests/inhibition.rs` on the shared harness; ADR-0089 took the reversal and wrote H-17 before any run — 3 072 trials with the mapping flipped at the half, in both directions — predicted to fail, because the gate is a deterministic comparison with no exploration and a pair consolidates only when it is selected and correct, and run for the size of that failure, which is the measured need an exploration or a depression would have to meet; ADR-0090 amended H-17's assertion before any run — the first trial after the flip still consolidates the last reward of the first mapping, so the old answer's pairs are held from the 1 537th trial — and ADR-0091 ran H-17 once and read **no**: 128 of the last 128 correct before the flip and 0 of the last 128 after it in both arms, the first half H-16's arm bit for bit; over the 1 536 trials after the flip the other readout selected 2 and 1 times with as many rewards, the old answer's readout counting 2.2 to 3.2 times the new's, 16 to 24 spikes a trial, and the old answer's pairs unmoved through 1 534 punishments each while their trace went on pairing +5 to +10 per synapse per trial; the inhibitory sum still falling, to 0.17 of the image's; the next decision, by H-17's stopping rule, an ADR choosing between an exploration in the selection and a depression under the gate with those readings as its need, not taken; the two arms two weekly tests in `tests/inhibition.rs`, the flip `run_on_flipped` on the shared harness; ADR-0093 took the depression and wrote H-18 before any run — a signed gate, a parameter of the image brief 041 builds, under which an addressed excitatory synapse consolidates under `clamp(baseline + dopamine, −1, 1)` so a punishment moves its weight against its trace and spends the trace by as much, over 4 608 trials with the mapping flipped at 1 536, no prediction for the verdict; ADR-0094 built it — the modulator section's `[25]`, format 16, `consolidate_signed` beside `consolidate`, unset the rule before it bit for bit — ADR-0095 amended H-18's stopping rule before any run for the one whole-image pin the format moves, and ADR-0096 ran H-18 once and read **yes**: 128 of the last 128 correct before the flip and 128 of the last 128 at the 4 608th trial in both arms, the old answer's pairs punished back to 0.95 to 1.06 of the image's couplings as each stimulus's selection crossed in the fourteenth to the twentieth block after the flip, a punishment potentiating about a fifth of the synapse-trials it reached and depressing slightly more, the inhibitory sum stopping at 0.11 of the image's; the configuration the engine learns and revises in named — the excitatory synapses under the reward's gate with the signed gate set, the inhibitory ones under a baseline of their own — and the next decision, by H-18's stopping rule, an ADR choosing among the reward-prediction error, a schedule of reversals, the operating regime and another size, not taken; the two arms two weekly tests in `tests/inhibition.rs`, the oracle's signed branch `earned_run_signed` on the shared harness)
and every subsystem's real dynamics do not. The whitepaper's
[§1.6](docs/WHITEPAPER.md#16-implementation-status-at-a-glance) is the table of what is built;
[§11](docs/WHITEPAPER.md#11-risks-and-technical-debt) is the numbered list of what is wrong.

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
cargo test --workspace --release --locked -- --ignored exhaustive   # before a release; the weekly job runs it in four shards, dealt by cost (ADR-0073, ADR-0088, ADR-0092)
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
