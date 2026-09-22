---
status: archived
date: 2026-09-22
---

> **Executed 2026-09-22 in pull request #99.** Writes ADR-0079 (the reward's direction, measured:
> the calibration reproduced ADR-0077's settled candidate step by step and the withheld arm's first
> block reproduced its frozen run bit for bit before any rewarded run; the taught delivery built in
> the harness from the two calls the executor has; ADR-0072's oracle extended to replay the
> consolidation and held to the record at every trial of every arm; the three arms run once from the
> one image). **H-13's answer: yes** — in both assignments the assigned couplings rose by 12 to 15 per
> cent over 512 trials and were still rising, every synapse outside the pairs the image's bit for
> bit, and the assigned readout's response was above the same trial's with the reward withheld in
> 128 and 127 trials of the last 128, by six spikes per trial; the couplings' prediction held.
> **The step of H-13's stopping rule reached: step 3 — the next decision is an ADR on the reinforced
> form, a delivery the selection decides, and this round did not take it.** No finding. Image
> format 14 unchanged; the determinism pin untouched; no constant of the engine, the instrument or
> H-13 moved. Every deliverable is done; notes under the boxes say what each read. Relative links
> gained one `../` so that they resolve from `archive/`; no other word, claim or figure changed.
> *The body below describes the tree before execution and is not maintained.*

# Brief 036: The reward's direction — H-13, run once under the criterion and the stopping rule ADR-0078 wrote before it

## Mission

**This brief runs H-13, and it runs it once.** H-12 is closed: [ADR-0077](../../docs/adr/0077-the-background-side.md)
applied the third step of its stopping rule, and [ADR-0078](../../docs/adr/0078-the-rewards-direction.md)
chose, among the three routes the rule named, *the task that asks for a sign rather than a difference*,
and wrote it as **H-13** in whitepaper §11.1 — with its configuration, its three arms, its criterion,
its prediction and its own stopping rule — **before this brief existed**. This round builds the run,
checks the calibration, commits the criterion as integer rules, runs the three arms and reports what
they read. It chooses nothing that ADR-0078 fixed.

The question is the one beneath H-12's. Every rewarded run so far was made where the trace on a
stimulus's synapses into a readout was net depression, and ADR-0069's addressed delivery moved the
couplings from a stimulus into its rewarded readout *downward*, as that sign said it would. ADR-0077's
settled network is the first configuration in the tree where that trace is **net potentiation on
average while the readout sees** (+0.4 per synapse per trial, every pair's sum positive after the
frozen block, the sight 62 of 64, the sign 53). H-13 asks whether a reward delivered to those synapses
now moves them — and the response the engine reads through them — **up**.

When the round is done, the tree holds: the calibration reproduced (or the round stopped there); the
taught delivery; the three arms from one settled image as weekly `exhaustive` tests with their tables
pinned; the criterion's verdict; H-13 checked in §11.1 as a yes or a no with its scope; and **the step
of H-13's stopping rule the round reached** — a yes makes the reinforced form the next decision, a no
makes the operating regime the next decision, and this round takes neither.

---

## Standing directives

- **Latest ≠ Newest** (whitepaper §2.1, `CLAUDE.md` principle 4). This round adds **no mechanism**: the
  taught delivery is two calls the executor already has (`Executor::address`, `Executor::reward`) in a
  new order, the settled image is ADR-0077's, the oracle is ADR-0072's. **No** new learning rule,
  gradient, surrogate gradient, e-prop, reward predictor, critic, eligibility variant, structural
  plasticity, per-unit scaling, reward of one sign on H-12's task, or 2025–2026 method; no dependency,
  no new tool, no version bump of a tool.
- Every claim is Implemented, Specified, Target or Hypothesis. ADR-0078's prediction for the couplings
  is a **Hypothesis** and is reported as one ([ADR-0010](../../docs/adr/0010-measured-or-target.md));
  there is no prediction for the response and the round writes none after the fact. Nothing here says
  anything about 256 units or Appendix A's scale. No timing figure from a developer machine.
- The repository wins over the document; a disagreement is a numbered finding in whitepaper §11. Every
  number of ADR-0053 to ADR-0077 stays in them and their tests keep pinning them.
