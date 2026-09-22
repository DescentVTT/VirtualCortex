---
status: archived
date: 2026-09-22
---

> **Executed 2026-09-23 in pull request #109.** Writes ADR-0086 (the inhibitory baseline built: ADR-0085's
> parameter in the modulator section's reserved bytes — a flag byte at `[24]` and a Q16.16 value at
> `[28..32)` — the image format moved from 14 to 15, unset encoded as the zeros a format-14 writer left,
> the configuration taking an `Option`, a third modulation published beside ADR-0068's two and picked
> by the block's polarity in the fan-out; unset, every pinned number and the determinism pin stand bit
> for bit) and ADR-0087 (inhibition off the reward's gate, measured: the calibration reproduced
> ADR-0077's settled candidate with the parameter unset and a frozen block from its zero image bit for
> bit before any rewarded run; the three arms run once from the frozen image with the inhibitory
> baseline written at 0.5, in the new `tests/inhibition.rs` on the shared harness). **H-16's answer:
> yes** — the correct selections over the last 128 trials numbered 128 of 128 in the assignment and in
> the mirrored assignment, the crossing at trials 512 and 256 beside H-14's 448 and 384; the inhibitory
> sum fell in every block to 0.41 of the image's, H-15's course block for block, as predicted; every
> excitatory synapse outside the two answer pairs ended the run the image's bit for bit and none moved
> with the reward withheld, the assertion held on every arm; the withheld arm's lean read a coin either
> way, so ADR-0085's second predicted reading did not hold as written for A and is recorded so. H-15's
> no is thereby the excitatory synapses' unrewarded consolidation, not the inhibitory rule's course.
> **The step of H-16's stopping rule reached: step 3 — H-16 recorded yes with its scope, the
> configuration the engine learns in named (every excitatory synapse under the reward's gate, every
> inhibitory synapse under a baseline of its own), and the next decision an ADR choosing among the
> reward-prediction error, the assignment reversed within a run, the operating regime and another size;
> this round did not take it.** No finding. Image format 15; the determinism pin untouched; no constant
> of the instrument or of H-16 moved. Every deliverable is done; notes under the boxes say what each
> read. Relative links gained one `../` so that they resolve from `archive/`; no other word, claim or
> figure changed.
> *The body below describes the tree before execution and is not maintained.*

# Brief 039: Inhibition off the reward's gate — the inhibitory baseline built as ADR-0085 decided it, then H-16 run once under the criterion and the stopping rule written before it

## Mission

**This brief builds one mechanism and runs H-16 once.** H-15 read no ([ADR-0083](../../docs/adr/0083-plasticity-everywhere-measured.md)):
with the modulation baseline at 0.5 the reinforced form reached 78 of the last 128 in the assignment, two short,
and the no had two parts one run could not separate — the excitatory synapses' unrewarded consolidation, which
leaned every stimulus–readout pair toward readout 1, and the inhibitory rule's course, which halved the arena's
inhibition in twenty-four blocks. [ADR-0085](../../docs/adr/0085-inhibition-off-the-reward-gate.md) chose the route
that keeps the reward's gate — the configuration H-14 learned in — and took its named cost first: **an
inhibitory synapse may consolidate under a baseline of its own**, a parameter of the image, while every
excitatory synapse stays under the reward's gate. It wrote **H-16** in whitepaper §11.1 before this brief
existed: H-14's configuration with that inhibitory baseline at 0.5.

When the round is done, the tree holds: the inhibitory baseline in the executor and the image, format 15, with
its own ADR, its tests and every pinned number of the tree standing when it is unset; the calibration reproduced
(or the round stopped there); the three arms as weekly `exhaustive` tests with their tables pinned; the
assertion that every excitatory synapse outside the answer pairs ends the run as the image holds it, held; the
verdict; H-16 checked in §11.1 with its scope; and **the step of H-16's stopping rule reached**, with the next
decision it names and does not take.

---

## Standing directives

