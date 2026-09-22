---
status: archived
date: 2026-09-22
---

> **Executed 2026-09-22 in pull request #96.** Writes ADR-0077 (the background side, step 2 of
> H-12's stopping rule: the network settled before the task under the gain held — ADR-0055's
> criterion as an integer rule, held at the ninety-sixth window with the excitatory sum at
> 0.925 of the prior's and the readout units' rate the prior's — and the criticality controller
> on at its step of an eighth — settled at the thirty-fifth at 0.453, the gain swinging between
> 2.2 and 2.8 and the readouts seven to twenty-four times louder, as predicted — each alone, each
> run quiet, saved as an image with the baseline at zero and read over a frozen run by three
> measures written first; the settled network fires once, is seen at 62 of 64 and its trace is
> net potentiation on average for the first time at a gain at which the readout sees, +0.4 per
> synapse per trial, but the sign reads 53 against 56; the controller fails all three; no
> candidate passes and there is no rewarded run). **The step of the stopping rule reached: step 2,
> and step 3 applied — H-12 closed as a no for this task in this regime.** Each measure's outcome:
> the stimulus still fires once (the settled network holds, the controller fails on the volley),
> the sight (62 and 51), the sign (53 and 47). No finding. Image format 14 unchanged; the
> determinism pin untouched; no constant of the engine or the instrument moved; the gain 1.75 on
> the settled candidate and the controller's step ADR-0055's. Every deliverable is done or
> dispositioned; notes under the boxes say what each read. Relative links gained one `../` so
> that they resolve from `archive/`; no other word, claim or figure changed. *The body below
> describes the tree before execution and is not maintained.*

# Brief 035 — The background side: step 2 of H-12's stopping rule — the network settled before the task and the criticality controller on, each tried alone and picked by the sight and the sign; the last round on this line before the rule closes it

> A self-contained prompt for one round of work. Read `briefs/README.md`, then this file, then
> the files it names. Frozen when executed; the outcome lives in the ADRs that accept it, the
> changelog and the code.

## Mission

**This is step 2 of H-12's stopping rule** (whitepaper §11.1, written before brief 034's outcome).
[ADR-0076](../../docs/adr/0076-two-injections.md) reached step 1 and recorded the stimulus side as
exhausted: two injections fire each unit of the presented set once and not again in all 64
trials, the terms rose from −15.7 to **−2.0 per synapse per trial**, the offset the criterion needs
fell to **one spike**, the bias now favours the answer — and the sign read **13 of 64** against the
56 asked for. ADR-0076 said what remains and whose it is:

> *The remaining depression is the volley spike's own*: at one presynaptic spike per presentation,
> the rule pairs every readout unit's last background spike as depression at the volley … The ratio
> of the two is **the background rate's, not the stimulus's, and no stimulus that fires once can
> move it** … which is the background side, the rule's step 2.

The rule names two routes on that side — "the criticality controller on, or a network settled
before the task" — and allows **at most one round** for them. So this round tries **both, each
alone**, in a fixed order, and lets the blind calibrations pick: the variable is the background,
and these are its two named settings. The stimulus is ADR-0076's, pinned and unchanged; the rule,
the geometry, the readout window, the trial and the seeds are unchanged.

The expectations are written first, and one of them is a prediction of failure. The controller has
been measured at 1 024 units ([ADR-0053](../../docs/adr/0053-the-waking-day-and-the-target-period.md),
[ADR-0055](../../docs/adr/0055-a-weight-that-settles.md)): it raises the gain to 2.1–2.75 and the rate
to 9–36 Hz per unit, oscillating window to window — an order of magnitude louder than the
instrument's readouts, which is the opposite of what ADR-0076 says is needed. It is tried because
the rule names it and because closing the line without trying it would leave the objection that it
was not tried; it is expected to fail, and the brief says so before it runs.

If a candidate passes both calibrations, the stopping rule yields (its step 4) and ADR-0069's
criterion runs. **If neither passes, the rule's step 3 applies and this round writes it**: H-12
closes as a no for this task in this regime, recorded as a result, and the ADR names — without
building — the three routes the next decision chooses among. That outcome is not a failure of the
round; it is the decision the rule was written to reach.

---

## Standing directives

