---
status: archived
date: 2026-09-21
---

> **Executed 2026-09-21 in pull request #93.** Writes ADR-0076 (two injections: F-46's drive
> kept whole and a cancel, a negative basal message into the same units inside the trial, as a
> field of the task's stimulus with its refusals and a bit-for-bit pin; the brief's offset,
> `REFRACTORY_TICKS`, lands inside the refractory window, which the membrane rule drops, and
> its three sizes are too small for the soma the coupling has primed, so the cancel was
> derived from the engine's rule instead — the tick after the window, over the nine ticks the
> volley spreads, six messages at the bound per tick, each step by an integer oracle over
> `integrate` held to the engine's own probe through the task; under it every unit of the
> presented set fires once and not again, the volley's depression goes to nothing and the
> background's halves, but the volley's potentiation halves too, because the response
> ADR-0072 measured was the response to three volleys, and the terms sum to −2.0 per synapse
> per trial against the +5 hypothesised; the sight passes at 62 of 64 and the sign reads 13,
> so there is no rewarded run); F-46 resolved and F-47 narrowed; H-12's stopping rule at its
> first step, the stimulus side exhausted. Image format 14 unchanged; the determinism pin
> untouched; no constant of the engine or the instrument moved; the gain 1.75. Every
> deliverable is done or dispositioned; notes under the boxes say where the tree departs from
> the text: the offset and the sizes, which the membrane rule and the oracle overruled under
> the brief's empowerment and which are probed as written beside the derived ones; the
> criterion's runs were not made, because the sign passed under no candidate and the brief's
> own rule for that case is no rewarded run. The report is in the pull request and in
> `CHANGELOG.md`. Relative links gained one `../` so that they resolve from `archive/`; no
> word, claim or figure changed. The body below describes the tree before execution and is
> not maintained.

# Brief 034 — Two injections: F-46's drive kept whole, so the response it evokes keeps its strength, and a cancelling message at the end of the refractory window, so the unit does not fire again; each clause of the measure gets a knob of its own

> A self-contained prompt for one round of work. Read `briefs/README.md`, then this file, then
> the files it names. Frozen when executed; the outcome lives in the ADRs that accept it, the
> changelog and the code.

## Mission

[ADR-0074](../../docs/adr/0074-a-stimulus-that-fires-once.md) proved that no stimulus of **one**
message can fire each unit of the presented set once inside the pair window. It is not that the
right value was missed: "One message adds the same amount to every unit while the units' standing
basal potentials spread wider than the band between the drive that fires once and the drive that
fires twice … the after-count is monotone in the drive, so the two clauses do not overlap." A
drive small enough that no unit fires twice leaves units out of the volley; a drive large enough
for every unit to join the volley fires some of them again. One knob, two clauses, no overlap.

The same decision explained why [ADR-0072](../../docs/adr/0072-what-the-trace-is-made-of.md)'s
estimate of **+5 per synapse per trial** was not met: the estimate took the volley's depression to
nothing and held the volley's potentiation at 17.2, and under a weaker stimulus "the volley's
potentiation fell with the response it pairs (10.6–13.5 against 17.2)". The candidates removed
the stimulus's extra spikes by weakening the stimulus, and weakened the response the trace is
made of along with them.

ADR-0074 named the shape that does neither and did not build it: **two injections** — "a drive,
then a message at the end of the refractory window that cancels what the basal compartment
holds". This round builds it, and nothing else. The first injection is **F-46's own**, two
messages of 1.25, which already passes the volley clause (50.8 and 50.9 of 51 units, ADR-0072);
the second is a **negative** basal message injected at the end of `REFRACTORY_TICKS`, sized to
take away what the first leaves in the dendrite before the unit can fire on it again. Each clause
now has its own knob: the first injection fills the volley, the second empties the after-window.
And because the first injection is unchanged, the response it evokes — and the volley's
potentiation — should stay at F-46's strength, **which is the condition ADR-0072's +5 was
computed under and ADR-0074's candidates could not keep**. That estimate is tested here, for the
first time, under the assumption it was built on.

