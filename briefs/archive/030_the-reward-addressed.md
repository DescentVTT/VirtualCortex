---
status: archived
date: 2026-09-20
---

> **Executed 2026-09-20 in pull request #77.** Writes ADR-0068 (the reward addressed: the
> modulation a synapse consolidates under is a function of both its ends, the dopamine term
> reaching the synapses from the units of the stimulus presented onto the units of the readout
> the engine selected and every other synapse consolidating under the baseline alone, with
> `cortex-core`'s rule, the records and the image unchanged, a run that addresses every unit
> ADR-0066's bit for bit, and what the delivery can do on this task written before the run) and
> ADR-0069 (the measurement: at 1 024 units the addressed rewarded run reads 56 correct of the
> last 128 and the mirrored assignment 65 against the 80 the criterion asks for, the addressed
> shuffled reward 53, the fixed modulation 60 and the global form 49 reproduced, the run the same
> on one worker and on four; the couplings from a stimulus into its rewarded readout part from
> the other two by three to five per cent in the direction the trace's sign set, and the
> behaviour reads nothing of it; at 256 units the instrument is below the mark from the first
> block and the size is not measured; no constant moves). No finding; H-12 and H-11's synaptic
> half carry the outcome; image format 14 unchanged; the determinism pin untouched. Every
> deliverable is done; notes under the boxes say where the tree departs from the text: the
> addressed set is narrowed by the presynaptic side, and the refusal for a set outside the arena
> is the executor's rather than `Task::check`'s. The report is in the pull request and in
> `CHANGELOG.md`. The body below describes the tree before execution and is not maintained; its
> relative links gained one `../`.

# Brief 030 — The reward addressed: the same rule, the same instrument and the same four controls, and a modulation that reaches the synapses onto the readout the engine selected instead of every synapse alike

> A self-contained prompt for one round of work. Read `briefs/README.md`, then this file, then
> the files it names. Frozen when executed; the outcome lives in the ADRs that accept it, the
> changelog and the code.

## Mission

[ADR-0066](../../docs/adr/0066-the-reward-path-measured-again.md) asked whitepaper §11.1's **H-12**
through an instrument that had been shown to see, and answered no — and this time the answer
came with a mechanism: "a global scalar consolidates pairings that are alike into both readouts
… and net depression under this background". Its readings say it precisely. The two readouts'
responses to the two stimuli differ by **under 0.7 spikes per trial in every block**; the four
couplings fall alike under every feedback and stay equal within a few per cent, "so no asymmetry
the assignment could read emerges".

A reward that is one number, broadcast to every synapse in the arena, cannot produce that
asymmetry: the traces it multiplies are the same into the readout that was selected and the one
that was not. This round changes **exactly that, and nothing else** — where the reward is
delivered. The modulation a synapse consolidates under becomes a function of the unit that
synapse targets: the dopamine term reaches the synapses onto the readout the engine selected,
and every other synapse keeps the baseline it has today. The rule of `cortex-core` is untouched,
because `SynapseBlock::consolidate` already takes its modulation per slot; the instrument is
[ADR-0065](../../docs/adr/0065-the-instrument-recalibrated.md)'s, unchanged and uncalibrated again;
the controls, the seeds and the criterion are ADR-0066's, unchanged. The image format stays 14,
no record changes, and the determinism pin does not move — held by a test in which an addressing
that reaches every unit alike reproduces ADR-0066's run **bit for bit**.

The question is H-12's, for the third time: does a reward change which of two readouts the
engine selects for a stimulus? With the instrument held and the delivery the only variable, an
answer either way is about the delivery.

---

## Standing directives

- **Latest ≠ Newest** (whitepaper §2.1, `CLAUDE.md` principle 4). Addressing a reinforcement
  signal to the units an action was read from is not a new rule: it is the classical answer to
  credit assignment in a three-factor setting (Roelfsema and van Ooyen 2005; Roelfsema and
  Holtmaat 2018), and Legenstein, Pecevski and Maass 2008 — which ADR-0066 cites for the failure
  — is the analysis of *when a global signal is sufficient*, whose condition this network does
  not meet. It stays inside ADR-0032's three factors: the pairing, the trace, the modulator.
  **No** gradient, surrogate gradient, e-prop, reward predictor, critic, per-synapse learning
  rate, structural plasticity or 2025–2026 method enters this round, however recent its results;
  no dependency, no new tool, no version bump of a tool.