- **Latest ≠ Newest** (whitepaper §2.1, `CLAUDE.md` principle 4). This round adds **no mechanism**:
  the controller is [ADR-0036](../../docs/adr/0036-criticality-control.md)'s, run as built; a settled
  network is the harness running the engine under its drive before the task. **No** new learning
  rule, gradient, surrogate gradient, e-prop, reward predictor, critic, structural plasticity,
  per-unit synaptic scaling (gated on H-8), targeted inhibition of the readouts, rate balance of
  $A_-$ against $A_+$, or 2025–2026 method; no dependency, no new tool, no version bump of a tool.
- Every claim is Implemented, Specified, Target or Hypothesis. The prediction that the controller
  fails is a **Hypothesis** from ADR-0053's and ADR-0055's measurements on another configuration,
  named as one ([ADR-0010](../../docs/adr/0010-measured-or-target.md)). Nothing here says anything
  about 256 units (ADR-0070) or Appendix A's scale. No timing figure from a developer machine.
- The repository wins over the document; a disagreement is a numbered finding in whitepaper §11.
  Every number of ADR-0053 to ADR-0076 stays in them and their tests keep pinning them.
- No `f32`/`f64` anywhere; every operation on a state field saturates or wraps by name
  ([ADR-0029](../../docs/adr/0029-structural-enforcement.md)).
- Every loop ends by construction; **a settling lead-in runs a bounded number of windows and its
  clause is read afterwards**, never "until it settles"
  ([ADR-0062](../../docs/adr/0062-the-first-complete-sweeps-list.md)).
- Every quantity has one owner ([ADR-0016](../../docs/adr/0016-thirty-two-crate-architecture.md)): the
  gain and its regulation are `cortex-homeostasis`'s, the trace `cortex-core`'s, the task and its
  stimulus the runtime's. **No new crate**; no record changes; the image format stays 14.
- **Nothing is chosen after a rewarded run.** The configuration is picked by the measures of
  Deliverable C from the two candidates in their order, and those measures read no selection,
  reward or outcome; it is committed before the first rewarded run.
- The determinism pin of [ADR-0030](../../docs/adr/0030-verification-governance.md) does not move. The
  mutation gate on the changed lines must pass; every number an arithmetic oracle can produce is
  computed by it before the test that asserts it — and **the engine is read before an oracle is
  trusted about it**: brief 034 stated two facts about the membrane that `integrate` contradicts
  (ADR-0076), and a brief's description of the engine is not the engine.
- A heavy run is an `#[ignore]`d test whose name contains `exhaustive` and runs in the weekly job;
  the pull request's gate grows by at most one test
  ([ADR-0061](../../docs/adr/0061-the-learning-runs-leave-the-gate.md)).
- The engine never amends its own code ([ADR-0031](../../docs/adr/0031-policy-amendment.md)); no
  registry entry (F-37). Conventional Commits with a real body; never commit on `main`; the required
  checks keep their names.

## Context

Re-derived on 2026-09-22 against `main` at `3797fad`, after brief 034's round merged. Line numbers move; the
symbols and the quoted sentences are what to re-derive.

1. **What step 1 left** ([ADR-0076](../../docs/adr/0076-two-injections.md), per synapse of A→R0):

   | | After/unit | Response | Volley pot. | Volley dep. | Bg pot. | Bg dep. | Terms/trial | Sign | Sight |
   | :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
   | F-46's (ADR-0072) | 2.06 | 19.2 | 17.2 | −10.4 | 7.5 | −26.8 | −15.7 | 1 | 62 |
   | two injections (ADR-0076) | 0.00 | 8.9 | 8.0 | −0.2 | 6.6 | −12.7 | **−2.0** | **13** | 62 |

   The depression left is "every readout unit's last background spike" paired at the volley, over
   775 synapses whose targets fire 0.28 times per trial; the potentiation is the response to one
   volley. "The ratio of the two is the background rate's." Deliverable D under the pick: the offset
   the criterion needs **1**; the bias +6/34 and +3/30, toward the answer.
2. **The rule this round applies.** Whitepaper §11.1, H-12's stopping rule: step 2, "at most one
   further round, on the background side: the criticality controller on, or a network settled before
   the task … one variable, with the sight and the sign as its calibrations and every other constant
   held"; step 3, "if that round cannot make both pass either, H-12 closes as a no for this task in
   this regime, recorded as a result and not as a pause … The round after it chooses, by an ADR, among
   a task that asks for a sign rather than a difference, a structural rule that adds a synapse, and a
   review of the operating regime against the criticality the network is built to hold"; step 4, "the
   rule yields to a calibration that passes".