A negative message is what an inhibitory synapse delivers every tick; the executor already
carries one (`spike_message` sign-extends the efficacy). What changes is **when** the task
injects, which is the runtime's composition ([ADR-0059](../../docs/adr/0059-a-task-a-readout-and-a-reward.md)),
not a rule of `cortex-core`. The rule, the gain, the geometry, the readout window, the trial, the
block, the baseline, the reward and both seeds are untouched. If both calibrations pass, ADR-0069's
criterion runs with ADR-0068's addressed delivery, as brief 033 would have run it.

---

## Standing directives

- **Latest ≠ Newest** (whitepaper §2.1, `CLAUDE.md` principle 4). This round adds **no
  mechanism**: a negative basal message is the executor's existing message, delivered by an
  inhibitory synapse every tick, and the change is to the moment the runtime's task injects one.
  **No** new learning rule, gradient, surrogate gradient, e-prop, reward predictor, critic,
  structural plasticity, per-unit synaptic scaling (still gated on H-8), rate balance of $A_-$
  against $A_+$, controller, or 2025–2026 method; no dependency, no new tool, no version bump of a
  tool. A constant of the *engine* may not be touched at all.
- Every claim is Implemented, Specified, Target or Hypothesis. ADR-0072's +5 is a **Hypothesis**,
  named as one wherever it is compared with ([ADR-0010](../../docs/adr/0010-measured-or-target.md)).
  Nothing here says anything about 256 units (ADR-0070) or Appendix A's scale. No timing figure
  from a developer machine.
- The repository wins over the document; a disagreement is a numbered finding in whitepaper §11.
  Every number of ADR-0065, ADR-0066, ADR-0069, ADR-0070, ADR-0072 and ADR-0074 stays in them and
  their tests keep pinning them; a run under the new stimulus is a new run with new pins.
- No `f32`/`f64`, in the crates, the tests and the oracles; every operation on a state field
  saturates or wraps by name ([ADR-0029](../../docs/adr/0029-structural-enforcement.md)).
- Every loop ends by construction ([ADR-0062](../../docs/adr/0062-the-first-complete-sweeps-list.md)).
- Every quantity has one owner ([ADR-0016](../../docs/adr/0016-thirty-two-crate-architecture.md)):
  the membrane, its threshold, its leak and its refractory window are `cortex-core`'s; the gain's
  scaling of an injected message is the executor's (F-47); the task, its stimulus and **when it
  injects** are the runtime's. **No new crate**; the crate count stays 32; no record changes; the
  image format stays 14; no reserved byte is taken.
- **Nothing is chosen after a rewarded run.** The cancel is picked by the measure of Deliverable A
  from the candidates listed there, in their order, derived by a rule written there; the measure
  reads no selection, reward or outcome; all of it is committed before the first rewarded run.
- The determinism pin of [ADR-0030](../../docs/adr/0030-verification-governance.md) does not move: a
  `Stimulus` with no cancel must inject exactly what it injects today, bit for bit, held by a test.
  The mutation gate on the changed lines must pass; the new rule of the task module carries a test
  over the lattice of `testkit/prop.rs`; every number an arithmetic oracle can produce is computed
  by that oracle before the test that asserts it — **and checked against the engine's own probe**,
  because ADR-0074's oracle disagreed with the engine on exactly the gain's scaling (F-47).
- A heavy run is an `#[ignore]`d test whose name contains `exhaustive` and runs in the weekly job;
  the pull request's gate grows by at most one test
  ([ADR-0061](../../docs/adr/0061-the-learning-runs-leave-the-gate.md)).
- The engine never amends its own code ([ADR-0031](../../docs/adr/0031-policy-amendment.md)); no
  registry entry (F-37). Conventional Commits with a real body; never commit on `main`; the
  required checks keep their names.

## Context

Re-derived on 2026-09-21 against `main` at `17e06c6`. Line numbers move; the symbols and the
quoted sentences are what to re-derive.