- **Latest ≠ Newest** (whitepaper §2.1, `CLAUDE.md` principle 4). The one mechanism is ADR-0085's: the
  inhibitory rule of Vogels et al. (2011) consolidating under a baseline of its own instead of the reward's
  modulation, which is that rule as it was written — homeostatic, not reward-modulated. **Nothing else is
  adopted**: no new learning rule, gradient, surrogate gradient, e-prop, reward predictor, critic,
  reward-prediction error, eligibility variant, structural plasticity, per-unit scaling, second excitatory
  baseline, or 2025–2026 method; no dependency, no new tool, no version bump of a tool.
- Every claim is Implemented, Specified, Target or Hypothesis. ADR-0085's prediction — **yes**, with the
  inhibitory sum falling in every block and the withheld arm's excitatory weights at the image's — is a
  **Hypothesis** and is reported as one ([ADR-0010](../../docs/adr/0010-measured-or-target.md)). Nothing here says
  anything about 256 units or Appendix A's scale. No timing figure from a developer machine.
- The repository wins over the document; a disagreement is a numbered finding in whitepaper §11. Every number of
  ADR-0053 to ADR-0084 stays in them and their tests keep pinning them.
- No `f32`/`f64` anywhere, oracles included; every operation on a state field saturates or wraps by name
  ([ADR-0029](../../docs/adr/0029-structural-enforcement.md)); plain arithmetic operators are a Clippy error.
- Every state crate stays `#![no_std]` with no dependency; `unsafe` only in the runtime, under
  [ADR-0023](../../docs/adr/0023-executor.md)'s invariant; every loop ends by construction
  ([ADR-0062](../../docs/adr/0062-the-first-complete-sweeps-list.md)).
- **A change to a record's bytes, reserved ones included, moves `CortexFileHeader::version`, updates the record's
  table in whitepaper §5.2 and gets a changelog entry** (`CLAUDE.md`). A parameter that changes what a run does
  is part of the image, not of a configuration (§8.3).
- **Nothing is chosen after a rewarded run.** H-16's constants, arms, criterion, assertion, prediction and
  stopping rule are ADR-0085's; the round's integer form of the criterion is committed **before the first
  rewarded run**, and no clause is dropped or added because of what a run read.
- The determinism pin of [ADR-0030](../../docs/adr/0030-verification-governance.md) does not move. Every mutant
  `cargo-mutants` makes in the lines the change touches must be caught; a new rule carries a test over the
  lattice of `testkit/prop.rs`; every number an arithmetic oracle can produce is computed by it before the test
  that asserts it — and **the engine is read before a description of it is trusted**, this brief's included.
- A heavy run is an `#[ignore]`d test whose name contains `exhaustive` and runs in the weekly job; the runtime's
  gate grows by at most one test for H-16's runs ([ADR-0061](../../docs/adr/0061-the-learning-runs-leave-the-gate.md)),
  beside the mechanism's own unit and property tests.
- The engine never amends its own code ([ADR-0031](../../docs/adr/0031-policy-amendment.md)); **the inhibitory
  baseline is not a registry entry** (F-37: the behaviour gate admits no plasticity parameter). Conventional
  Commits with a real body; never commit on `main`; the required checks keep their names.

## Context

Re-derived on 2026-09-22 against `main` with ADR-0084 and ADR-0085 beside it. Line numbers move; the symbols and
the quoted sentences are what to re-derive.