3. **The controller, and what it does at 1 024 units.** `cortex-homeostasis::regulate`, once per
   window: $g \leftarrow g\,(1 - \kappa\,\operatorname{clamp}(\hat\sigma - 1, -1, 1))$, clamped to
   $[$`GAIN_MIN_Q16`$, $`GAIN_MAX_Q16`$] = [0.25, 4.0]$, $\kappa$ the executor's `control_step_q0_16`
   (0 in every instrument run). Measured under it: ADR-0053, "the controller holds the population in a
   limit cycle at the ceiling: the gain alternates every window between about 2.3 and 2.9 … and the
   rate with it between 12 and 38 Hz per unit"; ADR-0055's day at 1 024 units, $\kappa = 1/8$, the
   gain 2.09 at the first window and 2.17 to 2.75 thereafter, the rate 9.2 to 36.1 Hz per unit,
   swinging window to window, the excitatory sum settling near 107 M of 225 M. Those were the lattice
   prior from a gain of 2.0, not the instrument's prior at 1.75 — which is why this round measures the
   controller in the instrument's own configuration rather than inferring it. But the direction is
   measured: **the controller raises the gain on this network**, and the injected stimulus is scaled
   by the gain it raises (**F-47**), so the cancel ADR-0076 sized at 1.75 meets a larger drive.
4. **The settled network, and the squeeze it may meet.** [ADR-0070](../../docs/adr/0070-where-256-units-settle.md)
   at 256 units with the gain held: "brief 026's clause first holds at the ninth window, where the sum
   is 0.818 of the prior's, and the sum is still falling at the eightieth, at 0.551 of the prior's by
   0.24 per cent per window, **so the clause reads a rate**"; behind the lead-in "the calibration's
   measure reads 50 of 64 in the first block and 34 to 44 after", because "the drive alone carries the
   sum below the level at which the instrument sees". **Brief 026's clause is therefore not this
   round's settling rule.** ADR-0055's is stricter: at 1 024 units under the controller "every one of
   the last sixteen windows is within 0.75 per cent of the sixty-fourth". A quieter background is what
   ADR-0076 asks for, and a weaker propagation of the stimulus is what ADR-0070 and ADR-0072's ladder
   found beside it; whether settling the weights escapes that squeeze where lowering the gain did not
   is what this round measures, and nothing measured so far says it does.
5. **The stimulus, pinned** (ADR-0076, `tests/instrument.rs`): F-46's first injection (two messages
   of 1.25) and the cancel — six messages at the bound per tick, from `REFRACTORY_TICKS + 1`, over nine
   ticks — probed on a unit at rest at the gain 1.75 and read to fire each unit once in all 64 trials.
   **It is held as pinned under both candidates**: its efficacies, not its drives. Under the controller
   the drive it delivers follows the gain (F-47), which is part of what the controller candidate is.
6. **The instrument's configuration** (`tests/instrument.rs`): the prior of ADR-0044 at seed 22 at
   1 024 units, the gain 1.75 held through the image, `control_step_q0_16: 0`, `sleep_shift: 0`, the
   drive of ADR-0044; the geometry, the readout window and the trial as ADR-0065 built them.
   `reference.rs`'s `day(prior, windows, step, shift, period)` runs windows under a controller step and
   pins the gain, the rate and the sums per window; `windows(exec, drive, n)` and
   `weights_by_polarity(exec)` read the settling. `Image::encode` and `Image::decode` save and load an
   engine whole ([ADR-0024](../../docs/adr/0024-cortex-image-and-clock-sweep.md)), and `instrument.rs`'s
   `reload_with` already encodes, patches the homeostasis section and decodes. **Encoding requires the
   engine to be quiescent** (`Image::encode(exec).expect("quiescent")`): a network under its drive is
   never quiescent, so a settled engine is saved only after it is run quiet until it is —
   `reference.rs`'s `settle` does that — and the saved state is the settled network after it fell quiet,
   not the network mid-drive.
7. **The budget** ([ADR-0073](../../docs/adr/0073-the-whole-domain-tests-sharded.md),
   [ADR-0075](../../docs/adr/0075-the-dispatch-scope-follows-the-diff.md)). ADR-0055's day ran eighty
   windows at 1 024 units under the controller in the weekly job's form; each candidate's lead-in is of
   that size, and a settled engine saved as an image — after it falls quiet (item 6) — can be loaded
   for every run that needs it rather than settled again. `instrument.rs` is a shard of the whole-domain tests by itself — 43 m 21 s of its
   120-minute bound in ADR-0076's dispatch — and the round states its time after this round's runs. **This brief names no dispatch scope**: the round takes it
   from its own diff under ADR-0075 and says which clause applied.