1. **Why one message cannot do it** ([ADR-0074](../../docs/adr/0074-a-stimulus-that-fires-once.md),
   `tests/instrument.rs`, `ONCE_1024`): against the measure's two clauses — one spike per unit
   within two before the readout window opens (`VOLLEY_TOLERANCE_TENTHS = 20`) and at most a tenth
   per unit in the pair window after it (`AFTER_MAX_TENTHS = 1`) — the candidates read:

   | Stimulus | Volley of 51 | After, per unit | Sight | Sign | Terms per synapse per trial |
   | :--- | ---: | ---: | ---: | ---: | ---: |
   | F-46's, 2 × 1.25 | 50.8, 50.9 | 2.06 | 62 | 1 | −15.7 |
   | 1 × 1.0 | 48.6, 48.7 | 0.28 | 63 | 2 | −3.7 |
   | 1 × 1.25 | 50.4, 50.5 | 0.70 | 63 | 1 | −5.5 |
   | 1 × 1.125 | 50.1, 49.9 | 0.46 | 62 | 0 | −4.7 |

   **F-46's own stimulus passes the volley clause**; only the after clause fails it.
2. **Why the +5 was not met.** ADR-0074: "The volley's depression fell to about 3 (not to nothing),
   the background's to 15–18 (not 11), and the volley's potentiation fell with the response it
   pairs (10.6–13.5 against 17.2)." ADR-0072's estimate had held that potentiation at 17.2.
3. **The membrane that sets the timing** (`crates/cortex-core/src/dynamics/membrane.rs`,
   [ADR-0018](../../docs/adr/0018-membrane-integration.md)): `REFRACTORY_TICKS = 200`,
   `BASAL_LEAK_SHIFT = 9` (a basal time constant of $2^9 = 512$ ticks). The engine's own probe of
   one injection into a unit at rest at the gain 1.75 (`PROBED_1024`): F-46's two messages of 1.25
   fire it **at 5 and at 206 ticks** — once on the drive and again as the refractory window ends on
   what the dendrite still holds; one of 1.25 fires it once at 22; one of 1.0 not at all.
4. **The gain scales an injection** (**F-47**, open): "The executor scales an injected message by
   the tick's synaptic gain as it scales a synapse's (`scaled` in `executor.rs`)", so F-46's two
   messages of 1.25 arrive as 4.375 of basal drive at 1.75, not the 2.5 ADR-0038 described.
   **Any magnitude this round derives is derived after the gain**, and the efficacy injected is
   that magnitude divided by the gain.
5. **What a cancel meets in the network.** ADR-0074: "the instrument's drive delivers a message of
   0.125 — 0.219 after the gain — to a unit every 128 ticks on average, so a unit of the presented
   set is not at rest when the stimulus arrives: it carries a standing basal potential of the order
   of the threshold with a spread of the same order". A cancel sized for a unit at rest may leave
   the units that carried the most still above threshold; **a cancel that takes away more than is
   there cannot make a unit fire**, so erring large costs the stimulus units their own later firing
   — which is F-46's excess, the thing this round removes — and nothing else within the trial.
6. **Where the injection happens.** `runtime/cortex-runtime/src/task.rs`: `Stimulus { set,
   messages, efficacy_q16 }`, "injected between ticks before a trial's first tick", by
   `Stimulus::inject`; `Task::trial` runs the whole trial, and `tests/instrument.rs` calls it
   (`task.trial(&mut exec, trial as u64)`), so a second injection inside the trial is a change to
   `Task::trial` and the `Stimulus` type — to `src/`, not to the harness. The type's own comment,
   "Two messages of 1.25 (the replay drive's, ADR-0038) fire a unit at its base threshold exactly
   once", is what F-47 shows is not so at the gain the network runs at.
7. **What the criterion requires** (ADR-0074's Deliverable D): the offset the criterion needs is
   **one to three spikes per trial** on the answer readout; ADR-0069's delivery moved the margin
   **0.4 to 0.8 of a spike per trial, downward**; the instrument's bias is ±0.53. "The sign is the
   obstacle first; the magnitude is within a small factor of enough." The machinery (`bias`,
   `offset`, `block_bias` in `tests/instrument.rs`) exists and costs nothing to run again.