1. **What H-15 read** ([ADR-0083](../../docs/adr/0083-plasticity-everywhere-measured.md)). At a baseline of 0.5,
   every stimulus–readout pair rose by a third to a half of the image's in twenty-four blocks, faster onto
   readout 1; "the reward's differential on a pair is a tenth of that drift"; the withheld arm selected readout 1
   for both stimuli. The inhibitory sum fell in every block, from 0.974 of the image's after the first to
   **0.411** after the twenty-fourth, alike in all three arms; outside the four pairs the excitatory weights fell
   by about a tenth (ADR-0055's depression); a readout's count in the task's window rose from about 10 per trial
   to 43 to 51; the sight read 62 then 64; the stimulus still fired once.
2. **What H-14 read** ([ADR-0081](../../docs/adr/0081-the-reinforced-form-measured.md)), at a baseline of zero: 128
   of the last 128 in both assignments, the selection crossing 40 of 64 at trials 448 and 384, every wrong pair
   the image's to the LSB, the answer couplings at 1.35 to 1.45 and still rising.
3. **H-16 and the rule, as ADR-0085 wrote them** (whitepaper §11.1 and the ADR, which is the authority where
   this summary is shorter). The rule: when the inhibitory baseline is set, every synapse of an inhibitory block
   consolidates under `clamp(inhibitory baseline, 0, 1)`, addressed or not; every excitatory synapse as today;
   unset, every synapse as today. H-16: H-14's configuration — the settled image, the baseline **zero**, ADR-0076's
   stimulus, `Delivery::Addressed`, `Feedback::Answer`, 1 536 trials — with the inhibitory baseline at **0.5**;
   the assignment, the mirrored assignment and the reward withheld; at least 80 correct of the last 128 in both
   rewarded arms; the assertion; the readings; the stopping rule.
4. **The modulation as the engine applies it** (`runtime/cortex-runtime/src/executor.rs`). The fan-out reads a
   block's polarity from its presynaptic unit's flags (`Polarity::of_flags`, [ADR-0049](../../docs/adr/0049-dale-principle-in-plasticity.md)),
   computes each slot's modulation with `Modulations::for_synapse` (the addressed one for a synapse from an
   addressed source onto an addressed target, the one at rest otherwise, `Modulations::of` once per tick), runs
   `step_stdp_all` and then `consolidate_each(block, &per_slot, polarity)`. The polarity decides the half of the
   width a weight moves in and the pair rule (`SynapseBlock::step_stdp`: an inhibitory slot loses
   `istdp_alpha_q1_15` at every presynaptic spike and gains symmetrically on near pairings, ADR-0053); it does not
   decide the modulation. `Config::modulation_baseline_q16` defaults to 1.0.
5. **The image's modulator section** (`runtime/cortex-runtime/src/image.rs`): one 64-byte record — the
   modulator's 16 bytes, the baseline at `[16..20)`, the inhibitory rule's target period at `[20..24)`, and 40
   reserved bytes that decode refuses unless zero (`ImageError::ReservedNotZero`). The format is 14. The harness's
   `frozen_image` patches the baseline's four bytes.
6. **The harness** (`runtime/cortex-runtime/tests/instrument/harness.rs`, shared by `instrument.rs` and
   `everywhere.rs` since brief 038): `settled_image`, `calibration_holds`, `run_on`, `last_correct`, the oracle
   (`Composer::baseline` and ADR-0079's consolidation replay), which consolidates the stimulus–readout synapses —
   all excitatory — and does not model inhibitory ones. Since [ADR-0084](../../docs/adr/0084-the-shards-take-tests.md)
   the weekly shards take tests, not binaries, so **a new test binary may take any name**.
7. **The budget and the dispatch** ([ADR-0084](../../docs/adr/0084-the-shards-take-tests.md),
   [ADR-0075](../../docs/adr/0075-the-dispatch-scope-follows-the-diff.md)). After ADR-0084 the three exhaustive shards
   read 30, 25 and 36 per cent of their bound; H-16's runs are about H-15's size (twenty minutes of wall clock on
   the runner). **This brief names no dispatch scope**: the round takes it from its own diff under ADR-0075, which
   sends a change under `src/` through the whole-tree sweep, and says which clause applied.

## Deliverables

- [x] **The mechanism (the next free ADR number; `ls docs/adr`)** (`depends-on: ADR-0085`). The inhibitory
  baseline in the executor's configuration and state, read and written between ticks as the baseline is; the
  fan-out consolidating every slot of an inhibitory block under `clamp(inhibitory baseline, 0, 1)` when it is
  set, and every slot as today when it is unset; the parameter in the image, **the format moved from 14 to 15**,
  a format-14 image read as unset, the placement recorded in the ADR; whitepaper §5.2's modulator record table,
  §8.8's three-factor row and the changelog. Tests: the rule at its edges (set and unset; addressed and not;
  a positive and a negative dopamine signal reaching no inhibitory slot when set; the clamp), over the lattice of
  `testkit/prop.rs`; the image round trip at format 15 and a format-14 image decoding as unset; a bad value
  refused as the baseline's is. **Every pinned number of the tree and the determinism pin unchanged with the
  parameter unset.**
  **Done** (ADR-0086): `Config::inhibitory_baseline_q16: Option<i32>`, the third value of `Modulations` and `for_synapse` by the block's polarity, the word published beside ADR-0068's two; the modulator section's flag at `[24]` and value at `[28..32)`, format 15, a version-14 record's zeros read as unset (its header refused as every foreign version is, L-6); the tests named in the ADR; 625 tests passing in the debug profile where 620 did, the determinism pin unmoved, every `exhaustive` pin rerun in the dispatch.