- No `f32`/`f64` anywhere, oracles included; every operation on a state field saturates or wraps by
  name ([ADR-0029](../../docs/adr/0029-structural-enforcement.md)).
- Every loop ends by construction — a run of 512 trials is a `for` over the trials, a lead-in is
  bounded ([ADR-0062](../../docs/adr/0062-the-first-complete-sweeps-list.md)).
- Every quantity has one owner ([ADR-0016](../../docs/adr/0016-thirty-two-crate-architecture.md)): the
  trace and its consolidation are `cortex-core`'s, the dopamine signal `cortex-neuromod`'s, the
  addressing the executor's, the task the runtime's. **No new crate**; no record changes; the image
  format stays 14.
- **Nothing is chosen after a rewarded run.** H-13's constants, arms, criterion and stopping rule are
  ADR-0078's and are not this round's to move; the round's integer form of the criterion is committed
  **before the first rewarded run**, and no clause is dropped because it failed.
- The determinism pin of [ADR-0030](../../docs/adr/0030-verification-governance.md) does not move. The
  mutation gate on the changed lines must pass; every number an arithmetic oracle can produce is
  computed by it before the test that asserts it — and **the engine is read before a description of it
  is trusted**, this brief's included: brief 034 stated two facts about the membrane that `integrate`
  contradicts (ADR-0076).
- A heavy run is an `#[ignore]`d test whose name contains `exhaustive` and runs in the weekly job; the
  pull request's gate grows by at most one test
  ([ADR-0061](../../docs/adr/0061-the-learning-runs-leave-the-gate.md)).
- The engine never amends its own code ([ADR-0031](../../docs/adr/0031-policy-amendment.md)); no registry
  entry (F-37). Conventional Commits with a real body; never commit on `main`; the required checks keep
  their names.

## Context

Re-derived on 2026-09-22 against `main` at `7082922` with ADR-0078 beside it. Line numbers move; the
symbols and the quoted sentences are what to re-derive.

1. **What ADR-0077 left** (the settled network, per synapse of A→R0 over the frozen block of 64 trials;
   `tests/instrument.rs`, `BACKGROUND_1024`, `BACKGROUND_MEASURES_1024`):

   | | Lead-in | Exc. sum | Fires once | Sight | Sign | Volley pot. | Volley dep. | Bg pot. | Bg dep. | Terms/trial | Four sums after the block |
   | :--- | ---: | ---: | :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | :--- |
   | the prior frozen (ADR-0076) | 0 | 1.000 | holds | 62 | 13 | 8.0 | −0.2 | 6.6 | −12.7 | −2.0 | −9.0, −12.8, −3.5, +2.0 |
   | **the settled network** | 96 | 0.925 | holds | 62 | 53 | 8.1 | −0.2 | 6.4 | −10.2 | **+0.4** | **+0.7, +14.5, +8.4, +13.6** |

   The readout units' rate is the prior's (0.29 against 0.28 spikes per trial); the A→R0 coupling is
   6.25 M against the prior's 6.99 M, and "the depression scales with the magnitude (ADR-0055)"; the
   offset the difference criterion would need is 2 spikes per trial. "The net is the small difference
   of the background's two terms and the volley's … and its sign trial to trial is the background's."
2. **H-13, as ADR-0078 wrote it** (whitepaper §11.1, and the ADR's Decision Outcome, which is the
   authority where this summary is shorter). The network: ADR-0077's settled image, the gain 1.75, the
   controller off, the modulation baseline **zero**. The constants: ADR-0076's stimulus, `WINDOW`,
   `TRIAL_TICKS`, `SEED`, `TRIALS` (512), `REWARD_Q16` (1.0). The delivery: after each trial the
   addressed set is the synapses from the presented stimulus's units onto its **assigned** readout's
   units, then a reward of 1.0, **whatever the engine selected**. The arms: the assignment (A→0, B→1),
   the mirrored assignment (A→1, B→0), the reward withheld. The criterion, both clauses in both
   rewarded arms: (1) each assigned pair's excitatory coupling sum after the 512th trial above the
   image's; (2) over the last 128 trials, the assigned readout's count in the task's window above its
   count at the same trial of the withheld arm in **at least 80** trials (`REWARDED_MIN`), a tie
   against. The stopping rule: one round; a calibration that does not reproduce stops it before any
   rewarded run and is a finding; yes → the next decision is an ADR on the reinforced form; no → the
   next decision is the operating regime; no constant moved after a rewarded run and no second attempt.