- Every claim is Implemented, Specified, Target or Hypothesis. What a run holds at 256 or 1 024
  units is stated with the prior's parameters and the task's; what the same loop does at
  Appendix A's scale is a Target ([ADR-0010](../../docs/adr/0010-measured-or-target.md)). No timing
  figure from a developer machine. No "learns", "understands" or "generalises" without the task,
  the accuracy, the trials it was counted over and the controls it held against.
- The repository wins over the document; a disagreement is a numbered finding in whitepaper §11.
  ADR-0059, ADR-0060, ADR-0065 and ADR-0066 are history: their numbers stay in them and their
  tests keep pinning them. This round is compared with them, never folded into them.
- No `f32`/`f64`, in the crates, the tests and the oracles; an accuracy is a count of trials;
  every operation on a state field saturates or wraps by name
  ([ADR-0029](../../docs/adr/0029-structural-enforcement.md)).
- Every loop ends by construction: a countdown, a range, a scan by `get`, a slice's iterator;
  never by a comparison alone that one operator flip turns into a walk without end
  ([ADR-0062](../../docs/adr/0062-the-first-complete-sweeps-list.md)).
- Every quantity has one owner ([ADR-0016](../../docs/adr/0016-thirty-two-crate-architecture.md)):
  the selection is `cortex-basal-ganglia`'s, the modulation `cortex-neuromod`'s, the trace and
  the consolidation `cortex-core`'s, the task and its sets the runtime's. **No new crate**; the
  crate count stays 32; no record changes; the image format stays 14; no reserved byte is taken.
- **Nothing is chosen after a rewarded run.** Every constant is written in this brief or derived
  by a rule from the instrument's, and all of them are committed before the first rewarded run.
  What a rewarded run says beyond the criterion is a reading, never a reason to move a constant
  (ADR-0051, ADR-0054, ADR-0060, ADR-0066).
- The determinism pin of [ADR-0030](../../docs/adr/0030-verification-governance.md) does not move,
  and neither does any pinned number of ADR-0066: the uniform case of the new addressing is the
  old behaviour bit for bit, and a test holds it so. The mutation gate on the changed lines must
  pass; a new rule carries a test over the lattice of `testkit/prop.rs`; every number an
  arithmetic oracle can produce is computed by that oracle before the test that asserts it.
- A heavy run is an `#[ignore]`d test whose name contains `exhaustive` and runs in the weekly
  job; the pull request's gate grows by at most one run
  ([ADR-0061](../../docs/adr/0061-the-learning-runs-leave-the-gate.md)).
- The engine never amends its own code ([ADR-0031](../../docs/adr/0031-policy-amendment.md)); no
  registry entry (F-37). Conventional Commits with a real body; never commit on `main`; the
  required checks keep their names.

## Context

Re-derived on 2026-09-20 against `main` at `015bc13`. Line numbers move; the symbols and the
quoted sentences are what to re-derive.

1. **What ADR-0066 measured, and the sentence this round acts on.** Under the criterion over the
   last 128 trials (at least 80 correct rewarded, at most 76 under a control, the sequence the
   same on one worker and on four; the rates by an integer oracle, one in 337 and one in 74) the
   rewarded run read **53 at 256 units and 49 at 1 024**, the mirrored assignment 53 and 65, the
   controls 51, 51, 55 and 60. The calibration had passed first (the stimulus read in 58 of 64
   trials at 256 units at a gain of 2.0, 62 of 64 at 1 024 at 1.75, the weights frozen), so the
   no is about the rule. Its readings: the instrument "resolves the stimulus from the background
   … but not one stimulus from the other, the two readouts' responses differing by under 0.7
   spikes per trial in every block"; "every coupling from a stimulus set into a readout set falls
   alike under every feedback (by half at 256 units, by a quarter at 1 024) and the four stay
   equal within a few per cent, so no asymmetry the assignment could read emerges". Its named
   next steps, not built: **a per-unit learning signal**, a structural rule, a different task.