8. **The budget and the evidence** ([ADR-0073](../../docs/adr/0073-the-whole-domain-tests-sharded.md),
   [ADR-0075](../../docs/adr/0075-the-dispatch-scope-follows-the-diff.md)). `instrument.rs` is a shard
   of the whole-domain tests by itself and this round adds to it; the round states that shard's time
   against its 120-minute bound, and if it nears the bound says what would come out rather than
   raising it. **This brief names no dispatch scope**: under ADR-0075 the executing round decides
   from its own diff and records which clause applied. (This round's diff will change `src/task.rs`,
   which is ADR-0075's first clause; the round confirms that against the diff it has.)
9. **What waits on this.** H-12 carries three no's and, since ADR-0074, a quantified obstacle: the
   sign, first. H-11's synaptic half still waits on a reward that changes a behaviour. F-47's other
   question — what ADR-0038's replay does at the network's gain — is **not** this round's.

## Deliverables

- [x] **The two-injection stimulus (the next free ADR number; `ls docs/adr`)** (`depends-on:
  ADR-0074`; ADR-0018, ADR-0059, ADR-0065, F-46 and F-47 named). In
  `runtime/cortex-runtime/src/task.rs`, a stimulus may carry a **cancel**: a basal message of
  **negative** efficacy into every unit of the set, injected between ticks at an offset from the
  trial's first tick. The shape of the type is the round's to argue; all of these hold:
  - (a) **a stimulus with no cancel injects exactly what it injects today**, bit for bit — held by a
    test with the previous `Stimulus::inject` as its oracle — so every pinned number of every prior
    round stands and the determinism pin cannot move;
  - (b) `Task::check` keeps every refusal ADR-0059 and ADR-0065 wrote and gains one for each new
    way to be wrong — a cancel at offset zero (it would be the first injection), a cancel at or
    beyond the trial's last tick, a cancel whose efficacy is not negative (that is a second drive,
    not a cancel) — each with a test, and a lattice property over `testkit/prop.rs` that a trial
    with a cancel injects the first messages before its first tick and the cancel at its offset and
    at no other tick;
  - (c) the **offset is `REFRACTORY_TICKS`**, 200 ticks from the injection, and is not searched:
    every unit of the volley fires within the pair window before the readout window opens, so the
    cancel lands inside every such unit's refractory window, and the residual and the cancel decay
    in the same compartment at the same rate from there;
  - (d) the `Stimulus` comment F-47 shows false is corrected to what the probe reads, since this
    round edits the type.

  **Done, with (c) overruled by the tree** (ADR-0076): `Cancel { offset, ticks, messages, efficacy_q16 }` on `Stimulus::cancel`, a list of timed injections over a span, the empowerment's shape; (a) held by `a_stimulus_with_no_cancel_injects_what_it_injected_before` against ADR-0059's loop, and every pinned run reruns unchanged; (b) four refusals (`EmptyCancel` added for no message or no tick) each at its edges, the lattice property `a_cancel_injects_at_its_ticks_and_at_no_other`, and the engine held to the oracle's timing at a gain of 1.0; (c) **rejected as written**: the membrane rule drops an input inside the refractory window (ADR-0018), so a cancel at `REFRACTORY_TICKS` lands on tick 201, inside the window of every unit that fired in the volley, and is dropped — probed as written on the engine, `PROBED_CANCEL_1024` — and the offset is `REFRACTORY_TICKS + 1` over a span of nine ticks derived from the volley's census; (d) done.
