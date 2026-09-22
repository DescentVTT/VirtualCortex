---
status: archived
date: 2026-09-22
---

> **Executed 2026-09-22 in pull request #102.** Writes ADR-0081 (the reinforced form,
> measured: the calibration reproduced ADR-0077's settled candidate step by step and a frozen block
> from the image reproduced its frozen run bit for bit before any rewarded run; the task's own
> delivery composed in the harness with no change to the task; ADR-0079's oracle followed from the
> outcome with one field and held to the record at every trial of every arm; the three arms run once
> from the one image). **H-14's answer: yes** — the correct selections over the last 128 trials
> numbered 128 of 128 in the assignment and 128 in the mirrored assignment against 80, the selection
> passing 40 of 64 per block by trial 448 and 384, the mirrored first, as ADR-0080 predicted; the
> answer couplings rose to 1.35 to 1.45 of the image's and were still rising, every synapse outside
> them the image's bit for bit and the wrong pairs never consolidating, ADR-0080's derivation true on
> every arm; the shuffled reward locked stimulus A onto one readout. **The step of H-14's stopping
> rule reached: step 3 — H-14 recorded yes with its scope, the note written beside H-12, and the next
> decision an ADR choosing what the learning line asks next among the baseline at which every
> synapse consolidates, the assignment reversed within a run, the operating regime and another size;
> this round did not take it.** No finding. Image format 14 unchanged; the determinism pin
> untouched; no constant of the engine, the instrument or H-14 moved. Every deliverable is done;
> notes under the boxes say what each read. Relative links gained one `../` so that they resolve
> from `archive/`, and one `@spec-ignore` directive was placed above the standing directive whose
> word "blocks" spec-graph reads as a relation once the brief is retired; no other word, claim or
> figure changed.
> *The body below describes the tree before execution and is not maintained.*

# Brief 037: The reinforced form — H-14, run once under the criterion, the prediction and the stopping rule ADR-0080 wrote before it

## Mission

**This brief runs H-14, and it runs it once.** [ADR-0079](../../docs/adr/0079-the-rewards-direction-measured.md)
read H-13 yes: on the network ADR-0077 settled, a reward *handed* to the synapses from the presented
stimulus onto its assigned readout, whatever the engine selected, raised those couplings and the response
the engine reads. H-13's stopping rule named the next decision — the reinforced form — and
[ADR-0080](../../docs/adr/0080-the-reinforced-form.md) took it and wrote **H-14** in whitepaper §11.1 with its
configuration, its three arms, its length, its criterion, an assertion derived from the rules, its
prediction, its relation to H-12 and its own stopping rule, **before this brief existed**.

The question: whether a reward the engine **earns by its own selection** moves that selection to the
answer. The delivery is the task the engine already has — `Delivery::Addressed` with `Feedback::Answer`:
the reward reaches the synapses from the presented stimulus onto the readout the engine selected, $+1.0$
when that was the answer and $-1.0$ otherwise — on ADR-0077's settled image with the modulation baseline at
zero, over **1 536 trials**, in the assignment and the mirrored assignment, with the shuffled reward beside
them as the reading of lock-in.

When the round is done, the tree holds: the calibration reproduced (or the round stopped there); the three
arms from one settled image as weekly `exhaustive` tests with their tables pinned; the assertion that a
wrong selection's pair never consolidates, held; the criterion's verdict; H-14 checked in §11.1 as a yes or
a no with its scope — and, after a yes, the note beside H-12 that ADR-0080 wrote in advance; and **the step
of H-14's stopping rule the round reached**, with the next decision it names and does not take.

---

## Standing directives

- **Latest ≠ Newest** (whitepaper §2.1, `CLAUDE.md` principle 4). This round adds **no mechanism**: the
  delivery and the feedback are `task.rs`'s as they stand, the settled image is ADR-0077's, the oracle is
  ADR-0072's as ADR-0079 extended it. **No** new learning rule, gradient, surrogate gradient, e-prop,
  reward predictor, critic, eligibility variant, structural plasticity, per-unit scaling, reward of one sign,
  consolidation at the trial's end, or 2025–2026 method; no dependency, no new tool, no version bump of a
  tool.