2. **Where the modulation is applied, and why the change is small.**
   `runtime/cortex-runtime/src/executor.rs`, `phase_fan_out`: one
   `let modulation = shared.modulation.load(Ordering::Relaxed)` for the whole arena, then for
   each spiked unit the chain of its blocks, and per block `block.step_stdp_all(...)` followed by
   `block.consolidate_all(modulation, polarity)`. A block's polarity is its **presynaptic**
   unit's (ADR-0049) and its slots' targets are postsynaptic units, read in the same loop for
   `posts` through `block.target(slot)`. In `crates/cortex-core/src/dynamics/synapse.rs`,
   `consolidate(&mut self, slot, modulation_q16, polarity)` **already takes the modulation per
   slot**; `consolidate_all` is the one that broadcasts a single value over the block. Addressing
   therefore needs no change to `cortex-core`'s rule and no new record: it is the executor
   choosing a modulation per slot from the slot's target.
3. **What the baseline means, and what the addressing must leave alone.**
   `cortex-neuromod`'s `modulation(baseline)` is `clamp(baseline + dopamine, 0, 1)`; the harness
   runs at a baseline of 0.5 (`BASELINE_Q16 = 0x8000`) and ADR-0066's "fixed modulation" control
   is the baseline alone with no reward — the pair rule by itself. **The addressing applies to
   the dopamine term only**: an addressed synapse consolidates under
   `clamp(baseline + dopamine, 0, 1)` as today, and an unaddressed one under `clamp(baseline, 0, 1)`,
   which is the fixed-modulation control's rule. With the dopamine at zero the two are the same
   number, which is why the uniform case is bit for bit today's and why the determinism pin
   cannot move. Nothing about the background rule changes.
4. **The instrument, as ADR-0065 built it** (`runtime/cortex-runtime/tests/instrument.rs`, the
   block "written before the run"): `Set { first, period, mask, count }` in
   `runtime/cortex-runtime/src/task.rs`, with `Set::contiguous` kept as ADR-0059's shape;
   `PERIOD = 20`, `A_OFFSET = 0`, `B_OFFSET = 11`, `R0_MASK = 0xAA2AA`, `R1_MASK = 0x55554`,
   held by `const _` assertions (the sets disjoint, the readouts of equal size, the stimuli
   further apart than `PRIOR_WINDOW = 8`); `ROTATION_256 = 17` and `ROTATION_1024 = 0`, chosen
   as the smallest rotation whose four couplings are equal within ten per cent against the
   prior's census (`the_geometry_holds_against_the_census_at_both_sizes`);
   `WINDOW = Window { from: 100, ticks: 500 }` derived from the local delay band;
   `LEAD_IN = WINDOW.ticks`; `GAINS = [0x0001_C000, 0x0002_0000]` with the calibration's choice
   2.0 at 256 units and 1.75 at 1 024; `TRIAL_TICKS = 1 << DOPAMINE_TAU_SHIFT` ($2^{14}$),
   `BLOCK = 64`, `TRIALS = 8 × BLOCK`, `LAST_BLOCKS = 2`, `BASELINE_Q16 = 0x8000`,
   `REWARD_Q16 = ONE`, `STIMULUS_Q16 = 0x0001_4000`, `STIMULUS_MESSAGES = 2`, `SEED = 27`, the
   prior's seed 22. **None of these moves in this round.**
5. **Which size carries a criterion.** ADR-0066: at 256 units "the calibration's measure falls
   from 57 of 64 in the first block to 31 in the eighth as the excitatory sum falls to 0.55 of
   the prior's, so the instrument loses the stimulus within a block once the weights move, while
   at 1 024 units it stays at 57 to 64 through the run". A criterion read through a readout that
   no longer sees is ADR-0060 again. **The criterion is therefore at 1 024 units**; 256 is one
   reading, under the rule in the deliverables.