- [x] **The calibration, before any rewarded run.** The settled engine built and held to ADR-0077's tables; a frozen
  first block from the image with the baseline at zero and the inhibitory baseline unset, held to ADR-0077's
  frozen run. **A mismatch — or any pinned number that moved — stops the round**: no rewarded run, a numbered
  finding in §11.
  **Done** (ADR-0087): the lead-in's 96 windows, the quiet run and the image's sums, gain and step held to ADR-0077's; a frozen block from the zero image, the inhibitory baseline asserted unset, ADR-0077's frozen run bit for bit; no pinned number moved.
- [x] **The constants commit**, preceding the first commit that holds a rewarded outcome: the criterion as an
  integer rule over the pinned-table shape (`last_correct` of each rewarded arm at least `REWARDED_MIN`), the
  baseline zero and the inhibitory baseline 0.5, the arms and their feedback, ADR-0085's prediction and its
  predicted readings written as constants, the assertion's shape, and every constant of ADR-0065, ADR-0076,
  ADR-0077, ADR-0080 and ADR-0085 restated unchanged.
  **Done**: `aa35d22`, `tests/inhibition.rs` with its constants, its rules and its gate and every table empty, before the first rewarded run.
- [x] **The three arms** — the assignment, the mirrored assignment, the reward withheld — each 1 536 trials from the
  one image with the inhibitory baseline set to 0.5, as weekly `exhaustive` tests, their tables pinned per block:
  the correct selections and the selections per stimulus and readout, the four stimulus–readout couplings, the
  arena's excitatory and inhibitory sums, the readout units' rate and a readout's count in the task's window, the
  sight, the stimulus's volley and its spikes after it, and the dopamine signal at the trial's end.
  **Done**: `inhibition_off_the_gate_at_1024_units_exhaustive`, the tables `INHIBITION_*_1024` pinned from one run and reproduced by a second on the same machine.
- [x] **The assertion.** In both rewarded arms every excitatory synapse outside the two answer pairs ends the run
  as the image holds it, bit for bit, and in the withheld arm every excitatory synapse does; the oracle agrees
  with the record on the stimulus–readout synapses at every trial. If it fails, a numbered finding against
  ADR-0080's derivation as ADR-0085 extends it, reported beside the verdict and not in place of it.
  **Done**, held on every arm: the excitatory reach (1 584, 0), (1 604, 0) and (0, 0) — every synapse of the answer pairs and none outside, none with the reward withheld — the inhibitory synapses moving in every arm (6 491 to 6 497); the oracle held at every trial; ADR-0080's derivation true on every arm; no finding.
- [x] **The verdict.** The rule committed first, over the pinned tables: **H-16 is yes or no**, and §11.1's H-16 is
  checked with the result and its scope — this network, this delivery, this regime, the baseline zero and the
  inhibitory baseline 0.5, 1 536 trials — and its stopping rule's item with the step reached. Nothing is written
  beside H-12. The ADR states the next decision H-16's rule makes **and does not take it**.
  **Done** (ADR-0087): yes, 128 of 128 in both assignments; §11.1's H-16 checked with its scope, its stopping rule at step 3, the configuration named, the next decision named and not taken; nothing beside H-12.
