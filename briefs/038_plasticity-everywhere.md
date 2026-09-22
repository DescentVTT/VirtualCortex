---
status: proposed
date: 2026-09-22
---

# Brief 038: Plasticity everywhere — H-15, run once under the criterion, the predicted readings and the stopping rule ADR-0082 wrote before it

## Mission

**This brief runs H-15, and it runs it once.** [ADR-0081](../docs/adr/0081-the-reinforced-form-measured.md)
read H-14 yes: on the network ADR-0077 settled, the task as built — the reward addressed to the synapses
from the presented stimulus onto the readout the engine selected, its sign the outcome's — moved the
selection to the answer in 128 of the last 128 trials in both assignments. It did so with the modulation
baseline at **zero**, under which nothing in the arena moves but the pairs a reward reached.
[ADR-0082](../docs/adr/0082-plasticity-everywhere.md) chose, among the candidates H-14's stopping rule
named, to remove that condition first, and wrote **H-15** in whitepaper §11.1 before this brief existed:
**H-14's configuration with the baseline at 0.5**, every other constant unchanged, so that every synapse
consolidates half of what it pairs and a reward still has room.

When the round is done, the tree holds: the calibration reproduced (or the round stopped there); the three
arms — the assignment, the mirrored assignment and the reward withheld — from one settled image at 0.5, as
weekly `exhaustive` tests in **a test binary of their own**, their tables pinned; the assertion on the
modulation a punishment and a reward leave, held; the criterion's verdict; H-15 checked in §11.1 with its
scope and, after a yes, the sentence beside H-12 that ADR-0082 wrote in advance; and **the step of H-15's
stopping rule reached**, with the next decision it names and does not take.

---

## Standing directives

- **Latest ≠ Newest** (whitepaper §2.1, `CLAUDE.md` principle 4). This round adds **no mechanism**: the task,
  its delivery and its feedback are `task.rs`'s as they stand; the settled image is ADR-0077's; the oracle is
  ADR-0072's as ADR-0079 and ADR-0081 extended it. **No** new learning rule, gradient, surrogate gradient,
  e-prop, reward predictor, critic, eligibility variant, structural plasticity, per-unit scaling, reward of one
  sign, consolidation at the trial's end, or 2025–2026 method; no dependency, no new tool, no version bump of a
  tool.
- Every claim is Implemented, Specified, Target or Hypothesis. ADR-0082 writes **no prediction for the
  verdict**, and predicts two readings — the withheld arm drifting toward readout 1, and the stimulus whose
  answer is readout 0 deciding each rewarded arm — as **Hypotheses** from ADR-0077's terms; they are reported
  as such ([ADR-0010](../docs/adr/0010-measured-or-target.md)). Nothing here says anything about 256 units or
  Appendix A's scale. No timing figure from a developer machine.
- The repository wins over the document; a disagreement is a numbered finding in whitepaper §11. Every number
  of ADR-0053 to ADR-0081 stays in them and their tests keep pinning them.
- No `f32`/`f64` anywhere, oracles included; every operation on a state field saturates or wraps by name
  ([ADR-0029](../docs/adr/0029-structural-enforcement.md)).
- Every loop ends by construction — a run of 1 536 trials is a `for` over the trials, a lead-in is bounded
  ([ADR-0062](../docs/adr/0062-the-first-complete-sweeps-list.md)).
- Every quantity has one owner ([ADR-0016](../docs/adr/0016-thirty-two-crate-architecture.md)). **No new
  crate**; no record changes; the image format stays 14.
- **Nothing is chosen after a rewarded run.** H-15's constants, arms, criterion, assertion, predicted readings
  and stopping rule are ADR-0082's and are not this round's to move; the round's integer form of the criterion
  is committed **before the first rewarded run**, and no clause is dropped or added because of what a run read.