## Deliverables

- [x] **The settled network (the next free ADR number; `ls docs/adr`)** (`depends-on: ADR-0076`;
  ADR-0055, ADR-0070 and H-12's stopping rule named). At 1 024 units in the instrument's configuration
  — the gain **held at 1.75**, the controller off, no sleep — under the drive alone, no task: a lead-in
  of whole windows, read and pinned per window (the two sums of `weights_by_polarity`, the rate, the
  readout sets' own rate), **bounded at eighty** (160 if the sum is still moving at the eightieth, the
  cost stated). The settled point is **ADR-0055's criterion**, not brief 026's: the first window at
  which each of the sixteen windows before it is within 0.75 per cent of the window sixteen before
  that; if no window up to the bound satisfies it, the lead-in is the bound and the ADR says the sum
  was still moving and at what rate. The settled engine is run quiet until quiescent and saved as an image for the runs of Deliverable C,
  and the ADR states what the quiet run changed in the sums, which should be little and is read, not
  assumed.
  **Done** (ADR-0077): the lead-in read and pinned per window (`BACKGROUND_LEAD_IN_1024[0]`, the two sums, the population's rate, the readout sets' own spikes, the fraction at the target, the gain and the estimate); ADR-0055's criterion as the integer rule `settled_within`, at least as strict and written before the run; the rule held at the ninety-sixth window, past the eightieth, so the lead-in ran on under the extended bound (the cost stated: sixteen more windows); the excitatory sum 0.925 of the prior's, the readout units' rate the prior's; the quiet run 2 500 ticks, moving the inhibitory sum by 114 LSB and the excitatory not at all, read and pinned (`QUIET_1024[0]`); the settled engine saved as an image with the baseline patched to zero (`frozen_image`) for the frozen run.
- [x] **The controller (the same ADR).** At 1 024 units in the instrument's configuration with
  **`control_step_q0_16` at ADR-0055's step of an eighth** — the step already measured on this network
  size, not searched — a lead-in by the same rule and bound, the gain's course pinned per window beside
  the rate and the sums. **The prediction, written before it runs:** the gain rises above 1.75 and does
  not settle, the readout sets' rate rises with it, and the candidate fails at least one of the three
  measures below. A **Hypothesis**, from ADR-0053 and ADR-0055 on another prior; the round reports what
  the instrument's configuration reads, whatever it reads.
  **Done** (ADR-0077): ADR-0055's eighth, not searched (`CONTROL_STEP_EIGHTH`); the criterion held at the thirty-fifth window at 0.453 of the prior's; the gain's course pinned per window beside the rate and the sums; the prediction written first as four clauses (`CONTROLLER_PREDICTED`) and every clause read true (`CONTROLLER_READ`): the gain above 1.75 and never settled, the readouts 8.6 times louder, all three measures failed.
- [x] **The three measures, for each candidate in order — the settled network, then the controller.**
  Each over a frozen run of 64 trials of the task (weights frozen by a modulation baseline of zero; the
  controller, where it is the candidate, still regulating) from the candidate's lead-in, none reading a
  selection, a reward or an outcome:
  - **the stimulus still fires once** — ADR-0074's measure, unchanged: one spike per unit within two
    before the readout window opens and no more than a tenth per unit in the pair window after it;
  - **the sight** — ADR-0065's, 56 of 64;
  - **the sign** — ADR-0072's, 56 of 64.

  A candidate passes when all three do; the **first** that passes is the configuration. The order is
  fixed so that no preference chooses, and the candidates are never combined.
  **Done, with the outcome the box names — none passes** (ADR-0077's table; `BACKGROUND_MEASURES_1024`, `BACKGROUND_PICKED_1024`): the settled network fires once (50.8 of 51, 0.00 after), the sight 62, the sign 53 of 64; the controller fires the volley short (48.4 of 51), the sight 51, the sign 47; the order fixed and the candidates never combined; the pick none.
- [x] **What the criterion requires, under each candidate.** ADR-0074's three integer readings — the
  bias, the offset the criterion needs, what a delivery moved — beside ADR-0076's, and the composition
  by ADR-0072's test at no extra cost: the volley's and the background's terms per synapse per trial
  beside ADR-0076's 8.0, −0.2, 6.6 and −12.7, and the readout sets' own rate beside its 0.28 spikes per
  trial. Produced in **every** branch. If a candidate lowers the readout rate and the sign still fails,
  the ADR says what fell with it.
  **Done** in both branches' form: the bias +7/34 and −1/30 under the settled network, +9/34 and +22/30 under the controller (ADR-0076's +6/34 and +3/30); the offset 2 and 3 (ADR-0076's 1); what the delivery moved ADR-0074's reading, quoted; the composition per synapse per trial +4.3, −0.1, +6.4, −10.2 (sum +0.4) and +12.9, −5.3, +280.4, −284.5 (sum +3.6) beside ADR-0076's; the readout units' rate 0.29 and 4.45 per trial beside 0.28. Neither candidate lowered the readout rate, so the clause for one that does has nothing to say.
- [x] **The constants commit**, preceding the first commit that holds a rewarded outcome: the chosen
  configuration with its lead-in and its settled image, and every constant of ADR-0065, ADR-0068,
  ADR-0069 and ADR-0076 restated unchanged.
  **Done as the round's shape allows:** there is no rewarded outcome for it to precede. `552c82d` on `main` (`61ed272` on the branch before the rebase that merged pull request #96) holds the harness, the settling rule, the measures, the pick and the prediction before either candidate ran, and `96bdc19` (`f59655c`) the readings and the pick (none); every constant of ADR-0065, ADR-0068, ADR-0069 and ADR-0076 stands unchanged, and no configuration was chosen.
- [x] **If a candidate passes — the rule's step 4 — ADR-0069's criterion, unchanged**, at 1 024 units
  with ADR-0068's addressed delivery under the chosen configuration: the addressed rewarded run, the
  mirrored assignment, the addressed shuffled reward and the fixed modulation, as weekly `exhaustive`
  tests; ADR-0069's own runs are the comparison. Over the last 128 trials: rewarded **at least 80** in
  both assignments; the controls **at most 76**; the sequence the same on one worker and on four.
  **The expectation, written first:** with the trace positive, the answer couplings rise above the
  other two — the ratio above the census's 0.992 where ADR-0069 read 0.937.
  **Rejected** by the brief's own rule: no candidate passed all three measures, so step 4 did not arise and the criterion did not run; the expectation written for a passing candidate stays untested.
- [x] **If no candidate passes — the rule's step 3 — the closing, written by this round.** The ADR
  states, as a result: *the three-factor rule with ADR-0068's addressed delivery does not learn the
  two-alternative difference task on the reference network at 1 024 units, because its pairing
  statistics are the background's*, with the terms under each candidate as the evidence. Whitepaper
  §11.1's **H-12** is checked closed with that sentence and its scope — this task, this regime, this
  rule — and the stopping rule's item is checked closed at step 3. The ADR **names, without building**,
  the three routes the next decision chooses among — a task that asks for a sign rather than a
  difference, a structural rule that adds a synapse, a review of the operating regime against
  ADR-0036's criticality — and what this round's readings say about each.
  **Done** (ADR-0077; whitepaper §11.1): the closing sentence recorded as a result with the terms under each candidate as the evidence; H-12 checked closed with the sentence and its scope, the stopping rule's item checked closed at step 3; the three routes named, with what the readings say about each, and not built; no choice among them made.
- [x] **The gate.** At most one test, running no whole run: the first windows of the settled lead-in
  held to the first rows of its pinned table, as ADR-0061's gate test is held to its own. Nothing else
  added to the gate.
  **Done:** one test, `the_first_two_windows_of_the_settled_lead_in_at_1024_units_and_the_rules_over_their_tables`, the rules at their edges and over the pinned tables and the settled lead-in's first two windows held to the first two rows of its table; nothing else added to the gate.
- [x] **The evidence.** A weekly dispatched on this round's branch at the scope
  [ADR-0075](../../docs/adr/0075-the-dispatch-scope-follows-the-diff.md) gives **this round's own diff**,
  the clause that applied stated in the ADR; green in every job it runs, every pinned number
  reproduced; its run id and the three exhaustive shards' times with `instrument.rs`'s against its
  bound in the ADR.
  **Done:** the diff is test code and documents, ADR-0075's last clause, so the weekly was dispatched at `scope=exhaustive` on the round's last commit of code; the run id and the shards' times are in ADR-0077's evidence.
- [x] **The documents, in the same pull request.** Whitepaper §11.1's H-12 and its stopping rule (the
  step this round reached), §8.8's three-factor row, §6.5's loop, §9, the ADR index, `CHANGELOG.md`,
  `README.md`, `CLAUDE.md`'s opening paragraph and `docs/zh-TW`'s reader's guide as the result requires.
  The whitepaper's version moves in **both** declarations with its date
  ([ADR-0064](../../docs/adr/0064-the-documentation-gate-and-the-version.md)).
  **Done:** H-12 and its stopping rule closed in §11.1 (and H-11's note), §8.8's three-factor row, §6.5, §9, the directives, the ADR index, `CHANGELOG.md`, `README.md`, `CLAUDE.md`'s opening paragraph and the reader's guide; the whitepaper 4.29.0 in both declarations with its date.
- [x] **The brief archived** as `briefs/README.md` says, every deliverable dispositioned, the frozen
  banner naming the pull request, the ADRs, **the step of the stopping rule reached** and each measure's
  outcome.
  **Done:** this file.

## Not empowered

- **No constant of the engine moves**, including the controller's: not `CONTROL_STEP_MAX_Q0_16`,
  `GAIN_MIN_Q16`, `GAIN_MAX_Q16`, the target of $\hat\sigma = 1$, the estimator or its window, the
  saturation ceiling — nor the membrane's, the pair rule's, the trace's, the modulator's, the inhibitory
  rule, the sleep constants, `Prior` or the reference prior's parameters.
- **The controller's step is ADR-0055's eighth**, not searched; **the settled candidate's gain is 1.75,
  held**, not searched. No gain ladder.
- **The stimulus is ADR-0076's, pinned** — its efficacies, its offset, its span — under both candidates;
  it is not re-sized to follow the controller's gain.
- **No third background route**: not a quieter drive, not targeted inhibition of the readouts, not a
  different inhibitory period, not a mix of the two candidates.
- No change to ADR-0068's delivery; no reward of one sign; no consolidation at the trial's end.
- No change to the geometry, the readout window, the trial, the block, the run, the baseline, the reward
  magnitude, or either seed.
- **No rewarded run unless a candidate passes all three measures.** No constant chosen or moved after a
  rewarded run; no criterion changed after a run; no clause dropped because it failed.
- **If step 3 applies, this round records the closing and does not choose among the three routes** —
  that choice is the next ADR's, as the stopping rule says.
- No record change, no new section, no format bump; no new crate; no dependency in a state crate; no
  `unsafe` outside the runtime's arena access; no float anywhere.
- No more than one test added to the pull request's gate. No change to `.github/workflows/ci.yml`,
  `scripts/exhaustive-shard.sh` or the sweep's configuration. No move of the determinism pin, and no
  edit to a number a prior round pinned.
- F-47's replay question is not this round's. No claim about 256 units or Appendix A's scale; no use of
  "learns" outside the task, the accuracy, the trials and the controls that produced it.

## Architectural empowerment

The executing session may replace any instruction above with a better decision, recorded in the ADR
that owns it: the exact form of ADR-0055's settling criterion as a rule over windows, so long as it is
at least as strict as ADR-0055's and written before the lead-in runs; saving and loading the settled
engine as an image (after it falls quiet, which encoding requires) or settling it per run; whether the lead-ins and the frozen runs are one test or
several; whether the round writes one ADR or two; which numbers the ADRs restate; and whether the
criterion's controls are reduced **if and only if** the `instrument.rs` shard would otherwise near its
bound, the addressed shuffled reward first and then the mirrored assignment, the rewarded run, the fixed
modulation and the worker clause not droppable. It may not reach the standing directives, the
whitepaper's invariants, the constraints in `CLAUDE.md`, or H-12's stopping rule.

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

The closing message states: **which step of H-12's stopping rule the round reached**; each candidate's
lead-in — the windows it ran, whether ADR-0055's criterion held and where, the sums and the readout
rate at its end, and under the controller the gain's course; the three measures for each candidate in
order and which passed or that none did and at which measure; the composition and Deliverable D's three
numbers under each, beside ADR-0076's; whether the controller did what the prediction said, stated as
the Hypothesis it was; if a candidate passed, the criterion's clauses beside ADR-0069's and the
couplings' ratio against 0.937; if none did, the closing sentence as written into H-12 and what the
readings say about each of the three routes; the scope ADR-0075 gave this diff and the clause that
applied; the shards' times with `instrument.rs`'s against its bound; what the mutation gate and any sweep
found; what was not done and why; and what the re-examination after the round recommends next.