3. **The delivery as the tree has it** (`runtime/cortex-runtime/src/task.rs`, `executor.rs`,
   `crates/cortex-neuromod`). `Task::trial` injects the stimulus, runs the trial's ticks, counts the
   window, selects, and then writes the addressed set "whatever the feedback" — under
   `Delivery::Addressed`, the presented stimulus onto **the selected readout**. `Feedback::Withheld`
   returns before any reward call; `Feedback::Answer` delivers `+reward_q16` for a correct selection and
   `−reward_q16` otherwise. **No variant delivers to the assigned readout regardless of the selection**:
   the taught delivery is new, either in the harness (after a `Withheld` trial, `exec.address(stimulus
   units, assigned readout units)` then `exec.reward(REWARD_Q16)`) or as a variant in `task.rs`.
   `Executor::address(sources, targets)` addresses exactly the product of the two sets, so one call
   addresses one stimulus–readout pair. `Executor::reward` *adds* to the dopamine signal
   (`NeuromodulatorState::reward`, saturating), and the signal decays by `DOPAMINE_TAU_SHIFT` per tick;
   an addressed synapse consolidates under `clamp(baseline + dopamine, 0, 1)` and every other under
   `clamp(baseline, 0, 1)` (`Modulations::for_synapse`), **at its presynaptic spike** — which ADR-0068
   found is the next presentation of the stimulus, or a background spike of its units. Under the
   baseline at zero, nothing that is not addressed consolidates at all.
4. **The pair rule as it is** (`crates/cortex-core/src/dynamics/synapse.rs`, `SynapseBlock::step_stdp`):
   at a presynaptic spike at $t$, with $p$ the block's previous presynaptic stamp and $q$ the target's
   **last** somatic spike, an excitatory slot gains $A_+(1-2^{-11})^{q-p}$ if $p < q \le t$, and loses
   $A_-(1-2^{-11})^{t-q} \cdot M / 2^{13}$ if $q < t$, $M$ the slot's magnitude before the pairing. So a
   rising coupling depresses more at each pairing, and a readout that fires more moves $q$ later —
   smaller potentiation, larger depression. These are the two mechanisms ADR-0078 names as bounding
   the rise; the round reads them from the composition rather than from this paragraph.
5. **The harness** (`runtime/cortex-runtime/tests/instrument.rs`). ADR-0077's functions build the
   settled image: `candidate`, `settled_within`, `lead_in_until_settled`, `quiet`, `frozen_image`
   (the modulator's baseline patched to zero), `frozen_from`; `background_run` runs the frozen task;
   `run_on` and `compose_on` run and compose from an executor the caller made, the `Composer`'s oracle
   seeded from the record and held to it at every trial. `LAST_BLOCKS`, `REWARDED_MIN` and
   `the_criterion_reads_as_written` (80 of 128 "by noise about once in 337") are the criterion's.
   **The arms are paired by construction**: every arm decodes the same image, whose clock resumes where
   it was written ([ADR-0033](../../docs/adr/0033-tick-duration-in-the-header.md)); the stimulus of a trial
   is `Task::stimulus_at(trial)`, a function of the seed and the index; and the drive is "a function of
   the tick alone, so it needs no state and two forks under it are driven alike" (`Drive`,
   `synthesis.rs`). What differs between two arms at a trial is what the delivery consolidated and what
   that set in motion.
