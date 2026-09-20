---
status: archived
date: 2026-09-20
---

> **Executed 2026-09-20 in pull request #73.** Writes ADR-0065 (the instrument: a set as a
> periodic pattern of units, a readout window derived from the prior's delay band, a trial of one
> dopamine time constant, the geometry checked against the census at both sizes, and the
> calibration with the weights frozen: at 256 units 1.75 fails at 50 of 64 and 2.0 passes at 58,
> at 1 024 units 1.75 passes first at 62) and ADR-0066 (the measurement through it: the rewarded
> run reads 53 and 49 correct of the last 128 against the 80 the criterion asks for, the mirrored
> assignment 53 and 65, the controls 51, 51, 55 and 60, the run the same on one worker and on
> four; the rewarded clause fails at both sizes with the calibration passed, and no constant
> moves). No finding; H-12 and H-11's synaptic half carry the outcome; image format 14 unchanged;
> the determinism pin untouched. Every deliverable is done; notes under the boxes say where the
> tree departs from the text: the set shape is a periodic pattern rather than a stride, since a
> stride alone cannot name a readout that surrounds every stimulus unit on both sides, and the
> rotation the census picks is part of the geometry's rule. The report is in the pull request
> and in `CHANGELOG.md`. The body below describes the tree before execution and is not
> maintained; its relative links gained one `../`.

# Brief 029 — The instrument recalibrated: the same rule and the same controls, a stimulus that fires once, a readout that looks where it lands, one reward per trial, and a check that the readout can see before any reward is given

> A self-contained prompt for one round of work. Read `briefs/README.md`, then this file, then
> the files it names. Frozen when executed; the outcome lives in the ADRs that accept it, the
> changelog and the code.

## Mission

[ADR-0060](../../docs/adr/0060-the-reward-path-measured.md) answered brief 027's question with a
no, and located the no in the instrument rather than in the rule: the readout counted a whole
trial of background, the stimulus reverberated through its own quarter so that its synapses
paired as depression under every feedback, and five trials shared one dopamine window. It named
four changes and built none. This round builds them, **changes nothing else**, and asks the same
question again — whitepaper §11.1's **H-12**, whose own entry lists these four changes as the
next round's protocol: does a reward change a behaviour the engine reads back from its own spike
train?
The rule is [ADR-0032](../../docs/adr/0032-three-factor-plasticity.md)'s, untouched; the controls
are brief 027's four; the variable under test is still the reward. What is new is a step brief
027 did not have: **before any reward is given**, the readout is shown to see the stimulus at
all, by a calibration whose measure cannot see the answer (the weights frozen, no reward), whose
candidates and pass mark are written here, and whose outcome picks the gain and nothing else.
The chosen constants are committed before the first rewarded run, so the order is in the
history rather than in a sentence. **One ADR** writes the instrument (a stimulus set spaced
beyond the prior's window, a readout window derived from the prior's delays, a trial of one
dopamine time constant) and its calibration; **one ADR** measures the four controls at 256 and
1 024 units under the criterion written below, whose false-positive and false-failure rates are
computed rather than assumed. When the round is done, the answer is in an ADR whichever it is,
and it means more than ADR-0060's did: if the calibration passed, a no is a statement about the
rule on this task, not about the readout. The image format stays 14 and no record changes.

---

## Standing directives

- **Latest ≠ Newest** (whitepaper §2.1, `CLAUDE.md` principle 4): the rule stays the
  three-factor rule of ADR-0032 (Izhikevich 2007; Frémaux and Gerstner 2016), the selection
  `cortex-basal-ganglia`'s linear gate (Gurney, Prescott and Redgrave 2001), and the four changes
  are experimental design, not new mechanisms: a count in a window after the stimulus, a sparse
  stimulus, a trial matched to the reinforcement signal's time constant, a quieter background.
  No newer learning rule (e-prop, a surrogate gradient, a per-unit learning signal, a 2025–2026
  method), no dependency (no statistics crate: the criterion is integer counts), no new tool and
  no version bump enters this round, however recent its results.