6. **The budget** ([ADR-0061](../../docs/adr/0061-the-learning-runs-leave-the-gate.md),
   [ADR-0067](../../docs/adr/0067-the-weekly-dispatch-has-a-scope.md)). A run of 512 trials of
   $2^{14}$ ticks is $2^{23}$ ticks, and brief 029 put ten such runs into the weekly `exhaustive`
   job. That job's time is **a range, not a figure**: 33 m and 56 m before brief 029, 40 m in run
   `35502753399` and **1 h 11 m** in run `35509558631` — the same tests on a different runner,
   because runner variance of about two to one dominates it (ADR-0067, which measured it; ADR-0061
   measured the same two to one on an unchanged `reference.rs`). Against the job's 120-minute bound
   the headroom at the top of that range is under an hour, and this round adds six runs
   (Deliverable C). The round therefore **measures the job's time and states it**; if it would near
   the bound, the round drops the 256-unit reading, which the empowerment allows for this reason,
   and says so. The bound is never raised to fit a round. The gate gains one run (Deliverable D).
   This round **adds tests**, which is exactly
   [ADR-0067](../../docs/adr/0067-the-weekly-dispatch-has-a-scope.md)'s class, so its evidence
   dispatch is `-f scope=both`.
7. **What a yes would license, and what it would not.** H-12 closes with a yes only for this
   task at the sizes measured, under this instrument and this addressing; nothing about
   Appendix A's scale, and nothing about a task the round did not run. **H-11**'s synaptic half
   (a rewarded invention biasing a later behaviour) waits on a reward that changes a behaviour at
   all, so a yes here unblocks it and a no leaves it where it is. §6.5's Scenario R-5 carries the
   loop.

## Deliverables

- [x] **The addressing ADR (the next free number; `ls docs/adr`)**
  **Done as [ADR-0068](../../docs/adr/0068-the-reward-addressed.md).** The shape is two flags per unit, a source and a target, and a modulation per slot in the fan-out (`Modulations`, `consolidate_each`, `Executor::address`); (a) is held by `consolidate_each_under_one_modulation_is_consolidate_all` with the previous call as its oracle and by every pinned number of ADR-0060 and ADR-0066, unchanged; (b) holds, with `Set::units()` declaring its iterator `Clone`; (c): the presynaptic side narrows the set, argued before the run, because the reward is consolidated at the next presynaptic spike, the next presentation of a stimulus, so without the narrowing the reward of one trial would reach the other stimulus's synapses at half the trials; (d): every refusal `Task::check` writes is kept, and the refusal for a set outside the arena is the executor's (`AddressError::NoSuchUnit`, refused whole on either side with the set left as it was), surfaced as `TaskError::Address`, since `check` holds every readout inside the arena and a trial never meets it; the lattice property is `the_addressed_modulation_is_the_baseline_with_the_signal_and_the_other_the_baseline_alone`. (`depends-on: ADR-0066`;
  ADR-0032, ADR-0049, ADR-0059 and ADR-0065 named). In `runtime/cortex-runtime`, the modulation
  a synapse consolidates under becomes a function of the unit the synapse targets, applied to the
  dopamine term only (Context item 3). The shape is the round's to argue — a per-unit slice in
  `Shared`, a flag the task writes and phase 2 reads, a closure passed into the fan-out — subject
  to all of:
  - (a) **the uniform case is today's, bit for bit**: with every unit addressed, a run reproduces
    ADR-0066's pinned table exactly, held by a test that keeps the previous call as its oracle,
    as ADR-0062's rewrites are held; the determinism pin does not move;
  - (b) no change to `cortex-core`'s rule or constants, no record change, no format bump, no new
    crate, no allocation on the fan-out path, no syscall, and the per-slot choice adds no branch
    that a `#[cfg]` could make differ between platforms;
  - (c) the addressed set is written before the run and is a function of the trial's outcome
    only — the readout the engine selected through `cortex-basal-ganglia`'s gate. Whether the
    presynaptic side narrows it further (only the presented stimulus's units, rather than every
    unit that spiked) is the round's decision, argued in the ADR **before** any rewarded run and
    committed with the constants;
  - (d) every refusal `Task::check` already writes is kept, and one more for an addressed set
    that names a unit outside the arena, each with a test, plus a lattice property over
    `testkit/prop.rs` that the addressed modulation of a unit is the baseline when the unit is
    not addressed and `clamp(baseline + dopamine, 0, 1)` when it is.