6. **The budget** ([ADR-0073](../../docs/adr/0073-the-whole-domain-tests-sharded.md),
   [ADR-0075](../../docs/adr/0075-the-dispatch-scope-follows-the-diff.md)). In ADR-0077's dispatch
   `instrument.rs` took **2 928 s of its 7 200** (41 per cent; about 68 on a runner as slow as
   ADR-0072's), and its two new tests about six minutes of it. This round runs one lead-in of 96
   windows, a frozen calibration block and three runs of 512 trials (sixty-four windows of ticks each):
   about three times the settled candidate's cost. `scripts/exhaustive-shard.sh` lists the test binaries
   with `cargo test … --list` and assigns them round-robin, so a new test binary is sharded with no
   change to the workflow — and moves which shard the others land in. **This brief names no dispatch
   scope**: the round takes it from its own diff under ADR-0075 and says which clause applied.

## Deliverables

- [x] **The calibration, before anything else (the next free ADR number; `ls docs/adr`)**
  **Done** (ADR-0079): the lead-in's length 96 by `settled_within`, the lead-in's table, the quiet run and the image's sums, gain and step held to ADR-0077's; the withheld arm's first block held to ADR-0077's frozen run bit for bit — the stimulus fires once, the sight 62, the sign 53 of 64 — before any rewarded run. No mismatch, so no finding and no stop.
  (`depends-on: ADR-0078`; ADR-0077 and H-13's stopping rule named). The settled image built as
  ADR-0077 builds it, and before any rewarded run: the lead-in's length by `settled_within` (96), and
  over the frozen block the three measures — the stimulus fires once, the sight 62, the sign 53 of 64 —
  held to ADR-0077's pinned tables. **A mismatch stops the round**: no rewarded run, a numbered finding
  in §11, and the ADR says what differed. That is H-13's stopping rule's second step, and it is a
  defect, not an outcome.
- [x] **The taught delivery.** After every trial, whatever the selection, the addressed set is the
  **Done** (ADR-0079; `taught_run` in `tests/instrument.rs`): in the harness, `Executor::address` to the presented stimulus's units onto its assigned readout's units between the trial's last tick and the reward, then `Executor::reward` with `REWARD_Q16`, whatever the selection; the withheld arm delivers nothing. Held by `reach`: no synapse outside the arm's two assigned pairs moved in any arm (1 584 of 1 584 and 1 603 of 1 604 inside moved), and every synapse of the withheld arm is the image's. The dopamine signal read from the record at every trial's end and held to the oracle: 0.712 at the end and 1.712 after the reward from the eleventh trial — not ADR-0078's about 0.58 and 1.58, because the rule's per-tick step is the floor of $2^{-14}$ of the signal; recorded in ADR-0079, no finding, as ADR-0078 labelled its figures arithmetic and deferred to the record.
  synapses from the presented stimulus's units onto its assigned readout's units and a reward of
  `REWARD_Q16` is delivered; in the withheld arm neither. Held by an assertion in every rewarded arm:
  **every synapse outside the arm's two assigned pairs ends the run as the image holds it, bit for
  bit**, and in the withheld arm every synapse does. The dopamine signal read from the record at each
  trial's end and reported (ADR-0078's arithmetic says it approaches about 0.58 there; the round reads
  it).
- [x] **The constants commit**, preceding the first commit that holds a rewarded outcome: the
  **Done**: `6caa3a6` (the harness, the oracle, the rules `couplings_rose`, `paired`, `taught_blocks`, `last_paired`, `direction`, the arms `ARMS`, `COUPLINGS_PREDICTED`, every constant restated, the gate) precedes `a982efc`, the first commit that holds a rewarded outcome.
  criterion as integer rules over pinned-table shapes (the couplings clause and the paired count over
  `LAST_BLOCKS` against `REWARDED_MIN`, a tie against, both clauses in both rewarded arms), the arms,
  ADR-0078's prediction for the couplings written as a constant beside them, and every constant of
  ADR-0065, ADR-0076, ADR-0077 and ADR-0078 restated unchanged.
- [x] **The three arms** — the assignment, the mirrored assignment, the reward withheld — each 512
  **Done, as one weekly `exhaustive` test** (`the_rewards_direction_at_1024_units_exhaustive`, under the empowerment): the three arms from the one image, so that they are paired by construction and the lead-in is built once; per block the four couplings, the readouts' spikes per set, the paired tallies for both readouts, the selections of the assigned readout, the ties, the consolidation and the signal at the trial's end and after the reward (`DIRECTION_BLOCKS_1024`, `DIRECTION_TAUGHT_1024`, `DIRECTION_COMPOSITIONS_1024`), the per-trial readings by their hash (`DIRECTION_READ_1024`), the sequence traces and the census.
  trials from the one image, as weekly `exhaustive` tests, their tables pinned per block: each
  stimulus–readout pair's excitatory coupling sum, the counts per set per trial, the paired tallies
  against the withheld arm for both readouts, the selection, and the dopamine signal at the trial's end.
- [x] **The verdict.** Both clauses in both rewarded arms, computed by the rules committed first:
  **Done**: both clauses hold in both rewarded arms — the couplings 1.118 and 1.138 (the assignment), 1.153 and 1.139 (the mirrored) of the image's; the paired tally 128 and 127 of the last 128 against 80. **H-13 is yes**; whitepaper §11.1's H-13 is checked with the result and its scope, its stopping rule's item at step 3; ADR-0079 states the next decision (an ADR on the reinforced form) and does not take it.
  **H-13 is yes or no**, and whitepaper §11.1's H-13 is checked with the result and its scope — this
  network, this taught delivery, this regime, this rule — and its stopping rule's item with the step
  reached. The ADR states the next decision H-13's rule makes (the reinforced form after a yes, the
  operating regime after a no) **and does not take it**.