- [x] **The cancel, and the measure that picks it (the same ADR).** The first injection is **F-46's,
  unchanged** (`STIMULUS_MESSAGES = 2`, `STIMULUS_Q16`). The cancel's magnitude, **derived after the
  gain** (Context item 4) by an integer oracle from `BASAL_LEAK_SHIFT` and `REFRACTORY_TICKS`, and
  **checked against the engine's probe of a unit at rest before it is used** — the candidates, in
  this order and no other:
  - (i) **the residual**: what F-46's first injection, after the gain, leaves in the basal compartment
    of a unit at rest at the cancel's offset — the least that stops a unit at rest firing again;
  - (ii) **twice the residual**: (i) and as much again for the standing potential ADR-0074 reported,
    "of the order of the threshold with a spread of the same order";
  - (iii) **the bound**: the most negative efficacy a message carries (`MESSAGE_EFFICACY_MIN`), which
    cannot under-cancel and so tests whether the mechanism works at all, apart from its size.

  Each is converted to an efficacy by dividing by the gain, stated, and **probed on a unit at rest
  first**: it passes the probe when F-46's first injection with that cancel fires the unit exactly
  once in `PROBE_TICKS`. Then **the pick, ADR-0074's measure unchanged** — the presented set fires
  one spike per unit within two before the readout window opens and adds no more than a tenth per
  unit in the pair window after it — over a frozen run of 64 trials at 1 024 units and the gain
  1.75. The first candidate passing the probe and both clauses is the stimulus; if none does, there
  is no rewarded run, and the ADR records every candidate's probe and census and says which clause
  each failed.

  **Done, with the candidates re-derived under the empowerment** (ADR-0076): the brief's three — the residual (one message of −111 082, the 2.97 after the gain on tick 200 divided by it), twice it, the bound — were probed as written at the brief's offset (dropped: 5 and 206 each) and at the derived span (too small for the soma: 5 and 206 each), the oracle reading the residual as 2.94 after the gain on tick 205 and the least cancel on tick 206 as 7.41, 2.5 times it; the derived candidates in messages at the bound per tick, in order, 3 (the least at rest), 6 (the least at the extreme standing state) and 12 (twice it), each probed (5 alone) and run: 3 leaves 0.83 spikes per unit after the opening and fails the after clause; **6 fires every unit once and not again and is the pick** (`CANCEL_PICKED_1024`); 12 reads the same. The measure read no outcome.