- Every claim is Implemented, Specified, Target or Hypothesis. What a run holds on 256 or 1 024
  units is stated as what it is, with the prior's parameters and the task's; what the same loop
  does at Appendix A's scale is a Target with the same generator
  ([ADR-0010](../../docs/adr/0010-measured-or-target.md)). No timing figure enters a document from a
  developer machine. No "learns", "understands" or "generalises" without the task, the accuracy,
  the trials it was counted over and the controls it held against.
- The repository wins over the document; a disagreement is a numbered finding in whitepaper §11.
  An accepted ADR is history: ADR-0059's and ADR-0060's numbers stay in them, and brief 027's
  tests keep pinning them; this round is compared with them, never folded into them.
- No `f32`/`f64`, in the crates, the tests and the oracles; an accuracy is a count of trials; every
  operation on a state field saturates or wraps by name (`clippy::arithmetic_side_effects` is
  denied everywhere, [ADR-0029](../../docs/adr/0029-structural-enforcement.md)).
- Every loop ends by construction: a countdown, a range, a scan by `get`, a slice's iterator; never
  by a comparison alone that one operator flip turns into a walk without end.
- Every quantity has one owner ([ADR-0016](../../docs/adr/0016-thirty-two-crate-architecture.md)): the
  selection is `cortex-basal-ganglia`'s, the modulation `cortex-neuromod`'s, the trace
  `cortex-core`'s, the task and its sets the runtime's (`runtime/cortex-runtime/src/task.rs`). No
  new crate; the crate count stays 32; no record changes; the image format stays 14.
- **Nothing is chosen after a rewarded run.** A constant is written in this brief, derived by a
  rule from the prior's parameters, or picked by the calibration below from the candidates listed
  here by the measure written here; all of them are committed before the first rewarded run. What
  a rewarded run says beyond the criterion is a reading, never a reason to move a constant (brief
  024's lesson, [ADR-0051](../../docs/adr/0051-the-estimator-at-4096-units.md); brief 025's,
  [ADR-0054](../../docs/adr/0054-the-causal-count-inside-the-loop.md); brief 027's,
  [ADR-0060](../../docs/adr/0060-the-reward-path-measured.md)).
- The determinism pin of [ADR-0030](../../docs/adr/0030-verification-governance.md) does not move:
  nothing here changes a rule of `cortex-core`. The mutation gate on the changed lines must pass; a
  new rule of the task module carries a test over the lattice of `testkit/prop.rs`; every number an
  arithmetic oracle can produce is computed by that oracle before the test that asserts it; a
  number only the engine produces is pinned from one run and stated as the engine's.
- A heavy run is an `#[ignore]`d test whose name contains `exhaustive` and runs in the weekly job;
  the pull request's gate grows by at most one run (Context item 8), because the runtime's gate
  suite is what [ADR-0058](../../docs/adr/0058-the-weekly-sweep-and-its-timeouts.md)'s mutation sweep
  times and multiplies.
- The engine never amends its own code ([ADR-0031](../../docs/adr/0031-policy-amendment.md)); no
  registry entry (F-37). Conventional Commits with a real body; never commit on `main`; the
  required checks keep their names; no product name enters a crate.

## Context

Re-derived on 2026-09-20 against `main` at `03bcad4`. Line numbers move; the symbols and the quoted
sentences are what to re-derive.