- [x] **The readings, in every branch.** The other readout's paired tally by the same test; the
  **Done** (ADR-0079): the other readout's paired tally 59 and 45 (62 and 71 ties); the selection to the assigned readout 106 and 115 of the last 128 beside 80, stated as reopening nothing of H-12; the assigned pairs' couplings block by block; the composition on the assigned pairs in every block — the potentiation holds and grows as the couplings rise (A→R0's terms per synapse per trial +0.7 to +4.8), the magnitude-scaled depression is read in the depression's growth and outrun by the potentiation's, and the last-spike rule did not shrink the potentiation within 512 trials, so neither mechanism stopped the rise; the couplings clause held as ADR-0078's Hypothesis said.
  selection toward the assigned readout over the last 128 trials beside 80, stated as reopening nothing
  of H-12; the assigned pairs' couplings block by block; and the trace's composition on the assigned
  pairs by ADR-0072's oracle — the volley's and the background's terms per synapse per trial — at
  least in the first and the last block, so that whether the potentiation holds as the couplings rise,
  and which of the two mechanisms of Context 4 stopped it if it stopped, is read. Beside the prediction:
  whether the couplings clause held as ADR-0078's Hypothesis said.
- [x] **The gate.** At most one test, running no whole run: the criterion's rules at their edges over
  **Done**: one test, `the_first_eight_trials_of_the_taught_delivery_at_1024_units_and_the_rules_over_their_tables` — the signal's and the consolidation's rules at their edges, the criterion at its edges over tables written by hand (80 and 79, a tie, a clause failing in one arm only), the delivery's reach over eight trials (805 synapses inside the pairs moved, none outside) and the verdict over the pinned tables. Nothing else added.
  tables written by hand (80 and 79, a tie, a clause failing in one arm only), and the taught
  delivery's reach over a few trials — the synapses outside the assigned pair unmoved bit for bit.
  Nothing else added to the gate.
- [x] **The evidence.** A weekly dispatched on this round's branch at the scope ADR-0075 gives **this
  **Done**: the diff is `tests/instrument.rs` and documents, ADR-0075's last clause, so the weekly was dispatched at `scope=exhaustive` on this round's branch at `a982efc` (run 35690084220); its jobs, every pinned number and the shards' times against their bound are in ADR-0079's evidence.
  round's own diff**, the clause that applied stated in the ADR; green in every job it runs, every
  pinned number reproduced; its run id and the three exhaustive shards' times, with the shard that
  holds this round's runs against its bound, in the ADR.
- [x] **The documents, in the same pull request.** Whitepaper §11.1's H-13 and its stopping rule (the
  **Done**: whitepaper 4.31.0 (§11.1's H-13 checked yes with its scope, its stopping rule at step 3 with the next decision named and not taken, the operating regime's item open with one reading; §6.5; §9); the ADR index; `CHANGELOG.md`; `README.md`; `CLAUDE.md`'s opening paragraph; the zh-TW reader's guide.
  step reached) and, after a no, the operating regime's item named as the next decision; §6.5's loop;
  §9; the ADR index; `CHANGELOG.md`; `README.md`; `CLAUDE.md`'s opening paragraph; `docs/zh-TW`'s
  reader's guide as the result requires. The whitepaper's version moves in **both** declarations with
  its date ([ADR-0064](../../docs/adr/0064-the-documentation-gate-and-the-version.md)).