- [x] **The composition re-read, against the condition the +5 assumed.** Under the chosen stimulus,
  ADR-0072's composition by its own test at no extra cost: the volley's potentiation and depression,
  the background's, their sum per trial and the standing trace after the block, beside ADR-0072's
  17.2, 10.4, 7.5, 26.8, −15.7 and −66.6 and ADR-0074's three candidates. **The expectation, written
  before the run and not adjusted to it:** the volley's potentiation **stays near 17.2**, because the
  first injection is F-46's and the response it evokes is F-46's; the volley's depression **falls
  toward zero**, because the stimulus units do not fire again to pair the response as depression; and
  the terms reach **ADR-0072's +5, named as the Hypothesis it is**. If the potentiation falls anyway,
  the response itself changed and the ADR says what the census shows changed it; if the sum stays
  negative with the potentiation held, the background's depression is what remains, and the ADR says
  by how much.

  **Done, and the expectation not met** (ADR-0076's table): under the pick the volley's potentiation is 8.0 against 17.2, its depression −0.2 against −10.4, the background's potentiation 6.6 against 7.5 and its depression −12.7 against −26.8; the terms sum to −2.0 per synapse per trial against the +5 Hypothesis, the standing trace −9.0 against −66.6. The potentiation fell although the first injection is F-46's, and the census says what changed it: the response ADR-0072 measured in the pair window after the opening was the response to three volleys (the after-spikes were volleys too), 19.2 spikes per readout set per trial against 8.9 to one, and the potentiation fell in the same proportion; the background's depression is what remains, halved and not gone.
- [x] **The two calibrations at 1.75, unchanged.** ADR-0065's **sight** and ADR-0072's **sign**, each
  56 of 64, weights frozen, neither reading a selection, a reward or an outcome. **The gain does not
  move and the ladder is not walked.** If the sign fails, there is no rewarded run.

  **Done, with the outcome the box names:** the sight 63, 62 and 62 of 64 and the sign 2, 13 and 13 under the three candidates; the gain did not move and the ladder was not walked; the sign failed under the pick (13), so there is no rewarded run (`CANCEL_CALIBRATED_1024` is false).
- [x] **What the criterion requires, read off the trials again.** ADR-0074's three integer readings —
  the instrument's bias, the offset the criterion needs, what a delivery moved — under the chosen
  stimulus, beside ADR-0074's, in every branch of this round. If the bias exceeds what a delivery
  moves, a numbered finding, as brief 033 asked; nothing moves on these numbers.

  **Done** in the branch the round took: (a) the bias under the pick +6/34 and +3/30, toward the answer on both stimuli (+14/34 and +9/30 under 3, +5/34 and +4/30 under 12), against +18/34 and −16/30 under F-46's stimulus; (b) the offset 1 under the pick (2 and 1 under the others) at 40 of 64; (c) ADR-0074's reading of ADR-0069's blocks, 0.4 to 0.8 of a spike per trial downward, quoted. The bias is below what the delivery moved, so no finding fires and nothing moves.
- [x] **The constants commit**, preceding the first commit that holds a rewarded outcome: the chosen
  cancel with its derivation, its probe and its census, and every constant of ADR-0065, ADR-0068 and
  ADR-0069 restated unchanged.

  **Done as the round's shape allows:** there is no rewarded outcome for it to precede. `418b8fd` on the branch holds the mechanism, the oracle, the derived constants, the probes and both expectations, written before any run; `e2971eb` holds the census and the pick; every constant of ADR-0065, ADR-0068 and ADR-0069 is restated unchanged by the pinned runs, which pass `None` as the cancel and rerun bit for bit.
- [x] **The criterion, ADR-0069's, unchanged**, run only if both calibrations pass: at 1 024 units
  with ADR-0068's addressed delivery, the addressed rewarded run, the mirrored assignment, the
  addressed shuffled reward and the fixed modulation, as weekly `exhaustive` tests; ADR-0069's own
  runs are the comparison. Over the last 128 trials: rewarded **at least 80** in both assignments;
  the controls **at most 76**; the sequence the same on one worker and on four. **The expectation,
  written first:** with the trace positive the answer couplings **rise above** the other two — the
  ratio ADR-0069 read as 0.937 goes above the census's 0.992; a failure with the ratio above 1 is a
  magnitude problem, and with Deliverable D's offset beside it the ADR says how far short; a failure
  with the ratio below 1 means the sign did not reach the couplings.

  **Rejected** by the brief's own rule: both calibrations must pass first and the sign passed under no candidate; the expectation written for a positive trace stays untested.
- [x] **The gate.** At most one test, running no whole run: the probe of every candidate on a unit at
  rest, and the chosen stimulus's first block held to the first row of its pinned table (or, if
  none is chosen, candidate (iii)'s). Nothing else added to the gate.

  **Done:** one test, `the_first_eight_trials_of_the_two_injections_at_1024_units_and_the_rules_over_their_tables`, the eleven probes through the task held to their table and to the oracle, the rules over the pinned tables, and the first eight trials of the pick held to the first eight rows and counts of its tables (the first eight trials rather than the first block, as ADR-0072's and ADR-0074's gate tests are).
- [x] **The evidence.** A weekly dispatched on this round's branch at the scope
  [ADR-0075](../../docs/adr/0075-the-dispatch-scope-follows-the-diff.md) gives **this round's own
  diff**, the clause that applied stated in the ADR; green in every job it runs, every pinned number
  reproduced, no survivor; its run id, the three exhaustive shards' times with `instrument.rs`'s
  against its bound, and — if the sweep ran — the six runtime shards' times, all in the ADR.

  **Done:** the diff changes `runtime/cortex-runtime/src/task.rs` and `src/lib.rs`, ADR-0075's first clause, so the weekly was dispatched at `scope=both`; the run id, the shards' times and what the sweep found are in ADR-0076's evidence.
- [x] **The documents, in the same pull request.** Whitepaper §11's **F-46** (resolved, or narrowed
  with what remains) and **F-47** (the `Stimulus` comment corrected; the replay question left open
  and said to be left); §11.1's **H-12** and H-11's synaptic half; §8.8's three-factor row; §6.5's
  "The loop as the runtime composes it"; §9, the ADR index, `CHANGELOG.md`, `README.md`, `CLAUDE.md`'s
  opening paragraph and `docs/zh-TW`'s reader's guide as the result requires. The whitepaper's
  version moves in **both** declarations with its date
  ([ADR-0064](../../docs/adr/0064-the-documentation-gate-and-the-version.md)). Executable directives
  under every sentence that claims a test, a type or a constant exists.

  **Done:** F-46 resolved and F-47 narrowed in §11, H-11 and H-12 and the stopping rule's first step in §11.1, §8.8's three-factor row (§5.2.1's paragraph), §6.5, §9, the ADR index, `CHANGELOG.md`, `README.md`, `CLAUDE.md` and the reader's guide; the whitepaper at 4.28.0 in both declarations; directives under the type, the injection, the oracle, the two tests, the pick and the calibration.
- [x] **The brief archived** as `briefs/README.md` says, every deliverable dispositioned, the frozen
  banner naming the pull request, the ADRs and the outcome of each measure.

  **Done:** this file.

## Not empowered

- **No constant of the engine moves**: not `REFRACTORY_TICKS`, `BURST_REFRACTORY_TICKS`,
  `BASAL_LEAK_SHIFT`, the threshold, `STDP_A_PLUS_Q1_15`, `STDP_A_MINUS_Q1_15`,
  `STDP_DEPRESSION_REFERENCE_Q1_15`, `STDP_TAU_SHIFT`, `ELIGIBILITY_TAU_SHIFT`, `DOPAMINE_TAU_SHIFT`,
  ADR-0055's magnitude scaling, the inhibitory rule or its period, the estimator, the controller, the
  sleep constants, `Prior` or the reference prior's parameters.
- **The executor's scaling of an injected message by the gain is not changed** (F-47 records it; this
  round derives around it). No change to `spike_message` or to `MESSAGE_EFFICACY_MIN`.
