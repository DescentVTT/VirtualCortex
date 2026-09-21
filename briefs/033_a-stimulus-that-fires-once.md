---
status: proposed
date: 2026-09-21
---

# Brief 033 — A stimulus that fires each unit once inside the pair window: the one change F-46 names, the composition re-read against a number written before the run, and the sign calibration asked again at the gain that already sees

> A self-contained prompt for one round of work. Read `briefs/README.md`, then this file, then
> the files it names. Frozen when executed; the outcome lives in the ADRs that accept it, the
> changelog and the code.

## Mission

[ADR-0072](../docs/adr/0072-what-the-trace-is-made-of.md) measured what three decisions had only
inferred, and the measurement found a defect in the instrument rather than in the rule. The
stimulus of [ADR-0065](../docs/adr/0065-the-instrument-recalibrated.md), described in every
document since as firing each of its units once, **fires each about three times within one pair
window of the volley**: once before the readout window opens — the window its calibration
counted — and 2.06 more inside the pair window after it. That is **F-46**, and the pair rule
enters a depression at every presynaptic spike, so those extra spikes are most of the
background's depression on the stimulus's synapses into the readouts and all of the volley's own.

ADR-0072 also wrote down what removing them would meet: the volley's depression, "which is the
response paired at those spikes", goes "to nothing, and the terms would sum to about **+5 per
synapse per trial, net potentiation at a gain at which the readout sees**". And it closed the
other door in the same breath: "*The gain is not the handle*: below 1.75 the readout stops seeing
before the sign passes, and the two windows do not overlap on this ladder."

So this round makes the one change F-46 names — **a stimulus that fires each unit once inside the
pair window** — and nothing else. It re-reads ADR-0072's composition under it against that +5,
asks ADR-0065's sight and ADR-0072's sign again **at the present gain of 1.75**, and, only if
both pass, runs ADR-0069's criterion with ADR-0068's addressed delivery and every other thing
held. The rule is untouched; the geometry, the readout window, the trial, the block, the baseline,
the reward and both seeds are untouched; the gain does not move. The variable is the stimulus.

If the sign passes, this is the first time the trace the delivery consolidates is positive, and
the expectation written before the run is that the answer couplings **rise** where ADR-0069 read
them fall. If it fails, the terms say by how much and against which of ADR-0072's numbers, and
the tree has a second measurement of the composition instead of a second inference.

---

## Standing directives

- **Latest ≠ Newest** (whitepaper §2.1, `CLAUDE.md` principle 4). This round adds **no
  mechanism**: it changes how many messages an existing harness injects, which is a constant of
  the instrument, and re-reads two measures that already exist. **No** new learning rule,
  gradient, surrogate gradient, e-prop, reward predictor, critic, structural plasticity, per-unit
  synaptic scaling (still gated on H-8), rate balance of $A_-$ against $A_+$, controller, or
  2025–2026 method; no dependency, no new tool, no version bump of a tool. A constant of the
  *engine* may not be touched at all.
- Every claim is Implemented, Specified, Target or Hypothesis. ADR-0072's +5 is a **Hypothesis**
  and is named as one wherever this round compares against it
  ([ADR-0010](../docs/adr/0010-measured-or-target.md)). Nothing here says anything about 256
  units, whose instrument ADR-0070 found unusable behind a lead-in of any length, or about
  Appendix A's scale. No timing figure from a developer machine.
- The repository wins over the document; a disagreement is a numbered finding in whitepaper §11.
  ADR-0065's, ADR-0066's, ADR-0069's, ADR-0070's and ADR-0072's numbers stay in them and their
  tests keep pinning them: **every one of them was read under the stimulus F-46 describes**, so
  this round's runs are new runs with new pins beside them and never edits to theirs.
- No `f32`/`f64`, in the crates, the tests and the oracles; a sum of traces is an `i64` of Q1.15
  terms; every operation on a state field saturates or wraps by name
  ([ADR-0029](../docs/adr/0029-structural-enforcement.md)).
- Every loop ends by construction: a countdown, a range, a scan by `get`, a slice's iterator
  ([ADR-0062](../docs/adr/0062-the-first-complete-sweeps-list.md)).
- Every quantity has one owner ([ADR-0016](../docs/adr/0016-thirty-two-crate-architecture.md)):
  the membrane and its threshold are `cortex-core`'s, the trace `cortex-core`'s, the modulation
  `cortex-neuromod`'s, the task and its stimulus the runtime's. **No new crate**; the crate count
  stays 32; no record changes; the image format stays 14; no reserved byte is taken.
- **Nothing is chosen after a rewarded run.** The stimulus is picked by the composition measure of
  Deliverable A from the candidates listed there, in the order listed there, and that measure
  reads no selection, no reward and no outcome; it is committed before the first rewarded run,
  with every other constant restated (ADR-0051, ADR-0054, ADR-0060, ADR-0066, ADR-0069, ADR-0072).