<!-- @spec-ignore -->
- Every claim is Implemented, Specified, Target or Hypothesis. ADR-0080's prediction — yes, the selection
  passing 40 of 64 per block by about trial 900 — is a **Hypothesis** from arithmetic on ADR-0079's blocks
  and is reported as one ([ADR-0010](../../docs/adr/0010-measured-or-target.md)); its trial figures are
  estimates and are not held to the run. Nothing here says anything about 256 units or Appendix A's scale.
  No timing figure from a developer machine.
- The repository wins over the document; a disagreement is a numbered finding in whitepaper §11. Every
  number of ADR-0053 to ADR-0079 stays in them and their tests keep pinning them.
- No `f32`/`f64` anywhere, oracles included; every operation on a state field saturates or wraps by name
  ([ADR-0029](../../docs/adr/0029-structural-enforcement.md)).
- Every loop ends by construction — a run of 1 536 trials is a `for` over the trials, a lead-in is bounded
  ([ADR-0062](../../docs/adr/0062-the-first-complete-sweeps-list.md)).
- Every quantity has one owner ([ADR-0016](../../docs/adr/0016-thirty-two-crate-architecture.md)). **No new
  crate**; no record changes; the image format stays 14.
- **Nothing is chosen after a rewarded run.** H-14's constants, arms, length, criterion, assertion and
  stopping rule are ADR-0080's and are not this round's to move; the round's integer form of the criterion
  is committed **before the first rewarded run**, and no clause is dropped or added because of what a run
  read.
- The determinism pin of [ADR-0030](../../docs/adr/0030-verification-governance.md) does not move. The mutation
  gate on the changed lines must pass; every number an arithmetic oracle can produce is computed by it
  before the test that asserts it — and **the engine is read before a description of it is trusted**, this
  brief's and ADR-0080's included: ADR-0078's dopamine figures were an exponent where `decay_dopamine` takes
  a floor (ADR-0079), and brief 034 stated two facts about the membrane that `integrate` contradicts
  (ADR-0076).
- A heavy run is an `#[ignore]`d test whose name contains `exhaustive` and runs in the weekly job; the pull
  request's gate grows by at most one test
  ([ADR-0061](../../docs/adr/0061-the-learning-runs-leave-the-gate.md)).
- The engine never amends its own code ([ADR-0031](../../docs/adr/0031-policy-amendment.md)); no registry
  entry (F-37). Conventional Commits with a real body; never commit on `main`; the required checks keep
  their names.

## Context

Re-derived on 2026-09-22 against `main` at `8108ec4` with ADR-0080 beside it. Line numbers move; the symbols
and the quoted sentences are what to re-derive.

1. **What H-13 read** ([ADR-0079](../../docs/adr/0079-the-rewards-direction-measured.md), the taught delivery
   after every trial; `DIRECTION_BLOCKS_1024`, `DIRECTION_TAUGHT_1024`). The selection toward the assigned
   readout per block of 64, and the assigned couplings after the block as a fraction of the image's:

   | Block | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 |
   | :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
   | Assignment, selection | 32 | 32 | 27 | 34 | 43 | 49 | 46 | 60 |
   | Assignment, A→R0 | 1.006 | 1.013 | 1.024 | 1.030 | 1.047 | 1.078 | 1.094 | 1.118 |
   | Mirrored, selection | 28 | 33 | 41 | 44 | 45 | 49 | 60 | 55 |
   | Mirrored, A→R1 | 1.013 | 1.031 | 1.047 | 1.059 | 1.085 | 1.112 | 1.132 | 1.153 |
   | Withheld, selection of the assignment's readout | 29 | 29 | 25 | 24 | 29 | 28 | 30 | 29 |

   The rise is positive feedback — the trace's potentiation on A→R0 grew from +0.7 to +4.8 per synapse per
   trial as the readout fired 17 spikes per set against 10 — and was not settling at the eighth block. The
   dopamine signal under a reward every trial reached a fixed point of **1.712 after a reward and 0.712 at a
   trial's end** from the eleventh trial. The image's couplings (`IMAGE_COUPLINGS_1024`): A→R0 6 249 552,
   A→R1 6 698 611, B→R0 6 584 205, B→R1 6 815 470 in Q1.15, over 775, 806, 798 and 809 synapses.
