---
status: proposed
date: 2026-09-18
---

# Brief 027 — A task, a readout and a learning curve: the reward path closed on a behaviour the engine reads back

> A self-contained prompt for one round of work. Read `briefs/README.md`, then this file, then
> the files it names. Frozen when executed; the outcome lives in the ADRs that accept it, the
> changelog and the code.

## Mission

Close the loop the tree has every piece of and has never run: a stimulus into the reference
network, a behaviour read out of the engine's own spike train, a reward as an input, and a
measurement that says whether the behaviour changed because of the reward or because of
something else. **One ADR** composes it in the runtime: a stimulus that is a function of the
tick and a seed (the `Drive`'s shape of [ADR-0044](../docs/adr/0044-reference-network.md)), a
readout that turns a trial's spike counts into an action through `cortex-basal-ganglia`'s
`compute_gating` — the crate that owns action selection
([ADR-0016](../docs/adr/0016-thirty-two-crate-architecture.md)), composed by nothing until now —
and a reward the caller passes to `Executor::reward` between trials, inside the eligibility
trace's window. Nothing is invented: the plasticity is
[ADR-0032](../docs/adr/0032-three-factor-plasticity.md)'s, the selection is
`cortex-basal-ganglia`'s, the composition is the runtime's
([ADR-0023](../docs/adr/0023-executor.md)). **One ADR** measures it under the criterion written
in this brief before the run, at 256 units on the pull request's gate and at 1 024 units in the
weekly job, against four controls that each remove one thing the change could otherwise be: a
shuffled reward, a fixed modulation, a mirrored assignment and a second worker count. When the
round is done the tree holds the first falsifiable answer to "does a reward change what this
engine does", the answer is in an ADR whether it is yes or no, the whitepaper says which of the
two, and this brief is archived with every check green. The image format stays 14 and no record
changes.

---

## Standing directives

- Every claim is Implemented, Specified, Target or Hypothesis. What a run holds on a network of
  256 or 1 024 units is stated as what it is, with the prior's parameters and the task's; what
  the same rules do at Appendix A's scale is a Target with the same generator
  ([ADR-0010](../docs/adr/0010-measured-or-target.md)). No timing figure enters a document from a
  developer machine. A rule is what it does: no "learns", no "understands", no "generalises"
  without the task, the accuracy, the block size and the controls it held against; "a trial" is
  the engine's own, stated in ticks and in simulated milliseconds, never a clock's.
- The repository wins over the document; a disagreement is a numbered finding in whitepaper §11,
  never a silent edit. An accepted ADR is history: what ADR-0044 to ADR-0057 pinned stays in
  them, with a note where this round moves a number.
- No `f32`/`f64`, in the crates and in the tests; an accuracy is a pair of counts or a Q16.16
  ratio in `i32`, widened to `i64` to multiply; every operation on a state field saturates or
  wraps by name (`clippy::arithmetic_side_effects` is denied everywhere,
  [ADR-0029](../docs/adr/0029-structural-enforcement.md)); a shift amount is bounded a line above
  the shift.
- Every loop ends by construction: a countdown, a range, a scan by `get`, a slice's iterator;
  never by a comparison alone that one operator flip turns into a walk without end.
- 64-byte `#[repr(C, align(64))]` records with compile-time assertions; no heap types, threads or
  `unsafe` in a state crate (`unsafe_code = "forbid"`, ADR-0029). A rule of
  `cortex-basal-ganglia` reads its own fields and nothing else; what writes those fields is the
  runtime's.
- Every quantity has one owner ([ADR-0016](../docs/adr/0016-thirty-two-crate-architecture.md)):
  the selection is `cortex-basal-ganglia`'s, the modulation `cortex-neuromod`'s, the trace and
  its consolidation `cortex-core`'s, the spike train and the composition the executor's. No new
  crate; the crate count stays 32.
- No record changes and the image format stays 14: no field, no section, no reserved byte taken.
  The harness's own state (the trial cursor, the counts, the seeds) is the caller's, as
  ADR-0043's affect state was the caller's until
  [ADR-0052](../docs/adr/0052-the-term-arena-in-the-image.md) moved it in; that a learning run is
  therefore not resumable from an image is stated as accepted debt with that precedent, not
  hidden.
- Rule L-3 and §1.5: no word, no string and no language name enters a crate.
- The determinism pin of [ADR-0030](../docs/adr/0030-verification-governance.md) does not move
  this round: nothing here changes a rule of `cortex-core`. If a run says otherwise, the round
  stops and says why rather than re-pinning. The mutation gate on the changed lines must pass; a
  new rule carries a test over the lattice of `testkit/prop.rs`; every pinned number an
  arithmetic oracle can produce is computed by that oracle before the test that asserts it is
  written; a number only the engine produces is pinned from one run and stated as the engine's.
- The decision rule for the measurement is written in this brief (Context item 9) and applied as
  written. What the numbers say beyond it is recorded as a reading, never folded into the rule
  after the fact. No constant of the task, the readout or any rule is tuned after the run it was
  measured in (brief 024's lesson,
  [ADR-0051](../docs/adr/0051-the-estimator-at-4096-units.md); brief 025's,
  [ADR-0054](../docs/adr/0054-the-causal-count-inside-the-loop.md)).
- The engine never amends its own code ([ADR-0031](../docs/adr/0031-policy-amendment.md)); a
  parameter that changes what a run does is in the image or in the trace (§8.3), never in a
  configuration alone. No registry entry is added: a plasticity parameter cannot pass the
  behaviour gate (F-37).
- A heavy exit test runs in the weekly job as an ignored test whose name contains `exhaustive`;
  the pull request's gate runs at most what it runs today plus this round's 256-unit tests, sized
  so that the gate's runtime tests stay within their present order.
- No product name enters a crate. Conventional Commits with a real body; never commit on `main`;
  the required checks keep their names.

## Context

Re-derived on 2026-09-18 against `main` at `16f09d3`. Line numbers move; the symbols and the
quoted sentences are what to re-derive.

1. **The reward path, and the one thing it cannot do.**
   `crates/cortex-neuromod/src/lib.rs`: `modulation(baseline_q16)` is
   `baseline.saturating_add(dopamine_rpe).clamp(0, Q16_ONE)`, with the doc sentence "There is no
   anti-Hebbian reversal: a dip below zero is clamped, as in Izhikevich 2007"; `reward(rpe)`
   saturating-adds into `dopamine_rpe`; `decay_dopamine(shift)` moves it toward zero by
   $2^{-\text{shift}}$ of itself and at least one LSB; `DOPAMINE_TAU_SHIFT` is 14, which the
   crate's own test states as "164 ms at 10 µs". `Executor::reward` (`executor.rs`) is "an input,
   like an injection, so a run that replays its rewards at the same ticks is the same run", and
   `Executor::tick` decays the signal once per tick. **The consequence the round must design
   around:** a punishment cannot reverse a pairing, only withhold it, so the reward's whole
   effect is the *fraction* of each pending trace that a presynaptic spike consolidates.
2. **Why the modulation baseline is the experiment's main knob.**
   `Config::modulation_baseline_q16` defaults to `MODULATION_ONE_Q16` (1.0), and at 1.0 every
   pairing consolidates in full while a positive reward adds nothing, because the clamp is
   already at the ceiling; the field's doc says "a lower baseline leaves the pairings pending for
   a reward to consolidate". A reward-gated run therefore sets it strictly between 0 and 1, and
   the value is stated before the run, not chosen after it.
3. **The trace and its window.** `crates/cortex-core/src/dynamics/synapse.rs`:
   `step_stdp(slot, pre_now_tick, post_last_tick, polarity, istdp_alpha_q1_15)` puts the
   pairing's amount, sign and all, into `eligibility_q1_15`; `decay_eligibility(elapsed_ticks)`
   decays it by `ELIGIBILITY_TAU_SHIFT` = 16, which at `TICK_NS` = 10 000
   (`dispatch/wheel.rs`) is $2^{16}$ ticks ≈ **655 ms**;
   `consolidate(slot, modulation_q16, polarity)` moves
   $\operatorname{round}(\lvert e \rvert \cdot m)$ with the trace's sign into the magnitude inside
   the polarity's half of the width and takes what the magnitude absorbed out of the trace. The
   dopamine window (164 ms) is four times shorter than the trace's, so a reward delivered at the
   end of a trial shorter than the trace's window still finds that trial's pairings pending: the
   trial's length is bounded by this and by nothing else.
4. **The drift a result must be told apart from.**
   [ADR-0055](../docs/adr/0055-a-weight-that-settles.md): the excitatory depression scales with
   the weight's magnitude, and over eighty windows at 1 024 units under the controller the
   excitatory sum settles at 0.45 of the prior's, every one of the last sixteen windows within
   0.75 per cent of the sixty-fourth. That force acts on every synapse whose two units pair,
   whatever the stimulus was, which is exactly what a shuffled-reward control removes and a bare
   accuracy number does not.
5. **The network, the drive and the input trace.** `runtime/cortex-runtime/src/synthesis.rs`:
   `Drive { every, messages, efficacy_q16, units, seed }` with `is_due(tick)`,
   `unit_at(tick, index)` and `step(inject, tick)`; "A function of the tick alone, so it needs no
   state and two forks under it are driven alike", which is what makes it an input trace as a
   rule (§8.3). `synthesize(units, blocks, prior)` writes `cortex-connectome`'s seeded prior into
   the arenas. `tests/reference.rs` runs that prior at 256 and 1 024 units: every fifth unit
   inhibitory at the rail (`inhibitory_gain_q4_4` 255), 32 synapses per unit, a window of eight,
   a quarter rewired, a local delay band for window synapses and a far one for rewired ones.
6. **The readout's owner, composed by nothing.**
   `crates/cortex-basal-ganglia/src/lib.rs`: `compute_gating(&mut self) -> bool` sets
   `gpi_snr_inhibition = striatal_d2_drive + stn_hyperdirect_drive − striatal_d1_drive`
   (saturating) and selects when it is strictly below zero; its own test pins that a net output
   of exactly zero is **not** a selection and one LSB below it is.
   `grep -rn compute_gating runtime/` finds nothing and `runtime/cortex-runtime/Cargo.toml` does
   not name `cortex-basal-ganglia`: whitepaper §1.6's Logic column names the rule, and no runtime
   module has ever called it. The round therefore gains one path dependency on a workspace crate,
   as [ADR-0043](../docs/adr/0043-discovery-path.md) gained three ("the runtime gains path
   dependencies on `cortex-affect`, `cortex-tools` and `cortex-knowledge`"); `npm run spec:deps`
   allows it for the runtime and for no state crate. Whitepaper §5.2.5 says lateral inhibition
   and the dopamine-scaled D1/D2 balance are Specified; this round composes the linear gate and
   leaves both Specified.
7. **What the executor already gives a harness.** `Executor::{inject, activate}` and
   `Inject::push` (the injector ring, drained by worker 0 in the delivery phase);
   `cortex_core::spike_message(efficacy_q16, apical)`; `train()` → `&[(tick, unit)]`, "the last
   `Config::train_capacity` spikes … in tick order and unit order within a tick, whichever worker
   ran the unit, so that the train is the same on every worker count"
   ([ADR-0050](../docs/adr/0050-the-train-inside-the-executor.md)), with `train_overwritten()`
   counting what the ring let go; `reward(rpe)`; `units()`, `blocks()`, `homeostasis()`;
   `Config::{control_step_q0_16, sleep_shift}` at 0 meaning "the gain stays at 1.0 and the
   dynamics are the reference ones" and "the engine never sleeps". A trial whose spikes exceed
   `train_capacity` is silently mis-read, which is a refusal the round must write, not a note.
8. **What this round is not, and what waits on it.** Whitepaper §11.1's **H-11** and
   `runtime/cortex-runtime/tests/discovery.rs`: "the synaptic half (that the trace consolidated
   biases a later behaviour toward the invention) needs a rule that maps an id to a pattern of
   units, which the tree does not have". This round does **not** write that rule. It builds what
   must work before that rule could be tested at all: a reward that measurably changes a
   behaviour the engine reads back from its own train. If the answer is no, H-11's synaptic half
   is not testable by the path it names, and the ADR says so.
9. **The decision rule, written before the run.** A *trial* is $2^{12}$ ticks (41.0 ms simulated,
   one estimator bin). A *block* is 64 trials. *Accuracy* over a block is the trials whose
   selected action equals the rewarded one, over 64; a tie (no selection, by item 6's rule)
   counts as an error, so chance on a two-action task is 0.5. The round says **the engine learned
   the task** when all four hold:
   - **The rewarded run:** accuracy over the last block minus accuracy over the first block is at
     least **0.15**, at 256 units on the gate and at 1 024 units in the weekly job.
   - **Shuffled reward:** the same stimuli and the same total dopamine, its sign drawn from the
     trial index by `mix64` instead of from the answer — the difference is **below 0.05**.
   - **Fixed modulation:** the baseline at 1.0 and no `reward` call, the pair rule alone — the
     difference is **below 0.05**.
   - **Mirrored assignment:** the two stimulus-to-action assignments swapped — the rewarded run's
     clause holds for both, or the anatomy won and not the rule.

   And, separately, the accuracy sequence is identical on one worker and on four (ADR-0023's
   property, which this round exercises and does not re-prove). **If the rewarded run's clause
   fails**, the round records the curve and the controls as a reading, names what the reading
   constrains, changes no constant of the task or of any rule, and the ADR says the loop as
   composed does not move this behaviour at these sizes — the outcome of
   [ADR-0054](../docs/adr/0054-the-causal-count-inside-the-loop.md)'s class, which is a result.
10. **What the gate can afford.** A window is 32 bins of $2^{12}$ ticks = 131 072 ticks = 1.31 s
    simulated; ADR-0044 records that "a window on a thousand units is 22 s in the debug profile
    on a developer machine", which is why the gate runs 256-unit forms and the weekly job runs
    the 1 024-unit ones. 512 trials at $2^{12}$ ticks is 2 097 152 ticks, sixteen windows: the
    size of the 256-unit day the gate already carries.
11. **The literature, under §2.1.** Izhikevich 2007 (the eligibility trace times a global
    dopamine signal solves the distal reward problem); Legenstein, Pecevski and Maass 2008 (the
    conditions under which reward-modulated STDP ascends the expected reward, and where it does
    not); Frémaux and Gerstner 2016 (three-factor rules and the baseline's role), already
    Appendix D reference 5; Gurney, Prescott and Redgrave 2001, already reference 6. All are old,
    third-party reproduced and carry their failure modes, the relevant one being that a **global
    scalar cannot separate two synapses whose traces are alike**: the task must be one whose
    traces differ, which is why the stimuli are disjoint unit sets and one trial holds one
    stimulus.

## Deliverables

- [ ] **The task ADR (the next free number; `ls docs/adr`)** (`depends-on: ADR-0044`; ADR-0023,
  ADR-0032, ADR-0050 and ADR-0016 named). A new runtime module
  `runtime/cortex-runtime/src/task.rs`, exported from `lib.rs`, with `cortex-basal-ganglia` added
  to the runtime's path dependencies (Context item 6), owning nothing but the composition:
  - `Stimulus`: a contiguous set of units and the messages into them, drawn as `Drive` draws, a
    function of the tick and the seed, so the whole run is an input trace.
  - `Readout`: two disjoint unit sets, neither overlapping any stimulus set; the trial's spikes
    are counted per set from `Executor::train()`, written into two `BasalGangliaChannelState`s
    (each channel's own count into `striatal_d1_drive`, the other's into `striatal_d2_drive`,
    `stn_hyperdirect_drive` at zero), and `compute_gating` decides; two selections or none is no
    selection.
  - `Task::trial(executor, trial_index) -> Result<Outcome, TaskError>`: injects the trial's
    stimulus, runs its ticks, reads the train once, selects, calls `Executor::reward` with the
    round's constant times the outcome's sign, and returns the stimulus, the selection and
    whether it was correct.
  - Refusals, each with a named variant and a test: a set beyond the arena; sets that overlap; a
    zero-length set; a trial of zero ticks; a `train_capacity` below the most spikes a trial can
    hold, so that a mis-read is refused rather than counted (Context item 7); a modulation
    baseline at the ceiling with a reward magnitude above zero, which is the configuration that
    silently does nothing (Context item 2).
  - Tests in the module: each refusal; a trial with all spikes in one set selecting that set; the
    tie; the counts equal to a hand-built train; the reward's sign and magnitude at the modulator
    after one correct and one incorrect trial, against an oracle of `modulation`; a trial that
    reads the same counts on one and four workers.
  - A lattice property over `testkit/prop.rs`: over seeded trains and seeded set pairs, the
    selection equals the sign of the count difference, and is no selection exactly when the
    counts are equal.
- [ ] **The measurement ADR (the number after it)** (`depends-on:` the task ADR; ADR-0036,
  ADR-0049, ADR-0053 and ADR-0055 named). `runtime/cortex-runtime/tests/learning.rs`: the harness
  that runs the rewarded run and the four controls of Context item 9 on the ADR-0044 prior with
  `control_step_q0_16` at 0, `sleep_shift` at 0, the modulation baseline stated before the run
  and the reward magnitude stated before the run; 512 trials at 256 units on the gate, 2 048
  trials at 1 024 units as an `#[ignore]`d test whose name contains `exhaustive`; per block the
  accuracy, the weight sums by polarity, the spikes per set and the modulator's signal, so that a
  curve that does not move can be told from a network that fell silent. The ADR carries: the
  criterion as written above and its outcome, clause by clause; the per-block table at both sizes
  for the rewarded run and each control; the weight sums beside ADR-0055's settling figure; what
  the mutation gate found; and, if the clause failed, what the reading constrains and what a next
  round would have to change (a per-unit learning signal, a structural rule, a different task),
  named without being built.
- [ ] **The documents, in the same pull request.** Whitepaper: §1.6's `cortex-basal-ganglia` row
  and the runtime's row say the rule is composed; §5.2.5 says the linear gate is composed and
  lateral inhibition and the D1/D2 balance stay Specified; §8.8's three-factor row gains what the
  measurement read, with the task named; §6 gains a scenario for the loop, or an existing one
  gains it, whichever the round argues for; §11.1 gains a numbered item (or H-11's disposition
  moves) saying what is now known about a reward changing a behaviour, with the sizes and the
  criterion. README's Implemented cell, `CLAUDE.md`'s opening paragraph, `docs/zh-TW`'s reader's
  guide, `docs/adr/README.md`'s index and `CHANGELOG.md` all say it. Executable directives under
  every sentence that claims the module or the test exists.
- [ ] **The brief archived** as `briefs/README.md` says, with every deliverable dispositioned and
  the frozen banner naming the pull request, the ADRs and the criterion's outcome.

## Not empowered

- No new plasticity rule, no per-unit learning signal, no surrogate gradient, no e-prop: this
  round composes what exists. A rule of that kind is its own ADR in a later round, and naming it
  in the measurement ADR's "what would have to change" is the whole of this round's licence about
  it.
- No record change, no new section, no format bump, no reserved byte taken; `[52..54)` of the
  unit stays reserved and zero; no new crate; no dependency in a state crate; no `unsafe` outside
  the runtime's arena access.
- No float anywhere, including the tests and the oracles.
- No change to `STDP_A_PLUS_Q1_15`, `STDP_A_MINUS_Q1_15`, `STDP_DEPRESSION_REFERENCE_Q1_15`, the
  windows, `ELIGIBILITY_TAU_SHIFT`, `DOPAMINE_TAU_SHIFT`, the inhibitory rule or its period, the
  estimator, the controller, the sleep constants, the ripple, the replay drive or `Prior`.
- No reward predictor: the reward-prediction error this round passes is the outcome's sign times
  a constant, and `cortex-predictive` stays composed by nothing.
- No registry entry and no gate class (F-37; ADR-0057's restatement).
- No tuning after the run: not the baseline, not the reward's magnitude, not the trial's length,
  not the sets, not the criterion.
- No structural plasticity, no synaptogenesis, no pruning, no sensory driver, no symbol-to-pattern
  rule (H-11's synaptic half).
- No test in the pull request's gate beyond this round's 256-unit forms; the 1 024-unit runs are
  the weekly job's; nothing at 4 096 units.
- No claim that a result at 256 or 1 024 units says what the same loop does at Appendix A's
  scale, and no use of "learns" about the engine outside the task, the accuracy and the controls
  that produced it.
- No renaming of the CI jobs the ruleset requires; no move of the determinism pin.

## Architectural empowerment

The executing session may replace any instruction above with a better decision, recorded in the
ADR that owns it: the task's arity (a two-action task is written here; three or four is
admissible if chance is restated and every control is run at the same arity); the trial's length,
so long as it stays inside the eligibility window of Context item 3 and the block and gate sizes
of item 10 are restated; how a trial's spikes reach the channels (counts, rates, or a count over
a sub-window), so long as the selection stays `compute_gating`'s and the runtime writes only the
drives; the stimulus's shape (a `Drive` with two seeds, or a struct of its own), so long as it is
a function of the tick and a seed; where the harness's cursor and counts live, so long as the
image stays format 14 — if the round finds the loop cannot be measured without state in the
image, it says so first, in the ADR, and the format moves once for the round with §5.2's table,
every fixed offset of the image tests and a changelog entry; the modulation baseline and the
reward magnitude, so long as both are written down before the run and neither is touched after
it; whether the four controls run as four tests or one parameterised harness; and which numbers
the ADRs restate. It may not reach the standing directives, the whitepaper's invariants or the
constraints in `CLAUDE.md`.

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
```

Every command exits 0; the last one reports no survivor in CI (a local Windows run marks mutants
whose build failed as unviable and is not the gate's truth). `npm run spec:deps` still reports no
dependency in a state crate. The format stays 14 and the determinism pin does not move. The
1 024-unit runs pass in the weekly job's form before the pull request merges, and their numbers
are the ADR's.

## Report

The closing message states: the loop as it is composed, module by module, and which crate owns
each part; the modulation baseline, the reward magnitude, the trial's length and the sets, as
they were written before the run; the criterion clause by clause with its outcome at both sizes,
the per-block accuracy of the rewarded run and of each of the four controls, and the weight sums
beside ADR-0055's settling figure; whether the round's answer is that a reward changes this
behaviour, that it does not, or that the run could not tell, and what that constrains; what the
mutation gate found on the changed lines and how each survivor was answered; what was not done (a
per-unit learning signal, structural plasticity, a sensory driver, a reward predictor, H-11's
synaptic half, anything at 4 096 units, resumability of a learning run from an image) and why;
and what the re-examination after the round recommends next.