- The determinism pin of [ADR-0030](../docs/adr/0030-verification-governance.md) does not move:
  nothing here changes a rule of any crate. The mutation gate on the changed lines must pass; a
  new rule of the task module carries a test over the lattice of `testkit/prop.rs`; every number
  an arithmetic oracle can produce is computed by that oracle before the test that asserts it.
- A heavy run is an `#[ignore]`d test whose name contains `exhaustive` and runs in the weekly job;
  the pull request's gate grows by at most one test
  ([ADR-0061](../docs/adr/0061-the-learning-runs-leave-the-gate.md)).
- The engine never amends its own code ([ADR-0031](../docs/adr/0031-policy-amendment.md)); no
  registry entry (F-37). Conventional Commits with a real body; never commit on `main`; the
  required checks keep their names.

## Context

Re-derived on 2026-09-21 against `main` at the commit that merges brief 032's round. Line numbers
move; the symbols and the quoted sentences are what to re-derive.

1. **F-46, as the whitepaper records it.** The stimulus — `STIMULUS_Q16 = 0x0001_4000` (1.25) and
   `STIMULUS_MESSAGES = 2` into every unit of a set spaced beyond the prior's window — "fires each
   about three times within one pair window of the volley at 1 024 units and the present gain:
   once before the readout window opens, which is the window ADR-0065's calibration counted and
   where it read one per unit within two spikes, and 2.06 more inside the pair window after it".
   Its disposition is **Open**, "recorded, not fixed", because the stimulus is a constant brief
   032 held.
2. **The terms, per synapse, at the present gain** ([ADR-0072](../docs/adr/0072-what-the-trace-is-made-of.md),
   `tests/instrument.rs`): the volley's potentiation 17.2 per presentation and its depression
   10.4, net **+6.8** (+6.8 to +8.4 over the four pairs); the background's potentiation 7.5 per
   trial and its depression 26.8, net **−19.3**; the terms sum to **−15.7 per trial**, and the
   standing trace after the block is **−66.6 per synapse**, "that flow over the trace's time
   constant of four trials". The depression is 0.64 to 0.66 of the terms' magnitude. The four
   pairs agree within a quarter.