1. **What ADR-0060 measured and where it put the failure.** At 256 units (512 trials) and 1 024
   (2 048), every run of every control was at chance in every block (22 to 40 of 64); the rewarded
   run's last block minus its first was −3 and −7 against the ten brief 027 asked for. Its readings,
   quoted: "A readout quarter holds about twenty spikes per trial at 256 units and seventy at
   1 024 with no stimulus in it"; a stimulus adds "three to six to each readout quarter in the first
   block … and it adds the same to both"; "a stimulus quarter fires 3.4 spikes per unit per
   presentation, so each of its synapses onto a readout unit carries three presynaptic spikes per
   presentation, and the readout unit follows within the window on few of them; the pair rule reads
   the rest as depression"; and the dopamine signal "is the last four or five outcomes … a walk that
   decays by 0.78 per trial". It named, not built: (1) the readout's window, (2) a stimulus that
   fires its set once, (3) a trial at or beyond the dopamine window, (4) the gain, and after those
   a per-unit learning signal, a structural rule or a different task. It also records that the
   three-factor consolidation itself is held by the two-unit tests of ADR-0032 and ADR-0043: the
   failure was the loop's, not the rule's.
2. **The task module as it is.** `runtime/cortex-runtime/src/task.rs` (ADR-0059): `Set { first, len }`
   is **contiguous** (`first..first + len`) and is what `Stimulus` and `Readout` take;
   `Readout::count(train, start, ticks)` already counts a sub-window of the train; `Task { stimuli,
   readout, drive, ticks, seed, reward_q16, mirrored, feedback }` with `Feedback::{Answer,
   Shuffled, Withheld}`; `Task::check` refuses with `TaskError::{NoStimulus, EmptySet,
   SetOutsideArena, SetsOverlap, NoTicks, TrainTooSmall, CountBeyondWidth, NegativeReward,
   NoReward, RewardAtCeiling, Inject}`; `MIN_INTERVAL_TICKS` and `spikes_per_unit(ticks)` bound a
   trial's spikes. The module's doc: the stimulus's units "fire together about twelve ticks on".
   A spaced stimulus therefore needs a set that is not contiguous: a change to a runtime type,
   not to a record.
3. **The harness as it is.** `runtime/cortex-runtime/tests/learning.rs`: the block "written before
   the run" (`TRIAL_TICKS = 1 << 12`, `BLOCK = 64`, `BASELINE_Q16 = 0x8000`, `REWARD_Q16` = 1.0,
   two stimulus messages of 1.25 (`STIMULUS_Q16 = 0x0001_4000`), `GAIN_Q16 = 0x0002_0000`,
   `SEED = 27`); `quarters(units)` lays the ring out as stimulus A, readout 0, stimulus B,
   readout 1, "so that each readout touches both stimuli on the ring at the same window";
   `at_gain` holds the gain by patching the image's homeostasis record, with
   `control_step_q0_16` and `sleep_shift` at 0; every full run, at 256 units and at 1 024, is an `#[ignore]`d weekly test
   (`the_rewarded_run_at_256_units_on_four_workers_exhaustive`, …,
   `the_criterion_at_1024_units_as_written_exhaustive`), and the gate runs
   `the_first_block_of_the_rewarded_run_at_256_units` — sixty-four trials held to the first row of
   the full run's pinned table — and the criterion over the tables
   ([ADR-0061](../../docs/adr/0061-the-learning-runs-leave-the-gate.md)).
4. **The prior's geometry and delays.** `crates/cortex-connectome/src/prior.rs`: "a local synapse
   reaches a unit within this many places on the ring" — the window is eight places on each side
   (`place = between(1, 2 × window)`); a quarter of the synapses are rewired and may land anywhere.
   The harness's prior (ADR-0044 at seed 22): 32 synapses per unit, local delays of 1 to 3 ms and
   far ones of 14 to 25.6 ms, which at `TICK_NS` = 10 000 are 100 to 300 ticks and 1 400 to 2 560.
   A stimulus unit's spike therefore reaches a readout unit inside its window within about
   300 ticks of the volley, and a rewired one within about 2 600; a readout window derived from
   these bands and the volley's tick is a rule of the prior, not of a run.