- [x] **The readings, in every branch.** ADR-0085's predicted readings beside what the runs read: the inhibitory
  sum per block beside H-15's course; the withheld arm's excitatory weights and selection; the crossings of 40 of
  64 beside H-14's 448 and 384; the couplings' course; the readout's rate and count per block — and, if the
  selection stops moving, how much louder the readouts were when it stopped; the sight and whether the stimulus
  still fires once; the composition on the answer pairs in the first and the last block; the dopamine signal.
  **Done** (ADR-0087): the inhibitory sum 0.420, 0.409 and 0.418 of the image's after twenty-four blocks beside H-15's 0.411, falling in every block as predicted; the withheld arm's excitatory weights the image's and its lean readout 0 for A and 1 for B against the frozen block's 1 and 1, a coin either way, the reading not holding as written for A; the crossings 512 and 256 beside H-14's 448 and 384; the answer pairs at 1.356, 1.365, 1.455 and 1.341; the readouts' background rate the image's throughout and the answer readout's count 10 to 30 per trial, the withheld arm's 9 to 12; the sight 62 to 64 and the stimulus firing once; the composition +0.5 to +6.7 per synapse per trial on A→R0; the signal at the fixed point.
- [x] **The gate.** The mechanism's unit and property tests; and at most one runtime test for H-16, running no whole
  run: the criterion's rule at its edges over tables written by hand, and a few trials with the inhibitory
  baseline set, the excitatory synapses outside the answer pairs unmoved and the inhibitory ones moved.
  **Done**: the mechanism's five tests (ADR-0086) and one runtime test for H-16, `the_first_eight_trials_of_inhibition_off_the_gate_at_1024_units_and_the_rules_over_their_tables`.