3. **The census behind them**, per trial at 1.75: the presented set holds 52.5 spikes in the pair
   window before the readout window opens ("its volley of one per unit for 51 units and about 1.5
   of background") and **105.3 in the pair window after it**. A stimulus unit's spikes per trial
   are **1.89** — the volley's 0.53, the pair window's **1.09**, and 0.27 elsewhere — against a
   readout unit's 0.28, 6.7 to one. Readout 0 holds 16.2 spikes in the pair window before the
   opening and 35.4 after; readout 1, 16.2 and 35.5.
4. **The estimate this round is measured against.** ADR-0072: a stimulus that fires each unit once
   would take "the volley's depression, which is the response paired at those spikes, to nothing,
   and the terms would sum to about **+5 per synapse per trial**, net potentiation at a gain at
   which the readout sees". It is a **Hypothesis**, from the terms of item 2 and not from a run.
5. **Why the gain is not the handle.** ADR-0072's ladder at 1 024 units: at 1.75 the sign reads
   **1 of 64** with the sight at 62; at 1.5 the trace turns net potentiation (the sign 45) and the
   readout no longer sees (the sight **49**); at 1.25 and 1.0 the readouts are all but silent. "No
   rung passes both", and "below 1.75 the readout stops seeing before the sign passes, and the two
   windows do not overlap on this ladder". **The gain stays at 1.75 in this round.**
6. **What the delivery does, once the trace is positive.** [ADR-0068](../docs/adr/0068-the-reward-addressed.md)
   showed the dopamine term reaching the synapses from the presented stimulus onto the selected
   readout, and [ADR-0069](../docs/adr/0069-the-addressed-reward-measured.md) measured the answer
   couplings parting from the other two by four to five per cent **downward**, "in the direction
   the trace's sign set", with the ratio of the answer couplings to the others at **0.937** against
   0.982 under the global form and the census's 0.992 before any trial. ADR-0069: "a net-positive
   trace raises the coupling from a stimulus into its answer readout above the coupling into the
   other, and the selection follows it toward the answer".
7. **The budget** ([ADR-0067](../docs/adr/0067-the-weekly-dispatch-has-a-scope.md),
   [ADR-0071](../docs/adr/0071-the-exhaustive-job-times-its-own-binaries.md),
   [ADR-0073](../docs/adr/0073-the-whole-domain-tests-sharded.md)). The whole-domain tests now run
   in three shards, and `instrument.rs` is a shard by itself: 22 tests in **3 007 s** of that
   shard's 120-minute bound, half of the tree's `exhaustive` seconds, and briefs 029, 030 and 032
   each added to it. ADR-0073 names its own limit: sharding cannot split that file. **This round
   adds to that shard**, so it states its shard's time from the dispatch's table, and if the shard
   nears its bound the round says so and names what would come out — it does not raise the bound.
   This round **adds tests**, which is ADR-0067's class, so its evidence dispatch is
   `-f scope=both`.
8. **What waits on this.** Whitepaper §11.1's **H-12** carries three no's and, since ADR-0069, a
   delivery that works into a trace with the wrong sign. **H-11**'s synaptic half still waits on a
   reward that changes a behaviour at all.

## Deliverables

- [ ] **The stimulus, and the composition under it (the next free ADR number; `ls docs/adr`)**
  (`depends-on: ADR-0072`; ADR-0065, ADR-0068 and F-46 named). In
  `runtime/cortex-runtime/tests/instrument.rs`, a stimulus that fires each unit of the presented
  set **once inside the pair window**. The candidates, in this order and no other:
  - (a) **one message at the unit's threshold** rather than two of 1.25 — the value derived from
    the record's threshold and stated, not searched;
  - (b) **one message of 1.25**, if (a) does not fire every unit;
  - (c) the shape the round argues for, written in the ADR **before** it is measured, which must
    leave the geometry, the window, the trial and the seeds untouched.

  **The measure that picks, which reads no outcome:** the presented set's spikes in the pair
  window after the readout window opens, against its volley. A candidate passes when the set fires
  **one spike per unit within two** in the window before the opening — ADR-0065's own reading,
  kept — **and adds no more than 0.1 spikes per unit** inside the pair window after it, against
  the 2.06 F-46 records. The first candidate that passes is the stimulus; if none passes, there is
  no rewarded run and the round records the candidates with their censuses.
  - **The composition re-read** under the chosen stimulus, by ADR-0072's own test at no extra
    cost: the volley's potentiation and depression, the background's, their sum per trial and the
    standing trace per synapse after the block, each beside ADR-0072's 17.2, 10.4, 7.5, 26.8,
    −15.7 and −66.6. The ADR states whether the volley's depression went "to nothing" and whether
    the sum met the **+5 per synapse per trial** ADR-0072 estimated, **naming that number as the
    Hypothesis it is** and not adjusting anything to it.
- [ ] **The two calibrations, at the gain 1.75, unchanged.** ADR-0065's **sight** (56 of 64) and
  ADR-0072's **sign** (the presented stimulus's summed eligibility onto both readouts positive in
  56 of 64), both with the weights frozen and no reward, both written first and neither reading a
  selection, a reward or an outcome. **The gain does not move and the ladder is not walked**
  (Context item 5). If the sign fails, there is no rewarded run: the round reports the terms, the
  census and both measures, and names — without building — what remains.
- [ ] **The constants commit**, preceding the first commit that holds a rewarded outcome: the
  chosen stimulus with its census, and every constant of ADR-0065, ADR-0068 and ADR-0069 restated
  unchanged.
- [ ] **The criterion, ADR-0069's, unchanged**, run only if both calibrations pass. At **1 024
  units** with ADR-0068's addressed delivery: the addressed rewarded run, the mirrored assignment,
  the addressed shuffled reward and the fixed modulation, as weekly `exhaustive` tests. The
  comparison is **ADR-0069's own runs**, already in the suite, which must still read 56, 65, 53
  and 60 under the old stimulus; no run is repeated to produce it. Over the last 128 trials:
  rewarded **at least 80** in both assignments; the controls **at most 76**; the sequence the same
  on one worker and on four.
  - **The expectation, written before the run and not adjusted to the tables:** with the trace
    positive, the answer couplings **rise above** the other two — the ratio ADR-0069 read as 0.937
    goes above the census's 0.992 — and the behaviour follows the couplings, so an accuracy above
    chance in both assignments is what a delivery into a positive trace looks like. A criterion
    that fails with the ratio above 1 is a magnitude problem and the round says what magnitude; one
    that fails with the ratio still below 1 means the sign did not reach the couplings, and the
    round says which of the terms it did not.
- [ ] **The gate.** At most one test, running no whole run: the composition's first block under the
  chosen stimulus, held to the first row of its pinned table as ADR-0061's gate test is. Nothing
  else added to the gate.
- [ ] **The evidence.** `gh workflow run ci.yml --ref <branch> -f scope=both`, green in every job,
  every pinned number reproduced, no survivor; its run id, **the three exhaustive shards' times
  with `instrument.rs`'s named against its 120-minute bound** (Context item 7), the six runtime
  sweep shards' times and the runtime suite's time before and after, all in the ADR.