- The determinism pin of [ADR-0030](../docs/adr/0030-verification-governance.md) does not move. The mutation
  gate on the changed lines must pass; every number an arithmetic oracle can produce is computed by it before
  the test that asserts it — and **the engine is read before a description of it is trusted**, this brief's and
  ADR-0082's included: ADR-0078's dopamine figures were an exponent where `decay_dopamine` takes a floor
  (ADR-0079).
- A heavy run is an `#[ignore]`d test whose name contains `exhaustive` and runs in the weekly job; the pull
  request's gate grows by at most one test ([ADR-0061](../docs/adr/0061-the-learning-runs-leave-the-gate.md)).
- The engine never amends its own code ([ADR-0031](../docs/adr/0031-policy-amendment.md)); no registry entry
  (F-37). Conventional Commits with a real body; never commit on `main`; the required checks keep their names.

## Context

Re-derived on 2026-09-22 against `main` at `1e39625` with ADR-0082 beside it. Line numbers move; the symbols
and the quoted sentences are what to re-derive.

1. **What H-14 read at a baseline of zero** ([ADR-0081](../docs/adr/0081-the-reinforced-form-measured.md),
   `REINFORCED_BLOCKS_1024`): the correct selections per block rose from 31 and 28 to 64 in both rewarded arms,
   crossing 40 of 64 at trial 448 in the assignment and 384 in the mirrored assignment; the answer couplings
   rose to 1.366 and 1.348 (assignment) and 1.445 and 1.354 (mirrored) of the image's, still rising; every wrong
   pair stayed the image's to the LSB; the sight read 62 to 64 in every block. The shuffled arm, whose reward
   carried no information, leaned to readout 1: A→R1 1.258 against A→R0 1.036, B→R1 1.168 against B→R0 1.095,
   and A went to readout 1 in 60 of its last 61 presentations.
2. **Why readout 1** ([ADR-0077](../docs/adr/0077-the-background-side.md), `BACKGROUND_1024`): on the settled
   network the trace over the frozen block was net potentiation on average on all four stimulus–readout pairs
   and larger onto readout 1 — **+0.4, +1.6, +0.8 and +1.7 per synapse per trial** on A→R0, A→R1, B→R0 and
   B→R1 (the order ADR-0072 fixed), their sums after the block +0.7, +14.5, +8.4 and +13.6. The image's
   couplings (`IMAGE_COUPLINGS_1024`): A→R0 6 249 552, A→R1 6 698 611, B→R0 6 584 205, B→R1 6 815 470 in Q1.15.
3. **H-15, as ADR-0082 wrote it** (whitepaper §11.1, and the ADR's Decision Outcome, which is the authority
   where this summary is shorter). H-14's configuration — ADR-0077's settled image, the gain 1.75, the
   controller off, ADR-0076's stimulus, `Delivery::Addressed`, `Feedback::Answer`, `REWARD_Q16` (1.0), 1 536
   trials, `LAST_BLOCKS`, `REWARDED_MIN` (80) — with the baseline at **0.5**. The arms: the assignment and the
   mirrored assignment, and the reward withheld (`Feedback::Withheld`) as a reading bounded by no clause. The
   criterion: at least 80 correct of the last 128 in both rewarded arms, a tie not correct. The assertion. The
   predicted readings. The sentence beside H-12 for a yes. The stopping rule: one round; a calibration that does
   not reproduce stops it; yes → an ADR choosing among the reversal at this baseline, where the positive feedback
   stops, the regime and a size; no → an ADR choosing between consolidation gated by the reward alone and the
   regime; no constant moved after a rewarded run, no second attempt.
4. **The baseline, as the tree has it.** `Config::default` sets `modulation_baseline_q16` to 1.0 (ADR-0022's
   rule), at which `Task::check` refuses a reward (`TaskError::RewardAtCeiling`). `BASELINE_Q16` in
   `tests/instrument.rs` is 0.5, brief 027's; `candidate` builds ADR-0077's lead-in under it and asserts it, so
   the settled engine's image **as encoded carries 0.5**. `frozen_image` patches the modulator section's
   baseline (bytes 16 to 20 of the section) to zero and re-computes its CRC; `frozen_from` decodes and asserts
   the baseline zero. H-15's arms need the image at 0.5 and a decode that does not assert zero; the calibration
   still needs the zero image.