- [x] **The brief archived** as `briefs/README.md` says, every deliverable dispositioned, the frozen
  **Done**: this file.
  banner naming the pull request, the ADRs, **H-13's answer and the step of its stopping rule
  reached**.

## Not empowered

- **No constant of H-13 moves**: not the baseline of zero, the reward of 1.0, the 512 trials, the
  window, the trial, the seed, the last 128 trials, the mark of 80, the tie counted against, the three
  arms, or the assignment. They are ADR-0078's.
- **No constant of the engine moves** — the membrane's, the pair rule's, the trace's, the modulator's,
  the controller's, the inhibitory rule, the sleep constants, `Prior` or the reference prior's
  parameters — and the gain stays 1.75 as the image holds it, the controller off.
- **The network is ADR-0077's settled image**, built by its rule; not re-settled another way, not
  lengthened, not shortened. **The stimulus is ADR-0076's, pinned.**
- **No delivery decided by the selection** — that is the reinforced form, the next decision after a
  yes, not this round's. No reward of one sign on H-12's task, no run of H-12's criterion, no
  consolidation at the trial's end.
- **No rewarded run unless the calibration reproduces.** No constant chosen or moved after a rewarded
  run; no criterion changed after a run; no clause dropped because it failed; **no second attempt** at
  H-13 in this configuration.
- **The round does not take the next decision** its answer names.
- No structural rule, no change of regime; no record change, no new section, no format bump; no new
  crate; no dependency in a state crate; no `unsafe` outside the runtime's arena access; no float
  anywhere.
- No more than one test added to the pull request's gate. No change to `.github/workflows/ci.yml`,
  `scripts/exhaustive-shard.sh` or the sweep's configuration. No move of the determinism pin, and no
  edit to a number a prior round pinned.
- No use of "learns" for a yes: H-13 is a claim about the rule's direction under a taught delivery, not
  about learning a task.

## Architectural empowerment

The executing session may replace any instruction above with a better decision, recorded in the ADR
that owns it: whether the taught delivery lives in the harness (test code) or as a variant of
`Delivery` or `Feedback` in `task.rs` (source, which ADR-0075 then sends through the sweep); whether the
runs live in `instrument.rs` or in a test binary of their own — which it should choose if the shard
holding `instrument.rs` would otherwise pass 60 per cent of its bound on ADR-0077's runner — and how the
harness is shared with it; whether the calibration and the three arms are one test or several; how
often the composition is read if ADR-0072's oracle over 512 trials costs too much, so long as the first
and the last block are read; whether the round writes one ADR or two; and which numbers the ADRs
restate. **A defect found in H-13's criterion or its stopping rule before the first rewarded run** — a
clause that cannot be computed as written, a pairing that does not hold — is written as an ADR amending
ADR-0078, with the reason, and committed before that run; never after it. The round may not reach the
standing directives, the whitepaper's invariants, the constraints in `CLAUDE.md`, or H-13's criterion
and stopping rule by any other route.

## Verification

```bash
cargo check --workspace --all-targets --locked
cargo test --workspace --locked
cargo test --workspace --release --locked
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

Every command exits 0; the in-diff gate reports no survivor in CI. The dispatched run is green in every
job it runs and reproduces every pinned number. The constants commit precedes any commit that holds a
rewarded outcome. The determinism pin, the image format and every number a prior round pinned are
unchanged.

## Report

The closing message states: **whether the calibration reproduced**, and if not what differed and that
no rewarded run was made; **H-13's answer, yes or no**, with both clauses in both rewarded arms — the
four assigned pairs' coupling sums against the image's and the paired tallies against 80 — and **the
step of H-13's stopping rule reached** and the next decision it names; whether the couplings did what
ADR-0078's Hypothesis said; the readings — the other readout's tally, the selection beside 80 (stated as
reopening nothing of H-12), the couplings block by block, the composition in the first and the last
block and which mechanism, if either, stopped the rise, and the dopamine signal at the trials' ends;
the scope ADR-0075 gave this diff and the clause that applied; the shards' times with the shard holding
this round's runs against its bound; what the mutation gate and any sweep found; what was not done and
why; and what the re-examination after the round recommends next.