2. **H-14, as ADR-0080 wrote it** (whitepaper §11.1, and the ADR's Decision Outcome, which is the authority
   where this summary is shorter). The network: ADR-0077's settled image, the gain 1.75, the controller off,
   the modulation baseline **zero**. The constants: ADR-0076's stimulus, `WINDOW`, `TRIAL_TICKS`, `SEED`,
   `REWARD_Q16` (1.0), `BLOCK`, `LAST_BLOCKS`, `REWARDED_MIN` (80), and **1 536 trials, twenty-four
   blocks**. The arms: the assignment and the mirrored assignment under `Feedback::Answer`, and the shuffled
   reward (`Feedback::Shuffled`) as a reading, all `Delivery::Addressed`, the same trials from the one
   image. The criterion: correct selections over the last 128 trials **at least 80 in both rewarded arms**
   (`Outcome::correct`; a tie is not correct). The assertion: in a rewarded arm every synapse outside the two
   answer pairs, the wrong pairs among them, ends the run as the image holds it, bit for bit. The relation to
   H-12 and the stopping rule: yes → the note beside H-12 and an ADR choosing what the line asks next among
   named candidates; no → an ADR choosing between a consolidation at the trial's end and the operating
   regime; no constant moved after a rewarded run, no second attempt.
3. **The task as built** (`runtime/cortex-runtime/src/task.rs`). `Task::trial` injects the stimulus, runs the
   trial's ticks, counts the window, selects through `cortex-basal-ganglia`'s gate, then writes the addressed
   set "whatever the feedback" — under `Delivery::Addressed` the presented stimulus's units onto the
   **selected** readout's units, onto none at a tie — and delivers the reward: under `Feedback::Answer`
   `+reward_q16` when `selection == Some(answer)` and `−reward_q16` otherwise, a tie included; under
   `Feedback::Shuffled` the sign is `coin_at(trial)`, a bit of the seed's draw the stimulus bit does not read.
   `mirrored` swaps each stimulus's answer. **No change to the task is needed for H-14.** The harness's
   `run_on` asserts that every unit is a source and a target before the first trial.
4. **The signal and the derivation ADR-0080 asks the round to hold** (`crates/cortex-neuromod`,
   `runtime/cortex-runtime/src/executor.rs`). An addressed synapse consolidates under
   `clamp(baseline + signal, 0, 1)` and every other under `clamp(baseline, 0, 1)` (`Modulations::for_synapse`);
   `NeuromodulatorState::reward` adds to the signal, saturating; `decay_dopamine` moves it toward zero by
   `max(|signal| >> DOPAMINE_TAU_SHIFT, 1)` and never by more than its magnitude, so it never crosses zero.
   The signal at a trial's end is therefore at most the fixed point of a reward every trial, 0.712; a
   punishment of 1.0 leaves it negative; with the baseline at zero the wrong selection's pair, addressed for
   the next trial, consolidates nothing. **Read these functions before trusting this paragraph.**