5. **The modulation under 0.5** (`Modulations::for_synapse`, `NeuromodulatorState::modulation`, `reward`,
   `decay_dopamine` — **read them**). An addressed synapse consolidates under `clamp(0.5 + signal, 0, 1)` and
   every other under 0.5. `decay_dopamine` moves the signal toward zero by `max(|signal| >> DOPAMINE_TAU_SHIFT,
   1)`, never past zero and reading only the magnitude, so the signal at a trial's end lies between −0.712 and
   0.712, the fixed points of a punishment and of a reward every trial. Hence ADR-0082's assertion: in the trial
   after a wrong selection the addressed (wrong) pair consolidates under at most 0.212, and in the trial after a
   correct one the addressed (answer) pair under at least 0.788.
6. **The harness** (`runtime/cortex-runtime/tests/instrument.rs`): `candidate`, `lead_in_until_settled`,
   `settled_within`, `quiet`, `frozen_image`, `frozen_from`, `settled_image`, `calibration_holds`, `run_on`,
   `last_correct`, and ADR-0079's and ADR-0081's oracle (`Composer::taught`, `signal_course`, `decayed_signal`,
   `consolidated`), which consolidates the stimulus–readout synapses under the modulation the executor published
   where the synapse is addressed and **zero elsewhere** — under 0.5 it must consolidate every one of them under
   0.5 where it is not addressed. `run_on` asserts every unit a source and a target before the first trial.
7. **Where the runs go** ([ADR-0073](../docs/adr/0073-the-whole-domain-tests-sharded.md), ADR-0081, ADR-0082).
   `instrument.rs` took **4 047 s of its shard's 7 200** in ADR-0081's dispatch, H-14's test alone 1 222 s of
   wall clock on a developer machine and 1 017 s added on the runner. `scripts/exhaustive-shard.sh` pipes the
   paths `--list` prints through `sort` and gives shard $k$ the binaries whose position $NR$ has
   $NR \bmod 3 = k$: today `cortex_core`, `instrument`, `learning`, `reference` — so `instrument` is alone in
   shard 2. **A new binary whose name sorts after `instrument` puts `reference` or itself into `instrument`'s
   shard** (4 047 + 1 241 s is 73 per cent); one that sorts before it leaves `instrument` alone. **A module that
   includes `instrument.rs` whole would compile its twenty-eight `exhaustive` tests into the new binary as well**
   and run them twice; `--list` after the change is the check. **This brief names no dispatch scope**: the round
   takes it from its own diff under [ADR-0075](../docs/adr/0075-the-dispatch-scope-follows-the-diff.md).

## Deliverables

