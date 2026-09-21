---
status: archived
date: 2026-09-21
---

> **Executed 2026-09-21 in pull request #83.** Writes ADR-0072 (what the trace is made of:
> the eligibility on a stimulus's synapses into a readout, read from the record at 1 024
> units with the weights frozen and split by what paired it through an oracle that replays
> the pair rule over the executor's own train and agrees with the record at every reading;
> at the present gain the volley's pairing is net potentiation and the background's net
> depression of four to five times it, so ADR-0066's sentence is a measurement, and its
> mechanism is in the census: each stimulus unit fires about twice more within one pair
> window after its volley, and the rule depresses once per presynaptic spike and potentiates
> at most once per postsynaptic one; the ladder of 1.75, 1.5, 1.25 and 1.0 under the sight
> and the sign, both written first, passes at no rung — at 1.5 the trace turns net
> potentiation while the readout stops seeing the stimulus — so there is no rewarded run
> and no constant moves) and finding F-46 (the instrument's stimulus fires each unit three
> times within the pair window, not once). Image format 14 unchanged; the determinism pin
> untouched; no constant of the engine or the instrument moved. Every deliverable is done;
> notes under the boxes say where the tree departs from the text: the criterion's runs were
> not made, because no rung passed both measures and the brief's own rule for that case is
> no rewarded run; the gate's test runs the first eight trials against the first eight rows,
> the composition being one block; the ladder was not walked at 256 units. The report is in
> the pull request and in `CHANGELOG.md`. The body below describes the tree before execution
> and is not maintained.

# Brief 032 — What the trace is made of: the first measurement of the sentence three decisions have reasoned from, and the first background, chosen by a measure that cannot see the answer, under which the pairings rise

> A self-contained prompt for one round of work. Read `briefs/README.md`, then this file, then
> the files it names. Frozen when executed; the outcome lives in the ADRs that accept it, the
> changelog and the code.

## Mission

[ADR-0069](../../docs/adr/0069-the-addressed-reward-measured.md) reduced the learning question to one
sentence: **"What the rule cannot do is turn a net-depression trace into a coupling that rises;
the reward's addressing changes which synapses fall faster and not whether they fall."** The
delivery of [ADR-0068](../../docs/adr/0068-the-reward-addressed.md) was shown to work — the answer
couplings parted from the other two by four to five per cent, downward, in the direction written
before the run — and to be beside the point while the trace it consolidates is negative.

Why the trace is negative is stated in [ADR-0066](../../docs/adr/0066-the-reward-path-measured-again.md)
and has never been measured: "a readout unit's background spike within the pair window before the
volley is depression, and the background's are the more." Three decisions have reasoned from that
sentence. Nothing in the tree holds a number for it.

This round measures it — what the eligibility on a stimulus's synapses into a readout is actually
made of, split by what paired it — and then, from candidates fixed here in a fixed order and by a
measure that cannot see the answer, finds whether any admissible background makes that trace **net
potentiation**. If one does, ADR-0069's criterion runs under it with every other thing held, and
the expectation is written first: the answer couplings should now **rise** above the other two
rather than fall below them. If none does, the round says so with the ladder and the numbers, and
the tree stops inferring the sentence and starts quoting a measurement of it.

No rule changes. Not one STDP constant, not a window, not a time constant, not ADR-0068's
delivery, not the geometry, the readout window, the trial, the block or either seed. The variable
is the background, and the only handle on it that this round is given is the gain — the same
handle ADR-0065's calibration already turns, with its ladder extended downward and its own pass
mark kept beside the new one.

---

## Standing directives

- **Latest ≠ Newest** (whitepaper §2.1, `CLAUDE.md` principle 4). This round adds **no
  mechanism**. Its first half is a measurement of the tree's own pair rule
  ([ADR-0022](../../docs/adr/0022-synapse-fan-out-and-stdp.md), [ADR-0032](../../docs/adr/0032-three-factor-plasticity.md);
  Bi and Poo 1998; Izhikevich 2007), and its second is a choice among values of a constant the
  instrument already has. **No** new learning rule, gradient, surrogate gradient, e-prop, reward
  predictor, critic, structural plasticity, per-unit synaptic scaling (still gated on H-8), rate
  balance of $A_-$ against $A_+$, or 2025–2026 method; no dependency, no new tool, no version bump
  of a tool. A constant of the *instrument* may be chosen from a ladder written here; a constant
  of the *engine* may not be touched at all.
- Every claim is Implemented, Specified, Target or Hypothesis. What a run holds at 1 024 units is
  stated with the prior's parameters and the configuration; nothing here says anything about
  Appendix A's scale ([ADR-0010](../../docs/adr/0010-measured-or-target.md)). No timing figure from a
  developer machine. No "learns" without the task, the accuracy, the trials and the controls.