- [x] **The constants commit**
  **Done:** `3733c08` on `main` (`c5b125c` on the branch before the rebase), before `8ad02e3` (`d3c5d8d`), the first commit that holds an addressed run's outcome; ADR-0069 cites both. Preceding the first commit that holds a rewarded outcome: the
  addressed set's definition, the instrument's constants restated unchanged, the seeds, and the
  criterion's counts. The measurement ADR cites both commits.
- [x] **The measurement ADR (the number after it)**
  **Done as [ADR-0069](../../docs/adr/0069-the-addressed-reward-measured.md).** At 1 024 units the rewarded clause fails with the global form reproduced (56 and 65 against 80; the addressed shuffled 53, the fixed modulation 60, the global form 49; the same run on one worker and on four); at 256 units the calibration's measure is 55 of 64 in the first block and the size is not measured under the rule. The fixed modulation under the addressed delivery is ADR-0066's run bit for bit and is held to ADR-0066's table; the global form is ADR-0066's rewarded run, rerun by its own weekly test under this round's code. (`depends-on:` the addressing ADR; ADR-0066
  named). In `runtime/cortex-runtime/tests/instrument.rs` or a sibling, as weekly `exhaustive`
  tests, **at 1 024 units**: the addressed rewarded run, the addressed rewarded run with the
  assignment mirrored, the addressed shuffled reward, the fixed modulation, and **the global
  form re-run under this round's code**, which must reproduce ADR-0066's 49 and is the direct
  comparison the claim rests on; and the addressed rewarded run on one worker and on four. At
  **256 units**, one reading: the addressed rewarded run. Per block, ADR-0066's readings (the
  correct trials, the readouts' spikes by stimulus, the window before the injection, the
  calibration's measure as the weights move, the sums by polarity, the signal, the four
  couplings, the ties). **The criterion, written before the run, is ADR-0066's:**
  - **Addressed rewarded:** the correct trials over the last 128 are **at least 80**, in both
    assignments, at 1 024 units;
  - **Addressed shuffled, fixed modulation, and the global form:** **at most 76** at 1 024 units;
  - **Workers:** the addressed rewarded run's sequence of trials is identical on one worker and
    on four.

  **The rule for a size that stops seeing**, written before the run: a size whose calibration
  measure falls below **56 of 64** in any block before the criterion's window (blocks 7 and 8) is
  recorded as **not measured**, never as a pass or a fail — ADR-0066 says 256 units does this and
  1 024 does not, and this rule is what makes the 256 reading a reading. If the addressed form
  keeps 256 above the mark, that is itself a result and is stated as one.

  If the rewarded clause fails with the global form reproduced, the ADR says that addressing the
  reward does not turn this rule into this behaviour at this size, and names what a next round
  would change (a structural rule that adds a synapse, a task that asks for a sign rather than a
  difference, a background under which the pairings can rise) **without building it**.
- [x] **The gate.**
  **Done:** `the_first_block_of_the_addressed_rewarded_run_at_256_units`, held to `ADDRESSED_256[..1]`; the bit-for-bit test of (a) is a block-level oracle test that costs nothing and runs in the gate. One test at 256 units: the addressed rewarded run's first block, 64 trials of
  $2^{14}$ ticks, held to the first row of the weekly run's pinned table as ADR-0061's gate test
  is, so that no number is pinned twice. The bit-for-bit test of (a) runs in the gate only if it
  costs no more than one block; otherwise it is an `exhaustive` test and the gate keeps the first
  block alone. Nothing else added to the gate.
- [x] **The evidence.**
  **Done:** run `35517026903`, `scope=both`; the times, the counts and the survivors are in ADR-0069's evidence. `gh workflow run ci.yml --ref <branch> -f scope=both`
  ([ADR-0067](../../docs/adr/0067-the-weekly-dispatch-has-a-scope.md): this round adds tests, so the
  whole-tree sweep is in its class), green in every job, every pinned number reproduced on the
  hosted runner, no survivor; its run id, the six runtime shards' times, the runtime suite's time
  before and after the round, and the weekly exhaustive job's time against its 120-minute bound,
  all in the measurement ADR.
