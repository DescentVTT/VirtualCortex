---
status: archived
date: 2026-09-24
---

> **Executed 2026-09-24 in pull request #117.** Writes ADR-0094 (the signed gate built: the modulator
> section's `[25]` its flag, the image format moved from 15 to 16; `cortex-neuromod`'s `signed_modulation` and
> `cortex-core`'s `consolidate_signed` beside `modulation` and `consolidate`, which are untouched; below zero
> the weight moved against its trace's sign and the trace spent by what the magnitude moved; the addressed word
> the coordinator already publishes carries the signed value while the gate is set, an inhibitory slot taking
> it at no less than zero; unset, every run the run it was, bit for bit, the determinism pin unmoved),
> ADR-0095 (H-18's stopping rule, step 2, amended before any run: the one whole-image pin of the tree, H-17's
> image CRC, moves with the header's version, so the numbers that stop a round are the engine's and the
> records'; that CRC re-pinned at format 16 and the image with its header written back to 15 held to the CRC
> H-17 read) and ADR-0096 (the punished pair, measured: the calibration reproduced ADR-0077's settled
> candidate before each rewarded run; the two arms run once in `tests/inhibition.rs`, two weekly tests, the
> oracle's signed branch `earned_run_signed` on the shared harness). **H-18's answer: yes** — clause 1 read
> 128 and 128 and clause 2 read 128 and 128, against 80; after the flip the old answer's pairs, punished
> against their traces, fell back to 0.95 to 1.06 of the image's couplings while each stimulus's selection
> crossed in the fourteenth to the twentieth block after it; a punishment potentiated 18 to 24 per cent of the
> addressed synapse-trials it reached and depressed slightly more; ADR-0093's three predicted readings held;
> the assertion held in both arms. **The step of H-18's stopping rule reached: step 3 — H-18 recorded yes with
> its scope, the configuration the engine learns and revises in named, and the next decision an ADR choosing
> among the reward-prediction error, a schedule of reversals, the operating regime and another size; this
> round did not take it.** No finding. Image format 16; the determinism pin untouched; no constant of the
> instrument or of H-18 moved. Every deliverable is done, and the notes under the boxes say what each read and
> where the round decided what the brief left to it. Relative links gained one `../` so that they resolve
> from `archive/`; no other word, claim or figure changed.
> *The body below describes the tree before execution and is not maintained.*

# Brief 041: The punished pair — the signed gate built as ADR-0093 decided it, then H-18 run once under the criterion and the stopping rule written before it

## Mission

**This brief builds one mechanism and runs H-18 once.** H-17 read no
([ADR-0091](../../docs/adr/0091-the-assignment-reversed-measured.md)): an engine that had learned the answer in 128
of the last 128 trials selected the new answer in 0 of the last 128 after the mapping was flipped, because under
the reward's gate a punished pair is frozen, not depressed — the modulation is clamped at zero — and the
deterministic gate almost never tries the other readout. [ADR-0093](../../docs/adr/0093-the-punished-pair-consolidates-against-its-trace.md)
chose, of the two mechanisms H-17's stopping rule named, **the depression under the gate**: a **signed gate**, a
parameter of the image, under which an addressed excitatory synapse consolidates under
`clamp(baseline + dopamine, −1, 1)`, so that a punishment moves its weight **against** its trace's sign and
**spends the trace** by as much. It wrote **H-18** before this brief existed: H-17's configuration with the signed
gate set and a second mapping twice as long, 4 608 trials in all.

When the round is done, the tree holds: the signed gate in the executor, the rule and the image, format 16, with its
own ADR, its tests, and every pinned number of the tree standing when it is unset; the calibration reproduced (or
the round stopped there); the two arms as weekly `exhaustive` tests with their tables pinned; the assertion held;
the verdict; H-18 checked in §11.1 with its scope; **the step of H-18's stopping rule reached**, with the next
decision it names and does not take; and the whole-domain tests' cost table regenerated from this round's own
dispatch ([ADR-0092](../../docs/adr/0092-the-shards-dealt-by-cost.md)).

---

## Standing directives

- **Latest ≠ Newest** (whitepaper §2.1, `CLAUDE.md` principle 4). The one mechanism is ADR-0093's: reward-modulated
  STDP with a signed modulation, Florian (2007) and Frémaux, Sprekeler and Gerstner (2010), with the trace spent so
  a punishment cannot run away. **Nothing else is adopted**: no exploration, noise, temperature or sampling in the
  selection; no reward predictor, critic or reward-prediction error; no new learning rule, gradient, surrogate
  gradient, e-prop, eligibility variant, structural plasticity or 2025–2026 method; no dependency, no new tool.
- Every claim is Implemented, Specified, Target or Hypothesis. ADR-0093 writes **no prediction for the verdict**
  and three predicted readings, each a **Hypothesis** ([ADR-0010](../../docs/adr/0010-measured-or-target.md)); the
  run's length is an estimate from a starting rate and a named slowdown, labelled so. Nothing here says anything
  about 256 units or Appendix A's scale. No timing figure from a developer machine.
- The repository wins over the document; a disagreement is a numbered finding in whitepaper §11. Every number of
  ADR-0053 to ADR-0092 stays in them and their tests keep pinning them.
- No `f32`/`f64` anywhere, oracles included; every operation on a state field saturates or wraps by name
  ([ADR-0029](../../docs/adr/0029-structural-enforcement.md)); plain arithmetic operators are a Clippy error.
- Every state crate stays `#![no_std]` with no dependency; `unsafe` only in the runtime, under
  [ADR-0023](../../docs/adr/0023-executor.md)'s invariant; every loop ends by construction.
- **A change to a record's bytes, reserved ones included, moves `CortexFileHeader::version`, updates the record's
  table in whitepaper §5.2 and gets a changelog entry** (`CLAUDE.md`). A parameter that changes what a run does is
  part of the image (§8.3). The loader refuses a foreign version (L-6); **a brief's sentence does not override L-6**
  ([ADR-0086](../../docs/adr/0086-the-inhibitory-baseline-built.md) read that correctly against brief 039).
- **Nothing is chosen after a rewarded run.** H-18's constants, arms, length, flip, criterion, assertion and
  stopping rule are ADR-0093's; the round's integer form of the criterion is committed **before the first rewarded
  run**, and no clause is dropped or added because of what a run read.
- The determinism pin of [ADR-0030](../../docs/adr/0030-verification-governance.md) does not move. Every mutant
  `cargo-mutants` makes in the lines the change touches must be caught; a new rule carries a test over the lattice
  of `testkit/prop.rs`; every number an arithmetic oracle can produce is computed by it before the test that asserts
  it — and **the engine is read before a description of it is trusted**, this brief's included: ADR-0089's
  assertion was a trial early because a trial's reward is consolidated during the next one, and ADR-0090 caught it
  before any run.
- A heavy run is an `#[ignore]`d test whose name contains `exhaustive` and runs in the weekly job; the runtime's gate
  grows by at most one test for H-18's runs ([ADR-0061](../../docs/adr/0061-the-learning-runs-leave-the-gate.md)),
  beside the mechanism's own unit and property tests.
- The engine never amends its own code ([ADR-0031](../../docs/adr/0031-policy-amendment.md)); **the signed gate is not a
  registry entry** (F-37). Conventional Commits with a real body; never commit on `main`; the required checks keep
  their names.

## Context

Re-derived on 2026-09-24 against `main` with ADR-0092 and ADR-0093 beside it. Line numbers move; the symbols and
the quoted sentences are what to re-derive.

1. **What H-17 read** ([ADR-0091](../../docs/adr/0091-the-assignment-reversed-measured.md)): clause 1 held in both arms
   (128 of 128 before the flip, H-16's arm reproduced bit for bit), clause 2 in neither (0 of 128 after it). After
   the flip the gate selected the new answer 2 and 1 times of 1 536, earned 2 and 1 rewards, the counts' gap at 16
   to 24 spikes; one rewarded selection of the new answer consolidated 162 to 3 415 into its pair against a lead of
   2.2 to 3.0 million; the old pair's trace held +5 to +10 per synapse per trial in each of 1 534 punished trials,
   and the old pair did not move ([ADR-0090](../../docs/adr/0090-the-old-answer-held-from-the-trial-after-the-flip.md)'s
   assertion held).
2. **H-18, as ADR-0093 wrote it** (whitepaper §11.1 and the ADR, which is the authority where this summary is
   shorter): H-17's configuration — the settled image, the excitatory baseline zero, the inhibitory baseline 0.5,
   ADR-0076's stimulus, the task as built, the flip between trial 1 536 and 1 537 by `mirrored` — with **the signed
   gate set** and **4 608 trials, seventy-two blocks**; two arms, each its own test; clause 1 (at least 80 of trials
   1 409 to 1 536 under the first mapping) and clause 2 (at least 80 of trials 4 481 to 4 608 under the second) in
   both; the assertion; the readings; no verdict prediction and three predicted readings; the stopping rule.
3. **The engine's consolidation** (`crates/cortex-core/src/dynamics/synapse.rs`, `crates/cortex-neuromod/src/lib.rs`,
   `runtime/cortex-runtime/src/executor.rs` — **read them**). `NeuromodulatorState::modulation` returns
   `clamp(baseline + dopamine, 0, 1)`. `SynapseBlock::consolidate` clamps the modulation to `[0, 1]`, moves
   `round(|trace| × m)` into the magnitude with the trace's sign, clamps the magnitude to `[0, 2^15)`, and takes
   what the magnitude absorbed out of the trace — "the magnitude and the trace conserve their sum". `Modulations::of`
   publishes the addressed, at-rest and inhibitory modulations once per tick; `for_synapse(addressed, polarity)`
   picks per slot; the fan-out runs `step_stdp_all` then `consolidate_each`. **Every caller of `consolidate` passes a
   modulation in `[0, 1]`.**
4. **The rule ADR-0093 decided.** When the signed gate is set, an addressed **excitatory** synapse consolidates under
   `m = clamp(baseline + dopamine, −1, 1)`: for `m ≥ 0` today's rule bit for bit; for `m < 0` the weight moves
   against the trace's sign by `round(|trace| × |m|)`, clamped to `[0, 2^15)` of the magnitude, and the trace is
   spent toward zero by what the magnitude moved. Every other synapse as today; unset, every synapse as today. A
   synapse whose trace is negative is potentiated by a punishment under this rule, as the literature's signed rule
   does — a reading, not a defect.
5. **The signal under a punishment every trial** (`NeuromodulatorState::reward`, `decay_dopamine` — read them): the
   mirror of the fixed point [ADR-0079](../../docs/adr/0079-the-rewards-direction-measured.md) read under a reward every
   trial — −1.712 after the punishment and −0.712 at the trial's end — since `decay_dopamine` reads the magnitude
   only. A trial's reward is consolidated during the **next** trial (`Task::trial` writes the addressed set and the
   reward after the trial's ticks; ADR-0090).
6. **The image's modulator section** (`runtime/cortex-runtime/src/image.rs`): the modulator's 16 bytes, the baseline
   at `[16..20)`, the inhibitory period at `[20..24)`, the inhibitory baseline's flag at `[24]` and value at
   `[28..32)` (ADR-0086), and reserved bytes at `[25..28)` and `[32..64)` that the loader refuses unless zero. The
   format is 15.
7. **The harness** (`runtime/cortex-runtime/tests/instrument/harness.rs`; `tests/inhibition.rs`):
   `settled_image`, `calibration_holds`, `frozen_from`, `run_on_flipped`, `earned_run`, `earned_run_under`,
   `earned_run_flipped`, and ADR-0079's oracle as ADR-0083, ADR-0087 and ADR-0091 extended it — which consolidates
   under a modulation it clamps to `[0, 1]`, so it needs the signed branch.
8. **The weekly shards** ([ADR-0092](../../docs/adr/0092-the-shards-dealt-by-cost.md)): dealt longest first by
   `scripts/exhaustive-costs.tsv`, a test the table does not know costed at 900 s. **A round that adds or changes a
   whole-domain test regenerates the table from its own dispatch** (`node scripts/exhaustive-costs.mjs from <dir>
   --run <id>` over the run's downloaded `exhaustive-tests-*` artifacts) in its evidence commit, and `npm run
   spec:costs` then holds every line to a test in the tree. **This brief names no dispatch scope**: the round takes
   it from its own diff under [ADR-0075](../../docs/adr/0075-the-dispatch-scope-follows-the-diff.md), which sends a
   change under `src/` or `crates/` through the whole-tree sweep.

## Deliverables

- [x] **The mechanism (the next free ADR number; `ls docs/adr`)** (`depends-on: ADR-0093`). The signed gate in the
  executor's configuration and state, read and written between ticks; the addressed modulation of an excitatory
  synapse clamped to `[−1, 1]` when it is set and to `[0, 1]` when it is not; `SynapseBlock::consolidate` (or a rule
  beside it) taking a modulation in `[−1, 1]` with ADR-0093's negative branch — the weight against the trace's sign,
  the trace spent by what the magnitude moved — and today's rule bit for bit for `m ≥ 0`; the parameter in the
  image, **the format moved from 15 to 16**, the placement recorded in the ADR; whitepaper §5.2's modulator record
  table, §8.8's three-factor row and the changelog. Tests: the negative branch at its edges (a positive and a
  negative trace; a magnitude at zero and at the rail; `m` of −1, −1 LSB and 0; the trace never growing), over the
  lattice of `testkit/prop.rs`; the executor's modulation set and unset, addressed and not, excitatory and
  inhibitory; the image round trip at format 16 and each refusal. **Every pinned number of the tree and the
  determinism pin unchanged with the signed gate unset.**
  **Done** as ADR-0094: the gate a `bool` in the configuration and the executor, a flag byte at `[25]` of the modulator section, format 16; the negative branch a rule beside `consolidate` (`consolidate_signed`), `consolidate` untouched; the signed modulation carried in the addressed word the coordinator already publishes, floored at zero for an inhibitory slot; the tests named in the ADR, one of them over the lattice for each rule; 634 tests passed in the debug profile where 626 did, the determinism pin unmoved.
- [x] **The calibration, before any rewarded run.** The settled engine built and held to ADR-0077's tables; a frozen
  first block from the image with all three parameters unset, held to ADR-0077's frozen run. **A mismatch — or any
  pinned number that moved — stops the round**: no rewarded run, a numbered finding in §11.
  **Done** (ADR-0096): reproduced in each arm's test before its rewarded run. One pinned number moved with the format and not with the gate — H-17's whole-image CRC, whose header carries the version — and ADR-0095 amended ADR-0093's stopping rule for it before any run, under this brief's empowerment, rather than stopping the round on a header field the same ADR ordered moved; the test now holds that image, with its header written back to 15, to the CRC H-17 read.
- [x] **The constants commit**, preceding the first commit that holds a rewarded outcome: the criterion's two clauses
  as integer rules over the pinned-table shape, the 4 608 trials and the flip, the three parameters, the arms,
  ADR-0093's predicted readings written as constants, the assertion's shape, the readings' rules, and every constant
  of ADR-0065, ADR-0076, ADR-0077, ADR-0080, ADR-0085, ADR-0089 and ADR-0093 restated unchanged.
  **Done**: committed before the first rewarded run, after the mechanism and ADR-0095, with the tables empty.
- [x] **The two arms** — the assignment first, the mirrored first — **each its own `exhaustive` test**, each 4 608
  trials from the one image with the signed gate set, the mapping flipped once between trials 1 536 and 1 537 and
  nothing else changed at the flip; their tables pinned per block: the correct selections under the mapping in
  force, the selections per stimulus and readout, the ties, the four couplings, the rewards and punishments, the
  arena's sums, the sight, the stimulus's volley and the dopamine signal.
  **Done** (ADR-0096): two weekly tests in `tests/inhibition.rs`, pinned whole, the first half not being H-16's run under the gate; the moves per block beside the tables the brief names.
- [x] **The assertion.** No excitatory synapse outside the four stimulus–readout pairs moves in either arm, and the
  oracle — extended to the signed branch — agrees with the record's weights, traces and signal at every trial. If it
  fails, a numbered finding, reported beside the verdict and not in place of it.
  **Done**: held in both arms — 3 188 synapses of the four pairs moved and no excitatory synapse outside them; the oracle equal to the record at every trial. No finding.
- [x] **The verdict.** The rule committed first, over the pinned tables: **H-18 is yes or no**, §11.1's H-18 checked
  with the result and its scope, and its stopping rule's item with the step reached. A failure of clause 1 is a no
  about the signed gate, not a failure to replicate H-16. The ADR states the next decision H-18's rule makes **and
  does not take it**.
  **Done**: H-18 is yes, 128 and 128 before the flip and 128 and 128 at the end; §11.1's H-18 checked with its scope; its stopping rule at step 3; the next decision named in ADR-0096 and not taken.
- [x] **The readings, in every branch.** ADR-0093's three predicted readings beside what the runs read; after the
  flip, the old pair's course per block, the trial of the first crossing to the new answer and of the first block at
  40 of 64; in the first half, where the wrong pairs end against the image's; **the fraction of a punished trial's
  addressed synapses whose trace was negative and which the punishment potentiated**; the composition on the answer
  pairs in the first block, the block of the flip and the last; the inhibitory sum, the sight, whether the stimulus
  still fires once, the dopamine signal.
  **Done** (ADR-0096): the three predicted readings true for both stimuli in both arms; the old pairs' course, the first new selection and the crossing per stimulus; the wrong pairs at 0.96 to 0.98 of the image's after the first half; a punishment potentiating 23 to 24 per cent of the addressed synapse-trials before the flip and 18 after it; the composition in the first block, the blocks either side of the flip and the last; the inhibitory sum, the sight, the volley and the signal.
- [x] **The gate.** The mechanism's unit and property tests; and at most one runtime test for H-18, running no whole
  run: the criterion's clauses at their edges over tables written by hand, and a few trials over a punishment on the
  instrument's network with the signed gate set — the addressed pair moving against its trace, every unaddressed
  excitatory synapse unmoved, the oracle held at every trial.
  **Done**: the mechanism's unit and property tests, and one runtime test for H-18 (`a_few_trials_over_a_punishment_at_1024_units_and_the_rules_of_the_punished_pair`), which runs no whole run.
- [x] **The evidence.** A weekly dispatched on this round's branch at the scope ADR-0075 gives **this round's own
  diff**, the clause that applied stated in the ADR; green in every job it runs, every pinned number reproduced, and
  the sweep's survivors, if any, dispositioned; its run id and the four shards' times in the ADR; **and the cost
  table regenerated from that run's artifacts** (ADR-0092), `npm run spec:costs` passing on it.
  **Done** (ADR-0096's evidence): files under `src/` and `crates/` changed, so ADR-0075 sent the dispatch through the whole-tree sweep as well as the whole-domain tests; the run id, the four shards' times, the sweep's survivors and the regenerated cost table are in the ADR.
- [x] **The documents, in the same pull request.** Whitepaper §5.2 (the modulator record and the format), §8.8, §6.5's
  loop, §11.1's H-18 and its stopping rule (the step reached), §9; the ADR index; `CHANGELOG.md`; `README.md`;
  `CLAUDE.md`'s opening paragraph and wherever it names the format; `docs/zh-TW`'s reader's guide as the result
  requires. The whitepaper's version moves in **both** declarations with its date
  ([ADR-0064](../../docs/adr/0064-the-documentation-gate-and-the-version.md)), in the same commit as any edit to it.
  **Done**: whitepaper 4.46.0 to 4.48.0 across the three commits that edit it, each moving both declarations; the ADR index, `CHANGELOG.md`, `README.md`, `CLAUDE.md` and the reader's guide.
- [x] **The brief archived** as `briefs/README.md` says, every deliverable dispositioned, the frozen banner naming the
  pull request, the ADRs, **H-18's answer and the step of its stopping rule reached**.

  **Done**: this file.
## Not empowered

- **No other change to the engine's rules**: the pair rule, the trace's decay, the modulator's reward and decay, the
  membrane, the controller, the inhibitory rule and its baseline, the sleep constants and the reference prior stay as
  they are. **Unset, the signed gate is today's rule bit for bit.** No exploration, no reward-prediction error, no
  signed modulation on an unaddressed or an inhibitory synapse.
- **No constant of H-18 moves**: not the 4 608 trials, the flip's trial, the three parameters, the reward of 1.0, the
  feedback, the delivery, the window, the trial, the seed, the two windows of 128 trials, the mark of 80, or the tie
  counted as not correct. They are ADR-0093's.
- **The network is ADR-0077's settled engine**, built by its rule; **the stimulus is ADR-0076's, pinned**; the gain
  stays 1.75 as the image holds it, the controller off; no change to the task.
- **No clause added or dropped.** No second flip, no schedule of reversals.
- **No rewarded run unless the calibration reproduces and every pinned number stands.** No constant chosen or moved
  after a rewarded run; **no second attempt** at H-18 in this configuration.
- **The round does not take the next decision** its answer names, and writes nothing beside H-12, H-16 or H-17.
- No registry entry for the signed gate. No new crate; no dependency in a state crate; no `unsafe` outside the
  runtime's arena access; no float anywhere.
- **The two arms are two tests.** No change to `.github/workflows/ci.yml`, `scripts/exhaustive-shard.sh`,
  `scripts/exhaustive-costs.mjs` or the sweep's configuration; the cost table changes only by regeneration from this
  round's dispatch.
- "Learns" and "revises" only with their scope: this task, these arms, this flip, 4 608 trials, this network at
  1 024 units, the excitatory gate at zero with the signed gate set and the inhibitory baseline at 0.5.

## Architectural empowerment

The executing session may replace any instruction above with a better decision, recorded in the ADR that owns it:
where the signed gate lives in the configuration and the image (a byte of `[25..28)`, a bit beside the inhibitory
flag, or elsewhere), how "set" is encoded, and whether the configuration takes a `bool` or an enum — so long as unset
is today's rule bit for bit, the loader refuses what the writer never produces, and the format moves to 16; whether
`SynapseBlock::consolidate` takes the signed domain or a sibling rule does, so long as every existing caller is
unchanged and the negative branch is ADR-0093's; how the fan-out selects the signed modulation, so long as the hot
path gains no allocation and no syscall; where H-18's arms live; whether the calibration is its own test; how the
oracle is extended to the signed branch, so long as it is held to the record at every trial; how often the
composition is read, so long as the three named blocks are; whether the round writes one ADR for the mechanism and
one for the measurement, or one for both; and which numbers the ADRs restate. **A defect found in H-18's criterion,
assertion or stopping rule, or in ADR-0093's rule, before the first rewarded run** is written as an ADR amending
ADR-0093, with the reason, and committed before that run; never after it. The round may not reach the standing
directives, the whitepaper's invariants, the constraints in `CLAUDE.md`, or H-18's criterion and stopping rule by
any other route.

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
node scripts/exhaustive-costs.mjs from <the dispatch's downloaded artifacts> --run <its id> > scripts/exhaustive-costs.tsv
```

Every command exits 0; the in-diff gate reports no survivor in CI. The dispatched run is green in every job it runs
and reproduces every pinned number. The constants commit precedes any commit that holds a rewarded outcome. The
determinism pin and every number a prior round pinned are unchanged; the image format is 16.

## Report

The closing message states: the mechanism as built — where the parameter lives, how the negative branch spends the
trace, the format, and that every pinned number and the determinism pin stood with it unset; **whether the
calibration reproduced**; **H-18's answer, yes or no**, with clause 1's and clause 2's counts in both arms against 80,
and **the step of H-18's stopping rule reached** and the next decision it names; whether the assertion held; ADR-0093's
predicted readings beside the run — the old pair's course after the flip, its rate, the wrong pairs after the first
half; the crossing's trial; how often a punishment potentiated; the sight, the inhibitory sum and the stimulus's
volley; the scope ADR-0075 gave this diff and the clause that applied; the four shards' times and the regenerated
cost table; what the mutation gate and the sweep found; what was not done and why; and what the re-examination after
the round recommends next.