- The repository wins over the document; a disagreement is a numbered finding in whitepaper §11.
  ADR-0066's, ADR-0068's and ADR-0069's numbers stay in them and their tests keep pinning them;
  this round is compared with them, never folded into them.
- No `f32`/`f64`, in the crates, the tests and the oracles; a sum of traces is an `i64` of Q1.15
  terms and a ratio is an integer ratio in Q16.16; every operation on a state field saturates or
  wraps by name ([ADR-0029](../../docs/adr/0029-structural-enforcement.md)).
- Every loop ends by construction: a countdown, a range, a scan by `get`, a slice's iterator;
  never by a comparison alone that one operator flip turns into a walk without end
  ([ADR-0062](../../docs/adr/0062-the-first-complete-sweeps-list.md)).
- Every quantity has one owner ([ADR-0016](../../docs/adr/0016-thirty-two-crate-architecture.md)):
  the trace is `cortex-core`'s, the modulation `cortex-neuromod`'s, the gain
  `cortex-homeostasis`'s, the task and its sets the runtime's. **No new crate**; the crate count
  stays 32; no record changes; the image format stays 14; no reserved byte is taken.
- **Nothing is chosen after a rewarded run.** The gain is picked by the two calibrations of
  Deliverable B from the ladder in Deliverable B, both of which read no selection, no reward and
  no outcome, and it is committed before the first rewarded run. What a rewarded run says beyond
  the criterion is a reading (ADR-0051, ADR-0054, ADR-0060, ADR-0066, ADR-0069).
- The determinism pin of [ADR-0030](../../docs/adr/0030-verification-governance.md) does not move,
  and neither does any pinned number of ADR-0065, ADR-0066, ADR-0068 or ADR-0069: nothing here
  changes a rule. A run at a new gain is a **new** run with its own pins beside them. The mutation
  gate on the changed lines must pass; every number an arithmetic oracle can produce is computed
  by that oracle before the test that asserts it.
- A heavy run is an `#[ignore]`d test whose name contains `exhaustive` and runs in the weekly job;
  the pull request's gate grows by at most one test
  ([ADR-0061](../../docs/adr/0061-the-learning-runs-leave-the-gate.md)).
- The engine never amends its own code ([ADR-0031](../../docs/adr/0031-policy-amendment.md)); no
  registry entry (F-37). Conventional Commits with a real body; never commit on `main`; the
  required checks keep their names.

## Context

Re-derived on 2026-09-21 against `main` at `75ab7e1`. Line numbers move; the symbols and the
quoted sentences are what to re-derive.

1. **The sentence, and that it is an inference.** ADR-0066: "a readout unit's background spike
   within the pair window before the volley is depression, and the background's are the more."
   ADR-0069 makes it the binding constraint (quoted in the Mission) and names as its first next
   step "*a background under which the pairings can rise* — a network whose excitatory sum has
   settled before the task …, or a readout whose background spikes fall outside the pair window
   before the volley — so that the trace the delivery consolidates more of is potentiation and the
   answer couplings rise rather than fall; the addressed delivery is the instrument that would
   then read it." **No test in the tree reads the trace's composition.** ADR-0069 reads the
   couplings (the weights) and the readouts' spikes; the eligibility is read only as a whole in
   the two-unit tests of ADR-0032.
2. **The pair rule's constants**, `crates/cortex-core/src/dynamics/synapse.rs`:
   `STDP_TAU_SHIFT = 11` (a time constant of $2^{11} = 2\,048$ ticks, 20.48 ms at
   `TICK_NS` 10 000), `STDP_A_PLUS_Q1_15 = 328` and `STDP_A_MINUS_Q1_15 = 344` — depression is
   the larger by about five per cent per pair before any timing is counted —
   `STDP_DEPRESSION_REFERENCE_Q1_15` and ADR-0055's magnitude scaling,
   `ELIGIBILITY_TAU_SHIFT = 16`. `SynapseBlock::eligibility_q1_15` is a public field of the
   record, so a test that walks the arena reads it as `reference.rs`'s `weights_by_polarity`
   walks the weights. **None of these constants moves in this round.**
3. **What ADR-0069 measured around it.** At 1 024 units, gain 1.75: the readouts hold **3.4 to
   4.3 spikes per trial before the injection**; the presented set fires 50.8 spikes per
   presentation before the readout window opens; the readouts' response is about eight spikes per
   trial with a trial-to-trial spread of about 3.3; a coupling moved by five per cent is half a
   spike of that response, "and the selection is the coin of which readout's background and
   response came out larger that trial". The excitatory sum after the eighth block is 207.40 M
   against the census's 235.82 M.