- [x] **The documents, in the same pull request.**
  **Done:** whitepaper 4.20.0 (§6.5 with five directives, §8.8, §9, §11.1's H-11 and H-12), README, `CLAUDE.md`, the reader's guide, the ADR index, `CHANGELOG.md`. Whitepaper §8.8's three-factor row, §11.1's
  **H-12** (and H-11's synaptic half, which waits on it), §6.5's "The loop as the runtime
  composes it", README's Implemented cell, `CLAUDE.md`'s opening paragraph, `docs/zh-TW`'s
  reader's guide, the ADR index, whitepaper §9, and `CHANGELOG.md`. The whitepaper's version
  moves in **both** declarations with its date
  ([ADR-0064](../../docs/adr/0064-the-documentation-gate-and-the-version.md); `npm run spec:version`
  and the pull request's own check hold it). Executable directives under every sentence that
  claims a module, a test or a constant exists.
- [x] **The brief archived**
  **Done:** this file. As `briefs/README.md` says, every deliverable dispositioned, the
  frozen banner naming the pull request, the ADRs and the criterion's outcome.

## Not empowered

- No new learning rule beyond the delivery of the modulation: no gradient, surrogate gradient,
  e-prop, reward predictor, critic, eligibility rule of a different form, per-synapse learning
  rate, structural plasticity, sensory driver or symbol-to-pattern rule. Naming one in the
  measurement ADR is the whole of this round's licence about it.
- No change to `STDP_A_PLUS_Q1_15`, `STDP_A_MINUS_Q1_15`, `STDP_DEPRESSION_REFERENCE_Q1_15`, the
  windows, `ELIGIBILITY_TAU_SHIFT`, `DOPAMINE_TAU_SHIFT`, the inhibitory rule or its period, the
  estimator, the controller, the sleep constants, `Prior` or the reference prior's parameters.
- No change to the instrument: not the geometry or its masks, the rotations, the readout window,
  the gains the calibration picked, the trial, the block, the run, the baseline, the reward, the
  stimulus, or either seed. No re-calibration, and no new candidate gain.
- No constant chosen or moved after a rewarded run; no criterion changed after a run; no clause
  dropped because it failed.
- No record change, no new section, no format bump, no reserved byte taken; no new crate; no
  dependency in a state crate; no `unsafe` outside the runtime's arena access; no float anywhere.
- No more than one run added to the pull request's gate; no change to `.github/workflows/ci.yml`
  beyond nothing at all — the sweep's jobs, bounds, shards, completeness check and scope input
  are ADR-0058's and ADR-0067's; no renaming of a required check; no move of the determinism pin.
- No claim that a result at 256 or 1 024 units says what the loop does at Appendix A's scale, and
  no use of "learns" outside the task, the accuracy, the trials and the controls that produced it.

## Architectural empowerment

The executing session may replace any instruction above with a better decision, recorded in the
ADR that owns it: the shape by which a per-slot modulation reaches `phase_fan_out` (a slice, a
flag on the unit, a closure), so long as (a) and (b) of the first deliverable hold; whether the
addressed set is narrowed by the presynaptic side, so long as it is argued and committed before
any rewarded run; whether the bit-for-bit test is a gate test or an `exhaustive` one; whether the
256 reading runs at all, if the weekly exhaustive job would otherwise near its bound (and the ADR
says so); whether the round writes one ADR or two; and which numbers the ADRs restate. It may not
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
every job, reproduces every pinned number of the exhaustive runs and finds no survivor over the
tree. The constants commit precedes the first commit that holds a rewarded run's outcome. The
determinism pin, the image format and every number ADR-0066 pinned are unchanged.

## Report

The closing message states: the addressing as built (the shape, what is addressed, whether the
presynaptic side narrows it, and the test that holds the uniform case bit for bit); the constants
commit and the first outcome commit; the criterion clause by clause at 1 024 units beside
ADR-0066's numbers, with the global form's re-run as the comparison; the 256 reading and whether
the addressed form keeps the calibration's measure above the mark; whether a reward now changes
this behaviour, and if not, what the global form's reproduction lets the ADR say about the
delivery; the runtime suite's time before and after, the weekly exhaustive job's time against its
bound, and what the mutation gate and the sweep found; what was not done and why; and what the
re-examination after the round recommends next.