5. **The weights freeze at a modulation of zero.** `runtime/cortex-runtime/src/executor.rs`,
   phase 2: every block, of either polarity, runs `step_stdp_all` and then
   `consolidate_all(modulation, polarity)`; `cortex-neuromod`'s `modulation(baseline)` is
   `clamp(baseline + dopamine, 0, 1)`. With the baseline at zero and no reward, no weight of either
   polarity moves: the traces fill and decay and are never written. That is what lets a
   calibration read the prior's own response to a stimulus without teaching it anything.
6. **The gain.** [ADR-0044](../../docs/adr/0044-reference-network.md)'s table, spikes per window:
   at 256 units 555 and 489 at a gain of 1.75 against 3 156 and 2 563 at 2.0; at 1 024 units
   2 406 and 2 297 against 12 757 and 11 328 — five to six times quieter at 1.75. A quieter
   background leaves more room for a stimulus's spikes and also carries a stimulus less far;
   which of the two wins is a measurement, which is why the gain is the calibration's one choice.
7. **One reward per trial.** `DOPAMINE_TAU_SHIFT` = 14 (164 ms), `ELIGIBILITY_TAU_SHIFT` = 16
   (655 ms), a tick 10 µs. At a trial of $2^{12}$ ticks the signal left by one outcome at the next
   trial's end is $e^{-1/4} ≈ 0.78$ of itself, which is ADR-0060's walk; at $2^{14}$ ticks it is
   $e^{-1} ≈ 0.37$, and the trial is still a quarter of the trace's window. A trial of $2^{14}$
   ticks is the shortest that puts one time constant of the signal between two rewards.
8. **The budget.** ADR-0058's sweep bounds every runtime mutant at three times the runtime's own
   suite plus thirty seconds, so a run in the pull request's gate is paid again by each of the
   runtime's 1 060 mutants every week. Brief 027 put five runs of $2^{21}$ ticks at 256 units in
   the gate and the runtime suite went from 216–464 s to 709–854 s on the hosted runners, the six
   runtime shards from under two hours to 3 h 27 m – 4 h 19 m (run `35444829805`);
   [ADR-0061](../../docs/adr/0061-the-learning-runs-leave-the-gate.md) moved those runs to the weekly
   job, and the suite is back at 230–437 s and the shards at 56 m – 2 h 00 m (run `35459078284`).
   This round keeps it there. A run of 512 trials of $2^{14}$ ticks is $2^{23}$ ticks, the size of
   brief 027's 1 024-unit weekly runs: the criterion's runs belong in the weekly job at both sizes,
   and the gate gains one test, the rewarded run's first block (64 trials, $2^{20}$ ticks at 256
   units) held to the first row of the weekly run's pinned table, as ADR-0061's gate test is. The
   weekly exhaustive job has a 120-minute bound and took 33 and 56 minutes in the two runs above,
   the difference mostly the runner (the unchanged `reference.rs` alone moved from 1 238 s to
   2 118 s); ten more runs of $2^{23}$ ticks go into it, so its time after this round is measured
   and stated, and if it nears the bound the round says so rather than raising it.
9. **The criterion's statistics, computed.** At chance a block of 64 has a standard deviation of
   four trials. Brief 027's clauses, by exact enumeration: its rewarded clause (a rise of ten
   between two blocks) passes by noise with probability 0.046, and a control's (a rise of at most
   three) fails by noise with probability 0.27 — ADR-0060's "one in twenty-five" and "one in four".
   Counting the last 128 trials instead, with $X \sim \mathrm{Bin}(128, 1/2)$:
   $P(X \ge 80) = 0.00296$, so a rewarded clause passes by noise about once in 340 and both
   assignments at both sizes together about once in $10^{10}$; $P(X \ge 77) = 0.0134$, so a
   control exceeds a ceiling of 76 by noise about once in 75, and one of four does so about once
   in 19. The executing round recomputes these with an integer oracle before the tests that use
   them.