- [ ] **The calibration, before anything else (the next free ADR number; `ls docs/adr`)**
  (`depends-on: ADR-0082`; ADR-0077, ADR-0081 and H-15's stopping rule named). The settled engine built and held
  to ADR-0077's tables; before any rewarded run a frozen first block from the **zero** image held to ADR-0077's
  frozen run — the stimulus firing once, the sight 62, the sign 53. **A mismatch stops the round**: no rewarded
  run, a numbered finding in §11, and the ADR says what differed.
- [ ] **The 0.5 image.** The same quiescent engine's image with its baseline at 0.5 — as encoded, or patched as
  `frozen_image` patches zero — decoded with the baseline asserted 0.5, the sums, the gain and the step asserted
  carried, the clock resumed where the image was written.
- [ ] **The binary.** The runs in a test binary of their own whose name sorts before `instrument` in the paths
  `--list` prints, the harness shared with `instrument.rs` rather than copied, and `--list` read after the change:
  `instrument.rs` still holds its twenty-eight `exhaustive` tests, the new binary holds only its own, and the ADR
  states which binaries each shard takes.
- [ ] **The constants commit**, preceding the first commit that holds a rewarded outcome: the criterion as an
  integer rule over the pinned-table shape (`last_correct` of each rewarded arm at least `REWARDED_MIN`, both
  arms), the baseline, the arms and their feedback, ADR-0082's two predicted readings written as constants, the
  assertion's shape, and every constant of ADR-0065, ADR-0076, ADR-0077, ADR-0080 and ADR-0082 restated
  unchanged.
- [ ] **The three arms** — the assignment, the mirrored assignment, the reward withheld — each 1 536 trials from
  the one 0.5 image under `Delivery::Addressed`, as weekly `exhaustive` tests, their tables pinned per block: the
  correct selections and the selections per stimulus and readout, the four stimulus–readout couplings, the
  arena's excitatory and inhibitory sums, the sight, the stimulus's volley and its spikes after it, and the
  dopamine signal at the trial's end.
- [ ] **The assertion.** In both rewarded arms the signal at every trial's end between −0.712 and 0.712, the
  wrong pair's modulation in the trial after a wrong selection at most 0.212 and the answer pair's in the trial
  after a correct one at least 0.788, held by the oracle against the record at every trial. If it fails, a
  numbered finding against ADR-0082's derivation, reported beside the verdict and not in place of it.
- [ ] **The verdict.** The rule committed first, over the pinned tables: **H-15 is yes or no**, and whitepaper
  §11.1's H-15 is checked with the result and its scope — this network, this delivery, this regime, this rule,
  the baseline 0.5, 1 536 trials — and its stopping rule's item with the step reached. After a yes, the sentence
  beside H-12 exactly as ADR-0082 wrote it in advance, and nothing else of H-12. The ADR states the next decision
  H-15's rule makes **and does not take it**.
- [ ] **The readings, in every branch.** The two predicted readings beside what the runs read: the withheld arm's
  couplings and selection per stimulus (whether it drifted toward readout 1 for both stimuli), and in each
  rewarded arm which stimulus reached its answer later or not at all; the selection's crossing of 40 of 64 in
  each arm beside H-14's 448 and 384; the four couplings' course and the wrong pairs' course, now that they move;
  the arena's sums (the settling ADR-0077 stopped, resumed under the task); the sight and whether the stimulus
  still fires once; the trace's composition on the four pairs by the oracle, at least in the first and the last
  block; and the dopamine signal.
- [ ] **The gate.** At most one test, running no whole run: the criterion's rule at its edges over tables written
  by hand, and the modulation's bounds under 0.5 over a few trials with the oracle held to the record. Nothing
  else added to the gate.
- [ ] **The evidence.** A weekly dispatched on this round's branch at the scope ADR-0075 gives **this round's own
  diff**, the clause that applied stated in the ADR; green in every job it runs, every pinned number reproduced;
  its run id and the three exhaustive shards' times, each shard's binaries named, in the ADR.
- [ ] **The documents, in the same pull request.** Whitepaper §11.1's H-15 and its stopping rule (the step
  reached) and, after a yes, the sentence beside H-12; §6.5's loop; §9; the ADR index; `CHANGELOG.md`;
  `README.md`; `CLAUDE.md`'s opening paragraph; `docs/zh-TW`'s reader's guide as the result requires. The
  whitepaper's version moves in **both** declarations with its date
  ([ADR-0064](../docs/adr/0064-the-documentation-gate-and-the-version.md)).
- [ ] **The brief archived** as `briefs/README.md` says, every deliverable dispositioned, the frozen banner naming
  the pull request, the ADRs, **H-15's answer and the step of its stopping rule reached**.

## Not empowered