- [x] **The evidence.** A weekly dispatched on this round's branch at the scope ADR-0075 gives **this round's own
  diff**, the clause that applied stated in the ADR; green in every job it runs, every pinned number reproduced,
  and the sweep's survivors, if any, dispositioned; its run id and the shards' times in the ADR.
  **Done** (ADR-0087's evidence): files under `src/` changed, so ADR-0075's first clause applied and the weekly was dispatched at `scope=both`; the run id, the shards' times and the sweep's result are in the ADR.
- [x] **The documents, in the same pull request.** Whitepaper §5.2 (the modulator record and the format), §8.8,
  §6.5's loop, §11.1's H-16 and its stopping rule (the step reached), §9; the ADR index; `CHANGELOG.md`;
  `README.md`; `CLAUDE.md`'s opening paragraph and its invariants where the format is named; `docs/zh-TW`'s
  reader's guide as the result requires. The whitepaper's version moves in **both** declarations with its date
  ([ADR-0064](../../docs/adr/0064-the-documentation-gate-and-the-version.md)), in the same commit as any edit to it.
  **Done**: whitepaper 4.38.0 (the mechanism: §5.2.2, §5.2.14, §8.7, §8.8, §9, §11.1's note) and 4.39.0 (the measurement: §11.1, §6.5 and its directives, §8.8, §9); the ADR index; `CHANGELOG.md`; `README.md`; `CLAUDE.md`; the zh-TW guide.
- [x] **The brief archived** as `briefs/README.md` says, every deliverable dispositioned, the frozen banner naming
  the pull request, the ADRs, **H-16's answer and the step of its stopping rule reached**.
  **Done**: this file.

## Not empowered

- **No other change to the engine's rules**: the pair rule, the trace, the modulator, the membrane, the controller,
  the inhibitory target period, the sleep constants and the reference prior stay as they are. **Unset, the
  inhibitory baseline is today's rule bit for bit.** No reward-prediction error, no critic, no second excitatory
  baseline, no consolidation at the trial's end.
- **No constant of H-16 moves**: not the baseline of zero, the inhibitory baseline of 0.5, the 1 536 trials, the
  reward of 1.0, the feedback of each arm, the delivery, the window, the trial, the seed, the last 128 trials, the
  mark of 80, or the tie counted as not correct. They are ADR-0085's.
- **No other inhibitory baseline** is run, before or after the rewarded runs.
- **The network is ADR-0077's settled engine**, built by its rule. **The stimulus is ADR-0076's, pinned.** The gain
  stays 1.75 as the image holds it, the controller off. No change to the task.
- **No clause added on the withheld arm**, and none dropped from the rewarded ones.
- **No rewarded run unless the calibration reproduces and every pinned number stands.** No constant chosen or moved
  after a rewarded run; no criterion changed after a run; **no second attempt** at H-16 in this configuration.
- **The round does not take the next decision** its answer names, and writes nothing beside H-12.
- No registry entry for the inhibitory baseline. No new crate; no dependency in a state crate; no `unsafe` outside
  the runtime's arena access; no float anywhere.
- No change to `.github/workflows/ci.yml`, `scripts/exhaustive-shard.sh` or the sweep's configuration. No move of
  the determinism pin, and no edit to a number a prior round pinned.
- "Learns" only with its scope: this task, these arms, these 1 536 trials, this network at 1 024 units, the
  baseline zero and the inhibitory baseline 0.5.

## Architectural empowerment

The executing session may replace any instruction above with a better decision, recorded in the ADR that owns it:
where the inhibitory baseline lives in the configuration and the image (a field of the modulator record's reserved
bytes or another section), how "unset" is encoded (a sentinel, a flag, or a format-14 default), and whether the
configuration takes it as an `Option` or a value with a sentinel — so long as unset is today's rule bit for bit,
a format-14 image reads as unset, and the format moves to 15; how the fan-out selects the modulation for an
inhibitory block (a third value of `Modulations`, or a branch on the polarity), so long as the hot path gains no
allocation and no syscall; whether H-16's runs live in `everywhere.rs`, a new binary or `instrument.rs`; whether
the calibration and the three arms are one test or several; whether the oracle models the inhibitory synapses or
the readings read them from the record; how often the composition is read, so long as the first and the last
block are; whether the round writes one ADR for the mechanism and one for the measurement, or one for both; and
which numbers the ADRs restate. **A defect found in H-16's criterion, assertion or stopping rule, or in ADR-0085's
rule, before the first rewarded run** is written as an ADR amending ADR-0085, with the reason, and committed before
that run; never after it. The round may not reach the standing directives, the whitepaper's invariants, the
constraints in `CLAUDE.md`, or H-16's criterion and stopping rule by any other route.

## Verification

```bash
cargo check --workspace --all-targets --locked
cargo test --workspace --locked
cargo test --workspace --release --locked
cargo test --workspace --release --locked -- --ignored exhaustive --list
cargo test -p cortex-runtime --release --locked -- --ignored exhaustive
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --locked -- -D warnings
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --locked
cargo bench -p cortex-bench --bench hot_path --locked -- --test
cargo +1.85 check --workspace --all-targets --locked
cargo +1.85 test --workspace --locked
npm ci
npm run spec
git diff main...HEAD > target/pr.diff && cargo mutants --workspace --in-diff target/pr.diff
gh workflow run ci.yml --ref <this round's branch> -f scope=<what ADR-0075 gives this diff>
```

Every command exits 0; the in-diff gate reports no survivor in CI. The dispatched run is green in every job it runs
and reproduces every pinned number. The constants commit precedes any commit that holds a rewarded outcome. The
determinism pin and every number a prior round pinned are unchanged; the image format is 15 and a format-14 image
decodes.

## Report

The closing message states: the mechanism as built — where the parameter lives, how unset is encoded, the format,
and that every pinned number and the determinism pin stood with it unset; **whether the calibration reproduced**;
**H-16's answer, yes or no**, with the correct selections of the last 128 in each rewarded arm against 80, and
**the step of H-16's stopping rule reached** and the next decision it names; whether the assertion held; ADR-0085's
prediction and predicted readings beside the run — the inhibitory sum's course beside H-15's, the withheld arm, the
crossings beside H-14's; the readouts' rate and count and, if the selection stopped, how loud they were; the sight;
the composition in the first and the last block; the dopamine signal; the scope ADR-0075 gave this diff and the
clause that applied; the shards' times; what the mutation gate and the sweep found; what was not done and why; and
what the re-examination after the round recommends next.