4. **The calibration this round extends.** `runtime/cortex-runtime/tests/instrument.rs`:
   `GAINS = [0x0001_C000, 0x0002_0000]` (1.75, then 2.0) tried in that order with the weights
   frozen (a modulation baseline of zero writes no weight of either polarity, ADR-0065), the pass
   mark **56 of 64** trials in which the window after the volley held more readout spikes than a
   window of the same length ending at the injection. ADR-0065 picked 2.0 at 256 units and 1.75
   at 1 024, and records that 2.0 "would have passed too (63; 2 119 and 2 094 against 1 293 and
   1 215)" at 1 024. The measure reads no selection, reward or outcome, which is why it is not
   tuning; this round keeps it and adds a second of the same kind.
5. **Why the gain is the handle, and the only one given.** ADR-0044's table: at 1 024 units the
   population fires 2 406 and 2 297 spikes per window at a gain of 1.75 against 12 757 and 11 328
   at 2.0 — five to six times quieter at 1.75. A background spike of a readout unit in the
   $2^{11}$-tick window before the volley is a depression term; fewer background spikes is fewer
   such terms, and the volley's own pairing is unchanged in count. ADR-0069's other route — "a
   network whose excitatory sum has settled before the task" — is **brief 031's subject** at 256
   units and ADR-0055's at 1 024 **under the controller**, which the instrument does not run
   (`instrument.rs`: `control_step_q0_16: 0`, the gain held, `sleep_shift: 0`). Turning the
   controller on moves the gain during the run and would make a calibrated gain meaningless, so
   it is **not** this round's variable; it is named in the ADR as what a next round would try.
6. **The budget** ([ADR-0061](../../docs/adr/0061-the-learning-runs-leave-the-gate.md),
   [ADR-0067](../../docs/adr/0067-the-weekly-dispatch-has-a-scope.md)). The weekly `exhaustive` job's
   time is a range — 33 m, 56 m, 40 m and 1 h 11 m on the runs on record, runner variance of about
   two to one — against a 120-minute bound, and briefs 029, 030 and 031 have each added runs that
   stay. **The comparison at the present gain is free**: ADR-0069's runs are already in the suite
   and are the round's baseline, so the round adds only the runs at the chosen gain. The round
   **measures the job's time and states it**, and if it would near the bound it drops the
   controls it names as droppable in Deliverable D and says so, rather than raising the bound.
   This round **adds tests**, which is
   [ADR-0067](../../docs/adr/0067-the-weekly-dispatch-has-a-scope.md)'s class, so its evidence
   dispatch is `-f scope=both`.
7. **What the answer decides.** If a gain on the ladder makes the trace net potentiation and the
   criterion passes, H-12 closes with a yes and H-11's synaptic half unblocks. If it makes the
   trace net potentiation and the criterion still fails, the tree has the sign it has been
   inferring, the couplings' direction as a check on the whole account, and a next round that
   attacks the magnitude rather than the sign. If no gain on the ladder makes it positive, the
   sentence three decisions rested on is measured and wrong, or right and not reachable from the
   gain, and either is worth more than a fourth decision inferring it again.

## Deliverables