- **The gain does not move and the ladder is not walked; the controller stays off.**
- **F-46's first injection is not changed**: it is what keeps the response at its strength, and it
  is the design's premise.
- No cancel outside the three candidates and their order; no offset other than `REFRACTORY_TICKS`;
  no second positive injection.
- No change to ADR-0068's delivery — its addressing, its narrowing by both ends, the moment it
  consolidates — no reward of one sign, no consolidation at the trial's end.
- No change to the geometry or its masks, the rotations, the readout window, the trial, the block,
  the run, the baseline, the reward magnitude, or either seed.
- **Not F-47's replay question**: what ADR-0038's replay does at the network's gain is named in the
  ADR and left, as ADR-0074 left it.
- No constant chosen or moved after a rewarded run; no criterion changed after a run; no clause
  dropped because it failed.
- No record change, no new section, no format bump, no reserved byte taken; no new crate; no
  dependency in a state crate; no `unsafe` outside the runtime's arena access; no float anywhere.
- No more than one test added to the pull request's gate. No change to `.github/workflows/ci.yml`,
  `scripts/exhaustive-shard.sh` or the sweep's configuration. No renaming of a required check. No
  move of the determinism pin, and no edit to a number a prior round pinned.
- No claim about 256 units or Appendix A's scale; no use of "learns" outside the task, the accuracy,
  the trials and the controls that produced it.

## Architectural empowerment

The executing session may replace any instruction above with a better decision, recorded in the ADR
that owns it: the shape of the cancel in the type (a field of `Stimulus`, a list of timed injections
of which today's is the first); how the probe and the calibration are organised into tests; the
oracle's form, so long as it is integer, written before the run and checked against the engine's
probe; whether the round writes one ADR or two; which numbers the ADRs restate; and whether the
criterion's controls are reduced **if and only if** the `instrument.rs` shard would otherwise near
its bound, in which case the addressed shuffled reward goes first and then the mirrored assignment,
and the rewarded run, the fixed modulation and the worker clause are not droppable. It may not reach
the standing directives, the whitepaper's invariants or the constraints in `CLAUDE.md`.

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
job it runs and reproduces every pinned number. A stimulus with no cancel injects what it did, bit for
bit. The constants commit precedes the first commit that holds a rewarded run's outcome. The
determinism pin, the image format and every number a prior round pinned are unchanged.

## Report

The closing message states: the cancel as built — the type's shape, the offset and why it is
`REFRACTORY_TICKS`, the refusals and the bit-for-bit test; the three candidates with their derivation
after the gain, their probe on a unit at rest and their census, and which passed or that none did and
at which clause; the composition under the chosen stimulus term by term beside ADR-0072's and
ADR-0074's, whether the volley's potentiation held near 17.2, and whether the +5 was met, said as the
Hypothesis it was; both calibrations at 1.75; Deliverable D's three numbers beside ADR-0074's; the
constants commit and the first outcome commit; if the criterion ran, its clauses beside ADR-0069's 56,
65, 53 and 60 and the couplings' ratio against 0.937 and the expectation; the scope ADR-0075 gave this
diff and the clause that applied; the shards' times with `instrument.rs`'s against its bound; what the
mutation gate and any sweep found; what was not done and why; and what the re-examination after the
round recommends next.