5. **The harness** (`runtime/cortex-runtime/tests/instrument.rs`, the blocks "the background side" and
   "brief 036"). `settled_image` builds ADR-0077's image and holds it to its tables; `calibration_holds` holds
   a frozen first block to ADR-0077's frozen run; `frozen_from` decodes the image; `run_on` runs a `Task`
   from an executor the caller made and returns the blocks (`last_correct` sums a run's last `LAST_BLOCKS`
   blocks' correct trials); ADR-0079's oracle (`Composer::taught`, `signal_course`, `decayed_signal`,
   `consolidated`) replays the consolidation under the signal where the synapse is addressed and was held to
   the record's weights, traces and signal at every trial — under this delivery the addressed pair is the
   presented stimulus onto the **selected** readout and the signal takes negative rewards, which the oracle
   must follow; `weights_of` and `reach` compare every weight of the arena with the image's.
6. **The budget** ([ADR-0073](../../docs/adr/0073-the-whole-domain-tests-sharded.md),
   [ADR-0075](../../docs/adr/0075-the-dispatch-scope-follows-the-diff.md)). In ADR-0079's dispatch `instrument.rs`
   took **3 030 s of its 7 200** (42 per cent), its tests running side by side, H-13's one test 545 s of wall
   clock alone on a developer machine. This round runs one lead-in of 96 windows, a frozen calibration block
   and three runs of 1 536 trials: about two and a half times H-13's test.
   `scripts/exhaustive-shard.sh` lists the test binaries with `cargo test … --list` and assigns them
   round-robin, so a new test binary is sharded with no change to the workflow — and moves which shard the
   others land in. **This brief names no dispatch scope**: the round takes it from its own diff under
   ADR-0075 and says which clause applied.

## Deliverables

- [x] **The calibration, before anything else (the next free ADR number; `ls docs/adr`)**
  **Done** (ADR-0081): the lead-in's length 96 by `settled_within`, the lead-in's table, the quiet run and the image's sums, gain and step held to ADR-0077's; a frozen block from a decode of the image held to ADR-0077's frozen run bit for bit — the stimulus fires once, the sight 62, the sign 53 of 64 — before any rewarded run. No mismatch, so no finding and no stop.
  (`depends-on: ADR-0080`; ADR-0077, ADR-0079 and H-14's stopping rule named). The settled image built and
  held to ADR-0077's tables, and before any rewarded run a frozen first block from it held to ADR-0077's
  frozen run — the stimulus firing once, the sight 62, the sign 53. **A mismatch stops the round**: no
  rewarded run, a numbered finding in §11, and the ADR says what differed.
- [x] **The constants commit**, preceding the first commit that holds a rewarded outcome: the criterion as an
  **Done**: `857fe23` on `main` (`bdb68c9` on the branch before the rebase that merged pull request #102; the harness `earned_run`, the oracle's one field `Composer::rewarded`, the criterion `reinforced` over `last_correct`, the 1 536 trials, the arms `Earned` and their feedback, ADR-0080's prediction as constants, the assertion's shape `reachable_pairs` and `derivation`, the lock-in reading's shape `splits`, `last_splits` and `locked_in`, every constant restated, the gate) precedes `96b57ca` (`2a7c083`), the first commit that holds a rewarded outcome.
  integer rule over the pinned-table shape (`last_correct` of each rewarded arm at least `REWARDED_MIN`, both
  arms), the 1 536 trials, the arms and their feedback, ADR-0080's prediction written as a constant, the
  assertion's shape, the lock-in reading's shape, and every constant of ADR-0065, ADR-0076, ADR-0077 and
  ADR-0080 restated unchanged.
- [x] **The three arms** — the assignment, the mirrored assignment, the shuffled reward — each 1 536 trials
  **Done, as one weekly `exhaustive` test** (`the_reinforced_form_at_1024_units_exhaustive`, under the empowerment): the three arms from the one image, each 1 536 trials under `Delivery::Addressed` and its feedback; per block the correct selections, the selections per stimulus and readout with the ties, the four couplings, the sight and the signal at the trial's end and after the reward (`REINFORCED_BLOCKS_1024`, `REINFORCED_EARNED_1024`, `REINFORCED_COMPOSITIONS_1024`), the per-trial readings by their hash (`REINFORCED_READ_1024`), the sequence traces and the census.
  from the one image under `Delivery::Addressed`, as weekly `exhaustive` tests, their tables pinned per
  block: the correct selections and the selections per stimulus and readout, the four stimulus–readout
  couplings, the sight, and the dopamine signal at the trial's end.
- [x] **The assertion.** In both rewarded arms every synapse outside the two answer pairs — the two wrong
  **Done** (ADR-0081; `reach`, `derivation`, `REINFORCED_REACH_1024`, `DERIVATION_1024`): in both rewarded arms every synapse outside the two answer pairs — the two wrong pairs among them — ended the run as the image holds it, bit for bit (1 584 of 1 584 and 1 604 of 1 604 inside moved, none outside), and in the shuffled arm every synapse outside the four pairs (3 183 of 3 188 inside, none outside); the signal at every trial's end at most 0.712, below zero after every negative reward, nothing consolidated in the trial after one; the oracle agreed with the record at every trial of every arm. It held: no finding.
  pairs among them — ends the run as the image holds it, bit for bit, and the oracle agrees with the record
  at every trial; in the shuffled arm every synapse outside the four stimulus–readout pairs. If it fails, a
  numbered finding against ADR-0080's derivation, reported beside the verdict and not in place of it.
- [x] **The verdict.** The rule committed first, over the pinned tables: **H-14 is yes or no**, and
  **Done**: 128 of the last 128 correct in the assignment and 128 in the mirrored, against 80. **H-14 is yes**; whitepaper §11.1's H-14 is checked with the result and its scope, its stopping rule's item at step 3; the note beside H-12's closing sentence written as ADR-0080 wrote it; ADR-0081 states the next decision (an ADR choosing what the learning line asks next) and does not take it.
  whitepaper §11.1's H-14 is checked with the result and its scope — this network, this delivery, this
  regime, this rule, 1 536 trials — and its stopping rule's item with the step reached. After a yes, the note
  beside H-12's closing sentence exactly as ADR-0080 wrote it in advance, and nothing else of H-12. The ADR
  states the next decision H-14's rule makes **and does not take it**.
- [x] **The readings, in every branch.** The selection per block and per stimulus in each arm, beside ADR-0080's
  **Done** (ADR-0081): the selection per block and per stimulus in every arm, crossing 40 of 64 at trial 448 and 384, the mirrored first, against ADR-0080's about 900; the four couplings per block, the answer pairs' rising in every block and not stopped within 1 536 trials; the sight 62 to 64 in every block; the shuffled arm's splits — A to readout 1 in 60 of 61, B to readout 1 in 50 of 67 — one tie short of the lock-in rule's mark for A and read as lock-in beside the rule; the dopamine signal at the fixed point over the last 128 trials of both rewarded arms; the composition on the answer pairs in every block, the potentiation growing with the couplings.
  prediction (whether the crossing of 40 of 64 came by about trial 900, the mirrored first); the four
  couplings per block, and where the answer pairs' rise stops if it stops; the sight per block (whether the
  readout still sees as the couplings rise); in the shuffled arm each stimulus's split between its readouts
  over the last 128 trials and per block, read as lock-in when a stimulus goes to one readout as often as a
  rewarded arm goes to its answer; the dopamine signal at the trials' ends; and the trace's composition on the
  answer pairs by ADR-0079's oracle, at least in the first and the last block.
- [x] **The gate.** At most one test, running no whole run: the criterion's rule at its edges over tables
  **Done**: one test, `the_first_eight_trials_of_the_reinforced_form_at_1024_units_and_the_rules_over_their_tables` — the criterion's rule at its edges over tables written by hand (80 and 79, one arm failing, an arm of no block), the readings' rules over trials written by hand, the delivery over eight trials on the instrument's network at 1 024 units with the baseline at zero (797 synapses inside the answer pairs moved, none outside, nothing consolidated on the wrong pairs or in the trial after a punishment, the oracle held at every trial), and the verdict over the pinned tables. The instrument's network and not the settled one, under the empowerment: the settled network's lead-in is the weekly test's cost and the derivation's rules read the same on any network with the baseline at zero. Nothing else added.
  written by hand (80 and 79, a tie, one arm failing), and the delivery's reach over a few trials on the
  settled network — a wrong selection's pair unmoved bit for bit, the oracle held at every trial. Nothing else
  added to the gate.
- [x] **The evidence.** A weekly dispatched on this round's branch at the scope ADR-0075 gives **this round's
  **Done**: the diff is `tests/instrument.rs` and documents, ADR-0075's last clause, so the weekly was dispatched at `scope=exhaustive` on this round's branch at `2a7c083` (`96b57ca` on `main`); its jobs, every pinned number and the shards' times against their bound are in ADR-0081's evidence.
  own diff**, the clause that applied stated in the ADR; green in every job it runs, every pinned number
  reproduced; its run id and the three exhaustive shards' times, with the shard that holds this round's runs
  against its bound, in the ADR.
- [x] **The documents, in the same pull request.** Whitepaper §11.1's H-14 and its stopping rule (the step
  **Done**: whitepaper 4.33.0 (§11.1's H-14 checked yes with its scope, its stopping rule at step 3 with the next decision named and not taken, the note beside H-12, the operating regime's item open with one more reading; §6.5; §9); the ADR index; `CHANGELOG.md`; `README.md`; `CLAUDE.md`'s opening paragraph; the zh-TW reader's guide.
  reached) and, after a yes, the note beside H-12; §6.5's loop; §9; the ADR index; `CHANGELOG.md`;
  `README.md`; `CLAUDE.md`'s opening paragraph; `docs/zh-TW`'s reader's guide as the result requires. The
  whitepaper's version moves in **both** declarations with its date
  ([ADR-0064](../../docs/adr/0064-the-documentation-gate-and-the-version.md)).
- [x] **The brief archived** as `briefs/README.md` says, every deliverable dispositioned, the frozen banner
  **Done**: this file, with the banner above.
  naming the pull request, the ADRs, **H-14's answer and the step of its stopping rule reached**.

## Not empowered

- **No constant of H-14 moves**: not the 1 536 trials, the baseline of zero, the reward of 1.0, the feedback
  of each arm, the delivery, the window, the trial, the seed, the last 128 trials, the mark of 80, or the tie
  counted as not correct. They are ADR-0080's.
- **No constant of the engine moves**, and the gain stays 1.75 as the image holds it, the controller off.
- **The network is ADR-0077's settled image**, built by its rule. **The stimulus is ADR-0076's, pinned.**
- **No change to the task**: not its delivery, its feedback, its selection or its addressing. No reward of
  one sign, no consolidation at the trial's end, no baseline of 0.5, no taught delivery.
- **No clause added on the shuffled arm**, and none dropped from the rewarded ones.
- **No rewarded run unless the calibration reproduces.** No constant chosen or moved after a rewarded run; no
  criterion changed after a run; **no second attempt** at H-14 in this configuration, and a selection still
  rising in the last blocks is a reading, not a licence for a longer run.
- **The round does not take the next decision** its answer names, and writes nothing beside H-12 but the note
  ADR-0080 wrote for a yes.
- No record change, no new section, no format bump; no new crate; no dependency in a state crate; no `unsafe`
  outside the runtime's arena access; no float anywhere.
- No more than one test added to the pull request's gate. No change to `.github/workflows/ci.yml`,
  `scripts/exhaustive-shard.sh` or the sweep's configuration. No move of the determinism pin, and no edit to a
  number a prior round pinned.
- "Learns" only with its scope: this task, these arms, these 1 536 trials, this network at 1 024 units, the
  plasticity confined to the addressed synapses.

## Architectural empowerment

The executing session may replace any instruction above with a better decision, recorded in the ADR that
owns it: whether the runs live in `instrument.rs` or in a test binary of their own — which it should choose if
the shard holding `instrument.rs` would otherwise pass 60 per cent of its bound on ADR-0079's runner — and
how the harness is shared with it; whether the calibration and the three arms are one test or several; how
ADR-0079's oracle is extended to the selection's addressing and to negative rewards, so long as it is held to
the record at every trial; how often the composition is read, so long as the first and the last block are;
whether to add a withheld arm, if its cost is small beside the three; whether the round writes one ADR or
two; and which numbers the ADRs restate. **A defect found in H-14's criterion, assertion or stopping rule
before the first rewarded run** — a clause that cannot be computed as written — is written as an ADR amending
ADR-0080, with the reason, and committed before that run; never after it. The round may not reach the
standing directives, the whitepaper's invariants, the constraints in `CLAUDE.md`, or H-14's criterion and
stopping rule by any other route.

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

Every command exits 0; the in-diff gate reports no survivor in CI. The dispatched run is green in every job
it runs and reproduces every pinned number. The constants commit precedes any commit that holds a rewarded
outcome. The determinism pin, the image format and every number a prior round pinned are unchanged.

## Report

The closing message states: **whether the calibration reproduced**, and if not what differed and that no
rewarded run was made; **H-14's answer, yes or no**, with the correct selections of the last 128 in each
rewarded arm against 80, and **the step of H-14's stopping rule reached** and the next decision it names;
whether the assertion held (the wrong pairs bit for bit the image's); the prediction beside the run — where
each arm's selection crossed 40 of 64, against ADR-0080's "by about trial 900, the mirrored first"; the
shuffled arm's splits and whether it locked in; the couplings' course and where the rise stopped, if it did;
the sight's course; the composition in the first and the last block; the dopamine signal; after a yes, the
note written beside H-12; the scope ADR-0075 gave this diff and the clause that applied; the shards' times
with the shard holding this round's runs against its bound; what the mutation gate and any sweep found; what
was not done and why; and what the re-examination after the round recommends next.