10. **Why four changes at once does not break the principle brief 027 kept.** The variable under
    test is the reward, and the controls isolate it exactly as brief 027's did; the rule is held.
    The four changes are the instrument, recalibrated as one unit by a measure that cannot see the
    answer, and frozen before the answer is asked for. The round claims nothing about which of the
    four mattered; if the answer is yes, a later round may take them away one at a time.
11. **What this round answers, and what waits on it.** Whitepaper §11.1's **H-12**: "That a reward
    through `Executor::reward`, consolidating the eligibility traces pending at its moment, changes
    which of two readouts the engine selects for a stimulus", measured at 256 and 1 024 units by
    ADR-0060 and closing with "The protocol for the next round, named in ADR-0060 and not built" —
    the four changes of this brief. **H-11**'s synaptic half (a rewarded invention biasing a later
    behaviour) needs a reward that changes a behaviour at all, so it waits on H-12's answer here.
    §6.5's Scenario R-5 carries the loop ("The loop as the runtime composes it").

## Deliverables

- [x] **The instrument ADR (the next free number; `ls docs/adr`)**
  **Done as [ADR-0065](../../docs/adr/0065-the-instrument-recalibrated.md).** The set is a periodic pattern (`first`, `period`, `mask`, `count`), not a stride: a stride names a stimulus of spaced units but not a readout that surrounds every stimulus unit on both sides, which (b) needs; the geometry is periods of twenty from a rotation the census picks, and (a) to (d) are held by `the_geometry_holds_against_the_census_at_both_sizes` in the gate. One refusal for the shape (`MalformedSet`) and two for the window (`EmptyWindow`, `WindowOutsideTrial`); the lattice property is `a_set_s_units_are_exactly_the_ones_it_names`. (`depends-on: ADR-0060`;
  ADR-0059, ADR-0044, ADR-0032 and ADR-0016 named). In `runtime/cortex-runtime/src/task.rs`, sets
  that need not be contiguous — a first unit, a count and a stride, or the shape the round argues
  for — with every refusal of ADR-0059 kept and one more for a stride or shape that would place a
  unit outside the arena or on another set's unit, each with a test, and a lattice property over
  `testkit/prop.rs` that a set's units are exactly the ones it names. The geometry, written as a
  rule of the prior's parameters and checked against the prior's census before any run, at both
  sizes:
  - (a) no unit of either stimulus lies within the prior's local window (a ring distance of eight
    or less) of another unit of either stimulus, so a presentation fires each once and neither
    stimulus drives the other through a local synapse;
  - (b) two readout sets of equal size, disjoint from every stimulus unit;
  - (c) the four excitatory couplings of the prior (A→R0, A→R1, B→R0, B→R1, as ADR-0060 defines
    them: the sum of the weights an excitatory unit of the one sends to a unit of the other) equal
    within ten per cent, and none zero;
  - (d) a readout window derived from the volley's tick and the delay bands of Context item 4, the
    same window for both readouts and every trial, never adjusted from a run.
- [x] **The calibration, in the same ADR, before any rewarded run.**
  **Done.** At 256 units 1.75 fails (50 of 64) and 2.0 passes (58); at 1 024 units 1.75 passes first (62). Both sizes have a rewarded run. At each size and for each
  candidate gain, **1.75 then 2.0**, one run of 64 trials of $2^{14}$ ticks with the stimuli in the
  task's order, the modulation baseline at **zero** and no reward (Context item 5: no weight
  moves). The measure, per trial: the two readouts' spikes in the readout window after the volley
  against their spikes in a window of the same length ending at the stimulus's injection (the
  first trial preceded by a lead-in of that length with no stimulus). **A
  gain passes** when the window after the volley holds more readout spikes than the window before
  it in at least **56 of the 64 trials**, and each readout's total after the volley exceeds its
  total before. The first candidate that passes is the gain at that size; the order is fixed so
  that no preference chooses. If neither passes at a size, the round runs no rewarded run at that
  size, records the calibration as a reading, and the ADR says what the instrument would need —
  a rewarded run through a readout that cannot see is ADR-0060 again.