- **No constant of H-15 moves**: not the baseline of 0.5, the 1 536 trials, the reward of 1.0, the feedback of
  each arm, the delivery, the window, the trial, the seed, the last 128 trials, the mark of 80, or the tie counted
  as not correct. They are ADR-0082's.
- **No other baseline** is run, before or after the rewarded runs.
- **No constant of the engine moves**, and the gain stays 1.75 as the image holds it, the controller off.
- **The network is ADR-0077's settled engine**, built by its rule. **The stimulus is ADR-0076's, pinned.**
- **No change to the task**: not its delivery, its feedback, its selection or its addressing.
- **No clause added on the withheld arm**, and none dropped from the rewarded ones.
- **No rewarded run unless the calibration reproduces.** No constant chosen or moved after a rewarded run; no
  criterion changed after a run; **no second attempt** at H-15 in this configuration.
- **The round does not take the next decision** its answer names, and writes nothing beside H-12 but the sentence
  ADR-0082 wrote for a yes.
- **No runs added to `instrument.rs`**, and no binary whose name sorts after `instrument`. No change to
  `.github/workflows/ci.yml`, `scripts/exhaustive-shard.sh` or the sweep's configuration.
- No record change, no new section, no format bump; no new crate; no dependency in a state crate; no `unsafe`
  outside the runtime's arena access; no float anywhere.
- No more than one test added to the pull request's gate. No move of the determinism pin, and no edit to a number
  a prior round pinned.
- "Learns" only with its scope: this task, these arms, these 1 536 trials, this network at 1 024 units, the
  baseline 0.5.

## Architectural empowerment

The executing session may replace any instruction above with a better decision, recorded in the ADR that owns
it: how the harness is shared between `instrument.rs` and the new binary (a module both include, with the
shared functions and tables moved into it, so long as every pinned number of `instrument.rs` reruns unchanged
and neither binary's `--list` gains the other's tests); the new binary's name, so long as it sorts before
`instrument`; whether the 0.5 image is the image as encoded or patched; whether the calibration and the three
arms are one test or several; how the oracle is extended to consolidate every stimulus–readout synapse under
0.5, so long as it is held to the record at every trial; how often the composition is read, so long as the first
and the last block are; whether to add the shuffled arm, if the new binary's shard stays under 60 per cent of its
bound; whether the round writes one ADR or two; and which numbers the ADRs restate. **A defect found in H-15's
criterion, assertion or stopping rule before the first rewarded run** — a clause that cannot be computed as
written — is written as an ADR amending ADR-0082, with the reason, and committed before that run; never after it.
The round may not reach the standing directives, the whitepaper's invariants, the constraints in `CLAUDE.md`, or
H-15's criterion and stopping rule by any other route.

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

Every command exits 0; the in-diff gate reports no survivor in CI. The `--list` line shows `instrument.rs` with its
twenty-eight `exhaustive` tests and the new binary with only its own. The dispatched run is green in every job it
runs and reproduces every pinned number. The constants commit precedes any commit that holds a rewarded outcome.
The determinism pin, the image format and every number a prior round pinned are unchanged.

## Report

The closing message states: **whether the calibration reproduced**, and if not what differed and that no rewarded
run was made; **H-15's answer, yes or no**, with the correct selections of the last 128 in each rewarded arm
against 80, and **the step of H-15's stopping rule reached** and the next decision it names; whether the assertion
held; the two predicted readings beside the run — the withheld arm's drift and which stimulus decided each
rewarded arm; the crossings beside H-14's; the couplings' course, the wrong pairs' among them, and where the rise
stopped, if it did; the arena's sums; the sight and whether the stimulus still fires once; the composition in the
first and the last block; the dopamine signal; after a yes, the sentence written beside H-12; the new binary's name
and each shard's binaries; the scope ADR-0075 gave this diff and the clause that applied; the shards' times against
their bound; what the mutation gate and any sweep found; what was not done and why; and what the re-examination
after the round recommends next.