- [ ] **The documents, in the same pull request.** Whitepaper §11's **F-46**, whose fix this is —
  resolved with the census under the new stimulus, or narrowed with what remains; §11.1's **H-12**
  and H-11's synaptic half; §8.8's three-factor row; §6.5's "The loop as the runtime composes it";
  §9, the ADR index, `CHANGELOG.md`, `README.md`, `CLAUDE.md`'s opening paragraph and
  `docs/zh-TW`'s reader's guide as the result requires. The whitepaper's version moves in **both**
  declarations with its date
  ([ADR-0064](../docs/adr/0064-the-documentation-gate-and-the-version.md)). Executable directives
  under every sentence that claims a test or a constant exists.
- [ ] **The brief archived** as `briefs/README.md` says, every deliverable dispositioned, the
  frozen banner naming the pull request, the ADRs and the outcome of each measure.

## Not empowered

- **No constant of the engine moves**: not `STDP_A_PLUS_Q1_15`, `STDP_A_MINUS_Q1_15`,
  `STDP_DEPRESSION_REFERENCE_Q1_15`, `STDP_TAU_SHIFT`, `ELIGIBILITY_TAU_SHIFT`,
  `DOPAMINE_TAU_SHIFT`, ADR-0055's magnitude scaling, the membrane's threshold or its refractory
  window, the inhibitory rule or its period, the estimator, the controller, the sleep constants,
  `Prior` or the reference prior's parameters.
- **The gain does not move and the ladder is not walked.** ADR-0072 measured that the two windows
  do not overlap on it; a round that walked it again would be spending the weekly job on a
  measured negative.
- **The controller stays off.** It is ADR-0069's and ADR-0072's other route and a second variable.
- No change to ADR-0068's delivery — not its addressing, not its narrowing by both ends, not the
  moment it consolidates; no reward of one sign; no consolidation at the trial's end. ADR-0069 and
  ADR-0072 name these and naming them in the ADR is the whole of this round's licence about them.
- No change to the geometry or its masks, the rotations, the readout window, the trial, the block,
  the run, the baseline, the reward magnitude, or either seed.
- No constant chosen or moved after a rewarded run; no criterion changed after a run; no clause
  dropped because it failed.
- No record change, no new section, no format bump, no reserved byte taken; no new crate; no
  dependency in a state crate; no `unsafe` outside the runtime's arena access; no float anywhere.
- No more than one test added to the pull request's gate. No change to
  `.github/workflows/ci.yml`, to `scripts/exhaustive-shard.sh` or to the sweep's configuration. No
  renaming of a required check. No move of the determinism pin, and no edit to a number ADR-0065,
  ADR-0066, ADR-0068, ADR-0069, ADR-0070 or ADR-0072 pinned.
- No claim about 256 units or about Appendix A's scale; no use of "learns" outside the task, the
  accuracy, the trials and the controls that produced it.

## Architectural empowerment

The executing session may replace any instruction above with a better decision, recorded in the
ADR that owns it: the shape of the stimulus, within the candidates and their order, or a shape of
its own argued **before** it is measured; how the pair-window census is read under it, so long as
it is ADR-0072's reading and comparable with its numbers; whether the composition and the
calibrations are one test or three; whether the round writes one ADR or two; which numbers the
ADRs restate; and whether the criterion's controls are reduced **if and only if** the
`instrument.rs` shard would otherwise near its 120-minute bound, in which case the droppable runs
are the addressed shuffled reward and then the mirrored assignment, the rewarded run, the fixed
modulation and the worker clause are not droppable, and whatever is dropped is stated. It may not
reach the standing directives, the whitepaper's invariants or the constraints in `CLAUDE.md`.

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
gh workflow run ci.yml --ref <this round's branch> -f scope=both
```

Every command exits 0; the in-diff gate reports no survivor in CI. The dispatched run is green in
every job, reproduces every pinned number and finds no survivor over the tree. The constants
commit precedes the first commit that holds a rewarded run's outcome. The determinism pin, the
image format and every number ADR-0065, ADR-0066, ADR-0068, ADR-0069, ADR-0070 and ADR-0072 pinned
are unchanged.

## Report

The closing message states: the stimulus as chosen, which candidate it was and its census against
F-46's 2.06; the composition under it, term by term beside ADR-0072's, and whether the sum met the
+5 that decision estimated, said as the Hypothesis it was; both calibrations at 1.75; the
constants commit and the first outcome commit; if the criterion ran, its clauses at 1 024 units
beside ADR-0069's 56, 65, 53 and 60, and the couplings' ratio against the 0.937 and the expectation
written first; if it did not, which measure stopped it and what the terms say remains; the three
exhaustive shards' times with `instrument.rs`'s against its bound; the runtime suite's time before
and after, and what the mutation gate and the sweep found; what was not done and why; and what the
re-examination after the round recommends next.