- [x] **The constants commit.**
  **Done:** `4f2da6a`, before `1780b38`, the first commit that holds a rewarded run's outcome; ADR-0066 cites both. One commit holding the whole block "written before the run": the
  geometry, the readout window, the gain the calibration picked at each size, the trial
  ($2^{14}$ ticks), the block (64), the run (512 trials), the baseline (0.5) and the reward (1.0)
  as brief 027 had them, the seeds as brief 027 had them (the prior at 22, the trials at 27), and
  the criterion's counts below. It precedes the first commit that holds a rewarded run's outcome,
  and the measurement ADR cites both commits.
- [x] **The measurement ADR (the number after it)**
  **Done as [ADR-0066](../../docs/adr/0066-the-reward-path-measured-again.md)**, in `runtime/cortex-runtime/tests/instrument.rs`. The rewarded clause fails at both sizes (53 and 49 of 128), the mirrored assignment 53 and 65, the controls hold (51, 51, 55, 60), the worker clause holds; the readings carry the ties per block beside ADR-0060's, since a tie is an error and the chance level of a count of a few spikes is below one half. (`depends-on:` the instrument ADR; ADR-0060
  named). In `runtime/cortex-runtime/tests/learning.rs` or a sibling test file, at both sizes, as
  weekly `exhaustive` tests: the rewarded run, the rewarded run with the assignment mirrored, the
  shuffled reward, the fixed modulation (baseline 1.0, no reward: the pair rule alone), and the
  rewarded run on one worker and on four. Per block, ADR-0060's readings (the correct trials, the
  spikes per readout by stimulus, the sums by polarity, the signal, the four couplings) and the
  calibration's measure, so that the instrument's resolution is visible as the weights move. **The
  criterion, written before the run:**
  - **Rewarded:** the correct trials over the last 128 (blocks 7 and 8) are **at least 80**, in
    both assignments, at both sizes;
  - **Shuffled reward and fixed modulation:** the correct trials over the last 128 are **at most
    76**, at both sizes;
  - **Workers:** the rewarded run's sequence of trials is identical on one worker and on four, at
    both sizes.

  The criterion is applied at every size whose calibration passed; a size whose calibration failed
  has no rewarded run and is recorded as not measured, never as a pass or a fail. Brief 027's
  clause (the last block minus the first) and the per-block accuracy are reported as
  readings beside ADR-0060's, not as clauses. If the rewarded clause fails with the calibration
  passed, the ADR says the rule, on this task at these sizes, does not turn a reward into this
  behaviour, and names what a next round would change (a per-unit learning signal, a structural
  rule, a different task) without building it.
- [x] **The gate.**
  **Done:** `the_first_block_of_the_recalibrated_rewarded_run_at_256_units`, held to `REWARDED_256[..1]`; the geometry's census check, the picked gains and the criterion's oracle run nothing heavy. The suite's time before and after is in ADR-0066. One test at 256 units: the rewarded run's first block, 64 trials of $2^{14}$
  ticks, held to the first row of the weekly run's pinned table as ADR-0061's gate test is, so that
  no number is pinned twice; nothing else added to the gate. The runtime suite's time before
  and after the round, from the weekly sweep's timed step, stated in the measurement ADR.