- [x] **The decomposition (the next free ADR number; `ls docs/adr`)** (`depends-on: ADR-0069`;
  ADR-0022, ADR-0032, ADR-0055, ADR-0065 named). At **1 024 units**, in ADR-0065's configuration
  with the **weights frozen** (the modulation baseline at zero, no reward: the traces fill and
  decay and are never written) and the gain at its present 1.75, over **64 trials**, read and pin
  per trial and per block:
  - (a) the summed `eligibility_q1_15` over the synapses from each stimulus set onto each readout
    set, as a signed `i64`, and its sign;
  - (b) that sum split by **what paired it**: the part accumulated within one STDP time constant
    ($2^{11}$ ticks) after the volley's arrival at the readout, and the part accumulated outside
    it, so that the volley's pairing and the background's are two numbers rather than one
    inference;
  - (c) the census behind them: the readout units' spikes in the $2^{11}$ ticks **before** the
    volley's arrival and in the $2^{11}$ ticks **after** it, per trial.

  The round states, as the first measurement of ADR-0066's sentence, whether the background's
  terms outnumber the volley's and by how much. **This is a reading**, in the sense of ADR-0047,
  ADR-0051 and ADR-0054: it stands whatever the rest of the round finds.
  **Done as ADR-0072**, with (b) split further by sign into four classes (the volley's
  potentiation and depression, the background's potentiation and depression), a term
  attributed to the readout spike that paired it, and (c) extended to the presented set's own
  spikes, which is where the mechanism turned out to be.

- [x] **The two calibrations and the ladder, in the same ADR.** The ladder, in this order and no
  other: **1.75 (the present), 1.5 (`0x0001_8000`), 1.25 (`0x0001_4000`), 1.0 (`0x0001_0000`)**.
  At each, with the weights frozen and no reward, over 64 trials, both measures:
  - **Sight** — ADR-0065's, unchanged: the window after the volley holds more readout spikes than
    the window of the same length ending at the injection, in at least **56 of 64** trials, and
    each readout's total after the volley exceeds its total before.
  - **Sign** — this round's: the summed eligibility of (a) over the synapses from the presented
    stimulus onto both readouts is **positive** in at least **56 of 64** trials.

  A gain passes only when **both** pass; the first that does is the gain, and if none does there
  is **no rewarded run** and the round records the ladder with both measures at every rung.
  Neither measure reads a selection, a reward or an outcome, which is what makes this a
  calibration and not a tuning; the order is fixed so that no preference chooses.
  **Done:** 1.75 reads sight 62 and sign 1; 1.5 reads 49 and 45; 1.25 reads 10 and 32; 1.0
  reads 0 and 0. No rung passes both; `GAIN_PICKED_1024` is none.

- [x] **The constants commit**, preceding the first commit that holds a rewarded outcome: the
  chosen gain, the ladder and both pass marks, and every other constant of ADR-0065 and ADR-0068
  restated unchanged.
  **Done:** `3b368f7` holds the ladder, both marks, the split rule, the oracle, the pick (none)
  and every reading; no commit of the round holds a rewarded outcome, since none was run.

- [x] **The criterion, ADR-0069's, unchanged** (the next ADR number, or the same). At **1 024
  units** at the chosen gain, with ADR-0068's addressed delivery and every other thing held: the
  addressed rewarded run, the mirrored assignment, the addressed shuffled reward and the fixed
  modulation. The comparison at the present gain is **ADR-0069's own runs**, already in the suite,
  which must still read 56, 65, 53 and 60; no run is repeated to produce it. Over the last 128
  trials: rewarded **at least 80** in both assignments; the controls **at most 76**; the sequence
  the same on one worker and on four.
  - **The expectation, written before the run and not adjusted to the tables:** if the sign
    calibration passed, the answer couplings (A→R0 and B→R1; mirrored A→R1 and B→R0) **rise above
    the other two** rather than falling below them — the ratio ADR-0069 reads as 0.937 goes above
    the census's 0.992 rather than below it. A criterion that fails with the ratio above 1 is a
    magnitude problem; one that fails with the ratio still below is an account that is wrong
    somewhere this round did not look, and the ADR says which.
  - If the exhaustive job would near its bound, the droppable runs, in this order, are the
    addressed shuffled reward and then the mirrored assignment; the rewarded run, the fixed
    modulation and the worker clause are not droppable, and whatever is dropped is stated.
  **Discharged by the ladder's outcome:** no rung passed both measures, and the deliverable
  above says that then there is no rewarded run. ADR-0069's runs stand as the criterion's
  comparison at the present gain, unchanged in the suite and still reading 56, 65, 53 and 60;
  the expectation about the couplings' ratio was not tested, and ADR-0072 says so.

- [x] **The gate.** One test at 1 024 units running no whole run: the decomposition's first block,
  held to the first row of its pinned table, as ADR-0061's gate test is held to its own. Nothing
  else added to the gate.
  **Done**, as the first eight trials held to the first eight rows of the per-trial table (the
  composition is one block, so its "first block" is the whole run), beside the rules over the
  pinned tables; 19 s in the debug profile on a developer machine.

- [x] **The evidence.** `gh workflow run ci.yml --ref <branch> -f scope=both`
  ([ADR-0067](../../docs/adr/0067-the-weekly-dispatch-has-a-scope.md): this round adds tests), green
  in every job, every pinned number reproduced on the hosted runner, no survivor; its run id, the
  six runtime shards' times, the runtime suite's time before and after, and the weekly exhaustive
  job's time against its 120-minute bound, all in the ADR.
  **Done:** run 35548410926 at `3b368f7`, `scope=both`; the numbers are in ADR-0072's evidence.

- [x] **The documents, in the same pull request.** Whitepaper §8.8's three-factor row gains the
  trace's composition as a measurement rather than a sentence; §11.1's **H-12** says what is now
  known, with the ladder and the criterion; §6.5's "The loop as the runtime composes it" if the
  gain moved; §9, the ADR index, `CHANGELOG.md`, `README.md`, `CLAUDE.md`'s opening paragraph and
  `docs/zh-TW`'s reader's guide as the round's result requires. The whitepaper's version moves in
  **both** declarations with its date
  ([ADR-0064](../../docs/adr/0064-the-documentation-gate-and-the-version.md)). Executable directives
  under every sentence that claims a test or a constant exists.
  **Done:** whitepaper 4.23.0; §6.5 (the gain did not move, and the ladder is stated there),
  §8.8, §9, §11.1's H-11, H-12 and F-46; the ADR index, `CHANGELOG.md`, `README.md`,
  `CLAUDE.md` and the reader's guide.

- [x] **The brief archived** as `briefs/README.md` says, every deliverable dispositioned, the
  frozen banner naming the pull request, the ADRs, the ladder's outcome and the criterion's.
  **Done.**

## Not empowered

- **No constant of the engine moves**: not `STDP_A_PLUS_Q1_15`, `STDP_A_MINUS_Q1_15`,
  `STDP_DEPRESSION_REFERENCE_Q1_15`, `STDP_TAU_SHIFT`, `ELIGIBILITY_TAU_SHIFT`,
  `DOPAMINE_TAU_SHIFT`, ADR-0055's magnitude scaling, the inhibitory rule or its period, the
  estimator, the controller's constants, the sleep constants, `Prior` or the reference prior's
  parameters. The gain is a constant of the **instrument**, held through the image, and is the
  only one this round chooses.
- **No new mechanism**: no per-unit synaptic scaling (gated on H-8, and §11.1 argues it cannot
  lift a weight from zero), no second controller, no rate-dependent balance of $A_-$ against
  $A_+$, no structural rule, no reward predictor, no change to ADR-0068's delivery — not its
  addressing, not its narrowing by both ends, not the moment it consolidates. ADR-0069 names a
  consolidation at the trial's end and a reward of one sign; **naming them in the ADR is the whole
  of this round's licence about them**.
- **The controller stays off** (`control_step_q0_16: 0`, the gain held): it is ADR-0069's other
  route and a second variable, and this round has one.
- No change to the geometry or its masks, the rotations, the readout window, the stimulus, the
  trial, the block, the run, the baseline, the reward magnitude, or either seed.
- No gain outside the ladder; no constant chosen or moved after a rewarded run; no criterion
  changed after a run; no clause dropped because it failed, and no run dropped except as
  Deliverable D allows and states.
- No record change, no new section, no format bump, no reserved byte taken; no new crate; no
  dependency in a state crate; no `unsafe` outside the runtime's arena access; no float anywhere.
- No more than one test added to the pull request's gate. No change to
  `.github/workflows/ci.yml`. No renaming of a required check. No move of the determinism pin, and
  no edit to a number ADR-0065, ADR-0066, ADR-0068 or ADR-0069 pinned.
- No claim about 256 units, whose instrument ADR-0069 read as below the mark and whose settling is
  brief 031's subject; none about Appendix A's scale.

## Architectural empowerment

The executing session may replace any instruction above with a better decision, recorded in the
ADR that owns it: how the decomposition is read (a test-only walk of the arena beside
`weights_by_polarity`, a counter in the harness, a fork of the image per trial), so long as it
reads the record and changes no rule; the exact split in (b), so long as it is a rule of
`STDP_TAU_SHIFT` and the volley's arrival and is written before the measurement runs; the sign
calibration's statistic, so long as it is written in the ADR **before** it runs, reads no
selection, reward or outcome, and is at least as strict as 56 of 64; whether the round writes one
ADR or two; which numbers the ADRs restate; and whether the ladder is walked at 256 units as well
**as a reading only**, if and only if the exhaustive job's time allows and the ADR states the
cost. It may not reach the standing directives, the whitepaper's invariants or the constraints in
`CLAUDE.md`.

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
image format and every number ADR-0065, ADR-0066, ADR-0068 and ADR-0069 pinned are unchanged.

## Report

The closing message states: what the trace is made of — the volley's terms against the
background's, per block, with the census behind them, and whether ADR-0066's sentence is confirmed
as a measurement; the ladder, both measures at every rung, and which gain passed or that none did;
the constants commit and the first outcome commit; the criterion clause by clause at 1 024 units
beside ADR-0069's 56, 65, 53 and 60; the couplings' ratio against the expectation written before
the run, and what a failure with the ratio above 1 means against one with it still below; the
runtime suite's time before and after, the weekly exhaustive job's time against its bound, and
what the mutation gate and the sweep found; what was not done and why; and what the re-examination
after the round recommends next.