- [x] **The evidence.**
  **Done:** the run ids and times are in ADR-0066. A weekly dispatch on the round's branch (`gh workflow run ci.yml --ref
  <branch>`) in which every pinned number of the exhaustive runs holds on the hosted runner and the
  mutation sweep is green; its run id, the six runtime shards' times and the weekly exhaustive
  job's time against its 120-minute bound in the measurement ADR.
- [x] **The documents, in the same pull request.**
  **Done:** whitepaper 4.17.0 (§5.2.5, §6.5 with eight directives, §8.8, §9, §11.1's H-11 and H-12), README, `CLAUDE.md`, the reader's guide, the ADR index, `CHANGELOG.md`. Whitepaper §8.8's three-factor row and §11.1's
  **H-12** (and H-11's synaptic half, which waits on it) say what is now known, with the sizes, the
  calibration and the criterion; §6.5's "The loop as the runtime composes it" gains the
  calibration and the spaced stimulus; README's
  Implemented cell, `CLAUDE.md`'s opening paragraph, `docs/zh-TW`'s reader's guide, the ADR index
  and `CHANGELOG.md`. Executable directives under every sentence that claims a module, a test or a
  constant exists.
- [x] **The brief archived**
  **Done:** this file. as `briefs/README.md` says, every deliverable dispositioned, the
  frozen banner naming the pull request, the ADRs, the calibration's outcome and the criterion's.

## Not empowered

- No new learning rule, no per-unit learning signal, no surrogate gradient, no e-prop, no reward
  predictor, no structural plasticity, no sensory driver, no symbol-to-pattern rule (H-11's
  synaptic half): naming them in the measurement ADR is the whole of this round's licence about
  them.
- No change to `STDP_A_PLUS_Q1_15`, `STDP_A_MINUS_Q1_15`, `STDP_DEPRESSION_REFERENCE_Q1_15`, the
  windows, `ELIGIBILITY_TAU_SHIFT`, `DOPAMINE_TAU_SHIFT`, the inhibitory rule or its period, the
  estimator, the controller, the sleep constants, `Prior` or the reference prior's parameters.
- No gain outside the two candidates, no other constant calibrated, no calibration measure that
  reads a selection, a reward or a correct trial, and no constant chosen or moved after a
  rewarded run.
- No change to brief 027's tests, to ADR-0059's or ADR-0060's numbers, or to the refusals
  ADR-0059 wrote; the new set shape is added beside them.
- No record change, no new section, no format bump, no reserved byte taken; no new crate; no
  dependency in a state crate; no `unsafe` outside the runtime's arena access; no float anywhere.
- No more than one run added to the pull request's gate; no change to `.github/workflows/ci.yml`'s
  sweep, its bounds, its shards or its completeness check (ADR-0058); no renaming of a required
  check; no move of the determinism pin.
- No claim that a result at 256 or 1 024 units says what the loop does at Appendix A's scale, and
  no use of "learns" outside the task, the accuracy, the trials and the controls that produced it.

## Architectural empowerment

The executing session may replace any instruction above with a better decision, recorded in the ADR
that owns it: the shape of a set that is not contiguous (a stride, an explicit list, a type of its
own) and the exact geometry, so long as (a) to (c) hold at both sizes and are checked against the
census before any run; the formula that derives the readout window from the prior's delay bands and
the volley's tick, so long as no run informs it; the calibration's measure, so long as it is written
in the ADR before the calibration runs, reads no selection, reward or outcome, and is at least as
strict as 56 of 64; whether the calibration runs are gate or weekly tests, within the gate's one
run; whether the round writes one ADR or two; and
which numbers the ADRs restate. It may not reach the standing directives, the whitepaper's
invariants or the constraints in `CLAUDE.md`.

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
gh workflow run ci.yml --ref <this round's branch>
```

Every command exits 0; the in-diff gate reports no survivor in CI. The dispatched weekly run is green
in every job and reproduces every pinned number of the exhaustive runs. The constants commit
precedes the first commit that holds a rewarded run's outcome. The determinism pin and the format
are unchanged.

## Report

The closing message states: the instrument as built (the set shape, the geometry at both sizes and
its census check, the readout window and its derivation); the calibration (both candidates at both
sizes, the measure per run, which gain passed or that none did); the constants commit and the first
outcome commit; the criterion clause by clause at both sizes, beside ADR-0060's readings; whether a
reward now changes this behaviour, and if not, what the calibration's pass lets the ADR say about the
rule; the runtime suite's time before and after, and what the mutation gate found; what was not done
and why; and what the re-examination after the round recommends next.
